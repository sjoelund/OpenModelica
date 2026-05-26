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
use openmodelica_util_datatypes_basic::List;

// Below is the instance specific code. For each hashtable the user must define:
// Key      - The key used to uniquely define elements in a hashtable
// Value    - The data to associate with each key
// hashFunc - A function that maps a key to a positive integer.
// keyEqual - A comparison function between two keys, returns true if equal.
// Generic hashtable code below
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
// You can also use Util.nextPrime if you know exactly how large the hash table
// should be.
pub const lowBucketSize: i32 = 257;

pub const avgBucketSize: i32 = 2053;

pub const bigBucketSize: i32 = 4013;

pub const biggerBucketSize: i32 = 25343;

pub const hugeBucketSize: i32 = 536870879;

pub const defaultBucketSize: i32 = avgBucketSize;

pub type HashEntry<Key, Value> = (Key, Value);

pub type HashNode<Key> = Arc<metamodelica::List<(Key, i32)>>;

pub type HashTable<Key, Value> = (metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>, (i32, i32, metamodelica::Array<Option<(Key, Value)>>), i32, (FuncHash<Key>, FuncEq<Key>, FuncKeyString<Key>, FuncValString<Value>));

pub type HashVector<Key> = metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;

pub type ValueArray<Key, Value> = (i32, i32, metamodelica::Array<Option<(Key, Value)>>);

pub type FuncsTuple<Key, Value> = (FuncHash<Key>, FuncEq<Key>, FuncKeyString<Key>, FuncValString<Value>);

pub type FuncHash<Key: Clone> = fn(Key) -> Result<i32>;

pub type FuncEq<Key: Clone> = fn(Key, Key) -> Result<bool>;

pub type FuncKeyString<Key: Clone> = fn(Key) -> Result<ArcStr>;

pub type FuncValString<Value: Clone> = fn(Value) -> Result<ArcStr>;

pub fn bucketToValuesSize(mut szBucket: i32) -> i32 {
    let mut szArr: i32 = 0;
    szArr = (((intReal(szBucket.clone())) * (metamodelica::OrderedFloat(0.6_f64))).0 as i32);
    szArr
}

pub fn emptyHashTableWork<Key: Clone + 'static, Value: Clone + 'static>(mut szBucket: i32, mut fntpl: FuncsTuple<Key, Value>) -> HashTable<Key, Value> {
    let mut hashTable: HashTable<Key, Value>;
    let mut arr: metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;
    let mut lst: Arc<metamodelica::List<Option<(Key, Value)>>> = metamodelica::nil();
    let mut emptyarr: metamodelica::Array<Option<(Key, Value)>>;
    let mut szArr: i32 = 0;
    let mut szBucketFixed: i32 = intMax(szBucket.clone(), 1);
    arr = arrayCreate(szBucketFixed.clone(), metamodelica::nil());
    szArr = bucketToValuesSize(szBucketFixed.clone());
    emptyarr = arrayCreate(szArr.clone(), None);
    hashTable = (arr.clone(), (0, szArr.clone(), emptyarr.clone()), szBucketFixed.clone(), fntpl.clone());
    hashTable
}

