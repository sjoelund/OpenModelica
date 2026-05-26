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

use crate::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/// An implementation of a generic unordered set, a.k.a. hash set.
///
///   This implementation uses separate chaining and automatically rehashes the set
///   when the load factor becomes too large to keep the performance up.
#[derive(Clone, Debug, PartialEq)]
pub struct UnorderedSet<T: Clone> {
    pub buckets: Mutable::Mutable<metamodelica::Array<Arc<metamodelica::List<T>>>>,
    pub size: Mutable::Mutable<i32>,
    pub hashFn: Hash<T>,
    pub eqFn: KeyEq<T>,
}

impl<T: Clone> Default for UnorderedSet<T> {
    fn default() -> Self {
        Self {
            buckets: Default::default(),
            size: Default::default(),
            hashFn: { let __placeholder: Hash<T> = |_| panic!("default-constructed placeholder fn must not be called"); __placeholder },
            eqFn: { let __placeholder: KeyEq<T> = |_, _| panic!("default-constructed placeholder fn must not be called"); __placeholder },
        }
    }
}

pub type UNORDERED_SET<T> = UnorderedSet<T>;

pub type Hash<T: Clone> = fn(T) -> Result<i32>;

pub type KeyEq<T: Clone> = fn(T, T) -> Result<bool>;

pub fn new<T: Clone + 'static>(mut hash: Hash<T>, mut keyEq: KeyEq<T>, mut bucketCount: i32) -> Arc<UnorderedSet<T>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut buckets: Mutable::Mutable<metamodelica::Array<Arc<metamodelica::List<T>>>>;
    buckets = Mutable::create(arrayCreate(bucketCount.clone(), metamodelica::nil()));
    set = Arc::new(UnorderedSet { buckets: buckets.clone(), size: Mutable::create(0), hashFn: hash, eqFn: keyEq });
    set
}

pub fn fromList<T: Clone + 'static>(mut elements: Arc<metamodelica::List<T>>, mut hash: Hash<T>, mut keyEq: KeyEq<T>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    set = new(hash, keyEq, Util::nextPrime((elements.clone().len() as i32)));
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        add(e.clone(), set.clone())?;
    }
    Ok(set)
}

pub fn copy<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> Arc<UnorderedSet<T>> {
    let mut outSet: Arc<UnorderedSet<T>>;
    outSet = Arc::new(UnorderedSet { buckets: Mutable::create(metamodelica::arrayFromVec(Mutable::access(set.buckets.clone()).borrow().clone())), size: Mutable::create(Mutable::access(set.size.clone())), hashFn: set.hashFn, eqFn: set.eqFn });
    outSet
}

pub fn add<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut hash: i32 = 0;
    let mut pos: i32 = 0;
    let mut okey: Option<T> = None;
    (okey, hash) = find(key.clone(), set.clone())?;
    if isNone(okey.clone()) {
        addKey(key.clone(), hash.clone(), set.clone())?;
    }
    Ok(())
}

pub fn addNew<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut hashfn: Hash<T> = set.hashFn;
    let mut hash: i32 = 0;
    let mut pos: i32 = 0;
    hash = intMod(hashfn(key.clone())?, (Mutable::access(set.buckets.clone()).borrow().len() as i32));
    addKey(key.clone(), hash.clone(), set.clone())?;
    Ok(())
}

pub fn addUnique<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut hash: i32 = 0;
    let (None, __pa0) = (find(key.clone(), set.clone())?) else { bail!("pattern mismatch") };
    hash = __pa0.clone();
    addKey(key.clone(), hash.clone(), set.clone())?;
    Ok(())
}

pub fn remove<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut removed: bool = false;
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>> = Mutable::access(set.buckets.clone());
    let mut hashfn: Hash<T> = set.hashFn;
    let mut eqfn: KeyEq<T> = set.eqFn;
    let mut hash: i32 = 0;
    let mut bucket: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut okey: Option<T> = None;
    hash = intMod(hashfn(key.clone())?, (buckets.clone().borrow().len() as i32));
    bucket = buckets.clone().borrow()[(hash.clone() + 1-1) as usize].clone();
    (bucket, okey) = List::deleteMemberOnTrue(key.clone(), bucket.clone(), Arc::new(eqfn))?;
    removed = isSome(okey.clone());
    if removed.clone() {
        {let _arr = buckets.clone(); _arr.borrow_mut()[(hash.clone() + 1-1) as usize] = bucket.clone(); _arr};
        Mutable::update(set.size.clone(), Mutable::access(set.size.clone()) - 1);
    }
    Ok(removed)
}

