//! [A type][ROA] that emulates [TypeScript's `ReadOnlyArray` interface][TS].
//!
//! Unlike the [`js-sys`](js_sys) [`Array` type](js_sys::Array), this type does
//! not erase the type of the array.
//!
//! If there's interest, this is a good candidate for being spun off into its
//! own crate.
//!
//! [ROA]: ReadOnlyArray
//! [TS]: https://www.typescriptlang.org/docs/handbook/interfaces.html#readonly-properties

use super::idx_to_opt;

use js_sys::{Array, JsString, Object};
use wasm_bindgen::{
    convert::{
        FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi,
        RefFromWasmAbi,
    },
    describe::WasmDescribe,
    prelude::*,
    JsCast,
};

use core::convert::{identity as id, AsRef};
use core::fmt::Debug;
use core::marker::PhantomData;
use core::mem::{transmute, ManuallyDrop};
use core::ops::Deref;

/// Mirrors [TypeScript's `ReadOnlyArray` interface][TS].
///
/// This is built on the [`Array` type in `js-sys`][`Array`] and primarily just
/// mirrors over the [subset][interface-docs] of regular array functions that
/// `ReadOnlyArray` provides. This wrapper also offers typed versions of all the
/// functions that it forwards (prefixed with `typed_`).
///
/// Because `wasm-bindgen` doesn't support generic structs, this type uses
/// `#[repr(transparent)]` and leans on its inner [`Array`] for all the
/// `wasm-bindgen` specific impls.
///
/// Note that this struct does **not** provide an [`AsRef`] impl for [`Array`]
/// as that would defeat the "read only" part of this type's guarantees.
///
/// [TS]: https://www.typescriptlang.org/docs/handbook/interfaces.html#readonly-properties
/// [`Array`]: js_sys::Array
/// [interface-docs]: https://microsoft.github.io/PowerBI-JavaScript/interfaces/_node_modules_typedoc_node_modules_typescript_lib_lib_es5_d_.readonlyarray.html
/// [`Index`]: core::ops::Index
/// [`AsRef`]: core::convert::AsRef
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct ReadOnlyArray<T: JsCast> {
    /// The `Array` we're backed by.
    inner: Array,
    /// Marker for `T`.
    _t: PhantomData<T>,
}

impl<T: JsCast> From<Array> for ReadOnlyArray<T> {
    fn from(inner: Array) -> Self {
        // Note that we don't check that `inner` actually contains elements that
        // can successfully be cast into `T` here and this is fine.
        Self {
            inner,
            _t: PhantomData,
        }
    }
}

////////////////////////////// Forwarded methods. //////////////////////////////

