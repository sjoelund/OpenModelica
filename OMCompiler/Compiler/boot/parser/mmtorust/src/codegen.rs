#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use mmwinnow::Absyn;
use crate::MM;
use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind, Ty};

// ── Import-aware generation context ──────────────────────────────────────────

struct GenCtx {
    /// Name of the top-level class being generated (e.g. "Absyn").
    top_name: String,
    /// Modules imported with `.*`; their types are referenced by bare name.
    unqual_modules: HashSet<String>,
    /// Explicit imports: dotted qualified name → local name.
    named: HashMap<String, String>,
    /// Uniontypes (Rust enums) whose variants are referenced via UnionTypeVariant.
    /// Their qualified names need to be imported so the generated code can use `UnionType::Variant`.
    uniontype_imports: HashSet<String>,
}

impl GenCtx {
    fn new(top_name: &str) -> Self {
        Self {
            top_name: top_name.to_owned(),
            unqual_modules: HashSet::new(),
            named: HashMap::new(),
            uniontype_imports: HashSet::new(),
        }
    }

    /// Shorten a dot-separated qualified name to the shortest valid reference
    /// for this file, based on the collected imports.
    fn shorten(&self, dotted: &str) -> String {
        // Same-module: strip the prefix, keep only the last segment (all types
        // in one package are emitted flat in the same file).
        if let Some(rest) = dotted.strip_prefix(&format!("{}.", self.top_name)) {
            return rest.rsplit('.').next().unwrap_or(rest).to_owned();
        }
        // Named / qualified import.
        if let Some(local) = self.named.get(dotted) {
            return local.clone();
        }
        // Wildcard import: if a module prefix matches, keep only the last segment.
        for module in &self.unqual_modules {
            if let Some(rest) = dotted.strip_prefix(&format!("{module}.")) {
                return rest.rsplit('.').next().unwrap_or(rest).to_owned();
            }
        }
        // Fully-qualified Rust path.
        dotted.replace('.', "::")
    }

    /// Sorted `use` lines to emit at the top of the file.
    fn use_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for module in &self.unqual_modules {
            let rust = module_rust_prefix(module);
            lines.push(format!("use {rust}::*;"));
        }
        for (dotted, local) in &self.named {
            let rust = dotted_to_rust_path(dotted);
            let last = dotted.rsplit('.').next().unwrap_or(dotted);
            if local == last {
                lines.push(format!("use {rust};"));
            } else {
                lines.push(format!("use {rust} as {local};"));
            }
        }
        // Import uniontypes that are referenced via UnionTypeVariant syntax.
        for uniontype_qname in &self.uniontype_imports {
            let rust = dotted_to_rust_path(uniontype_qname);
            let last = uniontype_qname.rsplit('.').next().unwrap_or(uniontype_qname);
            lines.push(format!("use {rust};"));
        }
        lines.sort();
        lines.dedup();
        lines
    }
}

/// Walk the subtree collecting file-level import nodes into `ctx`.
/// Stops at function boundaries — imports inside a function body are local to that function.
fn collect_imports(node: &NameNode<'_>, ctx: &mut GenCtx) {
    for child in node.children.values() {
        match &child.kind {
            NodeKind::Import(m) => match &m.import {
                Absyn::Import::UNQUAL_IMPORT { path } => {
                    ctx.unqual_modules.insert(path_to_dotted(path));
                }
                Absyn::Import::QUAL_IMPORT { path } => {
                    let dotted = path_to_dotted(path);
                    let last = dotted.rsplit('.').next().unwrap_or(&dotted).to_owned();
                    ctx.named.insert(dotted, last);
                }
                Absyn::Import::NAMED_IMPORT { name, path } => {
                    ctx.named.insert(path_to_dotted(path), name.clone());
                }
                Absyn::Import::GROUP_IMPORT { prefix, groups } => {
                    let prefix_str = path_to_dotted(prefix);
                    for g in groups.into_iter() {
                        let (local, orig) = match g {
                            Absyn::GroupImport::GROUP_IMPORT_NAME { name } => (name.clone(), name.clone()),
                            Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, name } => (rename.clone(), name.clone()),
                        };
                        ctx.named.insert(format!("{prefix_str}.{orig}"), local);
                    }
                }
            },
            NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) => {
                // Don't recurse into functions — their imports are local to that function.
            }
            _ => collect_imports(child, ctx),
        }
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn generate_all(hier: &InstanceHierarchy<'_>, output_dir: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(output_dir)?;
    for (name, node) in &hier.top_level {
        let content = generate_file(name, node);
        std::fs::write(format!("{output_dir}/{name}.rs"), content)?;
    }
    let main_content = generate_main_file(hier);
    std::fs::write(format!("{output_dir}/main.rs"), main_content)?;
    Ok(())
}

