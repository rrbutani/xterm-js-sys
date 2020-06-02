//! Utilities for [Addons].
//!
//! [Addons]: crate::xterm::TerminalAddon

use super::{object, Disposable, IntoJsInterface, TerminalAddon, Terminal};
use super::disposable::XtermDisposable;

use js_sys::Object;
use wasm_bindgen::{
    prelude::Closure,
    JsCast,
};

/// This is the Rust version of the [`TerminalAddon`] interface.
///
/// See the ["mirroring interfaces" section](../../xterm#mirroring-interfaces)
/// of the `xterm` module docs for more information.
///
/// [`TerminalAddon`]: crate::xterm::TerminalAddon
#[allow(clippy::module_name_repetitions)]
pub trait XtermAddon: XtermDisposable {
    /// This is called when the addon is activated. Dual of
    /// [`TerminalAddon::activate`].
    ///
    /// [`TerminalAddon::activate`]: TerminalAddon::activate
    fn activate(&self, terminal: Terminal);

    //////////////// Internal Functions For Interface Mirroring ////////////////

    /// Copy of [`IntoJsInterface::by_ref`].
    ///
    /// [`IntoJsInterface::by_ref`]: IntoJsInterface::by_ref
    fn into_js_by_ref(&self) -> TerminalAddon
    where
        Self: Clone + 'static,
    {
        XtermAddon::into_js(self.clone())
    }

    /// Copy of [`IntoJsInterface::to`].
    ///
    /// [`IntoJsInterface::to`]: IntoJsInterface::to
    fn into_js(self) -> TerminalAddon
    where
        Self: Sized + 'static,
    {
        let b = Box::leak(Box::new(self));
        XtermAddon::into_js_inner(b).unchecked_into()
    }

    /// Internal version of `into_js_by_ref` that doesn't leak `self`.
    ///
    /// Useful for trait hierarchies.
    fn into_js_inner(&'static self) -> Object
    where
        Self: 'static,
    {
        let act: Box<dyn FnMut(_)> =
            Box::new(move |t| Self::activate(self, t));
        let act = Closure::wrap(act);

        let obj = object! { (<Self as XtermDisposable>::into_js_inner(self)) += {
            activate: act
        }};

        Closure::forget(act);

        obj
    }
}

/// Anything that implements [`XtermAddon`] (and is `Clone + 'static`)
/// implements [`IntoJsInterface<TerminalAddon>`][`IntoJsInterface`].
impl<A> IntoJsInterface<TerminalAddon> for A
where
    A: XtermAddon + Clone + 'static,
{
    /// Converts the [`XtermAddon`] implementor into an instance of
    /// [`TerminalAddon`] (the corresponding JS interface).
    fn to(self) -> TerminalAddon {
        XtermAddon::into_js(self)
    }

    /// Converts the [`XtermAddon`] implementor into an instance of
    /// [`TerminalAddon`] (the corresponding JS interface) _by reference_.
    fn by_ref(&self) -> TerminalAddon {
        XtermAddon::into_js_by_ref(self)
    }
}

/// This provides an impl of the [`XtermAddon`] Rust trait for all things that
/// 'implement' the [`TerminalAddon`] JS interface the `wasm-bindgen` way.
///
/// See the ["mirroring interfaces" section](../../xterm#mirroring-interfaces)
/// of the `xterm` module docs for more information.
impl<A: Clone + 'static> XtermAddon for A
where
    A: AsRef<TerminalAddon>,
    A: AsRef<Disposable>,
{
    /// `activate` for types that implement the [`TerminalAddon`] interface.
    fn activate(&self, terminal: Terminal) {
        TerminalAddon::activate(self.as_ref(), terminal)
    }

    /// `into_js_by_ref` for types that implement the [`TerminalAddon`]
    /// interface.
    ///
    /// This differs from the default impl in that in manages to avoid a `Clone`
    /// before effectively doing what `into_js` does.
    fn into_js_by_ref(&self) -> TerminalAddon {
        AsRef::<TerminalAddon>::as_ref(self).clone()
    }

    /// `into_js` for types that implement the [`TerminalAddon`] interface.
    ///
    /// This differs from the default impl in that in manages to avoid "double
    /// wrapping" the methods in the interface (types that impl
    /// [`TerminalAddon`] the `wasm-bindgen` way already have an wrapped up
    /// [`Object`] they can hand us).
    fn into_js(self) -> TerminalAddon {
        AsRef::<TerminalAddon>::as_ref(&self).clone()
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
impl Terminal {
    /// Loads an [addon] into this instance of the xterm.js [`Terminal`].
    ///
    /// This supports [Rust defined addons](XtermAddon) as well as
    /// [JS defined addons](TerminalAddon) and is otherwise identical to
    /// [`Terminal::load_addon`].
    ///
    /// [addon]: TerminalAddon
    pub fn load_xterm_addon<A: IntoJsInterface<TerminalAddon>>(
        &self,
        addon: &A,
    ) {
        self.load_addon(addon.by_ref())
    }
}
