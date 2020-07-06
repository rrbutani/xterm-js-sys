#![cfg_attr(all(docs, not(doctest)), feature(doc_cfg))]
#![cfg_attr(all(docs, not(doctest)), feature(external_doc))]
#![cfg_attr(all(docs, not(doctest)), doc(include = "../README.md"))]
//!
// ^ is there so it looks like we have at some crate level docs when building
// without `--cfg docs` (i.e. on stable, when not building docs).

//! ## A Live Demo
//! <iframe
//!     title="Terminal"
//!     src="https://rrbutani.github.io/xterm-js-sys/examples/with-input/index.html"
//!     height="600"
//!     width="100%"
//!     frameborder="0"
//! ></iframe>

#![forbid(
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_lifetimes,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![deny(
    unused,
    bad_style,
    missing_debug_implementations,
    intra_doc_link_resolution_failure,
    missing_docs,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    rust_2018_idioms,
    variant_size_differences
)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(clippy::type_repetition_in_bounds)]
#![doc(test(attr(
    deny(rust_2018_idioms, warnings),
    allow(unused_extern_crates)
)))]
#![doc(
    html_logo_url = "https://avatars2.githubusercontent.com/u/11927490?s=800&v=5",
    html_root_url = "https://docs.rs/xterm-js-sys/*", // * → latest version
)]

// TODO:
//  - add in an example/crate level docs here

/// Converts an `i32` into an `Option<u32>` (following the JS convention where
/// -1 indicates an error/lack of an element).
#[allow(clippy::cast_sign_loss)]
fn idx_to_opt(idx: i32) -> Option<u32> {
    match idx {
        -1 => None,
        0..=i32::MAX => Some(idx as u32),
        _ => unreachable!(),
    }
}

mod readonly_array;
pub use readonly_array::ReadOnlyArray;

pub mod xterm;
pub use xterm::Terminal;

#[cfg(feature = "crossterm-support")]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "crossterm-support")))]
pub mod crossterm_support;

#[cfg(feature = "ext")]
#[cfg_attr(all(docs, not(doctest)), doc(cfg(feature = "ext")))]
pub mod ext;
