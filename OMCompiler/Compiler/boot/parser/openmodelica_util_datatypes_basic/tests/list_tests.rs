use anyhow::Result;
use std::sync::Arc;
use metamodelica::*;
use arcstr::{ArcStr, literal};
use openmodelica_util_datatypes_basic::List as L;

// ── helper predicates (must be fn pointers) ──
fn is_positive(x: i32) -> Result<bool> { Ok(x > 0) }
fn is_even(x: i32) -> Result<bool> { Ok(x % 2 == 0) }
fn always_true(_: i32) -> Result<bool> { Ok(true) }
fn always_false(_: i32) -> Result<bool> { Ok(false) }
fn double(x: i32) -> Result<i32> { Ok(x * 2) }
fn square(x: i32) -> Result<i32> { Ok(x * x) }
fn inc(x: i32) -> Result<i32> { Ok(x + 1) }
fn to_string_i32(x: i32) -> Result<ArcStr> { Ok(arcstr::format!("{}", x)) }
fn add_i(a: i32, b: i32) -> Result<i32> { Ok(a + b) }
fn mul_i(a: i32, b: i32) -> Result<i32> { Ok(a * b) }
fn less_i(a: i32, b: i32) -> Result<bool> { Ok(a < b) }
fn eq_i(a: i32, b: i32) -> Result<bool> { Ok(a == b) }
fn cmp_i(a: i32, b: i32) -> Result<i32> { Ok(if a < b { -1 } else if a > b { 1 } else { 0 }) }
fn is_gt_5(x: i32) -> Result<bool> { Ok(x > 5) }
fn is_gt_3(x: i32) -> Result<bool> { Ok(x > 3) }
fn abs_val(x: i32) -> Result<i32> { Ok(x.abs()) }
fn half(x: i32) -> Result<i32> { Ok(x / 2) }
fn id_i32(x: i32) -> Result<i32> { Ok(x) }

// ── AccumulateMapAccum ──
#[test]
fn test_accumulate_map_accum() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (result, acc) = L::accumulateMapAccum(Arc::clone(&lst), 0i32, |x, a| Ok((x + a, a + 1)))?;
    assert_eq!(*result, list![1i32, 3, 6]);
    assert_eq!(acc, 3);
    Ok(())
}

// ── All ──
#[test]
fn test_all_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(L::all(Arc::clone(&lst), is_positive)?);
    Ok(())
}
#[test]
fn test_all_false() -> Result<()> {
    let lst = list![1i32, -2, 3];
    assert!(!L::all(Arc::clone(&lst), is_positive)?);
    Ok(())
}
#[test]
fn test_all_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    assert!(L::all(Arc::clone(&lst), is_positive)?);
    Ok(())
}

// ── AllEqual ──
#[test]
fn test_all_equal_true() -> Result<()> {
    let lst = list![5i32, 5, 5];
    assert!(L::allEqual(Arc::clone(&lst))?);
    Ok(())
}
#[test]
fn test_all_equal_false() -> Result<()> {
    let lst = list![5i32, 3, 5];
    assert!(!L::allEqual(Arc::clone(&lst))?);
    Ok(())
}
#[test]
fn test_all_equal_single() -> Result<()> {
    let lst = list![1i32];
    assert!(L::allEqual(Arc::clone(&lst))?);
    Ok(())
}
#[test]
fn test_all_equal_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    assert!(L::allEqual(Arc::clone(&lst))?);
    Ok(())
}

// ── AllReferenceEq ──
#[test]
fn test_all_reference_eq_true() -> Result<()> {
    let inner: Arc<List<i32>> = list![1i32];
    let lst = list![Arc::clone(&inner), Arc::clone(&inner)];
    assert!(L::allReferenceEq(Arc::clone(&lst))?);
    Ok(())
}
#[test]
fn test_all_reference_eq_false() -> Result<()> {
    let lst = list![list![1i32], list![1i32]];
    assert!(!L::allReferenceEq(Arc::clone(&lst))?);
    Ok(())
}

// ── Any ──
#[test]
fn test_any_found() -> Result<()> {
    let lst = list![-1i32, 2, -3];
    assert!(L::any(Arc::clone(&lst), is_positive)?);
    Ok(())
}
#[test]
fn test_any_none() -> Result<()> {
    let lst = list![-1i32, -2];
    assert!(!L::any(Arc::clone(&lst), is_positive)?);
    Ok(())
}
#[test]
fn test_any_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    assert!(!L::any(Arc::clone(&lst), is_positive)?);
    Ok(())
}

// ── AppendElt ──
#[test]
fn test_append_elt() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::appendElt(Arc::clone(&lst), 3)?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── AppendLastList ──
#[test]
fn test_append_last_list() -> Result<()> {
    let lst1 = list![1i32, 2];
    let lst2 = list![3i32, 4];
    let list_of_lists = list![Arc::clone(&lst1), Arc::clone(&lst2)];
    let result = L::appendLastList(list_of_lists)?;
    assert_eq!(result, list![1i32, 2, 3, 4]);
    Ok(())
}

// ── Append_reverse ──
#[test]
fn test_append_reverse() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::append_reverse(Arc::clone(&lst), nil())?;
    assert_eq!(result, list![3i32, 2, 1]);
    Ok(())
}

// ── ApplyAndFold ──
#[test]
fn test_apply_and_fold() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::applyAndFold(Arc::clone(&lst), 0i32, |x, acc| Ok(acc + x))?;
    assert_eq!(result, 6);
    Ok(())
}

// ── ApplyAndFold1 ──
#[test]
fn test_apply_and_fold1() -> Result<()> {
    let lst = list![1i32];
    let result = L::applyAndFold1(Arc::clone(&lst), 10i32, |x, acc| Ok(acc + x))?;
    assert_eq!(result, 11);
    Ok(())
}

// ── BalancedPartition ──
#[test]
fn test_balanced_partition() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let (a, b) = L::balancedPartition(Arc::clone(&lst))?;
    assert_eq!(*a, list![1i32, 2]);
    assert_eq!(*b, list![3i32, 4, 5]);
    Ok(())
}

// ── Combination ──
#[test]
fn test_combination() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::combination(Arc::clone(&lst), 2)?;
    // Combinations of 2 from [1,2,3]: [1,2], [1,3], [2,3]
    assert_eq!(result.len(), 3);
    Ok(())
}

// ── CombinationMap ──
#[test]
fn test_combination_map() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::combinationMap(Arc::clone(&lst), 2, |pair| {
        let a = pair.get(1)?;
        let b = pair.get(2)?;
        Ok::<i32, anyhow::Error>(a + b)
    })?;
    assert_eq!(*result, list![3i32, 4, 5]);
    Ok(())
}

// ── Compare ──
#[test]
fn test_compare_equal() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![1i32, 2, 3];
    assert_eq!(L::compare(Arc::clone(&a), Arc::clone(&b), cmp_i)?, 0);
    Ok(())
}
#[test]
fn test_compare_less() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![1i32, 3];
    assert_eq!(L::compare(Arc::clone(&a), Arc::clone(&b), cmp_i)?, -1);
    Ok(())
}
#[test]
fn test_compare_greater() -> Result<()> {
    let a = list![1i32, 4];
    let b = list![1i32, 3];
    assert_eq!(L::compare(Arc::clone(&a), Arc::clone(&b), cmp_i)?, 1);
    Ok(())
}

// ── CompareLength ──
#[test]
fn test_compare_length() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![1i32, 2, 3];
    assert!(L::compareLength(Arc::clone(&a), Arc::clone(&b))? < 0);
    assert_eq!(L::compareLength(Arc::clone(&b), Arc::clone(&b))? , 0);
    assert!(L::compareLength(Arc::clone(&b), Arc::clone(&a))? > 0);
    Ok(())
}

// ── ConsN ──
#[test]
fn test_cons_n() -> Result<()> {
    let result = L::consN(42i32, 3)?;
    assert_eq!(result, list![42i32, 42, 42]);
    Ok(())
}
#[test]
fn test_cons_n_zero() -> Result<()> {
    let result = L::consN(42i32, 0)?;
    assert!(result.is_empty());
    Ok(())
}

// ── ConsOnTrue ──
#[test]
fn test_cons_on_true_true() -> Result<()> {
    let result = L::consOnTrue(1i32, true)?;
    assert_eq!(result, list![1i32]);
    Ok(())
}
#[test]
fn test_cons_on_true_false() -> Result<()> {
    let result = L::consOnTrue(1i32, false)?;
    assert!(result.is_empty());
    Ok(())
}

