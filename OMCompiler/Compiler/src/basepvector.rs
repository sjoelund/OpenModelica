//! Translation of Util/BasePVector.mo
//!
//! This module implements a persistent bit-partitioned vector trie with tail
//! optimization. All operations are non-destructive (return new vectors).
//! Lookup and modifications are effectively O(log_32(N)).
//!
//! To use, instantiate with a concrete type:
//!   let v = BasePVector::new();
//!   let v = v.add(42);
//!   assert_eq!(v.get(0), Ok(&42));

use anyhow::{bail, Result};
use im::Vector as ImmutableList;
use std::fmt;

// ============================================================================
// Types: Node and Vector
// ============================================================================

/// Node in the trie: internal NODE (32 children), VALUE (leaf with data), or EMPTY.
#[derive(Debug, Clone, PartialEq)]
pub enum Node<T> {
    NODE { children: Vec<Node<T>> },
    VALUE { value: T },
    EMPTY,
}

/// A persistent dynamic array backed by a bit-partitioned trie with tail optimization.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T> {
    root: Node<T>,
    tail: Vec<Node<T>>,
    size: i32,
    shift: i32,
}

impl<T: Clone> Vector<T> {
    // ========================================================================
    // Constants
    // ========================================================================

    fn empty_node() -> Node<T> {
        Node::NODE {
            children: vec![Node::EMPTY; 32],
        }
    }

    fn empty_vec() -> Vector<T> {
        Vector {
            root: Vector::<T>::empty_node(),
            tail: Vec::new(),
            size: 0,
            shift: 5,
        }
    }

    // ========================================================================
    // Public API
    // ========================================================================

    /// Returns a new empty Vector.
    pub fn new() -> Self {
        Self::empty_vec()
    }

    /// Appends a value to the end of the Vector.
    pub fn add(&self, in_value: T) -> Self {
        match &self.root {
            Node::NODE { children: _ } => {}
            _ => {}
        }
        // Pattern match on the vector structure
        let tail = &self.tail;
        let sz = self.size;
        let shift = self.shift;

        if (tail.len() as i32) < 32 {
            // Space left in the tail, insert the value in the tail.
            let mut new_tail = tail.clone();
            new_tail.push(Node::VALUE { value: in_value });
            Vector {
                root: self.root.clone(),
                tail: new_tail,
                size: sz + 1,
                shift,
            }
        } else {
            // No space left in the tail. Push the tail into the tree.
            let tail_node = Node::NODE {
                children: tail.clone(),
            };
            let (new_root, new_shift) = Self::push_tail(&self.root, sz, shift, tail_node);
            let new_tail = vec![Node::VALUE { value: in_value }];
            Vector {
                root: new_root,
                tail: new_tail,
                size: sz + 1,
                shift: new_shift,
            }
        }
    }

    /// Appends a list of values to the end of the Vector.
    pub fn add_list(&self, in_list: &[T]) -> Self
    where
        T: Clone,
    {
        let root = &self.root;
        let tail = &self.tail;
        let sz = self.size;
        let shift = self.shift;
        let tail_len = tail.len() as i32;
        let list_len = in_list.len() as i32;

        // Clone in_list into owned values for the Vec approach
        let owned_list: Vec<T> = in_list.to_vec();

        let (new_root, new_tail, new_size) = if tail_len + list_len <= 32 {
            // Space left in the tail, just append the list to it.
            let mut new_tail = tail.clone();
            for v in &owned_list {
                new_tail.push(Node::VALUE { value: v.clone() });
            }
            (root.clone(), new_tail, sz + list_len)
        } else {
            // More elements than can fit in the tail.
            let mut new_tail = tail.clone();
            let mut remaining = owned_list;
            let mut new_size = sz;

            // If the tail isn't already full, fill it up.
            let tail_current_len = new_tail.len() as i32;
            if tail_current_len < 32 {
                let fill_count = 32 - tail_current_len;
                for i in 0..(fill_count as usize) {
                    new_tail.push(Node::VALUE {
                        value: remaining[i].clone(),
                    });
                }
                remaining = remaining[(fill_count as usize)..].to_vec();
                new_size += fill_count;
            }

            // Push the now full tail into the tree.
            let tail_node = Node::NODE {
                children: new_tail.clone(),
            };
            let (mut new_root, _new_shift) =
                Self::push_tail(root, new_size, shift, tail_node);
            new_size += 32 - tail_current_len;
            let mut rest_len = list_len - (32 - tail_current_len);

            // While we have more than 32 elements left, take 32 at a time.
            while rest_len > 32 {
                let mut chunk_tail = vec![Node::EMPTY; 32];
                for i in 0..32 {
                    chunk_tail[i] = Node::VALUE {
                        value: remaining[i as usize].clone(),
                    };
                }
                new_size += 32;
                let tail_node = Node::NODE { children: chunk_tail.clone() };
                let (new_n, _ns) =
                    Self::push_tail(&new_root, new_size, shift, tail_node);
                new_root = new_n;
                rest_len -= 32;
            }

            // Make a new tail of the remaining elements.
            let mut final_tail = Vec::new();
            for v in &remaining {
                final_tail.push(Node::VALUE { value: v.clone() });
            }
            let final_tail_size = final_tail.len() as i32;

            (new_root, final_tail, new_size + final_tail_size)
        };

        Vector {
            root: new_root,
            tail: new_tail,
            size: new_size,
            shift,
        }
    }

