//! Bindings for the xterm.js public API.
//!
//! Unfortunately we can't (yet) generate the below from the TypeScript type
//! definitions for xterm.js, so we do it by hand.
//!
//! This isn't a pure mechanical translation of the xterm.js bindings; docs have
//! been adjusted in places (mainly just to link to the right things on the Rust
//! side) but most importantly interfaces have been converted to either concrete
//! Rust types (that are accessible from JavaScript), imported types (duck types
//! that won't correspond exactly to any concrete type on the JavaScript side
//! and thus can't be _constructed_ from Rust), or imported types + a concrete
//! type that satisfies the interface with a Rust trait with methods that can
//! construct the concrete type for anything satisfying the trait.
//!
//! Generic interfaces are also problematic; these have been "manually
//! monomorphized" (i.e. `IEvent<Object, Void>` → `KeyEventListener` on the
//! Rust side).
//!
//! In general, the rule used for interfaces has been:
//!   - If instances are constructed by users of the xterm.js API and _written_
//!     (i.e. _given_ to the xterm.js API and never _received_ through a field
//!     access or a method call), we have a corresponding _concrete type_ that
//!     satisfies the interface. This cannot be used to manipulate/interact with
//!     externally constructed instances of the interface.
//!   - If instances are given by the xterm.js API and never constructed by
//!     users of the API (i.e. `IBuffer` or `IBufferLine`), an extern-ed
//!     JavaScript type is made (or rather, we get `wasm-bindgen` to make the
//!     necessary glue so we can access the fields/methods of the interface on
//!     whatever object we get passed that has said fields/methods).
//!   - If we need to both consume and produce implementations of an interface
//!     we'd do both of the above (as of this writing we haven't had to do this
//!     for any type).
//!   - If we need to be able to have more than one true concrete type
//!     satisfying the interface on the Rust side, we also create a Rust trait
//!     that matches the shape of the interface along with a blanket impl for
//!     the trait that makes it so that all `wasm-bindgen` 'implementations' of
//!     the interface also get an impl of the trait. See the [section on
//!     mirroring interfaces](#mirroring-interfaces) for more details.
//!
//! See: [this](https://github.com/rustwasm/wasm-bindgen/issues/18) and
//! [this](https://github.com/rustwasm/wasm-bindgen/issues/1341).
//!
//! ### Mirroring Interfaces
//!
//! As mentioned, when it's desirable to construct types that satisfy an
//! interface within Rust, we create a Rust trait that's mirror of the interface
//! in question. [`XtermAddon`] (behind the `ext` feature) is probably the best
//! example of this; we want to be able to make it so that addons can be written
//! in Rust.
//!
//! So, to make this possible we do these things:
//!   - Make a Rust trait that matches the interface.
//!   - Add a blanket impl so that the Rust trait is implemented for all the
//!     types `wasm-bindgen` produces that impl the type.
//!       + `wasm-bindgen` represents things like extending a class and
//!         implementing interfaces with [`Deref`] and [`AsRef`] impls that
//!         literally 'convert' the type into an instance of the type they're
//!         subclassing/implementing.
//!       + This works because internally these instances are represented by a
//!         [`JsValue`] that (I think) is just an object that holds all the
//!         methods the object has (including the methods that are part of the
//!         interface). When one of these methods is actually called on the Rust
//!         side of things, the [`JsValue`] (or a special `wasm-bindgen`
//!         reference to it, at least) is passed across the FFI boundary to a
//!         special JS function that `wasm-bindgen` made which knows to look up
//!         the function that we want in the JS value and call it with the
//!         arguments we passed.
//!
//! Okay! So at this point, we've got a Rust trait that mirrors a JS interface
//! and all things that implement the interface impl the Rust trait
//! automagically. Presumably, when we want to write an impl of the interface on
//! the Rust side of things, we just impl the trait.
//!
//! And this works, but there's one catch: if we're just using the impls of the
//! interface that we made in Rust, this will work just fine. Implementations
//! that are actually written in JavaScript will internally go call their JS
//! methods and the thing in Rust that's using the trait implementation won't be
//! any the wiser.
//!
//! But, if we want to pass along implementations written in Rust to a
//! _JavaScript user of the interface_, this isn't enough.
//!
//! Addons are a good example, again. It isn't enough to just be able to write
//! something in Rust that has the shape of an addon; the point here is that
//! we're able to pass it to xterm.js and actually use it! So, to do this, there
//! are some more things we have to understand and do.
//!
//! First some background:
//!   - So, `wasm-bindgen` represents interfaces as concrete types that contain
//!     a [`JsValue`] that (presumably) contains all the methods needed to
//!     satisfy the interface.
//!   - The [`AsRef`] and [`Deref`] impls pretty much just take the inner
//!     [`JsValue`] and put it into a different type that'll use the [`JsValue`]
//!     to look up and call different functions; this works because the JS value
//!     is just an object with a table of methods — all the methods the object
//!     has, not just the ones belonging to the interface we were treating the
//!     object as an instance of. The interface types (and regular class types
//!     for that matter) and kind of just a window into the object's methods,
//!     showing us a limited subset of what the object actually has.
//!   - The mechanism by which this casting happens is [`JsCast::unchecked_ref`]
//!     (and the other methods on [`JsCast`]). As the docs on that method say,
//!     no checking actually happens! We're pretty much just changing the label
//!     that lets us know what methods the corresponding JS value actually has
//!     (as in, we're going from, for example, `Terminal` to `Disposable` but
//!     nothing has actually changed; the literal bits that represent the
//!     variable are the same, but the type has changed which will let us call
//!     different methods that will look up and call different methods on the JS
//!     side). There are checked variants in [`JsCast`]; I think the way this
//!     works is by having JS functions per type/interface that check that an
//!     object actually has all the things it needs to have for an interface.
//!     [`JsCast::instanceof`] calls the JS function that does this and the
//!     checked casts (i.e. [`JsCast::dyn_ref`]) calls it.
//!   - So, anyways, anytime a JS function takes something that "satisfies an
//!     interface" it gets represented, via `wasm-bindgen`, as taking an
//!     instance of the type that corresponds to the interface. As in, something
//!     that takes an Addon won't take `impl Addon` or even `dyn Addon`, it'll
//!     just take `Addon` (sidenote: if you think about what the [`JsValue`]
//!     inside the interface types actually contain, it's basically the same as
//!     the vtables in trait objects — except that the table has all the methods
//!     in the actual type and that this is how all method calls work in JS).
//!   - All this is to say that what we need to do is make a [`JsValue`] that
//!     has entries for the methods that are part of the interface where each
//!     entry actually points to the Rust functions that are part of the
//!     implementation of the trait we're trying to pass along to JS. Once we
//!     have such an object, we can cast it as the concrete type that
//!     `wasm-bindgen` has given us for the interface and then be on our way.
//!
//! A couple of other considerations, though:
//!   - First, we'd like to this in a generic way (i.e. make it so that any
//!     Rust trait impl for a particular trait can be turned into it's concrete
//!     interface type counterpart) and we _can_, but we need to be able to
//!     distinguish between actual JS implementations and Rust implementations
//!     (both of which implement the Rust trait) because we don't want to
//!     'double wrap' the JS implementation (i.e. if we were to do the above
//!     for a JS impl for a particular method call on the interface we'd be
//!     calling a JS function that calls a Rust function that then calls the
//!     actual JS function, when we could have just called the JS function).
//!      + Luckily, this is not hard to remedy; we can have the function that
//!        turns the trait impl into the concrete type be a part of the trait
//!        _and_ we can provide a default impl that does the wrapped. Then, we
//!        can have the blanket impl (which is bounded by [`AsRef`] anyways)
//!        just call `as_ref`.
//!   - Being able to turn Rust function into things that can be called from JS
//!     comes with some restrictions:
//!      + All types in each function have to be Wasm ABI compatible which means
//!        no lifetimes or generics or trait objects, etc. This actually isn't
//!        a problem for us since we're mirroring a JS interface which means the
//!        functions are Wasm ABI compliant anyways.
//!      + The functions and everything they point to have to be `'static`. This
//!        is because we can't enforce lifetimes across the FFI boundary.
//!        Realistically this probably means using `Box::leak` whenever a Rust
//!        trait impl needs to be passed along to JS.
//!         * Rather than do this leaking internally, we'll let the user do it.
//!           We enforce the `'static` bit by having the `into_js` method on the
//!           trait require a `'static` lifetime. So, in order to actually
//!           convert their impl for use with JS users will have to leak it.
//!         * Update: for symmetry with the _actually backed by a JS impl_ case,
//!           we do preform the leaking (we don't want to require a `'static`
//!           reference for JS impls which would require users to leak them
//!           unnecessarily).
//!      + Traits that take a mutable reference to self **and** have more than
//!        one method aren't possible (safely) because the closures we pass
//!        along hold a reference to the actual instance and we can't have
//!        mutability with aliasing. So, we'll just make all trait methods only
//!        take an immutable reference to self (as `wasm-bindgen` does). Rust
//!        trait implementors will need to use interior mutability.
//!
//! The final piece required is an extension method that takes the Rust trait
//! impl instead of the concrete type and then converts it to concrete type
//! using the trait impl and passes the concrete type instance along to the
//! `wasm-bindgen` method that expects it.
//!
//! [`JsValue`]: wasm_bindgen::JsValue
//! [`JsCast`]: wasm_bindgen::JsCast
//! [`JsCast::unchecked_ref`]: wasm_bindgen::JsCast::unchecked_ref
//! [`JsCast::dyn_ref`]: wasm_bindgen::JsCast::dyn_ref
//! [`JsCast::instanceof`]: wasm_bindgen::JsCast::instanceof
//!
//! [`XtermAddon`]: super::ext::addon::XtermAddon
//!
//! [`Deref`]: core::ops::Deref
//! [`AsRef`]: core::convert::AsRef

