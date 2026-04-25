//! Translation of Util/DiffAlgorithm.mo
//!
//! Compares text and other sequences, generating a sequence of additions
//! and deletions. Based on Eugene Myers' O(ND) Difference Algorithm.
//!
//! Note: MetaModelica uses 1-based indexing; this Rust code uses 0-based indexing.

use anyhow::Result;
use im::Vector;

// Persistent list type
type List<T> = Vector<T>;

// Type alias for diff entries: tuple<Diff, list<T>>
type DiffEntry<T> = (Diff, List<T>);
type DiffList<T> = Vec<DiffEntry<T>>;

// ============================================================================
// Diff - Enumeration for diff operations
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Diff {
    Add,
    Delete,
    Equal,
}

// ============================================================================
// Helper: threaded for - check if all elements satisfy a predicate
// ============================================================================

/// Checks if all elements in the range satisfy the equality predicate.
fn all_equal_range<T, F>(arr1: &[T], arr2: &[T], start1: usize, start2: usize, len: usize, equals: F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    arr1[start1..start1 + len]
        .iter()
        .zip(arr2[start2..start2 + len].iter())
        .all(|(a, b)| equals(a, b))
}

// ============================================================================
// addToList<T>
// ============================================================================

fn add_to_list<T: Clone>(
    mut lst: DiffList<T>,
    cur_diff: Diff,
    mut acc: List<T>,
    new_diff: Diff,
    t: T,
) -> (DiffList<T>, Diff, List<T>) {
    let cur_diff = if cur_diff == new_diff { new_diff } else { cur_diff };
    if cur_diff == new_diff {
        let mut new_acc = List::new();
        new_acc.push_front(t);
        acc = new_acc;
    } else {
        if !acc.is_empty() {
            let reversed: List<T> = acc.iter().rev().cloned().collect();
            lst.insert(0, (cur_diff, reversed));
        }
        let mut new_acc = List::new();
        new_acc.push_front(t);
        acc = new_acc;
    }
    (lst, new_diff, acc)
}

// ============================================================================
// endList<T>
// ============================================================================

fn end_list<T: Clone>(mut lst: DiffList<T>, cur_diff: Diff, acc: List<T>) -> DiffList<T> {
    if !acc.is_empty() {
        let reversed: List<T> = acc.iter().rev().cloned().collect();
        lst.insert(0, (cur_diff, reversed));
    }
    lst
}

// ============================================================================
// OnlyAdditions<T>
// ============================================================================

fn only_additions<T: Clone, E, W>(
    arr1: &[T],
    arr2: &[T],
    equals: E,
    is_whitespace: W,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
) -> Option<DiffList<T>>
where
    E: Fn(&T, &T) -> bool,
    W: Fn(&T) -> bool,
{
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut d = Diff::Equal;
    let mut lst: List<T> = List::new();
    let mut result: DiffList<T> = Vec::new();

    while start1 + x <= end1 && start2 + y <= end2 {
        if equals(&arr1[start1 + x], &arr2[start2 + y]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Equal, arr1[start1 + x].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            x += 1;
            y += 1;
        } else if is_whitespace(&arr1[start1 + x]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Delete, arr1[start1 + x].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            x += 1;
        } else {
            return None;
        }
    }

    while start1 + x <= end1 {
        if is_whitespace(&arr1[start1 + x]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Delete, arr1[start1 + x].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            x += 1;
        } else {
            return None;
        }
    }

    while start2 + y <= end2 {
        if is_whitespace(&arr2[start2 + y]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Add, arr2[start2 + y].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            y += 1;
        } else {
            return None;
        }
    }

    let final_lst = end_list(result, d, lst);
    let mut reversed = Vec::new();
    for item in final_lst.iter().rev() {
        reversed.push(item.clone());
    }
    Some(reversed)
}

// ============================================================================
// onlyRemovals<T>
// ============================================================================