pub fn get<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<Option<T>> {
    let mut outKey: Option<T> = None;
    (outKey, _) = find(key.clone(), set.clone())?;
    Ok(outKey)
}

pub fn getOrFail<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<T> {
    let mut outKey: T;
    let mut okey: Option<T> = None;
    (okey, _) = find(key.clone(), set.clone())?;
    let Some(__pa0) = (okey.clone()) else { bail!("pattern mismatch") };
    outKey = __pa0.clone();
    Ok(outKey)
}

pub fn contains<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut res: bool = false;
    res = isSome((find(key.clone(), set.clone())?).0);
    Ok(res)
}

pub fn first<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> Result<T> {
    let mut val: T;
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            val = k.clone();
            return Ok(val);
        }
    }
    bail!("fail");
    Ok(val)
}

pub fn isEqual<T: Clone + 'static>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut equal: bool = true;
    if Mutable::access(set1.size.clone()) != Mutable::access(set2.size.clone()) {
        equal = false;
        return Ok(equal);
    }
    let __range0 = Mutable::access(set1.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set2.clone())?) {
                equal = false;
                return Ok(equal);
            }
        }
    }
    Ok(equal)
}

pub fn toList<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            outList = cons(k.clone(), outList.clone());
        }
    }
    outList
}

pub fn toArray<T: Clone + 'static + Default>(mut set: Arc<UnorderedSet<T>>) -> metamodelica::Array<T> {
    let mut outArray: metamodelica::Array<T>;
    let mut dummy: T;
    let mut i: i32 = 1;
    outArray = metamodelica::arrayCreateDefault(Mutable::access(set.size.clone()));
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), k.clone()) };
            i = i.clone() + 1;
        }
    }
    outArray
}

pub fn fold<FT: Clone + 'static, T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>, mut startValue: FT) -> FT {
    pub type FoldFn<T: Clone, FT: Clone> = fn(T, FT) -> Result<FT>;

    let mut result: FT = startValue.clone();
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            result = r#fn(k.clone(), result.clone()).unwrap();
        }
    }
    result
}

pub fn apply<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>) -> Result<()> {
    pub type ApplyFn<T: Clone> = fn(T) -> Result<T>;

    let mut hashfn: Hash<T> = set.hashFn;
    let mut eqfn: KeyEq<T> = set.eqFn;
    let mut bucket_count: i32 = 0;
    let mut hash: i32 = 0;
    let mut size: i32 = 0;
    let mut new_buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    let mut newKey: T;
    let mut bucket: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut duplicate: bool = false;
    bucket_count = Util::nextPrime(Mutable::access(set.size.clone()));
    new_buckets = arrayCreate(bucket_count.clone(), metamodelica::nil());
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            newKey = r#fn(k.clone())?;
            hash = intMod(hashfn(newKey.clone())?, bucket_count.clone());
            bucket = new_buckets.clone().borrow()[(hash.clone() + 1-1) as usize].clone();
            duplicate = false;
            for mut nk in &*bucket.clone() {
                let mut nk = nk.clone();
                if eqfn(nk.clone(), newKey.clone())? {
                    duplicate = true;
                    break;
                }
            }
            if !(duplicate.clone()) {
                {let _arr = new_buckets.clone(); _arr.borrow_mut()[(hash.clone() + 1-1) as usize] = cons(newKey.clone(), bucket.clone()); _arr};
                size = size.clone() + 1;
            }
        }
    }
    Mutable::update(set.buckets.clone(), new_buckets.clone());
    Mutable::update(set.size.clone(), size.clone());
    Ok(())
}

pub fn all<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFn<T: Clone> = fn(T) -> Result<bool>;

    let mut res: bool = false;
    if isEmpty(set.clone()) {
        res = true;
        return res;
    }
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(r#fn(k.clone()).unwrap()) {
                res = false;
                return res;
            }
        }
    }
    res = true;
    res
}

pub fn any<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFn<T: Clone> = fn(T) -> Result<bool>;

    let mut res: bool = false;
    if isEmpty(set.clone()) {
        res = false;
        return res;
    }
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if r#fn(k.clone()).unwrap() {
                res = true;
                return res;
            }
        }
    }
    res = false;
    res
}

pub fn none<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFn<T: Clone> = fn(T) -> Result<bool>;

    let mut res: bool = false;
    if isEmpty(set.clone()) {
        res = true;
        return res;
    }
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if r#fn(k.clone()).unwrap() {
                res = false;
                return res;
            }
        }
    }
    res = true;
    res
}

