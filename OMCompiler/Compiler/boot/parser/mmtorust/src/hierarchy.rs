#![allow(unused)]

use std::collections::HashMap;
use std::fmt;
use mmwinnow::Absyn;
use crate::MM;

// ── Types ─────────────────────────────────────────────────────────────────────

/// The resolved type of a named entity, populated during type-checking.
#[derive(Debug, Clone, Default)]
pub enum Ty {
    #[default]
    Unknown,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Unknown => f.write_str("?"),
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

/// Expand one import member into (local_name, node) pairs.
/// A group import like `import A.{B, C as D}` yields two entries.
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

// ── Display helpers ───────────────────────────────────────────────────────────

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
        let count = names.len();
        writeln!(f, "Hierarchy ({count} top-level classes):")?;
        for (i, (name, node)) in names.iter().enumerate() {
            let last = i + 1 == count;
            fmt_node(f, name, node, "", last)?;
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
            if matches!(c.body, MM::ClassDef::Derived { ref type_spec, .. } if true) {
                if let MM::ClassDef::Derived { type_spec, .. } = &c.body {
                    write!(f, " = {}", fmt_type_spec(type_spec))?;
                }
            }
        }
        NodeKind::Component(m) => write!(f, " : {}", fmt_type_spec(&m.type_spec))?,
        NodeKind::Import(m) => write!(f, "  // {}", fmt_import(m))?,
        NodeKind::EnumLiteral => write!(f, " [enum literal]")?,
    }
    writeln!(f, "  [{}]", node.ty)?;

    let child_prefix = format!("{}{}", prefix, if is_last { "   " } else { "│  " });

    let mut children: Vec<_> = node.children.iter().collect();
    children.sort_by_key(|(n, _)| n.as_str());
    let n_children = children.len();
    let n_extends = node.extends.len();
    let total = n_children + n_extends;

    for (i, (child_name, child_node)) in children.iter().enumerate() {
        let child_last = i + 1 == total;
        fmt_node(f, child_name, child_node, &child_prefix, child_last)?;
    }
    for (i, ext) in node.extends.iter().enumerate() {
        let ext_last = n_children + i + 1 == total;
        let conn = if ext_last { "└─ " } else { "├─ " };
        writeln!(f, "{child_prefix}{conn}extends {}", fmt_path(&ext.path))?;
    }
    Ok(())
}
