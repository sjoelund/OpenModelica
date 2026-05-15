#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write;
use rayon::prelude::*;
use mmwinnow::Absyn;
use crate::MM;
use std::collections::HashMap;
use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind, Ty, extract_default, extract_default_exp, lookup_node, lookup_node_ty, lookup_record_through_unions, uniontype_needs_mod, collect_type_vars_in_ty};
use crate::typedexp::{self, TypedExp, TypedPat, TypedCase, Lit, BinOpKind, UnOpKind, MatchKind, ReductionIter, ReductionIterKind, cref_to_dotted, CrefSegment};
use anyhow::{Result,bail};

// ── Import-aware generation context ──────────────────────────────────────────

const DEFAULT_TRAITS: &str = "Clone + PartialEq";

/// How to propagate a Result error from a fallible sub-expression.
///
/// MetaModelica calls return `Result<T>` in our lowering. The Rust expression
/// we emit to "extract the value or propagate the error" depends on the
/// surrounding syntactic context, because Rust's `?` operator only works
/// inside a function/try-block that returns `Result`.
///
///   - `Function` — the default. The current function returns `Result<_>`,
///     so we can append `?` and let an error bubble out.
///   - `Filter` — inside an `Iterator::filter(|e| <bool>)` closure. The
///     predicate must yield a plain `bool`, so `?` is unavailable. We use
///     `.unwrap()` (panic on Err). We only enter this mode for guard
///     predicates of `for`-comprehensions / reductions, where the predicate
///     is expected to be infallible at runtime.
///   - `TryBlock(label)` — inside a MetaModelica `try`/`else` block lowered
///     to a labeled Rust block. The closure-based IIFE form cannot capture
///     pre-declared `let mut` locals before they are initialised, so we use
///     a labeled `'l: { ... }` and `unwrap_break_err!(expr, 'l)` instead of
///     `expr?`. The macro evaluates `expr`; on `Err(e)` it breaks the block
///     with `Err(e)`, on `Ok(v)` it yields `v` — playing the same role as
///     `?` but exiting the labeled block rather than the function.
#[derive(Clone, Debug)]
enum QMode {
    Function,
    Filter,
    TryBlock(String),
    /// Emit the call expression itself without any error-propagation wrapper,
    /// returning the raw `Result<T, E>`. Used in `if let Ok(PAT) = CALL { … }`
    /// conditions for the single-statement try optimisation.
    Bare,
}

struct GenCtx {
    /// Name of the top-level class being generated (e.g. "Absyn").
    top_name: String,
    /// Current module path within the top-level file (e.g. `["Connect"]` when
    /// inside `mod Connect`). Used by `shorten` to produce context-relative paths.
    current_path: Vec<String>,
    /// Modules imported with `.*`; their types are referenced by bare name.
    unqual_modules: HashSet<String>,
    /// Explicit imports: dotted qualified name → local name.""
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
    /// Fully-qualified names of struct/enum/uniontype types that transitively
    /// embed a `Mutable<T>` field. Such types cannot derive `PartialEq` / `Eq`
    /// / `Hash` because `Mutex<T>` doesn't implement those traits. Populated
    /// from `InstanceHierarchy::types_containing_mutable`.
    types_containing_mutable: BTreeSet<String>,
    /// Fully-qualified names of nested uniontypes that are NOT wrapped in a `pub mod`
    /// (because they contain only records). These must NOT get the `::TypeName` doubling
    /// that mod-wrapped uniontypes require.
    no_mod_uniontypes: HashSet<String>,
    /// Maps fully-qualified function/partial-function names to their effective type variables
    /// (collected from inputs and output). Used at codegen time to emit generic arguments when
    /// a FunctionAlias type is referenced as a parameter type (e.g. `toStringT` → `toStringT<T>`).
    fn_type_vars: BTreeMap<String, Vec<String>>,
    /// How `?` should be lowered for fallible calls emitted from inside the
    /// expression we are currently generating. See [`QMode`]. Callers that
    /// enter a non-`Function` context are responsible for saving and
    /// restoring this field around the recursive `emit_exp` call (see
    /// [`GenCtx::with_qmode`]).
    qmode: QMode,
    /// Variables in scope at the enclosing function level: inputs, outputs,
    /// and protected locals. Used to seed the per-arm `LocalEnv` when entering
    /// a match-expression case body, so assignments to function-level outputs
    /// or protected components are recognised as re-assignments (plain `=`)
    /// rather than fresh `let` shadowings. Cleared between functions.
    fn_env_vars: HashMap<String, Ty>,
    /// Output component names (in declaration order) of the enclosing function.
    /// Used by `S::Return` inside a match-arm body to expand `return;` into
    /// `return Ok((outputs...));`. The arm body is emitted with a fresh
    /// `LocalEnv`, which would otherwise lose this information.
    fn_outputs: Vec<String>,
    /// Variables currently known to hold a specific uniontype variant. Mirrors
    /// `LocalEnv::variants` but is accessible from `emit_exp` / `emit_var`,
    /// which don't receive the per-statement `LocalEnv`. Populated around the
    /// emission of a match arm's body+result; saved and restored at arm
    /// boundaries so nested match expressions don't leak into the enclosing
    /// scope. The keys are MetaModelica variable names (pre-escape); values
    /// are `(enum_qname, variant_simple_name)`.
    variants: HashMap<String, (String, String)>,
    /// Rust binding shape for variables in `variants`. Determines the deref
    /// form emitted around `var_field!` scrutinee:
    ///   `Owned` — plain `T` enum value (`var_field!(v.f, V::X)`).
    ///   `Arc`   — `Arc<T>` / similar smart pointer (`var_field!((*v).f, ..)`).
    ///   `RefArc` — `&Arc<T>` (a `ref`-bound pattern field whose declared type
    ///              is itself `Arc<T>`); requires `(**v).f`.
    /// Absent entries default to `Owned` for top-level function vars whose
    /// Arc-ness is read from `fn_env_vars` instead.
    variant_shapes: HashMap<String, VarShape>,
}

/// Binding-shape classification for variables tracked in `GenCtx::variants`.
/// Determines how `var_field!` should dereference the variable to obtain the
/// underlying enum value for pattern matching.
#[derive(Clone, Copy, Debug, PartialEq)]
enum VarShape {
    /// Owned `T` (the variable holds an enum value directly).
    Owned,
    /// `Arc<T>` / smart-pointer wrapped enum (e.g. recursive uniontype value).
    Arc,
    /// `&Arc<T>` — a reference to an Arc-wrapped enum. Produced when a
    /// uniontype variant pattern destructures an Arc-typed field with
    /// `ref binding @ ...`.
    RefArc,
}

