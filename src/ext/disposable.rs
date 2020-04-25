//! A concrete type and an adapter for [`Disposable`].
//!
//! [`Disposable`]: crate::xterm::Disposable

// use super::{Disposable, wasm_bindgen};

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
