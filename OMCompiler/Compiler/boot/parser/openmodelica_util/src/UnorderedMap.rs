// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::Error;
use crate::IOStream;
use crate::UnorderedSet;
use crate::Util;
use crate::Vector;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/// An implementation of a generic unordered map, a.k.a. hash map.
///
///   This implementation uses separate chaining and automatically rehashes the map
///   when the load factor becomes too large to keep the performance up.
#[derive(Clone, Debug, PartialEq)]
pub struct UnorderedMap<K: Clone, V: Clone> {
    pub buckets: Arc<Vector::Vector<Arc<metamodelica::List<i32>>>>,
    pub keys: Arc<Vector::Vector<K>>,
    pub values: Arc<Vector::Vector<V>>,
    pub hashFn: Hash<K>,
    pub eqFn: KeyEq<K>,
}

impl<K: Clone, V: Clone> Default for UnorderedMap<K, V> {
    fn default() -> Self {
        Self {
            buckets: Default::default(),
            keys: Default::default(),
            values: Default::default(),
            hashFn: { let __placeholder: Hash<K> = |_| panic!("default-constructed placeholder fn must not be called"); __placeholder },
            eqFn: { let __placeholder: KeyEq<K> = |_, _| panic!("default-constructed placeholder fn must not be called"); __placeholder },
        }
    }
}

pub type UNORDERED_MAP<K, V> = UnorderedMap<K, V>;

pub type Hash<K: Clone> = fn(K) -> Result<i32>;

pub type KeyEq<K: Clone> = fn(K, K) -> Result<bool>;

pub type KeyStringFn<K: Clone> = fn(K) -> Result<ArcStr>;

pub type ValueStringFn<V: Clone> = fn(V) -> Result<ArcStr>;

pub fn new<V: Clone + 'static, K: Clone + 'static>(mut hash: Hash<K>, mut keyEq: KeyEq<K>, mut bucketCount: i32) -> Arc<UnorderedMap<K, V>> {
    let mut map: Arc<UnorderedMap<K, V>>;
    map = Arc::new(UnorderedMap { buckets: Vector::newFill(bucketCount.clone(), metamodelica::nil()), keys: Vector::new(0), values: Vector::new(0), hashFn: hash, eqFn: keyEq });
    map
}

pub fn fromLists<V: Clone + 'static + Default, K: Clone + 'static + Default>(mut keys: Arc<metamodelica::List<K>>, mut values: Arc<metamodelica::List<V>>, mut hash: Hash<K>, mut keyEq: KeyEq<K>) -> Result<Arc<UnorderedMap<K, V>>> {
    let mut map: Arc<UnorderedMap<K, V>>;
    let mut key_count: i32 = 0;
    let mut bucket_count: i32 = 0;
    let mut v: V;
    let mut rest_v: Arc<metamodelica::List<V>> = values.clone();
    key_count = (keys.clone().len() as i32);
    bucket_count = Util::nextPrime(key_count.clone());
    map = Arc::new(UnorderedMap { buckets: Vector::newFill(bucket_count.clone(), metamodelica::nil()), keys: Vector::new(key_count.clone()), values: Vector::new(key_count.clone()), hashFn: hash, eqFn: keyEq });
    for mut k in &*keys.clone() {
        let mut k = k.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_v.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        v = __pa0.clone();
        rest_v = __pa1.clone();
        add(k.clone(), v.clone(), map.clone())?;
    }
    Ok(map)
}

pub fn copy<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Arc<UnorderedMap<K, V>> {
    let mut outMap: Arc<UnorderedMap<K, V>>;
    outMap = Arc::new(UnorderedMap { buckets: Vector::copy(map.buckets.clone()), keys: Vector::copy(map.keys.clone()), values: Vector::copy(map.values.clone()), hashFn: map.hashFn, eqFn: map.eqFn });
    outMap
}