impl GenCtx {
    fn new(top_name: &str, current_crate: Option<String>, crate_map: BTreeMap<String, String>, top_level_uniontype_names: HashSet<String>, recursive_types: BTreeSet<String>, types_containing_mutable: BTreeSet<String>, fn_type_vars: BTreeMap<String, Vec<String>>) -> Self {
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
            types_containing_mutable,
            no_mod_uniontypes: HashSet::new(),
            fn_type_vars,
            qmode: QMode::Function,
            fn_env_vars: HashMap::new(),
            fn_outputs: Vec::new(),
            variants: HashMap::new(),
            variant_shapes: HashMap::new(),
        }
    }

    /// Produce the `#[derive(...)]` attribute appropriate for the type identified
    /// by `qname`. Types whose fields transitively embed `Mutable<T>` cannot
    /// derive `PartialEq` / `Eq` / `Hash`; they get only `Clone, Debug`.
    fn derives_for(&self, qname: &str) -> &'static str {
        // We added a custom implementation for PartialEq for Mutable<T>, so we can still derive PartialEq for types containing it.
        if self.types_containing_mutable.contains(qname) {
            "#[derive(Clone, Debug, PartialEq)]"
        } else {
            "#[derive(Clone, Debug, PartialEq)]"
        }
    }

    /// Wrap a fallible Rust expression with the appropriate error-propagation
    /// form for the current [`QMode`]. The argument should be a complete
    /// expression that evaluates to `Result<T, _>`; the returned string is
    /// an expression of type `T` in the surrounding context.
    fn q(&self, expr: &str) -> String {
        match &self.qmode {
            QMode::Function => format!("{expr}?"),
            // `Iterator::filter` requires a plain `bool` predicate; we cannot
            // propagate via `?`. The reductions we generate this for invoke
            // user predicates that should not fail at runtime; if one does,
            // surfacing the panic is preferable to silently swallowing it.
            QMode::Filter => format!("{expr}.unwrap()"),
            QMode::TryBlock(label) => format!("unwrap_break_err!({expr}, {label})"),
            QMode::Bare => expr.to_owned(),
        }
    }

    /// Run `body` with `qmode` temporarily replaced. Restores the previous
    /// mode on exit, including on panic — though we don't rely on panic
    /// safety, it keeps the invariant readable.
    fn with_qmode<R>(&mut self, mode: QMode, body: impl FnOnce(&mut Self) -> R) -> R {
        let saved = std::mem::replace(&mut self.qmode, mode);
        let r = body(self);
        self.qmode = saved;
        r
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
                            ctx.named.insert(effective, name.to_string());
                        }
                    }
                }
                Absyn::Import::GROUP_IMPORT { prefix, groups } => {
                    let prefix_str = path_to_dotted(prefix);
                    if prefix_str != ctx.top_name && !prefix_str.starts_with(&same_file_prefix) {
                        for g in (&**groups).into_iter() {
                            let (local, orig): (String, String) = match g {
                                Absyn::GroupImport::GROUP_IMPORT_NAME { name } => (name.to_string(), name.to_string()),
                                Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, name } => (rename.to_string(), name.to_string()),
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
    let trace_codegen = matches!(
        std::env::var("MMTORUST_TRACE_CODEGEN").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    );
    let file_timeout_secs: u64 = std::env::var("MMTORUST_FILE_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
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
    let all_file_t0 = std::time::Instant::now();

    // Serial pass: create directories and collect (dir, name, node) tuples for parallel codegen.
    let mut file_jobs: Vec<(&str, &str, &NameNode<'_>)> = Vec::new();
    for (dir, classes) in &dir_classes {
        if dir == "openmodelica/src" {
            continue; // Builtin - handwritten code
        };
        std::fs::create_dir_all(dir)?;
        for (name, node) in classes {
            match *name {
                "Mutable" | "GCExt" | "Pointer" => continue,
                _ => {}
            };
            file_jobs.push((dir.as_str(), name, node));
        }
    }

    // Parallel pass: each file is generated independently.
    file_jobs.par_iter().try_for_each(|(dir, name, node)| -> std::io::Result<()> {
        let current_crate = if let NodeKind::Class(c) = &node.kind {
            c.crate_name.clone()
        } else {
            None
        };
        let file_path = format!("{dir}/{name}.rs");
        if trace_codegen {
            eprintln!("[mmtorust] codegen start {file_path}");
        }
        let file_t0 = std::time::Instant::now();
        let content = generate_file(name, node, &crate_map, current_crate, &top_level_uniontype_names, hier.recursive_types.clone(), hier.types_containing_mutable.clone(), &no_mod_uniontypes, &hier.top_level, &fn_type_vars);
        let file_elapsed = file_t0.elapsed();
        if trace_codegen {
            eprintln!("[mmtorust] codegen done  {file_path} ({:.2}s)", file_elapsed.as_secs_f64());
        }
        if file_timeout_secs > 0 && file_elapsed.as_secs() > file_timeout_secs {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                format!("codegen for {file_path} exceeded {file_timeout_secs}s"),
            ));
        }
        std::fs::write(&file_path, content)?;
        Ok(())
    })?;

    // Serial pass: write lib.rs per directory.
    for dir in dir_classes.keys() {
        if dir == "openmodelica/src" {
            continue;
        }
        let lib_content = generate_lib_file(hier, dir, output_dir);
        std::fs::write(format!("{dir}/lib.rs"), lib_content)?;
    }
    let all_file_elapsed = all_file_t0.elapsed();
    if trace_codegen {
        eprintln!("[mmtorust] codegen done all files ({:.2}s)", all_file_elapsed.as_secs_f64());
    }
    Ok(())
}

fn generate_lib_file(hier: &InstanceHierarchy<'_>, this_dir: &str, default_dir: &str) -> String {
    let mut out = String::new();
    writeln!(out, "// Auto-generated lib file").unwrap();
    writeln!(out, "// TODO: Decide if we go with nightly rust for deref patterns, or https://crates.io/crates/match_deref").unwrap();
    writeln!(out, "#![feature(deref_patterns)]").unwrap(); // We have long lists to macro through...
    writeln!(out, "#![recursion_limit = \"1024\"]").unwrap(); // We have long lists to macro through...
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

fn generate_file<'a>(top_name: &str, node: &NameNode<'_>, crate_map: &BTreeMap<String, String>, current_crate: Option<String>, top_level_uniontype_names: &HashSet<String>, recursive_types: BTreeSet<String>, types_containing_mutable: BTreeSet<String>, no_mod_uniontypes: &HashSet<String>, top_level: &'a BTreeMap<String, NameNode<'a>>, fn_type_vars: &BTreeMap<String, Vec<String>>) -> String {
    let mut ctx = GenCtx::new(top_name, current_crate, crate_map.clone(), top_level_uniontype_names.clone(), recursive_types, types_containing_mutable, fn_type_vars.clone());
    ctx.no_mod_uniontypes = no_mod_uniontypes.clone();
    collect_imports(node, &mut ctx, top_level);

    // First pass: emit the body so that shorten() can populate implicit_modules.
    let mut body = String::new();
    emit_node(&mut body, top_name, node, "", &mut ctx, top_level);

    // Second pass: emit header + complete use lines (now including implicit modules).
    let mut out = String::new();
    writeln!(out, "// Auto-generated from MetaModelica source").unwrap();
    writeln!(out, "#![allow(warnings)]").unwrap();
    writeln!(out, "#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]").unwrap();
    writeln!(out, "{}", "
use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};
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
            if let Some(exp) = extract_default_exp(&m.modification) {
                let pkg_prefix = if ctx.current_path.is_empty() {
                    ctx.top_name.to_owned()
                } else {
                    format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
                };
                let typed = typedexp::infer_exp(exp, &HashMap::new(), top_level, &pkg_prefix, &[]);
                let ename = escape_ident(name);

                let rust_ty = match &node.ty {
                    Ty::Str => Some("ArcStr"),
                    Ty::I32 => Some("i32"),
                    Ty::F64 => Some("f64"),
                    Ty::Bool => Some("bool"),
                    _ => None,
                };
                if let Some(r_ty) = rust_ty {
                    let val = emit_exp(&typed, /*is_const=*/true, ctx, top_level);
                    writeln!(out, "{indent}pub const {ename}: {r_ty} = {val};").unwrap();
                    writeln!(out).unwrap();
                } else if is_static_const_emittable(&typed, ctx, top_level) {
                    // Record/struct of pure literals and other const-emittable values:
                    // emit a plain `pub static` whose initializer is a const expression.
                    // No `LazyLock` indirection needed — the value can be built at compile time.
                    let r_ty = fmt_ty(&node.ty, ctx);
                    let val = emit_exp(&typed, /*is_const=*/true, ctx, top_level);
                    writeln!(out, "{indent}pub static {ename}: {r_ty} = {val};").unwrap();
                    writeln!(out).unwrap();
                } else {
                    let r_ty = fmt_ty(&node.ty, ctx);
                    let val = emit_exp(&typed, /*is_const=*/false, ctx, top_level); // dynamic expr
                    writeln!(out, "{indent}pub static {ename}: std::sync::LazyLock<{r_ty}> = std::sync::LazyLock::new(|| {{ {val} }});").unwrap();
                    writeln!(out).unwrap();
                }
            }
        }
        return;
    }
    let NodeKind::Class(c) = &node.kind else { return };
    // `partial` packages (`partial package Foo`) are still processed — their
    // contents need to be emitted. Partial functions used to be skipped here,
    // but they are now dispatched into `emit_function`, which detects
    // `partial_prefix` and emits a Rust `type Foo<T> = fn(...) -> Result<...>`
    // alias for them. Other partial classes (records, types, ...) have no
    // useful Rust representation and are skipped.
    if c.partial_prefix && !matches!(
        &c.restriction,
        Absyn::Restriction::R_PACKAGE | Absyn::Restriction::R_FUNCTION { .. },
    ) {
        return;
    }
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
            if !c.partial_prefix {
                let mut children: Vec<_> = node.children.iter().collect();
                children.sort_by_key(|(n, _)| n.as_str());
                for (child_name, child_node) in children {
                    emit_node(out, child_name, child_node, &child_indent, &mut *ctx, top_level);
                }
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
        Ty::RustEnum(qname) => {
            let type_vars: Vec<String> = match &c.body {
                MM::ClassDef::Parts { type_vars, .. } => type_vars.clone(),
                _ => vec![],
            };
            let type_params = if type_vars.is_empty() { String::new() } else { format!("<{}>", type_vars.join(", ")) };
            let mut emitted_variants: Vec<String> = Vec::new();
            writeln!(out, "{inner}{}", ctx.derives_for(qname)).unwrap();
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
            // Emit a type alias from the record name to the struct so that code
            // written as `RECORD_NAME { field: ... }` or `let RECORD_NAME { field } = ...`
            // continues to work after the struct is renamed to the uniontype name.
            // Type aliases are usable as struct constructor and pattern syntax in Rust.
            if !rec_name.is_empty() && rec_name != name {
                let ename_alias = escape_ident(name);
                let alias_name = escape_ident(&rec_name);
                // Collect type parameters from the struct so the alias is properly generic.
                let type_vars: Vec<String> = if let Some(rec_node) = node.children.get(&rec_name) {
                    if let NodeKind::Class(rc) = &rec_node.kind {
                        let fields = component_fields(rc, &rec_node.children);
                        let mut tvs = Vec::new();
                        for (_, fty) in &fields { collect_type_vars_in_ty(fty, &mut tvs); }
                        tvs
                    } else { vec![] }
                } else { vec![] };
                let type_params = if type_vars.is_empty() {
                    String::new()
                } else {
                    format!("<{}>", type_vars.join(", "))
                };
                writeln!(out, "{inner}pub type {alias_name}{type_params} = {ename_alias}{type_params};").unwrap();
                writeln!(out).unwrap();
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
    let type_params = if type_vars.is_empty() { String::new() } else { format!("<{}: {DEFAULT_TRAITS}>", type_vars.join(", ")) };
    let derives = match &node.ty {
        Ty::RustStruct(qname) => ctx.derives_for(qname),
        _ => "#[derive(Clone, Debug, PartialEq)]",
    };
    writeln!(out, "{derives}").unwrap();
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
        MM::ClassDef::Derived { type_spec: Absyn::TypeSpec::TCOMPLEX { path: Absyn::Path::IDENT { name }, .. }, .. } if &**name == "polymorphic" => (),
        MM::ClassDef::Derived { .. } => {
            let mut type_vars: Vec<String> = Vec::new();
            collect_type_vars_in_ty(&node.ty, &mut type_vars);
            let type_params = if type_vars.is_empty() { String::new() } else { format!("<{}>", type_vars.join(", ")) };
            writeln!(out, "{indent}pub type {}{type_params} = {};", escape_ident(name), fmt_ty(&node.ty, &mut *ctx)).unwrap();
            writeln!(out).unwrap();
        }
        MM::ClassDef::Enumeration { enum_literals, .. } => {
            if let Absyn::EnumDef::ENUMLITERALS { enumLiterals } = enum_literals {
                writeln!(out, "{indent}#[derive(Clone, Debug, PartialEq)]").unwrap();
                writeln!(out, "{indent}pub enum {} {{", escape_ident(name)).unwrap();
                for lit in &**enumLiterals {
                    let Absyn::EnumLiteral::ENUMLITERAL { literal, .. } = lit;
                    writeln!(out, "{indent}    {},", escape_ident(&**literal)).unwrap();
                }
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
            }
        }
        _ => {}
    }
}

fn emit_function<'a>(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
    // Short class definition: `function Alias = Base[(arg=default, ...)]`.
    // Resolved by hierarchy::resolve_function_type to Ty::FunctionAlias.
    //
    // Without modifications we re-export the base function under the alias name
    // (`pub use crate::path::base as alias;`). The base path is shortened to be
    // relative to the current module/scope just like a normal type reference.
    //
    // With modifications (default-argument overrides) we cannot use a plain
    // re-export because the alias must apply those overrides. That case isn't
    // wired up yet, so we emit a `todo!()` placeholder so it surfaces at compile
    // time rather than silently dropping the alias.
    if let Ty::FunctionAlias { base, modifications } = &node.ty {
        let pub_kw = if node.visibility == MM::Visibility::Public { "pub " } else { "" };
        let alias_name = escape_ident(name);
        if modifications.is_empty() {
            let base_short = ctx.shorten(base);
            writeln!(out, "{indent}{pub_kw}use {base_short} as {alias_name};").unwrap();
            writeln!(out).unwrap();
        } else {
            // TODO: emit a wrapper `pub fn {name}(...) -> ... {{ {base}(..., {arg}={value}, ...) }}`
            // by looking up the base function's signature and substituting defaults
            // for the overridden parameters.
            let mods = modifications.iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(out, "{indent}// {pub_kw}fn {alias_name} = {base}({mods}) -- function alias with default-arg modifications not yet supported").unwrap();
            writeln!(out, "{indent}{pub_kw}fn {alias_name}() {{ todo!(\"function alias {name} = {base}({mods})\") }}").unwrap();
            writeln!(out).unwrap();
        }
        return;
    }

    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return,
    };

    // Use types from the resolved Ty::Function — those were computed by resolve_function_type
    // with the correct type_vars in scope, so type-variable parameters resolve correctly.
    // Child node .ty values are resolved without that context and may be Unknown for ArgT etc.
    let Ty::Function { type_vars, inputs: fn_inputs, output: fn_output, .. } = &node.ty else { return };

    // `partial function` declarations are MetaModelica's way of naming a function
    // signature — they have no body and are used as a type for function-valued
    // parameters (e.g. `KeyEq` inside `UnorderedSet`). Emit them as a Rust type
    // alias so that consumers can write `KeyEq<T>` instead of repeating the raw
    // `fn(...) -> Result<...>` signature.
    //
    // TODO: also rewrite parameter types that resolve to a partial function into
    // a reference to this alias (currently we still inline the raw `fn` type at
    // those sites, which compiles but loses the named-type readability).
    if c.partial_prefix {
        let mut all_type_vars = type_vars.clone();
        for inp in fn_inputs.iter() {
            collect_type_vars_in_ty(&inp.ty, &mut all_type_vars);
        }
        collect_type_vars_in_ty(fn_output, &mut all_type_vars);
        let type_params = if all_type_vars.is_empty() {
            String::new()
        } else {
            format!("<{}: {DEFAULT_TRAITS}>", all_type_vars.join(", "))
        };
        let ins = fn_inputs.iter().map(|inp| fmt_ty(&inp.ty, ctx)).collect::<Vec<_>>().join(", ");
        let out_ty = fmt_ty(fn_output, ctx);
        let pub_kw = if node.visibility == MM::Visibility::Public { "pub " } else { "" };
        let ename = escape_ident(name);
        writeln!(out, "{indent}{pub_kw}type {ename}{type_params} = fn({ins}) -> Result<{out_ty}>;").unwrap();
        writeln!(out).unwrap();
        return;
    }

    let mut all_type_vars = type_vars.clone();
    for inp in fn_inputs.iter() {
        collect_type_vars_in_ty(&inp.ty, &mut all_type_vars);
    }
    collect_type_vars_in_ty(fn_output, &mut all_type_vars);
    let type_params = if all_type_vars.is_empty() {
        String::new()
    } else {
        format!("<{}>", all_type_vars.iter().map(|v| format!("{v}: {DEFAULT_TRAITS}")).collect::<Vec<_>>().join(", "))
    };

    let params = fn_inputs.iter()
        .map(|inp| format!("{}: {}", escape_ident(&inp.name), fmt_param_ty(&inp.ty, ctx)))
        .collect::<Vec<_>>()
        .join(", ");

    let ret_ty = fmt_ty(fn_output, ctx);
    let ename = escape_ident(name);

    let pub_kw = if node.visibility == MM::Visibility::Public { "pub " } else { "" };

    // Walk components to find outputs (with names) and protected locals.
    let mut outputs: Vec<(String, Ty, Option<Absyn::Modification>, bool)> = Vec::new();
    let mut protected: Vec<(String, Ty, Option<Absyn::Modification>, bool)> = Vec::new();
    let mut input_names: HashSet<String> = HashSet::new();
    for inp in fn_inputs.iter() { input_names.insert(inp.name.clone()); }
    for member in members {
        let MM::ClassMember::Component(cm) = member else { continue };
        let child_ty = node.children.get(&cm.name).map(|n| n.ty.clone()).unwrap_or(Ty::Unknown);
        match cm.direction {
            Absyn::Direction::OUTPUT | Absyn::Direction::INPUT_OUTPUT =>
                outputs.push((
                    cm.name.clone(),
                    child_ty,
                    cm.modification.clone(),
                    cm.variability == Absyn::Variability::CONST,
                )),
            Absyn::Direction::BIDIR => {
                if !input_names.contains(&cm.name) {
                    protected.push((
                        cm.name.clone(),
                        child_ty,
                        cm.modification.clone(),
                        cm.variability == Absyn::Variability::CONST,
                    ));
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
    for (n, t, _, _) in &outputs { infer_env.insert(n.clone(), t.clone()); }
    for (n, t, _, _) in &protected { infer_env.insert(n.clone(), t.clone()); }

    let alg_items: &[Absyn::AlgorithmItem] = match &c.body {
        MM::ClassDef::Parts { algorithms, .. } | MM::ClassDef::ClassExtends { algorithms, .. } => algorithms,
        _ => &[],
    };

    let typed_stmts = typedexp::infer_stmts(alg_items, &mut infer_env, top_level, &pkg_prefix, &all_type_vars);

    let mut env = LocalEnv::default();
    for inp in fn_inputs.iter() { env.vars.insert(inp.name.clone(), inp.ty.clone()); }
    for (n, t, _, _) in &outputs   { env.vars.insert(n.clone(), t.clone()); }
    for (n, t, _, _) in &protected { env.vars.insert(n.clone(), t.clone()); }
    env.outputs = outputs.iter().map(|(n, _, _, _)| n.clone()).collect();

    // Expose function-level scope on the codegen context so emit_match's
    // per-arm `LocalEnv` (which would otherwise start empty) can recognise
    // re-assignments to outputs/protected components and so `return;` inside
    // a match arm can expand to the same Ok(...) tuple as the function tail.
    ctx.fn_env_vars = env.vars.clone();
    ctx.fn_outputs = env.outputs.clone();

    writeln!(out, "{indent}{pub_kw}fn {ename}{type_params}({params}) -> Result<{ret_ty}> {{").unwrap();
    let body_indent = format!("{indent}    ");

    for (n, t, modif, is_const_local) in outputs.iter().chain(protected.iter()) {
        let ty_s = fmt_ty(t, ctx);
        let modif_opt: Option<Absyn::Modification> = modif.clone();
        let init_raw = extract_default_exp(&modif_opt).map(|exp| {
            let typed = typedexp::infer_exp(exp, &infer_env, top_level, &pkg_prefix, &all_type_vars);
            emit_exp(&typed, false, ctx, top_level)
        });
        let init = if input_names.contains(n) {
            Some(escape_ident(n))
        } else {
            let cloned_s = format!("{}.clone()", escape_ident(n));
            init_raw.filter(|s| s != &escape_ident(n) && s != &cloned_s)
        };
        match (is_const_local, init) {
            (true, Some(s)) => writeln!(out, "{body_indent}let {}: {ty_s} = {s};", escape_ident(n)).unwrap(),
            (true, None) => writeln!(out, "{body_indent}let mut {}: {ty_s};", escape_ident(n)).unwrap(),
            (false, Some(s)) => writeln!(out, "{body_indent}let mut {}: {ty_s} = {s};", escape_ident(n)).unwrap(),
            (false, None) => writeln!(out, "{body_indent}let mut {}: {ty_s};", escape_ident(n)).unwrap(),
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
            let parts: Vec<String> = outputs.iter().map(|(n, _, _, _)| escape_ident(n)).collect();
            writeln!(out, "{body_indent}Ok(({}))", parts.join(", ")).unwrap();
        }
    }
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();
}

// ── Expression and pattern emission ──────────────────────────────────────────

fn emit_exp<'a>(exp: &TypedExp, is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    match exp {
        TypedExp::Lit(Lit::Int(v))  => v.to_string(),
        TypedExp::Lit(Lit::Real(v)) => v.clone(),
        TypedExp::Lit(Lit::Str(v))  => {
            let escaped = format!("{v:?}");
            format!("literal!({escaped})")
        }
        TypedExp::Lit(Lit::Bool(v)) => v.to_string(),

        TypedExp::Var { name, segments, ty, .. } => {
            let var_str = emit_var(name, segments, ty, ctx, top_level);
            match ty {
                // Anonymous function types resolve to `impl Fn(...)` in
                // parameter position (see `fmt_param_ty`). `impl Fn` is not
                // `Copy`, so passing the variable by value moves it; once
                // moved the binding can't be used again (which trips up the
                // very common pattern of forwarding a callback through a
                // loop or to multiple helper calls). Pass it by reference
                // instead — `&F: Fn` when `F: Fn`, so receivers that also
                // declare `impl Fn(...)` accept the borrow transparently.
                Ty::Function { name: None, .. } => format!("&{var_str}"),
                // Named `partial function` aliases (e.g. `KeyEq`) lower to a
                // concrete `fn(...)` pointer — that *is* `Copy`, so the move
                // hazard above doesn't apply and we keep emitting the bare
                // identifier. `Ty::FunctionAlias` (re-export aliases) is the
                // same case.
                Ty::Function { .. } | Ty::FunctionAlias { .. } => var_str,
                _ => format!("{var_str}.clone()"),
            }
        }

        TypedExp::BinOp { op, lhs, rhs, ty, .. } => {
            let mut l = emit_exp(lhs, is_const, ctx, top_level);
            if lhs.ty() == Ty::I32 && rhs.ty() == Ty::F64 {
                l = format!("({l} as f64)");
            }
            let mut r = emit_exp(rhs, is_const, ctx, top_level);
            if rhs.ty() == Ty::I32 && lhs.ty() == Ty::F64 {
                r = format!("({r} as f64)");
            }
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
                    // Collect all string parts from a chain of Add ops and emit one ArcStr concat.
                    let mut parts: Vec<String> = Vec::new();
                    collect_string_concat_parts(exp, is_const, ctx, top_level, &mut parts);
                    if parts.is_empty() {
                        panic!("How can a binary expression have 0 arguments?")
                    } else {
                        let mut s = String::from("{ let mut __mm_s = String::new(); ");
                        for p in parts {
                            let _ = write!(s, "__mm_s.push_str(&*{p}); ");
                        }
                        s.push_str("ArcStr::from(__mm_s) }");
                        s
                    }
                }
                BinOpKind::Add if *ty == Ty::Str => format!("(*{l}).clone() + &*{r}"),
                BinOpKind::Add => {
                    let lp = if lhs.ty() == Ty::I32 && rhs.ty() == Ty::F64 { format!("({l} as f64)") } else { l };
                    let rp = if rhs.ty() == Ty::I32 && lhs.ty() == Ty::F64 { format!("({r} as f64)") } else { r };
                    format!("{lp} + {rp}")
                },
                BinOpKind::Sub => {
                    let lp = if lhs.ty() == Ty::I32 && rhs.ty() == Ty::F64 { format!("({l} as f64)") } else { l };
                    let rp = if rhs.ty() == Ty::I32 && lhs.ty() == Ty::F64 { format!("({r} as f64)") } else { r };
                    format!("{lp} - {rp}")

                },
                BinOpKind::Mul => {
                    let lp = if lhs.ty() == Ty::I32 && rhs.ty() == Ty::F64 { format!("({l} as f64)") } else { l };
                    let rp = if rhs.ty() == Ty::I32 && lhs.ty() == Ty::F64 { format!("({r} as f64)") } else { r };
                    format!("{lp} * {rp}")
                },
                BinOpKind::Div => {
                    let lp = if lhs.ty() == Ty::I32 && rhs.ty() == Ty::F64 { format!("({l} as f64)") } else { l };
                    let rp = if rhs.ty() == Ty::I32 && lhs.ty() == Ty::F64 { format!("({r} as f64)") } else { r };
                    format!("{lp} / {rp}")
                },
                BinOpKind::Pow => {
                    let lp = if lhs.ty() == Ty::I32 { format!("({l} as f64)") } else { l };
                    let rp = if rhs.ty() == Ty::I32 { format!("({r} as f64)") } else { r };
                    format!("({lp}).powf({rp})")
                }
                BinOpKind::And => format!("{l} && {r}"),
                BinOpKind::Or  => format!("{l} || {r}"),
            }
        }

        TypedExp::UnOp { op, operand, .. } => {
            let s = emit_exp(operand, is_const, ctx, top_level);
            match op {
                UnOpKind::Neg => format!("-({s})"),
                UnOpKind::Not => format!("!({s})"),
            }
        }

        // TODO: Comprehensions
        TypedExp::Call { func, args, named_args, sig_ty, .. } => {
            let num_named = named_args.len();
            if named_args.is_empty() {
                if let Ok(res) = emit_builtin_call(func, args, is_const, ctx, top_level) {
                    return res;
                }
            }
            let func_str = if func.contains('.') {
                &ctx.shorten(func)
            } else {
                func
            };
            let func_str = escape_ident(func_str);
            let formals = resolve_call_formals(func, ctx, top_level);
            let parts: Vec<String> = if let Some(formals) = formals {
                let has_defaults = formals.iter().any(|(_, _, d)| d.is_some());
                if has_defaults {
                let mut slots: Vec<Option<TypedExp>> = vec![None; formals.len()];
                let mut failed = false;

                for (i, a) in args.iter().enumerate() {
                    if i >= slots.len() {
                        failed = true;
                        break;
                    }
                    slots[i] = Some(a.clone());
                }

                if !failed {
                    for (n, v) in named_args {
                        let Some(idx) = formals.iter().position(|(fname, _, _)| fname == n) else {
                            failed = true;
                            break;
                        };
                        if slots[idx].is_some() {
                            failed = true;
                            break;
                        }
                        slots[idx] = Some(v.clone());
                    }
                }

                if !failed {
                    for i in 0..slots.len() {
                        if slots[i].is_some() {
                            continue;
                        }
                        if let Some(default_tpl) = formals[i].2.clone() {
                            let mut bindings: HashMap<String, TypedExp> = HashMap::new();
                            for (j, slot) in slots.iter().enumerate() {
                                if let Some(e) = slot {
                                    bindings.insert(formals[j].0.clone(), e.clone());
                                }
                            }
                            slots[i] = Some(substitute_formal_refs(&default_tpl, &bindings));
                        }
                    }
                }

                if !failed {
                    for slot in &slots {
                        if slot.is_none() {
                            failed = true;
                            break;
                        }
                    }
                }

                if failed {
                    let mut parts: Vec<String> = args.iter().enumerate().map(|(i, a)| {
                        emit_call_arg_with_formal(a, formals.get(i).map(|f| &f.1), is_const, ctx, top_level)
                    }).collect();
                    for (n, v) in named_args {
                        let formal_ty = formals.iter().find_map(|(fname, fty, _)| if fname == n { Some(fty) } else { None });
                        let v = emit_call_arg_with_formal(v, formal_ty, is_const, ctx, top_level);
                        parts.push(format!("{n}={v}"));
                    }
                    parts
                } else {
                    slots.into_iter().enumerate()
                        .map(|(i, s)| {
                            emit_call_arg_with_formal(&s.unwrap(), formals.get(i).map(|f| &f.1), is_const, ctx, top_level)
                        })
                        .collect()
                }
                } else {
                    let mut parts: Vec<String> = args.iter().enumerate().map(|(i, a)| {
                        emit_call_arg_with_formal(a, formals.get(i).map(|f| &f.1), is_const, ctx, top_level)
                    }).collect();
                    for (n, v) in named_args {
                        let formal_ty = formals.iter().find_map(|(fname, fty, _)| if fname == n { Some(fty) } else { None });
                        let v = emit_call_arg_with_formal(v, formal_ty, is_const, ctx, top_level);
                        parts.push(format!("{n}={v}"));
                    }
                    parts
                }
            } else {
                let mut parts: Vec<String> = args.iter().map(|a| emit_cloned_call_arg(a, is_const, ctx, top_level)).collect();
                for (n, v) in named_args {
                    let v = emit_cloned_call_arg(v, is_const, ctx, top_level);
                    parts.push(format!("{n}={v}"));
                }
                parts
            };

            let mut is_ctor = is_constructor(&func_str, ctx, top_level) || is_constructor(func, ctx, top_level);
            if is_ctor {
                println!("{:?} is a constructor, but was not detected as such in typedexp.rs", exp);
                is_ctor = false;
            }
            let mut call = format!("{func_str}({})", parts.join(", "));

            // Add `?` to propagate Result errors from fallible calls. Skip in const
            // context, for constructors (uppercase first char), and for known-infallible
            // note that infallible builtins still return a Result because they can be used as function pointers.
            //  In order to skip the error checking here, it needs to have a special case handling the function above
            if is_const {
                call
            } else {
                ctx.q(&call)
            }
        }

        TypedExp::If { cond, then_, elseif, else_, .. } => {
            let c = emit_exp(cond, is_const, ctx, top_level);
            let t = emit_exp(then_, is_const, ctx, top_level);
            let e = emit_exp(else_, is_const, ctx, top_level);
            let ei: String = elseif.iter()
                .map(|(ec, eb)| format!(" else if ({}) {{{}}}", emit_exp(ec, is_const, ctx, top_level), emit_exp(eb, is_const, ctx, top_level)))
                .collect();
            format!("if ({c}) {{{t}}}{ei} else {{{e}}}")
        }

        TypedExp::Cons { head, tail, .. } => {
            format!("cons({}, {})", emit_exp(head, is_const, ctx, top_level), emit_exp(tail, is_const, ctx, top_level))
        }

        TypedExp::Tuple(elems) => {
            let parts: Vec<String> = elems.iter().map(|e| emit_exp(e, is_const, ctx, top_level)).collect();
            format!("({})", parts.join(", "))
        }

        TypedExp::Array { elems, .. } => {
            if elems.is_empty() {
                "metamodelica::nil()".to_owned()
            } else {
                let parts: Vec<String> = elems.iter().map(|e| emit_exp(e, is_const, ctx, top_level)).collect();
                format!("list![{}]", parts.join(", "))
            }
        }

        TypedExp::Match { kind, input, cases, .. } => {
            emit_match(kind, input, cases, is_const, ctx, top_level)
        }

        TypedExp::Range { start, step, stop, .. } => {
            emit_range(start, step.as_deref(), stop, is_const, ctx, top_level)
        }

        TypedExp::Constructor { name, args, named_args, ty, field_names } => {
            let mut arg_strs = Vec::new();
            if let Ty::RustStruct(qname) | Ty::RustEnum(qname) = ty {
                let mut remaining_named = named_args.clone();
                for (i, fa) in args.iter().enumerate() {
                    let val = emit_cloned_call_arg(fa, is_const, ctx, top_level);
                    if i < field_names.len() {
                        let fname = &field_names[i];
                        let fname_safe = escape_ident(fname);
                        // Wrap in Arc::new if the struct field is stored as Arc<T> due to
                        // size-recursive cycles.  String fields are excluded: their expressions
                        // already yield ArcStr from emit_exp / emit_cloned_call_arg.
                        let val = if struct_field_is_arc(qname, fname, top_level, ctx) {
                            format!("Arc::new({val})")
                        } else {
                            val
                        };
                        arg_strs.push(format!("{fname_safe}: {val}"));
                    } else {
                        // fallback if we have more pos args than fields? shouldn't happen for valid mm
                        arg_strs.push(val);
                    }
                }
                for (n, na) in remaining_named {
                    let val = emit_cloned_call_arg(&na, is_const, ctx, top_level);
                    let val = if struct_field_is_arc(qname, &n, top_level, ctx) {
                        format!("Arc::new({val})")
                    } else {
                        val
                    };
                    arg_strs.push(format!("{}: {val}", escape_ident(&n)));
                }

                // Resolve the struct's Rust path the same way `fmt_ty` does for type
                // positions: a single-record uniontype's struct lives at `Mod::Struct`
                // (the struct shares its name with the enclosing module/file). Without
                // the doubling, `crate::SBSet { ... }` resolves to the module, not the
                // struct, and Rust rejects it (E0574).
                //
                // Special case: a record inside a multi-record uniontype is typed as
                // `RustStruct(record_qname)` here (seed_metarecords doesn't promote it
                // to `UnionTypeVariant`), but in Rust it's an enum variant — not its
                // own struct. Detect this by looking up the parent ty; if the parent
                // is `RustEnum`, build the path as `<EnumPath>::<Variant>` so the
                // path-resolution rules (including `no_mod_uniontypes`) apply to the
                // enum, not the variant.
                let last = qname.rsplit('.').next().unwrap_or(qname);
                let parent_qname: Option<&str> = qname.rfind('.').map(|i| &qname[..i]);
                let parent_is_enum = parent_qname
                    .and_then(|p| lookup_node_ty(p, top_level))
                    .map(|t| matches!(t, Ty::RustEnum(_)))
                    .unwrap_or(false);
                let c_rust = if parent_is_enum {
                    build_variant_path(parent_qname.unwrap(), last, ctx)
                } else {
                    let first = qname.split('.').next().unwrap_or(qname);
                    let shortened = ctx.shorten(qname);
                    let in_own_mod = ctx.current_path.last().map(|p| p == last).unwrap_or(false);
                    let needs_doubling = !in_own_mod && !ctx.no_mod_uniontypes.contains(qname.as_str()) && (
                        (ctx.top_level_uniontype_names.contains(first) && first != ctx.top_name) ||
                        (qname.contains('.') && first != last)
                    );
                    if needs_doubling {
                        format!("{shortened}::{last}")
                    } else {
                        shortened
                    }
                };
                let ctor_expr = if arg_strs.is_empty() {
                    format!("{c_rust}")
                } else if field_names.is_empty() {
                    // if we failed to get fields, assume tuple-like variant/struct (rare in MM)
                    format!("{c_rust}({})", arg_strs.join(", "))
                } else {
                    format!("{c_rust} {{ {} }}", arg_strs.join(", "))
                };
                // Wrap in Arc::new when constructing a variant of a recursive type;
                // all usages of such types in fields are already Arc<T>.
                if !is_const && constructor_needs_arc(ty, ctx) {
                    format!("Arc::new({ctor_expr})")
                } else {
                    ctor_expr
                }
            } else if matches!(ty, Ty::RustUnitVariant)
                || (matches!(ty, Ty::UnionTypeVariant(_, _))
                    && args.is_empty()
                    && named_args.is_empty()
                    && field_names.is_empty())
            {
                // Unit variant: no fields, no parentheses.
                ctx.dotted_to_rust_path(name)
            } else if let Ty::UnionTypeVariant(enum_qname, _variant_name) = ty {
                // Struct variant inside a multi-record uniontype (Rust enum).
                // Look up field names from the record in the hierarchy.
                let field_tys = record_field_tys(name, top_level)
                    .unwrap_or_else(|| record_field_tys_by_simple_name(name, top_level));
                let variant_rust = ctx.dotted_to_rust_path(name);
                for (i, a) in args.iter().enumerate() {
                    let val = emit_cloned_call_arg(a, is_const, ctx, top_level);
                    let fname = field_tys.get(i).map(|(n, _)| n.as_str()).unwrap_or("_");
                    let val = if struct_field_is_arc(enum_qname, fname, top_level, ctx) {
                        format!("Arc::new({val})")
                    } else { val };
                    arg_strs.push(format!("{}: {val}", escape_ident(fname)));
                }
                for (n, na) in named_args {
                    let val = emit_cloned_call_arg(&na, is_const, ctx, top_level);
                    let val = if struct_field_is_arc(enum_qname, &n, top_level, ctx) {
                        format!("Arc::new({val})")
                    } else { val };
                    arg_strs.push(format!("{}: {val}", escape_ident(&n)));
                }
                let ctor_expr = if arg_strs.is_empty() {
                    format!("{variant_rust}")
                } else {
                    format!("{variant_rust} {{ {} }}", arg_strs.join(", "))
                };
                if !is_const && constructor_needs_arc(ty, ctx) {
                    format!("Arc::new({ctor_expr})")
                } else {
                    ctor_expr
                }
            } else {
                // Unknown/fallback: emit as a function call.
                for a in args {
                    arg_strs.push(emit_cloned_call_arg(a, is_const, ctx, top_level));
                }
                let c_rust = ctx.dotted_to_rust_path(name);
                if arg_strs.is_empty() && field_names.is_empty() {
                    // No args and no known fields — likely a unit variant or nullary constructor.
                    // Emit without parentheses to avoid E0618.
                    c_rust
                } else {
                    format!("{c_rust}({})", arg_strs.join(", "))
                }
            }
        }

        TypedExp::Reduction { func, body, iterators, iter_kind, ty } =>
            emit_reduction(func, body, iterators, *iter_kind, ty, is_const, ctx, top_level),

        TypedExp::PartEval { func, args, named_args, sig_ty, .. } =>
            emit_parteval(func, args, named_args, sig_ty, is_const, ctx, top_level),

        TypedExp::Todo(s) => format!("todo!(/*{}*/)", s.chars().take(60).collect::<String>()),
    }
}

/// Emit a partial function application `function f(arg=val, ...)` as a Rust
/// `move` closure that captures the bound expressions and forwards the
/// remaining (unbound) formals to `f`.
///
/// We need the underlying function's formal list (names in declaration order)
/// to know:
///   * which formals are bound by named-arg syntax (matched by name),
///   * which positions remain unbound (and so become the closure's parameters).
///
/// `sig_ty` carries that information; it was set by `infer_exp` from a
/// hierarchy lookup. If the lookup failed, we still emit a closure but
/// surface the loss with a `todo!()` body so the gap is visible at compile
/// time rather than silently producing wrong code.
///
/// Capture handling: bound expressions are evaluated once when the closure
/// is constructed and stored in `let __pe_b{i} = ...;` bindings inside an
/// outer block. The closure body re-clones each captured value before
/// passing it to `f` (the closure must implement `Fn`, so each invocation
/// can read but not consume its captures). For non-`Clone` values this
/// would fail to compile — that's the desired signal that the captured
/// value's type doesn't support the `Fn` semantics we promise to callers.
fn emit_parteval<'a>(
    func: &str,
    args: &[TypedExp],
    named_args: &[(String, TypedExp)],
    sig_ty: &Ty,
    is_const: bool,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> String {
    let func_str = if func.contains('.') {
        ctx.shorten(func)
    } else {
        func.to_owned()
    };
    let func_str = escape_ident(&func_str);

    // Pull the formal-name order from the resolved signature.
    let formal_names: Vec<String> = match sig_ty {
        Ty::Function { inputs, .. } => inputs.iter().map(|i| i.name.clone()).collect(),
        _ => {
            // We have no signature info, so we can't tell how many formals
            // remain unbound or which ones the named-args refer to. Emit a
            // todo so the missing-signature case is visible at compile time.
            return format!("todo!(\"PARTEVALFUNCTION of {func}: function signature not resolved\")");
        }
    };

    let bound_pos = args.len();
    if bound_pos > formal_names.len() {
        return format!("todo!(\"PARTEVALFUNCTION of {func}: too many positional bindings\")");
    }
    // Map: formal name → bound expression (for named bindings).
    use std::collections::HashMap as HM;
    let mut named_map: HM<&str, &TypedExp> = HM::new();
    for (n, e) in named_args {
        named_map.insert(n.as_str(), e);
    }

    // Emit each bound expression once into a `let __pe_b{i}` binding so the
    // captured value lives outside the closure (the closure captures by
    // `move` and clones inside the body on each invocation).
    let mut captures: Vec<String> = Vec::new();
    // For each formal: record either a capture name (to clone in body) or
    // a fresh closure parameter name (forwarded to the call).
    let mut call_arg_exprs: Vec<String> = Vec::new();
    let mut closure_params: Vec<String> = Vec::new();

    for (i, formal_name) in formal_names.iter().enumerate() {
        if i < bound_pos {
            // Positional binding.
            let v = emit_exp(&args[i], is_const, ctx, top_level);
            let cap_name = format!("__pe_b{i}");
            captures.push(format!("let {cap_name} = {v};"));
            call_arg_exprs.push(format!("{cap_name}.clone()"));
        } else if let Some(named_expr) = named_map.remove(formal_name.as_str()) {
            // Named binding (looked up by formal name).
            let v = emit_exp(named_expr, is_const, ctx, top_level);
            let cap_name = format!("__pe_b{i}");
            captures.push(format!("let {cap_name} = {v};"));
            call_arg_exprs.push(format!("{cap_name}.clone()"));
        } else {
            // Unbound — becomes a closure parameter.
            let p = format!("__pe_a{i}");
            closure_params.push(p.clone());
            call_arg_exprs.push(p);
        }
    }

    // Any named-args that did NOT match a formal name are a type error —
    // surface them so the bug is visible rather than silently dropping them.
    if !named_map.is_empty() {
        let bad: Vec<&str> = named_map.keys().copied().collect();
        return format!(
            "todo!(\"PARTEVALFUNCTION of {func}: named args do not match any formal: {:?}\")",
            bad
        );
    }

    let params_list = closure_params.join(", ");
    let call_expr = format!("{func_str}({})", call_arg_exprs.join(", "));
    let closure = format!("move |{params_list}| {call_expr}");

    if captures.is_empty() {
        closure
    } else {
        format!("{{ {} {closure} }}", captures.join(" "))
    }
}

