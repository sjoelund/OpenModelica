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
    /// A polymorphic type parameter declared with `replaceable type T subtypeof Any`.
    TypeVar(String),
    /// `Option<T>`
    Option(Box<Ty>),
    /// Modelica `list<T>` → Rust `Vec<T>`
    List(Box<Ty>),
    /// Modelica `array<T>`
    Array(Box<Ty>),
    /// Multiple values (multiple output components of a function).
    Tuple(Vec<Ty>),
    /// No output.
    Unit,
    /// A resolved function type.
    Function {
        type_vars: Vec<String>,
        inputs: Vec<Ty>,
        output: Box<Ty>,
    },
    /// A metarecord with fields — maps to a Rust struct. Carries its qualified name.
    RustStruct(String),
    /// A metarecord with no fields — maps to a unit enum variant, not a struct.
    RustUnitVariant,
    /// A uniontype with ≥2 records — maps to a Rust enum. Carries its qualified name.
    RustEnum(String),
    /// A single-record uniontype — transparent alias to the sole record.
    /// Carries the simple name of that record.
    AliasTo(String),
    /// A user-defined parameterized type with resolved type arguments, e.g. `ExpandableArray<T>`.
    Generic(String, Vec<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Unknown => f.write_str("?"),
            Ty::I32 => f.write_str("i32"),
            Ty::F64 => f.write_str("f64"),
            Ty::Bool => f.write_str("bool"),
            Ty::Str => f.write_str("String"),
            Ty::Enumeration(name) => f.write_str(&name.replace('.', "::")),
            Ty::TypeVar(name) => write!(f, "{name}"),
            Ty::Option(inner) => write!(f, "Option<{inner}>"),
            Ty::List(inner) => write!(f, "List<{inner}>"),
            Ty::Array(inner) => write!(f, "Array<{inner}>"),
            Ty::Unit => f.write_str("()"),
            Ty::Tuple(tys) => {
                f.write_str("(")?;
                for (i, ty) in tys.iter().enumerate() {
                    if i > 0 { f.write_str(", ")?; }
                    write!(f, "{ty}")?;
                }
                f.write_str(")")
            }
            Ty::Function { type_vars, inputs, output } => {
                if !type_vars.is_empty() {
                    write!(f, "<{}>", type_vars.join(", "))?;
                }
                f.write_str("fn(")?;
                for (i, ty) in inputs.iter().enumerate() {
                    if i > 0 { f.write_str(", ")?; }
                    write!(f, "{ty}")?;
                }
                write!(f, ") -> {output}")
            }
            Ty::RustStruct(name) => f.write_str(&name.replace('.', "::")),
            Ty::RustUnitVariant => f.write_str("unit variant"),
            Ty::RustEnum(name) => f.write_str(&name.replace('.', "::")),
            Ty::AliasTo(name) => write!(f, "= {}", name.replace('.', "::")),
            Ty::Generic(name, args) => {
                write!(f, "{name}<")?;
                for (i, ty) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{ty}")?;
                }
                write!(f, ">")
            }
        }
    }
}

// ── NodeKind ──────────────────────────────────────────────────────────────────

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

#[derive(Debug)]
pub struct NameNode<'a> {
    pub kind: NodeKind<'a>,
    pub ty: Ty,
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

// ── Type-variable helpers ─────────────────────────────────────────────────────

fn type_spec_path(ts: &Absyn::TypeSpec) -> &Absyn::Path {
    match ts {
        Absyn::TypeSpec::TPATH { path, .. } => path,
        Absyn::TypeSpec::TCOMPLEX { path, .. } => path,
    }
}

/// `replaceable type T subtypeof Any` — the defining pattern for a type variable.
fn is_subtype_of_any(class: &MM::Class) -> bool {
    if !matches!(class.restriction, Absyn::Restriction::R_TYPE) {
        return false;
    }
    match &class.body {
        MM::ClassDef::Derived { type_spec, .. } => path_last(type_spec_path(type_spec)) == "Any",
        _ => false,
    }
}

/// Collect all type-variable names declared in a class:
/// from `<T, U>` (typeVars list) and from `replaceable type T subtypeof Any` members.
fn class_type_vars(c: &MM::Class) -> Vec<String> {
    match &c.body {
        MM::ClassDef::Parts { type_vars, members, .. } => {
            let mut vars: Vec<String> = type_vars.clone();
            for m in members {
                if let MM::ClassMember::ClassDef(cdm) = m {
                    if cdm.replaceable && is_subtype_of_any(&cdm.class_def) {
                        vars.push(cdm.class_def.name.clone());
                    }
                }
            }
            vars
        }
        _ => vec![],
    }
}

