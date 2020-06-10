//! A xterm.js-backed backend for the [tui] crate.
//!
//! [tui]: ::tui

use super::xterm::Terminal;

use std::io::Result as IoResult;

use crossterm as _;
use tui::{backend::Backend, buffer::Cell, layout::Rect, style::Style};

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "tui-backend")))]
#[derive(Debug, Clone)]
pub struct XtermJsBackend {
    term: Terminal,
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "tui-backend")))]
impl XtermJsBackend {
    pub fn new(term: Terminal) -> Self {
        Self { term }
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "tui-backend")))]
impl Backend for XtermJsBackend {
    fn draw<'a, I>(&mut self, content: I) -> IoResult<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        //! This is mostly taken from the `TermionBackend` [`Backend`] impl.

        // We allocate for close to the worst case.
        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut style = Style::default();

        let (mut cx, mut cy) = (None, None);

        for (x, y, cell) in content {
            if Some(y) != cy || Some(x) != cx.map(|x| x + 1) {
                // write!(string, "{}", )?
            }

            cx = Some(x);
            cy = Some(y);
            // (cx, cy) = (Some(x), Some(y));
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> IoResult<()> {
        todo!()
    }

    fn show_cursor(&mut self) -> IoResult<()> {
        todo!()
    }

    fn get_cursor(&mut self) -> IoResult<(u16, u16)> {
        todo!()
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> IoResult<()> {
        todo!()
    }

    fn clear(&mut self) -> IoResult<()> {
        todo!()
    }

    fn size(&self) -> IoResult<Rect> {
        todo!()
    }

    fn flush(&mut self) -> IoResult<()> {
        todo!()
    }
}
