//! Glue for the Xterm.js types.

use super::xterm::{Disposable, Terminal, TerminalAddon, TerminalOptions};

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsCast;

// The `object` macro and the `IntoJsInterface` trait are good candidates for
// being spun off into their own crate, along with macros that generate some of
// the boilerplate needed to mirror over JS interfaces as Rust traits.

/// Supporting items for the macros in this module; re-exported here so we don't
/// have to make any assumptions about the call-site.
#[doc(hidden)]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub mod _macro_support {
    pub use core::clone::Clone;
    pub use core::convert::AsRef;
    pub use core::marker::Sized;
    pub use core::{concat, stringify};
    pub use std::boxed::Box;

    pub use js_sys::{Object, Reflect};
    pub use wasm_bindgen::prelude::Closure;
    pub use wasm_bindgen::{JsCast, JsValue};
}

/// Uses the workaround detailed [here] to let us 'generate' a doc literal.
/// [here]: https://github.com/rust-lang/rust/issues/52607
#[doc(hidden)]
#[macro_export]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
macro_rules! calculated_doc {
  ( $(#[doc = $doc:expr])* >>> $thing:item $(#[$metas:meta])* ) => {
      $(
      #[$metas]
      )*
      $(
      #[doc = $doc]
      )*
      $thing
  };
}

/// Creates a Rust trait to match a particular JS interface.
///
/// In addition to the actual trait, this produces:
///   - glue that lets `'static` instances of the Rust trait be "turned into"
///     instances of the JS interface
///   - a blanket impl that implements the Rust trait for all `wasm-bindgen`
///     produced types that extend the JS interface
///   - an implementation of [`IntoJsInterface`] for all things that implement
///     the Rust trait that's generated; this can be used to accept
///     implementations of the Rust or implementations of the JS interface
///     with `impl IntoJsInterface<JsInterfaceName>`
///
/// [`IntoJsInterface`]: crate::ext::IntoJsInterface
#[macro_export]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
macro_rules! interface {
    (
        $(#[$metas:meta])*
        $vis:vis trait $nom:ident
            mirrors $js_interface:ident
            $(where
                $(Self extends $ext_js:path as $ext_rs:path,)+
            )?
    {
        $(
            $(#[$fn_metas:meta])*
            // All functions that we can mirror need to take `&self` so this is
            // okay.
            fn $fn_name:ident (&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?)
                $(-> $ret_ty:ty)?
                ;
            // Default impls are not supported for now.

            // This is intentionally very constrained. The idea is that this
            // just mirrors the JS interface. if you want to offer additional
            // functionality on your Rust trait, use an extension trait.
        )*
    }) => {
        calculated_doc! {
            #[doc = $crate::ext::_macro_support::concat!(
                " Rust version of the ",
                "[`", $crate::ext::_macro_support::stringify!($js_interface), "`]",
                " interface.\n",
            )]
            #[doc = "\n"]
            #[doc = $crate::ext::_macro_support::concat!(
                " See the [\"mirroring interfaces\" section]",
                "(",
                    $crate::ext::_macro_support::stringify!(/*$*/crate),
                    "::xterm#mirroring-interfaces",
                ")",
                "\n of the `xterm` module docs for more information.",
            )]
            >>>
            $vis trait $nom
            $(where
                $(Self: $ext_rs,)+
            )?
            {
                $(
                    calculated_doc! {
                        #[doc = "\n"]
                        #[doc = $crate::ext::_macro_support::concat!(
                            " Dual of ",
                            "[`",
                                $crate::ext::_macro_support::stringify!($js_interface),
                                "::",
                                $crate::ext::_macro_support::stringify!($fn_name),
                            "`].",
                        )]
                        >>>
                        fn $fn_name(
                            &self,
                            $(
                                $arg_name: $arg_ty,
                            )*
                        ) $(-> $ret_ty)?;
                        $(#[$fn_metas])*
                    }
                )*

                //////////////// Internal Functions For Interface Mirroring ////////////////
                calculated_doc! {
                    #[doc = " Copy of [`IntoJsInterface::by_ref`].\n"]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_macro_support::concat!(
                        " [`IntoJsInterface::by_ref`]: ",
                        $crate::ext::_macro_support::stringify!(/*$*/crate),
                        "::ext::IntoJsInterface::by_ref"
                    )]
                    >>>
                    fn into_js_by_ref(&self) -> $js_interface
                    where
                        Self: $crate::ext::_macro_support::Clone + 'static,
                    {
                        $nom::into_js(self.clone())
                    }
                }

                calculated_doc! {
                    #[doc = " Copy of [`IntoJsInterface::to`].\n"]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_macro_support::concat!(
                        " [`IntoJsInterface::to`]: ",
                            $crate::ext::_macro_support::stringify!(/*$*/crate),
                        "::ext::IntoJsInterface::to",
                    )]
                    >>>
                    fn into_js(self) -> $js_interface
                    where
                        Self: $crate::ext::_macro_support::Sized + 'static,
                    {
                        use $crate::ext::_macro_support::{Box, JsCast};
                        let b = Box::leak(Box::new(self));
                        $nom::into_js_inner(b).unchecked_into()
                    }
                }

                calculated_doc! {
                    #[doc = $crate::ext::_macro_support::concat!(
                        " Internal version of [`into_js_by_ref`]",
                        "(",
                            $crate::ext::_macro_support::stringify!($nom),
                            "::into_js_by_ref",
                        ")",
                        " that doesn't\n leak `self`.\n",
                    )]
                    #[doc = "\n"]
                    #[doc = " Useful for trait/interface hierarchies."]
                    >>>
                    fn into_js_inner(&'static self) -> $crate::ext::_macro_support::Object
                    where
                        Self: 'static,
                    {
                        use $crate::ext::_macro_support::{Box, Closure, Object};
                        use $crate::ext::object;

                        let base = Object::new();

                        // The things we extend, first:
                        $($(
                            let base = Object::assign(
                                &base,
                                &<Self as $ext_rs>::into_js_inner(self)
                            );
                        )*)?

                        // Next, the functions of the interface:
                        struct Inner {
                            $($fn_name: Closure<dyn FnMut(
                                    $($arg_ty,)*
                                )>
                            ,)*
                        }

                        let Inner {
                            $($fn_name,)*
                        } = Inner {
                            $($fn_name: {
                                Closure::wrap(Box::new(move |$($arg_name: $arg_ty, )*| {
                                    Self::$fn_name(self $(, $arg_name)*)
                                }))
                            })*
                        };

                        let obj = object! { (base) += {
                            $($fn_name: $fn_name)*
                        }};

                        $(Closure::forget($fn_name);)*

                        obj
                    }
                }
            }

            $(#[$metas])*
        }

        calculated_doc! {
            #[doc = $crate::ext::_macro_support::concat!(
                " Anything that implements ",
                "[`",
                    $crate::ext::_macro_support::stringify!($nom),
                "`]",
                " (and is `Clone + 'static`) ",
                "gets an implementation \n ",
                " of ",
                "[`IntoJsInterface<",
                    $crate::ext::_macro_support::stringify!($js_interface),
                ">`]",
                "(",
                    $crate::ext::_macro_support::stringify!(/*$*/crate),
                "::ext::IntoJsInterface).",
            )]
            >>>
            impl<X> IntoJsInterface<$js_interface> for X
            where
                X: $nom,
                X: $crate::ext::_macro_support::Clone + 'static
            {
                calculated_doc! {
                    #[doc = $crate::ext::_macro_support::concat!(
                        " Converts the ",
                        "[`",
                            $crate::ext::_macro_support::stringify!($nom),
                        "`]",
                        " implementor into an instance of ",
                        "[`",
                            $crate::ext::_macro_support::stringify!($js_interface),
                        "`]\n ",
                        "(the corresponding JS interface).",
                    )]
                    >>>
                    fn to(self) -> $js_interface {
                        $nom::into_js(self)
                    }
                }

                calculated_doc! {
                    #[doc = $crate::ext::_macro_support::concat!(
                        " Converts the ",
                        "[`",
                            $crate::ext::_macro_support::stringify!($nom),
                        "`]",
                        " implementor into an instance of ",
                        "[`",
                        $crate::ext::_macro_support::stringify!($js_interface),
                        "`]\n ",
                        "(the corresponding JS interface) _by reference_.",
                    )]
                    >>>
                    fn by_ref(&self) -> $js_interface {
                        $nom::into_js_by_ref(self)
                    }
                }
            }
        }

        calculated_doc! {
            #[doc = $crate::ext::_macro_support::concat!(
                " This provides an impl of the ",
                "[`",
                    $crate::ext::_macro_support::stringify!($nom),
                "`]",
                " Rust trait for all things that 'implement'\n ",
                "the ",
                "[`",
                $crate::ext::_macro_support::stringify!($js_interface),
                "`]",
                " JS interface the `wasm-bindgen` way.\n",
            )]
            #[doc = "\n"]
            #[doc = $crate::ext::_macro_support::concat!(
                " See the [\"mirroring interfaces\" section]",
                "(",
                $crate::ext::_macro_support::stringify!(/*$*/crate),
                "::xterm#mirroring-interfaces",
                ")",
                " of the\n ",
                "`xterm` module docs for more information.",
            )]
            >>>
            impl<X> $nom for X
            where
                X: $crate::ext::_macro_support::Clone + 'static,
                $($(X: $crate::ext::_macro_support::AsRef<$ext_js>,)*)?
                X: AsRef<$js_interface>,
            {
                $(
                    calculated_doc! {
                        #[doc = $crate::ext::_macro_support::concat!(
                            " [`",
                                $crate::ext::_macro_support::stringify!($fn_name),
                            "`](",
                                $crate::ext::_macro_support::stringify!($nom),
                                "::",
                                $crate::ext::_macro_support::stringify!($fn_name),
                            ")",
                            " for types that implement the ",
                            "[`",
                                $crate::ext::_macro_support::stringify!($js_interface),
                            "`]",
                            " interface.",
                        )]
                        >>>
                        fn $fn_name(&self $(, $arg_name: $arg_ty)*) $(-> $ret_ty)? {
                            $js_interface::$fn_name(
                                $crate::ext::_macro_support::AsRef::<$js_interface>::as_ref(self),
                                $($arg_name,)*
                            )
                        }
                    }
                )*

                calculated_doc! {
                    #[doc = $crate::ext::_macro_support::concat!(
                        " [`into_js_by_ref`](",
                            $crate::ext::_macro_support::stringify!($nom),
                        "::into_js_by_ref)",
                        " for types that implement the\n ",
                        "[`",
                            $crate::ext::_macro_support::stringify!($js_interface),
                        "`]",
                        " interface.\n",
                    )]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_macro_support::concat!(
                        " This differs from the default impl in that it",
                        " manages to avoid a `Clone` before effectively\n",
                        " doing what ",
                        "[`into_js`](",
                        $crate::ext::_macro_support::stringify!($nom),
                        "::into_js) does.",
                    )]
                    >>>
                    fn into_js_by_ref(&self) -> $js_interface {
                        use $crate::ext::_macro_support::{AsRef, Clone};

                        AsRef::<$js_interface>::as_ref(self).clone()
                    }
                }

                calculated_doc! {
                    #[doc = $crate::ext::_macro_support::concat!(
                        " [`into_js`](",
                            $crate::ext::_macro_support::stringify!($nom),
                        "::into_js)",
                        " for types that implement the\n ",
                        "[`",
                            $crate::ext::_macro_support::stringify!($js_interface),
                        "`]",
                        " interface.\n",
                    )]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_macro_support::concat!(
                        " This differs from the default impl in that it",
                        " manages to avoid \"double wrapping\" the methods\n",
                        " in the interface (types that impl ",
                        "[`",
                            $crate::ext::_macro_support::stringify!($js_interface),
                        "`]",
                        " the `wasm-bindgen` way already have\n",
                        " a wrapped up",
                        " [`Object`](",
                            $crate::ext::_macro_support::stringify!(/*$*/crate),
                        "::ext::_macro_support::Object)",
                        " they can hand us).",
                    )]
                    >>>
                    fn into_js(self) -> $js_interface {
                        use $crate::ext::_macro_support::{AsRef, Clone};

                        AsRef::<$js_interface>::as_ref(&self).clone()
                    }
                }
            }
        }
    };
}