fn is_function_class(r: &Absyn::Restriction) -> bool {
    matches!(r, Absyn::Restriction::R_FUNCTION { .. })
}

/// Returns the simple names of all direct record children of a uniontype node.
/// The mmwinnow parser assigns `R_RECORD` (not `R_METARECORD`) to every `record`
/// declaration, so we identify records by restriction and by being a direct class child.
fn record_child_names(node: &NameNode<'_>) -> Vec<String> {
    node.children
        .iter()
        .filter_map(|(name, child)| {
            if let NodeKind::Class(c) = &child.kind {
                if matches!(c.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. }) {
                    return Some(name.clone());
                }
            }
            None
        })
        .collect()
}

fn has_component_children(node: &NameNode<'_>) -> bool {
    node.children.values().any(|c| matches!(c.kind, NodeKind::Component(_)))
}

// ── Type resolution ───────────────────────────────────────────────────────────

pub fn resolve_pass(hier: &mut InstanceHierarchy<'_>) -> bool {
    let mut changed = false;
    seed_enumerations(&mut hier.top_level, "", &mut changed);
    seed_metarecords(&mut hier.top_level, "", &mut changed);
    seed_type_vars(&mut hier.top_level, &mut changed);
    let mut known: HashMap<String, Ty> = HashMap::new();
    seed_builtins(&mut known);
    collect_known(&hier.top_level, "", &mut known);
    resolve_nodes(&mut hier.top_level, "", &known, &mut changed);
    changed
}

/// Pre-populate the known map with MM builtin types that are defined in the runtime
/// rather than in any source file and therefore never appear in the hierarchy.
fn seed_builtins(known: &mut HashMap<String, Ty>) {
    // SourceInfo — builtin single-record uniontype; fields are all primitives.
    //   record SOURCEINFO
    //     String fileName; Boolean isReadOnly;
    //     Integer lineNumberStart, columnNumberStart, lineNumberEnd, columnNumberEnd;
    //     Real lastModification;
    //   end SOURCEINFO;
    known.entry("SOURCEINFO".into()).or_insert(Ty::RustStruct("SOURCEINFO".into()));
    known.entry("SourceInfo".into()).or_insert(Ty::AliasTo("SourceInfo".into()));
}

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

/// Seed the record children of every R_UNIONTYPE node.
/// Records with fields → RustStruct; records with no fields → RustUnitVariant.
/// We seed by context (record inside a uniontype) rather than by restriction because
/// the mmwinnow parser assigns R_RECORD (not R_METARECORD) to all record declarations.
fn seed_metarecords(nodes: &mut HashMap<String, NameNode<'_>>, prefix: &str, changed: &mut bool) {
    for (name, node) in nodes.iter_mut() {
        let qname = qualify(prefix, name);
        if let NodeKind::Class(c) = &node.kind {
            if matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) {
                let rec_names: Vec<String> = record_child_names(node);
                for rec_name in rec_names {
                    let rec_qname = qualify(&qname, &rec_name);
                    let child = node.children.get_mut(&rec_name).unwrap();
                    if child.ty == Ty::Unknown {
                        child.ty = if has_component_children(child) {
                            Ty::RustStruct(rec_qname)
                        } else {
                            Ty::RustUnitVariant
                        };
                        *changed = true;
                    }
                }
            }
        }
        seed_metarecords(&mut node.children, &qname, changed);
    }
}

/// Mark `replaceable type T subtypeof Any` class members as `Ty::TypeVar`.
fn seed_type_vars(nodes: &mut HashMap<String, NameNode<'_>>, changed: &mut bool) {
    for node in nodes.values_mut() {
        if let NodeKind::Class(c) = &node.kind {
            let vars = class_type_vars(c);
            for var_name in vars {
                if let Some(child) = node.children.get_mut(&var_name) {
                    if child.ty == Ty::Unknown {
                        child.ty = Ty::TypeVar(var_name.clone());
                        *changed = true;
                    }
                }
            }
        }
        seed_type_vars(&mut node.children, changed);
    }
}

/// Snapshot all resolved types into a lookup map.
/// TypeVars are intentionally excluded — they are local to their enclosing class
/// and must not resolve names in sibling scopes.
fn collect_known(nodes: &HashMap<String, NameNode<'_>>, prefix: &str, known: &mut HashMap<String, Ty>) {
    for (name, node) in nodes {
        let qname = qualify(prefix, name);
        if node.ty != Ty::Unknown && !matches!(node.ty, Ty::TypeVar(_)) {
            known.insert(qname.clone(), node.ty.clone());
            known.entry(name.clone()).or_insert_with(|| node.ty.clone());
        }
        collect_known(&node.children, &qname, known);
    }
}