pub fn deepCopy<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut r#fn: Arc<dyn ::std::ops::Fn(V) -> Result<V> + 'static>) -> Arc<UnorderedMap<K, V>> {
    pub type CopyFn<V: Clone> = fn(V) -> Result<V>;

    let mut outMap: Arc<UnorderedMap<K, V>>;
    outMap = Arc::new(UnorderedMap { buckets: Vector::copy(map.buckets.clone()), keys: Vector::copy(map.keys.clone()), values: Vector::deepCopy(map.values.clone(), r#fn.clone()), hashFn: map.hashFn, eqFn: map.eqFn });
    outMap
}

pub fn add<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut key: K, mut value: V, mut map: Arc<UnorderedMap<K, V>>) -> Result<()> {
    let mut index: i32 = 0;
    let mut hash: i32 = 0;
    (index, hash) = find(key.clone(), map.clone())?;
    if index.clone() > 0 {
        Vector::update(map.values.clone(), index.clone(), value.clone())?;
    } else {
        addEntry(key.clone(), value.clone(), hash.clone(), map.clone())?;
    }
    Ok(())
}

pub fn addNew<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut key: K, mut value: V, mut map: Arc<UnorderedMap<K, V>>) -> Result<()> {
    let mut hashfn: Hash<K> = map.hashFn;
    let mut hash: i32 = intMod(hashfn(key.clone())?, Vector::size(map.buckets.clone()));
    addEntry(key.clone(), value.clone(), hash.clone(), map.clone())?;
    Ok(())
}

pub fn addUnique<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut key: K, mut value: V, mut map: Arc<UnorderedMap<K, V>>) -> Result<()> {
    let mut index: i32 = 0;
    let mut hash: i32 = 0;
    (index, hash) = find(key.clone(), map.clone())?;
    let false = (index.clone() > 0) else { bail!("pattern mismatch") };
    addEntry(key.clone(), value.clone(), hash.clone(), map.clone())?;
    Ok(())
}

pub fn tryAdd<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut key: K, mut value: V, mut map: Arc<UnorderedMap<K, V>>) -> Result<V> {
    let mut outValue: V;
    let mut index: i32 = 0;
    let mut hash: i32 = 0;
    (index, hash) = find(key.clone(), map.clone())?;
    if index.clone() > 0 {
        outValue = Vector::getNoBounds(map.values.clone(), index.clone());
    } else {
        outValue = value.clone();
        addEntry(key.clone(), value.clone(), hash.clone(), map.clone())?;
    }
    Ok(outValue)
}

pub fn tryUpdate<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut value: V, mut map: Arc<UnorderedMap<K, V>>) -> Result<bool> {
    let mut updated: bool = false;
    let mut index: i32 = 0;
    let mut hash: i32 = 0;
    (index, hash) = find(key.clone(), map.clone())?;
    updated = index.clone() > 0;
    if updated.clone() {
        Vector::update(map.values.clone(), index.clone(), value.clone())?;
    }
    Ok(updated)
}

pub fn addUpdate<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut key: K, mut r#fn: Arc<dyn ::std::ops::Fn(Option<V>) -> Result<V> + 'static>, mut map: Arc<UnorderedMap<K, V>>) -> Result<V> {
    pub type UpdateFn<V: Clone> = fn(Option<V>) -> Result<V>;

    let mut value: V;
    let mut index: i32 = 0;
    let mut hash: i32 = 0;
    (index, hash) = find(key.clone(), map.clone())?;
    if index.clone() > 0 {
        value = r#fn(Some(Vector::getNoBounds(map.values.clone(), index.clone())))?;
        Vector::updateNoBounds(map.values.clone(), index.clone(), value.clone());
    } else {
        value = r#fn(None)?;
        addEntry(key.clone(), value.clone(), hash.clone(), map.clone())?;
    }
    Ok(value)
}

