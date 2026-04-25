//! Translation of NFFrontEnd/NFLookupTree.mo
//!
//! Provides a lookup tree for the NF (Normal Form) frontend, mapping string
//! identifiers to Entry variants (CLASS, COMPONENT, IMPORT).
//!
//! Extends BaseAvlTree with Key=String and Value=Entry.

// ============================================================================
// Type aliases
// ============================================================================

/// Type alias for the key type (String in MetaModelica).
pub type Key = String;

/// Type alias for the Entry union type.
pub type Entry = entry::Entry;

/// Type alias for the Tree<Key, Entry>.
pub type TreeKV = Tree<Key, Entry>;

/// Empty tree constructor (matches `new` in BaseAvlTree).
pub fn new() -> TreeKV {
    Tree::EMPTY
}

// ============================================================================
// Entry uniontype (CLASS, COMPONENT, IMPORT)
// ============================================================================

pub mod entry {
    /// Union type for lookup entries.
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[allow(non_camel_case_types)]
    pub enum Entry {
        CLASS { index: i32 },
        COMPONENT { index: i32 },
        IMPORT { index: i32 },
    }

    /// index(entry) - extracts the index field from any Entry variant.
    /// Matches: `index := match entry case CLASS() then entry.index; case COMPONENT() then entry.index; case IMPORT() then entry.index; end match;`
    pub fn index(e: &Entry) -> i32 {
        match e {
            Entry::CLASS { index }
            | Entry::COMPONENT { index }
            | Entry::IMPORT { index } => *index,
        }
    }

    /// isEqual(entry1, entry2) - compares two entries by their index.
    /// Matches: `output Boolean isEqual = index(entry1) == index(entry2);`
    pub fn is_equal(entry1: &Entry, entry2: &Entry) -> bool {
        index(entry1) == index(entry2)
    }

    /// isImport(entry) - checks if an entry is of type IMPORT.
    /// Matches: `isImport := match entry case IMPORT() then true; else false; end match;`
    pub fn is_import(e: &Entry) -> bool {
        matches!(e, Entry::IMPORT { .. })
    }
}

// ============================================================================
// Overridden functions from BaseAvlTree
// ============================================================================

/// keyStr: convert a String key to a String.
/// Matches: `outString := inKey`
pub fn key_str_fn(in_key: &str) -> String {
    in_key.to_string()
}

/// valueStr: convert an Entry value to a display string.
/// Matches: `outString := match inValue case Entry.CLASS() then "class " + String(inValue.index); case Entry.COMPONENT() then "comp " + String(inValue.index); end match;`
///
/// Note: IMPORT is not handled explicitly in the MetaModelica source and falls
/// through without output (empty string). We handle it the same way.
pub fn value_str(in_value: &Entry) -> String {
    match in_value {
        entry::Entry::CLASS { index } => format!("class {index}"),
        entry::Entry::COMPONENT { index } => format!("comp {index}"),
        entry::Entry::IMPORT { .. } => String::new(),
    }
}

/// keyCompare: compare two String keys.
/// Matches: `outResult := stringCompare(inKey1, inKey2)`
pub fn key_compare_fn(in_key1: &str, in_key2: &str) -> i32 {
    match in_key1.cmp(in_key2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

// ============================================================================
// Re-exported BaseAvlTree functions specialized for Tree<Key, Entry>
// ============================================================================

pub use crate::baseavltree::{
    add, add_list, balance, calculate_balance,
    from_list, get, get_opt, height,
    join, key_compare, list_keys, map, rotate_left,
    rotate_right, set_tree_left_right, update, Tree,
};

