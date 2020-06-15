//! This is a port of the [crossterm demo in the tui crate][demo].
//!
//! [demo]: https://github.com/fdehau/tui-rs/blob/3f62ce9c199bb0048996bbdeb236d6e5522ec9e0/examples/crossterm_demo.rs

use console_error_panic_hook::set_once as set_panic_hook;
use crossterm::{
    event::EventStream,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures_util::stream::StreamExt;
use tui::{backend::CrosstermBackend, Terminal as TuiTerminal};
use wasm_bindgen::prelude::*;
use xterm_js_sys::{
    crossterm_support::XtermJsCrosstermBackend,
    xterm::{LogLevel, /*RendererType, */ Terminal, TerminalOptions, Theme,},
};

use std::{
    io::Write,
    sync::{Arc, RwLock},
};

#[path = "../../common.rs"]
mod common;
use common::{log, AnimationFrameCallbackWrapper};

mod app;
mod ui;
mod util;

#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
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
            .with_font_size(11.0), // .with_renderer_type(RendererType::Dom)
    ));
    term.open(terminal_div.clone());

    let mut term_temp: XtermJsCrosstermBackend = (&term).into();

    execute!((&mut term_temp), EnterAlternateScreen, EnableMouseCapture)
        .unwrap();
    drop(term_temp);

    let term: &'static _ = Box::leak(Box::new(term));
    let backend = CrosstermBackend::new(term);

    term.resize(200, 45);
    term.focus();

    let mut tui = TuiTerminal::new(backend).unwrap();
    tui.hide_cursor().unwrap();

    // Create default app state
    let app = app::App::new(window.crypto().unwrap(), "Demo", true);
    let app = Arc::new(RwLock::new(app));

    let main_loop = AnimationFrameCallbackWrapper::new().leak();
    let app_draw = app.clone();
    main_loop.safe_start(move || {
        let mut app = app_draw.write().unwrap();
        tui.draw(
            |mut f: tui::terminal::Frame<'_, CrosstermBackend<'_, Vec<u8>>>| {
                ui::draw(&mut f, &mut app)
            },
        )
        .unwrap();

        app.on_tick();
        !app.should_quit
    });

    let mut s = EventStream::new(&term);

    loop {
        match s.next().await.unwrap() {
            Ok(Event::Key(k)) => {
                let mut app = app.write().unwrap();
                match k.code {
                    KeyCode::Char('q') => {
                        let mut term: XtermJsCrosstermBackend = (term).into();

                        execute!(
                            (&mut term),
                            LeaveAlternateScreen,
                            DisableMouseCapture,
                            Clear(ClearType::All),
                        )
                        .unwrap();
                        app.should_quit = true;
                    }
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    _ => {}
                }
            }
            Ok(ev) => log!("Unhandled event: {:?}", ev),
            Err(err) => panic!("Err: {:?}", err),
        }
    }
}