    /// Returns the element at the given index (0-based).
    /// Fails if the index is out of bounds.
    pub fn get(&self, in_index: i32) -> Result<T> {
        let tail_off = Self::tail_offset(self.size);
        if in_index < 0 || in_index >= self.size {
            bail!("Index out of bounds: {} (size {})", in_index, self.size);
        }
        if in_index < tail_off {
            // Look the element up in the tree.
            let parent = Self::node_parent(self, in_index)?;
            // in_index is 0-based, use bit extraction for 0-based indexing
            let idx = (in_index & 31) as usize;
            match parent {
                Node::NODE { children } => match &children[idx] {
                    Node::VALUE { value } => Ok(value.clone()),
                    _ => bail!("Expected VALUE node at index {}", in_index),
                },
                _ => bail!("Expected NODE node for parent at index {}", in_index),
            }
        } else {
            // Look the element up in the tail.
            let tail_idx = (in_index - tail_off) as usize;
            match &self.tail[tail_idx] {
                Node::VALUE { value } => Ok(value.clone()),
                _ => bail!("Expected VALUE node in tail at index {}", tail_idx),
            }
        }
    }

    /// Sets the element at the given index (0-based) to the given value.
    /// Fails if the index is out of bounds.
    pub fn set(&self, in_index: i32, in_value: T) -> Result<Self> {
        if in_index < 0 || in_index >= self.size {
            bail!("Index out of bounds: {} (size {})", in_index, self.size);
        }
        let tail_off = Self::tail_offset(self.size);
        let mut result = self.clone();
        if in_index < tail_off {
            // The element is in the tree.
            let new_root =
                Self::node_set(&result.root, in_index, Node::VALUE { value: in_value }, result.shift)?;
            result.root = new_root;
        } else {
            // The element is in the tail.
            let mut new_tail = result.tail.clone();
            let tail_idx = (in_index - tail_off) as usize;
            new_tail[tail_idx] = Node::VALUE { value: in_value };
            result.tail = new_tail;
        }
        Ok(result)
    }

    /// Returns the last value in the Vector. Fails if the Vector is empty.
    pub fn last(&self) -> Result<T> {
        if self.tail.is_empty() {
            bail!("Vector is empty");
        }
        let idx = self.tail.len() - 1;
        match &self.tail[idx] {
            Node::VALUE { value } => Ok(value.clone()),
            _ => bail!("Expected VALUE node in tail"),
        }
    }

