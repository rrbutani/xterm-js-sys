//! Utilities for [`Disposable`].
//!
//! [`Disposable`]: crate::xterm::Disposable

use super::{wasm_bindgen, Disposable};
use wasm_bindgen::{JsCast, JsValue};

use js_sys::{Function, Object};

/// A wrapper for [`Disposable`] that calls `dispose` on `Drop`.
///
/// [`Disposable`]: crate::xterm::Disposable
#[derive(Debug, Clone)]
#[cfg_attr(docs, doc(cfg(feature = "ext")))]
pub struct DisposableWrapper {
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
