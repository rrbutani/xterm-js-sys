#![deny(intra_doc_link_resolution_failure)]

extern crate wasm_bindgen;

pub mod xterm;

#[cfg(feature = "ext")]
pub mod ext;