// ── ConsOption ──
#[test]
fn test_cons_option_some() -> Result<()> {
    let result = L::consOption(Some(1i32))?;
    assert_eq!(result, list![1i32]);
    Ok(())
}
#[test]
fn test_cons_option_none() -> Result<()> {
    let result = L::consOption(Option::<i32>::None)?;
    assert!(result.is_empty());
    Ok(())
}

// ── Consr ──
#[test]
fn test_consr() -> Result<()> {
    let lst = list![2i32, 3];
    let result = L::consr(1i32, Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── Contains ──
#[test]
fn test_contains_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(L::contains(Arc::clone(&lst), 2)?);
    Ok(())
}
#[test]
fn test_contains_false() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(!L::contains(Arc::clone(&lst), 4)?);
    Ok(())
}

// ── Count ──
#[test]
fn test_count() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5, 6];
    assert_eq!(L::count(Arc::clone(&lst), is_even)?, 3);
    Ok(())
}

// ── CountingSort ──
#[test]
fn test_counting_sort() -> Result<()> {
    let lst = list![3i32, 1, 4, 1, 5, 9, 2, 6];
    let result = L::countingSort(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 1, 2, 3, 4, 5, 6, 9]);
    Ok(())
}

// ── Create ──
#[test]
fn test_create() -> Result<()> {
    let result = L::create(5, 0i32)?;
    assert_eq!(result, list![0i32, 0, 0, 0, 0]);
    Ok(())
}

// ── DeleteMemberOnTrue ──
#[test]
fn test_delete_member_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 2, 4];
    let result = L::deleteMemberOnTrue(2i32, Arc::clone(&lst), eq_i)?;
    assert_eq!(result, list![1i32, 3, 4]);
    Ok(())
}

// ── DeletePositions ──
#[test]
fn test_delete_positions() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let positions = list![2i32, 4];
    let result = L::deletePositions(Arc::clone(&lst), Arc::clone(&positions))?;
    assert_eq!(result, list![1i32, 3, 5]);
    Ok(())
}

// ── DeletePositionsSorted ──
#[test]
fn test_delete_positions_sorted() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let positions = list![1i32, 3, 5];
    let result = L::deletePositionsSorted(Arc::clone(&lst), Arc::clone(&positions))?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── Exist1 ──
#[test]
fn test_exist1_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(L::exist1(Arc::clone(&lst), is_positive)?);
    Ok(())
}
#[test]
fn test_exist1_false() -> Result<()> {
    let lst = list![-1i32, -2];
    assert!(!L::exist1(Arc::clone(&lst), is_positive)?);
    Ok(())
}

// ── ExtractOnTrue ──
#[test]
fn test_extract_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::extractOnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── Extract1OnTrue ──
#[test]
fn test_extract1_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::extract1OnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32]);
    Ok(())
}

// ── Fill ──
#[test]
fn test_fill() -> Result<()> {
    let result = L::fill(42i32, 5)?;
    assert_eq!(result, list![42i32, 42, 42, 42, 42]);
    Ok(())
}

// ── Filter ──
#[test]
fn test_filter() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5, 6];
    let result = L::filter(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}

// ── Filter1 ──
#[test]
fn test_filter1() -> Result<()> {
    let lst = list![1i32];
    let result = L::filter1(Arc::clone(&lst), is_positive)?;
    assert_eq!(result, list![1i32]);
    Ok(())
}

// ── Filter1OnTrue ──
#[test]
fn test_filter1_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::filter1OnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32]);
    Ok(())
}

// ── Filter1OnTrueAndUpdate ──
#[test]
fn test_filter1_on_true_and_update() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (kept, removed) = L::filter1OnTrueAndUpdate(Arc::clone(&lst), |x| Ok((x > 1, x)))?;
    assert_eq!(kept, list![2i32, 3]);
    assert_eq!(removed, list![1i32]);
    Ok(())
}

// ── Filter1OnTrueSync ──
#[test]
fn test_filter1_on_true_sync() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::filter1OnTrueSync(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32]);
    Ok(())
}

// ── Filter1rOnTrue ──
#[test]
fn test_filter1r_on_true() -> Result<()> {
    let lst = list![2i32, 4, 6];
    let result = L::filter1rOnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}

// ── Filter2OnTrue ──
#[test]
fn test_filter2_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::filter2OnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── FilterCons ──
#[test]
fn test_filter_cons() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::filterCons(Arc::clone(&lst), is_even)?;
    // Should keep elements where predicate is true
    assert_eq!(result, list![2i32]);
    Ok(())
}

// ── FilterMap ──
#[test]
fn test_filter_map() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::filterMap(Arc::clone(&lst), |x| Ok(if x % 2 == 0 { Some(x * 10) } else { None }))?;
    assert_eq!(result, list![20i32, 40]);
    Ok(())
}

// ── FilterMap1 ──
#[test]
fn test_filter_map1() -> Result<()> {
    let lst = list![2i32];
    let result = L::filterMap1(Arc::clone(&lst), |x| Ok(if x > 0 { Some(x * 2) } else { None }))?;
    assert_eq!(result, list![4i32]);
    Ok(())
}

// ── FilterOnFalse ──
#[test]
fn test_filter_on_false() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::filterOnFalse(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![1i32, 3]);
    Ok(())
}

// ── FilterOnTrue ──
#[test]
fn test_filter_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::filterOnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── FilterOnTrueSync ──
#[test]
fn test_filter_on_true_sync() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::filterOnTrueSync(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── Find ──
#[test]
fn test_find_found() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert_eq!(L::find(Arc::clone(&lst), 2)?, Some(2));
    Ok(())
}
#[test]
fn test_find_not_found() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert_eq!(L::find(Arc::clone(&lst), 4)?, None);
    Ok(())
}

// ── Find1 ──
#[test]
fn test_find1_found() -> Result<()> {
    let lst = list![1i32];
    assert_eq!(L::find1(Arc::clone(&lst))? , Some(1));
    Ok(())
}

// ── FindAndMap ──
#[test]
fn test_find_and_map() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::findAndMap(Arc::clone(&lst), |x| Ok(if x > 1 { Some(x * 10) } else { None }))?;
    assert_eq!(result, Some(20));
    Ok(())
}

// ── FindAndRemove ──
#[test]
fn test_find_and_remove() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (found, rest) = L::findAndRemove(Arc::clone(&lst), |x| Ok(x == 2))?;
    assert_eq!(found, true);
    assert_eq!(rest, list![1i32, 3]);
    Ok(())
}

// ── FindAndRemove1 ──
#[test]
fn test_find_and_remove1() -> Result<()> {
    let lst = list![1i32, 2];
    let (found, rest) = L::findAndRemove1(Arc::clone(&lst), |x| Ok(x == 2))?;
    assert_eq!(found, true);
    assert_eq!(rest, list![1i32]);
    Ok(())
}

// ── FindBoolList ──
#[test]
fn test_find_bool_list() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::findBoolList(Arc::clone(&lst), is_positive)?;
    assert_eq!(result, list![true, true, true]);
    Ok(())
}

// ── FindMap ──
#[test]
fn test_find_map() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::findMap(Arc::clone(&lst), double)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}

// ── FindOption ──
#[test]
fn test_find_option_some() -> Result<()> {
    let lst = list![Some(1i32), None, Some(3)];
    let result = L::findOption(Arc::clone(&lst))?;
    assert_eq!(result, Some(1));
    Ok(())
}
#[test]
fn test_find_option_none() -> Result<()> {
    let lst: Arc<List<Option<i32>>> = nil();
    let result = L::findOption(Arc::clone(&lst))?;
    assert_eq!(result, None);
    Ok(())
}

// ── FindSome ──
#[test]
fn test_find_some() -> Result<()> {
    let lst = list![Some(1i32), None, Some(3)];
    assert!(L::findSome(Arc::clone(&lst))?);
    Ok(())
}

// ── FirstN ──
#[test]
fn test_first_n() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::firstN(Arc::clone(&lst), 3)?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}
#[test]
fn test_first_n_more_than_length() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::firstN(Arc::clone(&lst), 5)?;
    assert_eq!(result, list![1i32, 2]);
    Ok(())
}

// ── FirstOrEmpty ──
#[test]
fn test_first_or_empty_some() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::firstOrEmpty(Arc::clone(&lst));
    assert_eq!(result, Some(1));
    Ok(())
}
#[test]
fn test_first_or_empty_none() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::firstOrEmpty(Arc::clone(&lst));
    assert_eq!(result, None);
    Ok(())
}

