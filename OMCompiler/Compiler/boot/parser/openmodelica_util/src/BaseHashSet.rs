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

use openmodelica_util_datatypes_basic::Array;

// Below is the instance specific code. For each hashset the user must define:
// Key      - The key used to uniquely define elements in a hashset
// hashFunc - A function that maps a key to a positive integer.
// keyEqual - A comparison function between two keys, returns true if equal.
// Generic hashset code below
// adrpo: use a prime here (pick your poison):
//        3   5   7  11  13  17  19  23  29  31  37  41  43  47  53  59  61  67
//       71  73  79  83  89  97 101 103 107 109 113 127 131 137 139 149 151 157
//      163 167 173 179 181 191 193 197 199 211 223 227 229 233 239 241 251 257
//      263 269 271 277 281 283 293 307 311 313 317 331 337 347 349 353 359 367
//      373 379 383 389 397 401 409 419 421 431 433 439 443 449 457 461 463 467
//      479 487 491 499 503 509 521 523 541 547 557 563 569 571 577 587 593 599
//      601 607 613 617 619 631 641 643 647 653 659 661 673 677 683 691 701 709
//      719 727 733 739 743 751 757 761 769 773 787 797 809 811 821 823 827 829
//      839 853 857 859 863 877 881 883 887 907 911 919 929 937 941 947 953 967
//      971 977 983 991 997 1013 2053 3023 4013 4999 5051 5087 24971
//
// You can also use Util.nextPrime if you know exactly how large the hash set
// should be.
pub const lowBucketSize: i32 = 257;

pub const avgBucketSize: i32 = 2053;

pub const bigBucketSize: i32 = 4013;

pub const biggerBucketSize: i32 = 25343;

pub const hugeBucketSize: i32 = 536870879;

pub const defaultBucketSize: i32 = avgBucketSize;

pub type HashSet<Key> = (metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>, (i32, i32, metamodelica::Array<Option<Key>>), i32, i32, (FuncHash<Key>, FuncEq<Key>, FuncKeyString<Key>));

pub type HashVector<Key> = metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;

pub type ValueArray<Key> = (i32, i32, metamodelica::Array<Option<Key>>);

pub type FuncsTuple<Key> = (FuncHash<Key>, FuncEq<Key>, FuncKeyString<Key>);

pub type FuncHash<Key: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<i32> + 'static>;

pub type FuncEq<Key: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Key) -> Result<bool> + 'static>;

pub type FuncKeyString<Key: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<ArcStr> + 'static>;

pub fn bucketToValuesSize(mut szBucket: i32) -> i32 {
    let mut szArr: i32 = 0;
    szArr = (((intReal(szBucket.clone())) * (metamodelica::OrderedFloat(0.6_f64))).0 as i32);
    szArr
}

pub fn emptyHashSetWork<Key: Clone + 'static>(mut szBucket: i32, mut fntpl: FuncsTuple<Key>) -> HashSet<Key> {
    let mut hashSet: HashSet<Key>;
    let mut arr: metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;
    let mut emptyarr: metamodelica::Array<Option<Key>>;
    let mut szArr: i32 = 0;
    arr = arrayCreate(szBucket.clone(), metamodelica::nil());
    szArr = bucketToValuesSize(szBucket.clone());
    emptyarr = arrayCreate(szArr.clone(), None);
    hashSet = (arr.clone(), (0, szArr.clone(), emptyarr.clone()), szBucket.clone(), 0, fntpl.clone());
    hashSet
}

pub fn add<Key: Clone + 'static>(mut entry: Key, mut hashSet: HashSet<Key>) -> Result<HashSet<Key>> {
    let mut outHashSet: HashSet<Key>;
    outHashSet = (match (entry.clone(), hashSet.clone()) {
        (mut key, (mut hashvec, mut varr, mut bsize, mut n, ref fntpl @ (ref hashFunc, _, _))) => {
            let mut indx: i32 = 0;
            let mut newpos: i32 = 0;
            let mut indexes: Arc<metamodelica::List<(Key, i32)>> = metamodelica::nil();
            let mut fkey: Option<Key> = None;
            (fkey, indx) = get1(key.clone(), hashSet.clone())?;
            if isSome(fkey.clone()) {
                varr = valueArraySetnth(varr.clone(), indx.clone(), key.clone())?;
            } else {
                indx = intMod(hashFunc(key.clone())?, bsize.clone());
                newpos = valueArrayLength(varr.clone());
                varr = valueArrayAdd(varr.clone(), key.clone())?;
                indexes = hashvec.borrow()[(indx.clone() + 1-1) as usize].clone();
                hashvec = {let _arr = hashvec.clone(); _arr.borrow_mut()[(indx.clone() + 1-1) as usize] = cons((key.clone(), newpos.clone()), indexes.clone()); _arr};
                n = valueArrayLength(varr.clone());
            }
            (hashvec.clone(), varr.clone(), bsize.clone(), n.clone(), fntpl.clone())
        },
        (mut key, (_, _, mut bsize, _, (mut hashFunc, _, mut keystrFunc))) => {
            let mut hval: i32 = 0;
            let mut s: ArcStr = arcstr::literal!("");
            println!("{}", (literal!("- BaseHashSet.add failed: ")).clone());
            println!("{}", (literal!("bsize: ")).clone());
            println!("{}", (intString(bsize.clone())).clone());
            println!("{}", (literal!(" key: ")).clone());
            s = keystrFunc(key.clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" Hash: ")); ArcStr::from(__mm_s) }).clone());
            hval = intMod(hashFunc(key.clone())?, bsize.clone());
            println!("{}", (intString(hval.clone())).clone());
            println!("{}", (literal!("\n")).clone());
            bail!("fail")
        },
    });
    Ok(outHashSet)
}

