#![cfg(feature = "ext")]

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsCast;
use xterm_js_sys::interface;


#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type Frobber;

    #[wasm_bindgen(structural, method)]
    pub fn frob(this: &Frobber, a: u64) -> u64;


    #[wasm_bindgen(structural, method)]
    pub fn quaz(this: &Frobber, a: u64, b: u16) -> u64;
}

// This is an ugly hack...
pub trait IntoJsInterface<Interface: FromWasmAbi + IntoWasmAbi + JsCast> {
    fn to(self) -> Interface;
    fn by_ref(&self) -> Interface;
}

interface! {
    pub trait RustFrobber mirrors Frobber {
        fn frob(&self, a: u64) -> u64;
        fn quaz(&self, a: u64, b: u16) -> u64;
    }
}

#[derive(Debug, Clone, Default)]
struct FooFrob(u64);

impl RustFrobber for FooFrob {
    fn frob(&self, a: u64) -> u64 {
        self.0 + a * 2
    }

    fn quaz(&self, a: u64, b: u16) -> u64 {
        self.frob(a) + (b as u64)
    }
}

fn use_a_frob<F: IntoJsInterface<Frobber>>(
    f: F,
    a: u64,
    b: u16,
    frob_val: u64,
    quaz_val: u64,
) {
    let f: Frobber = f.to();

    assert_eq!(f.frob(a), frob_val);
    assert_eq!(f.quaz(a, b), quaz_val);
}

#[wasm_bindgen_test]
fn simple_interface() {
    use_a_frob(FooFrob(0), 2, 3, 4, 7);
    use_a_frob(FooFrob(7), 1, 2, 9, 11);
}

#[wasm_bindgen_test]
fn interface_with_extends() {
    // assert!(false);
}