// ── Flatten ──
#[test]
fn test_flatten() -> Result<()> {
    let lst = list![list![1i32, 2], list![3i32, 4], list![5i32]];
    let result = L::flatten(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 2, 3, 4, 5]);
    Ok(())
}

// ── FlattenReverse ──
#[test]
fn test_flatten_reverse() -> Result<()> {
    let lst = list![list![1i32, 2], list![3i32]];
    let result = L::flattenReverse(Arc::clone(&lst))?;
    assert_eq!(result, list![2i32, 1, 3]);
    Ok(())
}

// ── Fold ──
#[test]
fn test_fold() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::fold(Arc::clone(&lst), 0i32, |x, acc| Ok(acc + x))?;
    assert_eq!(result, 15);
    Ok(())
}

// ── Fold1 ──
#[test]
fn test_fold1() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::fold1(Arc::clone(&lst), |x, acc| Ok(acc + x))?;
    assert_eq!(result, 6);
    Ok(())
}

// ── Fold1r ──
#[test]
fn test_fold1r() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::fold1r(Arc::clone(&lst), |acc, x| Ok(acc + x))?;
    assert_eq!(result, 6);
    Ok(())
}

// ── Fold2 ──
#[test]
fn test_fold2() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::fold2(Arc::clone(&a), Arc::clone(&b), 0i32, |x, y, acc| Ok(acc + x + y))?;
    assert_eq!(result, 33);
    Ok(())
}

// ── Fold2r ──
#[test]
fn test_fold2r() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::fold2r(Arc::clone(&a), Arc::clone(&b), 0i32, |acc, x, y| Ok(acc + x + y))?;
    assert_eq!(result, 33);
    Ok(())
}

// ── Fold3 ──
#[test]
fn test_fold3() -> Result<()> {
    let a = list![1i32];
    let b = list![2i32];
    let c = list![3i32];
    let result = L::fold3(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), 0i32, |x, y, z, acc| Ok(acc + x + y + z))?;
    assert_eq!(result, 6);
    Ok(())
}

// ── Fold31 ──
#[test]
fn test_fold31() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let c = list![100i32, 200];
    let result = L::fold31(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), |x, y, z, acc| Ok(acc + x + y + z))?;
    assert_eq!(result, 333);
    Ok(())
}

// ── Fold4 ──
#[test]
fn test_fold4() -> Result<()> {
    let a = list![1i32];
    let b = list![2i32];
    let c = list![3i32];
    let d = list![4i32];
    let result = L::fold4(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), Arc::clone(&d), 0i32, |w, x, y, z, acc| Ok(acc + w + x + y + z))?;
    assert_eq!(result, 10);
    Ok(())
}

// ── FoldAllValue ──
#[test]
fn test_fold_all_value() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::foldAllValue(Arc::clone(&lst), 0i32, |x, acc| Ok(acc + x))?;
    assert_eq!(result, 6);
    Ok(())
}

// ── FoldList ──
#[test]
fn test_fold_list() -> Result<()> {
    let outer = list![list![1i32, 2], list![3i32, 4]];
    let result = L::foldList(Arc::clone(&outer), 0i32, |inner, acc| {
        L::fold(inner, acc, |x, a| Ok(a + x))
    })?;
    assert_eq!(result, 10);
    Ok(())
}

// ── Foldr ──
#[test]
fn test_foldr() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::foldr(Arc::clone(&lst), 0i32, |acc, x| Ok(acc + x))?;
    assert_eq!(result, 6);
    Ok(())
}

// ── Fold20, Fold21, Fold22 ──
#[test]
fn test_fold20() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::fold20(Arc::clone(&a), Arc::clone(&b), |x, y, acc| Ok(acc + x + y))?;
    assert_eq!(result, 33);
    Ok(())
}
#[test]
fn test_fold21() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::fold21(Arc::clone(&a), Arc::clone(&b), 0i32, |x, y, acc| Ok(acc + x + y))?;
    assert_eq!(result, 33);
    Ok(())
}
#[test]
fn test_fold22() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::fold22(Arc::clone(&a), Arc::clone(&b), |x, y, acc| Ok(acc + x + y))?;
    assert_eq!(result, 33);
    Ok(())
}

// ── FromOption ──
#[test]
fn test_from_option_some() -> Result<()> {
    let result = L::fromOption(Some(42i32))?;
    assert_eq!(result, list![42i32]);
    Ok(())
}
#[test]
fn test_from_option_none() -> Result<()> {
    let result = L::fromOption(Option::<i32>::None)?;
    assert!(result.is_empty());
    Ok(())
}

// ── GetAtIndexLst ──
#[test]
fn test_get_at_index_lst() -> Result<()> {
    let lst = list![10i32, 20, 30];
    assert_eq!(L::getAtIndexLst(Arc::clone(&lst), 1)?, 10);
    assert_eq!(L::getAtIndexLst(Arc::clone(&lst), 2)?, 20);
    assert_eq!(L::getAtIndexLst(Arc::clone(&lst), 3)?, 30);
    Ok(())
}

// ── GetIndexFirst ──
#[test]
fn test_get_index_first() -> Result<()> {
    let lst = list![10i32, 20, 30];
    assert_eq!(L::getIndexFirst(Arc::clone(&lst))?, 10);
    Ok(())
}

// ── GetMember ──
#[test]
fn test_get_member() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert_eq!(L::getMember(Arc::clone(&lst), 2)?, Some(2));
    assert_eq!(L::getMember(Arc::clone(&lst), 4)?, None);
    Ok(())
}

// ── GetMemberOnTrue ──
#[test]
fn test_get_member_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(L::getMemberOnTrue(2i32, Arc::clone(&lst), eq_i)?);
    assert!(!L::getMemberOnTrue(4i32, Arc::clone(&lst), eq_i)?);
    Ok(())
}

// ── HasOneElement ──
#[test]
fn test_has_one_element_true() -> Result<()> {
    assert!(L::hasOneElement(list![1i32])?);
    Ok(())
}
#[test]
fn test_has_one_element_false() -> Result<()> {
    assert!(!L::hasOneElement(list![1i32, 2])?);
    assert!(!L::hasOneElement(nil::<i32>())?);
    Ok(())
}

// ── HasSeveralElements ──
#[test]
fn test_has_several_elements_true() -> Result<()> {
    assert!(L::hasSeveralElements(list![1i32, 2])?);
    Ok(())
}
#[test]
fn test_has_several_elements_false() -> Result<()> {
    assert!(!L::hasSeveralElements(list![1i32])?);
    assert!(!L::hasSeveralElements(nil::<i32>())?);
    Ok(())
}

// ── HeapSortIntList ──
#[test]
fn test_heap_sort_int_list() -> Result<()> {
    let lst = list![3i32, 1, 4, 1, 5, 9, 2, 6];
    let result = L::heapSortIntList(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 1, 2, 3, 4, 5, 6, 9]);
    Ok(())
}

// ── Insert ──
#[test]
fn test_insert() -> Result<()> {
    let lst = list![1i32, 4, 5];
    let result = L::insert(Arc::clone(&lst), 3, 2)?;
    assert_eq!(result, list![1i32, 3, 4, 5]);
    Ok(())
}

// ── InsertListSorted ──
#[test]
fn test_insert_list_sorted() -> Result<()> {
    let lst = list![1i32, 3, 5];
    let result = L::insertListSorted(Arc::clone(&lst), 2)?;
    assert_eq!(result, list![1i32, 2, 3, 5]);
    Ok(())
}

// ── IntRange ──
#[test]
fn test_int_range() -> Result<()> {
    let result = L::intRange(1, 5)?;
    assert_eq!(result, list![1i32, 2, 3, 4, 5]);
    Ok(())
}
#[test]
fn test_int_range_descending() -> Result<()> {
    let result = L::intRange(5, 1)?;
    assert_eq!(result, list![5i32, 4, 3, 2, 1]);
    Ok(())
}

// ── IntRange2 ──
#[test]
fn test_int_range2() -> Result<()> {
    let result = L::intRange2(1, 5, 1)?;
    assert_eq!(result, list![1i32, 2, 3, 4, 5]);
    Ok(())
}
#[test]
fn test_int_range2_step() -> Result<()> {
    let result = L::intRange2(0, 10, 3)?;
    assert_eq!(result, list![0i32, 3, 6, 9]);
    Ok(())
}

