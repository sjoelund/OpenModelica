#![allow(unused)]

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write;
use mmwinnow::Absyn;
use crate::MM;
use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind, Ty};

// ── Import-aware generation context ──────────────────────────────────────────

struct GenCtx {
    /// Name of the top-level class being generated (e.g. "Absyn").
    top_name: String,
    /// Current module path within the top-level file (e.g. `["Connect"]` when
    /// inside `mod Connect`). Used by `shorten` to produce context-relative paths.
    current_path: Vec<String>,
    /// Modules imported with `.*`; their types are referenced by bare name.
    unqual_modules: HashSet<String>,
    /// Explicit imports: dotted qualified name → local name.
    named: BTreeMap<String, String>,
    /// Uniontypes (Rust enums) whose variants are referenced via UnionTypeVariant.
    /// Their qualified names need to be imported so the generated code can use `UnionType::Variant`.
    uniontype_imports: HashSet<String>,
    /// The Rust crate name for the file being generated (e.g. "openmodelica_frontend").
    /// `None` means the default openmodelica crate.
    current_crate: Option<String>,
    /// Maps top-level MM package names to their Rust crate names.
    crate_map: BTreeMap<String, String>,
    /// Simple names of top-level uniontypes (those emitted as their own module+type file).
    /// When used as a type from outside their module, they need `Name::Name` not just `Name`.
    top_level_uniontype_names: HashSet<String>,
}

impl GenCtx {
    fn new(top_name: &str, current_crate: Option<String>, crate_map: BTreeMap<String, String>, top_level_uniontype_names: HashSet<String>) -> Self {
        Self {
            top_name: top_name.to_owned(),
            current_path: Vec::new(),
            unqual_modules: HashSet::new(),
            named: BTreeMap::new(),
            uniontype_imports: HashSet::new(),
            current_crate,
            crate_map,
            top_level_uniontype_names,
        }
    }

    /// Shorten a dot-separated qualified name to the shortest valid reference
    /// for this file, based on the collected imports and current nesting context.
    fn shorten(&self, dotted: &str) -> String {
        // Build the current module's full dotted prefix (e.g. "DAE.Connect").
        let cur_prefix = if self.current_path.is_empty() {
            self.top_name.clone()
        } else {
            format!("{}.{}", self.top_name, self.current_path.join("."))
        };

        // Exact same-module item: strip the current prefix (longest match first).
        if let Some(rest) = dotted.strip_prefix(&format!("{cur_prefix}.")) {
            return rest.replace('.', "::");
        }

        // Item in the same top-level file but a different nested module.
        // With `use super::*;` in every nested mod, sibling/ancestor items and
        // modules are visible by their path relative to the top-level package.
        if let Some(rest) = dotted.strip_prefix(&format!("{}.", self.top_name)) {
            return rest.replace('.', "::");
        }

        // Named / qualified import.
        if let Some(local) = self.named.get(dotted) {
            return local.clone();
        }

        // Wildcard import: if a module prefix matches, convert the remainder to a
        // Rust path so nested-package items resolve through their `mod` blocks.
        for module in &self.unqual_modules {
            if let Some(rest) = dotted.strip_prefix(&format!("{module}.")) {
                return rest.replace('.', "::");
            }
        }

        // Fully-qualified Rust path.
        dotted.replace('.', "::")
    }