pub fn addNoUpdCheck<Key: Clone + 'static>(mut entry: Key, mut hashSet: HashSet<Key>) -> Result<HashSet<Key>> {
    let mut outHashSet: HashSet<Key>;
    outHashSet = (match (entry.clone(), hashSet.clone()) {
        (mut key, (mut hashvec, mut varr, mut bsize, _, ref fntpl @ (ref hashFunc, _, _))) => {
            let mut indx: i32 = 0;
            let mut newpos: i32 = 0;
            let mut n_1: i32 = 0;
            let mut varr_1: (i32, i32, metamodelica::Array<Option<Key>>);
            let mut indexes: Arc<metamodelica::List<(Key, i32)>> = metamodelica::nil();
            let mut hashvec_1: metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;
            indx = intMod(hashFunc(key.clone())?, bsize.clone());
            newpos = valueArrayLength(varr.clone());
            varr_1 = valueArrayAdd(varr.clone(), key.clone())?;
            indexes = hashvec.borrow()[(indx.clone() + 1-1) as usize].clone();
            hashvec_1 = {let _arr = hashvec.clone(); _arr.borrow_mut()[(indx.clone() + 1-1) as usize] = cons((key.clone(), newpos.clone()), indexes.clone()); _arr};
            n_1 = valueArrayLength(varr_1.clone());
            (hashvec_1.clone(), varr_1.clone(), bsize.clone(), n_1.clone(), fntpl.clone())
        },
    });
    Ok(outHashSet)
}