// ── IntRange3 ──
#[test]
fn test_int_range3() -> Result<()> {
    let result = L::intRange3(1, 5)?;
    assert_eq!(result, list![1i32, 2, 3, 4]);
    Ok(())
}

// ── Intersection1OnTrue ──
#[test]
fn test_intersection1_on_true() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![2i32, 3, 4];
    let result = L::intersection1OnTrue(Arc::clone(&a), Arc::clone(&b), eq_i)?;
    assert_eq!(result, list![2i32, 3]);
    Ok(())
}

// ── IntersectionOnTrue ──
#[test]
fn test_intersection_on_true() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![2i32, 3, 4];
    let result = L::intersectionOnTrue(Arc::clone(&a), Arc::clone(&b), eq_i)?;
    assert_eq!(result, list![2i32, 3]);
    Ok(())
}

// ── IsEqual ──
#[test]
fn test_is_equal_true() -> Result<()> {
    assert!(L::isEqual(list![1i32, 2], list![1i32, 2])?);
    Ok(())
}
#[test]
fn test_is_equal_false() -> Result<()> {
    assert!(!L::isEqual(list![1i32, 2], list![1i32, 3])?);
    Ok(())
}

// ── IsEqualOnTrue ──
#[test]
fn test_is_equal_on_true() -> Result<()> {
    assert!(L::isEqualOnTrue(list![1i32, 2], list![1i32, 2], eq_i)?);
    assert!(!L::isEqualOnTrue(list![1i32, 2], list![1i32, 3], eq_i)?);
    Ok(())
}

// ── IsMemberOnTrue ──
#[test]
fn test_is_member_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(L::isMemberOnTrue(2i32, Arc::clone(&lst), eq_i)?);
    assert!(!L::isMemberOnTrue(4i32, Arc::clone(&lst), eq_i)?);
    Ok(())
}

// ── IsPrefixOnTrue ──
#[test]
fn test_is_prefix_on_true() -> Result<()> {
    let prefix = list![1i32, 2];
    let full = list![1i32, 2, 3, 4];
    assert!(L::isPrefixOnTrue(Arc::clone(&prefix), Arc::clone(&full), eq_i)?);
    assert!(!L::isPrefixOnTrue(list![1i32, 3], full, eq_i)?);
    Ok(())
}

// ── KeepPositions ──
#[test]
fn test_keep_positions() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let positions = list![1i32, 3, 5];
    let result = L::keepPositions(Arc::clone(&lst), Arc::clone(&positions))?;
    assert_eq!(result, list![1i32, 3, 5]);
    Ok(())
}

// ── KeepPositionsSorted ──
#[test]
fn test_keep_positions_sorted() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let positions = list![1i32, 3, 5];
    let result = L::keepPositionsSorted(Arc::clone(&lst), Arc::clone(&positions))?;
    assert_eq!(result, list![1i32, 3, 5]);
    Ok(())
}

// ── Last ──
#[test]
fn test_last() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert_eq!(L::last(Arc::clone(&lst))?, 3);
    Ok(())
}

// ── LastListOrEmpty ──
#[test]
fn test_last_list_or_empty() -> Result<()> {
    let lst = list![list![1i32], list![2i32, 3]];
    let result = L::lastListOrEmpty(Arc::clone(&lst))?;
    assert_eq!(result, list![2i32, 3]);
    Ok(())
}

// ── LastN ──
#[test]
fn test_last_n() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::lastN(Arc::clone(&lst), 3)?;
    assert_eq!(result, list![3i32, 4, 5]);
    Ok(())
}

// ── LengthListElements ──
#[test]
fn test_length_list_elements() -> Result<()> {
    let lst = list![list![1i32, 2], list![3i32, 4, 5], list![6i32]];
    assert_eq!(L::lengthListElements(Arc::clone(&lst))?, 6);
    Ok(())
}

// ── ListArrayReverse ──
#[test]
fn test_list_array_reverse() -> Result<()> {
    let a = arrayFromVec(vec![1i32, 2, 3]);
    let lst = list![Arc::new(a.clone())];
    let result = L::listArrayReverse(Arc::clone(&lst))?;
    assert_eq!(*result.borrow(), vec![1i32, 2, 3]);
    Ok(())
}

// ── ListIsLonger ──
#[test]
fn test_list_is_longer() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![1i32, 2];
    assert!(L::listIsLonger(Arc::clone(&a), Arc::clone(&b))?);
    assert!(!L::listIsLonger(Arc::clone(&b), Arc::clone(&a))?);
    Ok(())
}

// ── Map ──
#[test]
fn test_map() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::map(Arc::clone(&lst), double)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}
#[test]
fn test_map_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::map(Arc::clone(&lst), double)?;
    assert!(result.is_empty());
    Ok(())
}

// ── Map1 ──
#[test]
fn test_map1() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::map1(Arc::clone(&lst), double)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}

// ── Map1Fold ──
#[test]
fn test_map1_fold() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (result, acc) = L::map1Fold(Arc::clone(&lst), 0i32, |x, a| Ok((x + a, a + 1)))?;
    assert_eq!(result, list![1i32, 3, 6]);
    assert_eq!(acc, 3);
    Ok(())
}

// ── Map1List ──
#[test]
fn test_map1_list() -> Result<()> {
    let lst = list![list![1i32, 2]];
    let result = L::map1List(Arc::clone(&lst), |inner| L::map(inner, double))?;
    assert_eq!(result, list![list![2i32, 4]]);
    Ok(())
}

// ── Map1Option ──
#[test]
fn test_map1_option() -> Result<()> {
    let lst = list![Some(1i32)];
    let result = L::map1Option(Arc::clone(&lst), |x| Ok(x * 2))?;
    assert_eq!(result, list![Some(2i32)]);
    Ok(())
}

// ── Map1_0 ──
#[test]
fn test_map1_0() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::map1_0(Arc::clone(&lst), double)?;
    assert!(result.is_empty());
    Ok(())
}

// ── Map1_2 ──
#[test]
fn test_map1_2() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::map1_2(Arc::clone(&lst), double)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── Map1r ──
#[test]
fn test_map1r() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::map1r(Arc::clone(&lst), double)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}

// ── Map2 ──
#[test]
fn test_map2() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::map2(Arc::clone(&a), Arc::clone(&b), |x, y| Ok(x + y))?;
    assert_eq!(result, list![11i32, 22]);
    Ok(())
}

// ── Map2Fold ──
#[test]
fn test_map2_fold() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let (result, acc) = L::map2Fold(Arc::clone(&a), Arc::clone(&b), 0i32, |x, y, a| Ok((x + y, a + 1)))?;
    assert_eq!(result, list![11i32, 22]);
    assert_eq!(acc, 2);
    Ok(())
}

// ── Map2FoldCheckReferenceEq ──
#[test]
fn test_map2_fold_check_reference_eq() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let (result, acc) = L::map2FoldCheckReferenceEq(Arc::clone(&a), Arc::clone(&b), nil::<i32>(), |x, y, a| Ok((cons(x + y, a.clone()), a.clone())))?;
    assert_eq!(result, list![11i32, 22]);
    Ok(())
}

// ── Map2List ──
#[test]
fn test_map2_list() -> Result<()> {
    let a = list![list![1i32, 2]];
    let b = list![list![10i32, 20]];
    let result = L::map2List(Arc::clone(&a), Arc::clone(&b), |x, y| L::map2(x, y, |a, b| Ok(a + b)))?;
    assert_eq!(result, list![list![11i32, 22]]);
    Ok(())
}

// ── Map2Option ──
#[test]
fn test_map2_option() -> Result<()> {
    let a = list![Some(1i32)];
    let b = list![Some(10i32)];
    let result = L::map2Option(Arc::clone(&a), Arc::clone(&b), |x, y| Ok(x + y))?;
    assert_eq!(result, Some(11));
    Ok(())
}

// ── Map2Reverse ──
#[test]
fn test_map2_reverse() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::map2Reverse(Arc::clone(&a), Arc::clone(&b), |x, y| Ok(x + y))?;
    assert_eq!(result, list![22i32, 11]);
    Ok(())
}

// ── Map2_0 ──
#[test]
fn test_map2_0() -> Result<()> {
    let a: Arc<List<i32>> = nil();
    let b: Arc<List<i32>> = nil();
    let result = L::map2_0(Arc::clone(&a), Arc::clone(&b), |x, y| Ok(x + y))?;
    assert!(result.is_empty());
    Ok(())
}