macro_rules! forward {
    ($(
        $(#[$m:meta])*
        $f:ident: (&$s:ident$(, $arg:ident: $t:ty)* $(,)?) $(-> $ret:ty)?
    )+) => {$(
        $(#[$m])*
        pub fn $f(&$s $(, $arg: $t)*) $(-> $ret)? {
            $s.inner.$f($($arg),*)
        }
    )+};
}

// These docs are copied nearly verbatim from the [js-sys Array docs][arr].
//
// [arr]: https://docs.rs/js-sys/0.3.36/src/js_sys/lib.rs.html#128
impl<T: JsCast> ReadOnlyArray<T> {
    forward! {
        /// Retrieves the element at the index (returns `undefined` if the index
        /// is out of range).
        #[must_use]
        get: (&self, index: u32) -> JsValue

        /// The `concat()` method is used to merge two or more arrays. This
        /// method does not change the existing arrays, but instead returns a
        /// new array.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat)
        #[must_use]
        concat: (&self, array: &Array) -> Array

        /// The `every()` method tests whether all elements in the array pass
        /// the test implemented by the provided function.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every)
        #[must_use]
        every: (
            &self,
            predicate: &mut dyn FnMut(JsValue, u32, Array) -> bool,
        ) -> bool

        /// The `filter()` method creates a new array with all elements that
        /// pass the test implemented by the provided function.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter)
        #[must_use]
        filter: (
            &self,
            predicate: &mut dyn FnMut(JsValue, u32, Array) -> bool,
        ) -> Array

        /// The `find()` method returns the value of the first element in the
        /// array that satisfies the provided testing function. Otherwise
        /// `undefined` is returned.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find)
        #[must_use]
        find: (
            &self,
            predicate: &mut dyn FnMut(JsValue, u32, Array) -> bool
        ) -> JsValue

        /// The `findIndex()` method returns the index of the first element in
        /// the array that satisfies the provided testing function. Otherwise -1
        /// is returned.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex)
        #[must_use]
        find_index: (
            &self,
            predicate: &mut dyn FnMut(JsValue, u32, Array) -> bool
        ) -> i32

        /// The `flat()` method creates a new array with all sub-array elements
        /// concatenated into it recursively up to the specified depth.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat)
        #[must_use]
        flat: (&self, depth: i32) -> Array

        /// The `flatMap()` method first maps each element using a mapping
        /// function, then flattens the result into a new array.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap)
        #[must_use]
        flat_map: (
            &self,
            callback: &mut dyn FnMut(JsValue, u32, Array) -> Vec<JsValue>,
        ) -> Array

        /// The `forEach()` method executes a provided function once for each
        /// array element.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach)
        for_each: (&self, callback: &mut dyn FnMut(JsValue, u32, Array))

        /// The `includes()` method determines whether an array includes a
        /// certain element, returning true or false as appropriate.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes)
        #[must_use]
        includes: (&self, value: &JsValue, from_index: i32) -> bool

        /// The `indexOf()` method returns the first index at which a given
        /// element can be found in the array, or -1 if it is not present.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf)
        #[must_use]
        index_of: (&self, value: &JsValue, from_index: i32) -> i32

        /// The `join()` method joins all elements of an array (or an array-like
        /// object) into a string and returns this string.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join)
        #[must_use]
        join: (&self, delimiter: &str) -> JsString

        /// The `lastIndexOf()` method returns the last index at which a given
        /// element can be found in the array, or -1 if it is not present. The
        /// array is searched backwards, starting at fromIndex.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf)
        #[must_use]
        last_index_of: (&self, value: &JsValue, from_index: i32) -> i32

        /// The length property of an object which is an instance of type Array
        /// sets or returns the number of elements in that array. The value is
        /// an unsigned, 32-bit integer that is always numerically greater than
        /// the highest index in the array.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/length)
        #[must_use]
        length: (&self) -> u32

        /// `map()` calls a provided callback function once for each element in
        /// an array, in order, and constructs a new array from the results.
        /// callback is invoked only for indexes of the array which have
        /// assigned values, including undefined. It is not called for missing
        /// elements of the array (that is, indexes that have never been set,
        /// which have been deleted or which have never been assigned a value).
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map)
        #[must_use]
        map: (
            &self,
            predicate: &mut dyn FnMut(JsValue, u32, Array) -> JsValue,
        ) -> Array

        /// The `reduce()` method applies a function against an accumulator and
        /// each element in the array (from left to right) to reduce it to a
        /// single value.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Reduce)
        #[must_use]
        reduce: (
            &self,
            predicate: &mut dyn FnMut(JsValue, JsValue, u32, Array) -> JsValue,
            initial_value: &JsValue,
        ) -> JsValue

        /// The `reduceRight()` method applies a function against an accumulator
        /// and each value of the array (from right-to-left) to reduce it to a
        /// single value.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/ReduceRight)
        #[must_use]
        reduce_right: (
            &self,
            predicate: &mut dyn FnMut(JsValue, JsValue, u32, Array) -> JsValue,
            initial_value: &JsValue,
        ) -> JsValue

        /// The `slice()` method returns a shallow copy of a portion of an array
        /// into a new array object selected from begin to end (end not
        /// included). The original array will not be modified.
        ///
        /// Note that because a copy is returned, the fact that an [`Array`] is
        /// produced from a [`ReadOnlyArray`] is not a problem.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice)
        #[must_use]
        slice: (&self, start: u32, end: u32) -> Array

        /// The `some()` method tests whether at least one element in the array
        /// passes the test implemented by the provided function.
        ///
        /// Note: This method returns false for any condition put on an empty
        /// array.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some)
        #[must_use]
        some: (&self, predicate: &mut dyn FnMut(JsValue) -> bool) -> bool

        /// The `toLocaleString()` method returns a string representing the
        /// elements of the array. The elements are converted to Strings using
        /// their toLocaleString methods and these Strings are separated by a
        /// locale-specific String (such as a comma “,”).
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString)
        #[must_use]
        to_locale_string: (
            &self,
            locales: &JsValue,
            options: &JsValue,
        ) -> JsString

        /// The `toString()` method returns a string representing the specified
        /// array and its elements.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toString)
        #[must_use]
        to_string: (&self) -> JsString

        /// Returns an iterator over the values of the JS array.
        #[must_use]
        iter: (&self) -> js_sys::ArrayIter<'_>

        /// Converts the JS array into a new `Vec`.
        #[must_use]
        to_vec: (&self) -> Vec<JsValue>

        /// The `keys()` method returns a new Array Iterator object that
        /// contains the keys for each index in the array.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/keys)
        #[must_use]
        keys: (&self) -> js_sys::Iterator

        /// The `entries()` method returns a new Array Iterator object that
        /// contains the key/value pairs for each index in the array.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/entries)
        #[must_use]
        entries: (&self) -> js_sys::Iterator

        /// The `values()` method returns a new Array Iterator object that
        /// contains the values for each index in the array.
        ///
        /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/values)
        #[must_use]
        values: (&self) -> js_sys::Iterator
    }

    /// The `Array.isArray()` method determines whether the passed value is an
    /// Array.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray)
    #[must_use]
    pub fn is_array(value: &JsValue) -> bool {
        Array::is_array(value)
    }
}

