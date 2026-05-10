#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write;
use mmwinnow::Absyn;
use crate::MM;
use std::collections::HashMap;
use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind, Ty, extract_default, extract_default_exp, lookup_node, lookup_node_ty, uniontype_needs_mod};
use crate::typedexp::{self, TypedExp, TypedPat, TypedCase, Lit, BinOpKind, UnOpKind, MatchKind, cref_to_dotted};

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
    /// Top-level module names that are used via fully-qualified paths (e.g. `FCore::Node`)
    /// but not explicitly imported by the MetaModelica source — discovered during emit.
    implicit_modules: BTreeSet<String>,
    /// The Rust crate name for the file being generated (e.g. "openmodelica_frontend").
    /// `None` means the default openmodelica crate.
    current_crate: Option<String>,
    /// Maps top-level MM package names to their Rust crate names.
    crate_map: BTreeMap<String, String>,
    /// Simple names of top-level uniontypes (those emitted as their own module+type file).
    /// When used as a type from outside their module, they need `Name::Name` not just `Name`.
    top_level_uniontype_names: HashSet<String>,
    /// Fully-qualified names of types that are recursive (form size cycles); their field
    /// references are wrapped in `Arc<>` to give Rust a fixed-size indirection.
    recursive_types: BTreeSet<String>,
    /// Fully-qualified names of nested uniontypes that are NOT wrapped in a `pub mod`
    /// (because they contain only records). These must NOT get the `::TypeName` doubling
    /// that mod-wrapped uniontypes require.
    no_mod_uniontypes: HashSet<String>,
    /// Maps fully-qualified function/partial-function names to their effective type variables
    /// (collected from inputs and output). Used at codegen time to emit generic arguments when
    /// a FunctionAlias type is referenced as a parameter type (e.g. `toStringT` → `toStringT<T>`).
    fn_type_vars: BTreeMap<String, Vec<String>>,
}

impl GenCtx {
    fn new(top_name: &str, current_crate: Option<String>, crate_map: BTreeMap<String, String>, top_level_uniontype_names: HashSet<String>, recursive_types: BTreeSet<String>, fn_type_vars: BTreeMap<String, Vec<String>>) -> Self {
        Self {
            top_name: top_name.to_owned(),
            current_path: Vec::new(),
            unqual_modules: HashSet::new(),
            named: BTreeMap::new(),
            uniontype_imports: HashSet::new(),
            implicit_modules: BTreeSet::new(),
            current_crate,
            crate_map,
            top_level_uniontype_names,
            recursive_types,
            no_mod_uniontypes: HashSet::new(),
            fn_type_vars,
        }
    }

    /// Shorten a dot-separated qualified name to the shortest valid reference
    /// for this file, based on the collected imports and current nesting context.
    fn shorten(&mut self, dotted: &str) -> String {
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

        // Named / qualified import (exact match: e.g. `FUnit` → `Unit`).
        if let Some(local) = self.named.get(dotted) {
            return local.clone();
        }
        // Named import prefix match: e.g. `FUnit.Unit` with `FUnit → Unit` becomes `Unit::Unit`.
        for (module_path, local_alias) in &self.named {
            if let Some(rest) = dotted.strip_prefix(&format!("{module_path}.")) {
                return format!("{local_alias}::{}", rest.replace('.', "::"));
            }
        }

        // Wildcard import: if a module prefix matches, convert the remainder to a
        // Rust path so nested-package items resolve through their `mod` blocks.
        for module in &self.unqual_modules {
            if let Some(rest) = dotted.strip_prefix(&format!("{module}.")) {
                return rest.replace('.', "::");
            }
        }

        // Fully-qualified Rust path — record the top-level module as implicitly needed,
        // but only for known MM packages (those in crate_map). Bare names like `SourceInfo`
        // that fall through here are builtins provided by `metamodelica::*`, not modules.
        let top = dotted.split('.').next().unwrap_or(dotted);
        if top != self.top_name && self.crate_map.contains_key(top) {
            self.implicit_modules.insert(top.to_owned());
        }
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
        // Modules referenced transitively (e.g. FCore in type aliases resolved through FNode).
        for module in &self.implicit_modules {
            let rust = self.module_rust_prefix(module);
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
                        format!("{mc}::{top}{}", rest.replace('.', "::"))
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
                    format!("{mc}::{top}{}", rest.replace('.', "::"))
                }
                None => format!("crate::{}", dotted.replace('.', "::")),
            },
        }
    }
}

fn is_const_component<'a>(dotted: &str, top_level: &'a BTreeMap<String, NameNode<'a>>) -> bool {
    match lookup_node(dotted, top_level) {
        Some(n) => matches!(&n.kind, NodeKind::Component(m) if m.variability == Absyn::Variability::CONST),
        None => false,
    }
}

/// Return true if `dotted` (e.g. `"NFInstNode.NodeTree"`) resolves to a real node in the
/// hierarchy. Used to skip dangling imports that appear in the MetaModelica source but
/// refer to names that no longer exist (or never did) in the parsed tree.
fn path_exists_in_hierarchy<'a>(dotted: &str, top_level: &'a BTreeMap<String, NameNode<'a>>) -> bool {
    let mut parts = dotted.split('.');
    let first = parts.next().unwrap_or("");
    let Some(mut node) = top_level.get(first) else { return false };
    for part in parts {
        let Some(child) = node.children.get(part) else { return false };
        node = child;
    }
    true
}

/// If `dotted` ends at an Import node (i.e. an alias defined inside a module),
/// return the resolved target's fully-qualified dotted name from the node's `ty`.
/// This prevents generating `use crate::Mod::PrivateAlias;` when the alias is a
/// private `use` inside that module; callers should use the resolved path instead.
fn resolve_through_import_node<'a>(
    dotted: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<String> {
    let mut parts = dotted.split('.');
    let first = parts.next().unwrap_or("");
    let Some(mut node) = top_level.get(first) else { return None };
    for part in parts {
        let Some(child) = node.children.get(part) else { return None };
        node = child;
    }
    if let NodeKind::Import(m) = &node.kind {
        // Prefer the resolved type if available (gives the canonical dotted name).
        match &node.ty {
            Ty::RustEnum(n) | Ty::AliasTo(n) | Ty::RustStruct(n) => return Some(n.clone()),
            _ => {}
        }
        // Fallback: read the target path directly from the AST (covers packages,
        // which have Ty::Unknown since they are containers rather than value types).
        let target = match &m.import {
            Absyn::Import::NAMED_IMPORT { path, .. } | Absyn::Import::QUAL_IMPORT { path } => {
                let d = path_to_dotted(path);
                if d.is_empty() { return None; }
                d
            }
            _ => return None,
        };
        Some(target)
    } else {
        None
    }
}

