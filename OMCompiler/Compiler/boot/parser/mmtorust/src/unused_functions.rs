//! Reachability analysis from `Main.main`.
//!
//! Optional pass invoked via the `unused-functions` subcommand. Walks every
//! user-defined MetaModelica function class, collects the union of names it
//! references (direct calls, partial applications, and bare `CREF` uses that
//! resolve to an R_FUNCTION node — the function-pointer case used by
//! higher-order helpers like `List.map`), then performs a forward reachability
//! search from `Main.main` and reports every function that the search did not
//! reach.
//!
//! Limitations:
//!   * Approximate — a function referenced only through a name constructed at
//!     runtime (string-based reflection, code templates) will appear unused.
//!     There is no such mechanism in the current Compiler/, but unresolved
//!     names are reported in the summary so users can audit.
//!   * Builtins / externals are ignored — only user-defined functions
//!     participate in the call graph.
//!   * `function Foo = Bar(...)` aliases keep the base reachable: visiting
//!     the alias enqueues its base.
//!   * Functions referenced from non-function contexts (e.g. as values inside
//!     constant initializers in a `package` body) are NOT walked — only
//!     function bodies are scanned. If this matters in the future, expand the
//!     walker to cover component default expressions.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use openmodelica_ast::Absyn;

use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind, Ty};
use crate::typedexp::{cref_to_dotted, resolve_call_node};
use crate::MM;

const ROOT: &str = "Main.main";

/// Collect every R_FUNCTION node together with its FQN and the resolved alias
/// base (if `function Foo = Bar(...)`).
fn collect_functions<'a>(
    nodes: &BTreeMap<String, NameNode<'a>>,
    prefix: &str,
    out: &mut Vec<(String, &'a MM::Class, Option<String>)>,
) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Class(c) = &node.kind
            && matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. })
        {
            let alias_base = match &node.ty {
                Ty::FunctionAlias { base, .. } => Some(base.clone()),
                _ => None,
            };
            out.push((qname.clone(), *c, alias_base));
        }
        collect_functions(&node.children, &qname, out);
    }
}

#[derive(Default)]
struct RefScan {
    /// Raw dotted names that appear as callees (CALL / ALG_NORETCALL /
    /// PARTEVALFUNCTION callees) and as standalone CREF expressions that
    /// might denote a function value.
    refs: BTreeSet<String>,
}

impl RefScan {
    fn scan_class(c: &MM::Class) -> Self {
        let mut s = RefScan::default();
        let algorithms = match &c.body {
            MM::ClassDef::Parts { algorithms, .. } => algorithms,
            MM::ClassDef::ClassExtends { algorithms, .. } => algorithms,
            _ => return s,
        };
        for it in algorithms {
            s.scan_algorithm_item(it);
        }
        s
    }