/// Wraps an array iter function so that it only sees actual types instead of
/// [`JsValue`].
fn typify<T: JsCast, R, R2: Clone>(
    on_type_mismatch: R2,
    mut func: impl FnMut(T, u32, ReadOnlyArray<T>) -> R,
    convert: impl Fn(R) -> R2,
) -> impl FnMut(JsValue, u32, Array) -> R2 {
    move |val, idx, arr| {
        if let Ok(v) = val.dyn_into() {
            convert(func(v, idx, arr.into()))
        } else {
            on_type_mismatch.clone()
        }
    }
}

// Typechecked versions of the above:
impl<T: JsCast> ReadOnlyArray<T> {
    /// Typed version of [`Array::get`].
    ///
    /// Returns `None` when the element can't be casted into `T` or the index is
    /// out of bounds.
    #[must_use]
    pub fn typed_get(&self, index: u32) -> Option<T> {
        self.inner.get(index).dyn_into().ok()
    }

    /// Typed version of [`Array::concat`].
    #[must_use]
    pub fn typed_concat(&self, array: &Self) -> Self {
        self.inner.concat(&array.inner).into()
    }

    /// Typed version of [`Array::every`].
    ///
    /// Returns `false` is any of the elements can't be casted into `T`.
    pub fn typed_every(
        &self,
        predicate: impl FnMut(T, u32, Self) -> bool,
    ) -> bool {
        // If we can't cast into `T`, return false.
        self.inner.every(&mut typify(false, predicate, id))
    }

    /// Typed version of [`Array::filter`].
    ///
    /// Elements that can't be casted into `T` are automatically filtered out.
    #[must_use]
    pub fn typed_filter(
        &self,
        predicate: impl FnMut(T, u32, Self) -> bool,
    ) -> Self {
        // If we can't cast into `T`, return false (i.e. filter the element
        // out).
        self.inner.filter(&mut typify(false, predicate, id)).into()
    }

    /// Typed version of [`Array::find`].
    ///
    /// Elements that can't be casted into `T` are automatically ignored.
    #[must_use]
    pub fn typed_find(
        &self,
        predicate: impl FnMut(T, u32, Self) -> bool,
    ) -> Option<T> {
        // If we can't cast into `T`, return false (it's not what we're looking
        // for).
        self.inner
            .find(&mut typify(false, predicate, id))
            .dyn_into()
            .ok()
    }