/// Emit a reduction expression as a Rust iterator pipeline.
///
/// Strategy:
///   1. Build the iterator over the cartesian product (or zip for `threaded`) of
///      the iterators' ranges, with each binding produced in declaration order.
///   2. Apply each iterator's optional `guard` as a `.filter(...)` step.
///   3. Map each tuple of bindings through `body`.
///   4. Apply the reduction (sum/product/min/max/collect/fold) according to `func`.
///
/// For user-defined reductions the function must declare its accumulator's default
/// value; we look that up via `resolve_call_formals` and synthesize a `fold`. If
/// the lookup fails we emit a `todo!(...)` so the missing default is visible at
/// compile time of the generated code.
fn emit_reduction<'a>(
    func: &str,
    body: &TypedExp,
    iterators: &[ReductionIter],
    iter_kind: ReductionIterKind,
    ty: &Ty,
    is_const: bool,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> String {
    if iterators.is_empty() {
        return format!("todo!(\"empty-iterator reduction {func}\")");
    }

    // We lower reductions to a block expression containing one or more `for`
    // loops driving an accumulator. This is more efficient than an iterator
    // chain (no `.collect()` to a temporary `Vec` for `list(...)`, no double
    // collect for nested reductions) and — crucially — lets the body and
    // guards use `?` directly: the loop body is plain function-context Rust,
    // so a fallible call inside it propagates with `?` rather than needing
    // `.unwrap()` inside a `filter`/`map` closure (which would also discard
    // the `Result` and not compile).
    //
    // Shape:
    //
    //   {
    //       let mut __acc: <AccTy> = <seed>;
    //       for <pat1> in <range1>... {
    //           if !(<guard1>) { continue; }   // when guard present
    //           for <pat2> in <range2>... {    // for Combine (cartesian)
    //               if !(<guard2>) { continue; }
    //               let __x = <body>;
    //               <update __acc with __x>;
    //           }
    //       }
    //       <finalize __acc>
    //   }
    //
    // For `Thread` iterators we collapse multiple iterators into a single
    // `.zip` chain and emit one for-loop that destructures all bindings —
    // matching MetaModelica's parallel-iteration semantics.

    // Build the for-loop opening for one iterator.
    fn open_for(it: &ReductionIter, is_const: bool, ctx: &mut GenCtx, top_level: &BTreeMap<String, NameNode>) -> String {
        let range_s = emit_exp(&it.range, is_const, ctx, top_level);
        let iter_expr = match it.range.ty() {
            // Lists yield &T; clone so the loop body owns its element (matches
            // the rest of the generated code which clones liberally).
            Ty::List(_) => format!("({range_s}).into_iter().cloned()"),
            Ty::Array(_) => format!("({range_s}).borrow().iter()"),
            _ => format!("({range_s}).into_iter()"),
        };
        format!("for {} in {iter_expr} {{\n", escape_ident(&it.name))
    }

    // Emit guard check as `if !(...) { continue; }`. Guards may be fallible,
    // so they're emitted under the caller's current qmode — `?` (Function),
    // `unwrap_break_err!` (TryBlock), etc. — same as any other call site.
    fn guard_check(it: &ReductionIter, is_const: bool, ctx: &mut GenCtx, top_level: &BTreeMap<String, NameNode>, indent: &str) -> String {
        match &it.guard {
            None => String::new(),
            Some(g) => {
                let s = emit_exp(g, is_const, ctx, top_level);
                format!("{indent}if !({s}) {{ continue; }}\n")
            }
        }
    }

    let body_ty = body.ty();
    // Render a type for an accumulator slot. If our inferred type isn't known
    // (typedexp can't always propagate through nested reductions / fallible
    // bodies), emit `_` so Rust infers from the loop body — that's strictly
    // better than `/* ? */`, which `fmt_ty` produces for `Ty::Unknown` and
    // which is not valid Rust in a type position.
    fn ty_or_underscore(t: &Ty, ctx: &mut GenCtx) -> String {
        if matches!(t, Ty::Unknown) {
            "_".to_owned()
        } else {
            // `fmt_ty` renders nested `Ty::Unknown` as `/* ? */`, which is not
            // valid in a type position once the comment is stripped. Patch any
            // such occurrences to `_` so Rust's inference fills them in.
            fmt_ty(t, ctx).replace("/* ? */", "_")
        }
    }
    // Determine the accumulator declaration and the update statement.
    // `acc_decl` is the `let mut __acc...` line; `update` is the per-iteration
    // assignment using the in-scope `__x` (the body's value); `finalize` is
    // the trailing expression that yields the block's value.
    let (acc_decl, update, finalize): (String, String, String) = match func {
        "list" => {
            // List<T> in forward iteration order. Cons inside the loop builds
            // a reversed list (no intermediate Vec); a single `.reverse()` at
            // the end flips it back to forward order. This is one linear pass
            // over the cons-cells and avoids the Vec allocation entirely.
            let elem_ty = match ty { Ty::List(t) => ty_or_underscore(t, ctx), _ => "_".to_owned() };
            (
                format!("let mut __acc: Arc<metamodelica::List<{elem_ty}>> = metamodelica::nil();"),
                "__acc = cons(__x, __acc);".to_owned(),
                "__acc.reverse()".to_owned(),
            )
        }
        "listReverse" => {
            // Reverse-iteration order: cons directly onto the accumulator.
            let elem_ty = match ty { Ty::List(t) => ty_or_underscore(t, ctx), _ => "_".to_owned() };
            (
                format!("let mut __acc: Arc<metamodelica::List<{elem_ty}>> = metamodelica::nil();"),
                "__acc = cons(__x, __acc);".to_owned(),
                "__acc".to_owned(),
            )
        }
        "sum" => {
            // MetaModelica's `sum` is overloaded: numeric addition for Integer/Real,
            // string concatenation for String. We discriminate on the body type so
            // a sum-of-strings builds an ArcStr via a String buffer (matching the
            // pattern used elsewhere in this file for string concatenation), rather
            // than falling back to numeric addition with the wrong accumulator type.
            if matches!(body_ty, Ty::Str) {
                (
                    "let mut __acc = String::new();".to_owned(),
                    "__acc.push_str(&__x);".to_owned(),
                    "ArcStr::from(__acc)".to_owned(),
                )
            } else {
                let ty_s = numeric_sum_ty(&body_ty);
                let zero = if ty_s == "f64" { "0.0" } else { "0" };
                (
                    format!("let mut __acc: {ty_s} = {zero};"),
                    "__acc += __x;".to_owned(),
                    "__acc".to_owned(),
                )
            }
        }
        "product" => {
            let ty_s = numeric_sum_ty(&body_ty);
            let one = if ty_s == "f64" { "1.0" } else { "1" };
            (
                format!("let mut __acc: {ty_s} = {one};"),
                "__acc *= __x;".to_owned(),
                "__acc".to_owned(),
            )
        }
        "min" | "max" => {
            // Empty reduction is a runtime error: surface it as Result via
            // the caller's qmode so it joins the surrounding error path.
            let cmp = if func == "min" { "<" } else { ">" };
            let elem_ty = ty_or_underscore(&body_ty, ctx);
            let final_expr = ctx.q(&format!("__acc.ok_or_else(|| anyhow::anyhow!(\"empty {func} reduction\"))"));
            (
                format!("let mut __acc: Option<{elem_ty}> = None;"),
                format!("__acc = Some(match __acc {{ None => __x, Some(__cur) => if __x {cmp} __cur {{ __x }} else {{ __cur }} }});"),
                final_expr,
            )
        }
        "listAppend" => {
            // `listAppend(elem for ...)` accumulates by appending each element
            // (a list) to the previously-accumulated list. `__x.append(&acc)`
            // matches the existing helper's argument order: prepend __x onto acc.
            let inner_ty = match ty { Ty::List(t) => ty_or_underscore(t, ctx), _ => "_".to_owned() };
            (
                format!("let mut __acc: Arc<metamodelica::List<{inner_ty}>> = metamodelica::nil();"),
                "__acc = __x.append(&__acc);".to_owned(),
                "__acc".to_owned(),
            )
        }
        _ => {
            // User-defined reduction: resolve the accumulator default from the
            // function's formal parameter list. We assume the accumulator is the
            // last (or default-valued) parameter and that the call shape is
            // `f(elem, acc)` — same as the built-ins above.
            let formals = resolve_call_formals(func, ctx, top_level);
            let default_expr: Option<String> = formals
                .as_ref()
                .and_then(|fs| fs.iter().rev().find_map(|(_, _, d)| d.clone()))
                .map(|tpl| emit_exp(&tpl, is_const, ctx, top_level));
            match default_expr {
                Some(seed) => {
                    let fname = if func.contains('.') {
                        ctx.shorten(func)
                    } else {
                        func.to_owned()
                    };
                    let fname = escape_ident(&fname);
                    let acc_ty = ty_or_underscore(ty, ctx);
                    let call_expr = format!("{fname}(__x, __acc)");
                    let update = format!("__acc = {};", ctx.q(&call_expr));
                    (
                        format!("let mut __acc: {acc_ty} = {seed};"),
                        update,
                        "__acc".to_owned(),
                    )
                }
                None => {
                    // Unknown reduction operator with no resolvable default: emit a
                    // todo! so the generated code's compile failure points us at the
                    // specific reduction that still needs lowering.
                    return format!("todo!(\"reduction {func}: cannot resolve default value\")");
                }
            }
        }
    };

    // Build the nested for-loops. Thread iterators collapse to a single zipped
    // loop; Combine iterators nest.
    //
    // Indentation matches the rest of the codegen's expression output: assume
    // the call site is at 4-space function-body indent, so the block's contents
    // start at 8 spaces and nest from there. The closing brace is at 4 spaces
    // so that `<lhs> = <block>;` reads as a properly-nested assignment. This
    // is the same convention `emit_match` uses for its multi-line arms.
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str(&format!("        {acc_decl}\n"));
    let body_s = emit_exp(body, is_const, ctx, top_level);

    match iter_kind {
        ReductionIterKind::Thread => {
            // Build a zipped iterator over all ranges and one for-loop that
            // destructures the bindings as a (possibly nested) tuple.
            let mut zip_expr = String::new();
            for (i, it) in iterators.iter().enumerate() {
                let range_s = emit_exp(&it.range, is_const, ctx, top_level);
                let part = match it.range.ty() {
                    Ty::List(_) => format!("(&({range_s})).into_iter()"),
                    _ => format!("({range_s}).into_iter()"),
                };
                if i == 0 {
                    zip_expr = part;
                } else {
                    zip_expr = format!("{zip_expr}.zip({part})");
                }
            }
            let pat = iter_pattern(iterators);
            s.push_str(&format!("        for {pat} in {zip_expr} {{\n"));
            for it in iterators {
                s.push_str(&guard_check(it, is_const, ctx, top_level, "            "));
            }
            s.push_str(&format!("            let __x = {body_s};\n"));
            s.push_str(&format!("            {update}\n"));
            s.push_str("        }\n");
        }
        ReductionIterKind::Combine => {
            // Cartesian: nest the loops. Each iterator's guard is checked
            // immediately after its loop opens so we skip work as early as
            // possible (mirrors the MetaModelica semantics). Base indent is
            // 8 spaces (block body); each nested for-loop adds 4.
            let base = 2; // 2 * 4 = 8 spaces for the first for-loop
            let indents: Vec<String> = (0..iterators.len())
                .map(|d| "    ".repeat(base + d))
                .collect();
            for (i, it) in iterators.iter().enumerate() {
                s.push_str(&indents[i]);
                s.push_str(&open_for(it, is_const, ctx, top_level));
                s.push_str(&guard_check(it, is_const, ctx, top_level, &format!("{}    ", indents[i])));
            }
            let inner_indent = format!("{}    ", indents.last().unwrap());
            s.push_str(&format!("{inner_indent}let __x = {body_s};\n"));
            s.push_str(&format!("{inner_indent}{update}\n"));
            for i in (0..iterators.len()).rev() {
                s.push_str(&indents[i]);
                s.push_str("}\n");
            }
        }
    }

    s.push_str(&format!("        {finalize}\n"));
    s.push_str("    }");
    s
}

/// Build the binding pattern produced by the current iterator chain.
/// For a single iterator: just the iterator's identifier.
/// For multiple iterators with `flat_map` chaining: a nested tuple `((a, b), c)`
/// in left-fold order, matching what `iter_pair` constructs below.
fn iter_pattern(iters: &[ReductionIter]) -> String {
    match iters.len() {
        0 => "_".to_owned(),
        1 => escape_ident(&iters[0].name),
        _ => {
            // Build `((((i0, i1), i2), i3), ...)` left-associatively.
            let mut s = escape_ident(&iters[0].name);
            for it in &iters[1..] {
                s = format!("({s}, {})", escape_ident(&it.name));
            }
            s
        }
    }
}

/// Build the tuple value produced by a nested `flat_map` to forward bindings.
/// Mirrors `iter_pattern` so destructuring matches construction.
fn iter_pair(prev: &[ReductionIter], cur: &str) -> String {
    let prev_pat = iter_pattern(prev);
    format!("({prev_pat}, {})", escape_ident(cur))
}

/// Pick the numeric type used in `Iterator::sum::<T>()` / `product::<T>()`.
/// Falls back to `i32` for Unknown so the generated code remains type-checkable;
/// the wider value path will surface any mismatch at compile time.
fn numeric_sum_ty(ty: &Ty) -> &'static str {
    match ty {
        Ty::F64 => "f64",
        Ty::I32 => "i32",
        _ => "i32",
    }
}

/// Look up the declared formal-parameter type of a builtin by index, using the
/// `typedexp::builtin_function_ty` registry as the single source of truth.
/// Returns `None` if the builtin has no entry (e.g. constructor-shaped builtins
/// such as `SOME`, `list`, or `SOURCEINFO` whose argument shapes are
/// context-dependent and don't have a static formal type).
fn builtin_formal_ty(func: &str, idx: usize) -> Option<Ty> {
    if let Some(Ty::Function { inputs, .. }) = typedexp::builtin_function_ty(func) {
        return inputs.get(idx).map(|i| i.ty.clone());
    }
    None
}

/// Emit a builtin call argument with MetaModelica's implicit tuple→first
/// coercion applied when the actual argument is a tuple and the formal expects
/// a scalar. The clone-for-value behavior matches `emit_cloned_call_arg`.
fn emit_builtin_call_arg<'a>(
    func: &str,
    idx: usize,
    arg: &TypedExp,
    is_const: bool,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> String {
    let formal = builtin_formal_ty(func, idx);
    emit_call_arg_with_formal(arg, formal.as_ref(), is_const, ctx, top_level)
}

/// Like `emit_builtin_call_arg` but without the clone — for argument positions
/// where the surrounding builtin emission already takes a reference or a Copy
/// scalar (e.g. an index, a boolean flag). The tuple→`.0` coercion is still
/// applied so a tuple-returning call passed in scalar context is unpacked.
fn emit_builtin_call_arg_raw<'a>(
    func: &str,
    idx: usize,
    arg: &TypedExp,
    is_const: bool,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> String {
    let formal = builtin_formal_ty(func, idx);
    let needs_first = matches!(arg.ty(), Ty::Tuple(_))
        && matches!(&formal, Some(t) if !matches!(t, Ty::Tuple(_) | Ty::Unknown));
    let raw = emit_exp(arg, is_const, ctx, top_level);
    if needs_first { format!("({raw}).0") } else { raw }
}

fn emit_builtin_call<'a>(func: &str, args: &[TypedExp], is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> Result<String> {
    // All argument emission below goes through `emit_builtin_call_arg{,_raw}`,
    // which consults `typedexp::builtin_function_ty` and applies MetaModelica's
    // implicit tuple→first coercion when a tuple-returning call is passed where
    // a scalar is expected (e.g. `intMod(hashfn(k), len)` where `hashfn` returns
    // a single value but a future signature change to a tuple would still emit
    // valid code at this site). Builtins without a registry entry — `SOME`,
    // `NONE`, `list`, `SOURCEINFO`, etc. — get `None` as the formal type and
    // fall back to the prior non-coercing behavior, which is fine for them.
    match func {
        "SOME" => {
            let arg = args
                .first()
                .map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level))
                .unwrap_or_default();
            Ok(format!("Some({arg})"))
        }
        "NONE" => Ok("None".to_owned()),
        "fail" => if is_const { Ok("{ panic!(\"fail\") }".to_owned()) } else { Ok("bail!(\"fail\")".to_owned()) },
        "sourceInfo" if args.is_empty() => {
            // MetaModelica's `sourceInfo()` returns a SourceInfo populated from the
            // *compiler* call-site — i.e. the location in the .mo source. We emit
            // the `sourceInfo!()` macro, which uses Rust's `file!()`/`line!()` to
            // capture the Rust call-site (i.e. the generated .rs file). For a
            // bootstrap that's the closest equivalent: the Rust file lines map
            // 1:1 to the original MetaModelica statements.
            Ok("metamodelica::sourceInfo!()".to_owned())
        }
        "list" => {
            if args.is_empty() {
                Ok("metamodelica::nil()".to_owned())
            } else {
                let parts: Vec<String> = args.iter().enumerate()
                    .map(|(i, a)| emit_builtin_call_arg(func, i, a, is_const, ctx, top_level))
                    .collect();
                Ok(format!("metamodelica::list![{}]", parts.join(", ")))
            }
        },
        "min" | "max" => {
            if args.len() == 2 {
                let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
                let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
                Ok(format!("std::cmp::{func}({arg1}, {arg2})"))
            } else {
                let parts: Vec<String> = args.iter().enumerate()
                    .map(|(i, a)| emit_builtin_call_arg_raw(func, i, a, is_const, ctx, top_level))
                    .collect();
                Ok(format!("{func}({})", parts.join(", ")))
            }
        },
        "String" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("format!(\"{{}}\", {arg})"))
        },
        "stringGet" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(ctx.q(&format!("stringGet({},{})", arg1, arg2)))
        },
        "realNeg" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("-( {} as f64)", arg1))
        },
        "realMul" | "realAdd" | "realSub" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            let op = match func {
                "realMul" => "*",
                "realAdd" => "+",
                "realSub" => "-",
                _ => unreachable!()
            };
            Ok(format!("({} as f64) {} ({} as f64)", arg1, op, arg2))
        },
        "realInt" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("(({arg}) as i32)"))
        },
        // Integer(x) is a Modelica/MetaModelica built-in type cast.
        // For Real → Integer: floor to i32.
        // For Enumeration → Integer: the discriminant (as i32).
        // For Boolean → Integer: false=0, true=1.
        "Integer" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            // Check the argument type to emit the right conversion.
            match args.first().map(|a| a.ty()).as_ref() {
                Some(crate::hierarchy::Ty::F64) => Ok(format!("(({arg}) as i32)")),
                Some(crate::hierarchy::Ty::Bool) => Ok(format!("(({arg}) as i32)")),
                Some(crate::hierarchy::Ty::Enumeration(_)) => Ok(format!("(({arg}) as i32)")),
                // Unknown argument type — emit a generic cast; it may need manual review.
                _ => Ok(format!("(({arg}) as i32 /* Integer(...) with unknown arg type */)")),
            }
        },
        "print" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("println!(\"{{}}\", {arg})"))
        },
        "arrayGet" | "arrayGetNoBoundsChecking" => {
            // `arr` is `metamodelica::Array<T>` = `Rc<RefCell<Vec<T>>>`.
            // `.borrow()` returns a `Ref<Vec<T>>` whose lifetime extends to the end
            // of the enclosing expression, so the inline index + clone is sound.
            // NoBoundsChecking falls back to checked indexing here; lifting it to
            // `get_unchecked` would require a longer-lived borrow scope. TODO if profiling demands it.
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{}.borrow()[({}-1) as usize].clone()", arg1, arg2))
        },
        "valueEq" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{} == {}", arg1, arg2))
        },
        "arrayLength" => {
            // `Array<T>` is `Rc<RefCell<Vec<T>>>` so we go through `.borrow()`.
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("({}.borrow().len() as i32)", arg))
        },
        "listLength" | "stringLength" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("({}.len() as i32)", arg))
        },
        "floor" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{}.floor()", arg))
        },
        "mod" if args.len()==2 => {
            let a1 = args.get(0).unwrap();
            let a2 = args.get(1).unwrap();
            let f = if a1.ty() == Ty::I32 && a2.ty() == Ty::I32 {"intMod"} else {"realMod"};
            // Route through the typed-formal helper for the resolved underlying
            // builtin (intMod/realMod) so a tuple-returning operand still gets
            // unpacked via `.0` against the correct scalar formal type.
            let arg1 = emit_builtin_call_arg(f, 0, a1, is_const, ctx, top_level);
            let arg2 = emit_builtin_call_arg(f, 1, a2, is_const, ctx, top_level);
            Ok(ctx.q(&format!("{f}({arg1}, {arg2})")))
        },
        "div" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{arg1} / {arg2}"))
        },
        "abs" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{}.abs()", arg))
        },
        "integer" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("(({} as f64).floor() as i32)", arg))
        },
        "listHead" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(ctx.q(&format!("listHead({})", arg)))
        },
        "listRest" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(ctx.q(&format!("listRest({})", arg)))
        },
        "listGet" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(ctx.q(&format!("({arg1}).get({arg2})")))
        },
        "referenceEq" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(ctx.q(&format!("referenceEq(&{arg1},&{arg2})")))
        },
        "isPresent" => {
            Ok(format!("true /* isPresent not implemented in Rust */"))
        },
        "listReverse" | "listReverseInPlace" => {
            let arg = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{}.reverse()", arg))
        },
        "arrayCopy" => {
            // Deep (by-element) copy: a fresh Array not aliasing the source.
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("metamodelica::arrayFromVec({}.borrow().clone())", arg))
        },
        // MetaModelica.Dangerous.arrayCreateNoInit(size, dummy): the `dummy`
        // argument is a type witness only in the MM signature — the Rust
        // counterpart is generic, so we drop the second argument here. The
        // function is fallible (returns Result), hence wrapped with `?` via
        // ctx.q for non-const contexts.
        "arrayCreateNoInit" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let call = format!("metamodelica::Dangerous::arrayCreateNoInit({arg1})");
            if is_const { Ok(call) } else { Ok(ctx.q(&call)) }
        },
        "arrayClearIndex" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            let call = format!("metamodelica::Dangerous::arrayClearIndex({arg1}, {arg2})");
            if is_const { Ok(call) } else { Ok(ctx.q(&call)) }
        },
        "arrayUpdate"| "arrayUpdateNoBoundsChecking" => {
            // MM semantics: mutate in place, return the same array (aliases see the change).
            // We bind the array once so {arg1} (which may be a non-trivial expression) is
            // evaluated only once, mutate through `borrow_mut()`, then yield the same Rc.
            // NoBoundsChecking uses checked indexing here; same TODO as arrayGet.
            let arg1 = args.get(0).map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg3 = args.get(2).map(|a| emit_builtin_call_arg(func, 2, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{{let _arr = {}; _arr.borrow_mut()[({}-1) as usize] = {}; _arr}}", arg1, arg2, arg3))
        },
        "arrayEmpty" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{}.borrow().is_empty()", arg))
        },
        "listEmpty" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{}.is_empty()", arg))
        },
        "SOURCEINFO" | "SourceInfo" => {
            let a0 = args.get(0).map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_else(|| "Arc::new(\"\".to_string())".to_owned());
            let a1 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_else(|| "false".to_owned());
            let a2 = args.get(2).map(|a| emit_builtin_call_arg_raw(func, 2, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a3 = args.get(3).map(|a| emit_builtin_call_arg_raw(func, 3, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a4 = args.get(4).map(|a| emit_builtin_call_arg_raw(func, 4, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a5 = args.get(5).map(|a| emit_builtin_call_arg_raw(func, 5, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a6 = args.get(6).map(|a| emit_builtin_call_arg_raw(func, 6, a, is_const, ctx, top_level)).unwrap_or_else(|| "0.0".to_owned());
            Ok(format!(
                "SourceInfo {{ fileName: {a0}, isReadOnly: {a1}, lineNumberStart: {a2}, columnNumberStart: {a3}, lineNumberEnd: {a4}, columnNumberEnd: {a5}, lastModification: {a6} }}"
            ))
        },
        "listArray" =>{
            // List -> Array: collect into a Vec, then wrap into the shared-mutable Array.
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("metamodelica::arrayFromVec({arg}.into_iter().cloned().collect())"))
        },
        "arrayList" /* | "stringAppendList" */ => {
            // Array -> List: borrow the inner Vec, clone elements into a fresh List.
            let arg = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("Arc::new({arg}.borrow().iter().cloned().collect())"))
        },
        _ => bail!("Not a builtin function")
    }
}

/// Emit a variable reference, handling subscripts and the package/field boundary.
fn emit_var<'a>(
    name: &str,
    segments: &[CrefSegment],
    _ty: &Ty,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> String {

    // If segments is empty but name has dots, it means the name contains the whole path.
    // We should split it into segments so we can resolve the package/field boundary.
    let mut real_segments = segments.to_vec();
    let name_str = name.to_owned();
    if real_segments.is_empty() && name_str.contains('.') {
        for part in name_str.split('.') {
            real_segments.push(CrefSegment {
                name: part.to_owned(),
                subscripts: vec![],
            });
        }
    }

    // Apply subscripts on the first (base) segment as 1-based indexing.
    //
    // The base may be either a `Vec<T>`-shaped value (e.g. fn-call result whose
    // type we don't see here) or a `metamodelica::Array<T>` = `Rc<RefCell<Vec<T>>>`.
    // For Array, we must go through `.borrow()` before subscripting. We detect
    // that case by consulting `ctx.fn_env_vars`, which holds the declared types
    // of every input/output/protected local of the enclosing function.
    //
    // Limitation: match-arm pattern bindings aren't reflected in fn_env_vars, so
    // an Array bound by a pattern won't get `.borrow()` here. That falls under
    // the broader "thread more type info through CrefSegment" cleanup; today it
    // produces a compile error pointing at the call site, which is what we want
    // rather than silent wrong codegen.
    let base_name = if real_segments.is_empty() {
        name_str.clone()
    } else {
        let mut base = escape_ident(&real_segments[0].name);
        if !real_segments[0].subscripts.is_empty() {
            if matches!(ctx.fn_env_vars.get(&real_segments[0].name), Some(Ty::Array(_))) {
                base = format!("{base}.borrow()");
            }
            for sub in &real_segments[0].subscripts {
                base = format!("{}[({}-1) as usize]", base, emit_exp(sub, false, ctx, top_level));
            }
        }
        base
    };

    // If there are no further segments, we're done.
    if real_segments.len() <= 1 {
        let only_name = if real_segments.is_empty() { name_str.clone() } else { real_segments[0].name.clone() };
        if !only_name.contains('.') {
            if real_segments.is_empty() && only_name == "child" {
                return "node".to_owned();
            }
            return if real_segments.is_empty() { escape_ident(&only_name) } else { base_name };
        }
    }

    // Multiple segments// Multiple segments: find the package/field boundary.
// Multiple segments: find the package/field boundary.
    // Walk backwards from the segments to find the deepest record type prefix.
    // Everything before the record is a Rust path (::); everything from the record
    // onwards uses field access (.).
    let split_idx = find_record_split(&real_segments, ctx, top_level);

    let (pkg_segs, field_segs) = real_segments.split_at(split_idx);


    // Emit the package prefix part using shorten.
    let pkg_dotted: String = pkg_segs.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".");
    let base = if pkg_dotted.is_empty() {
        base_name
    } else {
        let shortened = ctx.shorten(&pkg_dotted);
        if shortened == "List::Nil" {
            return "metamodelica::nil()".to_owned();
        }
        // Apply subscripts from the last package segment.
        let last_pkg_segs = pkg_segs.last();
        if let Some(last_seg) = last_pkg_segs {
            if !last_seg.subscripts.is_empty() {
                let mut b = escape_ident(&shortened);
                for sub in &last_seg.subscripts {
                    b = format!("{}[({}-1) as usize]", b, emit_exp(sub, false, ctx, top_level));
                }
                b
            } else {
                escape_ident(&shortened)
            }
        } else {
            escape_ident(&shortened)
        }
    };

    // Emit field access for the remaining segments.
    //
    // Special case: if the base is a single local variable known to currently
    // hold a specific uniontype variant (e.g. inside a `match` arm whose
    // pattern fixed that variable's variant), the enum has no `.field`
    // directly — the field lives inside the matched record. We lower the
    // first field access through `var_field!`, which destructures the enum
    // and yields a reference to the field. Subsequent field accesses on the
    // returned reference auto-deref normally.
    //
    // Only triggered when the variable base actually resolves to an enum-like
    // type; package/module bases are handled by the regular field-emit path.
    let mut res = base.clone();
    let mut field_iter = field_segs.iter();
    if pkg_segs.len() == 1 && !field_segs.is_empty() {
        let var_name = &pkg_segs[0].name;
        if let Some((enum_qname, variant_name)) = ctx.variants.get(var_name).cloned() {
            let first = field_iter.next().unwrap();
            // Resolve the enum path through the same shorten/import machinery
            // we use for constructor calls so the emitted path is valid in the
            // current file's import scope.
            let variant_path = build_variant_path(&enum_qname, &variant_name, ctx);
            // Determine the binding shape. Explicit shapes (from nested-As
            // pattern bindings) take precedence; otherwise fall back to the
            // function-level variable type via `fn_env_vars`.
            let shape = ctx.variant_shapes.get(var_name).copied().unwrap_or_else(|| {
                let var_ty = ctx.fn_env_vars.get(var_name).cloned().unwrap_or(Ty::Unknown);
                if is_arc_wrapped(&var_ty, ctx) { VarShape::Arc } else { VarShape::Owned }
            });
            let field_id = escape_ident(&first.name);
            let macro_call = match shape {
                VarShape::Owned  => format!("{base}.{field_id}"),
                VarShape::Arc    => format!("(*{base}).{field_id}"),
                VarShape::RefArc => format!("(**{base}).{field_id}"),
            };
            res = format!("var_field!({macro_call}, {variant_path})");
            for sub in &first.subscripts {
                res = format!("{}[({}-1) as usize]", res, emit_exp(sub, false, ctx, top_level));
            }
        }
    }

    for seg in field_iter {
        res = format!("{}.{}", res, escape_ident(&seg.name));
        for sub in &seg.subscripts {
            res = format!("{}[({}-1) as usize]", res, emit_exp(sub, false, ctx, top_level));
        }
    }
    res
}

/// Find the index at which to split segments into [package prefix] and [record fields].
/// Returns the index of the first field segment. Everything before is the package path.
/// Returns `segments.len()` if no record boundary is found (entire path is a package path).
fn resolve_fully_qualified<'a>(prefix_dotted: &str, ctx: &GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> Option<&'a NameNode<'a>> {
    // 1. Literal top level
    if let Some(n) = lookup_node(prefix_dotted, top_level) {
        return Some(n);
    }
    // 2. Relative to the full current path (top_name + current_path + suffix), walking up.
    // `ctx.current_path` alone is not enough because `lookup_node` needs top-level keys.
    // Build the full path by combining `top_name` with each scope level.
    {
        let mut scope_parts: Vec<&str> = std::iter::once(ctx.top_name.as_str())
            .chain(ctx.current_path.iter().map(|s| s.as_str()))
            .collect();
        while !scope_parts.is_empty() {
            let path = format!("{}.{}", scope_parts.join("."), prefix_dotted);
            if let Some(n) = lookup_node(&path, top_level) {
                return Some(n);
            }
            scope_parts.pop();
        }
    }
    // 2b. Relative to top_name
    let path_top = format!("{}.{}", ctx.top_name, prefix_dotted);
    if let Some(n) = lookup_node(&path_top, top_level) {
        return Some(n);
    }
    // 3. Named imports (reverse lookup: named values are the local names)
    // Wait, ctx.named maps "Fully.Qualified" => "Local"
    for (fq, local) in &ctx.named {
        if prefix_dotted == local {
            if let Some(n) = lookup_node(fq, top_level) {
                return Some(n);
            }
        } else if let Some(rest) = prefix_dotted.strip_prefix(&format!("{}.", local)) {
            let path = format!("{}.{}", fq, rest);
            if let Some(n) = lookup_node(&path, top_level) {
                return Some(n);
            }
        }
    }
    // 4. Unqualified modules (.* imports)
    for unq in &ctx.unqual_modules {
        let path = format!("{}.{}", unq, prefix_dotted);
        if let Some(n) = lookup_node(&path, top_level) {
            return Some(n);
        }
    }
    None
}

fn find_record_split<'a>(segments: &[CrefSegment], ctx: &GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> usize {
    if segments.len() <= 1 {
        return segments.len(); // 0 or 1
    }

    if !segments[0].subscripts.is_empty() {
        return 1;
    }

    // Walk backwards from the longest possible split point
    for i in (1..=segments.len()).rev() {
        let prefix_dotted: String = segments[..i].iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".");

        // If the whole prefix actually resolves to something
        let resolved_opt = resolve_fully_qualified(&prefix_dotted, ctx, top_level);
        if let Some(node) = resolved_opt {
            if let crate::hierarchy::NodeKind::Class(c) = &node.kind {
                use mmwinnow::Absyn::Restriction::*;
                match c.restriction {
                    // uniontypes and enums act as namespaces for their constructors.
                    // If we found the uniontype/enum itself, the VERY NEXT segment is its constructor!
                    // so the rust module path covers up to the constructor.
                    mmwinnow::Absyn::Restriction::R_ENUMERATION | mmwinnow::Absyn::Restriction::R_UNIONTYPE => {
                        if i < segments.len() {
                            return i + 1;
                        } else {
                            return i;
                        }
                    }
                    // A package acts as a transparent namespace.
                    // If the path exactly resolves to a package, it's a module path.
                    mmwinnow::Absyn::Restriction::R_PACKAGE => return i,
                    // Records and classes have fields.
                    mmwinnow::Absyn::Restriction::R_RECORD | mmwinnow::Absyn::Restriction::R_METARECORD { .. } | mmwinnow::Absyn::Restriction::R_CLASS | mmwinnow::Absyn::Restriction::R_MODEL | mmwinnow::Absyn::Restriction::R_BLOCK | mmwinnow::Absyn::Restriction::R_CONNECTOR => {
                        return i;
                    }
                    // Functions, variables, or anything else: they are NOT Rust modules.
                    // The base element (the function/variable) is the start of the field access!
                    _ => {
                        // Keep looking for a shorter prefix that might be a package...
                        // But wait! If we found a function or a local variable,
                        // its parent MUST be the package. So `i` was not a package, but `i-1` might be.
                        // Actually, if it's a known non-package, the Rust module path ends BEFORE IT!
                        // Oh wait, if the Rust path is `UnorderedMap.map`, in Rust it is `UnorderedMap::map`.
                        // Then `map` is the variable/function. And `.keys` is a field.
                        // So the Rust package path is just `i`, wait...
                        // If it's `UnorderedMap::map`, the expression is `UnorderedMap::map`.
                        // The field is `keys`.
                        return i;
                    }
                }
            } else {
                // If it resolves but isn't a class (e.g. constant, etc), it's the base of a field access.
                return i;
            }
        }
    }

    // Fallback: assume the first segment is a variable/module and everything else is fields.
    1
}