fn generate_main_file(hier: &InstanceHierarchy<'_>) -> String {
    let mut out = String::new();
    writeln!(out, "// Auto-generated main file").unwrap();
    for name in hier.top_level.keys() {
        match hier.top_level[name].kind {
            NodeKind::Class(MM::Class{restriction: Absyn::Restriction::R_PACKAGE, ..}) |
            NodeKind::Class(MM::Class{restriction: Absyn::Restriction::R_UNIONTYPE, ..}) => {
                writeln!(out, "mod {name};").unwrap();
            },
            _ => continue,
        }
    }
    writeln!(out).unwrap();
    out = out + r#"fn main() {
    Main.main()
}"#;
    out
}

fn generate_file(top_name: &str, node: &NameNode<'_>) -> String {
    let mut ctx = GenCtx::new(top_name);
    collect_imports(node, &mut ctx);

    let mut out = String::new();
    writeln!(out, "// Auto-generated from MetaModelica source").unwrap();
    writeln!(out, "#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]").unwrap();
    writeln!(out, "{}", "
use mmwinnow::metamodelica::*; // Built-in types and functions
").unwrap();
    for line in ctx.use_lines() {
        writeln!(out, "{line}").unwrap();
    }
    if !ctx.unqual_modules.is_empty() || !ctx.named.is_empty() {
        writeln!(out).unwrap();
    }
    emit_node(&mut out, top_name, node, "", &mut ctx);
    out
}

// ── Node emission ─────────────────────────────────────────────────────────────

fn emit_node(out: &mut String, name: &str, node: &NameNode<'_>, indent: &str, ctx: &mut GenCtx) {
    let NodeKind::Class(c) = &node.kind else { return };
    use Absyn::Restriction::*;
    match &c.restriction {
        R_PACKAGE => {
            let mut children: Vec<_> = node.children.iter().collect();
            children.sort_by_key(|(n, _)| n.as_str());
            for (child_name, child_node) in children {
                emit_node(out, child_name, child_node, indent, &mut *ctx);
            }
        }
        R_UNIONTYPE => emit_uniontype(out, name, node, c, indent, &mut *ctx),
        R_TYPE => emit_type_item(out, name, node, c, indent, &mut *ctx),
        R_RECORD | R_METARECORD { .. } => emit_struct(out, name, node, c, indent, &mut *ctx),
        R_FUNCTION { .. } => emit_function(out, name, node, c, indent, &mut *ctx),
        _ => {}
    }
}

fn emit_uniontype(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx) {
    match &node.ty {
        Ty::RustEnum(_) => {
            writeln!(out, "{indent}pub enum {} {{", escape_ident(name)).unwrap();
            for rec_name in &records_in_order(c) {
                let Some(rec_node) = node.children.get(rec_name) else { continue };
                let NodeKind::Class(rc) = &rec_node.kind else { continue };
                match &rec_node.ty {
                    Ty::RustUnitVariant => {
                        writeln!(out, "{indent}    {rec_name},").unwrap();
                    }
                    Ty::RustStruct(_) => {
                        let fields = component_fields(rc, &rec_node.children);
                        if fields.is_empty() {
                            writeln!(out, "{indent}    {rec_name},").unwrap();
                        } else {
                            writeln!(out, "{indent}    {rec_name} {{").unwrap();
                            for (fname, fty) in &fields {
                                writeln!(out, "{indent}        {}: {},", escape_ident(fname), fmt_ty(fty, &mut *ctx)).unwrap();
                            }
                            writeln!(out, "{indent}    }},").unwrap();
                        }
                    }
                    _ => writeln!(out, "{indent}    {rec_name}, // unresolved").unwrap(),
                }
            }
            writeln!(out, "{indent}}}").unwrap();
            writeln!(out, "pub use {name}::*;").unwrap();
            writeln!(out).unwrap();
        }
        Ty::AliasTo(_) => {
            // Single-record uniontype: emit the struct, then a type alias.
            // Derive the struct name from the class definition (AliasTo now carries
            // the alias qname, not the struct name).
            let recs = records_in_order(c);
            let rec_name = recs.into_iter().next().unwrap_or_default();
            if let Some(rec_node) = node.children.get(&rec_name) {
                if let NodeKind::Class(rc) = &rec_node.kind {
                    emit_struct(out, &rec_name, rec_node, rc, indent, &mut *ctx);
                }
            }
            writeln!(out, "{indent}pub type {} = {};", escape_ident(name), escape_ident(&rec_name)).unwrap();
            writeln!(out).unwrap();
        }
        _ => {
            // No records — emit an opaque struct, with PhantomData to hold any type params.
            let type_vars: Vec<String> = match &c.body {
                MM::ClassDef::Parts { type_vars, .. } => type_vars.clone(),
                _ => vec![],
            };
            let ename = escape_ident(name);
            if type_vars.is_empty() {
                writeln!(out, "{indent}pub struct {ename};").unwrap();
            } else {
                let params = type_vars.join(", ");
                let phantom = if type_vars.len() == 1 {
                    type_vars[0].clone()
                } else {
                    format!("({})", params)
                };
                writeln!(out, "{indent}pub struct {ename}<{params}>(std::marker::PhantomData<{phantom}>);").unwrap();
            }
            writeln!(out).unwrap();
        }
    }
    // Emit function children of the uniontype
    emit_uniontype_functions(out, name, node, c, indent, &mut *ctx);
}

/// Emit functions that are direct children of a uniontype.
/// These are sorted alphabetically to match the pattern used for other children.
fn emit_uniontype_functions(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx) {
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return,
    };

    // Collect function children in declaration order
    let mut fns: Vec<(&str, &NameNode<'_>)> = Vec::new();
    for member in members {
        if let MM::ClassMember::ClassDef(cdm) = member {
            if matches!(cdm.class_def.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
                if let Some(child_node) = node.children.get(&cdm.class_def.name) {
                    fns.push((cdm.class_def.name.as_str(), child_node));
                }
            }
        }
    }

    for (fn_name, fn_node) in fns {
        if let NodeKind::Class(fn_class) = &fn_node.kind {
            emit_function(out, fn_name, fn_node, fn_class, indent, &mut *ctx);
        }
    }
}

fn emit_struct(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx) {
    let fields = component_fields(c, &node.children);
    let ename = escape_ident(name);
    if fields.is_empty() {
        writeln!(out, "{indent}pub struct {ename};").unwrap();
    } else {
        writeln!(out, "{indent}pub struct {ename} {{").unwrap();
        for (fname, fty) in &fields {
            writeln!(out, "{indent}    pub {}: {},", escape_ident(fname), fmt_ty(fty, &mut *ctx)).unwrap();
        }
        writeln!(out, "{indent}}}").unwrap();
    }
    writeln!(out).unwrap();
}

fn emit_type_item(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx) {
    match &c.body {
        MM::ClassDef::Derived { type_spec: Absyn::TypeSpec::TCOMPLEX { path: Absyn::Path::IDENT { name }, .. }, .. } if name == "polymorphic" => (),
        MM::ClassDef::Derived { .. } => {
            writeln!(out, "{indent}pub type {} = {};", escape_ident(name), fmt_ty(&node.ty, &mut *ctx)).unwrap();
            writeln!(out).unwrap();
        }
        MM::ClassDef::Enumeration { enum_literals, .. } => {
            if let Absyn::EnumDef::ENUMLITERALS { enumLiterals } = enum_literals {
                writeln!(out, "{indent}pub enum {} {{", escape_ident(name)).unwrap();
                for lit in enumLiterals {
                    let Absyn::EnumLiteral::ENUMLITERAL { literal, .. } = lit;
                    writeln!(out, "{indent}    {},", escape_ident(literal.as_str())).unwrap();
                }
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
            }
        }
        _ => {}
    }
}

fn emit_function(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx) {
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return,
    };

    // Use types from the resolved Ty::Function — those were computed by resolve_function_type
    // with the correct type_vars in scope, so type-variable parameters resolve correctly.
    // Child node .ty values are resolved without that context and may be Unknown for ArgT etc.
    let Ty::Function { type_vars, inputs: fn_inputs, output: fn_output } = &node.ty else { return };

    let type_params = if type_vars.is_empty() {
        String::new()
    } else {
        format!("<{}>", type_vars.join(", "))
    };

    let params = fn_inputs.iter()
        .map(|inp| format!("{}: {}", escape_ident(&inp.name), fmt_ty(&inp.ty, ctx)))
        .collect::<Vec<_>>()
        .join(", ");

    let ret = fmt_ty(fn_output, ctx);
    let ename = escape_ident(name);

    if c.partial_prefix {
        let type_only_params = fn_inputs.iter()
            .map(|inp| fmt_ty(&inp.ty, ctx))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(out, "{indent}pub type {ename} = fn({type_only_params}) -> {ret};").unwrap();
    } else {
        writeln!(out, "{indent}pub fn {ename}{type_params}({params}) -> {ret} {{").unwrap();
        writeln!(out, "{indent}    todo!()").unwrap();
        writeln!(out, "{indent}}}").unwrap();
    }
    writeln!(out).unwrap();
}