pub fn filterOnFalse<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<Arc<UnorderedSet<T>>> {
    pub type PredFn<T: Clone> = fn(T) -> Result<bool>;

    let mut falseSet: Arc<UnorderedSet<T>> = new(set.hashFn, set.eqFn, 13);
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(r#fn(k.clone())?) {
                add(k.clone(), falseSet.clone())?;
            }
        }
    }
    Ok(falseSet)
}

pub fn splitOnTrue<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<(Arc<UnorderedSet<T>>, Arc<UnorderedSet<T>>)> {
    pub type PredFn<T: Clone> = fn(T) -> Result<bool>;

    let mut trueSet: Arc<UnorderedSet<T>> = new(set.hashFn, set.eqFn, 13);
    let mut falseSet: Arc<UnorderedSet<T>> = new(set.hashFn, set.eqFn, 13);
    let __range0 = Mutable::access(set.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            add(k.clone(), if (r#fn(k.clone())?) {trueSet.clone()} else {falseSet.clone()})?;
        }
    }
    Ok((trueSet, falseSet))
}

pub fn size<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> i32 {
    let mut s: i32 = Mutable::access(set.size.clone());
    s
}

pub fn isEmpty<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> bool {
    let mut empty: bool = Mutable::access(set.size.clone()) == 0;
    empty
}

pub fn bucketCount<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> i32 {
    let mut count: i32 = (Mutable::access(set.buckets.clone()).borrow().len() as i32);
    count
}

pub fn loadFactor<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> metamodelica::Real {
    let mut load: metamodelica::Real = intReal(Mutable::access(set.size.clone())) / metamodelica::OrderedFloat((bucketCount(set.clone())) as f64);
    load
}

pub fn rehash<T: Clone + 'static>(mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut old_buckets: metamodelica::Array<Arc<metamodelica::List<T>>> = Mutable::access(set.buckets.clone());
    let mut new_buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    let mut bucket_count: i32 = 0;
    let mut hash: i32 = 0;
    let mut hashfn: Hash<T> = set.hashFn;
    bucket_count = Util::nextPrime(Mutable::access(set.size.clone()) * 2);
    new_buckets = arrayCreate(bucket_count.clone(), metamodelica::nil());
    let __range0 = old_buckets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            hash = intMod(hashfn(k.clone())?, bucket_count.clone());
            {let _arr = new_buckets.clone(); let _val = cons(k.clone(), new_buckets.clone().borrow()[(hash.clone() + 1-1) as usize].clone()); _arr.borrow_mut()[(hash.clone() + 1-1) as usize] = _val; _arr};
        }
    }
    Mutable::update(set.buckets.clone(), new_buckets.clone());
    Ok(())
}

pub fn toString<T: Clone + 'static + Default>(mut set: Arc<UnorderedSet<T>>, mut stringFn: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut delimiter: ArcStr) -> ArcStr {
    pub type StringFn<T: Clone> = fn(T) -> Result<ArcStr>;

    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut k in (toArray(set.clone())).borrow().iter() {
            let __x = stringFn(k.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (delimiter.clone()).clone());
    r#str
}

pub fn dump<T: Clone + 'static + Default>(mut set: Arc<UnorderedSet<T>>, mut stringFn: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> () {
    pub type StringFn<T: Clone> = fn(T) -> Result<ArcStr>;

    println!("{}", (toString(set.clone(), stringFn.clone(), (literal!("\n")).clone())).clone());
    println!("{}", (literal!("\n")).clone());
    ()
}

pub fn unique_list<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut hashFunc: Hash<T>, mut keyEqFunc: KeyEq<T>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = if (List::hasSeveralElements(inList.clone())) {toList(fromList(inList.clone(), hashFunc, keyEqFunc).unwrap())} else {inList.clone()};
    outList
}

pub fn union<T: Clone + 'static>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set = set1.clone();
        buckets = Mutable::access(set2.buckets.clone());
    } else {
        set = set2.clone();
        buckets = Mutable::access(set1.buckets.clone());
    }
    let __range0 = buckets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            add(k.clone(), set.clone())?;
        }
    }
    Ok(set)
}