pub fn add<Key: Clone + 'static, Value: Clone + 'static>(mut entry: HashEntry<Key, Value>, mut hashTable: HashTable<Key, Value>) -> Result<HashTable<Key, Value>> {
    let mut outHashTable: HashTable<Key, Value>;
    let mut hashvec: HashVector<Key>;
    let mut varr: ValueArray<Key, Value>;
    let mut bsize: i32 = 0;
    let mut hash_idx: i32 = 0;
    let mut arr_idx: i32 = 0;
    let mut new_pos: i32 = 0;
    let mut fntpl: FuncsTuple<Key, Value>;
    let mut hashFunc: FuncHash<Key>;
    let mut keyEqual: FuncEq<Key>;
    let mut key: Key;
    let mut key2: Key;
    let mut val: Value;
    let mut indices: HashNode<Key> = metamodelica::nil();
    (key, _) = entry.clone();
    let (__pa0, __pa1, __pa2, ref __pa5 @ (ref __pa3, ref __pa4, _, _)) = hashTable.clone();
    hashvec = __pa0.clone();
    varr = __pa1.clone();
    bsize = __pa2.clone();
    hashFunc = __pa3.clone();
    keyEqual = __pa4.clone();
    fntpl = __pa5.clone();
    hash_idx = intMod(hashFunc(key.clone())?, bsize.clone()) + 1;
    indices = hashvec.borrow()[(hash_idx.clone()-1) as usize].clone();
    for mut i in &*indices.clone() {
        let mut i = i.clone();
        (key2, _) = i.clone();
        if keyEqual(key.clone(), key2.clone())? {
            (_, arr_idx) = i.clone();
            valueArraySet(varr.clone(), arr_idx.clone(), entry.clone())?;
            outHashTable = hashTable.clone();
            return Ok(outHashTable);
        }
    }
    (varr, new_pos) = valueArrayAdd(varr.clone(), entry.clone())?;
    {let _arr = hashvec.clone(); _arr.borrow_mut()[(hash_idx.clone()-1) as usize] = cons((key.clone(), new_pos.clone()), indices.clone()); _arr};
    outHashTable = (hashvec.clone(), varr.clone(), bsize.clone(), fntpl.clone());
    Ok(outHashTable)
}

pub fn dumpHashTableStatistics<Key: Clone + 'static, Value: Clone + 'static>(mut hashTable: HashTable<Key, Value>) -> () {
    let _ = (match hashTable.clone() {
        (mut hvec, _, _, _) => {
            println!("{}", (literal!("index list lengths:\n")).clone());
            println!("{}", stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut l in (hvec.clone()).borrow().iter() {
            let __x = intString((l.clone().len() as i32));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(",")).clone()));
            println!("{}", (literal!("\n")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("non-zero: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", {
        let mut __acc: i32 = 0;
        for mut l in (hvec.clone()).borrow().iter() {
            if !(!(l.clone().is_empty())) { continue; }
            let __x = 1;
            __acc += __x;
        }
        __acc
    }))); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (hvec.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("max element: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", {
        let mut __acc: Option<i32> = None;
        for mut l in (hvec.clone()).borrow().iter() {
            let __x = (l.clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction")).unwrap()
    }))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("total entries: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", {
        let mut __acc: i32 = 0;
        for mut l in (hvec.clone()).borrow().iter() {
            let __x = (l.clone().len() as i32);
            __acc += __x;
        }
        __acc
    }))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            ()
        },
    });
    ()
}

pub fn addNoUpdCheck<Key: Clone + 'static, Value: Clone + 'static>(mut entry: HashEntry<Key, Value>, mut hashTable: HashTable<Key, Value>) -> Result<HashTable<Key, Value>> {
    let mut outHashTable: HashTable<Key, Value>;
    outHashTable = (match (entry.clone(), hashTable.clone()) {
        (ref v @ (ref key, _), (mut hashvec, mut varr, mut bsize, ref fntpl @ (ref hashFunc, _, _, _))) => {
            let mut indx: i32 = 0;
            let mut newpos: i32 = 0;
            let mut indexes: HashNode<Key> = metamodelica::nil();
            indx = intMod(hashFunc(key.clone())?, bsize.clone()) + 1;
            (varr, newpos) = valueArrayAdd(varr.clone(), v.clone())?;
            indexes = hashvec.borrow()[(indx.clone()-1) as usize].clone();
            hashvec = {let _arr = hashvec.clone(); _arr.borrow_mut()[(indx.clone()-1) as usize] = cons((key.clone(), newpos.clone()), indexes.clone()); _arr};
            (hashvec.clone(), varr.clone(), bsize.clone(), fntpl.clone())
        },
    });
    Ok(outHashTable)
}

