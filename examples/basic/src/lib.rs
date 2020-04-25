extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

extern crate xterm_js_sys;

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use xterm_js_sys::xterm::Terminal;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document.get_element_by_id("terminal").expect("should have a terminal div");

    let term = Terminal::new(None);
    term.open(terminal_div);

    Ok(())
}
