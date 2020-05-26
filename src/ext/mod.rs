//! Glue for the Xterm.js types.

use super::xterm::{Disposable, Terminal, TerminalOptions};

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsCast;

#[doc(hidden)]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub mod _obj_macro_support {
    pub use core::stringify;
    pub use js_sys::{Object, Reflect};
    pub use wasm_bindgen::JsValue;
}

/// Defines a JS object with some properties.
#[macro_export]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
macro_rules! object {
    (
        $($f:ident: $v:expr),* $(,)?
    ) => {{
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

/// Represents a Rust type that satisfies a JS interface and can be turned into
/// the concrete type that represents the JS interface.
///
/// See the [`disposable`] module for an example.
///
/// As mentioned in the [`xterm` module docs](crate::xterm#mirroring-interfaces)
/// we make a Rust trait dual for each JS interface (or the ones we want to make
/// instances of, anyways). Ideally we'd be able to do something like this:
/// `trait RustTrait: IntoJsInterface<JsTrait>`. The problem with that is that
/// the impl of `IntoJsInterface` _requires_ the impl of `RustTrait`; we need
/// the functions that satisfy the interface to actually make the instance of
/// the interface type.
///
/// So, instead we do the weird backwards thing that we do in [`disposable`]
/// where the Rust trait (i.e. [`Disposable`](disposable::Disposable)) ends up
/// effectively having these same functions and _then_ providing a blanket impl
/// so that [`IntoJsInterface`] is impled for all things that impl the Rust
/// trait.
///
/// It's unfortunate that we don't really have a way to encode that each Rust
/// trait can have one (and only one) interface type dual. We encode this within
/// the trait itself, but we can't seem to do this in a way that's generic over
/// traits (not until we get HKTs anyways).
///
/// So it's unclear exactly where this trait would be useful. I guess it lets
/// you be generic over the interface you're asking for? Something like this:
/// ```rust
/// # use wasm_bindgen::{convert::{FromWasmAbi, IntoWasmAbi}, JsCast};
/// # use xterm_js_sys::ext::IntoJsInterface;
/// # #[allow(dead_code)]
/// pub fn foo<I>(inst: impl IntoJsInterface<I>)
/// where
///     I: FromWasmAbi + IntoWasmAbi + JsCast,
/// {
///    inst.to();
/// }
/// ```
///
/// Combined with `AsRef` you can do things like accept Rust implementations
/// of interfaces that subclass some base class:
/// ```rust
/// # use wasm_bindgen::{convert::{FromWasmAbi, IntoWasmAbi}, JsCast};
/// # use xterm_js_sys::ext::IntoJsInterface;
/// # #[allow(dead_code)]
/// pub fn bar<I>(inst: impl IntoJsInterface<I>)
/// where
///     I: FromWasmAbi + IntoWasmAbi + JsCast,
///     I: AsRef<js_sys::Iterator>,
/// {
///    inst.to();
/// }
/// ```
///
/// But it's still unclear if/how this is useful.
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub trait IntoJsInterface<Interface: FromWasmAbi + IntoWasmAbi + JsCast> {
    /// Convert to an instance of the JS interface type.
    fn to(self) -> Interface;

    /// Produce an instance of the JS interface type without consuming the Rust
    /// instance.
    ///
    /// For Rust impls of a trait this will probably require `Self` to implement
    /// `Clone` since as part of creating the instance the instance needs to be
    /// leaked (for methods to still work), but we'll leave that up to
    /// implementors.
    fn to_by_ref(&self) -> Interface;
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
#[doc(inline)]
pub use super::object;

pub mod disposable;
pub use disposable::*;

pub mod event;
pub use event::*;