pub fn tryAddUpdate<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut r#fn: Arc<dyn ::std::ops::Fn(Option<V>) -> Result<V> + 'static>, mut map: Arc<UnorderedMap<K, V>>) -> Result<bool> {
    pub type UpdateFn<V: Clone> = fn(Option<V>) -> Result<V>;

    let mut updated: bool = false;
    let mut index: i32 = 0;
    let mut hash: i32 = 0;
    let mut value: V;
    (index, hash) = find(key.clone(), map.clone())?;
    updated = index.clone() > 0;
    if updated.clone() {
        value = r#fn(Some(Vector::getNoBounds(map.values.clone(), index.clone())))?;
        Vector::updateNoBounds(map.values.clone(), index.clone(), value.clone());
    }
    Ok(updated)
}

pub fn remove<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>) -> Result<bool> {
    fn update_indices(mut bucket: Arc<metamodelica::List<i32>>, mut removedIndex: i32) -> Arc<metamodelica::List<i32>> {
        let mut outBucket: Arc<metamodelica::List<i32>> = metamodelica::nil();
        outBucket = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (bucket.clone()).into_iter().cloned() {
            let __x = if (i.clone() > removedIndex.clone()) {i.clone() - 1} else {i.clone()};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        outBucket
    }

    let mut removed: bool = false;
    let mut hash: i32 = 0;
    let mut index: i32 = 0;
    let mut bucket: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (index, hash) = find(key.clone(), map.clone())?;
    removed = index.clone() > 0;
    if !(removed.clone()) {
        return Ok(removed);
    }
    bucket = Vector::get(map.buckets.clone(), hash.clone() + 1)?;
    (bucket, _) = List::deleteMemberOnTrue(index.clone(), bucket.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
    Vector::updateNoBounds(map.buckets.clone(), hash.clone() + 1, bucket.clone());
    Vector::remove(map.keys.clone(), index.clone())?;
    Vector::remove(map.values.clone(), index.clone())?;
    Vector::apply(map.buckets.clone(), Arc::new({ let __pe_b1 = index.clone(); move |__pe_a0| Ok(update_indices(__pe_a0, __pe_b1.clone())) }));
    Ok(removed)
}

pub fn clear<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> () {
    Vector::clear(map.buckets.clone());
    Vector::push(map.buckets.clone(), metamodelica::nil());
    Vector::clear(map.keys.clone());
    Vector::clear(map.values.clone());
    ()
}

pub fn get<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>) -> Option<V> {
    let mut value: Option<V> = None;
    let (mut index, _): (i32, i32) = find(key.clone(), map.clone()).unwrap();
    value = if (index.clone() > 0) {Some(Vector::getNoBounds(map.values.clone(), index.clone()))} else {None};
    value
}

pub fn getSafe<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>, mut info: SourceInfo) -> Result<V> {
    let mut value: V;
    let (mut index, _): (i32, i32) = find(key.clone(), map.clone())?;
    if index.clone() > 0 {
        value = Vector::getNoBounds(map.values.clone(), index.clone());
    } else {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UnorderedMap.getSafe")); __mm_s.push_str(&*literal!(" failed because the key did not exist.")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
        bail!("fail");
    }
    Ok(value)
}

pub fn getOrFail<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>) -> V {
    let mut value: V = Vector::get(map.values.clone(), (find(key.clone(), map.clone()).unwrap()).0).unwrap();
    value
}

pub fn getOrDefault<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>, mut default: V) -> V {
    let mut value: V;
    let (mut index, _): (i32, i32) = find(key.clone(), map.clone()).unwrap();
    value = if (index.clone() > 0) {Vector::getNoBounds(map.values.clone(), index.clone())} else {default.clone()};
    value
}

pub fn getList<K: Clone + 'static, V: Clone + 'static>(mut keys: Arc<metamodelica::List<K>>, mut map: Arc<UnorderedMap<K, V>>) -> Result<Arc<metamodelica::List<V>>> {
    let mut values: Arc<metamodelica::List<V>> = metamodelica::nil();
    let mut index: i32 = 0;
    for mut key in &*keys.clone() {
        let mut key = key.clone();
        (index, _) = find(key.clone(), map.clone())?;
        if index.clone() > 0 {
            values = cons(Vector::getNoBounds(map.values.clone(), index.clone()), values.clone());
        }
    }
    values = values.clone().reverse();
    Ok(values)
}

