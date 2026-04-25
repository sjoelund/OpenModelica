//! Translation of FrontEnd/Graphviz.mo
//!
//! This module provides data structures and utilities for generating
//! Graphviz (dot) textual representations of graphs/trees.

use anyhow::{bail, Result};
use im::Vector;
use std::sync::atomic::{AtomicI32, Ordering};

/// Persistent list type
type List<T> = Vector<T>;

// ============================================================================
// Simple type aliases
// ============================================================================

/// A type identifier (mapped from Graphviz.Type)
type Type_ = String;

/// An identifier (mapped from Graphviz.Ident)
type Ident = String;

/// A label string (mapped from Graphviz.Label)
type Label = String;

/// Attributes list
type Attributes = List<Attribute>;

/// Children list
type Children = List<Node>;

// ============================================================================
// Attribute uniontype
// ============================================================================

/// An Attribute is a pair of name and value.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Attribute {
    ATTR { name: String, value: String },
}

// ============================================================================
// Node uniontype
// ============================================================================

/// A graphviz Node is a node of the graph.
/// It has a type and attributes and children.
/// It can also have a list of labels, provided by the LNODE constructor.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Node {
    NODE {
        type_: Type_,
        attributes: Attributes,
        children: Children,
    },
    LNODE {
        type_: Type_,
        label_lst: List<Label>,
        attributes: Attributes,
        children: Children,
    },
}

// ============================================================================
// Helper: box attribute
// ============================================================================

/// Returns the default box attribute.
fn box_attr() -> Attribute {
    Attribute::ATTR {
        name: String::from("shape"),
        value: String::from("box"),
    }
}

/// Default box attribute constant
const BOX: &str = "box attribute (use box_attr() function)";

// ============================================================================
// tick() equivalent - unique integer generator
// ============================================================================

static TICK_COUNTER: AtomicI32 = AtomicI32::new(0);

/// Returns a unique integer (mirrors MetaModelica tick()).
fn tick() -> i32 {
    TICK_COUNTER.fetch_add(1, Ordering::Relaxed)
}

// ============================================================================
// matchcontinue helper functions
// ============================================================================

/// Returns Some(single element) if list has exactly one element.
fn single_elem(lst: &List<String>) -> Option<String> {
    if lst.len() == 1 {
        lst.get(0).cloned()
    } else {
        None
    }
}

/// Returns Some((first, second)) if list has exactly two elements.
fn two_elem(lst: &List<String>) -> Option<(String, String)> {
    if lst.len() == 2 {
        let a = lst.get(0)?.clone();
        let b = lst.get(1)?.clone();
        Some((a, b))
    } else {
        None
    }
}

/// Returns (head, rest) if the list has at least one element.
fn cons_elem(lst: &List<String>) -> Option<(String, List<String>)> {
    if lst.len() >= 1 {
        let head = lst.get(0)?.clone();
        let rest: List<String> = List::from_iter(lst.iter().skip(1).cloned());
        Some((head, rest))
    } else {
        None
    }
}

/// Returns Some((name, value)) if list has exactly one ATTR.
fn single_attr_elem(lst: &Attributes) -> Option<(String, String)> {
    if lst.len() == 1 {
        if let Some(Attribute::ATTR { name, value }) = lst.get(0) {
            return Some((name.clone(), value.clone()));
        }
    }
    None
}

/// Returns (first_attr, rest) if the list has at least one element.
fn cons_attr_elem(lst: &Attributes) -> Option<(Attribute, Attributes)> {
    if lst.len() >= 1 {
        let attr = lst.get(0)?.clone();
        let rest: Attributes = List::from_iter(lst.iter().skip(1).cloned());
        Some((attr, rest))
    } else {
        None
    }
}

// ============================================================================
// matchcontinue success helper
// ============================================================================

/// Helper for matchcontinue: succeeds if the string is non-empty.
fn match_ok(s: &str) -> Result<String> {
    if !s.is_empty() {
        return Ok(s.to_string());
    }
    bail!("no match")
}

// ============================================================================
// Helper: prepend item to front of list (returns new list)
// ============================================================================

fn list_prepend<T: Clone>(lst: &List<T>, item: T) -> List<T> {
    let mut v: Vec<T> = Vec::with_capacity(lst.len() + 1);
    v.push(item);
    v.extend(lst.iter().cloned());
    List::from_iter(v.into_iter())
}

// ============================================================================
// Functions
// ============================================================================

/// Dumps a Graphviz Node on stdout.
pub fn dump(node: &Node) {
    print!("graph AST {{\n");
    let _ = dump_node(node);
    print!("}}\n");
}

