//! A concrete type and an adapter for [`Disposable`].
//!
//! [`Disposable`]: crate::xterm::Disposable

use super::{Disposable, wasm_bindgen};
use wasm_bindgen::{JsValue, JsCast};

use js_sys::{Object, Function};

// #[wasm_bindgen]
// pub struct DisposableInstance {
//     inner: Box<dyn Drop>,
// }

// #[wasm_bindgen]
// impl DisposableInstance {
//     pub fn dispose(&mut self) {
//         self.inner.drop()
//     }
// }

// // Unfortunately we can't do this:
// impl<D: AsMut<Disposable>> Drop for D {
//     fn drop(&mut self) {
//         self.as_mut().dispose()
//     }
// }

// impl Drop for Disposable {
//     fn drop(&mut self) {
//         self.dispose();
//     }
// }

#[wasm_bindgen]
pub struct NoOpDispose {
    obj: Object,
}

impl NoOpDispose {
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