pub fn addUnique<Key: Clone + 'static>(mut key: Key, mut hashSet: HashSet<Key>) -> Result<HashSet<Key>> {
    let mut outHashSet: HashSet<Key>;
    outHashSet = (match hashSet.clone() {
        (mut hashvec, mut varr, mut bsize, _, ref fntpl @ (ref hashFunc, _, _)) if (!(has(key.clone(), hashSet.clone())?)) => {
            let mut indx: i32 = 0;
            let mut newpos: i32 = 0;
            let mut n_1: i32 = 0;
            let mut varr_1: (i32, i32, metamodelica::Array<Option<Key>>);
            let mut indexes: Arc<metamodelica::List<(Key, i32)>> = metamodelica::nil();
            let mut hashvec_1: metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;
            indx = intMod(hashFunc(key.clone())?, bsize.clone());
            newpos = valueArrayLength(varr.clone());
            varr_1 = valueArrayAdd(varr.clone(), key.clone())?;
            indexes = hashvec.borrow()[(indx.clone() + 1-1) as usize].clone();
            hashvec_1 = {let _arr = hashvec.clone(); _arr.borrow_mut()[(indx.clone() + 1-1) as usize] = cons((key.clone(), newpos.clone()), indexes.clone()); _arr};
            n_1 = valueArrayLength(varr_1.clone());
            (hashvec_1.clone(), varr_1.clone(), bsize.clone(), n_1.clone(), fntpl.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outHashSet)
}

pub fn delete<Key: Clone + 'static>(mut key: Key, mut hashSet: HashSet<Key>) -> Result<HashSet<Key>> {
    let mut outHashSet: HashSet<Key>;
    let mut indx: i32 = 0;
    let mut n: i32 = 0;
    let mut bsize: i32 = 0;
    let mut varr_1: (i32, i32, metamodelica::Array<Option<Key>>);
    let mut varr: (i32, i32, metamodelica::Array<Option<Key>>);
    let mut hashvec: metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;
    let mut fntpl: FuncsTuple<Key>;
    (hashvec, varr, bsize, n, fntpl) = hashSet.clone();
    let __pa0 = ::match_deref::match_deref! { match &(get1(key.clone(), hashSet.clone())?) {
        (Some(_), __pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    indx = __pa0.clone();
    varr_1 = valueArrayClearnth(varr.clone(), indx.clone())?;
    outHashSet = (hashvec.clone(), varr_1.clone(), bsize.clone(), n.clone(), fntpl.clone());
    Ok(outHashSet)
}

pub fn has<Key: Clone + 'static>(mut key: Key, mut hashSet: HashSet<Key>) -> Result<bool> {
    let mut b: bool = false;
    b = (match hashSet.clone() {
        (_, (0, _, _), _, _, _) => {
            false
        },
        _ => {
            let mut oKey: Option<Key> = None;
            (oKey, _) = get1(key.clone(), hashSet.clone())?;
            isSome(oKey.clone())
        },
    });
    Ok(b)
}

pub fn hasAll<Key: Clone + 'static>(mut keys: Arc<metamodelica::List<Key>>, mut hashSet: HashSet<Key>) -> Result<bool> {
    let mut b: bool = true;
    for mut key in &*keys.clone() {
        let mut key = key.clone();
        b = has(key.clone(), hashSet.clone())?;
        if !(b.clone()) {
            return Ok(b.clone());
        }
    }
    Ok(b)
}

pub fn get<Key: Clone + 'static>(mut key: Key, mut hashSet: HashSet<Key>) -> Result<Option<Key>> {
    let mut okey: Option<Key> = None;
    (okey, _) = get1(key.clone(), hashSet.clone())?;
    Ok(okey)
}

fn get1<Key: Clone + 'static>(mut key: Key, mut hashSet: HashSet<Key>) -> Result<(Option<Key>, i32)> {
    let mut okey: Option<Key> = None;
    let mut indx: i32 = 0;
    (okey, indx) = (match hashSet.clone() {
        (mut hashvec, mut varr, mut bsize, _, (mut hashFunc, mut keyEqual, _)) => {
            let mut hashindx: i32 = 0;
            let mut indexes: Arc<metamodelica::List<(Key, i32)>> = metamodelica::nil();
            let mut k: Option<Key> = None;
            let mut b: bool = false;
            hashindx = intMod(hashFunc(key.clone())?, bsize.clone());
            indexes = hashvec.borrow()[(hashindx.clone() + 1-1) as usize].clone();
            (indx, b) = get2(key.clone(), indexes.clone(), keyEqual.clone());
            k = if (b.clone()) {valueArrayNthT(varr.clone(), indx.clone())?} else {None};
            (k.clone(), indx.clone())
        },
    });
    Ok((okey, indx))
}

fn get2<Key: Clone + 'static>(mut key: Key, mut keyIndices: Arc<metamodelica::List<(Key, i32)>>, mut keyEqual: Arc<dyn ::std::ops::Fn(Key, Key) -> Result<bool> + 'static>) -> (i32, bool) {
    let mut index: i32 = -1;
    let mut found: bool = true;
    let mut key2: Key;
    for mut t in &*keyIndices.clone() {
        let mut t = t.clone();
        (key2, index) = t.clone();
        if keyEqual(key.clone(), key2.clone()).unwrap() {
            return (index.clone(), found.clone());
        }
    }
    found = false;
    (index, found)
}

pub fn printHashSet<Key: Clone + 'static>(mut hashSet: HashSet<Key>) -> Result<()> {
    let mut printKey: FuncKeyString<Key>;
    let (_, _, _, _, (_, _, __pa0)) = hashSet.clone();
    printKey = __pa0.clone();
    println!("{}", stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (hashSetList(hashSet.clone())?).into_iter().cloned() {
            let __x = printKey(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone()));
    Ok(())
}

pub fn dumpHashSet<Key: Clone + 'static>(mut hashSet: HashSet<Key>) -> Result<()> {
    println!("{}", (literal!("HashSet:\n")).clone());
    printHashSet(hashSet.clone())?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn hashSetList<Key: Clone + 'static>(mut hashSet: HashSet<Key>) -> Result<Arc<metamodelica::List<Key>>> {
    let mut lst: Arc<metamodelica::List<Key>> = metamodelica::nil();
    lst = (match hashSet.clone() {
        (_, mut varr, _, _, _) => {
            valueArrayList(varr.clone())?
        },
    });
    Ok(lst)
}

