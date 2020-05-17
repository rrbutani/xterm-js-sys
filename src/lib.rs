#![cfg_attr(docs, feature(doc_cfg))]
#![cfg_attr(docs, feature(external_doc))]
#![cfg_attr(docs, doc(include = "../README.md"))]
//!
// ^ is there so it looks like we have at some crate level docs when building
// without `--cfg docs` (i.e. on stable, when not building docs).

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
    rust_2018_idioms
)]
#![doc(test(attr(deny(rust_2018_idioms, warnings))))]
#![doc(
    html_logo_url = "https://avatars2.githubusercontent.com/u/11927490?s=800&v=5"
)]

// TODO:
//  - colours for badges in the README
//  - deny the warnings here
//  - add in an example/crate level docs here
//  - add in the attr for marking feature specific things in docs

pub mod xterm;

#[cfg(feature = "ext")]
#[cfg_attr(docs, doc(cfg(feature = "ext")))]
pub mod ext;

#[cfg(feature = "tui-backend")]
#[cfg_attr(docs, doc(cfg(feature = "tui-backend")))]
pub mod tui;
