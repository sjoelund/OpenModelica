#![allow(unused)]

use std::collections::HashMap;
use std::fmt;
use mmwinnow::Absyn;
use crate::MM;

// ── Ty ───────────────────────────────────────────────────────────────────────

/// The resolved type of a named entity, populated during type-checking passes.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Ty {
    #[default]
    Unknown,
    /// Modelica `Integer` → Rust `i32`
    I32,
    /// Modelica `Real` → Rust `f64`
    F64,
    /// Modelica `Boolean` → Rust `bool`
    Bool,
    /// Modelica `String` → Rust `String`
    Str,
    /// An enumeration type; carries its qualified class name.
    Enumeration(String),
    /// `Option<T>` — already present in both Modelica and Rust.
    Option(Box<Ty>),
    /// Modelica `list<T>` → Rust `Vec<T>`
    List(Box<Ty>),
    /// Modelica `array<T>`
    Array(Box<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Unknown => f.write_str("?"),
            Ty::I32 => f.write_str("i32"),
            Ty::F64 => f.write_str("f64"),
            Ty::Bool => f.write_str("bool"),
            Ty::Str => f.write_str("String"),
            Ty::Enumeration(name) => write!(f, "{name}"),
            Ty::Option(inner) => write!(f, "Option<{inner}>"),
            Ty::List(inner) => write!(f, "Vec<{inner}>"),
            Ty::Array(inner) => write!(f, "Array<{inner}>"),
        }
    }
}

// ── NodeKind ──────────────────────────────────────────────────────────────────

/// What kind of AST node a name refers to.
#[derive(Debug)]
pub enum NodeKind<'a> {
    Class(&'a MM::Class),
    Component(&'a MM::ComponentMember),
    /// A single import statement; the map key is the locally introduced name.
    Import(&'a MM::ImportMember),
    /// One literal inside an `enumeration(...)` body.
    EnumLiteral,
}

// ── NameNode ──────────────────────────────────────────────────────────────────

/// A single named entry in the instance hierarchy.
#[derive(Debug)]
pub struct NameNode<'a> {
    pub kind: NodeKind<'a>,
    pub ty: Ty,
    /// Named members: nested classes, components, imports, enum literals.
    pub children: HashMap<String, NameNode<'a>>,
    /// Extends clauses — no local name, but must be followed during lookup.
    pub extends: Vec<&'a MM::ExtendsMember>,
}

impl<'a> NameNode<'a> {
    fn new(kind: NodeKind<'a>) -> Self {
        Self { kind, ty: Ty::default(), children: HashMap::new(), extends: Vec::new() }
    }
}

// ── InstanceHierarchy ─────────────────────────────────────────────────────────

/// The full name hierarchy for a parsed MM program.
#[derive(Debug)]
pub struct InstanceHierarchy<'a> {
    pub top_level: HashMap<String, NameNode<'a>>,
}

impl<'a> InstanceHierarchy<'a> {
    pub fn from_program(program: &'a MM::Program) -> Self {
        let top_level = program
            .iter()
            .map(|class| (class.name.clone(), build_class_node(class)))
            .collect();
        Self { top_level }
    }
}

// ── Building ──────────────────────────────────────────────────────────────────

fn build_class_node(class: &MM::Class) -> NameNode<'_> {
    let mut node = NameNode::new(NodeKind::Class(class));
    populate_from_class_def(&class.body, &mut node);
    node
}