use wasm_bindgen::prelude::*;

/// An alias for [`String`].
pub type Str = String;

/// The type of the bell notification the terminal will use.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BellStyle {
    /// No bell notification.
    None = "none",
    /// [Removed](https://github.com/xtermjs/xterm.js/issues/1155).
    #[deprecated(
        since = "3.0.0",
        note = "See: https://github.com/xtermjs/xterm.js/issues/1155"
    )]
    /// A visual bell notification.
    Visual = "visual",
    /// An auditory bell notification.
    Sound = "sound",
    /// [Removed](https://github.com/xtermjs/xterm.js/issues/1155).
    #[deprecated(
        since = "3.0.0",
        note = "See: https://github.com/xtermjs/xterm.js/issues/1155"
    )]
    Both = "both",
}

/// The style of the cursor.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorStyle {
    /// Block cursor style: `█`.
    Block = "block",
    /// Underline cursor style: `_`.
    Underline = "underline",
    /// Bar cursor style: `|`.
    Bar = "bar",
}

/// The modifier key hold to multiply scroll speed.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FastScrollModifier {
    /// The 'alt' key.
    Alt = "alt",
    /// The 'ctrl' key.
    Ctrl = "ctrl",
    /// The 'shift' key.
    Shift = "shift",
}

/// A string representing text font weight.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    /// Normal font weight.
    Normal = "normal",
    /// Bold font weight.
    Bold = "bold",
    /// 100 font weight.
    _100 = "100",
    /// 200 font weight.
    _200 = "200",
    /// 300 font weight.
    _300 = "300",
    /// 400 font weight.
    _400 = "400",
    /// 500 font weight.
    _500 = "500",
    /// 600 font weight.
    _600 = "600",
    /// 700 font weight.
    _700 = "700",
    /// 800 font weight.
    _800 = "800",
    /// 900 font weight.
    _900 = "900",
}

/// A string representing log level.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    /// Show debug (and above) log level information (all logs).
    Debug = "debug",
    /// Show information (and above) log level information.
    Info = "info",
    /// Show warning (and above) log level information.
    Warn = "warn",
    /// Show errors.
    Error = "error",
    /// Show no logs.
    Off = "off",
}

/// A string representing a renderer type.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RendererType {
    /// The DOM renderer. This is faster but doesn't support some features
    /// (letter spacing, blinking cursor). As such, this is the _fallback_.
    Dom = "dom",
    /// The Canvas renderer.
    Canvas = "canvas",
}

/// A string representing the type of a buffer.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferType {
    /// A normal buffer.
    Normal = "normal",
    /// An alternate buffer.
    Alternate = "alternate",
}


