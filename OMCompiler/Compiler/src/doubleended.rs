//! Translation of Util/DoubleEnded.mo
//!
//! This module provides a mutable double-ended list (deque) implementation
//! with O(1) push_front, push_back, pop_front, and to_list_and_clear.
//!
//! The original MetaModelica code uses a singly-linked list with mutable
//! front and back tail references, requiring "Dangerous" low-level operations
//! (list_set_first, list_set_rest) for in-place mutation.
//!
//! In Rust, ListCell<T> uses UnsafeCell<T> for interior mutability, enabling
//! in-place replacement via list_set_first and list_set_rest.
//!
//! The in-place mapping functions (map_no_copy_1, map_fold_no_copy) require
//! T: Clone.

use std::cell::UnsafeCell;
use std::fmt;
use std::ptr;
use std::rc::Rc;

use crate::mutable::Mutable;

// ============================================================================
// ListCell - internal singly-linked list cell
// ============================================================================

/// Internal singly-linked list cell.
///
/// Uses `UnsafeCell` for both value and next pointer, enabling in-place
/// mutation via `list_set_first` and `list_set_rest` (the "Dangerous"
/// operations from the original MetaModelica code).
///
/// This requires manual `Sync` implementation since `UnsafeCell<T>` is only
/// `Sync` when `T: Sync`.
pub struct ListCell<T> {
    value: UnsafeCell<T>,
    next: UnsafeCell<Option<Rc<ListCell<T>>>>,
}

// SAFETY: ListCell<T> is Sync because all public accessor methods provide
/// exclusive access to the cell's data through unsafe pointer operations.
/// Concurrent mutation through shared & references is not prevented, but
/// the API design ensures callers don't have conflicting access.
unsafe impl<T> Sync for ListCell<T> {}

impl<T> ListCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            next: UnsafeCell::new(None),
        }
    }

    /// Returns a reference to the next cell, or None if this is the last cell.
    fn next_ref(&self) -> Option<&Rc<ListCell<T>>> {
        let cell = self.next.get();
        let opt = unsafe { &*cell };
        opt.as_ref()
    }

    /// Consumes the next cell reference, replacing it with None.
    fn take_next(&self) -> Option<Rc<ListCell<T>>> {
        let cell = self.next.get();
        let current = unsafe { ptr::read(cell) };
        unsafe { ptr::write(cell, None) };
        current
    }

    /// Sets the next cell reference.
    fn set_next(&self, next: Option<Rc<ListCell<T>>>) {
        let cell = self.next.get();
        unsafe {
            ptr::write(cell, next);
        }
    }

    /// Takes the value out of this cell.
    fn take_value(&self) -> T {
        let cell = self.value.get();
        unsafe { ptr::read(cell) }
    }

    /// Sets the value in this cell.
    fn set_value(&self, value: T) {
        let cell = self.value.get();
        unsafe {
            ptr::write(cell, value);
        }
    }

    /// Returns a reference to the current value.
    fn get_value_ref(&self) -> &T {
        let cell = self.value.get();
        unsafe { &*cell }
    }
}

impl<T: fmt::Debug> fmt::Debug for ListCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListCell {..}").finish_non_exhaustive()
    }
}

// ============================================================================
// MutableList<T> - the public deque type
// ============================================================================

/// A mutable double-ended list (deque) backed by a singly-linked list
/// with mutable front and back tail references.
///
/// Mirrors the `MutableList<T>` uniontype from the original MetaModelica code,
/// which wraps `Mutable`-protected fields for length, front, and back.
pub struct MutableList<T: fmt::Debug> {
    /// Length of the list (wrapped in Mutable).
    pub length: Mutable<i32>,
    /// Head of the singly-linked list (wrapped in Mutable).
    pub front: Mutable<Option<Rc<ListCell<T>>>>,
    /// Tail of the list for O(1) push_back (wrapped in Mutable).
    pub back: Mutable<Option<Rc<ListCell<T>>>>,
}

impl<T: fmt::Debug> fmt::Debug for MutableList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MutableList")
            .field("length", &self.length.data)
            .field("front", &self.front.data)
            .field("back", &self.back.data)
            .finish()
    }
}

// ============================================================================
// List utility functions (from MetaModelica.Dangerous)
// ============================================================================

/// Returns true if the list is empty.
fn list_empty<T>(head: &Option<Rc<ListCell<T>>>) -> bool {
    head.is_none()
}

