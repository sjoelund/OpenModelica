#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write;
use std::sync::Arc;
use rayon::prelude::*;
use openmodelica_ast::Absyn;
use crate::MM;
use std::collections::HashMap;
use crate::hierarchy::{InstanceHierarchy, FunctionInput, NameNode, NodeKind, Ty, extract_default, extract_default_exp, lookup_node, lookup_node_ty, lookup_record_through_unions, uniontype_needs_mod, collect_type_vars_in_ty};
use crate::typedexp::{self, TypedExp, TypedPat, TypedCase, Lit, BinOpKind, UnOpKind, MatchKind, ReductionIter, ReductionIterKind, cref_to_dotted, CrefSegment};
use anyhow::{Result,bail};

// ── Import-aware generation context ──────────────────────────────────────────

/// Trait bounds applied to every type-variable parameter on a generated
/// function or type. We need:
///  * `Clone` — MetaModelica passes by value with copy-on-store semantics; the
///    lowering inserts `.clone()` aggressively to keep that semantics.
///  * `PartialEq` — required so values can flow through `case` matches and
///    `==` comparisons that the source code may use on type-variable values.
///  * `'static` — required wherever a generic value crosses a *type-erased*
///    storage boundary. The two such boundaries we generate code for are
///    `setGlobalRoot`/`getGlobalRoot` (which downcast through `Rc<dyn Any>`,
///    and `Any` is `: 'static`) and `referenceEq`/`referenceDebugString`.
///    Without the bound, a generic function like
///    `getCurrentComponent<T>() { tpl = getGlobalRoot(...); ... }` won't
///    compile because the inferred type for the slot includes `T`, and the
///    `Any` bound requires `T: 'static`. MetaModelica values never carry
///    borrowed references (they're all `Rc<…>`/`ArcStr`/owned data), so
///    `'static` is satisfied by every type we ever instantiate generics with;
///    making it part of the default bound is honest, not a shortcut.
const DEFAULT_TRAITS: &str = "Clone + PartialEq + 'static";

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
    /// Maps the simple name of a member exposed by a wildcard (`.*`) import to
    /// the dotted name of the module that exposes it (e.g. `InnerOuter` →
    /// `NFPrefixes` when `import NFPrefixes.*;` is in scope and `NFPrefixes`
    /// contains a uniontype `InnerOuter`). Consulted by [`Self::shorten`] to
    /// avoid mis-qualifying a bare reference to such a member as a top-level
    /// package — there may be an unrelated top-level package of the same name
    /// (e.g. `FrontEnd/InnerOuter.mo`) which would otherwise be pulled in via
    /// `implicit_modules` and shadow the wildcard-imported type.
    wildcard_members: BTreeMap<String, String>,
    /// Simple names of direct children of the file's top-level node (other
    /// uniontypes / packages / functions defined inside the same `.mo` file).
    /// These are reachable by bare name from anywhere in the file (Rust's
    /// `use super::*;` in nested mods plus same-module scoping) and must not be
    /// mistaken for a top-level package of the same name — otherwise a bare
    /// reference like `InnerOuter.NOT_INNER_OUTER` from inside `NFPrefixes.mo`
    /// (where `InnerOuter` is a sibling uniontype) would pull in
    /// `openmodelica_frontend::InnerOuter` via `implicit_modules`.
    self_members: HashSet<String>,
    /// Explicit imports: dotted qualified name → local name.""
    named: BTreeMap<String, String>,
    /// Local names that alias the current module itself (e.g. `import Type = NFType;`
    /// inside `NFType.mo`). MetaModelica lets you self-alias a uniontype so the
    /// source can refer to it as `Type.ANY` instead of `NFType.ANY`. We don't emit
    /// a `use` line for self-aliases (a `use crate::NFType as Type;` from inside
    /// NFType.rs would be a cycle), but we still need to *recognise* the local
    /// name in lookups so dotted refs like `Type.ANY` resolve to `NFType::ANY`.
    self_aliases: BTreeSet<String>,
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
    /// Fully-qualified names of types that transitively contain a `metamodelica::Array<T>`
    /// field. Such types are not `Sync` (because `Array<T> = Rc<RefCell<Vec<T>>>`) and
    /// therefore cannot be the type of a `pub static`. Populated from
    /// `InstanceHierarchy::types_containing_array`. Consulted by `ty_is_sync`.
    types_containing_array: BTreeSet<String>,
    /// Fully-qualified MetaModelica names (dot-separated) of `constant`
    /// components emitted as `pub const fn name() -> T { ... }` getters rather
    /// than as `pub static`. These are constants whose value is const-emittable
    /// but whose type is not `Sync` (because it transitively contains
    /// `Array<T>`); a `pub static` of that type fails to compile, and the value
    /// has no allocations so a `const fn` getter is the right escape hatch.
    /// References to such names in expression position must be emitted as a
    /// function call (`name()`) rather than as a bare value — see the `Var`
    /// arm of [`emit_exp`].
    const_fn_getters: BTreeSet<String>,
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
    /// Variables whose current value was created by `arrayCreateNoInit` and
    /// whose slots have not yet all been initialised. Indexed writes to such
    /// variables must use `ptr::write` (via `Dangerous::arrayInitSlot`) rather
    /// than a plain `=` assignment, because `=` would drop the uninitialised
    /// garbage value that occupies the slot — UB for non-trivially-destructed
    /// types such as `Arc<T>`. Cleared / updated in `emit_stmt` alongside
    /// `fn_env_vars`; saved and restored around nested-function emissions.
    uninit_arrays: HashSet<String>,
    /// Variables in scope at the enclosing function level: inputs, outputs,
    /// and protected locals. Used to seed the per-arm `LocalEnv` when entering
    /// a match-expression case body, so assignments to function-level outputs
    /// or protected components are recognised as re-assignments (plain `=`)
    /// rather than fresh `let` shadowings. Cleared between functions.
    fn_env_vars: HashMap<String, Ty>,
    /// Names of the enclosing function's INPUT parameters (subset of
    /// `fn_env_vars`). Input parameters of an anonymous function type are
    /// declared `Arc<dyn Fn(...) + 'static>` by [`fmt_param_ty`] — i.e. the
    /// binding's Rust value is already an `Arc`. Outputs / protected locals of
    /// the same MM type use [`fmt_ty`], which emits the bare `fn(...)` pointer.
    /// `emit_call_arg_with_formal` consults this set to forward such a
    /// parameter with `.clone()` rather than wrapping it in `Arc::new(...)`,
    /// which is what keeps recursive HOFs like `List.sort` working (each
    /// recursive call gets a clone of the same `Arc`, staying at one type level).
    fn_input_names: HashSet<String>,
    /// Names of function-scope locals (and inputs) whose Rust declaration is
    /// emitted with an initialiser. Used by matchcontinue arm codegen to
    /// decide whether the per-arm shadow `let mut x: T = x.clone();` is safe
    /// — if the outer binding has no initialiser and the matchcontinue itself
    /// is the first write to it, the shadow's RHS would be uninitialised and
    /// fail to compile. Populated in [`emit_function`] alongside `fn_env_vars`;
    /// saved/restored around nested-function emissions.
    fn_initialized_vars: HashSet<String>,
    /// Output component names (in declaration order) of the enclosing function.
    /// Used by `S::Return` inside a match-arm body to expand `return;` into
    /// `return Ok((outputs...));`. The arm body is emitted with a fresh
    /// `LocalEnv`, which would otherwise lose this information.
    fn_outputs: Vec<String>,
    /// Subset of `fn_outputs` whose declarations carry no default modification
    /// (`output T x;` with no `= expr`). These outputs are uninitialised at the
    /// function entry — if a `try` body assigns one and the matching `else`
    /// branch does not, MetaModelica semantics require the function to fail
    /// (matchfailure) on reaching the tail. The codegen emits an explicit
    /// `bail!` at the end of the `else` branch in that case (see the
    /// `S::Try` lowering). Outputs *with* a default modification are pre-
    /// initialised and need no such fallback.
    fn_outputs_no_default: Vec<String>,
    /// Variables currently known to hold a specific uniontype variant. Mirrors
    /// `LocalEnv::variants` but is accessible from `emit_exp` / `emit_var`,
    /// which don't receive the per-statement `LocalEnv`. Populated around the
    /// emission of a match arm's body+result; saved and restored at arm
    /// boundaries so nested match expressions don't leak into the enclosing
    /// scope. The keys are MetaModelica variable names (pre-escape); values
    /// are `(enum_qname, variant_simple_name)`.
    variants: HashMap<String, (String, String)>,
    /// Fully-qualified names of user-defined functions classified as
    /// *fallible* by [`crate::fallibility::analyze`]. Functions absent from
    /// this set are infallible: their Rust signature drops `Result<>` and
    /// their call sites omit `?` (or the surrounding [`QMode`] equivalent).
    fallible_functions: BTreeSet<String>,
    /// Per-function set of type parameter names that need a `+ PartialEq`
    /// bound. Populated by [`analyze_partial_eq`] and consulted by
    /// [`emit_function`] when formatting the signature's type parameters.
    /// Keyed by FQN (same convention as `fallible_functions`).
    partial_eq_required: BTreeMap<String, HashSet<String>>,
    /// Fallibility of the function currently being emitted. Set at the start
    /// of [`emit_function`] and consulted by the return-statement emitter and
    /// by the implicit-return tail in [`emit_function`] to decide whether to
    /// wrap output tuples in `Ok(...)`.
    current_fn_fallible: bool,
    /// Fully-qualified MetaModelica name (dot-separated) of the function
    /// currently being emitted. Used by the `getInstanceName()` builtin —
    /// MetaModelicaBuiltin.mo specifies this returns the enclosing function's
    /// qualified name, and the C bootstrap rewrites it to a literal at compile
    /// time. We do the same in codegen rather than at runtime so the constant
    /// is folded directly into the call site.
    current_fn_qname: String,
    /// Fully-qualified names of `partial function` declarations whose parent
    /// is itself a `function` (i.e. nested inside a function body rather than
    /// declared at module / package / uniontype scope). MetaModelica permits
    /// this — the alias is a function-local type — but Rust has no equivalent:
    /// a nested `type` declaration inside `fn` is not accessible to its own
    /// parameter list, and lifting the alias to module scope would collide
    /// across functions that all declare a same-named local alias (e.g.
    /// `Error.getCurrentComponent.prefixToStr` and
    /// `Error.updateCurrentComponent.prefixToStr`). For these qnames we
    /// fall back to inline `fn(...) -> Result<...>` emission in `fmt_ty`.
    nested_partial_aliases: BTreeSet<String>,
    /// Rust binding shape for variables in `variants`. Determines the deref
    /// form emitted around `var_field!` scrutinee:
    ///   `Owned` — plain `T` enum value (`var_field!(v.f, V::X)`).
    ///   `Arc`   — `Arc<T>` / similar smart pointer (`var_field!((*v).f, ..)`).
    ///   `RefArc` — `&Arc<T>` (a `ref`-bound pattern field whose declared type
    ///              is itself `Arc<T>`); requires `(**v).f`.
    /// Absent entries default to `Owned` for top-level function vars whose
    /// Arc-ness is read from `fn_env_vars` instead.
    variant_shapes: HashMap<String, VarShape>,
    /// `true` while emitting the body of a function that has been
    /// `#[tailcall::tailcall]`-annotated. The macro forbids both `?` and
    /// `bail!`/`return Err(...)` anywhere in the body — even in non-tail
    /// positions like preamble matches — because it transforms the whole
    /// body into a `Thunk<'_, _>`-returning state machine. We use this
    /// flag in [`emit_match`] (and similar) to rewrite the non-exhaustive
    /// match fallback from `bail!("match: no arm matched")` to
    /// `unreachable!("…")`, whose `!` type unifies with whatever the
    /// macro expects. This trades a clean `Err` for a panic on a truly
    /// non-exhaustive runtime case — acceptable because the analysis
    /// pass in [`plan_tail_call_lowering`] only fires for functions that
    /// the codegen has already verified to be `?`-free.
    in_tail_lowered_fn: bool,
    /// Active tail-call lowering context, set by `emit_tail_value_exp` while
    /// emitting the tail expression of a function selected for `#[tailcall]`.
    ///
    /// `(self_name, fallible)`:
    ///   * `self_name` — the bare name of the recursive function. Used by
    ///     `emit_match` to detect self-call results in case position so they
    ///     can be emitted bare (no `?`/no `Ok(...)` wrap), and by `emit_exp`'s
    ///     `Call` arm to suppress `?` on the self-call itself.
    ///   * `fallible` — when `true`, non-self tail-position leaves in match
    ///     arms must be wrapped in `Ok(...)` so every arm yields
    ///     `Result<T>` and the match's value can flow out as the function's
    ///     return.
    ///
    /// `None` outside the tail-expression of a tail-call-lowered function;
    /// the field is restored on exit (see `emit_tail_value_exp`).
    tail_lowering: Option<(String, bool)>,
    /// Extra match-arm guard expressions accumulated while emitting a
    /// pattern. Some patterns (currently only `TypedPat::Lit(Lit::Real)`)
    /// cannot be expressed directly as Rust patterns: f64 literal patterns
    /// were deprecated in edition 2024. We lower such a literal into a
    /// fresh binding *plus* a guard expression that compares the bound
    /// value to the literal, and stash the comparison here for the
    /// surrounding arm-emission code to combine with `case.guard`. The
    /// match-arm emitter is responsible for clearing this vector before
    /// emitting a pattern and draining it after. In non-arm contexts
    /// (e.g. function parameter patterns, `let` bindings) real-literal
    /// patterns are nonsensical anyway; the codegen still produces a
    /// fresh binding but the guards will simply accumulate until the
    /// next arm clears them — verifying that no such guards leak is
    /// outside the scope of this hook.
    pat_extra_guards: Vec<String>,
    /// Monotonic counter for synthesizing fresh names for real-literal
    /// pattern bindings (see `pat_extra_guards`).
    pat_lit_counter: u32,
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
    fn new(top_name: &str, current_crate: Option<String>, crate_map: BTreeMap<String, String>, top_level_uniontype_names: HashSet<String>, recursive_types: BTreeSet<String>, types_containing_mutable: BTreeSet<String>, types_containing_array: BTreeSet<String>, fn_type_vars: BTreeMap<String, Vec<String>>, fallible_functions: BTreeSet<String>, partial_eq_required: BTreeMap<String, HashSet<String>>) -> Self {
        Self {
            top_name: top_name.to_owned(),
            current_path: Vec::new(),
            unqual_modules: HashSet::new(),
            wildcard_members: BTreeMap::new(),
            self_members: HashSet::new(),
            named: BTreeMap::new(),
            self_aliases: BTreeSet::new(),
            uniontype_imports: HashSet::new(),
            implicit_modules: BTreeSet::new(),
            current_crate,
            crate_map,
            top_level_uniontype_names,
            recursive_types,
            types_containing_mutable,
            types_containing_array,
            const_fn_getters: BTreeSet::new(),
            no_mod_uniontypes: HashSet::new(),
            fn_type_vars,
            qmode: QMode::Function,
            uninit_arrays: HashSet::new(),
            fn_env_vars: HashMap::new(),
            fn_input_names: HashSet::new(),
            fn_initialized_vars: HashSet::new(),
            fn_outputs: Vec::new(),
            fn_outputs_no_default: Vec::new(),
            variants: HashMap::new(),
            variant_shapes: HashMap::new(),
            fallible_functions,
            partial_eq_required,
            current_fn_fallible: true,
            current_fn_qname: String::new(),
            nested_partial_aliases: BTreeSet::new(),
            in_tail_lowered_fn: false,
            tail_lowering: None,
            pat_extra_guards: Vec::new(),
            pat_lit_counter: 0,
        }
    }

    /// True when `qname` is a known *user-defined* function whose generated
    /// Rust signature returns `Result<T>` (i.e. is fallible per the
    /// fallibility analysis). Returns `false` for infallible user functions.
    ///
    /// Note: this is the membership test for [`Self::fallible_functions`].
    /// Builtins are NOT in that set (the analysis stores only user-defined
    /// functions); call-site decisions for builtins are handled separately
    /// because every builtin still returns `Result<T>` in its current Rust
    /// signature regardless of its semantic fallibility classification.
    fn is_fallible_user_fn(&self, qname: &str) -> bool {
        self.fallible_functions.contains(qname)
    }

    /// True when `qname` is a *known user-defined function* (present in the
    /// hierarchy as a function) that has been classified infallible.  This is
    /// strictly stronger than `!is_fallible_user_fn`: an unknown name (e.g.
    /// builtin, callback parameter, unresolved import) is *not* considered
    /// infallible by this predicate, which is the right answer for call-site
    /// decisions ("should I drop the `?`?") — we only drop `?` when we have
    /// positive evidence the callee returns a non-`Result` value.
    fn is_known_infallible_user_fn<'a>(&self, qname: &str, top_level: &BTreeMap<String, NameNode<'a>>) -> bool {
        if self.fallible_functions.contains(qname) {
            return false;
        }
        // Confirm `qname` actually denotes a user-defined function node
        // (R_FUNCTION). Anything else — record constructor, type alias,
        // builtin — has its own emission logic and must not be re-classified
        // here based on absence-from-the-fallible-set alone.
        let Some(node) = lookup_node(qname, top_level) else { return false };
        let NodeKind::Class(c) = &node.kind else { return false };
        if !matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
            return false;
        }
        // `partial function` declarations are *type aliases* for callback
        // signatures; they have no body and their referenced function values
        // are always fallible-shaped at the type level. The classification
        // doesn't apply to them as a *callee*.
        if c.partial_prefix {
            return false;
        }
        // `function Foo = Bar(...)` aliases are re-exports; their effective
        // fallibility comes from `Bar`. The fallibility analysis records the
        // alias under its own qname, so the membership check above is the
        // authoritative answer — but we still require the alias to denote a
        // real function (not an unresolved name). FunctionAlias node.ty would
        // be the cleaner signal; we don't have it cheaply here, so we accept
        // any non-partial R_FUNCTION as known.
        true
    }

    /// Produce the `#[derive(...)]` attribute appropriate for the type identified
    /// by `qname`. Types whose fields transitively embed `Mutable<T>` cannot
    /// derive `PartialEq` / `Eq` / `Hash`; they get only `Clone, Debug`.
    fn derives_for(&self, qname: &str) -> &'static str {
        // Interior-mutability container fields (`Mutable<T>`,
        // `Array<T>` = `Rc<RefCell<Vec<T>>>`) cannot derive
        // `Eq`/`Ord`/`Hash` — `RefCell` is excluded by design because
        // it breaks the immutability assumption those traits require.
        // `PartialEq` is still derivable: we hand-roll a `Mutable<T>: PartialEq`
        // that returns `false`, and `RefCell` itself derives `PartialEq` via
        // a `borrow()` comparison.
        // Otherwise emit the full algebraic set so `valueCompare`, hash
        // containers, and `Eq`-bounded generics work.
        // (`Real` is `OrderedFloat<f64>`, so containing a Real does not block
        // Ord/Eq/Hash.)
        if self.types_containing_mutable.contains(qname)
            || self.types_containing_array.contains(qname)
        {
            "#[derive(Clone, Debug, PartialEq)]"
        } else {
            "#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]"
        }
    }

    /// Wrap a fallible Rust expression with the appropriate error-propagation
    /// form for the current [`QMode`]. The argument should be a complete
    /// expression that evaluates to `Result<T, _>`; the returned string is
    /// an expression of type `T` in the surrounding context.
    fn q(&self, expr: &str) -> String {
        match &self.qmode {
            // `?` only works in a function body that returns `Result`. When the
            // surrounding function is infallible we still call into builtins
            // whose Rust signature returns `Result<T>` (the builtin layer has
            // not yet been switched away from `Result`); the fallibility
            // analysis only marks the enclosing function infallible when every
            // such call is known to never produce `Err`, so `.unwrap()` is
            // safe and surfaces as a panic if the analysis is ever wrong.
            QMode::Function if !self.current_fn_fallible => format!("{expr}.unwrap()"),
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

    /// Run `body` with `tail_lowering` temporarily replaced; restores on
    /// exit. Used to scope the tail-call lowering signal to the exact span
    /// of code being emitted as a tail expression.
    fn with_tail_lowering<R>(&mut self, t: Option<(String, bool)>, body: impl FnOnce(&mut Self) -> R) -> R {
        let saved = std::mem::replace(&mut self.tail_lowering, t);
        let r = body(self);
        self.tail_lowering = saved;
        r
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

        // Self-alias rewrite: `Type.ANY` inside NFType.mo (where `import Type = NFType;`
        // declares `Type` as a self-alias) → `NFType.ANY` so the same-module strip
        // below catches it. Without this the bare `Type` head survives shorten and
        // ends up emitted as `Type::ANY` — `Type` is not bound in Rust, so E0433.
        let dotted_owned;
        let dotted: &str = {
            let mut parts = dotted.splitn(2, '.');
            let head = parts.next().unwrap_or(dotted);
            let tail = parts.next();
            if self.self_aliases.contains(head) {
                dotted_owned = match tail {
                    Some(t) => format!("{}.{}", self.top_name, t),
                    None => self.top_name.clone(),
                };
                &dotted_owned
            } else {
                dotted
            }
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
        //
        // Skip the insertion when `top` is the same string as a local import alias
        // (e.g. `Binding` from `import Binding = NFBinding`): such names are already
        // bound to the aliased target by a `use … as Binding;` line, and pulling in
        // a same-named top-level package from another crate (e.g. `Script/Binding.mo`
        // in `openmodelica_backend`) would both collide with the alias and, worse,
        // create a cross-crate dependency cycle. The presence of a literal "Binding"
        // here means upstream code did not resolve the alias to its canonical qname;
        // log a diagnostic so the underlying resolution gap can be fixed.
        let top = dotted.split('.').next().unwrap_or(dotted);
        if top != self.top_name && self.crate_map.contains_key(top) {
            let shadowed_by_alias = self.named.values().any(|local| local == top);
            // If `top` is the simple name of a member exposed by a wildcard
            // import (e.g. `InnerOuter` brought in by `use NFPrefixes::*;`),
            // the wildcard takes precedence over the top-level package of the
            // same name. Skip the implicit-module insert so we don't emit a
            // conflicting `use openmodelica_frontend::InnerOuter;` line, and
            // emit the dotted path as-is (Rust resolves the head via the
            // already-emitted wildcard `use`).
            let shadowed_by_wildcard = self.wildcard_members.contains_key(top);
            let shadowed_by_self = self.self_members.contains(top);
            if !shadowed_by_alias && !shadowed_by_wildcard && !shadowed_by_self {
                self.implicit_modules.insert(top.to_owned());
            } else if std::env::var("MMTORUST_TRACE_ALIAS_SHADOW").is_ok() {
                eprintln!(
                    "[mmtorust] shorten: skipping implicit_modules insert of '{top}' \
                     (shadowed by local alias) for dotted='{dotted}' in {}::{}",
                    self.top_name,
                    self.current_path.join("::"),
                );
            }
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
                format!("metamodelica{}", dotted[top.len()..].replace('.', "::"))
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
/// Apply a single Import node to `ctx`. Shared between top-level
/// [`collect_imports`] and function-local imports (declared inside a
/// function body via `import Foo.Bar;` between the signature and `algorithm`).
fn apply_import<'a>(m: &MM::ImportMember, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
    let same_file_prefix = format!("{}.", ctx.top_name);
    match &m.import {
        Absyn::Import::UNQUAL_IMPORT { path } => {
            let dotted = path_to_dotted(path);
            if dotted != ctx.top_name && !dotted.starts_with(&same_file_prefix) && path_exists_in_hierarchy(&dotted, top_level) {
                if is_const_component(&dotted, top_level) {
                    let last = dotted.rsplit('.').next().unwrap_or(&dotted).to_owned();
                    ctx.named.insert(dotted, last);
                } else {
                    // Record each child member name so `shorten` can recognise
                    // bare references to a wildcard-imported type/function and
                    // avoid pulling in a same-named top-level package via
                    // `implicit_modules` (which would generate a conflicting
                    // `use` line and shadow the wildcard import).
                    if let Some(node) = lookup_node(&dotted, top_level) {
                        for child_name in node.children.keys() {
                            ctx.wildcard_members.insert(child_name.clone(), dotted.clone());
                        }
                    }
                    ctx.unqual_modules.insert(dotted);
                }
            }
        }
        Absyn::Import::QUAL_IMPORT { path } => {
            let dotted = path_to_dotted(path);
            if dotted != ctx.top_name && !dotted.starts_with(&same_file_prefix) && path_exists_in_hierarchy(&dotted, top_level) {
                let last = dotted.rsplit('.').next().unwrap_or(&dotted).to_owned();
                let effective = resolve_through_import_node(&dotted, top_level)
                    .unwrap_or(dotted.clone());
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
            if dotted == ctx.top_name {
                ctx.self_aliases.insert(name.to_string());
            } else if !dotted.starts_with(&same_file_prefix) && path_exists_in_hierarchy(&dotted, top_level) {
                let effective = resolve_through_import_node(&dotted, top_level)
                    .unwrap_or(dotted.clone());
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
                        let effective = match lookup_node_ty(&full, top_level) {
                            Some(Ty::RustStruct(struct_qname)) if struct_qname != &full => struct_qname.clone(),
                            _ => full,
                        };
                        ctx.named.insert(effective, local);
                    }
                }
            }
        }
    }
}

/// Saved import-scope state for function-local imports. Returned by
/// [`apply_local_imports`] and consumed by [`restore_imports`].
struct ImportScope {
    named: BTreeMap<String, String>,
    unqual_modules: HashSet<String>,
    wildcard_members: BTreeMap<String, String>,
    self_aliases: BTreeSet<String>,
}

/// Apply Import children of `node` to `ctx`, returning a snapshot that
/// [`restore_imports`] can use to undo the changes. Used to layer
/// function-local imports on top of the file-level scope for the duration
/// of one function body.
fn apply_local_imports<'a>(node: &NameNode<'_>, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> ImportScope {
    let saved = ImportScope {
        named: ctx.named.clone(),
        unqual_modules: ctx.unqual_modules.clone(),
        wildcard_members: ctx.wildcard_members.clone(),
        self_aliases: ctx.self_aliases.clone(),
    };
    for child in node.children.values() {
        if let NodeKind::Import(m) = &child.kind {
            apply_import(m, ctx, top_level);
        }
    }
    saved
}

fn restore_imports(ctx: &mut GenCtx, saved: &ImportScope) {
    ctx.named = saved.named.clone();
    ctx.unqual_modules = saved.unqual_modules.clone();
    ctx.wildcard_members = saved.wildcard_members.clone();
    ctx.self_aliases = saved.self_aliases.clone();
}

fn collect_imports<'a>(node: &NameNode<'_>, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
    for child in node.children.values() {
        match &child.kind {
            NodeKind::Import(m) => { apply_import(m, ctx, top_level); continue; }
            NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) => {
                continue;
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
                // `File` is an ExternalObject whose runtime semantics (file
                // handle + reference count) are hand-written in
                // `openmodelica_util/src/File.rs`. Skip codegen so the manual
                // implementation isn't overwritten.
                //
                // `Global` contains index constants and an `initialize()`
                // function; it is hand-written in
                // `openmodelica_util/src/Global.rs` so that `initialize()` can
                // reference typed `Globals::` variables correctly.
                // `ErrorExt` declares only `external "C"` functions that
                // implement OMC's per-thread error/warning message buffer
                // (mirrors `OMCompiler/Compiler/runtime/errorext.cpp`).
                // The generated `todo!()` stubs panic the moment any code
                // path queries error counts (e.g. `Tpl::tplString*`), so we
                // hand-write the runtime in `openmodelica_util/src/ErrorExt.rs`
                // and skip codegen for it here.
                // `Print` mirrors the per-thread print/error string buffer
                // from `OMCompiler/Compiler/runtime/printimpl.c` — every
                // function is `external "C"` so the auto-generated body is
                // a useless `todo!()`. Hand-write in
                // `openmodelica_util/src/Print.rs`.
                //
                // `System` is the umbrella `external "C"` wrapper around
                // `OMCompiler/Compiler/runtime/systemimpl.c` (strings, file
                // ops, env vars, timers, dlopen, regex, ...). Same story as
                // `Print` — codegen produces `todo!()` for every body.
                // Hand-write in `openmodelica_util/src/System.rs`.
                "Mutable" | "GCExt" | "Pointer" | "File" | "Global" | "Vector" | "ErrorExt" | "Print" | "ParserExt" | "System" => continue,
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
        let content = generate_file(name, node, &crate_map, current_crate, &top_level_uniontype_names, hier.recursive_types.clone(), hier.types_containing_mutable.clone(), hier.types_containing_array.clone(), &no_mod_uniontypes, &hier.top_level, &fn_type_vars, &hier.fallible_functions, &hier.partial_eq_required);
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
    // Match patterns that cross an `Arc<…>` boundary are lowered through the
    // `match_deref` crate's `match_deref!{ match X { Deref @ Pat => … } }`
    // syntax — stable Rust replacement for the nightly-only `deref_patterns`
    // feature. The macro is invoked through its full path
    // (`::match_deref::match_deref!`) inside generated code, so no `use`
    // re-export is needed at the crate root.
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
    // Include the manually-maintained Globals module when the file exists.
    // Globals.rs is not auto-generated because its declarations require
    // hand-written type annotations that cannot be inferred from MetaModelica
    // alone (in particular, the types stored at each global-root slot).
    let globals_path = format!("{this_dir}/Globals.rs");
    if std::path::Path::new(&globals_path).exists() {
        writeln!(out, "pub mod Globals;").unwrap();
    }
    // Include the hand-written `parser` submodule when present. Like Globals.rs,
    // this lives next to the generated files but is not auto-generated.
    let parser_path = format!("{this_dir}/parser/mod.rs");
    if std::path::Path::new(&parser_path).exists() {
        writeln!(out, "pub mod parser;").unwrap();
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

/// Walk the hierarchy collecting fully-qualified names of `partial function`
/// declarations whose parent class is itself a function. See
/// [`GenCtx::nested_partial_aliases`] for why these get inlined rather than
/// referenced by name. `parent_is_fn` carries down whether the *current*
/// scope is inside a function class, so children declared inside it get
/// recorded.
fn collect_nested_partial_aliases(
    nodes: &BTreeMap<String, NameNode<'_>>,
    prefix: &str,
    parent_is_fn: bool,
    out: &mut BTreeSet<String>,
) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        let mut is_fn = false;
        let mut is_partial = false;
        if let NodeKind::Class(c) = &node.kind {
            // `partial_prefix` marks `partial function` aliases (see
            // emit_function's early-return for these). Function-restricted
            // classes — both regular and partial — propagate `parent_is_fn`
            // into their children.
            if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
                is_fn = true;
            }
            if c.partial_prefix {
                is_partial = true;
            }
        }
        if is_partial && parent_is_fn {
            out.insert(qname.clone());
        }
        collect_nested_partial_aliases(&node.children, &qname, is_fn, out);
    }
}

/// Pre-pass mirror of the `pub static` vs. `pub const fn` decision in [`emit_node`]:
/// walk every component in the hierarchy and record the FQNs of those that will
/// be emitted as `pub const fn` getters. These are constant components whose
/// value expression is const-emittable but whose type is *not* `Sync` (it
/// transitively contains `metamodelica::Array<T>`). We run this before any
/// expression emission so that `emit_exp`'s `Var` arm can decide, at every
/// reference site, whether to append `()` to dereference the getter.
///
/// The conditions must stay in sync with [`emit_node`]; if a new emission path
/// for `constant` components is added there, mirror it here.
fn collect_const_fn_getters<'a>(
    nodes: &BTreeMap<String, NameNode<'a>>,
    prefix: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    ctx: &GenCtx,
    out: &mut BTreeSet<String>,
) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Component(m) = &node.kind
            && m.variability == Absyn::Variability::CONST
            && (node.override_default_exp.is_some() || extract_default_exp(&m.modification).is_some())
        {
            // The primitive/string emission paths in `emit_node` go through
            // `pub const` (not `pub static`) regardless of Sync-ness, so they
            // never need a getter. Skip them.
            let primitive = matches!(node.ty, Ty::Str | Ty::I32 | Ty::F64 | Ty::Bool);
            let is_array_const = matches!(node.ty, Ty::Array(_));
            // An `Array<T>` constant becomes a `pub static StaticArray<T>` only
            // when T itself is Sync. When the element type transitively contains
            // another `Array<U>`, even the `StaticArray<T>` wrapper is non-Sync,
            // and `emit_node` falls back to a `thread_local! + pub fn` getter
            // pair (see the matching block there). Those names must also be
            // registered here so call sites get the trailing `()`.
            let array_inner_non_sync = if let Ty::Array(inner) = &node.ty {
                !ty_is_sync(inner, ctx)
            } else {
                false
            };
            // Three emission paths in `emit_node` produce a getter (not a value):
            //   1. Non-Sync, const-emittable initializer  → `pub const fn`
            //   2. Non-Sync, non-const-emittable, non-Array → `pub fn { todo!(...) }`
            //   3. `Array<T>` where T is non-Sync → `thread_local!` + getter
            // The plain `Ty::Array(_)` with Sync element type takes the
            // `StaticArray<T>` rewrite (a `pub static`, Sync via the wrapper)
            // so it does NOT become a getter.
            if !primitive && ((!is_array_const && !ty_is_sync(&node.ty, ctx)) || array_inner_non_sync) {
                out.insert(qname.clone());
            }
        }
        collect_const_fn_getters(&node.children, &qname, top_level, ctx, out);
    }
}

fn collect_no_mod_uniontypes(nodes: &BTreeMap<String, NameNode<'_>>, prefix: &str, out: &mut HashSet<String>) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Class(c) = &node.kind
            && matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE)
                && !prefix.is_empty()
                && !uniontype_needs_mod(node)
            {
                out.insert(qname.clone());
            }
        collect_no_mod_uniontypes(&node.children, &qname, out);
    }
}

fn generate_file<'a>(top_name: &str, node: &NameNode<'_>, crate_map: &BTreeMap<String, String>, current_crate: Option<String>, top_level_uniontype_names: &HashSet<String>, recursive_types: BTreeSet<String>, types_containing_mutable: BTreeSet<String>, types_containing_array: BTreeSet<String>, no_mod_uniontypes: &HashSet<String>, top_level: &'a BTreeMap<String, NameNode<'a>>, fn_type_vars: &BTreeMap<String, Vec<String>>, fallible_functions: &BTreeSet<String>, partial_eq_required: &BTreeMap<String, HashSet<String>>) -> String {
    let mut ctx = GenCtx::new(top_name, current_crate, crate_map.clone(), top_level_uniontype_names.clone(), recursive_types, types_containing_mutable, types_containing_array, fn_type_vars.clone(), fallible_functions.clone(), partial_eq_required.clone());
    ctx.no_mod_uniontypes = no_mod_uniontypes.clone();
    // Pre-walk the *whole* hierarchy (not just this file's node) so that
    // function-nested partial-function aliases are recognised regardless of
    // whether they live in the file currently being emitted or in a sibling.
    collect_nested_partial_aliases(top_level, "", false, &mut ctx.nested_partial_aliases);
    // Identify constants emitted as `pub const fn` getters (see fn doc) so that
    // `emit_exp`'s `Var` arm can append `()` at reference sites. Run after the
    // module-context-independent fields of `ctx` are populated; this pre-pass
    // only reads `recursive_types` and `types_containing_array`, both of which
    // are set in `GenCtx::new`.
    let mut getters = BTreeSet::new();
    collect_const_fn_getters(top_level, "", top_level, &ctx, &mut getters);
    ctx.const_fn_getters = getters;
    collect_imports(node, &mut ctx, top_level);
    // Same-file siblings: record names of the top node's direct children so
    // bare references to them are not mistaken for top-level packages with the
    // same name (e.g. `NFPrefixes.InnerOuter` referenced as plain `InnerOuter`
    // from inside `NFPrefixes.mo`, while a top-level `InnerOuter.mo` exists in
    // a different crate).
    for child_name in node.children.keys() {
        ctx.self_members.insert(child_name.clone());
    }

    // First pass: emit the body so that shorten() can populate implicit_modules.
    let mut body = String::new();
    emit_node(&mut body, top_name, node, "", &mut ctx, top_level);

    // Second pass: emit header + complete use lines (now including implicit modules).
    let mut out = String::new();
    writeln!(out, "// Auto-generated from MetaModelica source").unwrap();
    // The top-level class's `commentsBeforeClass` is typically the license
    // header that the .mo source carries before its `package …` keyword.
    // Emit it right after the auto-generated marker so it shows up at the
    // top of the file, ahead of `#![allow]` and `use` lines, instead of
    // being buried after the imports inside the body.
    if let NodeKind::Class(c) = &node.kind {
        for txt in &c.comments_before_class {
            emit_lexer_comment(&mut out, "", txt);
        }
    }
    writeln!(out, "#![allow(warnings)]").unwrap();
    writeln!(out, "#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]").unwrap();
    writeln!(out, "
use std::sync::Arc;
use anyhow::{{Result, bail}};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{{ArcStr, literal, format}};
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
        if m.variability == Absyn::Variability::CONST
            // `override_default_exp` wins over the AST default. It carries the
            // RHS of a parent package's `extends Base(field=expr)` modification
            // and was attached by `flatten_sibling_package_extends`. Without
            // this, the derived package's constant would silently emit the
            // base's value (e.g. `WithGenericSubscript`) instead of the
            // overridden one (`WithoutSubscripts`, etc.).
            && let Some(exp) = node.override_default_exp.or_else(|| extract_default_exp(&m.modification)) {
                let pkg_prefix = if ctx.current_path.is_empty() {
                    ctx.top_name.to_owned()
                } else {
                    format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
                };
                let typed = typedexp::infer_exp(exp, &HashMap::new(), top_level, &pkg_prefix, &[]);
                let ename = escape_ident(name);

                // String `pub const`s are emitted as `&'static str`, not
                // `ArcStr`, so that:
                //   1. Their *names* survive `const_str::equal!` calls in
                //      other const initializers (an `ArcStr` cref can't be
                //      passed to `const_str::equal!` because the macro
                //      requires const `&str` arguments, and `ArcStr::Deref`
                //      isn't a `const` trait).
                //   2. Runtime use sites recover an `ArcStr` cheaply via
                //      `arcstr::literal!(<cref>)`, which constructs a static
                //      `ArcStr` from any const `&'static str` expression with
                //      zero allocation (see the macro definition for
                //      details). That wrap is added by [`emit_exp`]'s `Var`
                //      arm whenever the cref resolves to a string `pub const`
                //      (see [`is_const_str_cref`]).
                let rust_ty = match &node.ty {
                    Ty::I32 => Some("i32"),
                    Ty::F64 => Some("metamodelica::Real"),
                    Ty::Bool => Some("bool"),
                    _ => None,
                };
                if matches!(&node.ty, Ty::Str) {
                    let val = emit_const_str_operand(&typed, ctx, top_level).unwrap_or_else(|| {
                        // Same surface as the `==`/`!=` fallback: surface the
                        // unhandled shape with a Rust `compile_error!` rather
                        // than producing wrong code. Extend
                        // `emit_const_str_operand` when new shapes appear.
                        let dump = format!("{typed:?}");
                        format!("{{ compile_error!(\"const string init not reducible to &'static str: {dump}\"); \"\" }}")
                    });
                    writeln!(out, "{indent}pub const {ename}: &'static str = {val};").unwrap();
                    writeln!(out).unwrap();
                } else if let Some(r_ty) = rust_ty {
                    let val = emit_exp(&typed, /*is_const=*/true, ctx, top_level);
                    writeln!(out, "{indent}pub const {ename}: {r_ty} = {val};").unwrap();
                    writeln!(out).unwrap();
                } else if is_static_const_emittable(&typed, ctx, top_level)
                    && !is_arc_wrapped(&node.ty, ctx)
                {
                    // If the destination type is Arc-wrapped (recursive uniontype
                    // stored as `Arc<Enum>` everywhere), the unit-variant expression
                    // `Enum::VARIANT` needs an `Arc::new(...)` wrap to match — but
                    // `Arc::new` is not a const fn, so we cannot emit a `pub static`
                    // / `pub const`. Fall through to the `LazyLock` path below,
                    // where `emit_exp` (with `is_const=false`) wraps unit variants
                    // in `Arc::new` automatically.
                    // Record/struct of pure literals and other const-emittable values.
                    //
                    // Two sub-cases depending on whether the type is `Sync`:
                    //
                    //   * Sync — emit as a plain `pub static`. No `LazyLock`
                    //     indirection needed; the value is built at compile time
                    //     and shared across threads.
                    //
                    //   * Non-Sync (the type transitively embeds an
                    //     `Array<T> = Rc<RefCell<Vec<T>>>`) — `pub static`
                    //     requires `T: Sync` and would fail to compile, even
                    //     though the *value* we are storing (e.g. a unit-variant
                    //     constructor like `InstStore::NOSTORE`) doesn't actually
                    //     use the non-Sync variant. Emit a `pub const fn` getter
                    //     instead: the body is a `const` expression so the call
                    //     is free, and the getter sidesteps the `Sync` bound
                    //     because there is no shared static storage. References
                    //     to such symbols are rewritten by `emit_exp`'s `Var`
                    //     arm to append `()` (the FQN is registered in
                    //     `ctx.const_fn_getters` by `collect_const_fn_getters`).
                    //
                    // `LazyLock` is the wrong tool for the non-Sync case too:
                    // `LazyLock<T>` also requires `T: Sync` to be a `pub static`,
                    // and the value is const-buildable so caching is pointless.
                    let r_ty = fmt_ty(&node.ty, ctx);
                    let val = emit_exp(&typed, /*is_const=*/true, ctx, top_level);
                    if ty_is_sync(&node.ty, ctx) {
                        writeln!(out, "{indent}pub static {ename}: {r_ty} = {val};").unwrap();
                    } else {
                        writeln!(out, "{indent}pub const fn {ename}() -> {r_ty} {{ {val} }}").unwrap();
                    }
                    writeln!(out).unwrap();
                } else {
                    // Top-level `pub static` of type `Array<T>` is special-cased
                    // to `StaticArray<T>`. `Array<T> = Rc<RefCell<Vec<T>>>` is
                    // not `Sync`, so `LazyLock<Array<T>>` would fail to compile
                    // as a `pub static`. Constant `array<T>` declarations (lexer
                    // tables, NF builtin caches, ...) are never written to, so
                    // we emit them as the `Sync`-safe `StaticArray<T>` wrapper
                    // (an `Arc<Vec<T>>` under the hood) which exposes the same
                    // `.borrow()` / `[idx]` / `.clone()` surface generated code
                    // expects from an `Array<T>`. See `metamodelica::StaticArray`
                    // for the full rationale.
                    let is_array_const = matches!(&node.ty, Ty::Array(_));
                    // For a constant of type `Array<T>` (lowered to `StaticArray<T>`),
                    // the LazyLock wrapping requires the *element* type to be `Sync`.
                    // When T transitively embeds another `Array<U>` (non-Sync), the
                    // outer `StaticArray<T>` is still non-Sync and cannot be a
                    // `pub static`. NFBuiltin's `EMPTY_NODE_CACHE: Array<CachedData>`
                    // is the canonical case — CachedData has `Array<...>` fields.
                    // Emit a `thread_local!` + getter pair instead: per-thread
                    // storage sidesteps the `Sync` bound, and `StaticArray::share`
                    // is a cheap Arc bump so callers paying the per-call cost is
                    // acceptable for these tables.
                    let array_inner_non_sync = if let Ty::Array(inner) = &node.ty {
                        !ty_is_sync(inner, ctx)
                    } else {
                        false
                    };
                    if is_array_const && array_inner_non_sync {
                        let inner_ty_s = match &node.ty { Ty::Array(t) => fmt_ty(t, ctx), _ => unreachable!() };
                        let r_ty = format!("metamodelica::StaticArray<{inner_ty_s}>");
                        let saved_fallible = ctx.current_fn_fallible;
                        ctx.current_fn_fallible = false;
                        let mut val = emit_exp(&typed, /*is_const=*/false, ctx, top_level);
                        ctx.current_fn_fallible = saved_fallible;
                        val = rewrite_array_init_for_static(&val);
                        // `thread_local!` defines a `LocalKey<T>` keyed by the
                        // const's name. The public getter clones via `share()`
                        // (Arc refcount bump) and returns a fresh `StaticArray`
                        // handle so call sites match the `pub static` API shape.
                        // The getter is registered in `ctx.const_fn_getters` so
                        // `emit_var` appends `()` at every reference.
                        writeln!(out, "{indent}thread_local! {{ static __{ename}_TLS: {r_ty} = {val}; }}").unwrap();
                        writeln!(out, "{indent}pub fn {ename}() -> {r_ty} {{ __{ename}_TLS.with(|__t| __t.share()) }}").unwrap();
                        writeln!(out).unwrap();
                        return;
                    }
                    // Non-Sync, non-const-emittable, and not a top-level `Array<T>`:
                    // there is no good emission path today. The `LazyLock<T>` route
                    // below requires `T: Sync`; the `StaticArray<T>` rewrite only
                    // applies to a direct `Ty::Array(_)`; a `pub const fn` getter
                    // would require the initializer to be a const expression, which
                    // by definition it isn't here. Surface the case with a
                    // `compile_error!` so it's visible at the use site rather than
                    // silently producing a `pub static` that won't compile. When this
                    // ever fires in practice, extend the emission (e.g. by collecting
                    // initializer-side allocations into a `StaticArray`-equivalent
                    // wrapper, or by lowering to a `thread_local!` getter if MM
                    // semantics permit).
                    if !is_array_const && !ty_is_sync(&node.ty, ctx) {
                        let r_ty = fmt_ty(&node.ty, ctx);
                        let dump = format!("{typed:?}").replace('"', "'");
                        writeln!(out, "{indent}// TODO: non-Sync, non-const-emittable constant — needs new emission path.").unwrap();
                        writeln!(out, "{indent}// Type: {r_ty}").unwrap();
                        writeln!(out, "{indent}// Expr: {dump}").unwrap();
                        writeln!(out, "{indent}pub fn {ename}() -> {r_ty} {{ todo!(\"non-Sync, non-const-emittable constant {ename} — extend codegen\") }}").unwrap();
                        writeln!(out).unwrap();
                        return;
                    }
                    let r_ty = if is_array_const {
                        let inner = match &node.ty { Ty::Array(t) => t.as_ref(), _ => unreachable!() };
                        format!("metamodelica::StaticArray<{}>", fmt_ty(inner, ctx))
                    } else {
                        fmt_ty(&node.ty, ctx)
                    };
                    // The closure body returns `T`, not `Result<T>`, so emit
                    // the initializer in an infallible context: the `q()`
                    // helper turns `?` into `.unwrap()` for calls whose Rust
                    // signature still returns `Result` (per the fallibility
                    // analysis these are the calls that won't actually fail
                    // at runtime — if they did, the program is broken anyway).
                    let saved_fallible = ctx.current_fn_fallible;
                    ctx.current_fn_fallible = false;
                    let mut val = emit_exp(&typed, /*is_const=*/false, ctx, top_level);
                    ctx.current_fn_fallible = saved_fallible;
                    if is_array_const {
                        // Rewrite the array-constructor calls in the initializer
                        // so the resulting value is a `StaticArray<T>` rather
                        // than an `Array<T>`. The two constructor shapes we see
                        // here are:
                        //
                        //   1. `metamodelica::Dangerous::listArray(<L>).unwrap()`
                        //      — emitted from `MetaModelica.Dangerous.listArrayLiteral({...})`.
                        //   2. `metamodelica::arrayFromVec(<V>)`
                        //      — emitted from `MetaModelica.arrayCreate`-style inits
                        //        whose RHS is already a `Vec<T>` collected at the
                        //        call site (see e.g. NFBuiltin EMPTY_NODE_CACHE).
                        //
                        // Both are pure local-shape rewrites: the inner argument
                        // is unchanged, only the outer constructor differs.
                        // Anything else falls through as a TODO comment so the
                        // missing case is visible at the use site rather than
                        // silently producing the wrong type.
                        val = rewrite_array_init_for_static(&val);
                    }
                    writeln!(out, "{indent}pub static {ename}: std::sync::LazyLock<{r_ty}> = std::sync::LazyLock::new(|| {{ {val} }});").unwrap();
                    writeln!(out).unwrap();
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
            // commentsBeforeClass / commentsAfterEnd for *nested* packages are
            // emitted by the parent's `emit_package_body` loop because the
            // parent knows where to place them relative to siblings. For the
            // *top-level* class, `commentsBeforeClass` is emitted by
            // `generate_file` ahead of the `use` lines so the license/header
            // appears at the top of the generated file; only
            // `commentsAfterEnd` is emitted here.
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
                emit_package_body(out, node, c, &child_indent, &mut *ctx, top_level);
            }
            if wrap {
                ctx.current_path.pop();
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
            } else {
                for txt in &c.comments_after_end {
                    emit_lexer_comment(out, indent, txt);
                }
            }
        }
        R_UNIONTYPE => emit_uniontype(out, name, node, c, indent, &mut *ctx, top_level),
        R_TYPE | R_ENUMERATION => emit_type_item(out, name, node, c, indent, &mut *ctx),
        R_RECORD | R_METARECORD { .. } => emit_struct(out, name, node, c, indent, &mut *ctx, top_level),
        R_FUNCTION { .. } => emit_function(out, name, node, c, indent, &mut *ctx, top_level),
        R_CLASS if crate::hierarchy::is_external_object_class(node) => {
            emit_external_object(out, name, node, c, indent, &mut *ctx, top_level)
        }
        _ => {}
    }
}

/// Emit a single captured lexer comment (line or block) with the surrounding
/// indent and trailing newline. Comment text from
/// `MM::ClassMember::LexerComment` and the `comments_*` lists already includes
/// the `//` / `/* */` delimiters.
///
/// We must, however, neutralise comment forms that Rust reserves for
/// documentation, otherwise the generated file fails to parse:
///   * `///` and `//!` are *doc comments*; they must immediately precede an
///     item, so a stray one in the middle of a file is a hard error.
///   * `/** … */` and `/*! … */` are outer/inner block doc comments with the
///     same constraint.
/// MetaModelica freely uses `//!!! …`, `/***** …` separator banners and the
/// like, so we rewrite the leading two-or-three character delimiter to a
/// plain `// `/`/* ` form. The rest of the comment text is preserved
/// verbatim so the reader still sees the original wording.
fn emit_lexer_comment(out: &mut String, indent: &str, text: &str) {
    let normalised = neutralise_doc_comment(text);
    for line in normalised.lines() {
        writeln!(out, "{indent}{line}").unwrap();
    }
}

fn neutralise_doc_comment(text: &str) -> String {
    // Line comments: only the *first* `//` of the comment can be misread as
    // `///` / `//!` (subsequent lines of a line comment do not exist in our
    // capture model — each `// …` is its own comment).
    if let Some(rest) = text.strip_prefix("///") {
        return format!("// /{rest}"); // turn `///foo` into `// /foo`
    }
    if let Some(rest) = text.strip_prefix("//!") {
        return format!("// !{rest}"); // turn `//!foo` into `// !foo`
    }
    if let Some(rest) = text.strip_prefix("/**") {
        // Distinguish the legitimate `/**/` empty comment (rare but seen).
        if rest == "/" { return text.to_owned(); }
        return format!("/* *{rest}");
    }
    if let Some(rest) = text.strip_prefix("/*!") {
        return format!("/* !{rest}");
    }
    text.to_owned()
}

/// Walk a package class body in *source order*, dispatching named children to
/// `emit_node` and emitting captured lexer comments as Rust comments at their
/// original position.
///
/// Children added by `flatten_extends` (members copied in from a base class)
/// are not present in the local `members` list — they are emitted after the
/// in-source members, in alphabetical order for stable output.
fn emit_package_body<'a>(
    out: &mut String,
    node: &NameNode<'_>,
    c: &MM::Class,
    indent: &str,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) {
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => &[],
    };
    let mut emitted: HashSet<&str> = HashSet::new();
    for m in members {
        match m {
            MM::ClassMember::LexerComment(text) => {
                emit_lexer_comment(out, indent, text);
            }
            MM::ClassMember::ClassDef(cdm) => {
                let cn = cdm.class_def.name.as_str();
                // MetaModelica allows the same class name to appear twice in
                // a package via `redeclare package X` (e.g. LexerJSON
                // declares `replaceable package LexTable` once at the top
                // and redeclares it later with the actual table). The
                // hierarchy collapses these into a single entry, so we must
                // only emit the dispatch once — preferring the *last*
                // occurrence's surrounding comments, mirroring how the
                // hierarchy resolves the override.
                if emitted.contains(cn) { continue; }
                if let Some(child) = node.children.get(cn) {
                    // commentsBeforeClass / commentsAfterEnd belong to this
                    // specific child; emit them around the dispatch so they
                    // appear lexically next to the class they document.
                    for txt in &cdm.class_def.comments_before_class {
                        emit_lexer_comment(out, indent, txt);
                    }
                    emit_node(out, cn, child, indent, &mut *ctx, top_level);
                    for txt in &cdm.class_def.comments_after_end {
                        emit_lexer_comment(out, indent, txt);
                    }
                    emitted.insert(cn);
                }
            }
            MM::ClassMember::Component(comp) => {
                let cn = comp.name.as_str();
                if emitted.contains(cn) { continue; }
                if let Some(child) = node.children.get(cn) {
                    emit_node(out, cn, child, indent, &mut *ctx, top_level);
                    emitted.insert(cn);
                }
            }
            // Extends / Import members do not produce a hierarchy child
            // visible at this iteration site — extends contents are pulled
            // in by `flatten_extends`, and imports are resolved at use sites.
            MM::ClassMember::Extends(_) | MM::ClassMember::Import(_) => {}
        }
    }
    // Any children present in the hierarchy but not in the source members
    // (typically pulled in by `flatten_extends` from a base package) are
    // emitted after the source-ordered ones, alphabetically for determinism.
    let mut leftover: Vec<(&String, &NameNode<'_>)> = node.children.iter()
        .filter(|(n, _)| !emitted.contains(n.as_str()))
        .collect();
    leftover.sort_by_key(|(n, _)| n.as_str());
    for (cn, child) in leftover {
        emit_node(out, cn, child, indent, &mut *ctx, top_level);
    }
}

/// Emit a Rust representation for a MetaModelica ExternalObject class.
///
/// External objects are opaque, nominal types whose lifecycle is governed by a
/// `constructor` and a `destructor` function declared inside the class body.
/// The actual implementation lives in C runtime code (the `external "C" ...`
/// annotations point at functions like `om_file_new`). The codegen cannot
/// know how to bridge those calls in general — so we emit a placeholder Rust
/// surface that compiles, names the type correctly, and clearly marks where
/// runtime glue is missing.
///
/// What we emit, given `class Foo extends ExternalObject; function constructor ...`:
///
/// ```text
/// pub struct Foo {
///     // Opaque handle; the real representation is provided by the runtime.
/// }
/// impl Foo {
///     pub fn new(/* constructor inputs */) -> Result<Foo> {
///         todo!("external object Foo: constructor not implemented")
///     }
/// }
/// // For callers that spell `Foo.Foo(...)` (the MM convention for invoking the
/// // constructor by class name), we also expose a free function with the
/// // class name as a thin shim that forwards to `Foo::new`.
/// pub fn Foo(/* constructor inputs */) -> Result<Foo> { Foo::new(...) }
/// // Destructor is emitted as a no-op stub; the runtime is responsible for
/// // releasing resources when the Rust value is dropped.
/// pub fn destructor(/* destructor inputs */) {}
/// ```
///
/// Classes whose runtime is hand-implemented (e.g. `File`) are skipped at the
/// dispatch layer so this generic emission does not overwrite them.
fn emit_external_object<'a>(
    out: &mut String,
    name: &str,
    node: &NameNode<'_>,
    c: &MM::Class,
    indent: &str,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) {
    let ename = escape_ident(name);
    writeln!(out, "{indent}/// Opaque external object `{name}`. The runtime owns the").unwrap();
    writeln!(out, "{indent}/// representation; this struct exists only to give the type a").unwrap();
    writeln!(out, "{indent}/// nominal identity in Rust so call sites type-check.").unwrap();
    writeln!(out, "{indent}#[derive(Clone, Debug)]").unwrap();
    writeln!(out, "{indent}pub struct {ename} {{").unwrap();
    writeln!(out, "{indent}    _opaque: std::sync::Arc<std::sync::Mutex<()>>,").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    // Locate the constructor and destructor child functions.
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => &[],
    };
    let mut ctor: Option<(&MM::ClassMember, &str)> = None;
    let mut dtor: Option<(&MM::ClassMember, &str)> = None;
    for m in members {
        if let MM::ClassMember::ClassDef(cdm) = m {
            match cdm.class_def.name.as_str() {
                "constructor" => ctor = Some((m, "constructor")),
                "destructor" => dtor = Some((m, "destructor")),
                _ => {}
            }
        }
    }

    // Helper: gather (name, ty) tuples for the inputs/outputs of a child function
    // by reading its component members. We use the hierarchy node's children so
    // the types are resolved (rather than raw Absyn paths).
    let get_io = |child_name: &str| -> Option<(Vec<(String, Ty)>, Vec<(String, Ty)>)> {
        let child = node.children.get(child_name)?;
        let NodeKind::Class(cc) = &child.kind else { return None };
        let cm: &[MM::ClassMember] = match &cc.body {
            MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
            _ => &[],
        };
        let mut ins = Vec::new();
        let mut outs = Vec::new();
        for m in cm {
            if let MM::ClassMember::Component(comp) = m {
                let ty = child.children.get(&comp.name).map(|n| n.ty.clone()).unwrap_or(Ty::Unknown);
                match comp.direction {
                    Absyn::Direction::INPUT => ins.push((comp.name.clone(), ty)),
                    Absyn::Direction::OUTPUT => outs.push((comp.name.clone(), ty)),
                    Absyn::Direction::BIDIR | Absyn::Direction::INPUT_OUTPUT => {}
                }
            }
        }
        Some((ins, outs))
    };

    // Constructor → `impl Foo { pub fn new(...) -> Result<Foo> { todo!(...) } }`
    // plus a free-function shim with the class name so `Foo.Foo(...)` call sites
    // resolve. Destructor → no-op stub.
    if let Some((_, cname)) = ctor {
        let (ins, _outs) = get_io(cname).unwrap_or((vec![], vec![]));
        let params: Vec<String> = ins.iter()
            .map(|(n, t)| format!("{}: {}", escape_ident(n), fmt_ty(t, ctx)))
            .collect();
        let arg_names: Vec<String> = ins.iter().map(|(n, _)| escape_ident(n)).collect();
        writeln!(out, "{indent}impl {ename} {{").unwrap();
        writeln!(out, "{indent}    pub fn new({}) -> Result<{ename}> {{", params.join(", ")).unwrap();
        writeln!(out, "{indent}        // TODO: implement runtime constructor for external object `{name}`.").unwrap();
        writeln!(out, "{indent}        // The MetaModelica source declares `external \"C\" ...`; the").unwrap();
        writeln!(out, "{indent}        // matching C runtime function must be ported to Rust or FFI'd.").unwrap();
        let _ = arg_names; // bindings are unused until the runtime is wired up
        writeln!(out, "{indent}        todo!(\"external object `{name}`: constructor not implemented\")").unwrap();
        writeln!(out, "{indent}    }}").unwrap();
        writeln!(out, "{indent}}}").unwrap();
        // Free-function shim with the class name. MetaModelica spells the
        // constructor call as `Foo(...)` (or `Pkg.Foo(...)` from outside),
        // so we expose a function with the class name that forwards.
        let shim_args = ins.iter().map(|(n, _)| escape_ident(n)).collect::<Vec<_>>().join(", ");
        writeln!(out, "{indent}#[allow(non_snake_case)]").unwrap();
        writeln!(out, "{indent}pub fn {ename}({}) -> Result<{ename}> {{", params.join(", ")).unwrap();
        writeln!(out, "{indent}    {ename}::new({shim_args})").unwrap();
        writeln!(out, "{indent}}}").unwrap();
        writeln!(out).unwrap();
    }
    if let Some((_, dname)) = dtor {
        let (ins, _outs) = get_io(dname).unwrap_or((vec![], vec![]));
        // Prefix the parameter name with `_` to silence "unused" warnings.
        // Apply the underscore *before* `escape_ident` so we don't end up with
        // `_r#name` (an invalid Rust identifier — the raw-identifier prefix
        // must be at the start of the token).
        let params: Vec<String> = ins.iter()
            .map(|(n, t)| format!("{}: {}", escape_ident(&format!("_{n}")), fmt_ty(t, ctx)))
            .collect();
        writeln!(out, "{indent}/// Destructor stub. The Rust value's `Drop` impl (added by the").unwrap();
        writeln!(out, "{indent}/// runtime when implemented) is responsible for releasing the").unwrap();
        writeln!(out, "{indent}/// underlying resource; calling this explicitly is a no-op.").unwrap();
        writeln!(out, "{indent}pub fn destructor({}) {{}}", params.join(", ")).unwrap();
        writeln!(out).unwrap();
    }

    // Emit any other function children (rare for ExternalObjects since the
    // useful operations are usually declared at the enclosing package level,
    // but we recurse here for completeness).
    for member in members {
        if let MM::ClassMember::ClassDef(cdm) = member {
            if matches!(cdm.class_def.name.as_str(), "constructor" | "destructor") {
                continue;
            }
            if let Some(child_node) = node.children.get(&cdm.class_def.name)
                && let NodeKind::Class(child_class) = &child_node.kind
                    && matches!(child_class.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
                        emit_function(out, &cdm.class_def.name, child_node, child_class, indent, &mut *ctx, top_level);
                    }
        }
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
                        let fields = component_fields_with_spec(rc, &rec_node.children);
                        if fields.is_empty() {
                            writeln!(out, "{inner}    {rec_name},").unwrap();
                        } else {
                            // Look up the enclosing-package scope so a field whose
                            // declared type is a bare alias name (e.g. `Value value`)
                            // can be emitted as that alias rather than its resolved
                            // concrete type. The lookup is done once per record;
                            // failure (e.g. top-level package not in `top_level`)
                            // simply falls back to the resolved-type path.
                            let scope = current_scope_children(ctx, top_level);
                            writeln!(out, "{inner}    {rec_name} {{").unwrap();
                            for (fname, fty, fspec) in &fields {
                                let ty_str = scope
                                    .and_then(|sc| field_type_alias_name(fspec, sc))
                                    .unwrap_or_else(|| fmt_ty(fty, &mut *ctx));
                                writeln!(out, "{inner}        {}: {},", escape_ident(fname), ty_str).unwrap();
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
            // Use `self::<Enum>::{...}` so the inner module name and the
            // outer module wrapping it (we emit a `pub mod {ename}` around
            // each uniontype that needs sibling imports) don't shadow each
            // other: `Entry::{CLASS,...}` is ambiguous when both
            // `mod Entry` and `enum Entry` are in scope, but
            // `self::Entry::{CLASS,...}` resolves uniquely to the inner
            // type because `self::` skips the value namespace altogether.
            writeln!(out, "{inner}pub use self::{ename}::{{{variant_list}}};").unwrap();
        }
        Ty::AliasTo(_) => {
            // Single-record uniontype: emit one struct named after the uniontype.
            // The record's Ty::RustStruct was updated in seed_metarecords to carry the
            // uniontype's qname, so all constructor/pattern references already resolve here.
            let recs = records_in_order(c);
            let rec_name = recs.into_iter().next().unwrap_or_default();
            if let Some(rec_node) = node.children.get(&rec_name)
                && let NodeKind::Class(rc) = &rec_node.kind {
                    emit_struct(out, name, rec_node, rc, &inner, &mut *ctx, top_level);
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
        match member {
            MM::ClassMember::ClassDef(cdm) => {
                if let Some(child_node) = node.children.get(&cdm.class_def.name)
                    && let NodeKind::Class(child_class) = &child_node.kind {
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
            // Constants declared directly inside the uniontype body
            // (e.g. `constant Binding EMPTY_BINDING = UNBOUND();` in
            // NFBinding.mo) must still be emitted. The `pub mod` wrapper
            // created above gives them a stable lookup path
            // (`Binding::EMPTY_BINDING`); without this dispatch they were
            // silently dropped and every use site failed with E0425.
            MM::ClassMember::Component(comp) => {
                if let Some(child_node) = node.children.get(&comp.name) {
                    emit_node(out, &comp.name, child_node, &inner, &mut *ctx, top_level);
                }
            }
            MM::ClassMember::Extends(_) | MM::ClassMember::Import(_) | MM::ClassMember::LexerComment(_) => {}
        }
    }

    if wrap_in_mod {
        ctx.current_path.pop();
        writeln!(out, "{indent}}}").unwrap();
    }
    writeln!(out).unwrap();
}

fn emit_struct<'a>(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) {
    let fields = component_fields_with_spec(c, &node.children);
    let ename = escape_ident(name);
    let mut type_vars: Vec<String> = Vec::new();
    for (_, fty, _) in &fields {
        collect_type_vars_in_ty(fty, &mut type_vars);
    }
    // Use the minimum bound `Clone + 'static` on struct type parameters.
    // We intentionally drop `PartialEq` here even though `#[derive(Clone,
    // Debug, PartialEq)]` is emitted below: `#[derive(PartialEq)]` produces
    // a *conditional* impl (`impl<T: PartialEq> PartialEq for Foo<T>`), so
    // the struct itself does not require `T: PartialEq` to be defined or
    // used — it only requires it where the receiver code actually invokes
    // structural equality. Keeping `PartialEq` in the struct's *declared*
    // bound, on the other hand, would force every use site (including
    // functions that just forward the value) to also carry `T: PartialEq`,
    // breaking code that flows non-`PartialEq` values like `&impl Fn(...)`
    // through generic forwarders.
    let type_params = if type_vars.is_empty() {
        String::new()
    } else {
        let bounded: Vec<String> = type_vars.iter().map(|v| format!("{v}: Clone")).collect();
        format!("<{}>", bounded.join(", "))
    };
    let derives = match &node.ty {
        Ty::RustStruct(qname) => ctx.derives_for(qname),
        _ => "#[derive(Clone, Debug, PartialEq)]",
    };
    writeln!(out, "{derives}").unwrap();
    if fields.is_empty() {
        writeln!(out, "{indent}pub struct {ename}{type_params};").unwrap();
    } else {
        let scope = current_scope_children(ctx, top_level);
        writeln!(out, "{indent}pub struct {ename}{type_params} {{").unwrap();
        for (fname, fty, fspec) in &fields {
            let ty_str = scope
                .and_then(|sc| field_type_alias_name(fspec, sc))
                .unwrap_or_else(|| fmt_ty(fty, &mut *ctx));
            writeln!(out, "{indent}    pub {}: {},", escape_ident(fname), ty_str).unwrap();
        }
        writeln!(out, "{indent}}}").unwrap();
    }
    writeln!(out).unwrap();
}

fn emit_type_item(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str, ctx: &mut GenCtx) {
    match &c.body {
        MM::ClassDef::Derived { type_spec, .. } if {
            if let Absyn::TypeSpec::TCOMPLEX { path, .. } = type_spec.as_ref() {
                matches!(path.as_ref(), Absyn::Path::IDENT { name } if name.as_str() == "polymorphic")
            } else { false }
        } => (),
        MM::ClassDef::Derived { .. } => {
            let mut type_vars: Vec<String> = Vec::new();
            collect_type_vars_in_ty(&node.ty, &mut type_vars);
            let type_params = if type_vars.is_empty() { String::new() } else { format!("<{}>", type_vars.join(", ")) };
            writeln!(out, "{indent}pub type {}{type_params} = {};", escape_ident(name), fmt_ty(&node.ty, &mut *ctx)).unwrap();
            writeln!(out).unwrap();
        }
        MM::ClassDef::Enumeration { enum_literals, .. } => {
            if let Absyn::EnumDef::ENUMLITERALS { enumLiterals } = enum_literals.as_ref() {
                // Modelica enumeration semantics (Modelica spec 4.8.5.2 and built-in
                // `Integer(E.ei)`):
                //   - Literals are ordered by declaration: Integer(E.e1) = 1, …,
                //     Integer(E.en) = n.
                //   - Relational ops (`<`, `<=`, `>=`, `>`) compare values by ordinal.
                //
                // We therefore emit a fieldless `#[repr(i32)]` enum with explicit
                // discriminants starting at 1, so `value as i32` directly yields the
                // Modelica ordinal — matching the C-runtime convention and what the
                // `Integer(<enum>)` built-in expects (see emit_builtin_call "Integer").
                //
                // We do NOT `#[derive(PartialOrd, Ord)]`: while the derived order
                // would coincide here (declaration order == ordinal order), a custom
                // impl rooted in the discriminant cast makes the Modelica semantics
                // explicit and survives reordering of variants by future maintenance.
                //
                // `Copy` is added because Modelica enumeration values are scalar
                // values; this also lets `*self as i32` in the comparison impls work
                // without consuming `self`.
                let ename = escape_ident(name);
                writeln!(out, "{indent}#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]").unwrap();
                writeln!(out, "{indent}#[repr(i32)]").unwrap();
                writeln!(out, "{indent}pub enum {ename} {{").unwrap();
                let mut i: i32 = 0;
                for lit in &**enumLiterals {
                    let Absyn::EnumLiteral { literal, .. } = &**lit;
                    i += 1;
                    writeln!(out, "{indent}    {} = {i},", escape_ident(literal)).unwrap();
                }
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out, "{indent}impl PartialOrd for {ename} {{").unwrap();
                writeln!(out, "{indent}    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {{ Some(self.cmp(other)) }}").unwrap();
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out, "{indent}impl Ord for {ename} {{").unwrap();
                writeln!(out, "{indent}    fn cmp(&self, other: &Self) -> std::cmp::Ordering {{ (*self as i32).cmp(&(*other as i32)) }}").unwrap();
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
            }
        }
        _ => {}
    }
}

// ── Tail-call analysis ──────────────────────────────────────────────────────
//
// We detect single-output, infallible functions whose body either is, or can
// be rewritten into, a single tail expression that contains at least one
// self-call in tail position.  The classes of body we recognise are:
//
//   (A) `<preamble>; outVar := <tail-expr>;`
//       — the canonical "match in tail position" shape, e.g. `BaseAvlTree.get`.
//
//   (B) `<preamble>; if c then … else … end if;` where every branch is itself
//       a body of class (A) / (B) / (C) targeting the same `outVar`.
//       — the gcd-via-if shape:
//           algorithm
//             if b == 0 then  outResult := a;
//             else            outResult := gcd(b, mod(a,b));
//             end if;
//
//   (C) Match-expression cases with algorithm-side recursion:
//           case X algorithm  outLocal := f(...);  then outLocal;
//       The case's tail-position value is the RHS of its last `Assign` (when
//       that assigns to the variable yielded by the `then`).  This is the
//       `BaseAvlTree.listKeys` / `listValues` shape.
//
// "Tail position" inside an expression descends through arms of `if`/`match`
// and the algorithm-side last-assign of `match` cases.
//
// When a function matches one of these shapes *and* is infallible (the
// `#[tailcall]` crate forbids `?` in the body), we drop the `let mut outVar;
// … outVar = e; outVar` framing and emit the body as a single trailing
// expression — recursively pushing branch-final assignments down into
// `if`/`else` arms so the self-call ends up syntactically last.  We then
// annotate the function with `#[tailcall::tailcall]`.
//
// Multi-mutual recursion (SCCs of size 2–3) is *not* implemented — see the
// open TODO. The current MetaModelica source rarely needs it (every
// important case in `BaseAvlTree`/`List` is a single self-recursive
// function), so we leave it as a documented follow-up rather than introduce
// a trampoline scheme on speculation.

/// Return true if any tail position of `exp` is a call to `self_short_name`.
///
/// We look at the *MetaModelica* function name as it appears in
/// `TypedExp::Call.func`. MM source written *inside* `function f` references
/// the function as bare `f(...)` — `Call.func == "f"` exactly. A dotted form
/// in `Call.func` (e.g. `ErrorExt.getMessages`) always denotes a *cross-module*
/// call, even when the trailing segment coincides with the enclosing
/// function's name (cross-module forwarders are a common pattern: an
/// infallible `getMessages()` in `Error` that forwards to
/// `ErrorExt.getMessages()`). We therefore require strict equality: matching
/// the trailing segment of a dotted call would create false positives that
/// then drive `#[tailcall]` annotations on non-recursive forwarders.
///
/// TODO: handle nested-function shadowing — if `function f` is itself nested
/// inside `function g.f`, a call to `f(...)` in `g.f`'s body resolves to
/// whichever `f` is in scope by MetaModelica's lookup rules. The current
/// `TypedExp::Call.func` representation does not carry that resolution.
/// Strict short-name matching is correct for the (overwhelmingly common)
/// case where there is no shadowing.
fn exp_has_tail_self_call(exp: &TypedExp, self_short_name: &str) -> bool {
    match exp {
        TypedExp::Call { func, .. } => func == self_short_name,
        TypedExp::If { then_, elseif, else_, .. } => {
            exp_has_tail_self_call(then_, self_short_name)
                || elseif.iter().any(|(_, e)| exp_has_tail_self_call(e, self_short_name))
                || exp_has_tail_self_call(else_, self_short_name)
        }
        TypedExp::Match { cases, .. } => {
            cases.iter().any(|case| {
                // A case's tail position is its `result` expression, *unless* the
                // case has algorithm-side statements ending in an assignment to
                // a local that's then yielded — in that case the `then` expr
                // here is just the local. We could descend into the
                // algorithm-side via `stmts_have_tail_self_call`, but for now
                // we only inspect the `result` expression because that's the
                // shape the current TypedCase representation exposes uniformly.
                // Cases that use algorithm-side recursion (the
                // `BaseAvlTree.listKeys` pattern with `algorithm lst := f(...); then lst;`)
                // are not yet detected — see the TODO above.
                case_tail_has_self_call(case, self_short_name)
            })
        }
        // Constructors, binary ops, etc. never have a tail-position call: the
        // call's value flows into a wrapping operation, not back to the caller.
        _ => false,
    }
}

/// Detect a self-call in the tail position of a match case.
///
/// Two shapes can carry a tail self-call:
///   * `then <expr>;` — the result expression itself contains a self-call;
///   * `algorithm <stmts>; then <var>;` where the last statement of `<stmts>`
///     is `<var> := <expr>;` and `<expr>` contains a tail self-call. In that
///     case `<expr>` is what the case yields, so it *is* in tail position
///     even though `result` is just a variable reference.
fn case_tail_has_self_call(case: &TypedCase, self_short_name: &str) -> bool {
    if let TypedExp::Var { name: result_name, .. } = &case.result
        && let Some(typedexp::TypedStmt::Assign { lhs: TypedPat::Var(asg_name), rhs }) = case.stmts.last()
            && asg_name == result_name && exp_has_tail_self_call(rhs, self_short_name) {
                return true;
            }
    exp_has_tail_self_call(&case.result, self_short_name)
}

/// Return true if the body `stmts` (interpreted as code that will produce
/// `out_name`'s value) has a self-call in some tail position. Mirrors the
/// recursion in [`stmts_lowerable_as_tail_expr`].
fn stmts_have_tail_self_call(
    stmts: &[typedexp::TypedStmt],
    out_name: &str,
    self_short_name: &str,
) -> bool {
    let Some(last) = stmts.last() else { return false; };
    match last {
        typedexp::TypedStmt::Assign { lhs: TypedPat::Var(name), rhs } if name == out_name => {
            exp_has_tail_self_call(rhs, self_short_name)
        }
        typedexp::TypedStmt::If { then_, elseif, else_, .. } => {
            stmts_have_tail_self_call(then_, out_name, self_short_name)
                || elseif.iter().any(|(_, b)| stmts_have_tail_self_call(b, out_name, self_short_name))
                || stmts_have_tail_self_call(else_, out_name, self_short_name)
        }
        _ => false,
    }
}

/// Return true if `stmts` can be re-emitted as a single tail expression that
/// evaluates to `out_name`'s value.
///
/// This is a *structural* check; it does not look at self-calls. We use it
/// together with [`stmts_have_tail_self_call`] to gate the lowering.
fn stmts_lowerable_as_tail_expr(stmts: &[typedexp::TypedStmt], out_name: &str) -> bool {
    let Some(last) = stmts.last() else { return false; };
    match last {
        typedexp::TypedStmt::Assign { lhs: TypedPat::Var(name), .. } => name == out_name,
        typedexp::TypedStmt::If { then_, elseif, else_, .. } => {
            // An incomplete if (no else, or an `else` whose branch leaves
            // `out` unset) leaves the function's value undefined unless the
            // preamble assigned `out` first — and we already restrict to
            // bodies whose *last* statement is the if. So require a real
            // else branch *and* require every branch to itself end with a
            // tail-position assignment to `out`.
            !else_.is_empty()
                && stmts_lowerable_as_tail_expr(then_, out_name)
                && elseif.iter().all(|(_, b)| stmts_lowerable_as_tail_expr(b, out_name))
                && stmts_lowerable_as_tail_expr(else_, out_name)
        }
        _ => false,
    }
}

/// A plan describing how to lower a function body as a tail-position
/// expression rather than as `let mut out; … out = e; out`.
struct TailCallPlan {
    /// Name of the single output variable whose declaration we suppress and
    /// whose final assignment(s) become the function's tail expression.
    suppressed_out: String,
    /// `true` when the enclosing function returns `Result<T>` — tail-position
    /// leaves that are not self-calls need an `Ok(...)` wrap to match the
    /// function's return type; the self-call itself already returns
    /// `Result<T>` and is emitted bare (no `?`, no wrap).
    fallible: bool,
}

/// Builtin calls whose [`emit_builtin_call`] handler lowers them to a bare
/// Rust expression (no `Result<T>` wrapper, no `ctx.q(...)`, hence no `?`
/// in a fallible enclosing function).  We need this list because the
/// runtime signatures in `metamodelica/src/lib.rs` for many builtins are
/// still `fn(…) -> Result<T>` for historical reasons (see the comment on
/// `fallibility::builtin_fallibility`) — relying on the signature alone
/// would over-approximate.  Keep this in sync with the no-`ctx.q` arms of
/// `emit_builtin_call`.
const BUILTINS_EMIT_BARE: &[&str] = &[
    // `valueEq(a, b)` lowers to `a == b`.
    "valueEq",
];

/// Return true if `func` resolves to a user function that emits its calls
/// with `?` / `.unwrap()` in the current context.
///
/// Conservative: when resolution fails or the callee is unknown we report
/// `true` (fallible) — callers use this only to *skip* the lowering, so a
/// false positive simply means we miss an optimisation, never that we
/// produce wrong code.
fn call_emits_question_mark<'a>(
    func: &str,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> bool {
    if BUILTINS_EMIT_BARE.contains(&func) {
        return false;
    }
    if let Some(q) = resolve_call_qname(func, ctx, top_level) {
        !ctx.is_known_infallible_user_fn(&q, top_level)
    } else {
        // Other builtins (`intMod`, `stringDelimitList`, …) are routed
        // through `ctx.q(...)` which adds `?` in a fallible function.
        // Treat them as fallible.
        true
    }
}

/// Return true if any *non-tail-position* subexpression of `exp` (interpreted
/// as a tail expression yielding the function's value) would emit `?` when
/// the enclosing function is fallible.
///
/// Tail-position self-calls are *not* counted: those will be emitted bare by
/// [`emit_tail_value_exp`].
fn tail_exp_has_non_tail_question_mark<'a>(
    exp: &TypedExp,
    self_name: &str,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> bool {
    match exp {
        // Tail self-call: arguments are evaluated *before* the call returns,
        // so they are not in tail position — any fallible call inside them
        // would force a `?`.
        TypedExp::Call { func, args, named_args, .. } if func == self_name => {
            args.iter().any(|a| exp_uses_question_mark(a, ctx, top_level))
                || named_args.iter().any(|(_, a)| exp_uses_question_mark(a, ctx, top_level))
        }
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            exp_uses_question_mark(cond, ctx, top_level)
                || tail_exp_has_non_tail_question_mark(then_, self_name, ctx, top_level)
                || elseif.iter().any(|(ec, eb)|
                    exp_uses_question_mark(ec, ctx, top_level)
                    || tail_exp_has_non_tail_question_mark(eb, self_name, ctx, top_level))
                || tail_exp_has_non_tail_question_mark(else_, self_name, ctx, top_level)
        }
        // Match in tail position: the scrutinee is non-tail; each case's
        // guards / locals / stmts are non-tail; the case's `result` is the
        // case's tail expression, *except* when the case ends in an
        // algorithm-side `<name> := <rhs>;` whose `<name>` is exactly what
        // `result` yields — in that case `<rhs>` is the tail and the last
        // stmt's RHS is what we will emit. Mirror that logic so the gate
        // doesn't falsely reject lowerable patterns.
        TypedExp::Match { input, cases, .. } => {
            if exp_uses_question_mark(input, ctx, top_level) { return true; }
            cases.iter().any(|case| {
                if case.guard.as_ref().is_some_and(|g| exp_uses_question_mark(g, ctx, top_level)) { return true; }
                if case.locals.iter().any(|(_, _, d, _)| d.as_ref().is_some_and(|e| exp_uses_question_mark(e, ctx, top_level))) {
                    return true;
                }
                // Detect the algorithm-side tail pattern. When present, the
                // last stmt's RHS is in tail position; everything before
                // (and the `result` itself, which is just a var read) is
                // non-tail-but-?-free.
                let algo_tail_rhs = if !case.stmts.is_empty() {
                    if let (typedexp::TypedStmt::Assign { lhs: TypedPat::Var(an), rhs }, TypedExp::Var { name: rn, .. }) =
                        (case.stmts.last().unwrap(), &case.result)
                    {
                        if an == rn { Some(rhs) } else { None }
                    } else { None }
                } else { None };
                let stmts_to_check: &[typedexp::TypedStmt] = if algo_tail_rhs.is_some() {
                    &case.stmts[..case.stmts.len() - 1]
                } else {
                    &case.stmts[..]
                };
                if stmts_to_check.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level)) { return true; }
                match algo_tail_rhs {
                    Some(rhs) => tail_exp_has_non_tail_question_mark(rhs, self_name, ctx, top_level),
                    None => tail_exp_has_non_tail_question_mark(&case.result, self_name, ctx, top_level),
                }
            })
        }
        // Other tail leaves are non-self values that we will wrap in `Ok(...)`
        // for a fallible function. Their subexpressions are not in tail
        // position, so any fallible call inside them yields `?`.
        _ => exp_uses_question_mark(exp, ctx, top_level),
    }
}

/// Return true if `exp` (in *any* position — no tail-position awareness)
/// contains a subexpression that would emit `?` in a fallible function.
fn exp_uses_question_mark<'a>(
    exp: &TypedExp,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> bool {
    match exp {
        TypedExp::Call { func, args, named_args, .. } => {
            if call_emits_question_mark(func, ctx, top_level) { return true; }
            args.iter().any(|a| exp_uses_question_mark(a, ctx, top_level))
                || named_args.iter().any(|(_, a)| exp_uses_question_mark(a, ctx, top_level))
        }
        TypedExp::Constructor { args, named_args, .. } => {
            args.iter().any(|a| exp_uses_question_mark(a, ctx, top_level))
                || named_args.iter().any(|(_, a)| exp_uses_question_mark(a, ctx, top_level))
        }
        TypedExp::PartEval { args, named_args, .. } => {
            args.iter().any(|a| exp_uses_question_mark(a, ctx, top_level))
                || named_args.iter().any(|(_, a)| exp_uses_question_mark(a, ctx, top_level))
        }
        TypedExp::BinOp { lhs, rhs, .. } => {
            exp_uses_question_mark(lhs, ctx, top_level) || exp_uses_question_mark(rhs, ctx, top_level)
        }
        TypedExp::UnOp { operand, .. } => exp_uses_question_mark(operand, ctx, top_level),
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            exp_uses_question_mark(cond, ctx, top_level)
                || exp_uses_question_mark(then_, ctx, top_level)
                || elseif.iter().any(|(ec, eb)|
                    exp_uses_question_mark(ec, ctx, top_level)
                    || exp_uses_question_mark(eb, ctx, top_level))
                || exp_uses_question_mark(else_, ctx, top_level)
        }
        TypedExp::Cons { head, tail, .. } => {
            exp_uses_question_mark(head, ctx, top_level) || exp_uses_question_mark(tail, ctx, top_level)
        }
        TypedExp::Tuple(es) => es.iter().any(|e| exp_uses_question_mark(e, ctx, top_level)),
        TypedExp::Array { elems, .. } => elems.iter().any(|e| exp_uses_question_mark(e, ctx, top_level)),
        TypedExp::Match { input, cases, .. } => {
            exp_uses_question_mark(input, ctx, top_level)
                || cases.iter().any(|c| case_uses_question_mark(c, ctx, top_level))
        }
        TypedExp::Range { start, step, stop, .. } => {
            exp_uses_question_mark(start, ctx, top_level)
                || step.as_ref().is_some_and(|s| exp_uses_question_mark(s, ctx, top_level))
                || exp_uses_question_mark(stop, ctx, top_level)
        }
        TypedExp::Reduction { body, iterators, .. } => {
            exp_uses_question_mark(body, ctx, top_level)
                || iterators.iter().any(|it| exp_uses_question_mark(&it.range, ctx, top_level))
        }
        TypedExp::Var { segments, .. } => {
            // CrefSegment subscripts may evaluate fallible expressions; scan them.
            segments.iter().any(|seg| seg.subscripts.iter().any(|s| exp_uses_question_mark(s, ctx, top_level)))
        }
        TypedExp::Lit(_) | TypedExp::Todo(_) => false,
    }
}

fn case_uses_question_mark<'a>(
    case: &TypedCase,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> bool {
    if let Some(g) = &case.guard
        && exp_uses_question_mark(g, ctx, top_level) { return true; }
    if case.locals.iter().any(|(_, _, d, _)| d.as_ref().is_some_and(|e| exp_uses_question_mark(e, ctx, top_level))) {
        return true;
    }
    if case.stmts.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level)) { return true; }
    exp_uses_question_mark(&case.result, ctx, top_level)
}

/// Do `pats` (treated as a sum-of-cases pattern with no guard between them)
/// cover every value of `ty`? Conservative: returns `false` for types whose
/// constructor space we cannot enumerate (uniontypes, enumerations, the
/// scalar number types — exhaustiveness over those requires either a
/// hierarchy lookup, which we have not yet wired up here, or an irrefutable
/// fallback case, which is handled by the leading `pat_is_irrefutable`
/// check).
fn pats_cover_ty(pats: &[&TypedPat], ty: &Ty) -> bool {
    if pats.iter().any(|p| pat_is_irrefutable(p)) { return true; }
    match ty {
        Ty::Bool => {
            pats.iter().any(|p| matches!(p, TypedPat::Lit(Lit::Bool(true))))
                && pats.iter().any(|p| matches!(p, TypedPat::Lit(Lit::Bool(false))))
        }
        Ty::List(_) => {
            // Exhaustive iff some case matches Nil AND some case matches every
            // Cons. A `Cons{head, tail}` with both subpatterns irrefutable
            // covers every Cons; richer coverage (a finite union of length-
            // pinned Cons cases plus a recursive Cons that handles the rest)
            // is not yet attempted.
            let nil = pats.iter().any(|p| matches!(p, TypedPat::EmptyList));
            let cons = pats.iter().any(|p| matches!(p,
                TypedPat::Cons { head, tail }
                    if pat_is_irrefutable(head) && pat_is_irrefutable(tail)));
            nil && cons
        }
        Ty::Option(_) => {
            let none = pats.iter().any(|p| matches!(p, TypedPat::None_));
            let some = pats.iter().any(|p| matches!(p,
                TypedPat::Some_(inner) if pat_is_irrefutable(inner)));
            none && some
        }
        Ty::Tuple(elem_tys) => {
            // A single `Tuple(ps)` case is exhaustive iff every component
            // covers its respective element type.
            pats.iter().any(|p| match p {
                TypedPat::Tuple(ps) if ps.len() == elem_tys.len() => {
                    ps.iter().zip(elem_tys.iter()).all(|(p, t)| pats_cover_ty(&[p], t))
                }
                _ => false,
            })
        }
        // TODO: Ty::RustEnum / Ty::AliasTo — would need to enumerate the
        // variants via the hierarchy and check that each is covered by a
        // Constructor pattern whose fields are themselves covering. Left
        // unhandled for now; callers fall through to "non-exhaustive".
        //
        // TODO: Ty::Enumeration — same, need the enum's literal list.
        //
        // Numeric / string scalars can only be made exhaustive by an
        // irrefutable case (handled at the top of this function).
        _ => false,
    }
}

/// Are `cases` exhaustive over `scrut_ty`? Excludes guarded cases (a guard
/// can fail, so a guarded case never *covers* anything in the
/// exhaustiveness sense). `matchcontinue` is never considered exhaustive
/// because any arm body may `fail()` and fall through to the next arm,
/// eventually exhausting all arms even with full pattern coverage.
fn cases_exhaustive(kind: &MatchKind, cases: &[TypedCase], scrut_ty: &Ty) -> bool {
    if !matches!(kind, MatchKind::Match) { return false; }
    let pats: Vec<&TypedPat> = cases.iter()
        .filter(|c| c.guard.is_none())
        .map(|c| &c.pattern)
        .collect();
    pats_cover_ty(&pats, scrut_ty)
}

/// Return true if `exp` contains a non-tail-position match whose cases do
/// not exhaustively cover the scrutinee type. Such a match would lower its
/// fallback to `unreachable!()` inside a `#[tailcall]` body — see
/// [`emit_match`] — converting MM's "fail when no arm matches" semantics
/// into a Rust panic. We refuse the lowering when that would happen so the
/// generated code stays panic-free.
fn exp_has_nonexhaustive_match(exp: &TypedExp) -> bool {
    match exp {
        TypedExp::Match { kind, input, cases, .. } => {
            if !cases_exhaustive(kind, cases, &input.ty()) { return true; }
            exp_has_nonexhaustive_match(input)
                || cases.iter().any(|c| {
                    c.guard.as_ref().is_some_and(exp_has_nonexhaustive_match)
                        || c.locals.iter().any(|(_, _, d, _)| d.as_ref().is_some_and(exp_has_nonexhaustive_match))
                        || c.stmts.iter().any(stmt_has_nonexhaustive_match)
                        || exp_has_nonexhaustive_match(&c.result)
                })
        }
        TypedExp::Call { args, named_args, .. }
        | TypedExp::Constructor { args, named_args, .. }
        | TypedExp::PartEval { args, named_args, .. } => {
            args.iter().any(exp_has_nonexhaustive_match)
                || named_args.iter().any(|(_, e)| exp_has_nonexhaustive_match(e))
        }
        TypedExp::BinOp { lhs, rhs, .. } => exp_has_nonexhaustive_match(lhs) || exp_has_nonexhaustive_match(rhs),
        TypedExp::UnOp { operand, .. } => exp_has_nonexhaustive_match(operand),
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            exp_has_nonexhaustive_match(cond)
                || exp_has_nonexhaustive_match(then_)
                || elseif.iter().any(|(c, e)| exp_has_nonexhaustive_match(c) || exp_has_nonexhaustive_match(e))
                || exp_has_nonexhaustive_match(else_)
        }
        TypedExp::Cons { head, tail, .. } => exp_has_nonexhaustive_match(head) || exp_has_nonexhaustive_match(tail),
        TypedExp::Tuple(es) => es.iter().any(exp_has_nonexhaustive_match),
        TypedExp::Array { elems, .. } => elems.iter().any(exp_has_nonexhaustive_match),
        TypedExp::Range { start, step, stop, .. } => {
            exp_has_nonexhaustive_match(start)
                || step.as_ref().is_some_and(|s| exp_has_nonexhaustive_match(s))
                || exp_has_nonexhaustive_match(stop)
        }
        TypedExp::Reduction { body, iterators, .. } => {
            exp_has_nonexhaustive_match(body)
                || iterators.iter().any(|it| exp_has_nonexhaustive_match(&it.range)
                    || it.guard.as_ref().is_some_and(exp_has_nonexhaustive_match))
        }
        TypedExp::Var { segments, .. } => {
            segments.iter().any(|s| s.subscripts.iter().any(exp_has_nonexhaustive_match))
        }
        TypedExp::Lit(_) | TypedExp::Todo(_) => false,
    }
}

fn stmt_has_nonexhaustive_match(stmt: &typedexp::TypedStmt) -> bool {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { rhs, .. } => exp_has_nonexhaustive_match(rhs),
        S::NoRetCall { call } => exp_has_nonexhaustive_match(call),
        S::If { cond, then_, elseif, else_ } => {
            exp_has_nonexhaustive_match(cond)
                || then_.iter().any(stmt_has_nonexhaustive_match)
                || elseif.iter().any(|(ec, eb)| exp_has_nonexhaustive_match(ec) || eb.iter().any(stmt_has_nonexhaustive_match))
                || else_.iter().any(stmt_has_nonexhaustive_match)
        }
        S::For { range, body, .. } => exp_has_nonexhaustive_match(range) || body.iter().any(stmt_has_nonexhaustive_match),
        S::While { cond, body } => exp_has_nonexhaustive_match(cond) || body.iter().any(stmt_has_nonexhaustive_match),
        S::Try { body, else_body } => body.iter().any(stmt_has_nonexhaustive_match) || else_body.iter().any(stmt_has_nonexhaustive_match),
        S::Failure { body } => body.iter().any(stmt_has_nonexhaustive_match),
        S::Return | S::Break | S::Continue | S::Todo(_) => false,
    }
}

/// Same as `exp_has_nonexhaustive_match`, but treats `exp` as a tail
/// position: a tail-position match whose cases all yield `Result<T>` lowers
/// its fallback to `Err(anyhow::anyhow!(...))` (a value, not a panic), so
/// it does *not* count against the panic-free constraint. Sub-expressions
/// that are themselves non-tail (the scrutinee, case guards/locals/stmts,
/// and per-case sub-results that are themselves non-tail) are still
/// checked recursively.
fn tail_exp_has_nonexhaustive_nontail_match(
    exp: &TypedExp,
    self_name: &str,
    fallible: bool,
) -> bool {
    match exp {
        // The tail-self-call's arguments are non-tail.
        TypedExp::Call { func, args, named_args, .. } if func == self_name => {
            args.iter().any(exp_has_nonexhaustive_match)
                || named_args.iter().any(|(_, e)| exp_has_nonexhaustive_match(e))
        }
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            exp_has_nonexhaustive_match(cond)
                || tail_exp_has_nonexhaustive_nontail_match(then_, self_name, fallible)
                || elseif.iter().any(|(c, e)|
                    exp_has_nonexhaustive_match(c)
                    || tail_exp_has_nonexhaustive_nontail_match(e, self_name, fallible))
                || tail_exp_has_nonexhaustive_nontail_match(else_, self_name, fallible)
        }
        TypedExp::Match { kind, input, cases, .. } => {
            // Tail-position match: its own fallback is `Err(...)` (fallible)
            // or `unreachable!()` (infallible). Only the fallible case is
            // panic-free, so for an infallible tail-lowered function we
            // still reject a non-exhaustive tail-position match.
            if !cases_exhaustive(kind, cases, &input.ty()) && !fallible { return true; }
            if exp_has_nonexhaustive_match(input) { return true; }
            cases.iter().any(|c| {
                if c.guard.as_ref().is_some_and(exp_has_nonexhaustive_match) { return true; }
                if c.locals.iter().any(|(_, _, d, _)| d.as_ref().is_some_and(exp_has_nonexhaustive_match)) { return true; }
                // Detect algorithm-side tail (last stmt is `Assign(name, rhs)` + result `Var(name)`).
                let algo_tail_rhs = if !c.stmts.is_empty() {
                    if let (typedexp::TypedStmt::Assign { lhs: TypedPat::Var(an), rhs }, TypedExp::Var { name: rn, .. }) =
                        (c.stmts.last().unwrap(), &c.result)
                    {
                        if an == rn { Some(rhs) } else { None }
                    } else { None }
                } else { None };
                let stmts_to_check: &[typedexp::TypedStmt] = if algo_tail_rhs.is_some() {
                    &c.stmts[..c.stmts.len() - 1]
                } else {
                    &c.stmts[..]
                };
                if stmts_to_check.iter().any(stmt_has_nonexhaustive_match) { return true; }
                match algo_tail_rhs {
                    Some(rhs) => tail_exp_has_nonexhaustive_nontail_match(rhs, self_name, fallible),
                    None => tail_exp_has_nonexhaustive_nontail_match(&c.result, self_name, fallible),
                }
            })
        }
        // Any other leaf is non-tail-control-flow; recurse over it as a
        // non-tail subexpression.
        _ => exp_has_nonexhaustive_match(exp),
    }
}

fn body_has_nonexhaustive_nontail_match(
    stmts: &[typedexp::TypedStmt],
    out_name: &str,
    self_name: &str,
    fallible: bool,
) -> bool {
    let Some((last, head)) = stmts.split_last() else { return false; };
    if head.iter().any(stmt_has_nonexhaustive_match) { return true; }
    match last {
        typedexp::TypedStmt::Assign { lhs: TypedPat::Var(name), rhs } if name == out_name => {
            tail_exp_has_nonexhaustive_nontail_match(rhs, self_name, fallible)
        }
        typedexp::TypedStmt::If { cond, then_, elseif, else_ } => {
            exp_has_nonexhaustive_match(cond)
                || body_has_nonexhaustive_nontail_match(then_, out_name, self_name, fallible)
                || elseif.iter().any(|(ec, eb)|
                    exp_has_nonexhaustive_match(ec)
                    || body_has_nonexhaustive_nontail_match(eb, out_name, self_name, fallible))
                || body_has_nonexhaustive_nontail_match(else_, out_name, self_name, fallible)
        }
        _ => true,
    }
}

fn stmt_uses_question_mark<'a>(
    stmt: &typedexp::TypedStmt,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> bool {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { rhs, .. } => exp_uses_question_mark(rhs, ctx, top_level),
        S::NoRetCall { call } => exp_uses_question_mark(call, ctx, top_level),
        S::If { cond, then_, elseif, else_ } => {
            exp_uses_question_mark(cond, ctx, top_level)
                || then_.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level))
                || elseif.iter().any(|(ec, eb)|
                    exp_uses_question_mark(ec, ctx, top_level)
                    || eb.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level)))
                || else_.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level))
        }
        S::For { range, body, .. } => {
            exp_uses_question_mark(range, ctx, top_level)
                || body.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level))
        }
        S::While { cond, body } => {
            exp_uses_question_mark(cond, ctx, top_level)
                || body.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level))
        }
        S::Try { body, else_body } => {
            body.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level))
                || else_body.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level))
        }
        S::Failure { body } => body.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level)),
        S::Return | S::Break | S::Continue | S::Todo(_) => false,
    }
}

/// Walk the body, with last-stmt awareness, to decide whether the lowered
/// form would emit any `?` (other than on the tail self-call, which we
/// suppress). `#[tailcall]` rejects `?` in the body — so we use this as a
/// gate for the fallible case.
fn body_emits_extra_question_mark<'a>(
    stmts: &[typedexp::TypedStmt],
    out_name: &str,
    self_name: &str,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> bool {
    let Some((last, head)) = stmts.split_last() else { return false; };
    // Preamble stmts are emitted verbatim; any `?` they would produce stays.
    if head.iter().any(|s| stmt_uses_question_mark(s, ctx, top_level)) { return true; }
    last_stmt_emits_extra_question_mark(last, out_name, self_name, ctx, top_level)
}

fn last_stmt_emits_extra_question_mark<'a>(
    stmt: &typedexp::TypedStmt,
    out_name: &str,
    self_name: &str,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> bool {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { lhs: TypedPat::Var(name), rhs } if name == out_name => {
            tail_exp_has_non_tail_question_mark(rhs, self_name, ctx, top_level)
        }
        S::If { cond, then_, elseif, else_ } => {
            exp_uses_question_mark(cond, ctx, top_level)
                || body_emits_extra_question_mark(then_, out_name, self_name, ctx, top_level)
                || elseif.iter().any(|(ec, eb)|
                    exp_uses_question_mark(ec, ctx, top_level)
                    || body_emits_extra_question_mark(eb, out_name, self_name, ctx, top_level))
                || body_emits_extra_question_mark(else_, out_name, self_name, ctx, top_level)
        }
        // Anything else as the last stmt fails the structural lowerability
        // check earlier; the caller should never reach this arm. Be loud.
        _ => true,
    }
}

/// Decide whether `typed_stmts` qualifies for the tail-call lowering
/// described at the top of this section. Returns the plan when it does,
/// `None` otherwise.
/// Return true if `name` is read anywhere in `stmts` — either as a bare
/// variable reference on the RHS of an assignment, as a function/sub-call
/// argument, or as part of a more complex expression. Used by
/// [`plan_tail_call_lowering`] to refuse suppressing the declaration of an
/// output that the body reads (e.g. an accumulator threaded through
/// self-calls in `fold`).
///
/// The "LHS-only" position of an `Assign { lhs: TypedPat::Var(n), .. }` does
/// not count as a read, but if `name` appears in a *destructuring* pattern
/// (`(a, b) = ...`) or in a tuple/multi-output LHS, it's still being
/// assigned, not read. The single-output suppression case is the only one
/// we generate today, so we treat any non-top-level reference as a read.
fn stmts_read_name(stmts: &[typedexp::TypedStmt], name: &str) -> bool {
    stmts.iter().any(|s| stmt_reads_name(s, name))
}

fn stmt_reads_name(stmt: &typedexp::TypedStmt, name: &str) -> bool {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { lhs, rhs } => {
            // The LHS being exactly `Var(name)` is a write, not a read. Any
            // other LHS shape (tuple destructure, segments) doesn't have the
            // name at top-level — but if it appears nested (e.g. as a tuple
            // element on the LHS), that's still a write site; the codegen
            // for tuple destructure doesn't read.
            let lhs_write_only = matches!(lhs, TypedPat::Var(n) if n == name);
            (!lhs_write_only && pat_reads_name(lhs, name)) || exp_reads_name(rhs, name)
        }
        S::NoRetCall { call } => exp_reads_name(call, name),
        S::If { cond, then_, elseif, else_ } => {
            exp_reads_name(cond, name)
                || stmts_read_name(then_, name)
                || elseif.iter().any(|(c, b)| exp_reads_name(c, name) || stmts_read_name(b, name))
                || stmts_read_name(else_, name)
        }
        S::For { range, body, .. } => exp_reads_name(range, name) || stmts_read_name(body, name),
        S::While { cond, body } => exp_reads_name(cond, name) || stmts_read_name(body, name),
        S::Try { body, else_body } => stmts_read_name(body, name) || stmts_read_name(else_body, name),
        S::Failure { body } => stmts_read_name(body, name),
        S::Return | S::Break | S::Continue | S::Todo(_) => false,
    }
}

fn pat_reads_name(_pat: &TypedPat, _name: &str) -> bool {
    // Patterns are write positions (binders). They don't read prior values.
    false
}

fn exp_reads_name(exp: &typedexp::TypedExp, name: &str) -> bool {
    use typedexp::TypedExp as E;
    match exp {
        E::Lit(_) | E::Todo(_) => false,
        E::Var { name: n, segments, .. } => {
            // Bare reference, or a base reference of a segmented path
            // (`outResult.something`). Either way it's a read of `name`.
            if n == name { return true; }
            segments.iter().any(|seg| seg.subscripts.iter().any(|s| exp_reads_name(s, name)))
        }
        E::BinOp { lhs, rhs, .. } => exp_reads_name(lhs, name) || exp_reads_name(rhs, name),
        E::UnOp { operand, .. } => exp_reads_name(operand, name),
        E::Call { args, named_args, .. }
        | E::Constructor { args, named_args, .. }
        | E::PartEval { args, named_args, .. } => {
            args.iter().any(|a| exp_reads_name(a, name))
                || named_args.iter().any(|(_, v)| exp_reads_name(v, name))
        }
        E::If { cond, then_, elseif, else_, .. } => {
            exp_reads_name(cond, name)
                || exp_reads_name(then_, name)
                || elseif.iter().any(|(c, b)| exp_reads_name(c, name) || exp_reads_name(b, name))
                || exp_reads_name(else_, name)
        }
        E::Cons { head, tail, .. } => exp_reads_name(head, name) || exp_reads_name(tail, name),
        E::Tuple(elems) | E::Array { elems, .. } => elems.iter().any(|e| exp_reads_name(e, name)),
        E::Match { input, cases, .. } => {
            if exp_reads_name(input, name) { return true; }
            cases.iter().any(|c| {
                c.guard.as_ref().is_some_and(|g| exp_reads_name(g, name))
                    || c.locals.iter().any(|(_, _, d, _)| d.as_ref().is_some_and(|d| exp_reads_name(d, name)))
                    || stmts_read_name(&c.stmts, name)
                    || exp_reads_name(&c.result, name)
            })
        }
        E::Range { start, step, stop, .. } => {
            exp_reads_name(start, name)
                || step.as_ref().is_some_and(|s| exp_reads_name(s, name))
                || exp_reads_name(stop, name)
        }
        // List comprehension / reduction `f(body for it in r, ...)`. The body
        // and iterator ranges are expressions that can read `name`. The
        // iterators themselves *bind* new names but a shadowed binding only
        // matters at write-position detection, not for reads.
        E::Reduction { body, iterators, .. } => {
            if exp_reads_name(body, name) { return true; }
            iterators.iter().any(|it| exp_reads_name(&it.range, name)
                || it.guard.as_ref().is_some_and(|g| exp_reads_name(g, name)))
        }
    }
}

/// Return true if the local named `name` is *needed* in this case arm.
///
/// MetaModelica `match` declares all locals in one `local …` block before the
/// cases, and the same `case.locals` list is replicated onto every typed case
/// by `infer_case`. Without filtering, the codegen would emit a `let mut …;`
/// declaration for every match-level local in every Rust arm — for
/// `Expression.traverseExpBottomUp` (~40 locals × ~80 arms) that bloats the
/// generated file by tens of thousands of lines and slows `rustc` noticeably.
///
/// A local is "used" if it appears as a read (in the guard, an assignment RHS
/// or no-return call, the case `result`, or another local's default
/// initialiser), or as an assignment target inside the case body. Locals that
/// appear only as pattern bindings are emitted separately by the match
/// pattern, never as a `let mut`, so callers must filter pattern names out
/// before consulting this helper.
///
/// Locals declared with a `Some(default)` initialiser are *always* required
/// because their initialiser may have observable side effects; the caller
/// emits those unconditionally and does not consult this helper for them.
fn case_uses_local_name(case: &typedexp::TypedCase, name: &str) -> bool {
    if case.guard.as_ref().is_some_and(|g| exp_reads_name(g, name)) { return true; }
    if exp_reads_name(&case.result, name) { return true; }
    if stmts_read_name(&case.stmts, name) { return true; }
    for (other_name, _, default, _) in &case.locals {
        if other_name == name { continue; }
        if let Some(d) = default {
            if exp_reads_name(d, name) { return true; }
        }
    }
    let mut written: HashSet<String> = HashSet::new();
    stmts_assigned_var_names(&case.stmts, &mut written);
    written.contains(name)
}

fn plan_tail_call_lowering<'a>(
    typed_stmts: &[typedexp::TypedStmt],
    outputs: &[(String, Ty, Option<Arc<Absyn::Modification>>, bool)],
    fn_short_name: &str,
    is_fallible_fn: bool,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<TailCallPlan> {
    // Multi-output functions return tuples and would require a much more
    // invasive rewrite (each branch must produce a tuple literal). Skip
    // until a real use-case shows up.
    if outputs.len() != 1 { return None; }
    let (out_name, _, _, _) = &outputs[0];

    if !stmts_lowerable_as_tail_expr(typed_stmts, out_name) { return None; }
    if !stmts_have_tail_self_call(typed_stmts, out_name, fn_short_name) { return None; }

    // Suppressing the output's `let mut <n>` declaration only works if the
    // body never reads `<n>` as a value — every reference becomes a
    // "cannot find value" error otherwise. Fold-style functions
    // (`BaseAvlTree.fold`) declare an accumulator output and thread it
    // through self-calls (`outResult := f(left, outResult); outResult :=
    // step(outResult); outResult := f(right, outResult)`); the RHS of each
    // assignment reads the prior value, so we cannot drop the declaration.
    // The tail-call rewrite would still be valid in principle (keep the
    // declaration *and* emit the trailing assignment as a tail expression),
    // but the current `emit_stmts_as_tail` is structured around full
    // suppression; refuse the plan in this case and fall back to the
    // ordinary `let mut out; out = match ...; out` emission, which compiles
    // (no `#[tailcall]` optimisation for accumulator-threading folds).
    if stmts_read_name(typed_stmts, out_name) { return None; }

    // We deliberately do *not* pre-check the body for `?`-emitting calls
    // here. The earlier `body_emits_extra_question_mark` analysis was too
    // conservative: it treated every call whose callee wasn't a
    // known-infallible user function as a `?`-emitter, even though many
    // builtins (`valueEq` → `==`, `referenceEq` as a binary operator,
    // `intMod`, etc.) lower without going through `ctx.q(...)` and never
    // emit `?` at all. That over-approximation was throwing away the
    // bulk of OMC's `List`/`Avl*` tail-recursive functions. We now rely on
    // a *dry-run emission* in `emit_function` to make the final
    // accept/reject decision against the actual emitted text — the only
    // ground truth for what `?` characters the macro will see.
    //
    // A non-exhaustive match outside tail position would be lowered to
    // `unreachable!()` because the macro forbids `bail!`/`?`/`return Err(_)`
    // and the surrounding `T`-typed local can't accept an `Err(...)` value.
    // That replaces MM's "fail()-on-miss" semantics with a Rust panic — a
    // new failure mode the codebase otherwise avoids. Refuse here so the
    // generated code remains panic-free.
    //
    // The same restriction applies to a tail-position match in an
    // *infallible* tail-lowered function: there's no `Result<T>` slot for
    // an `Err(...)` value, so the fallback would again have to panic.
    // Fallible tail-position matches are fine — their fallback is a
    // plain `Err(anyhow::anyhow!(...))` value.
    if body_has_nonexhaustive_nontail_match(typed_stmts, out_name, fn_short_name, is_fallible_fn) {
        return None;
    }

    Some(TailCallPlan { suppressed_out: out_name.clone(), fallible: is_fallible_fn })
}

/// Emit `stmts` as a function body whose value is the value the source
/// would have assigned to `out_name`. The last statement of `stmts` (and,
/// recursively, the last statement of each `If` branch) is consumed and
/// rewritten into expression form. Preceding statements emit normally via
/// [`emit_stmts`].
///
/// `fallible` controls how tail-position leaves are wrapped:
///   * a self-call leaf is emitted bare — its `Result<T>` is the function's
///     return type, so no `?` and no `Ok(...)` is needed.
///   * any *other* tail leaf in a fallible function is wrapped in `Ok(...)`
///     so the surrounding `Result<T>` slot is satisfied.
///   * in an infallible function, both leaf kinds emit verbatim.
///
/// Precondition: `stmts_lowerable_as_tail_expr(stmts, out_name)` is true.
/// Violating it falls into the wildcard arm which emits a `todo!()` so the
/// failure is visible at build time rather than silently producing wrong
/// code.
fn emit_stmts_as_tail<'a>(
    out: &mut String,
    stmts: &[typedexp::TypedStmt],
    out_name: &str,
    self_name: &str,
    fallible: bool,
    indent: &str,
    ctx: &mut GenCtx,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    fresh: &mut u32,
) {
    let Some((last, head)) = stmts.split_last() else {
        writeln!(out, "{indent}todo!(\"tail-call lowering: empty body for `{out_name}`\")").unwrap();
        return;
    };
    emit_stmts(out, indent, head, FailureMode::Function, ctx, env, top_level, fresh);
    match last {
        typedexp::TypedStmt::Assign { lhs: TypedPat::Var(name), rhs } if name == out_name => {
            let s = emit_tail_value_exp(rhs, self_name, fallible, ctx, top_level);
            writeln!(out, "{indent}{s}").unwrap();
        }
        typedexp::TypedStmt::If { cond, then_, elseif, else_ } => {
            // Each branch is itself lowered recursively. We emit
            //   if <cond> { <then branch as tail expr> }
            //   else if <cond_i> { <elseif_i as tail expr> } ...
            //   else { <else branch as tail expr> }
            // Inner indentation matches the surrounding `if {` block. We
            // clone `env` per branch because each branch may locally rebind
            // pattern variables (mirroring what `emit_stmt` does for an
            // ordinary `if`).
            let c = emit_exp(cond, /*is_const=*/false, ctx, top_level);
            let inner = format!("{indent}    ");
            writeln!(out, "{indent}if {c} {{").unwrap();
            let mut tenv = env.clone();
            emit_stmts_as_tail(out, then_, out_name, self_name, fallible, &inner, ctx, &mut tenv, top_level, fresh);
            for (ec, eb) in elseif {
                let cs = emit_exp(ec, /*is_const=*/false, ctx, top_level);
                writeln!(out, "{indent}}} else if {cs} {{").unwrap();
                let mut eenv = env.clone();
                emit_stmts_as_tail(out, eb, out_name, self_name, fallible, &inner, ctx, &mut eenv, top_level, fresh);
            }
            writeln!(out, "{indent}}} else {{").unwrap();
            let mut elenv = env.clone();
            emit_stmts_as_tail(out, else_, out_name, self_name, fallible, &inner, ctx, &mut elenv, top_level, fresh);
            writeln!(out, "{indent}}}").unwrap();
        }
        other => {
            // Should be unreachable if `stmts_lowerable_as_tail_expr` was
            // honoured. Emit a loud `todo!()` rather than fall through to a
            // silently-wrong body.
            writeln!(out, "{indent}todo!(\"tail-call lowering: unsupported last stmt: {:?}\")",
                std::mem::discriminant(other)).unwrap();
        }
    }
}

/// Emit a tail-position expression for the lowering: the self-call is
/// emitted bare (its `Result<T>` is the function's return value, no `?`
/// needed), tail-position `if`-expressions are recursed into, `match`
/// expressions are emitted via [`emit_match`] with `ctx.tail_lowering` set
/// so each case's result is itself emitted in tail context, and everything
/// else is emitted via the normal [`emit_exp`] and wrapped in `Ok(...)` if
/// the enclosing function is fallible.
fn emit_tail_value_exp<'a>(
    exp: &TypedExp,
    self_name: &str,
    fallible: bool,
    ctx: &mut GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> String {
    match exp {
        TypedExp::Call { func, .. } if func == self_name => {
            // Emit the call via the normal path so import/arg formatting is
            // shared, then strip the trailing `?` (only present when the
            // enclosing function is fallible). We *can't* use
            // `QMode::Bare` for the call because that would also propagate
            // into the argument sub-expressions, suppressing `?` on
            // non-tail sub-calls — which would be incorrect.
            //
            // The `tailcall` v2 API requires the recursive call to be
            // wrapped in `tailcall::call!{ … }` so the macro's body
            // rewrite knows to thread this call through the trampoline.
            // An *unwrapped* recursive call would still compile (and the
            // function would still be correct) but the macro would not
            // turn it into a loop, defeating the point of the
            // annotation.
            ctx.with_tail_lowering(None, |ctx| {
                let s = emit_exp(exp, /*is_const=*/false, ctx, top_level);
                let bare = s.strip_suffix('?').map(str::to_owned).unwrap_or(s);
                format!("tailcall::call!{{ {bare} }}")
            })
        }
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            // The condition is *not* in tail position — clear `tail_lowering`
            // so that any self-call inside it goes through the normal
            // fallibility-handling path.
            let c = ctx.with_tail_lowering(None, |ctx| emit_exp(cond, /*is_const=*/false, ctx, top_level));
            let t = emit_tail_value_exp(then_, self_name, fallible, ctx, top_level);
            let mut ei = String::new();
            for (ec, eb) in elseif {
                let cs = ctx.with_tail_lowering(None, |ctx| emit_exp(ec, /*is_const=*/false, ctx, top_level));
                let bs = emit_tail_value_exp(eb, self_name, fallible, ctx, top_level);
                ei.push_str(&format!(" else if ({cs}) {{{bs}}}"));
            }
            let e = emit_tail_value_exp(else_, self_name, fallible, ctx, top_level);
            format!("if ({c}) {{{t}}}{ei} else {{{e}}}")
        }
        TypedExp::Match { .. } => {
            // Signal `emit_match` to lower each case's result (or
            // algorithm-side terminal assign) in tail context. The
            // surrounding match expression's value flows out as the
            // function's return — *no* outer `Ok(...)` wrap, because each
            // case is already responsible for producing `Result<T>` (when
            // fallible) or `T` (infallible).
            ctx.with_tail_lowering(Some((self_name.to_owned(), fallible)), |ctx| {
                emit_exp(exp, /*is_const=*/false, ctx, top_level)
            })
        }
        _ => {
            // Non-control-flow leaf. The body's `?`-freedom has already
            // been verified, so `emit_exp` won't emit `?` here.
            let s = ctx.with_tail_lowering(None, |ctx| emit_exp(exp, /*is_const=*/false, ctx, top_level));
            if fallible { format!("Ok({s})") } else { s }
        }
    }
}

/// Collect `Ty::TypeVar(name)` names referenced by match-arm `local`
/// declarations inside a list of typed statements (recursively walking
/// nested control flow and the cases of every `Match` expression). Used by
/// [`emit_function`] to add module-scope `replaceable type X subtypeof Any`
/// names — those that `resolve_function_type` only sees through a function's
/// input/output types — to the function's generic parameter list when they
/// only appear on a local like `tuple<TraverseFunc, Argument> tup;`. Without
/// this, that local emits as `let mut tup: (.., Argument);` inside a
/// function generic over `<ArgT>` and `Argument` is undefined. See
/// `SCodeUtil.mapFoldStatementExps` for the canonical case.
///
/// We deliberately *do not* harvest type vars from arbitrary subexpressions'
/// `ty()` or from `Call::sig_ty`: builtin generic signatures (e.g.
/// `Array::expand<T>`, `valueEq<T>`) carry placeholder `Ty::TypeVar("T")`
/// parameters that are resolved away at the call site, and exposing them as
/// the caller's generic would create a phantom `T` whose binding cannot be
/// inferred (E0283).
fn collect_type_vars_in_typed_stmts(stmts: &[typedexp::TypedStmt], out: &mut Vec<String>) {
    use typedexp::{TypedStmt, TypedExp};

    fn visit_exp(e: &TypedExp, out: &mut Vec<String>) {
        match e {
            TypedExp::BinOp  { lhs, rhs, .. } => { visit_exp(lhs, out); visit_exp(rhs, out); }
            TypedExp::UnOp   { operand, .. } => visit_exp(operand, out),
            TypedExp::Call   { args, named_args, .. }
            | TypedExp::Constructor { args, named_args, .. }
            | TypedExp::PartEval { args, named_args, .. } => {
                for a in args { visit_exp(a, out); }
                for (_, a) in named_args { visit_exp(a, out); }
            }
            TypedExp::If { cond, then_, elseif, else_, .. } => {
                visit_exp(cond, out); visit_exp(then_, out); visit_exp(else_, out);
                for (c, e) in elseif { visit_exp(c, out); visit_exp(e, out); }
            }
            TypedExp::Cons { head, tail, .. } => { visit_exp(head, out); visit_exp(tail, out); }
            TypedExp::Tuple(v) => for e in v { visit_exp(e, out); },
            TypedExp::Array { elems, .. } => for e in elems { visit_exp(e, out); },
            TypedExp::Match { input, cases, .. } => {
                visit_exp(input, out);
                for case in cases {
                    if let Some(g) = &case.guard { visit_exp(g, out); }
                    for (_, t, default, _) in &case.locals {
                        collect_type_vars_in_ty(t, out);
                        if let Some(d) = default { visit_exp(d, out); }
                    }
                    visit_stmts(&case.stmts, out);
                    visit_exp(&case.result, out);
                }
            }
            TypedExp::Range { start, step, stop, .. } => {
                visit_exp(start, out); visit_exp(stop, out);
                if let Some(s) = step { visit_exp(s, out); }
            }
            TypedExp::Reduction { body, iterators, .. } => {
                visit_exp(body, out);
                for it in iterators {
                    visit_exp(&it.range, out);
                    if let Some(g) = &it.guard { visit_exp(g, out); }
                }
            }
            _ => {}
        }
    }

    fn visit_stmts(stmts: &[TypedStmt], out: &mut Vec<String>) {
        for s in stmts {
            match s {
                TypedStmt::Assign { rhs, .. } => visit_exp(rhs, out),
                TypedStmt::NoRetCall { call } => visit_exp(call, out),
                TypedStmt::If { cond, then_, elseif, else_ } => {
                    visit_exp(cond, out); visit_stmts(then_, out); visit_stmts(else_, out);
                    for (c, body) in elseif { visit_exp(c, out); visit_stmts(body, out); }
                }
                TypedStmt::For { range, body, .. } => { visit_exp(range, out); visit_stmts(body, out); }
                TypedStmt::While { cond, body } => { visit_exp(cond, out); visit_stmts(body, out); }
                TypedStmt::Try { body, else_body } => { visit_stmts(body, out); visit_stmts(else_body, out); }
                TypedStmt::Failure { body } => visit_stmts(body, out),
                _ => {}
            }
        }
    }

    visit_stmts(stmts, out);
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
        let base_short = ctx.shorten(base);
        if !modifications.is_empty() {
            // TODO: emit a wrapper `pub fn {name}(...) -> ... {{ {base}(..., {arg}={value}, ...) }}`
            // that actually applies the modifications. For now we re-export the base
            // unchanged so callers compile; the default-argument overrides are silently
            // ignored, which may produce wrong runtime behaviour for callers that rely
            // on the override (e.g. `pathStringNoQual = pathString(usefq=false)`).
            let mods = modifications.iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(out, "{indent}// FIXME: function alias `{name} = {base}({mods})` is emitted as a plain re-export;").unwrap();
            writeln!(out, "{indent}//        the default-argument modifications are NOT applied at call sites yet.").unwrap();
        }
        writeln!(out, "{indent}{pub_kw}use {base_short} as {alias_name};").unwrap();
        writeln!(out).unwrap();
        return;
    }

    fn absyn_path_to_dotted(p: &Absyn::Path) -> String {
        let raw = match p {
            Absyn::Path::IDENT { name } => name.to_string(),
            Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", absyn_path_to_dotted(path)),
            Absyn::Path::FULLYQUALIFIED { path } => format!(".{}", absyn_path_to_dotted(path)),
        };
        raw.trim_start_matches('.').to_owned()
    }

    let local_members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return,
    };
    // Inherited inputs/outputs/protected components aren't repeated in `c.body`,
    // but we still need their declarations so they end up in env.vars and the
    // function signature. Two forms to handle:
    //
    //   1. `function F extends G ... end F;` — `c.body` is `ClassDef::ClassExtends`
    //      and `flatten_extends` has stored the base class in `node.base_fn`.
    //   2. `function F ... extends G; ... end F;` — `c.body` is `ClassDef::Parts`
    //      with one or more `ClassMember::Extends`. `flatten_extends` has copied
    //      the base's children into `node.children`, but not the original AST
    //      members. We resolve each `Extends.path` against the hierarchy and
    //      merge its component members in declaration order.
    //
    // Without this, an inherited `output array<...> indices;` is not declared
    // in the emitted Rust function and `indices[i] := ...` later falls through
    // to the "indexed assign on non-Array base" branch because env.vars has no
    // entry for the variable.
    let base_members_storage: Vec<MM::ClassMember>;
    let local_names: HashSet<String> = local_members.iter().filter_map(|m| match m {
        MM::ClassMember::Component(cm) => Some(cm.name.clone()),
        _ => None,
    }).collect();
    let mut inherited_components: Vec<MM::ClassMember> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    // Type fallback for inherited components: their types live in the base
    // function's NameNode.children, not ours, so node.children.get(name) below
    // would yield Unknown. We collect them here as we walk the bases.
    let mut inherited_tys: HashMap<String, Ty> = HashMap::new();
    let mut collect_from_class = |base_c: &MM::Class,
                                   base_children: Option<&BTreeMap<String, NameNode<'_>>>,
                                   sink: &mut Vec<MM::ClassMember>,
                                   seen: &mut HashSet<String>,
                                   tys: &mut HashMap<String, Ty>| {
        let bm: &[MM::ClassMember] = match &base_c.body {
            MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
            _ => &[],
        };
        for m in bm {
            if let MM::ClassMember::Component(cm) = m {
                if local_names.contains(&cm.name) || !seen.insert(cm.name.clone()) {
                    continue;
                }
                if let Some(bc) = base_children
                    && let Some(child) = bc.get(&cm.name)
                        && child.ty != Ty::Unknown {
                            tys.insert(cm.name.clone(), child.ty.clone());
                        }
                sink.push(m.clone());
            }
        }
    };
    if let Some(base_fn) = node.base_fn {
        // base_fn has no associated NameNode handle here (we only have the
        // MM::Class). Type fallback for header-form extends still works because
        // `flatten_extends` did copy children into our node — but we keep the
        // call shape uniform.
        collect_from_class(base_fn, None, &mut inherited_components, &mut seen, &mut inherited_tys);
    }
    // Snapshot the point at which only `base_fn` inherited components are in
    // the list. Anything appended after this index came from `node.extends`
    // (the `function F ... extends X; ... end F` form, which lives in
    // ClassMember::Extends rather than ClassDef::ClassExtends) and is *not*
    // reflected in `node.ty`'s inputs/outputs — those need a follow-up merge.
    let base_fn_inherited_count = inherited_components.len();
    // Compute the enclosing package qname so a bare `extends X;` (the typical
    // form when the base function lives in the same package) can be resolved
    // as `<pkg>.X`. `ctx.current_path` includes the surrounding package
    // segments up to but not including this function's own name.
    let enclosing_pkg = if ctx.current_path.is_empty() {
        ctx.top_name.clone()
    } else {
        format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
    };
    for ext in &node.extends {
        // ExtendsMember.path is the base function's qualified name. Try
        // resolution in order: as-is (works for fully-qualified extends),
        // as a sibling of this function (the common case for `extends X;`
        // inside a package member), and finally as a top-level name.
        let dotted = absyn_path_to_dotted(&ext.path);
        let base_node = lookup_node(&dotted, top_level)
            .or_else(|| {
                if enclosing_pkg.is_empty() {
                    None
                } else {
                    lookup_node(&format!("{enclosing_pkg}.{dotted}"), top_level)
                }
            })
            .or_else(|| {
                let last = dotted.rsplit('.').next().unwrap_or(&dotted);
                lookup_node(last, top_level)
            });
        if let Some(bn) = base_node
            && let NodeKind::Class(base_c) = &bn.kind {
                collect_from_class(base_c, Some(&bn.children), &mut inherited_components, &mut seen, &mut inherited_tys);
            }
    }
    let members: &[MM::ClassMember] = if inherited_components.is_empty() {
        local_members
    } else {
        let mut merged = inherited_components.clone();
        merged.extend(local_members.iter().cloned());
        base_members_storage = merged;
        &base_members_storage
    };

    // Use types from the resolved Ty::Function — those were computed by resolve_function_type
    // with the correct type_vars in scope, so type-variable parameters resolve correctly.
    // Child node .ty values are resolved without that context and may be Unknown for ArgT etc.
    let Ty::Function { type_vars, inputs: fn_inputs, output: fn_output, .. } = &node.ty else { return };

    // Build a (component-name → original TypeSpec) lookup. The TypeSpec lets
    // us recover the type *name as written* (e.g. `Key`/`Value`) when emitting
    // a parameter, return, or local — so the signature reads
    // `fn add(inKey: Key, inValue: Value, ...)` instead of the resolved
    // concrete types. The alias still expands correctly via the sibling
    // `pub type Key = …;` declarations.
    let comp_type_specs: HashMap<String, &Absyn::TypeSpec> = members.iter()
        .filter_map(|m| if let MM::ClassMember::Component(cm) = m { Some((cm.name.clone(), cm.type_spec.as_ref())) } else { None })
        .collect();
    let alias_scope = current_scope_children(ctx, top_level);
    // Closure-equivalent: given a component name and its resolved `Ty`,
    // return the alias name if the component's original TypeSpec refers to
    // a sibling type alias in scope; otherwise fall back to `fmt_ty`.
    // (Written as an inline helper rather than a closure so each call site
    // can pass `&mut ctx`.)
    let try_alias = |comp_name: &str, ts_override: Option<&Absyn::TypeSpec>| -> Option<String> {
        let ts = ts_override.or_else(|| comp_type_specs.get(comp_name).copied())?;
        let sc = alias_scope?;
        field_type_alias_name(ts, sc)
    };

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
        // Partial-function aliases lower to a Rust type alias for a `fn`
        // pointer (or an `impl Fn` parameter signature). The aliased type
        // never participates in structural equality, so drop `PartialEq`
        // from the type-parameter bound — keeping it would force every
        // user of the alias to also carry `T: PartialEq`, which fails
        // when the alias is parametric over a callback or function-
        // pointer type.
        let type_params = if all_type_vars.is_empty() {
            String::new()
        } else {
            let bounded: Vec<String> = all_type_vars.iter().map(|v| format!("{v}: Clone")).collect();
            format!("<{}>", bounded.join(", "))
        };
        let ins = fn_inputs.iter()
            .map(|inp| try_alias(&inp.name, None).unwrap_or_else(|| fmt_ty(&inp.ty, ctx)))
            .collect::<Vec<_>>()
            .join(", ");
        // Single-output: the partial function's `output X out` component name
        // gives us its TypeSpec for alias preservation. Multi-output partial
        // functions are uncommon; fall back to the resolved tuple type.
        let output_name = members.iter().find_map(|m| match m {
            MM::ClassMember::Component(cm) if matches!(cm.direction, Absyn::Direction::OUTPUT | Absyn::Direction::INPUT_OUTPUT)
                => Some(cm.name.as_str()),
            _ => None,
        });
        let out_ty = output_name
            .and_then(|n| try_alias(n, None))
            .unwrap_or_else(|| fmt_ty(fn_output, ctx));
        let pub_kw = if node.visibility == MM::Visibility::Public { "pub " } else { "" };
        let ename = escape_ident(name);
        writeln!(out, "{indent}{pub_kw}type {ename}{type_params} = fn({ins}) -> Result<{out_ty}>;").unwrap();
        writeln!(out).unwrap();
        return;
    }

    // Inherited inputs/outputs: `resolve_function_type` for the `Parts + extends X;`
    // form does not walk into the base function's components, so `fn_inputs`
    // omits inherited INPUT params and `fn_output` omits inherited OUTPUTs. We
    // recover both here by walking the already-merged `members` slice.
    //
    // `fn_inputs_owned` holds the merged input list; `merged_output_ty` is the
    // function's return type, recomputed from local + inherited outputs when
    // extends added any. Local-only functions still use `fn_inputs` / `fn_output`
    // unchanged to avoid disturbing well-typed paths.
    let mut merged_inputs: Vec<FunctionInput> = fn_inputs.clone();
    let mut merged_output_tys: Vec<Ty> = Vec::new();
    let mut had_inherited_output = false;
    // Only the *Parts+Extends-member* form needs the merge — the header-form
    // (ClassExtends with `base_fn`) goes through `resolve_function_from_base`,
    // which already produced a complete Ty::Function.
    let extends_member_inherited = &inherited_components[base_fn_inherited_count..];
    if !extends_member_inherited.is_empty() {
        let existing_in: HashSet<String> = merged_inputs.iter().map(|i| i.name.clone()).collect();
        for member in extends_member_inherited {
            let MM::ClassMember::Component(cm) = member else { continue };
            let ty = inherited_tys.get(&cm.name).cloned().unwrap_or(Ty::Unknown);
            match cm.direction {
                Absyn::Direction::INPUT => {
                    if !existing_in.contains(&cm.name) {
                        merged_inputs.push(FunctionInput {
                            name: cm.name.clone(),
                            ty,
                            default: extract_default(&cm.modification),
                        });
                    }
                }
                Absyn::Direction::OUTPUT => {
                    had_inherited_output = true;
                    merged_output_tys.push(ty);
                }
                Absyn::Direction::INPUT_OUTPUT => {
                    had_inherited_output = true;
                    merged_output_tys.push(ty.clone());
                    if !existing_in.contains(&cm.name) {
                        merged_inputs.push(FunctionInput {
                            name: cm.name.clone(),
                            ty,
                            default: extract_default(&cm.modification),
                        });
                    }
                }
                _ => {}
            }
        }
        // If extends contributed outputs, also fold in any local outputs in
        // declaration order so the tuple matches the source.
        if had_inherited_output {
            for member in local_members {
                let MM::ClassMember::Component(cm) = member else { continue };
                if matches!(cm.direction, Absyn::Direction::OUTPUT | Absyn::Direction::INPUT_OUTPUT) {
                    let ty = node.children.get(&cm.name).map(|n| n.ty.clone()).unwrap_or(Ty::Unknown);
                    merged_output_tys.push(ty);
                }
            }
        }
    }
    let merged_output_ty: Ty = if had_inherited_output {
        match merged_output_tys.len() {
            0 => Ty::Unit,
            1 => merged_output_tys.into_iter().next().unwrap(),
            _ => Ty::Tuple(merged_output_tys),
        }
    } else {
        fn_output.as_ref().clone()
    };
    let fn_inputs_eff: &[FunctionInput] = &merged_inputs;
    let fn_output_eff: &Ty = &merged_output_ty;

    let mut all_type_vars = type_vars.clone();
    for inp in fn_inputs_eff.iter() {
        collect_type_vars_in_ty(&inp.ty, &mut all_type_vars);
    }
    collect_type_vars_in_ty(fn_output_eff, &mut all_type_vars);
    let type_params = if all_type_vars.is_empty() {
        String::new()
    } else {
        format!("<{}>", all_type_vars.iter().map(|v| format!("{v}: {DEFAULT_TRAITS}")).collect::<Vec<_>>().join(", "))
    };

    // Mark every parameter `mut`. MetaModelica freely reassigns inputs
    // (e.g. `classPart` rewritten via `assign_variant_field!`), and detecting
    // every shape of reassignment (plain assigns, macro-based field updates,
    // pattern lets that re-bind the name) would require walking the full
    // typed body twice. Since `#![allow(unused_mut)]` covers the case where
    // the body never actually mutates the binding, emitting `mut` everywhere
    // is correct and avoids E0384 across the corpus.
    let params = fn_inputs_eff.iter()
        .map(|inp| {
            let ty_s = try_alias(&inp.name, None)
                .unwrap_or_else(|| fmt_param_ty(&inp.ty, ctx));
            format!("mut {}: {}", escape_ident(&inp.name), ty_s)
        })
        .collect::<Vec<_>>()
        .join(", ");

    // Return type: prefer a sibling type alias when the (single) output's
    // declared TypeSpec is a bare alias reference. Multi-output functions
    // lower to a tuple — substitute element-wise so `(Key, Value)` survives
    // rather than collapsing to the resolved types.
    let output_specs: Vec<(&str, &Absyn::TypeSpec)> = members.iter()
        .filter_map(|m| match m {
            MM::ClassMember::Component(cm) if matches!(cm.direction, Absyn::Direction::OUTPUT | Absyn::Direction::INPUT_OUTPUT)
                => Some((cm.name.as_str(), cm.type_spec.as_ref())),
            _ => None,
        })
        .collect();
    let ret_ty = match (fn_output_eff, output_specs.as_slice()) {
        (_, [(name, _ts)]) => try_alias(name, None).unwrap_or_else(|| fmt_ty(fn_output_eff, ctx)),
        (Ty::Tuple(elems), specs) if elems.len() == specs.len() => {
            let parts: Vec<String> = elems.iter().zip(specs.iter())
                .map(|(t, (n, _))| try_alias(n, None).unwrap_or_else(|| fmt_ty(t, ctx)))
                .collect();
            format!("({})", parts.join(", "))
        }
        _ => fmt_ty(fn_output_eff, ctx),
    };
    let ename = escape_ident(name);

    let pub_kw = if node.visibility == MM::Visibility::Public { "pub " } else { "" };

    // Determine this function's fallibility. The fallibility analysis records
    // *fallible* user functions by fully-qualified MM name; absence means
    // infallible. The FQN we construct here must match the convention used by
    // `fallibility::collect_functions` (dot-separated, top package first).
    //
    // When `ctx.current_fn_qname` is non-empty we are recursing into a
    // function-nested function (e.g. `make_result` inside `minAtomPW`).
    // `fallibility::collect_functions` walks `node.children` recursively, so
    // it records the nested function under its parent function's FQN —
    // `SBFunctions.minAtomPW.make_result`, not `SBFunctions.make_result`.
    // We must mirror that here, otherwise the fallibility lookup misses and
    // the wrapper signature ends up out of sync with the caller's `?`.
    let fn_qname = if !ctx.current_fn_qname.is_empty() {
        format!("{}.{}", ctx.current_fn_qname, name)
    } else if ctx.current_path.is_empty() {
        format!("{}.{}", ctx.top_name, name)
    } else {
        format!("{}.{}.{}", ctx.top_name, ctx.current_path.join("."), name)
    };
    let saved_fn_fallible = ctx.current_fn_fallible;
    ctx.current_fn_fallible = ctx.fallible_functions.contains(&fn_qname);
    let saved_fn_qname = std::mem::replace(&mut ctx.current_fn_qname, fn_qname.clone());
    let is_fallible_fn = ctx.current_fn_fallible;

    // Function-local imports (`import Foo.Bar;` between the signature and
    // `algorithm`) extend the import scope for the body of this function only.
    // Without this, references like `Op.ADD` inside `expandBinary` — which
    // declares `import NFOperator.Op;` locally — fail to resolve and the
    // codegen falls back to emitting the bare identifier.
    let saved_imports = apply_local_imports(node, ctx, top_level);

    // Walk components to find outputs (with names) and protected locals.
    let mut outputs: Vec<(String, Ty, Option<Arc<Absyn::Modification>>, bool)> = Vec::new();
    let mut protected: Vec<(String, Ty, Option<Arc<Absyn::Modification>>, bool)> = Vec::new();
    let mut input_names: HashSet<String> = HashSet::new();
    for inp in fn_inputs_eff.iter() { input_names.insert(inp.name.clone()); }
    let pkg_prefix_for_typespec = if ctx.current_path.is_empty() {
        ctx.top_name.clone()
    } else {
        format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
    };
    for member in members {
        let MM::ClassMember::Component(cm) = member else { continue };
        let child_ty = node.children.get(&cm.name)
            .map(|n| n.ty.clone())
            .filter(|t| *t != Ty::Unknown)
            .or_else(|| inherited_tys.get(&cm.name).cloned().filter(|t| *t != Ty::Unknown))
            // Last-resort: resolve the component's `TypeSpec` directly. This
            // covers inherited components from a `function F extends G` base
            // whose NameNode children were not copied across by
            // `flatten_extends` — e.g. `output Integer outResult` inherited
            // from `BaseAvlSet.keyCompare` into `AvlTreeString.keyCompare`.
            // Without this, the output ends up `Ty::Unknown` and the body
            // emits `let mut outResult; // TODO: local with unresolved type`.
            .or_else(|| {
                let ty = typedexp::resolve_typespec(&cm.type_spec, &all_type_vars, top_level, &pkg_prefix_for_typespec);
                if matches!(ty, Ty::Unknown) { None } else { Some(ty) }
            })
            .unwrap_or(Ty::Unknown);
        match cm.direction {
            Absyn::Direction::OUTPUT | Absyn::Direction::INPUT_OUTPUT =>
                outputs.push((
                    cm.name.clone(),
                    child_ty,
                    cm.modification.clone(),
                    cm.variability == Absyn::Variability::CONST,
                )),
            Absyn::Direction::BIDIR
                if !input_names.contains(&cm.name) => {
                    protected.push((
                        cm.name.clone(),
                        child_ty,
                        cm.modification.clone(),
                        cm.variability == Absyn::Variability::CONST,
                    ));
                }
            _ => {}
        }
    }

    // Include the *function's own name* in `pkg_prefix` so that nested
    // function definitions (e.g. `function is_equal_dim ... end is_equal_dim`
    // declared in `SBSet.new`'s protected section, with `function-instance` qname
    // `SBSet.new.is_equal_dim`) resolve when referenced as a bare name from
    // inside the enclosing function's body — including `function-instance`
    // partial-application call sites like `function is_equal_dim(dim = dim)`.
    // Without this the bare name only resolved against the *parent package*'s
    // scope, which doesn't see the nested function and so `resolve_call_node`
    // returned `None`, dropping the `PartEval` signature and producing a
    // `todo!()` at codegen time.
    let pkg_prefix = if ctx.current_path.is_empty() {
        format!("{}.{}", ctx.top_name, name)
    } else {
        format!("{}.{}.{}", ctx.top_name, ctx.current_path.join("."), name)
    };

    // Protected locals (and inherited outputs whose types weren't reflected in
    // the resolved `Ty::Function` signature) may reference module-level
    // `replaceable type X subtypeof Any` declarations that aren't in scope as
    // an own type-variable of this function. `hierarchy::resolve_type_spec`
    // resolves such names to `Ty::TypeVar("X")`, but the function signature
    // computed by `resolve_function_type` only carries this function's *own*
    // type vars (`class_type_vars(c)`) plus those reached through input/output
    // types — protected components' types are never inspected there.
    //
    // Without this merge, a local like `tuple<TraverseFunc, Argument> tup;` in
    // a function generic over `<ArgT>` (e.g. `SCodeUtil.mapFoldStatementExps`)
    // emits `let mut tup: (..., Argument);` while the function header is still
    // `fn mapFoldStatementExps<ArgT: …>(…)` — `Argument` is then an undefined
    // type. Add every type-var reachable through outputs/protected component
    // types to `all_type_vars` before we compute the final `type_params`.
    for (_, t, _, _) in outputs.iter().chain(protected.iter()) {
        collect_type_vars_in_ty(t, &mut all_type_vars);
    }

    let mut infer_env: HashMap<String, Ty> = HashMap::new();
    for inp in fn_inputs_eff.iter() { infer_env.insert(inp.name.clone(), inp.ty.clone()); }
    for (n, t, _, _) in &outputs { infer_env.insert(n.clone(), t.clone()); }
    for (n, t, _, _) in &protected { infer_env.insert(n.clone(), t.clone()); }

    let alg_items: &[Arc<Absyn::AlgorithmItem>] = match &c.body {
        MM::ClassDef::Parts { algorithms, .. } | MM::ClassDef::ClassExtends { algorithms, .. } => algorithms,
        _ => &[],
    };

    let typed_stmts = typedexp::infer_stmts(alg_items, &mut infer_env, top_level, &pkg_prefix, &all_type_vars);

    // Also collect type vars referenced inside the body — most importantly,
    // those declared on match-arm `local` components like
    // `tuple<TraverseFunc, Argument> tup;`. These are typed by `infer_stmts`
    // against the module's hierarchy and can resolve to `Ty::TypeVar(name)`
    // for module-level `replaceable type X subtypeof Any` declarations that
    // aren't part of this function's own signature.
    collect_type_vars_in_typed_stmts(&typed_stmts, &mut all_type_vars);

    let mut env = LocalEnv::default();
    for inp in fn_inputs_eff.iter() { env.vars.insert(inp.name.clone(), inp.ty.clone()); }
    for (n, t, _, _) in &outputs   { env.vars.insert(n.clone(), t.clone()); }
    for (n, t, _, _) in &protected { env.vars.insert(n.clone(), t.clone()); }
    env.outputs = outputs.iter().map(|(n, _, _, _)| n.clone()).collect();

    // Expose function-level scope on the codegen context so emit_match's
    // per-arm `LocalEnv` (which would otherwise start empty) can recognise
    // re-assignments to outputs/protected components and so `return;` inside
    // a match arm can expand to the same Ok(...) tuple as the function tail.
    ctx.fn_env_vars = env.vars.clone();
    ctx.fn_input_names = fn_inputs_eff.iter().map(|inp| inp.name.clone()).collect();
    // Inputs are always initialised at the outer scope. Locals are added as
    // we emit their declarations below.
    ctx.fn_initialized_vars = ctx.fn_input_names.clone();
    ctx.fn_outputs = env.outputs.clone();
    // An output is "no default" if it has no `= expr` initializer. The
    // Modification struct carries both `elementArgLst` (class modifications)
    // and `eqMod` (the equation form). Only `eqMod` provides a value to
    // initialize the output, so we check that field rather than treating
    // every `Some(modification)` as "has default" — `Option::Some` is common
    // even without an `= expr` initializer (e.g. for `output T x annotation(...);`).
    ctx.fn_outputs_no_default = outputs.iter()
        .filter(|(_, _, modif, _)| match modif {
            None => true,
            Some(m) => matches!(*m.eqMod, Absyn::EqMod::NOMOD),
        })
        .map(|(n, _, _, _)| n.clone())
        .collect();
    ctx.uninit_arrays.clear(); // fresh function scope — no uninitialised arrays yet

    // Infallible functions drop the `Result<>` wrapper — the surrounding code
    // can then call them without `?` and use the value directly. Fallible
    // functions keep `Result<T>`, which propagates anyhow errors via `?`.
    let sig_ret = if is_fallible_fn {
        format!("Result<{ret_ty}>")
    } else {
        ret_ty.clone()
    };

    // Recompute `type_params` using the workspace-wide PartialEq
    // requirement analysis. `ctx.partial_eq_required[fn_qname]` was
    // populated by [`analyze_partial_eq`] before code generation; it
    // captures both direct uses (`==`/`!=` and PartialEq-requiring
    // builtins) and transitive propagation through user-function call
    // sites. Type parameters not in that set are emitted with
    // `Clone + 'static` only, which lets MM callbacks
    // (`Arc<dyn Fn(...) + 'static>`, which is not `PartialEq`) flow through
    // generic forwarders like `List::map3(.., extra_arg, ..)`.
    let type_params = if all_type_vars.is_empty() {
        String::new()
    } else {
        let empty: HashSet<String> = HashSet::new();
        let eq_vars = ctx.partial_eq_required.get(&fn_qname).unwrap_or(&empty);
        // We used to add `'static` only selectively (per-var analysis for
        // `getGlobalRoot`/`setGlobalRoot`/`referenceEq` builtins and for
        // type-vars appearing in `Arc<dyn Fn + 'static>` callback params).
        // That was necessary when callbacks were `&impl Fn` (borrowed,
        // non-`'static`) — blanket `'static` caused E0521.
        //
        // Now that callbacks are `Arc<dyn Fn + 'static>` (owned), all MM
        // runtime values are either primitive (Copy) or owned (Arc/ArcStr/Vec).
        // None carry a lifetime shorter than `'static`, so adding `T: 'static`
        // to every type var is always safe AND necessary for the transitive
        // case: e.g. `allCombinations<T>` calls `applyAndFold<T: 'static>`,
        // which requires `T: 'static` at the call site — a dependency the
        // per-var analysis couldn't detect.
        let bounded: Vec<String> = all_type_vars.iter().map(|v| {
            let mut bounds = vec!["Clone", "'static"];
            if eq_vars.contains(v) { bounds.push("PartialEq"); }
            format!("{v}: {}", bounds.join(" + "))
        }).collect();
        format!("<{}>", bounded.join(", "))
    };

    // Plan tail-call lowering before emitting the body so we can (a) annotate
    // the function with `#[tailcall::tailcall]`, (b) suppress the `let mut
    // <out>;` declaration that would otherwise turn the recursive call into a
    // non-tail expression on the RHS of an assignment, and (c) splice the
    // assignment's RHS in as the function's trailing expression.
    //
    // The `tailcall` crate rewrites *syntactic* tail-position self-calls into
    // a loop. Our generated bodies normally end with `outVar := …; outVar`,
    // which is not a syntactic tail call; the lowering below converts the
    // common `outVar := <expr>;` shape into the equivalent expression form.
    let mut tail_plan = plan_tail_call_lowering(&typed_stmts, &outputs, name, is_fallible_fn, ctx, top_level);

    // `#[tailcall::tailcall]` and `::match_deref::match_deref!{…}` are mutually
    // exclusive. The tailcall macro rewrites tail-position self-calls into a
    // trampoline by walking the body's syntax — but match_deref hides match
    // arms behind a `Deref @ <pat>` rewrite the tailcall macro doesn't see,
    // so any `tailcall::call!{…}` inside such an arm becomes invisible to the
    // rewriter and the function fails to typecheck (raw `T` returned where
    // `Thunk<T>` is expected). When the body contains any match that *needs*
    // match_deref (Arc-crossing scrutinee, string-literal patterns, …), drop
    // the tail-call plan and emit a marker comment so the loss of TCO is
    // visible in the generated source.
    let body_needs_match_deref = stmts_need_match_deref(&typed_stmts, ctx, top_level);
    let tailcall_disabled_for_match_deref = tail_plan.is_some() && body_needs_match_deref;
    if tailcall_disabled_for_match_deref {
        tail_plan = None;
    }

    // Dry-run the tail-lowered body emission so we can validate it against
    // the macro's "no `?` anywhere" constraint *after* all the per-builtin
    // and per-arg emission decisions have been made — which is the only
    // ground truth. The structural analysis in `body_emits_extra_question_mark`
    // is necessarily approximate: it treats every fallible-call site as if
    // it would emit `?`, but several builtins (e.g. `valueEq`, `referenceEq`
    // when used as a binary operator, simple arithmetic) lower without
    // going through `ctx.q(...)` and therefore do not actually produce a
    // `?` even in a fallible enclosing function. Without the dry-run we
    // would refuse the lowering for every recursive function whose body
    // contains such a call — `List::isEqual`, `List::merge`, and most of
    // the tree-walking infallible-after-lowering family.
    //
    // The dry-run is done with cloned `env` / `fresh` so it cannot
    // influence the upcoming real emission. `ctx.qmode` / `tail_lowering`
    // are save/restored by the helpers themselves; `ctx.implicit_modules`
    // accumulates monotonically (any imports the dry-run discovers are
    // also needed by the real emission, whether tail-lowered or not), so
    // letting it grow is correct.
    let mut tail_lowered_body: Option<String> = None;
    if let Some(plan) = &tail_plan {
        let mut scratch = String::new();
        let mut scratch_env = env.clone();
        let mut scratch_fresh: u32 = 0;
        let saved_in_tc_dry = std::mem::replace(&mut ctx.in_tail_lowered_fn, true);
        emit_stmts_as_tail(&mut scratch, &typed_stmts, &plan.suppressed_out, name, plan.fallible, &format!("{indent}    "), ctx, &mut scratch_env, top_level, &mut scratch_fresh);
        ctx.in_tail_lowered_fn = saved_in_tc_dry;
        if scratch.contains('?') {
            // `?` survived the lowering — the macro would reject the body.
            // Drop the plan; the regular non-lowered emission runs below.
            tail_plan = None;
        } else {
            tail_lowered_body = Some(scratch);
        }
    }

    if tail_plan.is_some() {
        writeln!(out, "{indent}#[tailcall::tailcall]").unwrap();
    } else if tailcall_disabled_for_match_deref {
        writeln!(out, "{indent}// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{{…}}` match,").unwrap();
        writeln!(out, "{indent}// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.").unwrap();
    }
    // Set the ambient "we're inside a tail-lowered body" flag so that
    // helpers called during body emission (notably [`emit_match`]) can
    // adjust their output — see the field's doc comment.
    let saved_in_tc = std::mem::replace(&mut ctx.in_tail_lowered_fn, tail_plan.is_some());

    writeln!(out, "{indent}{pub_kw}fn {ename}{type_params}({params}) -> {sig_ret} {{").unwrap();
    let body_indent = format!("{indent}    ");

    // Emit `use` statements for function-local imports — those declared
    // inside this function class via `import Foo.Bar;` (between the
    // signature and `algorithm`). They've been added to `ctx.named` /
    // `ctx.unqual_modules` by [`apply_local_imports`] but the file-level
    // [`GenCtx::use_lines`] doesn't see them. Emitting them as
    // function-scoped `use` statements keeps the rewrite that [`Self::shorten`]
    // performs (e.g. `Op.MUL` → `Op::MUL`) consistent with what's actually
    // in scope at the call site, without polluting the enclosing module.
    for (dotted, local) in &ctx.named {
        if saved_imports.named.contains_key(dotted) { continue; }
        let rust = ctx.dotted_to_rust_path(dotted);
        let last = dotted.rsplit('.').next().unwrap_or(dotted);
        if local == last {
            writeln!(out, "{body_indent}use {rust};").unwrap();
        } else {
            writeln!(out, "{body_indent}use {rust} as {local};").unwrap();
        }
    }
    for module in &ctx.unqual_modules {
        if saved_imports.unqual_modules.contains(module) { continue; }
        let rust = ctx.module_rust_prefix(module);
        writeln!(out, "{body_indent}use {rust}::*;").unwrap();
    }

    // Emit nested `function ... end ...;` definitions as Rust inner-`fn`
    // items declared at the top of the parent function's body.
    //
    // Rationale: MetaModelica nested functions do not close over the
    // enclosing function's locals — they are syntactically scoped to the
    // parent but semantically independent. In Rust, items declared inside
    // a function body have exactly that behaviour: they can be called by
    // name from the surrounding body, but they cannot capture outer locals.
    // Emitting them here also gives every parent function a fresh
    // namespace for its nested functions, so two unrelated parents with a
    // helper called `make_result` do not clash (which is why we don't need
    // the C backend's `omc_<Mod>_<Parent>_<Nested>` name mangling).
    //
    // `populate_from_class_def` already registers each nested
    // `ClassDef` in `node.children`, so `emit_function` recurses with the
    // correct type information for the inner function. Visibility prefixes
    // (`pub fn`) become harmless `pub`s inside a body; the type-resolution
    // information and fallibility state are saved/restored by the
    // recursive `emit_function` call itself.
    if let MM::ClassDef::Parts { members: parent_members, .. } | MM::ClassDef::ClassExtends { members: parent_members, .. } = &c.body {
        // Nested `emit_function` overwrites `ctx.fn_env_vars` / `ctx.fn_outputs`
        // with its own scope and does not restore them. Without this snapshot,
        // the outer body that follows would consult the *inner* function's
        // environment for things like the `Ty::Array` check in
        // `emit_var_segments` — so a `dom[i]` access on the outer function's
        // array would lose the `.borrow()` prefix whenever a nested helper
        // omitted the same local.
        let saved_fn_env_vars = ctx.fn_env_vars.clone();
        let saved_fn_input_names = ctx.fn_input_names.clone();
        let saved_fn_initialized_vars = ctx.fn_initialized_vars.clone();
        let saved_fn_outputs = ctx.fn_outputs.clone();
        let saved_fn_outputs_no_default = ctx.fn_outputs_no_default.clone();
        let saved_uninit_arrays = ctx.uninit_arrays.clone();
        for member in parent_members.iter() {
            if let MM::ClassMember::ClassDef(cdm) = member
                && matches!(&cdm.class_def.restriction, Absyn::Restriction::R_FUNCTION { .. })
                    && let Some(child_node) = node.children.get(&cdm.class_def.name)
                        && let NodeKind::Class(child_class) = &child_node.kind {
                            emit_function(out, &cdm.class_def.name, child_node, child_class, &body_indent, ctx, top_level);
                        }
        }
        ctx.fn_env_vars = saved_fn_env_vars;
        ctx.fn_input_names = saved_fn_input_names;
        ctx.fn_initialized_vars = saved_fn_initialized_vars;
        ctx.fn_outputs = saved_fn_outputs;
        ctx.fn_outputs_no_default = saved_fn_outputs_no_default;
        ctx.uninit_arrays = saved_uninit_arrays;
    }

    for (n, t, modif, is_const_local) in outputs.iter().chain(protected.iter()) {
        // The output `n` is consumed by the tail-call lowering: its declaration
        // would otherwise force `let mut <n>: …;`, but `<n>` never appears in
        // the rewritten body — the RHS of `<n> := <tail-expr>;` becomes the
        // function's trailing expression directly. Suppressing the declaration
        // also avoids an `unused_variables` lint after `unused_mut` is allowed.
        if let Some(plan) = &tail_plan
            && &plan.suppressed_out == n { continue; }
        // When the local's type is unknown, omit the annotation entirely and
        // let Rust infer from the later assignment. `fmt_ty(Ty::Unknown)`
        // produces `/* ? */`, which is not a valid type, so emitting
        // `let mut x: /* ? */;` would fail to parse. Match-arm locals use the
        // same fallback (see "TODO: local with unresolved type" below).
        let is_unknown_ty = matches!(t, Ty::Unknown);
        let ty_s = try_alias(n, None).unwrap_or_else(|| fmt_ty(t, ctx));
        let modif_opt: Option<Arc<Absyn::Modification>> = modif.clone();
        // Carry along the inferred type of the initializer so we can detect a
        // multi-output call assigned into a single-valued local. MetaModelica
        // silently drops the extra outputs in that case; we model it as a
        // tuple destructure (`let (mut x, _, _): (T1, T2, T3) = call;`).
        let init_raw: Option<(String, Ty)> = extract_default_exp(&modif_opt).map(|exp| {
            let typed = typedexp::infer_exp(exp, &infer_env, top_level, &pkg_prefix, &all_type_vars);
            let init_ty = typed.ty();
            (emit_exp(&typed, false, ctx, top_level), init_ty)
        });
        let init: Option<(String, Ty)> = if input_names.contains(n) {
            Some((escape_ident(n).to_string(), t.clone()))
        } else {
            let cloned_s = format!("{}.clone()", escape_ident(n));
            init_raw.filter(|(s, _)| s != &escape_ident(n) && s != &cloned_s)
        };
        let ty_annot = if is_unknown_ty { String::new() } else { format!(": {ty_s}") };
        match (is_const_local, init) {
            (true, Some((s, init_ty))) => {
                ctx.fn_initialized_vars.insert(n.to_string());
                if let Some(line) = emit_multi_output_let(/*is_mut=*/false, n, t, &init_ty, &s, &body_indent, ctx) {
                    out.push_str(&line);
                } else {
                    let s = coerce_assign_expr_pub(s, &init_ty, Some(t));
                    writeln!(out, "{body_indent}let {}{ty_annot} = {s};", escape_ident(n)).unwrap();
                }
            },
            (true, None) => writeln!(out, "{body_indent}let mut {}{ty_annot}; // TODO: local with unresolved type", escape_ident(n)).unwrap(),
            (false, Some((s, init_ty))) => {
                ctx.fn_initialized_vars.insert(n.to_string());
                if let Some(line) = emit_multi_output_let(/*is_mut=*/true, n, t, &init_ty, &s, &body_indent, ctx) {
                    out.push_str(&line);
                } else {
                    let s = coerce_assign_expr_pub(s, &init_ty, Some(t));
                    writeln!(out, "{body_indent}let mut {}{ty_annot} = {s};", escape_ident(n)).unwrap();
                }
            },
            (false, None) => {
                if is_unknown_ty {
                    writeln!(out, "{body_indent}let mut {}; // TODO: local with unresolved type", escape_ident(n)).unwrap();
                } else if let Some(def) = ty_default_init(t) {
                    // Modelica/MetaModelica gives every output and protected
                    // local an implicit initial value equal to its type's
                    // default (Boolean → false, Integer → 0, list → nil, …).
                    // Bodies that only assign the variable on some branches
                    // (a common MM idiom) then rely on that default in the
                    // remaining branches. Without the explicit initialiser
                    // Rust rejects those uses as E0381.
                    ctx.fn_initialized_vars.insert(n.to_string());
                    writeln!(out, "{body_indent}let mut {}{ty_annot} = {def};", escape_ident(n)).unwrap();
                } else {
                    writeln!(out, "{body_indent}let mut {}{ty_annot};", escape_ident(n)).unwrap();
                }
            }
        }
    }

    let mut fresh: u32 = 0;
    // `function F extends G(...) end F;` with no local algorithm section: the
    // body is supposed to come from `G`. We do *not* yet implement this in the
    // codegen — doing it correctly requires:
    //   1. Taking `alg_items` from `node.base_fn`'s body and retyping them
    //      under the derived function's input/output environment (which may
    //      involve type-variable substitution beyond what we currently do).
    //   2. For each modification on a `replaceable package P { constant ... }`
    //      member of the base, substituting references `P.field` inside the
    //      inherited algorithm with the override value. The DiffAlgorithm
    //      module's `printDiffXml`, `printDiffTerminalColor`, `printActual`
    //      are the canonical examples — each refines `partialPrintDiff`'s
    //      `DiffStrings` package with concrete open/close strings.
    // Until both pieces land, emit a `todo!()` so the broken empty body fails
    // loudly at runtime instead of silently returning an uninitialised local.
    // Inherit-from-base detection covers both function-extends forms:
    //
    //   (a) `function F extends G(...) end F;` — `c.body` is `ClassDef::ClassExtends`.
    //       `flatten_extends` sets `node.base_fn` to G.
    //   (b) `function F ... extends G(...); ... end F;` — `c.body` is `ClassDef::Parts`
    //       with a `ClassMember::Extends`. There's no `node.base_fn`; the base
    //       can be looked up in the hierarchy via the Extends member's path.
    //
    // In both cases, when there is *no local algorithm section*, the function
    // body must come from G. We don't yet implement that inheritance, so emit
    // a loud `todo!()` instead of letting the empty body fall through to a
    // bare reference to an uninitialised output local.
    let base_alg_nonempty = |base_c: &MM::Class| matches!(&base_c.body,
        MM::ClassDef::Parts { algorithms, .. } | MM::ClassDef::ClassExtends { algorithms, .. } if !algorithms.is_empty());
    let local_algs_empty = match &c.body {
        MM::ClassDef::Parts { algorithms, .. } | MM::ClassDef::ClassExtends { algorithms, .. } => algorithms.is_empty(),
        _ => true,
    };
    let header_form_inherits_alg = node.base_fn.is_some_and(&base_alg_nonempty);
    let member_form_inherits_alg = !node.extends.is_empty() && node.extends.iter().any(|ext| {
        let dotted = absyn_path_to_dotted(&ext.path);
        let lookup = lookup_node(&dotted, top_level)
            .or_else(|| {
                if enclosing_pkg.is_empty() { None } else { lookup_node(&format!("{enclosing_pkg}.{dotted}"), top_level) }
            })
            .or_else(|| {
                let last = dotted.rsplit('.').next().unwrap_or(&dotted);
                lookup_node(last, top_level)
            });
        matches!(lookup.and_then(|bn| match &bn.kind { NodeKind::Class(bc) => Some(bc), _ => None }),
            Some(bc) if base_alg_nonempty(bc))
    });
    let inherit_from_base_unimplemented = local_algs_empty
        && typed_stmts.is_empty()
        && (header_form_inherits_alg || member_form_inherits_alg);
    match &c.body {
        MM::ClassDef::Parts { external: Some(ext), .. } => {
            // If the C function has a hand-written Rust replacement (see
            // `external_c_calls::external_c_impl_path`), delegate to it; the
            // arguments are the function's input parameters in declaration
            // order. Otherwise fall back to a `todo!()` placeholder so
            // calling the function panics loudly at runtime rather than
            // returning garbage.
            let Absyn::ExternalDecl { funcName, output_, args, .. } = &*ext.decl;
            let func_name = funcName.as_deref().unwrap_or("");
            if let Some(rust_path) = crate::external_c_calls::external_c_impl_path(func_name) {
                let arg_names: Vec<String> = collect_external_arg_names(args);
                let arg_list = arg_names.iter()
                    .map(|n| format!("{}.clone()", escape_ident(n)))
                    .collect::<Vec<_>>()
                    .join(", ");
                if let Some(out_cref) = output_ {
                    let out_name = component_ref_simple_name(out_cref);
                    let q = if ctx.current_fn_fallible { "?" } else { "" };
                    writeln!(out, "{body_indent}{} = {rust_path}({arg_list}){q};", escape_ident(&out_name)).unwrap();
                } else {
                    let q = if ctx.current_fn_fallible { "?" } else { "" };
                    writeln!(out, "{body_indent}{rust_path}({arg_list}){q};").unwrap();
                }
            } else {
                writeln!(out, "{body_indent}todo!(); // {:?}", ext).unwrap();
            }
        },
        _ if inherit_from_base_unimplemented => {
            writeln!(out, "{body_indent}// TODO: inherit algorithm from base function via `extends` clause.").unwrap();
            writeln!(out, "{body_indent}// Requires substituting `replaceable package` constant overrides into the inherited body.").unwrap();
            writeln!(out, "{body_indent}todo!(\"function `{name}` inherits its algorithm via `extends` — not yet implemented\")").unwrap();
            writeln!(out, "{indent}}}").unwrap();
            writeln!(out).unwrap();
            ctx.current_fn_fallible = saved_fn_fallible;
            ctx.current_fn_qname = saved_fn_qname;
            ctx.in_tail_lowered_fn = saved_in_tc;
            restore_imports(ctx, &saved_imports);
            return;
        }
        _ => {
            // When the tail-call lowering survived the dry-run, splice the
            // pre-emitted body text straight in. We never re-run
            // `emit_stmts_as_tail` here because that would do all the
            // per-builtin work twice — wasteful, and a non-monotonic ctx
            // change in the recursive emitter could differ between runs.
            if let Some(body_str) = tail_lowered_body.take() {
                out.push_str(&body_str);
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
                ctx.current_fn_fallible = saved_fn_fallible;
                ctx.current_fn_qname = saved_fn_qname;
                ctx.in_tail_lowered_fn = saved_in_tc;
                restore_imports(ctx, &saved_imports);
                return;
            }
            emit_stmts(out, &body_indent, &typed_stmts, FailureMode::Function, ctx, &mut env, top_level, &mut fresh);
        }
    };

    let tail_val: String = match outputs.len() {
        0 => "()".to_owned(),
        1 => escape_ident(&outputs[0].0),
        _ => {
            let parts: Vec<String> = outputs.iter().map(|(n, _, _, _)| escape_ident(n)).collect();
            format!("({})", parts.join(", "))
        }
    };
    if is_fallible_fn {
        writeln!(out, "{body_indent}Ok({tail_val})").unwrap();
    } else {
        writeln!(out, "{body_indent}{tail_val}").unwrap();
    }
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();
    ctx.current_fn_fallible = saved_fn_fallible;
    ctx.current_fn_qname = saved_fn_qname;
    ctx.in_tail_lowered_fn = saved_in_tc;
    restore_imports(ctx, &saved_imports);
}

// ── Expression and pattern emission ──────────────────────────────────────────

/// True when `ty` references any `Ty::TypeVar` — directly or through a
/// container/Generic/Function shape. Used by [`fnptr!`] emission to decide
/// whether the closure parameter type can be spelled at the caller (concrete
/// types) or must be left as `_` for inference (type vars from the callee's
/// scope that are not in scope here).
/// Map a function name that collides with a Rust language construct (macro,
/// keyword, prelude type) to the qualified runtime path that actually denotes
/// the MetaModelica builtin. Only applies when the resolved FQN is the bare
/// builtin name at top level — qualified user references like
/// `MyMod.print(...)` are left alone.
fn remap_shadowed_builtin_path(var_str: &str, resolved_qname: Option<&str>) -> String {
    let qname = match resolved_qname { Some(q) => q, None => return var_str.to_owned() };
    if var_str != qname { return var_str.to_owned(); }
    match qname {
        // `print!` is a Rust macro; the function value lives in `metamodelica::print`.
        "print" => "metamodelica::print".to_owned(),
        _ => var_str.to_owned(),
    }
}

fn ty_mentions_typevar(ty: &Ty) -> bool {
    let mut tvs = Vec::new();
    crate::hierarchy::collect_type_vars_in_ty(ty, &mut tvs);
    !tvs.is_empty()
}

/// Resolve a top-level dotted name from the current emission context AND return the
/// fully-qualified path that succeeded. Mirrors `resolve_fully_qualified` but also
/// hands back the resolved path so callers can use the *target's* package as the
/// `pkg_prefix` when re-typing its default expression. Without that, an inner
/// reference to a sibling constant of the target would fail to resolve.
fn resolve_with_path<'a>(prefix_dotted: &str, ctx: &GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> Option<(String, &'a NameNode<'a>)> {
    if let Some(n) = lookup_node(prefix_dotted, top_level) {
        return Some((prefix_dotted.to_owned(), n));
    }
    let mut scope_parts: Vec<&str> = std::iter::once(ctx.top_name.as_str())
        .chain(ctx.current_path.iter().map(|s| s.as_str()))
        .collect();
    while !scope_parts.is_empty() {
        let path = format!("{}.{}", scope_parts.join("."), prefix_dotted);
        if let Some(n) = lookup_node(&path, top_level) {
            return Some((path, n));
        }
        scope_parts.pop();
    }
    for (fq, local) in &ctx.named {
        if prefix_dotted == local {
            if let Some(n) = lookup_node(fq, top_level) {
                return Some((fq.clone(), n));
            }
        } else if let Some(rest) = prefix_dotted.strip_prefix(&format!("{}.", local)) {
            let path = format!("{}.{}", fq, rest);
            if let Some(n) = lookup_node(&path, top_level) {
                return Some((path, n));
            }
        }
    }
    for unq in &ctx.unqual_modules {
        let path = format!("{}.{}", unq, prefix_dotted);
        if let Some(n) = lookup_node(&path, top_level) {
            return Some((path, n));
        }
    }
    None
}

/// Emit `exp` as a Rust expression of type `&'static str` suitable for use as
/// either:
///   * an operand to `const_str::equal!` inside a `pub const Bool` initializer
///     of a `==` / `!=` of strings, or
///   * the initializer of a top-level `pub const FOO: &'static str = …` item.
///
/// Both use sites need a const-evaluable `&str` expression. `arcstr::ArcStr`
/// doesn't fit because `Deref<Target=str>` is not a `const` trait — so we emit
/// raw `&str` from literals and from sibling string `pub const`s (which are
/// themselves `&'static str`; see [`emit_node`]).
///
/// Coverage today:
///   * `Lit::Str(s)`        → `"escaped"` — a raw `&str` literal (no `literal!`
///                            wrapping; that's the `ArcStr` form used only for
///                            non-const emission in [`emit_exp`]).
///   * `Var` (CONST `Ty::Str` cref) → the cref emitted by its full name. The
///                            referenced item is `pub const X: &'static str =
///                            "…";`, so a Rust path expression of the right
///                            type works directly.
///   * `If / elseif / else` (Ty::Str) → lowered as a Rust `if/else if/else`
///                            chain, with each branch recursively emitted as
///                            a `&'static str` and the condition emitted in
///                            `is_const=true` context. Rust const-eval handles
///                            `if` over const conditions since 1.46, so this
///                            preserves the named `isWindows`/`is64Bit` refs
///                            in the declaration text rather than collapsing
///                            to the live arm.
///
/// Returns `None` if any sub-expression doesn't fit one of the above shapes
/// (string concat, runtime call, …) — the caller emits `compile_error!` so
/// the unhandled case is visible at the use site.
fn emit_const_str_operand<'a>(exp: &TypedExp, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> Option<String> {
    match exp {
        TypedExp::Lit(Lit::Str(s)) => Some(format!("{:?}", unescape_mm_string(s))),
        TypedExp::Var { name, segments, ty, .. } if segments.iter().all(|s| s.subscripts.is_empty()) => {
            let node = resolve_fully_qualified(name, ctx, top_level)?;
            let NodeKind::Component(comp) = &node.kind else { return None };
            if comp.variability != Absyn::Variability::CONST { return None; }
            if node.ty != Ty::Str { return None; }
            Some(emit_var(name, segments, ty, ctx, top_level))
        }
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            // Lower to a Rust `if … else if … else` chain so the condition
            // identifiers (e.g. `isWindows`) show up in the emitted declaration
            // instead of collapsing to whichever literal the build happens to
            // resolve. The condition is emitted in `is_const=true` context so
            // any nested string compare goes through the same `const_str::equal!`
            // path as the top-level boolean `==` lowering.
            let c = emit_exp(cond, /*is_const=*/true, ctx, top_level);
            let t = emit_const_str_operand(then_, ctx, top_level)?;
            let mut out = format!("if {c} {{ {t} }}");
            for (ec, eb) in elseif {
                let ec_s = emit_exp(ec, /*is_const=*/true, ctx, top_level);
                let eb_s = emit_const_str_operand(eb, ctx, top_level)?;
                out.push_str(&format!(" else if {ec_s} {{ {eb_s} }}"));
            }
            let e = emit_const_str_operand(else_, ctx, top_level)?;
            out.push_str(&format!(" else {{ {e} }}"));
            Some(out)
        }
        _ => None,
    }
}

/// True iff `name`/`segments` resolve to a `constant String` package item in
/// the hierarchy — i.e. a Rust `pub const FOO: &'static str = …` item that
/// `emit_node` will emit. Used by `emit_exp`'s `Var` arm to decide when a
/// runtime use site needs to wrap the reference in `arcstr::literal!(…)` to
/// recover an `ArcStr` (the type the surrounding generated code expects for
/// string values). Returns false for local string variables, function inputs,
/// match-arm pattern bindings, and non-constant components.
fn is_const_str_cref<'a>(name: &str, segments: &[CrefSegment], ctx: &GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> bool {
    // Local-scope short-circuit: a bare name that's an entry in `fn_env_vars`
    // is a function-local binding, not a package-level pub const.
    if segments.len() <= 1 {
        let first: &str = segments.first().map(|s| s.name.as_str()).unwrap_or(name);
        if ctx.fn_env_vars.contains_key(first) { return false; }
    }
    let Some(node) = resolve_fully_qualified(name, ctx, top_level) else { return false };
    let NodeKind::Component(comp) = &node.kind else { return false };
    comp.variability == Absyn::Variability::CONST && node.ty == Ty::Str
}

/// Fold a `TypedExp` to a literal `String` at codegen time when it is a chain of
/// constants that bottoms out in string literals (with `If` branching folded
/// through). Returns `None` if any node along the way is not foldable — callers
/// should treat that as "not const-evaluable" and emit a hard error rather than
/// silently producing broken code, since ArcStr's `Deref` is not a `const`
/// trait and runtime string equality cannot appear in a `const` initializer.
///
/// Coverage today: `Lit::Str`, references to a sibling/imported `constant String`,
/// and `if/elseif/else` whose conditions fold via [`resolve_const_bool`].
/// Anything else (string concat, function calls, ...) is intentionally a `None`
/// — extend as the need arises.
fn resolve_const_str<'a>(exp: &TypedExp, ctx: &GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> Option<String> {
    match exp {
        TypedExp::Lit(Lit::Str(s)) => Some(unescape_mm_string(s)),
        TypedExp::Var { name, segments, .. } if segments.iter().all(|s| s.subscripts.is_empty()) => {
            let (path, node) = resolve_with_path(name, ctx, top_level)?;
            let NodeKind::Component(comp) = &node.kind else { return None };
            if comp.variability != Absyn::Variability::CONST { return None; }
            if node.ty != Ty::Str { return None; }
            let default = extract_default_exp(&comp.modification)?;
            let parent_pkg = path.rsplit_once('.').map(|(p, _)| p.to_owned()).unwrap_or_default();
            let typed = typedexp::infer_exp(default, &HashMap::new(), top_level, &parent_pkg, &[]);
            resolve_const_str(&typed, ctx, top_level)
        }
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            let c = resolve_const_bool(cond, ctx, top_level)?;
            if c { return resolve_const_str(then_, ctx, top_level); }
            for (ec, eb) in elseif {
                let cb = resolve_const_bool(ec, ctx, top_level)?;
                if cb { return resolve_const_str(eb, ctx, top_level); }
            }
            resolve_const_str(else_, ctx, top_level)
        }
        _ => None,
    }
}

/// Fold a `TypedExp` to a literal `bool` at codegen time. Mirrors
/// [`resolve_const_str`] in structure. Used both to choose the live branch of an
/// `If` when folding strings, and (for completeness) when a `BinOpKind::Eq` /
/// `BinOpKind::NEq` of strings appears in a `const Boolean` initializer.
///
/// Coverage today: `Lit::Bool`, references to a sibling/imported `constant
/// Boolean`, `not` on a foldable bool, string `==` / `!=`, and `if/elseif/else`.
/// Integer / real comparisons, `and` / `or` short-circuits, and other binops are
/// intentionally `None` — extend as the need arises.
fn resolve_const_bool<'a>(exp: &TypedExp, ctx: &GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> Option<bool> {
    match exp {
        TypedExp::Lit(Lit::Bool(b)) => Some(*b),
        TypedExp::Var { name, segments, .. } if segments.iter().all(|s| s.subscripts.is_empty()) => {
            let (path, node) = resolve_with_path(name, ctx, top_level)?;
            let NodeKind::Component(comp) = &node.kind else { return None };
            if comp.variability != Absyn::Variability::CONST { return None; }
            if node.ty != Ty::Bool { return None; }
            let default = extract_default_exp(&comp.modification)?;
            let parent_pkg = path.rsplit_once('.').map(|(p, _)| p.to_owned()).unwrap_or_default();
            let typed = typedexp::infer_exp(default, &HashMap::new(), top_level, &parent_pkg, &[]);
            resolve_const_bool(&typed, ctx, top_level)
        }
        TypedExp::UnOp { op: UnOpKind::Not, operand, .. } => {
            Some(!resolve_const_bool(operand, ctx, top_level)?)
        }
        TypedExp::BinOp { op, lhs, rhs, .. } if lhs.ty() == Ty::Str => {
            match op {
                BinOpKind::Eq | BinOpKind::NEq => {
                    let l = resolve_const_str(lhs, ctx, top_level)?;
                    let r = resolve_const_str(rhs, ctx, top_level)?;
                    Some(if *op == BinOpKind::Eq { l == r } else { l != r })
                }
                _ => None,
            }
        }
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            let c = resolve_const_bool(cond, ctx, top_level)?;
            if c { return resolve_const_bool(then_, ctx, top_level); }
            for (ec, eb) in elseif {
                let cb = resolve_const_bool(ec, ctx, top_level)?;
                if cb { return resolve_const_bool(eb, ctx, top_level); }
            }
            resolve_const_bool(else_, ctx, top_level)
        }
        _ => None,
    }
}

/// Rust binary-operator precedence levels, used by [`emit_exp`] to decide
/// when an operand needs parenthesization. Higher numbers bind tighter.
/// Matches Rust's published precedence table (so re-parsing the emitted
/// expression keeps the original associativity).
fn binop_prec(op: BinOpKind) -> u8 {
    match op {
        BinOpKind::Or                                                                 => 1,
        BinOpKind::And                                                                => 2,
        BinOpKind::Eq | BinOpKind::NEq
        | BinOpKind::Lt | BinOpKind::LEq | BinOpKind::Gt | BinOpKind::GEq             => 3,
        BinOpKind::Add | BinOpKind::Sub                                               => 4,
        BinOpKind::Mul | BinOpKind::Div                                               => 5,
        // `Pow` is lowered to `.powf(..)`, a method call whose receiver is
        // already grouped; treat it as the tightest level so callers don't
        // double-parenthesize it.
        BinOpKind::Pow                                                                => 6,
    }
}

/// Decode MetaModelica string escape sequences (per BaseModelica_Lexer.g
/// `SESCAPE`) into their character values. The lexer preserves escapes
/// verbatim in the token (so `"\n"` is stored as the two chars `\` + `n`)
/// for source-length tracking; this helper recovers the semantic string
/// value so callers can hand it to Rust's `{:?}` formatter without producing
/// a Rust literal that contains literal backslashes.
///
/// Recognised escapes: `\' \" \\ \? \a \b \f \n \r \t \v`.
/// Anything else (including invalid escape sequences) is left as-is — the
/// lexer in OMC emits a warning for these and treats them as a literal `\`,
/// which is what we do here.
fn unescape_mm_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            None => out.push('\\'),
            Some(n) => {
                let decoded = match n {
                    '\'' => '\'',
                    '"'  => '"',
                    '\\' => '\\',
                    '?'  => '?',
                    'a'  => '\x07',
                    'b'  => '\x08',
                    'f'  => '\x0c',
                    'n'  => '\n',
                    'r'  => '\r',
                    't'  => '\t',
                    'v'  => '\x0b',
                    other => {
                        // Unknown escape — preserve as `\X` (matches OMC lexer's
                        // warning behaviour of treating `\` as a literal char).
                        out.push('\\');
                        out.push(other);
                        continue;
                    }
                };
                out.push(decoded);
            }
        }
    }
    out
}

fn emit_exp<'a>(exp: &TypedExp, is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    match exp {
        TypedExp::Lit(Lit::Int(v))  => v.to_string(),
        // MetaModelica `Real` literal — wrap in `metamodelica::Real` (alias for
        // `OrderedFloat<f64>`) so values containing Real can derive `Ord` / `Eq` / `Hash`.
        // We use the tuple-struct constructor (`OrderedFloat(...)`) instead of
        // `Real::from(...)` because the latter is not a `const fn`, and Real
        // literals appear in `static`/`const` initializers.
        TypedExp::Lit(Lit::Real(v)) => format!("metamodelica::OrderedFloat({v}_f64)"),
        TypedExp::Lit(Lit::Str(v))  => {
            let escaped = format!("{:?}", unescape_mm_string(v));
            format!("literal!({escaped})")
        }
        TypedExp::Lit(Lit::Bool(v)) => v.to_string(),

        TypedExp::Var { name, segments, ty, .. } => {
            let mut var_str = emit_var(name, segments, ty, ctx, top_level);
            // If this reference resolves to a `pub const fn` / `pub fn` getter
            // emitted by `emit_node` for a non-Sync constant (see
            // `collect_const_fn_getters` + the const-emittable/non-Sync branch),
            // we must call it to get the value. Append `()` to `var_str` and let
            // the existing `.clone()` fallthrough emit `name().clone()` — calling
            // a getter that returns owned `T` and then cloning is a no-op clone
            // on an owned value (the consumer of this expression expects an
            // owned value the same way it would from a plain `pub static`).
            //
            // Resolution: the symbol may be referenced by bare name (resolved
            // through the scope chain) or by a fully-qualified dotted path.
            // We use the dotted segments first if present, otherwise the bare
            // name. Pattern bindings shadowing a const are not a concern here:
            // `const_fn_getters` only contains FQNs of constants at module/
            // class scope, and `resolve_call_qname` does not promote shadowed
            // first-segment locals (see its `fn_env_vars` check).
            let first_seg_is_local = segments.first()
                .map(|s| ctx.fn_env_vars.contains_key(&s.name))
                .unwrap_or(false);
            if !first_seg_is_local {
                let lookup_name: String = if !segments.is_empty() {
                    segments.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".")
                } else {
                    name.clone()
                };
                if let Some(qn) = resolve_call_qname(&lookup_name, ctx, top_level)
                    && ctx.const_fn_getters.contains(&qn)
                {
                    var_str = format!("{var_str}()");
                }
            }
            // typedexp::infer_exp only consults the *first* segment when typing
            // a dotted CREF (see `Absyn::Exp::CREF` arm), so a path like
            // `Util.tuple21` carries `ty = Ty::Unknown` even though the path
            // resolves to a function in the hierarchy. Promote those references
            // here by re-resolving the full dotted name; this lets the
            // `Ty::Function` arm below pick up the infallible fnptr! wrapping
            // and avoids the spurious `.clone()` fallthrough that would
            // otherwise treat a function path like a value.
            //
            // We only attempt the promotion when the inferred ty is Unknown
            // and the source has dotted form with more than one segment (a
            // genuinely qualified path). Anything that already inferred to a
            // concrete ty (record value, local var, etc.) is left alone.
            //
            // We deliberately exclude single-segment bare references: those
            // are either local pattern bindings (whose typedexp-derived type
            // is Ty::Unknown because `pat_bindings` doesn't propagate the
            // scrutinee type) or top-level value/function references that
            // typedexp already classified. Promoting a bare name would lift
            // a local pattern binding into a same-named imported function
            // (e.g. `tell` bound by `case BT_FILE_TEXT(tell=tell)` getting
            // resolved to `File.tell` because `Tpl.mo` imports `File`).
            // Additionally skip when the first segment is a known local
            // binding even at length>1, in case a future change adds field
            // access on a pattern-bound record.
            let first_seg_is_local = segments.first()
                .map(|s| ctx.fn_env_vars.contains_key(&s.name))
                .unwrap_or(false);
            let promoted_ty: Option<Ty> = if matches!(ty, Ty::Unknown) && segments.len() > 1 && !first_seg_is_local {
                let dotted: String = segments.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".");
                resolve_call_qname(&dotted, ctx, top_level)
                    .and_then(|q| lookup_node(&q, top_level).map(|n| n.ty.clone()))
                    .filter(|t| matches!(t, Ty::Function { .. } | Ty::FunctionAlias { .. }))
            } else {
                None
            };
            let effective_ty: &Ty = promoted_ty.as_ref().unwrap_or(ty);

            // Try to resolve this reference to a fully-qualified function name
            // in the hierarchy. `Ty::Function::name` is only populated for
            // `partial function` declarations (callback signature aliases), so
            // for *concrete* function references we have to derive the FQN
            // from the variable's path. We use the dotted segments first; if
            // empty, fall back to the local name resolved against the
            // surrounding scope (mirrors what `emit_var` does for bare names).
            let resolved_fn_qname: Option<String> = match effective_ty {
                Ty::Function { .. } | Ty::FunctionAlias { .. } => {
                    let lookup_name: String = if !segments.is_empty() {
                        segments.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".")
                    } else {
                        name.clone()
                    };
                    resolve_call_qname(&lookup_name, ctx, top_level)
                }
                _ => None,
            };

            // A reference is infallible if:
            //   (a) it resolves to a user-defined function that the fallibility
            //       analysis classified as infallible, OR
            //   (b) it is one of the builtins that `is_infallible_builtin`
            //       recognises (e.g. `valueEq`, `intEq`). Builtins are not in
            //       `top_level`, so `resolve_call_qname` returns `None` for
            //       them; we fall back to checking the bare `name` directly.
            //       When the name *does* resolve (qualified path in `top_level`),
            //       the qualified form is checked against both sources so that
            //       e.g. `MetaModelica.valueEq` still matches.
            // When the name resolves to a user function in the hierarchy, that
            // function shadows any same-named builtin: do not consult the
            // builtin table. Only fall back to the builtin classification
            // when the name does not resolve to a user-defined function.
            let infallible_ref = match resolved_fn_qname.as_deref() {
                Some(q) => ctx.is_known_infallible_user_fn(q, top_level),
                None => is_infallible_builtin(name),
            };

            // Some MetaModelica builtins share a name with a Rust language
            // construct (`print` is a macro, `String` is a struct, etc.).
            // Emitting the bare identifier as a value resolves to the Rust
            // construct rather than the runtime function. Remap such names
            // to their qualified runtime path before fnptr! wrapping.
            let var_str = remap_shadowed_builtin_path(&var_str, resolved_fn_qname.as_deref());
            match effective_ty {
                // Infallible concrete function used as a value. Wrap with
                // `fnptr!(path, ArgTy1, …)` so the closure satisfies the
                // surrounding `fn(...) -> Result<T>` slot (see the macro doc
                // in `metamodelica::fnptr!`).
                Ty::Function { inputs, .. } if infallible_ref => {
                    // The closure synthesized by `fnptr!` annotates each
                    // parameter with its declared type. Concrete types are
                    // emitted as-is; type variables from the function's own
                    // signature (e.g. `T1`, `T2` on a generic `tuple21`) are
                    // *not* in scope at the caller, so we emit `_` for any
                    // input type that mentions a type variable and let Rust
                    // infer it from the surrounding higher-order context.
                    let input_tys: Vec<String> = inputs.iter().map(|i| {
                        if ty_mentions_typevar(&i.ty) {
                            "_".to_owned()
                        } else {
                            fmt_ty(&i.ty, ctx)
                        }
                    }).collect();
                    if input_tys.is_empty() {
                        format!("fnptr!({var_str})")
                    } else {
                        format!("fnptr!({}, {})", var_str, input_tys.join(", "))
                    }
                }
                // Anonymous function types resolve to `impl Fn(...)` in
                // parameter position (see `fmt_param_ty`). `impl Fn` is not
                // `Copy`, so passing the variable by value moves it; once
                // moved the binding can't be used again (which trips up the
                // very common pattern of forwarding a callback through a
                // loop or to multiple helper calls). Pass it by reference
                // instead — `&F: Fn` when `F: Fn`, so receivers that also
                // declare `impl Fn(...)` accept the borrow transparently.
                //
                // The `&` is only correct for *callback parameters* whose
                // type is genuinely anonymous (no resolved FQN). Concrete
                // function references — which we just resolved into
                // `resolved_fn_qname` — are `fn` items (zero-sized, `Copy`)
                // and should be passed by value, just like the named
                // `partial function` aliases below. A free MetaModelica
                // builtin like `stringEq` that the hierarchy doesn't expose
                // as a node also falls into the "fn item" group; we
                // distinguish callback parameters by their presence in the
                // surrounding function's variable environment (`fn_env_vars`
                // is populated for inputs/protected locals — see
                // `emit_function`).
                //
                // Note: we no longer prefix `&` here. Input params are now
                // `Arc<dyn Fn + 'static>` (see `fmt_param_ty`). We emit
                // `.clone()` on every reference to an Arc<dyn Fn> binding so
                // that the original binding is not moved when it appears in a
                // match scrutinee tuple (e.g. `(list, inCompFunc, ...)`) — the
                // `.clone()` leaves the original binding alive for subsequent
                // uses in match-arm bodies. At call sites,
                // `emit_call_arg_with_formal` receives the already-cloned raw
                // string and forwards it as-is (no double `.clone()`).
                // Anonymous-function-typed INPUT PARAMETER: `inCompFunc`, `inMapFunc`, etc.
                // These are `Arc<dyn Fn + 'static>` (see `fmt_param_ty`).  Emit `.clone()`
                // on every reference so the binding is not moved into a match-scrutinee
                // tuple or a for-loop call and thus unavailable for subsequent uses.
                //
                // We detect callback parameters by their presence in `fn_env_vars` (which
                // holds ALL local bindings: inputs, outputs, protected).  We do NOT gate
                // on `resolved_fn_qname.is_none()` because function parameters ARE
                // registered as child nodes in the hierarchy (e.g. the FQN
                // `List.insertListSorted1.inCompFunc` resolves), so the qname is often
                // Some — but that does not make the parameter a concrete fn-item (Copy).
                // The guard below is therefore just the `fn_env_vars` membership check.
                Ty::Function { name: None, .. }
                    if ctx.fn_env_vars.contains_key(&name.split('.').next().unwrap_or(name).to_owned()) =>
                    format!("{var_str}.clone()"),
                // Nested `partial function` aliases (declared inside another
                // function — see `nested_partial_aliases`) are stored as
                // `Arc<dyn Fn(...) + 'static>` by `fmt_param_ty`, not as
                // bare `fn(...)` pointers. They share the same move-hazard
                // as the anonymous-callback case above: a pattern binding
                // for one is `&Arc<dyn Fn>` under `match_deref!` (or owned
                // `Arc<dyn Fn>` outside), so forwarding it to another call
                // without `.clone()` either passes a reference (wrong type)
                // or moves the binding (preventing further use). Emit
                // `.clone()` so the right `<Arc<T>>::clone` impl fires.
                Ty::Function { name: Some(qn), .. }
                    if ctx.nested_partial_aliases.contains(qn)
                    && ctx.fn_env_vars.contains_key(&name.split('.').next().unwrap_or(name).to_owned()) =>
                    format!("{var_str}.clone()"),
                // Other named `partial function` aliases (e.g. top-level
                // `KeyEq`) lower to a concrete `fn(...)` pointer — that
                // *is* `Copy`, so the move hazard above doesn't apply and
                // we keep emitting the bare identifier. `Ty::FunctionAlias`
                // (re-export aliases) is the same case. Concrete function
                // references (resolved through the hierarchy above) follow
                // the same path.
                Ty::Function { .. } | Ty::FunctionAlias { .. } => var_str,
                // Inside a `const` initializer Rust can't call `.clone()` —
                // `Clone` isn't a stable const trait yet (E0658 + "Clone is
                // not yet stable as a const trait"). For const operands of
                // primitive Copy types (Bool, Int, Real) the clone is a
                // no-op anyway, and for ArcStr the value can be referenced
                // directly. Suppress the clone in const context; the call
                // sites we know about (const initializers using other
                // const values) don't need it.
                _ if is_const => var_str,
                // Runtime use of a string `pub const` cref: the item is
                // `pub const X: &'static str = …` (see `emit_node` for the
                // rationale), but the surrounding generated code expects an
                // `ArcStr`. `arcstr::literal!(<cref>)` recovers an `ArcStr`
                // from a const `&'static str` expression with zero allocation
                // — it constructs the same kind of static-storage `ArcStr`
                // that `arcstr::literal!("foo")` produces from a literal.
                Ty::Str if is_const_str_cref(name, segments, ctx, top_level) =>
                    format!("arcstr::literal!({var_str})"),
                _ => format!("{var_str}.clone()"),
            }
        }

        TypedExp::BinOp { op, lhs, rhs, ty, .. } => {
            let mut l = emit_exp(lhs, is_const, ctx, top_level);
            let mut r = emit_exp(rhs, is_const, ctx, top_level);
            // MetaModelica implicitly takes the first output of a tuple-returning
            // expression when it appears in scalar context (e.g. `find(k,m) > 0`,
            // where `find` returns `(Integer, Integer)`). Both operands of a
            // binary op are scalar slots, so when one side has tuple type but the
            // other doesn't, apply `.0` to unpack. Type vars are deliberately not
            // unpacked — they could be instantiated as a tuple downstream.
            let lhs_is_tuple = matches!(lhs.ty(), Ty::Tuple(_));
            let rhs_is_tuple = matches!(rhs.ty(), Ty::Tuple(_));
            if lhs_is_tuple && !rhs_is_tuple && !matches!(rhs.ty(), Ty::Unknown | Ty::TypeVar(_)) {
                l = format!("({l}).0");
            }
            if rhs_is_tuple && !lhs_is_tuple && !matches!(lhs.ty(), Ty::Unknown | Ty::TypeVar(_)) {
                r = format!("({r}).0");
            }
            if lhs.ty() == Ty::I32 && rhs.ty() == Ty::F64 {
                l = format!("metamodelica::OrderedFloat(({l}) as f64)");
            }
            if rhs.ty() == Ty::I32 && lhs.ty() == Ty::F64 {
                r = format!("metamodelica::OrderedFloat(({r}) as f64)");
            }
            // Parenthesize operands when the inner operator binds more loosely
            // than the outer, or — for the right-hand side of a left-associative
            // op — when it binds equally. Without this, `(a - b) / c` would
            // re-emit as `a - b / c`, silently changing the meaning.
            let parent_prec = binop_prec(*op);
            if let TypedExp::BinOp { op: lop, .. } = &**lhs {
                if binop_prec(*lop) < parent_prec {
                    l = format!("({l})");
                }
            }
            if let TypedExp::BinOp { op: rop, .. } = &**rhs {
                if binop_prec(*rop) <= parent_prec {
                    r = format!("({r})");
                }
            }
            match op {
                BinOpKind::Eq => {
                    // String `==` in a `const` initializer can't lower to
                    // `*l == *r` (PartialEq isn't const). The runtime emission
                    // above wraps string literals in `arcstr::literal!(...)`,
                    // producing `ArcStr` values whose `Deref` to `&str` isn't
                    // const either. Re-emit each operand as a raw `&'static str`
                    // expression (literals lose the `literal!()` wrapper; const
                    // crefs of string-typed `pub const`s resolve through their
                    // chain via [`resolve_const_str`] to the underlying `&str`),
                    // then defer the comparison itself to `const_str::equal!`
                    // — which is a true const-eval string compare.
                    // If we can't reduce an operand to a `&'static str` (e.g. a
                    // function call), we emit `compile_error!` so the unhandled
                    // case is visible at the use site rather than silently
                    // producing broken code.
                    if is_const && (lhs.ty() == Ty::Str || rhs.ty() == Ty::Str) {
                        let lstr = emit_const_str_operand(lhs, ctx, top_level);
                        let rstr = emit_const_str_operand(rhs, ctx, top_level);
                        return match (lstr, rstr) {
                            (Some(ls), Some(rs)) => format!("const_str::equal!({ls}, {rs})"),
                            _ => format!(
                                "{{ compile_error!(\"const string `==` operand not const-foldable; extend emit_const_str_operand: {l} == {r}\"); false }}"
                            ),
                        };
                    }
                    format!("{l} == {r}")
                }
                BinOpKind::NEq => {
                    // Mirror the `==` case for `!=` — same const-eval constraint.
                    if is_const && (lhs.ty() == Ty::Str || rhs.ty() == Ty::Str) {
                        let lstr = emit_const_str_operand(lhs, ctx, top_level);
                        let rstr = emit_const_str_operand(rhs, ctx, top_level);
                        return match (lstr, rstr) {
                            (Some(ls), Some(rs)) => format!("!const_str::equal!({ls}, {rs})"),
                            _ => format!(
                                "{{ compile_error!(\"const string `!=` operand not const-foldable; extend emit_const_str_operand: {l} != {r}\"); false }}"
                            ),
                        };
                    }
                    format!("{l} != {r}")
                }
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
                // For +,-,*,/ the i32→Real promotion was already applied to
                // `l`/`r` by the general block above; don't wrap twice.
                BinOpKind::Add => format!("{l} + {r}"),
                BinOpKind::Sub => format!("{l} - {r}"),
                BinOpKind::Mul => format!("{l} * {r}"),
                BinOpKind::Div => format!("{l} / {r}"),
                BinOpKind::Pow => {
                    // `Float::powf` (via `num_traits::Float` re-export) works on
                    // Real directly and returns Real. The general i32→Real
                    // promotion above already widens mixed I32/F64 operands; the
                    // only remaining case where promotion is missing is I32^I32,
                    // which we widen here so both sides are Real.
                    let lp = if lhs.ty() == Ty::I32 && rhs.ty() == Ty::I32 {
                        format!("metamodelica::OrderedFloat(({l}) as f64)")
                    } else { l };
                    let rp = if lhs.ty() == Ty::I32 && rhs.ty() == Ty::I32 {
                        format!("metamodelica::OrderedFloat(({r}) as f64)")
                    } else { r };
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
            if named_args.is_empty()
                && let Ok(res) = emit_builtin_call(func, args, is_const, ctx, top_level) {
                    return res;
                }
            // `String(value, significantDigits=N, ...)` is the MetaModelica
            // value-to-string builtin with optional named formatting args.
            // Rust's `String` is a struct, not a function — falling through to
            // the regular call path would emit `String(...)` which is a struct
            // tuple-constructor and fails to compile. We don't yet handle the
            // full set of formatting options (minimumLength, leftJustified,
            // significantDigits) so surface this as a `todo!` rather than
            // generating wrong code.
            if func == "String" {
                // We don't yet lower the named formatting options
                // (minimumLength, leftJustified, significantDigits). Surface
                // this clearly at runtime rather than silently emitting a
                // wrong call. The block returns an `ArcStr` so the
                // surrounding string-concat path (`&*expr`) still
                // type-checks against the deref target.
                let names: Vec<String> = named_args.iter().map(|(n, _)| n.to_string()).collect();
                let msg = format!("String() builtin with named args [{}] not yet lowered", names.join(","));
                let escaped = format!("{msg:?}");
                return format!("{{ let __mm_unimpl: ArcStr = todo!({escaped}); __mm_unimpl }}");
            }
            // When the call target is a bare name that collides with an
            // in-scope local binding (typically an output parameter that
            // shares its name with a sibling function — a common
            // MetaModelica pattern, e.g. `isAlwaysMatchedBool` whose output
            // is `isAlwaysMatched` and whose body calls function
            // `isAlwaysMatched`), Rust resolves the bare identifier to the
            // local binding instead of the function item. Re-resolve the
            // call name through the hierarchy and emit the shortened FQN so
            // the call lands on the function. Only apply when the resolved
            // qname names a different path than the local (i.e. it is a
            // real function), and only for bare (non-dotted) names — dotted
            // forms are already qualified.
            // Only rewrite when the local binding is NOT itself a function
            // (callbacks parameters are legitimately invoked by name).
            let local_shadows_fn = match ctx.fn_env_vars.get(func) {
                Some(Ty::Function { .. }) | Some(Ty::FunctionAlias { .. }) | None => false,
                Some(_) => true,
            };
            // Walk hierarchy scopes for the first match that is actually a
            // function. `resolve_call_qname` would return the shadowing
            // local-output child first and stop, defeating the purpose.
            let resolved_fn_qname = if local_shadows_fn {
                resolve_call_qname_fn(func, ctx, top_level)
            } else {
                None
            };
            // For the shadow-disambiguation case we cannot use `ctx.shorten`
            // because it strips the current-module prefix and leaves a bare
            // name that still collides with the local. Use `self::<func>` to
            // force module-scope name resolution; this works regardless of
            // whether the function lives in the top-level module or a
            // nested `mod` block (both cases resolve `self::` to the
            // currently-being-emitted module). For cross-module functions
            // (different module than the current one) `ctx.shorten` already
            // produces a non-colliding `Mod::func` path, so we use that.
            let func_str = if func.contains('.') {
                // A dotted source name like `FunctionTree.new` may need a
                // type-alias-head rewrite to land on the right function (see
                // `resolve_call_qname`'s type-alias branch: `type FunctionTree
                // = FunctionTreeImpl.Tree;` makes `FunctionTree.new` shorthand
                // for `FunctionTreeImpl.new`). Use the resolved qname when it
                // names a *different* path than the literal source, since the
                // literal would otherwise dispatch to a non-existent associated
                // function on the alias's target type (E0061 / E0599).
                match resolve_call_qname(func, ctx, top_level) {
                    Some(qname) if qname.as_str() != func => ctx.shorten(&qname),
                    _ => ctx.shorten(func),
                }
            } else if let Some(qname) = resolved_fn_qname.clone() {
                let cur_prefix = if ctx.current_path.is_empty() {
                    ctx.top_name.clone()
                } else {
                    format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
                };
                if qname == format!("{cur_prefix}.{func}") {
                    format!("self::{func}")
                } else {
                    ctx.shorten(&qname)
                }
            } else {
                func.to_owned()
            };
            let func_str = escape_ident(&func_str);
            let formals = resolve_call_formals(func, ctx, top_level);
            let parts: Vec<String> = if let Some(formals) = formals {
                // Reorder positional + named arguments into formal-declaration
                // order. Rust calls are positional-only, so a MetaModelica
                // call like `f(p, name=v)` has to be lowered to `f(p, v)` —
                // emitting `name=v` would be invalid Rust syntax. This block
                // runs for every user-function call where we have formal
                // information; the prior gate restricted it to functions
                // with default arguments, which silently mis-emitted any
                // call that mixed named args with non-defaulted formals.
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

                // Apply default values for any still-empty slot. Defaults are
                // typed in the callee's scope and may reference earlier
                // formals, so we substitute the already-filled slot values
                // for those name references before emitting.
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
                    // Last resort: emit positional args followed by `n=v`
                    // pairs. The `n=v` form is not valid Rust call syntax
                    // and will fail to compile — that's intentional: it
                    // surfaces the residual case (typically an unresolved
                    // named arg) instead of silently dropping it.
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

            // Decide whether to attach the `?`/q-wrapping. A call expression in
            // the generated Rust returns:
            //   * `T`         — if the callee is an *infallible user function*
            //                   (its Rust signature was generated without
            //                   `Result<>` by `emit_function`), or if it is an
            //                   infallible builtin (whose lib.rs signature now
            //                   returns bare `T`).
            //   * `Result<T>` — otherwise (fallible user functions or fallible
            //                   builtins return `Result`).
            //
            // We can only resolve "infallible user function" from the dotted
            // name through the hierarchy. Const contexts produce constant
            // expressions and never wrap.
            if is_const {
                call
            } else {
                // When a local binding shadows the function name, the general
                // `resolve_call_qname` picks the local (a non-function child)
                // and the fallibility check then falls through to
                // `is_infallible_builtin` and (wrongly) wraps the call with
                // `.unwrap()`. We already resolved the *function* qname via
                // `resolve_call_qname_fn` for `func_str`; reuse it so the
                // fallibility check matches the actual call target.
                let callee_qname = if local_shadows_fn {
                    resolved_fn_qname.clone().or_else(|| resolve_call_qname(func, ctx, top_level))
                } else {
                    resolve_call_qname(func, ctx, top_level)
                };
                // A user-defined function shadows any same-named builtin. We
                // must NOT fall through to `is_infallible_builtin` when the
                // name resolves to a user function — e.g. `exp` in
                // `Template.TplMain` is a recursive AST-printer, not the
                // math exponential. Only consult the builtin table when the
                // name didn't resolve to a user function in the hierarchy.
                let infallible = match callee_qname.as_deref() {
                    Some(q) => ctx.is_known_infallible_user_fn(q, top_level),
                    None => is_infallible_builtin(func),
                };
                if infallible {
                    call
                } else {
                    ctx.q(&call)
                }
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

        TypedExp::Array { elems, ty } => {
            if elems.is_empty() {
                "metamodelica::nil()".to_owned()
            } else {
                // The list's declared element type acts as the "formal" for each
                // element expression. Route through `emit_call_arg_with_formal`
                // so multi-output calls used in single-value context (e.g.
                // `{System.dladdr(func)}` where `dladdr` returns a 3-tuple but
                // each list element must be a scalar `String`) get the
                // implicit first-element extraction applied (Tuple → `.0`).
                let elem_ty = match ty {
                    Ty::List(inner) => Some((**inner).clone()),
                    _ => None,
                };
                let parts: Vec<String> = elems.iter().map(|e| {
                    emit_call_arg_with_formal(e, elem_ty.as_ref(), is_const, ctx, top_level)
                }).collect();
                format!("list![{}]", parts.join(", "))
            }
        }

        TypedExp::Match { kind, input, cases, as_binding, .. } => {
            emit_match(kind, input, cases, as_binding.as_deref(), is_const, ctx, top_level)
        }

        TypedExp::Range { start, step, stop, .. } => {
            emit_range(start, step.as_deref(), stop, is_const, ctx, top_level)
        }

        TypedExp::Constructor { name, args, named_args, ty, field_names } => {
            let mut arg_strs = Vec::new();
            if let Ty::RustStruct(qname) | Ty::RustEnum(qname) = ty {
                let remaining_named = named_args.clone();
                // Look up the struct's declared field types so we can apply
                // type-driven coercions (e.g. Integer-literal arguments
                // assigned to a `Real` field need `as f64`).
                // For multi-record uniontype variant constructors the `ty`
                // carries the parent enum qname (e.g.
                // `NFExpression.NFExpression`) whose `record_field_tys` is
                // empty (the parent has no fields of its own — they live on
                // each variant record). Always try the variant's own qname
                // (the constructor `name`) first so we get the variant
                // record's actual field types; fall back to the enum qname
                // for the single-record-folded case.
                let field_tys_map: Vec<(String, Ty)> = record_field_tys(name, top_level)
                    .filter(|v| !v.is_empty())
                    .or_else(|| record_field_tys(qname, top_level))
                    .or_else(|| {
                        lookup_record_through_unions(qname, top_level)
                            .and_then(|(canonical, _)| record_field_tys(&canonical, top_level))
                    })
                    .unwrap_or_default();
                let field_ty_lookup = |fname: &str| -> Option<Ty> {
                    field_tys_map.iter().find(|(n, _)| n == fname).map(|(_, t)| t.clone())
                };
                for (i, fa) in args.iter().enumerate() {
                    let val = emit_cloned_call_arg(fa, is_const, ctx, top_level);
                    if i < field_names.len() {
                        let fname = &field_names[i];
                        let fname_safe = escape_ident(fname);
                        let val = if struct_field_is_arc(qname, fname, top_level, ctx)
                            && !value_emitted_as_arc(fa, ctx)
                        {
                            format!("Arc::new({val})")
                        } else {
                            // Apply Integer→Real / tuple→first coercion based on
                            // the declared field type. Without this, MetaModelica
                            // constructors like `SOURCEINFO(...,0)` whose last
                            // argument is `0: Integer` going into a `Real` field
                            // emit `lastModification: 0` and rustc rejects.
                            let ft = field_ty_lookup(fname);
                            coerce_assign_expr_pub(val, &fa.ty(), ft.as_ref())
                        };
                        arg_strs.push(format!("{fname_safe}: {val}"));
                    } else {
                        // fallback if we have more pos args than fields? shouldn't happen for valid mm
                        arg_strs.push(val);
                    }
                }
                for (n, na) in remaining_named {
                    let val = emit_cloned_call_arg(&na, is_const, ctx, top_level);
                    let val = if struct_field_is_arc(qname, &n, top_level, ctx)
                        && !value_emitted_as_arc(&na, ctx)
                    {
                        format!("Arc::new({val})")
                    } else {
                        let ft = field_ty_lookup(&n);
                        coerce_assign_expr_pub(val, &na.ty(), ft.as_ref())
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
                    c_rust.to_string()
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
                //
                // If the enclosing uniontype is recursive, its values are stored as
                // `Arc<Enum>` everywhere they appear (variable slots, struct fields,
                // function parameters and returns). The unit-variant *expression*
                // however evaluates to a bare `Enum`. To keep types consistent at
                // use sites we wrap the variant value in `Arc::new(...)` — unless
                // we are in a `const` context (`Arc::new` is not const).
                //
                // The parent enum is determined either from the constructor's `name`
                // (e.g. `"Pkg.Tree.EMPTY"` → parent `"Pkg.Tree"`) or from
                // `Ty::UnionTypeVariant(parent, _)` if available.
                let path = ctx.dotted_to_rust_path(name);
                let parent_recursive = match ty {
                    Ty::UnionTypeVariant(parent, _) => ctx.recursive_types.contains(parent.as_str()),
                    _ => name
                        .rsplit_once('.')
                        .map(|(parent, _)| ctx.recursive_types.contains(parent))
                        .unwrap_or(false),
                };
                if !is_const && parent_recursive {
                    format!("Arc::new({path})")
                } else {
                    path
                }
            } else if let Ty::UnionTypeVariant(enum_qname, _variant_name) = ty {
                // Struct variant inside a multi-record uniontype (Rust enum).
                // Look up field names from the record in the hierarchy.
                let field_tys = record_field_tys(name, top_level)
                    .unwrap_or_else(|| record_field_tys_by_simple_name(name, top_level));
                let field_ty_lookup = |fname: &str| -> Option<Ty> {
                    field_tys.iter().find(|(n, _)| n == fname).map(|(_, t)| t.clone())
                };
                let variant_rust = ctx.dotted_to_rust_path(name);
                for (i, a) in args.iter().enumerate() {
                    let val = emit_cloned_call_arg(a, is_const, ctx, top_level);
                    let fname = field_tys.get(i).map(|(n, _)| n.as_str()).unwrap_or("_");
                    let val = if struct_field_is_arc(enum_qname, fname, top_level, ctx)
                        && !value_emitted_as_arc(a, ctx)
                    {
                        format!("Arc::new({val})")
                    } else {
                        // Same Int→Real coercion as the RustStruct/RustEnum
                        // branch above — without it, an `Integer` expression
                        // (e.g. `intExp1 / intExp2`) flowing into a `Real`
                        // variant field is emitted as `value: i32 / i32` and
                        // rustc rejects (expected OrderedFloat<f64>).
                        let ft = field_ty_lookup(fname);
                        coerce_assign_expr_pub(val, &a.ty(), ft.as_ref())
                    };
                    arg_strs.push(format!("{}: {val}", escape_ident(fname)));
                }
                for (n, na) in named_args {
                    let val = emit_cloned_call_arg(na, is_const, ctx, top_level);
                    let val = if struct_field_is_arc(enum_qname, n, top_level, ctx)
                        && !value_emitted_as_arc(na, ctx)
                    {
                        format!("Arc::new({val})")
                    } else {
                        let ft = field_ty_lookup(n);
                        coerce_assign_expr_pub(val, &na.ty(), ft.as_ref())
                    };
                    arg_strs.push(format!("{}: {val}", escape_ident(n)));
                }
                let ctor_expr = if arg_strs.is_empty() {
                    variant_rust.to_string()
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

    // Pull the formal-name order *and* declared types from the resolved
    // signature. The types let us route each captured binding through
    // `emit_call_arg_with_formal`, which applies the same actual→formal
    // coercions the inner call site does (e.g. wrap a bare `fnptr!(...)`
    // closure in `Arc::new(...)` when the formal is `Arc<dyn Fn(...)>`).
    // Without that, `__pe_b{i}` would hold a raw closure and the inner call
    // would fail with `expected Arc<dyn Fn>, found closure` (see e.g.
    // `findIteratorIndexedCrefs` in `AbsynUtil.mo`, which partial-applies
    // `List.unionEltOnTrue(inCompFunc = iteratorIndexedCrefsEqual)`).
    let (formal_names, formal_tys): (Vec<String>, Vec<Ty>) = match sig_ty {
        Ty::Function { inputs, .. } => (
            inputs.iter().map(|i| i.name.clone()).collect(),
            inputs.iter().map(|i| i.ty.clone()).collect(),
        ),
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

    // When the formal is an anonymous callback slot (`Arc<dyn Fn(...) -> Result<_>>`),
    // give the capture an explicit `Arc<dyn Fn(...)>` annotation so the
    // `Arc::new(closure)` emitted by `emit_call_arg_with_formal` unsizes to
    // a trait object at the binding site. Without the annotation Rust would
    // infer `Arc<ClosureType>` for `__pe_b{i}` (no unsizing in a let without
    // a target type) and the inner call would still see a concrete closure
    // type, not `Arc<dyn Fn>` — failing with "expected Arc<dyn Fn>, found
    // closure" (see `findIteratorIndexedCrefs` in `AbsynUtil.mo`).
    //
    // For inputs / outputs that mention type variables not in scope at this
    // partial-application site (e.g. a generic `Fn(T) -> bool`), we emit `_`
    // so the compiler infers them from the inner call's signature.
    fn dyn_fn_annot(ty: &Ty, ctx: &mut GenCtx) -> Option<String> {
        let Ty::Function { inputs, output, name: None, .. } = ty else { return None };
        let ins = inputs.iter().map(|i| {
            if ty_mentions_typevar(&i.ty) { "_".to_owned() } else { fmt_ty(&i.ty, ctx) }
        }).collect::<Vec<_>>().join(", ");
        let out = if ty_mentions_typevar(output) { "_".to_owned() } else { fmt_ty(output, ctx) };
        Some(format!("Arc<dyn ::std::ops::Fn({ins}) -> Result<{out}> + 'static>"))
    }

    for (i, formal_name) in formal_names.iter().enumerate() {
        let formal_ty = formal_tys.get(i).cloned();
        let cap_annot = formal_ty.as_ref().and_then(|t| dyn_fn_annot(t, ctx));
        let cap_decl = |cap_name: &str, v: &str| -> String {
            match &cap_annot {
                Some(ann) => format!("let {cap_name}: {ann} = {v};"),
                None => format!("let {cap_name} = {v};"),
            }
        };
        if i < bound_pos {
            // Positional binding.
            let v = emit_call_arg_with_formal(&args[i], formal_ty.as_ref(), is_const, ctx, top_level);
            let cap_name = format!("__pe_b{i}");
            captures.push(cap_decl(&cap_name, &v));
            call_arg_exprs.push(format!("{cap_name}.clone()"));
        } else if let Some(named_expr) = named_map.remove(formal_name.as_str()) {
            // Named binding (looked up by formal name).
            let v = emit_call_arg_with_formal(named_expr, formal_ty.as_ref(), is_const, ctx, top_level);
            let cap_name = format!("__pe_b{i}");
            captures.push(cap_decl(&cap_name, &v));
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
    // The closure escapes into a function-pointer slot whose signature is
    // `... -> Result<T>`. When the inner function is *infallible* (its Rust
    // signature returns `T`), the closure body produces `T` and we have to
    // re-wrap it in `Ok(...)` to satisfy the slot. The fallible case yields a
    // `Result<T>` from the call directly and needs no extra wrapping.
    let callee_qname = resolve_call_qname(func, ctx, top_level);
    // User functions shadow same-named builtins — only consult the builtin
    // table when the name does not resolve to a user function.
    let inner_infallible = match callee_qname.as_deref() {
        Some(q) => ctx.is_known_infallible_user_fn(q, top_level),
        None => is_infallible_builtin(func),
    };
    let body_expr = if inner_infallible {
        format!("Ok({call_expr})")
    } else {
        call_expr
    };
    let closure = format!("move |{params_list}| {body_expr}");

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
        format!("for mut {} in {iter_expr} {{\n", escape_ident(&it.name))
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
        } else if ty_mentions_typevar(t) {
            // The reduction body's static type carries type variables from a
            // callee's signature (e.g. `tuple21<T1,T2>`'s `T1` flowing into
            // the accumulator) that aren't in scope at the caller. Emitting
            // them verbatim would produce `Arc<List<T1>>` against an
            // undeclared name; defer to Rust inference instead. The body
            // assignment to `__x` pins the concrete type from the call site.
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
                let zero = if ty_s == "metamodelica::Real" { "metamodelica::OrderedFloat(0.0_f64)" } else { "0" };
                (
                    format!("let mut __acc: {ty_s} = {zero};"),
                    "__acc += __x;".to_owned(),
                    "__acc".to_owned(),
                )
            }
        }
        "product" => {
            let ty_s = numeric_sum_ty(&body_ty);
            let one = if ty_s == "metamodelica::Real" { "metamodelica::OrderedFloat(1.0_f64)" } else { "1" };
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
    // Register iterator bindings in fn_env_vars so the body's emit_exp can
    // resolve their types (e.g. for `var_field!` shape selection on Arc-wrapped
    // element types). Saved + restored so iterators don't leak.
    let saved_iter_tys: Vec<(String, Option<Ty>)> = iterators.iter().map(|it| {
        let elem_ty = match it.range.ty() {
            Ty::List(t) | Ty::Array(t) | Ty::Range(t) => *t,
            _ => Ty::Unknown,
        };
        let prev = ctx.fn_env_vars.insert(it.name.clone(), elem_ty);
        (it.name.clone(), prev)
    }).collect();
    let body_s = emit_exp(body, is_const, ctx, top_level);
    for (name, prev) in saved_iter_tys.into_iter().rev() {
        match prev {
            Some(t) => { ctx.fn_env_vars.insert(name, t); }
            None => { ctx.fn_env_vars.remove(&name); }
        }
    }

    match iter_kind {
        ReductionIterKind::Thread => {
            // Build a zipped iterator over all ranges and one for-loop that
            // destructures the bindings as a (possibly nested) tuple.
            let mut zip_expr = String::new();
            for (i, it) in iterators.iter().enumerate() {
                let range_s = emit_exp(&it.range, is_const, ctx, top_level);
                let part = match it.range.ty() {
                    Ty::List(_) => format!("(&({range_s})).into_iter()"),
                    // Array<T> = Rc<RefCell<Vec<T>>>; iterate through .borrow().
                    // Clone elements so the loop body owns its value, matching
                    // the List branch and the open_for() helper used by
                    // Combine iteration.
                    Ty::Array(_) => format!("({range_s}).borrow().iter().cloned()"),
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
        Ty::F64 => "metamodelica::Real",
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
        && matches!(&formal, Some(t) if !matches!(t, Ty::Tuple(_) | Ty::Unknown | Ty::TypeVar(_)));
    let raw = emit_exp(arg, is_const, ctx, top_level);
    let raw = if needs_first { format!("({raw}).0") } else { raw };
    // Mirror the Integer→Real widening in `emit_call_arg_with_formal`. Without
    // it, builtins like `SOURCEINFO(_,_,_,_,_,_,lastModification)` reject a
    // bare integer literal for the `Real` slot.
    if matches!(formal, Some(Ty::F64)) && matches!(arg.ty(), Ty::I32) {
        return format!("metamodelica::OrderedFloat(({raw}) as f64)");
    }
    raw
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
        // setGlobalRoot(index, value)
        //   MetaModelicaBuiltin.mo: writes `value` into the process-wide
        //   global-root table at the integer slot `index`. The MMC C runtime
        //   (`nobox_setGlobalRoot` in meta_modelica_builtin.c) splits the
        //   table at index 8: slots 0..8 live in `threadData->localRoots`,
        //   slots 9..N in `mmc_GC_state->global_roots`. We mirror that
        //   semantic in `metamodelica::setGlobalRoot` (currently a single
        //   thread_local Vec — see the long comment there for why we can't
        //   use a process-wide static while values may contain non-Send
        //   `Rc<RefCell<…>>`).
        //
        // We pin the type parameter `A` of the runtime function with an
        // explicit turbofish whenever the value's type is known. The Rust
        // runtime function is
        //   `setGlobalRoot<A: Any + 'static>(idx: i32, value: A)`
        // and without the turbofish the inference fails when `value` is
        // `None` (E0282 on the `Option::T` parameter, exactly the shape seen
        // in `Global::initialize`) or when the value is a tuple/`Some(…)` of
        // a type that the surrounding context doesn't pin.
        //
        // The index argument is emitted as-is. We deliberately do *not*
        // constant-fold a `constant Integer` cref like `Global.instHashIndex`
        // to its literal value: the generated Rust module already exposes
        // these as `pub const` items, and emitting `Global::instHashIndex`
        // keeps the call site in sync with future edits to Global.mo. (The
        // C bootstrap folds them, but here the const item is right there in
        // the same crate.)
        //
        // TODO(per-index typed slots): the longer-term design is to resolve
        // each *known* index to its own typed `LazyLock`/thread-local (e.g.
        // `Global.instHashIndex` → a dedicated
        // `LazyLock<RefCell<InstHashTable::HashTable>>` initialised from
        // `InstHashTable::emptyInstHashTable()`). That requires a
        // whole-program pre-pass to gather (idx_const_qname, value_type)
        // pairs across files and resolve type-variable indirection.
        "setGlobalRoot" if args.len() == 2 => {
            // Fast path: if the index argument resolves to a named `constant Integer`
            // in a package that has a typed `Globals` module, emit a direct
            // thread-local assignment.  The `thread_local!` / `RefCell` pair is
            // infallible, so no `?` is needed.
            if let Some(grc) = try_resolve_global_root_const(&args[0], ctx, top_level) {
                let val_expr = emit_builtin_call_arg(func, 1, &args[1], is_const, ctx, top_level);
                let var_path = global_root_var_path(&grc, ctx);
                // Both thread-local (index 0..=8) and process-global (index 9..)
                // roots use `thread_local!` + `RefCell` because `Array<T> =
                // Rc<RefCell<Vec<T>>>` is not `Send`, preventing `Mutex<T>`.
                // TODO: when `Array<T>` is made `Send`, upgrade index-9+ roots
                // to `static Mutex<T>` and change the emitted accessor here.
                return Ok(format!("{var_path}.with(|__root| *__root.borrow_mut() = {val_expr})"));
            }
            // Fallback: dynamic / unresolved index — keep the type-erased runtime call.
            let idx_expr = emit_builtin_call_arg_raw(func, 0, &args[0], is_const, ctx, top_level);
            let val_ty = args[1].ty();
            let val_expr = emit_builtin_call_arg(func, 1, &args[1], is_const, ctx, top_level);
            let turbofish = if ty_contains_unknown(&val_ty) {
                // We can't pin `A` because the value's type is not fully
                // known here. Leave inference to Rust — at the few sites
                // where this happens (notably the `NONE()` calls in
                // `Global::initialize`) the user will still see an E0282 or
                // unused-init warning. Documented at the call site.
                String::new()
            } else {
                format!("::<{}>", fmt_ty(&val_ty, ctx))
            };
            let q = if ctx.current_fn_fallible { "?" } else { ".unwrap()" };
            Ok(format!("metamodelica::setGlobalRoot{turbofish}({idx_expr}, {val_expr}){q}"))
        }
        // getGlobalRoot(index) — read the value previously stored at `index`.
        //   Mirrors `nobox_getGlobalRoot` (see comment on setGlobalRoot for
        //   the slot-layout details). The runtime fails with an Err if the
        //   slot was never written — matching the C runtime's
        //   `MMC_THROW_INTERNAL` behavior — so we propagate with `?`.
        //
        // We don't emit a turbofish here: the result type is inferred from
        // the assignment LHS, which is almost always a typed local in
        // generated code (e.g. `let mut tpl: Option<…>; tpl = getGlobalRoot(I)?;`).
        // The `'static` bound on `A` is now satisfied because every
        // generated generic type-parameter carries `: 'static` via
        // `DEFAULT_TRAITS`.
        "getGlobalRoot" if args.len() == 1 => {
            // Fast path: named constant → typed thread-local read.
            if let Some(grc) = try_resolve_global_root_const(&args[0], ctx, top_level) {
                let var_path = global_root_var_path(&grc, ctx);
                // `.borrow().clone()` is infallible; no `?` needed.
                return Ok(format!("{var_path}.with(|__root| __root.borrow().clone())"));
            }
            // Fallback: dynamic / unresolved index.
            let idx_expr = emit_builtin_call_arg_raw(func, 0, &args[0], is_const, ctx, top_level);
            let q = if ctx.current_fn_fallible { "?" } else { ".unwrap()" };
            Ok(format!("metamodelica::getGlobalRoot({idx_expr}){q}"))
        }
        "valueConstructor" if args.len() == 1 => {
            // MetaModelica `valueConstructor(x)` returns the variant index of
            // the *value* `x` — for two values of the same uniontype but
            // different records it must yield distinct integers. We lower it
            // to `metamodelica::valueConstructor(&x)`, which inspects the
            // value at runtime via `std::mem::discriminant`.
            //
            // For `Arc<T>`-wrapped values we deref through the `Arc` so that
            // the discriminant belongs to the inner enum rather than to
            // `Arc` itself (`Arc` is a struct, so `mem::discriminant` would
            // collapse every variant onto a single value, reproducing the
            // bug this code replaces).
            let arg_ty = args[0].ty();
            let arg_expr = emit_builtin_call_arg_raw(func, 0, &args[0], is_const, ctx, top_level);
            let is_arc_wrapped = match &arg_ty {
                Ty::RustEnum(n) | Ty::AliasTo(n) | Ty::RustStruct(n) => {
                    ctx.recursive_types.contains(n.as_str())
                }
                _ => false,
            };
            let ref_expr = if is_arc_wrapped {
                format!("(&*{arg_expr})")
            } else {
                format!("(&{arg_expr})")
            };
            let q = if ctx.current_fn_fallible { "?" } else { ".unwrap()" };
            Ok(format!("metamodelica::valueConstructor({ref_expr}){q}"))
        }
        "getInstanceName" if args.is_empty() => {
            // MetaModelicaBuiltin.mo: returns a String literal with the
            // fully-qualified name of the enclosing function. The MMC bootstrap
            // resolves this at compile time, and so do we — emit the captured
            // `current_fn_qname` as a string literal!. This is preferable to a
            // runtime lookup because it avoids any thread-local plumbing and
            // makes the result a true compile-time constant.
            let escaped = format!("{:?}", ctx.current_fn_qname);
            Ok(format!("literal!({escaped})"))
        }
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
                // Both `metamodelica::Real` (OrderedFloat<f64>) and `i32` impl
                // `Ord`, so `std::cmp::max/min` works uniformly. NaN ordering
                // follows `ordered_float` semantics.
                Ok(format!("std::cmp::{func}({arg1}, {arg2})"))
            } else {
                let parts: Vec<String> = args.iter().enumerate()
                    .map(|(i, a)| emit_builtin_call_arg_raw(func, i, a, is_const, ctx, top_level))
                    .collect();
                Ok(format!("{func}({})", parts.join(", ")))
            }
        },
        "String" => {
            let arg_exp = args.first();
            let arg = arg_exp.map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            // Pick the format spec based on the argument's static type.
            // `Display` is implemented for primitives and `ArcStr`; user
            // enums/structs only derive `Debug`. Without this, `String(tok.id)`
            // where `tok.id: TokenId` (enum) fails to compile because
            // `TokenId: Display` is unimplemented. The `Debug` fallback
            // matches `anyString`'s behaviour and keeps the runtime output
            // human-readable.
            let spec = match arg_exp.map(|a| a.ty()) {
                Some(Ty::I32) | Some(Ty::F64) | Some(Ty::Bool) | Some(Ty::Str) => "{}",
                _ => "{:?}",
            };
            // Use the fully-qualified path `::std::format` to avoid an
            // ambiguity when a `use super::*;` line in the enclosing module
            // also brings `arcstr::format!` into scope (E0659).
            Ok(format!("ArcStr::from(::std::format!(\"{spec}\", {arg}))"))
        },
        "stringGet" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(ctx.q(&format!("stringGet({},{})", arg1, arg2)))
        },
        "realNeg" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("-({arg1})"))
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
            Ok(format!("({arg1}) {op} ({arg2})"))
        },
        "realInt" => {
            // metamodelica::Real wraps f64; access via `.0` then truncate to i32.
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("(({arg}).0 as i32)"))
        },
        // Integer(x) is a Modelica/MetaModelica built-in type cast.
        // For Real → Integer: floor to i32.
        // For Enumeration → Integer: the discriminant (as i32).
        // For Boolean → Integer: false=0, true=1.
        "Integer" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            // Check the argument type to emit the right conversion.
            match args.first().map(|a| a.ty()).as_ref() {
                // Real value is `metamodelica::Real` (OrderedFloat<f64>):
                // reach the inner f64 via `.0` before truncating.
                Some(crate::hierarchy::Ty::F64) => Ok(format!("(({arg}).0 as i32)")),
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
            // floor/ceil are Real-only. `num_traits::Float` (re-exported by
            // metamodelica) lets us call `.floor()` directly on Real; the
            // result is Real, no rewrap needed.
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("({arg}).floor()"))
        },
        "ceil" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("({arg}).ceil()"))
        },
        "mod" if args.len()==2 => {
            let a1 = args.first().unwrap();
            let a2 = args.get(1).unwrap();
            let f = if a1.ty() == Ty::I32 && a2.ty() == Ty::I32 {"intMod"} else {"realMod"};
            // Route through the typed-formal helper for the resolved underlying
            // builtin (intMod/realMod) so a tuple-returning operand still gets
            // unpacked via `.0` against the correct scalar formal type.
            let arg1 = emit_builtin_call_arg(f, 0, a1, is_const, ctx, top_level);
            let arg2 = emit_builtin_call_arg(f, 1, a2, is_const, ctx, top_level);
            // intMod and realMod are both infallible: no ctx.q() needed
            Ok(format!("{f}({arg1}, {arg2})"))
        },
        "div" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("{arg1} / {arg2}"))
        },
        "abs" => {
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            // `abs` is i32-or-Real overloaded. For both: with `num_traits::Float`
            // in scope `.abs()` resolves on Real directly and returns Real;
            // i32::abs returns i32. Same syntax either way.
            Ok(format!("{arg}.abs()"))
        },
        "integer" => {
            // Modelica `integer(Real)` truncates toward -inf. `metamodelica::Real`
            // wraps `f64`; reach the inner value via `.0`.
            let arg = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("(({arg}).0.floor() as i32)"))
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
            // referenceEq is infallible: returns bool directly, no ctx.q() needed
            Ok(format!("referenceEq(&{arg1},&{arg2})"))
        },
        "isPresent" => {
            Ok("true /* isPresent not implemented in Rust */".to_string())
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
            // arrayCreateNoInit is infallible: returns Array<A> directly
            Ok(format!("metamodelica::Dangerous::arrayCreateNoInit({arg1})"))
        },
        "arrayClearIndex" => {
            let arg1 = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            // arrayClearIndex is infallible: returns () directly
            Ok(format!("metamodelica::Dangerous::arrayClearIndex({arg1}, {arg2})"))
        },
        "arrayUpdate"| "arrayUpdateNoBoundsChecking" => {
            // MM semantics: mutate in place, return the same array (aliases see the change).
            // We bind the array once so {arg1} (which may be a non-trivial expression) is
            // evaluated only once, mutate through `borrow_mut()`, then yield the same Rc.
            // NoBoundsChecking uses checked indexing here; same TODO as arrayGet.
            //
            // If the target array was created by arrayCreateNoInit its slots hold
            // uninitialised bytes. A plain `slot = value` would first DROP the garbage
            // bytes as if they were a live Arc<T> → SIGSEGV. We use arrayInitSlot
            // (which calls ptr::write) so the old bytes are not touched.
            let arr_is_uninit = matches!(
                args.first(),
                Some(TypedExp::Var { name, .. }) if ctx.uninit_arrays.contains(name.as_str())
            );
            let arg1 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            let arg3 = args.get(2).map(|a| emit_builtin_call_arg(func, 2, a, is_const, ctx, top_level)).unwrap_or_default();
            if arr_is_uninit {
                Ok(format!("unsafe {{ metamodelica::Dangerous::arrayInitSlot({arg1}, {arg2}, {arg3}) }}"))
            } else {
                // If the value expression reads from any RefCell (`.borrow(`) it may
                // alias the array we're about to `borrow_mut()` — hoist to a temp first
                // so the read finishes before the mutable borrow is acquired. Otherwise
                // emit the compact form without a temp.
                let rhs_may_alias = arg3.contains(".borrow(") || arg3.contains(".borrow_mut(");
                if rhs_may_alias {
                    Ok(format!("{{let _arr = {}; let _val = {}; _arr.borrow_mut()[({}-1) as usize] = _val; _arr}}", arg1, arg3, arg2))
                } else {
                    Ok(format!("{{let _arr = {}; _arr.borrow_mut()[({}-1) as usize] = {}; _arr}}", arg1, arg2, arg3))
                }
            }
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
            let a0 = args.first().map(|a| emit_builtin_call_arg(func, 0, a, is_const, ctx, top_level)).unwrap_or_else(|| "Arc::new(\"\".to_string())".to_owned());
            let a1 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_else(|| "false".to_owned());
            let a2 = args.get(2).map(|a| emit_builtin_call_arg_raw(func, 2, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a3 = args.get(3).map(|a| emit_builtin_call_arg_raw(func, 3, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a4 = args.get(4).map(|a| emit_builtin_call_arg_raw(func, 4, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a5 = args.get(5).map(|a| emit_builtin_call_arg_raw(func, 5, a, is_const, ctx, top_level)).unwrap_or_else(|| "0".to_owned());
            let a6 = args.get(6).map(|a| {
                let s = emit_builtin_call_arg_raw(func, 6, a, is_const, ctx, top_level);
                // The `lastModification` field is Real (f64); MetaModelica callers
                // routinely pass an Integer literal (`SOURCEINFO(...,0)`). Coerce
                // to f64 so the struct initializer typechecks.
                coerce_assign_expr_pub(s, &a.ty(), Some(&Ty::F64))
            }).unwrap_or_else(|| "metamodelica::OrderedFloat(0.0_f64)".to_owned());
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
        // Modelica trigonometric, exponential and square-root builtins.
        // The MetaModelica compiler accepts these bare-name builtins (declared
        // in the Modelica standard, used e.g. in BackendDAE inverse-derivation
        // tables). They are pure (Real, Real) -> Real or Real -> Real and have
        // identical Rust equivalents on `f64`. We coerce the operand to `f64`
        // because the upstream typed-exp inference sometimes reports
        // `Ty::Unknown` (the call is forwarded through a generic helper) which
        // would otherwise leave the `.sin()` etc. method call unresolved.
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan"
        | "sinh" | "cosh" | "tanh" | "exp" | "log" | "log10" | "sqrt" if args.len() == 1 => {
            // `num_traits::Float` (re-exported by metamodelica) provides these
            // methods directly on Real, returning Real — no `.0`/rewrap needed.
            // `ln` in Rust / `num_traits::Float` is the natural log; Modelica
            // calls it `log`.
            let arg = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let method = match func { "log" => "ln", other => other };
            Ok(format!("({arg}).{method}()"))
        },
        "atan2" if args.len() == 2 => {
            let a1 = args.first().map(|a| emit_builtin_call_arg_raw(func, 0, a, is_const, ctx, top_level)).unwrap_or_default();
            let a2 = args.get(1).map(|a| emit_builtin_call_arg_raw(func, 1, a, is_const, ctx, top_level)).unwrap_or_default();
            Ok(format!("({a1}).atan2({a2})"))
        },
        // Modelica `assert(condition, message[, level])`. The optional level
        // argument is an AssertionLevel enum (error|warning); we don't yet
        // distinguish severities, so it's accepted but ignored. The Rust
        // `assert!` macro panics on failure, which matches the Modelica
        // `AssertionLevel.error` default — non-fatal `warning` assertions
        // would need a different lowering (TODO).
        "assert" if args.len() >= 2 => {
            let cond = emit_builtin_call_arg_raw(func, 0, &args[0], is_const, ctx, top_level);
            let msg = emit_builtin_call_arg(func, 1, &args[1], is_const, ctx, top_level);
            // The message lowers to `ArcStr`; `assert!` requires a `&str` or
            // a format string, so deref into `&str` via `&*`.
            Ok(format!("assert!({cond}, \"{{}}\", &*{msg})"))
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
        //
        // Package-level constants of type `Array<T>` are emitted as
        // `LazyLock<metamodelica::Array<T>>` = `LazyLock<Rc<RefCell<Vec<T>>>>`.
        // Indexing requires `.borrow()` first (the LazyLock deref-coerces to
        // Rc<RefCell<_>> automatically, but Rust does not auto-`.borrow()`).
        // We detect Array-typed constants by resolving the fully-qualified
        // package path to its declared node and inspecting its type. This is
        // the package-path mirror of the local-variable `Ty::Array` check at
        // the top of this function.
        let last_pkg_segs = pkg_segs.last();
        if let Some(last_seg) = last_pkg_segs {
            if !last_seg.subscripts.is_empty() {
                let is_array_const = resolve_fully_qualified(&pkg_dotted, ctx, top_level)
                    .map(|n| matches!(n.ty, Ty::Array(_)))
                    .unwrap_or(false);
                let mut b = escape_ident(&shortened);
                if is_array_const {
                    b = format!("{b}.borrow()");
                }
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

    // If the package portion resolves to a `pub fn`/`pub const fn` getter for
    // a non-Sync constant (see `collect_const_fn_getters`) AND we have
    // subsequent field accesses (`.returnType`, etc.), inject `()` here so
    // the field chain runs on the *called* value rather than on the function-
    // item type itself. The bare-reference case (no field segments) is
    // handled instead by the Var arm in emit_exp.
    let base = if !field_segs.is_empty()
        && !pkg_dotted.is_empty()
        && let Some(qn) = resolve_call_qname(&pkg_dotted, ctx, top_level)
        && ctx.const_fn_getters.contains(&qn)
    {
        format!("{base}()")
    } else {
        base
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

    // Track the type of the value `res` currently refers to so we can detect
    // when a downstream field access lands on an `Array<T>` (= `Rc<RefCell<Vec<T>>>`)
    // and inject the required `.borrow()` before indexing.
    //
    // Starting point: a single-segment package base resolves to the local
    // variable's declared type (via `fn_env_vars`); a multi-segment package
    // path resolves to the node's declared type. Anything we can't resolve
    // (untyped match-arm bindings, captured closure vars, …) leaves the
    // tracker at `Ty::Unknown` and disables the array-borrow injection for
    // that chain — which falls back to the prior (incorrect) behavior, i.e.
    // a compile error at the call site rather than silent wrong codegen.
    let mut cur_ty: Ty = if pkg_segs.len() == 1 {
        ctx.fn_env_vars.get(&pkg_segs[0].name).cloned().unwrap_or(Ty::Unknown)
    } else if !pkg_dotted.is_empty() {
        resolve_fully_qualified(&pkg_dotted, ctx, top_level)
            .map(|n| n.ty.clone())
            .unwrap_or(Ty::Unknown)
    } else if !real_segments.is_empty() {
        ctx.fn_env_vars.get(&real_segments[0].name).cloned().unwrap_or(Ty::Unknown)
    } else {
        Ty::Unknown
    };

    if pkg_segs.len() == 1 && !field_segs.is_empty() {
        let var_name = &pkg_segs[0].name;
        if let Some((enum_qname, variant_name)) = ctx.variants.get(var_name).cloned()
            // Only emit `var_field!` when `enum_qname` truly names a Rust enum
            // (a multi-record uniontype). Single-record uniontypes are
            // transparent `Ty::AliasTo` structs — there is no enum to
            // destructure, and emitting `var_field!(v.x, Pkg::Foo::CTOR)` for
            // them produces ambiguous-associated-type errors because `Foo`
            // resolves to the underlying struct, not an enum. We can't filter
            // on the variable's own `fn_env_vars` type because nested-As
            // bindings (e.g. `right: child @ NODE { .. }`) don't appear there.
            && uniontype_is_enum(&enum_qname, top_level) {
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
            if !first.subscripts.is_empty() {
                // Look up the field's declared type on the variant. If it's
                // Array<T> (= Rc<RefCell<Vec<T>>>), insert `.borrow()` before
                // indexing — matching the package-path and inherited-field
                // branches below. Without this, `[i]` is applied directly to
                // an `&Rc<RefCell<Vec<T>>>` reference, which doesn't impl
                // `Index` (E0608).
                let variant_qname = format!("{enum_qname}.{variant_name}");
                let field_ty = record_field_tys(&variant_qname, top_level)
                    .unwrap_or_default()
                    .into_iter()
                    .find(|(n, _)| n == &first.name)
                    .map(|(_, t)| t)
                    .unwrap_or(Ty::Unknown);
                if matches!(field_ty, Ty::Array(_)) {
                    res = format!("{res}.borrow()");
                }
                for sub in &first.subscripts {
                    res = format!("{}[({}-1) as usize]", res, emit_exp(sub, false, ctx, top_level));
                }
            }
        }
    }

    for seg in field_iter {
        // Look up the type of `seg.name` on the current (record-shaped) ty
        // *before* mutating `res`, so we know whether to insert `.borrow()`
        // for an Array-valued field. `cur_ty` is then advanced to the field
        // type, with subscripts peeling off the outer Array/List layer in
        // the same way `resolve_first_segment_type` does on the typedexp side.
        let field_ty: Ty = lookup_field_ty(&cur_ty, &seg.name, top_level).unwrap_or(Ty::Unknown);
        res = format!("{}.{}", res, escape_ident(&seg.name));
        if !seg.subscripts.is_empty() && matches!(field_ty, Ty::Array(_)) {
            res = format!("{res}.borrow()");
        }
        for sub in &seg.subscripts {
            res = format!("{}[({}-1) as usize]", res, emit_exp(sub, false, ctx, top_level));
        }
        cur_ty = field_ty;
        for _ in &seg.subscripts {
            cur_ty = match cur_ty {
                Ty::Array(inner) | Ty::List(inner) => *inner,
                other => other,
            };
        }
    }
    res
}

/// Look up the Rust type of a field on a record-shaped Modelica type. Used by
/// `emit_var` to thread enough type information through dotted CREF emission
/// to decide whether `.borrow()` is required before subscripting an
/// `Array<T>` field. Walks through the same shapes that
/// `resolve_first_segment_type` (in `typedexp`) handles.
fn lookup_field_ty(ty: &Ty, field: &str, top_level: &BTreeMap<String, NameNode<'_>>) -> Option<Ty> {
    match ty {
        Ty::RustStruct(qname) | Ty::AliasTo(qname) => {
            record_field_tys(qname, top_level)?
                .into_iter()
                .find(|(n, _)| n == field)
                .map(|(_, t)| t)
        }
        Ty::RustEnum(qname) => {
            // Field access on an enum value is only legal after pattern-narrowing;
            // we don't track that narrowing here, so search all variants and
            // return the first matching field. Same-named fields must agree on
            // type across variants in MetaModelica.
            let node = lookup_node(qname, top_level)?;
            let NodeKind::Class(c) = &node.kind else { return None };
            if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) { return None; }
            for child in node.children.values() {
                if let Some(field_tys) = record_field_tys(&format!("{qname}.{}", child_qname_simple(child)), top_level)
                    && let Some((_, t)) = field_tys.into_iter().find(|(n, _)| n == field) {
                        return Some(t);
                    }
            }
            None
        }
        Ty::Generic(rust_name, _args) => {
            let dotted = rust_name.replace("::", ".");
            record_field_tys(&dotted, top_level)?
                .into_iter()
                .find(|(n, _)| n == field)
                .map(|(_, t)| t)
        }
        // Pattern-narrowed uniontype scrutinee — look up the field directly on
        // the specific record variant. This is the precise path; the
        // `Ty::RustEnum` arm above is the fallback when narrowing is not
        // available and is only correct when all variants agree on the field's
        // type (which MetaModelica does not enforce — see e.g. JSON.values).
        Ty::UnionTypeVariant(union_qname, variant_name) => {
            let variant_qname = format!("{union_qname}.{variant_name}");
            record_field_tys(&variant_qname, top_level)?
                .into_iter()
                .find(|(n, _)| n == field)
                .map(|(_, t)| t)
        }
        _ => None,
    }
}

/// Does `qname` name a uniontype with ≥2 record variants — i.e. one that is
/// rendered as a Rust enum (so `var_field!(value, Qname::VARIANT)` is valid)?
/// Single-record uniontypes are rendered as transparent struct aliases
/// (`Ty::AliasTo`) and return `false` here.
fn uniontype_is_enum(qname: &str, top_level: &BTreeMap<String, NameNode<'_>>) -> bool {
    let Some(node) = lookup_node(qname, top_level) else { return false };
    let NodeKind::Class(c) = &node.kind else { return false };
    if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) { return false; }
    let record_count = node.children.values()
        .filter(|child| matches!(&child.kind,
            NodeKind::Class(rc) if matches!(rc.restriction,
                Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })))
        .count();
    record_count >= 2
}

/// Return a simple-name reference for a child node — used only as a key in the
/// dotted lookup performed by `lookup_field_ty` for uniontype variants. The
/// hierarchy stores variant nodes under their simple name, which matches the
/// dotted layout `{parent_qname}.{variant_name}`.
fn child_qname_simple(child: &NameNode<'_>) -> String {
    match &child.kind {
        NodeKind::Class(c) => c.name.clone(),
        _ => String::new(),
    }
}

// ── Global-root typed-slot helpers ───────────────────────────────────────────

/// A resolved reference to a `constant Integer` component in a MetaModelica
/// package that names a global-root slot (e.g. `Global.currentInstVar = 23`).
struct GlobalRootConst {
    /// Top-level package plus any sub-path segments, dot-separated
    /// (e.g. `"Global"`).
    pkg: String,
    /// Simple name of the constant within that package (e.g. `"currentInstVar"`).
    const_name: String,
    /// Integer value of the constant; 0–8 are thread-local (C `localRoots`),
    /// 9+ are process-global (C `global_roots`). Both use `thread_local!` in
    /// Rust for now because `Array<T> = Rc<RefCell<Vec<T>>>` is not `Send`.
    #[allow(dead_code)]
    index_value: i32,
}

/// Attempt to resolve `arg` as a reference to a `constant Integer` component
/// in a known package (e.g. `Global.currentInstVar`).
///
/// Returns `None` when:
///   * `arg` is not a `TypedExp::Var` (e.g. it's a literal integer),
///   * the name contains subscripts,
///   * no matching `CONST` component with an `INTEGER` initialiser is found, or
///   * the reference is a local variable (not resolvable in the hierarchy).
///
/// Both qualified names (`Global.flagsIndex`) and unqualified names
/// (`flagsIndex` used inside the `Global` package itself) are handled.
/// Unqualified names are resolved by trying each scope from innermost to
/// outermost: `{top_name}.{current_path[0]}.…{name}`, …, `{top_name}.{name}`.
fn try_resolve_global_root_const<'a>(
    arg: &TypedExp,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<GlobalRootConst> {
    let TypedExp::Var { name, segments, .. } = arg else { return None; };
    // Reject names that carry subscripts on any segment.
    if segments.iter().any(|s| !s.subscripts.is_empty()) {
        return None;
    }

    // Build the list of fully-qualified candidate names to try, in order from
    // innermost scope to outermost.
    let candidates: Vec<String> = if name.contains('.') {
        // Already qualified — use as-is.
        vec![name.clone()]
    } else {
        // Unqualified name: try qualifying it with each enclosing scope,
        // starting from the most specific (deepest) and working outward.
        //
        // For ctx.top_name = "Global", ctx.current_path = ["initialize"],
        // name = "instOnlyForcedFunctions":
        //   ["Global.initialize.instOnlyForcedFunctions",
        //    "Global.instOnlyForcedFunctions"]
        let mut scopes: Vec<String> = Vec::new();
        let mut scope = ctx.top_name.clone();
        scopes.push(scope.clone());
        for seg in &ctx.current_path {
            scope = format!("{scope}.{seg}");
            scopes.push(scope.clone());
        }
        scopes.reverse(); // innermost first
        scopes.iter().map(|s| format!("{s}.{name}")).collect()
    };

    for fqn in &candidates {
        let dot = fqn.rfind('.').unwrap(); // safe: all candidates contain a dot
        let pkg = &fqn[..dot];
        let const_name_str = &fqn[dot + 1..];
        let Some(node) = lookup_node(fqn, top_level) else { continue };
        let NodeKind::Component(m) = &node.kind else { continue };
        if m.variability != Absyn::Variability::CONST { continue; }
        let Some(exp) = extract_default_exp(&m.modification) else { continue };
        let Absyn::Exp::INTEGER { value } = exp else { continue };
        return Some(GlobalRootConst {
            pkg: pkg.to_owned(),
            const_name: const_name_str.to_owned(),
            index_value: *value,
        });
    }
    None
}

/// Compute the Rust path to the named global-root variable for `grc` in the
/// appropriate crate's `Globals` module.
///
/// Emits `crate::Globals::NAME` when the constant's top-level package belongs
/// to the crate currently being generated, and `some_crate::Globals::NAME`
/// otherwise. Uses the same `crate_map` as the rest of codegen.
fn global_root_var_path(grc: &GlobalRootConst, ctx: &GenCtx) -> String {
    // Per-name overrides: some roots whose *index* constant lives in
    // Global.mo (→ openmodelica_util) but whose *value type* lives in a
    // downstream crate.  The thread_local! declaration was placed in the
    // downstream crate's Globals.rs to avoid circular dependencies.
    let crate_override: Option<&str> = match grc.const_name.as_str() {
        // openmodelica_frontend — types from Absyn, SCode, FCore, etc.
        "instHashIndex"
        | "instNFInstCacheIndex"
        | "instNFNodeCacheIndex"
        | "instNFLookupCacheIndex"
        | "builtinIndex"
        | "builtinGraphIndex"
        | "inlineHashTable"
        | "operatorOverloadingCache"
        | "backendCevalInterface" => Some("openmodelica_frontend"),
        // openmodelica_frontend_dump — backendInterface root holds the
        // function table populated by the frontend_dump-side interface
        // (see FrontEnd/BackendInterface.mo and its `__OpenModelica_Interface`).
        "backendInterface" => Some("openmodelica_frontend_dump"),
        // openmodelica_backend — types from SymbolTable, SimCode, etc.
        "symbolTable" | "rewriteRulesIndex" | "optionSimCode" | "interactiveCache" => {
            Some("openmodelica_backend")
        }
        _ => None,
    };
    let pkg_top = grc.pkg.split('.').next().unwrap_or(&grc.pkg);
    let globals_prefix = match crate_override.or_else(|| ctx.crate_map.get(pkg_top).map(|s| s.as_str())) {
        Some(mc) if Some(mc) == ctx.current_crate.as_deref() => "crate::Globals".to_owned(),
        Some(mc) => format!("{mc}::Globals"),
        None => "crate::Globals".to_owned(),
    };
    format!("{globals_prefix}::{}", grc.const_name)
}

/// Find the index at which to split segments into [package prefix] and [record fields].
/// Returns the index of the first field segment. Everything before is the package path.
/// Returns `segments.len()` if no record boundary is found (entire path is a package path).
/// Returns true if `ty` mentions any `Ty::Unknown` anywhere in its structure.
/// Used by the `setGlobalRoot` lowering to decide whether the value's type is
/// fully resolved enough to emit as a turbofish on the runtime call. A `Ty`
/// containing `Unknown` would emit `?` in `fmt_ty` and fail to compile.
fn ty_contains_unknown(ty: &Ty) -> bool {
    match ty {
        Ty::Unknown => true,
        Ty::I32 | Ty::F64 | Ty::Bool | Ty::Str | Ty::Unit
        | Ty::Enumeration(_) | Ty::TypeVar(_) | Ty::RustStruct(_)
        | Ty::RustEnum(_) | Ty::RustUnitVariant | Ty::AliasTo(_)
        | Ty::UnionTypeVariant(_, _) | Ty::ExternalObject(_)
        | Ty::FunctionAlias { .. } => false,
        Ty::Option(t) | Ty::List(t) | Ty::Array(t) | Ty::Range(t) => ty_contains_unknown(t),
        Ty::Tuple(ts) => ts.iter().any(ty_contains_unknown),
        Ty::Generic(_, args) => args.iter().any(ty_contains_unknown),
        Ty::Function { inputs, output, .. } => {
            inputs.iter().any(|i| ty_contains_unknown(&i.ty)) || ty_contains_unknown(output)
        }
    }
}

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
    // 2c. Self-alias of the current module (e.g. `import Type = NFType;` inside
    // NFType.mo). The head segment is the local alias; the rest is a path inside
    // the current module. Rewrite the head to `top_name` and try again. Without
    // this, `Type.ANY` references in NFType.rs fall through to the bare-name
    // fallback in `find_record_split` and emit `Type.ANY` (field access) — but
    // `Type` is not in scope anywhere, surfacing as E0433.
    {
        let mut head_parts = prefix_dotted.splitn(2, '.');
        let head = head_parts.next().unwrap_or(prefix_dotted);
        let tail = head_parts.next().unwrap_or("");
        if ctx.self_aliases.contains(head) {
            let candidate = if tail.is_empty() {
                ctx.top_name.clone()
            } else {
                format!("{}.{}", ctx.top_name, tail)
            };
            if let Some(n) = lookup_node(&candidate, top_level) {
                return Some(n);
            }
        }
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

    // A local variable in scope always shadows any same-named module / function
    // / package at the top level. Without this short-circuit a CREF like
    // `prefixPath.path` — where `prefixPath` is both an input parameter and a
    // sibling function name (e.g. AbsynUtil.removePrefixOpt referencing the
    // sibling `prefixPath` function) — emits as `prefixPath::path`, which Rust
    // resolves to the sibling function's `path` item and not to the local's
    // field. Detect the shadowing here so the field-access path is taken
    // instead.
    if ctx.fn_env_vars.contains_key(&segments[0].name) {
        return 1;
    }

    // Walk backwards from the longest possible split point
    for i in (1..=segments.len()).rev() {
        let prefix_dotted: String = segments[..i].iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".");

        // If the whole prefix actually resolves to something
        let resolved_opt = resolve_fully_qualified(&prefix_dotted, ctx, top_level);
        if let Some(node) = resolved_opt {
            if let crate::hierarchy::NodeKind::Class(c) = &node.kind {
                use openmodelica_ast::Absyn::Restriction::*;
                // `type X = enumeration(...)` carries restriction `R_TYPE` in
                // the AST even though the body is an enumeration. emit_type_item
                // emits it as a Rust `enum`, so its variants must be accessed
                // via `::`. Treat it the same as `R_ENUMERATION` for splitting.
                let is_enum_body = matches!(&c.body, MM::ClassDef::Enumeration { .. });
                match c.restriction {
                    _ if is_enum_body => {
                        if i < segments.len() {
                            return i + 1;
                        } else {
                            return i;
                        }
                    }
                    // uniontypes and enums act as namespaces for their constructors.
                    // If we found the uniontype/enum itself, the VERY NEXT segment is its constructor!
                    // so the rust module path covers up to the constructor.
                    openmodelica_ast::Absyn::Restriction::R_ENUMERATION | openmodelica_ast::Absyn::Restriction::R_UNIONTYPE => {
                        if i < segments.len() {
                            return i + 1;
                        } else {
                            return i;
                        }
                    }
                    // A package acts as a transparent namespace.
                    // If the path exactly resolves to a package, it's a module path.
                    openmodelica_ast::Absyn::Restriction::R_PACKAGE => return i,
                    // Records and classes have fields.
                    openmodelica_ast::Absyn::Restriction::R_RECORD | openmodelica_ast::Absyn::Restriction::R_METARECORD { .. } | openmodelica_ast::Absyn::Restriction::R_CLASS | openmodelica_ast::Absyn::Restriction::R_MODEL | openmodelica_ast::Absyn::Restriction::R_BLOCK | openmodelica_ast::Absyn::Restriction::R_CONNECTOR => {
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

/// True when `arg` is a bare reference to an INPUT parameter of the surrounding
/// function whose static type is an anonymous function (`Ty::Function { name:
/// None, .. }`). Such parameters are declared with the `&impl Fn(...)` shape by
/// [`fmt_param_ty`], so their Rust value is *already a shared reference*.
/// Forwarding them into another `&impl Fn(...)` slot must NOT prefix another
/// `&`, or the binding's type widens to `&&F`, then `&&&F`, etc., on each
/// recursive call — Rust monomorphizes a fresh `sort::<T, &F>` /
/// `sort::<T, &&F>` instance per level and quickly hits the recursion limit.
///
/// We only match a single-segment CREF: a dotted path (`obj.callback`) is not
/// a direct parameter forward — its first segment is the parameter, but the
/// path itself produces a field value, not the parameter's binding. Multi-segment
/// paths fall through to the default `&(raw)` borrow.
fn arg_is_input_fn_param(arg: &TypedExp, ctx: &GenCtx) -> bool {
    let TypedExp::Var { name, segments, ty, .. } = arg else { return false };
    if segments.len() > 1 { return false; }
    // Whether the binding's Rust value is already shaped as `Arc<dyn Fn>` (as
    // opposed to a bare `fn(...)` pointer). [`fmt_param_ty`] emits `Arc<dyn Fn>`
    // for `Ty::Function { name: None, .. }` and for named partial-function
    // aliases declared *inside* another function (tracked in
    // `nested_partial_aliases`); top-level named aliases stay as `fn(...)`
    // pointers and DO need an outer `Arc::new(...)` when fed into an anonymous
    // `Arc<dyn Fn>` slot.
    let stored_as_arc_dyn = |ty: &Ty| -> bool {
        match ty {
            Ty::Function { name: None, .. } => true,
            Ty::Function { name: Some(qn), .. } => ctx.nested_partial_aliases.contains(qn),
            _ => false,
        }
    };
    if !stored_as_arc_dyn(ty) { return false; }
    let base = name.split('.').next().unwrap_or(name);
    // Input parameters AND function-typed local variables (outputs/protected /
    // pattern bindings) are declared with the same shape `fmt_param_ty` chose
    // for the formal — already `Arc<dyn Fn + 'static>`. Forward with
    // `.clone()`; wrapping in `Arc::new(...)` would produce
    // `Arc<Arc<dyn Fn>>` and fail typeck (E0277).
    if ctx.fn_input_names.contains(base) { return true; }
    match ctx.fn_env_vars.get(base) {
        Some(env_ty) => stored_as_arc_dyn(env_ty),
        None => false,
    }
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
    // Array literals: the actual's element type comes from the local inference
    // of the literal, but the *call-site formal* (`list<String>`, etc.) is the
    // type the elements must conform to. Re-route through `emit_call_arg_with_formal`
    // per element using the formal's element type, so multi-output calls used
    // as single-value list elements (e.g. `{System.dladdr(func)}` where
    // `dladdr` returns a 3-tuple but the list expects `String`) get the
    // implicit first-element extraction applied.
    if let TypedExp::Array { elems, .. } = arg
        && let Some(Ty::List(inner)) = formal_ty {
            if elems.is_empty() {
                return "metamodelica::nil()".to_owned();
            }
            let elem_formal = (**inner).clone();
            let parts: Vec<String> = elems.iter().map(|e| {
                emit_call_arg_with_formal(e, Some(&elem_formal), is_const, ctx, top_level)
            }).collect();
            return format!("list![{}]", parts.join(", "));
        }
    // Tuple→first coercion only kicks in when the formal is a *concrete* scalar
    // slot. TypeVar formals (e.g. `Vector.updateNoBounds<T>`) may be instantiated
    // with a tuple type at the call site (here, `T = (K, V)`), so applying `.0`
    // would corrupt a legitimately-tupled argument.
    let needs_first = matches!(arg.ty(), Ty::Tuple(_))
        && matches!(formal_ty, Some(t) if !matches!(t, Ty::Tuple(_) | Ty::Unknown | Ty::TypeVar(_)));
    let raw = if !needs_first {
        emit_cloned_call_arg(arg, is_const, ctx, top_level)
    } else {
        // Emit the call (with `?` propagation already attached by emit_exp via ctx.q),
        // wrap in parens so `.0` binds to the whole call result, then re-apply the
        // clone-for-string heuristic against the *first element's* type.
        let raw_exp = emit_exp(arg, is_const, ctx, top_level);
        let first_ty = match arg.ty() {
            Ty::Tuple(elems) => elems.into_iter().next().unwrap_or(Ty::Unknown),
            _ => Ty::Unknown,
        };
        let extracted = format!("({raw_exp}).0");
        if is_const { extracted } else { maybe_clone_string_value(extracted, &first_ty) }
    };

    // When the formal is an anonymous callback slot (`Arc<dyn Fn(...) + 'static>`)
    // the argument must be an owned `Arc`. There are two argument shapes:
    //
    //   1. The argument is an INPUT parameter of the surrounding function
    //      whose declared shape is already `Arc<dyn Fn(...) + 'static>` (see
    //      [`fmt_param_ty`]). The binding's Rust value is already an `Arc`;
    //      forward it with `.clone()` so ownership is preserved without
    //      moving and so that the same callback can be passed to multiple
    //      call sites (e.g. recursive left + right calls in `List.sort`).
    //
    //   2. Everything else (PartEval closures, `fnptr!(...)` macro expansions,
    //      freshly synthesized `move |..| ..` closures, `fn`-pointer locals
    //      emitted by `fmt_ty` for outputs / protected locals) is produced
    //      as a bare value expression and needs `Arc::new(...)` so the
    //      closure is heap-allocated and reference-counted before entering
    //      the `Arc<dyn Fn>` slot.
    //
    // We only intervene when the formal is `name: None`. Named `partial
    // function` aliases (e.g. `ConflictFunc`) keep their concrete `fn(...)`
    // shape in `fmt_param_ty` — wrapping a `fn` pointer in `Arc::new` and
    // passing it into a `fn(...)` slot would be a type error.
    // `resolve_call_formals` pulls the formal type from `node.ty.inputs`,
    // which matches what `fmt_param_ty` saw at the function-definition site.
    // The formal slot is `Arc<dyn Fn(...) + 'static>` for two `fmt_param_ty`
    // shapes (kept in sync with [`fmt_param_ty`]):
    //   * `Ty::Function { name: None, .. }` — an anonymous callback signature
    //     (e.g. `input function(...) f`).
    //   * `Ty::Function { name: Some(qn), .. }` where `qn` is a
    //     function-NESTED `partial function` alias (recorded in
    //     `nested_partial_aliases`). Top-level partial aliases stay as bare
    //     `fn(...)` pointers and must NOT trigger Arc::new wrapping here.
    let formal_is_arc_dyn = match formal_ty {
        Some(Ty::Function { name: None, .. }) => true,
        Some(Ty::Function { name: Some(qn), .. }) => ctx.nested_partial_aliases.contains(qn),
        _ => false,
    };
    if let Some(Ty::Function { inputs, output, .. }) = formal_ty
        && formal_is_arc_dyn {
        if arg_is_input_fn_param(arg, ctx) {
            // case 1: `raw` is already `"inCompFunc.clone()"` (emit_exp appends
            // `.clone()` for every Ty::Function{name:None} var reference — see
            // the Var arm in emit_exp). Forward as-is; do NOT add another
            // `.clone()` or we'd produce `inCompFunc.clone().clone()`.
            return raw;
        }
        // case 2 (special): an `if … else …` whose branches are different
        // closure types. Wrapping the whole if-expression in a single
        // `Arc::new(...)` doesn't help — the *inner* if branches each have a
        // unique anonymous closure type, so the if-expression itself fails
        // to typecheck before the outer Arc::new can promote anything to
        // `Arc<dyn Fn>`. Push the Arc::new + dyn-Fn coercion *inside* each
        // branch so the if-expression's branch types unify as the same
        // `Arc<dyn Fn(..) -> ..>` slot type. See `mergeAnnotations2`
        // (`if mergeSubMods then function mergeAnnotations3(...) else
        // function subModsInSameOrder(...)`) for the trigger shape.
        if let TypedExp::If { cond, then_, elseif, else_, .. } = arg {
            // Use `as _` so the Rust compiler infers the concrete `Arc<dyn Fn>`
            // slot type from the surrounding call's parameter type. We can't
            // spell the cast explicitly here because the formal's `inputs` /
            // `output` may still reference uninstantiated type variables
            // (e.g. `findAndMap<T>`'s `Fn(T) -> bool`) that are not in scope at
            // this call site.
            let _ = (inputs, output);
            let wrap = |branch: &TypedExp, ctx: &mut GenCtx| -> String {
                let inner = emit_call_arg_with_formal(branch, formal_ty, is_const, ctx, top_level);
                format!("({inner} as _)")
            };
            let cond_s = emit_exp(cond, is_const, ctx, top_level);
            let then_s = wrap(then_, ctx);
            let elseif_parts: Vec<String> = elseif.iter().map(|(c, e)| {
                let cs = emit_exp(c, is_const, ctx, top_level);
                let es = wrap(e, ctx);
                format!(" else if ({cs}) {{ {es} }}")
            }).collect();
            let else_s = wrap(else_, ctx);
            return format!("(if ({cond_s}) {{ {then_s} }}{} else {{ {else_s} }})", elseif_parts.join(""));
        }
        // case 2 (default): wrap the fresh closure / fn-pointer in Arc::new.
        return format!("Arc::new({raw})");
    }
    // Implicit Integer→Real promotion: MetaModelica silently widens i32 actuals
    // to f64 when the formal is declared `Real`. Without this the generated
    // call passes a raw integer literal (or i32 expression) into an f64 slot
    // and rustc rejects it. We only widen when *both* sides are statically
    // known: i32 actual, f64 formal. Any other shape (unknown formal, mixed
    // arithmetic, container types) is left to the surrounding emit logic so
    // we don't accidentally cast things that aren't pure scalars.
    if matches!(formal_ty, Some(&Ty::F64)) && matches!(arg.ty(), Ty::I32) {
        // Wrap in `metamodelica::OrderedFloat(... as f64)` so the i32 actual is
        // promoted to the `Real` (`OrderedFloat<f64>`) the formal expects. We
        // use the tuple-struct constructor (a const fn) rather than `Real::from`
        // (not const) so this form also works inside `static`/`const`
        // initializers. The extra paren-pair around `raw` ensures the `as` binds
        // to the whole expression — `as` has higher precedence than `-`/`+`/etc.,
        // so an actual like `a - b` would otherwise parse as `a - (b as f64)`.
        return format!("metamodelica::OrderedFloat(({raw}) as f64)");
    }
    raw
}

/// Like [`resolve_call_qname`] but only accepts qnames that point at a real
/// function-typed node. Used when the bare call name collides with an in-scope
/// non-function local (typically an output parameter — see
/// `TplAbsyn.isAlwaysMatchedBool`, whose output `isAlwaysMatched` shadows the
/// sibling function of the same name). The plain `resolve_call_qname` returns
/// the first hierarchy match, which for nested-children resolution (line ~5812
/// in this file) is the output parameter itself — so we'd never escape the
/// shadow. This variant walks the same scopes but rejects matches that aren't
/// `Ty::Function` / `Ty::FunctionAlias`.
fn resolve_call_qname_fn<'a>(
    func: &str,
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<String> {
    let is_fn = |name: &str| -> bool {
        lookup_node(name, top_level)
            .map(|n| matches!(n.ty, Ty::Function { .. } | Ty::FunctionAlias { .. }))
            .unwrap_or(false)
    };
    if func.is_empty() { return None; }
    if func.contains('.') {
        if is_fn(func) { return Some(func.to_owned()); }
        let mut parts = func.splitn(2, '.');
        let head = parts.next().unwrap_or(func);
        let tail = parts.next().unwrap_or("");
        for (dotted, local) in &ctx.named {
            if local == head {
                let candidate = if tail.is_empty() { dotted.clone() } else { format!("{dotted}.{tail}") };
                if is_fn(&candidate) { return Some(candidate); }
            }
        }
        return None;
    }
    let cur_prefix = if ctx.current_path.is_empty() {
        ctx.top_name.clone()
    } else {
        format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
    };
    if !ctx.current_fn_qname.is_empty() {
        let candidate = format!("{}.{}", ctx.current_fn_qname, func);
        if is_fn(&candidate) { return Some(candidate); }
    }
    let mut scope: &str = &cur_prefix;
    loop {
        let candidate = format!("{scope}.{func}");
        if is_fn(&candidate) { return Some(candidate); }
        match scope.rfind('.') {
            Some(dot) => scope = &scope[..dot],
            None => break,
        }
    }
    for module in &ctx.unqual_modules {
        let candidate = format!("{module}.{func}");
        if is_fn(&candidate) { return Some(candidate); }
    }
    for (dotted, local) in &ctx.named {
        if local == func {
            if is_fn(dotted) { return Some(dotted.clone()); }
            continue;
        }
        let candidate = format!("{dotted}.{func}");
        if is_fn(&candidate) { return Some(candidate); }
    }
    if is_fn(func) { Some(func.to_owned()) } else { None }
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
        // Named-import aliases on the head segment win over any globally-visible
        // package with the same name. MetaModelica's `import Expression = NFExpression;`
        // means a call written `Expression.makeArray` must resolve to
        // `NFExpression.makeArray`, even though a top-level `Expression` package
        // (the old frontend's) also exists with a same-named-but-differently-
        // typed `makeArray`. Without this check, `resolve_call_qname` would
        // happily return the global `Expression.makeArray` and downstream
        // formal-resolution would pick the wrong formals — silently emitting
        // `name=value` for the unmatched named arg.
        let mut alias_head_parts = func.splitn(2, '.');
        let alias_head = alias_head_parts.next().unwrap_or(func);
        let alias_tail = alias_head_parts.next().unwrap_or("");
        for (dotted, local) in &ctx.named {
            if local == alias_head {
                let candidate = if alias_tail.is_empty() {
                    dotted.clone()
                } else {
                    format!("{dotted}.{alias_tail}")
                };
                if exists(&candidate) {
                    return Some(candidate);
                }
            }
        }
        // Self-alias: `import Type = NFType;` inside `NFType.mo` lets the source
        // refer to its own top class as `Type`. A call like `Type.isUnknown(...)`
        // must resolve to `NFType.isUnknown`, not fall through to a non-existent
        // top-level `Type` package. Without this, fallibility classification
        // misses the user function and the call site wraps the result with
        // `.unwrap()` even though `isUnknown` returns plain `bool`.
        if ctx.self_aliases.contains(alias_head) {
            let candidate = if alias_tail.is_empty() {
                ctx.top_name.clone()
            } else {
                format!("{}.{alias_tail}", ctx.top_name)
            };
            if exists(&candidate) {
                return Some(candidate);
            }
        }
        if exists(func) {
            return Some(func.to_owned());
        }
        // For qualified calls whose head segment names a *nested* package or
        // class of the current module (e.g. `CacheTree.add` from inside
        // `TplParser`, where `CacheTree` is a nested package), walk outward
        // from the current scope until we find a match. Without this, formals
        // and default values of inherited members of nested packages are
        // invisible at the call site (the default-injection path in emit_exp
        // depends on resolve_call_formals → resolve_call_qname succeeding).
        let cur_prefix = if ctx.current_path.is_empty() {
            ctx.top_name.clone()
        } else {
            format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
        };
        let mut scope: &str = &cur_prefix;
        loop {
            if !scope.is_empty() {
                let candidate = format!("{scope}.{func}");
                if exists(&candidate) {
                    return Some(candidate);
                }
            }
            match scope.rfind('.') {
                Some(dot) => scope = &scope[..dot],
                None => break,
            }
        }
        // Type-alias head: `type FunctionTree = FunctionTreeImpl.Tree;` makes
        // `FunctionTree.new()` shorthand for `FunctionTreeImpl.new()` — the
        // function `new` lives in the package that contains the aliased type,
        // not in the type itself. Detect this by walking outward from the
        // current scope looking for a nested R_TYPE class whose `Derived`
        // type_spec points to `Parent.SomeType`, then retry resolution with
        // `Parent` substituted for the alias head.
        {
            let mut parts2 = func.splitn(2, '.');
            let alias_head2 = parts2.next().unwrap_or(func);
            let alias_tail2 = parts2.next().unwrap_or("");
            if !alias_tail2.is_empty() {
                let mut scope: &str = &cur_prefix;
                // Pair: (parent of the aliased type's path, scope in which the
                // alias was declared). The aliased path is interpreted
                // relative to the alias's enclosing scope.
                let alias_target: Option<(String, String)> = loop {
                    let qual = if scope.is_empty() { alias_head2.to_owned() } else { format!("{scope}.{alias_head2}") };
                    if let Some(node) = lookup_node(&qual, top_level)
                        && let NodeKind::Class(c) = &node.kind
                        && matches!(c.restriction, Absyn::Restriction::R_TYPE)
                        && let MM::ClassDef::Derived { type_spec, .. } = &c.body
                        && let Absyn::TypeSpec::TPATH { path, arrayDim: None, .. } = type_spec.as_ref()
                    {
                        let aliased_str = path_to_dotted(path);
                        if let Some((parent_rel, _)) = aliased_str.rsplit_once('.') {
                            break Some((parent_rel.to_owned(), scope.to_owned()));
                        }
                    }
                    match scope.rfind('.') {
                        Some(dot) => scope = &scope[..dot],
                        None => break None,
                    }
                };
                if let Some((parent_rel, alias_scope)) = alias_target {
                    // Try absolute first (`Foo.bar`), then qualified by the
                    // alias's scope (`NFFlatten.FunctionTreeImpl.new`) walking
                    // outward, then fully-qualified through the alias's scope
                    // chain — same name resolution pattern Modelica's own
                    // lookup uses.
                    let bare = format!("{parent_rel}.{alias_tail2}");
                    if exists(&bare) { return Some(bare); }
                    let mut s: &str = &alias_scope;
                    loop {
                        let cand = if s.is_empty() {
                            format!("{parent_rel}.{alias_tail2}")
                        } else {
                            format!("{s}.{parent_rel}.{alias_tail2}")
                        };
                        if exists(&cand) { return Some(cand); }
                        match s.rfind('.') {
                            Some(d) => s = &s[..d],
                            None => break,
                        }
                    }
                }
            }
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
        // Wildcard imports (`import M.*`) make M's nested packages/classes
        // visible as bare names — including in the head segment of a dotted
        // call. For `ConnectorType.isFlow` in a scope with `import NFPrefixes.*`,
        // the head `ConnectorType` is a nested package of `NFPrefixes`, so the
        // full path is `NFPrefixes.ConnectorType.isFlow`. Without this lookup,
        // such calls go unresolved and the call site falls back to treating
        // the callee as fallible (emitting `?` / `.unwrap()`), even when the
        // target function returns a plain value.
        for module in &ctx.unqual_modules {
            let candidate = format!("{module}.{func}");
            if exists(&candidate) {
                return Some(candidate);
            }
        }
        // Modelica enclosing-scope fallback: `Variability.variabilityMax` —
        // `variabilityMax` is a function in the same package that declares
        // `Variability` (the enum). Modelica's lookup, when a qualified name
        // does not resolve as written, retries by treating the head segment
        // as a name in scope and looking up the tail in the head's enclosing
        // package. Replicate that here: resolve the head to its FQN, then
        // strip one segment to get the package and look up `package.tail`.
        {
            let mut parts = func.splitn(2, '.');
            let head = parts.next().unwrap_or(func);
            let tail = parts.next().unwrap_or("");
            if !tail.is_empty() {
                // Find the FQN of the head segment using the same resolution
                // order: named imports, self-alias, in-scope, wildcard.
                let head_fqn: Option<String> = {
                    let mut found: Option<String> = None;
                    for (dotted, local) in &ctx.named {
                        if local == head && exists(dotted) {
                            found = Some(dotted.clone());
                            break;
                        }
                    }
                    if found.is_none() && ctx.self_aliases.contains(head) {
                        found = Some(ctx.top_name.clone());
                    }
                    if found.is_none() {
                        let mut s: &str = &cur_prefix;
                        loop {
                            let cand = if s.is_empty() { head.to_owned() } else { format!("{s}.{head}") };
                            if exists(&cand) { found = Some(cand); break; }
                            match s.rfind('.') { Some(d) => s = &s[..d], None => break }
                        }
                    }
                    if found.is_none() {
                        for module in &ctx.unqual_modules {
                            let cand = format!("{module}.{head}");
                            if exists(&cand) { found = Some(cand); break; }
                        }
                    }
                    found
                };
                if let Some(head_fqn) = head_fqn
                    && let Some((parent_pkg, _)) = head_fqn.rsplit_once('.') {
                    let candidate = format!("{parent_pkg}.{tail}");
                    if exists(&candidate) {
                        return Some(candidate);
                    }
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

    // Function-nested children come first: when we are inside a function
    // whose `node.children` registers nested `function ... end ...;`
    // declarations, an unqualified call to one of those should resolve to
    // the nested name rather than walking out to the enclosing module.
    // `ctx.current_fn_qname` is the FQN of the currently-emitted function
    // (set by [`emit_function`]); it is empty at module scope, in which
    // case this lookup does nothing.
    //
    // Without this, `make_result(..)` inside `minAtomPW` falls through to
    // the module-scope walk, fails to find `SBFunctions.make_result`, and
    // (a) the codegen treats the callee as an unknown name (so it
    //     conservatively wraps `?` even when the nested function is
    //     infallible), and
    // (b) the fallibility analysis can't propagate "infallibility of
    //     `make_result`" to the caller because the call doesn't resolve.
    if !ctx.current_fn_qname.is_empty() {
        // Walk outward through enclosing function scopes. The current FQN is
        // e.g. `NFComponentRef.isTopLevel.isTopLevelRecord` when emitting the
        // body of a nested function; from there both `isTopLevelRecord` (self,
        // recursion) and `isTopLevel` (parent) should resolve. Only the
        // innermost-only test misses the recursive call to a *sibling* nested
        // fn, so iterate.
        let mut scope: &str = &ctx.current_fn_qname;
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
    }

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
        // ExternalObject classes are constructed via `ClassName(...)`. The
        // formals live on the inner `constructor` function (which carries the
        // .mo's default values). Without forwarding here, calls like
        // `File.File()` would fall through to the no-formal fallback path,
        // losing the constructor's `fromID = noReference()` default.
        if crate::hierarchy::is_external_object_class(node) {
            let ctor_qname = format!("{qname}.constructor");
            return resolve_call_formals(&ctor_qname, ctx, top_level);
        }
        return None;
    }

    // Follow MetaModelica function aliases (`function select = filterOnTrue;`)
    // to the underlying function so callers see the real formal list — the
    // alias node carries no `Component` members of its own, so without this
    // we'd return an empty formal list and call sites would lose the type
    // information needed to e.g. wrap callback args with `&(fnptr!(..))`.
    if let Ty::FunctionAlias { base, .. } = &node.ty {
        // Resolve the alias's base name relative to the alias's containing
        // module. `base` is the dotted path as written in the source (often
        // bare, e.g. `filterOnTrue`); try the alias's own module first, then
        // fall back to top-level resolution.
        let alias_module = qname.rsplit_once('.').map_or("", |(p, _)| p);
        let candidates = [
            format!("{alias_module}.{base}"),
            base.clone(),
        ];
        for cand in &candidates {
            if cand.is_empty() { continue; }
            if let Some(formals) = resolve_call_formals(cand, ctx, top_level) {
                return Some(formals);
            }
        }
    }

    let module_prefix = qname.rsplit_once('.').map_or("", |(p, _)| p).to_owned();
    let direct_members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => &[],
    };
    // For `function extends Base` the canonical input order is the one
    // `emit_function` writes from `node.ty.inputs`: inherited inputs come
    // first, then any additionally-declared ones (with redeclared inputs
    // keeping their inherited position). We use that order as the source
    // of truth here so positional call arguments line up with the right
    // formal slot — earlier this list was built from the directly-declared
    // members only, which dropped inherited slots and forced calls like
    // `f(a, b, c, isLhs=true)` against a `function extends` to fall into
    // the failure path that re-emits `isLhs=true` (invalid Rust syntax,
    // E0425). Defaults still come from the AST modifications (either the
    // direct declaration's `= v` or the base function's), since
    // `node.ty.inputs` records only types and names.
    let base_members: &[MM::ClassMember] = node.base_fn
        .and_then(|bf| match &bf.body {
            MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => Some(members.as_slice()),
            _ => None,
        })
        .unwrap_or(&[]);
    let find_component = |name: &str| -> Option<&MM::ComponentMember> {
        // Direct declaration wins because it can tighten the default;
        // fall back to the base function for the inherited modifier.
        for m in direct_members.iter().chain(base_members.iter()) {
            if let MM::ClassMember::Component(cm) = m
                && cm.name == name {
                return Some(cm);
            }
        }
        None
    };

    let mut fn_type_vars: Vec<String> = Vec::new();
    collect_type_vars_in_ty(&node.ty, &mut fn_type_vars);

    let inputs_from_ty: Vec<(String, Ty)> = if let Ty::Function { inputs, .. } = &node.ty {
        inputs.iter().map(|inp| (inp.name.clone(), inp.ty.clone())).collect()
    } else {
        Vec::new()
    };

    if !inputs_from_ty.is_empty() {
        let mut infer_env: HashMap<String, Ty> = HashMap::new();
        let mut out: Vec<CallFormal> = Vec::with_capacity(inputs_from_ty.len());
        for (name, ty) in inputs_from_ty {
            let default = find_component(&name)
                .and_then(|cm| {
                    // Only consider INPUT/INPUT_OUTPUT components: a same-named
                    // OUTPUT in the base would have no default value relevant
                    // to call sites.
                    if !matches!(cm.direction, Absyn::Direction::INPUT | Absyn::Direction::INPUT_OUTPUT) {
                        return None;
                    }
                    extract_default_exp(&cm.modification)
                })
                .map(|exp| typedexp::infer_exp(exp, &infer_env, top_level, &module_prefix, &fn_type_vars))
                .map(|tpl| canonicalize_call_funcs(tpl, &module_prefix, top_level));
            infer_env.insert(name.clone(), ty.clone());
            out.push((name, ty, default));
        }
        return Some(out);
    }

    // Fall back to walking AST members when no `Ty::Function` is available
    // (rare — typically only for nodes whose type wasn't fully inferred).
    let mut infer_env: HashMap<String, Ty> = HashMap::new();
    let mut out: Vec<CallFormal> = Vec::new();
    for member in direct_members {
        let MM::ClassMember::Component(m) = member else { continue };
        if !matches!(m.direction, Absyn::Direction::INPUT | Absyn::Direction::INPUT_OUTPUT) {
            continue;
        }
        let ty = node.children.get(&m.name).map(|n| n.ty.clone()).unwrap_or(Ty::Unknown);
        let default = extract_default_exp(&m.modification)
            .map(|exp| typedexp::infer_exp(exp, &infer_env, top_level, &module_prefix, &fn_type_vars))
            .map(|tpl| canonicalize_call_funcs(tpl, &module_prefix, top_level));
        out.push((m.name.clone(), ty.clone(), default));
        infer_env.insert(m.name.clone(), ty);
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

/// Resolve every `TypedExp::Call.func` in `exp` to its canonical (fully-qualified)
/// name using `module_prefix` as the lookup context.
///
/// `infer_exp` keeps `Call.func` as the bare/dotted name written in the source.
/// That is fine when the codegen later re-emits the call from the same module
/// the source lives in, because `resolve_call_qname` will walk the current path
/// and find the target. But default expressions cross module boundaries: a
/// default like `fromID = noReference()` written inside `File.File.constructor`
/// gets substituted at every call site, including ones in modules where
/// `noReference` does not resolve. By rewriting bare names to their qualified
/// form *while* we still have the callee's module context, the resulting
/// `TypedExp` is self-contained and can be emitted from any call site.
fn canonicalize_call_funcs<'a>(exp: TypedExp, module_prefix: &str, top_level: &'a BTreeMap<String, NameNode<'a>>) -> TypedExp {
    use TypedExp as E;
    let recur = |e: TypedExp| canonicalize_call_funcs(e, module_prefix, top_level);
    match exp {
        // A bare `Var` whose name resolves to a function in the callee's
        // scope is a function reference used as a value (e.g. the default
        // for `conflictFunc = addConflictDefault` in BaseAvlTree). When the
        // call site inlines that default, the bare name has to be rewritten
        // to its fully-qualified form so the call-site shorten step can
        // resolve it to e.g. `CacheTree::addConflictDefault` — otherwise it
        // emits as a bare identifier and Rust reports E0425 ("cannot find
        // value `addConflictDefault` in this scope").
        // A dotted Var whose head segment names a type defined in the callee's
        // enclosing scope (e.g. `ScopeType.RELATIVE` where
        // `type ScopeType = enumeration(RELATIVE, …)` is declared in the
        // same uniontype/package as the callee). When the default is inlined
        // at a call site in another module, the bare head segment `ScopeType`
        // is not in scope there. Qualify the head to its FQN
        // (e.g. `NFInstNode.ScopeType`) so the call-site `emit_var` /
        // `ctx.shorten` step can resolve it.
        E::Var { ref name, ref segments, ref ty }
            if segments.len() >= 2
                && segments.iter().all(|s| s.subscripts.is_empty()) =>
        {
            let head = &segments[0].name;
            // Resolve the head as a *type* / module reference, not a call target.
            // `typedexp::resolve_call_node` deliberately collapses a package to
            // its same-named record (so `f(DAE(...))` resolves to the `DAElist.DAE`
            // record); that's wrong for a `Var`-head, where `DAE.emptyCref` is
            // a member of the *package* `DAE`, not the record. Walk the
            // scope chain manually with plain `lookup_node` instead.
            let resolved = (|| -> Option<(String, &NameNode<'_>)> {
                // Helper: a lookup that follows Import nodes to their target.
                // Inside a module, a named/qual import registers a child node
                // (e.g. `NFDimension.Variability`) whose kind is `Import`, not
                // `Class`. We must follow it to the actual class so the
                // `is_type_like` check below sees the real declaration.
                let follow = |start_qname: String, n: &'a NameNode<'a>| -> Option<(String, &'a NameNode<'a>)> {
                    if let NodeKind::Import(m) = &n.kind {
                        let local = start_qname.rsplit('.').next().unwrap_or(&start_qname);
                        let target_dotted: Option<String> = match &m.import {
                            Absyn::Import::NAMED_IMPORT { path, .. }
                            | Absyn::Import::QUAL_IMPORT { path } => Some(path_to_dotted(path)),
                            Absyn::Import::GROUP_IMPORT { prefix, groups } => {
                                let prefix_str = path_to_dotted(prefix);
                                (&**groups).into_iter().find_map(|g| match g {
                                    Absyn::GroupImport::GROUP_IMPORT_NAME { name } if &**name == local =>
                                        Some(format!("{prefix_str}.{name}")),
                                    Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, name } if &**rename == local =>
                                        Some(format!("{prefix_str}.{name}")),
                                    _ => None,
                                })
                            }
                            Absyn::Import::UNQUAL_IMPORT { .. } => None,
                        };
                        target_dotted
                            .and_then(|t| lookup_node(&t, top_level).map(|tn| (t, tn)))
                    } else {
                        Some((start_qname, n))
                    }
                };
                if let Some(n) = lookup_node(head, top_level) {
                    if let Some(r) = follow(head.clone(), n) { return Some(r); }
                }
                if !module_prefix.is_empty() {
                    let mut parts: Vec<&str> = module_prefix.split('.').collect();
                    while !parts.is_empty() {
                        let candidate = format!("{}.{head}", parts.join("."));
                        if let Some(n) = lookup_node(&candidate, top_level) {
                            if let Some(r) = follow(candidate, n) { return Some(r); }
                        }
                        parts.pop();
                    }
                }
                None
            })();
            if let Some((qname, node)) = resolved {
                let is_type_like = matches!(&node.kind, NodeKind::Class(c) if matches!(
                    c.restriction,
                    Absyn::Restriction::R_TYPE
                        | Absyn::Restriction::R_ENUMERATION
                        | Absyn::Restriction::R_UNIONTYPE
                        | Absyn::Restriction::R_PACKAGE
                        | Absyn::Restriction::R_RECORD
                ));
                if is_type_like && qname != *head {
                    let mut new_segments: Vec<CrefSegment> = qname
                        .split('.')
                        .map(|p| CrefSegment { name: p.to_owned(), subscripts: vec![] })
                        .collect();
                    new_segments.extend(segments.iter().skip(1).cloned());
                    let new_name = new_segments
                        .iter()
                        .map(|s| s.name.clone())
                        .collect::<Vec<_>>()
                        .join(".");
                    return E::Var {
                        name: new_name,
                        segments: new_segments,
                        ty: ty.clone(),
                    };
                }
            }
            return exp;
        }
        E::Var { ref name, ref segments, ref ty }
            if !name.contains('.')
                && segments.len() <= 1
                && segments.iter().all(|s| s.subscripts.is_empty() && s.name == *name) =>
        {
            if let Some((qname, node)) = typedexp::resolve_call_node(name, top_level, module_prefix) {
                let is_function = matches!(&node.kind, NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }));
                // A bare reference to a package-level constant (e.g.
                // `defaultOptions` declared in `SCodeDump`) must also be
                // qualified — when the default expression is inlined at a
                // call site in a different module, the bare name has no
                // binding there. We accept any `Component` whose qualified
                // parent is NOT a function (function parameters are also
                // stored as `Component` children, so without the parent
                // check we would mis-qualify a formal-parameter reference
                // that `substitute_formal_refs` should later replace with
                // a concrete argument).
                let is_pkg_constant = matches!(&node.kind, NodeKind::Component(_))
                    && qname.rsplit_once('.').is_some_and(|(parent, _)| {
                        lookup_node(parent, top_level)
                            .map(|p| !matches!(&p.kind, NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. })))
                            .unwrap_or(false)
                    });
                if is_function || is_pkg_constant {
                    // Rewrite the bare reference to its fully-qualified path.
                    // Keep `segments` empty so the call-site emit path
                    // (`emit_var`) re-splits the dotted name and applies
                    // `ctx.shorten`, producing e.g. `SCodeDump::defaultOptions`
                    // or `CacheTree::addConflictDefault`.
                    E::Var { name: qname, segments: Vec::new(), ty: ty.clone() }
                } else {
                    exp
                }
            } else {
                exp
            }
        }
        E::Call { func, args, named_args, ty, sig_ty } => {
            let canonical = match typedexp::resolve_call_node(&func, top_level, module_prefix) {
                Some((q, _)) => q,
                None => func,
            };
            E::Call {
                func: canonical,
                args: args.into_iter().map(recur).collect(),
                named_args: named_args.into_iter().map(|(n, a)| (n, recur(a))).collect(),
                ty,
                sig_ty,
            }
        }
        E::Constructor { name, args, named_args, ty, field_names } => E::Constructor {
            name,
            args: args.into_iter().map(recur).collect(),
            named_args: named_args.into_iter().map(|(n, a)| (n, recur(a))).collect(),
            ty,
            field_names,
        },
        E::BinOp { op, lhs, rhs, ty } => E::BinOp {
            op,
            lhs: Box::new(recur(*lhs)),
            rhs: Box::new(recur(*rhs)),
            ty,
        },
        E::UnOp { op, operand, ty } => E::UnOp { op, operand: Box::new(recur(*operand)), ty },
        E::If { cond, then_, elseif, else_, ty } => E::If {
            cond: Box::new(recur(*cond)),
            then_: Box::new(recur(*then_)),
            elseif: elseif.into_iter().map(|(c, e)| (recur(c), recur(e))).collect(),
            else_: Box::new(recur(*else_)),
            ty,
        },
        E::Tuple(elems) => E::Tuple(elems.into_iter().map(recur).collect()),
        E::Array { elems, ty } => E::Array { elems: elems.into_iter().map(recur).collect(), ty },
        E::Cons { head, tail, ty } => E::Cons {
            head: Box::new(recur(*head)),
            tail: Box::new(recur(*tail)),
            ty,
        },
        other => other,
    }
}

/// Substitute references to formal parameter names in `exp` using concrete
/// argument expressions already chosen for earlier slots.
fn substitute_formal_refs(exp: &TypedExp, bindings: &HashMap<String, TypedExp>) -> TypedExp {
    match exp {
        TypedExp::Lit(l) => TypedExp::Lit(l.clone()),
        TypedExp::Var { name, segments, ty } => {
            // Recurse into subscript expressions in either branch.
            let new_segments: Vec<CrefSegment> = segments.iter().map(|seg| CrefSegment {
                name: seg.name.clone(),
                subscripts: seg.subscripts.iter().map(|s| substitute_formal_refs(s, bindings)).collect(),
            }).collect();
            // If the base `name` matches a formal binding, splice the binding's
            // dotted path in front of the trailing segments. This is required
            // for default-argument expressions like `Type returnType = fn.returnType`
            // — at the call site the formal `fn` is replaced by the actual
            // argument (e.g. `NFBuiltinFuncs.ABS_REAL`), so the substituted
            // expression becomes `NFBuiltinFuncs.ABS_REAL.returnType`. Without
            // this, the residual `fn` is emitted verbatim and renamed to
            // `r#fn` (escaped because `fn` is a Rust keyword), failing with
            // "cannot find value `r#fn`".
            //
            // Only handle Var bindings here — that covers the common case
            // (callers pass a constant, a local variable, or a dotted path).
            // A binding whose value is a Call or Constructor would need a
            // FieldAccess shape in TypedExp, which it doesn't have yet; in
            // that case fall through and emit `fn.returnType` literally so
            // the failure is visible at compile time rather than silently
            // wrong at runtime.
            // Pick a canonical (head, tail_segs) view of this Var's dotted path.
            // Typed inference can store the same path in several equivalent
            // shapes:
            //   * `name = "fn.returnType", segments = []`        (dotted in name)
            //   * `name = "fn", segments = [returnType]`         (split)
            //   * `name = "fn.returnType", segments = [fn, returnType]`  (duplicated; emit_var picks segments)
            //
            // `emit_var` consults `segments` when non-empty and falls back to
            // parsing `name` otherwise. Mirror that here so the substitution
            // doesn't double-count when both are present (which would splice
            // the binding's path in *and* leave the original head in segments,
            // producing nonsense like `NFBuiltinFuncs::ABS_REAL.returnType.r#fn.returnType`).
            let (head, mut tail_segs): (String, Vec<CrefSegment>) = if !new_segments.is_empty() {
                let mut it = new_segments.iter().cloned();
                let h = it.next().unwrap().name;
                (h, it.collect())
            } else if name.contains('.') {
                let mut it = name.splitn(2, '.');
                let h = it.next().unwrap_or("").to_owned();
                let rest = it.next().unwrap_or("");
                let segs: Vec<CrefSegment> = if rest.is_empty() {
                    Vec::new()
                } else {
                    rest.split('.').map(|n| CrefSegment { name: n.to_owned(), subscripts: vec![] }).collect()
                };
                (h, segs)
            } else {
                (name.clone(), Vec::new())
            };
            if let Some(repl) = bindings.get(&head) {
                if tail_segs.is_empty() {
                    return repl.clone();
                }
                if let TypedExp::Var { name: bname, segments: bsegments, ty: _bty } = repl {
                    let mut joined: Vec<CrefSegment> = bsegments.clone();
                    joined.append(&mut tail_segs);
                    return TypedExp::Var { name: bname.clone(), segments: joined, ty: ty.clone() };
                }
            }
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
        TypedExp::Match { kind, input, cases, ty, as_binding } => TypedExp::Match {
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
            as_binding: as_binding.clone(),
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

fn emit_match<'a>(kind: &MatchKind, input: &TypedExp, cases: &[TypedCase], as_binding: Option<&str>, is_const: bool, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    let raw_input_str = emit_exp(input, is_const, ctx, top_level);
    let input_ty = input.ty();
    // When the source wrote `match id as expr`, materialize the scrutinee
    // into a local named `id` so that arm bodies can read fields off it.
    // We wrap the whole match in a block: `{ let id = expr; (match id.clone() { ... }) }`.
    let (input_str, as_prefix) = if let Some(name) = as_binding {
        let id = escape_ident(name);
        let prefix = format!("{{ let {id} = {raw_input_str}; ");
        (id, Some(prefix))
    } else {
        (raw_input_str, None)
    };
    // Decide whether this match needs to be wrapped in `::match_deref::match_deref!{}`.
    // The macro provides stable-Rust replacements for nightly's `deref_patterns`:
    // a `::match_deref::Deref @ <inner>` token sequence inside an arm pattern
    // desugars to `if let inner = Deref::deref(binding) { … }`. This lets us match
    // through `Arc<List<T>>`, recursive `Arc<Enum>` variants, and `ArcStr` literal
    // patterns — all of which were the previous use cases for the
    // `#![feature(deref_patterns)]` attribute.
    //
    // When match_deref is in scope:
    //   * The match subject is wrapped in `&(…)` so that the binding the macro
    //     hands to `Deref::deref(…)` is `&T` (required by `Deref::deref` taking
    //     `&self`).
    //   * Every name binding inside the arm pattern is by reference (match
    //     ergonomics on `&T` plus the macro's own `Deref::deref` returns).
    //     `emit_pat_with_implicit_bind` is told this via `implicit_ref = true`,
    //     which makes it emit bare names (Rust 2024 rejects an explicit
    //     `ref <name>` inside an implicit-borrow pattern).
    //
    // When NOT in match_deref scope, we keep the legacy `match subject { … }`
    // form. `match_uses_match_deref` covers every case the old `input_is_arc`
    // check covered (recursive Arc<Enum>) plus tuples-containing-Arc and
    // string-literal patterns, so this is a strict broadening.
    // `#[tailcall::tailcall]` is an outer attribute proc-macro that rewrites
    // the function so each `tailcall::call!{…}` site becomes a `Thunk`
    // constructor and the *outer* trailing expression must itself produce a
    // `Thunk`. Wrapping the match in `match_deref!{ match … }` hides the arms
    // from tailcall's rewriter — the arms keep returning raw `T` values
    // (e.g. `iacc.clone().reverse()`), and Rust then complains they aren't
    // `Thunk<…>`. Until match_deref grows tailcall integration we cannot use
    // it inside tail-call lowered bodies, so fall back to the legacy
    // `.as_ref()` approach there. This means tail-call functions whose
    // scrutinees were previously decoded via the nightly `deref_patterns`
    // feature (tuple-of-Arc, ArcStr literal patterns, …) will not compile on
    // stable — those are pre-existing limitations of the lowering, not
    // regressions of this change, and will be addressed by a separate pass.
    // `match_deref!{ match X { … } }` is a true `match` expression, not the
    // `let pat = … else { bail }` cascade `MatchKind::MatchContinue` lowers
    // to — so the macro form is unavailable for matchcontinue and the
    // per-element `tuple_arc_rewrite` path below carries that case instead.
    let use_match_deref = !ctx.in_tail_lowered_fn
        && matches!(kind, MatchKind::Match)
        && match_uses_match_deref(&input_ty, cases, ctx, top_level);
    // Even when we're not opting into the full `match_deref!{…}` wrapping,
    // an `Arc<…>` scrutinee still needs `.as_ref()` to obtain a `&T`
    // reference that Rust can match against the inner enum patterns. Rust's
    // own match ergonomics on `&T` then makes the bindings by-reference,
    // which matches the implicit_ref regime in
    // `emit_pat_with_implicit_bind`. We extend this to `Ty::List`
    // (`Arc<metamodelica::List<T>>`) so tail-call lowered bodies — where
    // `match_deref!` is unavailable because it hides arms from `tailcall`'s
    // rewrite pass — can still destructure list-typed scrutinees.
    let input_is_arc_recursive = is_arc_wrapped(&input_ty, ctx)
        || matches!(input_ty, Ty::List(_));
    // Special case: `#[tailcall::tailcall]` body with a tuple scrutinee whose
    // elements include `Arc<…>` values. The legacy path can't handle these
    // (the bare tuple-of-Arc pattern needs `deref_patterns` to descend through
    // each Arc), and we've ruled out `match_deref!` for tailcall bodies.
    // Rebuild the subject as a tuple of references: each `Arc<List<T>>` /
    // `Arc<Enum>` element gets `.as_ref()` (yielding `&List<T>` / `&Enum`),
    // every other element is passed by value. Rust's match ergonomics then
    // makes all bindings inside the tuple pattern by-reference, which is
    // exactly the regime `emit_pat_with_implicit_bind` already supports via
    // `implicit_ref = true`. We re-emit each element with `emit_exp` so
    // expressions that have side effects only get evaluated once each.
    // Apply the per-element `.as_ref()` rewrite in two situations:
    //   * Tail-call lowered functions, where we cannot use `match_deref!`
    //     (the macro hides arms from the tailcall rewriter).
    //   * `matchcontinue` (`MatchKind::MatchContinue`) — its lowering uses
    //     `let pat = __mc_input.as_ref() else { … }` per arm and tuples have
    //     no `.as_ref()` method. Rewriting the input to a per-element tuple
    //     of refs lets each arm destructure via `let pat = __mc_input.clone()
    //     else { … }` (refs are `Copy`, owned components clone cheaply) and
    //     bind via the normal match ergonomics on `&T`.
    let needs_tuple_arc_rewrite = ctx.in_tail_lowered_fn
        || matches!(kind, MatchKind::MatchContinue);
    // For `MatchKind::Match` the rewrite yields a single subject expression
    // whose temporary `Arc<T>`s live for the duration of the match (Rust
    // extends their lifetime to the end of the enclosing statement). For
    // `MatchKind::MatchContinue` the subject is stashed in `__mc_input` and
    // re-used across multiple `let pat = … else { … }` statements, so the
    // temporaries would be dropped before later arms run (E0716) — the
    // MatchContinue path therefore performs its OWN rewrite below using
    // named `let __mc_arc_N = …` bindings that live for the full `'mc:`
    // block, and we skip the rewrite here for that case.
    let tuple_arc_rewrite: Option<String> = if needs_tuple_arc_rewrite
        && matches!(kind, MatchKind::Match)
        && let TypedExp::Tuple(elems) = input
        && elems.iter().any(|e| {
            let t = e.ty();
            is_arc_wrapped(&t, ctx) || matches!(t, Ty::List(_))
        })
    {
        let parts: Vec<String> = elems.iter().map(|e| {
            let et = e.ty();
            let s = emit_exp(e, is_const, ctx, top_level);
            if is_arc_wrapped(&et, ctx) || matches!(et, Ty::List(_)) {
                format!("({s}).as_ref()")
            } else {
                s
            }
        }).collect();
        Some(format!("({})", parts.join(", ")))
    } else {
        None
    };
    let match_subject = if let Some(s) = &tuple_arc_rewrite {
        s.clone()
    } else if use_match_deref {
        format!("&({input_str})")
    } else if input_is_arc_recursive {
        format!("{input_str}.as_ref()")
    } else if as_prefix.is_some() {
        // `id` is a fresh let-binding that arm bodies will read — clone so
        // the match doesn't move it.
        format!("{input_str}.clone()")
    } else {
        input_str.clone()
    };
    // `input_is_arc` is the "bindings are by-ref" flag for the legacy
    // pattern-emit path: true whenever the subject is presented as `&T` to
    // the match arms (either via `.as_ref()` on a recursive uniontype Arc,
    // via `&(…)` under match_deref, or via the per-element `.as_ref()`
    // rewrite for tail-call tuple subjects).
    let input_is_arc = use_match_deref || input_is_arc_recursive
        || (needs_tuple_arc_rewrite && matches!(input, TypedExp::Tuple(elems) if elems.iter().any(|e| {
            let t = e.ty();
            is_arc_wrapped(&t, ctx) || matches!(t, Ty::List(_))
        })));
    // The tail-call lowering applies *to this match* (not to nested matches
    // inside guards / locals / stmts). Snapshot it now and clear `ctx`'s
    // copy so deeper emit_exp calls — including those used to render the
    // match scrutinee and per-case guards/stmts — see no tail context. We
    // re-introduce the signal *just* when rendering the tail-position
    // pieces (the case result, or the algorithm-side terminal assign).
    let active_tail = ctx.tail_lowering.take();
    let body_str = match kind {
        MatchKind::Match => {
            // True when the case patterns cover every value of the scrutinee
            // type — see [`cases_exhaustive`]. When exhaustive we elide the
            // fallback `_ => bail!(...)` arm entirely, both to silence Rust's
            // unreachable-pattern lint and to keep the lowered function
            // infallible (the fallibility analysis uses the same predicate
            // on the Absyn side; the two must agree).
            let exhaustive = cases_exhaustive(kind, cases, &input_ty);
            let arms: Vec<String> = cases.iter().map(|case| {
                // Real-literal patterns lower to a binding + guard. Clear the
                // accumulator before emitting the pattern, drain after so
                // they can be merged into the arm's guard below.
                ctx.pat_extra_guards.clear();
                let pat = emit_pat_with_implicit_bind_md(&case.pattern, /*allow_implicit_bind=*/true, /*mut_bindings=*/true, /*in_deref=*/false, /*implicit_ref=*/input_is_arc, /*in_match_deref=*/use_match_deref, Some(&input_ty), ctx, top_level);
                let pat_extra_guards: Vec<String> = std::mem::take(&mut ctx.pat_extra_guards);
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
                    // `match e as v case Variant(..) => …`: the as-bound name
                    // also denotes the scrutinee, narrowed to this arm's
                    // variant. Without registering this, `v.field` reads in
                    // the arm body emit as plain `Arc<Enum>.field` accesses
                    // (E0609) instead of `var_field!`.
                    if let Some(name) = as_binding {
                        let inner_pat = match &case.pattern {
                            TypedPat::As { pat: inner, .. } => inner.as_ref(),
                            other => other,
                        };
                        if let Some((enum_q, variant)) = variant_of_pat(inner_pat, &input_ty, top_level) {
                            ctx.variants.insert(name.to_string(), (enum_q, variant));
                        }
                    }
                }
                // Register pattern bindings' types into ctx.fn_env_vars *before*
                // emitting the guard / result / body, since each of those needs
                // to see the binding's type (e.g. to decide whether an Array
                // requires a `.borrow()`). Saved/restored around the arm.
                let saved_fn_env_vars = ctx.fn_env_vars.clone();
                let saved_uninit_arrays_match = ctx.uninit_arrays.clone();
                let typed_pat_bindings: Vec<(String, Ty)> =
                    typedexp::pat_bindings_with_scrut_ty_tl(&case.pattern, &input_ty, top_level);
                for (n, t) in &typed_pat_bindings {
                    if !matches!(t, Ty::Unknown) {
                        ctx.fn_env_vars.insert(n.clone(), t.clone());
                    }
                }
                // Register this arm's case-locals (declared in the match's
                // `local` section) into ctx.fn_env_vars so any *nested* match
                // / function-call codegen inside the arm body recognises them
                // as in-scope locals. Without this, an assignment to an
                // outer-arm local from inside a nested match arm (which seeds
                // its own LocalEnv from ctx.fn_env_vars) would not find the
                // name and would emit a shadowing `let` instead of a plain
                // reassignment. See Tpl.nextIter: txt2/haveToken declared in
                // the outer `match txt` locals and assigned inside an inner
                // `match listGet(...)` algorithm. Mirrors the MatchContinue
                // path. Saved/restored alongside saved_fn_env_vars.
                for (n, t, _, _) in &case.locals {
                    if !matches!(t, Ty::Unknown) {
                        ctx.fn_env_vars.insert(n.clone(), t.clone());
                    }
                }
                let user_guard = case.guard.as_ref()
                    .map(|g| emit_exp(g, is_const, ctx, top_level));
                // Combine pattern-induced guards (e.g. from real-literal
                // patterns) with any user-written guard via `&&`.
                let guard = {
                    let mut parts: Vec<String> = pat_extra_guards.clone();
                    if let Some(g) = user_guard { parts.push(format!("({g})")); }
                    if parts.is_empty() { String::new() } else { format!(" if {}", parts.join(" && ")) }
                };
                // Tail-call lowering: detect the algorithm-side terminal
                // self-assign pattern. When the case ends with
                // `<name> := <rhs>;` and the case `result` is just
                // `<name>`, the *rhs* is the case's tail expression — emit
                // that as a tail value and elide both the last assign and
                // the trailing `result` reference.
                let algo_tail_rhs: Option<&TypedExp> = if active_tail.is_some() && !case.stmts.is_empty() {
                    if let (typedexp::TypedStmt::Assign { lhs: TypedPat::Var(asg_name), rhs }, TypedExp::Var { name: res_name, .. }) =
                        (case.stmts.last().unwrap(), &case.result)
                    {
                        if asg_name == res_name { Some(rhs) } else { None }
                    } else { None }
                } else { None };
                let result = match (active_tail.as_ref(), algo_tail_rhs) {
                    (Some((sn, fb)), None) => emit_tail_value_exp(&case.result, sn, *fb, ctx, top_level),
                    (Some((sn, fb)), Some(rhs)) => emit_tail_value_exp(rhs, sn, *fb, ctx, top_level),
                    (None, _) => emit_exp(&case.result, is_const, ctx, top_level),
                };
                let stmts_for_arm: &[typedexp::TypedStmt] = if algo_tail_rhs.is_some() {
                    // Drop the terminal assignment — its rhs is the tail
                    // expression and is emitted below as the arm's trailing
                    // expression instead.
                    &case.stmts[..case.stmts.len() - 1]
                } else {
                    &case.stmts[..]
                };
                let arm_str = if stmts_for_arm.is_empty() && case.locals.is_empty() {
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
                        // Inherit any variant narrowings established by enclosing
                        // match arms. `ctx.variants` is the global accumulator
                        // updated at every arm boundary; without seeding here the
                        // inner arm's plan_field_assign would not see that e.g.
                        // `call` was already narrowed to TYPED_CALL by the outer
                        // arm, and would fall back to a broken `todo!()` for
                        // `call.field := ..` shapes.
                        variants: ctx.variants.clone(),
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
                    let arm_alias_scope = current_scope_children(ctx, top_level);
                    for (name, ty, default, type_spec) in &case.locals {
                        // Always register the local in the arm's env so any later
                        // `name := ...` becomes a plain re-assignment and not a
                        // `let mut name = ...` that would only live inside the
                        // sub-block that emitted it. For Ty::Unknown we still
                        // pre-declare without a type annotation and let Rust
                        // infer it from the eventual write — the alternative
                        // (skipping the declaration) caused E0425/"scope" errors
                        // when the local was assigned inside an `if` arm and read
                        // after the `if`.
                        local_env.vars.insert(name.clone(), ty.clone());
                        if pat_binding_names.contains(name) {
                            continue;
                        }
                        // Skip emitting a declaration for match-level locals
                        // that are not referenced (read or written) anywhere
                        // in this arm. Locals with a `Some(default)` are
                        // emitted unconditionally — their initialiser may
                        // have side effects we must not drop. See
                        // `case_uses_local_name`.
                        if default.is_none() && !case_uses_local_name(case, name) {
                            continue;
                        }
                        if matches!(ty, Ty::Unknown) {
                            // No usable type — let Rust infer from later assignment.
                            body.push_str(&format!("            let mut {}; // TODO: local with unresolved type\n", escape_ident(name)));
                            continue;
                        }
                        // Prefer the alias name written in the MM source (e.g.
                        // `Value value;` → `let mut value: Value;`) when the
                        // declaration's TypeSpec refers to a sibling type alias
                        // in scope. Falls back to the resolved concrete type.
                        let ty_s = type_spec.as_ref()
                            .and_then(|ts| arm_alias_scope.and_then(|sc| field_type_alias_name(ts, sc)))
                            .unwrap_or_else(|| fmt_ty(ty, ctx));
                        match default {
                            Some(d) => {
                                let init = emit_exp(d, is_const, ctx, top_level);
                                let init_ty = d.ty();
                                if let Some(line) = emit_multi_output_let(true, name, ty, &init_ty, &init, "            ", ctx) {
                                    body.push_str(&line);
                                } else {
                                    body.push_str(&format!("            let mut {}: {ty_s} = {init};\n", escape_ident(name)));
                                }
                            }
                            None => {
                                body.push_str(&format!("            let mut {}: {ty_s};\n", escape_ident(name)));
                            }
                        }
                    }
                    for (n, t) in typed_pat_bindings.iter().cloned() {
                        local_env.vars.insert(n, t);
                    }
                    // Re-bind any pattern names that live inside `deref!(..)` AND
                    // are reassigned in the arm body. The pattern emitted those as
                    // `ref <name>`, giving `&T`; without a fresh owned `mut` shadow
                    // the body's `name = ...` would fail to compile. Cloning the
                    // referenced value yields an owned, mutable local that the
                    // body can both read and reassign.
                    let mut deref_names: Vec<String> = Vec::new();
                    if use_match_deref {
                        // The `match_deref!{ match &(<subj>) { … } }` wrapping
                        // makes the scrutinee a reference, so Rust's match
                        // ergonomics binds *every* name in the pattern by
                        // reference — not just those that lie behind an Arc
                        // edge. Collect them all so the reassign-shadow loop
                        // below can rebind owned `mut` locals for any name
                        // the arm body reassigns.
                        pat_collect_all_bindings(&case.pattern, &mut deref_names);
                    } else {
                        pat_deref_bindings(&case.pattern, &input_ty, ctx, top_level, &mut deref_names);
                    }
                    if !deref_names.is_empty() {
                        let mut assigned: HashSet<String> = HashSet::new();
                        stmts_assigned_var_names(&case.stmts, &mut assigned);
                        for n in &deref_names {
                            if assigned.contains(n) {
                                let id = escape_ident(n);
                                // Under match_deref the binding is by
                                // reference, so `<&T as Clone>::clone`
                                // would just hand back another `&T`. The
                                // deref-then-clone form routes through
                                // `<T as Clone>` and produces the owned `T`
                                // the body needs to reassign and pass by
                                // value. In the non-match_deref legacy
                                // path `pat_deref_bindings` reports a mix
                                // of owned and ref bindings, so we keep
                                // the plain `name.clone()` form.
                                if use_match_deref {
                                    body.push_str(&format!("            let mut {id} = (*{id}).clone();\n"));
                                } else {
                                    body.push_str(&format!("            let mut {id} = {id}.clone();\n"));
                                }
                                // The rebind drops one level of indirection:
                                // a `RefArc` (&Arc<T>) name becomes a plain
                                // `Arc<T>` value, so subsequent `var_field!`
                                // emits must use `(*v)` not `(**v)`. Without
                                // this update the codegen still believes
                                // `n` is `&Arc<T>` and produces an extra
                                // deref that doesn't typecheck (E0614).
                                if let Some(shape) = ctx.variant_shapes.get_mut(n) {
                                    if matches!(*shape, VarShape::RefArc) {
                                        *shape = VarShape::Arc;
                                    }
                                }
                            }
                        }
                    }
                    // Use emit_stmts (not a raw emit_stmt loop) so consecutive
                    // record-field updates within the arm get batched into one
                    // `assign_field!` / `assign_variant_field!` call.
                    emit_stmts(&mut body, "            ", stmts_for_arm, FailureMode::Function, ctx, &mut local_env, top_level, &mut fresh_local);
                    format!("        {pat}{guard} => {{\n{body}            {result}\n        }}")
                };
                ctx.fn_env_vars = saved_fn_env_vars;
                ctx.variants = saved_variants;
                ctx.variant_shapes = saved_shapes;
                ctx.uninit_arrays = saved_uninit_arrays_match;
                arm_str
            }).collect();
            // Inside a `#[tailcall::tailcall]` body the macro rewrites the
            // function so that early returns (`return`, `?`, `bail!`)
            // don't typecheck — every arm has to yield a value, not jump
            // out. The fallback for a non-exhaustive match is therefore
            // emitted differently inside such a function:
            //
            //   * In the *tail-position* match of a fallible function the
            //     arms each yield `Result<T>` (either a bare self-call or
            //     an `Ok(...)`-wrapped non-self leaf). The fallback is an
            //     `Err(...)` *value* of the same type — error
            //     propagation still happens because the match expression
            //     is the function's trailing return.
            //
            //   * In a *non-tail* match — bound to a `T`-typed local in
            //     the preamble, for example — the arms yield `T`, so the
            //     fallback must too. `unreachable!()` produces `!` which
            //     unifies with any expected type and panics if reached.
            //     `body_emits_extra_question_mark` only allows us to
            //     lower a function when every reachable path's value is
            //     known, so a runtime miss here is a real bug, not a
            //     soft error worth `bail!`-recovering from.
            // `match_deref!{ … }` disables Rust's exhaustiveness check for any
            // arm whose pattern contains a `Deref @ …`. Per the crate's own
            // README: "all arms with Deref @ are ignored when compiler
            // performs exhaustiveness checking. So sometimes you will need to
            // add `_ => unreachable!()` to the end." Even when our analysis
            // determined `cases_exhaustive`, the compiler will report the
            // match as non-exhaustive when wrapped — so we always emit a
            // catch-all fallback under match_deref. Outside match_deref we
            // keep the existing behaviour (no fallback for proven-exhaustive
            // matches, since Rust's own check handles it).
            let force_fallback = use_match_deref;
            let fallback = if exhaustive && !force_fallback {
                String::new()
            } else if ctx.in_tail_lowered_fn {
                if active_tail.is_some() && ctx.current_fn_fallible {
                    ",\n        _ => Err(anyhow::anyhow!(\"match: no arm matched\"))".to_owned()
                } else {
                    ",\n        _ => unreachable!(\"tail-call lowered match: no arm matched\")".to_owned()
                }
            } else if exhaustive {
                // We *proved* exhaustiveness; a runtime miss is a codegen bug.
                // `unreachable!` keeps the expression-typed (yields `!`,
                // unifies with any T) and avoids `bail!`-style return-out
                // semantics that wouldn't typecheck in every callsite.
                ",\n        _ => unreachable!(\"match_deref! exhaustiveness placeholder\")".to_owned()
            } else {
                ",\n        _ => bail!(\"match: no arm matched\")".to_owned()
            };
            // When the match is itself the tail expression of a
            // `#[tailcall::tailcall]` body, drop the outer parentheses.
            // The macro's tail-position detector matches on the literal
            // shape of the function's trailing expression and does *not*
            // peek through grouping parens — emitting `(match …)` would
            // make every `tailcall::call!{…}` inside an arm fail with
            // "tailcall::call! must be used in tail position". In any
            // non-tail context the parens are needed for precedence
            // (the match expression is often the operand of a wider
            // expression, e.g. `(match …).clone()`).
            // When `use_match_deref` is set, wrap the assembled match in the
            // `::match_deref::match_deref!{ … }` proc-macro. The macro rewrites
            // each `Deref @ pat` token sequence into an `if let` guard plus
            // body, which is what gives us "deref patterns" on stable. The
            // wrapping is at the *outermost* level of this match expression —
            // nested matches inside arm bodies decide independently whether
            // they need their own wrapping.
            let raw = if active_tail.is_some() {
                format!(
                    "match {match_subject} {{\n{}{fallback},\n    }}",
                    arms.join(",\n"),
                )
            } else {
                format!(
                    "(match {match_subject} {{\n{}{fallback},\n    }})",
                    arms.join(",\n"),
                )
            };
            if use_match_deref {
                // The macro tokenises the entire `match { … }` block, so the
                // surrounding parens (added when not in tail position) must be
                // *outside* the macro invocation — otherwise the proc-macro
                // sees a parenthesised expression where it expects `match …`.
                if active_tail.is_some() {
                    format!("::match_deref::match_deref! {{ {raw} }}")
                } else {
                    format!("(::match_deref::match_deref! {{ {} }})",
                        // Strip the outer `(` / `)` we added — re-add them
                        // around the macro call instead.
                        &raw[1..raw.len()-1])
                }
            } else {
                raw
            }
        }
        MatchKind::MatchContinue => {
            // Each arm is an IIFE returning anyhow::Result<T>; first Ok wins.
            // Failures inside an arm (pattern mismatch, ?-propagated errors) drop to next arm.
            let mut s = String::new();
            s.push_str("'mc: {\n");
            // The per-arm let-else uses one of three forms below depending on
            // how the subject was assembled:
            //   * `tuple_arc_rewrite` is Some: __mc_input is a tuple whose
            //     Arc/List components were already turned into `&T` refs at
            //     binding time. Arms destructure via `__mc_input.clone()`
            //     (refs are `Copy`, owned components clone cheaply) and bind
            //     by reference through match ergonomics.
            //   * Subject is `Arc<T>` (recursive uniontype): arms call
            //     `__mc_input.as_ref()` to descend into the inner enum.
            //   * Plain owned value: arms call `__mc_input.clone()`.
            // When any arm of the matchcontinue needs match_deref (e.g. its
            // pattern crosses an Arc edge or matches a string literal), the
            // arms are wrapped per-arm in `::match_deref::match_deref!{ … }`,
            // and the per-arm match needs the *unmodified* tuple as the
            // subject — `Deref @` patterns destructure each `Arc<T>` element
            // directly. The tuple-of-refs `tuple_arc_rewrite` strips the
            // `Arc<T>` to `&T`, which breaks both the per-arm `Deref @` and
            // the binding-by-Arc-clone behaviour the arm bodies depend on.
            // Detect this up front and pick one strategy for all arms.
            let tuple_has_arc_elems = matches!(input, TypedExp::Tuple(es) if es.iter().any(|e| {
                let t = e.ty();
                is_arc_wrapped(&t, ctx) || matches!(t, Ty::List(_))
            }));
            // `tuple_arc_rewrite` strips `Arc<T>` to `&T`, which prevents the
            // arm body from passing the original `Arc<T>` to other functions.
            // For matchcontinue we therefore prefer per-arm `match_deref!{…}`
            // wrapping whenever the tuple contains an `Arc<T>` element — the
            // macro's `Deref @ <pat>` form destructures through the `Arc` AND
            // preserves the binding's outer `Arc<T>` shape for arm bodies.
            // Tuple-of-non-Arc subjects (or scalar Arc subjects) still take
            // the legacy `let pat = __mc_input.as_ref()` path.
            let any_arm_needs_match_deref = tuple_has_arc_elems
                || cases.iter().any(|c|
                    match_uses_match_deref(&input_ty, std::slice::from_ref(c), ctx, top_level));
            let mc_uses_tuple_rewrite: bool = false;
            let _ = needs_tuple_arc_rewrite;
            if mc_uses_tuple_rewrite {
                let TypedExp::Tuple(elems) = input else { unreachable!() };
                let mut parts: Vec<String> = Vec::with_capacity(elems.len());
                for (i, e) in elems.iter().enumerate() {
                    let et = e.ty();
                    let sub = emit_exp(e, is_const, ctx, top_level);
                    if is_arc_wrapped(&et, ctx) || matches!(et, Ty::List(_)) {
                        let n = format!("__mc_arc_{i}");
                        s.push_str(&format!("        let {n} = {sub};\n"));
                        parts.push(format!("{n}.as_ref()"));
                    } else {
                        parts.push(sub);
                    }
                }
                s.push_str(&format!("        let __mc_input = ({});\n", parts.join(", ")));
            } else {
                s.push_str(&format!("        let __mc_input = {input_str};\n"));
            }
            for case in cases {
                // Per-arm match_deref: matchcontinue cannot wrap its overall
                // body in `::match_deref::match_deref!{}` (each arm is a
                // separate `let pat = … else { bail }`), but we *can* wrap a
                // single arm's destructure when its pattern crosses an Arc
                // edge (e.g. `Cons { tail: Nil }` against `Arc<List<T>>`) or
                // matches a `String` literal against an `ArcStr`. The arm
                // then becomes `match_deref!{ match &__mc_input { <pat> => …
                // _ => bail }}` inside the IIFE — the `_` branch falls
                // through to the next matchcontinue arm via `bail!`.
                // Per-arm decision, but escalate to *every* arm when any arm
                // needs match_deref: __mc_input is bound as the raw subject
                // (no `tuple_arc_rewrite`) in that case, so non-match_deref
                // arms would otherwise try to `let pat = __mc_input.as_ref()`
                // on a tuple and fail. Routing every arm through match_deref
                // keeps the binding rules uniform.
                let arm_needs_match_deref = any_arm_needs_match_deref
                    || match_uses_match_deref(&input_ty, std::slice::from_ref(case), ctx, top_level);
                // See the MatchKind::Match path: real-literal patterns
                // synthesize extra guards which must be merged with
                // `case.guard` below.
                ctx.pat_extra_guards.clear();
                let pat = emit_pat_with_implicit_bind_md(&case.pattern, /*allow_implicit_bind=*/true, /*mut_bindings=*/true, /*in_deref=*/false, /*implicit_ref=*/input_is_arc || arm_needs_match_deref, /*in_match_deref=*/arm_needs_match_deref, Some(&input_ty), ctx, top_level);
                let pat_extra_guards: Vec<String> = std::mem::take(&mut ctx.pat_extra_guards);
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
                    // `match e as v case Variant(..) => …`: the as-bound name
                    // also denotes the scrutinee, narrowed to this arm's
                    // variant. Without registering this, `v.field` reads in
                    // the arm body emit as plain `Arc<Enum>.field` accesses
                    // (E0609) instead of `var_field!`.
                    if let Some(name) = as_binding {
                        let inner_pat = match &case.pattern {
                            TypedPat::As { pat: inner, .. } => inner.as_ref(),
                            other => other,
                        };
                        if let Some((enum_q, variant)) = variant_of_pat(inner_pat, &input_ty, top_level) {
                            ctx.variants.insert(name.to_string(), (enum_q, variant));
                        }
                    }
                }
                let guard_check = {
                    let mut parts: Vec<String> = pat_extra_guards.clone();
                    if let Some(g) = case.guard.as_ref() {
                        parts.push(format!("({})", emit_exp(g, is_const, ctx, top_level)));
                    }
                    if parts.is_empty() {
                        String::new()
                    } else {
                        format!("            if !({}) {{ bail!(\"guard\") }}\n", parts.join(" && "))
                    }
                };

                // Case-body statements & locals. MetaModelica `matchcontinue` arms
                // can carry an `algorithm` block (declarations + statements) that
                // executes between pattern destructuring and the `then <result>`
                // expression. Without emitting these the arm would just return
                // the result of an unmodified scrutinee — silently dropping work
                // such as recursive computations into the variable that the
                // result expression references. Mirrors the MatchKind::Match path.
                //
                // Register case-locals and pattern bindings in `ctx.fn_env_vars`
                // for the duration of the arm body so that `emit_exp` recognises
                // them as in-scope locals. Without this, a call whose callee
                // name collides with a case-local (e.g. `annotationFooter` —
                // both a sibling function and a local `String` declared in the
                // arm) is emitted as a bare call that Rust resolves to the
                // local binding. Save/restore around the arm to keep
                // sibling arms isolated.
                let saved_fn_env_vars_mc = ctx.fn_env_vars.clone();
                // Each matchcontinue arm is a fresh closure scope. Arm-local
                // declarations and assignments must not pollute the outer
                // function's initialised-vars map: a same-named local in arm 1
                // being assigned would otherwise make arm 2's shadow logic
                // emit `let mut x: T = x.clone();`, reading the outer (still
                // uninitialised) binding.
                let saved_fn_initialized_vars_mc = ctx.fn_initialized_vars.clone();
                for (n, t, _, _) in &case.locals {
                    if !matches!(t, Ty::Unknown) {
                        ctx.fn_env_vars.insert(n.clone(), t.clone());
                    }
                }
                for (n, t) in typedexp::pat_bindings_with_scrut_ty(&case.pattern, &input_ty) {
                    if !matches!(t, Ty::Unknown) {
                        ctx.fn_env_vars.insert(n, t);
                    }
                }
                let mut body = String::new();
                if !case.stmts.is_empty() || !case.locals.is_empty() {
                    let mut local_env = LocalEnv {
                        vars: ctx.fn_env_vars.clone(),
                        outputs: ctx.fn_outputs.clone(),
                        // See the MatchKind::Match branch above for why we seed
                        // from ctx.variants: nested match arms must inherit the
                        // enclosing arm's variant narrowings so field-assigns
                        // route through the variant-aware macro.
                        variants: ctx.variants.clone(),
                    };
                    record_pattern_variants(&case.pattern, input, &mut local_env, top_level);
                    let pat_binding_names: std::collections::HashSet<String> =
                        typedexp::pat_bindings(&case.pattern).iter().map(|(n, _)| n.clone()).collect();
                    let arm_alias_scope = current_scope_children(ctx, top_level);
                    for (name, ty, default, type_spec) in &case.locals {
                        local_env.vars.insert(name.clone(), ty.clone());
                        if pat_binding_names.contains(name) {
                            continue;
                        }
                        // Skip declarations for match-level locals not
                        // referenced in this arm. See `case_uses_local_name`
                        // and the parallel filter on the MatchKind::Match
                        // path above.
                        if default.is_none() && !case_uses_local_name(case, name) {
                            continue;
                        }
                        if matches!(ty, Ty::Unknown) {
                            body.push_str(&format!("            let mut {}; // TODO: local with unresolved type\n", escape_ident(name)));
                            continue;
                        }
                        let ty_s = type_spec.as_ref()
                            .and_then(|ts| arm_alias_scope.and_then(|sc| field_type_alias_name(ts, sc)))
                            .unwrap_or_else(|| fmt_ty(ty, ctx));
                        match default {
                            Some(d) => {
                                let init = emit_exp(d, is_const, ctx, top_level);
                                let init_ty = d.ty();
                                if let Some(line) = emit_multi_output_let(true, name, ty, &init_ty, &init, "            ", ctx) {
                                    body.push_str(&line);
                                } else {
                                    body.push_str(&format!("            let mut {}: {ty_s} = {init};\n", escape_ident(name)));
                                }
                            }
                            None => {
                                body.push_str(&format!("            let mut {}: {ty_s};\n", escape_ident(name)));
                            }
                        }
                    }
                    for (n, t) in typedexp::pat_bindings(&case.pattern) {
                        local_env.vars.insert(n, t);
                    }
                    let mut deref_names: Vec<String> = Vec::new();
                    if arm_needs_match_deref {
                        // Per-arm match_deref wrapping forces every pattern
                        // binding into reference position (see the
                        // `use_match_deref` branch in the MatchKind::Match
                        // path for the same reasoning).
                        pat_collect_all_bindings(&case.pattern, &mut deref_names);
                    } else {
                        pat_deref_bindings(&case.pattern, &input_ty, ctx, top_level, &mut deref_names);
                    }
                    let mut assigned: HashSet<String> = HashSet::new();
                    stmts_assigned_var_names(&case.stmts, &mut assigned);
                    if !deref_names.is_empty() {
                        for n in &deref_names {
                            if assigned.contains(n) {
                                let id = escape_ident(n);
                                if arm_needs_match_deref {
                                    body.push_str(&format!("            let mut {id} = (*{id}).clone();\n"));
                                } else {
                                    body.push_str(&format!("            let mut {id} = {id}.clone();\n"));
                                }
                            }
                        }
                    }
                    // Shadow any *function-scope* variable that this arm assigns
                    // to. Each arm is wrapped in an IIFE; the closure would
                    // otherwise capture the outer variable by `&mut`, which the
                    // borrow checker rejects if the outer is declared but not
                    // initialised (`let mut x: T; x = 'mc: { ... };`). A local
                    // shadow inside the IIFE makes assignments and subsequent
                    // reads refer to the shadow, so the outer needs no prior
                    // init. The matchcontinue's value is still returned via
                    // `Ok(...)` and assigned to the outer `'mc:` block target.
                    //
                    // We skip names that are already pattern bindings (those are
                    // owned bindings introduced by the destructuring `let`),
                    // case-local declarations, or `deref!`-shadowed locals
                    // (handled above), since each of those already introduces
                    // an in-scope local.
                    let mut shadow_seen: HashSet<String> = HashSet::new();
                    for (case_local_name, _, _, _) in &case.locals { shadow_seen.insert(case_local_name.clone()); }
                    for n in &deref_names { shadow_seen.insert(n.clone()); }
                    for (n, _) in typedexp::pat_bindings(&case.pattern) { shadow_seen.insert(n); }
                    for name in &assigned {
                        if shadow_seen.contains(name) { continue; }
                        let Some(ty) = ctx.fn_env_vars.get(name).cloned() else { continue };
                        let id = escape_ident(name);
                        // The shadow carries the outer value in (`let mut x =
                        // x.clone();`) when the outer binding is known to have
                        // been initialised at its declaration — i.e. it's a
                        // function input, or a local declared with `= init`.
                        // For bare `let mut x: T;` outer locals the outer is
                        // uninitialised at the point the matchcontinue runs
                        // when the matchcontinue itself is the first writer:
                        // a shadow `= x.clone()` there would use uninitialised
                        // memory. The bare-shadow fallback lets arms freely
                        // assign such locals (matching MM matchcontinue
                        // semantics where the arm body's writes propagate to
                        // the outer scope via the `'mc:` block's tuple return).
                        let init_from_outer = ctx.fn_initialized_vars.contains(name);
                        if matches!(ty, Ty::Unknown) {
                            if init_from_outer {
                                body.push_str(&format!("            let mut {id} = {id}.clone(); // TODO: shadow of function-scope input with unresolved type\n"));
                            } else {
                                body.push_str(&format!("            let mut {id}; // TODO: shadow of function-scope local with unresolved type\n"));
                            }
                        } else {
                            let ty_s = fmt_ty(&ty, ctx);
                            if init_from_outer {
                                body.push_str(&format!("            let mut {id}: {ty_s} = {id}.clone();\n"));
                            } else {
                                body.push_str(&format!("            let mut {id}: {ty_s};\n"));
                            }
                        }
                    }
                    let mut fresh_local: u32 = 0;
                    // FailureMode::Function: a `fail()` inside the IIFE expands
                    // to `bail!()`, which makes the closure return Err — the
                    // outer `if let Ok(__v) = ...` then skips to the next arm,
                    // which matches MetaModelica matchcontinue semantics.
                    emit_stmts(&mut body, "            ", &case.stmts, FailureMode::Function, ctx, &mut local_env, top_level, &mut fresh_local);
                }
                let result = emit_exp(&case.result, is_const, ctx, top_level);
                s.push_str("        if let Ok(__v) = (|| -> Result<_> {\n");
                // Whole-input Var binding under an Arc-wrapped scrutinee: bind
                // directly to the Arc instead of going through `.as_ref()`.
                // The default path (`.as_ref()` + match ergonomics) is needed
                // when the pattern destructures into an enum variant, because
                // the variant patterns can only match `&T`, not `Arc<T>`. But
                // a bare `Var` doesn't destructure — and routing it through
                // `.as_ref()` would bind the name as `&T`, so a downstream
                // `name.clone()` would clone `T` instead of the `Arc<T>` the
                // formal parameter type expects. (Wildcard: same reasoning,
                // but no name needs to keep the Arc, so `.as_ref()` is fine.)
                let bind_arc_directly = input_is_arc && matches!(&case.pattern, TypedPat::Var(_));
                if arm_needs_match_deref {
                    // Wrap the arm in `::match_deref::match_deref!{ match &__mc_input { … } }`.
                    // Inside the macro, `Deref @ <inner>` derefs an `Arc<T>` /
                    // `ArcStr` binding into its target so that patterns like
                    // `Cons { tail: Deref @ Nil }` and `Deref @ "str"` match
                    // through the smart pointer. The `_ => bail!("nomatch")`
                    // fallback drops out of the IIFE on a mismatch, so the
                    // next matchcontinue arm runs — preserving the
                    // try-each-arm semantics of `matchcontinue`.
                    s.push_str("            ::match_deref::match_deref! { match &__mc_input {\n");
                    s.push_str(&format!("                {pat} => {{\n"));
                    s.push_str(&guard_check.replace("            ", "                    "));
                    s.push_str(&body.replace("            ", "                    "));
                    s.push_str(&format!("                    Ok({result})\n"));
                    s.push_str("                }\n");
                    s.push_str("                _ => bail!(\"nomatch\"),\n");
                    s.push_str("            }}\n");
                } else {
                    if bind_arc_directly {
                        let var_name = match &case.pattern {
                            TypedPat::Var(n) => escape_ident(n),
                            _ => unreachable!(),
                        };
                        let mut_prefix = if true { "mut " } else { "" };
                        s.push_str(&format!("            let {mut_prefix}{var_name} = __mc_input.clone();\n"));
                    } else if mc_uses_tuple_rewrite {
                        s.push_str(&format!("            let {pat} = __mc_input.clone() else {{ bail!(\"nomatch\") }};\n"));
                    } else if input_is_arc {
                        s.push_str(&format!("            let {pat} = __mc_input.as_ref() else {{ bail!(\"nomatch\") }};\n"));
                    } else {
                        s.push_str(&format!("            let {pat} = __mc_input.clone() else {{ bail!(\"nomatch\") }};\n"));
                    }
                    s.push_str(&guard_check);
                    s.push_str(&body);
                    s.push_str(&format!("            Ok({result})\n"));
                }
                s.push_str("        })() { break 'mc __v; }\n");
                ctx.variants = saved_variants;
                ctx.variant_shapes = saved_shapes;
                ctx.fn_env_vars = saved_fn_env_vars_mc;
                ctx.fn_initialized_vars = saved_fn_initialized_vars_mc;
            }
            s.push_str("        bail!(\"matchcontinue: no arm matched\")\n");
            s.push_str("    }");
            s
        }
    };
    match as_prefix {
        Some(prefix) => format!("{prefix}{body_str} }}"),
        None => body_str,
    }
}

fn emit_pat<'a>(pat: &TypedPat, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    emit_pat_with_implicit_bind(pat, /*allow_implicit_bind=*/true, /*mut_bindings=*/false, /*in_deref=*/false, /*implicit_ref=*/false, None, ctx, top_level)
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

/// True when at least one pattern subtree contains a `Lit::Str(_)` literal.
/// String literals in patterns always require `match_deref!{ ... }` because
/// `ArcStr` (the lowered representation of MetaModelica `String`) is a
/// `Deref<Target = str>` smart pointer and the literal lives behind that
/// deref. We use this as one of the triggers for the outer `emit_match` to
/// switch into match_deref mode.
fn pat_has_str_lit(pat: &TypedPat) -> bool {
    match pat {
        TypedPat::Lit(Lit::Str(_)) => true,
        TypedPat::Some_(inner) => pat_has_str_lit(inner),
        TypedPat::As { pat: inner, .. } => pat_has_str_lit(inner),
        TypedPat::Cons { head, tail } => pat_has_str_lit(head) || pat_has_str_lit(tail),
        TypedPat::Tuple(ps) => ps.iter().any(pat_has_str_lit),
        TypedPat::Constructor { fields, named_fields, .. } => {
            fields.iter().any(pat_has_str_lit)
                || named_fields.iter().any(|(_, p)| pat_has_str_lit(p))
        }
        _ => false,
    }
}

/// Decide whether the match expression's outer scrutinee needs to be wrapped
/// in `match_deref!{ ... }`. We switch into match_deref mode whenever any of
/// the following holds for the input or its destructured sub-elements:
///
///   * The scrutinee is `Arc<…>` over a recursive uniontype (`is_arc_wrapped`).
///   * The scrutinee is `Arc<metamodelica::List<T>>` (every `Ty::List`).
///   * A `(_, _, …)` tuple scrutinee contains at least one Arc-crossing
///     element — `type_destructure_needs_borrow` already recurses through
///     tuples.
///   * A case pattern contains a string literal — see `pat_has_str_lit`.
///
/// When this returns true, `emit_match` uses `&(subject)` as the match
/// expression (so the macro can call `Deref::deref(binding)` against a `&T`)
/// AND passes `implicit_ref = true` to `emit_pat_with_implicit_bind`. The
/// pattern emitter then prefixes each Arc-crossing variant pattern with
/// `::match_deref::Deref @ `.
fn match_uses_match_deref(input_ty: &Ty, cases: &[TypedCase], ctx: &GenCtx, top_level: &BTreeMap<String, NameNode<'_>>) -> bool {
    if type_destructure_needs_borrow(input_ty, ctx) {
        return true;
    }
    cases.iter().any(|c| pat_has_str_lit(&c.pattern) || pat_crosses_arc_edge(&c.pattern, input_ty, ctx, top_level))
}

/// True when destructuring `pat` against a scrutinee of type `scrut_ty`
/// would require descending through an `Arc<…>` edge — e.g. a constructor
/// pattern whose field type is `Arc<List<…>>` and whose sub-pattern is
/// anything other than a bare `_` / variable binding (which doesn't
/// destructure further). When this returns true the match must be wrapped
/// in `::match_deref::match_deref!{ … }` so the inner pattern can match
/// through the Arc, even when the outer scrutinee type itself does not need
/// a borrow.
fn pat_crosses_arc_edge(pat: &TypedPat, scrut_ty: &Ty, ctx: &GenCtx, top_level: &BTreeMap<String, NameNode<'_>>) -> bool {
    fn sub_destructures(p: &TypedPat) -> bool {
        !matches!(p, TypedPat::Wildcard | TypedPat::Var(_))
    }
    // A Constructor / Cons / EmptyList pattern matched against an Arc-wrapped
    // (or List, which is itself Arc<List<...>>) scrutinee must peel the smart
    // pointer with `Deref @ ...`. This applies whether the Arc edge is the
    // outer scrutinee or sits behind a Some/Cons.tail; the caller already
    // recurses with the inner type, so checking the *current* scrutinee here
    // covers every Arc edge uniformly.
    if matches!(pat, TypedPat::Constructor { .. } | TypedPat::Cons { .. } | TypedPat::EmptyList)
        && ty_needs_arc_match_deref(scrut_ty, ctx)
    {
        return true;
    }
    match pat {
        TypedPat::Constructor { name, fields, named_fields, .. } => {
            let field_tys = record_field_tys(name, top_level)
                .or_else(|| lookup_record_through_unions(name, top_level)
                    .and_then(|(canonical, _)| record_field_tys(&canonical, top_level)))
                .or_else(|| record_field_tys_from_scrutinee_ctor(name, scrut_ty, top_level))
                .unwrap_or_else(|| {
                    let simple = name.rsplit_once('.').map_or(name.as_str(), |(_, s)| s);
                    record_field_tys_by_simple_name(simple, top_level)
                });
            let by_name = |fname: &str| -> Option<Ty> {
                field_tys.iter().find(|(n, _)| n == fname).map(|(_, t)| t.clone())
            };
            let by_idx = |i: usize| -> Option<Ty> {
                field_tys.get(i).map(|(_, t)| t.clone())
            };
            for (i, fp) in fields.iter().enumerate() {
                let fty = by_idx(i).unwrap_or(Ty::Unknown);
                if sub_destructures(fp) && type_destructure_needs_borrow(&fty, ctx) {
                    return true;
                }
                if pat_crosses_arc_edge(fp, &fty, ctx, top_level) {
                    return true;
                }
            }
            for (fname, fp) in named_fields {
                let fty = by_name(fname).unwrap_or(Ty::Unknown);
                if sub_destructures(fp) && type_destructure_needs_borrow(&fty, ctx) {
                    return true;
                }
                if pat_crosses_arc_edge(fp, &fty, ctx, top_level) {
                    return true;
                }
            }
            false
        }
        TypedPat::Cons { head, tail } => {
            let elem_ty = match scrut_ty { Ty::List(t) => (**t).clone(), _ => Ty::Unknown };
            if sub_destructures(tail) { return true; }
            pat_crosses_arc_edge(head, &elem_ty, ctx, top_level)
                || pat_crosses_arc_edge(tail, &Ty::List(Box::new(elem_ty)), ctx, top_level)
        }
        TypedPat::Tuple(ps) => {
            let elem_tys: Vec<Ty> = match scrut_ty {
                Ty::Tuple(ts) if ts.len() == ps.len() => ts.clone(),
                _ => vec![Ty::Unknown; ps.len()],
            };
            ps.iter().zip(elem_tys.iter()).any(|(p, t)| pat_crosses_arc_edge(p, t, ctx, top_level))
        }
        TypedPat::Some_(inner) => {
            let inner_ty = match scrut_ty { Ty::Option(t) => (**t).clone(), _ => Ty::Unknown };
            pat_crosses_arc_edge(inner, &inner_ty, ctx, top_level)
        }
        TypedPat::As { pat, .. } => pat_crosses_arc_edge(pat, scrut_ty, ctx, top_level),
        _ => false,
    }
}

/// Recursively check whether any `TypedExp::Match` reachable from `exp` would
/// require `match_deref!{…}` wrapping (per [`match_uses_match_deref`]).
///
/// Used at function-emission time to decide between two mutually exclusive
/// lowering strategies: `#[tailcall::tailcall]` (which forbids the macro
/// because the rewriter cannot see inside arms hidden behind `match_deref!`)
/// versus the `match_deref!` form. When the body needs the macro, we drop
/// the tail-call plan in favour of plain recursion.
fn exp_needs_match_deref(exp: &TypedExp, ctx: &GenCtx, top_level: &BTreeMap<String, NameNode<'_>>) -> bool {
    match exp {
        TypedExp::Match { input, cases, .. } => {
            let input_ty = input.ty();
            if match_uses_match_deref(&input_ty, cases, ctx, top_level) {
                return true;
            }
            if exp_needs_match_deref(input, ctx, top_level) {
                return true;
            }
            cases.iter().any(|c| {
                c.guard.as_ref().is_some_and(|g| exp_needs_match_deref(g, ctx, top_level))
                    || exp_needs_match_deref(&c.result, ctx, top_level)
                    || stmts_need_match_deref(&c.stmts, ctx, top_level)
                    || c.locals.iter().any(|(_, _, d, _)| {
                        d.as_ref().is_some_and(|e| exp_needs_match_deref(e, ctx, top_level))
                    })
            })
        }
        TypedExp::BinOp { lhs, rhs, .. } => {
            exp_needs_match_deref(lhs, ctx, top_level) || exp_needs_match_deref(rhs, ctx, top_level)
        }
        TypedExp::UnOp { operand, .. } => exp_needs_match_deref(operand, ctx, top_level),
        TypedExp::Call { args, named_args, .. }
        | TypedExp::Constructor { args, named_args, .. }
        | TypedExp::PartEval { args, named_args, .. } => {
            args.iter().any(|e| exp_needs_match_deref(e, ctx, top_level))
                || named_args.iter().any(|(_, e)| exp_needs_match_deref(e, ctx, top_level))
        }
        TypedExp::If { cond, then_, elseif, else_, .. } => {
            exp_needs_match_deref(cond, ctx, top_level)
                || exp_needs_match_deref(then_, ctx, top_level)
                || elseif.iter().any(|(c, b)| {
                    exp_needs_match_deref(c, ctx, top_level) || exp_needs_match_deref(b, ctx, top_level)
                })
                || exp_needs_match_deref(else_, ctx, top_level)
        }
        TypedExp::Cons { head, tail, .. } => {
            exp_needs_match_deref(head, ctx, top_level) || exp_needs_match_deref(tail, ctx, top_level)
        }
        TypedExp::Tuple(es) | TypedExp::Array { elems: es, .. } => {
            es.iter().any(|e| exp_needs_match_deref(e, ctx, top_level))
        }
        TypedExp::Range { start, step, stop, .. } => {
            exp_needs_match_deref(start, ctx, top_level)
                || step.as_ref().is_some_and(|s| exp_needs_match_deref(s, ctx, top_level))
                || exp_needs_match_deref(stop, ctx, top_level)
        }
        TypedExp::Reduction { body, iterators, .. } => {
            exp_needs_match_deref(body, ctx, top_level)
                || iterators.iter().any(|it| {
                    exp_needs_match_deref(&it.range, ctx, top_level)
                        || it.guard.as_ref().is_some_and(|g| exp_needs_match_deref(g, ctx, top_level))
                })
        }
        TypedExp::Lit(_) | TypedExp::Var { .. } | TypedExp::Todo(_) => false,
    }
}

/// Mirrors [`exp_needs_match_deref`] for statement lists; see that function
/// for the rationale.
fn stmts_need_match_deref(stmts: &[typedexp::TypedStmt], ctx: &GenCtx, top_level: &BTreeMap<String, NameNode<'_>>) -> bool {
    use typedexp::TypedStmt as S;
    stmts.iter().any(|s| match s {
        S::Assign { rhs, .. } => exp_needs_match_deref(rhs, ctx, top_level),
        S::NoRetCall { call } => exp_needs_match_deref(call, ctx, top_level),
        S::If { cond, then_, elseif, else_ } => {
            exp_needs_match_deref(cond, ctx, top_level)
                || stmts_need_match_deref(then_, ctx, top_level)
                || elseif.iter().any(|(c, b)| exp_needs_match_deref(c, ctx, top_level) || stmts_need_match_deref(b, ctx, top_level))
                || stmts_need_match_deref(else_, ctx, top_level)
        }
        S::For { range, body, .. } => {
            exp_needs_match_deref(range, ctx, top_level) || stmts_need_match_deref(body, ctx, top_level)
        }
        S::While { cond, body } => {
            exp_needs_match_deref(cond, ctx, top_level) || stmts_need_match_deref(body, ctx, top_level)
        }
        S::Try { body, else_body } => {
            stmts_need_match_deref(body, ctx, top_level) || stmts_need_match_deref(else_body, ctx, top_level)
        }
        S::Failure { body } => stmts_need_match_deref(body, ctx, top_level),
        S::Return | S::Break | S::Continue | S::Todo(_) => false,
    })
}

/// Collect every binding name in `pat` that lies inside an Arc-deref path
/// (across an `Arc<T>` edge) when the pattern is matched against a value of
/// type `scrut_ty`. Such bindings *cannot* be `mut` directly: the
/// `deref_patterns` feature yields references through the `Arc`, and `mut`
/// on a by-reference binding is rejected by the Rust compiler. We emit
/// `ref <name>` for them in the pattern, and then optionally re-bind them
/// with `let mut <name> = <name>.clone();` in the arm body if the source
/// MetaModelica reassigns them.
/// Collect every binding name in `pat`, regardless of whether the binding
/// crosses an Arc edge. Used when the enclosing match is wrapped in
/// `::match_deref::match_deref!{ match &(<subj>) { … } }`: the leading `&`
/// forces every pattern binding into reference position via match
/// ergonomics, so reassign-shadow rebinding has to consider all names —
/// not just those `pat_deref_bindings` reports.
fn pat_collect_all_bindings(pat: &TypedPat, out: &mut Vec<String>) {
    match pat {
        TypedPat::Var(name) => out.push(name.clone()),
        TypedPat::Some_(inner) => pat_collect_all_bindings(inner, out),
        TypedPat::Cons { head, tail } => {
            pat_collect_all_bindings(head, out);
            pat_collect_all_bindings(tail, out);
        }
        TypedPat::Tuple(pats) => pats.iter().for_each(|p| pat_collect_all_bindings(p, out)),
        TypedPat::Constructor { fields, named_fields, .. } => {
            fields.iter().for_each(|p| pat_collect_all_bindings(p, out));
            named_fields.iter().for_each(|(_, p)| pat_collect_all_bindings(p, out));
        }
        TypedPat::As { var, pat } => {
            out.push(var.clone());
            pat_collect_all_bindings(pat, out);
        }
        _ => {}
    }
}

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
        TypedPat::As { var, pat } => {
            // Mirror `emit_pat_with_implicit_bind`'s `force_ref`: when the
            // inner pattern introduces a sub-binding, the @-bound `var` is
            // emitted as `ref var` to avoid a move-overlap with that
            // sub-binding. That means `var` is `&T` and any subsequent
            // assignment (including `var.field := …`) needs an owned
            // `let mut var = var.clone();` rebind. Treat the @-bound name
            // like a deref binding so the existing rebind path picks it up.
            if pat_introduces_binding(pat) {
                out.push(var.clone());
            }
            pat_deref_bindings(pat, scrut_ty, ctx, top_level, out);
        }
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
        // `base.field := rhs` is a destructive mutation of `base`'s value, so
        // record `base` as assigned. Without this, an outer `ref`-bound base
        // would never be re-rebinned to an owned `mut` shadow, and
        // `assign_variant_field!` would later emit `&mut base` against an
        // immutable binding (E0596 / E0594).
        TypedPat::FieldAccess { base, .. } => pat_assigned_names(base, out),
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
fn emit_pat_with_implicit_bind<'a>(pat: &TypedPat, allow_implicit_bind: bool, mut_bindings: bool, in_deref: bool, implicit_ref: bool, scrut_ty: Option<&Ty>, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    // Outer wrapper that omits the new `in_match_deref` flag; existing call
    // sites that don't yet know about match_deref get the legacy behaviour
    // (no `Deref @` prefix, no string-literal-via-Deref).
    emit_pat_with_implicit_bind_md(pat, allow_implicit_bind, mut_bindings, in_deref, implicit_ref, /*in_match_deref=*/false, scrut_ty, ctx, top_level)
}

#[allow(clippy::too_many_arguments)]
fn emit_pat_with_implicit_bind_md<'a>(pat: &TypedPat, allow_implicit_bind: bool, mut_bindings: bool, in_deref: bool, implicit_ref: bool, in_match_deref: bool, scrut_ty: Option<&Ty>, ctx: &mut GenCtx, top_level: &'a BTreeMap<String, NameNode<'a>>) -> String {
    // Two distinct match-ergonomics regimes — see also `emit_match`:
    //
    // 1. `implicit_ref`: the outer subject was wrapped in `.as_ref()` by
    //    `emit_match` because it's a recursive Arc-wrapped uniontype. The
    //    scrutinee is `&T`, default binding mode is `ref`, and Rust 2024
    //    rejects an explicit `ref <name>` (E0658, "cannot explicitly borrow
    //    within an implicitly-borrowing pattern"). Emit bare names. Once
    //    set, it propagates to every nested sub-pattern.
    //
    // 2. `in_deref` (regime 2): subject is `Arc<List<T>>` matched directly
    //    via `deref_patterns`, or we're inside the `Cons.tail` of one. A
    //    bare `head: x` would try to move from behind the Arc (E0507), so
    //    we emit explicit `ref x`.
    //
    // `implicit_ref` only enters via the outer `emit_match` call (which
    // tracks whether the match is wrapped in `match_deref!{ ... }`). Don't
    // re-derive it from inner scrut_ty: a nested `(Arc<List<T>>, Arc<Tree>)`
    // tuple scrutinee is *not* implicit-borrowed even though its second
    // element is a recursive Arc-wrapped uniontype — `emit_match` decides at
    // the outer level whether match_deref is in scope. Just propagate what
    // the caller told us.
    let in_deref = in_deref
        || (!implicit_ref && scrut_ty.map(|t| ty_needs_arc_match_deref(t, ctx)).unwrap_or(false));
    let bind_var = |name: &str| -> String {
        if implicit_ref {
            escape_ident(name)
        } else if in_deref {
            format!("ref {}", escape_ident(name))
        } else if mut_bindings {
            format!("mut {}", escape_ident(name))
        } else {
            escape_ident(name)
        }
    };
    // Inside `match_deref!{ match &(subject) { ... } }` (signalled by
    // `implicit_ref = true`), an `Arc<T>` value matched against the inner
    // `T`'s constructor pattern must be peeled with `::match_deref::Deref @`.
    // Likewise an `ArcStr` value matched against a string literal — ArcStr is
    // a Deref smart pointer to `str`, so the literal lives behind the deref.
    // Outside a match_deref scope (`implicit_ref = false`) these prefixes are
    // invalid (`Deref @ ...` is only meaningful inside the macro), so we fall
    // back to the legacy plain patterns there; the caller is expected to set
    // `implicit_ref` whenever the match contains any Arc edge or string-literal
    // pattern (see `match_uses_match_deref` in `emit_match`).
    let at_arc_edge = scrut_ty.map(|t| ty_needs_arc_match_deref(t, ctx)).unwrap_or(false);
    // `match_deref!` recognises the literal token `Deref` in pattern position
    // (it looks for an `i.ident == "Deref"` syn::PatIdent) and rewrites the
    // `Deref @ <subpat>` into an `if let <subpat> = ::core::ops::Deref::deref(binding)`
    // guard. The token must therefore be emitted as the bare identifier `Deref` —
    // a path like `::match_deref::Deref @ …` is parsed as `PatPath` rather than
    // `PatIdent` and the macro silently skips it (the resulting pattern then
    // fails to compile with "expected `,`"). No `use match_deref::Deref;` is
    // needed at the crate root because the macro hardcodes
    // `::core::ops::Deref::deref` in its expansion.
    //
    // The `Deref @` prefix is only valid inside a `match_deref!{ … }` block
    // (`in_match_deref = true`). For the legacy `.as_ref()` path (used in
    // `#[tailcall::tailcall]` bodies where match_deref would hide arms from
    // the tailcall rewriter), the OUTER scrutinee has already been peeled
    // by `.as_ref()` — and inner Arc edges (e.g. `Cons.tail` typed
    // `Arc<List<T>>`) are not destructured further by the legacy lowering,
    // so no Deref prefix is needed there either. Hence we gate the prefix
    // on `in_match_deref` alone, not on `implicit_ref`.
    let arc_prefix: &str = if in_match_deref && at_arc_edge { "Deref @ " } else { "" };
    match pat {
        TypedPat::Wildcard    => "_".to_owned(),
        TypedPat::Var(name)   => bind_var(name),
        TypedPat::EmptyList   => format!("{arc_prefix}metamodelica::List::Nil"),
        TypedPat::Some_(inner) => {
            // Propagate the Option's inner type into the sub-pattern so it can
            // decide whether a `Deref @` prefix is required at an Arc edge
            // (`Option<Arc<T>>` is the common case for uniontype fields).
            let inner_scrut = match scrut_ty { Some(Ty::Option(t)) => Some(&**t), _ => None };
            format!("Some({})", emit_pat_with_implicit_bind_md(inner, allow_implicit_bind, mut_bindings, in_deref, implicit_ref, in_match_deref, inner_scrut, ctx, top_level))
        }
        TypedPat::None_       => "None".to_owned(),

        TypedPat::Lit(Lit::Int(v))  => {
            if *v < 0 { format!("({v})") } else { v.to_string() }
        }
        TypedPat::Lit(Lit::Bool(v)) => v.to_string(),
        TypedPat::Lit(Lit::Str(s))  => {
            // String literal patterns match against `ArcStr` (always — every
            // MetaModelica `String` lowers to `arcstr::ArcStr`). `ArcStr`
            // implements `Deref<Target = str>`, and `match_deref!` translates
            // `Deref @ "abc"` into an `if let "abc" = Deref::deref(binding)`
            // guard. Outside a match_deref scope we have no way to make a
            // string literal match an `ArcStr`, so we emit a TODO marker that
            // the user can grep for — better than silently emitting a
            // wildcard that would let the wrong arm fire.
            let decoded = unescape_mm_string(s);
            if in_match_deref {
                format!("Deref @ {decoded:?}")
            } else {
                format!("_ /* TODO: string literal pattern {decoded:?} requires the enclosing match to use match_deref!{{ ... }} */")
            }
        }
        TypedPat::Lit(Lit::Real(v)) => {
            // Rust does not permit `f64` literal patterns directly
            // (deprecated since edition 2024). Lower the literal to a
            // fresh binding plus an equality guard that the surrounding
            // match-arm emitter will splice into the arm's guard
            // (alongside `case.guard`). The binding name is unique
            // per-codegen so it cannot collide with user names.
            //
            // The guard uses `.eq(&(<v>) as f64)` rather than `==` so
            // it works regardless of the binding mode: `match_deref!`
            // / `&`-rooted matches bind struct fields as `&f64`,
            // whereas plain matches bind by copy as `f64`. Method-call
            // resolution auto-refs/derefs the receiver in both cases.
            //
            // Wrapping `<v>` in `(<...>) as f64` keeps the literal
            // syntactically self-contained: it tolerates exponent or
            // leading-minus forms (`1e-3`, `-1.0`) that don't accept
            // a bare `_f64` suffix.
            let id = ctx.pat_lit_counter;
            ctx.pat_lit_counter += 1;
            let name = format!("__rlit_{id}");
            // Compare against `metamodelica::Real` (OrderedFloat<f64>). `.eq` is
            // chosen over `==` for the same auto-ref/auto-deref reason described
            // above; both binding modes (`&Real` from match_deref!, copy from a
            // plain match) resolve through method-call auto-deref.
            ctx.pat_extra_guards.push(format!("{name}.eq(&metamodelica::OrderedFloat(({v}) as f64))"));
            name
        }

        TypedPat::Cons { head, tail } => {
            // Element type: pull from scrut_ty when known so any sub-pattern
            // that itself crosses an Arc edge (e.g. an element which is a
            // recursive uniontype) gets `in_deref` set correctly.
            let elem_ty: Ty = match scrut_ty { Some(Ty::List(t)) => (**t).clone(), _ => Ty::Unknown };
            // The `tail` field of `metamodelica::List::Cons` is `Arc<List<T>>`
            // (itself an Arc edge), so emit the tail sub-pattern with a
            // synthetic `Ty::List(elem)` scrutinee. Inside `match_deref!`
            // this triggers another `::match_deref::Deref @` prefix on the
            // tail's variant pattern; outside it forces `ref <name>` binding
            // (the legacy Arc<List> behavior).
            let tail_ty = Ty::List(Box::new(elem_ty.clone()));
            let body = format!("metamodelica::List::Cons {{ head: {}, tail: {} }}",
                emit_pat_with_implicit_bind_md(head, allow_implicit_bind, mut_bindings, in_deref, implicit_ref, in_match_deref, Some(&elem_ty), ctx, top_level),
                emit_pat_with_implicit_bind_md(tail, allow_implicit_bind, mut_bindings, true, implicit_ref, in_match_deref, Some(&tail_ty), ctx, top_level));
            format!("{arc_prefix}{body}")
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
                    emit_pat_with_implicit_bind_md(p, /*allow_implicit_bind=*/false, mut_bindings, in_deref, implicit_ref, in_match_deref, elem_ty, ctx, top_level)
                })
                .collect();
            format!("({})", parts.join(", "))
        }

        TypedPat::Constructor { name, fields, named_fields, ty, .. } => {
            // The Constructor arm emits a record-style pattern (`Foo { f: p, .. }`)
            // or a bare-name variant. When the value being matched is an
            // `Arc<Enum>` (recursive uniontype) AND we're inside a `match_deref!`
            // block, the variant pattern lives behind the Arc and needs a
            // `::match_deref::Deref @` prefix; that's what `arc_prefix` carries.
            // We compute the inner pattern body first then apply the prefix
            // uniformly at the end, instead of sprinkling it through every
            // branch (and the early-return path for empty patterns).
            // Resolve to the canonical Rust path. For a single-record uniontype
            // the record gets folded onto the parent name (Absyn.Comment.COMMENT
            // becomes the struct `Absyn::Comment`), and typed inference has
            // already collapsed it onto `Ty::RustStruct(parent_qname)`. Use that
            // when available so the pattern path matches the struct's Rust path
            // exactly; fall back to shortening the raw constructor name when
            // typing left the variant as RustEnum / Unknown.
            // When `ty` didn't resolve to a RustStruct (e.g. RustEnum or Unknown),
            // detect the single-record uniontype shape directly from the hierarchy:
            // if `name`'s parent is a uniontype with exactly one record child, the
            // record was folded into the parent struct's Rust path (no enum
            // wrapper), so shorten the parent qname rather than the record's. Without
            // this the pattern emits `Parent::Record` which doesn't exist (E0223).
            // Resolve `name` to its canonical hierarchy path so the emitted
            // Rust path reflects nested-uniontype structure (e.g.
            // `ClassTree.FLAT_TREE` → `NFClassTree.ClassTree.FLAT_TREE` once
            // import aliases and the inner `uniontype ClassTree` are walked).
            // Used both by the folded-parent detection below (so it can locate
            // a uniontype parent reachable only via import alias) and by the
            // shorten-name path further down.
            let pkg_prefix_for_lookup = if ctx.current_path.is_empty() {
                ctx.top_name.clone()
            } else {
                format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
            };
            let canonical_ctor_name: String = if name.contains('.') {
                crate::typedexp::resolve_call_node(name, top_level, &pkg_prefix_for_lookup)
                    .map(|(q, _)| q)
                    .or_else(|| lookup_record_through_unions(name, top_level).map(|(q, _)| q))
                    .unwrap_or_else(|| name.clone())
            } else {
                name.clone()
            };
            let folded_parent_qname: Option<String> = (|| {
                if !canonical_ctor_name.contains('.') { return None; }
                let parent = canonical_ctor_name.rsplit_once('.').map(|(p, _)| p.to_owned())?;
                let parent_node = lookup_node(&parent, top_level)?;
                let NodeKind::Class(c) = &parent_node.kind else { return None };
                if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) { return None; }
                if uniontype_needs_mod(parent_node) { return None; }
                let record_count = parent_node.children.values().filter(|ch| {
                    matches!(&ch.kind, NodeKind::Class(cc)
                        if matches!(cc.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. }))
                }).count();
                if record_count == 1 { Some(parent) } else { None }
            })();
            // Helper: shorten a struct qname, then append `::<inner>` when the
            // qname is a `pub mod`-wrapped uniontype whose only record was
            // folded into a struct of the same name (e.g. `NFFunction.Slot`
            // → mod `Slot { pub struct Slot; pub type SLOT = Slot; }`). A bare
            // shortened path (`Slot`) names the module, not the struct — the
            // pattern path must be `Slot::Slot` (or `Slot::SLOT`, both work via
            // the type alias). For uniontypes without a mod wrapper (records
            // only, no nested functions/constants), the shorten alone is
            // correct since the parent struct lives directly in the parent
            // scope.
            let shorten_struct_qname = |ctx: &mut GenCtx, qname: &str, ctor_name: &str| -> String {
                let short = ctx.shorten(qname);
                if let Some(parent_node) = lookup_node(qname, top_level)
                    && let NodeKind::Class(c) = &parent_node.kind
                    && matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE)
                    && uniontype_needs_mod(parent_node)
                {
                    // `emit_uniontype` skips the `pub mod` wrapper when the
                    // uniontype is the file's top-level name (e.g. `SBInterval`
                    // declared inside `SBInterval.mo`): the struct lives
                    // directly at the file's top scope, not inside a same-named
                    // mod. Don't double the segment in that case.
                    let parent_simple = qname.rsplit('.').next().unwrap_or(qname);
                    let qname_top = qname.split('.').next().unwrap_or(qname);
                    let is_file_top = qname_top == parent_simple && qname_top == ctx.top_name;
                    // Already inside the parent's `pub mod` (e.g. emitting from
                    // a method of the uniontype itself): `pub type CTOR =
                    // Parent;` etc. are in scope by simple name, so doubling
                    // would point at an associated item of the struct (E0223).
                    let in_parent_mod = ctx.current_path.last().map(|p| p == parent_simple).unwrap_or(false);
                    if is_file_top || in_parent_mod {
                        return short;
                    }
                    // Prefer the original constructor's simple name when it
                    // differs from the parent — the mod exposes it via
                    // `pub type CTOR = ParentStruct;`, giving an unambiguous
                    // path `Parent::CTOR`. If the constructor's name matches
                    // the parent (single-record uniontype whose record was
                    // renamed during folding), fall back to repeating the
                    // parent name.
                    let ctor_simple = ctor_name.rsplit('.').next().unwrap_or(ctor_name);
                    let last = if ctor_simple != parent_simple { ctor_simple } else { parent_simple };
                    format!("{short}::{last}")
                } else {
                    short
                }
            };
            let rust_raw = match ty {
                Ty::RustStruct(qname) if name.contains('.') => shorten_struct_qname(ctx, qname, name),
                _ if folded_parent_qname.is_some() => ctx.shorten(folded_parent_qname.as_ref().unwrap()),
                _ if name.contains('.') => ctx.shorten(&canonical_ctor_name),
                _ => normalize_builtin_ctor_name(name),
            };
            let rust = escape_ident(&rust_raw);
            // Scope-aware constructor resolution: the same mechanism `infer_call`
            // uses, which follows `import Foo = NFFoo;` aliases declared in the
            // enclosing package. Without this, a pattern like `Expression.ARRAY`
            // in a file whose package imports `import Expression = NFExpression`
            // would fail direct hierarchy lookup and fall back to a simple-name
            // search, which can match an unrelated same-named record in another
            // package.
            let pkg_prefix_for_lookup = if ctx.current_path.is_empty() {
                ctx.top_name.clone()
            } else {
                format!("{}.{}", ctx.top_name, ctx.current_path.join("."))
            };
            let field_tys_for_ctor = || {
                if name.contains('.') {
                    if let Some((qname, _)) = crate::typedexp::resolve_call_node(name, top_level, &pkg_prefix_for_lookup)
                        && let Some(tys) = record_field_tys(&qname, top_level)
                    {
                        return Some(tys);
                    }
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
                if let Some(ty) = scrut_ty
                    && let Some(from_scrut) = record_field_tys_from_scrutinee_ctor(name, ty, top_level) {
                        return Some(from_scrut);
                    }
                Some(record_field_tys_by_simple_name(name, top_level))
            };
            let body = if named_fields.is_empty() && fields.is_empty() {
                // Empty MetaModelica pattern `case NODE()` or `case NODE` —
                // decide based on the type: struct/variant types need `{ .. }` to avoid
                // E0532; constants and unknown types use the bare name.
                // The SOURCEINFO builtin has no hierarchy node we can probe via
                // `ty`, so short-circuit on the constructor name; otherwise the
                // empty fall-through emits a bare `SourceInfo` that Rust reads
                // as a variable binding and rejects as a repeated identifier.
                if is_sourceinfo_ctor(name) {
                    return format!("{arc_prefix}{rust} {{ .. }}");
                }
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
                            format!("{fname}: {}", emit_pat_with_implicit_bind_md(p, allow_implicit_bind, mut_bindings, in_deref, implicit_ref, in_match_deref, None, ctx, top_level))
                        }
                    }).collect();
                    format!("{rust} {{ {} }}", pats.join(", "))
                } else {
                    // Positional patterns for named-field struct variants must use struct
                    // syntax in Rust; tuple syntax only applies to tuple-style variants.
                    // Look up field names so we can emit `Ctor { f1: p1, f2: p2 }`.
                    let field_tys_opt = field_tys_for_ctor();
                    // Distinguish "known to have zero fields" (Some(empty), a unit
                    // variant whose MetaModelica pattern was written as `CTOR(__)`)
                    // from "lookup failed" (None). For the former, emit the bare
                    // variant name — Rust unit variants reject `(_)` (E0532).
                    if let Some(tys) = &field_tys_opt
                        && tys.is_empty()
                        && fields.iter().all(|p| matches!(p, TypedPat::Wildcard)) {
                        return format!("{arc_prefix}{rust}");
                    }
                    let field_tys = field_tys_opt.unwrap_or_default();
                    // If our lookup returned fewer fields than the pattern has,
                    // the lookup found the wrong record (e.g. simple-name fallback
                    // matched an unrelated same-named record). Emit a TODO marker
                    // rather than synthesising `_:` field labels which are invalid
                    // Rust ("expected identifier or integer").
                    if !field_tys.is_empty() && field_tys.len() < fields.len() {
                        return format!("/* TODO: field-name lookup for {name} returned {} fields but pattern has {} */ {rust} {{ .. }}",
                            field_tys.len(), fields.len());
                    }
                    if !field_tys.is_empty() {
                        let pats: Vec<String> = fields.iter().enumerate().map(|(i, p)| {
                            let fname = field_tys.get(i).map(|(n, _)| n.as_str()).unwrap_or("_");
                            // Pass the field's declared type as the inner scrutinee so a
                            // nested constructor pattern (e.g. `Expression.BOOLEAN(false)`
                            // inside `INDEX(...)`) can disambiguate against other records
                            // sharing the same simple name (e.g. an empty `Dimension.BOOLEAN`).
                            let inner_scrut = field_tys.get(i).map(|(_, t)| t);
                            let pstr = emit_pat_with_implicit_bind_md(p, allow_implicit_bind, mut_bindings, in_deref, implicit_ref, in_match_deref, inner_scrut, ctx, top_level);
                            if matches!(p, TypedPat::Var(v) if v == fname) {
                                if implicit_ref {
                                    escape_ident(fname)
                                } else if in_deref {
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
                            .map(|p| emit_pat_with_implicit_bind_md(p, allow_implicit_bind, mut_bindings, in_deref, implicit_ref, in_match_deref, None, ctx, top_level))
                            .collect();
                        format!("/* TODO: unknown fields for {name} */ {rust}({})", pats.join(", "))
                    }
                }
            } else {
                let field_tys = field_tys_for_ctor().unwrap_or_default();
                let mut pats: Vec<String> = named_fields.iter()
                    .map(|(fname, p)| {
                        // Look up the named field's type so the inner pattern can
                        // disambiguate against similarly-named records — same reason
                        // as the positional branch above.
                        let inner_scrut = field_tys.iter().find(|(n, _)| n == fname).map(|(_, t)| t);
                        let pstr = emit_pat_with_implicit_bind_md(p, allow_implicit_bind, mut_bindings, in_deref, implicit_ref, in_match_deref, inner_scrut, ctx, top_level);
                        if matches!(p, TypedPat::Var(v) if v == fname) {
                            if implicit_ref {
                                escape_ident(fname)
                            } else if in_deref {
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
                let all_covered = !field_tys.is_empty()
                    && field_tys.iter().all(|(n, _)| named_fields.iter().any(|(m, _)| m == n));
                if !all_covered {
                    pats.push("..".to_owned());
                }
                format!("{rust} {{ {} }}", pats.join(", "))
            };
            format!("{arc_prefix}{body}")
        }

        TypedPat::As { var, pat } => {
            // `var @ pat` binds the whole value to `var` AND introduces any
            // sub-bindings declared in `pat`. If both sides take ownership
            // (move) of overlapping memory the borrow checker rejects it
            // (E0382/E0505). Rust's only way out is to make *every* binding
            // in the construct a borrow: `ref var @ (..., ref name, ...)`.
            // We then rely on the lowering's `.clone()`-everywhere style at
            // use sites to coerce `&T` back to owned `T` where needed.
            let pat_has_subbinding = pat_introduces_binding(pat);
            let force_ref = pat_has_subbinding && !implicit_ref;
            let outer = if implicit_ref {
                escape_ident(var)
            } else if in_deref || force_ref {
                format!("ref {}", escape_ident(var))
            } else if mut_bindings {
                format!("mut {}", escape_ident(var))
            } else {
                escape_ident(var)
            };
            // Propagate the scrutinee type into the inner sub-pattern.
            // `var @ inner` matches the same value against both sides, so the
            // inner needs the same `scrut_ty` to decide whether to emit a
            // `Deref @ ` prefix (e.g. an `As` pattern sitting on `Cons.tail`,
            // which is itself `Arc<List<T>>`, must wrap its inner Cons/Nil
            // sub-pattern in `Deref @ …`). Passing `None` here previously
            // dropped that information, producing `tail: rest @ List::Cons{…}`
            // and a `&Arc<…>` vs `List<…>` mismatch.
            format!("{} @ {}", outer, emit_pat_with_implicit_bind_md(pat, false, false, in_deref || force_ref, implicit_ref, in_match_deref, scrut_ty, ctx, top_level))
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
    // Peel Arc / Box / Mutable wrappers to reach the underlying enum/struct.
    fn peel<'a>(t: &'a Ty) -> &'a Ty {
        match t {
            Ty::Generic(_, args) => args.first().map(peel).unwrap_or(t),
            other => other,
        }
    }
    match peel(scrut_ty) {
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
        if let NodeKind::Class(c) = &node.kind
            && matches!(c.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_UNIONTYPE) {
                return true;
            }
        // Node was found in the hierarchy and is NOT a record/uniontype (e.g., it is a
        // function, package, import, etc.). Return false without applying any heuristic.
        return false;
    }
    // Shouldn't reach here. But let's assume it's not a constructor in that case
    false
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
        0 => "fileName",
        1 => "isReadOnly",
        2 => "lineNumberStart",
        3 => "columnNumberStart",
        4 => "lineNumberEnd",
        5 => "columnNumberEnd",
        6 => "lastModification",
        _ => "",
    }
}

fn is_infallible_builtin(func: &str) -> bool {
    use crate::external_c_calls::Fallibility;
    use crate::fallibility::builtin_fallibility;
    // Strip any module prefix so "MetaModelica.Dangerous.arrayGetNoBoundsChecking"
    // → "arrayGetNoBoundsChecking", "Dangerous.arrayCreateNoInit" → "arrayCreateNoInit",
    // and plain "intMul" → "intMul".
    let bare = func.rsplit('.').next().unwrap_or(func);
    matches!(builtin_fallibility(bare), Some(Fallibility::Infallible))
}

/// True if `pat` contains any identifier binding (Var or nested As). Used by
/// the As-pattern emitter to decide whether to force `ref` semantics on the
/// outer binding to avoid partial-move conflicts.
fn pat_introduces_binding(pat: &TypedPat) -> bool {
    match pat {
        TypedPat::Var(_) | TypedPat::As { .. } => true,
        TypedPat::Wildcard | TypedPat::EmptyList | TypedPat::None_
        | TypedPat::Lit(_) | TypedPat::FieldAccess { .. } | TypedPat::Index { .. }
        | TypedPat::Todo(_) => false,
        TypedPat::Some_(inner) => pat_introduces_binding(inner),
        TypedPat::Cons { head, tail } => pat_introduces_binding(head) || pat_introduces_binding(tail),
        TypedPat::Tuple(ps) => ps.iter().any(pat_introduces_binding),
        TypedPat::Constructor { fields, named_fields, .. } => {
            fields.iter().any(pat_introduces_binding)
                || named_fields.iter().any(|(_, p)| pat_introduces_binding(p))
        }
    }
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
        TypedPat::Var(name) => format!("{}.{}", escape_ident(name), escape_ident(field)),
        TypedPat::FieldAccess { base: inner, field: f } => {
            let inner_str = field_access_to_dotted(inner, f);
            format!("{}.{}", inner_str, escape_ident(field))
        },
        _ => format!("/*?*/.{}", escape_ident(field)),
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
        if let NodeKind::Class(cc) = &child.kind
            && matches!(cc.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. }) {
                record_children.push(child_name.as_str());
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
    // Follow import aliases. A pattern's constructor is recorded in its
    // MetaModelica spelling (e.g. `Expression.ARRAY`) which only resolves once
    // an enclosing `import Expression = NFExpression` is applied. Without this
    // pass, `lookup_node` would miss it and we'd fall through to a simple-name
    // search that can match a same-named record from an unrelated package
    // (e.g. `Absyn.Exp.ARRAY`), producing patterns with wrong field names.
    let node = crate::typedexp::walk_dotted_with_imports(qname, top_level, 0)
        .map(|(_, n)| n)
        .or_else(|| lookup_node(qname, top_level))?;
    let NodeKind::Class(c) = &node.kind else { return None };
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return None,
    };
    let direct: Vec<(String, Ty)> = members.iter().filter_map(|m| {
        let MM::ClassMember::Component(cm) = m else { return None };
        let child = node.children.get(&cm.name)?;
        Some((cm.name.clone(), child.ty.clone()))
    }).collect();
    if !direct.is_empty() {
        return Some(direct);
    }
    // Single-record uniontype: the parent uniontype's body holds the record
    // declaration rather than components — the actual fields live on the
    // sole record child. Mirror the lookup walk in
    // `typedexp::record_field_tys`.
    if matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) {
        let record_children: Vec<&NameNode> = node.children.values()
            .filter(|child| matches!(&child.kind, NodeKind::Class(cc)
                if matches!(cc.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })))
            .collect();
        if record_children.len() == 1 {
            let rec_node = record_children[0];
            if let NodeKind::Class(rc) = &rec_node.kind {
                let rec_members: &[MM::ClassMember] = match &rc.body {
                    MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
                    _ => return Some(direct),
                };
                let from_rec: Vec<(String, Ty)> = rec_members.iter().filter_map(|m| {
                    let MM::ClassMember::Component(cm) = m else { return None };
                    let child = rec_node.children.get(&cm.name)?;
                    Some((cm.name.clone(), child.ty.clone()))
                }).collect();
                return Some(from_rec);
            }
        }
    }
    Some(direct)
}

/// Fallback lookup for constructor patterns written with bare names (e.g. `SEMVER`).
/// Searches class nodes by simple name and returns declaration-order field types.
fn record_field_tys_by_simple_name<'a>(
    simple_name: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Vec<(String, Ty)> {
    fn walk<'a>(node: &'a NameNode<'a>, simple_name: &str) -> Option<Vec<(String, Ty)>> {
        for (child_name, child) in &node.children {
            if child_name == simple_name
                && let NodeKind::Class(c) = &child.kind {
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
                if let Some((parent, _)) = qname.rsplit_once('.')
                    && ctx.recursive_types.contains(parent) { return false; }
            }
            args.iter().all(|a| is_static_const_emittable(a, ctx, top_level))
                && named_args.iter().all(|(_, a)| is_static_const_emittable(a, ctx, top_level))
        }
        // Unit variants of an enum are const-constructable when the enum is not Arc-wrapped.
        TypedExp::Var { ty: Ty::RustUnitVariant, .. } => true,
        _ => false,
    }
}

/// True if the lowered Rust type for `ty` implements `Sync` for the purposes of
/// `pub static` storage. The only non-`Sync` MetaModelica primitive we generate
/// today is `Array<T> = Rc<RefCell<Vec<T>>>` (see `metamodelica::Array`); every
/// other built-in maps to a `Sync` Rust type (`Arc<List<T>>`, `ArcStr`,
/// `Mutable<T> = Arc<Mutex<T>>`, primitives, function pointers, etc.).
///
/// User-defined struct/enum types are checked against
/// [`GenCtx::types_containing_array`], which is the closure of "transitively
/// embeds an `Array<T>` field" computed once in
/// [`crate::hierarchy::detect_types_containing_array`].
///
/// Type variables (`Ty::TypeVar`) are conservatively treated as `Sync` here —
/// the only call site is the `pub static` decision in [`emit_node`], where the
/// type of a top-level `constant` is always concrete; if a generic ever flows
/// in, the surrounding `pub static` would have to declare a `T: Sync` bound
/// itself, and we'd want a separate analysis at that point.
fn ty_is_sync(ty: &Ty, ctx: &GenCtx) -> bool {
    match ty {
        Ty::Array(_) => false,
        Ty::Option(t) | Ty::List(t) | Ty::Range(t) => ty_is_sync(t, ctx),
        Ty::Tuple(ts) => ts.iter().all(|t| ty_is_sync(t, ctx)),
        Ty::Generic(name, args) => {
            let dotted = name.replace("::", ".");
            if ctx.types_containing_array.contains(&dotted) { return false; }
            args.iter().all(|a| ty_is_sync(a, ctx))
        }
        Ty::RustStruct(qname) | Ty::RustEnum(qname) | Ty::AliasTo(qname) | Ty::ExternalObject(qname) => {
            !ctx.types_containing_array.contains(qname)
        }
        Ty::UnionTypeVariant(qname, _) => !ctx.types_containing_array.contains(qname),
        // Primitives, function types, type variables, Unknown — all Sync (see fn doc).
        _ => true,
    }
}

fn is_arc_wrapped(ty: &Ty, ctx: &GenCtx) -> bool {
    let qname = match ty {
        Ty::RustStruct(n) | Ty::RustEnum(n) | Ty::AliasTo(n) | Ty::ExternalObject(n) => n.as_str(),
        Ty::Generic(name, _) => return ctx.recursive_types.contains(&name.replace("::", ".")),
        // Variant-narrowed types (a uniontype field narrowed to one of its
        // records by an outer match) still live behind an `Arc<Enum>` at the
        // Rust level — the narrowing is a typing-level concept that doesn't
        // change the runtime representation. Without this, a nested `match`
        // on the narrowed variable emits patterns without the `Deref @`
        // prefix even though the scrutinee is still `&Arc<Enum>` (see e.g.
        // `filterSubMods` in `SCodeUtil.mo`).
        Ty::UnionTypeVariant(parent, _) => return ctx.recursive_types.contains(parent.as_str()),
        _ => return false,
    };
    ctx.recursive_types.contains(qname)
}

/// Return true if a pattern-let against a scrutinee of this type needs the
/// scrutinee borrowed (`&(...)`) so that match ergonomics auto-refs bindings
/// inside any sub-pattern that destructures through an `Arc<T>` deref_pattern.
/// We must recurse through `Tuple` (and other transparent wrappers) because a
/// non-Arc Tuple can contain an Arc-wrapped element whose Constructor sub-pattern
/// would otherwise try to move a non-Copy field out of a shared reference.
/// `Option<T>` doesn't need recursion: its built-in `Some` pattern doesn't
/// auto-deref through `Arc`, so an inner Arc-wrapped sub-pattern would already
/// be handled by its own Constructor rendering.
fn type_destructure_needs_borrow(ty: &Ty, ctx: &GenCtx) -> bool {
    if matches!(ty, Ty::List(_)) || is_arc_wrapped(ty, ctx) {
        return true;
    }
    if let Ty::Tuple(elems) = ty {
        return elems.iter().any(|t| type_destructure_needs_borrow(t, ctx));
    }
    // A pattern like `Some(Pkg::Foo { … })` on a scrutinee typed
    // `Option<Arc<Foo>>` needs the same `match_deref!` treatment as a bare
    // `Arc<Foo>` scrutinee, because the inner Arc still needs to be peeled
    // for the destructure to match the enum/struct shape. The Option layer
    // itself is fine, but propagate the check through its inner type.
    if let Ty::Option(inner) = ty {
        return type_destructure_needs_borrow(inner, ctx);
    }
    false
}

/// Return true if the value produced by emitting `arg` will already be wrapped
/// in `Arc<T>`. Used by struct-field emission to avoid emitting a redundant
/// outer `Arc::new(...)` around an expression that already yields `Arc<T>`.
///
/// This is the case when:
///   * The argument's static type is itself a recursive uniontype (variables,
///     calls, etc. of `Ty::RustEnum(qname)` where `qname` is recursive).
///   * The argument is a *constructor expression* for a variant of a recursive
///     uniontype — `emit_exp` wraps it in `Arc::new(...)` via `constructor_needs_arc`.
///   * The argument is a *unit-variant constructor* whose parent uniontype is
///     recursive — once Bug B is fixed below, `emit_exp` wraps these in `Arc::new`.
fn value_emitted_as_arc(arg: &TypedExp, ctx: &GenCtx) -> bool {
    let ty = arg.ty();
    if is_arc_wrapped(&ty, ctx) || constructor_needs_arc(&ty, ctx) {
        return true;
    }
    if matches!(&ty, Ty::RustUnitVariant)
        && let TypedExp::Constructor { name, .. } = arg
            && let Some((parent, _)) = name.rsplit_once('.')
                && ctx.recursive_types.contains(parent) {
                    return true;
                }
    false
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
        // Generic instantiations of a user-defined record/uniontype carry their
        // base type's Rust-form qname (e.g. "ExpandableArray"). The wrapping
        // rule mirrors `Ty::RustStruct`: wrap when the type itself or its
        // parent uniontype is marked recursive.
        Ty::Generic(rust_name, _) => {
            let dotted = rust_name.replace("::", ".");
            if ctx.recursive_types.contains(dotted.as_str()) {
                return true;
            }
            if let Some((parent, _)) = dotted.rsplit_once('.') {
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
            // First-output extraction (`.0`) for tuple-typed scrutinees must
            // happen *after* the `Result` is unwrapped — the `Result<(...)>`
            // itself has no `.0` field. In the failure mode we bind the whole
            // tuple, then extract; in the infallible let we can append `.0`
            // to the expression directly.
            let actual_ty = if let Ty::Tuple(tys) = scrut_ty {
                tys.first().cloned().unwrap_or(Ty::Unknown)
            } else {
                scrut_ty.clone()
            };
            let extract_tuple = matches!(scrut_ty, Ty::Tuple(_));
            env.vars.insert(name.clone(), actual_ty);
            if let FailureMode::IfLetElse(else_code) = fail_mode {
                let n = *fresh; *fresh += 1;
                let tmp = format!("__iflet{n}");
                let inner = format!("{indent}    ");
                writeln!(out, "{indent}if let Ok({tmp}) = {scrut_expr} {{").unwrap();
                if extract_tuple {
                    writeln!(out, "{inner}{} = {tmp}.0;", escape_ident(name)).unwrap();
                } else {
                    writeln!(out, "{inner}{} = {tmp};", escape_ident(name)).unwrap();
                }
                writeln!(out, "{indent}}} else {{").unwrap();
                out.push_str(else_code.as_str());
                writeln!(out, "{indent}}}").unwrap();
            } else {
                let actual_expr = if extract_tuple {
                    format!("{scrut_expr}.0")
                } else {
                    scrut_expr.to_string()
                };
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
            let mut reassign_pairs: Vec<(String, String, Ty)> = Vec::new();
            let pat_owned = rewrite_pat_for_existing_bindings(pat, env, fresh, &mut reassign_pairs);
            let pat_for_render = &pat_owned;
            // Render shallow with deferrals for Arc-edge crossings.
            let mut deferrals: Vec<(String, TypedPat, Ty)> = Vec::new();
            let surface = render_shallow(pat_for_render, scrut_ty, ctx, env, top_level, fresh, &mut deferrals, /*force_ref=*/false);
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
            //
            // We must also recurse through Tuple types: a let-let pattern
            // `(STRING{r#str=key}, tokens) := parse_string(..)` has
            // scrut_ty = (Arc<JSON>, Arc<List<Token>>), where the outer tuple
            // itself isn't Arc-wrapped but the first element is. Without
            // borrowing the whole tuple, the inner Constructor pattern would
            // still trigger Arc deref_patterns and fail to move non-Copy
            // fields (`ArcStr`, `Arc<T>`, …) out of the shared reference.
            // Borrowing the whole tuple makes match ergonomics auto-ref all
            // bindings uniformly.
            let needs_borrow = type_destructure_needs_borrow(scrut_ty, ctx);
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
                    for (orig, fresh_name, orig_ty) in &reassign_pairs {
                        // Always `.clone()`. The `fresh_name` may have been
                        // emitted with `ref` (e.g. an `As` pattern with inner
                        // sub-bindings forced to `ref` to dodge a partial
                        // move), in which case it has type `&T` and a plain
                        // assignment would mismatch. `T::clone()` is defined
                        // on both `T` and `&T` so this is a safe blanket
                        // coercion and keeps with the codegen's clone-heavy
                        // lowering style elsewhere.
                        //
                        // MetaModelica allows `Integer → Real` promotion in
                        // pattern-let LHSs (e.g. `Real lo; INTERVAL(lo,..) :=
                        // int_with_integer_lo;`). Coerce the clone-expression
                        // to the original variable's declared type so the
                        // generated Rust assignment type-checks.
                        let rhs = format!("{}.clone()", escape_ident(fresh_name));
                        // MetaModelica allows silent `Integer → Real`
                        // promotion in pattern LHSs (e.g. `Real lo;
                        // INTERVAL(lo,..) := int_with_integer_lo;`). If the
                        // outer variable is `Real` and the pattern binding
                        // (looked up via env after the pattern emit) is i32,
                        // wrap through f64 and rewrap as `metamodelica::Real`.
                        // Same-type rebinds (Real-into-Real) leave `rhs` as-is.
                        let src_ty = env.vars.get(fresh_name.as_str()).cloned().unwrap_or(Ty::Unknown);
                        let rhs = if matches!(orig_ty, Ty::F64) && matches!(src_ty, Ty::I32) {
                            format!("metamodelica::OrderedFloat(({rhs}) as f64)")
                        } else {
                            rhs
                        };
                        writeln!($out, "{}{} = {};", $ind, escape_ident(orig), rhs).unwrap();
                    }
                };
            }

            // Match-deref transform for refutable Arc-crossing pattern-lets.
            //
            // The legacy `let PAT = &(expr) else { fail };` form relied on the
            // nightly `deref_patterns` feature to peel the `Arc<…>` around
            // `&Arc<List<T>>` etc. On stable we have no auto-deref, so the
            // raw pattern fails to compile (E0308 "expected `Arc<List<T>>`,
            // found `List<_>`"). Replace it with a `::match_deref::match_deref!`
            // wrapped `match` that yields a tuple of the pattern's clones:
            //
            //     let (b1, b2, …) = ::match_deref::match_deref! { match &(expr) {
            //         <emit_pat with implicit_ref=true>  => (b1.clone(), b2.clone(), …),
            //         _ => bail!(…),
            //     } };
            //
            // We re-emit the pattern through `emit_pat_with_implicit_bind`
            // (which now inserts `Deref @ …` at Arc edges) instead of
            // `render_shallow`'s deferral machinery — recursion through
            // nested Arc fields is handled directly by the macro's repeated
            // `Deref @ …` instead of follow-up `emit_pat_assign` calls.
            let pat_needs_match_deref =
                !pat_is_irrefutable(pat_for_render)
                && !matches!(fail_mode, FailureMode::IfLetElse(_))
                && (type_destructure_needs_borrow(scrut_ty, ctx)
                    || pat_has_str_lit(pat_for_render));
            if pat_needs_match_deref {
                let fail_owned;
                let fail: &str = match &fail_mode {
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
                // Collect all binding names introduced by the (possibly
                // re-named) pattern, together with their owned types. The
                // owned type is what the outer `let` binding will see after
                // each `<name>.clone()` in the arm body.
                // Use the `_tl` variant so constructor field types resolve to
                // their declared types (e.g. INTERVAL.lo: Integer) instead of
                // `Ty::Unknown`. The reassign-back step at end of this block
                // reads these types from `env.vars` to decide whether an
                // Integer-into-Real promotion wrap is needed.
                let bindings = typedexp::pat_bindings_with_scrut_ty_tl(pat_for_render, scrut_ty, top_level);
                // Render the pattern with match_deref semantics in scope.
                // `implicit_ref = true` turns on the `Deref @` prefix at Arc
                // edges and emits bare names (match ergonomics gives them
                // `&T`, which `.clone()` then promotes to owned `T`).
                let inner_pat = emit_pat_with_implicit_bind_md(
                    pat_for_render,
                    /*allow_implicit_bind=*/false,
                    /*mut_bindings=*/false,
                    /*in_deref=*/false,
                    /*implicit_ref=*/true,
                    /*in_match_deref=*/true,
                    Some(scrut_ty),
                    ctx,
                    top_level,
                );
                // Register the binding types into the surrounding env so
                // later statements (and the reassign-back step) see them.
                for (n, t) in &bindings {
                    env.vars.insert(n.clone(), t.clone());
                }
                if bindings.is_empty() {
                    // Pattern destructures only for the side effect of the
                    // refutability check (no names bound). Use `match` with
                    // unit-valued arms; the let-binding to `()` would be
                    // wasteful, so emit a bare match expression.
                    writeln!(out, "{indent}::match_deref::match_deref! {{ match &({scrut_expr}) {{").unwrap();
                    writeln!(out, "{indent}    {inner_pat} => (),").unwrap();
                    writeln!(out, "{indent}    _ => {fail},").unwrap();
                    writeln!(out, "{indent}}} }};").unwrap();
                } else if bindings.len() == 1 {
                    let (n, _) = &bindings[0];
                    let id = escape_ident(n);
                    writeln!(out, "{indent}let {id} = ::match_deref::match_deref! {{ match &({scrut_expr}) {{").unwrap();
                    writeln!(out, "{indent}    {inner_pat} => {id}.clone(),").unwrap();
                    writeln!(out, "{indent}    _ => {fail},").unwrap();
                    writeln!(out, "{indent}}} }};").unwrap();
                } else {
                    let lhs: Vec<String> = bindings.iter().map(|(n, _)| escape_ident(n)).collect();
                    let rhs: Vec<String> = bindings.iter().map(|(n, _)| format!("{}.clone()", escape_ident(n))).collect();
                    writeln!(out, "{indent}let ({}) = ::match_deref::match_deref! {{ match &({scrut_expr}) {{", lhs.join(", ")).unwrap();
                    writeln!(out, "{indent}    {inner_pat} => ({}),", rhs.join(", ")).unwrap();
                    writeln!(out, "{indent}    _ => {fail},").unwrap();
                    writeln!(out, "{indent}}} }};").unwrap();
                }
                // No deferrals to emit — emit_pat_with_implicit_bind already
                // produced the full nested pattern. Still write back any
                // reassign pairs (`name = __paN.clone();`) for pattern
                // names that collided with the surrounding scope.
                for (orig, fresh_name, orig_ty) in &reassign_pairs {
                    // Mirror the Integer→Real promotion in the shallow path.
                    let rhs = format!("{}.clone()", escape_ident(fresh_name));
                    let src_ty = env.vars.get(fresh_name.as_str()).cloned().unwrap_or(Ty::Unknown);
                    let rhs = if matches!(orig_ty, Ty::F64) && matches!(src_ty, Ty::I32) {
                        format!("metamodelica::OrderedFloat(({rhs}) as f64)")
                    } else {
                        rhs
                    };
                    writeln!(out, "{indent}{} = {};", escape_ident(orig), rhs).unwrap();
                }
                return;
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
///
/// Each reassign entry also carries the original variable's declared type (from
/// the caller's `env`). Used downstream to coerce the value before assigning,
/// e.g. `Real lo; INTERVAL(lo,..) := int;` where `int.lo : Integer` requires an
/// `as f64` cast in the synthesised `lo = __paN.clone()` assignment.
fn rewrite_pat_for_existing_bindings(
    pat: &TypedPat,
    env: &LocalEnv,
    fresh: &mut u32,
    reassign: &mut Vec<(String, String, Ty)>,
) -> TypedPat {
    let mk_fresh = |fresh: &mut u32| -> String {
        let n = *fresh; *fresh += 1;
        format!("__pa{n}")
    };
    match pat {
        TypedPat::Var(name) if env.vars.contains_key(name) => {
            let new_name = mk_fresh(fresh);
            let orig_ty = env.vars.get(name).cloned().unwrap_or(Ty::Unknown);
            reassign.push((name.clone(), new_name.clone(), orig_ty));
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
                let orig_ty = env.vars.get(var).cloned().unwrap_or(Ty::Unknown);
                reassign.push((var.clone(), new_name.clone(), orig_ty));
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
    // When true, every name binding rendered by this call (and its recursive
    // descendants) is emitted with a leading `ref`. Set by the As-pattern
    // arm when the outer `ref var @ ...` borrows the whole scrutinee — without
    // it, the inner field bindings would try to move out of memory that the
    // outer `ref` is already borrowing (E0382/E0507).
    force_ref: bool,
) -> String {
    match pat {
        TypedPat::Wildcard => "_".to_owned(),
        TypedPat::Var(name) => {
            env.vars.insert(name.clone(), scrut_ty.clone());
            if force_ref {
                format!("ref {}", escape_ident(name))
            } else {
                escape_ident(name)
            }
        }
        TypedPat::EmptyList => "metamodelica::List::Nil".to_owned(),
        TypedPat::None_ => "None".to_owned(),
        TypedPat::Some_(inner) => {
            let inner_ty = match scrut_ty {
                Ty::Option(t) => (**t).clone(),
                _ => Ty::Unknown,
            };
            let inner_s = render_shallow(inner, &inner_ty, ctx, env, top_level, fresh, deferrals, force_ref);
            format!("Some({inner_s})")
        }
        TypedPat::Lit(Lit::Int(v)) => if *v < 0 { format!("({v})") } else { v.to_string() },
        TypedPat::Lit(Lit::Bool(v)) => v.to_string(),
        TypedPat::Lit(_) => "_ /* lit — guard not yet implemented */".to_owned(),
        TypedPat::Cons { head, tail } => {
            let elem_ty = match scrut_ty { Ty::List(t) => (**t).clone(), _ => Ty::Unknown };
            let h = render_shallow(head, &elem_ty, ctx, env, top_level, fresh, deferrals, force_ref);
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
                TypedPat::Var(_) => render_shallow(tail, scrut_ty, ctx, env, top_level, fresh, deferrals, force_ref),
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
                .map(|(p, t)| render_shallow(p, t, ctx, env, top_level, fresh, deferrals, force_ref))
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
            if field_tys.is_empty()
                && let Some(q) = resolved_qname.as_deref()
                    && let Some((canonical, _)) = lookup_record_through_unions(q, top_level)
                        && let Some(tys) = record_field_tys(&canonical, top_level) {
                            field_tys = tys;
                            resolved_qname = Some(canonical);
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
            // When the record's parent is a single-record uniontype, the record was
            // folded onto the parent struct (no enum wrapper); the path stops at
            // the parent, not at `Parent::Record`. Same logic as the let-pattern
            // emission path above — kept in sync so render_shallow and
            // emit_pat_with_implicit_bind_md agree on the canonical Rust path.
            let folded_parent_qname: Option<String> = (|| {
                let qname = resolved_qname.as_deref()?;
                if !qname.contains('.') { return None; }
                let parent_str = qname.rsplit_once('.').map(|(p, _)| p.to_owned())?;
                // Look up parent directly, falling back to alias-aware walk
                // for import-qualified names like `Sets.DISJOINT_SETS` whose
                // parent is reachable only via an `extends`/`import` alias
                // (e.g. canonical `DisjointSets.Sets`).
                let (canonical_parent, parent_node) = match lookup_node(&parent_str, top_level) {
                    Some(n) => (parent_str.clone(), n),
                    None => crate::typedexp::walk_dotted_with_imports(&parent_str, top_level, 0)
                        // Last-resort: use the scrutinee's static type. When
                        // the pattern is a bare or alias-qualified constructor
                        // (e.g. `Sets.DISJOINT_SETS` from an `extends`-inherited
                        // uniontype), the parent's canonical qname isn't
                        // directly reachable from the import scope here, but
                        // the value's own type already names it.
                        .or_else(|| {
                            let scrut_qname = match scrut_ty {
                                Ty::AliasTo(q) | Ty::RustStruct(q) | Ty::RustEnum(q) => Some(q.clone()),
                                Ty::Generic(_, args) => args.first().and_then(|t| match t {
                                    Ty::AliasTo(q) | Ty::RustStruct(q) | Ty::RustEnum(q) => Some(q.clone()),
                                    _ => None,
                                }),
                                _ => None,
                            }?;
                            let n = lookup_node(&scrut_qname, top_level)?;
                            Some((scrut_qname, n))
                        })?,
                };
                let NodeKind::Class(c) = &parent_node.kind else { return None };
                if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) { return None; }
                if uniontype_needs_mod(parent_node) { return None; }
                let record_count = parent_node.children.values().filter(|ch| {
                    matches!(&ch.kind, NodeKind::Class(cc)
                        if matches!(cc.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. }))
                }).count();
                if record_count == 1 { Some(canonical_parent) } else { None }
            })();
            let rust_ctor_raw = if let Some(p) = &folded_parent_qname {
                ctx.shorten(p)
            } else if name.contains('.') {
                ctx.shorten(name)
            } else {
                normalize_builtin_ctor_name(name)
            };
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
                    render_shallow(sub, fty, ctx, env, top_level, fresh, deferrals, force_ref)
                }
            };

            if !named_fields.is_empty() {
                let parts: Vec<String> = named_fields.iter().map(|(fname, sp)| {
                    let fty = field_tys.iter().find(|(n, _)| n == fname).map(|(_, t)| t.clone()).unwrap_or(Ty::Unknown);
                    let s = handle(sp, &fty, ctx, env, fresh, deferrals);
                    let rust_field = fname;
                    if matches!(sp, TypedPat::Var(v) if v == fname) {
                        escape_ident(rust_field)
                    } else {
                        format!("{}: {s}", escape_ident(rust_field))
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
            // When the inner sub-pattern introduces bindings, both outer and inner
            // would move out of overlapping memory (E0382). Make the outer a borrow
            // AND propagate `force_ref` into the inner sub-pattern so its bindings
            // also borrow rather than move. Without that propagation, the inner
            // field bindings (e.g. `description: __pa0`) would still move out of
            // the value while the outer `ref __pa2` is borrowing it.
            let inner_force_ref = force_ref || pat_introduces_binding(inner);
            let inner_s = render_shallow(inner, scrut_ty, ctx, env, top_level, fresh, deferrals, inner_force_ref);
            if pat_introduces_binding(inner) || force_ref {
                format!("ref {} @ {}", escape_ident(var), inner_s)
            } else {
                format!("{} @ {}", escape_ident(var), inner_s)
            }
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
        if let Some(plan) = plan_field_assign(&stmts[i], env, top_level)
            && plan.is_macro(ctx) {
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
        // Mirror the `value_emitted_as_arc` guard used by the constructor
        // emission path: if the RHS already evaluates to `Arc<T>` (e.g. it
        // is a recursive-uniontype-typed call result, where every such
        // value is materialised as an `Arc`), wrapping again would give
        // `Arc::new(Arc<T>)` and the receiving field's type-check fails
        // with `expected T, found Arc<T>`. The guard mirrors how the
        // recursive-record constructor argument path avoids the same
        // double-wrap (see the `struct_field_is_arc && !value_emitted_as_arc`
        // check around line 2127).
        let value = if struct_field_is_arc(&record_qname, field, top_level, ctx)
            && !value_emitted_as_arc(rhs, ctx)
        {
            format!("Arc::new({expr})")
        } else {
            expr
        };
        let base_safe = escape_ident(&base_name).to_string();
        let field_safe = escape_ident(field).to_string();

        if let Some((enum_qname, variant_name)) = variant {
            let variant_path = build_variant_path(&enum_qname, &variant_name, ctx);
            // Multi-record uniontypes that aren't recursive are stored by
            // value, not behind an `Arc`. The `assign_variant_field!` macro
            // unconditionally rewraps via `Arc::new`, so for owned values we
            // emit an explicit `if let` destructure instead. `is_arc_wrapped`
            // returns true only for recursive types, so this also covers
            // mutable-locals of non-recursive enums like `IOStreamData`.
            if is_arc_wrapped(&base_ty, ctx) {
                FieldAssignKind::ArcVariant { base: base_safe, variant_path, field: field_safe, value }
            } else {
                FieldAssignKind::OwnedVariant { base: base_safe, variant_path, field: field_safe, value }
            }
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
        // Single-record uniontypes are collapsed to a plain struct named after
        // the parent. There is no variant to select, so emitting
        // `assign_variant_field!(.. => Parent::ONLY; ..)` produces an invalid
        // path. Skip the variant branch in that case and fall through to the
        // plain `assign_field!` lowering below, which uses the struct path.
        let is_single_record_uniontype = uniontype_is_enum(&enum_qname, top_level) == false
            && resolve_single_record_qname(&enum_qname, top_level).is_some();
        // Only commit to the variant path if the record actually exists.
        if !is_single_record_uniontype
            && record_field_tys(&record_qname, top_level).is_some() {
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
        // Generic instantiations of user-defined records/uniontypes (e.g.
        // `ExpandableArray<T>`) carry the Rust-form name in `Ty::Generic`,
        // so normalise back to the dotted hierarchy key for field lookup.
        Ty::Generic(rust_name, _) => rust_name.replace("::", "."),
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
    /// Field assignment on an *owned* (not Arc-wrapped) uniontype enum value.
    /// `if let <variant_path> { <field>, .. } = &mut <base> { *<field> = <value>; }`
    /// — used for non-recursive uniontypes (e.g. `IOStreamData`) where the
    /// variable is stored by value rather than behind an `Arc`. The
    /// `assign_variant_field!` macro is hard-coded to call `Arc::new` and
    /// cannot lower these owned cases.
    OwnedVariant { base: String, variant_path: String, field: String, value: String },
    /// `<base>.<field> = <value>;` — plain owned struct, no macro needed.
    Plain { base: String, field: String, value: String },
}

impl FieldAssignKind {
    fn is_macro(&self) -> bool {
        matches!(self, FieldAssignKind::ArcStruct { .. } | FieldAssignKind::ArcVariant { .. } | FieldAssignKind::OwnedVariant { .. })
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
            (
                FieldAssignKind::OwnedVariant { base: a, variant_path: va, .. },
                FieldAssignKind::OwnedVariant { base: b, variant_path: vb, .. },
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
            FieldAssignKind::OwnedVariant { field, value, .. } => format!("{field} = {value}"),
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
            FieldAssignKind::OwnedVariant { base, variant_path, .. } => {
                // Owned (non-Arc) uniontype: lower to an `if let <variant_path>
                // { <field>, .. } = &mut <base>` destructure and mutate the
                // matched bindings in place.
                //
                // Two correctness concerns:
                //   1. The RHS often reads the *current* value of the same field
                //      (`s_data.data := listAppend(..., s_data.data)`); evaluating
                //      it after taking `&mut base` would violate borrow rules.
                //   2. The destructure introduces bindings named after the fields,
                //      which can shadow names used inside the RHS (a parameter
                //      named `data` is shadowed by the field binding `data`).
                // Both are solved by computing each RHS into a fresh local first,
                // then mutating inside a single destructure.
                let parsed: Vec<(String, String, String)> = clauses.iter().enumerate().map(|(i, c)| {
                    let (lhs, rhs) = c.split_once('=').unwrap_or((c.as_str(), ""));
                    let field = lhs.trim().to_string();
                    let tmp = format!("__owned_variant_{}_{}", field, i);
                    (field, tmp, rhs.trim().to_string())
                }).collect();
                for (_field, tmp, rhs) in &parsed {
                    writeln!(out, "{indent}let {tmp} = {rhs};").unwrap();
                }
                let field_pat = parsed.iter().map(|(f, _, _)| f.clone()).collect::<Vec<_>>().join(", ");
                writeln!(out, "{indent}if let {variant_path} {{ {field_pat}, .. }} = &mut {base} {{").unwrap();
                for (field, tmp, _) in &parsed {
                    writeln!(out, "{inner_indent}*{field} = {tmp};").unwrap();
                }
                writeln!(out, "{indent}}} else {{ panic!(\"owned-variant field-assign: value held a different variant than {variant_path}\"); }}").unwrap();
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
/// MetaModelica permits assigning the result of a multi-output function call
/// to a single-valued variable; the remaining outputs are silently discarded.
/// When we detect that the initializer type is a tuple but the declared local
/// is scalar, emit a destructuring let `let (mut name, _, _) : (T1, T2, T3) = init;`
/// so the discard is explicit and the call is still evaluated exactly once.
/// Returns `None` when the regular `let mut name: T = init;` form is fine.
fn emit_multi_output_let(
    is_mut: bool,
    name: &str,
    decl_ty: &Ty,
    init_ty: &Ty,
    init_expr: &str,
    indent: &str,
    ctx: &mut GenCtx,
) -> Option<String> {
    let elems = match init_ty {
        Ty::Tuple(es) if es.len() >= 2 => es,
        _ => return None,
    };
    if matches!(decl_ty, Ty::Tuple(_)) {
        return None;
    }
    let elem_tys: Vec<String> = elems.iter().map(|t| fmt_ty(t, ctx)).collect();
    let m = if is_mut { "mut " } else { "" };
    let mut slots: Vec<String> = Vec::with_capacity(elems.len());
    slots.push(format!("{m}{}", escape_ident(name)));
    for _ in 1..elems.len() { slots.push("_".to_owned()); }
    Some(format!(
        "{indent}let ({}): ({}) = {init_expr};\n",
        slots.join(", "),
        elem_tys.join(", "),
    ))
}

fn coerce_assign_expr_pub(scrut_expr: String, scrut_ty: &Ty, lhs_ty: Option<&Ty>) -> String {
    let mut expr = scrut_expr;
    if let Ty::Tuple(_) = scrut_ty
        && !matches!(lhs_ty, Some(Ty::Tuple(_))) {
            expr = format!("{expr}.0");
        }
    if matches!(lhs_ty, Some(Ty::F64)) && *scrut_ty == Ty::I32 {
        expr = format!("metamodelica::OrderedFloat(({expr}) as f64)");
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
        // The `as` binding is `var @ pat`; when the surrounding match crosses
        // an Arc edge (and so emits `match_deref!{ match &x ...}`), the binding
        // has Rust type `&Arc<T>` — VarShape::RefArc. Without recording this,
        // a downstream `var_field!` on `var` would default to VarShape::Arc
        // (from fn_env_vars storing the *value* type) and emit `(*var).f`,
        // producing the wrong number of derefs.
        if let Some(c) = ctx {
            if ty_needs_arc_match_deref(&scrut_ty, c) && is_arc_wrapped(&scrut_ty, c) {
                shapes.insert(var.clone(), VarShape::RefArc);
            }
        }
    }
    // (2) Scrutinee that's a bare variable narrowed by the arm's pattern.
    let inner_pat = match pat {
        TypedPat::As { pat: inner, .. } => inner.as_ref(),
        other => other,
    };
    if let Some((enum_q, variant)) = variant_of_pat(inner_pat, &scrut_ty, top_level)
        && let TypedExp::Var { name, .. } = scrutinee {
            env.variants.insert(name.clone(), (enum_q, variant));
        }
    // (3) Tuple scrutinee + tuple pattern: pair each element. This is the
    //     common MetaModelica idiom
    //         match (v1, v2) { (CTOR_A { .. }, CTOR_B { .. }) => ... }
    //     where the arm body wants to read `v1.field_of_A` and
    //     `v2.field_of_B`. Without this pass those reads would not know
    //     which variant each element holds.
    if let (TypedPat::Tuple(pat_elems), TypedExp::Tuple(scrut_elems)) = (inner_pat, scrutinee)
        && pat_elems.len() == scrut_elems.len() {
            for (sub_pat, sub_scrut) in pat_elems.iter().zip(scrut_elems.iter()) {
                record_pattern_variants_inner(sub_pat, sub_scrut, env, top_level, shapes, ctx);
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
    record_constructor_pattern_bindings(inner_pat, &scrut_ty, env, top_level, shapes, ctx);
    // (5) List-cons patterns: walk through `head :: tail` (= TypedPat::Cons).
    //     `head` is a single element; `tail` is the same list type. When the
    //     scrutinee is `Arc<List<T>>` and the list crosses Arc (always true
    //     for `Ty::List`), every name binding inside the pattern is by-ref
    //     under match_deref!. Record the shape so downstream `var_field!`
    //     and `.field` emission deref correctly, and recurse so nested
    //     constructor patterns inside the head/tail also register.
    if let TypedPat::Cons { head, tail } = inner_pat {
        let elem_ty = match &scrut_ty {
            Ty::List(t) => (**t).clone(),
            _ => Ty::Unknown,
        };
        record_cons_subpattern(head.as_ref(), &elem_ty, env, top_level, shapes, ctx);
        record_cons_subpattern(tail.as_ref(), &scrut_ty, env, top_level, shapes, ctx);
    }
}

/// Helper for step (5) in `record_pattern_variants_inner`: register the
/// variant/shape implied by a `Cons` sub-pattern. The sub-pattern may bind
/// the slot via `Var` or `As`, or carry a nested constructor; in all cases
/// we narrow against the slot's type and recurse to pick up deeper bindings.
fn record_cons_subpattern<'a>(
    sub_pat: &TypedPat,
    slot_ty: &Ty,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    shapes: &mut HashMap<String, VarShape>,
    ctx: Option<&GenCtx>,
) {
    // Outer As / bare Var binding for this slot.
    if let Some(var) = match sub_pat {
        TypedPat::As { var, .. } | TypedPat::Var(var) => Some(var),
        _ => None,
    } {
        if let TypedPat::As { pat: inner_as, .. } = sub_pat
            && let Some((enum_q, variant)) = variant_of_pat(inner_as, slot_ty, top_level) {
            env.variants.insert(var.clone(), (enum_q, variant));
        }
        if let Some(c) = ctx {
            // `head`/`tail` slots inside `match_deref!` are reached through a
            // `&Arc<…>` reference whenever the slot's own type is Arc-wrapped.
            if is_arc_wrapped(slot_ty, c) {
                shapes.insert(var.clone(), VarShape::RefArc);
            }
        }
    }
    let next_pat = match sub_pat {
        TypedPat::As { pat: inner_as, .. } => inner_as.as_ref(),
        other => other,
    };
    record_constructor_pattern_bindings(next_pat, slot_ty, env, top_level, shapes, ctx);
    // Recurse into nested Cons (e.g. `_ :: rest :: _`).
    if let TypedPat::Cons { head, tail } = next_pat {
        let inner_elem_ty = match slot_ty {
            Ty::List(t) => (**t).clone(),
            _ => Ty::Unknown,
        };
        record_cons_subpattern(head.as_ref(), &inner_elem_ty, env, top_level, shapes, ctx);
        record_cons_subpattern(tail.as_ref(), slot_ty, env, top_level, shapes, ctx);
    }
}

/// Walk a `Constructor` pattern and register variants + shapes for any name
/// bindings it carries (including in nested constructor / `As` sub-patterns).
///
/// The original logic lived inline in `record_pattern_variants_inner`'s
/// step (4) and only inspected the *outermost* constructor. Patterns of the
/// form `el @ ELEMENT { specification: spec @ CLASSDEF { .. } }` (common in
/// MM's traversal code) carry a nested constructor pattern that asserts the
/// inner variant — without recursion, the inner `spec` binding's variant is
/// never recorded and `emit_var` falls back to a plain `.field` access that
/// doesn't typecheck against the enum scrutinee.
fn record_constructor_pattern_bindings<'a>(
    pat: &TypedPat,
    scrut_ty: &Ty,
    env: &mut LocalEnv,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    shapes: &mut HashMap<String, VarShape>,
    ctx: Option<&GenCtx>,
) {
    // Option's `Some(inner)` pattern is represented separately (TypedPat::Some_).
    // Recurse into the payload using the inner type from `Option<T>`, so any
    // bindings the payload carries (variant narrowings, nested constructors)
    // are registered.
    if let TypedPat::Some_(inner_pat) = pat {
        let inner_ty = match scrut_ty {
            Ty::Option(t) => (**t).clone(),
            _ => Ty::Unknown,
        };
        if let TypedPat::As { var, pat: inner_as } = inner_pat.as_ref()
            && let Some((enum_q, variant)) = variant_of_pat(inner_as, &inner_ty, top_level) {
            env.variants.insert(var.clone(), (enum_q, variant));
        }
        let next_pat = match inner_pat.as_ref() {
            TypedPat::As { pat: inner_as, .. } => inner_as.as_ref(),
            other => other,
        };
        record_constructor_pattern_bindings(next_pat, &inner_ty, env, top_level, shapes, ctx);
        return;
    }
    let TypedPat::Constructor { name, named_fields, .. } = pat else { return };
    // Resolve the record's qname so we can look up field types. The
    // pattern's `ty` may already carry it; otherwise look it up against
    // the scrutinee's enum.
    let record_qname_opt = match record_field_tys_from_scrutinee_ctor(
        name.rsplit('.').next().unwrap_or(name),
        scrut_ty,
        top_level,
    ) {
        Some(tys) => Some(tys),
        None => {
            let v = record_field_tys_by_simple_name(name.rsplit('.').next().unwrap_or(name), top_level);
            if v.is_empty() { None } else { Some(v) }
        }
    };
    if let Some(field_tys) = record_qname_opt {
        let scrut_crosses_arc = ctx.map(|c| ty_needs_arc_match_deref(scrut_ty, c)).unwrap_or(false);
        for (fname, fpat) in named_fields {
            let Some(field_ty) = field_tys.iter().find(|(n, _)| n == fname).map(|(_, t)| t.clone()) else { continue };
            // (i) Register the field's own binding (if any) and its narrowing.
            //     `field = var as inner_pat` — `As` binding with a sub-pattern
            //     that may further narrow the variant.
            //     `field = var` — bare `Var` binding. No further constraint
            //     on `var` from this pattern, but the binding's Rust shape
            //     is still determined by whether scrut crosses Arc and the
            //     field itself is Arc-wrapped.
            //     Anything else (including a bare `Constructor` pattern with
            //     no binding) skips this step but still falls through to (ii)
            //     so nested bindings inside are recorded.
            if let Some(var) = match fpat {
                TypedPat::As { var, .. } | TypedPat::Var(var) => Some(var),
                _ => None,
            } {
                if let TypedPat::As { pat: inner_as, .. } = fpat
                    && let Some((enum_q, variant)) = variant_of_pat(inner_as, &field_ty, top_level) {
                    env.variants.insert(var.clone(), (enum_q, variant));
                }
                if let Some(c) = ctx {
                    let field_is_arc = is_arc_wrapped(&field_ty, c);
                    // Shape only matters for the combinations we know how to
                    // emit. Other combinations leave the shape unset and let
                    // emit_var's fallback handle them.
                    if let (true, true) = (scrut_crosses_arc, field_is_arc) {
                        shapes.insert(var.clone(), VarShape::RefArc);
                    }
                }
            }
            // (ii) Recurse into the field's sub-pattern unconditionally — a
            //      nested constructor (directly or wrapped in `As`) may carry
            //      further bindings whose variants/shapes must be registered.
            //      E.g. `Some(Annotation { modification: r#mod @ MOD { .. } })`:
            //      without recursion, `r#mod`'s MOD variant is never recorded
            //      and `r#mod.subModLst` lowers as a plain field access on
            //      `Arc<Mod>` (E0609) instead of `var_field!`.
            let sub_pat = match fpat {
                TypedPat::As { pat: inner_as, .. } => inner_as.as_ref(),
                other => other,
            };
            record_constructor_pattern_bindings(sub_pat, &field_ty, env, top_level, shapes, ctx);
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
    if let Ty::RustStruct(qname) = ty
        && let Some((parent, variant)) = qname.rsplit_once('.') {
            let simple_name = name.rsplit('.').next().unwrap_or(name);
            if simple_name == variant {
                return Some((parent.to_owned(), variant.to_owned()));
            }
        }
    // Fall back to scrutinee type: find the uniontype's record whose simple
    // name equals the pattern's name. Peel any number of Arc/Box/Mutable/
    // similar smart-pointer Generic wrappers — the underlying uniontype is
    // what matters for variant-name lookup.
    fn peel_to_enum_qname(t: &Ty) -> Option<String> {
        match t {
            Ty::RustEnum(q) | Ty::AliasTo(q) => Some(q.clone()),
            Ty::Generic(_, args) => args.first().and_then(peel_to_enum_qname),
            _ => None,
        }
    }
    let Some(enum_qname) = peel_to_enum_qname(scrut_ty) else { return None };
    let simple_name = name.rsplit('.').next().unwrap_or(name);
    let enum_node = lookup_node(&enum_qname, top_level)?;
    let NodeKind::Class(c) = &enum_node.kind else { return None };
    if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) {
        return None;
    }
    for (child_name, child_node) in &enum_node.children {
        if let NodeKind::Class(cc) = &child_node.kind
            && matches!(cc.restriction,
                Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })
                && child_name == simple_name
            {
                return Some((enum_qname.clone(), child_name.clone()));
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
        if let Ty::Tuple(_) = scrut_ty
            && !matches!(lhs_ty, Some(Ty::Tuple(_))) {
                expr = format!("{expr}.0");
            }
        if matches!(lhs_ty, Some(Ty::F64)) && *scrut_ty == Ty::I32 {
            expr = format!("metamodelica::OrderedFloat(({expr}) as f64)");
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
            // True when the RHS is an arrayCreateNoInit call.  Slots of the
            // resulting array are uninitialised; the first write to each slot
            // must use ptr::write (Dangerous::arrayInitSlot) rather than plain
            // `=` assignment so Rust does not drop the garbage bytes as if
            // they were a live Arc<T> value.
            let rhs_is_no_init = matches!(rhs, TypedExp::Call { func, .. } if func == "arrayCreateNoInit");
            let scrut_ty = rhs.ty();
            let scrut_expr = emit_exp(rhs, /*is_const=*/false, ctx, top_level);
            // For irrefutable patterns we still want a single binding form.
            // But MetaModelica often assigns to *existing* variables (declared as outputs
            // or `protected` components). For a Var pattern we emit a plain `<name> = expr;`
            // when the binding is already in scope, else a `let`. Heuristic: if env has it,
            // it's an output or earlier protected — emit assignment.
            if let TypedPat::Var(name) = lhs
                && env.vars.contains_key(name) {
                    // Plain reassignment may switch to a different variant — the
                    // previously-known variant assertion no longer holds.
                    env.variants.remove(name);
                    // Keep uninit_arrays in sync: if this variable is now
                    // holding a freshly-created noInit array, mark it;
                    // otherwise clear any prior mark (the slots are now valid).
                    if rhs_is_no_init {
                        ctx.uninit_arrays.insert(name.clone());
                    } else {
                        ctx.uninit_arrays.remove(name.as_str());
                    }
                    let lhs_ty = env.vars.get(name).cloned();
                    // MetaModelica permits assigning a multi-output call to a single
                    // variable; the unmentioned outputs are silently discarded.
                    // Emit `(name, _, _, ...) = expr;` so the user-visible binding
                    // gets the first output and the rest are dropped, while keeping
                    // the call expression evaluated exactly once.
                    if let Ty::Tuple(tys) = &scrut_ty
                        && !matches!(lhs_ty, Some(Ty::Tuple(_))) && tys.len() >= 2 {
                            let mut slots: Vec<String> = Vec::with_capacity(tys.len());
                            slots.push(escape_ident(name).to_string());
                            for _ in 1..tys.len() { slots.push("_".to_owned()); }
                            writeln!(out, "{indent}({}) = {scrut_expr};", slots.join(", ")).unwrap();
                            return;
                        }
                    let scrut_expr = coerce_assign_expr(scrut_expr, &scrut_ty, lhs_ty.as_ref());
                    writeln!(out, "{indent}{} = {scrut_expr};", escape_ident(name)).unwrap();
                    // Track that this function-scope variable now holds a value.
                    // Used by the matchcontinue-arm shadow logic (see
                    // `init_from_outer`) to decide whether to seed the arm-local
                    // shadow with `id.clone()` or leave it bare — the latter
                    // would propagate the outer's pre-init state, causing E0381
                    // when the arm reads `id` before assigning it again.
                    ctx.fn_initialized_vars.insert(name.clone());
                    return;
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
                    let mut slots: Vec<String> = pats.iter().map(|p| match p {
                        TypedPat::Var(n) => escape_ident(n).to_string(),
                        TypedPat::Wildcard => "_".to_owned(),
                        _ => unreachable!(),
                    }).collect();
                    // MetaModelica permits destructuring a wider tuple into a
                    // narrower LHS — trailing outputs are silently discarded.
                    // Pad the Rust pattern with `_` so the arities match.
                    if let Ty::Tuple(tys) = &scrut_ty
                        && tys.len() > slots.len() {
                            for _ in slots.len()..tys.len() { slots.push("_".to_owned()); }
                        }
                    writeln!(out, "{indent}({}) = {scrut_expr};", slots.join(", ")).unwrap();
                    for p in pats {
                        if let TypedPat::Var(n) = p {
                            ctx.fn_initialized_vars.insert(n.clone());
                        }
                    }
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
                        // Check if the base array is uninitialised (from arrayCreateNoInit).
                        // If so, use ptr::write via arrayInitSlot to avoid dropping garbage bytes.
                        let base_is_uninit = matches!(
                            base,
                            TypedExp::Var { name, .. } if ctx.uninit_arrays.contains(name.as_str())
                        );
                        writeln!(out, "{indent}{{").unwrap();
                        writeln!(out, "{indent}    let {tmp} = {scrut_expr};").unwrap();
                        if base_is_uninit {
                            writeln!(out, "{indent}    unsafe {{ metamodelica::Dangerous::arrayInitSlot({base_str}.clone(), {idx_str}, {tmp}); }}").unwrap();
                        } else {
                            writeln!(out, "{indent}    {base_str}.borrow_mut()[({idx_str}-1) as usize] = {tmp};").unwrap();
                        }
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
                // Fallback: emit `todo!()` instead of a broken direct assignment.
                // The previous version emitted `base.field = rhs;` so unhandled
                // shapes would surface as Rust errors, but for an Arc-wrapped
                // record (which is the usual case here) `base.field` does not
                // exist as a place — the assignment fails with E0609 and the
                // entire crate then fails to compile. A `todo!()` keeps the
                // surrounding function callable while still flagging the gap.
                let lhs_str = field_access_to_dotted(base, field);
                // Intentionally NOT embedding the rhs in the emitted comment:
                // `coerce_assign_expr` may return a multi-line rendering (e.g.
                // a list-comprehension lowered to a `{ let __acc; for ... {} }`
                // block). Pasting that into a `//` comment lets the body lines
                // escape the comment and break the surrounding parse. The
                // identifier in the `todo!` message is enough to locate the
                // gap; the full shape is recoverable from the MM source.
                writeln!(out, "{indent}todo!(\"unhandled field-assign shape: {lhs_str}\");").unwrap();
                return;
            }
            // For a fresh `let` binding of a single variable, also update
            // uninit_arrays so subsequent indexed writes use ptr::write.
            if let TypedPat::Var(name) = lhs {
                if rhs_is_no_init {
                    ctx.uninit_arrays.insert(name.clone());
                } else {
                    ctx.uninit_arrays.remove(name.as_str());
                }
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
            // Rust extends temporaries in a `for` operand to the entire loop scope.
            // If the operand reads through a `RefCell` (`.borrow()`), that Ref stays
            // alive across every iteration — and any `borrow_mut()` on the same cell
            // inside the body panics. Hoist into a local first so the Ref dies at
            // the `;` before the loop runs.
            if r.contains(".borrow(") || r.contains(".borrow_mut(") {
                let n = *fresh; *fresh += 1;
                writeln!(out, "{indent}let __range{n} = {r};").unwrap();
                writeln!(out, "{indent}for mut {} in __range{n} {{", escape_ident(var)).unwrap();
            } else {
                writeln!(out, "{indent}for mut {} in {r} {{", escape_ident(var)).unwrap();
            }
            // Element type: peel List/Array.
            let elem_ty = match range.ty() { Ty::List(t) | Ty::Array(t) | Ty::Range(t) => *t, _ => Ty::Unknown };
            // The `&*lst`/`arr.borrow().iter().cloned()` iterators above yield
            // `&T` for List and `T` for the cloned Array path; integer ranges
            // yield `T` directly. To present a uniform owned binding to the
            // body — so downstream codegen (e.g. `var_field!` shape selection)
            // can assume `var: T` rather than threading a `&T` borrow shape —
            // we shadow the loop variable with an owned clone for the List
            // case. For Arc-wrapped element types this is a cheap atomic
            // increment; for primitives/ArcStr it is also cheap. For other
            // types, MetaModelica semantics already treat list elements as
            // independent copies.
            if matches!(range.ty(), Ty::List(_)) {
                // `mut` because the loop body may rebind the variable
                // (assign_variant_field! mutates through &mut on the binding,
                // and MetaModelica's `for x in xs loop x := ... end for;`
                // semantics are translated as reassignments to the loop var).
                writeln!(out, "{indent}    let mut {0} = {0}.clone();", escape_ident(var)).unwrap();
            }
            let mut inner = env.clone();
            inner.vars.insert(var.clone(), elem_ty.clone());
            // Register the loop binding in ctx.fn_env_vars so emit_var's
            // variant-shape lookup can see its type (e.g. recognise an
            // Arc-wrapped element and select VarShape::Arc for var_field!).
            // Saved/restored so sibling scopes don't see the binding.
            let saved_arg_ty = ctx.fn_env_vars.insert(var.clone(), elem_ty);
            emit_stmts(out, &format!("{indent}    "), body, fail_mode, ctx, &mut inner, top_level, fresh);
            match saved_arg_ty {
                Some(t) => { ctx.fn_env_vars.insert(var.clone(), t); }
                None => { ctx.fn_env_vars.remove(var); }
            }
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
            // The fast path emits `if let Ok(PAT) = CALL { body } else { else }`
            // which requires the call to actually return `Result<T>`. When the
            // RHS is a known-infallible user call (returning `T` directly),
            // wrapping it in `Ok(..)` is a type error — fall through to the
            // general labeled-block lowering instead. We can't classify
            // builtins as infallible here (they all return `Result<T>` in the
            // current Rust signatures), so the check is restricted to
            // user-defined `Call` exprs.
            // Resolve through `resolve_call_qname` so unqualified call names
            // (e.g. `systemCall` inside `System.mo` calling a sibling) are
            // looked up in the right package scope before consulting the
            // fallibility map.
            let rhs_is_known_infallible = if body.len() == 1 {
                if let typedexp::TypedStmt::Assign { rhs: typedexp::TypedExp::Call { func, .. }, .. } = &body[0] {
                    resolve_call_qname(func, ctx, top_level)
                        .map(|q| ctx.is_known_infallible_user_fn(&q, top_level))
                        .unwrap_or(false)
                } else { false }
            } else { false };
            if !rhs_is_known_infallible
                && body.len() == 1 && matches!(stmts_flow(else_body), FlowResult::Diverges)
                && let typedexp::TypedStmt::Assign { lhs, rhs } = &body[0] {
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
            // MetaModelica semantics: if the body assigns a function output
            // the else branch does not (and else falls through), reaching the
            // function tail leaves outputs unset, which in MM is a matchfailure.
            // Materialise that as an explicit `bail!`/`panic!` at the end of
            // the else branch — see emission below. When this trigger fires
            // the else branch effectively diverges for flow-analysis purposes,
            // so treat it as Diverges in the yield-variable filter.
            let else_needs_fail: bool = match (&body_flow, &else_flow) {
                (FlowResult::FallsThrough(body_init), FlowResult::FallsThrough(else_init)) => {
                    // Only outputs that lack a default modification are uninitialised
                    // at function entry; an output declared `output T x = expr;` is
                    // already set and a try-fail leaves it at the default rather
                    // than unset. Restricting to no-default outputs avoids panicking
                    // on idioms like `List.filter` where `output list<T> outList = {};`
                    // is appended to in the try body and the else branch deliberately
                    // skips failed elements.
                    ctx.fn_outputs_no_default.iter().any(|o| body_init.contains(o) && !else_init.contains(o))
                }
                _ => false,
            };
            let yield_vars: Vec<String> = match &body_flow {
                FlowResult::Diverges => Vec::new(),
                FlowResult::FallsThrough(body_init) => {
                    let mut v: Vec<String> = body_init.iter()
                        .filter(|name| env.vars.contains_key(*name))
                        .filter(|name| else_needs_fail || match &else_flow {
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
                if else_needs_fail {
                    if ctx.current_fn_fallible {
                        writeln!(out, "{indent}    bail!(\"try/else: outputs not set in else branch\");").unwrap();
                    } else {
                        writeln!(out, "{indent}    panic!(\"try/else: outputs not set in else branch\");").unwrap();
                    }
                }
                writeln!(out, "{indent}}}").unwrap();
            } else {
                let escaped_vars: Vec<String> = yield_vars.iter().map(|n| escape_ident(n)).collect();
                // Yield via `.clone()` so the else branch (which reads the
                // pre-try value of these vars) still sees an owned value.
                // Without this, building `(exp,)` would move out of `exp` and
                // the Err arm's `exp.clone()` would fail with E0382.
                let yielded_exprs: Vec<String> = escaped_vars.iter()
                    .map(|n| format!("{n}.clone()"))
                    .collect();
                let yield_tuple = if yielded_exprs.len() == 1 {
                    format!("({},)", yielded_exprs[0])
                } else {
                    format!("({})", yielded_exprs.join(", "))
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
                if else_needs_fail {
                    if ctx.current_fn_fallible {
                        writeln!(out, "{indent}        bail!(\"try/else: outputs not set in else branch\");").unwrap();
                    } else {
                        writeln!(out, "{indent}        panic!(\"try/else: outputs not set in else branch\");").unwrap();
                    }
                }
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
            // Expand `return;` into the same shape that emit_function produces
            // at the tail: `Ok(tuple)` when the enclosing function is fallible,
            // bare `tuple` otherwise.
            let tail: String = match env.outputs.len() {
                0 => "()".to_owned(),
                1 => escape_ident(&env.outputs[0]),
                _ => {
                    let parts: Vec<String> = env.outputs.iter().map(|n| escape_ident(n)).collect();
                    format!("({})", parts.join(", "))
                }
            };
            if ctx.current_fn_fallible {
                writeln!(out, "{indent}return Ok({tail});").unwrap();
            } else {
                writeln!(out, "{indent}return {tail};").unwrap();
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
/// `Arc<dyn Fn(...) -> Result<...> + 'static>` rather than a concrete
/// `fn(...)` pointer or a named `partial function` alias (`KeyEq<T>`).
/// The `Arc<dyn Fn + 'static>` form:
///   * accepts a function item / `fn` pointer, a closure, or any other
///     `Fn(...) -> ...` implementor wrapped in `Arc::new(...)`,
///   * is `Clone` so callbacks can be forwarded freely through HOF chains
///     via `.clone()` without the `&&F` / monomorphisation-depth hazard
///     that `&impl Fn` caused, and
///   * is `'static` so a callback can be stored in a thread-local global
///     (e.g. `Error.updateCurrentComponent` stores its `prefixToStr`
///     parameter into `Globals::currentInstVar`).
///
/// `Fn` (not `FnMut`/`FnOnce`) is chosen because callbacks in this codebase
/// are invoked repeatedly inside `fold`/`map`/`forEach` loops and must not
/// consume their captures.
///
/// Notes / limitations:
///   * Named `partial function` aliases (e.g. `KeyEq<T>`) are still emitted
///     by `fmt_ty` itself, so they continue to work as struct field types
///     and `type X = ...` aliases. Inside such aliases the function value
///     is still a `fn` pointer; closures cannot be stored there. Lifting
///     that limitation requires switching the alias to `Arc<dyn Fn(...)>`
///     and updating every passing convention — out of scope for this change.
///   * `Ty::FunctionAlias` (re-export `function Foo = Bar`) is treated
///     identically to a plain function pointer; we still emit it by alias
///     name here, since lifting the same restriction needs the same refactor.
fn fmt_param_ty(ty: &Ty, ctx: &mut GenCtx) -> String {
    match ty {
        // Anonymous function-typed parameters are taken as
        // `Arc<dyn Fn(...) + 'static>`. Using `Arc` (rather than the
        // earlier `&impl Fn`) lets callers:
        //   * forward the same callback through nested HOF calls via
        //     `.clone()` without the `&&F`-type explosion that a raw
        //     reference caused (each `.clone()` produces the same
        //     `Arc<dyn Fn>`, so recursive monomorphisation stays
        //     at a single type level), and
        //   * store the callback in a `'static` thread-local slot.
        //
        // Named `partial function` aliases (e.g. `KeyEq`) are emitted
        // as the alias type directly because they may also live in
        // record fields where `impl Trait` / `dyn Trait` is undesired.
        Ty::Function { inputs, output, name: None, .. } => {
            let ins = inputs.iter().map(|inp| fmt_ty(&inp.ty, ctx)).collect::<Vec<_>>().join(", ");
            // Use a fully-qualified path so the trait reference doesn't collide
// with a same-named MetaModelica `partial function` type alias that
// may be brought into scope as `type Fn = fn(...);` (E0404).
format!("Arc<dyn ::std::ops::Fn({ins}) -> Result<{}> + 'static>", fmt_ty(output, ctx))
        }
        _ => fmt_ty(ty, ctx),
    }
}

/// Rewrite an `Array<T>` initializer string so it produces a
/// `StaticArray<T>` instead. Used for top-level `pub static` constants of
/// type `array<T>` — see `emit_node` for the rationale.
///
/// Only the well-known constructor shapes emitted by `emit_exp` /
/// `emit_builtin_call` are recognised:
///
///   - `metamodelica::Dangerous::listArray(<arg>).unwrap()` (from
///     `MetaModelica.Dangerous.listArrayLiteral`).
///   - `metamodelica::arrayFromVec(<arg>)` (from the `listArray` builtin
///     emit path, which is what plain `listArray(...)` initializers turn
///     into).
///
/// If the initializer does not match a known shape, prepend a TODO so the
/// (broken) static is impossible to miss at the use site rather than
/// silently producing the wrong type. Constant `array<T>` declarations
/// from new sources should be added here.
fn rewrite_array_init_for_static(init: &str) -> String {
    // Trim a trailing `.unwrap()` that follows a `Result`-returning call —
    // both rewritten constructors are infallible.
    let trimmed = init.trim();
    let prefix_listarray = "metamodelica::Dangerous::listArray(";
    let prefix_arrayfromvec = "metamodelica::arrayFromVec(";

    if let Some(rest) = trimmed.strip_prefix(prefix_listarray) {
        // Match the balanced parenthesised arg, then an optional `.unwrap()`.
        if let Some((arg, _tail)) = split_balanced_call_arg(rest) {
            return format!(
                "metamodelica::StaticArray::new({arg}.into_iter().cloned().collect())"
            );
        }
    }
    if let Some(rest) = trimmed.strip_prefix(prefix_arrayfromvec)
        && let Some((arg, _tail)) = split_balanced_call_arg(rest) {
            return format!("metamodelica::StaticArray::new({arg})");
        }
    // Unknown shape: keep the original initializer (which will not compile
    // as a `StaticArray<T>`) and annotate it so the failure is loud and
    // self-describing rather than a confusing type mismatch.
    format!(
        "/* TODO(StaticArray): unrecognised array-constant initializer shape; \
         add the constructor pattern to rewrite_array_init_for_static. */ {init}"
    )
}

/// Given `s` starting just past an opening `(`, return `(inner_arg, tail)`
/// where `inner_arg` is the substring up to the matching `)` and `tail` is
/// the substring after that `)`. Returns `None` if parentheses are unbalanced.
/// Skips over string literals so `(... "(" ...)` does not confuse the count.
fn split_balanced_call_arg(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 1;
    let mut i = 0usize;
    let mut in_str = false;
    let mut in_char = false;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if b == b'\\' { i += 2; continue; }
            if b == b'"' { in_str = false; }
        } else if in_char {
            if b == b'\\' { i += 2; continue; }
            if b == b'\'' { in_char = false; }
        } else {
            match b {
                b'"' => in_str = true,
                b'\'' => in_char = true,
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((&s[..i], &s[i + 1..]));
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    None
}

/// Run the workspace-wide `PartialEq` requirements analysis.
///
/// Walks every user-defined function in the hierarchy, type-checks its
/// body once, and computes the per-function set of type parameters that
/// need a `+ PartialEq` bound in the emitted Rust signature. The result
/// is consumed by [`emit_function`] (via `ctx.partial_eq_required`).
///
/// Two sources of requirement are recognised:
///   1. **Direct**: the body contains `==`/`!=` whose operand types
///      mention the type parameter, OR calls (or references) one of the
///      builtins in [`PARTIAL_EQ_REQUIRING_BUILTINS`] with such an
///      operand. Collected by [`visit_stmt_for_eq`] /
///      [`visit_exp_for_eq`].
///   2. **Transitive**: the body calls another user function `g` where
///      one of `g`'s type parameters needs `PartialEq` and the call's
///      argument substitution carries one of the caller's type
///      parameters into that slot. Computed by fixed-point on the call
///      graph using [`unify_subst`] over the typed argument/formal
///      types.
///
/// The fixed-point terminates because the requirement set per function
/// can only grow and is bounded by the function's declared type
/// parameters. Functions whose body cannot be typed (e.g. `external`
/// declarations) contribute no statements and so produce an empty set.
pub fn analyze_partial_eq<'a>(
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> BTreeMap<String, HashSet<String>> {
    // Collect every user-defined function class together with its FQN
    // (top-level package first), matching the convention used by
    // `fallibility::collect_functions`.
    let mut all_fns: Vec<(String, &'a NameNode<'a>)> = Vec::new();
    collect_all_function_nodes(top_level, "", &mut all_fns);

    // Functions without any type parameters cannot possibly need
    // `PartialEq` bounds — there's no `T` to bound. Skip them entirely:
    // re-typing their bodies is the dominant cost of this pass and the
    // overwhelming majority of MM functions are monomorphic. We still
    // visit them transitively (a non-generic function can be called from
    // a generic one), but only via the cached requirement set, which is
    // permanently empty for them — looked up by name in `required`.
    let mut cache: BTreeMap<String, Vec<typedexp::TypedStmt>> = BTreeMap::new();
    let mut required: BTreeMap<String, HashSet<String>> = BTreeMap::new();

    for (qname, node) in &all_fns {
        let has_type_vars = matches!(&node.ty, Ty::Function { inputs, output, type_vars, .. }
            if !type_vars.is_empty()
                || inputs.iter().any(|inp| ty_has_type_var(&inp.ty))
                || ty_has_type_var(output));
        if !has_type_vars {
            // Record an empty requirement set so downstream lookups still
            // resolve, and don't typecheck the body.
            required.insert(qname.clone(), HashSet::new());
            continue;
        }
        let stmts = typedexp_function_body_for_analysis(qname, node, top_level);
        let mut direct: HashSet<String> = HashSet::new();
        for s in &stmts { visit_stmt_for_eq(s, &mut direct); }
        required.insert(qname.clone(), direct);
        cache.insert(qname.clone(), stmts);
    }

    // Fixed-point: each iteration walks every cached body, looks up the
    // current requirement set for any user-function callee, and
    // propagates back through the call's substitution. Stops when no
    // function's set grows in an iteration.
    loop {
        let mut changed = false;
        // Keys are sorted (BTreeMap), so the iteration order is
        // deterministic across runs — useful for diffing generated code.
        let qnames: Vec<String> = cache.keys().cloned().collect();
        for qname in &qnames {
            let pkg_prefix = qname.rsplit_once('.').map_or("", |(p, _)| p).to_owned();
            let stmts = &cache[qname];
            let mut current = required[qname].clone();
            let before = current.len();
            for s in stmts {
                propagate_stmt_partial_eq(s, &required, top_level, &pkg_prefix, &mut current);
            }
            if current.len() > before {
                changed = true;
                required.insert(qname.clone(), current);
            }
        }
        if !changed { break; }
    }

    required
}

fn collect_all_function_nodes<'a>(
    nodes: &'a BTreeMap<String, NameNode<'a>>,
    prefix: &str,
    out: &mut Vec<(String, &'a NameNode<'a>)>,
) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Class(c) = &node.kind
            && matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
                out.push((qname.clone(), node));
            }
        collect_all_function_nodes(&node.children, &qname, out);
    }
}

/// Type-check a function's body in isolation so the analysis pass can
/// walk it for `PartialEq` requirements. Mirrors the env-setup in
/// `emit_function` but doesn't replicate its extends-merge logic — the
/// analysis only cares about the *expressions* in the body, not the
/// exact set of declared locals, so the simpler env-from-children build
/// is sufficient.
fn typedexp_function_body_for_analysis<'a>(
    fn_qname: &str,
    node: &NameNode<'_>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Vec<typedexp::TypedStmt> {
    let NodeKind::Class(c) = &node.kind else { return Vec::new(); };

    let mut all_type_vars: Vec<String> = Vec::new();
    if let Ty::Function { type_vars, inputs, output, .. } = &node.ty {
        all_type_vars = type_vars.clone();
        for inp in inputs.iter() {
            collect_type_vars_in_ty(&inp.ty, &mut all_type_vars);
        }
        collect_type_vars_in_ty(output, &mut all_type_vars);
    }

    let mut env: HashMap<String, Ty> = HashMap::new();
    for (name, child) in &node.children {
        env.insert(name.clone(), child.ty.clone());
    }

    let alg_items: &[Arc<Absyn::AlgorithmItem>] = match &c.body {
        MM::ClassDef::Parts { algorithms, .. } | MM::ClassDef::ClassExtends { algorithms, .. } => algorithms,
        _ => return Vec::new(),
    };

    let pkg_prefix = fn_qname.rsplit_once('.').map_or("", |(p, _)| p);
    typedexp::infer_stmts(alg_items, &mut env, top_level, pkg_prefix, &all_type_vars)
}

/// Walk a typed statement, looking for user-function calls. For each
/// such call, consult `required` for the callee's current requirement
/// set; if non-empty, derive the caller's type variables that flow into
/// the requiring callee slots via [`unify_subst`] and union them into
/// `out`.
///
/// This is the propagation half of the analysis. Direct requirements
/// are produced by [`visit_stmt_for_eq`].
fn propagate_stmt_partial_eq<'a>(
    stmt: &typedexp::TypedStmt,
    required: &BTreeMap<String, HashSet<String>>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    out: &mut HashSet<String>,
) {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { rhs, .. } => propagate_exp_partial_eq(rhs, required, top_level, pkg_prefix, out),
        S::NoRetCall { call } => propagate_exp_partial_eq(call, required, top_level, pkg_prefix, out),
        S::If { cond, then_, elseif, else_ } => {
            propagate_exp_partial_eq(cond, required, top_level, pkg_prefix, out);
            for s in then_ { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
            for (c, body) in elseif {
                propagate_exp_partial_eq(c, required, top_level, pkg_prefix, out);
                for s in body { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
            }
            for s in else_ { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
        }
        S::For { range, body, .. } => {
            propagate_exp_partial_eq(range, required, top_level, pkg_prefix, out);
            for s in body { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
        }
        S::While { cond, body } => {
            propagate_exp_partial_eq(cond, required, top_level, pkg_prefix, out);
            for s in body { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
        }
        S::Try { body, else_body } => {
            for s in body { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
            for s in else_body { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
        }
        S::Failure { body } => {
            for s in body { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
        }
        S::Return | S::Break | S::Continue | S::Todo(_) => {}
    }
}

fn propagate_exp_partial_eq<'a>(
    exp: &typedexp::TypedExp,
    required: &BTreeMap<String, HashSet<String>>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    out: &mut HashSet<String>,
) {
    use typedexp::TypedExp as E;
    match exp {
        E::Lit(_) | E::Todo(_) => {}
        E::Var { name, segments, ty } => {
            // A reference to a user function as a *value* (e.g. via
            // `fnptr!(union, _, _)` or a bare `&someFn` callback)
            // instantiates the callee's type parameters with whatever
            // the caller has at the use site. If any of the callee's
            // type parameters need `PartialEq`, the corresponding caller
            // type parameter needs it too. We use `unify_subst_collect`
            // between the *declared* function signature (looked up in
            // `top_level`) and the *use-site* type carried by the `Var`.
            //
            // Resolve via the full dotted name if it has dots; otherwise
            // try plain lookup followed by package-walked resolution.
            // The resolver in `typedexp::resolve_call_node` mirrors what
            // call sites use, so the same name resolves consistently.
            let lookup_name: String = if !segments.is_empty() {
                segments.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".")
            } else {
                name.clone()
            };
            if let Some(qname) = typedexp::resolve_call_node(&lookup_name, top_level, pkg_prefix).map(|(q, _)| q)
                && let Some(callee_req) = required.get(&qname)
                    && !callee_req.is_empty()
                        && let Some(callee_node) = typedexp_lookup_node(&qname, top_level) {
                            let mut subst: HashMap<String, HashSet<String>> = HashMap::new();
                            unify_subst_collect(&callee_node.ty, ty, &mut subst);
                            for callee_tv in callee_req {
                                if let Some(caller_tvs) = subst.get(callee_tv) {
                                    out.extend(caller_tvs.iter().cloned());
                                }
                            }
                        }
            for seg in segments {
                for sub in &seg.subscripts {
                    propagate_exp_partial_eq(sub, required, top_level, pkg_prefix, out);
                }
            }
        }
        E::BinOp { lhs, rhs, .. } => {
            propagate_exp_partial_eq(lhs, required, top_level, pkg_prefix, out);
            propagate_exp_partial_eq(rhs, required, top_level, pkg_prefix, out);
        }
        E::UnOp { operand, .. } => propagate_exp_partial_eq(operand, required, top_level, pkg_prefix, out),
        E::Call { func, args, named_args, .. } => {
            if let Some(qname) = typedexp::resolve_call_node(func, top_level, pkg_prefix).map(|(q, _)| q)
                && let Some(callee_req) = required.get(&qname)
                    && !callee_req.is_empty()
                        && let Some(callee_node) = typedexp_lookup_node(&qname, top_level)
                            && let Ty::Function { inputs: formals, .. } = &callee_node.ty {
                                let mut subst: HashMap<String, HashSet<String>> = HashMap::new();
                                for (i, arg) in args.iter().enumerate() {
                                    if let Some(formal) = formals.get(i) {
                                        unify_subst_collect(&formal.ty, &arg.ty(), &mut subst);
                                    }
                                }
                                for (n, arg) in named_args {
                                    if let Some(formal) = formals.iter().find(|f| &f.name == n) {
                                        unify_subst_collect(&formal.ty, &arg.ty(), &mut subst);
                                    }
                                }
                                for callee_tv in callee_req {
                                    if let Some(caller_tvs) = subst.get(callee_tv) {
                                        out.extend(caller_tvs.iter().cloned());
                                    }
                                }
                            }
            for a in args { propagate_exp_partial_eq(a, required, top_level, pkg_prefix, out); }
            for (_, v) in named_args { propagate_exp_partial_eq(v, required, top_level, pkg_prefix, out); }
        }
        E::Constructor { args, named_args, .. } | E::PartEval { args, named_args, .. } => {
            for a in args { propagate_exp_partial_eq(a, required, top_level, pkg_prefix, out); }
            for (_, v) in named_args { propagate_exp_partial_eq(v, required, top_level, pkg_prefix, out); }
        }
        E::If { cond, then_, elseif, else_, .. } => {
            propagate_exp_partial_eq(cond, required, top_level, pkg_prefix, out);
            propagate_exp_partial_eq(then_, required, top_level, pkg_prefix, out);
            for (c, b) in elseif {
                propagate_exp_partial_eq(c, required, top_level, pkg_prefix, out);
                propagate_exp_partial_eq(b, required, top_level, pkg_prefix, out);
            }
            propagate_exp_partial_eq(else_, required, top_level, pkg_prefix, out);
        }
        E::Cons { head, tail, .. } => {
            propagate_exp_partial_eq(head, required, top_level, pkg_prefix, out);
            propagate_exp_partial_eq(tail, required, top_level, pkg_prefix, out);
        }
        E::Tuple(elems) | E::Array { elems, .. } => {
            for e in elems { propagate_exp_partial_eq(e, required, top_level, pkg_prefix, out); }
        }
        E::Match { input, cases, .. } => {
            propagate_exp_partial_eq(input, required, top_level, pkg_prefix, out);
            for c in cases {
                if let Some(g) = &c.guard { propagate_exp_partial_eq(g, required, top_level, pkg_prefix, out); }
                for (_, _, d, _) in &c.locals {
                    if let Some(d) = d { propagate_exp_partial_eq(d, required, top_level, pkg_prefix, out); }
                }
                for s in &c.stmts { propagate_stmt_partial_eq(s, required, top_level, pkg_prefix, out); }
                propagate_exp_partial_eq(&c.result, required, top_level, pkg_prefix, out);
            }
        }
        E::Range { start, step, stop, .. } => {
            propagate_exp_partial_eq(start, required, top_level, pkg_prefix, out);
            if let Some(s) = step { propagate_exp_partial_eq(s, required, top_level, pkg_prefix, out); }
            propagate_exp_partial_eq(stop, required, top_level, pkg_prefix, out);
        }
        E::Reduction { body, iterators, .. } => {
            propagate_exp_partial_eq(body, required, top_level, pkg_prefix, out);
            for it in iterators {
                propagate_exp_partial_eq(&it.range, required, top_level, pkg_prefix, out);
                if let Some(g) = &it.guard { propagate_exp_partial_eq(g, required, top_level, pkg_prefix, out); }
            }
        }
    }
}

/// Walk both types in parallel; whenever the callee side reaches a
/// `Ty::TypeVar(c)`, record every type variable that appears in the
/// caller's side as a value substituted in for `c`. The mapping is
/// many-to-many (over-approximation) because a callee tv can appear in
/// multiple positions of the formal type, and the caller's type at the
/// argument position may itself be a structured type whose own type
/// vars all flow into the callee's `c` slot.
///
/// Used by the call-graph propagation step of [`analyze_partial_eq`]:
/// once we know which of the callee's type vars need `PartialEq`, we
/// look them up in this substitution to find the caller's type vars
/// that inherit the same requirement.
fn unify_subst_collect(callee_ty: &Ty, caller_ty: &Ty, out: &mut HashMap<String, HashSet<String>>) {
    match (callee_ty, caller_ty) {
        (Ty::TypeVar(c), _) => {
            let mut tvs: Vec<String> = Vec::new();
            collect_type_vars_in_ty(caller_ty, &mut tvs);
            out.entry(c.clone()).or_default().extend(tvs);
        }
        (Ty::Option(c), Ty::Option(t))
        | (Ty::List(c), Ty::List(t))
        | (Ty::Array(c), Ty::Array(t))
        | (Ty::Range(c), Ty::Range(t)) => {
            unify_subst_collect(c, t, out);
        }
        (Ty::Tuple(cs), Ty::Tuple(ts)) if cs.len() == ts.len() => {
            for (c, t) in cs.iter().zip(ts) { unify_subst_collect(c, t, out); }
        }
        (Ty::Generic(cn, cargs), Ty::Generic(tn, targs)) if cn == tn && cargs.len() == targs.len() => {
            for (c, t) in cargs.iter().zip(targs) { unify_subst_collect(c, t, out); }
        }
        (Ty::Function { inputs: ci, output: co, .. }, Ty::Function { inputs: ti, output: to, .. })
            if ci.len() == ti.len() =>
        {
            for (c, t) in ci.iter().zip(ti) { unify_subst_collect(&c.ty, &t.ty, out); }
            unify_subst_collect(co, to, out);
        }
        _ => {}
    }
}

/// True when `ty` mentions any `Ty::TypeVar` directly or through any
/// container. Used by [`analyze_partial_eq`] to skip the body re-typing
/// for fully monomorphic functions (the dominant majority).
fn ty_has_type_var(ty: &Ty) -> bool {
    match ty {
        Ty::TypeVar(_) => true,
        Ty::Option(t) | Ty::List(t) | Ty::Array(t) | Ty::Range(t) => ty_has_type_var(t),
        Ty::Tuple(ts) => ts.iter().any(ty_has_type_var),
        Ty::Generic(_, args) => args.iter().any(ty_has_type_var),
        Ty::Function { inputs, output, .. } => {
            inputs.iter().any(|inp| ty_has_type_var(&inp.ty)) || ty_has_type_var(output)
        }
        _ => false,
    }
}

fn typedexp_lookup_node<'a>(
    dotted: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<&'a NameNode<'a>> {
    let mut parts = dotted.split('.');
    let first = parts.next()?;
    let mut node = top_level.get(first)?;
    for part in parts {
        node = node.children.get(part)?;
    }
    Some(node)
}

/// MM-level builtins (and a handful of metamodelica-crate functions exposed
/// as builtins) whose Rust signature requires `T: PartialEq` on the type
/// parameter that flows into the operand position. Used by
/// [`collect_partial_eq_type_vars`] to decide which type parameters of the
/// enclosing function need a `+ PartialEq` bound.
///
/// `valueEq`, `listMember`, and `referenceEq` are the only such builtins in
/// MetaModelica today; add to this list if more are introduced (or if a
/// builtin gains a `PartialEq` bound on a previously-unconstrained type
/// parameter). The handful of builtins listed here is the entire set of
/// places MM source code can require structural equality on a generic
/// type, so a body that never reaches one of these (and never uses `==`
/// directly) genuinely does not need `PartialEq` on its type parameters.
const PARTIAL_EQ_REQUIRING_BUILTINS: &[&str] = &["valueEq", "listMember", "referenceEq"];

/// Builtins that require `T: 'static` on the type vars flowing into them.
/// These cross a `dyn Any` type-erased boundary in the metamodelica runtime:
///   * `getGlobalRoot`/`setGlobalRoot` store/recover values through
///     `Rc<dyn Any>` (and `Any: 'static`).
///   * `referenceEq`/`referenceDebugString` lower to pointer comparisons that
///     need the value to outlive the comparison; we keep them in this set to
///     match the runtime's `'static` bound.
const STATIC_REQUIRING_BUILTINS: &[&str] = &[
    "getGlobalRoot",
    "setGlobalRoot",
    "valueConstructor", // also touches Any
    "referenceEq",
    "referenceDebugString",
];

/// Walk a function body looking for builtins that require `T: 'static` on
/// their type arguments. Returns the set of caller type-variable names that
/// flow into such builtin call sites. Used by [`emit_function`] to add the
/// `'static` bound only to type parameters that actually need it.
fn analyze_static_required_in_body(stmts: &[typedexp::TypedStmt]) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    for s in stmts { visit_stmt_for_static(s, &mut out); }
    out
}

fn visit_stmt_for_static(stmt: &typedexp::TypedStmt, out: &mut std::collections::HashSet<String>) {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { rhs, .. } => visit_exp_for_static(rhs, out),
        S::NoRetCall { call } => visit_exp_for_static(call, out),
        S::If { cond, then_, elseif, else_ } => {
            visit_exp_for_static(cond, out);
            for s in then_ { visit_stmt_for_static(s, out); }
            for (c, body) in elseif {
                visit_exp_for_static(c, out);
                for s in body { visit_stmt_for_static(s, out); }
            }
            for s in else_ { visit_stmt_for_static(s, out); }
        }
        S::For { range, body, .. } => {
            visit_exp_for_static(range, out);
            for s in body { visit_stmt_for_static(s, out); }
        }
        S::While { cond, body } => {
            visit_exp_for_static(cond, out);
            for s in body { visit_stmt_for_static(s, out); }
        }
        S::Try { body, else_body } => {
            for s in body { visit_stmt_for_static(s, out); }
            for s in else_body { visit_stmt_for_static(s, out); }
        }
        S::Failure { body } => {
            for s in body { visit_stmt_for_static(s, out); }
        }
        S::Return | S::Break | S::Continue | S::Todo(_) => {}
    }
}

fn visit_exp_for_static(exp: &typedexp::TypedExp, out: &mut std::collections::HashSet<String>) {
    use typedexp::TypedExp as E;
    match exp {
        E::Lit(_) | E::Todo(_) => {}
        E::Var { name, segments, ty } => {
            let bare = name.rsplit('.').next().unwrap_or(name);
            if STATIC_REQUIRING_BUILTINS.contains(&bare) {
                let mut tvs = Vec::new();
                crate::hierarchy::collect_type_vars_in_ty(ty, &mut tvs);
                out.extend(tvs);
            }
            for seg in segments {
                for sub in &seg.subscripts {
                    visit_exp_for_static(sub, out);
                }
            }
        }
        E::BinOp { lhs, rhs, .. } => { visit_exp_for_static(lhs, out); visit_exp_for_static(rhs, out); }
        E::UnOp { operand, .. } => visit_exp_for_static(operand, out),
        E::Call { func, args, named_args, ty, .. } => {
            let bare = func.rsplit('.').next().unwrap_or(func);
            if STATIC_REQUIRING_BUILTINS.contains(&bare) {
                // Result type (e.g. `getGlobalRoot(): Option<(Array<T>, ...)>`)
                // carries the most precise type-arg info; arguments do too
                // (e.g. `setGlobalRoot(idx, val)` — `val.ty()` mentions T).
                let mut tvs = Vec::new();
                crate::hierarchy::collect_type_vars_in_ty(ty, &mut tvs);
                for a in args {
                    crate::hierarchy::collect_type_vars_in_ty(&a.ty(), &mut tvs);
                }
                for (_, v) in named_args {
                    crate::hierarchy::collect_type_vars_in_ty(&v.ty(), &mut tvs);
                }
                out.extend(tvs);
            }
            for a in args { visit_exp_for_static(a, out); }
            for (_, v) in named_args { visit_exp_for_static(v, out); }
        }
        E::Constructor { args, named_args, .. } | E::PartEval { args, named_args, .. } => {
            for a in args { visit_exp_for_static(a, out); }
            for (_, v) in named_args { visit_exp_for_static(v, out); }
        }
        E::If { cond, then_, elseif, else_, .. } => {
            visit_exp_for_static(cond, out);
            visit_exp_for_static(then_, out);
            for (c, b) in elseif { visit_exp_for_static(c, out); visit_exp_for_static(b, out); }
            visit_exp_for_static(else_, out);
        }
        E::Cons { head, tail, .. } => { visit_exp_for_static(head, out); visit_exp_for_static(tail, out); }
        E::Tuple(elems) | E::Array { elems, .. } => { for e in elems { visit_exp_for_static(e, out); } }
        E::Match { input, cases, .. } => {
            visit_exp_for_static(input, out);
            for c in cases {
                if let Some(g) = &c.guard { visit_exp_for_static(g, out); }
                for (_, _, d, _) in &c.locals {
                    if let Some(d) = d { visit_exp_for_static(d, out); }
                }
                for s in &c.stmts { visit_stmt_for_static(s, out); }
                visit_exp_for_static(&c.result, out);
            }
        }
        E::Range { start, step, stop, .. } => {
            visit_exp_for_static(start, out);
            if let Some(s) = step { visit_exp_for_static(s, out); }
            visit_exp_for_static(stop, out);
        }
        _ => {}
    }
}

/// Walk a list of [`TypedStmt`]s (typically the algorithm body of a
/// function) and collect the names of type variables that appear in
/// operand types of `==` / `!=` or in argument types of a known
/// `PartialEq`-requiring builtin call. These are exactly the type
/// parameters of the enclosing function that need a `+ PartialEq` bound.
///
/// Type parameters not in the result set are emitted with `Clone + 'static`
/// only — callers can then forward a non-`PartialEq` value (e.g. a
/// `&impl Fn(...)` callback) through generic forwarders like
/// `List::map3(.., extra_arg, ..)` without tripping the bound. See the
/// `Graph.rs` call sites of `map3`/`map2`/`fold2` for the motivating
/// MetaModelica idiom.
///
/// The walk does NOT cross into user-defined callees (no
/// transitive analysis): a call `f(x)` where the type variable `T` of the
/// enclosing function flows into `f`'s `T: PartialEq` parameter would not
/// be detected here, and the callee's bound would surface at the call
/// site as `T: PartialEq is not satisfied`. In practice, MetaModelica
/// equality goes through the listed builtins, so this conservative scope
/// is sufficient for current code; if a propagation gap arises, extend
/// this with a fixed-point pass keyed on the call graph (see
/// `fallibility::analyze` for a similar shape).
fn collect_partial_eq_type_vars(stmts: &[typedexp::TypedStmt]) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    for s in stmts {
        visit_stmt_for_eq(s, &mut out);
    }
    out
}

fn visit_stmt_for_eq(stmt: &typedexp::TypedStmt, out: &mut std::collections::HashSet<String>) {
    use typedexp::TypedStmt as S;
    match stmt {
        S::Assign { rhs, .. } => visit_exp_for_eq(rhs, out),
        S::NoRetCall { call } => visit_exp_for_eq(call, out),
        S::If { cond, then_, elseif, else_ } => {
            visit_exp_for_eq(cond, out);
            for s in then_ { visit_stmt_for_eq(s, out); }
            for (c, body) in elseif {
                visit_exp_for_eq(c, out);
                for s in body { visit_stmt_for_eq(s, out); }
            }
            for s in else_ { visit_stmt_for_eq(s, out); }
        }
        S::For { range, body, .. } => {
            visit_exp_for_eq(range, out);
            for s in body { visit_stmt_for_eq(s, out); }
        }
        S::While { cond, body } => {
            visit_exp_for_eq(cond, out);
            for s in body { visit_stmt_for_eq(s, out); }
        }
        S::Try { body, else_body } => {
            for s in body { visit_stmt_for_eq(s, out); }
            for s in else_body { visit_stmt_for_eq(s, out); }
        }
        S::Failure { body } => {
            for s in body { visit_stmt_for_eq(s, out); }
        }
        S::Return | S::Break | S::Continue | S::Todo(_) => {}
    }
}

fn visit_exp_for_eq(exp: &typedexp::TypedExp, out: &mut std::collections::HashSet<String>) {
    use typedexp::{TypedExp as E, BinOpKind};
    match exp {
        E::Lit(_) | E::Todo(_) => {}
        E::Var { name, segments, ty } => {
            // A reference to one of the PartialEq-requiring builtins as a
            // *value* (e.g. `&valueEq` passed as a callback to
            // `deleteMemberOnTrue(..., &valueEq)`) instantiates the
            // builtin's `T: PartialEq` bound with whatever type-var the
            // caller has — and the caller therefore needs `T: PartialEq`
            // too. Match on the bare last segment so qualified forms
            // (`MetaModelica.valueEq`, `Dangerous.referenceEq`, …) are
            // recognised.
            let bare = name.rsplit('.').next().unwrap_or(name);
            if PARTIAL_EQ_REQUIRING_BUILTINS.contains(&bare) {
                let mut tvs = Vec::new();
                crate::hierarchy::collect_type_vars_in_ty(ty, &mut tvs);
                out.extend(tvs);
            }
            for seg in segments {
                for sub in &seg.subscripts {
                    visit_exp_for_eq(sub, out);
                }
            }
        }
        E::BinOp { op, lhs, rhs, .. } => {
            if matches!(op, BinOpKind::Eq | BinOpKind::NEq) {
                // Structural equality requires `PartialEq` on the type of
                // each operand. Collect the type variables that participate.
                let mut tvs = Vec::new();
                crate::hierarchy::collect_type_vars_in_ty(&lhs.ty(), &mut tvs);
                crate::hierarchy::collect_type_vars_in_ty(&rhs.ty(), &mut tvs);
                out.extend(tvs);
            }
            visit_exp_for_eq(lhs, out);
            visit_exp_for_eq(rhs, out);
        }
        E::UnOp { operand, .. } => visit_exp_for_eq(operand, out),
        E::Call { func, args, named_args, .. } => {
            // Direct calls to the listed builtins propagate `PartialEq`
            // requirements onto the type vars that appear in the operand
            // types. Match against the bare last segment so qualified
            // forms (`MetaModelica.valueEq`, `Dangerous.referenceEq`,
            // etc.) are recognised too.
            let bare = func.rsplit('.').next().unwrap_or(func);
            if PARTIAL_EQ_REQUIRING_BUILTINS.contains(&bare) {
                for a in args {
                    let mut tvs = Vec::new();
                    crate::hierarchy::collect_type_vars_in_ty(&a.ty(), &mut tvs);
                    out.extend(tvs);
                }
                for (_, v) in named_args {
                    let mut tvs = Vec::new();
                    crate::hierarchy::collect_type_vars_in_ty(&v.ty(), &mut tvs);
                    out.extend(tvs);
                }
            }
            for a in args { visit_exp_for_eq(a, out); }
            for (_, v) in named_args { visit_exp_for_eq(v, out); }
        }
        E::Constructor { args, named_args, .. } => {
            for a in args { visit_exp_for_eq(a, out); }
            for (_, v) in named_args { visit_exp_for_eq(v, out); }
        }
        E::PartEval { args, named_args, .. } => {
            for a in args { visit_exp_for_eq(a, out); }
            for (_, v) in named_args { visit_exp_for_eq(v, out); }
        }
        E::If { cond, then_, elseif, else_, .. } => {
            visit_exp_for_eq(cond, out);
            visit_exp_for_eq(then_, out);
            for (c, b) in elseif {
                visit_exp_for_eq(c, out);
                visit_exp_for_eq(b, out);
            }
            visit_exp_for_eq(else_, out);
        }
        E::Cons { head, tail, .. } => {
            visit_exp_for_eq(head, out);
            visit_exp_for_eq(tail, out);
        }
        E::Tuple(elems) => { for e in elems { visit_exp_for_eq(e, out); } }
        E::Array { elems, .. } => { for e in elems { visit_exp_for_eq(e, out); } }
        E::Match { input, cases, .. } => {
            visit_exp_for_eq(input, out);
            for c in cases {
                if let Some(g) = &c.guard { visit_exp_for_eq(g, out); }
                for (_, _, d, _) in &c.locals {
                    if let Some(d) = d { visit_exp_for_eq(d, out); }
                }
                for s in &c.stmts { visit_stmt_for_eq(s, out); }
                visit_exp_for_eq(&c.result, out);
            }
        }
        E::Range { start, step, stop, .. } => {
            visit_exp_for_eq(start, out);
            if let Some(s) = step { visit_exp_for_eq(s, out); }
            visit_exp_for_eq(stop, out);
        }
        E::Reduction { body, iterators, .. } => {
            visit_exp_for_eq(body, out);
            for it in iterators {
                visit_exp_for_eq(&it.range, out);
                if let Some(g) = &it.guard { visit_exp_for_eq(g, out); }
            }
        }
    }
}

/// Return a Rust expression for the Modelica default value of `ty`, when one
/// exists. Used to mimic MetaModelica's implicit type-default initialisation
/// of outputs and protected locals: `Boolean` defaults to `false`, `Integer`
/// to `0`, lists to `nil`, etc. Returns `None` for types whose default is
/// either undefined (function types, type variables) or non-trivial to emit
/// (records, enums, generics) — those locals stay uninitialised and rely on
/// every code path assigning them before use.
fn ty_default_init(ty: &Ty) -> Option<String> {
    match ty {
        Ty::Bool => Some("false".to_owned()),
        Ty::I32 => Some("0".to_owned()),
        Ty::F64 => Some("metamodelica::OrderedFloat(0.0_f64)".to_owned()),
        Ty::Str => Some("arcstr::literal!(\"\")".to_owned()),
        Ty::Option(_) => Some("None".to_owned()),
        Ty::List(_) => Some("metamodelica::nil()".to_owned()),
        _ => None,
    }
}

fn fmt_ty(ty: &Ty, ctx: &mut GenCtx) -> String {
    match ty {
        Ty::Unknown => "/* ? */".to_owned(),
        Ty::I32 => "i32".to_owned(),
        Ty::F64 => "metamodelica::Real".to_owned(),
        Ty::Bool => "bool".to_owned(),
        Ty::Str => "ArcStr".to_owned(),
        Ty::Unit => "()".to_owned(),
        Ty::TypeVar(name) => name.clone(),
        Ty::RustUnitVariant => "()".to_owned(),
        Ty::Enumeration(name) => ctx.shorten(name),
        Ty::RustStruct(name) => {
            // Single-record uniontypes at file top level have their record's
            // hierarchy-resolved type set to `Ty::RustStruct(<uniontype-qname>)`
            // (see the uniontype seeding in hierarchy.rs). The actual Rust
            // emission is `pub struct <Name>` inside the file's own module
            // `<Name>.rs` — so from another file, an `import Local = <Name>;`
            // binds the *module*, and the type is reached as `Local::<Name>`.
            // `shorten` matches the named import and returns just `Local`, so
            // we need the same `::<Name>` doubling the RustEnum branch applies.
            //
            // We only double for the top-level same-name case: nested records
            // (`UnionType.Record1`) already shorten to `UnionType::Record1`,
            // which is correct.
            let first = name.split('.').next().unwrap_or(name);
            let last = name.rsplit('.').next().unwrap_or(name);
            let shortened = ctx.shorten(name);
            let in_own_mod = ctx.current_path.last().map(|p| p == last).unwrap_or(false);
            let needs_doubling = !in_own_mod && !ctx.no_mod_uniontypes.contains(name.as_str()) && (
                (ctx.top_level_uniontype_names.contains(first) && first != ctx.top_name && first == last)
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
                // Function-nested `partial function` aliases cannot be referenced
                // by their qualified Rust name (the parent function has no
                // namespace to host them). Fall through to the inline
                // `fn(...) -> Result<...>` form for those.
                if ctx.nested_partial_aliases.contains(qname.as_str()) {
                    let ins = inputs.iter().map(|inp| fmt_ty(&inp.ty, ctx)).collect::<Vec<_>>().join(", ");
                    return // Use a fully-qualified path so the trait reference doesn't collide
// with a same-named MetaModelica `partial function` type alias that
// may be brought into scope as `type Fn = fn(...);` (E0404).
format!("Arc<dyn ::std::ops::Fn({ins}) -> Result<{}> + 'static>", fmt_ty(output, ctx));
                }
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
        Ty::ExternalObject(qname) => {
            // External objects are nominal opaque types. Each one is a distinct
            // Rust struct defined in the module that declares the class. The
            // qname is the fully-qualified dotted path (e.g. `File.File`); turn
            // it into a Rust path so callers in other modules can name the type.
            // Inside its own module the doubled `Mod::Mod` is what callers in
            // other crates write today — we keep the path uniform rather than
            // collapse the doubling, matching how user code (e.g. `Tpl.mo`)
            // writes `File.File` to reference the type.
            let dotted = qname.as_str();
            ctx.shorten(dotted)
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
            if let MM::ClassMember::ClassDef(cdm) = m
                && matches!(cdm.class_def.restriction,
                    Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })
                {
                    return Some(cdm.class_def.name.clone());
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

/// Same as [`component_fields`] but also returns each component's original
/// AST `TypeSpec`. The TypeSpec lets the emitter recover the type *name as
/// written* (e.g. `Key`, `Value`) rather than its fully resolved Rust type
/// (e.g. `ArcStr`, `Arc<metamodelica::List<TplAbsyn::ASTDef>>`). This is what
/// lets uniontype record fields reference sibling type aliases instead of
/// inlining the resolved type — keeping the generated code stable across
/// future redeclarations of the alias.
fn component_fields_with_spec<'a>(
    c: &'a MM::Class,
    children: &'a BTreeMap<String, NameNode<'_>>,
) -> Vec<(&'a str, &'a Ty, &'a Absyn::TypeSpec)> {
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return vec![],
    };
    members.iter()
        .filter_map(|m| {
            if let MM::ClassMember::Component(comp) = m {
                let ty = children.get(&comp.name).map(|n| &n.ty)?;
                Some((comp.name.as_str(), ty, comp.type_spec.as_ref()))
            } else {
                None
            }
        })
        .collect()
}

/// Return the `BTreeMap` of children for the package/uniontype currently
/// being emitted, i.e. the scope where this uniontype/struct's siblings
/// (such as type aliases) live. Walks `top_level` along `ctx.top_name` +
/// `ctx.current_path`; returns `None` if any segment is missing.
fn current_scope_children<'a, 'b>(
    ctx: &GenCtx,
    top_level: &'a BTreeMap<String, NameNode<'b>>,
) -> Option<&'a BTreeMap<String, NameNode<'b>>> {
    let root = top_level.get(ctx.top_name.as_str())?;
    let mut node = root;
    for seg in &ctx.current_path {
        node = node.children.get(seg.as_str())?;
    }
    Some(&node.children)
}

/// If the given `type_spec` is a single-identifier reference to a sibling
/// type alias in `scope_children` (i.e. a `(redeclare) type Foo = …` class
/// member at the enclosing-package scope), return that identifier escaped
/// as a Rust ident. Otherwise return `None`.
///
/// Preserves alias references in uniontype record fields: `Key key;` /
/// `Value value;` in a MetaModelica `extends BaseAvlTree` package emits
/// `key: Key,` / `value: Value,` instead of the resolved concrete type. The
/// alias still expands to the correct Rust type via the `pub type Key = …;`
/// / `pub type Value = …;` emitted at the package level by `emit_type_item`.
fn field_type_alias_name(
    type_spec: &Absyn::TypeSpec,
    scope_children: &BTreeMap<String, NameNode<'_>>,
) -> Option<String> {
    // Only a bare TPATH with one segment (no module prefix, no generics) can
    // be an alias to a sibling — anything else (e.g. `Pkg.Type`, `list<T>`)
    // already names what it means.
    let path = match type_spec {
        Absyn::TypeSpec::TPATH { path, arrayDim: None, .. } => path,
        _ => return None,
    };
    let name = match &**path {
        Absyn::Path::IDENT { name } => name.to_string(),
        _ => return None,
    };
    let child = scope_children.get(&name)?;
    let NodeKind::Class(cc) = &child.kind else { return None };
    if !matches!(cc.restriction, Absyn::Restriction::R_TYPE) { return None; }
    // The child must itself be a type alias (Derived) — not an enumeration etc.
    if !matches!(cc.body, MM::ClassDef::Derived { .. }) { return None; }
    // Skip aliases that ARE a type variable themselves
    // (`replaceable type T subtypeof Any` — child.ty == Ty::TypeVar(T)). The
    // generated Rust already has `T` as a function/struct type parameter,
    // and emitting `T<T>` would be a non-existent generic — `escape_ident`
    // already handles the bare-T case via the normal fmt_ty path.
    if matches!(child.ty, Ty::TypeVar(_)) { return None; }
    // Generic aliases (`type HashSet<K> = …`) emit `pub type Name<T1, T2> = …`
    // with the TypeVars collected from the alias body in
    // `collect_type_vars_in_ty` order — mirror that here so the use-site
    // `Name<T1, T2>` references match the declaration. The names refer to
    // the package's `replaceable type T subtypeof Any` declarations, which
    // are in scope at every sibling function — so emitting the bare TypeVar
    // names is valid wherever this helper is used.
    let mut tvs = Vec::new();
    collect_type_vars_in_ty(&child.ty, &mut tvs);
    let base = escape_ident(&name);
    if tvs.is_empty() {
        Some(base)
    } else {
        Some(format!("{base}<{}>", tvs.join(", ")))
    }
}

/// Prefix Rust keywords with `r#` so they are valid identifiers.
/// `self`, `super`, `crate`, and `Self` cannot be raw identifiers and are left as-is.
fn escape_ident(name: &str) -> String {
    // Path-style identifiers (e.g. `Expression::typeof`, `crate::Foo`) must
    // have *every* segment checked for keyword collisions, not just the whole
    // string. Without this, a path whose tail is a Rust reserved keyword
    // (`Expression::typeof`, `Foo::r#try::bar`, …) fails to parse. We handle
    // the `MetaModelica::Dangerous` rewrite before the split so the special
    // case still fires.
    if name.starts_with("MetaModelica::Dangerous") {
        let rewritten = name.replace("MetaModelica::Dangerous", "metamodelica::Dangerous");
        if rewritten.contains("::") {
            return rewritten.split("::").map(escape_ident).collect::<Vec<_>>().join("::");
        }
        return rewritten;
    }
    if name.contains("::") {
        return name.split("::").map(escape_ident).collect::<Vec<_>>().join("::");
    }
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
        return name.replace("MetaModelica::Dangerous", "metamodelica::Dangerous").to_string();
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

/// Walk the trailing simple name of a `ComponentRef`. External output
/// declarations are always a bare identifier (no subscripts, no
/// qualification), so we ignore the recursive arms — they would indicate
/// a malformed `external` clause and should be surfaced as `_unknown`.
fn component_ref_simple_name(cref: &Absyn::ComponentRef) -> String {
    match cref {
        Absyn::ComponentRef::CREF_IDENT { name, .. } => name.to_string(),
        Absyn::ComponentRef::CREF_QUAL { name, .. } => name.to_string(),
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef } => {
            component_ref_simple_name(componentRef)
        }
        Absyn::ComponentRef::WILD => "_".to_owned(),
        _ => "_unknown".to_owned(),
    }
}

/// Extract the argument identifiers from an external "C" call's argument
/// list. The C side typically receives `OpenModelica.threadData()` as the
/// first argument; that call has no MetaModelica-visible value, so we
/// drop it. Everything else is expected to be a plain `CREF` to a
/// function input parameter — anything more complex (a literal, a nested
/// expression) is rejected with a placeholder so the broken case shows
/// up at compile time instead of silently dropping arguments.
fn collect_external_arg_names(args: &std::sync::Arc<metamodelica::List<std::sync::Arc<Absyn::Exp>>>) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = args.clone();
    while let metamodelica::List::Cons { head, tail } = &*cur {
        match &**head {
            // `OpenModelica.threadData()` — drop.
            Absyn::Exp::CALL { function_, .. }
                if matches!(&**function_,
                    Absyn::ComponentRef::CREF_QUAL { name, componentRef, .. }
                        if name.as_str() == "OpenModelica"
                            && matches!(&**componentRef,
                                Absyn::ComponentRef::CREF_IDENT { name: inner, .. }
                                    if inner.as_str() == "threadData")) =>
            {
                // skip
            }
            Absyn::Exp::CREF { componentRef } => {
                out.push(component_ref_simple_name(componentRef));
            }
            _ => {
                out.push("/* unsupported external arg */".to_owned());
            }
        }
        cur = tail.clone();
    }
    out
}