    fn scan_algorithm_item(&mut self, it: &Absyn::AlgorithmItem) {
        let alg = match it {
            Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_, .. } => &**algorithm_,
            Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { .. } => return,
        };
        match alg {
            Absyn::Algorithm::ALG_ASSIGN { assignComponent, value } => {
                self.scan_exp(assignComponent);
                self.scan_exp(value);
            }
            Absyn::Algorithm::ALG_IF { ifExp, trueBranch, elseIfAlgorithmBranch, elseBranch } => {
                self.scan_exp(ifExp);
                for it in &**trueBranch { self.scan_algorithm_item(it); }
                for (cond, branch) in &**elseIfAlgorithmBranch {
                    self.scan_exp(cond);
                    for it in &**branch { self.scan_algorithm_item(it); }
                }
                for it in &**elseBranch { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_FOR { forBody, .. }
            | Absyn::Algorithm::ALG_PARFOR { parforBody: forBody, .. } => {
                for it in &**forBody { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_WHILE { boolExpr, whileBody } => {
                self.scan_exp(boolExpr);
                for it in &**whileBody { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_WHEN_A { boolExpr, whenBody, elseWhenAlgorithmBranch } => {
                self.scan_exp(boolExpr);
                for it in &**whenBody { self.scan_algorithm_item(it); }
                for (e, branch) in &**elseWhenAlgorithmBranch {
                    self.scan_exp(e);
                    for it in &**branch { self.scan_algorithm_item(it); }
                }
            }
            Absyn::Algorithm::ALG_NORETCALL { functionCall, functionArgs } => {
                self.refs.insert(cref_to_dotted(functionCall));
                self.scan_function_args(&**functionArgs);
            }
            Absyn::Algorithm::ALG_FAILURE { equ } => {
                for it in &**equ { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_TRY { body, elseBody } => {
                for it in &**body { self.scan_algorithm_item(it); }
                for it in &**elseBody { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_RETURN
            | Absyn::Algorithm::ALG_BREAK
            | Absyn::Algorithm::ALG_CONTINUE => {}
        }
    }

    fn scan_exp(&mut self, e: &Absyn::Exp) {
        use Absyn::Exp::*;
        match e {
            INTEGER { .. } | REAL { .. } | STRING { .. } | BOOL { .. } | END | BREAK => {}
            CODE { .. } => {}
            // A bare CREF in expression position may denote a function value
            // (e.g. `List.map(stringGet, xs)`). Record the dotted name; the
            // resolver later filters out anything that does not actually point
            // at a function class.
            CREF { componentRef } => {
                self.refs.insert(cref_to_dotted(componentRef));
            }
            BINARY { exp1, exp2, .. } | LBINARY { exp1, exp2, .. } | RELATION { exp1, exp2, .. } => {
                self.scan_exp(exp1); self.scan_exp(exp2);
            }
            UNARY { exp, .. } | LUNARY { exp, .. } => self.scan_exp(exp),
            IFEXP { ifExp, trueBranch, elseBranch, elseIfBranch } => {
                self.scan_exp(ifExp);
                self.scan_exp(trueBranch);
                self.scan_exp(elseBranch);
                for (c, t) in &**elseIfBranch { self.scan_exp(c); self.scan_exp(t); }
            }
            CALL { function_, functionArgs, .. } => {
                self.refs.insert(cref_to_dotted(function_));
                self.scan_function_args(&**functionArgs);
            }
            PARTEVALFUNCTION { function_, functionArgs } => {
                self.refs.insert(cref_to_dotted(function_));
                self.scan_function_args(&**functionArgs);
            }
            ARRAY { arrayExp } | LIST { exps: arrayExp } => {
                for e in &**arrayExp { self.scan_exp(e); }
            }
            MATRIX { matrix } => {
                for row in &**matrix {
                    for e in &**row { self.scan_exp(e); }
                }
            }
            RANGE { start, step, stop } => {
                self.scan_exp(start);
                if let Some(s) = step.as_deref() { self.scan_exp(s); }
                self.scan_exp(stop);
            }
            TUPLE { expressions } => {
                for e in &**expressions { self.scan_exp(e); }
            }
            AS { exp, .. } => self.scan_exp(exp),
            CONS { head, rest } => { self.scan_exp(head); self.scan_exp(rest); }
            MATCHEXP { inputExp, cases, .. } => {
                self.scan_exp(inputExp);
                for case in &**cases {
                    match &**case {
                        Absyn::Case::CASE { pattern, patternGuard, classPart, result, .. } => {
                            self.scan_exp(pattern);
                            if let Some(g) = patternGuard.as_deref() { self.scan_exp(g); }
                            self.scan_class_part(classPart);
                            self.scan_exp(result);
                        }
                        Absyn::Case::ELSE { classPart, result, .. } => {
                            self.scan_class_part(classPart);
                            self.scan_exp(result);
                        }
                    }
                }
            }
            DOT { exp, index } => { self.scan_exp(exp); self.scan_exp(index); }
            EXPRESSIONCOMMENT { exp, .. } => self.scan_exp(exp),
            SUBSCRIPTED_EXP { exp, .. } => self.scan_exp(exp),
        }
    }

    fn scan_class_part(&mut self, part: &Absyn::ClassPart) {
        if let Absyn::ClassPart::ALGORITHMS { contents } = part {
            for it in &**contents { self.scan_algorithm_item(it); }
        }
    }

    fn scan_function_args(&mut self, fa: &Absyn::FunctionArgs) {
        match fa {
            Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
                for e in &**args { self.scan_exp(e); }
                for na in &**argNames {
                    let Absyn::NamedArg { argValue, .. } = &**na;
                    self.scan_exp(argValue);
                }
            }
            Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterators, .. } => {
                self.scan_exp(exp);
                for it in &**iterators {
                    let Absyn::ForIterator { range, guardExp, .. } = &**it;
                    if let Some(r) = range.as_deref() { self.scan_exp(r); }
                    if let Some(g) = guardExp.as_deref() { self.scan_exp(g); }
                }
            }
        }
    }
}

/// Hold the scan + resolved edges together so the caller can pass them around.
pub struct UnusedReport {
    pub total_functions: usize,
    pub reachable: BTreeSet<String>,
    pub unreachable: BTreeSet<String>,
    /// Raw names that did not resolve to any function class. Aggregated to
    /// help spot reflective lookups that the static analysis cannot follow.
    pub unresolved_sample: Vec<String>,
}

pub fn analyze(hier: &InstanceHierarchy<'_>) -> UnusedReport {
    let mut functions: Vec<(String, &MM::Class, Option<String>)> = Vec::new();
    collect_functions(&hier.top_level, "", &mut functions);

    // Group FQNs that point at the same `MM::Class` (i.e. the same physical
    // declaration). `flatten_extends` copies each base function node into
    // every derived class but keeps the same `&MM::Class` reference, so the
    // hierarchy contains one FQN per `extends`-chain ancestor — e.g.
    // `BaseAvlTree.add`, `AvlTreeCRToInt.add`, `AvlSetCR.add`, … all point at
    // the *same* MetaModelica function. We collapse them to a single canonical
    // FQN so the unused report doesn't flag every derived copy of the same
    // function, only the original declaration.
    //
    // The canonical pick: prefer the FQN whose top-level package name matches
    // the basename of `c.info.fileName` (the original definition site).
    // Fall back to the lexicographically smallest FQN if no match — gives a
    // deterministic result either way.
    let mut by_ptr: BTreeMap<usize, Vec<(String, &MM::Class, Option<String>)>> = BTreeMap::new();
    for entry in &functions {
        let key = (entry.1 as *const MM::Class) as usize;
        by_ptr.entry(key).or_default().push((entry.0.clone(), entry.1, entry.2.clone()));
    }
    let mut canonical_of: BTreeMap<String, String> = BTreeMap::new();
    let mut canonical_fns: Vec<(String, &MM::Class, Option<String>)> = Vec::new();
    for group in by_ptr.values() {
        let (_, class, _) = group[0];
        let base = std::path::Path::new(class.info.fileName.as_str())
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_owned());
        // Pick canonical: first FQN whose top-level package matches the file
        // basename (the package the function was originally declared in);
        // otherwise the lexicographically smallest FQN in the group.
        let canonical = base
            .as_deref()
            .and_then(|stem| {
                group.iter()
                    .map(|(q, _, _)| q.as_str())
                    .find(|q| q.split('.').next() == Some(stem))
                    .map(|q| q.to_owned())
            })
            .unwrap_or_else(|| {
                group.iter().map(|(q, _, _)| q.clone()).min().expect("non-empty group")
            });
        for (q, _, _) in group {
            canonical_of.insert(q.clone(), canonical.clone());
        }
        let alias_base = group.iter().find_map(|(_, _, a)| a.clone());
        canonical_fns.push((canonical.clone(), class, alias_base));
    }

    let canonical_set: BTreeSet<String> =
        canonical_fns.iter().map(|(q, _, _)| q.clone()).collect();

    // Per-function reference scan, keyed by canonical FQN. Bodies are
    // identical across the duplicate FQNs (they all reference the same
    // `MM::Class`), so scanning once per canonical is enough.
    let mut refs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut alias_bases: BTreeMap<String, String> = BTreeMap::new();
    for (qname, class, alias_base) in &canonical_fns {
        let s = RefScan::scan_class(class);
        refs.insert(qname.clone(), s.refs);
        if let Some(base) = alias_base {
            alias_bases.insert(qname.clone(), base.clone());
        }
    }

    // Resolve raw names → canonical FQN edges. The scope used by
    // `resolve_call_node` is the canonical FQN — sufficient because the body
    // sources reference names relative to their original declaration site.
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut unresolved: BTreeSet<String> = BTreeSet::new();
    let resolve_to_canonical = |raw: &str, scope: &str| -> Option<String> {
        let (qname, node) = resolve_call_node(raw, &hier.top_level, scope)?;
        let NodeKind::Class(c) = &node.kind else { return None };
        if !matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) { return None; }
        canonical_of.get(&qname).cloned()
    };
    for (qname, raw_refs) in &refs {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for raw in raw_refs {
            if raw == "_" || raw == "__" || raw.is_empty() { continue; }
            match resolve_to_canonical(raw, qname) {
                Some(target) => { set.insert(target); }
                None => {
                    if unresolved.len() < 64 {
                        unresolved.insert(raw.clone());
                    }
                }
            }
        }
        if let Some(base) = alias_bases.get(qname) {
            if let Some(target) = resolve_to_canonical(base, qname) {
                set.insert(target);
            }
        }
        edges.insert(qname.clone(), set);
    }

    // BFS from Main.main (canonicalize the root too in case the entry-point
    // function has been re-exported elsewhere — unlikely but cheap).
    let root_canonical = canonical_of.get(ROOT).cloned().unwrap_or_else(|| ROOT.to_owned());
    let mut reachable: BTreeSet<String> = BTreeSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    if canonical_set.contains(&root_canonical) {
        reachable.insert(root_canonical.clone());
        queue.push_back(root_canonical);
    }
    while let Some(cur) = queue.pop_front() {
        if let Some(targets) = edges.get(&cur) {
            for t in targets {
                if reachable.insert(t.clone()) {
                    queue.push_back(t.clone());
                }
            }
        }
    }

    let unreachable: BTreeSet<String> =
        canonical_set.iter().filter(|q| !reachable.contains(q.as_str())).cloned().collect();

    UnusedReport {
        total_functions: canonical_set.len(),
        reachable,
        unreachable,
        unresolved_sample: unresolved.into_iter().collect(),
    }
}

pub fn print_report(report: &UnusedReport) {
    println!("═══════════════════════════════════════════════════════════");
    println!("  Unused-function analysis (reachability from {ROOT})");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!(
        "  Functions total:      {}",
        report.total_functions
    );
    println!("  Reachable from root:  {}", report.reachable.len());
    println!("  Unreachable:          {}", report.unreachable.len());
    println!();

    if !report.reachable.contains(ROOT) {
        println!(
            "  WARNING: root `{ROOT}` was not found in the hierarchy — no reachability \
             could be computed."
        );
        println!();
    }

    // Group unreachable by top-level package so the output stays scannable on
    // a large codebase.
    let mut by_pkg: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for q in &report.unreachable {
        let pkg = q.split('.').next().unwrap_or(q.as_str());
        by_pkg.entry(pkg).or_default().push(q.as_str());
    }

    if !report.unresolved_sample.is_empty() {
        println!(
            "── Unresolved names (sample of up to 64 — not function classes, likely builtins/locals) ─"
        );
        for n in &report.unresolved_sample {
            println!("    · {n}");
        }
        println!();
    }

    println!("── Unreachable functions, grouped by top-level package ─────");
    for (pkg, fns) in &by_pkg {
        println!("  {pkg}  ({} functions)", fns.len());
        for f in fns {
            println!("    · {f}");
        }
    }
    println!();
}