// ── Map2_2 ──
#[test]
fn test_map2_2() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::map2_2(Arc::clone(&a), Arc::clone(&b), |x, y| Ok(x + y))?;
    assert_eq!(result, list![11i32, 22]);
    Ok(())
}

// ── Map3 ──
#[test]
fn test_map3() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let c = list![100i32, 200];
    let result = L::map3(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), |x, y, z| Ok(x + y + z))?;
    assert_eq!(result, list![111i32, 222]);
    Ok(())
}

// ── Map3Fold ──
#[test]
fn test_map3_fold() -> Result<()> {
    let a = list![1i32];
    let b = list![10i32];
    let c = list![100i32];
    let (result, acc) = L::map3Fold(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), 0i32, |x, y, z, acc| Ok((x + y + z, acc + 1)))?;
    assert_eq!(result, list![111i32]);
    assert_eq!(acc, 1);
    Ok(())
}

// ── Map4 ──
#[test]
fn test_map4() -> Result<()> {
    let a = list![1i32];
    let b = list![2i32];
    let c = list![3i32];
    let d = list![4i32];
    let result = L::map4(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), Arc::clone(&d), |w, x, y, z| Ok(w + x + y + z))?;
    assert_eq!(result, list![10i32]);
    Ok(())
}

// ── Map4_0 ──
#[test]
fn test_map4_0() -> Result<()> {
    let a: Arc<List<i32>> = nil();
    let b: Arc<List<i32>> = nil();
    let c: Arc<List<i32>> = nil();
    let d: Arc<List<i32>> = nil();
    let result = L::map4_0(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), Arc::clone(&d), |w, x, y, z| Ok(w + x + y + z))?;
    assert!(result.is_empty());
    Ok(())
}

// ── Map5 ──
#[test]
fn test_map5() -> Result<()> {
    let a = list![1i32];
    let b = list![2i32];
    let c = list![3i32];
    let d = list![4i32];
    let e = list![5i32];
    let result = L::map5(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), Arc::clone(&d), Arc::clone(&e), |w, x, y, z, v| Ok(w + x + y + z + v))?;
    assert_eq!(result, list![15i32]);
    Ok(())
}

// ── Map6 ──
#[test]
fn test_map6() -> Result<()> {
    let a = list![1i32];
    let b = list![2i32];
    let c = list![3i32];
    let d = list![4i32];
    let e = list![5i32];
    let f = list![6i32];
    let result = L::map6(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c), Arc::clone(&d), Arc::clone(&e), Arc::clone(&f), |w, x, y, z, v, u| Ok(w + x + y + z + v + u))?;
    assert_eq!(result, list![21i32]);
    Ok(())
}

// ── MapArray ──
#[test]
fn test_map_array() -> Result<()> {
    let a = arrayFromVec(vec![1i32, 2, 3]);
    let result = L::mapArray(a, double)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}

// ── MapCheckReferenceEq ──
#[test]
fn test_map_check_reference_eq() -> Result<()> {
    let a: Arc<List<i32>> = list![1i32];
    let b = Arc::clone(&a);
    let lst = list![Arc::clone(&a), b];
    let result = L::mapCheckReferenceEq(Arc::clone(&lst), |x| Ok(*x == a))?;
    assert_eq!(result, list![true, true]);
    Ok(())
}

// ── MapFlat ──
#[test]
fn test_map_flat() -> Result<()> {
    let lst = list![list![1i32, 2], list![3i32, 4]];
    let result = L::mapFlat(Arc::clone(&lst), |inner| Ok(inner))?;
    assert_eq!(result, list![1i32, 2, 3, 4]);
    Ok(())
}

// ── MapFlatReverse ──
#[test]
fn test_map_flat_reverse() -> Result<()> {
    let lst = list![list![1i32, 2], list![3i32]];
    let result = L::mapFlatReverse(Arc::clone(&lst), |inner| Ok(inner))?;
    assert_eq!(result, list![2i32, 1, 3]);
    Ok(())
}

// ── MapFold ──
#[test]
fn test_map_fold() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (result, acc) = L::mapFold(Arc::clone(&lst), 0i32, |x, a| Ok((x * 2, a + x)))?;
    assert_eq!(result, list![2i32, 4, 6]);
    assert_eq!(acc, 6);
    Ok(())
}

// ── MapFold2 ──
#[test]
fn test_map_fold2() -> Result<()> {
    let lst = list![1i32, 2];
    let (result, acc1, acc2) = L::mapFold2(Arc::clone(&lst), (0i32, 0i32), |x, (a, b)| Ok((x + a, a + x, b + x)))?;
    assert_eq!(result, list![1i32, 3]);
    Ok(())
}

// ── MapFold3 ──
#[test]
fn test_map_fold3() -> Result<()> {
    let lst = list![1i32];
    let (result, a, b, c) = L::mapFold3(Arc::clone(&lst), (0i32, 0i32, 0i32), |x, a, b, c| Ok((x * 2, a + x, b + x, c + x)))?;
    assert_eq!(result, list![2i32]);
    assert_eq!(a, 1);
    assert_eq!(b, 1);
    assert_eq!(c, 1);
    Ok(())
}

// ── MapFold5 ──
#[test]
fn test_map_fold5() -> Result<()> {
    let lst = list![1i32];
    let (result, a, b, c, d, e) = L::mapFold5(Arc::clone(&lst), (0i32, 0i32, 0i32, 0i32, 0i32), |x, a, b, c, d, e| Ok((x, a+1, b+1, c+1, d+1, e+1)))?;
    assert_eq!(result, list![1i32]);
    assert_eq!(a, 1);
    Ok(())
}

// ── MapFoldList ──
#[test]
fn test_map_fold_list() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (result, acc) = L::mapFoldList(Arc::clone(&lst), nil::<i32>(), |x, a| Ok((x * 2, cons(x, a.clone()))))?;
    assert_eq!(result, list![2i32, 4, 6]);
    assert_eq!(acc, list![3i32, 2, 1]);
    Ok(())
}

// ── MapIndices ──
#[test]
fn test_map_indices() -> Result<()> {
    let lst = list![10i32, 20, 30];
    let result = L::mapIndices(Arc::clone(&lst), |x, i| Ok(x + i))?;
    assert_eq!(result, list![11i32, 22, 33]);
    Ok(())
}

// ── MapList ──
#[test]
fn test_map_list() -> Result<()> {
    let lst = list![list![1i32, 2], list![3i32, 4]];
    let result = L::mapList(Arc::clone(&lst), |inner| L::map(inner, double))?;
    assert_eq!(result, list![list![2i32, 4], list![6i32, 8]]);
    Ok(())
}

// ── MapListReverse ──
#[test]
fn test_map_list_reverse() -> Result<()> {
    let lst = list![list![1i32, 2], list![3i32]];
    let result = L::mapListReverse(Arc::clone(&lst), |inner| L::map(inner, double))?;
    assert_eq!(result, list![list![6i32], list![4, 2]]);
    Ok(())
}

// ── MapMap ──
#[test]
fn test_map_map() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let result = L::mapMap(Arc::clone(&a), Arc::clone(&b), |x, y| Ok(x + y))?;
    assert_eq!(result, list![list![11i32, 12], list![21i32, 22]]);
    Ok(())
}

// ── MapMapBoolAnd ──
#[test]
fn test_map_map_bool_and() -> Result<()> {
    let a = list![true, false];
    let b = list![true, true];
    let result = L::mapMapBoolAnd(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result, list![list![true, false], list![false, false]]);
    Ok(())
}

// ── MapOption ──
#[test]
fn test_map_option() -> Result<()> {
    let lst = list![Some(1i32), None, Some(3)];
    let result = L::mapOption(Arc::clone(&lst), |x| Ok(x * 2))?;
    assert_eq!(result, list![Some(2i32), None, Some(6)]);
    Ok(())
}

// ── MapReverse ──
#[test]
fn test_map_reverse() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::mapReverse(Arc::clone(&lst), double)?;
    assert_eq!(result, list![6i32, 4, 2]);
    Ok(())
}

// ── Map_0 ──
#[test]
fn test_map_0() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::map_0(Arc::clone(&lst), double)?;
    assert!(result.is_empty());
    Ok(())
}

// ── Map_2 ──
#[test]
fn test_map_2() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::map_2(Arc::clone(&lst), double)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── Map_3 ──
#[test]
fn test_map_3() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::map_3(Arc::clone(&lst), double)?;
    assert_eq!(result, list![2i32, 4, 6]);
    Ok(())
}

