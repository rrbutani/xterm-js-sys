//! A concrete type and helpers for `IEvent`.
//!
//! [`Disposable`]: crate::xterm::Disposable

use super::DisposableWrapper;
use crate::xterm::{KeyEventData, Terminal};

use wasm_bindgen::prelude::*;

// pub struct EventListener<T, U = (), R = NoOpDispose>
// where
//     R: AsRef<Disposable>,
// {
//     listener: Box<dyn FnMut(T, U) -> R + 'static>,
// }

// impl<T, U, R> EventListener<T, U, R>
// where
//     R: AsRef<Disposable>,
// {
//     fn new<F: FnMut(T, U) -> R + 'static>(func: F) -> Self {
//         let listener: Box<dyn FnMut(T, U) -> R + 'static> = Box::new(func);

//         Self { listener }
//     }
// }

#[cfg_attr(docs, doc(cfg(feature = "ext")))]
impl Terminal {
    /// Attaches a binary event listener and returns a [`DisposableWrapper`]
    /// that can be dropped to make xterm.js stop sending the event listener
    /// events.
    ///
    /// This is sugar for [`Terminal::on_binary`].
    ///
    /// We assume event listener closures are going to be long-lived, so we
    /// leak the closure that is produced here!
    ///
    /// [`Terminal::on_binary`]: Terminal::on_binary
    #[allow(trivial_casts)]
    pub fn attach_binary_event_listener<F>(
        &self,
        listener: F,
    ) -> DisposableWrapper
    where
        F: FnMut(String),
        F: 'static,
    {
        let listener =
            Closure::wrap(Box::new(listener) as Box<dyn FnMut(String)>);
        let ret = self.on_binary(&listener).into();

        Closure::forget(listener);
        ret
    }

    /// Attaches a key event listener and returns a [`DisposableWrapper`]
    /// that can be dropped to make xterm.js stop sending the event listener
    /// events.
    ///
    /// This is sugar for [`Terminal::on_key`].
    ///
    /// We assume event listener closures are going to be long-lived, so we
    /// leak the closure that is produced here!
    ///
    /// [`Terminal::on_key`]: Terminal::on_key
    #[allow(trivial_casts)]
    pub fn attach_key_event_listener<F>(&self, listener: F) -> DisposableWrapper
    where
        F: FnMut(KeyEventData),
        F: 'static,
    {
        let listener =
            Closure::wrap(Box::new(listener) as Box<dyn FnMut(KeyEventData)>);
        let ret = self.on_key(&listener).into();

        Closure::forget(listener);
        ret
    }
}