// ── Type formatting ───────────────────────────────────────────────────────────

fn fmt_ty(ty: &Ty, ctx: &mut GenCtx) -> String {
    match ty {
        Ty::Unknown => "/* ? */".to_owned(),
        Ty::I32 => "i32".to_owned(),
        Ty::F64 => "f64".to_owned(),
        Ty::Bool => "bool".to_owned(),
        Ty::Str => "String".to_owned(),
        Ty::Unit => "()".to_owned(),
        Ty::TypeVar(name) => name.clone(),
        Ty::RustUnitVariant => "()".to_owned(),
        Ty::Enumeration(name) | Ty::RustEnum(name) | Ty::RustStruct(name) | Ty::AliasTo(name) => {
            ctx.shorten(name)
        }
        Ty::UnionTypeVariant(union_qname, variant) => {
            // Rust cannot import through enums, so we emit `ShortenedUnionType::VariantName`.
            // The uniontype itself is shortened (or fully qualified) and the variant is appended.
            let union_short = ctx.shorten(union_qname);
            ctx.uniontype_imports.insert(union_qname.to_owned());
            format!("{union_short}::{variant}")
        }
        Ty::Option(inner) => format!("Option<{}>", fmt_ty(inner, ctx)),
        Ty::List(inner) => format!("List<{}>", fmt_ty(inner, ctx)),
        Ty::Array(inner) => format!("Array<{}>", fmt_ty(inner, ctx)),
        Ty::Tuple(tys) => {
            format!("({})", tys.iter().map(|t| fmt_ty(t, ctx)).collect::<Vec<_>>().join(", "))
        }
        Ty::Function { type_vars, inputs, output } => {
            let tvs = if type_vars.is_empty() {
                String::new()
            } else {
                format!("<{}>", type_vars.join(", "))
            };
            let ins = inputs.iter().map(|inp| fmt_ty(&inp.ty, ctx)).collect::<Vec<_>>().join(", ");
            format!("{tvs}fn({ins}) -> {}", fmt_ty(output, ctx))
        }
        Ty::FunctionAlias { base, .. } => ctx.shorten(base),
        Ty::Generic(name, args) => {
            format!("{name}<{}>", args.iter().map(|t| fmt_ty(t, ctx)).collect::<Vec<_>>().join(", "))
        }
        Ty::ExternalObject(_) => {
            // External objects are opaque C handles in Rust.
            "std::ffi::c_void".to_owned()
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn records_in_order(c: &MM::Class) -> Vec<String> {
    let members = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return vec![],
    };
    members.iter()
        .filter_map(|m| {
            if let MM::ClassMember::ClassDef(cdm) = m {
                if matches!(cdm.class_def.restriction,
                    Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })
                {
                    return Some(cdm.class_def.name.clone());
                }
            }
            None
        })
        .collect()
}

fn component_fields<'a>(c: &'a MM::Class, children: &'a HashMap<String, NameNode<'_>>) -> Vec<(&'a str, &'a Ty)> {
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return vec![],
    };
    members.iter()
        .filter_map(|m| {
            if let MM::ClassMember::Component(comp) = m {
                let ty = children.get(&comp.name).map(|n| &n.ty)?;
                Some((comp.name.as_str(), ty))
            } else {
                None
            }
        })
        .collect()
}

