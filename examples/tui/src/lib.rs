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

use std::cell::Cell;

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

    pub fn safe_start(&'static mut self, func: impl FnMut() -> bool + 'static) {
        unsafe { self.inner(func) }
    }

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
pub fn run() -> Result<Option<AnimationFrameCallbackWrapper>, JsValue> {
// pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(None);

    term.open(terminal_div.clone());

    let mut a = AnimationFrameCallbackWrapper::new();

    let mut b = 0;
    let f = move || {
        log!("heyo! {}", b);
        b += 1;

        if b % 1 == 0 {
            term.write(format!("ayo: {}\r\n", b));
        }

        if b % 600 == 0 {
            term.reset()
        }

        b != 3600
    };

    // unsafe { a.start(f) }
    let mut a = a.leak();
    a.safe_start(f);

    // log!("runnin!!: {:?} @ {:?}", a.handle, &a.handle as *const _);

    // drop(a);

    // Ok(())


    let mut b = AnimationFrameCallbackWrapper::new().leak();
    b.safe_start(move || {
        log!("yak!");

        true
    });

    // Ok(Some(a))
    Ok(None)
    // Ok(())
    // let term = term_orig.clone();
    // let l = term_orig.attach_key_event_listener(move |e| {
    //     // A port of the xterm.js echo demo:
    //     let key = e.key();
    //     let ev = e.dom_event();

    //     let printable = matches!(
    //         (ev.alt_key(), ev.ctrl_key(), ev.meta_key()),
    //         (false, false, false)
    //     );

    //     const ENTER_ASCII_KEY_CODE: u32 = 13;
    //     const BACKSPACE_ASCII_KEY_CODE: u32 = 8;

    //     match ev.key_code() {
    //         ENTER_ASCII_KEY_CODE => {
    //             term.write("\n\r\x1B[1;3;31m$ \x1B[0m".to_string())
    //         }
    //         BACKSPACE_ASCII_KEY_CODE => {
    //             term.write("\u{0008} \u{0008}".to_string())
    //         }
    //         _ if printable => term.write(key),
    //         _ => { },
    //     }

    //     log!("[key event] got {:?}", e);
    // });

    // // Don't drop!
    // Box::leak(Box::new(l));

    // let term = term_orig;

    // term.focus();

    // term.write(String::from("\x1B[35;31m hello!\n"));
    // term.write(String::from("\x1B[1;3;31mxterm.js\x1B[0m with 🦀\n$ "));
    // // window.request_animation_frame()

    // Ok(())
}
