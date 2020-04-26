extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

extern crate xterm_js_sys;

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use xterm_js_sys::xterm::Terminal;

macro_rules! log { ($($t:tt)*) => {web_sys::console::log_1(&format!($($t)*).into())}; }

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document.get_element_by_id("terminal").expect("should have a terminal div");

    let term_orig = Terminal::new(None);

    term_orig.open(terminal_div);

    let term = term_orig.clone();
    let l = term_orig.attach_key_event_listener(move |e| {
        /// A port of the xterm.js echo demo:
        let key = e.key();
        let ev = e.dom_event();

        let printable = matches!((ev.alt_key(), ev.ctrl_key(), ev.meta_key()),
            (false, false, false));

        const ENTER_ASCII_KEY_CODE: u32 = 13;
        const BACKSPACE_ASCII_KEY_CODE: u32 = 8;

        match ev.key_code() {
            ENTER_ASCII_KEY_CODE => term.write("\n\r\x1B[1;3;31m$ \x1B[0m".to_string()),
            BACKSPACE_ASCII_KEY_CODE => term.write("\u{0008} \u{0008}".to_string()),
            _ => term.write(key),
        }

        log!("[key event] got {:?}", e);
    });

    // Don't drop!
    Box::leak(Box::new(l));

    let term = term_orig;

    term.focus();

    term.write(String::from("\x1B[35;31m hello\n"));
    term.write(String::from("\x1B[1;3;31mxterm.js\x1B[0m $ "));
    // window.request_animation_frame()

    Ok(())
}