macro_rules! wasm_struct {
    (
        #[wasm_bindgen]
        $(#[$metas:meta])*
        pub struct $nom:ident {
            $(
                $(#[doc = $docs_field:literal])*
                $(#[wasm_bindgen($($wasm_opt:ident = $wasm_val:tt),+)])?
                // $(#[$metas_field:meta])*
                $(#[deprecated($($depr:tt)+)])?
                $(pub $field:ident: $field_ty:ty)?
                $(|
                    clone(
                        set = $set:ident,
                        js_name = $js_name:ident
                        $(, pub = $public:ident)?
                    )
                    $priv_field:ident: $priv_field_ty:ty
                )?
                ,
            )+
        }
    ) => {
        #[wasm_bindgen]
        $(#[$metas])*
        pub struct $nom {
            $(
                $(#[doc = $docs_field])*
                $(#[wasm_bindgen($($wasm_opt = $wasm_val),+)])?
                // $(#[$metas_field])*
                $(#[deprecated($($depr)+)])?
                $(pub $field: $field_ty)?
                $(
                    $(
                       #[doc = $public]
                       #[wasm_bindgen(skip)] pub
                    )?
                    $priv_field: $priv_field_ty
                )?
                ,
            )+
        }

        #[wasm_bindgen]
        impl $nom {
            $(
                $(#[doc = $docs_field])*

                // Some garbage to swallow the doc comment in case we're not
                // dealing with a private field:
                $(
                    #[allow(unused_doc_comments)]
                    #[cfg(__never__)]
                    fn $field() -> () { }
                )?

                $(
                    #[doc = "\n\nGetter."]
                    #[wasm_bindgen(getter = $js_name)]
                    #[must_use]
                    pub fn $priv_field(&self) -> $priv_field_ty {
                        self.$priv_field.clone()
                    }
                )?

                $(#[doc = $docs_field])*

                // Again: garbage to swallow the doc comment.
                $(
                    #[allow(unused_doc_comments)]
                    #[cfg(__never__)]
                    fn $field() -> () { }
                )?

                $(
                    #[doc = "\n\nSetter."]
                    #[wasm_bindgen(setter = $js_name)]
                    pub fn $set(&mut self, $priv_field: $priv_field_ty) {
                        self.$priv_field = $priv_field;
                    }
                )?
            )*
        }
    };
}

wasm_struct! {
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Contains colors to theme the terminal with.
///
/// (This is really an interface, but we just go and define our own type that
/// satisfies the interface.)
pub struct Theme {
    /// The default background color.
    |clone(set = set_background, js_name = background)
    background: Option<Str>,

    /// ANSI black (eg. `\x1b[30m`).
    |clone(set = set_black, js_name = black)
    black: Option<Str>,

    /// ANSI blue (eg. `\x1b[34m`).
    |clone(set = set_blue, js_name = blue)
    blue: Option<Str>,

    /// ANSI bright black (eg. `\x1b[1;30m`).
    |clone(set = set_bright_black, js_name = brightBlack)
    bright_black: Option<Str>,

    /// ANSI bright blue (eg. `\x1b[1;34m`).
    |clone(set = set_bright_blue, js_name = brightBlue)
    bright_blue: Option<Str>,

    /// ANSI bright cyan (eg. `\x1b[1;36m`).
    |clone(set = set_bright_cyan, js_name = brightCyan)
    bright_cyan: Option<Str>,

    /// ANSI bright green (eg. `\x1b[1;32m`).
    |clone(set = set_bright_green, js_name = brightGreen)
    bright_green: Option<Str>,

    /// ANSI bright magenta (eg. `\x1b[1;35m`).
    |clone(set = set_bright_magenta, js_name = brightMagenta)
    bright_magenta: Option<Str>,

    /// ANSI bright red (eg. `\x1b[1;31m`).
    |clone(set = set_bright_red, js_name = brightRed)
    bright_red: Option<Str>,

    /// ANSI bright white (eg. `\x1b[1;37m`).
    |clone(set = set_bright_white, js_name = brightWhite)
    bright_white: Option<Str>,

    /// ANSI bright yellow (eg. `\x1b[1;33m`).
    |clone(set = set_bright_yellow, js_name = brightYellow)
    bright_yellow: Option<Str>,

    /// The cursor color.
    |clone(set = set_cursor, js_name = cursor)
    cursor: Option<Str>,

    /// The accent color of the cursor (fg color for a block cursor).
    |clone(set = set_cursor_accent, js_name = cursorAccent)
    cursor_accent: Option<Str>,

    /// ANSI cyan (eg. `\x1b[36m`).
    |clone(set = set_cyan, js_name = cyan)
    cyan: Option<Str>,

    /// The default foreground color.
    |clone(set = set_foreground, js_name = foreground)
    foreground: Option<Str>,

    /// ANSI green (eg. `\x1b[32m`).
    |clone(set = set_green, js_name = green)
    green: Option<Str>,

    /// ANSI magenta (eg. `\x1b[35m`).
    |clone(set = set_magenta, js_name = magenta)
    magenta: Option<Str>,

    /// ANSI red (eg. `\x1b[31m`).
    |clone(set = set_red, js_name = red)
    red: Option<Str>,

    /// The selection background color (can be transparent).
    |clone(set = set_selection, js_name = selection)
    selection: Option<Str>,

    /// ANSI white (eg. `\x1b[37m`).
    |clone(set = set_white, js_name = white)
    white: Option<Str>,

    /// ANSI yellow (eg. `\x1b[33m`).
    |clone(set = set_yellow, js_name = yellow)
    yellow: Option<Str>,
}}

wasm_struct! {
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Enable various window manipulation and report features (`CSI Ps ; Ps ; Ps
/// t`).
///
/// Most settings have no default implementation, as they heavily rely on the
/// embedding environment.
///
/// To implement a feature, create a custom CSI hook like this:
///
/// ```ts
/// term.parser.addCsiHandler({final: 't'}, params => {
///   const ps = params[0];
///   switch (ps) {
///     case XY:
///       ...            // your implementation for option XY
///       return true;   // signal Ps=XY was handled
///   }
///   return false;      // any Ps that was not handled
/// });
/// ```
///
/// Note on security: Most features are meant to deal with some information of
/// the host machine where the terminal runs on. This is seen as a security risk
/// possibly leaking sensitive data of the host to the program in the terminal.
/// Therefore all options (even those without a default implementation) are
/// guarded by the boolean flag and disabled by default.
///
/// (This is really an interface, but we just go and define our own type that
/// satisfies the interface.)
pub struct WindowOptions {
    /// Ps=10 ; 0 Undo full-screen mode. Ps=10 ; 1 Change to full-screen. Ps=10
    /// ; 2 Toggle full-screen.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = fullscreenWin)]
    pub fullscreen_win: Option<bool>,

    /// Ps=16 Report xterm character cell size in pixels. Result is “CSI 6 ;
    /// height ; width t”.
    ///
    /// Has a default implementation.
    #[wasm_bindgen(js_name = getCellSizePixels)]
    pub get_cell_size_pixels: Option<bool>,

    /// Ps=20 Report xterm window’s icon label. Result is “OSC L label ST”.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getIconTitle)]
    pub get_icon_title: Option<bool>,

    /// Ps=19 Report the size of the screen in characters. Result is “CSI 9 ;
    /// height ; width t”.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getScreenSizeChars)]
    pub get_screen_size_chars: Option<bool>,

    /// Ps=15 Report size of the screen in pixels. Result is “CSI 5 ; height ;
    /// width t”.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getScreenSizePixels)]
    pub get_screen_size_pixels: Option<bool>,

    /// Ps=13 Report xterm window position. Result is “CSI 3 ; x ; y t”. Ps=13 ;
    /// 2 Report xterm text-area position. Result is “CSI 3 ; x ; y t”.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getWinPosition)]
    pub get_win_position: Option<bool>,

    /// Ps=18 Report the size of the text area in characters. Result is “CSI 8 ;
    /// height ; width t”.
    ///
    /// Has a default implementation.
    #[wasm_bindgen(js_name = getWinSizeChars)]
    pub get_win_size_chars: Option<bool>,

    /// Ps=14 Report xterm text area size in pixels. Result is “CSI 4 ; height ;
    /// width t”. Ps=14 ; 2 Report xterm window size in pixels. Result is “CSI 4
    /// ; height ; width t”.
    ///
    /// Has a default implementation.
    #[wasm_bindgen(js_name = getWinSizePixels)]
    pub get_win_size_pixels: Option<bool>,

    /// Ps=11 Report xterm window state. If the xterm window is non-iconified,
    /// it returns “CSI 1 t”. If the xterm window is iconified, it returns “CSI
    /// 2 t”.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getWinState)]
    pub get_win_state: Option<bool>,

    /// Ps=21 Report xterm window’s title. Result is “OSC l label ST”.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getWinTitle)]
    pub get_win_title: Option<bool>,

    /// Ps=6 Lower the xterm window to the bottom of the stacking order.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = lowerWin)]
    pub lower_win: Option<bool>,

    /// Ps=9 ; 0 Restore maximized window. Ps=9 ; 1 Maximize window (i.e.,
    /// resize to screen size). Ps=9 ; 2 Maximize window vertically. Ps=9 ; 3
    /// Maximize window horizontally.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = maximizeWin)]
    pub maximize_win: Option<bool>,

    /// Ps=2 Iconify window.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = minimizeWin)]
    pub minimize_win: Option<bool>,

    /// Ps=23 ; 0 Restore xterm icon and window title from stack. Ps=23 ; 1
    /// Restore xterm icon title from stack. Ps=23 ; 2 Restore xterm window
    /// title from stack.
    ///
    /// All variants have a default implementation.
    #[wasm_bindgen(js_name = popTitle)]
    pub pop_title: Option<bool>,

    /// Ps=22 ; 0 Save xterm icon and window title on stack. Ps=22 ; 1 Save
    /// xterm icon title on stack. Ps=22 ; 2 Save xterm window title on stack.
    ///
    /// All variants have a default implementation.
    #[wasm_bindgen(js_name = pushTitle)]
    pub push_title: Option<bool>,

    /// Ps=5 Raise the window to the front of the stacking order.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = raiseWin)]
    pub raise_win: Option<bool>,

    /// Ps=7 Refresh the window.
    #[wasm_bindgen(js_name = refreshWin)]
    pub refresh_win: Option<bool>,

    /// Ps=1 De-iconify window.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = restoreWin)]
    pub restore_win: Option<bool>,

    /// Ps>=24 Resize to Ps lines (DECSLPP). DECSLPP is not implemented. This
    /// settings is also used to enable / disable DECCOLM (earlier variant of
    /// DECSLPP).
    #[wasm_bindgen(js_name = setWinLines)]
    pub set_win_lines: Option<bool>,

    /// Ps=3 ; x ; y Move window to [x, y].
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = setWinPosition)]
    pub set_win_position: Option<bool>,

    /// Ps = 8 ; height ; width Resize the text area to given height and width
    /// in characters. Omitted parameters should reuse the current height or
    /// width. Zero parameters use the display’s height or width.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = setWinSizeChars)]
    pub set_win_size_chars: Option<bool>,

    /// Ps = 4 ; height ; width Resize the window to given height and width in
    /// pixels. Omitted parameters should reuse the current height or width.
    /// Zero parameters should use the display’s height or width.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = setWinSizePixels)]
    pub set_win_size_pixels: Option<bool>,
}}

wasm_struct! {
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Default)]
/// An object containing start up options for the terminal.
///
/// (This is really an interface, but we just go and define our own type that
/// satisfies the interface.)
pub struct TerminalOptions {
    /// Whether to allow the use of proposed API. When false, any usage of APIs
    /// marked as experimental/proposed will throw an error. This defaults to
    /// true currently, but will change to false in v5.0.
    #[wasm_bindgen(js_name = allowProposedApi)]
    pub allow_proposed_api: Option<bool>,

    /// Whether background should support non-opaque color. It must be set
    /// before executing the [`Terminal::open`] method and can’t be changed
    /// later without executing it again. Note that enabling this can negatively
    /// impact performance.
    ///
    /// [`Terminal::open()`]: Terminal::open
    #[wasm_bindgen(js_name = allowTransparency)]
    pub allow_transparency: Option<bool>,

    /// A data uri of the sound to use for the bell when
    /// [`TerminalOptions.bell_style`] = 'sound'.
    |clone(set = set_bell_sound, js_name = bellSound)
    bell_sound: Option<Str>,

    /// The type of the bell notification the terminal will use.
    #[wasm_bindgen(js_name = bellStyle)]
    pub bell_style: Option<BellStyle>,

    /// The number of columns in the terminal.
    #[wasm_bindgen(js_name = cols)]
    pub cols: Option<u16>,

    /// When enabled the cursor will be set to the beginning of the next line
    /// with every new line. This is equivalent to sending ‘\r\n’ for each ‘\n’.
    /// Normally the termios settings of the underlying PTY deals with the
    /// translation of ‘\n’ to ‘\r\n’ and this setting should not be used. If
    /// you deal with data from a non-PTY related source, this settings might be
    /// useful.
    #[wasm_bindgen(js_name = convertEol)]
    pub convert_eol: Option<bool>,

    /// Whether the cursor blinks.
    #[wasm_bindgen(js_name = cursorBlink)]
    pub cursor_blink: Option<bool>,

    /// The style of the cursor.
    #[wasm_bindgen(js_name = cursorStyle)]
    pub cursor_style: Option<CursorStyle>,

    /// The width of the cursor in CSS pixels when [`cursor_style`] is set to
    /// [‘bar’].
    ///
    /// [`cursor_style`]: TerminalOptions.cursor_style
    /// [‘bar’]: CursorStyle::Bar
    #[wasm_bindgen(js_name = cursorWidth)]
    pub cursor_width: Option<f32>,

    /// Whether input should be disabled.
    #[wasm_bindgen(js_name = disableStdin)]
    pub disable_stdin: Option<bool>,

    /// Whether to draw bold text in bright colors. The default is true.
    #[wasm_bindgen(js_name = drawBoldTextInBrightColors)]
    pub draw_bold_text_in_bright_colors: Option<bool>,

    /// The modifier key hold to multiply scroll speed.
    #[wasm_bindgen(js_name = fastScrollModifier)]
    pub fast_scroll_modifier: Option<FastScrollModifier>,

    /// The scroll speed multiplier used for fast scrolling.
    #[wasm_bindgen(js_name = fastScrollSensitivity)]
    pub fast_scroll_sensitivity: Option<f32>,

    /// The font family used to render text.
    |clone(set = set_font_family, js_name = fontFamily)
    font_family: Option<Str>,

    /// The font size used to render text.
    #[wasm_bindgen(js_name = fontSize)]
    pub font_size: Option<f32>,

    /// The font weight used to render non-bold text.
    #[wasm_bindgen(js_name = fontWeight)]
    pub font_weight: Option<f32>,

    /// The font weight used to render bold text.
    #[wasm_bindgen(js_name = fontWeightBold)]
    pub font_weight_bold: Option<FontWeight>,

    /// The spacing in whole pixels between characters.
    #[wasm_bindgen(js_name = letterSpacing)]
    pub letter_spacing: Option<u16>,

    /// The line height used to render text.
    #[wasm_bindgen(js_name = lineHeight)]
    pub line_height: Option<u16>,

    /// The duration in milliseconds before link tooltip events fire when
    /// hovering on a link.
    #[wasm_bindgen(js_name = linkTooltipHoverDuration)]
    #[deprecated(
        since = "4.6.0",
        note = "This will be removed when the link matcher API is removed. \
        See: https://github.com/xtermjs/xterm.js/issues/2703"
    )]
    pub link_tooltip_hover_duration: Option<u16>,

    /// What log level to use, this will log for all levels below and including
    /// what is set:
    ///  1) debug
    ///  2) info __(default)__
    ///  3) warn
    ///  4) error
    ///  5) off
    #[wasm_bindgen(js_name = logLevel)]
    pub log_level: Option<LogLevel>,

    /// Whether holding a modifier key will force normal selection behavior,
    /// regardless of whether the terminal is in mouse events mode. This will
    /// also prevent mouse events from being emitted by the terminal. For
    /// example, this allows you to use xterm.js’ regular selection inside tmux
    /// with mouse mode enabled.
    #[wasm_bindgen(js_name = macOptionClickForcesSelection)]
    pub mac_option_click_forces_selection: Option<bool>,

    /// Whether to treat option as the meta key.
    #[wasm_bindgen(js_name = macOptionIsMeta)]
    pub mac_option_is_meta: Option<bool>,

    /// The minimum contrast ratio for text in the terminal, setting this will
    /// change the foreground color dynamically depending on whether the
    /// contrast ratio is met. Example values:
    ///   - 1: The default, do nothing.
    ///   - 4.5: Minimum for WCAG AA compliance.
    ///   - 7: Minimum for WCAG AAA compliance.
    ///   - 21: White on black or black on white.
    #[wasm_bindgen(js_name = minimumContrastRatio)]
    pub minimum_contrast_ratio: Option<f32>,

    /// The type of renderer to use, this allows using the fallback DOM renderer
    /// when canvas is too slow for the environment. The following features do
    /// not work when the DOM renderer is used:
    ///   - Letter spacing
    ///   - Cursor blink
    #[wasm_bindgen(js_name = rendererType)]
    pub renderer_type: Option<RendererType>,

    /// Whether to select the word under the cursor on right click, this is
    /// standard behavior in a lot of macOS applications.
    #[wasm_bindgen(js_name = rightClickSelectsWord)]
    pub right_click_selects_word: Option<bool>,

    /// The number of rows in the terminal.
    #[wasm_bindgen(js_name = rows)]
    pub rows: Option<u16>,

    /// Whether screen reader support is enabled. When on this will expose
    /// supporting elements in the DOM to support NVDA on Windows and VoiceOver
    /// on macOS.
    #[wasm_bindgen(js_name = screenReaderMode)]
    pub screen_reader_mode: Option<bool>,

    /// The scrolling speed multiplier used for adjusting normal scrolling
    /// speed.
    #[wasm_bindgen(js_name = scrollSensitivity)]
    pub scroll_sensitivity: Option<f32>,

    /// The amount of scrollback in the terminal. Scrollback is the amount of
    /// rows that are retained when lines are scrolled beyond the initial
    /// viewport.
    #[wasm_bindgen(js_name = scrollback)]
    pub scrollback: Option<u64>,

    /// The size of tab stops in the terminal.
    #[wasm_bindgen(js_name = tabStopWidth)]
    pub tab_stop_width: Option<u16>,

    /// The color theme of the terminal.
    |clone(set = set_theme, js_name = theme)
    theme: Option<Theme>,

    /// Enable various window manipulation and report features. All features are
    /// disabled by default for security reasons.
    |clone(set = set_window_options, js_name = windowOptions)
    window_options: Option<WindowOptions>,

    /// Whether “Windows mode” is enabled. Because Windows backends winpty and
    /// conpty operate by doing line wrapping on their side, xterm.js does not
    /// have access to wrapped lines. When Windows mode is enabled the following
    /// changes will be in effect:
    ///
    ///   - Reflow is disabled.
    ///   - Lines are assumed to be wrapped if the last character of the line is
    ///     not whitespace.
    #[wasm_bindgen(js_name = windowsMode)]
    pub windows_mode: Option<bool>,

    /// A string containing all characters that are considered word separated by
    /// the double click to select work logic.
    |clone(set = set_word_separator, js_name = wordSeparator)
    word_separator: Option<Str>,
}}

/// A Color for use with xterm.js.
///
/// Can represent:
///   - the Default color (0),
///   - a Palette number (0 to 255, inclusive).
///   - or, an RGB 'true color' (24 bits, 0xRRGGBB).
pub type Color = u32;

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents a single cell in the terminal’s buffer.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    pub type BufferCell;

    /// Gets a cell’s background color number, this differs depending on what
    /// the color mode of the cell is:
    ///   - Default: This should be 0, representing the default background color
    ///     (CSI 49 m).
    ///   - Palette: This is a number from 0 to 255 of ANSI colors (CSI 4(0-7)
    ///     m, CSI 10(0-7) m, CSI 48 ; 5 ; 0-255 m).
    ///   - RGB: A hex value representing a ‘true color’: 0xRRGGBB (CSI 4 8 ; 2
    ///     ; Pi ; Pr ; Pg ; Pb)
    #[wasm_bindgen(structural, method, js_name = getBgColor)]
    pub fn get_bg_color(this: &BufferCell) -> Color;

    /// Gets the number representation of the background color mode, this can be
    /// used to perform quick comparisons of 2 cells to see if they’re the same.
    /// Use [`is_bg_rgb`], [`is_bg_palette`], and [`is_bg_default`] to check
    /// what color mode a cell is.
    ///
    /// [`is_bg_rgb`]: BufferCell::is_bg_rgb
    /// [`is_bg_palette`]: BufferCell::is_bg_palette
    /// [`is_bg_default`]: BufferCell::is_bg_default
    #[wasm_bindgen(structural, method, js_name = getBgColorMode)]
    pub fn get_bg_color_mode(this: &BufferCell) -> u8;

    /// The character(s) within the cell. Examples of what this can contain:
    ///   - A normal width character
    ///   - A wide character (eg. CJK)
    ///   - An emoji
    #[wasm_bindgen(structural, method, js_name = getChars)]
    pub fn get_chars(this: &BufferCell) -> Str;

    /// Gets the UTF32 codepoint of single characters, if content is a combined
    /// string it returns the codepoint of the last character in the string.
    #[wasm_bindgen(structural, method, js_name = getCode)]
    pub fn get_code(this: &BufferCell) -> u32;

    /// Gets a cell’s foreground color number, this differs depending on what
    /// the color mode of the cell is:
    ///   - Default: This should be 0, representing the default foreground color
    ///     (CSI 39 m).
    ///   - Palette: This is a number from 0 to 255 of ANSI colors (CSI 3(0-7)
    ///     m, CSI 9(0-7) m, CSI 38 ; 5 ; 0-255 m).
    ///   - RGB: A hex value representing a ‘true color’: 0xRRGGBB. (CSI 3 8 ; 2
    ///     ; Pi ; Pr ; Pg ; Pb)
    #[wasm_bindgen(structural, method, js_name = getFgColor)]
    pub fn get_fg_color(this: &BufferCell) -> Color;

    /// Gets the number representation of the foreground color mode, this can be
    /// used to perform quick comparisons of 2 cells to see if they’re the same.
    /// Use [`is_fg_rgb`], [`is_fg_palette`], and [`is_fg_default`] to check
    /// what color mode a cell is.
    ///
    /// [`is_fg_rgb`]: BufferCell::is_fg_rgb
    /// [`is_fg_palette`]: BufferCell::is_fg_palette
    /// [`is_fg_default`]: BufferCell::is_fg_default
    #[wasm_bindgen(structural, method, js_name = getFgColorMode)]
    pub fn get_fg_color_mode(this: &BufferCell) -> u8;

    /// The width of the character. Some examples:
    ///   - `1` for most cells.
    ///   - `2` for wide character like CJK glyphs.
    ///   - `0` for cells immediately following cells with a width of `2`.
    #[wasm_bindgen(structural, method, js_name = getWidth)]
    pub fn get_width(this: &BufferCell) -> u8;

    /// Whether the cell has the default attribute (no color or style).
    #[wasm_bindgen(structural, method, js_name = isAttributeDefault)]
    pub fn is_attribute_default(this: &BufferCell) -> bool;

    /// Whether the cell is using the default background color mode.
    #[wasm_bindgen(structural, method, js_name = isBgDefault)]
    pub fn is_bg_default(this: &BufferCell) -> bool;

    /// Whether the cell is using the palette background color mode.
    #[wasm_bindgen(structural, method, js_name = isBgPalette)]
    pub fn is_bg_palette(this: &BufferCell) -> bool;

    /// Whether the cell is using the RGB background color mode.
    #[wasm_bindgen(structural, method, js_name = isBgRGB)]
    pub fn is_bg_rgb(this: &BufferCell) -> bool;

    /// Whether the cell has the inverse attribute (CSI 5 m).
    // Note: returns a number in the original API.
    #[wasm_bindgen(structural, method, js_name = isBlink)]
    pub fn is_blink(this: &BufferCell) -> bool;

    /// Whether the cell has the bold attribute (CSI 1 m).
    // Note: returns a number in the original API.
    #[wasm_bindgen(structural, method, js_name = isBold)]
    pub fn is_bold(this: &BufferCell) -> bool;

    /// Whether the cell has the inverse attribute (CSI 2 m).
    // Note: returns a number in the original API.
    #[wasm_bindgen(structural, method, js_name = isDim)]
    pub fn is_dim(this: &BufferCell) -> bool;

    /// Whether the cell is using the default foreground color mode.
    #[wasm_bindgen(structural, method, js_name = isFgDefault)]
    pub fn is_fg_default(this: &BufferCell) -> bool;

    /// Whether the cell is using the palette foreground color mode.
    #[wasm_bindgen(structural, method, js_name = isFgPalette)]
    pub fn is_fg_palette(this: &BufferCell) -> bool;

    /// Whether the cell is using the RGB foreground color mode.
    #[wasm_bindgen(structural, method, js_name = isFgRGB)]
    pub fn is_fg_rgb(this: &BufferCell) -> bool;

    /// Whether the cell has the inverse attribute (CSI 7 m).
    // Note: returns a number in the original API.
    #[wasm_bindgen(structural, method, js_name = isInverse)]
    pub fn is_inverse(this: &BufferCell) -> bool;

    /// Whether the cell has the inverse attribute (CSI 8 m).
    // Note: returns a number in the original API.
    #[wasm_bindgen(structural, method, js_name = isInvisible)]
    pub fn is_invisible(this: &BufferCell) -> bool;

    /// Whether the cell has the inverse attribute (CSI 3 m).
    // Note: returns a number in the original API.
    #[wasm_bindgen(structural, method, js_name = isItalic)]
    pub fn is_italic(this: &BufferCell) -> bool;

    /// Whether the cell has the underline attribute (CSI 4 m).
    // Note: returns a number in the original API.
    #[wasm_bindgen(structural, method, js_name = isUnderline)]
    pub fn is_underline(this: &BufferCell) -> bool;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents a line in the terminal’s buffer.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    pub type BufferLine;

    /// Whether the line is wrapped from the previous line.
    #[wasm_bindgen(structural, method, getter = isWrapped)]
    pub fn is_wrapped(this: &BufferLine) -> bool;

    /// The length of the line.
    ///
    /// All calls to [`BufferLine::get_cell`] beyond the length will result in
    /// `None`.
    ///
    /// [`BufferLine::get_cell`]: BufferLine::get_cell
    #[wasm_bindgen(structural, method, getter = length)]
    pub fn length(this: &BufferLine) -> u16;

    /// Gets a cell from the line, or `None` if the line index does not
    /// exist.
    ///
    /// Note that the result of this function should be used immediately after
    /// calling as when the terminal updates it could lead to unexpected
    /// behavior.
    ///
    /// Takes:
    ///   - `x`:    The character index to get.
    ///   - `cell`: Optional cell object to load data into for performance
    ///             reasons. This is mainly useful when every cell in the buffer
    ///             is being looped over to avoid creating new objects for every
    ///             cell.
    #[wasm_bindgen(structural, method, js_name = getCell)]
    pub fn get_cell(
        this: &BufferLine,
        cell: Option<BufferCell>,
    ) -> Option<BufferCell>;

    /// Gets the line as a string. Note that this is gets only the string for
    /// the line, not taking [`BufferLine::is_wrapped`] into account.
    ///
    /// Takes:
    ///   - `trim_right`:   Whether to trim any whitespace at the right of the
    ///                     line.
    ///   - `start_column`: The column to start from (inclusive).
    ///   - `end_column`:   The column to end at (exclusive).
    ///
    /// [`BufferLine::is_wrapped`]: BufferLine::is_wrapped
    #[wasm_bindgen(structural, method, js_name = translateToString)]
    pub fn translate_to_string(
        this: &BufferLine,
        trim_right: Option<bool>,
        start_column: Option<u16>,
        end_column: Option<u16>,
    ) -> Str;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents a terminal buffer.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type Buffer;

    /// Gets the line within the buffer where the top of the bottom page is
    /// (when fully scrolled down).
    #[wasm_bindgen(structural, method, getter = baseY)]
    pub fn base_y(this: &Buffer) -> u16;

    /// Gets the x position of the cursor. This ranges between 0 (left side) and
    /// [`Terminal::cols()`] (after last cell of the row).
    ///
    /// [`Terminal::cols()`]: Terminal::cols
    #[wasm_bindgen(structural, method, getter = cursorX)]
    pub fn cursor_x(this: &Buffer) -> u16;

    /// Gets the y position of the cursor. This ranges between 0 (when the
    /// cursor is at `Buffer::base_y()`) and [`Terminal::rows()`] - 1 (when the
    /// cursor is on the last row).
    ///
    /// [`Buffer::base_y()`]: Buffer::base_y
    /// [`Terminal::rows()`]: Terminal::rows
    #[wasm_bindgen(structural, method, getter = cursorY)]
    pub fn cursor_y(this: &Buffer) -> u16;

    /// Gets the amount of lines in the buffer.
    #[wasm_bindgen(structural, method, getter = length)]
    pub fn length(this: &Buffer) -> u16;

    /// Get the line within the buffer where the top of the viewport is.
    #[wasm_bindgen(structural, method, getter = viewportY)]
    pub fn viewport_y(this: &Buffer) -> u16;

    /// Gets a line from the buffer, or undefined if the line index does not
    /// exist.
    ///
    /// Note that the result of this function should be used immediately after
    /// calling as when the terminal updates it could lead to unexpected
    /// behavior.
    ///
    /// Takes `y`: the line index to get.
    #[wasm_bindgen(structural, method, js_name = getLine)]
    pub fn get_line(this: &Buffer, y: u16) -> Option<BufferLine>;

    /// Creates an empty cell object suitable as a cell reference in
    /// [`BufferLine::get_cell`]. Use this to avoid costly recreation of cell
    /// objects when dealing with tons of cells.
    ///
    /// [`BufferLine::get_cell`]: BufferLine::get_cell
    #[wasm_bindgen(structural, method, js_name = getNullCell)]
    pub fn get_null_cell(this: &Buffer) -> BufferCell;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// An object that can be disposed via a dispose function.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type Disposable;

    /// Disposes of the instance.
    ///
    /// This can involve unregistering an event listener or cleaning up
    /// resources or anything else that should happen when an instance is
    /// disposed of.
    #[wasm_bindgen(structural, method, js_name = dispose)]
    pub fn dispose(this: &Disposable);
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// An addon that can provide additional functionality to the terminal.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[wasm_bindgen(extends = Disposable)]
    #[derive(Debug, Clone)]
    pub type TerminalAddon;

    /// This is called when the addon is activated.
    #[wasm_bindgen(structural, method, js_name = activate)]
    pub fn activate(this: &TerminalAddon, terminal: Terminal);

}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Corresponds to `{ key: string, domEvent: KeyboardEvent }`.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type KeyEventData;

    /// Gets the `String` representing this key event that will be sent to
    /// [`Terminal::onData`].
    ///
    /// [`Terminal::onData`]: Terminal::onData
    #[wasm_bindgen(structural, method, getter = key)]
    pub fn key(this: &KeyEventData) -> Str;

    /// Gets the actual DOM Event ([`KeyboardEvent`]) that triggered the event.
    ///
    /// [`KeyboardEvent`]: web_sys::KeyboardEvent
    #[wasm_bindgen(structural, method, getter = domEvent)]
    pub fn dom_event(this: &KeyEventData) -> web_sys::KeyboardEvent;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// The class that represents an xterm.js terminal.
    #[wasm_bindgen(extends = Disposable)]
    #[derive(Debug, Clone)]
    pub type Terminal;

    /// Creates a new Terminal object.
    ///
    /// Takes `options`: an object containing a set of options.
    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<TerminalOptions>) -> Terminal;

    /////////////////////////////// Properties ///////////////////////////////

    /// **[EXPERIMENTAL]** The terminal’s current buffer, this might be either
    /// the normal buffer or the alt buffer depending on what’s running in the
    /// terminal.
    #[wasm_bindgen(method, getter = buffer)]
    pub fn buffer(this: &Terminal) -> Buffer;

    /// The number of columns in the terminal’s viewport. Use
    /// [`TerminalOptions.cols`] to set this in the [constructor] and
    /// [`Terminal::resize`] for when the terminal exists.
    ///
    /// [`TerminalOptions.cols`]: TerminalOptions.cols
    /// [constructor]: Terminal::new
    /// [`Terminal::resize`]: Terminal::resize
    #[wasm_bindgen(method, getter = cols)]
    pub fn cols(this: &Terminal) -> u16;

    /// The element containing the terminal.
    #[wasm_bindgen(method, getter = element)]
    pub fn element(this: &Terminal) -> Option<web_sys::Element>;

    /*  [TODO]
        markers
        • markers: ReadonlyArray‹IMarker›
        Defined in xterm.d.ts:583
        (EXPERIMENTAL) Get all markers registered against the buffer. If the alt buffer is active this will always return [].
    */

    /// Adds an event listener for when a binary event fires. This is used to
    /// enable non UTF-8 conformant binary messages to be sent to the backend.
    /// Currently this is only used for a certain type of mouse reports that
    /// happen to be not UTF-8 compatible. The event value is a `String`, pass
    /// it to the underlying pty as binary data, e.g.
    /// `pty.write(Buffer.from(data, 'binary'))`.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_binary_event_listener`] (if the `ext` feature is enabled)
    /// for a friendlier version of this function.
    ///
    /// [`Disposable`]: Disposable
    /// [`attach_binary_event_listener`]: Terminal::attach_binary_event_listener
    #[wasm_bindgen(method, js_name = onBinary)]
    pub fn on_binary(
        this: &Terminal,
        listener: &Closure<dyn FnMut(Str)>,
    ) -> Disposable;

    /// Adds an event listener for the cursor moves.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_cursor_move_event_listener`] (if the `ext` feature is
    /// enabled) for a friendlier version of this function.
    ///
    /// [`Disposable`]: Disposable
    /// [`attach_cursor_move_event_listener`]: Terminal::attach_cursor_move_event_listener
    #[wasm_bindgen(method, js_name = onCursorMove)]
    pub fn on_cursor_move(
        this: &Terminal,
        listener: &Closure<dyn FnMut()>,
    ) -> Disposable;

    // [TODO]
    //    onData
    //    • onData: IEvent‹string›
    //    Defined in xterm.d.ts:633
    //    Adds an event listener for when a data event fires. This happens for example when the user types or pastes into the terminal. The event value is whatever string results, in a typical setup, this should be passed on to the backing pty.
    //    returns an IDisposable to stop listening.

    /// Adds an event listener for when a key is pressed. The event value
    /// ([`KeyEventData`]) contains the string that will be sent in the data
    /// event as well as the DOM event that triggered it.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_key_event_listener`] (if the `ext` feature is enabled)
    /// for a friendlier version of this function.
    ///
    /// [`Disposable`]: Disposable
    /// [`KeyEventData`]: KeyEventData
    /// [`attach_key_event_listener`]: Terminal::attach_key_event_listener
    #[wasm_bindgen(method, js_name = onKey)]
    pub fn on_key(
        this: &Terminal,
        listener: &Closure<dyn FnMut(KeyEventData)>,
    ) -> Disposable;

    // [TODO]
    //    onLineFeed
    //    • onLineFeed: IEvent‹void›
    //    Defined in xterm.d.ts:647
    //    Adds an event listener for when a line feed is added.
    //    returns an IDisposable to stop listening.

    // [TODO]
    //    onRender
    //    • onRender: IEvent‹object›
    //    Defined in xterm.d.ts:668
    //    Adds an event listener for when rows are rendered. The event value contains the start row and end rows of the rendered area (ranges from 0 to Terminal.rows - 1).
    //    returns an IDisposable to stop listening.

    // [TODO]
    //    onResize
    //    • onResize: IEvent‹object›
    //    Defined in xterm.d.ts:675
    //    Adds an event listener for when the terminal is resized. The event value contains the new size.
    //    returns an IDisposable to stop listening.

    // [TODO]
    //    onScroll
    //    • onScroll: IEvent‹number›
    //    Defined in xterm.d.ts:654
    //    Adds an event listener for when a scroll occurs. The event value is the new position of the viewport.
    //    returns an IDisposable to stop listening.

    // [TODO]
    //   onSelectionChange
    //   • onSelectionChange: IEvent‹void›
    //   Defined in xterm.d.ts:660
    //   Adds an event listener for when a selection change occurs.
    //   returns an IDisposable to stop listening.

    // [TODO]
    //   onTitleChange
    //   • onTitleChange: IEvent‹string›
    //   Defined in xterm.d.ts:682
    //   Adds an event listener for when an OSC 0 or OSC 2 title change occurs. The event value is the new title.
    //   returns an IDisposable to stop listening.

    /*  [TODO]
        parser
        • parser: IParser
        Defined in xterm.d.ts:589
        (EXPERIMENTAL) Get the parser interface to register custom escape sequence handlers.
    */

    /// The number of rows in the terminal’s viewport. Use
    /// [`TerminalOptions.rows`] to set this in the [constructor] and
    /// [`Terminal.resize`] for when the terminal exists.
    ///
    /// [`TerminalOptions.rows`]: TerminalOptions.rows
    /// [constructor]: Terminal::new
    /// [`Terminal::resize`]: Terminal::resize
    #[wasm_bindgen(method, getter = rows)]
    pub fn rows(this: &Terminal) -> u16;

    /// The [textarea] that accepts input for the terminal.
    ///
    /// [textarea]: web_sys::HtmlTextAreaElement
    #[wasm_bindgen(method, getter = textarea)]
    pub fn textarea(this: &Terminal) -> Option<web_sys::HtmlTextAreaElement>;

    /*  [TODO]
        unicode
        • unicode: IUnicodeHandling
        Defined in xterm.d.ts:595
        (EXPERIMENTAL) Get the Unicode handling interface to register and switch Unicode version.
    */

    /*  [TODO]
        Static strings
        ▪ strings: ILocalizableStrings
        Defined in xterm.d.ts:600
        Natural language strings that can be localized.
    */

    ///////////////////////////////  Methods  ///////////////////////////////

    /*  [TODO]
        addMarker
        ▸ addMarker(cursorYOffset: number): IMarker | undefined
        deprecated use registerMarker instead.
        Parameters:
        Name    Type
        cursorYOffset   number
        Returns: IMarker | undefined
    */

    /*  [TODO]
        attachCustomKeyEventHandler
        ▸ attachCustomKeyEventHandler(customKeyEventHandler: function): void
        Attaches a custom key event handler which is run before keys are processed, giving consumers of xterm.js ultimate control as to what keys should be processed by the terminal and what keys should not.
        Parameters:
        ▪ customKeyEventHandler: function
        The custom KeyboardEvent handler to attach. This is a function that takes a KeyboardEvent, allowing consumers to stop propagation and/or prevent the default action. The function returns whether the event should be processed by xterm.js.
        ▸ (event: KeyboardEvent): boolean
        Parameters:
        Name    Type
        event   KeyboardEvent
        Returns: void
    */

    // [TODO]
    //   blur
    //   ▸ blur(): void
    //   Unfocus the terminal.
    //   Returns: void

    // [TODO]
    //   clear
    //   ▸ clear(): void
    //   Clear the entire buffer, making the prompt line the new first line.
    //   Returns: void

    // [TODO]
    //   clearSelection
    //   ▸ clearSelection(): void
    //   Clears the current terminal selection.
    //   Returns: void

    /*  [TODO]
        deregisterCharacterJoiner
        ▸ deregisterCharacterJoiner(joinerId: number): void
        (EXPERIMENTAL) Deregisters the character joiner if one was registered. NOTE: character joiners are only used by the canvas renderer.
        Parameters:
        Name    Type    Description
        joinerId    number  The character joiner’s ID (returned after register)
        Returns: void
    */

    /*  [TODO]
        deregisterLinkMatcher
        ▸ deregisterLinkMatcher(matcherId: number): void
        (EXPERIMENTAL) Deregisters a link matcher if it has been registered.
        @deprecated The link matcher API is now deprecated in favor of the link provider API, see `registerLinkProvider`.
        Parameters:
        Name    Type    Description
        matcherId   number  The link matcher’s ID (returned after register)
        Returns: void
    */

    /// Focus the terminal.
    #[wasm_bindgen(method, js_name = focus)]
    pub fn focus(this: &Terminal);

    /*  [TODO]
        getOption
        ▸ getOption(key: “bellSound”    “bellStyle”     “cursorStyle”   “fontFamily”    “fontWeight”    “fontWeightBold”    “logLevel”  “rendererType”  “termName”  “wordSeparator”): string
        Retrieves an option’s value from the terminal.
        Parameters:
        Name    Type    Description
        key     “bellSound” | “bellStyle” | “cursorStyle” | “fontFamily” | “fontWeight” | “fontWeightBold” | “logLevel” | “rendererType” | “termName” | “wordSeparator”     The option key.
        Returns: string

        ▸ getOption(key: “allowTransparency”    “cancelEvents”  “convertEol”    “cursorBlink”   “disableStdin”  “macOptionIsMeta”   “rightClickSelectsWord”     “popOnBell”     “visualBell”    “windowsMode”): boolean
        Retrieves an option’s value from the terminal.
        Parameters:
        Name    Type    Description
        key     “allowTransparency” | “cancelEvents” | “convertEol” | “cursorBlink” | “disableStdin” | “macOptionIsMeta” | “rightClickSelectsWord” | “popOnBell” | “visualBell” | “windowsMode”     The option key.
        Returns: boolean

        ▸ getOption(key: “cols”     “fontSize”  “letterSpacing”     “lineHeight”    “rows”  “tabStopWidth”  “scrollback”): number
        Retrieves an option’s value from the terminal.
        Parameters:
        Name    Type    Description
        key     “cols” | “fontSize” | “letterSpacing” | “lineHeight” | “rows” | “tabStopWidth” | “scrollback”   The option key.
        Returns: number

        ▸ getOption(key: string): any
        Retrieves an option’s value from the terminal.
        Parameters:
        Name    Type    Description
        key     string  The option key.
        Returns: any
    */

    /*  [TODO]
        getSelection
        ▸ getSelection(): string
        Gets the terminal’s current selection, this is useful for implementing copy behavior outside of xterm.js.
        Returns: string
    */

    /*  [TODO]
        getSelectionPosition
        ▸ getSelectionPosition(): *ISelectionPosition   undefined*
        Gets the selection position or undefined if there is no selection.
        Returns: *ISelectionPosition    undefined*
    */

    // [TODO]
    //   hasSelection
    //   ▸ hasSelection(): boolean
    //   Gets whether the terminal has an active selection.
    //   Returns: boolean

    /// Loads an addon into this instance of the xterm.js [`Terminal`].
    ///
    /// Takes:
    ///   - addon: The addon to load.
    ///
    /// See [`load_xterm_addon`] (if the `ext` feature is enabled) for a
    /// friendlier version of this function.
    ///
    /// [`load_xterm_addon`]: Terminal::load_xterm_addon
    #[wasm_bindgen(method, js_name = loadAddon)]
    pub fn load_addon(this: &Terminal, addon: TerminalAddon);

    /// Opens the terminal within an element.
    ///
    /// Takes:
    ///   - parent: The element to create the terminal within. This element must
    ///             be visible (have dimensions) when open is called as several
    ///             DOM-based measurements need to be performed when this
    ///             function is called.
    #[wasm_bindgen(method, js_name = open)]
    pub fn open(this: &Terminal, parent: web_sys::HtmlElement);

    /*  [TODO]
        paste
        ▸ paste(data: string): void
        Writes text to the terminal, performing the necessary transformations for pasted text.
        Parameters:
        Name    Type    Description
        data    string  The text to write to the terminal.
        Returns: void
    */

    /*  [TODO]
        refresh
        ▸ refresh(start: number, end: number): void
        Tells the renderer to refresh terminal content between two rows (inclusive) at the next opportunity.
        Parameters:
        Name    Type    Description
        start   number  The row to start from (between 0 and this.rows - 1).
        end     number  The row to end at (between start and this.rows - 1).
        Returns: void
    */

    /*  [TODO]
        registerCharacterJoiner
        ▸ registerCharacterJoiner(handler: function): number
        (EXPERIMENTAL) Registers a character joiner, allowing custom sequences of characters to be rendered as a single unit. This is useful in particular for rendering ligatures and graphemes, among other things.
        Each registered character joiner is called with a string of text representing a portion of a line in the terminal that can be rendered as a single unit. The joiner must return a sorted array, where each entry is itself an array of length two, containing the start (inclusive) and end (exclusive) index of a substring of the input that should be rendered as a single unit. When multiple joiners are provided, the results of each are collected. If there are any overlapping substrings between them, they are combined into one larger unit that is drawn together.
        All character joiners that are registered get called every time a line is rendered in the terminal, so it is essential for the handler function to run as quickly as possible to avoid slowdowns when rendering. Similarly, joiners should strive to return the smallest possible substrings to render together, since they aren’t drawn as optimally as individual characters.
        NOTE: character joiners are only used by the canvas renderer.
        Parameters:
        ▪ handler: function
        The function that determines character joins. It is called with a string of text that is eligible for joining and returns an array where each entry is an array containing the start (inclusive) and end (exclusive) indexes of ranges that should be rendered as a single unit.
        ▸ (text: string): [number, number][]
        Parameters:
        Name    Type
        text    string
        Returns: number
        The ID of the new joiner, this can be used to deregister
    */

    /*  [TODO]
        registerLinkMatcher
        ▸ registerLinkMatcher(regex: RegExp, handler: function, options?: ILinkMatcherOptions): number
        (EXPERIMENTAL) Registers a link matcher, allowing custom link patterns to be matched and handled.
        @deprecated The link matcher API is now deprecated in favor of the link provider API, see `registerLinkProvider`.
        Parameters:
        ▪ regex: RegExp
        The regular expression to search for, specifically this searches the textContent of the rows. You will want to use \s to match a space ‘ ‘ character for example.
        ▪ handler: function
        The callback when the link is called.
        ▸ (event: MouseEvent, uri: string): void
        Parameters:
        Name    Type
        event   MouseEvent
        uri     string
        ▪Optional options: ILinkMatcherOptions
        Options for the link matcher.
        Returns: number
        The ID of the new matcher, this can be used to deregister.
    */

    /*  [TODO]
        registerMarker
        ▸ registerMarker(cursorYOffset: number): IMarker | undefined
        (EXPERIMENTAL) Adds a marker to the normal buffer and returns it. If the alt buffer is active, undefined is returned.
        Parameters:
        Name    Type    Description
        cursorYOffset   number  The y position offset of the marker from the cursor.
        Returns: IMarker | undefined
    */

    // reset
    // ▸ reset(): void
    // Perform a full reset (RIS, aka ‘\x1bc’).
    // Returns: void

    // [TODO]
    //   resize
    //   ▸ resize(columns: number, rows: number): void
    //   Resizes the terminal. It’s best practice to debounce calls to resize, this will help ensure that the pty can respond to the resize event before another one occurs.
    //   Parameters:
    //   Name    Type
    //   columns     number
    //   rows    number
    //   Returns: void

    /*  [TODO]
        scrollLines
        ▸ scrollLines(amount: number): void
        Scroll the display of the terminal
        Parameters:
        Name    Type    Description
        amount  number  The number of lines to scroll down (negative scroll up).
        Returns: void
    */

    /*  [TODO]
        scrollPages
        ▸ scrollPages(pageCount: number): void
        Scroll the display of the terminal by a number of pages.
        Parameters:
        Name    Type    Description
        pageCount   number  The number of pages to scroll (negative scrolls up).
        Returns: void
    */

    /*  [TODO]
        scrollToBottom
        ▸ scrollToBottom(): void
        Scrolls the display of the terminal to the bottom.
        Returns: void
    */

    /*  [TODO]
        scrollToLine
        ▸ scrollToLine(line: number): void
        Scrolls to a line within the buffer.
        Parameters:
        Name    Type    Description
        line    number  The 0-based line index to scroll to.
        Returns: void
    */

    /*  [TODO]
        scrollToTop
        ▸ scrollToTop(): void
        Scrolls the display of the terminal to the top.
        Returns: void
    */

    /*  [TODO]
        select
        ▸ select(column: number, row: number, length: number): void
        Selects text within the terminal.
        Parameters:
        Name    Type    Description
        column  number  The column the selection starts at.
        row     number  The row the selection starts at.
        length  number  The length of the selection.
        Returns: void
    */

    /*  [TODO]
        selectAll
        ▸ selectAll(): void
        Selects all text within the terminal.
        Returns: void
    */

    /*  [TODO]
        selectLines
        ▸ selectLines(start: number, end: number): void
        Selects text in the buffer between 2 lines.
        Parameters:
        Name    Type    Description
        start   number  The 0-based line index to select from (inclusive).
        end     number  The 0-based line index to select to (inclusive).
        Returns: void
    */

    /*  [TODO]
        setOption
        ▸ setOption(key: “fontFamily”   “termName”  “bellSound”     “wordSeparator”, value: string): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “fontFamily” | “termName” | “bellSound” | “wordSeparator”   The option key.
        value   string  The option value.
        Returns: void

        ▸ setOption(key: “fontWeight”   “fontWeightBold”, value: null   “normal”    “bold”  “100”   “200”   “300”   “400”   “500”   “600”   “700”   “800”   “900”): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “fontWeight” | “fontWeightBold”     The option key.
        value   null | “normal” | “bold” | “100” | “200” | “300” | “400” | “500” | “600” | “700” | “800” | “900”    The option value.
        Returns: void

        ▸ setOption(key: “logLevel”, value: LogLevel): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “logLevel”  The option key.
        value   LogLevel    The option value.
        Returns: void

        ▸ setOption(key: “bellStyle”, value: null   “none”  “visual”    “sound”     “both”): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “bellStyle”     The option key.
        value   null | “none” | “visual” | “sound” | “both”     The option value.
        Returns: void

        ▸ setOption(key: “cursorStyle”, value: null     “block”     “underline”     “bar”): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “cursorStyle”   The option key.
        value   null | “block” | “underline” | “bar”    The option value.
        Returns: void

        ▸ setOption(key: “allowTransparency”    “cancelEvents”  “convertEol”    “cursorBlink”   “disableStdin”  “macOptionIsMeta”   “popOnBell”     “rightClickSelectsWord”     “visualBell”    “windowsMode”, value: boolean): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “allowTransparency” | “cancelEvents” | “convertEol” | “cursorBlink” | “disableStdin” | “macOptionIsMeta” | “popOnBell” | “rightClickSelectsWord” | “visualBell” | “windowsMode”     The option key.
        value   boolean     The option value.
        Returns: void

        ▸ setOption(key: “fontSize”     “letterSpacing”     “lineHeight”    “tabStopWidth”  “scrollback”, value: number): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “fontSize” | “letterSpacing” | “lineHeight” | “tabStopWidth” | “scrollback”     The option key.
        value   number  The option value.
        Returns: void

        ▸ setOption(key: “theme”, value: ITheme): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “theme”     The option key.
        value   ITheme  The option value.
        Returns: void

        ▸ setOption(key: “cols”     “rows”, value: number): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     “cols” | “rows”     The option key.
        value   number  The option value.
        Returns: void

        ▸ setOption(key: string, value: any): void
        Sets an option on the terminal.
        Parameters:
        Name    Type    Description
        key     string  The option key.
        value   any     The option value.
        Returns: void
    */

    // `Option<&Closure<dyn FnMut()>>` can't be passed to JS functions, so we
    // have a version of write with the callback and one without it.

    /// Writes data to the terminal.
    ///
    /// Takes:
    ///   - `data`: The data to write to the terminal. The actual API allows for
    ///             this to be either raw bytes given as `Uint8Array` from the
    ///             pty or a string (raw bytes will always be treated as UTF-8
    ///             encoded, string data as UTF-16). For simplicity, we just
    ///             take a `String`; this shouldn't cause problems (going from
    ///             UTF-8 encoded Rust `String`s to UTF-16 JS strings) and just
    ///             makes things simpler.
    #[wasm_bindgen(method, js_name = write)]
    pub fn write(this: &Terminal, data: Str);

    /// Writes data to the terminal and takes a callback.
    ///
    /// This identical to [`write`] except it also takes a callback.
    ///
    /// Takes:
    ///   - `data`:    The data to write to the terminal. The actual API allows
    ///                for this to be either raw bytes given as `Uint8Array`
    ///                from the pty or a string (raw bytes will always be
    ///                treated as UTF-8 encoded, string data as UTF-16). For
    ///                simplicity, we just take a `String`; this shouldn't cause
    ///                problems (going from UTF-8 encoded Rust `String`s to
    ///                UTF-16 JS strings) and just makes things simpler.
    ///  - `callback`: Callback that fires when the data was processed by the
    ///                parser.
    ///
    /// [`write`]: Terminal::write
    #[wasm_bindgen(method, js_name = write)]
    pub fn write_with_callback(
        this: &Terminal,
        data: Str,
        callback: &Closure<dyn FnMut()>,
    );

// TODO: registerLinkProvider

// [TODO]
//   writeUtf8
//   ▸ writeUtf8(data: Uint8Array, callback?: function): void
//   Defined in xterm.d.ts:896
//   Write UTF8 data to the terminal.
//   deprecated use write instead
//   Parameters:
//   ▪ data: Uint8Array
//   The data to write to the terminal.
//   ▪Optional callback: function
//   Optional callback when data was processed.
//   ▸ (): void
//   Returns: void

// [TODO]
//   writeln
//   ▸ writeln(data: string  Uint8Array, callback?: function): void
//   Writes data to the terminal, followed by a break line character (\n).
//   Parameters:
//   ▪ data: *string     Uint8Array*
//   The data to write to the terminal. This can either be raw bytes given as Uint8Array from the pty or a string. Raw bytes will always be treated as UTF-8 encoded, string data as UTF-16.
//   ▪Optional callback: function
//   Optional callback that fires when the data was processed by the parser.
//   ▸ (): void
//   Returns: void

}