fn populate_from_class_def<'a>(def: &'a MM::ClassDef, node: &mut NameNode<'a>) {
    let members: &[MM::ClassMember] = match def {
        MM::ClassDef::Parts { members, .. } => members,
        MM::ClassDef::ClassExtends { members, .. } => members,
        MM::ClassDef::Enumeration { enum_literals, .. } => {
            if let Absyn::EnumDef::ENUMLITERALS { enumLiterals } = enum_literals {
                for lit in enumLiterals {
                    let Absyn::EnumLiteral::ENUMLITERAL { literal, .. } = lit;
                    node.children.insert(literal.clone(), NameNode::new(NodeKind::EnumLiteral));
                }
            }
            return;
        }
        MM::ClassDef::Derived { .. } => return,
    };

    for member in members {
        match member {
            MM::ClassMember::ClassDef(m) => {
                node.children.insert(m.class_def.name.clone(), build_class_node(&m.class_def));
            }
            MM::ClassMember::Component(m) => {
                node.children.insert(m.name.clone(), NameNode::new(NodeKind::Component(m)));
            }
            MM::ClassMember::Import(m) => {
                for (local_name, child_node) in import_nodes(m) {
                    node.children.insert(local_name, child_node);
                }
            }
            MM::ClassMember::Extends(m) => {
                node.extends.push(m);
            }
            MM::ClassMember::LexerComment(_) => {}
        }
    }
}

fn import_nodes(m: &MM::ImportMember) -> Vec<(String, NameNode<'_>)> {
    let node = || NameNode::new(NodeKind::Import(m));
    match &m.import {
        Absyn::Import::NAMED_IMPORT { name, .. } => vec![(name.clone(), node())],
        Absyn::Import::QUAL_IMPORT { path } => vec![(path_last(path).to_owned(), node())],
        Absyn::Import::UNQUAL_IMPORT { .. } => vec![("*".to_owned(), node())],
        Absyn::Import::GROUP_IMPORT { groups, .. } => groups
            .into_iter()
            .map(|g| {
                let local = match g {
                    Absyn::GroupImport::GROUP_IMPORT_NAME { name } => name.clone(),
                    Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, .. } => rename.clone(),
                };
                (local, node())
            })
            .collect(),
    }
}

// ── Type resolution ───────────────────────────────────────────────────────────

/// Run one resolution pass over the hierarchy.
/// Returns `true` if any node's type was updated; call in a loop until `false`.
pub fn resolve_pass(hier: &mut InstanceHierarchy<'_>) -> bool {
    let mut changed = false;
    // Seed enumeration types first (they are always known regardless of other passes).
    seed_enumerations(&mut hier.top_level, "", &mut changed);
    // Snapshot all currently resolved types for use as a lookup table.
    let mut known: HashMap<String, Ty> = HashMap::new();
    collect_known(&hier.top_level, "", &mut known);
    // Resolve Unknown nodes that can now be determined.
    resolve_nodes(&mut hier.top_level, &known, &mut changed);
    changed
}

/// Set `Ty::Enumeration` on every enumeration class and its literals.
/// Uses the qualified name so that `Ty::Enumeration("Absyn.Restriction")` is accurate.
fn seed_enumerations(nodes: &mut HashMap<String, NameNode<'_>>, prefix: &str, changed: &mut bool) {
    for (name, node) in nodes.iter_mut() {
        let qname = qualify(prefix, name);
        if node.ty == Ty::Unknown {
            if let NodeKind::Class(c) = &node.kind {
                let is_enum = matches!(c.restriction, Absyn::Restriction::R_ENUMERATION)
                    || (matches!(c.restriction, Absyn::Restriction::R_TYPE)
                        && matches!(c.body, MM::ClassDef::Enumeration { .. }));
                if is_enum {
                    let ty = Ty::Enumeration(qname.clone());
                    for child in node.children.values_mut() {
                        if matches!(child.kind, NodeKind::EnumLiteral) && child.ty == Ty::Unknown {
                            child.ty = ty.clone();
                            *changed = true;
                        }
                    }
                    node.ty = ty;
                    *changed = true;
                }
            }
        }
        seed_enumerations(&mut node.children, &qname, changed);
    }
}