    /// Removes the last value in the Vector. Fails if the Vector is empty.
    pub fn pop(&self) -> Result<Self> {
        if self.size == 0 {
            bail!("Cannot pop from empty vector");
        }
        if self.size == 1 {
            return Ok(Self::empty_vec());
        }

        let tail = &self.tail;
        if tail.len() > 1 {
            // Tail contains more than one element, remove the last of them.
            let mut new_tail = tail.clone();
            new_tail.pop();
            Ok(Vector {
                root: self.root.clone(),
                tail: new_tail,
                size: self.size - 1,
                shift: self.shift,
            })
        } else {
            // Tail contains one element.
            // Get the parent node at position sz - 2 (0-based), extract its children as new tail.
            let parent_node = Self::node_parent(self, self.size - 2)?;
            let new_tail = match parent_node {
                Node::NODE { children } => children,
                _ => bail!("Expected NODE for parent in pop"),
            };

            let sz = self.size;
            let shift = self.shift;
            let mut new_root = Self::pop_tail(&self.root, shift, sz)?;

            if Self::is_empty_node(&new_root) {
                new_root = Self::empty_node();
            }

            // Check if we can shrink the tree height
            match &new_root {
                Node::NODE { children } => {
                    if shift > 5 && children.len() >= 2 && matches!(&children[1], Node::EMPTY) {
                        Ok(Vector {
                            root: children[0].clone(),
                            tail: new_tail,
                            size: sz - 1,
                            shift: shift - 5,
                        })
                    } else {
                        Ok(Vector {
                            root: new_root,
                            tail: new_tail,
                            size: sz - 1,
                            shift,
                        })
                    }
                }
                _ => Ok(Vector {
                    root: new_root,
                    tail: new_tail,
                    size: sz - 1,
                    shift,
                }),
            }
        }
    }

    /// Returns a new Vector where the given function has been applied to each element.
    pub fn map<F>(&self, in_func: F) -> Self
    where
        F: Fn(&T) -> T,
    {
        let new_root = Self::map_node(&self.root, &in_func);
        let new_tail = Self::map_node_array(&self.tail, &in_func);
        Vector {
            root: new_root,
            tail: new_tail,
            size: self.size,
            shift: self.shift,
        }
    }

    /// Applies the given function to each element, updating the accumulator.
    pub fn fold<F, A>(&self, in_start: A, in_func: F) -> A
    where
        F: Fn(&T, A) -> A,
    {
        let mut result = in_start;
        result = Self::fold_node(&self.root, &in_func, result);
        result = Self::fold_node_array(&self.tail, &in_func, result);
        result
    }

    /// Returns the number of elements in the Vector.
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Alias for size().
    pub fn length(&self) -> i32 {
        self.size
    }

    /// Returns true if the Vector is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Creates a Vector from a slice.
    pub fn from_list(in_list: &[T]) -> Self
    where
        T: Clone,
    {
        Vector::empty_vec().add_list(in_list)
    }

    /// Creates a vector from a list (reverse order).
    pub fn to_list(&self) -> ImmutableList<T>
    where
        T: Clone,
    {
        let result = self.to_reverse_list();
        // im::Vector has no reverse(), convert via Vec
        let vec: Vec<T> = result.into_iter().collect();
        let mut vec = vec;
        vec.reverse();
        vec.into_iter().collect()
    }

    /// Creates a list from the Vector in reverse order.
    pub fn to_reverse_list(&self) -> ImmutableList<T>
    where
        T: Clone,
    {
        // fold processes elements in order (first to last).
        // Using push_front (cons) builds the list in reverse order,
        // matching the original MetaModelica cons behavior.
        self.fold(ImmutableList::new(), |elem, mut acc| {
            acc.push_front(elem.clone());
            acc
        })
    }

    /// Creates a Vector from an array (slice).
    pub fn from_array(in_array: &[T]) -> Self
    where
        T: Clone,
    {
        Vector::from_list(in_array)
    }

    /// Creates a vector from an array (slice).
    pub fn to_array(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.to_list().into_iter().collect()
    }

    // ========================================================================
    // Debug output
    // ========================================================================

    /// Debug print of the vector structure.
    pub fn print_debug(&self) {
        let sz = self.size;
        let shift = self.shift;
        println!("PVector(size = {}, shift = {}):", sz, shift);
        println!("  tail: [");
        for e in &self.tail {
            Self::print_debug_node(e, "    ");
        }
        println!("  ]");
        Self::print_debug_node(&self.root, "  ");
        println!();
    }

    fn print_debug_node(node: &Node<T>, indent: &str) {
        match node {
            Node::NODE { children } => {
                print!("\n{}[", indent);
                for c in children {
                    Self::print_debug_node(c, &format!("{}  ", indent));
                }
                print!("],");
            }
            Node::VALUE { value: _ } => {
                print!("{}, ", "VALUE");
            }
            Node::EMPTY => {
                print!("E, ");
            }
        }
    }

