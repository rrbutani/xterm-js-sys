//! Sugar for methods that take `IEvent`s.

use super::{calculated_doc, Disposable, DisposableWrapper};
use crate::xterm::{
    Buffer, BufferNamespace, KeyEventData, RenderEventData, ResizeEventData,
    Str, Terminal,
};

use wasm_bindgen::prelude::*;

macro_rules! event_methods {
    ($(
        $(#[$metas:meta])*
        $(@doc: $kind:literal)? $vis:vis $nom:ident: ($($args:ty),*) => $js_func:path
    )*) => {$(
        event_method! {
            $(#[$metas])*
            $(@doc: $kind)? $vis $nom: ($($args),*) => $js_func
        }
    )*};
}

macro_rules! event_method {
    (
        $(#[$metas:meta])*
        $(@doc: $kind:literal)? $vis:vis $nom:ident: ($($args:ty),*) => $js_func:path
    ) => {
        calculated_doc! {
            $(#[doc = $crate::ext::_m_sprt::concat!(
                " Attaches a ",
                    $kind,
                " event listener and returns a [`DisposableWrapper`]\n",
                " that can be dropped to make xterm.js stop sending the event",
                " listener\n events.\n",
            )])?
            #[doc = $crate::ext::_m_sprt::concat!(
                "\n",
                " This is sugar for ",
                "[`",
                    $crate::ext::_m_sprt::stringify!($js_func),
                "`].",
                "\n\n",
                " We assume event listener closures are going to be long-lived,",
                " so we leak\n",
                " the closure that is produced here!\n",
                " \n",
                "  [`",
                    $crate::ext::_m_sprt::stringify!($js_func),
                "`]: ",
                    $crate::ext::_m_sprt::stringify!($js_func),
            )]
            >>>
            $vis fn $nom<F>(
                &self,
                listener: F,
            ) -> DisposableWrapper<Disposable>
            where
                F: FnMut($($args),*),
                F: 'static,
            {
                let listener: Box<dyn FnMut($($args),*)> = Box::new(listener);
                let listener = Closure::wrap(listener);

                let ret = $js_func(self, &listener).into();

                Closure::forget(listener);
                ret
            }
            $(#[$metas])*
        }
    };
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
impl Terminal {
    /// Attaches a binary event listener and returns a [`DisposableWrapper`]
    /// that can be dropped to make xterm.js stop sending the event listener
    /// events.
    ///
    /// This is sugar for [`Terminal::on_binary`].
    ///
    /// We assume event listener closures are going to be long-lived, so we leak
    /// the closure that is produced here!
    ///
    /// [`Terminal::on_binary`]: Terminal::on_binary
    pub fn attach_binary_event_listener<F>(
        &self,
        listener: F,
    ) -> DisposableWrapper<Disposable>
    where
        F: FnMut(String),
        F: 'static,
    {
        let listener: Box<dyn FnMut(String)> = Box::new(listener);
        let listener = Closure::wrap(listener);

        let ret = self.on_binary(&listener).into();

        Closure::forget(listener);
        ret
    }

    /// Attaches a cursor move event listener and returns a
    /// [`DisposableWrapper`] that can be dropped to make xterm.js stop sending
    /// the event listener events.
    ///
    /// This is sugar for [`Terminal::on_cursor_move`].
    ///
    /// [`Terminal::on_cursor_move`]: Terminal::on_cursor_move
    pub fn attach_cursor_move_event_listener<F>(
        &self,
        listener: F,
    ) -> DisposableWrapper<Disposable>
    where
        F: FnMut(),
        F: 'static,
    {
        let listener: Box<dyn FnMut()> = Box::new(listener);
        let listener = Closure::wrap(listener);

        let ret = self.on_cursor_move(&listener).into();

        Closure::forget(listener);
        ret
    }

    /// Attaches a key event listener and returns a [`DisposableWrapper`]
    /// that can be dropped to make xterm.js stop sending the event listener
    /// events.
    ///
    /// This is sugar for [`Terminal::on_key`].
    ///
    /// We assume event listener closures are going to be long-lived, so we leak
    /// the closure that is produced here!
    ///
    /// [`Terminal::on_key`]: Terminal::on_key
    pub fn attach_key_event_listener<F>(
        &self,
        listener: F,
    ) -> DisposableWrapper<Disposable>
    where
        F: FnMut(KeyEventData),
        F: 'static,
    {
        let listener: Box<dyn FnMut(KeyEventData)> = Box::new(listener);
        let listener = Closure::wrap(listener);

        let ret = self.on_key(&listener).into();

        Closure::forget(listener);
        ret
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
impl BufferNamespace {
    /// Attaches an event listener for when the active buffer changes and
    /// returns a [`DisposableWrapper`] that can be dropped to make xterm.js
    /// stop sending the event listener events.
    ///
    /// This is sugar for [`BufferNamespace::on_buffer_change`].
    ///
    /// We assume event listener closures are going to be long-lived, so we leak
    /// the closure that is produced here!
    ///
    /// [`BufferNamespace::on_buffer_change`]: BufferNamespace::on_buffer_change
    pub fn attach_buffer_change_event_listener<F>(
        &self,
        listener: F,
    ) -> DisposableWrapper<Disposable>
    where
        F: FnMut(Buffer),
        F: 'static,
    {
        let listener: Box<dyn FnMut(Buffer)> = Box::new(listener);
        let listener = Closure::wrap(listener);

        let ret = self.on_buffer_change(&listener).into();

        Closure::forget(listener);
        ret
    }
}
