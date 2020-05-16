#![cfg_attr(docs, feature(doc_cfg))]
#![cfg_attr(docs, feature(external_doc))]

extern crate wasm_bindgen;

pub mod xterm;

#[cfg(feature = "ext")]
pub mod ext;

#[cfg(feature = "tui-backend")]
pub mod tui;