fn only_removals<T: Clone, E, W>(
    arr1: &[T],
    arr2: &[T],
    equals: E,
    is_whitespace: W,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
) -> Option<DiffList<T>>
where
    E: Fn(&T, &T) -> bool,
    W: Fn(&T) -> bool,
{
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut d = Diff::Equal;
    let mut lst: List<T> = List::new();
    let mut result: DiffList<T> = Vec::new();

    while start1 + x <= end1 && start2 + y <= end2 {
        if equals(&arr1[start1 + x], &arr2[start2 + y]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Equal, arr1[start1 + x].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            x += 1;
            y += 1;
        } else if is_whitespace(&arr2[start2 + y]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Add, arr2[start2 + y].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            y += 1;
        } else {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Delete, arr1[start1 + x].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            x += 1;
        }
    }

    while start1 + x <= end1 {
        if is_whitespace(&arr1[start1 + x]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Delete, arr1[start1 + x].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            x += 1;
        } else {
            return None;
        }
    }

    while start2 + y <= end2 {
        if is_whitespace(&arr2[start2 + y]) {
            let (res, new_d, new_lst) = add_to_list(result, d, lst, Diff::Add, arr2[start2 + y].clone());
            result = res;
            d = new_d;
            lst = new_lst;
            y += 1;
        } else {
            return None;
        }
    }

    let final_lst = end_list(result, d, lst);
    let mut reversed = Vec::new();
    for item in final_lst.iter().rev() {
        reversed.push(item.clone());
    }
    Some(reversed)
}

// ============================================================================
// Myers' Greedy Diff
// ============================================================================

type PathPoint = (usize, usize);

fn myers_greedy_diff<T: Clone, E>(
    arr1: &[T],
    arr2: &[T],
    equals: E,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
) -> DiffList<T>
where
    E: Fn(&T, &T) -> bool,
{
    let len1 = end1.saturating_sub(start1) + 1;
    let len2 = end2.saturating_sub(start2) + 1;
    let max_iter = len1 + len2;

    // V array: offset by max_iter to handle negative indices for k
    let sz = 2 * max_iter + 1;
    let middle = max_iter;
    let mut v: Vec<usize> = vec![0; sz];
    let mut paths: Vec<Vec<PathPoint>> = vec![Vec::new(); sz];

    for d_val in 0..=max_iter {
        let mut k: isize = -(d_val as isize);
        let k_end = d_val as isize;
        while k <= k_end {
            let k_usize = (k + middle as isize) as usize;

            let (x, prev_path);
            if k == -(d_val as isize) || (k != k_end && v[k_usize.saturating_sub(1) + middle] < v[k_usize + 1]) {
                x = *v.get(k_usize + 1).unwrap_or(&0);
                prev_path = paths[k_usize + 1].clone();
            } else {
                x = *v.get(k_usize.saturating_sub(1)).unwrap_or(&0) + 1;
                prev_path = paths[k_usize.saturating_sub(1)].clone();
            }

            let y = if x as isize - k >= 0 {
                (x as isize - k) as usize
            } else {
                0
            };

            let mut new_path = prev_path.clone();
            new_path.push((x, y));
            paths[k_usize] = new_path;

            // Snake: extend along diagonal where elements are equal
            let mut snake_x = x;
            let mut snake_y = y;
            while snake_x < len1 && snake_y < len2 && equals(&arr1[start1 + snake_x], &arr2[start2 + snake_y]) {
                snake_x += 1;
                snake_y += 1;
                let mut sp = paths[k_usize].clone();
                sp.push((snake_x, snake_y));
                paths[k_usize] = sp;
            }
            v[k_usize] = snake_x;

            if snake_x >= len1 && snake_y >= len2 {
                return myers_greedy_path_to_diff(arr1, arr2, start1, start2, &paths[k_usize]);
            }

            k += 2;
        }
    }

    Vec::new()
}

fn myers_greedy_path_to_diff<T: Clone>(
    arr1: &[T],
    arr2: &[T],
    start1: usize,
    start2: usize,
    path: &[PathPoint],
) -> DiffList<T> {
    if path.is_empty() {
        return Vec::new();
    }

    let mut result: DiffList<T> = Vec::new();
    let (mut x2, mut y2) = path[0];
    let mut d1 = Diff::Equal;
    let mut d2 = Diff::Equal;
    let mut lst: Vec<T> = Vec::new();

    for point in path.iter().skip(1) {
        let (x1, y1) = *point;

        if x2.saturating_sub(x1) == 1 && y2.saturating_sub(y1) == 1 {
            d1 = Diff::Equal;
            lst.push(arr1[start1 + x1].clone());
        } else if x2.saturating_sub(x1) == 1 && y2 == y1 {
            d1 = Diff::Delete;
            lst.push(arr1[start1 + x1].clone());
        } else if y2.saturating_sub(y1) == 1 && x2 == x1 {
            d1 = Diff::Add;
            lst.push(arr2[start2 + y1].clone());
        } else {
            continue;
        }

        if d1 != d2 {
            if !lst.is_empty() {
                let reversed: List<T> = lst.iter().rev().cloned().collect();
                result.push((d2, reversed));
            }
            let new_elem = if d1 == Diff::Add {
                arr2[start2 + y1].clone()
            } else {
                arr1[start1 + x1].clone()
            };
            lst.clear();
            lst.push(new_elem);
        }
        d2 = d1;
        x2 = x1;
        y2 = y1;
    }

    if !lst.is_empty() {
        let reversed: List<T> = lst.iter().rev().cloned().collect();
        result.push((d2, reversed));
    }

    result
}

