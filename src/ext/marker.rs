//! Extra functions for [Marker]s.
//!
//! [Marker]: crate::xterm::Marker

use crate::idx_to_opt;
use crate::xterm::Marker;

impl Marker {
    /// The actual line index in the buffer at this point in time.
    ///
    /// Like [`line`], but returns an `Option` instead of `-1`.
    ///
    /// [`line`]: Marker::line
    #[must_use]
    pub fn get_line(&self) -> Option<u32> {
        idx_to_opt(self.line())
    }
}