/// Flatten a chain of string `Add` expressions into a list of individual string parts.
/// e.g. `(a + b) + c` → `["a", "b", "c"]`
fn collect_string_concat_parts<'a>(exp: &TypedExp, is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>, parts: &mut Vec<String>) {
    if let TypedExp::BinOp { op: BinOpKind::Add, ty: Ty::Str, lhs, rhs, .. } = exp {
        collect_string_concat_parts(lhs, is_const, ctx, top_level, parts);
        collect_string_concat_parts(rhs, is_const, ctx, top_level, parts);
    } else {
        parts.push(emit_exp(exp, is_const, ctx, top_level));
    }
}

fn maybe_clone_string_value(expr: String, ty: &Ty) -> String {
    if matches!(ty, Ty::Str) {
        format!("({expr}).clone()")
    } else {
        expr
    }
}

fn emit_cloned_call_arg<'a>(arg: &TypedExp, is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    let arg_str = emit_exp(arg, is_const, ctx, top_level);
    // In const/static context the value is consumed by-value at compile time;
    // wrapping with `.clone()` would break const evaluation (e.g. `literal!("")`
    // is a const ArcStr but `literal!("").clone()` is not a const expression).
    if is_const { arg_str } else { maybe_clone_string_value(arg_str, &arg.ty()) }
}

/// MetaModelica implicitly extracts the first element of a tuple-returning call
/// when it is used in a single-value context (e.g. as an argument whose formal
/// expects a non-tuple type). We mirror that here by wrapping the emitted call
/// in `.0` when (and only when) the actual argument's static type is a tuple
/// but the formal expects a non-tuple. The check is conservative: if the formal
/// type is itself a tuple (or unknown), we leave the expression alone so we
/// don't break legitimate tuple-passing.
fn emit_call_arg_with_formal<'a>(
    arg: &TypedExp,
    formal_ty: Option<&Ty>,
    is_const: bool,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> String {
    let needs_first = matches!(arg.ty(), Ty::Tuple(_))
        && matches!(formal_ty, Some(t) if !matches!(t, Ty::Tuple(_) | Ty::Unknown));
    if !needs_first {
        return emit_cloned_call_arg(arg, is_const, ctx, top_level);
    }
    // Emit the call (with `?` propagation already attached by emit_exp via ctx.q),
    // wrap in parens so `.0` binds to the whole call result, then re-apply the
    // clone-for-string heuristic against the *first element's* type.
    let raw = emit_exp(arg, is_const, ctx, top_level);
    let first_ty = match arg.ty() {
        Ty::Tuple(elems) => elems.into_iter().next().unwrap_or(Ty::Unknown),
        _ => Ty::Unknown,
    };
    let extracted = format!("({raw}).0");
    if is_const { extracted } else { maybe_clone_string_value(extracted, &first_ty) }
}

/// Resolve a function name as written at a call site to a fully-qualified dotted
/// name in the hierarchy.
fn resolve_call_qname<'a>(
    func: &str,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<String> {
    if func.is_empty() {
        return None;
    }

    let mut exists = |name: &str| lookup_node(name, top_level).is_some();

    if func.contains('.') {
        if exists(func) {
            return Some(func.to_owned());
        }
        // Handle named-import aliases in the first path segment.
        let mut parts = func.splitn(2, '.');
        let head = parts.next().unwrap_or(func);
        let tail = parts.next().unwrap_or("");
        for (dotted, local) in &ctx.named {
            if local == head {
                let candidate = if tail.is_empty() {
                    dotted.clone()
                } else {
                    format!("{dotted}.{tail}")
                };
                if exists(&candidate) {
                    return Some(candidate);
                }
            }
        }
        return None;
    }

    let cur_prefix = if ctx.current_path.is_empty() {
        ctx.top_name.clone()
    } else {
        format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
    };

    // Resolve relative to the current module first, then walk outwards.
    let mut scope: &str = &cur_prefix;
    loop {
        let candidate = format!("{scope}.{func}");
        if exists(&candidate) {
            return Some(candidate);
        }
        match scope.rfind('.') {
            Some(dot) => scope = &scope[..dot],
            None => break,
        }
    }

    // Unqualified imports can bring module members into scope.
    for module in &ctx.unqual_modules {
        let candidate = format!("{module}.{func}");
        if exists(&candidate) {
            return Some(candidate);
        }
    }

    // Named import aliases can also denote modules.
    for (dotted, local) in &ctx.named {
        if local == func {
            if exists(dotted) {
                return Some(dotted.clone());
            }
            continue;
        }
        let candidate = format!("{dotted}.{func}");
        if exists(&candidate) {
            return Some(candidate);
        }
    }

    if exists(func) {
        Some(func.to_owned())
    } else {
        None
    }
}

/// Return function formals in declaration order with typed default expressions,
/// if any. Defaults are inferred in the callee scope so names/types match the
/// callee's declaration context.
/// Each entry is (formal name, formal type, optional default-value expression).
/// The `Ty` lets call-site codegen perform implicit conversions that depend on
/// the formal's expected shape (currently: extracting the first element when a
/// tuple-returning call is passed where a non-tuple value is expected — see
/// `maybe_implicit_first_tuple_elem`).
pub type CallFormal = (String, Ty, Option<TypedExp>);

fn resolve_call_formals<'a>(
    func: &str,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<Vec<CallFormal>> {
    // Builtins (isSome, listLength, intReal, ...) have no AST node in the hierarchy.
    // We still need their formal types so `emit_call_arg_with_formal` can apply the
    // implicit tuple→first coercion when a tuple-returning call is passed in a
    // scalar context (e.g. `isSome(find(...))` where `find` returns `(Option<T>, i32)`).
    // The registry in typedexp::builtin_function_ty is the single source of truth.
    let qname = match resolve_call_qname(func, ctx, top_level) {
        Some(q) => q,
        None => {
            if let Some(Ty::Function { inputs, .. }) = typedexp::builtin_function_ty(func) {
                return Some(inputs.into_iter().map(|inp| (inp.name, inp.ty, None)).collect());
            }
            return None;
        }
    };
    let node = lookup_node(&qname, top_level)?;
    let NodeKind::Class(c) = &node.kind else { return None };
    if !matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
        return None;
    }

    let module_prefix = qname.rsplit_once('.').map_or("", |(p, _)| p).to_owned();
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => &[],
    };

    let mut infer_env: HashMap<String, Ty> = HashMap::new();
    let mut out: Vec<CallFormal> = Vec::new();

    for member in members {
        let MM::ClassMember::Component(m) = member else { continue };
        if !matches!(m.direction, Absyn::Direction::INPUT | Absyn::Direction::INPUT_OUTPUT) {
            continue;
        }

        let ty = node.children.get(&m.name)
            .map(|n| n.ty.clone())
            .unwrap_or(Ty::Unknown);
        let mut fn_type_vars: Vec<String> = Vec::new();
        collect_type_vars_in_ty(&node.ty, &mut fn_type_vars);
        let default = extract_default_exp(&m.modification)
            .map(|exp| typedexp::infer_exp(exp, &infer_env, top_level, &module_prefix, &fn_type_vars));
        out.push((m.name.clone(), ty.clone(), default));
        infer_env.insert(m.name.clone(), ty);
    }

    // For `function extends` redeclarations, defaults can live on the inherited
    // base signature. Fill missing defaults from `base_fn` when available.
    if let Some(base_fn) = node.base_fn {
        let base_members: &[MM::ClassMember] = match &base_fn.body {
            MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
            _ => &[],
        };

        let mut base_infer_env: HashMap<String, Ty> = HashMap::new();
        for member in base_members {
            let MM::ClassMember::Component(m) = member else { continue };
            if !matches!(m.direction, Absyn::Direction::INPUT | Absyn::Direction::INPUT_OUTPUT) {
                continue;
            }

            let ty = node.children.get(&m.name)
                .map(|n| n.ty.clone())
                .unwrap_or(Ty::Unknown);
            let mut fn_type_vars: Vec<String> = Vec::new();
            collect_type_vars_in_ty(&node.ty, &mut fn_type_vars);
            let default = extract_default_exp(&m.modification)
                .map(|exp| typedexp::infer_exp(exp, &base_infer_env, top_level, &module_prefix, &fn_type_vars));

            if let Some(idx) = out.iter().position(|(name, _, _)| name == &m.name) {
                if out[idx].2.is_none() {
                    out[idx].2 = default;
                }
            } else {
                out.push((m.name.clone(), ty.clone(), default));
            }
            base_infer_env.insert(m.name.clone(), ty);
        }
    }

    if out.is_empty() {
        // Fallback: use resolved function type inputs when AST members are unavailable.
        if let Ty::Function { inputs, .. } = &node.ty {
            return Some(inputs.iter().map(|inp| (inp.name.clone(), inp.ty.clone(), None)).collect());
        }
        return None;
    }

    Some(out)
}

/// Substitute references to formal parameter names in `exp` using concrete
/// argument expressions already chosen for earlier slots.
fn substitute_formal_refs(exp: &TypedExp, bindings: &HashMap<String, TypedExp>) -> TypedExp {
    match exp {
        TypedExp::Lit(l) => TypedExp::Lit(l.clone()),
        TypedExp::Var { name, segments, ty } => {
            if segments.is_empty() && !name.contains('.') {
                if let Some(repl) = bindings.get(name) {
                    return repl.clone();
                }
            }
            let new_segments = segments.iter().map(|seg| CrefSegment {
                name: seg.name.clone(),
                subscripts: seg.subscripts.iter().map(|s| substitute_formal_refs(s, bindings)).collect(),
            }).collect();
            TypedExp::Var { name: name.clone(), segments: new_segments, ty: ty.clone() }
        }
        TypedExp::BinOp { op, lhs, rhs, ty } => TypedExp::BinOp {
            op: *op,
            lhs: Box::new(substitute_formal_refs(lhs, bindings)),
            rhs: Box::new(substitute_formal_refs(rhs, bindings)),
            ty: ty.clone(),
        },
        TypedExp::UnOp { op, operand, ty } => TypedExp::UnOp {
            op: *op,
            operand: Box::new(substitute_formal_refs(operand, bindings)),
            ty: ty.clone(),
        },
        TypedExp::Call { func, args, named_args, ty, sig_ty } => TypedExp::Call {
            func: func.clone(),
            args: args.iter().map(|a| substitute_formal_refs(a, bindings)).collect(),
            named_args: named_args.iter()
                .map(|(n, a)| (n.clone(), substitute_formal_refs(a, bindings)))
                .collect(),
            ty: ty.clone(),
            sig_ty: sig_ty.clone(),
        },
        TypedExp::Constructor { name, args, named_args, ty, field_names } => TypedExp::Constructor {
            name: name.clone(),
            args: args.iter().map(|a| substitute_formal_refs(a, bindings)).collect(),
            named_args: named_args.iter()
                .map(|(n, a)| (n.clone(), substitute_formal_refs(a, bindings)))
                .collect(),
            ty: ty.clone(),
            field_names: field_names.clone(),
        },
        TypedExp::If { cond, then_, elseif, else_, ty } => TypedExp::If {
            cond: Box::new(substitute_formal_refs(cond, bindings)),
            then_: Box::new(substitute_formal_refs(then_, bindings)),
            elseif: elseif.iter()
                .map(|(c, e)| (substitute_formal_refs(c, bindings), substitute_formal_refs(e, bindings)))
                .collect(),
            else_: Box::new(substitute_formal_refs(else_, bindings)),
            ty: ty.clone(),
        },
        TypedExp::Cons { head, tail, ty } => TypedExp::Cons {
            head: Box::new(substitute_formal_refs(head, bindings)),
            tail: Box::new(substitute_formal_refs(tail, bindings)),
            ty: ty.clone(),
        },
        TypedExp::Tuple(elems) => TypedExp::Tuple(elems.iter().map(|e| substitute_formal_refs(e, bindings)).collect()),
        TypedExp::Array { elems, ty } => TypedExp::Array {
            elems: elems.iter().map(|e| substitute_formal_refs(e, bindings)).collect(),
            ty: ty.clone(),
        },
        TypedExp::Match { kind, input, cases, ty } => TypedExp::Match {
            kind: *kind,
            input: Box::new(substitute_formal_refs(input, bindings)),
            cases: cases.iter().map(|c| typedexp::TypedCase {
                pattern: c.pattern.clone(),
                guard: c.guard.as_ref().map(|g| substitute_formal_refs(g, bindings)),
                locals: c.locals.clone(),
                stmts: c.stmts.clone(),
                result: substitute_formal_refs(&c.result, bindings),
            }).collect(),
            ty: ty.clone(),
        },
        TypedExp::Range { start, step, stop, elem_ty } => TypedExp::Range {
            start: Box::new(substitute_formal_refs(start, bindings)),
            step: step.as_ref().map(|s| Box::new(substitute_formal_refs(s, bindings))),
            stop: Box::new(substitute_formal_refs(stop, bindings)),
            elem_ty: elem_ty.clone(),
        },
        TypedExp::Reduction { func, body, iterators, iter_kind, ty } => TypedExp::Reduction {
            func: func.clone(),
            body: Box::new(substitute_formal_refs(body, bindings)),
            iterators: iterators.iter().map(|it| ReductionIter {
                name: it.name.clone(),
                range: substitute_formal_refs(&it.range, bindings),
                guard: it.guard.as_ref().map(|g| substitute_formal_refs(g, bindings)),
                elem_ty: it.elem_ty.clone(),
            }).collect(),
            iter_kind: *iter_kind,
            ty: ty.clone(),
        },
        TypedExp::PartEval { func, args, named_args, sig_ty, ty } => TypedExp::PartEval {
            func: func.clone(),
            args: args.iter().map(|a| substitute_formal_refs(a, bindings)).collect(),
            named_args: named_args.iter()
                .map(|(n, a)| (n.clone(), substitute_formal_refs(a, bindings)))
                .collect(),
            sig_ty: sig_ty.clone(),
            ty: ty.clone(),
        },
        TypedExp::Todo(s) => TypedExp::Todo(s.clone()),
    }
}

/// Emit a Modelica range expression (`start:step:stop` or `start:stop`) as a Rust iterator.
/// Modelica ranges are arithmetic progressions: start, start+step, ..., while within [start, stop].
/// Positive steps map to `(start..=stop).step_by(n)`.
/// Negative steps reverse the range: `(stop..=start).step_by(-n)`.
fn emit_range<'a>(start: &TypedExp, step: Option<&TypedExp>, stop: &TypedExp, is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    let s = emit_exp(start, is_const, ctx, top_level);
    let e = emit_exp(stop, is_const, ctx, top_level);

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
                        format!("({s}..={e}).step_by(({n}) as usize)")
                    };
                }
                // Negative step: reverse the range, negate the step.
                if *n < 0 {
                    return if *n == -1 {
                        format!("({e}..={s}).rev()")
                    } else {
                        format!("({e}..={s}).step_by(({}) as usize).rev()", -n)
                    };
                }
            }

            let step_val = emit_exp(step_exp, is_const, ctx, top_level);

            // Dynamic step: positive path for the common case,
            // with a runtime branch that reverses for negative steps.
            format!(
                "({{let __s={s}; let __e={e}; let __step={step_val}; if __step>0 {{__s..=__e}} else {{__e..=__s}}}}).step_by((if {step_val}>0 {{{step_val}}} else {{-({step_val})}}) as usize)"
            )
        }
    }
}