// ============================================================================
// trimCommonPrefix<T>
// ============================================================================

fn trim_common_prefix<T: Clone, E, W>(
    arr1: &[T],
    mut start1: usize,
    end1: usize,
    arr2: &[T],
    mut start2: usize,
    end2: usize,
    equals: E,
    is_whitespace_not_comment: W,
) -> (List<T>, usize, usize)
where
    E: Fn(&T, &T) -> bool,
    W: Fn(&T) -> bool,
{
    let mut lst: List<T> = List::new();

    while start1 <= end1 && start2 <= end2 {
        if equals(&arr1[start1], &arr2[start2]) {
            lst.push_front(arr1[start1].clone());
            start1 += 1;
            start2 += 1;
        } else if start2 + 1 <= end2 && is_whitespace_not_comment(&arr2[start2]) {
            if !equals(&arr1[start1], &arr2[start2 + 1]) {
                break;
            }
            start2 += 1;
        } else {
            break;
        }
    }

    if !lst.is_empty() {
        let reversed: List<T> = lst.iter().rev().cloned().collect();
        (reversed, start1, start2)
    } else {
        (List::new(), start1, start2)
    }
}

// ============================================================================
// trimCommonSuffix<T>
// ============================================================================

fn trim_common_suffix<T: Clone, E, W>(
    arr1: &[T],
    start1: usize,
    mut end1: usize,
    arr2: &[T],
    start2: usize,
    mut end2: usize,
    equals: E,
    is_whitespace_not_comment: W,
) -> (List<T>, usize, usize)
where
    E: Fn(&T, &T) -> bool,
    W: Fn(&T) -> bool,
{
    let mut lst: List<T> = List::new();

    while start1 <= end1 && start2 <= end2 {
        if equals(&arr1[end1], &arr2[end2]) {
            lst.push_front(arr1[end1].clone());
            end1 = end1.saturating_sub(1);
            end2 = end2.saturating_sub(1);
        } else if start2 <= end2.saturating_sub(1) && is_whitespace_not_comment(&arr2[end2]) {
            if end2 > 0 && !equals(&arr1[end1], &arr2[end2 - 1]) {
                break;
            }
            end2 = end2.saturating_sub(1);
        } else {
            break;
        }
    }

    if !lst.is_empty() {
        let reversed: List<T> = lst.iter().rev().cloned().collect();
        (reversed, end1, end2)
    } else {
        (List::new(), end1, end2)
    }
}

// ============================================================================
// diffSeq<T> - core recursive diff function
// ============================================================================