pub fn valueArrayList<Key: Clone + 'static>(mut inValueArray: ValueArray<Key>) -> Result<Arc<metamodelica::List<Key>>> {
    let mut outList: Arc<metamodelica::List<Key>> = metamodelica::nil();
    let mut arr: metamodelica::Array<Option<Key>>;
    let mut size: i32 = 0;
    let mut e: Key;
    (size, _, arr) = inValueArray.clone();
    for mut i in 1..=size.clone() {
        if isSome(arr.borrow()[(i.clone()-1) as usize].clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(arr.borrow()[(i.clone()-1) as usize].clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            outList = cons(e.clone(), outList.clone());
        }
    }
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn currentSize<Key: Clone + 'static>(mut hashSet: HashSet<Key>) -> i32 {
    let mut sz: i32 = 0;
    let mut va: ValueArray<Key>;
    (_, va, _, _, _) = hashSet.clone();
    sz = valueArrayLength(va.clone());
    sz
}

pub fn valueArrayLength<Key: Clone + 'static>(mut valueArray: ValueArray<Key>) -> i32 {
    let mut sz: i32 = 0;
    (sz, _, _) = valueArray.clone();
    sz
}

pub fn valueArrayAdd<Key: Clone + 'static>(mut valueArray: ValueArray<Key>, mut entry: Key) -> Result<ValueArray<Key>> {
    let mut outValueArray: ValueArray<Key>;
    let mut n: i32 = 0;
    let mut size: i32 = 0;
    let mut expandsize: i32 = 0;
    let mut expandsize_1: i32 = 0;
    let mut arr: metamodelica::Array<Option<Key>>;
    let mut rsize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut rexpandsize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (n, size, arr) = valueArray.clone();
    if n.clone() >= size.clone() {
        rsize = intReal(size.clone());
        rexpandsize = rsize.clone() * metamodelica::OrderedFloat(0.4_f64);
        expandsize = ((rexpandsize.clone()).0 as i32);
        expandsize_1 = intMax(expandsize.clone(), 1);
        size = expandsize_1.clone() + size.clone();
        arr = Array::expand(expandsize_1.clone(), arr.clone(), None)?;
    }
    arr = {let _arr = arr.clone(); _arr.borrow_mut()[(n.clone() + 1-1) as usize] = Some(entry.clone()); _arr};
    outValueArray = (n.clone() + 1, size.clone(), arr.clone());
    Ok(outValueArray)
}

pub fn valueArraySetnth<Key: Clone + 'static>(mut valueArray: ValueArray<Key>, mut pos: i32, mut entry: Key) -> Result<ValueArray<Key>> {
    let mut outValueArray: ValueArray<Key>;
    let mut arr_1: metamodelica::Array<Option<Key>>;
    let mut arr: metamodelica::Array<Option<Key>>;
    let mut n: i32 = 0;
    let mut size: i32 = 0;
    (n, size, arr) = valueArray.clone();
    let true = (pos.clone() < size.clone()) else { bail!("pattern mismatch") };
    arr_1 = {let _arr = arr.clone(); _arr.borrow_mut()[(pos.clone() + 1-1) as usize] = Some(entry.clone()); _arr};
    outValueArray = (n.clone(), size.clone(), arr_1.clone());
    Ok(outValueArray)
}

pub fn valueArrayClearnth<Key: Clone + 'static>(mut valueArray: ValueArray<Key>, mut pos: i32) -> Result<ValueArray<Key>> {
    let mut outValueArray: ValueArray<Key>;
    let mut arr_1: metamodelica::Array<Option<Key>>;
    let mut arr: metamodelica::Array<Option<Key>>;
    let mut n: i32 = 0;
    let mut size: i32 = 0;
    (n, size, arr) = valueArray.clone();
    let true = (pos.clone() < size.clone()) else { bail!("pattern mismatch") };
    arr_1 = {let _arr = arr.clone(); _arr.borrow_mut()[(pos.clone() + 1-1) as usize] = None; _arr};
    outValueArray = (n.clone(), size.clone(), arr_1.clone());
    Ok(outValueArray)
}

pub fn valueArrayNth<Key: Clone + 'static>(mut valueArray: ValueArray<Key>, mut pos: i32) -> Result<Key> {
    let mut key: Key;
    key = (match valueArray.clone() {
        (mut n, _, mut arr) if (pos.clone() <= n.clone()) => {
            let mut k: Key;
            let __pa0 = ::match_deref::match_deref! { match &(arr.borrow()[(pos.clone() + 1-1) as usize].clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            k = __pa0.clone();
            k.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(key)
}

fn valueArrayNthT<Key: Clone + 'static>(mut valueArray: ValueArray<Key>, mut pos: i32) -> Result<Option<Key>> {
    let mut key: Option<Key> = None;
    key = (match valueArray.clone() {
        (mut n, _, mut arr) if (pos.clone() <= n.clone()) => {
            arr.borrow()[(pos.clone() + 1-1) as usize].clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(key)
}

