//! Utilities for [`Disposable`].
//!
//! [`Disposable`]: crate::xterm::Disposable

use super::{object, Disposable, Terminal, TerminalOptions};

use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsCast, JsValue};
use js_sys::{Function, Object};

use core::ops::{Deref, DerefMut};

/// This is the Rust version of the [`Disposable`](super::Disposable) interface.
///
/// See the ["mirroring interfaces" section](../../xterm#mirroring-interfaces)
/// of the `xterm` module docs for more information.
#[cfg_attr(docs, doc(cfg(feature = "ext")))]
pub trait XtermDisposable {
    fn dispose(&self);

    // fn into_js(&self) -> Disposable where Self: Clone + 'static {
    //     let b = Box::leak(Box::new(self.clone()));
    //     b.into_js_inner().unchecked_into()
    // }

    fn into_js(self) -> Disposable where Self: 'static {
        let b = Box::leak(Box::new(self);
        b.into_js_inner().unchecked_into()
    }

    #[doc(hidden)]
    fn into_js_inner(&'static self) -> Object where Self: 'static {
        let disp: Box<dyn FnMut(JsValue)> = Box::new(|_s| Self::dispose(self));
        let disp = Closure::wrap(disp);
        Closure::forget(disp);

        object!({
            dispose: disp
        })
    }
}

/// In the `wasm-bindgen` world, things that impl an interface or extend a class
/// `Deref` into it (technically, I think they only `Deref` into their immediate
/// parent and then impl `AsRef` for all the other things they implement).
///
/// We've chosen to represent the `Disposable` interface with a corresponding
/// Rust trait and this blanket impl implements the trait for all things that
/// 'implement' the interface the `wasm-bindgen` way.
///
/// See the ["mirroring interfaces" section](../../xterm#mirroring-interfaces)
/// of the `xterm` module docs for more information.
impl<D: AsRef<Disposable> + Clone + 'static> XtermDisposable for D {
    fn dispose(&self) {
        Disposable::dispose(self.as_ref())
    }

    fn into_js(&self) -> Disposable {
        self.as_ref().clone()
    }
}

/// A wrapper for [`Disposable`] that calls `dispose` on `Drop`.
#[derive(Debug, Clone)]
#[cfg_attr(docs, doc(cfg(feature = "ext")))]
#[allow(clippy::module_name_repetitions)]
pub struct DisposableWrapper {
    /// The actual [`Disposable`] instance that's being wrapped.
    inner: Disposable,
}

impl From<Disposable> for DisposableWrapper {
    fn from(inner: Disposable) -> Self {
        Self { inner }
    }
}

impl Drop for DisposableWrapper {
    fn drop(&mut self) {
        self.inner.dispose()
    }
}

/// A type that satisfies the [`Disposable`] interface and does nothing on
/// `dispose`.
///
/// Can be used wherever an `IDisposable` is required.
///
/// [`Disposable`]: crate::xterm::Disposable
#[wasm_bindgen]
#[derive(Debug, Clone)]
#[cfg_attr(docs, doc(cfg(feature = "ext")))]
pub struct NoOpDispose {
    /// JavaScript object that just has a no-op `dispose` function.
    obj: Object,
}

impl Default for NoOpDispose {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(docs, doc(cfg(feature = "ext")))]
impl NoOpDispose {
    /// Constructs a new [`NoOpDispose`].
    #[must_use]
    pub fn new() -> Self {
        let obj = Object::new();

        let obj = Object::define_property(
            &obj,
            &JsValue::from_str("dispose"),
            Function::new_no_args("return;").as_ref(),
        );

        Self { obj }
    }
}

impl AsRef<Disposable> for NoOpDispose {
    fn as_ref(&self) -> &Disposable {
        JsCast::unchecked_ref(&self.obj)
    }
}