fn emit_match<'a>(kind: &MatchKind, input: &TypedExp, cases: &[TypedCase], is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    let input_str = emit_exp(input, is_const, ctx, top_level);
    let input_ty = input.ty();
    // If the match scrutinee is a recursive enum/struct wrapped in Arc, we must use
    // `.as_ref()` to obtain a `&T` reference that Rust can match against enum patterns.
    // Without this, the match subject is `Arc<T>` and the enum patterns cannot match it.
    // With `as_ref()`, Rust's match ergonomics automatically adds `ref` to pattern
    // bindings, so bound variables become `&FieldType`; `.clone()` on them still works.
    let input_is_arc = is_arc_wrapped(&input_ty, ctx);
    let match_subject = if input_is_arc {
        format!("{input_str}.as_ref()")
    } else {
        input_str.clone()
    };
    match kind {
        MatchKind::Match => {
            let has_wild = cases.iter().any(|c| matches!(c.pattern, TypedPat::Wildcard) && c.guard.is_none());
            let arms: Vec<String> = cases.iter().map(|case| {
                let pat = emit_pat_with_implicit_bind(&case.pattern, /*allow_implicit_bind=*/true, /*mut_bindings=*/true, /*in_deref=*/false, Some(&input_ty), ctx, top_level);
                // Compute the variant narrowing established by this arm's pattern so
                // that reads of `v.field` inside the guard and the case result use
                // `var_field!`. Save ctx.variants to restore after the arm so sibling
                // arms / nested matches don't see these bindings.
                let saved_variants = ctx.variants.clone();
                let saved_shapes = ctx.variant_shapes.clone();
                {
                    let mut tmp_env = LocalEnv::default();
                    let mut tmp_shapes: HashMap<String, VarShape> = HashMap::new();
                    record_pattern_variants_with_shapes(&case.pattern, input, &mut tmp_env, top_level, &mut tmp_shapes, ctx);
                    for (k, v) in tmp_env.variants {
                        ctx.variants.insert(k, v);
                    }
                    for (k, s) in tmp_shapes {
                        ctx.variant_shapes.insert(k, s);
                    }
                }
                let guard = case.guard.as_ref()
                    .map(|g| format!(" if {}", emit_exp(g, is_const, ctx, top_level)))
                    .unwrap_or_default();
                let result = emit_exp(&case.result, is_const, ctx, top_level);
                let arm_str = if case.stmts.is_empty() {
                    format!("        {pat}{guard} => {result}")
                } else {
                    // Seed the arm's local env from the enclosing function scope:
                    // inputs/outputs/protected are visible inside the arm body and
                    // assignments to them must be plain `name = expr;` rather than
                    // `let name = ...;`. Also propagate the function's output names
                    // so `return;` expands to `return Ok((outputs...));`.
                    let mut local_env = LocalEnv {
                        vars: ctx.fn_env_vars.clone(),
                        outputs: ctx.fn_outputs.clone(),
                        variants: HashMap::new(),
                    };
                    // The match arm guarantees that the scrutinee (when it is a
                    // simple variable reference) holds the matched variant for
                    // the duration of this arm — propagate that into the arm's
                    // local env so field assignments on it can use the
                    // variant-aware macro.
                    record_pattern_variants(&case.pattern, input, &mut local_env, top_level);
                    let mut fresh_local: u32 = 0;
                    let mut body = String::new();
                    // Pattern bindings are declared by the match arm itself (as `mut`
                    // bindings). Don't shadow them with `let mut <name>;` declarations
                    // in the arm body — that would create a separate variable, so any
                    // subsequent re-assignment to <name> would update the local copy
                    // and the original pattern binding (read by user code) would stay
                    // stale. See appendLastList's inner `l :: ll := ll;` loop.
                    let pat_binding_names: std::collections::HashSet<String> =
                        typedexp::pat_bindings(&case.pattern).iter().map(|(n, _)| n.clone()).collect();
                    for (name, ty, default) in &case.locals {
                        if matches!(ty, Ty::Unknown) {
                            continue;
                        }
                        local_env.vars.insert(name.clone(), ty.clone());
                        if pat_binding_names.contains(name) {
                            continue;
                        }
                        let ty_s = fmt_ty(ty, ctx);
                        match default {
                            Some(d) => {
                                let init = emit_exp(d, is_const, ctx, top_level);
                                body.push_str(&format!("            let mut {}: {ty_s} = {init};\n", escape_ident(name)));
                            }
                            None => {
                                body.push_str(&format!("            let mut {}: {ty_s};\n", escape_ident(name)));
                            }
                        }
                    }
                    for (n, t) in typedexp::pat_bindings(&case.pattern) {
                        local_env.vars.insert(n, t);
                    }
                    // Re-bind any pattern names that live inside `deref!(..)` AND
                    // are reassigned in the arm body. The pattern emitted those as
                    // `ref <name>`, giving `&T`; without a fresh owned `mut` shadow
                    // the body's `name = ...` would fail to compile. Cloning the
                    // referenced value yields an owned, mutable local that the
                    // body can both read and reassign.
                    let mut deref_names: Vec<String> = Vec::new();
                    pat_deref_bindings(&case.pattern, &input_ty, ctx, top_level, &mut deref_names);
                    if !deref_names.is_empty() {
                        let mut assigned: HashSet<String> = HashSet::new();
                        stmts_assigned_var_names(&case.stmts, &mut assigned);
                        for n in &deref_names {
                            if assigned.contains(n) {
                                let id = escape_ident(n);
                                body.push_str(&format!("            let mut {id} = {id}.clone();\n"));
                            }
                        }
                    }
                    // Use emit_stmts (not a raw emit_stmt loop) so consecutive
                    // record-field updates within the arm get batched into one
                    // `assign_field!` / `assign_variant_field!` call.
                    emit_stmts(&mut body, "            ", &case.stmts, FailureMode::Function, ctx, &mut local_env, top_level, &mut fresh_local);
                    format!("        {pat}{guard} => {{\n{body}            {result}\n        }}")
                };
                ctx.variants = saved_variants;
                ctx.variant_shapes = saved_shapes;
                arm_str
            }).collect();
            let fallback = if has_wild { String::new() } else {
                ",\n        _ => bail!(\"match: no arm matched\")".to_owned()
            };
            format!(
                "(match {match_subject} {{\n{}{fallback},\n    }})",
                arms.join(",\n"),
            )
        }
        MatchKind::MatchContinue => {
            // Each arm is an IIFE returning anyhow::Result<T>; first Ok wins.
            // Failures inside an arm (pattern mismatch, ?-propagated errors) drop to next arm.
            let mut s = String::new();
            s.push_str("'mc: {\n");
            // For Arc-wrapped types, store the Arc then match via as_ref() per arm.
            if input_is_arc {
                s.push_str(&format!("        let __mc_input = {input_str};\n"));
            } else {
                s.push_str(&format!("        let __mc_input = {input_str};\n"));
            }
            for case in cases {
                let pat = emit_pat_with_implicit_bind(&case.pattern, /*allow_implicit_bind=*/true, /*mut_bindings=*/true, /*in_deref=*/false, Some(&input_ty), ctx, top_level);
                let saved_variants = ctx.variants.clone();
                let saved_shapes = ctx.variant_shapes.clone();
                {
                    let mut tmp_env = LocalEnv::default();
                    let mut tmp_shapes: HashMap<String, VarShape> = HashMap::new();
                    record_pattern_variants_with_shapes(&case.pattern, input, &mut tmp_env, top_level, &mut tmp_shapes, ctx);
                    for (k, v) in tmp_env.variants {
                        ctx.variants.insert(k, v);
                    }
                    for (k, s) in tmp_shapes {
                        ctx.variant_shapes.insert(k, s);
                    }
                }
                let guard_check = case.guard.as_ref()
                    .map(|g| format!("            if !({}) {{ bail!(\"guard\") }}\n", emit_exp(g, is_const, ctx, top_level)))
                    .unwrap_or_default();
                let result = emit_exp(&case.result, is_const, ctx, top_level);
                s.push_str("        if let Ok(__v) = (|| -> Result<_> {\n");
                if input_is_arc {
                    s.push_str(&format!("            let {pat} = __mc_input.as_ref() else {{ bail!(\"nomatch\") }};\n"));
                } else {
                    s.push_str(&format!("            let {pat} = __mc_input.clone() else {{ bail!(\"nomatch\") }};\n"));
                }
                s.push_str(&guard_check);
                s.push_str(&format!("            Ok({result})\n"));
                s.push_str("        })() { break 'mc __v; }\n");
                ctx.variants = saved_variants;
                ctx.variant_shapes = saved_shapes;
            }
            s.push_str("        bail!(\"matchcontinue: no arm matched\")\n");
            s.push_str("    }");
            s
        }
    }
}

fn emit_pat<'a>(pat: &TypedPat, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    emit_pat_with_implicit_bind(pat, /*allow_implicit_bind=*/true, /*mut_bindings=*/false, /*in_deref=*/false, None, ctx, top_level)
}

/// Return true if matching `pat` against a scrutinee of `ty` will cross an
/// Arc edge — i.e. the pattern destructures through an `Arc<T>` wrapper.
/// Crossing an Arc edge happens for:
///   - `Ty::List(_)` (lowered to `Arc<metamodelica::List<T>>`).
///   - Any type marked recursive in `ctx.recursive_types` (uniontype enums and
///     their variant records are stored behind `Arc`).
///   - The `tail` field of a `Cons` pattern (typed as `Arc<List<T>>` regardless
///     of the outer scrutinee, since the field itself is Arc-wrapped).
/// Once we cross an Arc edge, the `deref_patterns` feature binds every
/// nested name by reference; `mut <name>` is therefore rejected as a
/// non-reference binding mode and any move from the binding fails (E0507).
/// We must emit `ref <name>` for those bindings and, in the arm body,
/// rebind with `let mut <name> = <name>.clone();` if the body reassigns them.
fn ty_needs_arc_match_deref(ty: &Ty, ctx: &GenCtx) -> bool {
    matches!(ty, Ty::List(_)) || is_arc_wrapped(ty, ctx)
}

/// Collect every binding name in `pat` that lies inside an Arc-deref path
/// (across an `Arc<T>` edge) when the pattern is matched against a value of
/// type `scrut_ty`. Such bindings *cannot* be `mut` directly: the
/// `deref_patterns` feature yields references through the `Arc`, and `mut`
/// on a by-reference binding is rejected by the Rust compiler. We emit
/// `ref <name>` for them in the pattern, and then optionally re-bind them
/// with `let mut <name> = <name>.clone();` in the arm body if the source
/// MetaModelica reassigns them.
fn pat_deref_bindings(pat: &TypedPat, scrut_ty: &Ty, ctx: &GenCtx, top_level: &BTreeMap<String, NameNode<'_>>, out: &mut Vec<String>) {
    fn walk_inside_deref(p: &TypedPat, out: &mut Vec<String>) {
        match p {
            TypedPat::Var(name) => out.push(name.clone()),
            TypedPat::Some_(inner) => walk_inside_deref(inner, out),
            TypedPat::Cons { head, tail } => {
                // We are already inside a deref!: everything below is too.
                walk_inside_deref(head, out);
                walk_inside_deref(tail, out);
            }
            TypedPat::Tuple(pats) => pats.iter().for_each(|p| walk_inside_deref(p, out)),
            TypedPat::Constructor { fields, named_fields, .. } => {
                fields.iter().for_each(|p| walk_inside_deref(p, out));
                named_fields.iter().for_each(|(_, p)| walk_inside_deref(p, out));
            }
            TypedPat::As { var, pat } => {
                out.push(var.clone());
                walk_inside_deref(pat, out);
            }
            _ => {}
        }
    }
    // If the scrutinee type itself crosses an Arc edge, every binding below
    // this pattern is in deref territory.
    if ty_needs_arc_match_deref(scrut_ty, ctx) {
        walk_inside_deref(pat, out);
        return;
    }
    // Otherwise, recurse type-aware to find where Arc edges are crossed.
    match pat {
        TypedPat::Cons { head, tail } => {
            // The Cons.tail field is `Arc<List<T>>` — itself an Arc edge —
            // so everything below the tail subtree is in deref. The head
            // is bound by value (type T) without crossing an Arc edge, so
            // recurse type-aware into it.
            let elem_ty = match scrut_ty { Ty::List(t) => (**t).clone(), _ => Ty::Unknown };
            pat_deref_bindings(head, &elem_ty, ctx, top_level, out);
            walk_inside_deref(tail, out);
        }
        TypedPat::Some_(inner) => {
            let inner_ty = match scrut_ty { Ty::Option(t) => (**t).clone(), _ => Ty::Unknown };
            pat_deref_bindings(inner, &inner_ty, ctx, top_level, out);
        }
        TypedPat::Tuple(pats) => {
            let elem_tys: Vec<Ty> = match scrut_ty {
                Ty::Tuple(ts) if ts.len() == pats.len() => ts.clone(),
                _ => vec![Ty::Unknown; pats.len()],
            };
            for (p, ety) in pats.iter().zip(elem_tys.iter()) {
                pat_deref_bindings(p, ety, ctx, top_level, out);
            }
        }
        TypedPat::Constructor { fields, named_fields, name, .. } => {
            // Look up field types so we can detect Arc-wrapped fields. The
            // field types come from the record definition for this variant.
            let field_tys = record_field_tys(name, top_level)
                .or_else(|| lookup_record_through_unions(name, top_level)
                    .and_then(|(canonical, _)| record_field_tys(&canonical, top_level)))
                .or_else(|| record_field_tys_from_scrutinee_ctor(name, scrut_ty, top_level))
                .unwrap_or_default();
            for (i, p) in fields.iter().enumerate() {
                let fty = field_tys.get(i).map(|(_, t)| t.clone()).unwrap_or(Ty::Unknown);
                pat_deref_bindings(p, &fty, ctx, top_level, out);
            }
            for (fname, p) in named_fields {
                let fty = field_tys.iter().find(|(n, _)| n == fname)
                    .map(|(_, t)| t.clone()).unwrap_or(Ty::Unknown);
                pat_deref_bindings(p, &fty, ctx, top_level, out);
            }
        }
        TypedPat::As { pat, .. } => pat_deref_bindings(pat, scrut_ty, ctx, top_level, out),
        _ => {}
    }
}

/// Walk `stmts` and collect names that appear as the LHS of an `Assign`
/// (either as `Var(n)` directly, as `As { var: n, .. }`, or as a tuple
/// component). Used to decide which pattern-bound names need to be
/// Collect all variable names bound by an assignment LHS pattern.
fn pat_assigned_names(p: &TypedPat, out: &mut HashSet<String>) {
    match p {
        TypedPat::Var(n) => { out.insert(n.clone()); }
        TypedPat::As { var, pat } => { out.insert(var.clone()); pat_assigned_names(pat, out); }
        TypedPat::Tuple(pats) => pats.iter().for_each(|p| pat_assigned_names(p, out)),
        // A compound LHS pattern like `l :: ll := ll` is lowered by
        // `emit_pat_assign` to synthetic `name = __paN.clone();` writes
        // for every binding the pattern introduces. Each such name is a
        // *reassignment* of an already-in-scope variable (per
        // `rewrite_pat_for_existing_bindings`), so collect them here so
        // that the match-arm prologue knows to introduce an owned
        // `let mut <name>` shadow for any name also bound by ref in
        // the arm's case pattern.
        TypedPat::Cons { head, tail } => { pat_assigned_names(head, out); pat_assigned_names(tail, out); }
        TypedPat::Constructor { fields, named_fields, .. } => {
            fields.iter().for_each(|p| pat_assigned_names(p, out));
            named_fields.iter().for_each(|(_, p)| pat_assigned_names(p, out));
        }
        TypedPat::Some_(inner) => pat_assigned_names(inner, out),
        _ => {}
    }
}

/// re-bound as owned `mut` locals at the top of a match arm.
fn stmts_assigned_var_names(stmts: &[typedexp::TypedStmt], out: &mut HashSet<String>) {
    use typedexp::TypedStmt as S;
    for s in stmts {
        match s {
            S::Assign { lhs, .. } => pat_assigned_names(lhs, out),
            S::If { then_, elseif, else_, .. } => {
                stmts_assigned_var_names(then_, out);
                for (_, eb) in elseif { stmts_assigned_var_names(eb, out); }
                stmts_assigned_var_names(else_, out);
            }
            S::For { body, .. } | S::While { body, .. } | S::Failure { body } => {
                stmts_assigned_var_names(body, out);
            }
            S::Try { body, else_body } => {
                stmts_assigned_var_names(body, out);
                stmts_assigned_var_names(else_body, out);
            }
            _ => {}
        }
    }
}

/// Render a pattern.
///
/// `mut_bindings` controls whether `Var` bindings are emitted with the `mut`
/// keyword. We need this in match arms (and let-else destructuring assignments)
/// when the algorithm section that follows the pattern reassigns to one of the
/// names bound by the pattern. MetaModelica permits reassigning pattern bindings;
/// Rust requires the binding to be declared `mut` first. Since the unused-mut
/// lint is allowed for generated code, marking *all* such bindings as `mut` is
/// always safe.
fn emit_pat_with_implicit_bind<'a>(pat: &TypedPat, allow_implicit_bind: bool, mut_bindings: bool, in_deref: bool, scrut_ty: Option<&Ty>, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    // `in_deref` marks that we are emitting beneath a `deref!(..)` wrapper
    // (i.e. somewhere inside the tail subtree of a `Cons` pattern). The
    // `deref_patterns` macro expansion binds names by reference, so a
    // `mut <name>` binding would not compile. We emit `ref <name>` instead
    // and rely on the match-arm prologue to introduce a fresh `let mut`
    // shadow for any name that the arm body actually reassigns.
    //
    // If the current scrutinee type itself crosses an Arc edge (Ty::List
    // or a recursive uniontype wrapped in Arc), matching it via
    // `deref_patterns` produces by-ref bindings. Force `in_deref` true for
    // the rest of this subtree so `bind_var` emits `ref` rather than `mut`.
    let in_deref = in_deref || scrut_ty.map(|t| ty_needs_arc_match_deref(t, ctx)).unwrap_or(false);
    let bind_var = |name: &str| -> String {
        if in_deref {
            format!("ref {}", escape_ident(name))
        } else if mut_bindings {
            format!("mut {}", escape_ident(name))
        } else {
            escape_ident(name)
        }
    };
    match pat {
        TypedPat::Wildcard    => "_".to_owned(),
        TypedPat::Var(name)   => bind_var(name),
        TypedPat::EmptyList   => "metamodelica::List::Nil".to_owned(),
        TypedPat::Some_(inner) => format!("Some({})", emit_pat_with_implicit_bind(inner, allow_implicit_bind, mut_bindings, in_deref, None, ctx, top_level)),
        TypedPat::None_       => "None".to_owned(),

        TypedPat::Lit(Lit::Int(v))  => {
            if *v < 0 { format!("({v})") } else { v.to_string() }
        }
        TypedPat::Lit(Lit::Bool(v)) => v.to_string(),
        TypedPat::Lit(Lit::Str(_))  => "_ /* string — move to guard */".to_owned(),
        TypedPat::Lit(Lit::Real(_)) => "_ /* real — move to guard */".to_owned(),

        TypedPat::Cons { head, tail } => {
            // Element type: pull from scrut_ty when known so any sub-pattern
            // that itself crosses an Arc edge (e.g. an element which is a
            // recursive uniontype) gets `in_deref` set correctly.
            let elem_ty: Ty = match scrut_ty { Some(Ty::List(t)) => (**t).clone(), _ => Ty::Unknown };
            // The `tail` field of `metamodelica::List::Cons` is `Arc<List<T>>`
            // (itself an Arc edge), so emit the tail sub-pattern with a
            // synthetic `Ty::List(elem)` scrutinee. This is what makes a
            // nested tail-side `Cons` or `Var` get `ref` binding rather
            // than `mut` (which would fail to compile as a move out of a
            // shared reference under `deref_patterns`).
            let tail_ty = Ty::List(Box::new(elem_ty.clone()));
            format!("metamodelica::List::Cons {{ head: {}, tail: {} }}",
                emit_pat_with_implicit_bind(head, allow_implicit_bind, mut_bindings, in_deref, Some(&elem_ty), ctx, top_level),
                emit_pat_with_implicit_bind(tail, allow_implicit_bind, mut_bindings, true, Some(&tail_ty), ctx, top_level))
        }

        TypedPat::Tuple(pats) => {
            // Tuple elements can bind in the same pattern scope; avoid auto-binding all
            // constructor fields there to prevent duplicate-name bindings.
            let parts: Vec<String> = pats.iter()
                .enumerate()
                .map(|(i, p)| {
                    let elem_ty = match scrut_ty {
                        Some(Ty::Tuple(ts)) => ts.get(i),
                        _ => None,
                    };
                    emit_pat_with_implicit_bind(p, /*allow_implicit_bind=*/false, mut_bindings, in_deref, elem_ty, ctx, top_level)
                })
                .collect();
            format!("({})", parts.join(", "))
        }

        TypedPat::Constructor { name, fields, named_fields, ty, .. } => {
            let rust_raw = if name.contains('.') { ctx.shorten(name) } else { normalize_builtin_ctor_name(name) };
            let rust = escape_ident(&rust_raw);
            let field_tys_for_ctor = || {
                if name.contains('.') {
                    // Direct lookup first; fall back to lookup-through-unions for names
                    // like "Flags.FLAGS" where the record is at "Flags.Flag.FLAGS".
                    if let Some(tys) = record_field_tys(name, top_level) {
                        return Some(tys);
                    }
                    if let Some((canonical, _)) = lookup_record_through_unions(name, top_level) {
                        return record_field_tys(&canonical, top_level);
                    }
                    // The scrutinee's type tells us the enclosing uniontype, which lets
                    // us recover the record's field layout when neither direct lookup
                    // nor the bottom-up uniontype walk succeeds — e.g. when `name`
                    // resolves only through an import alias and the simple-name pass
                    // would otherwise miss it.
                    if let Some(ty) = scrut_ty {
                        let simple = name.rsplit_once('.').map_or(name.as_str(), |(_, s)| s);
                        if let Some(from_scrut) = record_field_tys_from_scrutinee_ctor(simple, ty, top_level) {
                            return Some(from_scrut);
                        }
                    }
                    // Last resort: search by simple name. Better than emitting a TODO
                    // for a record we just couldn't find by qualified path.
                    let simple = name.rsplit_once('.').map_or(name.as_str(), |(_, s)| s);
                    let by_simple = record_field_tys_by_simple_name(simple, top_level);
                    if !by_simple.is_empty() {
                        return Some(by_simple);
                    }
                    return None;
                }
                if let Some(ty) = scrut_ty {
                    if let Some(from_scrut) = record_field_tys_from_scrutinee_ctor(name, ty, top_level) {
                        return Some(from_scrut);
                    }
                }
                Some(record_field_tys_by_simple_name(name, top_level))
            };
            if named_fields.is_empty() && fields.is_empty() {
                // Empty MetaModelica pattern `case NODE()` or `case NODE` —
                // decide based on the type: struct/variant types need `{ .. }` to avoid
                // E0532; constants and unknown types use the bare name.
                let is_struct_ty = matches!(ty,
                    Ty::RustStruct(_) | Ty::UnionTypeVariant(_, _) | Ty::RustUnitVariant
                    | Ty::RustEnum(_) | Ty::AliasTo(_)
                );
                if is_struct_ty {
                    format!("{rust} {{ .. }}")
                } else {
                    // For non-struct types (constants, unknown), look up fields.
                    // If fields are found, it IS a struct variant and needs `{ .. }`.
                    let field_tys = field_tys_for_ctor().unwrap_or_default();
                    if field_tys.is_empty() {
                        rust  // constant, enum value, or truly-unit variant
                    } else {
                        format!("{rust} {{ .. }}")
                    }
                }
            } else if named_fields.is_empty() {
                if is_sourceinfo_ctor(name) {
                    let pats: Vec<String> = fields.iter().enumerate().map(|(i, p)| {
                        let fname = sourceinfo_field_name_by_index(i);
                        if fname.is_empty() {
                            "_".to_owned()
                        } else {
                            format!("{fname}: {}", emit_pat_with_implicit_bind(p, allow_implicit_bind, mut_bindings, in_deref, None, ctx, top_level))
                        }
                    }).collect();
                    format!("{rust} {{ {} }}", pats.join(", "))
                } else {
                    // Positional patterns for named-field struct variants must use struct
                    // syntax in Rust; tuple syntax only applies to tuple-style variants.
                    // Look up field names so we can emit `Ctor { f1: p1, f2: p2 }`.
                    let field_tys = field_tys_for_ctor().unwrap_or_default();
                    if !field_tys.is_empty() {
                        let pats: Vec<String> = fields.iter().enumerate().map(|(i, p)| {
                            let fname = field_tys.get(i).map(|(n, _)| n.as_str()).unwrap_or("_");
                            let pstr = emit_pat_with_implicit_bind(p, allow_implicit_bind, mut_bindings, in_deref, None, ctx, top_level);
                            if matches!(p, TypedPat::Var(v) if v == fname) {
                                if in_deref {
                                    format!("{}: ref {}", escape_ident(fname), escape_ident(fname))
                                } else if mut_bindings {
                                    format!("{}: mut {}", escape_ident(fname), escape_ident(fname))
                                } else {
                                    escape_ident(fname)
                                }
                            } else {
                                format!("{}: {pstr}", escape_ident(fname))
                            }
                        }).collect();
                        // If the pattern doesn't cover all fields, add `..` to avoid E0027.
                        let needs_dotdot = fields.len() < field_tys.len();
                        if needs_dotdot {
                            format!("{rust} {{ {}, .. }}", pats.join(", "))
                        } else {
                            format!("{rust} {{ {} }}", pats.join(", "))
                        }
                    } else {
                        // Field names unknown — fall back to tuple syntax with a comment.
                        // This will likely fail to compile; it is better than silently
                        // emitting wrong code.
                        let pats: Vec<String> = fields.iter()
                            .map(|p| emit_pat_with_implicit_bind(p, allow_implicit_bind, mut_bindings, in_deref, None, ctx, top_level))
                            .collect();
                        format!("/* TODO: unknown fields for {name} */ {rust}({})", pats.join(", "))
                    }
                }
            } else {
                let mut pats: Vec<String> = named_fields.iter()
                    .map(|(fname, p)| {
                        let pstr = emit_pat_with_implicit_bind(p, allow_implicit_bind, mut_bindings, in_deref, None, ctx, top_level);
                        if matches!(p, TypedPat::Var(v) if v == fname) {
                            if in_deref {
                                format!("{}: ref {}", escape_ident(fname), escape_ident(fname))
                            } else if mut_bindings {
                                format!("{}: mut {}", escape_ident(fname), escape_ident(fname))
                            } else {
                                escape_ident(fname)
                            }
                        } else {
                            format!("{}: {pstr}", escape_ident(fname))
                        }
                    })
                    .collect();

                // Check if any fields are missing; if so, add `..` to avoid E0027.
                // We no longer implicitly bind remaining fields (that shadows same-named
                // functions in scope and causes E0618).
                let field_tys = field_tys_for_ctor().unwrap_or_default();
                let all_covered = !field_tys.is_empty()
                    && field_tys.iter().all(|(n, _)| named_fields.iter().any(|(m, _)| m == n));
                if !all_covered {
                    pats.push("..".to_owned());
                }
                format!("{rust} {{ {} }}", pats.join(", "))
            }
        }

        TypedPat::As { var, pat } => {
            let outer = if in_deref {
                format!("ref {}", escape_ident(var))
            } else if mut_bindings {
                format!("mut {}", escape_ident(var))
            } else {
                escape_ident(var)
            };
            format!("{} @ {}", outer, emit_pat_with_implicit_bind(pat, false, mut_bindings, in_deref, None, ctx, top_level))
        }

        TypedPat::Index { base, index } => {
            // This shouldn't normally reach emit_pat (handled in emit_stmt), but emit as fallback.
            format!("{}[({}-1) as usize]", emit_exp(base, false, ctx, top_level), emit_exp(index, false, ctx, top_level))
        }

        TypedPat::FieldAccess { base, field } => {
            field_access_to_dotted(base, field)
        }

        TypedPat::Todo(s) => format!("_ /* todo: {} */", s.chars().take(40).collect::<String>()),
    }
}

fn record_field_tys_from_scrutinee_ctor<'a>(
    ctor_simple_name: &str,
    scrut_ty: &Ty,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<Vec<(String, Ty)>> {
    let mut bases: Vec<String> = Vec::new();
    match scrut_ty {
        Ty::RustEnum(q) | Ty::AliasTo(q) | Ty::RustStruct(q) => {
            bases.push(q.clone());
            if let Some((parent, _)) = q.rsplit_once('.') {
                bases.push(parent.to_owned());
            }
        }
        _ => {}
    }
    for b in bases {
        let cand = format!("{b}.{ctor_simple_name}");
        if let Some(tys) = record_field_tys(&cand, top_level) {
            return Some(tys);
        }
    }
    None
}