    // ========================================================================
    // Protected helper functions
    // ========================================================================

    /// Sets the element at the given index in the node tree.
    fn node_set(
        in_node: &Node<T>,
        in_index: i32,
        in_value: Node<T>,
        in_level: i32,
    ) -> Result<Node<T>> {
        let children = match in_node {
            Node::NODE { children } => children.clone(),
            _ => bail!("Expected NODE in node_set"),
        };
        let mut children = children;

        if in_level == 0 {
            // Reached a leaf, replace its value.
            let idx = (in_index & 31) as usize;
            children[idx] = in_value;
        } else {
            // Traverse down the tree using 0-based indexing.
            let child_idx = ((in_index >> in_level) & 31) as usize;
            let new_child = Self::node_set(&children[child_idx], in_index, in_value, in_level - 5)?;
            children[child_idx] = new_child;
        }

        Ok(Node::NODE { children })
    }

    /// Adds a node to the end of the tail.
    fn tail_add(in_tail: &[Node<T>], in_node: Node<T>) -> Vec<Node<T>> {
        let new_len = in_tail.len() + 1;
        let mut out_tail = vec![Node::EMPTY; new_len];
        for i in 0..new_len - 1 {
            out_tail[i] = in_tail[i].clone();
        }
        out_tail[new_len - 1] = in_node;
        out_tail
    }

    /// Pushes a tail into the tree as a new node.
    fn push_tail(
        in_root: &Node<T>,
        in_size: i32,
        in_shift: i32,
        tail_node: Node<T>,
    ) -> (Node<T>, i32) {
        // Do we have any space left in the tree?
        if (in_size >> 5) > (1 << in_shift) {
            // No space left, add another level to the tree.
            let mut nodes = vec![Node::EMPTY; 32];
            nodes[0] = in_root.clone();
            let new_path = Self::new_path(&tail_node, in_shift);
            nodes[1] = new_path;
            (Node::NODE { children: nodes }, in_shift + 5)
        } else {
            // Space left in the tree, push the tail node down.
            (Self::push_tail2(in_root, in_shift, in_size, tail_node), in_shift)
        }
    }

    /// Does the actual pushing into the tree.
    fn push_tail2(
        in_node: &Node<T>,
        in_level: i32,
        in_size: i32,
        in_tail: Node<T>,
    ) -> Node<T> {
        match in_node {
            Node::NODE { children: _ } => {
                let children = match in_node {
                    Node::NODE { children } => children.clone(),
                    _ => panic!("Expected NODE in push_tail2"),
                };
                let mut children = children;
                // Index calculation for where to insert the tail node
                // Uses (in_size - 1) to match original 1-based indexing semantics
                let idx = (((in_size - 1) >> in_level) & 31) as usize;

                let node = if in_level == 5 {
                    in_tail
                } else {
                    let child = &children[idx];
                    Self::push_tail2(child, in_level - 5, in_size, in_tail)
                };

                children[idx] = node;
                Node::NODE { children }
            }
            Node::EMPTY => Self::new_path(&in_tail, in_level),
            _ => in_node.clone(),
        }
    }

    /// Returns a new tail array with the last element removed.
    fn tail_pop(in_tail: &[Node<T>]) -> Vec<Node<T>> {
        let new_len = in_tail.len() - 1;
        let mut out_tail = vec![Node::EMPTY; new_len];
        for i in 0..new_len {
            out_tail[i] = in_tail[i].clone();
        }
        out_tail
    }

    /// Removes the last tail added to the given node.
    fn pop_tail(
        in_node: &Node<T>,
        in_level: i32,
        in_size: i32,
    ) -> Result<Node<T>> {
        let idx = (((in_size - 2) >> in_level) & 31) as usize;

        match in_node {
            Node::NODE { children } => {
                if in_level > 5 {
                    // More than one level in the tree, recurse.
                    let result = Self::pop_tail(&children[idx], in_level - 5, in_size)?;

                    // Only replace if not (empty AND last child at index 0)
                    if !(Self::is_empty_node(&result) && idx == 0) {
                        let mut new_children = children.clone();
                        new_children[idx] = result;
                        Ok(Node::NODE { children: new_children })
                    } else {
                        Ok(result)
                    }
                } else {
                    // idx == 0 means popping the first child (the last tail added)
                    if idx == 0 {
                        // Popping the last node, return empty.
                        Ok(Node::EMPTY)
                    } else {
                        // Replace the node with an empty node.
                        let mut new_children = children.clone();
                        new_children[idx] = Node::EMPTY;
                        Ok(Node::NODE { children: new_children })
                    }
                }
            }
            _ => Ok(Node::EMPTY),
        }
    }