pub fn getKey<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>) -> Option<K> {
    let mut outKey: Option<K> = None;
    let (mut index, _): (i32, i32) = find(key.clone(), map.clone()).unwrap();
    outKey = if (index.clone() > 0) {Some(Vector::getNoBounds(map.keys.clone(), index.clone()))} else {None};
    outKey
}

pub fn updateKey<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>) -> Result<()> {
    Vector::update(map.keys.clone(), (find(key.clone(), map.clone())?).0, key.clone())?;
    Ok(())
}

pub fn contains<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>) -> bool {
    let mut res: bool = (find(key.clone(), map.clone()).unwrap()).0 > 0;
    res
}

pub fn first<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> V {
    let mut value: V = Vector::get(map.values.clone(), 1).unwrap();
    value
}

pub fn firstKey<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> K {
    let mut key: K = Vector::get(map.keys.clone(), 1).unwrap();
    key
}

pub fn keyAt<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut index: i32) -> K {
    let mut key: K = Vector::get(map.keys.clone(), index.clone()).unwrap();
    key
}

pub fn valueAt<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut index: i32) -> V {
    let mut value: V = Vector::get(map.values.clone(), index.clone()).unwrap();
    value
}

pub fn toList<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Arc<metamodelica::List<(K, V)>> {
    let mut lst: Arc<metamodelica::List<(K, V)>> = List::zip(keyList(map.clone()), valueList(map.clone()));
    lst
}

pub fn keyList<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Arc<metamodelica::List<K>> {
    let mut keys: Arc<metamodelica::List<K>> = Vector::toList(map.keys.clone());
    keys
}

pub fn valueList<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Arc<metamodelica::List<V>> {
    let mut values: Arc<metamodelica::List<V>> = Vector::toList(map.values.clone());
    values
}

pub fn toArray<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut map: Arc<UnorderedMap<K, V>>) -> metamodelica::Array<(K, V)> {
    let mut entries: metamodelica::Array<(K, V)>;
    let mut keys: Arc<Vector::Vector<K>> = map.keys.clone();
    let mut values: Arc<Vector::Vector<V>> = map.values.clone();
    let mut t: (K, V);
    let mut sz: i32 = Vector::size(keys.clone());
    entries = metamodelica::arrayCreateDefault(sz.clone());
    for mut i in 1..=sz.clone() {
        unsafe { metamodelica::Dangerous::arrayInitSlot(entries.clone(), i.clone(), (Vector::getNoBounds(keys.clone(), i.clone()), Vector::getNoBounds(values.clone(), i.clone()))) };
    }
    entries
}

pub fn keyArray<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> metamodelica::Array<K> {
    let mut keys: metamodelica::Array<K> = Vector::toArray(map.keys.clone());
    keys
}

pub fn valueArray<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> metamodelica::Array<V> {
    let mut values: metamodelica::Array<V> = Vector::toArray(map.values.clone());
    values
}

pub fn toVector<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Arc<Vector::Vector<(K, V)>> {
    let mut entries: Arc<Vector::Vector<(K, V)>>;
    let mut keys: Arc<Vector::Vector<K>> = map.keys.clone();
    let mut values: Arc<Vector::Vector<V>> = map.values.clone();
    let mut sz: i32 = Vector::size(keys.clone());
    entries = Vector::new(sz.clone());
    for mut i in 1..=sz.clone() {
        Vector::updateNoBounds(entries.clone(), i.clone(), (Vector::getNoBounds(keys.clone(), i.clone()), Vector::getNoBounds(values.clone(), i.clone())));
    }
    entries
}

pub fn keyVector<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Arc<Vector::Vector<K>> {
    let mut keys: Arc<Vector::Vector<K>> = Vector::copy(map.keys.clone());
    keys
}