pub fn union_list<T: Clone + 'static>(mut set_lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>, mut hashFunc: Hash<T>, mut keyEqFunc: KeyEq<T>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut rest: Arc<metamodelica::List<Arc<UnorderedSet<T>>>> = metamodelica::nil();
    if set_lst.clone().is_empty() {
        set = new(hashFunc, keyEqFunc, 13);
    } else {
        (set, rest) = extractFromLst(set_lst.clone(), Arc::new(fnptr!(intGt, i32, i32)))?;
        for mut tmp in &*rest.clone() {
            let mut tmp = tmp.clone();
            set = union(set.clone(), tmp.clone())?;
        }
    }
    Ok(set)
}

pub fn merge<T: Clone + 'static>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set1: Arc<UnorderedSet<T>> = set1;
    let __range0 = Mutable::access(set2.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            add(k.clone(), set1.clone())?;
        }
    }
    Ok(set1)
}

pub fn intersection<T: Clone + 'static>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut set_small: Arc<UnorderedSet<T>>;
    let mut set_big: Arc<UnorderedSet<T>>;
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set_small = set2.clone();
        set_big = set1.clone();
    } else {
        set_small = set1.clone();
        set_big = set2.clone();
    }
    let __range0 = Mutable::access(set_small.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if contains(k.clone(), set_big.clone())? {
                acc = cons(k.clone(), acc.clone());
            }
        }
    }
    set = fromList(acc.clone(), set1.hashFn, set1.eqFn)?;
    Ok(set)
}

pub fn intersection_list<T: Clone + 'static>(mut set_lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>, mut hashFunc: Hash<T>, mut keyEqFunc: KeyEq<T>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut set_small: Arc<UnorderedSet<T>>;
    let mut rest: Arc<metamodelica::List<Arc<UnorderedSet<T>>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    if !(set_lst.clone().is_empty()) {
        (set_small, rest) = extractFromLst(set_lst.clone(), Arc::new(fnptr!(intLt, i32, i32)))?;
        let __range0 = Mutable::access(set_small.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
        for mut b in __range0 {
            for mut k in &*b.clone() {
                let mut k = k.clone();
                if List::all(rest.clone(), Arc::new({ let __pe_b0 = k.clone(); move |__pe_a1| contains(__pe_b0.clone(), __pe_a1) })) {
                    acc = cons(k.clone(), acc.clone());
                }
            }
        }
    }
    set = fromList(acc.clone(), hashFunc, keyEqFunc)?;
    Ok(set)
}

pub fn difference_list<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut hashFunc: Hash<T>, mut keyEqFunc: KeyEq<T>) -> Result<Arc<metamodelica::List<T>>> {
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut set2: Arc<UnorderedSet<T>>;
    let mut lst1: Arc<metamodelica::List<T>> = inList1.clone();
    let mut lst2: Arc<metamodelica::List<T>> = inList2.clone();
    while !(lst1.clone().is_empty() || lst2.clone().is_empty()) && keyEqFunc(listHead(lst1.clone())?, listHead(lst2.clone())?)? {
        lst1 = listRest(lst1.clone())?;
        lst2 = listRest(lst2.clone())?;
    }
    if lst1.clone().is_empty() || lst2.clone().is_empty() {
        acc = lst1.clone();
        return Ok(acc);
    }
    set2 = fromList(lst2.clone(), hashFunc, keyEqFunc)?;
    for mut k in &*lst1.clone() {
        let mut k = k.clone();
        if !(contains(k.clone(), set2.clone())?) {
            acc = cons(k.clone(), acc.clone());
        }
    }
    Ok(acc)
}

pub fn equal_list<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut hashFunc: Hash<T>, mut keyEqFunc: KeyEq<T>) -> Result<bool> {
    let mut b: bool = false;
    let mut set1: Arc<UnorderedSet<T>> = fromList(inList1.clone(), hashFunc, keyEqFunc)?;
    let mut set2: Arc<UnorderedSet<T>> = fromList(inList2.clone(), hashFunc, keyEqFunc)?;
    if Mutable::access(set1.size.clone()) != Mutable::access(set2.size.clone()) {
        return Ok(b);
    }
    for mut k in &*inList1.clone() {
        let mut k = k.clone();
        if !(contains(k.clone(), set2.clone())?) {
            return Ok(b);
        }
    }
    b = true;
    Ok(b)
}

pub fn difference<T: Clone + 'static>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    let __range0 = Mutable::access(set1.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set2.clone())?) {
                acc = cons(k.clone(), acc.clone());
            }
        }
    }
    set = fromList(acc.clone(), set1.hashFn, set1.eqFn)?;
    Ok(set)
}