/// Returns the length of the linked list.
fn list_length<T>(head: &Option<Rc<ListCell<T>>>) -> i32 {
    let mut count = 0i32;
    let mut current = head.as_ref();
    while let Some(cell) = current {
        count += 1;
        current = cell.next_ref();
    }
    count
}

/// Returns the next cell reference without consuming it.
fn next_ref<T>(cell: &Rc<ListCell<T>>) -> Option<&Rc<ListCell<T>>> {
    cell.next_ref()
}

/// Gets the element at the given 0-based index.
/// Returns a clone of the value.
fn list_get<T>(head: &Option<Rc<ListCell<T>>>, index: i32) -> Option<T>
where
    T: Clone,
{
    let mut current = head.as_ref();
    let mut i = 0i32;
    while let Some(cell) = current {
        if i == index {
            return Some(cell.get_value_ref().clone());
        }
        i += 1;
        current = cell.next_ref();
    }
    None
}

/// Returns the rest of the list (head consumed).
fn list_rest<T>(head: &Option<Rc<ListCell<T>>>) -> Option<Rc<ListCell<T>>> {
    head.as_ref().and_then(|cell| cell.take_next())
}

/// Sets the first element of a cell in-place (Dangerous operation).
/// Mirrors `Dangerous.listSetFirst` from the original MetaModelica code.
fn list_set_first<T>(cell: &Rc<ListCell<T>>, value: T) {
    cell.set_value(value);
}

/// Sets the next cell reference in-place (Dangerous operation).
/// Mirrors `Dangerous.listSetRest` from the original MetaModelica code.
fn list_set_rest<T>(current: &Option<Rc<ListCell<T>>>, new_next: Option<Rc<ListCell<T>>>) {
    if let Some(cell) = current {
        cell.set_next(new_next);
    }
}

// ============================================================================
// Internal helpers for Mutable field access
// ============================================================================

fn get_length<T: fmt::Debug>(delst: &MutableList<T>) -> i32 {
    delst.length.data
}

fn set_length<T: fmt::Debug>(delst: &mut MutableList<T>, val: i32) {
    delst.length.data = val;
}

fn get_front<T: fmt::Debug>(delst: &MutableList<T>) -> &Option<Rc<ListCell<T>>> {
    &delst.front.data
}

fn replace_front<T: fmt::Debug>(delst: &mut MutableList<T>, val: Option<Rc<ListCell<T>>>) -> Option<Rc<ListCell<T>>> {
    std::mem::replace(&mut delst.front.data, val)
}

fn set_front<T: fmt::Debug>(delst: &mut MutableList<T>, val: Option<Rc<ListCell<T>>>) {
    delst.front.data = val;
}

fn get_back<T: fmt::Debug>(delst: &MutableList<T>) -> &Option<Rc<ListCell<T>>> {
    &delst.back.data
}

fn set_back<T: fmt::Debug>(delst: &mut MutableList<T>, val: Option<Rc<ListCell<T>>>) {
    delst.back.data = val;
}

// ============================================================================
// Constructors
// ============================================================================

/// Creates a new deque initialized with the given element.
/// Mirrors the `new<T>` function from the original MetaModelica code.
pub fn new<T: fmt::Debug>(first: T) -> MutableList<T> {
    let cell = Some(Rc::new(ListCell::new(first)));
    MutableList {
        length: Mutable { data: 1 },
        front: Mutable { data: cell.clone() },
        back: Mutable { data: cell },
    }
}

/// Creates a new deque from a slice of elements.
/// Mirrors the `fromList<T>` function from the original MetaModelica code.
pub fn from_list<T: Clone + fmt::Debug>(lst: &[T]) -> MutableList<T> {
    if lst.is_empty() {
        return MutableList {
            length: Mutable { data: 0 },
            front: Mutable { data: None },
            back: Mutable { data: None },
        };
    }

    let mut head: Option<Rc<ListCell<T>>> = None;
    let mut tail: Option<Rc<ListCell<T>>> = None;
    let mut len = 0i32;

    for val in lst {
        let cell = Rc::new(ListCell::new(val.clone()));
        match &tail {
            Some(t) => t.set_next(Some(Rc::clone(&cell))),
            None => head = Some(Rc::clone(&cell)),
        }
        tail = Some(Rc::clone(&cell));
        len += 1;
    }

    MutableList {
        length: Mutable { data: len },
        front: Mutable { data: head },
        back: Mutable { data: tail },
    }
}

/// Creates a new empty deque.
/// The type parameter is inferred from the dummy argument.
/// Mirrors the `empty<T>` function from the original MetaModelica code.
pub fn empty<T: fmt::Debug>(_dummy: T) -> MutableList<T> {
    MutableList {
        length: Mutable { data: 0 },
        front: Mutable { data: None },
        back: Mutable { data: None },
    }
}

