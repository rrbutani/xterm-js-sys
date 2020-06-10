extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;

extern crate xterm_js_sys;

use console_error_panic_hook::set_once as set_panic_hook;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use xterm_js_sys::{
    crossterm_support::XtermJsCrosstermBackend,
    xterm::{LogLevel, Terminal, TerminalOptions},
};
use js_sys::Function;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline},
    Terminal as TuiTerminal,
};
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::{Crypto, Window};

use std::cell::Cell;
use std::io::Write;

macro_rules! log { ($($t:tt)*) => {web_sys::console::log_1(&format!($($t)*).into())}; }

#[wasm_bindgen]
pub struct AnimationFrameCallbackWrapper {
    // These are both boxed because we want stable addresses!
    handle: Box<Cell<Option<i32>>>,
    func: Option<Box<dyn FnMut() -> bool + 'static>>,
}


impl Drop for AnimationFrameCallbackWrapper {
    fn drop(&mut self) {
        self.handle.get().map(cancel_animation_frame);
    }
}

pub(crate) fn cancel_animation_frame(handle: i32) {
    log!("Cancelling {}..", handle);

    web_sys::window().unwrap()
        .cancel_animation_frame(handle).unwrap()
}

impl/*<'a>*/ AnimationFrameCallbackWrapper/*<'a>*/ {
    fn new() -> Self {
        Self {
            handle: Box::new(Cell::new(None)),
            func: None,
        }
    }

    pub fn leak(self) -> &'static mut Self {
        Box::leak(Box::new(self))
    }

    /// To use this, you'll probably have to leak the wrapper.
    ///
    /// `Self::leak` can help you with this.
    pub fn safe_start(&'static mut self, func: impl FnMut() -> bool + 'static) {
        unsafe { self.inner(func) }
    }

    /// This is extremely prone to crashing and is probably unsound; use at your
    /// own peril.
    #[inline(never)]
    pub unsafe fn start<'s, 'f: 's>(&'s mut self, func: impl FnMut() -> bool + 'f) {
        log!(""); // load bearing, somehow...
        self.inner(func)
    }

    // Not marked as unsafe so unsafe operations aren't implicitly allowed..
    /*unsafe */fn inner<'s, 'f: 's>(&'s mut self, func: impl FnMut() -> bool + 'f) {
        if let Some(handle) = self.handle.get() {
            cancel_animation_frame(handle)
        }

        let func: Box<dyn FnMut() -> bool + 'f> = Box::new(func);
        // Crime!
        let func: Box<dyn FnMut() -> bool + 'static> = unsafe { core::mem::transmute(func) };
        self.func = Some(func);

        // This is the dangerous part; we're saying this is okay because we
        // cancel the RAF on Drop of this structure so, in theory, when the
        // function goes out of scope, the RAF will also be cancelled and the
        // invalid reference won't be used.
        let wrapper: &'static mut Self = unsafe { core::mem::transmute(self) };

        let window = web_sys::window().unwrap();

        fn recurse(
            f: &'static mut Box<dyn FnMut() -> bool + 'static>,
            h: &'static Cell<Option<i32>>,
            window: Window,
        ) -> Function {
            let val = Closure::once_into_js(move || {
                // See: https://github.com/rust-lang/rust/issues/42574
                let f = f;

                if h.get().is_none() {
                    log!("you should never see this...");
                    return
                }

                if (f)() {
                    let next = recurse(f, h, window.clone());
                    let id = window.request_animation_frame(&next).unwrap();
                    h.set(Some(id));
                } else {
                    // No need to drop the function here, really.
                    // It'll get dropped whenever the wrapper gets dropped.
                    // drop(w.func.take());

                    // We should remove the handle though, so that when the
                    // wrapper gets dropped it doesn't try to cancel something
                    // that already ran.
                    drop(h.take())
                }
            });

            val.dyn_into().unwrap()
        }

        let func: &'static mut Box<dyn FnMut() -> bool + 'static> = wrapper.func.as_mut().unwrap();
        let starting = recurse(func, &wrapper.handle, window.clone());
        wrapper.handle.set(Some(window.request_animation_frame(&starting).unwrap()));
    }
}

