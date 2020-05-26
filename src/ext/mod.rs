//! Glue for the Xterm.js types.

use super::xterm::{Disposable, Terminal, TerminalOptions};

/// Defines a JS object with some properties.
#[macro_export]
macro_rules! object {
    ({
        $($f:ident: $v:expr),* $(,)?
    }) => {{

        let obj = js_sys::Object::new();

        object! {
            obj += {
                $($f: $v),*
            }
        }

        obj
    }};

    ($nom:ident += {
        $($f:ident: $v:expr),* $(,)?
    }) => {$(
        let $nom = js_sys::Object::define_property(
            &$nom,
            &wasm_bindgen::JsValue::from_str(core::stringify!($f)),
            ($v).as_ref(),
        );
    )*};
}

use super::object;

pub mod disposable;
pub use disposable::*;

pub mod event;
pub use event::*;