// ── Statement emission ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
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
    /// Single-statement try optimisation: emit `if let Ok(PAT) = CALL { body }
    /// else { else_code }` instead of a labeled block. The contained string is
    /// the already-emitted else-body (indented one level deeper than the `if`).
    IfLetElse(String),
}

#[derive(Debug, Clone, Default)]
struct LocalEnv {
    vars: HashMap<String, Ty>,
    /// Names of the enclosing function's output components, in declaration order.
    /// Used to expand `return;` into `return Ok((outputs...));` so the early-exit
    /// path mirrors the final implicit return at the end of `emit_function`.
    outputs: Vec<String>,
    /// For variables currently known to hold a specific uniontype variant
    /// (e.g. inside a match arm whose pattern is a `Constructor`, or after a
    /// refutable `let Constructor { .. } = expr;`), remember the enum qname and
    /// variant simple name. This lets us emit `assign_variant_field!` for
    /// `var.field := value` on multi-record uniontype values without having to
    /// re-analyze patterns at every use site.
    ///
    /// The mapping is cleared on any plain `var = ...` reassignment (since the
    /// new value may be a different variant) — see `S::Assign` handling.
    variants: HashMap<String, (String, String)>,
}

fn is_constructor(func: &str, ctx: &GenCtx, top_level: &BTreeMap<String, NameNode>) -> bool {
    // Some built-ins are always constructors structurally.
    if matches!(func, "SOME" | "NONE") {
        return true;
    }

    // Build the current package context so we can use the import-aware resolver from
    // typedexp — the same resolver that infer_exp uses. This handles import aliases
    // like `EvalFunctionExt = NFEvalFunctionExt` that `resolve_fully_qualified` cannot
    // follow because it only does literal hierarchy lookups.
    let pkg_prefix = if ctx.current_path.is_empty() {
        ctx.top_name.clone()
    } else {
        format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
    };

    // Normalize :: (Rust path) back to . (MM dotted name) so the resolver can look it up.
    let func_dotted = func.replace("::", ".");

    let node_opt = typedexp::resolve_call_node(&func_dotted, top_level, &pkg_prefix)
        .map(|(_, n)| n)
        // Fallback: try without import-alias resolution via the codegen's own context
        // (named imports, unqualified modules) which typedexp doesn't know about.
        .or_else(|| resolve_fully_qualified(func, ctx, top_level))
        .or_else(|| {
            lookup_record_through_unions(&func_dotted, top_level).map(|(_, n)| n)
        });

    if let Some(node) = node_opt {
        if let NodeKind::Class(c) = &node.kind {
            if matches!(c.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_UNIONTYPE { .. }) {
                return true;
            }
        }
        // Node was found in the hierarchy and is NOT a record/uniontype (e.g., it is a
        // function, package, import, etc.). Return false without applying any heuristic.
        return false;
    }
    // Shouldn't reach here. But let's assume it's not a constructor in that case
    return false;
}

fn normalize_builtin_ctor_name(name: &str) -> String {
    match name {
        "SOURCEINFO" => "SourceInfo".to_owned(),
        _ => name.to_owned(),
    }
}

fn is_sourceinfo_ctor(name: &str) -> bool {
    matches!(name, "SOURCEINFO" | "SourceInfo")
}

fn sourceinfo_field_name_by_index(i: usize) -> &'static str {
    match i {
        0 => "file_name",
        1 => "is_read_only",
        2 => "line_number_start",
        3 => "column_number_start",
        4 => "line_number_end",
        5 => "column_number_end",
        6 => "last_modification",
        _ => "",
    }
}

fn is_infallible_builtin(func: &str) -> bool {
    if func.ends_with("Dangerous.arrayGetNoBoundsChecking")
        || func.ends_with("Dangerous.arrayUpdateNoBoundsChecking")
        || func.ends_with("Dangerous.arrayCreateNoInit")
        || func.ends_with("Dangerous::arrayGetNoBoundsChecking")
        || func.ends_with("Dangerous::arrayUpdateNoBoundsChecking")
        || func.ends_with("Dangerous::arrayCreateNoInit")
    {
        return true;
    }

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
        | "MetaModelica.Dangerous.arrayGetNoBoundsChecking"
        | "MetaModelica.Dangerous.arrayUpdateNoBoundsChecking"
        | "MetaModelica.Dangerous.arrayCreateNoInit"
        | "Dangerous.arrayGetNoBoundsChecking"
        | "Dangerous.arrayUpdateNoBoundsChecking"
        | "Dangerous.arrayCreateNoInit"
        | "print" | "printError"
    )
}

fn pat_is_irrefutable(pat: &TypedPat) -> bool {
    match pat {
        TypedPat::Wildcard | TypedPat::Var(_) => true,
        TypedPat::Tuple(ps) => ps.iter().all(pat_is_irrefutable),
        TypedPat::As { pat, .. } => pat_is_irrefutable(pat),
        TypedPat::Index { .. } => true,
        TypedPat::FieldAccess { .. } => true,
        _ => false,
    }
}

/// Convert a FieldAccess pattern chain to dotted expression syntax (e.g., `a.b.c`).
fn field_access_to_dotted(base: &TypedPat, field: &str) -> String {
    match base {
        TypedPat::Var(name) => format!("{}.{}", escape_ident(name), escape_ident(&field)),
        TypedPat::FieldAccess { base: inner, field: f } => {
            let inner_str = field_access_to_dotted(inner, f);
            format!("{}.{}", inner_str, escape_ident(&field))
        },
        _ => format!("/*?*/.{}", escape_ident(&field)),
    }
}

/// For a single-record uniontype whose record has been renamed to its parent's name,
/// the record components live one level deeper (under the uniontype's child) than the
/// uniontype's `qname` suggests. Given a qname that may name either:
///   - a plain record/class (returned unchanged), or
///   - a uniontype with exactly one record child,
/// return the qname of the underlying record. Returns `None` if the node is not a
/// uniontype that fits the single-record shape (caller should use `qname` as-is).
fn resolve_single_record_qname<'a>(
    qname: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<String> {
    let node = lookup_node(qname, top_level)?;
    let NodeKind::Class(c) = &node.kind else { return None };
    if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) {
        return None;
    }
    // Find the single record child.
    let mut record_children: Vec<&str> = Vec::new();
    for (child_name, child) in &node.children {
        if let NodeKind::Class(cc) = &child.kind {
            if matches!(cc.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. }) {
                record_children.push(child_name.as_str());
            }
        }
    }
    if record_children.len() == 1 {
        Some(format!("{qname}.{}", record_children[0]))
    } else {
        None
    }
}

/// Look up the field types of a record/metarecord by qualified name.
/// Returns Some(Vec of (field_name, field_ty) in declaration order), or None if not found/not a class.
fn record_field_tys<'a>(
    qname: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<Vec<(String, Ty)>> {
    let node = lookup_node(qname, top_level)?;
    let NodeKind::Class(c) = &node.kind else { return None };
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return None,
    };
    Some(members.iter().filter_map(|m| {
        let MM::ClassMember::Component(cm) = m else { return None };
        let child = node.children.get(&cm.name)?;
        Some((cm.name.clone(), child.ty.clone()))
    }).collect())
}

/// Fallback lookup for constructor patterns written with bare names (e.g. `SEMVER`).
/// Searches class nodes by simple name and returns declaration-order field types.
fn record_field_tys_by_simple_name<'a>(
    simple_name: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Vec<(String, Ty)> {
    fn walk<'a>(node: &'a NameNode<'a>, simple_name: &str) -> Option<Vec<(String, Ty)>> {
        for (child_name, child) in &node.children {
            if child_name == simple_name {
                if let NodeKind::Class(c) = &child.kind {
                    let members: &[MM::ClassMember] = match &c.body {
                        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
                        _ => &[],
                    };
                    let tys: Vec<(String, Ty)> = members.iter().filter_map(|m| {
                        let MM::ClassMember::Component(cm) = m else { return None };
                        let fnode = child.children.get(&cm.name)?;
                        Some((cm.name.clone(), fnode.ty.clone()))
                    }).collect();
                    return Some(tys);
                }
            }
            if let Some(found) = walk(child, simple_name) {
                return Some(found);
            }
        }
        None
    }

    for node in top_level.values() {
        if let Some(found) = walk(node, simple_name) {
            return found;
        }
    }
    Vec::new()
}

/// Is this Ty stored behind an Arc due to recursion-cycle breaking?
/// Return true if `exp` can be lowered to a Rust *const expression* — i.e. the
/// value can be stored in a `pub static X: T = <expr>;` without needing the
/// `LazyLock` wrapper.
///
/// The criterion is structural: literals are const; tuples, options, and
/// struct/unit-variant constructors are const if all their parts are const and
/// the constructed type doesn't require an `Arc::new` wrapping (which is
/// non-const because `Arc::new` allocates at runtime). Calls, variable refs,
/// binary ops, ranges, ifs, matches, etc. are conservatively rejected — they
/// could be const in some cases but aren't worth detecting until needed.
///
/// `literal!("...")` (from arcstr) is a const ArcStr, so string literals qualify.
fn is_static_const_emittable(exp: &TypedExp, ctx: &GenCtx, top_level: &BTreeMap<String, NameNode<'_>>) -> bool {
    match exp {
        TypedExp::Lit(_) => true,
        TypedExp::Tuple(elems) => elems.iter().all(|e| is_static_const_emittable(e, ctx, top_level)),
        // A `Call` whose function is actually a record/uniontype constructor
        // gets rendered as a struct literal in `emit_exp`. The `typedexp` pass
        // can fail to classify these as `Constructor` when its scope-limited
        // resolver doesn't find the record (e.g., SOURCEINFO from MetaModelicaBuiltin
        // referenced from Util.mo) — codegen's `is_constructor` has wider lookups.
        TypedExp::Call { func, args, named_args, .. } => {
            // `is_constructor` covers records resolvable through the hierarchy;
            // `is_sourceinfo_ctor` covers the SOURCEINFO builtin whose definition
            // lives in MetaModelicaBuiltin.mo (not in the user-visible scope).
            let recognized_ctor = is_constructor(func, ctx, top_level) || is_sourceinfo_ctor(func);
            recognized_ctor
                && args.iter().all(|a| is_static_const_emittable(a, ctx, top_level))
                && named_args.iter().all(|(_, a)| is_static_const_emittable(a, ctx, top_level))
        }
        TypedExp::Constructor { ty, args, named_args, .. } => {
            // A constructor whose value would be Arc::new-wrapped at codegen time
            // (recursive uniontype variants) cannot be a const expression.
            if constructor_needs_arc(ty, ctx) { return false; }
            // Same for any individual field that gets wrapped in Arc::new
            // (struct_field_is_arc) — Arc::new is not const.
            // We approximate by rejecting if the struct stores any Arc-wrapped
            // field; a future refinement could check field-by-field.
            if let Ty::RustStruct(qname) | Ty::RustEnum(qname) = ty {
                if ctx.recursive_types.contains(qname.as_str()) { return false; }
                if let Some((parent, _)) = qname.rsplit_once('.') {
                    if ctx.recursive_types.contains(parent) { return false; }
                }
            }
            args.iter().all(|a| is_static_const_emittable(a, ctx, top_level))
                && named_args.iter().all(|(_, a)| is_static_const_emittable(a, ctx, top_level))
        }
        // Unit variants of an enum are const-constructable when the enum is not Arc-wrapped.
        TypedExp::Var { ty: Ty::RustUnitVariant, .. } => true,
        _ => false,
    }
}

fn is_arc_wrapped(ty: &Ty, ctx: &GenCtx) -> bool {
    let qname = match ty {
        Ty::RustStruct(n) | Ty::RustEnum(n) | Ty::AliasTo(n) | Ty::ExternalObject(n) => n.as_str(),
        Ty::Generic(name, _) => return ctx.recursive_types.contains(&name.replace("::", ".")),
        _ => return false,
    };
    ctx.recursive_types.contains(qname)
}

/// Return true if a *constructor expression* for this type should be wrapped in
/// `Arc::new(...)`.  This happens when the value being constructed is a variant of
/// a recursive uniontype (the parent type is in `recursive_types`), because all
/// fields that store this type are emitted as `Arc<T>`.
///
/// For a variant record `Ty::RustStruct("Pkg.Tree.NODE")` the parent type is
/// `"Pkg.Tree"`. For a uniontype itself `Ty::RustEnum("Pkg.Tree")` the type is
/// its own parent.
fn constructor_needs_arc(ty: &Ty, ctx: &GenCtx) -> bool {
    match ty {
        // Direct enum type: wrapped when the type itself is recursive.
        Ty::RustEnum(qname) => ctx.recursive_types.contains(qname.as_str()),
        // Variant record: wrapped when the PARENT enum/uniontype is recursive.
        Ty::RustStruct(qname) => {
            if ctx.recursive_types.contains(qname.as_str()) {
                return true;
            }
            // Strip the last segment to get the parent uniontype name.
            if let Some((parent, _)) = qname.rsplit_once('.') {
                ctx.recursive_types.contains(parent)
            } else {
                false
            }
        }
        Ty::AliasTo(qname) => {
            if ctx.recursive_types.contains(qname.as_str()) {
                return true;
            }
            if let Some((parent, _)) = qname.rsplit_once('.') {
                ctx.recursive_types.contains(parent)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Return true if the named field of a record (identified by its fully-qualified dotted name)
/// is stored behind `Arc` in the emitted Rust struct, meaning a constructor argument for that
/// field must be wrapped in `Arc::new(...)`.
///
/// Strings (Ty::Str) are always emitted as `ArcStr` by `fmt_ty`, but their expressions
/// are already `ArcStr` from `emit_exp` / `emit_cloned_call_arg`, so they do NOT need
/// an extra `Arc::new` layer here.
/// Only types in `ctx.recursive_types` get an additional `Arc` wrapping at the field level.
fn struct_field_is_arc<'a>(
    struct_qname: &str,
    field_name: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    ctx: &GenCtx,
) -> bool {
    let struct_node = match lookup_node(struct_qname, top_level) {
        Some(n) => n,
        None => return false,
    };
    let field_node = match struct_node.children.get(field_name) {
        Some(n) => n,
        None => return false,
    };
    is_arc_wrapped(&field_node.ty, ctx)
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
            if let FailureMode::IfLetElse(else_code) = fail_mode {
                writeln!(out, "{indent}if let Ok(_) = {scrut_expr} {{").unwrap();
                writeln!(out, "{indent}}} else {{").unwrap();
                out.push_str(else_code.as_str());
                writeln!(out, "{indent}}}").unwrap();
            } else {
                // Evaluate for side effects but discard.
                writeln!(out, "{indent}let _ = {scrut_expr};").unwrap();
            }
        }
        TypedPat::Var(name) => {
            let (actual_ty, actual_expr) = if let Ty::Tuple(tys) = scrut_ty {
                (tys.first().cloned().unwrap_or(Ty::Unknown), format!("{scrut_expr}.0"))
            } else {
                (scrut_ty.clone(), scrut_expr.to_string())
            };
            env.vars.insert(name.clone(), actual_ty);
            if let FailureMode::IfLetElse(else_code) = fail_mode {
                let n = *fresh; *fresh += 1;
                let tmp = format!("__iflet{n}");
                let inner = format!("{indent}    ");
                writeln!(out, "{indent}if let Ok({tmp}) = {actual_expr} {{").unwrap();
                writeln!(out, "{inner}{} = {tmp};", escape_ident(name)).unwrap();
                writeln!(out, "{indent}}} else {{").unwrap();
                out.push_str(else_code.as_str());
                writeln!(out, "{indent}}}").unwrap();
            } else {
                writeln!(out, "{indent}let {} = {actual_expr};", escape_ident(name)).unwrap();
            }
        }
        _ => {
            // MetaModelica permits a pattern-let to *reassign* names that are
            // already in scope (e.g. `l :: ll := ll;` where `l` and `ll` were
            // bound earlier by an enclosing case pattern). Rust's `let` always
            // introduces a new binding that shadows the outer name, so we'd
            // mutate a fresh local and the user's later read of `l`/`ll` would
            // see the stale outer value. Substitute fresh names for any pattern
            // bindings that collide with the current scope and emit follow-up
            // assignments to copy the new value back.
            let mut reassign_pairs: Vec<(String, String)> = Vec::new();
            let pat_owned = rewrite_pat_for_existing_bindings(pat, env, fresh, &mut reassign_pairs);
            let pat_for_render = &pat_owned;
            // Render shallow with deferrals for Arc-edge crossings.
            let mut deferrals: Vec<(String, TypedPat, Ty)> = Vec::new();
            let surface = render_shallow(pat_for_render, scrut_ty, ctx, env, top_level, fresh, &mut deferrals);
            // When the scrutinee is Arc-wrapped (list<T> → Arc<List<T>>; recursive
            // uniontypes wrapped in Arc), destructuring a variant pattern such as
            // `Cons { head, tail }` only succeeds via the `deref_patterns`
            // feature, which deref's through the Arc. The deref produces a
            // shared reference, so the bindings inside the pattern are bound
            // by reference and cannot be moved out (E0507). The fix is to
            // pattern-match against `&(scrutinee)` explicitly — making the
            // by-reference nature visible — and then `.clone()` each binding
            // at the point of use. For non-Arc scrutinees (tuples, plain
            // records, primitives), the bindings remain owned moves.
            let needs_borrow = matches!(scrut_ty, Ty::List(_)) || is_arc_wrapped(scrut_ty, ctx);
            let scrut_for_pat = if needs_borrow {
                format!("&({scrut_expr})")
            } else {
                format!("({scrut_expr})")
            };
            // Helper: emit deferrals then reassign-pairs at a given indent level.
            // Deferrals must run before reassigns (a deferral may produce a
            // `let __paN = (*__tM).clone();` binding that a reassign then copies).
            macro_rules! emit_body {
                ($out:expr, $ind:expr, $fm:expr) => {
                    for (sub_expr, sub_pat, sub_ty) in deferrals {
                        emit_pat_assign($out, $ind, &sub_pat, &sub_ty, &sub_expr, $fm, ctx, env, top_level, fresh);
                    }
                    for (orig, fresh_name) in &reassign_pairs {
                        let orig_ty = env.vars.get(orig).cloned().unwrap_or(Ty::Unknown);
                        let arc_shaped = matches!(&orig_ty, Ty::List(_)) || is_arc_wrapped(&orig_ty, ctx);
                        let needs_clone = needs_borrow || arc_shaped;
                        if needs_clone {
                            writeln!($out, "{}{} = {}.clone();", $ind, escape_ident(orig), escape_ident(fresh_name)).unwrap();
                        } else {
                            writeln!($out, "{}{} = {};", $ind, escape_ident(orig), escape_ident(fresh_name)).unwrap();
                        }
                    }
                };
            }

            if pat_is_irrefutable(pat_for_render) {
                match pat_for_render {
                    TypedPat::Tuple(_) => {
                        writeln!(out, "{indent}let {surface} = {scrut_expr};").unwrap();
                    }
                    _ => {
                        if needs_borrow {
                            writeln!(out, "{indent}let {surface} = {scrut_for_pat};").unwrap();
                        } else {
                            writeln!(out, "{indent}let {surface} = {scrut_expr};").unwrap();
                        }
                    }
                }
                emit_body!(out, indent, fail_mode.clone());
            } else if let FailureMode::IfLetElse(else_code) = fail_mode {
                // Single-statement try optimisation: emit
                //   if let Ok(PAT) = CALL { body } else { else_code }
                // `scrut_expr` is already the raw Result (emitted in Bare mode).
                let inner = format!("{indent}    ");
                writeln!(out, "{indent}if let Ok({surface}) = {scrut_expr} {{").unwrap();
                emit_body!(out, inner.as_str(), FailureMode::Function);
                writeln!(out, "{indent}}} else {{").unwrap();
                out.push_str(else_code.as_str());
                writeln!(out, "{indent}}}").unwrap();
            } else {
                // For a let-else inside a try-body lowered to a labeled block,
                // we must exit the *block* with `Err(_)`, not `bail!` out of the
                // surrounding function. `bail!` expands to `return Err(..)` and
                // would skip the `else_body` recovery entirely.
                let fail_owned;
                let fail: &str = match fail_mode {
                    FailureMode::Function => "bail!(\"pattern mismatch\")",
                    FailureMode::TryArm => match &ctx.qmode {
                        QMode::TryBlock(label) => {
                            fail_owned = format!("break {label} Err::<_, _>(anyhow::anyhow!(\"pattern mismatch\"))");
                            &fail_owned
                        }
                        _ => "bail!(\"pattern mismatch\")",
                    },
                    FailureMode::Failure => "()",
                    FailureMode::IfLetElse(_) => unreachable!(),
                };
                writeln!(out, "{indent}let {surface} = {scrut_for_pat} else {{ {fail} }};").unwrap();
                emit_body!(out, indent, fail_mode.clone());
            }
        }
    }
}

/// Walk a pattern, replacing any `Var(name)` (or `As { var, .. }`) whose name is
/// already bound in the current scope with a fresh `__pa<N>` name. Returns the
/// rewritten pattern; the caller emits `name = __paN;` after the let-binding so
/// the existing variable is updated rather than shadowed. This is needed because
/// MetaModelica pattern-lets reassign existing names (e.g. `l :: ll := ll;`)
/// while Rust's `let` always introduces a new binding.
fn rewrite_pat_for_existing_bindings(
    pat: &TypedPat,
    env: &LocalEnv,
    fresh: &mut u32,
    reassign: &mut Vec<(String, String)>,
) -> TypedPat {
    let mk_fresh = |fresh: &mut u32| -> String {
        let n = *fresh; *fresh += 1;
        format!("__pa{n}")
    };
    match pat {
        TypedPat::Var(name) if env.vars.contains_key(name) => {
            let new_name = mk_fresh(fresh);
            reassign.push((name.clone(), new_name.clone()));
            TypedPat::Var(new_name)
        }
        TypedPat::Some_(inner) => TypedPat::Some_(Box::new(
            rewrite_pat_for_existing_bindings(inner, env, fresh, reassign))),
        TypedPat::Cons { head, tail } => TypedPat::Cons {
            head: Box::new(rewrite_pat_for_existing_bindings(head, env, fresh, reassign)),
            tail: Box::new(rewrite_pat_for_existing_bindings(tail, env, fresh, reassign)),
        },
        TypedPat::Tuple(pats) => TypedPat::Tuple(
            pats.iter().map(|p| rewrite_pat_for_existing_bindings(p, env, fresh, reassign)).collect()),
        TypedPat::Constructor { name, fields, named_fields, ty } => {
            let new_fields = fields.iter()
                .map(|p| rewrite_pat_for_existing_bindings(p, env, fresh, reassign)).collect();
            let new_named = named_fields.iter()
                .map(|(n, p)| (n.clone(), rewrite_pat_for_existing_bindings(p, env, fresh, reassign))).collect();
            TypedPat::Constructor {
                name: name.clone(),
                fields: new_fields,
                named_fields: new_named,
                ty: ty.clone(),
            }
        }
        TypedPat::As { var, pat } => {
            let inner = rewrite_pat_for_existing_bindings(pat, env, fresh, reassign);
            if env.vars.contains_key(var) {
                let new_name = mk_fresh(fresh);
                reassign.push((var.clone(), new_name.clone()));
                TypedPat::As { var: new_name, pat: Box::new(inner) }
            } else {
                TypedPat::As { var: var.clone(), pat: Box::new(inner) }
            }
        }
        _ => pat.clone(),
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
            // The `tail` field of `metamodelica::List::Cons` is `Arc<List<T>>`, and
            // the surface MetaModelica type `list<T>` is also lowered to
            // `Arc<List<T>>`, so binding the tail directly in the pattern yields a
            // value of exactly the right user-visible type. We still route the
            // tail through a fresh `__tN` temporary so that a non-trivial sub-pattern
            // (e.g. another `Cons`, a constructor, or a name that shadows an
            // already-bound variable) can be re-emitted by `emit_pat_assign` against
            // an owned `Arc<List<T>>` scrutinee. The deferred expression is therefore
            // `__tN.clone()` (an Arc bump), NOT `(*__tN).clone()` — the latter would
            // strip the Arc and produce a `List<T>` value, which no longer matches
            // the surface type.
            // Wildcards pass through unchanged — there is nothing to bind.
            // A simple `Var(name)` sub-pattern can also be bound directly in the
            // pattern: by-value matching on an `Arc<List<T>>` moves the `tail`
            // field out as an `Arc<List<T>>`, which is exactly the surface type
            // of the user's variable. No fresh temporary or follow-up clone is
            // needed in that case. Note that `rewrite_pat_for_existing_bindings`
            // has already substituted any name that collides with the current
            // scope by a fresh `__paN`, so binding directly here is safe — the
            // reassign-back step in `emit_pat_assign` will copy the new value
            // into the original user variable.
            let t = match tail.as_ref() {
                TypedPat::Wildcard => "_".to_owned(),
                TypedPat::Var(_) => render_shallow(tail, scrut_ty, ctx, env, top_level, fresh, deferrals),
                _ => {
                    let n = *fresh; *fresh += 1;
                    let tmp = format!("__t{n}");
                    deferrals.push((format!("{tmp}.clone()"), (**tail).clone(), scrut_ty.clone()));
                    tmp
                }
            };
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
            let mut resolved_qname = if name.contains('.') {
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
            if name == "FAILURE" {
                eprintln!("DEBUG FAILURE BEFORE FALLBACK: resolved_qname={:?}, scrut_ty={:?}", resolved_qname, scrut_ty);
            }
            if resolved_qname.is_none() && !name.contains('.') {
                // Fallback: if matching against a known uniontype/enum value, variants are
                // commonly looked up as `EnumName.VARIANT`.
                let enum_qname = match scrut_ty {
                    Ty::RustEnum(q) | Ty::AliasTo(q) => Some(q.clone()),
                    _ => None,
                };
                if name == "FAILURE" {
                    eprintln!("DEBUG FAILURE: scrut_ty={:?}, enum_qname={:?}", scrut_ty, enum_qname);
                }
                if let Some(enum_qname) = enum_qname {
                    let candidate = format!("{enum_qname}.{name}");
                    if lookup_node(&candidate, top_level).is_some() {
                        resolved_qname = Some(candidate);
                    }
                }
            }
            // Resolve the record's field list, walking through enclosing uniontypes
            // when the qualified name (e.g. "Flags.FLAGS") isn't a direct node — the
            // canonical path is typically "Flags.Flag.FLAGS" via the Flag uniontype.
            let mut field_tys: Vec<(String, Ty)> = resolved_qname
                .as_deref()
                .and_then(|q| record_field_tys(q, top_level))
                .unwrap_or_default();
            if field_tys.is_empty() {
                if let Some(q) = resolved_qname.as_deref() {
                    if let Some((canonical, _)) = lookup_record_through_unions(q, top_level) {
                        if let Some(tys) = record_field_tys(&canonical, top_level) {
                            field_tys = tys;
                            resolved_qname = Some(canonical);
                        }
                    }
                }
            }
            if field_tys.is_empty() {
                // Last-resort search by simple name (handles cases where neither the
                // dotted path nor the uniontype walk yields a hit, e.g. records
                // reachable only via an import alias).
                let simple = name.rsplit_once('.').map_or(name.as_str(), |(_, s)| s);
                let by_simple = record_field_tys_by_simple_name(simple, top_level);
                if !by_simple.is_empty() {
                    field_tys = by_simple;
                }
            }
            let rust_ctor_raw = if name.contains('.') { ctx.shorten(name) } else { normalize_builtin_ctor_name(name) };
            let rust_ctor = escape_ident(&rust_ctor_raw);

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
                    let rust_field = fname;
                    if matches!(sp, TypedPat::Var(v) if v == fname) {
                        escape_ident(&rust_field)
                    } else {
                        format!("{}: {s}", escape_ident(&rust_field))
                    }
                }).collect();
                // Add `..` if not all fields are covered (or if the field list is
                // unknown), to satisfy Rust's E0027.
                let all_covered = !field_tys.is_empty()
                    && field_tys.iter().all(|(n, _)| named_fields.iter().any(|(m, _)| m == n));
                if all_covered {
                    format!("{rust_ctor} {{ {} }}", parts.join(", "))
                } else {
                    format!("{rust_ctor} {{ {}, .. }}", parts.join(", "))
                }
            } else if !fields.is_empty() {
                if is_sourceinfo_ctor(name) {
                    let parts: Vec<String> = fields.iter().enumerate().map(|(i, sp)| {
                        let fty = field_tys.get(i).map(|(_, t)| t.clone()).unwrap_or(Ty::Unknown);
                        let s = handle(sp, &fty, ctx, env, fresh, deferrals);
                        let fname = sourceinfo_field_name_by_index(i);
                        if fname.is_empty() { "_".to_owned() } else { format!("{fname}: {s}") }
                    }).collect();
                    format!("{rust_ctor} {{ {} }}", parts.join(", "))
                } else if !field_tys.is_empty() {
                    // Positional patterns for named-field struct variants must use struct
                    // syntax in Rust. Map positional sub-patterns to their field names.
                    let parts: Vec<String> = fields.iter().enumerate().map(|(i, sp)| {
                        let (fname, fty) = field_tys.get(i)
                            .map(|(n, t)| (n.as_str(), t.clone()))
                            .unwrap_or(("_", Ty::Unknown));
                        let s = handle(sp, &fty, ctx, env, fresh, deferrals);
                        if matches!(sp, TypedPat::Var(v) if v == fname) {
                            escape_ident(fname)
                        } else {
                            format!("{}: {s}", escape_ident(fname))
                        }
                    }).collect();
                    // Add `..` if the pattern covers fewer fields than the record has.
                    if fields.len() < field_tys.len() {
                        format!("{rust_ctor} {{ {}, .. }}", parts.join(", "))
                    } else {
                        format!("{rust_ctor} {{ {} }}", parts.join(", "))
                    }
                } else {
                    // Field names unknown — fall back to tuple syntax with a comment.
                    let parts: Vec<String> = fields.iter().enumerate().map(|(i, sp)| {
                        let fty = field_tys.get(i).map(|(_, t)| t.clone()).unwrap_or(Ty::Unknown);
                        handle(sp, &fty, ctx, env, fresh, deferrals)
                    }).collect();
                    format!("/* TODO: unknown fields for {name} */ {rust_ctor}({})", parts.join(", "))
                }
            } else {
                // Empty pattern: use `{ .. }` for struct/variant types, bare name for constants.
                let field_tys_empty = resolved_qname.as_deref()
                    .and_then(|q| record_field_tys(q, top_level))
                    .map(|v| v.is_empty())
                    .unwrap_or(true);
                let is_struct_ty = matches!(scrut_ty,
                    Ty::RustStruct(_) | Ty::UnionTypeVariant(_, _) | Ty::RustUnitVariant
                    | Ty::RustEnum(_) | Ty::AliasTo(_)
                ) || !field_tys_empty;
                if is_struct_ty || is_sourceinfo_ctor(name) {
                    format!("{rust_ctor} {{ .. }}")
                } else {
                    rust_ctor
                }
            }
        }
        TypedPat::As { var, pat: inner } => {
            env.vars.insert(var.clone(), scrut_ty.clone());
            let inner_s = render_shallow(inner, scrut_ty, ctx, env, top_level, fresh, deferrals);
            format!("{} @ {}", escape_ident(var), inner_s)
        }
        TypedPat::Index { base, index } => {
            // Array index in pattern position — emit as lvalue access.
            format!("{}[{}]", emit_exp(base, false, ctx, top_level), emit_exp(index, false, ctx, top_level))
        }
        TypedPat::FieldAccess { base, field } => {
            field_access_to_dotted(base, field)
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
    let mut i = 0;
    while i < stmts.len() {
        // Look for a run of consecutive `Assign` statements of the form
        // `<same-base>.<field> := <expr>;` that all dispatch to the same
        // record-update macro (`assign_field!` for Arc<Struct> or
        // `assign_variant_field!` for an Arc<Enum> with a known matched
        // variant). A run of length ≥ 2 is emitted as a single macro call:
        // one line per field, but only one `(*base).clone()` and one
        // `Arc::new(..)` at runtime.
        //
        // We pre-screen without rendering expressions (which would mutate
        // `ctx` state and double-emit `use` markers): only render rhs values
        // once we're committed to the macro path.
        if let Some(plan) = plan_field_assign(&stmts[i], env, top_level) {
            if plan.is_macro(ctx) {
                let mut plans: Vec<FieldAssignPlan> = vec![plan];
                let mut j = i + 1;
                while j < stmts.len() {
                    let Some(next) = plan_field_assign(&stmts[j], env, top_level) else { break };
                    if !next.same_batch_as(&plans[0]) { break; }
                    plans.push(next);
                    j += 1;
                }
                let kinds: Vec<FieldAssignKind> = plans.into_iter()
                    .map(|p| p.render(ctx, top_level))
                    .collect();
                let clauses: Vec<String> = kinds.iter().map(|k| k.clause()).collect();
                kinds[0].emit_batch(out, indent, &clauses);
                i = j;
                continue;
            }
        }
        emit_stmt(out, indent, &stmts[i], fail_mode.clone(), ctx, env, top_level, fresh);
        i += 1;
    }
}

