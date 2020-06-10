//! Helpers for [`TerminalOptions`], [`Theme`], and [`WindowOptions`].
//!
//! [`TerminalOptions`]: crate::xterm::TerminalOptions
//! [`Theme`]: crate::xterm::Theme
//! [`WindowOptions`]: crate::xterm::WindowOptions

use super::calculated_doc;
use crate::xterm::{
    BellStyle, CursorStyle, FastScrollModifier, FontWeight, LogLevel,
    RendererType, Str, TerminalOptions, Theme, WindowOptions,
};

// TODO: if we give in and use paste this can become a lot cleaner (we can just
// fold this into the `wasm_struct!` invocation).

/// Generates setters and a constructor for a struct made of `Option`al fields.
macro_rules! opt_struct {
    ($nom:path {
        $(
            $(use($pub_getter:ident, $pub_setter:ident) as $setter_name:ident)?
            $(as $setter_name_new:ident)?
                => $field:ident: $ty:ty
        ),*
        $(,)?
    }) => {
        impl $nom {
            calculated_doc! {
                #[doc = core::concat!(
                    "Constructor for [`",
                        core::stringify!($nom),
                    "`].",
                )]
                >>>
                pub const fn new() -> Self {
                    Self {$(
                        $field: None,
                    )*}
                }
                #[allow(deprecated)]
                #[must_use]
            }

            $(
                $(calculated_doc! {
                    #[doc = core::concat!(
                        "Builder pattern setter for [`",
                            core::stringify!($nom),
                            "::",
                            core::stringify!($field),
                        "`].",
                    )]
                    >>>
                    pub const fn $setter_name_new(mut self, $field: $ty) -> Self {
                        self.$field = Some($field);
                        self
                    }
                    #[allow(deprecated)]
                    #[must_use]
                })?

                $(calculated_doc! {
                    #[doc = core::concat!(
                        "Builder pattern setter for [`",
                            core::stringify!($nom),
                            "::",
                            core::stringify!($field),
                        "`] (accessible through ",
                        "[`",
                            core::stringify!($nom),
                            "::",
                            core::stringify!($pub_getter),
                        "`] and [`",
                            core::stringify!($nom),
                            "::",
                            core::stringify!($pub_setter),
                        "`].",
                    )]
                    >>>
                    // This can't be const because some of these non-Copy fields
                    // that have setters and getters implement `Drop` (i.e.
                    // `String`) which means they can't be used in const setter
                    // functions since we can't drop things with destructors in
                    // const functions.
                    //
                    pub /*const*/ fn $setter_name(mut self, $field: $ty) -> Self {
                        self.$field = Some($field);
                        self
                    }
                    #[allow(deprecated)]
                    #[must_use]
                })?
            )*
        }
    };
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
opt_struct! {
    TerminalOptions {
        as with_allow_proposed_api
            => allow_proposed_api: bool,

        as with_allow_transparency
            => allow_transparency: bool,

        use(bell_sound, set_bell_sound) as with_bell_sound
            => bell_sound: Str,

        as with_bell_style
            => bell_style: BellStyle,

        as with_cols
            => cols: u16,

        as with_convert_eol
            => convert_eol: bool,

        as with_cursor_blink
            => cursor_blink: bool,

        as with_cursor_style
            => cursor_style: CursorStyle,

        as with_cursor_width
            => cursor_width: f32,

        as with_disable_stdin
            => disable_stdin: bool,

        as with_draw_bold_text_in_bright_colors
            => draw_bold_text_in_bright_colors: bool,

        as with_fast_scroll_modifier
            => fast_scroll_modifier: FastScrollModifier,

        as with_fast_scroll_sensitivity
            => fast_scroll_sensitivity: f32,

        use(font_family, set_font_family) as with_font_family
            => font_family: Str,

        as with_font_size
            => font_size: f32,

        as with_font_weight
            => font_weight: f32,

        as with_font_weight_bold
            => font_weight_bold: FontWeight,

        as with_letter_spacing
            => letter_spacing: u16,

        as with_line_height
            => line_height: u16,

        as with_link_tooltip_hover_duration
            => link_tooltip_hover_duration: u16,

        as with_log_level
            => log_level: LogLevel,

        as with_mac_option_click_forces_selection
            => mac_option_click_forces_selection: bool,

        as with_mac_option_is_meta
            => mac_option_is_meta: bool,

        as with_minimum_contrast_ratio
            => minimum_contrast_ratio: f32,

        as with_renderer_type
            => renderer_type: RendererType,

        as with_right_click_selects_word
            => right_click_selects_word: bool,

        as with_rows
            => rows: u16,

        as with_screen_reader_mode
            => screen_reader_mode: bool,

        as with_scroll_sensitivity
            => scroll_sensitivity: f32,

        as with_scrollback
            => scrollback: u32,

        as with_tab_stop_width
            => tab_stop_width: u16,

        use(theme, set_theme) as with_theme
            => theme: Theme,

        use(window_options, set_window_options) as with_window_options
            => window_options: WindowOptions,

        as with_windows_mode
            => windows_mode: bool,

        use(word_separator, set_word_separator) as with_word_separator
            => word_separator: Str,
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
opt_struct! {
    Theme {
        use (background, set_background) as with_background
            => background: Str,

        use (black, set_black) as with_black
            => black: Str,

        use (blue, set_blue) as with_blue
            => blue: Str,

        use (bright_black, set_bright_black) as with_bright_black
            => bright_black: Str,

        use (bright_blue, set_bright_blue) as with_bright_blue
            => bright_blue: Str,

        use (bright_cyan, set_bright_cyan) as with_bright_cyan
            => bright_cyan: Str,

        use (bright_green, set_bright_green) as with_bright_green
            => bright_green: Str,

        use (bright_magenta, set_bright_magenta) as with_bright_magenta
            => bright_magenta: Str,

        use (bright_red, set_bright_red) as with_bright_red
            => bright_red: Str,

        use (bright_white, set_bright_white) as with_bright_white
            => bright_white: Str,

        use (bright_yellow, set_bright_yellow) as with_bright_yellow
            => bright_yellow: Str,

        use (cursor, set_cursor) as with_cursor
            => cursor: Str,

        use (cursor_accent, set_cursor_accent) as with_cursor_accent
            => cursor_accent: Str,

        use (cyan, set_cyan) as with_cyan
            => cyan: Str,

        use (foreground, set_foreground) as with_foreground
            => foreground: Str,

        use (green, set_green) as with_green
            => green: Str,

        use (magenta, set_magenta) as with_magenta
            => magenta: Str,

        use (red, set_red) as with_red
            => red: Str,

        use (selection, set_selection) as with_selection
            => selection: Str,

        use (white, set_white) as with_white
            => white: Str,

        use (yellow, set_yellow) as with_yellow
            => yellow: Str,
    }
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
opt_struct! {
    WindowOptions {
        as with_fullscreen_win
            => fullscreen_win: bool,

        as with_get_cell_size_pixels
            => get_cell_size_pixels: bool,

        as with_get_icon_title
            => get_icon_title: bool,

        as with_get_screen_size_chars
            => get_screen_size_chars: bool,

        as with_get_screen_size_pixels
            => get_screen_size_pixels: bool,

        as with_get_win_position
            => get_win_position: bool,

        as with_get_win_size_chars
            => get_win_size_chars: bool,

        as with_get_win_size_pixels
            => get_win_size_pixels: bool,

        as with_get_win_state
            => get_win_state: bool,

        as with_get_win_title
            => get_win_title: bool,

        as with_lower_win
            => lower_win: bool,

        as with_maximize_win
            => maximize_win: bool,

        as with_minimize_win
            => minimize_win: bool,

        as with_pop_title
            => pop_title: bool,

        as with_push_title
            => push_title: bool,

        as with_raise_win
            => raise_win: bool,

        as with_refresh_win
            => refresh_win: bool,

        as with_restore_win
            => restore_win: bool,

        as with_set_win_lines
            => set_win_lines: bool,

        as with_set_win_position
            => set_win_position: bool,

        as with_set_win_size_chars
            => set_win_size_chars: bool,

        as with_set_win_size_pixels
            => set_win_size_pixels: bool,
    }
}
