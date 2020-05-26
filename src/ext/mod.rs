//! Glue for the Xterm.js types.

use super::xterm::{Disposable, Terminal, TerminalOptions};

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsCast;

#[doc(hidden)]
pub mod _obj_macro_support {
    pub use js_sys::{Object, Reflect};
    pub use wasm_bindgen::JsValue;
    pub use core::stringify;
}

/// Defines a JS object with some properties.
#[macro_export]
macro_rules! object {
    ({
        $($f:ident: $v:expr),* $(,)?
    }) => {{
        let obj = $crate::ext::_obj_macro_support::Object::new();

        $crate::ext::object! { obj += {
                $($f: $v),*
        }}

        obj
    }};

    ($nom:ident += {
        $($f:ident: $v:expr),* $(,)?
    }) => {$(
        let _ = $crate::ext::_obj_macro_support::Reflect::set(
            &$nom,
            &$crate::ext::_obj_macro_support::JsValue::from_str(
                $crate::ext::_obj_macro_support::stringify!($f)
            ),
            ($v).as_ref(),
        ).unwrap();
    )*};
}

// pub trait IntoJsInterface {
//     type Interface: FromWasmAbi + IntoWasmAbi + JsCast;

//     fn into(self) -> Self::Interface;
//     fn into_by_ref(&self) -> Self::Interface;
// }

pub trait IntoJsInterface<Interface: FromWasmAbi + IntoWasmAbi + JsCast> {
    fn to(self) -> Interface;
    fn to_by_ref(&self) -> Interface;
}

use super::object;

pub mod disposable;
pub use disposable::*;

pub mod event;
pub use event::*;
