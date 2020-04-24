extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod xterm;

#[wasm_bindgen]
pub fn foo(x: &str) -> String {
  if x == "abc" {
    "yes".to_string()
  } else {
    "no".to_string()
  }
}