/// Lightweight pre-classification: identifies whether a statement is a
/// macro-batchable field assignment WITHOUT calling `emit_exp` (which mutates
/// codegen state). The full lowering happens later in `render`, by which time
/// we know we will actually emit the statement.
struct FieldAssignPlan<'s> {
    stmt: &'s typedexp::TypedStmt,
    base_name: String,
    /// The variable's full declared type, used in `render` to decide whether
    /// the record is stored as `Arc<T>` (macro lowering) or plain (in-place).
    base_ty: Ty,
    /// Resolved record qname whose fields we will look up (for struct path)
    /// OR `"<enum_qname>.<variant>"` for the variant path.
    record_qname: String,
    /// Set when the base is a known uniontype variant.
    variant: Option<(String, String)>,
}

impl<'s> FieldAssignPlan<'s> {
    /// Whether this plan will be lowered through one of the record-update
    /// macros (so batching is worthwhile). Variant assignments always use a
    /// macro; struct assignments use a macro only when the base is `Arc<T>`.
    fn is_macro(&self, ctx: &GenCtx) -> bool {
        self.variant.is_some() || constructor_needs_arc(&self.base_ty, ctx)
    }

    /// Two plans batch together iff they target the same base AND lower
    /// through the same macro (same struct OR same variant).
    fn same_batch_as(&self, other: &FieldAssignPlan<'_>) -> bool {
        self.base_name == other.base_name && self.variant == other.variant
    }

    fn render<'a>(
        self,
        ctx: &mut GenCtx,
        top_level: &'a BTreeMap<String, NameNode<'a>>,
    ) -> FieldAssignKind {
        let FieldAssignPlan { stmt, base_name, base_ty, record_qname, variant } = self;
        let typedexp::TypedStmt::Assign { lhs, rhs } = stmt else { unreachable!() };
        let TypedPat::FieldAccess { field, .. } = lhs else { unreachable!() };

        let scrut_ty = rhs.ty();
        let scrut_expr = emit_exp(rhs, /*is_const=*/false, ctx, top_level);

        let fields = record_field_tys(&record_qname, top_level)
            .filter(|v| !v.is_empty())
            .or_else(|| {
                let v = record_field_tys_by_simple_name(&record_qname, top_level);
                if v.is_empty() { None } else { Some(v) }
            })
            .unwrap_or_default();
        let lhs_ty = fields.iter().find(|(n, _)| n == field).map(|(_, t)| t.clone());
        let expr = coerce_assign_expr_pub(scrut_expr, &scrut_ty, lhs_ty.as_ref());
        let value = if struct_field_is_arc(&record_qname, field, top_level, ctx) {
            format!("Arc::new({expr})")
        } else {
            expr
        };
        let base_safe = escape_ident(&base_name).to_string();
        let field_safe = escape_ident(field).to_string();

        if let Some((enum_qname, variant_name)) = variant {
            let variant_path = build_variant_path(&enum_qname, &variant_name, ctx);
            FieldAssignKind::ArcVariant { base: base_safe, variant_path, field: field_safe, value }
        } else if constructor_needs_arc(&base_ty, ctx) {
            FieldAssignKind::ArcStruct { base: base_safe, field: field_safe, value }
        } else {
            FieldAssignKind::Plain { base: base_safe, field: field_safe, value }
        }
    }
}

/// Inspect a statement, using only `env`/`top_level` (no `ctx` mutation), to
/// decide whether it lowers to a record-update macro. Returns a plan for the
/// caller to render later when emission is committed.
fn plan_field_assign<'a, 's>(
    stmt: &'s typedexp::TypedStmt,
    env: &LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<FieldAssignPlan<'s>> {
    use typedexp::TypedStmt as S;
    let S::Assign { lhs, .. } = stmt else { return None };
    let TypedPat::FieldAccess { base, .. } = lhs else { return None };
    let TypedPat::Var(base_name) = base.as_ref() else { return None };
    let base_ty = env.vars.get(base_name)?.clone();

    // Variant path takes precedence: if the base variable is known to hold a
    // specific uniontype variant, lower through `assign_variant_field!`.
    if let Some((enum_qname, variant_name)) = env.variants.get(base_name).cloned() {
        let record_qname = format!("{enum_qname}.{variant_name}");
        // Only commit to the variant path if the record actually exists.
        if record_field_tys(&record_qname, top_level).is_some() {
            return Some(FieldAssignPlan {
                stmt,
                base_name: base_name.clone(),
                base_ty,
                record_qname,
                variant: Some((enum_qname, variant_name)),
            });
        }
    }

    let struct_qname = match &base_ty {
        Ty::RustStruct(q) | Ty::AliasTo(q) => q.clone(),
        _ => return None,
    };
    let record_qname = resolve_single_record_qname(&struct_qname, top_level)
        .unwrap_or_else(|| struct_qname.clone());
    // The record's fields must be resolvable for the rebuild to make sense.
    let has_fields = record_field_tys(&record_qname, top_level)
        .map(|v| !v.is_empty())
        .unwrap_or(false)
        || !record_field_tys_by_simple_name(&record_qname, top_level).is_empty();
    if !has_fields {
        return None;
    }
    Some(FieldAssignPlan {
        stmt,
        base_name: base_name.clone(),
        base_ty,
        record_qname: struct_qname.clone(),
        variant: None,
    })
}

fn build_variant_path(enum_qname: &str, variant_name: &str, ctx: &mut GenCtx) -> String {
    let shortened = ctx.shorten(enum_qname);
    if ctx.no_mod_uniontypes.contains(enum_qname) {
        return format!("{shortened}::{variant_name}");
    }
    let first = enum_qname.split('.').next().unwrap_or(enum_qname);
    let last = enum_qname.rsplit('.').next().unwrap_or(enum_qname);
    let in_own_mod = ctx.current_path.last().map(|p| p == last).unwrap_or(false);
    let needs_doubling = !in_own_mod && (
        (ctx.top_level_uniontype_names.contains(first) && first != ctx.top_name) ||
        (enum_qname.contains('.') && first != last)
    );
    if needs_doubling {
        format!("{shortened}::{last}::{variant_name}")
    } else {
        format!("{shortened}::{variant_name}")
    }
}

/// Result of inspecting an `Assign` statement to see whether it is a
/// record-field update that should be lowered through one of the
/// `assign_field!` / `assign_variant_field!` macros.
enum FieldAssignKind {
    /// `assign_field!(<base>.<field> = <value>);`
    ArcStruct { base: String, field: String, value: String },
    /// `assign_variant_field!(<base> => <variant_path>; <field> = <value>);`
    ArcVariant { base: String, variant_path: String, field: String, value: String },
    /// `<base>.<field> = <value>;` — plain owned struct, no macro needed.
    Plain { base: String, field: String, value: String },
}

impl FieldAssignKind {
    fn is_macro(&self) -> bool {
        matches!(self, FieldAssignKind::ArcStruct { .. } | FieldAssignKind::ArcVariant { .. })
    }

    /// Same base variable AND same macro path → safe to batch into one call.
    /// Plain assignments aren't batched (they don't share any setup cost).
    fn same_batch_as(&self, other: &FieldAssignKind) -> bool {
        match (self, other) {
            (FieldAssignKind::ArcStruct { base: a, .. }, FieldAssignKind::ArcStruct { base: b, .. }) => a == b,
            (
                FieldAssignKind::ArcVariant { base: a, variant_path: va, .. },
                FieldAssignKind::ArcVariant { base: b, variant_path: vb, .. },
            ) => a == b && va == vb,
            _ => false,
        }
    }

    /// The per-assignment clause inside the macro call (everything after the
    /// leading `<base> .` for struct, or just `<field> = <value>` for variant).
    fn clause(&self) -> String {
        match self {
            FieldAssignKind::ArcStruct { base, field, value } => format!("{base}.{field} = {value}"),
            FieldAssignKind::ArcVariant { field, value, .. } => format!("{field} = {value}"),
            FieldAssignKind::Plain { base, field, value } => format!("{base}.{field} = {value}"),
        }
    }

    fn emit_batch(&self, out: &mut String, indent: &str, clauses: &[String]) {
        let inner_indent = format!("{indent}    ");
        match self {
            FieldAssignKind::ArcStruct { base, .. } => {
                if clauses.len() == 1 {
                    writeln!(out, "{indent}assign_field!({});", clauses[0]).unwrap();
                } else {
                    writeln!(out, "{indent}assign_field!(").unwrap();
                    // Repeat the base ident on every line so the macro's
                    // repetition arm can match it; only the first one is the
                    // real binding, the rest are matched and discarded.
                    let _ = base; // silence unused warning when first clause already starts with base
                    for (k, c) in clauses.iter().enumerate() {
                        let comma = if k + 1 < clauses.len() { "," } else { "" };
                        writeln!(out, "{inner_indent}{c}{comma}").unwrap();
                    }
                    writeln!(out, "{indent});").unwrap();
                }
            }
            FieldAssignKind::ArcVariant { base, variant_path, .. } => {
                if clauses.len() == 1 {
                    writeln!(out, "{indent}assign_variant_field!({base} => {variant_path}; {});", clauses[0]).unwrap();
                } else {
                    writeln!(out, "{indent}assign_variant_field!({base} => {variant_path};").unwrap();
                    for (k, c) in clauses.iter().enumerate() {
                        let comma = if k + 1 < clauses.len() { "," } else { "" };
                        writeln!(out, "{inner_indent}{c}{comma}").unwrap();
                    }
                    writeln!(out, "{indent});").unwrap();
                }
            }
            FieldAssignKind::Plain { .. } => {
                // Plain assigns are not batched; emit each on its own line.
                for c in clauses {
                    writeln!(out, "{indent}{c};").unwrap();
                }
            }
        }
    }
}


/// Same as the nested `coerce_assign_expr` inside `emit_stmt`, lifted out so
/// the classification helper can use the same coercion logic.
fn coerce_assign_expr_pub(scrut_expr: String, scrut_ty: &Ty, lhs_ty: Option<&Ty>) -> String {
    let mut expr = scrut_expr;
    if let Ty::Tuple(_) = scrut_ty {
        if !matches!(lhs_ty, Some(Ty::Tuple(_))) {
            expr = format!("{expr}.0");
        }
    }
    if matches!(lhs_ty, Some(Ty::F64)) && *scrut_ty == Ty::I32 {
        expr = format!("({expr} as f64)");
    }
    expr
}

/// If a match arm's pattern asserts a particular uniontype variant, record
/// that fact for `local_env` so later field assignments on the scrutinee (or
/// on a `var as Constructor` binding) can be lowered through
/// `assign_variant_field!`.
///
/// Two pattern shapes seed `variants`:
///   1. `var as Constructor(..)` — the outer `var` binding is known to be
///      `Constructor`.
///   2. The match scrutinee is a plain `TypedExp::Var(name)` AND the pattern
///      is a `Constructor` for a uniontype variant — that named variable is
///      known to be the matched variant for the duration of the arm.
fn record_pattern_variants<'a>(
    pat: &TypedPat,
    scrutinee: &TypedExp,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) {
    record_pattern_variants_inner(pat, scrutinee, env, top_level, &mut HashMap::new(), None);
}

/// Like `record_pattern_variants`, plus collects per-binding `VarShape` info
/// for tracked variants. `shapes` is the output buffer.
fn record_pattern_variants_with_shapes<'a>(
    pat: &TypedPat,
    scrutinee: &TypedExp,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    shapes: &mut HashMap<String, VarShape>,
    ctx: &GenCtx,
) {
    record_pattern_variants_inner(pat, scrutinee, env, top_level, shapes, Some(ctx));
}

fn record_pattern_variants_inner<'a>(
    pat: &TypedPat,
    scrutinee: &TypedExp,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    shapes: &mut HashMap<String, VarShape>,
    ctx: Option<&GenCtx>,
) {
    let scrut_ty = scrutinee.ty();
    // (1) Outer `as` binding.
    if let TypedPat::As { var, pat: inner } = pat {
        if let Some((enum_q, variant)) = variant_of_pat(inner, &scrut_ty, top_level) {
            env.variants.insert(var.clone(), (enum_q, variant));
        }
    }
    // (2) Scrutinee that's a bare variable narrowed by the arm's pattern.
    let inner_pat = match pat {
        TypedPat::As { pat: inner, .. } => inner.as_ref(),
        other => other,
    };
    if let Some((enum_q, variant)) = variant_of_pat(inner_pat, &scrut_ty, top_level) {
        if let TypedExp::Var { name, .. } = scrutinee {
            env.variants.insert(name.clone(), (enum_q, variant));
        }
    }
    // (3) Tuple scrutinee + tuple pattern: pair each element. This is the
    //     common MetaModelica idiom
    //         match (v1, v2) { (CTOR_A { .. }, CTOR_B { .. }) => ... }
    //     where the arm body wants to read `v1.field_of_A` and
    //     `v2.field_of_B`. Without this pass those reads would not know
    //     which variant each element holds.
    if let (TypedPat::Tuple(pat_elems), TypedExp::Tuple(scrut_elems)) = (inner_pat, scrutinee) {
        if pat_elems.len() == scrut_elems.len() {
            for (sub_pat, sub_scrut) in pat_elems.iter().zip(scrut_elems.iter()) {
                record_pattern_variants_inner(sub_pat, sub_scrut, env, top_level, shapes, ctx);
            }
        }
    }
    // (4) Nested `As` bindings inside a Constructor pattern. The matched
    //     record carries field types; if a named-field pattern is
    //     `As { var, pat: Constructor(..) }` and that pattern asserts a
    //     specific uniontype variant, `var` is known to hold that variant
    //     for the duration of the arm.
    //
    //     The binding's Rust shape depends on whether the enclosing match
    //     crossed an Arc edge (causing `ref` bindings on every nested name)
    //     AND whether the bound field's own type is Arc-wrapped. When both
    //     hold, the binding is `&Arc<T>` — `VarShape::RefArc`. We only
    //     record a shape when the caller provided a `ctx` (so we can
    //     consult `recursive_types`); without it, `emit_var` falls back to
    //     the type-table-driven default.
    if let TypedPat::Constructor { name, named_fields, .. } = inner_pat {
        // Resolve the record's qname so we can look up field types. The
        // pattern's `ty` may already carry it; otherwise look it up against
        // the scrutinee's enum.
        let record_qname_opt = match record_field_tys_from_scrutinee_ctor(
            name.rsplit('.').next().unwrap_or(name),
            &scrut_ty,
            top_level,
        ) {
            Some(tys) => Some(tys),
            None => {
                let v = record_field_tys_by_simple_name(name.rsplit('.').next().unwrap_or(name), top_level);
                if v.is_empty() { None } else { Some(v) }
            }
        };
        if let Some(field_tys) = record_qname_opt {
            let scrut_crosses_arc = ctx.map(|c| ty_needs_arc_match_deref(&scrut_ty, c)).unwrap_or(false);
            for (fname, fpat) in named_fields {
                let TypedPat::As { var, pat: inner_as } = fpat else { continue };
                let Some(field_ty) = field_tys.iter().find(|(n, _)| n == fname).map(|(_, t)| t.clone()) else { continue };
                if let Some((enum_q, variant)) = variant_of_pat(inner_as, &field_ty, top_level) {
                    env.variants.insert(var.clone(), (enum_q, variant));
                    if let Some(c) = ctx {
                        let field_is_arc = is_arc_wrapped(&field_ty, c);
                        let shape = match (scrut_crosses_arc, field_is_arc) {
                            (true, true)   => VarShape::RefArc,
                            // Other combinations are not yet implemented:
                            //   (true, false)  → &T (would need a `var_field!(*v.f, ..)` arm)
                            //   (false, true)  → Arc<T> by-value (already handled by emit_var fallback)
                            //   (false, false) → owned T  (same)
                            // For (false, true)/(false, false) we leave the
                            // shape unset; emit_var's fn_env_vars fallback
                            // applies — though for pattern bindings that
                            // fallback returns Unknown ⇒ Owned, which is
                            // wrong for Arc fields. That case is not
                            // exercised by the current corpus; if it shows
                            // up, add `VarShape::Arc` here and the right
                            // arm above.
                            _ => continue,
                        };
                        shapes.insert(var.clone(), shape);
                    }
                }
            }
        }
    }
}

/// If `pat` is a constructor pattern that proves the matched value is a
/// specific multi-record uniontype variant, return `(enum_qname, variant)`.
///
/// We try, in order:
///   1. The pattern's `ty` already promoted to `UnionTypeVariant` by inference.
///   2. The pattern's `ty` as `RustStruct("Parent.Variant")` — a record qname
///      where the parent is the enclosing uniontype.
///   3. Fall back to the scrutinee's type: if it's a multi-record uniontype
///      and the pattern's name matches one of its records, accept that record
///      as the variant. This covers patterns written with a bare simple name
///      (e.g. `case NODE(...)`), which the type inferencer leaves as
///      `Ty::Unknown` because the unqualified name doesn't resolve at the top
///      level of the hierarchy.
fn variant_of_pat<'a>(
    pat: &TypedPat,
    scrut_ty: &Ty,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<(String, String)> {
    let TypedPat::Constructor { name, ty, .. } = pat else { return None };
    if let Ty::UnionTypeVariant(enum_q, variant) = ty {
        return Some((enum_q.clone(), variant.clone()));
    }
    if let Ty::RustStruct(qname) = ty {
        if let Some((parent, variant)) = qname.rsplit_once('.') {
            let simple_name = name.rsplit('.').next().unwrap_or(name);
            if simple_name == variant {
                return Some((parent.to_owned(), variant.to_owned()));
            }
        }
    }
    // Fall back to scrutinee type: find the uniontype's record whose simple
    // name equals the pattern's name.
    let enum_qname = match scrut_ty {
        Ty::RustEnum(q) | Ty::AliasTo(q) => q.clone(),
        _ => return None,
    };
    let simple_name = name.rsplit('.').next().unwrap_or(name);
    let enum_node = lookup_node(&enum_qname, top_level)?;
    let NodeKind::Class(c) = &enum_node.kind else { return None };
    if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) {
        return None;
    }
    for (child_name, child_node) in &enum_node.children {
        if let NodeKind::Class(cc) = &child_node.kind {
            if matches!(cc.restriction,
                Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })
                && child_name == simple_name
            {
                return Some((enum_qname.clone(), child_name.clone()));
            }
        }
    }
    None
}

/// Control-flow analysis for `try`/`else` lowering.
///
/// Either the statement list always diverges (return / fail() / break /
/// continue / nested constructs in which every branch diverges), or it can
/// fall through — in which case we report the set of variables definitely
/// assigned on **every** path that reaches the end. Conservative on shapes
/// we don't analyse (loops, match-in-expression position, complex LHSs):
/// fall-through with no contributed assignments.
enum FlowResult {
    Diverges,
    FallsThrough(HashSet<String>),
}

