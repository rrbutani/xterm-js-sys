//! Some [`Theme`]s.
//!
//! [`Theme`]: crate::xterm::Theme

use crate::xterm::Theme;

macro_rules! c {
    ($hex:literal) => {{
        // Ideally we'd be able to use const functions for all this, but alas;
        // things are not there yet.
        let _type_assert: u32 = $hex;

        format!("#{:6X}", $hex)
    }};
}

impl Theme {
    /// An xterm.js [`Theme`] based on the [Nord color palette][nord].
    ///
    /// This is loosely based on the [iTerm 2 Nord theme][iterm2].
    ///
    /// [nord]: https://www.nordtheme.com/
    /// [iterm2]: https://github.com/arcticicestudio/nord-iterm2
    #[allow(clippy::unreadable_literal)]
    #[rustfmt::skip]
    #[must_use]
    pub fn nord() -> Self {
        Self::default()
            // ? (0m)
            // ? (1m)

            .with_black(c!(0x343434))
            .with_bright_black(c!(0x434c5e))
            .with_red(c!(0xbf616a))
            .with_bright_red(c!(0xbf616a))
            .with_green(c!(0xa3be8c))
            .with_bright_green(c!(0xa3be8c))
            .with_yellow(c!(0xebcb8b))
            .with_bright_yellow(c!(0xebcb8b))
            .with_blue(c!(0x81a1c1))
            .with_bright_blue(c!(0x81a1c1))
            .with_magenta(c!(0xb48ead))
            .with_bright_magenta(c!(0xb48ead))
            .with_cyan(c!(0x88c0d0))
            .with_bright_cyan(c!(0x8fbcbb))
            .with_white(c!(0xe5e9f0))
            .with_bright_white(c!(0xeceff4))

            .with_background(c!(0x2e3440))
            .with_foreground(c!(0xd8dee9))

            // .with_cursor()
            // .with_cursor_accent()
    }
}
