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
use web_sys::Window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;

macro_rules! log { ($($t:tt)*) => {web_sys::console::log_1(&format!($($t)*).into())}; }

#[wasm_bindgen]
struct AnimationFrameCallbackWrapper {
    handle: Option<i32>,
    // func: Option<RefCell<Box<dyn FnMut() -> bool>>>,
    func: Option<Box<dyn FnMut() -> bool>>,
}

impl !Unpin for AnimationFrameCallbackWrapper { }

impl Drop for AnimationFrameCallbackWrapper {
    fn drop(&mut self) {
        self.handle.map(cancel_animation_frame)
    }
}

pub(crate) fn cancel_animation_frame(handle: i32) {
    web_sys::window().unwrap()
        .cancel_animation_frame(handle).unwrap()
}

impl AnimationFrameCallbackWrapper {
    fn new() -> Self {
        Self {
            handle: -1,
            func: None,
        }
    }

    fn start(&mut self, func: impl FnMut() -> bool) {
        if let Some(handle) = self.handle {
            cancel_animation_frame(handle)
        }

        // self.func = Some(RefCell::new(Box::new(func)));
        self.func = Some(Box::new(func));

        let func = self.func.borrow_mut();
        // This is the dangerous part; we're saying this is okay because we
        // cancel the RAF on Drop of this structure so, in theory, when the
        // function goes out of scope, the RAF will also be cancelled and the
        // invalid reference won't be used.
        // let func: &'static mut _ = unsafe { func }; // TODO: we can ditch this variable; by having it we're doing mutable aliasing which is A Crime
        let wrapper: &'static mut _ = unsafe { self };

        let window = web_sys::window().unwrap();

        fn recurse(w: &'static mut AnimationFrameCallbackWrapper, window: Window) -> Function {
            let val = Closure::once_into_js(move || {
                if w.as_mut().unwrap().func() {
                    let next = recurse(w, window);
                    let id = window.request_animation_frame(&next).unwrap();
                    w.handle = Some(id);
                } else {
                    cancel_animation_frame(wrapper.handle.take().unwrap());
                    drop(wrapper.func.take());
                }
            });

            val.dyn_into().unwrap()
        }

        // let cl: Closure<dyn FnMut()> = Closure::once(move || {
        //     if wrapper.func() {

        //         let id = window().request_animation_frame().unwrap();
        //         wrapper.handle = Some(id);
        //     } else {
        //         cancel_animation_frame(wrapper.handle.take().unwrap());
        //         drop(wrapper.func.take())
        //     }
        // });

        let starting = recurse(wrapper, window.clone());
        self.handle = Some(window.request_animation_frame(&starting).unwrap());
    }
}

#[wasm_bindgen]
pub fn run() -> Result<AnimationFrameCallbackWrapper, JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(None);

    term.open(terminal_div);

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

    // Don't drop!
    Box::leak(Box::new(l));

    let term = term_orig;

    term.focus();

    term.write(String::from("\x1B[35;31m hello!\n"));
    term.write(String::from("\x1B[1;3;31mxterm.js\x1B[0m with 🦀\n$ "));
    // window.request_animation_frame()

    Ok(())
}