fn is_fail_call(exp: &TypedExp) -> bool {
    matches!(exp, TypedExp::Call { func, .. } if func == "fail")
}

fn merge_branch_flows(branches: &[FlowResult]) -> FlowResult {
    let mut acc: Option<HashSet<String>> = None;
    for b in branches {
        match b {
            FlowResult::Diverges => continue,
            FlowResult::FallsThrough(set) => {
                acc = Some(match acc {
                    None => set.clone(),
                    Some(cur) => cur.intersection(set).cloned().collect(),
                });
            }
        }
    }
    match acc {
        None => FlowResult::Diverges,
        Some(s) => FlowResult::FallsThrough(s),
    }
}

fn stmt_flow(s: &typedexp::TypedStmt) -> FlowResult {
    use typedexp::TypedStmt as S;
    match s {
        S::Return | S::Break | S::Continue => FlowResult::Diverges,
        S::Assign { lhs, rhs } => {
            if is_fail_call(rhs) { return FlowResult::Diverges; }
            let mut set = HashSet::new();
            pat_assigned_names(lhs, &mut set);
            FlowResult::FallsThrough(set)
        }
        S::NoRetCall { call } => {
            if is_fail_call(call) { FlowResult::Diverges }
            else { FlowResult::FallsThrough(HashSet::new()) }
        }
        S::If { then_, elseif, else_, .. } => {
            let mut branches: Vec<FlowResult> = Vec::with_capacity(2 + elseif.len());
            branches.push(stmts_flow(then_));
            for (_, eb) in elseif { branches.push(stmts_flow(eb)); }
            branches.push(stmts_flow(else_));
            merge_branch_flows(&branches)
        }
        S::Try { body, else_body } => {
            merge_branch_flows(&[stmts_flow(body), stmts_flow(else_body)])
        }
        S::For { .. } | S::While { .. } | S::Failure { .. } | S::Todo(_) => {
            FlowResult::FallsThrough(HashSet::new())
        }
    }
}

fn stmts_flow(stmts: &[typedexp::TypedStmt]) -> FlowResult {
    let mut acc: HashSet<String> = HashSet::new();
    for s in stmts {
        match stmt_flow(s) {
            FlowResult::Diverges => return FlowResult::Diverges,
            FlowResult::FallsThrough(set) => acc.extend(set),
        }
    }
    FlowResult::FallsThrough(acc)
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
    fn coerce_assign_expr(scrut_expr: String, scrut_ty: &Ty, lhs_ty: Option<&Ty>) -> String {
        let mut expr = scrut_expr;
        if let Ty::Tuple(_) = scrut_ty {
            if !matches!(lhs_ty, Some(Ty::Tuple(_))) {
                expr = format!("{expr}.0");
            }
        }
        if matches!(lhs_ty, Some(Ty::F64)) && *scrut_ty == Ty::I32 {
            expr = format!("({expr} as f64)");
        }
        // A range can't be stored in an Array/List binding without
        // materialising it. We haven't lowered that path yet; emit a TODO so
        // the failure shows up at the call site rather than as opaque type
        // mismatch noise.
        if matches!(scrut_ty, Ty::Range(_))
            && matches!(lhs_ty, Some(Ty::Array(_)) | Some(Ty::List(_)))
        {
            expr = format!("/* TODO: materialise Range into Array/List */ {expr}");
        }
        maybe_clone_string_value(expr, scrut_ty)
    }

    fn lhs_assignment_ty(lhs: &TypedPat, env: &LocalEnv) -> Option<Ty> {
        match lhs {
            TypedPat::Var(name) => env.vars.get(name).cloned(),
            TypedPat::Index { base, .. } => {
                let base_ty = base.ty();
                match base_ty {
                    Ty::Array(inner) | Ty::List(inner) => Some(*inner),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { lhs, rhs } => {
            let scrut_ty = rhs.ty();
            let scrut_expr = emit_exp(rhs, /*is_const=*/false, ctx, top_level);
            // For irrefutable patterns we still want a single binding form.
            // But MetaModelica often assigns to *existing* variables (declared as outputs
            // or `protected` components). For a Var pattern we emit a plain `<name> = expr;`
            // when the binding is already in scope, else a `let`. Heuristic: if env has it,
            // it's an output or earlier protected — emit assignment.
            if let TypedPat::Var(name) = lhs {
                if env.vars.contains_key(name) {
                    // Plain reassignment may switch to a different variant — the
                    // previously-known variant assertion no longer holds.
                    env.variants.remove(name);
                    let lhs_ty = env.vars.get(name).cloned();
                    // MetaModelica permits assigning a multi-output call to a single
                    // variable; the unmentioned outputs are silently discarded.
                    // Emit `(name, _, _, ...) = expr;` so the user-visible binding
                    // gets the first output and the rest are dropped, while keeping
                    // the call expression evaluated exactly once.
                    if let Ty::Tuple(tys) = &scrut_ty {
                        if !matches!(lhs_ty, Some(Ty::Tuple(_))) && tys.len() >= 2 {
                            let mut slots: Vec<String> = Vec::with_capacity(tys.len());
                            slots.push(escape_ident(name).to_string());
                            for _ in 1..tys.len() { slots.push("_".to_owned()); }
                            writeln!(out, "{indent}({}) = {scrut_expr};", slots.join(", ")).unwrap();
                            return;
                        }
                    }
                    let scrut_expr = coerce_assign_expr(scrut_expr, &scrut_ty, lhs_ty.as_ref());
                    writeln!(out, "{indent}{} = {scrut_expr};", escape_ident(name)).unwrap();
                    return;
                }
            }
            // Special case: tuple of plain variables, all already in scope. Emit a
            // direct destructuring assignment so we don't need fresh temporaries.
            // This handles patterns like `(e1, e2, e3) := t;` where e1/e2/e3 were
            // declared earlier (e.g. as `protected` components).
            if let TypedPat::Tuple(pats) = lhs {
                let all_existing_vars = !pats.is_empty() && pats.iter().all(|p| match p {
                    TypedPat::Var(n) => env.vars.contains_key(n),
                    TypedPat::Wildcard => true,
                    _ => false,
                });
                if all_existing_vars {
                    let slots: Vec<String> = pats.iter().map(|p| match p {
                        TypedPat::Var(n) => escape_ident(n).to_string(),
                        TypedPat::Wildcard => "_".to_owned(),
                        _ => unreachable!(),
                    }).collect();
                    writeln!(out, "{indent}({}) = {scrut_expr};", slots.join(", ")).unwrap();
                    return;
                }
            }
            if let TypedPat::Index { base, index } = lhs {
                let lhs_ty = lhs_assignment_ty(lhs, env);
                let scrut_expr = coerce_assign_expr(scrut_expr, &scrut_ty, lhs_ty.as_ref());
                let idx_str = emit_exp(index, /*is_const=*/false, ctx, top_level);
                match base.ty() {
                    Ty::Array(_) => {
                        // Modelica `arr[i] := rhs;` on an Array<T> (= Rc<RefCell<Vec<T>>>).
                        // Two hazards to avoid:
                        //   1. The cell needs `borrow_mut()` for writing; plain indexing
                        //      on the Rc handle gives no IndexMut impl.
                        //   2. The rhs may itself borrow the same array (e.g. swap), so
                        //      we hoist it into a local temp first to drop any Ref before
                        //      acquiring the RefMut — otherwise RefCell would panic at runtime.
                        let n = *fresh; *fresh += 1;
                        let tmp = format!("__cell{n}");
                        let base_str = emit_exp(base, /*is_const=*/false, ctx, top_level);
                        writeln!(out, "{indent}{{").unwrap();
                        writeln!(out, "{indent}    let {tmp} = {scrut_expr};").unwrap();
                        writeln!(out, "{indent}    {base_str}.borrow_mut()[({idx_str}-1) as usize] = {tmp};").unwrap();
                        writeln!(out, "{indent}}}").unwrap();
                    }
                    _ => {
                        // Non-array indexed LHS (e.g. mutable slice/Vec passed by reference).
                        // No known MetaModelica construct hits this today; fall back to the
                        // direct form and let the Rust compiler diagnose if we got it wrong.
                        let lhs_str = emit_pat(lhs, ctx, top_level);
                        writeln!(out, "{indent}{lhs_str} = {scrut_expr}; // TODO: indexed assign on non-Array base").unwrap();
                    }
                }
                return;
            }
            if let TypedPat::FieldAccess { base, field } = lhs {
                // The macro lowering (assign_field!/assign_variant_field!) is
                // applied by `emit_stmts`'s pre-pass before this function runs,
                // using `plan_field_assign` to avoid double-rendering the rhs.
                // Reaching this branch in `emit_stmt` means the pre-pass found
                // no lowering — handle the single-statement non-macro case.
                if let Some(plan) = plan_field_assign(stmt, env, top_level) {
                    let kind = plan.render(ctx, top_level);
                    kind.emit_batch(out, indent, &[kind.clause()]);
                    return;
                }
                // Fallback: direct field assignment. Works only when `base` is not
                // an Arc-wrapped record (rare); kept so unhandled shapes surface as
                // a Rust error rather than a silent miscompile.
                let lhs_str = field_access_to_dotted(base, field);
                let lhs_ty = lhs_assignment_ty(lhs, env);
                let scrut_expr = coerce_assign_expr(scrut_expr, &scrut_ty, lhs_ty.as_ref());
                writeln!(out, "{indent}{lhs_str} = {scrut_expr}; // TODO: unhandled field-assign shape").unwrap();
                return;
            }
            emit_pat_assign(out, indent, lhs, &scrut_ty, &scrut_expr, fail_mode, ctx, env, top_level, fresh);
        }
        S::NoRetCall { call } => {
            let s = emit_exp(call, false, ctx, top_level);
            writeln!(out, "{indent}{s};").unwrap();
        }
        S::If { cond, then_, elseif, else_ } => {
            let c = emit_exp(cond, false, ctx, top_level);
            writeln!(out, "{indent}if {c} {{").unwrap();
            emit_stmts(out, &format!("{indent}    "), then_, fail_mode.clone(), ctx, env, top_level, fresh);
            for (ec, eb) in elseif {
                let cs = emit_exp(ec, false, ctx, top_level);
                writeln!(out, "{indent}}} else if {cs} {{").unwrap();
                emit_stmts(out, &format!("{indent}    "), eb, fail_mode.clone(), ctx, env, top_level, fresh);
            }
            if !else_.is_empty() {
                writeln!(out, "{indent}}} else {{").unwrap();
                emit_stmts(out, &format!("{indent}    "), else_, fail_mode, ctx, env, top_level, fresh);
            }
            writeln!(out, "{indent}}}").unwrap();
        }
        S::For { var, range, body } => {
            let r = emit_exp(range, false, ctx, top_level);
            // List<T> behind `Arc` → iterate via `&*lst` (Deref<Target=List>).
            // Array<T> = `Rc<RefCell<Vec<T>>>` → iterate via `arr.borrow().iter()`
            //   then `.cloned()` so the loop variable owns its element instead of
            //   borrowing through the RefCell guard for the whole body (avoids
            //   holding a borrow across user code that may itself touch the array).
            let r = match range.ty() {
                Ty::List(..) => format!("&*{r}"),
                Ty::Array(..) => format!("{r}.borrow().iter().cloned().collect::<Vec<_>>()"),
                Ty::Range(..) => r,
                // `(a..=b).step_by(..)`), so feed it straight to `for ... in`.
                t => format!("{r} /* Unknown type for iterator {:?} */", t),
            };
            writeln!(out, "{indent}for {} in {r} {{", escape_ident(var)).unwrap();
            // Element type: peel List/Array.
            let elem_ty = match range.ty() { Ty::List(t) | Ty::Array(t) | Ty::Range(t) => *t, _ => Ty::Unknown };
            let mut inner = env.clone();
            inner.vars.insert(var.clone(), elem_ty);
            emit_stmts(out, &format!("{indent}    "), body, fail_mode, ctx, &mut inner, top_level, fresh);
            writeln!(out, "{indent}}}").unwrap();
        }
        S::While { cond, body } => {
            let c = emit_exp(cond, false, ctx, top_level);
            writeln!(out, "{indent}while {c} {{").unwrap();
            emit_stmts(out, &format!("{indent}    "), body, fail_mode, ctx, env, top_level, fresh);
            writeln!(out, "{indent}}}").unwrap();
        }
        S::Try { body, else_body } => {
            // ── Single-statement fast path ──────────────────────────────────
            // When the body is exactly one `PAT := CALL` assignment and the
            // else-branch always diverges, emit a concise `if let Ok(PAT) =
            // CALL { body } else { else }` without any labeled block.
            // `CALL` is emitted in `QMode::Bare` so it yields the raw
            // `Result<T, E>`; the `if let Ok(…)` handles the matching.
            // Because the else diverges, Rust's flow analysis proves that any
            // variable assigned in the then-branch is definitely initialised
            // after the statement — no tuple-yield machinery needed.
            if body.len() == 1 && matches!(stmts_flow(else_body), FlowResult::Diverges) {
                if let typedexp::TypedStmt::Assign { lhs, rhs } = &body[0] {
                    let scrut_ty = rhs.ty();
                    let scrut_expr = ctx.with_qmode(QMode::Bare, |ctx| {
                        emit_exp(rhs, /*is_const=*/false, ctx, top_level)
                    });
                    let mut else_str = String::new();
                    let mut eenv = env.clone();
                    emit_stmts(&mut else_str, &format!("{indent}    "), else_body, fail_mode, ctx, &mut eenv, top_level, fresh);
                    emit_pat_assign(out, indent, lhs, &scrut_ty, &scrut_expr,
                        FailureMode::IfLetElse(else_str), ctx, env, top_level, fresh);
                    return;
                }
            }

            // Lower `try body else else_body end try;` to a labeled Rust block
            // rather than an IIFE. The IIFE form (`(|| -> Result<_> { .. })()`)
            // cannot use `let mut x: T;` declared in the surrounding statement
            // scope: the closure would have to capture `x`, but a mutable
            // borrow of an uninitialised binding is rejected by the borrow
            // checker. A labeled block executes in-line, so all surrounding
            // locals are in scope and can be assigned to.
            //
            // Inside the block we cannot use `?` either — that would propagate
            // out of the enclosing function, defeating the `try/else` recovery.
            // Instead, fallible calls are emitted as
            // `unwrap_break_err!(expr, '__tryN)`, which `break '__tryN Err(e)`
            // on failure, exiting the block with an `Err` value the surrounding
            // dispatch then handles.
            //
            // When the body definitely assigns outer-scoped variables on every
            // non-diverging path, Rust's flow analysis cannot connect "block
            // returned Ok" with "the assignment ran" — so reads after the try
            // appear as use-of-possibly-uninit. To fix this we yield those
            // variables as a tuple from the block's tail, then reassign them
            // in the `Ok` arm of a surrounding `match`. A variable is only
            // safe to yield if the else-branch either also definitely assigns
            // it OR diverges (so the join-point after the match is reached
            // only with the variable definitely assigned).
            let label = format!("'__try{}", *fresh);
            let label_n = *fresh;
            *fresh += 1;

            let body_flow = stmts_flow(body);
            let else_flow = stmts_flow(else_body);
            let yield_vars: Vec<String> = match &body_flow {
                FlowResult::Diverges => Vec::new(),
                FlowResult::FallsThrough(body_init) => {
                    let mut v: Vec<String> = body_init.iter()
                        .filter(|name| env.vars.contains_key(*name))
                        .filter(|name| match &else_flow {
                            FlowResult::Diverges => true,
                            FlowResult::FallsThrough(else_init) => else_init.contains(*name),
                        })
                        .cloned()
                        .collect();
                    v.sort();
                    v
                }
            };

            if yield_vars.is_empty() {
                writeln!(out, "{indent}if {label}: {{", ).unwrap();
                let mut benv = env.clone();
                ctx.with_qmode(QMode::TryBlock(label.clone()), |ctx| {
                    emit_stmts(out, &format!("{indent}    "), body, FailureMode::TryArm, ctx, &mut benv, top_level, fresh);
                });
                writeln!(out, "{indent}    Ok::<(), anyhow::Error>(())").unwrap();
                writeln!(out, "{indent}}}.is_err() {{").unwrap();
                let mut eenv = env.clone();
                emit_stmts(out, &format!("{indent}    "), else_body, fail_mode, ctx, &mut eenv, top_level, fresh);
                writeln!(out, "{indent}}}").unwrap();
            } else {
                let escaped_vars: Vec<String> = yield_vars.iter().map(|n| escape_ident(n)).collect();
                let yield_tuple = if yield_vars.len() == 1 {
                    format!("({},)", escaped_vars[0])
                } else {
                    format!("({})", escaped_vars.join(", "))
                };
                let temp_names: Vec<String> = (0..yield_vars.len())
                    .map(|i| format!("__try{label_n}_o{i}"))
                    .collect();
                let temp_pat = if yield_vars.len() == 1 {
                    format!("({},)", temp_names[0])
                } else {
                    format!("({})", temp_names.join(", "))
                };

                writeln!(out, "{indent}match {label}: {{").unwrap();
                let mut benv = env.clone();
                ctx.with_qmode(QMode::TryBlock(label.clone()), |ctx| {
                    emit_stmts(out, &format!("{indent}    "), body, FailureMode::TryArm, ctx, &mut benv, top_level, fresh);
                });
                writeln!(out, "{indent}    Ok::<_, anyhow::Error>({yield_tuple})").unwrap();
                writeln!(out, "{indent}}} {{").unwrap();
                writeln!(out, "{indent}    Ok({temp_pat}) => {{").unwrap();
                for (v, t) in escaped_vars.iter().zip(temp_names.iter()) {
                    writeln!(out, "{indent}        {v} = {t};").unwrap();
                }
                writeln!(out, "{indent}    }}").unwrap();
                writeln!(out, "{indent}    Err(_) => {{").unwrap();
                let mut eenv = env.clone();
                emit_stmts(out, &format!("{indent}        "), else_body, fail_mode, ctx, &mut eenv, top_level, fresh);
                writeln!(out, "{indent}    }}").unwrap();
                writeln!(out, "{indent}}}").unwrap();
            }
        }
        S::Failure { body } => {
            // `failure(body)` succeeds iff the body fails. We use the same
            // labeled-block lowering as `try`/`else` so any `let mut` locals
            // declared in the surrounding scope are still assignable from
            // within the body — and so a fallible call short-circuits to a
            // recoverable `Err` rather than to the enclosing function.
            let label = format!("'__try{}", *fresh);
            *fresh += 1;
            writeln!(out, "{indent}if {label}: {{").unwrap();
            let mut fenv = env.clone();
            ctx.with_qmode(QMode::TryBlock(label.clone()), |ctx| {
                emit_stmts(out, &format!("{indent}    "), body, FailureMode::TryArm, ctx, &mut fenv, top_level, fresh);
            });
            writeln!(out, "{indent}    Ok::<(), anyhow::Error>(())").unwrap();
            writeln!(out, "{indent}}}.is_ok() {{ bail!(\"failure(): body succeeded\") }}").unwrap();
        }
        S::Return => {
            // Expand `return;` into the same Ok(...) shape that emit_function produces
            // at the end of the function body, so the early-exit path returns the
            // current values of the output components.
            match env.outputs.len() {
                0 => writeln!(out, "{indent}return Ok(());").unwrap(),
                1 => writeln!(out, "{indent}return Ok({});", escape_ident(&env.outputs[0])).unwrap(),
                _ => {
                    let parts: Vec<String> = env.outputs.iter().map(|n| escape_ident(n)).collect();
                    writeln!(out, "{indent}return Ok(({}));", parts.join(", ")).unwrap();
                }
            }
        }
        S::Break    => writeln!(out, "{indent}break;").unwrap(),
        S::Continue => writeln!(out, "{indent}continue;").unwrap(),
        S::Todo(s)  => writeln!(out, "{indent}/* todo stmt: {} */", s.chars().take(60).collect::<String>()).unwrap(),
    }
}

// ── Type formatting ───────────────────────────────────────────────────────────

/// Format a type as it appears in a function-parameter position.
///
/// Differs from `fmt_ty` only for function-typed parameters: we emit
/// `impl Fn(...) -> Result<...>` rather than a concrete `fn(...)` pointer
/// or a named `partial function` alias (`KeyEq<T>`). The `impl Fn` form
/// accepts:
///   * a function item / `fn` pointer (a plain function name),
///   * a closure (e.g. emitted by PARTEVALFUNCTION lowering), and
///   * any other `Fn(...) -> ...` implementor.
///
/// `Fn` (not `FnMut`/`FnOnce`) is chosen because callbacks in this codebase
/// are invoked repeatedly inside `fold`/`map`/`forEach` loops and must not
/// consume their captures. If we ever lower a partial application that
/// needs to consume its captures (or mutate them), the bound must be
/// relaxed at that point rather than tightened ad hoc.
///
/// Notes / limitations:
///   * Named `partial function` aliases (e.g. `KeyEq<T>`) are still emitted
///     by `fmt_ty` itself, so they continue to work as struct field types
///     and `type X = ...` aliases. Inside such aliases the function value
///     is still a `fn` pointer; closures cannot be stored there. Lifting
///     that limitation requires switching the alias to `Rc<dyn Fn(...)>`
///     (or similar) and updating every passing convention — out of scope
///     for this change.
///   * `Ty::FunctionAlias` (re-export `function Foo = Bar`) is treated
///     identically to a plain function pointer; we still emit it by alias
///     name here, since lifting the same restriction needs the same
///     refactor.
fn fmt_param_ty(ty: &Ty, ctx: &mut GenCtx) -> String {
    match ty {
        // Only ANONYMOUS function types get the `impl Fn` treatment. A named
        // `partial function` (e.g. `KeyEq`) is typically also used as a
        // struct/record field (where `impl Trait` is illegal) — switching
        // those aliases is a larger refactor that has to change how field
        // storage and the corresponding constructors box the callback. Until
        // that lands, named aliases keep their concrete `fn(...)` shape so
        // that the same parameter can be stored in a record field of the
        // alias type.
        Ty::Function { inputs, output, name: None, .. } => {
            let ins = inputs.iter().map(|inp| fmt_ty(&inp.ty, ctx)).collect::<Vec<_>>().join(", ");
            format!("impl Fn({ins}) -> Result<{}>", fmt_ty(output, ctx))
        }
        _ => fmt_ty(ty, ctx),
    }
}

fn fmt_ty(ty: &Ty, ctx: &mut GenCtx) -> String {
    match ty {
        Ty::Unknown => "/* ? */".to_owned(),
        Ty::I32 => "i32".to_owned(),
        Ty::F64 => "f64".to_owned(),
        Ty::Bool => "bool".to_owned(),
        Ty::Str => "ArcStr".to_owned(),
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
        Ty::List(inner) => format!("Arc<metamodelica::List<{}>>", fmt_ty(inner, ctx)),
        // MetaModelica `array<T>` has reference (aliasing) semantics: arrayUpdate
        // mutates in place and the change is visible through every alias. We model
        // that with `metamodelica::Array<T>` (alias for `Rc<RefCell<Vec<T>>>`).
        // Single-threaded shared mutability — no lock cost, no deadlock risk.
        Ty::Array(inner) => format!("metamodelica::Array<{}>", fmt_ty(inner, ctx)),
        // Ranges have no surface Rust type that captures both `RangeInclusive`
        // and `StepBy<RangeInclusive>`; they're meant to be consumed in-place
        // by a for-loop or reduction iterator. Flowing into a typed slot means
        // the user wrote something we haven't lowered yet.
        Ty::Range(inner) => format!("/* TODO: Range<{}> escaped iterator context */ ()", fmt_ty(inner, ctx)),
        Ty::Tuple(tys) => {
            format!("({})", tys.iter().map(|t| fmt_ty(t, ctx)).collect::<Vec<_>>().join(", "))
        }
        Ty::Function { type_vars: _, inputs, output, name } => {
            // If this function type was introduced by a named `partial function`
            // declaration, emit a reference to the Rust type alias rather than
            // inlining the raw `fn(...) -> Result<...>` signature. The type
            // arguments come from whatever TypeVars are present in the inputs
            // and output after unification — at a use site like `eqFn: KeyEq`
            // inside `uniontype UnorderedSet<T>` those TypeVars are still `T`,
            // so we emit `KeyEq<T>`; after call-site unification they may be
            // concrete types (e.g. `KeyEq<i32>`).
            if let Some(qname) = name {
                let short = ctx.shorten(qname);
                let mut tvs: Vec<Ty> = Vec::new();
                let mut seen: Vec<String> = Vec::new();
                let mut push_unique = |t: &Ty, tvs: &mut Vec<Ty>, seen: &mut Vec<String>| {
                    let mut here: Vec<String> = Vec::new();
                    collect_type_vars_in_ty(t, &mut here);
                    for v in here {
                        if !seen.contains(&v) {
                            seen.push(v.clone());
                            tvs.push(Ty::TypeVar(v));
                        }
                    }
                };
                for inp in inputs.iter() {
                    push_unique(&inp.ty, &mut tvs, &mut seen);
                }
                push_unique(output, &mut tvs, &mut seen);
                if tvs.is_empty() {
                    return short;
                }
                let args = tvs.iter().map(|t| fmt_ty(t, ctx)).collect::<Vec<_>>().join(", ");
                return format!("{short}<{args}>");
            }
            let ins = inputs.iter().map(|inp| fmt_ty(&inp.ty, ctx)).collect::<Vec<_>>().join(", ");
            format!("fn({ins}) -> Result<{}>", fmt_ty(output, ctx))
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
        "unsized" | "virtual" | "yield" |
        // primitive/builtin types that can appear as identifiers in translated MM code
        "str" => format!("r#{name}"),
        _ => name.to_owned(),
    }
}

fn path_to_dotted(path: &Absyn::Path) -> String {
    match path {
        Absyn::Path::IDENT { name } => name.to_string(),
        Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", path_to_dotted(path)),
        Absyn::Path::FULLYQUALIFIED { path } => path_to_dotted(path),
    }
}