/// Dumps a node to a string, returning the node identifier.
fn dump_node(in_node: &Node) -> Result<Ident> {
    match in_node {
        Node::NODE {
            type_: typ,
            attributes: attr,
            children,
        } => {
            let nm = nodename(typ);
            let typlbl = make_label(&List::from_iter([typ.clone()]))?;
            let new_attr = list_prepend(attr, Attribute::ATTR {
                name: "label".to_string(),
                value: typlbl,
            });
            let out = make_node(&nm, &new_attr);
            print!("{out}");
            dump_children(&nm, children)?;
            Ok(nm)
        }
        Node::LNODE {
            type_: typ,
            label_lst: lbl,
            attributes: attr,
            children,
        } => {
            let nm = nodename(typ);
            // Build lbl_1 = typ :: lbl (prepends typ at front of lbl)
            let lbl_1 = list_prepend(lbl, typ.clone());
            let lblstr = make_label(&lbl_1)?;
            let new_attr = list_prepend(attr, Attribute::ATTR {
                name: "label".to_string(),
                value: lblstr,
            });
            let out = make_node(&nm, &new_attr);
            print!("{out}");
            dump_children(&nm, children)?;
            Ok(nm)
        }
    }
}

/// Creates a label from a list of strings.
fn make_label(sl: &List<String>) -> Result<String> {
    let s0 = make_label_req(sl, "")?;
    Ok(format!("\"{s0}\""))
}

/// Helper function to make_label (uses matchcontinue).
fn make_label_req(in_string_lst: &List<String>, in_string: &str) -> Result<String> {
    // case {s}
    if let Some(s) = single_elem(in_string_lst) {
        let result = format!("{in_string}{s}");
        if let Ok(v) = match_ok(&result) {
            return Ok(v);
        }
    }
    // case {s1, s2}
    if let Some((s1, s2)) = two_elem(in_string_lst) {
        let mut s = in_string.to_string();
        s = format!("{s}{s1}");
        s = format!("{s}\\n");
        s = format!("{s}{s2}");
        if let Ok(v) = match_ok(&s) {
            return Ok(v);
        }
    }
    // case (s1 :: rest)
    if let Some((s1, rest)) = cons_elem(in_string_lst) {
        let mut s = in_string.to_string();
        s = format!("{s}{s1}");
        s = format!("{s}\\n");
        return make_label_req(&rest, &s);
    }
    bail!("matchfailed in make_label_req")
}

/// Helper function to dump_node. Uses matchcontinue.
fn dump_children(parent_ident: &str, in_children: &Children) -> Result<()> {
    // case (_,{}) - empty list
    if in_children.is_empty() {
        return Ok(());
    }
    // case (parent,(node :: rest))
    if in_children.len() >= 1 {
        let node = in_children.get(0).unwrap().clone();
        let rest: Children = List::from_iter(in_children.iter().skip(1).cloned());
        let nm = dump_node(&node)?;
        print_edge(&nm, parent_ident);
        return dump_children(parent_ident, &rest);
    }
    bail!("matchfailed in dump_children")
}

/// Creates a unique node name.
/// Changed use of str as part of nodename, since str may contain spaces.
fn nodename(_str: &str) -> String {
    let i = tick();
    let is = i.to_string();
    format!("GVNOD{is}")
}

/// Prints an edge between two nodes.
fn print_edge(n1: &str, n2: &str) {
    let str_ = make_edge(n1, n2);
    print!("{str_};\n");
}

/// Creates a string representing an edge between two nodes.
fn make_edge(n1: &str, n2: &str) -> String {
    let s = format!("{n1} -- ");
    format!("{s}{n2}")
}

/// Creates string from a node.
fn make_node(nm: &str, attr: &Attributes) -> String {
    let s = make_attr(attr);
    let s_1 = format!("{nm}{s}");
    format!("{s_1};")
}

/// Creates a string from an Attribute list.
fn make_attr(l: &Attributes) -> String {
    let res = make_attr_req(l, "").unwrap_or_default();
    format!("[{res}]")
}

/// Helper function to make_attr. Uses matchcontinue.
fn make_attr_req(in_attribute_lst: &Attributes, in_string: &str) -> Result<String> {
    // case {ATTR(name = name,value = v)}
    if let Some((name, value)) = single_attr_elem(in_attribute_lst) {
        let mut s = in_string.to_string();
        s = format!("{s}{name}");
        s = format!("{s}=");
        let result = format!("{s}{value}");
        if let Ok(v) = match_ok(&result) {
            return Ok(v);
        }
    }
    // case (ATTR(name = name,value = v) :: rest)
    if let Some((attr, rest)) = cons_attr_elem(in_attribute_lst) {
        let Attribute::ATTR { name, value } = attr;
        let mut s = in_string.to_string();
        s = format!("{s}{name}");
        s = format!("{s}=");
        s = format!("{s}{value}");
        s = format!("{s},");
        return make_attr_req(&rest, &s);
    }
    bail!("matchfailed in make_attr_req")
}