// ── MaxElement ──
#[test]
fn test_max_element() -> Result<()> {
    let lst = list![3i32, 1, 4, 1, 5, 9, 2, 6];
    assert_eq!(L::maxElement(Arc::clone(&lst))?, 9);
    Ok(())
}

// ── MergeSorted ──
#[test]
fn test_merge_sorted() -> Result<()> {
    let a = list![1i32, 3, 5];
    let b = list![2i32, 4, 6];
    let result = L::mergeSorted(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result, list![1i32, 2, 3, 4, 5, 6]);
    Ok(())
}

// ── MinElement ──
#[test]
fn test_min_element() -> Result<()> {
    let lst = list![3i32, 1, 4, 1, 5];
    assert_eq!(L::minElement(Arc::clone(&lst))?, 1);
    Ok(())
}

// ── MkOption ──
#[test]
fn test_mk_option() -> Result<()> {
    let result = L::mkOption(42i32)?;
    assert_eq!(result, Some(42));
    Ok(())
}

// ── None ──
#[test]
fn test_none() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    assert!(L::none(Arc::clone(&lst), is_positive)?);
    Ok(())
}
#[test]
fn test_none_false() -> Result<()> {
    let lst = list![1i32, -2];
    assert!(!L::none(Arc::clone(&lst), is_positive)?);
    Ok(())
}

// ── NotMember ──
#[test]
fn test_not_member() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(L::notMember(Arc::clone(&lst), 4)?);
    assert!(!L::notMember(Arc::clone(&lst), 2)?);
    Ok(())
}

// ── Partition ──
#[test]
fn test_partition() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5, 6];
    let (a, b) = L::partition(Arc::clone(&lst), is_even)?;
    assert_eq!(a, list![2i32, 4, 6]);
    assert_eq!(b, list![1i32, 3, 5]);
    Ok(())
}

// ── Position ──
#[test]
fn test_position() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    assert_eq!(L::position(Arc::clone(&lst), 3)?, 3);
    assert_eq!(L::position(Arc::clone(&lst), 9)?, 0);
    Ok(())
}

// ── Position1OnTrue ──
#[test]
fn test_position1_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert_eq!(L::position1OnTrue(Arc::clone(&lst), is_even)?, 2);
    Ok(())
}

// ── PositionOnTrue ──
#[test]
fn test_position_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert_eq!(L::positionOnTrue(Arc::clone(&lst), is_even)?, 2);
    Ok(())
}

// ── Reduce ──
#[test]
fn test_reduce() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::reduce(Arc::clone(&lst), add_i)?;
    assert_eq!(result, 10);
    Ok(())
}

// ── RemoveOnTrue ──
#[test]
fn test_remove_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::removeOnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![1i32, 3]);
    Ok(())
}

// ── Repeat ──
#[test]
fn test_repeat() -> Result<()> {
    let result = L::repeat(42i32, 4)?;
    assert_eq!(result, list![42i32, 42, 42, 42]);
    Ok(())
}

// ── ReplaceAt ──
#[test]
fn test_replace_at() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::replaceAt(Arc::clone(&lst), 2, 99)?;
    assert_eq!(result, list![1i32, 99, 3]);
    Ok(())
}

// ── ReplaceAtIndexFirst ──
#[test]
fn test_replace_at_index_first() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::replaceAtIndexFirst(Arc::clone(&lst), 99)?;
    assert_eq!(result, list![99i32, 2, 3]);
    Ok(())
}

// ── ReplaceAtWithList ──
#[test]
fn test_replace_at_with_list() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::replaceAtWithList(Arc::clone(&lst), 4, 10)?;
    assert_eq!(result, list![10i32, 1, 2, 3, 4]);
    Ok(())
}

// ── ReplaceOnTrue ──
#[test]
fn test_replace_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::replaceOnTrue(Arc::clone(&lst), 2i32, 99, eq_i)?;
    assert_eq!(result, list![1i32, 99, 3]);
    Ok(())
}

// ── RestOrEmpty ──
#[test]
fn test_rest_or_empty() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::restOrEmpty(Arc::clone(&lst));
    assert_eq!(result, list![2i32, 3]);
    Ok(())
}
#[test]
fn test_rest_or_empty_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::restOrEmpty(Arc::clone(&lst));
    assert!(result.is_empty());
    Ok(())
}

// ── Second ──
#[test]
fn test_second() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert_eq!(L::second(Arc::clone(&lst))?, 2);
    Ok(())
}

// ── Separate1OnTrue ──
#[test]
fn test_separate1_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (a, b) = L::separate1OnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(a, list![2i32]);
    assert_eq!(b, list![1i32, 3]);
    Ok(())
}

// ── SeparateOnTrue ──
#[test]
fn test_separate_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let (a, b) = L::separateOnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(a, list![2i32, 4]);
    assert_eq!(b, list![1i32, 3]);
    Ok(())
}

// ── Set ──
#[test]
fn test_set() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::set(Arc::clone(&lst), 2, 99)?;
    assert_eq!(result, list![1i32, 99, 3]);
    Ok(())
}

// ── SetDifference ──
#[test]
fn test_set_difference() -> Result<()> {
    let a = list![1i32, 2, 3, 4];
    let b = list![3i32, 4, 5];
    let result = L::setDifference(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result, list![1i32, 2]);
    Ok(())
}

// ── SetDifferenceIntN ──
#[test]
fn test_set_difference_int_n() -> Result<()> {
    let a = list![1i32, 2, 3, 4];
    let b = list![3i32, 4, 5];
    let result = L::setDifferenceIntN(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result, list![1i32, 2]);
    Ok(())
}

// ── SetDifferenceOnTrue ──
#[test]
fn test_set_difference_on_true() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![2i32, 3];
    let result = L::setDifferenceOnTrue(Arc::clone(&a), Arc::clone(&b), eq_i)?;
    assert_eq!(result, list![1i32]);
    Ok(())
}

// ── SetEqualOnTrue ──
#[test]
fn test_set_equal_on_true() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![3i32, 1, 2];
    assert!(L::setEqualOnTrue(Arc::clone(&a), Arc::clone(&b), eq_i)?);
    Ok(())
}

// ── Sort ──
#[test]
fn test_sort() -> Result<()> {
    let lst = list![3i32, 1, 4, 1, 5, 9, 2, 6];
    let result = L::sort(Arc::clone(&lst), less_i)?;
    assert_eq!(result, list![1i32, 1, 2, 3, 4, 5, 6, 9]);
    Ok(())
}

// ── SortedDuplicates ──
#[test]
fn test_sorted_duplicates() -> Result<()> {
    let lst = list![1i32, 1, 2, 3, 3, 4];
    let result = L::sortedDuplicates(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 3]);
    Ok(())
}

// ── SortedListAllUnique ──
#[test]
fn test_sorted_list_all_unique() -> Result<()> {
    let lst = list![1i32, 2, 3];
    assert!(L::sortedListAllUnique(Arc::clone(&lst))?);
    Ok(())
}

// ── SortedUnique ──
#[test]
fn test_sorted_unique() -> Result<()> {
    let lst = list![1i32, 1, 2, 3, 3, 4];
    let result = L::sortedUnique(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 2, 3, 4]);
    Ok(())
}

// ── SortedUniqueAndDuplicates ──
#[test]
fn test_sorted_unique_and_duplicates() -> Result<()> {
    let lst = list![1i32, 1, 2, 3, 3, 4];
    let (unique, dups) = L::sortedUniqueAndDuplicates(Arc::clone(&lst))?;
    assert_eq!(unique, list![1i32, 2, 3, 4]);
    assert_eq!(dups, list![1i32, 3]);
    Ok(())
}

// ── SortedUniqueOnlyDuplicates ──
#[test]
fn test_sorted_unique_only_duplicates() -> Result<()> {
    let lst = list![1i32, 1, 2, 3, 3, 4];
    let result = L::sortedUniqueOnlyDuplicates(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 3]);
    Ok(())
}

// ── Split ──
#[test]
fn test_split() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let (a, b) = L::split(Arc::clone(&lst), 3)?;
    assert_eq!(a, list![1i32, 2, 3]);
    assert_eq!(b, list![4i32, 5]);
    Ok(())
}

// ── Split1OnTrue ──
#[test]
fn test_split1_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (a, b) = L::split1OnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(a, list![1i32]);
    assert_eq!(b, list![2i32, 3]);
    Ok(())
}

// ── Split2OnTrue ──
#[test]
fn test_split2_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let (a, b) = L::split2OnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(a, list![1i32]);
    assert_eq!(b, list![2i32, 3, 4]);
    Ok(())
}

