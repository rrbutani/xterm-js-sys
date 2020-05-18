//! Glue for the Xterm.js types.

use super::xterm::Disposable;

use wasm_bindgen::prelude::*;

pub mod disposable;
pub use disposable::*;

pub mod event;
pub use event::*;