pub fn addUnique<Key: Clone + 'static, Value: Clone + 'static>(mut entry: HashEntry<Key, Value>, mut hashTable: HashTable<Key, Value>) -> Result<HashTable<Key, Value>> {
    let mut outHashTable: HashTable<Key, Value>;
    let mut indx: i32 = 0;
    let mut newpos: i32 = 0;
    let mut bsize: i32 = 0;
    let mut varr: ValueArray<Key, Value>;
    let mut indexes: HashNode<Key> = metamodelica::nil();
    let mut hashvec: HashVector<Key>;
    let mut key: Key;
    let mut fntpl: FuncsTuple<Key, Value>;
    let mut hashFunc: FuncHash<Key>;
    (key, _) = entry.clone();
    let (__pa0, __pa1, __pa2, ref __pa4 @ (ref __pa3, _, _, _)) = hashTable.clone();
    hashvec = __pa0.clone();
    varr = __pa1.clone();
    bsize = __pa2.clone();
    hashFunc = __pa3.clone();
    fntpl = __pa4.clone();
    if '__try5: {
        let _ = unwrap_break_err!(get(key.clone(), hashTable.clone()), '__try5);
        Ok::<(), anyhow::Error>(())
    }.is_ok() { bail!("failure(): body succeeded") }
    indx = intMod(hashFunc(key.clone())?, bsize.clone()) + 1;
    (varr, newpos) = valueArrayAdd(varr.clone(), entry.clone())?;
    indexes = hashvec.borrow()[(indx.clone()-1) as usize].clone();
    hashvec = {let _arr = hashvec.clone(); _arr.borrow_mut()[(indx.clone()-1) as usize] = cons((key.clone(), newpos.clone()), indexes.clone()); _arr};
    outHashTable = (hashvec.clone(), varr.clone(), bsize.clone(), fntpl.clone());
    Ok(outHashTable)
}

pub fn update<Key: Clone + 'static, Value: Clone + 'static>(mut entry: HashEntry<Key, Value>, mut hashTable: HashTable<Key, Value>) -> Result<()> {
    let mut varr: ValueArray<Key, Value>;
    let mut index: i32 = 0;
    let mut key: Key;
    (key, _) = entry.clone();
    (_, varr, _, _) = hashTable.clone();
    index = hasKeyIndex(key.clone(), hashTable.clone());
    let true = (valueArrayKeyIndexExists(varr.clone(), index.clone())) else { bail!("pattern mismatch") };
    valueArraySet(varr.clone(), index.clone(), entry.clone())?;
    Ok(())
}

pub fn delete<Key: Clone + 'static, Value: Clone + 'static>(mut key: Key, mut hashTable: HashTable<Key, Value>) -> Result<()> {
    let mut indx: i32 = 0;
    let mut varr: ValueArray<Key, Value>;
    indx = hasKeyIndex(key.clone(), hashTable.clone());
    (_, varr, _, _) = hashTable.clone();
    if !(valueArrayKeyIndexExists(varr.clone(), indx.clone())) {
        println!("{}", (literal!("BaseHashTable.delete failed\n")).clone());
        bail!("fail");
    }
    valueArrayClear(varr.clone(), indx.clone())?;
    Ok(())
}

pub fn hasKey<Key: Clone + 'static, Value: Clone + 'static>(mut key: Key, mut hashTable: HashTable<Key, Value>) -> bool {
    let mut b: bool = false;
    let mut varr: ValueArray<Key, Value>;
    (_, varr, _, _) = hashTable.clone();
    b = valueArrayKeyIndexExists(varr.clone(), hasKeyIndex(key.clone(), hashTable.clone()));
    b
}

pub fn anyKeyInHashTable<Key: Clone + 'static, Value: Clone + 'static>(mut keys: Arc<metamodelica::List<Key>>, mut ht: HashTable<Key, Value>) -> bool {
    let mut res: bool = false;
    for mut key in &*keys.clone() {
        let mut key = key.clone();
        if hasKey(key.clone(), ht.clone()) {
            res = true;
            return res;
        }
    }
    res = false;
    res
}

pub fn get<Key: Clone + 'static, Value: Clone + 'static>(mut key: Key, mut hashTable: HashTable<Key, Value>) -> Result<Value> {
    let mut value: Value;
    let mut i: i32 = 0;
    let mut varr: ValueArray<Key, Value>;
    i = hasKeyIndex(key.clone(), hashTable.clone());
    let false = (i.clone() == -1) else { bail!("pattern mismatch") };
    (_, varr, _, _) = hashTable.clone();
    (_, value) = getValueArray(varr.clone(), i.clone())?;
    Ok(value)
}

