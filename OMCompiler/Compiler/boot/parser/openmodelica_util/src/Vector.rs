// Manually written - replaces the auto-generated translation of Vector.mo.
//
// The MetaModelica `Vector<T>` is a dynamic array backed by an `array<T>`
// allocated with `arrayCreateNoInit` plus a separately tracked logical size.
// The naive translation of that pattern crashes in Rust: writing into an
// uninitialised slot via `vec[i] = v` drops the garbage that already lives in
// that slot.
//
// Here we collapse the (data, size) pair onto Rust's `Vec<T>` directly. The
// `Vec`'s `len()` is the logical Vector size; `capacity()` is what
// MetaModelica calls the capacity. `Vector::new(n)` reserves `n` slots up
// front but starts with length 0, matching the MetaModelica semantics
// (initial capacity hint, empty Vector). `reserveCapacity` is no longer
// needed since `Vec::push` etc. handle growth themselves.
//
// The exported `Vector<T>` is the same shape as `metamodelica::Array<T>`
// (`Rc<RefCell<Vec<T>>>`); this lets `fromArray` / `toArray` share the
// underlying storage cheaply when desired, and keeps the public functions
// usable through the auto-generated `Arc<Vector<T>>` wrappers that the
// codegen produces at call sites.
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::sync::Arc;
use anyhow::{Result, bail};
use arcstr::ArcStr;
use metamodelica::{Array, List, stringDelimitList, cons, nil, listHead, listRest};

/// `Vector<T>` aliases `Array<T>`; the `Vec`'s length is the logical size.
///
/// The MetaModelica `record VECTOR` wrapper is intentionally not modelled
/// here — call sites only ever go through the module's functions, never
/// access `.data` / `.size` directly, so the simpler representation is
/// sufficient.
pub type Vector<T> = Array<T>;

#[inline]
fn idx(i: i32) -> usize { (i - 1) as usize }

pub fn new<T: Clone + 'static>(size: i32) -> Arc<Vector<T>> {
    // Initial capacity hint only; logical size is 0.
    Arc::new(metamodelica::arrayFromVec(Vec::with_capacity(size.max(0) as usize)))
}

pub fn newFill<T: Clone + 'static>(size: i32, value: T) -> Arc<Vector<T>> {
    let n = size.max(0) as usize;
    let mut v = Vec::with_capacity(n);
    v.resize(n, value);
    Arc::new(metamodelica::arrayFromVec(v))
}

pub fn fromArray<T: Clone + 'static>(arr: Array<T>) -> Arc<Vector<T>> {
    // Copy the array contents so that mutations to the Vector do not
    // alias the source array.
    Arc::new(metamodelica::arrayFromVec(arr.borrow().clone()))
}

pub fn toArray<T: Clone + 'static>(v: Arc<Vector<T>>) -> Array<T> {
    metamodelica::arrayFromVec(v.borrow().clone())
}

pub fn fromList<T: Clone + 'static>(l: Arc<List<T>>) -> Arc<Vector<T>> {
    Arc::new(metamodelica::arrayFromVec(l.into_iter().cloned().collect()))
}

pub fn toList<T: Clone + 'static>(v: Arc<Vector<T>>) -> Arc<List<T>> {
    let data = v.borrow();
    let mut acc: Arc<List<T>> = nil();
    for e in data.iter().rev() {
        acc = cons(e.clone(), acc);
    }
    acc
}

pub fn push<T: Clone + 'static>(v: Arc<Vector<T>>, value: T) {
    v.borrow_mut().push(value);
}

pub fn insert<T: Clone + 'static>(v: Arc<Vector<T>>, value: T, index: i32) -> Result<()> {
    let sz = v.borrow().len() as i32;
    if index == sz + 1 {
        v.borrow_mut().push(value);
        return Ok(());
    }
    if index < 1 || index > sz {
        bail!("Vector.insert: index {} out of bounds (size {})", index, sz);
    }
    v.borrow_mut().insert(idx(index), value);
    Ok(())
}

pub fn append<T: Clone + 'static>(v1: Arc<Vector<T>>, v2: Arc<Vector<T>>) {
    // Snapshot v2 first in case v1 == v2 (same Rc) — we still want
    // documented "append v2 to end of v1" semantics rather than
    // a RefCell borrow conflict.
    let extension: Vec<T> = v2.borrow().clone();
    v1.borrow_mut().extend(extension);
}

pub fn appendList<T: Clone + 'static>(v: Arc<Vector<T>>, l: Arc<List<T>>) -> Result<()> {
    let mut data = v.borrow_mut();
    let mut rest = l;
    while !rest.is_empty() {
        data.push(listHead(rest.clone())?);
        rest = listRest(rest)?;
    }
    Ok(())
}