    /// Sorted `use` lines to emit at the top of the file.
    fn use_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for module in &self.unqual_modules {
            let rust = self.module_rust_prefix(module);
            lines.push(format!("use {rust}::*;"));
        }
        for (dotted, local) in &self.named {
            let rust = self.dotted_to_rust_path(dotted);
            let last = dotted.rsplit('.').next().unwrap_or(dotted);
            if local == last {
                lines.push(format!("use {rust};"));
            } else {
                lines.push(format!("use {rust} as {local};"));
            }
        }
        // Import uniontypes that are referenced via UnionTypeVariant syntax.
        for uniontype_qname in &self.uniontype_imports {
            let rust = self.dotted_to_rust_path(uniontype_qname);
            lines.push(format!("use {rust};"));
        }
        lines.sort();
        lines.dedup();
        lines
    }

    /// Map a top-level dotted module name to the Rust path prefix for `use` statements.
    fn module_rust_prefix(&self, dotted_module: &str) -> String {
        match dotted_module {
            "MetaModelica" => "metamodelica".to_owned(),
            "MetaModelica.Dangerous" => "metamodelica::Dangerous".to_owned(),
            _ => {
                let top = dotted_module.split('.').next().unwrap_or(dotted_module);
                match self.crate_map.get(top) {
                    Some(mc) if Some(mc) == self.current_crate.as_ref() => {
                        format!("crate::{}", dotted_module.replace('.', "::"))
                    }
                    Some(mc) => {
                        let rest = &dotted_module[top.len()..];
                        format!("{mc}{}", rest.replace('.', "::"))
                    }
                    None => format!("crate::{}", dotted_module.replace('.', "::")),
                }
            }
        }
    }

    /// Convert a fully-dotted import path (e.g. `MetaModelica.List`) to a Rust path.
    fn dotted_to_rust_path(&self, dotted: &str) -> String {
        let top = dotted.split('.').next().unwrap_or(dotted);
        match top {
            "MetaModelica" => {
                format!("metamodelica{}", &dotted[top.len()..].replace('.', "::"))
            }
            _ => match self.crate_map.get(top) {
                Some(mc) if Some(mc) == self.current_crate.as_ref() => {
                    format!("crate::{}", dotted.replace('.', "::"))
                }
                Some(mc) => {
                    let rest = &dotted[top.len()..];
                    format!("{mc}{}", rest.replace('.', "::"))
                }
                None => format!("crate::{}", dotted.replace('.', "::")),
            },
        }
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
    // Build crate_map: top-level class name → Rust crate name.
    let crate_map: BTreeMap<String, String> = hier.top_level.iter()
        .filter_map(|(name, node)| {
            if let NodeKind::Class(c) = &node.kind {
                c.crate_name.as_ref().map(|cn| (name.clone(), cn.clone()))
            } else {
                None
            }
        })
        .collect();

    // Collect names of top-level uniontypes — they are emitted as both a module
    // and a type inside that module, so references from other files need `Name::Name`.
    let top_level_uniontype_names: HashSet<String> = hier.top_level.iter()
        .filter_map(|(name, node)| {
            if let NodeKind::Class(MM::Class { restriction: Absyn::Restriction::R_UNIONTYPE, .. }) = &node.kind {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();

    // Group top-level classes by their output directory.
    let mut dir_classes: BTreeMap<String, Vec<(&str, &NameNode<'_>)>> = BTreeMap::new();
    for (name, node) in &hier.top_level {
        let dir = if let NodeKind::Class(c) = &node.kind {
            if let Some(cn) = &c.crate_name {
                format!("{cn}/src")
            } else {
                output_dir.to_owned()
            }
        } else {
            output_dir.to_owned()
        };
        dir_classes.entry(dir).or_default().push((name.as_str(), node));
    }

    for (dir, classes) in &dir_classes {
        if dir == "openmodelica/src" {
            continue; // Builtin - handwritten code
        };
        std::fs::create_dir_all(dir)?;
        for (name, node) in classes {
            let current_crate = if let NodeKind::Class(c) = &node.kind {
                c.crate_name.clone()
            } else {
                None
            };
            let content = generate_file(name, node, &crate_map, current_crate, &top_level_uniontype_names);
            std::fs::write(format!("{dir}/{name}.rs"), content)?;
        }
        let lib_content = generate_lib_file(hier, dir, output_dir);
        std::fs::write(format!("{dir}/lib.rs"), lib_content)?;
    }
    Ok(())
}

fn generate_lib_file(hier: &InstanceHierarchy<'_>, this_dir: &str, default_dir: &str) -> String {
    let mut out = String::new();
    writeln!(out, "// Auto-generated lib file").unwrap();
    for (name, node) in &hier.top_level {
        let node_dir = if let NodeKind::Class(c) = &node.kind {
            if let Some(cn) = &c.crate_name { format!("{cn}/src") } else { default_dir.to_owned() }
        } else {
            default_dir.to_owned()
        };
        if node_dir != this_dir { continue; }
        match node.kind {
            NodeKind::Class(MM::Class{restriction: Absyn::Restriction::R_PACKAGE, ..}) |
            NodeKind::Class(MM::Class{restriction: Absyn::Restriction::R_UNIONTYPE, ..}) => {
                writeln!(out, "pub mod {name};").unwrap();
            },
            _ => continue,
        }
    }
    out
}

fn generate_file(top_name: &str, node: &NameNode<'_>, crate_map: &BTreeMap<String, String>, current_crate: Option<String>, top_level_uniontype_names: &HashSet<String>) -> String {
    let mut ctx = GenCtx::new(top_name, current_crate, crate_map.clone(), top_level_uniontype_names.clone());
    collect_imports(node, &mut ctx);

    let mut out = String::new();
    writeln!(out, "// Auto-generated from MetaModelica source").unwrap();
    writeln!(out, "#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]").unwrap();
    writeln!(out, "{}", "
use metamodelica::*; // Built-in types and functions
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
            let nested_indent = format!("{indent}    ");
            let wrap = name != ctx.top_name;
            let child_indent = if wrap {
                // Nested package: emit as a `pub mod` block so items don't
                // collide with same-named items in the parent package.
                writeln!(out, "{indent}pub mod {name} {{").unwrap();
                // Import everything from the parent module so that types
                // defined in the enclosing package are visible by simple name.
                writeln!(out, "{nested_indent}use super::*;").unwrap();
                ctx.current_path.push(name.to_owned());
                nested_indent
            } else {
                indent.to_owned()
            };
            let mut children: Vec<_> = node.children.iter().collect();
            children.sort_by_key(|(n, _)| n.as_str());
            for (child_name, child_node) in children {
                emit_node(out, child_name, child_node, &child_indent, &mut *ctx);
            }
            if wrap {
                ctx.current_path.pop();
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
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
            writeln!(out, "{indent}pub use {name}::*;").unwrap();
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
            let type_vars: Vec<String> = match &c.body {
                MM::ClassDef::Parts { type_vars, .. } => type_vars.clone(),
                _ => vec![],
            };
            let ename = escape_ident(name);
            let rec_ename = escape_ident(&rec_name);
            if type_vars.is_empty() {
                writeln!(out, "{indent}pub type {ename} = {rec_ename};").unwrap();
            } else {
                let params = type_vars.join(", ");
                writeln!(out, "{indent}pub type {ename}<{params}> = {rec_ename}<{params}>;").unwrap();
            }
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
    let mut type_vars: Vec<String> = Vec::new();
    for (_, fty) in &fields {
        collect_type_vars_in_ty(fty, &mut type_vars);
    }
    let type_params = if type_vars.is_empty() { String::new() } else { format!("<{}>", type_vars.join(", ")) };
    if fields.is_empty() {
        writeln!(out, "{indent}pub struct {ename}{type_params};").unwrap();
    } else {
        writeln!(out, "{indent}pub struct {ename}{type_params} {{").unwrap();
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
            let mut type_vars: Vec<String> = Vec::new();
            collect_type_vars_in_ty(&node.ty, &mut type_vars);
            let type_params = if type_vars.is_empty() { String::new() } else { format!("<{}>", type_vars.join(", ")) };
            writeln!(out, "{indent}pub type {}{type_params} = {};", escape_ident(name), fmt_ty(&node.ty, &mut *ctx)).unwrap();
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

    let mut all_type_vars = type_vars.clone();
    for inp in fn_inputs.iter() {
        collect_type_vars_in_ty(&inp.ty, &mut all_type_vars);
    }
    collect_type_vars_in_ty(fn_output, &mut all_type_vars);
    let type_params = if all_type_vars.is_empty() {
        String::new()
    } else {
        format!("<{}>", all_type_vars.join(", "))
    };

    let params = fn_inputs.iter()
        .map(|inp| format!("{}: {}", escape_ident(&inp.name), fmt_ty(&inp.ty, ctx)))
        .collect::<Vec<_>>()
        .join(", ");

    let ret = fmt_ty(fn_output, ctx);
    let ename = escape_ident(name);

    let pub_kw = if node.visibility == MM::Visibility::Public { "pub " } else { "" };
    if c.partial_prefix {
        let type_only_params = fn_inputs.iter()
            .map(|inp| fmt_ty(&inp.ty, ctx))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(out, "{indent}{pub_kw}type {ename}{type_params} = fn({type_only_params}) -> {ret};").unwrap();
    } else {
        writeln!(out, "{indent}{pub_kw}fn {ename}{type_params}({params}) -> {ret} {{").unwrap();
        writeln!(out, "{indent}    todo!()").unwrap();
        writeln!(out, "{indent}}}").unwrap();
    }
    writeln!(out).unwrap();
}

// ── Type formatting ───────────────────────────────────────────────────────────

fn collect_type_vars_in_ty(ty: &Ty, out: &mut Vec<String>) {
    match ty {
        Ty::TypeVar(name) => { if !out.contains(name) { out.push(name.clone()); } }
        Ty::Option(inner) | Ty::List(inner) | Ty::Array(inner) => collect_type_vars_in_ty(inner, out),
        Ty::Tuple(tys) => tys.iter().for_each(|t| collect_type_vars_in_ty(t, out)),
        Ty::Generic(_, args) => args.iter().for_each(|t| collect_type_vars_in_ty(t, out)),
        Ty::Function { inputs, output, .. } => {
            inputs.iter().for_each(|inp| collect_type_vars_in_ty(&inp.ty, out));
            collect_type_vars_in_ty(output, out);
        }
        _ => {}
    }
}

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
        Ty::Enumeration(name) | Ty::RustStruct(name) => ctx.shorten(name),
        Ty::RustEnum(name) | Ty::AliasTo(name) => {
            // Top-level uniontypes are emitted as both a module and a type inside
            // that module, so a reference from any other file needs `Module::Type`.
            let first = name.split('.').next().unwrap_or(name);
            let last = name.rsplit('.').next().unwrap_or(name);
            let shortened = ctx.shorten(name);
            if ctx.top_level_uniontype_names.contains(first) && first != ctx.top_name {
                format!("{shortened}::{last}")
            } else {
                shortened
            }
        }
        Ty::UnionTypeVariant(union_qname, variant) => {
            // Rust cannot import through enums, so we emit `ShortenedUnionType::VariantName`.
            // The uniontype itself is shortened (or fully qualified) and the variant is appended.
            let union_short = ctx.shorten(union_qname);
            ctx.uniontype_imports.insert(union_qname.to_owned());
            format!("{union_short}::{variant}")
        }
        Ty::Option(inner) => format!("Option<{}>", fmt_ty(inner, ctx)),
        Ty::List(inner) => format!("metamodelica::List<{}>", fmt_ty(inner, ctx)),
        Ty::Array(inner) => format!("Vec<{}>", fmt_ty(inner, ctx)),
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
            // `name` is already in Rust form (dots replaced with ::) by ty_rust_name.
            // Convert back to dotted form so shorten() can produce a context-relative
            // path, then apply the top-level uniontype doubling if needed.
            let dotted = name.replace("::", ".");
            let first = dotted.split('.').next().unwrap_or(&dotted);
            let last = dotted.rsplit('.').next().unwrap_or(&dotted);
            let shortened = ctx.shorten(&dotted);
            let base = if ctx.top_level_uniontype_names.contains(first) && first != ctx.top_name {
                format!("{shortened}::{last}")
            } else {
                shortened
            };
            format!("{base}<{}>", args.iter().map(|t| fmt_ty(t, ctx)).collect::<Vec<_>>().join(", "))
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

fn component_fields<'a>(c: &'a MM::Class, children: &'a BTreeMap<String, NameNode<'_>>) -> Vec<(&'a str, &'a Ty)> {
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

fn path_to_dotted(path: &Absyn::Path) -> String {
    match path {
        Absyn::Path::IDENT { name } => name.clone(),
        Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", path_to_dotted(path)),
        Absyn::Path::FULLYQUALIFIED { path } => path_to_dotted(path),
    }
}