pub fn getOrDefault<Key: Clone + 'static, Value: Clone + 'static>(mut key: Key, mut hashTable: HashTable<Key, Value>, mut default: Value) -> Result<Value> {
    let mut value: Value;
    value = if (hasKey(key.clone(), hashTable.clone())) {get(key.clone(), hashTable.clone())?} else {default.clone()};
    Ok(value)
}

fn hasKeyIndex<Key: Clone + 'static, Value: Clone + 'static>(mut key: Key, mut hashTable: HashTable<Key, Value>) -> i32 {
    let mut indx: i32 = 0;
    let mut hashindx: i32 = 0;
    let mut bsize: i32 = 0;
    let mut indexes: HashNode<Key> = metamodelica::nil();
    let mut hashvec: HashVector<Key>;
    let mut keyEqual: FuncEq<Key>;
    let mut hashFunc: FuncHash<Key>;
    let (__pa0, _, __pa1, (__pa2, __pa3, _, _)) = hashTable.clone();
    hashvec = __pa0.clone();
    bsize = __pa1.clone();
    hashFunc = __pa2.clone();
    keyEqual = __pa3.clone();
    hashindx = intMod(hashFunc(key.clone()).unwrap(), bsize.clone()) + 1;
    indexes = hashvec.borrow()[(hashindx.clone()-1) as usize].clone();
    indx = hasKeyIndex2(key.clone(), indexes.clone(), keyEqual);
    indx
}

fn hasKeyIndex2<Key: Clone + 'static>(mut key: Key, mut keyIndices: HashNode<Key>, mut keyEqual: FuncEq<Key>) -> i32 {
    let mut index: i32 = 0;
    let mut key2: Key;
    for mut keyIndex in &*keyIndices.clone() {
        let mut keyIndex = keyIndex.clone();
        (key2, index) = keyIndex.clone();
        if keyEqual(key.clone(), key2.clone()).unwrap() {
            return index;
        }
    }
    index = -1;
    index
}

pub fn dumpHashTable<Key: Clone + 'static, Value: Clone + 'static>(mut t: HashTable<Key, Value>) -> () {
    let mut printKey: FuncKeyString<Key>;
    let mut printValue: FuncValString<Value>;
    let mut k: Key;
    let mut v: Value;
    let (_, _, _, (_, _, __pa0, __pa1)) = t.clone();
    printKey = __pa0.clone();
    printValue = __pa1.clone();
    println!("{}", (literal!("HashTable:\n")).clone());
    for mut entry in &*hashTableList(t.clone()) {
        let mut entry = entry.clone();
        (k, v) = entry.clone();
        println!("{}", (literal!("{")).clone());
        println!("{}", (printKey(k.clone()).unwrap()).clone());
        println!("{}", (literal!(",{")).clone());
        println!("{}", (printValue(v.clone()).unwrap()).clone());
        println!("{}", (literal!("}}\n")).clone());
    }
    ()
}

