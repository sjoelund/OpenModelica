use anyhow::Result;
use std::sync::Arc;
use metamodelica::*;
use openmodelica_util_datatypes_basic::DoubleEnded;

#[test]
fn test_new() {
    let de = DoubleEnded::new(42);
    assert_eq!(DoubleEnded::length(de), 1);
}

#[test]
fn test_empty() {
    let de = DoubleEnded::empty(0i32);
    assert_eq!(DoubleEnded::length(de), 0);
}

#[test]
fn test_push_back() {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    DoubleEnded::push_back(de.clone(), 3);
    assert_eq!(DoubleEnded::length(de), 3);
}

#[test]
fn test_push_back_empty() -> Result<()> {
    let de = DoubleEnded::empty(0i32);
    DoubleEnded::push_back(de.clone(), 42);
    assert_eq!(DoubleEnded::length(de.clone()), 1);
    let val = DoubleEnded::pop_front(de)?;
    assert_eq!(val, 42);
    Ok(())
}

#[test]
fn test_push_front() -> Result<()> {
    let de = DoubleEnded::new(2);
    DoubleEnded::push_front(de.clone(), 1);
    DoubleEnded::push_front(de.clone(), 0);
    assert_eq!(DoubleEnded::length(de.clone()), 3);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 0);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 1);
    assert_eq!(DoubleEnded::pop_front(de)?, 2);
    Ok(())
}

#[test]
fn test_push_front_empty() -> Result<()> {
    let de = DoubleEnded::empty(0i32);
    DoubleEnded::push_front(de.clone(), 42);
    assert_eq!(DoubleEnded::length(de.clone()), 1);
    let val = DoubleEnded::pop_front(de)?;
    assert_eq!(val, 42);
    Ok(())
}

#[test]
fn test_pop_front() -> Result<()> {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    DoubleEnded::push_back(de.clone(), 3);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 1);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 2);
    assert_eq!(DoubleEnded::pop_front(de)?, 3);
    Ok(())
}

#[test]
fn test_pop_front_last_element() -> Result<()> {
    let de = DoubleEnded::new(42);
    let val = DoubleEnded::pop_front(de.clone())?;
    assert_eq!(val, 42);
    assert_eq!(DoubleEnded::length(de), 0);
    Ok(())
}

#[test]
fn test_pop_front_empty_fails() {
    let de = DoubleEnded::empty(0i32);
    let result = DoubleEnded::pop_front(de);
    assert!(result.is_err());
}

#[test]
fn test_from_list() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let de = DoubleEnded::fromList(Arc::clone(&lst))?;
    assert_eq!(DoubleEnded::length(de.clone()), 3);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 1);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 2);
    assert_eq!(DoubleEnded::pop_front(de)?, 3);
    Ok(())
}

#[test]
fn test_from_list_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let de = DoubleEnded::fromList(lst)?;
    assert_eq!(DoubleEnded::length(de), 0);
    Ok(())
}

#[test]
fn test_length() -> Result<()> {
    let de = DoubleEnded::new(1);
    assert_eq!(DoubleEnded::length(de.clone()), 1);
    DoubleEnded::push_back(de.clone(), 2);
    assert_eq!(DoubleEnded::length(de.clone()), 2);
    DoubleEnded::pop_front(de.clone())?;
    assert_eq!(DoubleEnded::length(de), 1);
    Ok(())
}