/// Walk the subtree collecting file-level import nodes into `ctx`.
/// Stops at function boundaries — imports inside a function body are local to that function.
/// Skips imports whose resolved path starts with `{top_name}.`: those refer to siblings
/// within the same file and are already in scope through `use super::*;`.
/// Skips imports that don't resolve to a real node in the hierarchy (dangling imports).
fn collect_imports<'a>(node: &NameNode<'_>, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
    let same_file_prefix = format!("{}.", ctx.top_name);
    for child in node.children.values() {
        match &child.kind {
            NodeKind::Import(m) => match &m.import {
                Absyn::Import::UNQUAL_IMPORT { path } => {
                    let dotted = path_to_dotted(path);
                    if dotted != ctx.top_name && !dotted.starts_with(&same_file_prefix) && path_exists_in_hierarchy(&dotted, top_level) {
                        if is_const_component(&dotted, top_level) {
                            // `import Pkg.CONST;` — target is a constant, not a module; emit as named import.
                            let last = dotted.rsplit('.').next().unwrap_or(&dotted).to_owned();
                            ctx.named.insert(dotted, last);
                        } else {
                            ctx.unqual_modules.insert(dotted);
                        }
                    }
                }
                Absyn::Import::QUAL_IMPORT { path } => {
                    let dotted = path_to_dotted(path);
                    if dotted != ctx.top_name && !dotted.starts_with(&same_file_prefix) && path_exists_in_hierarchy(&dotted, top_level) {
                        let last = dotted.rsplit('.').next().unwrap_or(&dotted).to_owned();
                        // If the path ends at an Import alias node (e.g. `NFCall.Call` where
                        // `Call = NFCall` inside NFCall), use the resolved target so we emit
                        // `use crate::NFCall as Call` rather than `use crate::NFCall::Call`
                        // (the latter would fail — the alias is a private `use` in that module).
                        let effective = resolve_through_import_node(&dotted, top_level)
                            .unwrap_or(dotted.clone());
                        // If the node is a RustStruct whose qname differs from the import path,
                        // the record was renamed to its parent uniontype (single-record uniontype).
                        // Import the struct at its actual path (the uniontype) aliased to the
                        // record's name, e.g. `use crate::SimCode::SimCode as SIMCODE;`.
                        let effective = match lookup_node_ty(&effective, top_level) {
                            Some(Ty::RustStruct(struct_qname)) if struct_qname != &effective => struct_qname.clone(),
                            _ => effective,
                        };
                        if effective != ctx.top_name && !effective.starts_with(&same_file_prefix) {
                            ctx.named.insert(effective, last);
                        }
                    }
                }
                Absyn::Import::NAMED_IMPORT { name, path } => {
                    let dotted = path_to_dotted(path);
                    if dotted != ctx.top_name && !dotted.starts_with(&same_file_prefix) && path_exists_in_hierarchy(&dotted, top_level) {
                        let effective = resolve_through_import_node(&dotted, top_level)
                            .unwrap_or(dotted.clone());
                        // Same renamed-record correction as for QUAL_IMPORT.
                        let effective = match lookup_node_ty(&effective, top_level) {
                            Some(Ty::RustStruct(struct_qname)) if struct_qname != &effective => struct_qname.clone(),
                            _ => effective,
                        };
                        if effective != ctx.top_name && !effective.starts_with(&same_file_prefix) {
                            ctx.named.insert(effective, name.clone());
                        }
                    }
                }
                Absyn::Import::GROUP_IMPORT { prefix, groups } => {
                    let prefix_str = path_to_dotted(prefix);
                    if prefix_str != ctx.top_name && !prefix_str.starts_with(&same_file_prefix) {
                        for g in groups.into_iter() {
                            let (local, orig) = match g {
                                Absyn::GroupImport::GROUP_IMPORT_NAME { name } => (name.clone(), name.clone()),
                                Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, name } => (rename.clone(), name.clone()),
                            };
                            let full = format!("{prefix_str}.{orig}");
                            if path_exists_in_hierarchy(&full, top_level) {
                                // Same renamed-record correction as for QUAL_IMPORT.
                                let effective = match lookup_node_ty(&full, top_level) {
                                    Some(Ty::RustStruct(struct_qname)) if struct_qname != &full => struct_qname.clone(),
                                    _ => full,
                                };
                                ctx.named.insert(effective, local);
                            }
                        }
                    }
                }
            },
            NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) => {
                // Don't recurse into functions — their imports are local to that function.
            }
            _ => collect_imports(child, ctx, top_level),
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

    // Collect fully-qualified names of all nested (non-top-level) uniontypes that have
    // no `pub mod` wrapper because they contain only records. Built once across the whole
    // hierarchy so that cross-file references (e.g. FlagsUtil referencing Flags.FlagData)
    // also suppress the `::TypeName` doubling.
    let mut no_mod_uniontypes: HashSet<String> = HashSet::new();
    for (top_name, top_node) in &hier.top_level {
        collect_no_mod_uniontypes(&top_node.children, top_name, &mut no_mod_uniontypes);
    }

    // Build a map from fully-qualified function names to their effective type variables.
    // Used at codegen time to emit generic arguments for FunctionAlias parameter types.
    let mut fn_type_vars: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (top_name, top_node) in &hier.top_level {
        collect_fn_type_vars(top_node, top_name, &mut fn_type_vars);
    }

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
            let content = generate_file(name, node, &crate_map, current_crate, &top_level_uniontype_names, hier.recursive_types.clone(), &no_mod_uniontypes, &hier.top_level, &fn_type_vars);
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

/// Collect fully-qualified names of nested uniontypes (inside packages) that have no
/// `pub mod` wrapper because they contain only records. Used to suppress the `::TypeName`
/// doubling in `fmt_ty` for those types.
/// Recursively walk the node tree and record, for each function node, the effective
/// type variables present in its inputs/output. Used to emit `foo<T>` when a
/// `FunctionAlias` referencing `foo` appears as a parameter type.
fn collect_fn_type_vars(node: &NameNode<'_>, qname: &str, map: &mut BTreeMap<String, Vec<String>>) {
    if let Ty::Function { .. } = &node.ty {
        let mut tvs: Vec<String> = Vec::new();
        collect_type_vars_in_ty(&node.ty, &mut tvs);
        map.insert(qname.to_owned(), tvs);
    }
    for (child_name, child_node) in &node.children {
        let child_qname = format!("{qname}.{child_name}");
        collect_fn_type_vars(child_node, &child_qname, map);
    }
}

fn collect_no_mod_uniontypes(nodes: &BTreeMap<String, NameNode<'_>>, prefix: &str, out: &mut HashSet<String>) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Class(c) = &node.kind {
            if matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE)
                && !prefix.is_empty()
                && !uniontype_needs_mod(node)
            {
                out.insert(qname.clone());
            }
        }
        collect_no_mod_uniontypes(&node.children, &qname, out);
    }
}

fn generate_file<'a>(top_name: &str, node: &NameNode<'_>, crate_map: &BTreeMap<String, String>, current_crate: Option<String>, top_level_uniontype_names: &HashSet<String>, recursive_types: BTreeSet<String>, no_mod_uniontypes: &HashSet<String>, top_level: &'a BTreeMap<String, NameNode<'a>>, fn_type_vars: &BTreeMap<String, Vec<String>>) -> String {
    let mut ctx = GenCtx::new(top_name, current_crate, crate_map.clone(), top_level_uniontype_names.clone(), recursive_types, fn_type_vars.clone());
    ctx.no_mod_uniontypes = no_mod_uniontypes.clone();
    collect_imports(node, &mut ctx, top_level);

    // First pass: emit the body so that shorten() can populate implicit_modules.
    let mut body = String::new();
    emit_node(&mut body, top_name, node, "", &mut ctx, top_level);

    // Second pass: emit header + complete use lines (now including implicit modules).
    let mut out = String::new();
    writeln!(out, "// Auto-generated from MetaModelica source").unwrap();
    writeln!(out, "#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals)]").unwrap();
    writeln!(out, "{}", "
use std::sync::Arc;
use anyhow::{Result, bail};
use metamodelica::*; // Built-in types and functions
use const_str;
").unwrap();
    for line in ctx.use_lines() {
        writeln!(out, "{line}").unwrap();
    }
    if !ctx.unqual_modules.is_empty() || !ctx.named.is_empty() || !ctx.implicit_modules.is_empty() {
        writeln!(out).unwrap();
    }
    out.push_str(&body);
    out
}

// ── Node emission ─────────────────────────────────────────────────────────────

fn emit_node<'a>(out: &mut String, name: &str, node: &NameNode<'_>, indent: &str, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
    if let NodeKind::Component(m) = &node.kind {
        if m.variability == Absyn::Variability::CONST {
            let rust_ty = match &node.ty {
                Ty::Str => "&'static str",
                Ty::I32 => "i32",
                Ty::F64 => "f64",
                Ty::Bool => "bool",
                _ => return,
            };
            if let Some(exp) = extract_default_exp(&m.modification) {
                let pkg_prefix = if ctx.current_path.is_empty() {
                    ctx.top_name.to_owned()
                } else {
                    format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
                };
                let typed = typedexp::infer_exp(exp, &HashMap::new(), top_level, &pkg_prefix);
                let val = emit_exp(&typed, /*is_const=*/true, ctx);
                let ename = escape_ident(name);
                writeln!(out, "{indent}pub const {ename}: {rust_ty} = {val};").unwrap();
                writeln!(out).unwrap();
            }
        }
        return;
    }
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
                // `use super::*` brings in all file-level imports (external crates,
                // metamodelica builtins, Arc, etc.) and sibling items defined in this
                // file. Encapsulated packages still need these Rust-level "builtins";
                // their MetaModelica encapsulation is enforced by collect_imports not
                // hoisting same-file sibling imports to the file's `use` list.
                writeln!(out, "{nested_indent}use super::*;").unwrap();
                ctx.current_path.push(name.to_owned());
                nested_indent
            } else {
                indent.to_owned()
            };
            let mut children: Vec<_> = node.children.iter().collect();
            children.sort_by_key(|(n, _)| n.as_str());
            for (child_name, child_node) in children {
                emit_node(out, child_name, child_node, &child_indent, &mut *ctx, top_level);
            }
            if wrap {
                ctx.current_path.pop();
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
            }
        }
        R_UNIONTYPE => emit_uniontype(out, name, node, c, indent, &mut *ctx, top_level),
        R_TYPE | R_ENUMERATION => emit_type_item(out, name, node, c, indent, &mut *ctx),
        R_RECORD | R_METARECORD { .. } => emit_struct(out, name, node, c, indent, &mut *ctx),
        R_FUNCTION { .. } => emit_function(out, name, node, c, indent, &mut *ctx, top_level),
        _ => {}
    }
}