// ── SplitEqualParts ──
#[test]
fn test_split_equal_parts() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::splitEqualParts(Arc::clone(&lst), 2)?;
    assert_eq!(result.len(), 2);
    Ok(())
}

// ── SplitEqualPrefix ──
#[test]
fn test_split_equal_prefix() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![1i32, 2, 4];
    let result = L::splitEqualPrefix(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result.0, list![1i32, 2]);
    Ok(())
}

// ── SplitLast ──
#[test]
fn test_split_last() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let (init, last) = L::splitLast(Arc::clone(&lst))?;
    assert_eq!(init, list![1i32, 2]);
    assert_eq!(last, list![3i32]);
    Ok(())
}

// ── SplitOnBoolList ──
#[test]
fn test_split_on_bool_list() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let blist = list![false, true, false, true, false];
    let result = L::splitOnBoolList(Arc::clone(&lst), Arc::clone(&blist))?;
    assert_eq!(result.len(), 3);
    Ok(())
}

// ── SplitOnFirstMatch ──
#[test]
fn test_split_on_first_match() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let (before, after) = L::splitOnFirstMatch(Arc::clone(&lst), is_even)?;
    assert_eq!(before, list![1i32]);
    assert_eq!(after, list![2i32, 3, 4]);
    Ok(())
}

// ── SplitOnTrue ──
#[test]
fn test_split_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::splitOnTrue(Arc::clone(&lst), is_even)?;
    assert_eq!(result.len(), 3);
    Ok(())
}

// ── Splitr ──
#[test]
fn test_splitr() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let (a, b) = L::splitr(Arc::clone(&lst), 2)?;
    assert_eq!(a, list![1i32, 2, 3]);
    assert_eq!(b, list![4i32, 5]);
    Ok(())
}

// ── StripLast ──
#[test]
fn test_strip_last() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::stripLast(Arc::clone(&lst));
    assert_eq!(result, list![1i32, 2]);
    Ok(())
}

// ── StripN ──
#[test]
fn test_strip_n() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::stripN(Arc::clone(&lst), 2)?;
    assert_eq!(result, list![3i32, 4, 5]);
    Ok(())
}

// ── Sublist ──
#[test]
fn test_sublist() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::sublist(Arc::clone(&lst), 2, 4)?;
    assert_eq!(result, list![2i32, 3, 4]);
    Ok(())
}

// ── Thread ──
#[test]
fn test_thread() -> Result<()> {
    let a = list![1i32, 2, 3];
    let result = L::thread(Arc::clone(&a), 0i32, |acc, x| Ok(acc + x))?;
    assert_eq!(result, list![1i32, 3, 6]);
    Ok(())
}

// ── Thread3Map ──
#[test]
fn test_thread3_map() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::thread3Map(Arc::clone(&lst), (0i32, 0i32, 0i32), |x, (a, b, c)| Ok((a + x, b + x, c + x, x * 10)))?;
    assert_eq!(result.1, list![10i32, 20]);
    Ok(())
}

// ── Thread3MapFold ──
#[test]
fn test_thread3_map_fold() -> Result<()> {
    let lst = list![1i32];
    let ((a, b, c), results) = L::thread3MapFold(Arc::clone(&lst), (0i32, 0i32, 0i32), 0i32, |x, a, b, c, acc| Ok((a + x, b + x, c + x, x * 10, acc + x)))?;
    assert_eq!(a, 1);
    assert_eq!(results, list![10i32]);
    Ok(())
}

// ── ThreadFold ──
#[test]
fn test_thread_fold() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::threadFold(Arc::clone(&lst), 0i32, |acc, x| Ok((acc + x, x * 10)))?;
    assert_eq!(result.1, list![10i32, 20, 30]);
    Ok(())
}

// ── ThreadFold1 ──
#[test]
fn test_thread_fold1() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::threadFold1(Arc::clone(&lst), |acc, x| Ok((acc + x, x * 10)))?;
    assert_eq!(result.1, list![10i32, 20]);
    Ok(())
}

// ── ThreadFold2 ──
#[test]
fn test_thread_fold2() -> Result<()> {
    let lst = list![1i32];
    let result = L::threadFold2(Arc::clone(&lst), (0i32, 0i32), |(a, b), x| Ok((a + x, b + x * 2, x)))?;
    assert_eq!(result.1, list![1i32]);
    Ok(())
}

// ── ThreadFold3 ──
#[test]
fn test_thread_fold3() -> Result<()> {
    let lst = list![1i32];
    let result = L::threadFold3(Arc::clone(&lst), (0i32, 0i32, 0i32), |(a, b, c), x| Ok((a + x, b + x, c + x, x)))?;
    assert_eq!(result.1, list![1i32]);
    Ok(())
}

// ── ThreadMap ──
#[test]
fn test_thread_map() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::threadMap(Arc::clone(&lst), 0i32, |x, acc| Ok((x + acc, acc + x)))?;
    assert_eq!(result.1, list![1i32, 4, 9]);
    Ok(())
}

// ── ThreadMap1 ──
#[test]
fn test_thread_map1() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::threadMap1(Arc::clone(&lst), 0i32, |x, acc| Ok((x + acc, acc + x)))?;
    assert_eq!(result.1, list![1i32, 4]);
    Ok(())
}

// ── ThreadMap1_0 ──
#[test]
fn test_thread_map1_0() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::threadMap1_0(Arc::clone(&lst), 0i32, |x, acc| Ok((x + acc, acc + x)))?;
    assert!(result.1.is_empty());
    Ok(())
}

// ── ThreadMap2 ──
#[test]
fn test_thread_map2() -> Result<()> {
    let lst = list![1i32];
    let result = L::threadMap2(Arc::clone(&lst), (0i32, 0i32), |x, a, b| Ok((a + x, b + x, x)))?;
    assert_eq!(result.2, list![1i32]);
    Ok(())
}

// ── ThreadMapAllValue ──
#[test]
fn test_thread_map_all_value() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::threadMapAllValue(Arc::clone(&lst), 0i32, |x, acc| Ok((x + acc, acc + x)))?;
    assert_eq!(result.1, list![1i32, 4, 9]);
    Ok(())
}

// ── ThreadMapFold ──
#[test]
fn test_thread_map_fold() -> Result<()> {
    let lst = list![1i32, 2];
    let (final_acc, results) = L::threadMapFold(Arc::clone(&lst), 0i32, |x, acc| Ok((acc + x, x * 10)))?;
    assert_eq!(final_acc, 3);
    assert_eq!(results, list![10i32, 20]);
    Ok(())
}

// ── ThreadMapList ──
#[test]
fn test_thread_map_list() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::threadMapList(Arc::clone(&lst), nil::<i32>(), |x, acc| Ok((cons(x * 2, acc.clone()), cons(x, acc.clone()))))?;
    assert_eq!(result.1, list![4i32, 2]);
    Ok(())
}

// ── ThreadMapList_2 ──
#[test]
fn test_thread_map_list_2() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::threadMapList_2(Arc::clone(&lst), nil::<i32>(), |x, acc| Ok((cons(x * 2, acc.clone()), cons(x, acc.clone()))))?;
    assert_eq!(result.1, list![4i32, 2]);
    Ok(())
}

// ── ThreadMap_2 ──
#[test]
fn test_thread_map_2() -> Result<()> {
    let lst = list![1i32, 2];
    let result = L::threadMap_2(Arc::clone(&lst), 0i32, |x, acc| Ok((x + acc, acc + x)))?;
    assert_eq!(result.1, list![1i32, 4]);
    Ok(())
}

// ── ToListWithPositions ──
#[test]
fn test_to_list_with_positions() -> Result<()> {
    let lst = list![10i32, 20, 30];
    let result = L::toListWithPositions(Arc::clone(&lst))?;
    assert_eq!(result, list![(1i32, 10i32), (2i32, 20), (3i32, 30)]);
    Ok(())
}

// ── ToString ──
#[test]
fn test_to_string() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::toString(Arc::clone(&lst), to_string_i32)?;
    assert_eq!(&*result, "{1, 2, 3}");
    Ok(())
}
#[test]
fn test_to_string_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::toString(Arc::clone(&lst), to_string_i32)?;
    assert_eq!(&*result, "{}");
    Ok(())
}

// ── TransposeList ──
#[test]
fn test_transpose_list() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![3i32, 4];
    let lst = list![Arc::clone(&a), Arc::clone(&b)];
    let result = L::transposeList(Arc::clone(&lst))?;
    assert_eq!(result.get(1)?, list![1i32, 3]);
    assert_eq!(result.get(2)?, list![2i32, 4]);
    Ok(())
}