    /// Returns the parent node at the given index (0-based).
    fn node_parent(&self, in_index: i32) -> Result<Node<T>> {
        let mut node = self.root.clone();
        let shift = self.shift;

        // Iterate from shift down to 1 in steps of 5
        let mut level = shift;
        while level >= 1 {
            let children = match &node {
                Node::NODE { children } => children.clone(),
                _ => bail!("Expected NODE in node_parent at level {}", level),
            };
            // 0-based index extraction
            let idx = ((in_index >> level) & 31) as usize;
            if idx >= children.len() {
                bail!("Index {} out of bounds at level {} (children.len() = {})",
                    in_index, level, children.len());
            }
            node = children[idx].clone();
            level -= 5;
        }

        Ok(node)
    }

    /// Returns the tail offset (number of elements before the tail).
    fn tail_offset(in_size: i32) -> i32 {
        if in_size < 32 {
            0
        } else {
            ((in_size - 1) >> 5) << 5
        }
    }

    /// Creates a new node and sets the given node as the first child.
    fn lift_node(in_node: &Node<T>) -> Node<T> {
        let mut nodes = vec![Node::EMPTY; 32];
        nodes[0] = in_node.clone();
        Node::NODE { children: nodes }
    }

    /// Creates a new path of a given length with the given node as leaf.
    fn new_path(in_node: &Node<T>, in_level: i32) -> Node<T> {
        if in_level > 0 {
            Self::lift_node(&Self::new_path(in_node, in_level - 5))
        } else {
            in_node.clone()
        }
    }

    /// Returns true if the given node is empty.
    fn is_empty_node(in_node: &Node<T>) -> bool {
        matches!(in_node, Node::EMPTY)
    }

    /// Maps over a single node.
    fn map_node<F>(in_node: &Node<T>, in_func: &F) -> Node<T>
    where
        F: Fn(&T) -> T,
    {
        match in_node {
            Node::NODE { children } => {
                let new_children = Self::map_node_array(children, in_func);
                Node::NODE {
                    children: new_children,
                }
            }
            Node::VALUE { value } => Node::VALUE {
                value: in_func(value),
            },
            _ => in_node.clone(),
        }
    }

    /// Maps over an array of nodes.
    fn map_node_array<F>(in_nodes: &[Node<T>], in_func: &F) -> Vec<Node<T>>
    where
        F: Fn(&T) -> T,
    {
        let mut out_nodes = in_nodes.to_vec();
        for i in 0..out_nodes.len() {
            let mapped = match &out_nodes[i] {
                Node::NODE { children: _ } | Node::VALUE { value: _ } => {
                    Self::map_node(&out_nodes[i], in_func)
                }
                _ => out_nodes[i].clone(),
            };
            out_nodes[i] = mapped;
        }
        out_nodes
    }

    /// Folds over a single node.
    fn fold_node<F, A>(in_node: &Node<T>, in_func: &F, in_start: A) -> A
    where
        F: Fn(&T, A) -> A,
    {
        match in_node {
            Node::NODE { children } => {
                Self::fold_node_array(children, in_func, in_start)
            }
            Node::VALUE { value } => in_func(value, in_start),
            _ => in_start,
        }
    }

    /// Folds over an array of nodes.
    fn fold_node_array<F, A>(in_nodes: &[Node<T>], in_func: &F, in_start: A) -> A
    where
        F: Fn(&T, A) -> A,
    {
        let mut out_result = in_start;
        for node in in_nodes {
            out_result = Self::fold_node(node, in_func, out_result);
        }
        out_result
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::NODE { children } => write!(f, "NODE({})", children.len()),
            Node::VALUE { value } => write!(f, "VALUE({})", value),
            Node::EMPTY => write!(f, "EMPTY"),
        }
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PVector(size = {}, shift = {})", self.size, self.shift)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let v: Vector<i32> = Vector::new();
        assert!(v.is_empty());
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn test_add_and_get() {
        let v = Vector::new();
        let v = v.add(1);
        let v = v.add(2);
        let v = v.add(3);
        assert_eq!(v.size(), 3);
        assert_eq!(v.get(0).unwrap(), 1);
        assert_eq!(v.get(1).unwrap(), 2);
        assert_eq!(v.get(2).unwrap(), 3);
    }