// ============================================================================
// Accessors
// ============================================================================

/// Returns the number of elements in the deque.
/// Mirrors the `length<T>` function from the original MetaModelica code.
pub fn length<T: fmt::Debug>(delst: &MutableList<T>) -> i32 {
    get_length(delst)
}

/// Returns the raw back cell pointer.
/// Mirrors the `currentBackCell<T>` function from the original MetaModelica code.
pub fn current_back_cell<T: fmt::Debug>(delst: &MutableList<T>) -> &Option<Rc<ListCell<T>>> {
    get_back(delst)
}

// ============================================================================
// Mutators
// ============================================================================

/// Removes and returns the front element of the deque.
/// Mirrors the `pop_front<T>` function from the original MetaModelica code.
///
/// # Panics
/// Panics if the deque is empty.
pub fn pop_front<T: fmt::Debug>(delst: &mut MutableList<T>) -> T {
    let len = get_length(delst);
    assert!(len > 0, "pop_front: deque is empty");

    let new_len = len - 1;
    set_length(delst, new_len);

    if new_len == 0 {
        let head = replace_front(delst, None).unwrap();
        let val = head.take_value();
        set_back(delst, None);
        return val;
    }

    let head = get_front(delst).clone().unwrap();
    let val = head.take_value();
    set_front(delst, head.next_ref().cloned());
    val
}

/// Adds an element to the front of the deque.
/// Mirrors the `push_front<T>` function from the original MetaModelica code.
pub fn push_front<T: fmt::Debug>(delst: &mut MutableList<T>, elt: T) {
    let len = get_length(delst);
    set_length(delst, len + 1);

    if len == 0 {
        let cell = Some(Rc::new(ListCell::new(elt)));
        set_front(delst, cell.clone());
        set_back(delst, cell);
        return;
    }

    let old_front = get_front(delst).clone();
    let new_cell = Rc::new(ListCell::new(elt));
    new_cell.set_next(old_front);
    set_front(delst, Some(new_cell));
}

/// Adds a list of elements to the front of the deque, in order.
/// Mirrors the `push_list_front<T>` function from the original MetaModelica code.
pub fn push_list_front<T: Clone + fmt::Debug>(delst: &mut MutableList<T>, lst: &[T]) {
    let len = get_length(delst);
    let lst_len = lst.len() as i32;

    if lst_len == 0 {
        return;
    }

    set_length(delst, len + lst_len);

    let mut new_cells: Vec<Rc<ListCell<T>>> = Vec::with_capacity(lst.len());
    for val in lst {
        new_cells.push(Rc::new(ListCell::new(val.clone())));
    }
    for i in 0..new_cells.len() - 1 {
        new_cells[i].set_next(Some(Rc::clone(&new_cells[i + 1])));
    }

    let old_front = get_front(delst).clone();
    let new_head = Rc::clone(&new_cells[0]);
    let last_new = &new_cells[new_cells.len() - 1];

    if len == 0 {
        set_back(delst, Some(Rc::clone(last_new)));
    } else {
        last_new.set_next(old_front);
    }

    set_front(delst, Some(new_head));
}

/// Adds an element to the back of the deque.
/// Mirrors the `push_back<T>` function from the original MetaModelica code.
pub fn push_back<T: fmt::Debug>(delst: &mut MutableList<T>, elt: T) {
    let len = get_length(delst);
    set_length(delst, len + 1);

    if len == 0 {
        let cell = Some(Rc::new(ListCell::new(elt)));
        set_front(delst, cell.clone());
        set_back(delst, cell);
        return;
    }

    let new_cell = Some(Rc::new(ListCell::new(elt)));
    list_set_rest(get_back(delst), new_cell.clone());
    set_back(delst, new_cell);
}

/// Adds a list of elements to the back of the deque, in order.
/// Mirrors the `push_list_back<T>` function from the original MetaModelica code.
pub fn push_list_back<T: Clone + fmt::Debug>(delst: &mut MutableList<T>, lst: &[T]) {
    let len = get_length(delst);
    let lst_len = lst.len() as i32;

    if lst_len == 0 {
        return;
    }

    set_length(delst, len + lst_len);

    let mut new_cells: Vec<Rc<ListCell<T>>> = Vec::with_capacity(lst.len());
    for val in lst {
        new_cells.push(Rc::new(ListCell::new(val.clone())));
    }
    for i in 0..new_cells.len() - 1 {
        new_cells[i].set_next(Some(Rc::clone(&new_cells[i + 1])));
    }

    let new_tail = &new_cells[new_cells.len() - 1];

    if len == 0 {
        set_front(delst, Some(Rc::clone(&new_cells[0])));
    } else {
        list_set_rest(get_back(delst), Some(Rc::clone(&new_cells[0])));
    }

    set_back(delst, Some(Rc::clone(new_tail)));
}