pub fn appendArray<T: Clone + 'static>(v: Arc<Vector<T>>, arr: Array<T>) {
    let extension: Vec<T> = arr.borrow().clone();
    v.borrow_mut().extend(extension);
}

pub fn pop<T: Clone + 'static>(v: Arc<Vector<T>>) {
    // Matches MetaModelica: undefined behaviour if the Vector is empty.
    // We choose to silently no-op in that case rather than panicking,
    // matching `arrayClearIndex` being a no-op in the Rust runtime.
    v.borrow_mut().pop();
}

pub fn clear<T: Clone + 'static>(v: Arc<Vector<T>>) {
    // MetaModelica's `clear` preserves capacity; `Vec::clear` does the same.
    v.borrow_mut().clear();
}

pub fn shrink<T: Clone + 'static>(v: Arc<Vector<T>>, newSize: i32) {
    let mut data = v.borrow_mut();
    let new_len = newSize.max(0) as usize;
    if new_len < data.len() {
        data.truncate(new_len);
    }
}

pub fn grow<T: Clone + 'static>(v: Arc<Vector<T>>, newSize: i32, fillValue: T) {
    let mut data = v.borrow_mut();
    let new_len = newSize.max(0) as usize;
    if new_len > data.len() {
        data.resize(new_len, fillValue);
    }
}

pub fn resize<T: Clone + 'static>(v: Arc<Vector<T>>, newSize: i32, fillValue: T) {
    v.borrow_mut().resize(newSize.max(0) as usize, fillValue);
}

pub fn remove<T: Clone + 'static>(v: Arc<Vector<T>>, index: i32) -> Result<()> {
    let mut data = v.borrow_mut();
    let sz = data.len() as i32;
    if index == sz {
        data.pop();
        return Ok(());
    }
    // Match the MetaModelica check exactly (note: index < 0, not <= 0).
    if index < 0 || index > sz {
        bail!("Vector.remove: index {} out of bounds (size {})", index, sz);
    }
    data.remove(idx(index));
    Ok(())
}

pub fn update<T: Clone + 'static>(v: Arc<Vector<T>>, index: i32, value: T) -> Result<()> {
    let mut data = v.borrow_mut();
    let sz = data.len() as i32;
    if index <= 0 || index > sz {
        bail!("Vector.update: index {} out of bounds (size {})", index, sz);
    }
    data[idx(index)] = value;
    Ok(())
}

pub fn updateNoBounds<T: Clone + 'static>(v: Arc<Vector<T>>, index: i32, value: T) {
    v.borrow_mut()[idx(index)] = value;
}

pub fn get<T: Clone + 'static>(v: Arc<Vector<T>>, index: i32) -> Result<T> {
    let data = v.borrow();
    let sz = data.len() as i32;
    if index <= 0 || index > sz {
        bail!("Vector.get: index {} out of bounds (size {})", index, sz);
    }
    Ok(data[idx(index)].clone())
}

pub fn getNoBounds<T: Clone + 'static>(v: Arc<Vector<T>>, index: i32) -> T {
    v.borrow()[idx(index)].clone()
}

pub fn last<T: Clone + 'static>(v: Arc<Vector<T>>) -> Result<T> {
    let data = v.borrow();
    match data.last() {
        Some(e) => Ok(e.clone()),
        None => bail!("Vector.last: empty vector"),
    }
}

pub fn size<T: Clone + 'static>(v: Arc<Vector<T>>) -> i32 {
    v.borrow().len() as i32
}

pub fn capacity<T: Clone + 'static>(v: Arc<Vector<T>>) -> i32 {
    v.borrow().capacity() as i32
}

pub fn isEmpty<T: Clone + 'static>(v: Arc<Vector<T>>) -> bool {
    v.borrow().is_empty()
}

pub fn reserve<T: Clone + 'static>(v: Arc<Vector<T>>, newCapacity: i32) {
    let mut data = v.borrow_mut();
    let want = newCapacity.max(0) as usize;
    let cap = data.capacity();
    if want > cap {
        let len = data.len();
        data.reserve(want - len);
    }
}

pub fn trim<T: Clone + 'static>(v: Arc<Vector<T>>) {
    v.borrow_mut().shrink_to_fit();
}

pub fn fill<T: Clone + 'static>(v: Arc<Vector<T>>, value: T, from: i32, to: i32) -> Result<()> {
    let mut data = v.borrow_mut();
    let sz = data.len() as i32;
    if from < 1 || to < 1 || from > sz || to > sz {
        bail!("Vector.fill: range [{}, {}] out of bounds (size {})", from, to, sz);
    }
    for i in from..=to {
        data[idx(i)] = value.clone();
    }
    Ok(())
}