pub fn sym_difference<T: Clone + 'static>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<Arc<UnorderedSet<T>>> {
    let mut set: Arc<UnorderedSet<T>>;
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    let __range0 = Mutable::access(set1.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range0 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set2.clone())?) {
                acc = cons(k.clone(), acc.clone());
            }
        }
    }
    let __range1 = Mutable::access(set2.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut b in __range1 {
        for mut k in &*b.clone() {
            let mut k = k.clone();
            if !(contains(k.clone(), set1.clone())?) {
                acc = cons(k.clone(), acc.clone());
            }
        }
    }
    set = fromList(acc.clone(), set1.hashFn, set1.eqFn)?;
    Ok(set)
}

pub fn isDisjoint<T: Clone + 'static>(mut set1: Arc<UnorderedSet<T>>, mut set2: Arc<UnorderedSet<T>>) -> Result<bool> {
    let mut b: bool = true;
    let mut set_small: Arc<UnorderedSet<T>>;
    let mut set_big: Arc<UnorderedSet<T>>;
    if Mutable::access(set1.size.clone()) > Mutable::access(set2.size.clone()) {
        set_small = set2.clone();
        set_big = set1.clone();
    } else {
        set_small = set1.clone();
        set_big = set2.clone();
    }
    let __range0 = Mutable::access(set_small.buckets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut buckets in __range0 {
        for mut k in &*buckets.clone() {
            let mut k = k.clone();
            if contains(k.clone(), set_big.clone())? {
                b = false;
                return Ok(b);
            }
        }
    }
    Ok(b)
}

fn find<T: Clone + 'static>(mut key: T, mut set: Arc<UnorderedSet<T>>) -> Result<(Option<T>, i32)> {
    let mut outKey: Option<T> = None;
    let mut hash: i32 = 0;
    let mut hashfn: Hash<T> = set.hashFn;
    let mut eqfn: KeyEq<T> = set.eqFn;
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>> = Mutable::access(set.buckets.clone());
    let mut bucket: Arc<metamodelica::List<T>> = metamodelica::nil();
    hash = intMod(hashfn(key.clone())?, (buckets.clone().borrow().len() as i32));
    bucket = buckets.clone().borrow()[(hash.clone() + 1-1) as usize].clone();
    for mut k in &*bucket.clone() {
        let mut k = k.clone();
        if eqfn(k.clone(), key.clone())? {
            outKey = Some(k.clone());
            break;
        }
    }
    Ok((outKey, hash))
}

fn addKey<T: Clone + 'static>(mut key: T, mut hash: i32, mut set: Arc<UnorderedSet<T>>) -> Result<()> {
    let mut buckets: metamodelica::Array<Arc<metamodelica::List<T>>>;
    let mut h: i32 = 0;
    let mut hashfn: Hash<T>;
    if loadFactor(set.clone()) > metamodelica::OrderedFloat((1) as f64) {
        rehash(set.clone())?;
        hashfn = set.hashFn;
        buckets = Mutable::access(set.buckets.clone());
        h = intMod(hashfn(key.clone())?, (buckets.clone().borrow().len() as i32));
    } else {
        buckets = Mutable::access(set.buckets.clone());
        h = hash.clone();
    }
    {let _arr = buckets.clone(); let _val = cons(key.clone(), buckets.clone().borrow()[(h.clone() + 1-1) as usize].clone()); _arr.borrow_mut()[(h.clone() + 1-1) as usize] = _val; _arr};
    Mutable::update(set.size.clone(), Mutable::access(set.size.clone()) + 1);
    Ok(())
}

fn extractFromLst<T: Clone + 'static>(mut lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>>, mut func: Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>) -> Result<(Arc<UnorderedSet<T>>, Arc<metamodelica::List<Arc<UnorderedSet<T>>>>)> {
    type size_compare = fn(i32, i32) -> Result<bool>;

    let mut single: Arc<UnorderedSet<T>>;
    let mut rest: Arc<metamodelica::List<Arc<UnorderedSet<T>>>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut tmp_lst: Arc<metamodelica::List<Arc<UnorderedSet<T>>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    single = __pa0.clone();
    tmp_lst = __pa1.clone();
    size = Mutable::access(single.size.clone());
    for mut tmp in &*tmp_lst.clone() {
        let mut tmp = tmp.clone();
        if func(Mutable::access(tmp.size.clone()), size.clone())? {
            size = Mutable::access(tmp.size.clone());
            rest = cons(single.clone(), rest.clone());
            single = tmp.clone();
        } else {
            rest = cons(tmp.clone(), rest.clone());
        }
    }
    Ok((single, rest))
}