pub fn valueVector<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Arc<Vector::Vector<V>> {
    let mut values: Arc<Vector::Vector<V>> = Vector::copy(map.values.clone());
    values
}

pub fn keySet<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Result<Arc<UnorderedSet::UnorderedSet<K>>> {
    let mut set: Arc<UnorderedSet::UnorderedSet<K>>;
    let mut bucket_count: i32 = Vector::size(map.buckets.clone());
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<K>>>;
    buckets = arrayCreate(bucket_count.clone(), metamodelica::nil());
    for mut h in 1..=bucket_count.clone() {
        {let _arr = buckets.clone(); _arr.borrow_mut()[(h.clone()-1) as usize] = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut i in (Vector::get(map.buckets.clone(), h.clone())?).into_iter().cloned() {
            let __x = Vector::getNoBounds(map.keys.clone(), i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }; _arr};
    }
    set = Arc::new(UnorderedSet::UnorderedSet { buckets: Mutable::create(buckets.clone()), size: Mutable::create(Vector::size(map.keys.clone())), hashFn: map.hashFn, eqFn: map.eqFn });
    Ok(set)
}

pub fn fold<FT: Clone + 'static, K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut r#fn: Arc<dyn ::std::ops::Fn(V, FT) -> Result<FT> + 'static>, mut arg: FT) -> FT {
    pub type FoldFn<V: Clone, FT: Clone> = fn(V, FT) -> Result<FT>;

    let mut arg: FT = arg;
    arg = Vector::fold(map.values.clone(), r#fn.clone(), arg.clone());
    arg
}

pub fn map<OT: Clone + 'static + Default, K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut r#fn: Arc<dyn ::std::ops::Fn(V) -> Result<OT> + 'static>) -> Arc<UnorderedMap<K, OT>> {
    pub type MapFn<V: Clone, OT: Clone> = fn(V) -> Result<OT>;

    let mut outMap: Arc<UnorderedMap<K, OT>>;
    let mut new_values: Arc<Vector::Vector<OT>>;
    new_values = Vector::map(map.values.clone(), r#fn.clone(), true);
    outMap = Arc::new(UnorderedMap { buckets: Vector::copy(map.buckets.clone()), keys: Vector::copy(map.keys.clone()), values: new_values.clone(), hashFn: map.hashFn, eqFn: map.eqFn });
    outMap
}

pub fn apply<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut r#fn: Arc<dyn ::std::ops::Fn(V) -> Result<V> + 'static>) -> () {
    pub type ApplyFn<V: Clone> = fn(V) -> Result<V>;

    Vector::apply(map.values.clone(), r#fn.clone());
    ()
}

pub fn merge<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut map1: Arc<UnorderedMap<K, V>>, mut map2: Arc<UnorderedMap<K, V>>, mut info: SourceInfo) -> Result<Arc<UnorderedMap<K, V>>> {
    let mut result: Arc<UnorderedMap<K, V>>;
    let mut tmp: Arc<UnorderedMap<K, V>>;
    let mut k: K;
    let mut v: V;
    if Vector::size(map1.keys.clone()) > Vector::size(map2.keys.clone()) {
        result = copy(map1.clone());
        tmp = map2.clone();
    } else {
        result = copy(map2.clone());
        tmp = map1.clone();
    }
    for mut i in 1..=Vector::size(tmp.keys.clone()) {
        k = Vector::getNoBounds(tmp.keys.clone(), i.clone());
        v = Vector::getNoBounds(tmp.values.clone(), i.clone());
        if '__try0: {
            unwrap_break_err!(addUnique(k.clone(), v.clone(), result.clone()), '__try0);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UnorderedMap.merge")); __mm_s.push_str(&*literal!(" failed because both maps contain the same key.")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
        }
    }
    Ok(result)
}

