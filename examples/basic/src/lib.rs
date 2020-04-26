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

    let term = Terminal::new(None);

    // Make it 'static:
    // Box::leak(Box::new(term));

    term.open(terminal_div);

    let term_copy = term.clone();
    let l = term.attach_key_event_listener(move |e| {
        term_copy.write("a".to_string());

        log!("yooo {:?}", e);
    });

    // Don't drop!
    Box::leak(Box::new(l));

    term.focus();

    term.write(String::from("\x1B[35;31m hello\n"));
    term.write(String::from("\x1B[1;3;31mxterm.js\x1B[0m $ "));

    // Closure::wrap(Box::new() as Box)

    // window.request_animation_frame()

    // loop {
    //     log!("hello!");

    // }

    Ok(())
}