    /// Typed version of [`Array::find_index`].
    ///
    /// Elements that can't be casted into `T` are automatically ignored.
    ///
    /// Returns `None` when the element can't be found.
    #[must_use]
    pub fn typed_find_index(
        &self,
        predicate: impl FnMut(T, u32, Self) -> bool,
    ) -> Option<u32> {
        let idx = self.inner.find_index(&mut typify(false, predicate, id));

        idx_to_opt(idx)
    }

    /// Typed version of [`Array::flat`].
    #[must_use]
    pub fn typed_flat(&self, depth: i32) -> Self {
        self.inner.flat(depth).into()
    }

    /// Typed version of [`Array::flat_map`].
    ///
    /// Elements that can't be casted into `T` are automatically ignored.
    #[must_use]
    pub fn typed_flat_map(
        &self,
        callback: impl FnMut(T, u32, Self) -> Vec<T>,
    ) -> Self {
        self.inner
            .flat_map(&mut typify(
                Vec::<JsValue>::new(),
                callback,
                |v: Vec<T>| v.iter().map(Into::into).collect(),
            ))
            .into()
    }

    /// Typed version of [`Array::for_each`].
    ///
    /// Elements that can't be casted into `T` are automatically ignored.
    pub fn typed_for_each(&self, callback: impl FnMut(T, u32, Self)) {
        // Skip elements that can't be cast into `T`.
        self.inner.for_each(&mut typify((), callback, id))
    }

    /// Typed version of [`Array::includes`].
    ///
    /// Note that this takes the element to search for by value (we have no way
    /// to convert from `&T` to `&JsValue`); if `T` impls `AsRef<JsValue>`,
    /// you're better off using [`ReadOnlyArray::includes`]; it will be cheaper
    /// (but possibly less ergonomic).
    pub fn typed_includes(&self, value: T, from_index: i32) -> bool {
        self.inner.includes(&value.into(), from_index)
    }

    /// Typed version of [`Array::index_of`].
    ///
    /// Note that this takes the element to search for by value (we have no way
    /// to convert from `&T` to `&JsValue`); if `T` impls `AsRef<JsValue>`,
    /// you're better off using [`ReadOnlyArray::index_of`]; it will be cheaper
    /// (but possibly less ergonomic).
    ///
    /// Returns `None` when the element can't be found.
    pub fn typed_index_of(&self, value: T, from_index: i32) -> Option<u32> {
        let idx = self.inner.index_of(&value.into(), from_index);

        idx_to_opt(idx)
    }

    // Nothing to typify for `Array::join`.

    /// Typed version of [`Array::last_index_of`].
    ///
    /// Note that this takes the element to search for by value (we have no way
    /// to convert from `&T` to `&JsValue`); if `T` impls `AsRef<JsValue>`,
    /// you're better off using [`ReadOnlyArray::last_index_of`]; it will be
    /// cheaper (but possibly less ergonomic).
    ///
    /// Returns `None` when the element can't be found.
    pub fn typed_last_index_of(
        &self,
        value: T,
        from_index: i32,
    ) -> Option<u32> {
        let idx = self.inner.last_index_of(&value.into(), from_index);

        idx_to_opt(idx)
    }

    // Nothing to typify for `Array::length`.

    /// Typed version of [`Array::map`].
    ///
    /// Elements that can't be cast into `T` are mapped to
    /// [`JsValue::UNDEFINED`].
    #[must_use]
    pub fn typed_map<R: JsCast>(
        &self,
        predicate: impl FnMut(T, u32, Self) -> R,
    ) -> ReadOnlyArray<R> {
        self.inner
            .map(&mut typify(JsValue::UNDEFINED, predicate, Into::into))
            .into()
    }

    /// Typed version of [`Array::reduce`].
    ///
    /// Effectively 'skips' elements that can't be cast to `T` (just returns the
    /// accumulator as is for those elements).
    #[must_use]
    pub fn typed_reduce<A: JsCast>(
        &self,
        mut predicate: impl FnMut(A, T, u32, Self) -> A,
        initial_value: A,
    ) -> A {
        self.inner
            .reduce(
                &mut move |acc: JsValue, val: JsValue, idx, arr: Array| {
                    let acc = acc.dyn_into().unwrap();

                    let acc = if let Ok(val) = val.dyn_into() {
                        predicate(acc, val, idx, arr.into())
                    } else {
                        acc
                    };

                    acc.into()
                },
                &initial_value.into(),
            )
            .dyn_into()
            .unwrap()
    }