fn resolve_nodes(nodes: &mut HashMap<String, NameNode<'_>>, prefix: &str, known: &HashMap<String, Ty>, changed: &mut bool) {
    for (name, node) in nodes.iter_mut() {
        let qname = qualify(prefix, name);
        if node.ty == Ty::Unknown {
            if let Some(ty) = try_resolve(node, &qname, known) {
                node.ty = ty;
                *changed = true;
            }
        }
        resolve_nodes(&mut node.children, &qname, known, changed);
    }
}

fn try_resolve(node: &NameNode<'_>, qname: &str, known: &HashMap<String, Ty>) -> Option<Ty> {
    match &node.kind {
        NodeKind::Class(c) if is_function_class(&c.restriction) => {
            try_resolve_function(c, node, known)
        }
        NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) => {
            try_resolve_uniontype(node, qname)
        }
        NodeKind::Class(c) => match &c.body {
            MM::ClassDef::Derived { type_spec, .. } => resolve_type_spec(type_spec, known, &[]),
            _ => None,
        },
        NodeKind::Component(m) => resolve_type_spec(&m.type_spec, known, &[]),
        NodeKind::Import(_) | NodeKind::EnumLiteral => None,
    }
}

fn try_resolve_uniontype(node: &NameNode<'_>, qname: &str) -> Option<Ty> {
    let mut record_names: Vec<String> = record_child_names(node);

    // Defer if any record child is still Unknown (seeding hasn't run yet).
    for name in &record_names {
        if node.children.get(name).map_or(true, |c| c.ty == Ty::Unknown) {
            return None;
        }
    }

    record_names.sort(); // deterministic order for AliasTo
    match record_names.len() {
        0 => None,
        1 => Some(Ty::AliasTo(qname.to_owned())),
        _ => Some(Ty::RustEnum(qname.to_owned())),
    }
}

fn try_resolve_function(c: &MM::Class, node: &NameNode<'_>, known: &HashMap<String, Ty>) -> Option<Ty> {
    resolve_function_type(c, node, known, &[])
}

/// Resolve a function's type, threading `outer_type_vars` into nested partial functions.
fn resolve_function_type(
    c: &MM::Class,
    node: &NameNode<'_>,
    known: &HashMap<String, Ty>,
    outer_type_vars: &[String],
) -> Option<Ty> {
    let mut type_vars = class_type_vars(c);
    for v in outer_type_vars {
        if !type_vars.contains(v) { type_vars.push(v.clone()); }
    }

    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } => members,
        MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return None,
    };

    // Resolve nested partial function children with the combined type vars so they
    // can reference type variables declared in the outer function.
    let mut local_fns: HashMap<String, Ty> = HashMap::new();
    for (child_name, child_node) in &node.children {
        if let NodeKind::Class(fn_class) = &child_node.kind {
            if is_function_class(&fn_class.restriction) {
                if let Some(fn_ty) = resolve_function_type(fn_class, child_node, known, &type_vars) {
                    local_fns.insert(child_name.clone(), fn_ty);
                }
            }
        }
    }

    let mut inputs: Vec<Ty> = Vec::new();
    let mut outputs: Vec<Ty> = Vec::new();

    for member in members {
        let MM::ClassMember::Component(m) = member else { continue };
        let child = node.children.get(&m.name)?;
        let ty = if child.ty != Ty::Unknown {
            child.ty.clone()
        } else {
            // Check local partial functions first (higher-order function args).
            let type_name = path_last(type_spec_path(&m.type_spec));
            if let Some(fn_ty) = local_fns.get(type_name).cloned() {
                fn_ty
            } else {
                resolve_type_spec(&m.type_spec, known, &type_vars)?
            }
        };
        match m.direction {
            Absyn::Direction::INPUT => inputs.push(ty),
            Absyn::Direction::OUTPUT => outputs.push(ty),
            Absyn::Direction::INPUT_OUTPUT => { inputs.push(ty.clone()); outputs.push(ty); }
            _ => {}
        }
    }

    let output = match outputs.len() {
        0 => Ty::Unit,
        1 => outputs.into_iter().next().unwrap(),
        _ => Ty::Tuple(outputs),
    };
    // Only report the type vars that belong to this function (not inherited outer ones).
    let own_type_vars = class_type_vars(c);
    Some(Ty::Function { type_vars: own_type_vars, inputs, output: Box::new(output) })
}

