#![cfg(feature = "ext")]

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsCast;
use xterm_js_sys::interface;

// This is an ugly hack...
pub trait IntoJsInterface<Interface: FromWasmAbi + IntoWasmAbi + JsCast> {
    fn to(self) -> Interface;
    fn by_ref(&self) -> Interface;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type Frobber;

    #[wasm_bindgen(structural, method)]
    pub fn frob(this: &Frobber, a: u64) -> u64;


    #[wasm_bindgen(structural, method)]
    pub fn quaz(this: &Frobber, a: u64, b: u16) -> u64;
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

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type Yap;

    #[wasm_bindgen(structural, method)]
    pub fn yap(this: &Yap) -> String;
}

interface! {
    pub trait RustYap mirrors Yap {
        fn yap(&self) -> String;
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Frobber, extends = Yap)]
    pub type Blabber;

    #[wasm_bindgen(structural, method)]
    pub fn blab(this: &Blabber, b: String);


    #[wasm_bindgen(structural, method)]
    pub fn shout(this: &Blabber, inp: String) -> String;
}

interface! {
    pub trait RustBlabber mirrors Blabber
    where
        Self extends Frobber as RustFrobber,
        Self extends Yap as RustYap,
    {
        fn blab(&self, b: String);
        fn shout(&self, inp: String) -> String;
    }
}

#[derive(Debug, Clone, Default)]
struct Blub;

impl RustFrobber for Blub {
    fn frob(&self, _a: u64) -> u64 { 12 }
    fn quaz(&self, a: u64, b: u16) -> u64 { self.frob(a) + (b as u64) }
}

impl RustYap for Blub {
    fn yap(&self) -> String { String::from("hello javascript!") }
}

impl RustBlabber for Blub {
    fn blab(&self, b: String) { console_log!("{}", b) }
    fn shout(&self, inp: String) -> String {
        inp.to_uppercase()
    }
}

#[wasm_bindgen_test]
fn interface_with_extends() {
    let blab: Blub = Blub;
    let blab: Blabber = blab.to();

    mod traitless {
        // No traits in scope!
        pub fn inner(blab: &super::Blabber) {
            // These don't need `as_ref` calls since `Blabber` `Deref`s into `Frobber`
            // (since it's the first extends).
            assert_eq!(blab.frob(0), 12);
            assert_eq!(blab.quaz(0, 4), 16);

            assert_eq!(AsRef::<super::Yap>::as_ref(blab).yap(), "hello javascript!".to_string());

            assert_eq!(blab.shout(AsRef::<super::Yap>::as_ref(blab).yap()), "HELLO JAVASCRIPT!".to_string());

            blab.blab(blab.shout(AsRef::<super::Yap>::as_ref(blab).yap()))
        }
    }

    traitless::inner(&blab)
}