    /// Typed version of [`Array::reduce_right`].
    ///
    /// Effectively 'skips' elements that can't be cast to `T` (just returns the
    /// accumulator as is for those elements).
    #[must_use]
    pub fn typed_reduce_right<A: JsCast>(
        &self,
        mut predicate: impl FnMut(A, T, u32, Self) -> A,
        initial_value: A,
    ) -> A {
        // Exact same inner function as `typed_reduce`.
        self.inner
            .reduce_right(
                &mut move |acc, val, idx, arr| {
                    let acc = acc.dyn_into().unwrap();

                    let acc = if let Ok(val) = val.dyn_into() {
                        predicate(acc, val, idx, arr.into())
                    } else {
                        acc
                    };

                    acc.into()
                },
                &initial_value.into(),
            )
            .dyn_into()
            .unwrap()
    }

    /// Typed version of [`Array::slice`].
    #[must_use]
    pub fn typed_slice(&self, start: u32, end: u32) -> Self {
        self.inner.slice(start, end).into()
    }

    /// Typed version of [`Array::some`].
    ///
    /// Elements that can't be casted into `T` are effectively ignored (`false`
    /// is returned).
    #[must_use]
    pub fn typed_some(&self, mut predicate: impl FnMut(T) -> bool) -> bool {
        self.inner.some(&mut move |val: JsValue| {
            val.dyn_into().map(|t| predicate(t)).unwrap_or(false)
        })
    }

    // Nothing to typify for `Array::to_locale_string`.

    // Nothing to typify for `Array::to_string`.

    /// Typed version of [`Array::iter`].
    ///
    /// Elements that can't be casted into `T` are omitted.
    pub fn typed_iter<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.inner
            .iter()
            .filter_map(|val: JsValue| val.dyn_into().ok())
    }

    /// Typed version of [`Array::to_vec`].
    ///
    /// Elements that can't be casted into `T` are omitted.
    #[must_use]
    pub fn typed_to_vec(&self) -> Vec<T> {
        self.typed_iter().collect()
    }

    /// Typed version of [`Array::keys`].
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn typed_keys(&self) -> impl Iterator<Item = u32> {
        self.inner
            .keys()
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(|val: JsValue| val.as_f64().map(|f| f as u32))
    }

    /// Typed version of [`Array::entries`].
    ///
    /// Returns `Err(JsValue)`s for elements that could not be casted as `T` and
    /// elements that would have caused `TypeError`s (i.e. non-objects). See
    /// [`js_sys::Iterator::next`] for more details about the latter case.
    pub fn typed_entries(
        &self,
    ) -> impl Iterator<Item = (u32, Result<T, JsValue>)> {
        self.typed_keys().zip(self.typed_values())
    }

    /// Typed version of [`Array::values`].
    ///
    /// Returns `Err(JsValue)`s for elements that could not be casted as `T` and
    /// elements that would have caused `TypeError`s (i.e. non-objects). See
    /// [`js_sys::Iterator::next`] for more details about the latter case.
    pub fn typed_values(&self) -> impl Iterator<Item = Result<T, JsValue>> {
        self.inner
            .values()
            .into_iter()
            .map(|v| v.and_then(JsCast::dyn_into))
    }
}

// Extra methods
impl<T: JsCast> ReadOnlyArray<T> {
    /// Produces a copy of the array that is mutable (i.e. an [`Array`]).
    #[must_use]
    pub fn mutable_array_copy(&self) -> Array {
        self.inner.clone()
    }

    /// Produces a copy of the array that excludes any elements that can't be
    /// cast as `T`.
    ///
    /// Indexing into this array (for valid indexes), for example, should always
    /// produce `Some(_)`s.
    #[must_use]
    pub fn excluding_other_types(&self) -> Self {
        self.typed_filter(|_, _, _| true)
    }
}