pub fn debugDump<Key: Clone + 'static, Value: Clone + 'static>(mut ht: HashTable<Key, Value>) -> Result<()> {
    let mut printKey: FuncKeyString<Key>;
    let mut printValue: FuncValString<Value>;
    let mut k: Key;
    let mut v: Value;
    let mut n: i32 = 0;
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut szBucket: i32 = 0;
    let mut arr: metamodelica::Array<Option<(Key, Value)>>;
    let mut he: HashEntry<Key, Value>;
    let mut hashVector: metamodelica::Array<Arc<metamodelica::List<(Key, i32)>>>;
    let (__pa0, (__pa1, __pa2, __pa3), __pa4, (_, _, __pa5, __pa6)) = ht.clone();
    hashVector = __pa0.clone();
    n = __pa1.clone();
    size = __pa2.clone();
    arr = __pa3.clone();
    szBucket = __pa4.clone();
    printKey = __pa5.clone();
    printValue = __pa6.clone();
    println!("{}", (literal!("Debug HashTable:\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("szBucket: ")); __mm_s.push_str(&*intString(szBucket.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("Debug ValueArray:\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of entires: ")); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size: ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    i = 0;
    let __range7 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut entry in __range7 {
        i = i.clone() + 1;
        if isSome(entry.clone()) {
            let Some(__pa8) = (entry.clone()) else { bail!("pattern mismatch") };
            he = __pa8.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*dumpTuple(he.clone(), printKey, printValue)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    println!("{}", (literal!("Debug HashVector:\n")).clone());
    i = 0;
    let __range9 = hashVector.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut node in __range9 {
        i = i.clone() + 1;
        if !(node.clone().is_empty()) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone());
            for mut n in &*node.clone() {
                let mut n = n.clone();
                (k, j) = n.clone();
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" {")); __mm_s.push_str(&*printKey(k.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(j.clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone());
            }
            println!("{}", (literal!("\n")).clone());
        }
    }
    Ok(())
}

fn dumpTuple<Key: Clone + 'static, Value: Clone + 'static>(mut tpl: HashEntry<Key, Value>, mut printKey: FuncKeyString<Key>, mut printValue: FuncValString<Value>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut k: Key;
    let mut v: Value;
    let mut sk: ArcStr = arcstr::literal!("");
    let mut sv: ArcStr = arcstr::literal!("");
    (k, v) = tpl.clone();
    sk = (printKey(k.clone()).unwrap()).clone();
    sv = (printValue(v.clone()).unwrap()).clone();
    r#str = stringAppendList(list![(literal!("{")).clone(), (sk.clone()).clone(), (literal!(",{")).clone(), (sv.clone()).clone(), (literal!("}}")).clone()]);
    r#str
}

pub fn hashTableValueList<Key: Clone + 'static, Value: Clone + 'static>(mut hashTable: HashTable<Key, Value>) -> Arc<metamodelica::List<Value>> {
    let mut valLst: Arc<metamodelica::List<Value>> = metamodelica::nil();
    valLst = List::unzipSecond(hashTableList(hashTable.clone()));
    valLst
}

pub fn hashTableKeyList<Key: Clone + 'static, Value: Clone + 'static>(mut hashTable: HashTable<Key, Value>) -> Arc<metamodelica::List<Key>> {
    let mut valLst: Arc<metamodelica::List<Key>> = metamodelica::nil();
    (valLst, _) = List::unzip(hashTableList(hashTable.clone()));
    valLst
}

pub fn hashTableList<Key: Clone + 'static, Value: Clone + 'static>(mut hashTable: HashTable<Key, Value>) -> Arc<metamodelica::List<(Key, Value)>> {
    let mut outEntries: Arc<metamodelica::List<(Key, Value)>> = metamodelica::nil();
    let mut varr: ValueArray<Key, Value>;
    (_, varr, _, _) = hashTable.clone();
    outEntries = valueArrayList(varr.clone());
    outEntries
}

pub fn hashTableListReversed<Key: Clone + 'static, Value: Clone + 'static>(mut hashTable: HashTable<Key, Value>) -> Arc<metamodelica::List<(Key, Value)>> {
    let mut entries: Arc<metamodelica::List<(Key, Value)>> = metamodelica::nil();
    let mut varr: ValueArray<Key, Value>;
    (_, varr, _, _) = hashTable.clone();
    entries = valueArrayListReversed(varr.clone());
    entries
}

pub fn valueArrayList<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>) -> Arc<metamodelica::List<(Key, Value)>> {
    let mut outEntries: Arc<metamodelica::List<(Key, Value)>> = metamodelica::nil();
    let mut arr: metamodelica::Array<Option<(Key, Value)>>;
    (_, _, arr) = valueArray.clone();
    outEntries = Array::fold(arr.clone(), Arc::new(fnptr!(List::consOption, _, _)), metamodelica::nil());
    outEntries = outEntries.clone().reverse();
    outEntries
}

pub fn valueArrayListReversed<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>) -> Arc<metamodelica::List<(Key, Value)>> {
    let mut entries: Arc<metamodelica::List<(Key, Value)>> = metamodelica::nil();
    let mut arr: metamodelica::Array<Option<(Key, Value)>>;
    (_, _, arr) = valueArray.clone();
    entries = Array::fold(arr.clone(), Arc::new(fnptr!(List::consOption, _, _)), metamodelica::nil());
    entries
}

pub fn hashTableCurrentSize<Key: Clone + 'static, Value: Clone + 'static>(mut hashTable: HashTable<Key, Value>) -> i32 {
    let mut sz: i32 = 0;
    let mut va: ValueArray<Key, Value>;
    (_, va, _, _) = hashTable.clone();
    sz = valueArrayLength(va.clone());
    sz
}

pub fn valueArrayLength<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>) -> i32 {
    let mut sz: i32 = 0;
    (sz, _, _) = valueArray.clone();
    sz
}