/// Emit a uniontype as a `pub mod <name>` containing the type definition and any
/// function children. Wrapping in a mod prevents name collisions when two unittypes
/// in the same package both define a function with the same name (e.g. `new`), and
/// mirrors how function calls are resolved: `SBGraph.IncidenceList.new` shortens to
/// `IncidenceList::new` naturally via `shorten()`.
fn emit_uniontype<'a>(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
    // Top-level unittypes (own file, name == top_name) are already a Rust module by
    // virtue of their file — don't add a redundant inner `pub mod`. Nested unittypes
    // that contain only records also don't need a mod: there are no functions or other
    // members to access via `TypeName::member`. The mod is only needed when non-record
    // children (functions, nested packages, …) must be reachable as `TypeName::fn_name`.
    let wrap_in_mod = name != ctx.top_name && uniontype_needs_mod(node);
    let inner;
    let ename = escape_ident(name);
    if wrap_in_mod {
        inner = format!("{indent}    ");
        writeln!(out, "{indent}pub mod {ename} {{").unwrap();
        writeln!(out, "{inner}use super::*;").unwrap();
        ctx.current_path.push(name.to_owned());
    } else {
        inner = indent.to_owned();
    }

    match &node.ty {
        Ty::RustEnum(_) => {
            let type_vars: Vec<String> = match &c.body {
                MM::ClassDef::Parts { type_vars, .. } => type_vars.clone(),
                _ => vec![],
            };
            let type_params = if type_vars.is_empty() { String::new() } else { format!("<{}>", type_vars.join(", ")) };
            let mut emitted_variants: Vec<String> = Vec::new();
            writeln!(out, "{inner}#[derive(Clone, Debug, PartialEq, Eq, Hash)]").unwrap();
            writeln!(out, "{inner}pub enum {ename}{type_params} {{").unwrap();
            for rec_name in &records_in_order(c) {
                let Some(rec_node) = node.children.get(rec_name) else { continue };
                let NodeKind::Class(rc) = &rec_node.kind else { continue };
                match &rec_node.ty {
                    Ty::RustUnitVariant => {
                        writeln!(out, "{inner}    {rec_name},").unwrap();
                        emitted_variants.push(rec_name.clone());
                    }
                    Ty::RustStruct(_) => {
                        let fields = component_fields(rc, &rec_node.children);
                        if fields.is_empty() {
                            writeln!(out, "{inner}    {rec_name},").unwrap();
                        } else {
                            writeln!(out, "{inner}    {rec_name} {{").unwrap();
                            for (fname, fty) in &fields {
                                writeln!(out, "{inner}        {}: {},", escape_ident(fname), fmt_ty(fty, &mut *ctx)).unwrap();
                            }
                            writeln!(out, "{inner}    }},").unwrap();
                        }
                        emitted_variants.push(rec_name.clone());
                    }
                    _ => {
                        writeln!(out, "{inner}    {rec_name}, // unresolved").unwrap();
                        emitted_variants.push(rec_name.clone());
                    }
                }
            }
            writeln!(out, "{inner}}}").unwrap();
            let variant_list = emitted_variants.join(",");
            writeln!(out, "{inner}pub use {ename}::{{{variant_list}}};").unwrap();
        }
        Ty::AliasTo(_) => {
            // Single-record uniontype: emit one struct named after the uniontype.
            // The record's Ty::RustStruct was updated in seed_metarecords to carry the
            // uniontype's qname, so all constructor/pattern references already resolve here.
            let recs = records_in_order(c);
            let rec_name = recs.into_iter().next().unwrap_or_default();
            if let Some(rec_node) = node.children.get(&rec_name) {
                if let NodeKind::Class(rc) = &rec_node.kind {
                    emit_struct(out, name, rec_node, rc, &inner, &mut *ctx);
                }
            }
        }
        _ => {
            // No records — emit an opaque struct with PhantomData for type params.
            let type_vars: Vec<String> = match &c.body {
                MM::ClassDef::Parts { type_vars, .. } => type_vars.clone(),
                _ => vec![],
            };
            if type_vars.is_empty() {
                writeln!(out, "{inner}pub struct {ename};").unwrap();
            } else {
                let params = type_vars.join(", ");
                let phantom = if type_vars.len() == 1 {
                    type_vars[0].clone()
                } else {
                    format!("({})", params)
                };
                writeln!(out, "{inner}pub struct {ename}<{params}>(std::marker::PhantomData<{phantom}>);").unwrap();
            }
        }
    }

    // Emit function children in declaration order.
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => &[],
    };
    for member in members {
        if let MM::ClassMember::ClassDef(cdm) = member {
            if let Some(child_node) = node.children.get(&cdm.class_def.name) {
                if let NodeKind::Class(child_class) = &child_node.kind {
                    match &cdm.class_def.restriction {
                        Absyn::Restriction::R_FUNCTION { .. } => {
                            emit_function(out, &cdm.class_def.name, child_node, child_class, &inner, &mut *ctx, top_level);
                        }
                        Absyn::Restriction::R_TYPE | Absyn::Restriction::R_ENUMERATION => {
                            emit_type_item(out, &cdm.class_def.name, child_node, child_class, &inner, &mut *ctx);
                        }
                        Absyn::Restriction::R_UNIONTYPE => {
                            emit_uniontype(out, &cdm.class_def.name, child_node, child_class, &inner, &mut *ctx, top_level);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if wrap_in_mod {
        ctx.current_path.pop();
        writeln!(out, "{indent}}}").unwrap();
    }
    writeln!(out).unwrap();
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
                writeln!(out, "{indent}#[derive(Clone, Debug, PartialEq, Eq, Hash)]").unwrap();
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

fn emit_function<'a>(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
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

    let ret_ty = fmt_ty(fn_output, ctx);
    let ename = escape_ident(name);

    let pub_kw = if node.visibility == MM::Visibility::Public { "pub " } else { "" };
    if c.partial_prefix {
        let type_only_params = fn_inputs.iter()
            .map(|inp| fmt_ty(&inp.ty, ctx))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(out, "{indent}{pub_kw}type {ename}{type_params} = fn({type_only_params}) -> Result<{ret_ty}>;").unwrap();
        writeln!(out).unwrap();
        return;
    }

    // Walk components to find outputs (with names) and protected locals.
    let mut outputs: Vec<(String, Ty, Option<Absyn::Modification>)> = Vec::new();
    let mut protected: Vec<(String, Ty, Option<Absyn::Modification>)> = Vec::new();
    let mut input_names: HashSet<String> = HashSet::new();
    for inp in fn_inputs.iter() { input_names.insert(inp.name.clone()); }
    for member in members {
        let MM::ClassMember::Component(cm) = member else { continue };
        let child_ty = node.children.get(&cm.name).map(|n| n.ty.clone()).unwrap_or(Ty::Unknown);
        match cm.direction {
            Absyn::Direction::OUTPUT | Absyn::Direction::INPUT_OUTPUT =>
                outputs.push((cm.name.clone(), child_ty, cm.modification.clone())),
            Absyn::Direction::BIDIR => {
                if !input_names.contains(&cm.name) {
                    protected.push((cm.name.clone(), child_ty, cm.modification.clone()));
                }
            }
            _ => {}
        }
    }

    let pkg_prefix = if ctx.current_path.is_empty() {
        ctx.top_name.clone()
    } else {
        format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
    };

    let mut infer_env: HashMap<String, Ty> = HashMap::new();
    for inp in fn_inputs.iter() { infer_env.insert(inp.name.clone(), inp.ty.clone()); }
    for (n, t, _) in &outputs { infer_env.insert(n.clone(), t.clone()); }
    for (n, t, _) in &protected { infer_env.insert(n.clone(), t.clone()); }

    let alg_items: &[Absyn::AlgorithmItem] = match &c.body {
        MM::ClassDef::Parts { algorithms, .. } | MM::ClassDef::ClassExtends { algorithms, .. } => algorithms,
        _ => &[],
    };

    let typed_stmts = typedexp::infer_stmts(alg_items, &mut infer_env, top_level, &pkg_prefix);

    let mut env = LocalEnv::default();
    for inp in fn_inputs.iter() { env.vars.insert(inp.name.clone(), inp.ty.clone()); }
    for (n, t, _) in &outputs   { env.vars.insert(n.clone(), t.clone()); }
    for (n, t, _) in &protected { env.vars.insert(n.clone(), t.clone()); }

    writeln!(out, "{indent}{pub_kw}fn {ename}{type_params}({params}) -> Result<{ret_ty}> {{").unwrap();
    let body_indent = format!("{indent}    ");

    for (n, t, modif) in outputs.iter().chain(protected.iter()) {
        let ty_s = fmt_ty(t, ctx);
        let modif_opt: Option<Absyn::Modification> = modif.clone();
        let init = extract_default_exp(&modif_opt).map(|exp| {
            let typed = typedexp::infer_exp(exp, &infer_env, top_level, &pkg_prefix);
            emit_exp(&typed, false, ctx)
        });
        match init {
            Some(s) => writeln!(out, "{body_indent}let mut {}: {ty_s} = {s};", escape_ident(n)).unwrap(),
            None    => writeln!(out, "{body_indent}let mut {}: {ty_s};", escape_ident(n)).unwrap(),
        }
    }

    let mut fresh: u32 = 0;
    match &c.body {
        MM::ClassDef::Parts { external: Some(ext), .. } => {
            writeln!(out, "{body_indent}todo!(); // {:?}", ext).unwrap();
        },
        _ => emit_stmts(out, &body_indent, &typed_stmts, FailureMode::Function, ctx, &mut env, top_level, &mut fresh)
    };

    match outputs.len() {
        0 => writeln!(out, "{body_indent}Ok(())").unwrap(),
        1 => writeln!(out, "{body_indent}Ok({})", escape_ident(&outputs[0].0)).unwrap(),
        _ => {
            let parts: Vec<String> = outputs.iter().map(|(n, _, _)| escape_ident(n)).collect();
            writeln!(out, "{body_indent}Ok(({}))", parts.join(", ")).unwrap();
        }
    }
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();
}

// ── Expression and pattern emission ──────────────────────────────────────────

fn emit_exp(exp: &TypedExp, is_const: bool, ctx: &mut GenCtx) -> String {
    match exp {
        TypedExp::Lit(Lit::Int(v))  => v.to_string(),
        TypedExp::Lit(Lit::Real(v)) => v.clone(),
        TypedExp::Lit(Lit::Str(v))  => format!("\"{v}\".to_string()"),
        TypedExp::Lit(Lit::Bool(v)) => v.to_string(),

        TypedExp::Var { name, .. } => {
            let res = if name.contains('.') { ctx.shorten(name) } else { name.clone() };
            escape_ident(&res)
        }

        TypedExp::BinOp { op, lhs, rhs, ty, .. } => {
            let l = emit_exp(lhs, is_const, ctx);
            let r = emit_exp(rhs, is_const, ctx);
            match op {
                BinOpKind::Eq => {
                    if is_const && (lhs.ty() == Ty::Str || rhs.ty() == Ty::Str) {
                        return format!("const_str::equal!({l},{r})");
                    }
                    format!("{l} == {r}")
                }
                BinOpKind::NEq => format!("{l} != {r}"),
                BinOpKind::Lt  => format!("{l} < {r}"),
                BinOpKind::LEq => format!("{l} <= {r}"),
                BinOpKind::Gt  => format!("{l} > {r}"),
                BinOpKind::GEq => format!("{l} >= {r}"),
                BinOpKind::Add if *ty == Ty::Str => {
                    // Collect all string parts from a chain of Add ops and emit a single join operation.
                    let mut parts: Vec<String> = Vec::new();
                    collect_string_concat_parts(exp, is_const, ctx, &mut parts);
                    let args = parts.join(", ");
                    format!("[{args}].join(\"\")")
                }
                BinOpKind::Add => format!("{l} + {r}"),
                BinOpKind::Sub => format!("{l} - {r}"),
                BinOpKind::Mul => format!("{l} * {r}"),
                BinOpKind::Div => format!("{l} / {r}"),
                BinOpKind::And => format!("{l} && {r}"),
                BinOpKind::Or  => format!("{l} || {r}"),
            }
        }

        TypedExp::UnOp { op, operand, .. } => {
            let s = emit_exp(operand, is_const, ctx);
            match op {
                UnOpKind::Neg => format!("-{s}"),
                UnOpKind::Not => format!("!{s}"),
            }
        }

        TypedExp::Call { func, args, named_args, .. } => {
            match func.as_str() {
                "SOME" => {
                    let arg = args.first().map(|a| emit_exp(a, is_const, ctx)).unwrap_or_default();
                    format!("Some({arg})")
                }
                "NONE" => "None".to_owned(),
                "fail" => if is_const { "{ panic!(\"fail\") }".to_owned() } else { "bail!(\"fail\")".to_owned() },
                "list" => {
                    let parts: Vec<String> = args.iter().map(|a| emit_exp(a, is_const, ctx)).collect();
                    format!("metamodelica::list![{}]", parts.join(", "))
                },
                "arrayGet" => {
                    let arg1 = args.first().map(|a| emit_exp(a, is_const, ctx)).unwrap_or_default();
                    let arg2 = args.get(1).map(|a| emit_exp(a, is_const, ctx)).unwrap_or_default();
                    format!("{}[{}-1]", arg1, arg2)
                },
                "arrayLength" => {
                    let arg = args.first().map(|a| emit_exp(a, is_const, ctx)).unwrap_or_default();
                    format!("{}.len() as i32", arg)
                },
                "listEmpty" => {
                    let arg = args.first().map(|a| emit_exp(a, is_const, ctx)).unwrap_or_default();
                    format!("{}.is_empty()", arg)
                },
                _ => {
                    let func_str = if func.contains('.') {
                        &ctx.shorten(func)
                    } else {
                        func
                    };
                    let func_str = escape_ident(func_str);
                    let mut parts: Vec<String> = args.iter().map(|a| emit_exp(a, is_const, ctx)).collect();
                    for (n, v) in named_args {
                        parts.push(format!("{n}={}", emit_exp(v, is_const, ctx)));
                    }
                    let call = format!("{func_str}({})", parts.join(", "));
                    // Add `?` to propagate Result errors from fallible calls. Skip in const
                    // context, for constructors (uppercase first char), and for known-infallible
                    // builtins (the core arithmetic / comparison / structural ops).
                    if is_const || is_constructor_name(func) || is_infallible_builtin(func) {
                        call
                    } else {
                        format!("{call}?")
                    }
                }
            }
        }

        TypedExp::If { cond, then_, elseif, else_, .. } => {
            let c = emit_exp(cond, is_const, ctx);
            let t = emit_exp(then_, is_const, ctx);
            let e = emit_exp(else_, is_const, ctx);
            let ei: String = elseif.iter()
                .map(|(ec, eb)| format!(" else if ({}) {{{}}}", emit_exp(ec, is_const, ctx), emit_exp(eb, is_const, ctx)))
                .collect();
            format!("if ({c}) {{{t}}}{ei} else {{{e}}}")
        }

        TypedExp::Cons { head, tail, .. } => {
            format!("cons({}, {})", emit_exp(head, is_const, ctx), emit_exp(tail, is_const, ctx))
        }

        TypedExp::Tuple(elems) => {
            let parts: Vec<String> = elems.iter().map(|e| emit_exp(e, is_const, ctx)).collect();
            format!("({})", parts.join(", "))
        }

        TypedExp::Array { elems, .. } => {
            if elems.is_empty() {
                "List::Nil".to_owned()
            } else {
                let parts: Vec<String> = elems.iter().map(|e| emit_exp(e, is_const, ctx)).collect();
                format!("list![{}]", parts.join(", "))
            }
        }

        TypedExp::Match { kind, input, cases, .. } => {
            emit_match(kind, input, cases, is_const, ctx)
        }

        TypedExp::Range { start, step, stop, .. } => {
            emit_range(start, step.as_deref(), stop, is_const, ctx)
        }

        TypedExp::Todo(s) => format!("todo!(/*{}*/)", s.chars().take(60).collect::<String>()),
    }
}

/// Flatten a chain of string `Add` expressions into a list of individual string parts.
/// e.g. `(a + b) + c` → `["a", "b", "c"]`
fn collect_string_concat_parts(exp: &TypedExp, is_const: bool, ctx: &mut GenCtx, parts: &mut Vec<String>) {
    if let TypedExp::BinOp { op: BinOpKind::Add, ty: Ty::Str, lhs, rhs, .. } = exp {
        collect_string_concat_parts(lhs, is_const, ctx, parts);
        collect_string_concat_parts(rhs, is_const, ctx, parts);
    } else {
        parts.push(emit_exp(exp, is_const, ctx));
    }
}

/// Emit a Modelica range expression (`start:step:stop` or `start:stop`) as a Rust iterator.
/// Modelica ranges are arithmetic progressions: start, start+step, ..., while within [start, stop].
/// Positive steps map to `(start..=stop).step_by(n)`.
/// Negative steps reverse the range: `(stop..=start).step_by(-n)`.
fn emit_range(start: &TypedExp, step: Option<&TypedExp>, stop: &TypedExp, is_const: bool, ctx: &mut GenCtx) -> String {
    let s = emit_exp(start, is_const, ctx);
    let e = emit_exp(stop, is_const, ctx);

    match step {
        // `start:stop` — default step of 1, forward.
        None => format!("{s}..={e}"),

        // `start:step:stop` — check the step sign at codegen time.
        Some(step_exp) => {
            // If the step is a known positive literal, use the standard forward form.
            if let TypedExp::Lit(Lit::Int(n)) = step_exp {
                if *n > 0 {
                    return if *n == 1 {
                        format!("{s}..={e}")
                    } else {
                        format!("({s}..={e}).step_by({n})")
                    };
                }
                // Negative step: reverse the range, negate the step.
                if *n < 0 {
                    return format!("({e}..={s}).step_by({}).rev()", -n);
                }
            }

            let step_val = emit_exp(step_exp, is_const, ctx);

            // Dynamic step: positive path for the common case,
            // with a runtime branch that reverses for negative steps.
            format!(
                "({{let __s={s}; let __e={e}; let __step={step_val}; if __step>0 {{__s..=__e}} else {{__e..=__s}}}}).step_by(if {step_val}>0 {{{step_val}}} else {{-({step_val})}})"
            )
        }
    }
}

fn emit_match(kind: &MatchKind, input: &TypedExp, cases: &[TypedCase], is_const: bool, ctx: &mut GenCtx) -> String {
    let input_str = emit_exp(input, is_const, ctx);
    match kind {
        MatchKind::Match => {
            let has_wild = cases.iter().any(|c| matches!(c.pattern, TypedPat::Wildcard) && c.guard.is_none());
            let arms: Vec<String> = cases.iter().map(|case| {
                let pat = emit_pat(&case.pattern, ctx);
                let guard = case.guard.as_ref()
                    .map(|g| format!(" if {}", emit_exp(g, is_const, ctx)))
                    .unwrap_or_default();
                let result = emit_exp(&case.result, is_const, ctx);
                format!("        {pat}{guard} => {result}")
            }).collect();
            let fallback = if has_wild { String::new() } else {
                ",\n        _ => bail!(\"match: no arm matched\")".to_owned()
            };
            format!(
                "(match {input_str} {{\n{}{fallback},\n    }})",
                arms.join(",\n"),
            )
        }
        MatchKind::MatchContinue => {
            // Each arm is an IIFE returning anyhow::Result<T>; first Ok wins.
            // Failures inside an arm (pattern mismatch, ?-propagated errors) drop to next arm.
            let mut s = String::new();
            s.push_str("'mc: {\n");
            s.push_str(&format!("        let __mc_input = {input_str};\n"));
            for case in cases {
                let pat = emit_pat(&case.pattern, ctx);
                let guard_check = case.guard.as_ref()
                    .map(|g| format!("            if !({}) {{ bail!(\"guard\") }}\n", emit_exp(g, is_const, ctx)))
                    .unwrap_or_default();
                let result = emit_exp(&case.result, is_const, ctx);
                s.push_str("        if let Ok(__v) = (|| -> Result<_> {\n");
                s.push_str(&format!("            let {pat} = __mc_input.clone() else {{ bail!(\"nomatch\") }};\n"));
                s.push_str(&guard_check);
                s.push_str(&format!("            Ok({result})\n"));
                s.push_str("        })() { break 'mc __v; }\n");
            }
            s.push_str("        bail!(\"matchcontinue: no arm matched\")\n");
            s.push_str("    }");
            s
        }
    }
}

fn emit_pat(pat: &TypedPat, ctx: &mut GenCtx) -> String {
    match pat {
        TypedPat::Wildcard    => "_".to_owned(),
        TypedPat::Var(name)   => escape_ident(name),
        TypedPat::EmptyList   => "metamodelica::List::Nil".to_owned(),
        TypedPat::Some_(inner) => format!("Some({})", emit_pat(inner, ctx)),
        TypedPat::None_       => "None".to_owned(),

        TypedPat::Lit(Lit::Int(v))  => {
            if *v < 0 { format!("({v})") } else { v.to_string() }
        }
        TypedPat::Lit(Lit::Bool(v)) => v.to_string(),
        TypedPat::Lit(Lit::Str(_))  => "_ /* string — move to guard */".to_owned(),
        TypedPat::Lit(Lit::Real(_)) => "_ /* real — move to guard */".to_owned(),

        TypedPat::Cons { head, tail } => {
            format!("metamodelica::List::Cons {{ head: {}, tail: {} }}",
                emit_pat(head, ctx), emit_pat(tail, ctx))
        }

        TypedPat::Tuple(pats) => {
            let parts: Vec<String> = pats.iter().map(|p| emit_pat(p, ctx)).collect();
            format!("({})", parts.join(", "))
        }

        TypedPat::Constructor { name, fields, named_fields, .. } => {
            let rust = if name.contains('.') { ctx.shorten(name) } else { escape_ident(name) };
            if named_fields.is_empty() && fields.is_empty() {
                rust
            } else if named_fields.is_empty() {
                let pats: Vec<String> = fields.iter().map(|p| emit_pat(p, ctx)).collect();
                format!("{rust}({})", pats.join(", "))
            } else {
                let pats: Vec<String> = named_fields.iter()
                    .map(|(fname, p)| {
                        let pstr = emit_pat(p, ctx);
                        // Use field shorthand when the binding name matches the field name.
                        if matches!(p, TypedPat::Var(v) if v == fname) {
                            escape_ident(fname)
                        } else {
                            format!("{}: {pstr}", escape_ident(fname))
                        }
                    })
                    .collect();
                format!("{rust} {{ {} }}", pats.join(", "))
            }
        }

        TypedPat::As { var, pat } => {
            format!("{} @ {}", escape_ident(var), emit_pat(pat, ctx))
        }

        TypedPat::Todo(s) => format!("_ /* todo: {} */", s.chars().take(40).collect::<String>()),
    }
}

// ── Statement emission ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum FailureMode {
    /// Top-level body: pattern mismatch / `fail()` becomes `bail!(...)` which
    /// returns Err from the enclosing function.
    Function,
    /// Inside a try arm or matchcontinue arm IIFE — bail propagates to that
    /// closure boundary, which the dispatcher catches.
    TryArm,
    /// Inside `failure(...)` body — success of the body is itself a failure;
    /// bodies still emit normally, the inversion happens at the `Failure` site.
    Failure,
}

#[derive(Debug, Clone, Default)]
struct LocalEnv {
    vars: HashMap<String, Ty>,
}

fn is_constructor_name(func: &str) -> bool {
    let last = func.rsplit('.').next().unwrap_or(func);
    last.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn is_infallible_builtin(func: &str) -> bool {
    matches!(func,
        "intAdd" | "intSub" | "intMul" | "intDiv" | "intMod" | "intAbs"
        | "intMax" | "intMin" | "intNeg" | "intBitAnd" | "intBitOr" | "intBitXor"
        | "intBitNot" | "intBitLShift" | "intBitRShift" | "intReal" | "intString"
        | "realAdd" | "realSub" | "realMul" | "realDiv" | "realMod" | "realPow"
        | "realAbs" | "realMax" | "realMin" | "realNeg" | "realFloor" | "realCeil"
        | "realInt" | "realString"
        | "boolAnd" | "boolOr" | "boolNot" | "boolEq" | "boolString"
        | "intEq" | "intNe" | "intLt" | "intLe" | "intGt" | "intGe"
        | "realEq" | "realNe" | "realLt" | "realLe" | "realGt" | "realGe"
        | "stringEq" | "stringEqual" | "stringCompare" | "stringHash" | "stringHashDjb2"
        | "stringLength" | "stringAppend" | "stringAppendList" | "anyString"
        | "referenceEq" | "valueEq" | "isEmpty" | "isSome" | "isNone"
        | "listLength" | "listEmpty" | "listMember" | "listAppend"
        | "listReverse" | "listHead" | "listFirst" | "listRest" | "listTail"
        | "arrayLength" | "arrayCreate" | "arrayGet" | "arrayUpdate" | "arrayCopy"
        | "print" | "printError"
    )
}

fn pat_is_irrefutable(pat: &TypedPat) -> bool {
    match pat {
        TypedPat::Wildcard | TypedPat::Var(_) => true,
        TypedPat::Tuple(ps) => ps.iter().all(pat_is_irrefutable),
        TypedPat::As { pat, .. } => pat_is_irrefutable(pat),
        _ => false,
    }
}

/// Look up the field types of a record/metarecord by qualified name.
/// Returns Vec of (field_name, field_ty) in declaration order.
fn record_field_tys<'a>(
    qname: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Vec<(String, Ty)> {
    let Some(node) = lookup_node(qname, top_level) else { return vec![] };
    let NodeKind::Class(c) = &node.kind else { return vec![] };
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return vec![],
    };
    members.iter().filter_map(|m| {
        let MM::ClassMember::Component(cm) = m else { return None };
        let child = node.children.get(&cm.name)?;
        Some((cm.name.clone(), child.ty.clone()))
    }).collect()
}

/// Is this Ty stored behind an Arc due to recursion-cycle breaking?
fn is_arc_wrapped(ty: &Ty, ctx: &GenCtx) -> bool {
    let qname = match ty {
        Ty::RustStruct(n) | Ty::RustEnum(n) | Ty::AliasTo(n) | Ty::ExternalObject(n) => n.as_str(),
        Ty::Generic(name, _) => return ctx.recursive_types.contains(&name.replace("::", ".")),
        _ => return false,
    };
    ctx.recursive_types.contains(qname)
}

/// Emit an irrefutable pattern binding `let pat = expr;` if pat is irrefutable, else
/// `let pat = expr else { <fail> };`. Then process any deferred Arc-edge sub-patterns.
/// Returns the emitted source.
fn emit_pat_assign<'a>(
    out: &mut String,
    indent: &str,
    pat: &TypedPat,
    scrut_ty: &Ty,
    scrut_expr: &str,
    fail_mode: FailureMode,
    ctx: &mut GenCtx,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    fresh: &mut u32,
) {
    match pat {
        TypedPat::Wildcard => {
            // Evaluate for side effects but discard.
            writeln!(out, "{indent}let _ = {scrut_expr};").unwrap();
        }
        TypedPat::Var(name) => {
            env.vars.insert(name.clone(), scrut_ty.clone());
            writeln!(out, "{indent}let {} = {scrut_expr};", escape_ident(name)).unwrap();
        }
        _ => {
            // Refutable: render shallow with deferrals for Arc-edge crossings.
            let mut deferrals: Vec<(String, TypedPat, Ty)> = Vec::new();
            let surface = render_shallow(pat, scrut_ty, ctx, env, top_level, fresh, &mut deferrals);
            let fail = match fail_mode {
                FailureMode::Function | FailureMode::TryArm => "bail!(\"pattern mismatch\")",
                FailureMode::Failure => "()",
            };
            writeln!(out, "{indent}let {surface} = ({scrut_expr}) else {{ {fail} }};").unwrap();
            for (sub_expr, sub_pat, sub_ty) in deferrals {
                emit_pat_assign(out, indent, &sub_pat, &sub_ty, &sub_expr, fail_mode, ctx, env, top_level, fresh);
            }
        }
    }
}

/// Render a pattern with shallow Arc-edge crossing: any Constructor field whose
/// type is Arc-wrapped AND whose pattern is non-trivial gets replaced with a fresh
/// binding `__t<N>`, and a `(*__tN).clone()` deferral is queued for follow-up lowering.
fn render_shallow<'a>(
    pat: &TypedPat,
    scrut_ty: &Ty,
    ctx: &mut GenCtx,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    fresh: &mut u32,
    deferrals: &mut Vec<(String, TypedPat, Ty)>,
) -> String {
    match pat {
        TypedPat::Wildcard => "_".to_owned(),
        TypedPat::Var(name) => {
            env.vars.insert(name.clone(), scrut_ty.clone());
            escape_ident(name)
        }
        TypedPat::EmptyList => "metamodelica::List::Nil".to_owned(),
        TypedPat::None_ => "None".to_owned(),
        TypedPat::Some_(inner) => {
            let inner_ty = match scrut_ty {
                Ty::Option(t) => (**t).clone(),
                _ => Ty::Unknown,
            };
            let inner_s = render_shallow(inner, &inner_ty, ctx, env, top_level, fresh, deferrals);
            format!("Some({inner_s})")
        }
        TypedPat::Lit(Lit::Int(v)) => if *v < 0 { format!("({v})") } else { v.to_string() },
        TypedPat::Lit(Lit::Bool(v)) => v.to_string(),
        TypedPat::Lit(_) => "_ /* lit — guard not yet implemented */".to_owned(),
        TypedPat::Cons { head, tail } => {
            let elem_ty = match scrut_ty { Ty::List(t) => (**t).clone(), _ => Ty::Unknown };
            let h = render_shallow(head, &elem_ty, ctx, env, top_level, fresh, deferrals);
            let t = render_shallow(tail, scrut_ty, ctx, env, top_level, fresh, deferrals);
            format!("metamodelica::List::Cons {{ head: {h}, tail: {t} }}")
        }
        TypedPat::Tuple(pats) => {
            let tys: Vec<Ty> = match scrut_ty {
                Ty::Tuple(ts) if ts.len() == pats.len() => ts.clone(),
                _ => vec![Ty::Unknown; pats.len()],
            };
            let parts: Vec<String> = pats.iter().zip(tys.iter())
                .map(|(p, t)| render_shallow(p, t, ctx, env, top_level, fresh, deferrals))
                .collect();
            format!("({})", parts.join(", "))
        }
        TypedPat::Constructor { name, fields, named_fields, .. } => {
            // Field types: look up the record by qname.
            let resolved_qname = if name.contains('.') {
                Some(name.clone())
            } else {
                // Bare constructor: try current pkg prefix and ancestors.
                let cur_prefix = if ctx.current_path.is_empty() {
                    ctx.top_name.clone()
                } else {
                    format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
                };
                let mut scope: &str = &cur_prefix;
                let mut found = None;
                loop {
                    let q = format!("{scope}.{name}");
                    if lookup_node(&q, top_level).is_some() { found = Some(q); break; }
                    match scope.rfind('.') {
                        Some(d) => scope = &scope[..d],
                        None => { if lookup_node(name, top_level).is_some() { found = Some(name.clone()); } break; }
                    }
                }
                found
            };
            let field_tys: Vec<(String, Ty)> = resolved_qname
                .as_deref()
                .map(|q| record_field_tys(q, top_level))
                .unwrap_or_default();
            let rust_ctor = if name.contains('.') { ctx.shorten(name) } else { escape_ident(name) };

            // Helper: render one sub-pattern, splitting on Arc edge.
            let mut handle = |sub: &TypedPat, fty: &Ty,
                              ctx: &mut GenCtx, env: &mut LocalEnv,
                              fresh: &mut u32, deferrals: &mut Vec<(String, TypedPat, Ty)>| -> String {
                if is_arc_wrapped(fty, ctx) && !matches!(sub, TypedPat::Wildcard | TypedPat::Var(_)) {
                    let n = *fresh; *fresh += 1;
                    let tmp = format!("__t{n}");
                    deferrals.push((format!("(*{tmp}).clone()"), sub.clone(), fty.clone()));
                    tmp
                } else {
                    render_shallow(sub, fty, ctx, env, top_level, fresh, deferrals)
                }
            };

            if !named_fields.is_empty() {
                let parts: Vec<String> = named_fields.iter().map(|(fname, sp)| {
                    let fty = field_tys.iter().find(|(n, _)| n == fname).map(|(_, t)| t.clone()).unwrap_or(Ty::Unknown);
                    let s = handle(sp, &fty, ctx, env, fresh, deferrals);
                    if matches!(sp, TypedPat::Var(v) if v == fname) {
                        escape_ident(fname)
                    } else {
                        format!("{}: {s}", escape_ident(fname))
                    }
                }).collect();
                format!("{rust_ctor} {{ {} }}", parts.join(", "))
            } else if !fields.is_empty() {
                let parts: Vec<String> = fields.iter().enumerate().map(|(i, sp)| {
                    let fty = field_tys.get(i).map(|(_, t)| t.clone()).unwrap_or(Ty::Unknown);
                    handle(sp, &fty, ctx, env, fresh, deferrals)
                }).collect();
                format!("{rust_ctor}({})", parts.join(", "))
            } else {
                rust_ctor
            }
        }
        TypedPat::As { var, pat: inner } => {
            env.vars.insert(var.clone(), scrut_ty.clone());
            let inner_s = render_shallow(inner, scrut_ty, ctx, env, top_level, fresh, deferrals);
            format!("{} @ {}", escape_ident(var), inner_s)
        }
        TypedPat::Todo(s) => format!("_ /* todo: {} */", s.chars().take(40).collect::<String>()),
    }
}

fn emit_stmts<'a>(
    out: &mut String,
    indent: &str,
    stmts: &[typedexp::TypedStmt],
    fail_mode: FailureMode,
    ctx: &mut GenCtx,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    fresh: &mut u32,
) {
    for s in stmts {
        emit_stmt(out, indent, s, fail_mode, ctx, env, top_level, fresh);
    }
}

fn emit_stmt<'a>(
    out: &mut String,
    indent: &str,
    stmt: &typedexp::TypedStmt,
    fail_mode: FailureMode,
    ctx: &mut GenCtx,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    fresh: &mut u32,
) {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { lhs, rhs } => {
            let scrut_ty = rhs.ty();
            let scrut_expr = emit_exp(rhs, /*is_const=*/false, ctx);
            // For irrefutable patterns we still want a single binding form.
            // But MetaModelica often assigns to *existing* variables (declared as outputs
            // or `protected` components). For a Var pattern we emit a plain `<name> = expr;`
            // when the binding is already in scope, else a `let`. Heuristic: if env has it,
            // it's an output or earlier protected — emit assignment.
            if let TypedPat::Var(name) = lhs {
                if env.vars.contains_key(name) {
                    writeln!(out, "{indent}{} = {scrut_expr};", escape_ident(name)).unwrap();
                    return;
                }
            }
            emit_pat_assign(out, indent, lhs, &scrut_ty, &scrut_expr, fail_mode, ctx, env, top_level, fresh);
        }
        S::NoRetCall { call } => {
            let s = emit_exp(call, false, ctx);
            writeln!(out, "{indent}let _ = {s};").unwrap();
        }
        S::If { cond, then_, elseif, else_ } => {
            let c = emit_exp(cond, false, ctx);
            writeln!(out, "{indent}if {c} {{").unwrap();
            emit_stmts(out, &format!("{indent}    "), then_, fail_mode, ctx, env, top_level, fresh);
            for (ec, eb) in elseif {
                let cs = emit_exp(ec, false, ctx);
                writeln!(out, "{indent}}} else if {cs} {{").unwrap();
                emit_stmts(out, &format!("{indent}    "), eb, fail_mode, ctx, env, top_level, fresh);
            }
            if !else_.is_empty() {
                writeln!(out, "{indent}}} else {{").unwrap();
                emit_stmts(out, &format!("{indent}    "), else_, fail_mode, ctx, env, top_level, fresh);
            }
            writeln!(out, "{indent}}}").unwrap();
        }
        S::For { var, range, body } => {
            let r = emit_exp(range, false, ctx);
            writeln!(out, "{indent}for {} in {r} {{", escape_ident(var)).unwrap();
            // Element type: peel List/Array.
            let elem_ty = match range.ty() { Ty::List(t) | Ty::Array(t) => *t, _ => Ty::Unknown };
            let mut inner = env.clone();
            inner.vars.insert(var.clone(), elem_ty);
            emit_stmts(out, &format!("{indent}    "), body, fail_mode, ctx, &mut inner, top_level, fresh);
            writeln!(out, "{indent}}}").unwrap();
        }
        S::While { cond, body } => {
            let c = emit_exp(cond, false, ctx);
            writeln!(out, "{indent}while {c} {{").unwrap();
            emit_stmts(out, &format!("{indent}    "), body, fail_mode, ctx, env, top_level, fresh);
            writeln!(out, "{indent}}}").unwrap();
        }
        S::Try { body, else_body } => {
            // Run `body` as an IIFE; on Err run else_body.
            writeln!(out, "{indent}if (|| -> Result<()> {{").unwrap();
            let mut benv = env.clone();
            emit_stmts(out, &format!("{indent}    "), body, FailureMode::TryArm, ctx, &mut benv, top_level, fresh);
            writeln!(out, "{indent}    Ok(())").unwrap();
            writeln!(out, "{indent}}})().is_err() {{").unwrap();
            let mut eenv = env.clone();
            emit_stmts(out, &format!("{indent}    "), else_body, fail_mode, ctx, &mut eenv, top_level, fresh);
            writeln!(out, "{indent}}}").unwrap();
        }
        S::Failure { body } => {
            // Body is *expected* to fail. If it runs cleanly, that itself is a failure.
            writeln!(out, "{indent}if (|| -> Result<()> {{").unwrap();
            let mut fenv = env.clone();
            emit_stmts(out, &format!("{indent}    "), body, FailureMode::TryArm, ctx, &mut fenv, top_level, fresh);
            writeln!(out, "{indent}    Ok(())").unwrap();
            writeln!(out, "{indent}}})().is_ok() {{ bail!(\"failure(): body succeeded\") }}").unwrap();
        }
        S::Return   => writeln!(out, "{indent}/* return */ todo!(\"return-with-outputs not yet wired\");").unwrap(),
        S::Break    => writeln!(out, "{indent}break;").unwrap(),
        S::Continue => writeln!(out, "{indent}continue;").unwrap(),
        S::Todo(s)  => writeln!(out, "{indent}/* todo stmt: {} */", s.chars().take(60).collect::<String>()).unwrap(),
    }
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
        Ty::Enumeration(name) => ctx.shorten(name),
        Ty::RustStruct(name) => {
            let short = ctx.shorten(name);
            if ctx.recursive_types.contains(name.as_str()) {
                format!("Arc<{short}>")
            } else {
                short
            }
        }
        Ty::RustEnum(name) | Ty::AliasTo(name) => {
            // All unittypes are wrapped in `pub mod <Name>`, so the type lives at
            // `ModName::TypeName`. Apply `::TypeName` doubling unless we are currently
            // emitting inside that mod (current_path ends with the simple name).
            // Top-level-file unittypes have a single-component qname (e.g. "List");
            // nested ones have a dotted qname (e.g. "LexerJSON.Token").
            let first = name.split('.').next().unwrap_or(name);
            let last = name.rsplit('.').next().unwrap_or(name);
            let shortened = ctx.shorten(name);
            let in_own_mod = ctx.current_path.last().map(|p| p == last).unwrap_or(false);
            let needs_doubling = !in_own_mod && !ctx.no_mod_uniontypes.contains(name.as_str()) && (
                (ctx.top_level_uniontype_names.contains(first) && first != ctx.top_name) ||
                // Nested uniontype: dotted qname AND first != last. The first == last case
                // (e.g. "IOStream.IOStream") means a package and its same-named uniontype —
                // emit_uniontype skips the inner `pub mod` for those, so no extra segment.
                (name.contains('.') && first != last)
            );
            let base = if needs_doubling {
                format!("{shortened}::{last}")
            } else {
                shortened
            };
            if ctx.recursive_types.contains(name.as_str()) {
                format!("Arc<{base}>")
            } else {
                base
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
            /*let tvs = if type_vars.is_empty() {
                String::new()
            } else {
                format!("<{}>", type_vars.join(", "))
            };*/ // The type variables are already included in the function type alias or item signature, so we don't need to repeat them here.
            let ins = inputs.iter().map(|inp| fmt_ty(&inp.ty, ctx)).collect::<Vec<_>>().join(", ");
            format!("fn({ins}) -> {}", fmt_ty(output, ctx))
        }
        Ty::FunctionAlias { base, .. } => {
            let short = ctx.shorten(base);
            // Look up the base function's type vars (e.g. ["T"] for toStringT<T>) so we can
            // emit the correct generic arguments when this alias is used as a parameter type.
            let cur_prefix = if ctx.current_path.is_empty() {
                ctx.top_name.clone()
            } else {
                format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
            };
            let tvs: Option<&Vec<String>> = if base.contains('.') {
                ctx.fn_type_vars.get(base.as_str())
            } else {
                // Scope-walk from current module up to find the base function.
                let mut result = None;
                let mut scope: &str = &cur_prefix;
                loop {
                    let qualified = format!("{scope}.{base}");
                    if let Some(v) = ctx.fn_type_vars.get(qualified.as_str()) {
                        result = Some(v);
                        break;
                    }
                    match scope.rfind('.') {
                        Some(dot) => scope = &scope[..dot],
                        None => break,
                    }
                }
                result
            };
            match tvs {
                Some(tvs) if !tvs.is_empty() => format!("{short}<{}>", tvs.join(", ")),
                _ => short,
            }
        }
        Ty::Generic(name, args) => {
            // `name` is already in Rust form (dots replaced with ::) by ty_rust_name.
            // Convert back to dotted form so shorten() can produce a context-relative
            // path, then apply the same uniontype doubling as for RustEnum/AliasTo.
            let dotted = name.replace("::", ".");
            let first = dotted.split('.').next().unwrap_or(&dotted);
            let last = dotted.rsplit('.').next().unwrap_or(&dotted);
            let shortened = ctx.shorten(&dotted);
            let in_own_mod = ctx.current_path.last().map(|p| p == last).unwrap_or(false);
            let needs_doubling = !in_own_mod && !ctx.no_mod_uniontypes.contains(dotted.as_str()) && (
                (ctx.top_level_uniontype_names.contains(first) && first != ctx.top_name) ||
                (dotted.contains('.') && first != last)
            );
            let base = if needs_doubling {
                format!("{shortened}::{last}")
            } else {
                shortened
            };
            let ty = format!("{base}<{}>", args.iter().map(|t| fmt_ty(t, ctx)).collect::<Vec<_>>().join(", "));
            if ctx.recursive_types.contains(dotted.as_str()) {
                format!("Arc<{ty}>")
            } else {
                ty
            }
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
    if let Some(start) = name.find('\'') {
        // Find the next quote relative to the first one.
        // We slice from start + 1 to ensure we find the *next* quote, not the current one.
        if let Some(end_offset) = name[start + 1..].find('\'') {
            // Calculate the absolute index of the second quote
            let end = start + 1 + end_offset;

            // Reconstruct the string:
            // 1. &name[..start]  -> Everything before the first quote
            // 2. &name[end + 1..] -> Everything after the second quote
            // (Using end + 1 removes the second quote character as well.
            //  Use &name[end..] if you want to keep the second quote.)

            let new_name = format!("{}_{}{}", &name[..start], name[start+1..end].replace("'", "").replace(".","_").replace("::","_"), &name[end + 1..]);
            return escape_ident(new_name.as_str());
        };
    };
    if name.starts_with("MetaModelica::Dangerous") {
        return format!("{}", name.replace("MetaModelica::Dangerous", "metamodelica::Dangerous"));
    }
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