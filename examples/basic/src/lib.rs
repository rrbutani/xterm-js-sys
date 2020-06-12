use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use xterm_js_sys::xterm::{LogLevel, Terminal, TerminalOptions};

#[path = "../../common.rs"]
mod common;
use common::log;

#[macro_export]
#[doc(hidden)]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

pub(crate) const ENABLE_MOUSE_MODE_CSI_SEQUENCE: &str = concat!(
    csi!("?1000h"),
    csi!("?1002h"),
    csi!("?1015h"),
    csi!("?1006h")
);

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let opts = TerminalOptions::new()
        .with_log_level(LogLevel::Debug);

    let term_orig = Terminal::new(Some(opts));

    term_orig.open(terminal_div);
    term_orig.write(ENABLE_MOUSE_MODE_CSI_SEQUENCE.to_string());

    let term = term_orig.clone();
    let l = term_orig.attach_key_event_listener(move |e| {
        // A port of the xterm.js echo demo:
        let key = e.key();
        let ev = e.dom_event();

        let printable = matches!(
            (ev.alt_key(), ev.ctrl_key(), ev.meta_key()),
            (false, false, false)
        );

        const ENTER_ASCII_KEY_CODE: u32 = 13;
        const BACKSPACE_ASCII_KEY_CODE: u32 = 8;

        match ev.key_code() {
            ENTER_ASCII_KEY_CODE => {
                term.write("\n\r\x1B[1;3;31m$ \x1B[0m".to_string())
            }
            BACKSPACE_ASCII_KEY_CODE => {
                term.write("\u{0008} \u{0008}".to_string())
            }
            _ if printable => term.write(key),
            _ => { },
        }

        log!("[key event] got {:?}", e);
    });

    let b = term_orig.attach_binary_event_listener(move |s| {
        log!("[binary event] bin: {:?}", s);
    });

    let d = term_orig.attach_data_event_listener(move |s| {
        log!("[data event] data: {:?}", s);
    });

    let r = term_orig.attach_resize_event_listener(move |r| {
        log!("[resize event] resize: {:?}", r);
    });

    // Don't drop!
    Box::leak(Box::new(l));
    Box::leak(Box::new(b));
    Box::leak(Box::new(d));
    Box::leak(Box::new(r));

    let term = term_orig;

    term.focus();

    term.write(String::from("\x1B[35;31m hello!\n"));
    term.write(String::from("\x1B[1;3;31mxterm.js\x1B[0m with 🦀\n$ "));

    Ok(())
}