#[test]
fn test_clear() -> Result<()> {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    DoubleEnded::clear(de.clone());
    assert_eq!(DoubleEnded::length(de.clone()), 0);
    let result = DoubleEnded::pop_front(de);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_to_list_and_clear() -> Result<()> {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    DoubleEnded::push_back(de.clone(), 3);
    let prepend: Arc<List<i32>> = list![0i32];
    let result = DoubleEnded::toListAndClear(de.clone(), Arc::clone(&prepend));
    assert_eq!(result.get(1)?, 1);
    assert_eq!(result.get(2)?, 2);
    assert_eq!(result.get(3)?, 3);
    assert_eq!(result.get(4)?, 0);
    assert_eq!(DoubleEnded::length(de), 0);
    Ok(())
}

#[test]
fn test_to_list_and_clear_empty() {
    let de = DoubleEnded::empty(0i32);
    let prepend = list![1i32, 2];
    let result = DoubleEnded::toListAndClear(de, Arc::clone(&prepend));
    assert_eq!(result, prepend);
}

#[test]
fn test_to_list_no_copy_no_clear() -> Result<()> {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    let result = DoubleEnded::toListNoCopyNoClear(de.clone());
    assert_eq!(result.get(1)?, 1);
    assert_eq!(result.get(2)?, 2);
    assert_eq!(DoubleEnded::length(de), 2);
    Ok(())
}

#[test]
fn test_current_back_cell() -> Result<()> {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    let back = DoubleEnded::currentBackCell(de);
    assert_eq!(back.get(1)?, 2);
    Ok(())
}

#[test]
fn test_push_list_back() -> Result<()> {
    let de = DoubleEnded::new(1);
    let lst = list![2i32, 3, 4];
    DoubleEnded::push_list_back(de.clone(), Arc::clone(&lst));
    assert_eq!(DoubleEnded::length(de.clone()), 4);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 1);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 2);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 3);
    assert_eq!(DoubleEnded::pop_front(de)?, 4);
    Ok(())
}

#[test]
fn test_push_list_back_empty_de() {
    let de = DoubleEnded::empty(0i32);
    let lst = list![1i32, 2];
    DoubleEnded::push_list_back(de.clone(), Arc::clone(&lst));
    assert_eq!(DoubleEnded::length(de), 2);
}

#[test]
fn test_push_list_back_empty_list() {
    let de = DoubleEnded::new(1);
    let lst: Arc<List<i32>> = nil();
    DoubleEnded::push_list_back(de.clone(), lst);
    assert_eq!(DoubleEnded::length(de), 1);
}

#[test]
fn test_push_list_front() -> Result<()> {
    let de = DoubleEnded::new(4);
    let lst = list![1i32, 2, 3];
    DoubleEnded::push_list_front(de.clone(), Arc::clone(&lst))?;
    assert_eq!(DoubleEnded::length(de.clone()), 4);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 1);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 2);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 3);
    assert_eq!(DoubleEnded::pop_front(de)?, 4);
    Ok(())
}

#[test]
fn test_push_list_front_empty_de() -> Result<()> {
    let de = DoubleEnded::empty(0i32);
    let lst = list![1i32, 2];
    DoubleEnded::push_list_front(de.clone(), Arc::clone(&lst))?;
    assert_eq!(DoubleEnded::length(de), 2);
    Ok(())
}

#[test]
fn test_push_list_front_empty_list() -> Result<()> {
    let de = DoubleEnded::new(1);
    let lst: Arc<List<i32>> = nil();
    DoubleEnded::push_list_front(de.clone(), lst)?;
    assert_eq!(DoubleEnded::length(de), 1);
    Ok(())
}

#[test]
fn test_map_fold_no_copy() -> Result<()> {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    DoubleEnded::push_back(de.clone(), 3);
    let result = DoubleEnded::mapFoldNoCopy(
        de.clone(),
        &|x, acc: i32| Ok((x * 10, acc + x)),
        0i32
    )?;
    assert_eq!(result, 6);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 10);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 20);
    assert_eq!(DoubleEnded::pop_front(de)?, 30);
    Ok(())
}

#[test]
fn test_map_no_copy_1() -> Result<()> {
    let de = DoubleEnded::new(1);
    DoubleEnded::push_back(de.clone(), 2);
    DoubleEnded::push_back(de.clone(), 3);
    DoubleEnded::mapNoCopy_1(
        de.clone(),
        &|x, _arg: i32| Ok(x * 2),
        0i32
    )?;
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 2);
    assert_eq!(DoubleEnded::pop_front(de.clone())?, 4);
    assert_eq!(DoubleEnded::pop_front(de)?, 6);
    Ok(())
}
