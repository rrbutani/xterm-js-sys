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
impl Terminal { event_methods!{
    @doc: "binary"
    pub attach_binary_event_listener: (Str) => Terminal::on_binary

    @doc: "cursor move"
    pub attach_cursor_move_event_listener: () => Terminal::on_cursor_move

    @doc: "data"
    pub attach_data_event_listener: (Str) => Terminal::on_data

    @doc: "key event"
    pub attach_key_event_listener: (KeyEventData) => Terminal::on_key

    @doc: "line feed"
    pub attach_line_feed_event_listener: () => Terminal::on_line_feed

    @doc: "render"
    pub attach_render_event_listener: (RenderEventData) => Terminal::on_render

    @doc: "resize"
    pub attach_resize_event_listener: (ResizeEventData) => Terminal::on_resize

    @doc: "scroll"
    pub attach_scroll_event_listener: (u32) => Terminal::on_scroll

    @doc: "selection change"
    pub attach_selection_change_event_listener: ()
        => Terminal::on_selection_change

    @doc: "title change"
    pub attach_title_change_event_listener: (Str) => Terminal::on_title_change
}}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
impl BufferNamespace { event_methods! {
    /// Attaches an event listener for when the active buffer changes and
    /// returns a [`DisposableWrapper`] that can be dropped to make xterm.js
    /// stop sending the event listener events.
    pub attach_buffer_change_event_listener: (Buffer)
        => BufferNamespace::on_buffer_change
}}