/////////////// The impls `wasm-bindgen` would normally provide. ///////////////
impl<T: JsCast> AsRef<ReadOnlyArray<T>> for ReadOnlyArray<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: JsCast> AsRef<JsValue> for ReadOnlyArray<T> {
    fn as_ref(&self) -> &JsValue {
        self.inner.as_ref()
    }
}

impl<T: JsCast> AsRef<Object> for ReadOnlyArray<T> {
    fn as_ref(&self) -> &Object {
        self.inner.as_ref()
    }
}

impl<T: JsCast> Clone for ReadOnlyArray<T> {
    fn clone(&self) -> Self {
        self.inner.clone().into()
    }
}

impl<T: JsCast> Debug for ReadOnlyArray<T> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(fmt, "ReadOnlyArray<{}> = ", core::any::type_name::<T>())?;
        self.inner.fmt(fmt)
    }
}

impl<T: JsCast> Deref for ReadOnlyArray<T> {
    type Target = Object;

    fn deref(&self) -> &Object {
        &*self.inner
    }
}

impl<T: JsCast> From<ReadOnlyArray<T>> for JsValue {
    fn from(arr: ReadOnlyArray<T>) -> Self {
        arr.inner.into()
    }
}

impl<T: JsCast> From<ReadOnlyArray<T>> for Object {
    fn from(arr: ReadOnlyArray<T>) -> Self {
        arr.inner.into()
    }
}

impl<T: JsCast> From<JsValue> for ReadOnlyArray<T> {
    fn from(val: JsValue) -> Self {
        <Array as From<JsValue>>::from(val).into()
    }
}

impl<T: JsCast> FromWasmAbi for ReadOnlyArray<T> {
    type Abi = <Array as FromWasmAbi>::Abi;

    #[allow(unsafe_code)]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        Array::from_abi(js).into()
    }
}

impl<'a, T: JsCast> IntoWasmAbi for &'a ReadOnlyArray<T> {
    type Abi = <&'a Array as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.inner.clone().into_abi()
    }
}

impl<T: JsCast> JsCast for ReadOnlyArray<T> {
    fn instanceof(val: &JsValue) -> bool {
        Array::instanceof(val)
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        Array::unchecked_from_js(val).into()
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        // We use `#[repr(transparent)]` for `ReadOnlyArray<T>` with the inner
        // type being `Array` so this is safe.
        #[allow(unsafe_code, clippy::transmute_ptr_to_ptr)]
        unsafe {
            transmute(Array::unchecked_from_js_ref(val))
        }
    }

    // `Array` overrides this so we shall too.
    fn is_type_of(val: &JsValue) -> bool {
        Array::is_array(val)
    }
}

impl<T: JsCast> OptionFromWasmAbi for ReadOnlyArray<T> {
    fn is_none(abi: &Self::Abi) -> bool {
        Array::is_none(abi)
    }
}

impl<'a, T: JsCast> OptionIntoWasmAbi for &'a ReadOnlyArray<T> {
    fn none() -> Self::Abi {
        Array::none()
    }
}

// We use the derive for this instead because it'll work and because we'll
// get `StructuralEq` and `StructuralPartialEq` which I don't think we, mere
// mortals, can implement ourselves.
/*
impl<T: JsCast> PartialEq for ReadOnlyArray<T> {
    fn eq(&self, other: &Self) -> bool { self.inner.eq(other.inner) }
}
impl<T: JsCast> Eq for ReadOnlyArray<T> { }
*/

impl<T: JsCast> RefFromWasmAbi for ReadOnlyArray<T> {
    type Abi = <Array as RefFromWasmAbi>::Abi;
    type Anchor = ManuallyDrop<Self>;

    #[allow(unsafe_code)]
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        // Again, because of `#[repr(transparent)]` (on `ManuallyDrop` and on
        // `ReadOnlyArray`) this is safe.
        #[allow(unsafe_code, unused_unsafe)]
        unsafe {
            transmute(Array::ref_from_abi(js))
        }
    }
}

impl<T: JsCast> WasmDescribe for ReadOnlyArray<T> {
    fn describe() {
        Array::describe()
    }
}