    #[test]
    fn test_add_persistent() {
        let v1 = Vector::new();
        let v2 = v1.add(1);
        assert!(v1.is_empty());
        assert_eq!(v2.size(), 1);
    }

    #[test]
    fn test_set() {
        let v = Vector::new().add(1).add(2).add(3);
        let v2 = v.set(1, 42).unwrap();
        assert_eq!(v.get(1).unwrap(), 2); // original unchanged
        assert_eq!(v2.get(1).unwrap(), 42);
    }

    #[test]
    fn test_last() {
        let v = Vector::new().add(10).add(20).add(30);
        assert_eq!(v.last().unwrap(), 30);
    }

    #[test]
    fn test_pop() {
        let v = Vector::new().add(1).add(2).add(3);
        let v2 = v.pop().unwrap();
        assert_eq!(v2.size(), 2);
        assert_eq!(v2.last().unwrap(), 2);
        assert_eq!(v.size(), 3); // original unchanged
    }

    #[test]
    fn test_pop_single() {
        let v = Vector::new().add(42);
        let v2 = v.pop().unwrap();
        assert!(v2.is_empty());
    }

    #[test]
    fn test_pop_empty() {
        let v: Vector<i32> = Vector::new();
        assert!(v.pop().is_err());
    }

    #[test]
    fn test_map() {
        let v = Vector::new().add(1).add(2).add(3);
        let v2 = v.map(|x| x * 10);
        assert_eq!(v2.get(0).unwrap(), 10);
        assert_eq!(v2.get(1).unwrap(), 20);
        assert_eq!(v2.get(2).unwrap(), 30);
    }

    #[test]
    fn test_fold() {
        let v = Vector::new().add(1).add(2).add(3);
        let sum = v.fold(0i32, |x, acc| x + acc);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_from_list() {
        let items = vec![10, 20, 30];
        let v = Vector::from_list(&items);
        assert_eq!(v.size(), 3);
        assert_eq!(v.get(0).unwrap(), 10);
    }

    #[test]
    fn test_to_list() {
        let v = Vector::new().add(1).add(2).add(3);
        let lst = v.to_list();
        assert_eq!(lst.len(), 3);
        assert_eq!(lst.get(0), Some(&1));
    }

    #[test]
    fn test_from_array() {
        let items = vec![100, 200];
        let v = Vector::from_array(&items);
        assert_eq!(v.size(), 2);
    }

    #[test]
    fn test_add_list() {
        let v = Vector::new();
        let items = vec![1, 2, 3, 4, 5];
        let v2 = v.add_list(&items);
        assert_eq!(v2.size(), 5);
        assert_eq!(v2.get(4).unwrap(), 5);
    }

    #[test]
    fn test_tail_optimization() {
        // Add 32 elements (fills the tail)
        let mut v = Vector::new();
        for i in 0..32 {
            v = v.add(i);
        }
        assert_eq!(v.size(), 32);
        // Adding one more should push tail into tree
        v = v.add(100);
        assert_eq!(v.size(), 33);
        assert_eq!(v.get(0).unwrap(), 0);
        assert_eq!(v.get(31).unwrap(), 31);
        assert_eq!(v.get(32).unwrap(), 100);
    }

    #[test]
    fn test_many_elements() {
        let mut v = Vector::new();
        for i in 0..200 {
            v = v.add(i);
        }
        assert_eq!(v.size(), 200);
        assert_eq!(v.get(0).unwrap(), 0);
        assert_eq!(v.get(199).unwrap(), 199);
    }

    #[test]
    fn test_to_reverse_list() {
        let v = Vector::new().add(1).add(2).add(3);
        let lst = v.to_reverse_list();
        // to_reverse_list uses fold which prepends, so order is reversed
        assert_eq!(lst.len(), 3);
    }
}