pub fn map<OT: Clone + 'static, T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<OT> + 'static>,
    shrink: bool,
) -> Arc<Vector<OT>> {
    let data = v.borrow();
    let mut new_vec: Vec<OT> = Vec::with_capacity(if shrink { data.len() } else { data.capacity() });
    for e in data.iter() {
        new_vec.push(r#fn(e.clone()).unwrap());
    }
    Arc::new(metamodelica::arrayFromVec(new_vec))
}

pub fn mapToList<OT: Clone + 'static, T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<OT> + 'static>,
) -> Arc<List<OT>> {
    let data = v.borrow();
    let mut l: Arc<List<OT>> = nil();
    for e in data.iter().rev() {
        l = cons(r#fn(e.clone()).unwrap(), l);
    }
    l
}

pub fn apply<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>,
) {
    let mut data = v.borrow_mut();
    for i in 0..data.len() {
        let cur = data[i].clone();
        data[i] = r#fn(cur).unwrap();
    }
}

pub fn fold<FT: Clone + 'static, T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>,
    arg: FT,
) -> FT {
    let data = v.borrow();
    let mut acc = arg;
    for e in data.iter() {
        acc = r#fn(e.clone(), acc).unwrap();
    }
    acc
}

pub fn find<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>,
) -> (Option<T>, i32) {
    let data = v.borrow();
    for (i, e) in data.iter().enumerate() {
        if r#fn(e.clone()).unwrap() {
            return (Some(e.clone()), (i + 1) as i32);
        }
    }
    (None, -1)
}

pub fn findLast<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>,
) -> (Option<T>, i32) {
    let data = v.borrow();
    for (i, e) in data.iter().enumerate().rev() {
        if r#fn(e.clone()).unwrap() {
            return (Some(e.clone()), (i + 1) as i32);
        }
    }
    (None, -1)
}

pub fn findFold<FT: Clone + 'static, T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T, FT) -> Result<(bool, FT)> + 'static>,
    arg: FT,
) -> (Option<T>, i32, FT) {
    let data = v.borrow();
    let mut oe: Option<T> = None;
    let mut index: i32 = -1;
    let mut acc = arg;
    for (i, e) in data.iter().enumerate() {
        let (res, new_acc) = r#fn(e.clone(), acc).unwrap();
        acc = new_acc;
        if res {
            oe = Some(e.clone());
            index = (i + 1) as i32;
        }
    }
    (oe, index, acc)
}

pub fn all<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>,
) -> bool {
    let data = v.borrow();
    for e in data.iter() {
        if !r#fn(e.clone()).unwrap() {
            return false;
        }
    }
    true
}

pub fn any<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>,
) -> bool {
    let data = v.borrow();
    for e in data.iter() {
        if r#fn(e.clone()).unwrap() {
            return true;
        }
    }
    false
}

pub fn none<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>,
) -> bool {
    !any(v, r#fn)
}

pub fn copy<T: Clone + 'static>(v: Arc<Vector<T>>) -> Arc<Vector<T>> {
    Arc::new(metamodelica::arrayFromVec(v.borrow().clone()))
}

pub fn deepCopy<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>,
) -> Arc<Vector<T>> {
    let data = v.borrow();
    let mut new_vec: Vec<T> = Vec::with_capacity(data.len());
    for e in data.iter() {
        new_vec.push(r#fn(e.clone()).unwrap());
    }
    Arc::new(metamodelica::arrayFromVec(new_vec))
}

pub fn swap<T: Clone + 'static>(v1: Arc<Vector<T>>, v2: Arc<Vector<T>>) {
    // Same-vector swap is a no-op; performing the borrow_mut pair would
    // otherwise panic on a double mutable borrow of the same RefCell.
    if Arc::ptr_eq(&v1, &v2) {
        return;
    }
    let mut b1 = v1.borrow_mut();
    let mut b2 = v2.borrow_mut();
    std::mem::swap(&mut *b1, &mut *b2);
}

pub fn toString<T: Clone + 'static>(
    v: Arc<Vector<T>>,
    stringFn: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>,
    strBegin: ArcStr,
    delim: ArcStr,
    strEnd: ArcStr,
) -> ArcStr {
    let data = v.borrow();
    let mut acc: Arc<List<ArcStr>> = nil();
    for e in data.iter().rev() {
        acc = cons(stringFn(e.clone()).unwrap(), acc);
    }
    let middle = stringDelimitList(acc, delim);
    let mut s = String::with_capacity(strBegin.len() + middle.len() + strEnd.len());
    s.push_str(&strBegin);
    s.push_str(&middle);
    s.push_str(&strEnd);
    ArcStr::from(s)
}