/// Resolve a TypeSpec to a Ty.
/// `type_vars` is the list of type-variable names in scope; they resolve to `Ty::TypeVar`.
fn resolve_type_spec(ts: &Absyn::TypeSpec, known: &HashMap<String, Ty>, type_vars: &[String]) -> Option<Ty> {
    match ts {
        Absyn::TypeSpec::TPATH { path, .. } => resolve_path(path, known, type_vars),
        Absyn::TypeSpec::TCOMPLEX { path, typeSpecs, .. } => {
            let args: Vec<_> = typeSpecs.into_iter().collect();
            let ctor = path_last(path);
            match ctor {
                "tuple" => {
                    let tys: Option<Vec<Ty>> = args.iter()
                        .map(|a| resolve_type_spec(a, known, type_vars))
                        .collect();
                    Some(Ty::Tuple(tys?))
                }
                "Option" if args.len() == 1 => {
                    Some(Ty::Option(Box::new(resolve_type_spec(&args[0], known, type_vars)?)))
                }
                "list" | "List" if args.len() == 1 => {
                    Some(Ty::List(Box::new(resolve_type_spec(&args[0], known, type_vars)?)))
                }
                "array" | "Array" if args.len() == 1 => {
                    Some(Ty::Array(Box::new(resolve_type_spec(&args[0], known, type_vars)?)))
                }
                "Mutable" if args.len() == 1 => {
                    let inner = resolve_type_spec(&args[0], known, type_vars)?;
                    Some(Ty::Generic("Mutable".to_owned(), vec![inner]))
                }
                _ => {
                    // User-defined generic: base type must be known, all args must resolve.
                    let full = fmt_path(path);
                    let lookup = full.trim_start_matches('.');
                    let base_ty = known.get(lookup).or_else(|| known.get(ctor))?;
                    let base_name = ty_rust_name(base_ty).unwrap_or_else(|| ctor.to_owned());
                    let resolved: Option<Vec<Ty>> = args.iter()
                        .map(|a| resolve_type_spec(a, known, type_vars))
                        .collect();
                    Some(Ty::Generic(base_name, resolved?))
                }
            }
        }
    }
}

fn resolve_path(path: &Absyn::Path, known: &HashMap<String, Ty>, type_vars: &[String]) -> Option<Ty> {
    let last = path_last(path);
    match last {
        "Integer" => return Some(Ty::I32),
        "Real" => return Some(Ty::F64),
        "Boolean" => return Some(Ty::Bool),
        "String" => return Some(Ty::Str),
        name if type_vars.iter().any(|v| v == name) => return Some(Ty::TypeVar(name.to_owned())),
        _ => {}
    }
    let qname = fmt_path(path);
    let qname = qname.trim_start_matches('.');
    known.get(qname).cloned()
}

// ── Display helpers ───────────────────────────────────────────────────────────

/// Extract the Rust-style name from a resolved type, for use as a generic base.
fn ty_rust_name(ty: &Ty) -> Option<String> {
    match ty {
        Ty::AliasTo(n) | Ty::RustEnum(n) | Ty::RustStruct(n) | Ty::Enumeration(n) => Some(n.replace('.', "::")),
        _ => None,
    }
}

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

/// Write `{ field1: Type1, field2: Type2 }` for a struct node, in declaration order.
fn fmt_struct_fields(
    f: &mut fmt::Formatter<'_>,
    c: &MM::Class,
    children: &HashMap<String, NameNode<'_>>,
) -> fmt::Result {
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } => members,
        MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return write!(f, "{{}}"),
    };
    write!(f, "{{ ")?;
    let mut first = true;
    for member in members {
        if let MM::ClassMember::Component(m) = member {
            if !first { write!(f, ", ")?; }
            let ty = children.get(&m.name).map(|n| &n.ty).unwrap_or(&Ty::Unknown);
            write!(f, "{}: {ty}", m.name)?;
            first = false;
        }
    }
    write!(f, " }}")
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

    let has_no_type = matches!(&node.kind, NodeKind::Import(_))
        || matches!(&node.kind, NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_PACKAGE));

    match &node.ty {
        Ty::Unknown if has_no_type => writeln!(f)?,
        Ty::Unknown => writeln!(f, "  [?]")?,
        Ty::RustStruct(_) => {
            // Show fields inline in declaration order; fall back to the name if no body.
            if let NodeKind::Class(c) = &node.kind {
                write!(f, "  ")?;
                fmt_struct_fields(f, c, &node.children)?;
                writeln!(f)?;
            } else {
                writeln!(f, "  [{}]", node.ty)?;
            }
        }
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
