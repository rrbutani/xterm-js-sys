//! A xterm.js-backed backend for [crossterm].
//!
//! [crossterm]: ::crossterm

use super::xterm::Terminal;

use std::io::{Error as IoError, ErrorKind, Result as IoResult, Write};
use std::mem::replace;

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "crossterm-support")))]
#[derive(Debug, Clone)]
/// TODO!
pub struct XtermJsCrosstermBackend<'a> {
    /// The xterm.js terminal that this struct instance wraps.
    pub terminal: &'a Terminal,
    buffer: Vec<u8>,
}

impl<'a> Drop for XtermJsCrosstermBackend<'a> {
    fn drop(&mut self) {
        self.flush().unwrap()
    }
}

impl<'a> XtermJsCrosstermBackend<'a> {
    /// TODO!
    pub fn new(terminal: &'a Terminal) -> Self {
        Self::new_with_capacity(terminal, 0)
    }

    /// TODO!
    pub fn new_with_capacity(terminal: &'a Terminal, capacity: usize) -> Self {
        Self {
            terminal,
            buffer: Vec::with_capacity(capacity),
        }
    }
}

impl<'a> Write for XtermJsCrosstermBackend<'a> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.buffer.flush()?;

        let s = String::from_utf8(replace(&mut self.buffer, Vec::new()))
            .map_err(|e| IoError::new(ErrorKind::Other, e))?;
        self.terminal.write(s);

        Ok(())
    }
}

impl<'a> From<&'a Terminal> for XtermJsCrosstermBackend<'a> {
    fn from(terminal: &'a Terminal) -> Self {
        Self::new(terminal)
    }
}
