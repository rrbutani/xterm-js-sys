//! This is a pretty direct port of the [sparkline demo in the tui crate][demo].
//!
//! [demo]: https://github.com/fdehau/tui-rs/blob/3f62ce9c199bb0048996bbdeb236d6e5522ec9e0/examples/sparkline.rs

use console_error_panic_hook::set_once as set_panic_hook;
use crossterm::{execute, terminal::EnterAlternateScreen};
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
use wasm_bindgen::prelude::*;
use web_sys::Crypto;
use xterm_js_sys::{
    crossterm_support::XtermJsCrosstermBackend,
    xterm::{LogLevel, Terminal, TerminalOptions, Theme},
};

use std::io::Write;

#[path = "../../common.rs"]
mod common;
use common::{log, AnimationFrameCallbackWrapper};

#[wasm_bindgen]
pub fn alt_run() -> Result<Option<AnimationFrameCallbackWrapper>, JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(Some(
        TerminalOptions::default()
            .with_log_level(LogLevel::Debug)
            .with_theme(Theme::nord())
            .with_font_family("'Fira Mono', monospace")
            .with_font_size(11.0),
    ));
    term.open(terminal_div);

    let a = AnimationFrameCallbackWrapper::new().leak();

    let mut b = 0;
    a.safe_start(move || {
        log!("heyo! {}", b);
        b += 1;

        if b % 10 == 0 {
            term.write(format!("ayo: {}\r\n", b));
        }
        if b % 600 == 0 {
            term.reset()
        }

        b != 3600
    });

    let b = AnimationFrameCallbackWrapper::new().leak();
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

    let term = Terminal::new(Some(
        TerminalOptions::default()
            .with_log_level(LogLevel::Debug)
            .with_theme(Theme::nord())
            .with_font_family("'Fira Mono', monospace")
            .with_font_size(11.0),
    ));
    term.open(terminal_div);

    let mut term_temp: XtermJsCrosstermBackend = (&term).into();
    execute!((&mut term_temp), EnterAlternateScreen).unwrap();
    drop(term_temp);

    let term: &'static _ = Box::leak(Box::new(term));
    let backend = CrosstermBackend::new(term);

    let mut tui = TuiTerminal::new(backend).unwrap();
    tui.hide_cursor().unwrap();

    // Create default app state
    let mut app = App::new(window.crypto().unwrap());

    let main_loop = AnimationFrameCallbackWrapper::new().leak();
    main_loop.safe_start(move || {
        tui.draw(
            |mut f: tui::terminal::Frame<'_, CrosstermBackend<'_, Vec<u8>>>| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(7),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                let sparkline = Sparkline::default()
                    .block(
                        Block::default()
                            .title("Data1")
                            .borders(Borders::LEFT | Borders::RIGHT),
                    )
                    .data(&app.data1)
                    .style(Style::default().fg(Color::Yellow));
                f.render_widget(sparkline, chunks[0]);
                let sparkline = Sparkline::default()
                    .block(
                        Block::default()
                            .title("Data2")
                            .borders(Borders::LEFT | Borders::RIGHT),
                    )
                    .data(&app.data2)
                    .style(Style::default().bg(Color::Green));
                f.render_widget(sparkline, chunks[1]);
                // Multiline
                let sparkline = Sparkline::default()
                    .block(
                        Block::default()
                            .title("Data3")
                            .borders(Borders::LEFT | Borders::RIGHT),
                    )
                    .data(&app.data3)
                    .style(Style::default().fg(Color::Red));
                f.render_widget(sparkline, chunks[2]);
            },
        )
        .unwrap();

        app.update();
        true
    });

    Ok(None)
}
