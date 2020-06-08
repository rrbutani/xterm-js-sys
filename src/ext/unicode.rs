//! Helpers and duals for [`UnicodeVersionProvider`] and [`UnicodeHandling`].
//!
//! [`UnicodeHandling`]: crate::xterm::UnicodeHandling
//! [`UnicodeVersionProvider`]: crate::xterm::UnicodeVersionProvider

use crate::xterm::{
    Str, UnicodeHandling, UnicodeVersionProvider, WideCharacterWidth
};
use super::{interface, IntoJsInterface};

interface! {
    #[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
    pub trait XtermUnicodeVersionProvider mirrors UnicodeVersionProvider {
        /// Gets a string indicating the Unicode version provided.
        fn version(&self) -> Str;

        /// Unicode version dependent `wcwidth` implementation.
        fn wcwidth(&self, codepoint: u32) -> WideCharacterWidth;
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
impl UnicodeHandling {
    /// Registers a [custom Unicode version provider].
    ///
    /// This supports [Rust defined providers](XtermUnicodeVersionProvider) as
    /// well as [JS defined providers](UnicodeVersionProvider) as is otherwise
    /// identical to [`UnicodeHandling::register`].
    ///
    /// [custom Unicode version provider]: XtermUnicodeVersionProvider
    pub fn register_version_provider<P>(
        &self,
        provider: &P,
    ) where
        P: IntoJsInterface<UnicodeVersionProvider>,
    {
        self.register(provider.by_ref())
    }
}