fn valueArrayAdd<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>, mut entry: HashEntry<Key, Value>) -> Result<(ValueArray<Key, Value>, i32)> {
    let mut outValueArray: ValueArray<Key, Value>;
    let mut newpos: i32 = 0;
    let mut n_1: i32 = 0;
    let mut n: i32 = 0;
    let mut size: i32 = 0;
    let mut expandsize: i32 = 0;
    let mut expandsize_1: i32 = 0;
    let mut newsize: i32 = 0;
    let mut arr_1: metamodelica::Array<Option<(Key, Value)>>;
    let mut arr: metamodelica::Array<Option<(Key, Value)>>;
    let mut arr_2: metamodelica::Array<Option<(Key, Value)>>;
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
    newpos = n.clone() + 1;
    Ok((outValueArray, newpos))
}

fn valueArraySet<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>, mut pos: i32, mut entry: HashEntry<Key, Value>) -> Result<ValueArray<Key, Value>> {
    let mut outValueArray: ValueArray<Key, Value>;
    outValueArray = (match (valueArray.clone(), pos.clone(), entry.clone()) {
        ((mut n, mut size, mut arr), _, _) => {
            let true = (pos.clone() <= size.clone()) else { bail!("pattern mismatch") };
            arr = {let _arr = arr.clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = Some(entry.clone()); _arr};
            (n.clone(), size.clone(), arr.clone())
        },
    });
    Ok(outValueArray)
}

fn valueArrayClear<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>, mut pos: i32) -> Result<()> {
    let mut arr: metamodelica::Array<Option<(Key, Value)>>;
    let mut size: i32 = 0;
    (_, size, arr) = valueArray.clone();
    let true = (pos.clone() <= size.clone()) else { bail!("pattern mismatch") };
    {let _arr = arr.clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = None; _arr};
    Ok(())
}

