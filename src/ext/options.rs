//! Helpers for [`TerminalOptions`], [`Theme`], and [`WindowOptions`].
//!
//! [`TerminalOptions`]: crate::xterm::TerminalOptions
//! [`Theme`]: crate::xterm::Theme
//! [`WindowOptions`]: crate::xterm::WindowOptions

use crate::xterm::{
    BellStyle, CursorStyle, FastScrollModifier, FontWeight, LogLevel,
    RendererType, Str, TerminalOptions, Theme, WindowOptions,
};
use super::calculated_doc;

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
                    // Unforunately this can't be const because of wasm_bindgen;
                    // this can be fixed by making wasm_struct use `paste` to
                    // make crate private const setters that this macro can then
                    // use.
                    pub /*const*/ fn $setter_name(mut self, $field: $ty) -> Self {
                        self.$pub_setter(Some($field));
                        self
                    }
                    #[allow(deprecated)]
                })?
            )*
        }
    };
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
opt_struct!{
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
            => scrollback: u64,

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

// #[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]