pub fn subMap<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut map: Arc<UnorderedMap<K, V>>, mut lst: Arc<metamodelica::List<K>>) -> Result<Arc<UnorderedMap<K, V>>> {
    let mut sub_map: Arc<UnorderedMap<K, V>>;
    let mut len: i32 = 0;
    len = (lst.clone().len() as i32);
    sub_map = Arc::new(UnorderedMap { buckets: Vector::newFill(Util::nextPrime(len.clone()), metamodelica::nil()), keys: Vector::new(len.clone()), values: Vector::new(len.clone()), hashFn: map.hashFn, eqFn: map.eqFn });
    for mut k in &*lst.clone() {
        let mut k = k.clone();
        add(k.clone(), getSafe(k.clone(), map.clone(), metamodelica::sourceInfo!())?, sub_map.clone())?;
    }
    Ok(sub_map)
}

pub fn all<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut r#fn: Arc<dyn ::std::ops::Fn(V) -> Result<bool> + 'static>) -> bool {
    pub type PredFn<V: Clone> = fn(V) -> Result<bool>;

    let mut res: bool = false;
    res = Vector::all(map.values.clone(), r#fn.clone());
    res
}

pub fn any<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut r#fn: Arc<dyn ::std::ops::Fn(V) -> Result<bool> + 'static>) -> bool {
    pub type PredFn<V: Clone> = fn(V) -> Result<bool>;

    let mut res: bool = false;
    res = Vector::any(map.values.clone(), r#fn.clone());
    res
}

pub fn none<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut r#fn: Arc<dyn ::std::ops::Fn(V) -> Result<bool> + 'static>) -> bool {
    pub type PredFn<V: Clone> = fn(V) -> Result<bool>;

    let mut res: bool = false;
    res = Vector::none(map.values.clone(), r#fn.clone());
    res
}

pub fn size<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> i32 {
    let mut s: i32 = Vector::size(map.keys.clone());
    s
}

pub fn isEmpty<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> bool {
    let mut empty: bool = Vector::isEmpty(map.keys.clone());
    empty
}

pub fn bucketCount<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> i32 {
    let mut count: i32 = Vector::size(map.buckets.clone());
    count
}

pub fn loadFactor<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> metamodelica::Real {
    let mut load: metamodelica::Real = intReal(Vector::size(map.keys.clone())) / metamodelica::OrderedFloat((Vector::size(map.buckets.clone())) as f64);
    load
}

pub fn rehash<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>) -> Result<()> {
    let mut keys: Arc<Vector::Vector<K>> = map.keys.clone();
    let mut buckets: Arc<Vector::Vector<Arc<metamodelica::List<i32>>>> = map.buckets.clone();
    let mut bucket_count: i32 = 0;
    let mut bucket_id: i32 = 0;
    let mut hashfn: Hash<K> = map.hashFn;
    Vector::clear(buckets.clone());
    bucket_count = Util::nextPrime(Vector::size(keys.clone()) * 2);
    Vector::resize(buckets.clone(), bucket_count.clone(), metamodelica::nil());
    for mut i in 1..=Vector::size(map.keys.clone()) {
        bucket_id = intMod(hashfn(Vector::get(keys.clone(), i.clone())?)?, bucket_count.clone()) + 1;
        Vector::updateNoBounds(buckets.clone(), bucket_id.clone(), cons(i.clone(), Vector::getNoBounds(buckets.clone(), bucket_id.clone())));
    }
    Ok(())
}

pub fn toString<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut keyStringFn: KeyStringFn<K>, mut valueStringFn: ValueStringFn<V>, mut delimiter: ArcStr, mut concatinator: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut keys: Arc<Vector::Vector<K>> = map.keys.clone();
    let mut values: Arc<Vector::Vector<V>> = map.values.clone();
    for mut i in (1..=Vector::size(keys.clone())).rev() {
        strl = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStringFn(Vector::get(keys.clone(), i.clone())?)?); __mm_s.push_str(&*concatinator.clone()); __mm_s.push_str(&*valueStringFn(Vector::get(values.clone(), i.clone())?)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }, strl.clone());
    }
    r#str = stringDelimitList(strl.clone(), (delimiter.clone()).clone());
    Ok(r#str)
}