fn getValueArray<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>, mut pos: i32) -> Result<(Key, Value)> {
    let mut key: Key;
    let mut value: Value;
    let mut arr: metamodelica::Array<Option<(Key, Value)>>;
    let mut n: i32 = 0;
    (n, _, arr) = valueArray.clone();
    let true = (pos.clone() <= n.clone()) else { bail!("pattern mismatch") };
    let Some((__pa0, __pa1)) = (arr.clone().borrow()[(pos.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
    key = __pa0.clone();
    value = __pa1.clone();
    Ok((key, value))
}

fn valueArrayKeyIndexExists<Key: Clone + 'static, Value: Clone + 'static>(mut valueArray: ValueArray<Key, Value>, mut pos: i32) -> bool {
    let mut b: bool = false;
    b = (match (valueArray.clone(), pos.clone()) {
        (_, (-1)) => {
            false
        },
        ((mut n, _, mut arr), _) => {
            if (pos.clone() <= n.clone()) {isSome(arr.borrow()[(pos.clone()-1) as usize].clone())} else {false}
        },
    });
    b
}

pub fn copy<Key: Clone + 'static, Value: Clone + 'static>(mut inHashTable: HashTable<Key, Value>) -> HashTable<Key, Value> {
    let mut outCopy: HashTable<Key, Value>;
    let mut hv: HashVector<Key>;
    let mut bs: i32 = 0;
    let mut sz: i32 = 0;
    let mut vs: i32 = 0;
    let mut ve: i32 = 0;
    let mut ft: FuncsTuple<Key, Value>;
    let mut vae: metamodelica::Array<Option<(Key, Value)>>;
    let (__pa0, (__pa1, __pa2, __pa3), __pa4, __pa5) = inHashTable.clone();
    hv = __pa0.clone();
    vs = __pa1.clone();
    ve = __pa2.clone();
    vae = __pa3.clone();
    bs = __pa4.clone();
    ft = __pa5.clone();
    hv = metamodelica::arrayFromVec(hv.clone().borrow().clone());
    vae = metamodelica::arrayFromVec(vae.clone().borrow().clone());
    outCopy = (hv.clone(), (vs.clone(), ve.clone(), vae.clone()), bs.clone(), ft.clone());
    outCopy
}

pub fn clear<Key: Clone + 'static, Value: Clone + 'static>(mut ht: HashTable<Key, Value>) -> Result<HashTable<Key, Value>> {
    let mut ht: HashTable<Key, Value> = ht;
    let mut hv: HashVector<Key>;
    let mut bs: i32 = 0;
    let mut sz: i32 = 0;
    let mut vs: i32 = 0;
    let mut ve: i32 = 0;
    let mut hash_idx: i32 = 0;
    let mut ft: FuncsTuple<Key, Value>;
    let mut hashFunc: FuncHash<Key>;
    let mut key: Key;
    let mut vae: metamodelica::Array<Option<(Key, Value)>>;
    let (__pa0, (__pa1, __pa2, __pa3), __pa4, ref __pa6 @ (ref __pa5, _, _, _)) = ht.clone();
    hv = __pa0.clone();
    vs = __pa1.clone();
    ve = __pa2.clone();
    vae = __pa3.clone();
    bs = __pa4.clone();
    hashFunc = __pa5.clone();
    ft = __pa6.clone();
    for mut i in 1..=vs.clone() {
        let _ = (match vae.clone().borrow()[(i.clone()-1) as usize].clone() {
        Some((mut key, _)) => {
            hash_idx = intMod(hashFunc(key.clone())?, bs.clone()) + 1;
            {let _arr = hv.clone(); _arr.borrow_mut()[(hash_idx.clone()-1) as usize] = metamodelica::nil(); _arr};
            {let _arr = vae.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = None; _arr};
            ()
        },
        _ => (),
    });
    }
    ht = (hv.clone(), (0, ve.clone(), vae.clone()), bs.clone(), ft.clone());
    Ok(ht)
}

pub fn clearAssumeNoDelete<Key: Clone + 'static, Value: Clone + 'static>(mut ht: HashTable<Key, Value>) -> Result<()> {
    let mut hv: HashVector<Key>;
    let mut bs: i32 = 0;
    let mut sz: i32 = 0;
    let mut vs: i32 = 0;
    let mut ve: i32 = 0;
    let mut hash_idx: i32 = 0;
    let mut ft: FuncsTuple<Key, Value>;
    let mut hashFunc: FuncHash<Key>;
    let mut key: Key;
    let mut vae: metamodelica::Array<Option<(Key, Value)>>;
    let workaroundForBug: bool = true;
    let debug: bool = false;
    let (__pa0, (__pa1, __pa2, __pa3), __pa4, ref __pa6 @ (ref __pa5, _, _, _)) = ht.clone();
    hv = __pa0.clone();
    vs = __pa1.clone();
    ve = __pa2.clone();
    vae = __pa3.clone();
    bs = __pa4.clone();
    hashFunc = __pa5.clone();
    ft = __pa6.clone();
    for mut i in 1..=ve.clone() {
        let _ = (match vae.clone().borrow()[(i.clone()-1) as usize].clone() {
        Some((mut key, _)) => {
            if !(workaroundForBug.clone()) {
                hash_idx = intMod(hashFunc(key.clone())?, bs.clone()) + 1;
                {let _arr = hv.clone(); _arr.borrow_mut()[(hash_idx.clone()-1) as usize] = metamodelica::nil(); _arr};
            }
            {let _arr = vae.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = None; _arr};
            ()
        },
        _ => {
            if !(workaroundForBug.clone()) {
                return Ok(());
            }
            ()
        },
    });
    }
    if debug.clone() {
        let __range7 = vae.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut i in __range7 {
            if isSome(i.clone()) {
                println!("{}", (literal!("vae not empty\n")).clone());
                break;
            }
        }
    }
    if workaroundForBug.clone() {
        let __range8 = 1..=(hv.clone().borrow().len() as i32);
        for mut i in __range8 {
            if !(hv.clone().borrow()[(i.clone()-1) as usize].clone().is_empty()) {
                if debug.clone() {
                    println!("{}", (literal!("hv not empty\n")).clone());
                }
                {let _arr = hv.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = metamodelica::nil(); _arr};
            }
        }
    }
    Ok(())
}