fn diff_seq<T, E, W, WC>(
    arr1: &[T],
    arr2: &[T],
    equals: E,
    is_whitespace: W,
    is_whitespace_not_comment: WC,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
    in_prefixes: &DiffList<T>,
    in_suffixes: &DiffList<T>,
) -> Result<DiffList<T>>
where
    T: Clone,
    E: Fn(&T, &T) -> bool,
    W: Fn(&T) -> bool,
    WC: Fn(&T) -> bool,
{
    let len1 = end1.saturating_sub(start1) + 1;
    let len2 = end2.saturating_sub(start2) + 1;
    let mut prefixes: DiffList<T> = in_prefixes.to_vec();
    let mut suffixes: DiffList<T> = in_suffixes.to_vec();

    // Both empty
    if len1 < 1 && len2 < 1 {
        let mut out = prefixes.to_vec();
        for s in suffixes.iter().rev() {
            out.push((s.0.clone(), s.1.clone()));
        }
        return Ok(out);
    }
    // arr1 empty: all additions
    if len1 < 1 {
        let items: List<T> = (start2..=end2).rev().map(|i| arr2[i].clone()).collect();
        let mut out = prefixes.to_vec();
        out.push((Diff::Add, items));
        for s in suffixes.iter().rev() {
            out.push((s.0.clone(), s.1.clone()));
        }
        return Ok(out);
    }
    // arr2 empty: all deletions
    if len2 < 1 {
        let items: List<T> = (start1..=end1).rev().map(|i| arr1[i].clone()).collect();
        let mut out = prefixes.to_vec();
        out.push((Diff::Delete, items));
        for s in suffixes.iter().rev() {
            out.push((s.0.clone(), s.1.clone()));
        }
        return Ok(out);
    }

    // Check if sequences are equal
    if len1 == len2 && all_equal_range(arr1, arr2, start1, start2, len1, &equals) {
        let items: List<T> = (start1..=end1).rev().map(|i| arr1[i].clone()).collect();
        let mut out = Vec::new();
        out.push((Diff::Equal, items));
        return Ok(out);
    }

    // Trim common prefix
    let (prefix_lst, new_start1, new_start2) =
        trim_common_prefix(arr1, start1, end1, arr2, start2, end2, &equals, &is_whitespace_not_comment);
    if !prefix_lst.is_empty() {
        prefixes.push((Diff::Equal, prefix_lst));
    }

    // Trim common suffix
    let (suffix_lst, new_end1, new_end2) =
        trim_common_suffix(arr1, new_start1, end1, arr2, new_start2, end2, &equals, &is_whitespace_not_comment);
    if !suffix_lst.is_empty() {
        suffixes.push((Diff::Equal, suffix_lst));
    }

    // Check if anything changed
    if new_start1 != start1 || new_start2 != start2 || new_end1 != end1 || new_end2 != end2 {
        return diff_seq(
            arr1, arr2, &equals, &is_whitespace, &is_whitespace_not_comment,
            new_start1, new_end1, new_start2, new_end2,
            &prefixes, &suffixes,
        );
    }

    // Try matchcontinue: onlyAdditions, then onlyRemovals, then myersGreedyDiff
    if let Some(out) = only_additions(arr1, arr2, &equals, &is_whitespace, new_start1, new_end1, new_start2, new_end2) {
        let prefixes_clone: DiffList<T> = prefixes.iter().map(|(d, l)| (d.clone(), l.clone())).collect();
        let suffixes_clone: DiffList<T> = suffixes.iter().map(|(d, l)| (d.clone(), l.clone())).collect();
        let mut result: DiffList<T> = Vec::new();
        for p in prefixes_clone.iter().rev() {
            result.push((p.0.clone(), p.1.clone()));
        }
        result.extend(out);
        for s in suffixes_clone.iter() {
            result.push((s.0.clone(), s.1.clone()));
        }
        return Ok(result);
    }

    if let Some(out) = only_removals(arr1, arr2, &equals, &is_whitespace, new_start1, new_end1, new_start2, new_end2) {
        let prefixes_clone: DiffList<T> = prefixes.iter().map(|(d, l)| (d.clone(), l.clone())).collect();
        let suffixes_clone: DiffList<T> = suffixes.iter().map(|(d, l)| (d.clone(), l.clone())).collect();
        let mut result: DiffList<T> = Vec::new();
        for p in prefixes_clone.iter().rev() {
            result.push((p.0.clone(), p.1.clone()));
        }
        result.extend(out);
        for s in suffixes_clone.iter() {
            result.push((s.0.clone(), s.1.clone()));
        }
        return Ok(result);
    }

    // Fall back to myersGreedyDiff
    let out = myers_greedy_diff(arr1, arr2, &equals, new_start1, new_end1, new_start2, new_end2);
    let mut result: DiffList<T> = prefixes.iter().map(|(d, l)| (d.clone(), l.clone())).collect();
    result.extend(out);
    result.extend(suffixes.iter().map(|(d, l)| (d.clone(), l.clone())));
    Ok(result)
}

// ============================================================================
// diff<T> - Public entry point
// ============================================================================