pub fn toJSON<K: Clone + 'static, V: Clone + 'static>(mut map: Arc<UnorderedMap<K, V>>, mut keyStringFn: KeyStringFn<K>, mut valueStringFn: ValueStringFn<V>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut io: IOStream::IOStream;
    let mut keys: Arc<Vector::Vector<K>> = map.keys.clone();
    let mut values: Arc<Vector::Vector<V>> = map.values.clone();
    let mut sz: i32 = Vector::size(keys.clone());
    io = IOStream::create((literal!("UnorderedMap.toJSON")).clone(), crate::IOStream::IOStreamType::LIST)?;
    io = IOStream::append(io.clone(), (literal!("{\n")).clone())?;
    if sz.clone() > 0 {
        io = IOStream::append(io.clone(), (literal!("  \"")).clone())?;
        io = IOStream::append(io.clone(), (keyStringFn(Vector::getNoBounds(keys.clone(), 1))?).clone())?;
        io = IOStream::append(io.clone(), (literal!("\": \"")).clone())?;
        io = IOStream::append(io.clone(), (valueStringFn(Vector::getNoBounds(values.clone(), 1))?).clone())?;
        io = IOStream::append(io.clone(), (literal!("\"")).clone())?;
        for mut i in 2..=sz.clone() {
            io = IOStream::append(io.clone(), (literal!(",\n  \"")).clone())?;
            io = IOStream::append(io.clone(), (keyStringFn(Vector::getNoBounds(keys.clone(), i.clone()))?).clone())?;
            io = IOStream::append(io.clone(), (literal!("\": \"")).clone())?;
            io = IOStream::append(io.clone(), (valueStringFn(Vector::getNoBounds(values.clone(), i.clone()))?).clone())?;
            io = IOStream::append(io.clone(), (literal!("\"")).clone())?;
        }
    }
    io = IOStream::append(io.clone(), (literal!("\n}")).clone())?;
    r#str = (IOStream::string(io.clone())?).clone();
    Ok(r#str)
}

fn find<K: Clone + 'static, V: Clone + 'static>(mut key: K, mut map: Arc<UnorderedMap<K, V>>) -> Result<(i32, i32)> {
    let mut index: i32 = -1;
    let mut hash: i32 = 0;
    let mut hashfn: Hash<K> = map.hashFn;
    let mut eqfn: KeyEq<K> = map.eqFn;
    let mut bucket: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if Vector::size(map.buckets.clone()) > 0 {
        hash = intMod(hashfn(key.clone())?, Vector::size(map.buckets.clone()));
        bucket = Vector::get(map.buckets.clone(), hash.clone() + 1)?;
        for mut i in &*bucket.clone() {
            let mut i = i.clone();
            if eqfn(key.clone(), Vector::getNoBounds(map.keys.clone(), i.clone()))? {
                index = i.clone();
                break;
            }
        }
    } else {
        hash = 0;
    }
    Ok((index, hash))
}

fn addEntry<K: Clone + 'static + Default, V: Clone + 'static + Default>(mut key: K, mut value: V, mut hash: i32, mut map: Arc<UnorderedMap<K, V>>) -> Result<()> {
    let mut buckets: Arc<Vector::Vector<Arc<metamodelica::List<i32>>>> = map.buckets.clone();
    Vector::push(map.keys.clone(), key.clone());
    Vector::push(map.values.clone(), value.clone());
    if loadFactor(map.clone()) > metamodelica::OrderedFloat((1) as f64) {
        rehash(map.clone())?;
    } else {
        Vector::update(buckets.clone(), hash.clone() + 1, cons(Vector::size(map.keys.clone()), Vector::get(buckets.clone(), hash.clone() + 1)?))?;
    }
    Ok(())
}