/// Prefix Rust keywords with `r#` so they are valid identifiers.
/// `self`, `super`, `crate`, and `Self` cannot be raw identifiers and are left as-is.
fn escape_ident(name: &str) -> String {
    match name {
        // strict keywords (edition-independent)
        "as" | "break" | "const" | "continue" | "else" | "enum" | "extern" |
        "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" |
        "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" |
        "static" | "struct" | "trait" | "true" | "type" | "unsafe" | "use" |
        "where" | "while" |
        // reserved keywords
        "abstract" | "async" | "await" | "become" | "box" | "do" | "dyn" |
        "final" | "macro" | "override" | "priv" | "try" | "typeof" |
        "unsized" | "virtual" | "yield" => format!("r#{name}"),
        _ => name.to_owned(),
    }
}

/// Map a top-level dotted module name to the Rust path prefix for `use` statements.
/// Known external modules are mapped to their crate paths; everything else is `crate::`.
fn module_rust_prefix(dotted_module: &str) -> String {
    match dotted_module {
        "MetaModelica" => "mmwinnow::metamodelica".to_owned(),
        "MetaModelica.Dangerous" => "mmwinnow::metamodelica::Dangerous".to_owned(),
        _ => format!("crate::{}", dotted_module.replace('.', "::")),
    }
}

/// Convert a fully-dotted import path (e.g. `MetaModelica.List`) to a Rust path.
fn dotted_to_rust_path(dotted: &str) -> String {
    // Find the top-level segment and reroute known external modules.
    let top = dotted.split('.').next().unwrap_or(dotted);
    match top {
        "MetaModelica" => format!("mmwinnow::metamodelica{}", &dotted[top.len()..].replace('.', "::")),
        _ => format!("crate::{}", dotted.replace('.', "::")),
    }
}

fn path_to_dotted(path: &Absyn::Path) -> String {
    match path {
        Absyn::Path::IDENT { name } => name.clone(),
        Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", path_to_dotted(path)),
        Absyn::Path::FULLYQUALIFIED { path } => path_to_dotted(path),
    }
}