/// Main diff function. Compares two sequences and returns a list of diff operations.
pub fn diff<T, E, W, WC, TS>(
    seq1: &List<T>,
    seq2: &List<T>,
    equals: E,
    is_whitespace: W,
    is_whitespace_not_comment: WC,
    _to_string: TS,
) -> DiffList<T>
where
    T: Clone,
    E: Fn(&T, &T) -> bool,
    W: Fn(&T) -> bool,
    WC: Fn(&T) -> bool,
    TS: Fn(&T) -> String,
{
    let arr1: Vec<T> = seq1.iter().cloned().collect();
    let arr2: Vec<T> = seq2.iter().cloned().collect();

    let start1 = 0;
    let start2 = 0;
    let end1 = arr1.len().saturating_sub(1);
    let end2 = arr2.len().saturating_sub(1);

    let empty_list: DiffList<T> = Vec::new();
    match diff_seq(
        &arr1, &arr2,
        equals, is_whitespace, is_whitespace_not_comment,
        start1, end1, start2, end2,
        &empty_list, &empty_list,
    ) {
        Ok(out) => out,
        Err(_) => Vec::new(),
    }
}

// ============================================================================
// partialPrintDiff<T> - Printing functions
// ============================================================================

/// Diff strings configuration for print functions.
pub struct DiffStrings {
    pub equal_open: &'static str,
    pub equal_close: &'static str,
    pub add_open: &'static str,
    pub add_close: &'static str,
    pub del_open: &'static str,
    pub del_close: &'static str,
    pub print_add: bool,
    pub print_equal: bool,
    pub print_delete: bool,
}

impl DiffStrings {
    pub fn terminal_color() -> Self {
        DiffStrings {
            equal_open: "",
            equal_close: "",
            add_open: "\x1B[4;32m",
            add_close: "\x1B[0m",
            del_open: "\x1B[9;31m",
            del_close: "\x1B[0m",
            print_add: true,
            print_equal: true,
            print_delete: true,
        }
    }

    pub fn xml() -> Self {
        DiffStrings {
            equal_open: "<equal>",
            equal_close: "</equal>",
            add_open: "<add>",
            add_close: "</add>",
            del_open: "<del>",
            del_close: "</del>",
            print_add: true,
            print_equal: true,
            print_delete: true,
        }
    }

    pub fn print_actual() -> Self {
        DiffStrings {
            equal_open: "",
            equal_close: "",
            add_open: "",
            add_close: "",
            del_open: "",
            del_close: "",
            print_add: true,
            print_equal: true,
            print_delete: false,
        }
    }
}

/// Prints the diff to a string with the given configuration.
pub fn partial_print_diff<T: std::fmt::Display + Clone>(
    seq: &DiffList<T>,
    to_string: impl Fn(&T) -> String,
    diff_strings: &DiffStrings,
) -> String {
    let mut res = String::new();

    for (diff_type, ts) in seq.iter() {
        let (open, close, print_this) = match diff_type {
            Diff::Equal => (
                diff_strings.equal_open,
                diff_strings.equal_close,
                diff_strings.print_equal,
            ),
            Diff::Add => (
                diff_strings.add_open,
                diff_strings.add_close,
                diff_strings.print_add,
            ),
            Diff::Delete => (
                diff_strings.del_open,
                diff_strings.del_close,
                diff_strings.print_delete,
            ),
        };

        if !ts.is_empty() && (print_this || (diff_strings.print_equal && diff_strings.print_add && diff_strings.print_delete)) {
            res.push_str(open);
            for t in ts.iter() {
                res.push_str(&to_string(t));
            }
            res.push_str(close);
        }
    }

    res
}

/// Prints diff with terminal color codes.
pub fn print_diff_terminal_color<T: std::fmt::Display + Clone>(
    seq: &DiffList<T>,
    to_string: impl Fn(&T) -> String,
) -> String {
    partial_print_diff(seq, to_string, &DiffStrings::terminal_color())
}

/// Prints diff with XML tags.
pub fn print_diff_xml<T: std::fmt::Display + Clone>(
    seq: &DiffList<T>,
    to_string: impl Fn(&T) -> String,
) -> String {
    partial_print_diff(seq, to_string, &DiffStrings::xml())
}

/// Prints only additions (actual output, hiding deletions).
pub fn print_actual<T: std::fmt::Display + Clone>(
    seq: &DiffList<T>,
    to_string: impl Fn(&T) -> String,
) -> String {
    partial_print_diff(seq, to_string, &DiffStrings::print_actual())
}

// ============================================================================
// printStartToEnd<T>
// ============================================================================

/// Prints array elements from startIndex to endIndex using toString.
pub fn print_start_to_end<T>(arr: &[T], start_index: usize, end_index: usize, to_string: impl Fn(&T) -> String) -> String {
    let mut result = String::new();
    for i in start_index..=end_index {
        if i < arr.len() {
            result.push_str(&to_string(&arr[i]));
        }
    }
    result
}