/// Walk the tree and record every resolved type under both its qualified name
/// (`Absyn.Restriction`) and its simple name (`Restriction`, first occurrence wins).
fn collect_known(nodes: &HashMap<String, NameNode<'_>>, prefix: &str, known: &mut HashMap<String, Ty>) {
    for (name, node) in nodes {
        let qname = qualify(prefix, name);
        if node.ty != Ty::Unknown {
            known.insert(qname.clone(), node.ty.clone());
            known.entry(name.clone()).or_insert_with(|| node.ty.clone());
        }
        collect_known(&node.children, &qname, known);
    }
}

/// Walk the tree mutably and resolve any Unknown node whose type can now be inferred.
fn resolve_nodes(nodes: &mut HashMap<String, NameNode<'_>>, known: &HashMap<String, Ty>, changed: &mut bool) {
    for node in nodes.values_mut() {
        if node.ty == Ty::Unknown {
            if let Some(ty) = try_resolve(node, known) {
                node.ty = ty;
                *changed = true;
            }
        }
        resolve_nodes(&mut node.children, known, changed);
    }
}

fn try_resolve(node: &NameNode<'_>, known: &HashMap<String, Ty>) -> Option<Ty> {
    match &node.kind {
        NodeKind::Class(c) => match &c.body {
            MM::ClassDef::Derived { type_spec, .. } => resolve_type_spec(type_spec, known),
            _ => None,
        },
        NodeKind::Component(m) => resolve_type_spec(&m.type_spec, known),
        NodeKind::Import(_) | NodeKind::EnumLiteral => None,
    }
}

fn resolve_type_spec(ts: &Absyn::TypeSpec, known: &HashMap<String, Ty>) -> Option<Ty> {
    match ts {
        Absyn::TypeSpec::TPATH { path, .. } => resolve_path(path, known),
        Absyn::TypeSpec::TCOMPLEX { path, typeSpecs, .. } => {
            let args: Vec<_> = typeSpecs.into_iter().collect();
            if args.len() != 1 {
                return None;
            }
            let inner = resolve_type_spec(&args[0], known)?;
            match path_last(path) {
                "Option" => Some(Ty::Option(Box::new(inner))),
                "list" => Some(Ty::List(Box::new(inner))),
                "array" => Some(Ty::Array(Box::new(inner))),
                _ => None,
            }
        }
    }
}

fn resolve_path(path: &Absyn::Path, known: &HashMap<String, Ty>) -> Option<Ty> {
    match path_last(path) {
        "Integer" => return Some(Ty::I32),
        "Real" => return Some(Ty::F64),
        "Boolean" => return Some(Ty::Bool),
        "String" => return Some(Ty::Str),
        _ => {}
    }
    let qname = fmt_path(path);
    let qname = qname.trim_start_matches('.');
    known.get(qname).cloned()
}

// ── Display helpers ───────────────────────────────────────────────────────────

fn qualify(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.to_owned() } else { format!("{prefix}.{name}") }
}

fn path_last(path: &Absyn::Path) -> &str {
    match path {
        Absyn::Path::IDENT { name } => name,
        Absyn::Path::QUALIFIED { path, .. } => path_last(path),
        Absyn::Path::FULLYQUALIFIED { path } => path_last(path),
    }
}

fn fmt_path(path: &Absyn::Path) -> String {
    match path {
        Absyn::Path::IDENT { name } => name.clone(),
        Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", fmt_path(path)),
        Absyn::Path::FULLYQUALIFIED { path } => format!(".{}", fmt_path(path)),
    }
}

fn fmt_type_spec(ts: &Absyn::TypeSpec) -> String {
    match ts {
        Absyn::TypeSpec::TPATH { path, .. } => fmt_path(path),
        Absyn::TypeSpec::TCOMPLEX { path, typeSpecs, .. } => {
            let args: Vec<_> = typeSpecs.into_iter().map(|t| fmt_type_spec(&t)).collect();
            format!("{}<{}>", fmt_path(path), args.join(", "))
        }
    }
}

