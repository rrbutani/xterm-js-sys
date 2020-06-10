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
    handle: Cell<Option<i32>>,
    func: Option<Box<dyn FnMut() -> bool + 'static>>,
}


impl Drop for AnimationFrameCallbackWrapper {
    fn drop(&mut self) {
        log!("Cancelling {:?}..", self.handle);
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
            handle: Cell::new(None),
            func: None,
        }
    }

    fn safe_start(&'static mut self, func: impl FnMut() -> bool + 'static) {
        unsafe { self.start(func) }
    }

    unsafe fn start<'s, 'f: 's>(&'s mut self, func: impl FnMut() -> bool + 'f) {
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
            // h: *const Cell<Option<i32>>,
            window: Window,
        ) -> Function {
            let val = Closure::once_into_js(move || {
                // See: https://github.com/rust-lang/rust/issues/42574
                let f = f;
                // let h: &'static Cell<_> = unsafe { &*h };

                let val = unsafe { core::ptr::read_volatile(h as *const _) };
                log!("raw read: {:?}", val);

                if h.get().is_none() {
                    log!("you should never see this...");
                    return
                }

                if (f)() {
                    log!("handle2: {:?} @ {:?}", h, h as *const _);
                    let next = recurse(f, h, window.clone());
                    let id = window.request_animation_frame(&next).unwrap();
                    h.set(Some(id));
                    log!("handle2: {:?} @ {:?}", h, h as *const _);
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
        log!("set up: {:?} @ {:?}", wrapper.handle, &wrapper.handle as *const _);
        wrapper.handle.set(Some(window.request_animation_frame(&starting).unwrap()));
        log!("set up: {:?} @ {:?}", wrapper.handle, &wrapper.handle as *const _);
    }
}

#[wasm_bindgen]
// pub fn run() -> Result<AnimationFrameCallbackWrapper, JsValue> {
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(None);

    term.open(terminal_div);

    let mut a = Box::new(AnimationFrameCallbackWrapper::new());
    let mut a = Box::leak(a);

    let mut b = 0;
    log!("ay");
    a.safe_start(move || {
        log!("heyo! {}", b);
        b += 1;

        b != 60
    });

    // log!("runnin!!: {:?} @ {:?}", a.handle, &a.handle as *const _);

    // drop(a);

    // Ok(())


    // Ok(a)
    Ok(())
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