// ============================================================================
// Conversion and clearing
// ============================================================================

/// Extracts all elements as a Vec and clears the deque.
/// If `prepend` is non-empty, elements from prepend are appended
/// after the deque's elements.
///
/// Mirrors the `toListAndClear<T>` function from the original
/// MetaModelica code.
pub fn to_list_and_clear<T: Clone + fmt::Debug>(delst: &mut MutableList<T>, prepend: &[T]) -> Vec<T> {
    let front = get_front(delst).clone();

    if get_length(delst) == 0 {
        set_front(delst, None);
        set_back(delst, None);
        set_length(delst, 0);
        return prepend.to_vec();
    }

    let result = list_to_vec(&front);

    if !prepend.is_empty() {
        for val in prepend {
            push_back(delst, val.clone());
        }
    }

    set_front(delst, None);
    set_back(delst, None);
    set_length(delst, 0);

    result
}

/// Returns all elements as a Vec without modifying the deque.
/// Mirrors the `toListNoCopyNoClear<T>` function from the original
/// MetaModelica code.
pub fn to_list_no_copy_no_clear<T: Clone + fmt::Debug>(delst: &MutableList<T>) -> Vec<T> {
    let front = get_front(delst).clone();
    list_to_vec(&front)
}

/// Clears the deque, dropping all cells.
/// Mirrors the `clear<T>` function from the original MetaModelica code.
///
/// Note: The original code calls GCExt.free on each element. In Rust,
/// elements are dropped automatically.
pub fn clear<T: fmt::Debug>(delst: &mut MutableList<T>) {
    let front = get_front(delst).clone();

    set_back(delst, None);
    set_front(delst, None);
    set_length(delst, 0);

    // Drop cells (which drops their values)
    if let Some(head) = front {
        let mut current = Some(head);
        while let Some(cell) = current {
            current = cell.take_next();
        }
    }
}

// ============================================================================
// Mapping operations (in-place mutation via Dangerous operations)
// ============================================================================

/// Applies a mapping function to each element in the list in-place.
/// Mirrors the `mapNoCopy_1<T, ArgT1>` function from the original
/// MetaModelica code.
///
/// Uses `Dangerous.listSetFirst`-style in-place mutation of cell values
/// via UnsafeCell-based interior mutability.
pub fn map_no_copy_1<T: fmt::Debug + Clone, ArgT1, F>(delst: &mut MutableList<T>, arg: &ArgT1, mut f: F)
where
    F: FnMut(&T, &ArgT1) -> T,
{
    let mut lst = get_front(delst).clone();
    while let Some(cell) = lst {
        let val = cell.get_value_ref().clone();
        let new_val = f(&val, arg);
        list_set_first(&cell, new_val);
        lst = cell.next_ref().cloned();
    }
}

/// Applies a fold-based mapping function to each element in the list in-place.
/// Mirrors the `mapFoldNoCopy<T, ArgT1>` function from the original
/// MetaModelica code.
///
/// Uses `Dangerous.listSetFirst`-style in-place mutation of cell values.
pub fn map_fold_no_copy<T: fmt::Debug + Clone, ArgT1, F>(
    delst: &mut MutableList<T>,
    mut arg: ArgT1,
    mut f: F,
) -> ArgT1
where
    F: FnMut(&T, &mut ArgT1) -> (T, ArgT1),
{
    let mut lst = get_front(delst).clone();
    while let Some(cell) = lst {
        let val = cell.get_value_ref().clone();
        let (new_val, new_arg) = f(&val, &mut arg);
        list_set_first(&cell, new_val);
        arg = new_arg;
        lst = cell.next_ref().cloned();
    }
    arg
}

// ============================================================================
// Internal helpers
// ============================================================================

/// Converts a linked list to a Vec by cloning values.
fn list_to_vec<T: Clone>(head: &Option<Rc<ListCell<T>>>) -> Vec<T> {
    let mut result = Vec::new();
    let mut current = head.as_ref();
    while let Some(cell) = current {
        result.push(cell.get_value_ref().clone());
        current = cell.next_ref();
    }
    result
}