// ── Trim ──
#[test]
fn test_trim() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::trim(Arc::clone(&lst), 2, 1)?;
    assert_eq!(result, list![3i32, 4]);
    Ok(())
}

// ── TrimToLength ──
#[test]
fn test_trim_to_length() -> Result<()> {
    let lst = list![1i32, 2, 3, 4, 5];
    let result = L::trimToLength(Arc::clone(&lst), 3)?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── Union ──
#[test]
fn test_union() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![3i32, 4, 5];
    let result = L::union(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result, list![1i32, 2, 3, 4, 5]);
    Ok(())
}

// ── UnionAppendListOnTrue ──
#[test]
fn test_union_append_list_on_true() -> Result<()> {
    let a = list![list![1i32, 2]];
    let b = list![list![2i32, 3]];
    let result = L::unionAppendListOnTrue(Arc::clone(&a), Arc::clone(&b), |x, y| L::isEqual(x, y))?;
    assert_eq!(result.len(), 2);
    Ok(())
}

// ── UnionElt ──
#[test]
fn test_union_elt() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::unionElt(Arc::clone(&lst), 4)?;
    assert_eq!(result, list![1i32, 2, 3, 4]);
    Ok(())
}
#[test]
fn test_union_elt_exists() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::unionElt(Arc::clone(&lst), 2)?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── UnionEltOnTrue ──
#[test]
fn test_union_elt_on_true() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::unionEltOnTrue(Arc::clone(&lst), 4, eq_i)?;
    assert_eq!(result, list![1i32, 2, 3, 4]);
    Ok(())
}

// ── UnionIntN ──
#[test]
fn test_union_int_n() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![3i32, 4];
    let result = L::unionIntN(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result, list![1i32, 2, 3, 4]);
    Ok(())
}

// ── UnionList ──
#[test]
fn test_union_list() -> Result<()> {
    let a = list![list![1i32, 2], list![3i32, 4]];
    let b = list![list![3i32, 4], list![5i32]];
    let result = L::unionList(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result.len(), 3);
    Ok(())
}

// ── UnionOnTrue ──
#[test]
fn test_union_on_true() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![2i32, 3];
    let result = L::unionOnTrue(Arc::clone(&a), Arc::clone(&b), eq_i)?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── UnionOnTrueList ──
#[test]
fn test_union_on_true_list() -> Result<()> {
    let a = list![list![1i32, 2]];
    let b = list![list![2i32, 3]];
    let result = L::unionOnTrueList(Arc::clone(&a), Arc::clone(&b), |x, y| L::isEqual(x, y))?;
    assert_eq!(result.len(), 2);
    Ok(())
}

// ── Unique ──
#[test]
fn test_unique() -> Result<()> {
    let lst = list![1i32, 2, 1, 3, 2];
    let result = L::unique(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── UniqueIntN ──
#[test]
fn test_unique_int_n() -> Result<()> {
    let lst = list![1i32, 2, 1, 3, 2];
    let result = L::uniqueIntN(Arc::clone(&lst))?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── UniqueOnTrue ──
#[test]
fn test_unique_on_true() -> Result<()> {
    let lst = list![1i32, 2, 1, 3];
    let result = L::uniqueOnTrue(Arc::clone(&lst), eq_i)?;
    assert_eq!(result, list![1i32, 2, 3]);
    Ok(())
}

// ── Unzip ──
#[test]
fn test_unzip() -> Result<()> {
    let lst = list![(1i32, 10i32), (2i32, 20), (3i32, 30)];
    let (a, b) = L::unzip(Arc::clone(&lst))?;
    assert_eq!(a, list![1i32, 2, 3]);
    assert_eq!(b, list![10i32, 20, 30]);
    Ok(())
}

// ── Unzip3 ──
#[test]
fn test_unzip3() -> Result<()> {
    let lst = list![(1i32, 10i32, 100i32)];
    let (a, b, c) = L::unzip3(Arc::clone(&lst))?;
    assert_eq!(a, list![1i32]);
    assert_eq!(b, list![10i32]);
    assert_eq!(c, list![100i32]);
    Ok(())
}

// ── UnzipSecond ──
#[test]
fn test_unzip_second() -> Result<()> {
    let lst = list![(1i32, 10i32), (2i32, 20)];
    let result = L::unzipSecond(Arc::clone(&lst))?;
    assert_eq!(result, list![10i32, 20]);
    Ok(())
}

// ── Zip ──
#[test]
fn test_zip() -> Result<()> {
    let a = list![1i32, 2, 3];
    let b = list![10i32, 20, 30];
    let result = L::zip(Arc::clone(&a), Arc::clone(&b))?;
    assert_eq!(result, list![(1i32, 10i32), (2i32, 20), (3i32, 30)]);
    Ok(())
}

// ── Zip3 ──
#[test]
fn test_zip3() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![10i32, 20];
    let c = list![100i32, 200];
    let result = L::zip3(Arc::clone(&a), Arc::clone(&b), Arc::clone(&c))?;
    assert_eq!(result, list![(1i32, 10i32, 100i32), (2i32, 20i32, 200)]);
    Ok(())
}

// ── Select (alias for filterOnTrue) ──
#[test]
fn test_select() -> Result<()> {
    let lst = list![1i32, 2, 3, 4];
    let result = L::select(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── Select1 (alias for filter1OnTrue) ──
#[test]
fn test_select1() -> Result<()> {
    let lst = list![2i32];
    let result = L::select1(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32]);
    Ok(())
}

// ── Select1r (alias for filter1rOnTrue) ──
#[test]
fn test_select1r() -> Result<()> {
    let lst = list![2i32, 4];
    let result = L::select1r(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── Select2 (alias for filter2OnTrue) ──
#[test]
fn test_select2() -> Result<()> {
    let lst = list![2i32, 4];
    let result = L::select2(Arc::clone(&lst), is_even)?;
    assert_eq!(result, list![2i32, 4]);
    Ok(())
}

// ── AllCombinations ──
#[test]
fn test_all_combinations() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::allCombinations(Arc::clone(&lst))?;
    // 2^n = 8 combinations
    assert_eq!(result.len(), 8);
    Ok(())
}

// ── FromList (not in DoubleEnded, in List) ──

// ── MapFold5 test ──

// ── InsertListSorted1 (private helper, skip) ──

// ── Additional edge cases ──

#[test]
fn test_map_empty_list() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::map(Arc::clone(&lst), double)?;
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_fold_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::fold(Arc::clone(&lst), 99i32, |_, acc| Ok(acc))?;
    assert_eq!(result, 99);
    Ok(())
}

#[test]
fn test_filter_empty_result() -> Result<()> {
    let lst = list![1i32, 3, 5];
    let result = L::filter(Arc::clone(&lst), is_even)?;
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_union_elt_duplicate() -> Result<()> {
    let lst = list![1i32, 2, 3];
    let result = L::unionElt(Arc::clone(&lst), 2)?;
    assert_eq!(result, list![1i32, 2, 3]); // no change, already present
    Ok(())
}

#[test]
fn test_intersection_on_true_no_overlap() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![3i32, 4];
    let result = L::intersectionOnTrue(Arc::clone(&a), Arc::clone(&b), eq_i)?;
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_set_difference_all_in_b() -> Result<()> {
    let a = list![1i32, 2];
    let b = list![1i32, 2, 3];
    let result = L::setDifference(Arc::clone(&a), Arc::clone(&b))?;
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_sort_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    let result = L::sort(Arc::clone(&lst), less_i)?;
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_sort_single() -> Result<()> {
    let lst = list![42i32];
    let result = L::sort(Arc::clone(&lst), less_i)?;
    assert_eq!(result, list![42i32]);
    Ok(())
}

#[test]
fn test_count_zero() -> Result<()> {
    let lst = list![1i32, 3, 5];
    assert_eq!(L::count(Arc::clone(&lst), is_even)?, 0);
    Ok(())
}

#[test]
fn test_position_empty() -> Result<()> {
    let lst: Arc<List<i32>> = nil();
    assert_eq!(L::position(Arc::clone(&lst), 1)?, 0);
    Ok(())
}

#[test]
fn test_max_min_single() -> Result<()> {
    let lst = list![42i32];
    assert_eq!(L::maxElement(Arc::clone(&lst))?, 42);
    assert_eq!(L::minElement(Arc::clone(&lst))?, 42);
    Ok(())
}