fn fmt_restriction(r: &Absyn::Restriction) -> &'static str {
    use Absyn::Restriction::*;
    match r {
        R_CLASS => "class",
        R_OPTIMIZATION => "optimization",
        R_MODEL => "model",
        R_RECORD => "record",
        R_BLOCK => "block",
        R_CONNECTOR => "connector",
        R_EXP_CONNECTOR => "expandable connector",
        R_TYPE => "type",
        R_PACKAGE => "package",
        R_FUNCTION { .. } => "function",
        R_OPERATOR => "operator",
        R_OPERATOR_RECORD => "operator record",
        R_ENUMERATION => "enumeration",
        R_UNIONTYPE => "uniontype",
        R_METARECORD { .. } => "metarecord",
        _ => "class",
    }
}

fn fmt_import(m: &MM::ImportMember) -> String {
    match &m.import {
        Absyn::Import::NAMED_IMPORT { name, path } => format!("import {} = {}", name, fmt_path(path)),
        Absyn::Import::QUAL_IMPORT { path } => format!("import {}", fmt_path(path)),
        Absyn::Import::UNQUAL_IMPORT { path } => format!("import {}.*", fmt_path(path)),
        Absyn::Import::GROUP_IMPORT { prefix, groups } => {
            let names: Vec<_> = groups.into_iter().map(|g| match g {
                Absyn::GroupImport::GROUP_IMPORT_NAME { name } => name.clone(),
                Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, name } => format!("{name} as {rename}"),
            }).collect();
            format!("import {}.{{{}}}", fmt_path(prefix), names.join(", "))
        }
    }
}

// ── Pretty-printing ───────────────────────────────────────────────────────────

impl fmt::Display for InstanceHierarchy<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut names: Vec<_> = self.top_level.iter().collect();
        names.sort_by_key(|(n, _)| n.as_str());
        writeln!(f, "Hierarchy ({} top-level classes):", names.len())?;
        for (i, (name, node)) in names.iter().enumerate() {
            fmt_node(f, name, node, "", i + 1 == names.len())?;
        }
        Ok(())
    }
}

fn fmt_node(
    f: &mut fmt::Formatter<'_>,
    name: &str,
    node: &NameNode<'_>,
    prefix: &str,
    is_last: bool,
) -> fmt::Result {
    let connector = if is_last { "└─ " } else { "├─ " };
    write!(f, "{prefix}{connector}{name}")?;

    match &node.kind {
        NodeKind::Class(c) => {
            write!(f, " [{}]", fmt_restriction(&c.restriction))?;
            if let MM::ClassDef::Derived { type_spec, .. } = &c.body {
                write!(f, " = {}", fmt_type_spec(type_spec))?;
            }
        }
        NodeKind::Component(m) => write!(f, " : {}", fmt_type_spec(&m.type_spec))?,
        NodeKind::Import(m) => write!(f, "  // {}", fmt_import(m))?,
        NodeKind::EnumLiteral => {}
    }

    // Append the resolved type when known; always show [?] when not.
    match &node.ty {
        Ty::Unknown => writeln!(f, "  [?]")?,
        ty => writeln!(f, "  [{ty}]")?,
    }

    let child_prefix = format!("{}{}", prefix, if is_last { "   " } else { "│  " });
    let mut children: Vec<_> = node.children.iter().collect();
    children.sort_by_key(|(n, _)| n.as_str());
    let total = children.len() + node.extends.len();

    for (i, (child_name, child_node)) in children.iter().enumerate() {
        fmt_node(f, child_name, child_node, &child_prefix, i + 1 == total)?;
    }
    for (i, ext) in node.extends.iter().enumerate() {
        let ext_last = children.len() + i + 1 == total;
        let conn = if ext_last { "└─ " } else { "├─ " };
        writeln!(f, "{child_prefix}{conn}extends {}", fmt_path(&ext.path))?;
    }
    Ok(())
}
