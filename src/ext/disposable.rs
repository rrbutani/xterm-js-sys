//! Utilities for [`Disposable`].
//!
//! [`Disposable`]: crate::xterm::Disposable

use super::{object, Disposable, Terminal, TerminalOptions, IntoJsInterface};

use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsCast, JsValue};
use js_sys::{Function, Object};

use core::ops::{Deref, DerefMut};

/// This is the Rust version of the [`Disposable`](super::Disposable) interface.
///
/// See the ["mirroring interfaces" section](../../xterm#mirroring-interfaces)
/// of the `xterm` module docs for more information.
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub trait XtermDisposable {
    /// Disposes of the instance. Dual of [`Disposable::dispose`].
    ///
    /// This can involve unregistering an event listener or cleaning up
    /// resources or anything else that should happen when an instance is
    /// disposed of.
    fn dispose(&self);

    //////////////// Internal Functions For Interface Mirroring ////////////////

    /// Copy of [`IntoJsInterface::to_by_ref`].
    ///
    /// [`IntoJsInterface::to_by_ref`]: IntoJsInterface::to_by_ref
    fn into_js_by_ref(&self) -> Disposable where Self: Clone + 'static {
        self.clone().into_js()
    }

    /// Copy of [`IntoJsInterface::to`].
    ///
    /// [`IntoJsInterface::to`]: IntoJsInterface::to_by_ref
    fn into_js(self) -> Disposable where Self: Sized + 'static {
        let b = Box::leak(Box::new(self));
        b.into_js_inner().unchecked_into()
    }

    /// Internal version of `into_js_by_ref` that doesn't leak `self`.
    ///
    /// Useful for trait hierarchies.
    fn into_js_inner(&'static self) -> Object where Self: 'static {
        let disp: Box<dyn FnMut(JsValue)> = Box::new(move |_s| Self::dispose(self));
        let disp = Closure::wrap(disp);

        let obj = object!({
            dispose: disp
        });

        Closure::forget(disp);

        obj
    }
}

// Anything that implements `XtermDisposable` (and is `Clone + 'static`)
// implements `IntoJsInterface<Disposable>`.
impl<D> IntoJsInterface<Disposable> for D
where
    D: XtermDisposable + Clone + 'static
{
    fn to(self) -> Disposable { self.into_js() }
    fn to_by_ref(&self) -> Disposable { self.into_js_by_ref() }
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

    fn into_js(self) -> Disposable {
        self.as_ref().clone()
    }
}

/// A wrapper for [`Disposable`] that calls `dispose` on `Drop`.
#[derive(Debug, Clone)]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
#[allow(clippy::module_name_repetitions)]
pub struct DisposableWrapper<D: XtermDisposable> {
    /// The actual [`Disposable`] instance that's being wrapped.
    inner: D,
}

impl<D: XtermDisposable> From<D> for DisposableWrapper<D> {
    fn from(inner: D) -> Self {
        Self { inner }
    }
}

impl<D: XtermDisposable> Deref for DisposableWrapper<D> {
    type Target = D;

    fn deref(&self) -> &D { &self.inner }
}

impl<D: XtermDisposable> DerefMut for DisposableWrapper<D> {
    fn deref_mut(&mut self) -> &mut D { &mut self.inner }
}

impl<D: XtermDisposable> Drop for DisposableWrapper<D> {
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
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub struct NoOpDispose {
    /// JavaScript object that just has a no-op `dispose` function.
    obj: Object,
}

impl Default for NoOpDispose {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
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

// This makes it so that we get an `XtermDisposable` and `IntoJsInterface` impl.
impl AsRef<Disposable> for NoOpDispose {
    fn as_ref(&self) -> &Disposable {
        JsCast::unchecked_ref(&self.obj)
    }
}

impl Deref for NoOpDispose {
    type Target = Disposable;

    fn deref(&self) -> &Disposable {
        self.as_ref()
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
impl Terminal {
    /// [`Terminal`] constructor that encloses the resulting [`Terminal`] in a
    /// [`DisposableWrapper`].
    ///
    /// This is otherwise identical to [`Terminal::new`].
    pub fn new_with_wrapper(
        options: Option<TerminalOptions>
    ) -> DisposableWrapper<Terminal> {
        Self::new(options).into()
    }
}