/// Defines a JS object with some properties.
#[macro_export]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
macro_rules! object {
    (
        $($f:ident: $v:expr),* $(,)?
    ) => {{
        let obj = $crate::ext::_macro_support::Object::new();

        $crate::ext::object! { obj += {
                $($f: $v),*
        }}

        obj
    }};

    (($base:expr) += {
        $($f:ident: $v:expr),* $(,)?
    }) => {{
        let obj = $base;

        $crate::ext::object! { obj += {
            $($f: $v),*
        }}

        obj
    }};

    ($nom:ident += {
        $($f:ident: $v:expr),* $(,)?
    }) => {{$(
        let _ = $crate::ext::_macro_support::Reflect::set(
            &$nom,
            &$crate::ext::_macro_support::JsValue::from_str(
                $crate::ext::_macro_support::stringify!($f)
            ),
            ($v).as_ref(),
        ).unwrap();

        ()
    )*}};
}

/// Represents a Rust type that satisfies a JS interface and can be turned into
/// the concrete type that represents the JS interface.
///
/// See the [`disposable`] module for an example.
///
/// As mentioned in the [`xterm` module docs](crate::xterm#mirroring-interfaces)
/// we make a Rust trait dual for each JS interface (or the ones we want to make
/// instances of, anyways). Ideally we'd be able to do something like this:
/// `trait RustTrait: IntoJsInterface<JsTrait>`. The problem with that is that
/// the impl of `IntoJsInterface` _requires_ the impl of `RustTrait`; we need
/// the functions that satisfy the interface to actually make the instance of
/// the interface type.
///
/// So, instead we do the weird backwards thing that we do in [`disposable`]
/// where the Rust trait (i.e. [`Disposable`](disposable::Disposable)) ends up
/// effectively having these same functions and _then_ providing a blanket impl
/// so that [`IntoJsInterface`] is impled for all things that impl the Rust
/// trait.
///
/// It's unfortunate that we don't really have a way to encode that each Rust
/// trait can have one (and only one) interface type dual. We encode this within
/// the trait itself, but we can't seem to do this in a way that's generic over
/// traits (not until we get HKTs anyways).
///
/// So it's unclear exactly where this trait would be useful. I guess it lets
/// you be generic over the interface you're asking for? Something like this:
/// ```rust
/// # use wasm_bindgen::{convert::{FromWasmAbi, IntoWasmAbi}, JsCast};
/// # use xterm_js_sys::ext::IntoJsInterface;
/// # #[allow(dead_code)]
/// pub fn foo<I>(inst: impl IntoJsInterface<I>)
/// where
///     I: FromWasmAbi + IntoWasmAbi + JsCast,
/// {
///    inst.to();
/// }
/// ```
///
/// Combined with `AsRef` you can do things like accept Rust implementations
/// of interfaces that subclass some base class:
/// ```rust
/// # use wasm_bindgen::{convert::{FromWasmAbi, IntoWasmAbi}, JsCast};
/// # use xterm_js_sys::ext::IntoJsInterface;
/// # #[allow(dead_code)]
/// pub fn bar<I>(inst: impl IntoJsInterface<I>)
/// where
///     I: FromWasmAbi + IntoWasmAbi + JsCast,
///     I: AsRef<js_sys::Iterator>,
/// {
///    inst.to();
/// }
/// ```
///
/// But it's still unclear if/how this is useful.
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub trait IntoJsInterface<Interface: FromWasmAbi + IntoWasmAbi + JsCast> {
    /// Convert to an instance of the JS interface type.
    fn to(self) -> Interface;

    /// Produce an instance of the JS interface type without consuming the Rust
    /// instance.
    ///
    /// For Rust impls of a trait this will probably require `Self` to implement
    /// `Clone` since as part of creating the instance the instance needs to be
    /// leaked (for methods to still work), but we'll leave that up to
    /// implementors.
    fn by_ref(&self) -> Interface;
}

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
#[doc(inline)]
pub use super::{interface, object};

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub mod disposable;
pub use disposable::*;

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub mod event;
pub use event::*;

#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub mod addon;
pub use addon::*;