#[wasm_bindgen]
pub fn alt_run() -> Result<Option<AnimationFrameCallbackWrapper>, JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(None);
    term.open(terminal_div.clone());

    let mut a = AnimationFrameCallbackWrapper::new().leak();

    let mut b = 0;
    a.safe_start(move || {
        log!("heyo! {}", b);
        b += 1;

        if b % 10 == 0 { term.write(format!("ayo: {}\r\n", b)); }
        if b % 600 == 0 { term.reset() }

        b != 3600
    });

    let mut b = AnimationFrameCallbackWrapper::new().leak();
    b.safe_start(move || {
        log!("yak!");

        true
    });

    Ok(None)
}



#[derive(Clone)]
pub struct RandomSignal {
    distribution: Uniform<u64>,
    rng: StdRng,
}

impl RandomSignal {
    pub fn new(crypto: Crypto, lower: u64, upper: u64) -> RandomSignal {
        let mut seed = [0u8; 32];

        crypto.get_random_values_with_u8_array(&mut seed).unwrap();

        RandomSignal {
            distribution: Uniform::new(lower, upper),
            rng: StdRng::from_seed(seed),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

struct App {
    signal: RandomSignal,
    data1: Vec<u64>,
    data2: Vec<u64>,
    data3: Vec<u64>,
}

impl App {
    fn new(crypto: Crypto) -> App {
        let mut signal = RandomSignal::new(crypto, 0, 100);
        let data1 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data2 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data3 = signal.by_ref().take(200).collect::<Vec<u64>>();
        App {
            signal,
            data1,
            data2,
            data3,
        }
    }

    fn update(&mut self) {
        let value = self.signal.next().unwrap();
        self.data1.pop();
        self.data1.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data2.pop();
        self.data2.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data3.pop();
        self.data3.insert(0, value);
    }
}

#[wasm_bindgen]
pub fn run() -> Result<Option<AnimationFrameCallbackWrapper>, JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(None);
    term.open(terminal_div.clone());

    let mut term_temp: XtermJsCrosstermBackend = (&term).into();
    execute!((&mut term_temp), EnterAlternateScreen);
    drop(term_temp);

    let term: &'static _ = Box::leak(Box::new(term));
    let backend = CrosstermBackend::new(term);

    let mut tui = TuiTerminal::new(backend).unwrap();
    tui.hide_cursor().unwrap();

    // Create default app state
    let mut app = App::new(window.crypto().unwrap());

    let mut main_loop = AnimationFrameCallbackWrapper::new().leak();
    main_loop.safe_start(move || {
        tui.draw(|mut f: tui::terminal::Frame<'_, CrosstermBackend<'_, Vec<u8>>>| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(7),
                    Constraint::Min(0),
                ].as_ref())
                .split(f.size());
            let sparkline = Sparkline::default()
                .block(Block::default()
                    .title("Data1")
                    .borders(Borders::LEFT | Borders::RIGHT),
                )
                .data(&app.data1)
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(sparkline, chunks[0]);
            let sparkline = Sparkline::default()
                .block(Block::default()
                    .title("Data2")
                    .borders(Borders::LEFT | Borders::RIGHT),
                )
                .data(&app.data2)
                .style(Style::default().bg(Color::Green));
            f.render_widget(sparkline, chunks[1]);
            // Multiline
            let sparkline = Sparkline::default()
                .block(Block::default()
                    .title("Data3")
                    .borders(Borders::LEFT | Borders::RIGHT),
                )
                .data(&app.data3)
                .style(Style::default().fg(Color::Red));
            f.render_widget(sparkline, chunks[2]);
        }).unwrap();

        app.update();
        log!("hiya!");

        true
    });

    Ok(None)
}

