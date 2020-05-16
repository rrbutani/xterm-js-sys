#![cfg_attr(docs, feature(doc_cfg))]
#![cfg_attr(docs, feature(external_doc))]


// TODO:
//  - colours for badges in the README
//  - deny the warnings here
//  - add in an example/crate level docs here
//  - add in the attr for marking feature specific things in docs

pub mod xterm;

#[cfg(feature = "ext")]
pub mod ext;

#[cfg(feature = "tui-backend")]
pub mod tui;
