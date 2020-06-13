//! Utilities for [Addons].
//!
//! [Addons]: crate::xterm::TerminalAddon

use super::disposable::XtermDisposable;
use super::{interface, Disposable, IntoJsInterface, Terminal, TerminalAddon};

interface! {
    #[allow(clippy::module_name_repetitions)]
    pub trait XtermAddon mirrors TerminalAddon
    where
        Self extends Disposable as XtermDisposable,
    {
        /// This is called when the addon is activated.
        fn activate(&self, terminal: Terminal);
    }
}

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
