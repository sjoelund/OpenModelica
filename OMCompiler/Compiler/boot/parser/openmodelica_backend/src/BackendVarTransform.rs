// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::BackendDAE;
use crate::BackendDAETransform;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub type ExpOpt = Option<Arc<DAE::Exp>>;

pub type CrefExpTable = Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>>>;

pub type CrefList = Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;

pub type CrefListOpt = Option<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>;

pub type CrefCrefListTable = Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Option<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>>;

pub type CrefSet = Arc<UnorderedSet::UnorderedSet<Arc<DAE::ComponentRef>>>;

/// VariableReplacements consists of a mapping between variables and expressions, the first binary tree of this type.
/// To eliminate a variable from an equation system a replacement rule varname->expression is added to this
/// datatype.
/// To be able to update these replacement rules incrementally a backward lookup mechanism is also required.
/// For instance, having a rule a->b and adding a rule b->c requires to find the first rule a->b and update it to
/// a->c. This is what the second binary tree is used for.
#[derive(Clone, Debug, PartialEq)]
pub struct VariableReplacements {
    /// src -> dst, used for replacing. src is variable, dst is expression.
    pub hashTable: CrefExpTable,
    /// dst -> list of sources. dst is a variable, sources are variables.
    pub invHashTable: CrefCrefListTable,
    /// src -> nothing, used for extend arrays and records.
    pub extendhashTable: CrefSet,
    /// this are the implicit declerate iteration variables for for and range expressions
    pub iterationVars: Arc<metamodelica::List<ArcStr>>,
    /// this is used if states are constant to replace der(state) with 0.0
    pub derConst: Option<Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>>>>,
}

pub type REPLACEMENTS = VariableReplacements;


pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

pub fn newCrefExpTable() -> CrefExpTable {
    let mut table: CrefExpTable = UnorderedMap::new(ComponentReference::hashComponentRef, ComponentReferenceBasics::crefEqual, 1);
    table
}

pub fn newCrefExpTableSized(mut size: i32) -> CrefExpTable {
    let mut table: CrefExpTable = UnorderedMap::new(ComponentReference::hashComponentRef, ComponentReferenceBasics::crefEqual, size.clone());
    table
}

pub fn newCrefCrefListTable() -> CrefCrefListTable {
    let mut table: CrefCrefListTable = UnorderedMap::new(ComponentReference::hashComponentRef, ComponentReferenceBasics::crefEqual, 1);
    table
}

pub fn newCrefCrefListTableSized(mut size: i32) -> CrefCrefListTable {
    let mut table: CrefCrefListTable = UnorderedMap::new(ComponentReference::hashComponentRef, ComponentReferenceBasics::crefEqual, size.clone());
    table
}

pub fn newCrefSet() -> CrefSet {
    let mut set: CrefSet = UnorderedSet::new(ComponentReference::hashComponentRef, ComponentReferenceBasics::crefEqual, 13);
    set
}

pub fn newCrefSetSized(mut size: i32) -> CrefSet {
    let mut set: CrefSet = UnorderedSet::new(ComponentReference::hashComponentRef, ComponentReferenceBasics::crefEqual, size.clone());
    set
}

pub fn emptyReplacements() -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements = VariableReplacements { hashTable: newCrefExpTable(), invHashTable: newCrefCrefListTable(), extendhashTable: newCrefSet(), iterationVars: metamodelica::nil(), derConst: None };
    outVariableReplacements
}

pub fn emptyReplacementsSized(mut size: i32) -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements = VariableReplacements { hashTable: newCrefExpTableSized(size.clone()), invHashTable: newCrefCrefListTableSized(size.clone()), extendhashTable: newCrefSetSized(size.clone()), iterationVars: metamodelica::nil(), derConst: None };
    outVariableReplacements
}

pub fn removeReplacement(mut repl: VariableReplacements, mut inSrc: Arc<DAE::ComponentRef>) -> Result<()> {
    let mut dst_opt: Option<Arc<DAE::Exp>> = None;
    let mut dst: Arc<DAE::Exp>;
    dst_opt = UnorderedMap::getOrDefault(inSrc.clone(), repl.hashTable.clone(), None);
    if isNone(dst_opt.clone()) {
        return Ok(());
    }
    if '__try0: {
        unwrap_break_err!(UnorderedMap::add(inSrc.clone(), None, repl.hashTable.clone()), '__try0);
        unwrap_break_err!(removeReplacementInv(repl.invHashTable.clone(), Util::getOption(dst_opt.clone())?), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-BackendVarTransform.removeReplacement failed for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inSrc.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(())
}

pub fn removeReplacements(mut iRepl: VariableReplacements, mut inSrcs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<()> {
    for mut cr in &*inSrcs.clone() {
        let mut cr = cr.clone();
        removeReplacement(iRepl.clone(), cr.clone())?;
    }
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn addReplacements(mut iRepl: VariableReplacements, mut inSrcs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inDsts: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = (::match_deref::match_deref! { match &((inSrcs.clone(), inDsts.clone(), inFuncTypeExpExpToBooleanOption.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            iRepl.clone()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: crlst }, Deref @ metamodelica::List::Cons { head: exp, tail: explst }, _) => {
            let mut repl: VariableReplacements;
            repl = addReplacement(iRepl.clone(), cr.clone(), exp.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            addReplacements(repl.clone(), crlst.clone(), explst.clone(), inFuncTypeExpExpToBooleanOption.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRepl)
}

pub fn addReplacement(mut repl: VariableReplacements, mut inSrc: Arc<DAE::ComponentRef>, mut inDst: Arc<DAE::Exp>, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = (inSrc.clone(), inDst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (src, dst) => {
                    let mut ht: CrefExpTable;
                    let mut eht: CrefSet;
                    let mut invHt: CrefCrefListTable;
                    let mut iv: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut derConst: Option<Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>>>> = None;
                    let mut src = (*src).clone();
                    let mut dst = (*dst).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(makeTransitive(repl.clone(), src.clone(), dst.clone(), inFuncTypeExpExpToBooleanOption.clone())?) {
                        (VariableReplacements { hashTable: __pa0, invHashTable: __pa1, extendhashTable: __pa2, iterationVars: __pa3, derConst: __pa4 }, __pa5, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ht = __pa0.clone();
                    invHt = __pa1.clone();
                    eht = __pa2.clone();
                    iv = __pa3.clone();
                    derConst = __pa4.clone();
                    src = __pa5.clone();
                    dst = __pa6.clone();
                    UnorderedMap::add(src.clone(), Some(dst.clone()), ht.clone())?;
                    invHt = addReplacementInv(invHt.clone(), src.clone(), dst.clone())?;
                    eht = addExtendReplacement(eht.clone(), src.clone(), None)?;
                    Ok(VariableReplacements { hashTable: ht.clone(), invHashTable: invHt.clone(), extendhashTable: eht.clone(), iterationVars: iv.clone(), derConst: derConst.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ComponentReferenceBasics::printComponentRefStr(inSrc.clone())?).clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-BackendVarTransform.addReplacement failed for ")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRepl)
}

pub fn performReplacementsEqSystem(mut inEqs: Arc<BackendDAE::EqSystem>, mut inRepl: VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqs: Arc<BackendDAE::EqSystem> = inEqs.clone();
    let mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    eqArr = inEqs.orderedEqs.clone();
    (_, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(inEqs.orderedVars.clone(), Arc::new(replaceVarTraverser), inRepl.clone())?;
    (eqArr, _) = replaceEquationsArr(eqArr.clone(), inRepl.clone(), None)?;
    assign_field!(outEqs.orderedEqs = eqArr.clone());
    Ok(outEqs)
}

fn addReplacementNoTransitive(mut repl: VariableReplacements, mut inSrc: Arc<DAE::ComponentRef>, mut inDst: Arc<DAE::Exp>) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = (repl.clone(), inSrc.clone(), inDst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (VariableReplacements { hashTable: ht, .. }, src, _) => {
                    if !((isSome(UnorderedMap::getOrDefault(src.clone(), ht.clone(), None)))) { bail!("guard") }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (VariableReplacements { hashTable: ht, invHashTable: invHt, extendhashTable: eht, iterationVars: iv, derConst }, src, dst) => {
                    let mut invHt = (*invHt).clone();
                    let mut eht = (*eht).clone();
                    UnorderedMap::add(src.clone(), Some(dst.clone()), ht.clone())?;
                    invHt = addReplacementInv(invHt.clone(), src.clone(), dst.clone())?;
                    eht = addExtendReplacement(eht.clone(), src.clone(), None)?;
                    Ok(VariableReplacements { hashTable: ht.clone(), invHashTable: invHt.clone(), extendhashTable: eht.clone(), iterationVars: iv.clone(), derConst: derConst.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-add_replacement failed for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inSrc.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inDst.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRepl)
}

fn removeReplacementInv(mut invHt: CrefCrefListTable, mut dst: Arc<DAE::Exp>) -> Result<()> {
    for mut d in &*Expression::extractCrefsFromExp(dst.clone())? {
        let mut d = d.clone();
        UnorderedMap::tryUpdate(d.clone(), None, invHt.clone())?;
    }
    Ok(())
}

fn addReplacementInv(mut invHt: CrefCrefListTable, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>) -> Result<CrefCrefListTable> {
    let mut invHt: CrefCrefListTable = invHt;
    for mut d in &*Expression::extractCrefsFromExp(dst.clone())? {
        let mut d = d.clone();
        invHt = addReplacementInv2(invHt.clone(), d.clone(), src.clone())?;
    }
    Ok(invHt)
}

fn addReplacementInv2(mut invHt: CrefCrefListTable, mut dst: Arc<DAE::ComponentRef>, mut src: Arc<DAE::ComponentRef>) -> Result<CrefCrefListTable> {
    let mut invHt: CrefCrefListTable = invHt;
    let mut srcs_opt: Option<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> = None;
    let mut srcs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    srcs_opt = UnorderedMap::getOrDefault(dst.clone(), invHt.clone(), None);
    if isSome(srcs_opt.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(srcs_opt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        srcs = __pa0.clone();
        srcs = cons(src.clone(), srcs.clone());
        UnorderedMap::add(dst.clone(), Some(srcs.clone()), invHt.clone())?;
    } else {
        UnorderedMap::add(dst.clone(), Some(list![src.clone()]), invHt.clone())?;
    }
    Ok(invHt)
}

fn makeTransitive(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(VariableReplacements, Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outDst: Arc<DAE::Exp>;
    (outRepl, outSrc, outDst) = (::match_deref::match_deref! { match &((repl.clone(), src.clone(), dst.clone(), inFuncTypeExpExpToBooleanOption.clone())) {
        (_, _, _, _) => {
            let mut repl_1: VariableReplacements;
            let mut repl_2: VariableReplacements;
            let mut src_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut src_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut dst_1: Arc<DAE::Exp>;
            let mut dst_2: Arc<DAE::Exp>;
            let mut dst_3: Arc<DAE::Exp>;
            (repl_1, src_1, dst_1) = makeTransitive1(repl.clone(), src.clone(), dst.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (repl_2, src_2, dst_2) = makeTransitive2(repl_1.clone(), src_1.clone(), dst_1.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (dst_3, _) = ExpressionSimplify::simplify1(dst_2.clone())?;
            (repl_2.clone(), src_2.clone(), dst_3.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outRepl, outSrc, outDst))
}

fn makeTransitive1(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(VariableReplacements, Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outDst: Arc<DAE::Exp>;
    (outRepl, outSrc, outDst) = 'mc: {
        let __mc_input = repl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let VariableReplacements { invHashTable: ref invHt, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut repl_1: VariableReplacements;
            let mut singleRepl: VariableReplacements;
            let __pa0 = ::match_deref::match_deref! { match &(UnorderedMap::getOrFail(src.clone(), invHt.clone())) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            lst = __pa0.clone();
            singleRepl = addReplacementNoTransitive(emptyReplacementsSized(53), src.clone(), dst.clone())?;
            repl_1 = makeTransitive12(lst.clone(), repl.clone(), singleRepl.clone(), inFuncTypeExpExpToBooleanOption.clone(), HashSet::emptyHashSet())?;
            Ok((repl_1.clone(), src.clone(), dst.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((repl.clone(), src.clone(), dst.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRepl, outSrc, outDst))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeTransitive12(mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut repl: VariableReplacements, mut singleRepl: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut inSet: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = lst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: cr, tail: crs } => {
                    let mut crDst: Arc<DAE::Exp>;
                    let mut repl1: VariableReplacements;
                    let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let false = (BaseHashSet::has(cr.clone(), inSet.clone())?) else { bail!("pattern mismatch") };
                    set = BaseHashSet::add(cr.clone(), inSet.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(UnorderedMap::getOrFail(cr.clone(), repl.hashTable.clone())) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    crDst = __pa0.clone();
                    (crDst, _) = replaceExp(crDst.clone(), singleRepl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    repl1 = addReplacementNoTransitive(repl.clone(), cr.clone(), crDst.clone())?;
                    Ok(makeTransitive12(crs.clone(), repl1.clone(), singleRepl.clone(), inFuncTypeExpExpToBooleanOption.clone(), set.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: crs } => {
                    Ok(makeTransitive12(crs.clone(), repl.clone(), singleRepl.clone(), inFuncTypeExpExpToBooleanOption.clone(), inSet.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRepl)
}

fn makeTransitive2(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>, mut dst: Arc<DAE::Exp>, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(VariableReplacements, Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outDst: Arc<DAE::Exp>;
    (outRepl, outSrc, outDst) = 'mc: {
        let __mc_input = inFuncTypeExpExpToBooleanOption.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut dst_1: Arc<DAE::Exp>;
            (dst_1, _) = replaceExp(dst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            Ok((repl.clone(), src.clone(), dst_1.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((repl.clone(), src.clone(), dst.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRepl, outSrc, outDst))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addExtendReplacement(mut extendrepl: CrefSet, mut cr: Arc<DAE::ComponentRef>, mut preCr: Option<Arc<DAE::ComponentRef>>) -> Result<CrefSet> {
    let mut outExtendrepl: CrefSet;
    outExtendrepl = 'mc: {
        let __mc_input = (cr.clone(), preCr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, ident, .. }, None) => {
                    let mut precr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    UnorderedSet::addUnique(precr.clone(), extendrepl.clone())?;
                    Ok(extendrepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, ident, .. }, Some(pcr)) => {
                    let mut precr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut precr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    precr1 = ComponentReference::joinCrefs(pcr.clone(), precr.clone())?;
                    UnorderedSet::addUnique(precr1.clone(), extendrepl.clone())?;
                    Ok(extendrepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty @ Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. }, ident, .. }, None) => {
                    let mut erepl: CrefSet;
                    let mut precr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    precr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    UnorderedSet::addUnique(precr.clone(), extendrepl.clone())?;
                    crefs = List::map(varLst.clone(), Arc::new(ComponentReference::creffromVar));
                    erepl = List::fold1r(crefs.clone(), Arc::new(addExtendReplacement), Some(precr.clone()), extendrepl.clone());
                    Ok(erepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty @ Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. }, ident, .. }, Some(pcr)) => {
                    let mut erepl: CrefSet;
                    let mut precr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let _ = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    precr1 = ComponentReference::joinCrefs(pcr.clone(), cr.clone())?;
                    UnorderedSet::addUnique(precr1.clone(), extendrepl.clone())?;
                    crefs = List::map(varLst.clone(), Arc::new(ComponentReference::creffromVar));
                    erepl = List::fold1r(crefs.clone(), Arc::new(addExtendReplacement), Some(precr1.clone()), extendrepl.clone());
                    Ok(erepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: _, tail: _ }, identType: ty, ident }, None) => {
                    let mut precr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    UnorderedSet::addUnique(precr.clone(), extendrepl.clone())?;
                    Ok(extendrepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: _, tail: _ }, identType: ty, ident }, Some(pcr)) => {
                    let mut precr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut precr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    precr1 = ComponentReference::joinCrefs(pcr.clone(), precr.clone())?;
                    UnorderedSet::addUnique(precr1.clone(), extendrepl.clone())?;
                    Ok(extendrepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _) => {
                    Ok(extendrepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: subcr, subscriptLst, identType: ty, ident }, None) => {
                    let mut erepl: CrefSet;
                    let mut precr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut precrn: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    UnorderedSet::addUnique(precr.clone(), extendrepl.clone())?;
                    precrn = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
                    erepl = addExtendReplacement(extendrepl.clone(), subcr.clone(), Some(precrn.clone()))?;
                    Ok(erepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: subcr, subscriptLst, identType: ty, ident }, Some(pcr)) => {
                    let mut erepl: CrefSet;
                    let mut precr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut precr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut precrn: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut precrn1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
                    precr1 = ComponentReference::joinCrefs(pcr.clone(), precr.clone())?;
                    UnorderedSet::addUnique(precr1.clone(), extendrepl.clone())?;
                    precrn = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
                    precrn1 = ComponentReference::joinCrefs(pcr.clone(), precrn.clone())?;
                    erepl = addExtendReplacement(extendrepl.clone(), subcr.clone(), Some(precrn1.clone()))?;
                    Ok(erepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: subcr, subscriptLst, identType: ty, ident }, None) => {
                    let mut erepl: CrefSet;
                    let mut precrn: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precrn = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
                    erepl = addExtendReplacement(extendrepl.clone(), subcr.clone(), Some(precrn.clone()))?;
                    Ok(erepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: subcr, subscriptLst, identType: ty, ident }, Some(pcr)) => {
                    let mut erepl: CrefSet;
                    let mut precrn: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut precrn1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    precrn = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
                    precrn1 = ComponentReference::joinCrefs(pcr.clone(), precrn.clone())?;
                    erepl = addExtendReplacement(extendrepl.clone(), subcr.clone(), Some(precrn1.clone()))?;
                    Ok(erepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendVarTransform.addExtendReplacement failed for ")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(extendrepl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExtendrepl)
}

fn addIterationVar(mut repl: VariableReplacements, mut inVar: ArcStr) -> VariableReplacements {
    let mut repl: VariableReplacements = repl;
    repl.iterationVars = cons(inVar.clone(), repl.iterationVars.clone());
    repl
}

fn removeIterationVar(mut repl: VariableReplacements, mut inVar: ArcStr) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements = repl;
    repl.iterationVars = List::deleteMemberOnTrue((inVar.clone()).clone(), repl.iterationVars.clone(), Arc::new(fnptr!(stringEq, ArcStr, ArcStr)))?.0;
    Ok(repl)
}

fn isIterationVar(mut repl: VariableReplacements, mut inVar: ArcStr) -> Result<bool> {
    let mut is: bool = false;
    let mut iv: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let (VariableReplacements { iterationVars: __pa0, .. }, _) = ((repl.clone(), inVar.clone())) else { bail!("pattern mismatch") };
    iv = __pa0.clone();
    is = listMember((inVar.clone()).clone(), iv.clone());
    Ok(is)
}

pub fn addDerConstRepl(mut inComponentRef: Arc<DAE::ComponentRef>, mut inExp: Arc<DAE::Exp>, mut repl: VariableReplacements) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements = repl;
    let mut derConst: CrefExpTable;
    if isSome(repl.derConst.clone()) {
        UnorderedMap::add(inComponentRef.clone(), Some(inExp.clone()), Util::getOption(repl.derConst.clone())?)?;
    } else {
        derConst = newCrefExpTable();
        UnorderedMap::add(inComponentRef.clone(), Some(inExp.clone()), derConst.clone())?;
        repl.derConst = Some(derConst.clone());
    }
    Ok(repl)
}

pub fn getReplacement(mut inVariableReplacements: VariableReplacements, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(UnorderedMap::getOrFail(inComponentRef.clone(), inVariableReplacements.hashTable.clone())) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outExp = __pa0.clone();
    Ok(outExp)
}

pub fn hasReplacement(mut repl: VariableReplacements, mut inComponentRef: Arc<DAE::ComponentRef>) -> bool {
    let mut bOut: bool = false;
    bOut = isSome(UnorderedMap::getOrDefault(inComponentRef.clone(), repl.hashTable.clone(), None));
    bOut
}

pub fn hasNoReplacement(mut inComponentRef: Arc<DAE::ComponentRef>, mut repl: VariableReplacements) -> bool {
    let mut bOut: bool = false;
    bOut = isNone(UnorderedMap::getOrDefault(inComponentRef.clone(), repl.hashTable.clone(), None));
    bOut
}

pub fn getAllReplacements(mut repl: VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dsts: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (crefs, dsts) = getCrefExpTableEntries(repl.hashTable.clone())?;
    Ok((crefs, dsts))
}

fn getCrefExpTableEntries(mut table: CrefExpTable) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dsts: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut opt_dsts: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>> = metamodelica::nil();
    crefs = UnorderedMap::keyList(table.clone());
    opt_dsts = UnorderedMap::valueList(table.clone());
    (opt_dsts, crefs) = List::filterOnTrueSync(opt_dsts.clone(), Arc::new(fnptr!(isSome, _)), crefs.clone())?;
    dsts = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut d in (opt_dsts.clone()).into_iter().cloned() {
            let __x = Util::getOption(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((crefs, dsts))
}

pub fn hasExtendReplacement(mut repl: VariableReplacements, mut src: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut exists: bool = false;
    exists = UnorderedSet::contains(ComponentReferenceBasics::crefStripLastSubs(src.clone())?, repl.extendhashTable.clone())?;
    Ok(exists)
}

fn avoidDoubleHashLookup(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_UNKNOWN } => {
                    Ok(Expression::makeCrefExp(cr.clone(), inType.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn isReplacementEmpty(mut repl: VariableReplacements) -> bool {
    let mut empty: bool = UnorderedMap::none(repl.hashTable.clone(), Arc::new(fnptr!(isSome, _))) && isNone(repl.derConst.clone());
    empty
}

/* ********************************************************/
/* replace Expression with condition function */
/* ********************************************************/
pub fn replaceExp(mut inExp: Arc<DAE::Exp>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut replacementPerformed: bool = false;
    (outExp, replacementPerformed) = 'mc: {
        let __mc_input = (inExp.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident, .. }, .. }, repl, _) => {
                    if !((isIterationVar(repl.clone(), (ident.clone()).clone())?)) { bail!("guard") }
                    Ok((e.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: t, componentRef: cr }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    (cr, _) = replaceCrefSubs(cr.clone(), repl.clone(), cond.clone())?;
                    e1 = getReplacement(repl.clone(), cr.clone())?;
                    e2 = avoidDoubleHashLookup(e1.clone(), t.clone())?;
                    Ok((e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: t, componentRef: cr }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    (_, dims) = TypesDump::flattenArrayType(t.clone());
                    let true = (List::none({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut dim in (dims.clone()).into_iter().cloned() {
                    let __x = Types::dimNotFixed(dim.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, Arc::new(fnptr!(Util::id, _)))) else { bail!("pattern mismatch") };
                    (cr, _) = replaceCrefSubs(cr.clone(), repl.clone(), cond.clone())?;
                    let true = (hasExtendReplacement(repl.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    (e3, _) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    Ok((e3.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: t, componentRef: cr }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut cr = (*cr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceCrefSubs(cr.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: t.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (e2_1, c2) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (e2_1, c2) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { exp: e1, operator: op }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { optionExpisASUB: isExpisASUB, index: index_, exp2: e2, operator: op, exp1: e1 }, repl, cond) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (e2_1, c2) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone(), index: index_.clone(), optionExpisASUB: isExpisASUB.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3_1: Arc<DAE::Exp>;
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    let mut c3: bool = false;
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (e2_1, c2) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    (e3_1, c3) = replaceExp(e3.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone() || c3.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, VariableReplacements { derConst: Some(derConst), .. }, cond) => {
                    let mut e: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(UnorderedMap::getOrFail(cr.clone(), derConst.clone())) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    (e, _) = replaceExp(e.clone(), inVariableReplacements.clone(), cond.clone())?;
                    Ok((e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { attr, expLst: expl, path }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl = (*expl).clone();
                    let mut path = (*path).clone();
                    cr = ComponentReference::toExpCref(AbsynUtil::pathToCref(path.clone())?)?;
                    if hasReplacement(repl.clone(), cr.clone()) {
                        e1_1 = getReplacement(repl.clone(), cr.clone())?;
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(e1_1.clone()) {
                            Deref @ DAE::Exp::PARTEVALFUNCTION { expList: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        expl_1 = __pa0.clone();
                        path = __pa1.clone();
                        expl = listAppend(expl_1.clone(), expl.clone());
                    }
                    let __pa2 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), repl.clone(), cond.clone())?) {
                        (__pa2, true) => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl_1 = __pa2.clone();
                    Ok((Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expl_1.clone(), attr: attr.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { path, exps: expl, comp: fields, ty: t }, repl, cond) => {
                    let mut expl = (*expl).clone();
                    let mut repl = (*repl).clone();
                    repl = addConstantRecordReplacements(t.clone(), expl.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::RECORD { path: path.clone(), exps: expl.clone(), comp: fields.clone(), ty: t.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::RATIONAL_CLOCK { resolution, intervalCounter: e } }, repl, cond) => {
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    let mut c3: bool = false;
                    let mut resolution = (*resolution).clone();
                    let mut e = (*e).clone();
                    (e, c1) = replaceExp(e.clone(), repl.clone(), cond.clone())?;
                    (resolution, c2) = replaceExp(resolution.clone(), repl.clone(), cond.clone())?;
                    c3 = c1.clone() || c2.clone();
                    Ok((if (c3.clone()) {Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e.clone(), resolution: resolution.clone() }) })} else {inExp.clone()}, c3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::REAL_CLOCK { interval: e } }, repl, cond) => {
                    let mut c1: bool = false;
                    let mut e = (*e).clone();
                    (e, c1) = replaceExp(e.clone(), repl.clone(), cond.clone())?;
                    Ok((if (c1.clone()) {Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::REAL_CLOCK { interval: e.clone() }) })} else {inExp.clone()}, c1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { startInterval, condition: e } }, repl, cond) => {
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    let mut c3: bool = false;
                    let mut startInterval = (*startInterval).clone();
                    let mut e = (*e).clone();
                    (e, c1) = replaceExp(e.clone(), repl.clone(), cond.clone())?;
                    (startInterval, c2) = replaceExp(startInterval.clone(), repl.clone(), cond.clone())?;
                    c3 = c1.clone() || c2.clone();
                    Ok((if (c3.clone()) {Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: e.clone(), startInterval: startInterval.clone() }) })} else {inExp.clone()}, c3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { solverMethod, c: e } }, repl, cond) => {
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    let mut c3: bool = false;
                    let mut solverMethod = (*solverMethod).clone();
                    let mut e = (*e).clone();
                    (e, c1) = replaceExp(e.clone(), repl.clone(), cond.clone())?;
                    (solverMethod, c2) = replaceExp(solverMethod.clone(), repl.clone(), cond.clone())?;
                    c3 = c1.clone() || c2.clone();
                    Ok((if (c3.clone()) {Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: e.clone(), solverMethod: solverMethod.clone() }) })} else {inExp.clone()}, c3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::PARTEVALFUNCTION { path, expList: expl, ty: tp, origType: t }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::PARTEVALFUNCTION { path: path.clone(), expList: expl_1.clone(), ty: tp.clone(), origType: t.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: expl, scalar: c, ty: tp }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: c.clone(), array: expl_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: bexpl, integer: b, ty: t }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut bexpl_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpMatrix(bexpl.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    bexpl_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::MATRIX { ty: t.clone(), integer: b.clone(), matrix: bexpl_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { stop: e2, step: None, start: e1, ty: tp }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (e2_1, c2) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: None, stop: e2_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { stop: e2, step: Some(e3), start: e1, ty: tp }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3_1: Arc<DAE::Exp>;
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    let mut c3: bool = false;
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (e2_1, c2) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    (e3_1, c3) = replaceExp(e3.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone() || c3.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: Some(e3_1.clone()), stop: e2_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::TUPLE { PR: expl }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::TUPLE { PR: expl_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CAST { exp: e1, ty: tp }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ASUB { sub: subs, exp: e1 }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    expl = List::map(subs.clone(), Arc::new(Expression::getSubscriptExp));
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (expl, c2) = replaceExpList(expl.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone()) else { bail!("pattern mismatch") };
                    Ok((Expression::makeASUB(e1_1.clone(), expl.clone())?, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TSUB { ty: tp, ix: i, exp: e1 }, repl, cond) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let true = (replaceExpCond(cond.clone(), e1.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::TSUB { exp: e1_1.clone(), ix: i.clone(), ty: tp.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::SIZE { sz: Some(e2), exp: e1 }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut c1: bool = false;
                    let mut c2: bool = false;
                    (e1_1, c1) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    (e2_1, c2) = replaceExp(e2.clone(), repl.clone(), cond.clone())?;
                    let true = (c1.clone() || c2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::SIZE { exp: e1_1.clone(), sz: Some(e2_1.clone()) }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CODE { ty: tp, code: a }, _, _) => {
                    println!("{}", (literal!("replace_exp on CODE not impl.\n")).clone());
                    Ok((Arc::new(DAE::Exp::CODE { code: a.clone(), ty: tp.clone() }), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::REDUCTION { iterators: iters, expr: e1, reductionInfo }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut iters = (*iters).clone();
                    (e1_1, _) = replaceExp(e1.clone(), repl.clone(), cond.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpIters(iters.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    iters = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1_1.clone(), iterators: iters.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BOX { exp: e1 }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::BOX { exp: e1_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNBOX { exp: e1, ty: tp }, repl, cond) => {
                    if !((replaceExpCond(cond.clone(), e.clone()))) { bail!("guard") }
                    let mut e1_1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), cond.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Exp::UNBOX { exp: e1_1.clone(), ty: tp.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, replacementPerformed))
}

pub fn addConstantRecordReplacements(mut ty: Arc<DAE::Type>, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut repl: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements = repl;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    repl = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            let mut bind: Arc<DAE::Exp>;
            for mut var in &*var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone() {
                let mut var = var.clone();
                if DAEUtil::isBound(var.binding.clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(DAEUtil::bindingExp(var.binding.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    bind = __pa0.clone();
                    cref = getRecordElement((var.name.clone()).clone(), expl.clone())?;
                    if Expression::isConst(bind.clone())? && !(ComponentReference::isWild(cref.clone())) {
                        repl = addReplacement(repl.clone(), cref.clone(), bind.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    }
                }
            }
            repl.clone()
        },
        _ => {
            repl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(repl)
}

pub fn getRecordElement(mut name: ArcStr, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD);
    let mut cref_name: ArcStr = arcstr::literal!("");
    for mut e in &*expl.clone() {
        let mut e = e.clone();
        let _ = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CREF { .. } if (ComponentReferenceBasics::crefLastIdent(var_field!((*e).componentRef, DAE::Exp::CREF).clone())? == name.clone()) => {
            cref = var_field!((*e).componentRef, DAE::Exp::CREF).clone();
            return Ok(cref);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(cref)
}

pub fn replaceCref(mut crefIn: Arc<DAE::ComponentRef>, mut replIn: VariableReplacements) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut expOut: Arc<DAE::Exp>;
    let mut changedOut: bool = false;
    (expOut, changedOut) = (match replIn.clone() {
        _ if (hasReplacement(replIn.clone(), crefIn.clone())) => {
            expOut = getReplacement(replIn.clone(), crefIn.clone())?;
            (expOut.clone(), true)
        },
        _ => {
            expOut = Arc::new(DAE::Exp::CREF { componentRef: crefIn.clone(), ty: ComponentReference::crefType(crefIn.clone())? });
            (expOut.clone(), false)
        },
    });
    Ok((expOut, changedOut))
}

fn replaceCrefSubs(mut inCref: Arc<DAE::ComponentRef>, mut repl: VariableReplacements, mut cond: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<DAE::ComponentRef>, bool)> {
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut replacementPerformed: bool = false;
    (outCr, replacementPerformed) = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, subscriptLst: subs, identType: ty, ident: name } => {
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut c1: bool = false;
            let mut c2: bool = false;
            let mut cr = (*cr).clone();
            let mut subs = (*subs).clone();
            (subs_1, c1) = replaceCrefSubs2(subs.clone(), repl.clone(), cond.clone())?;
            (cr_1, c2) = replaceCrefSubs(cr.clone(), repl.clone(), cond.clone())?;
            subs = if (c1.clone()) {subs_1.clone()} else {subs.clone()};
            cr = if (c2.clone()) {cr_1.clone()} else {cr.clone()};
            cr = if (c1.clone() || c2.clone()) {Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: cr.clone() })} else {inCref.clone()};
            (cr.clone(), c1.clone() || c2.clone())
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, identType: ty, ident: name } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut c1: bool = false;
            let mut subs = (*subs).clone();
            (subs, c1) = replaceCrefSubs2(subs.clone(), repl.clone(), cond.clone())?;
            cr = if (c1.clone()) {Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone() })} else {inCref.clone()};
            (cr.clone(), c1.clone())
        },
        _ => {
            (inCref.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCr, replacementPerformed))
}

fn replaceCrefSubs2(mut isubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut repl: VariableReplacements, mut cond: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<metamodelica::List<Arc<DAE::Subscript>>>, bool)> {
    let mut outSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    outSubs = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut sub in (isubs.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ DAE::Subscript::WHOLEDIM => {
            sub.clone()
        },
        Deref @ DAE::Subscript::SLICE { exp } => {
            let mut c1: bool = false;
            let mut exp = (*exp).clone();
            (exp, c1) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
            replacementPerformed = replacementPerformed.clone() || c1.clone();
            if (c1.clone()) {Arc::new(DAE::Subscript::SLICE { exp: exp.clone() })} else {sub.clone()}
        },
        Deref @ DAE::Subscript::INDEX { exp } => {
            let mut c1: bool = false;
            let mut exp = (*exp).clone();
            (exp, c1) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
            replacementPerformed = replacementPerformed.clone() || c1.clone();
            if (c1.clone()) {Arc::new(DAE::Subscript::INDEX { exp: exp.clone() })} else {sub.clone()}
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp } => {
            let mut c1: bool = false;
            let mut exp = (*exp).clone();
            (exp, c1) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
            replacementPerformed = replacementPerformed.clone() || c1.clone();
            if (c1.clone()) {Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: exp.clone() })} else {sub.clone()}
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outSubs, replacementPerformed))
}

pub fn replaceExpList(mut iexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut repl: VariableReplacements, mut cond: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, bool)> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    let mut exp_: Arc<DAE::Exp>;
    let mut c: bool = false;
    outExpl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut exp in (iexpl.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(exp.clone()) {
        _ => {
            (exp_, c) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
            if c.clone() {
                replacementPerformed = true;
            } else {
                exp_ = exp.clone();
            }
            exp_.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outExpl, replacementPerformed))
}

pub fn replaceExpList1(mut iexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut repl: VariableReplacements, mut cond: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<bool>>)> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut replacementPerformed: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut acc1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut acc2: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut c: bool = false;
    for mut exp in &*iexpl.clone() {
        let mut exp = exp.clone();
        (exp, c) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
        acc2 = cons(c.clone(), acc2.clone());
        acc1 = cons(exp.clone(), acc1.clone());
    }
    outExpl = acc1.clone().reverse();
    replacementPerformed = acc2.clone().reverse();
    Ok((outExpl, replacementPerformed))
}

fn replaceExpIters(mut inIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut repl: VariableReplacements, mut cond: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, bool)> {
    let mut outIter: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    let mut it: Arc<DAE::ReductionIterator>;
    outIter = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
        for mut iter in (inIters.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ DAE::ReductionIterator { id, exp, guardExp: None, ty } => {
            let mut b1: bool = false;
            let mut exp = (*exp).clone();
            (exp, b1) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
            if b1.clone() {
                it = Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp.clone(), guardExp: None, ty: ty.clone() });
                replacementPerformed = true;
            } else {
                it = iter.clone();
            }
            it.clone()
        },
        Deref @ DAE::ReductionIterator { id, exp, guardExp: Some(gexp), ty } => {
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut exp = (*exp).clone();
            let mut gexp = (*gexp).clone();
            (exp, b1) = replaceExp(exp.clone(), repl.clone(), cond.clone())?;
            (gexp, b2) = replaceExp(gexp.clone(), repl.clone(), cond.clone())?;
            if b1.clone() || b2.clone() {
                it = Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp.clone(), guardExp: Some(gexp.clone()), ty: ty.clone() });
                replacementPerformed = true;
            } else {
                it = iter.clone();
            }
            it.clone()
        },
        _ => {
            iter.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outIter, replacementPerformed))
}

fn replaceExpCond(mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inFuncTypeExpExpToBooleanOption.clone(), inExp.clone())) {
        (Some(cond), e) => {
            let mut res: bool = false;
            res = cond(e.clone()).unwrap();
            res.clone()
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn replaceExpMatrix(mut inTplExpExpBooleanLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, bool)> {
    let mut outTplExpExpBooleanLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    let mut exp_: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut c: bool = false;
    outTplExpExpBooleanLstLst = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut exp in (inTplExpExpBooleanLstLst.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(exp.clone()) {
        _ => {
            (exp_, c) = replaceExpList(exp.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            if c.clone() {
                replacementPerformed = true;
            } else {
                exp_ = exp.clone();
            }
            exp_.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outTplExpExpBooleanLstLst, replacementPerformed))
}

/* ********************************************************/
/* condition function for replace Expression  */
/* ********************************************************/
pub fn skipPreOperator(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, .. } if (idn.clone() == literal!("pre") || idn.clone() == literal!("previous")) => {
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn skipPreChangeEdgeOperator(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. } if (idn.clone() == literal!("pre") || idn.clone() == literal!("previous") || idn.clone() == literal!("change") || idn.clone() == literal!("edge")) => {
            selfGeneratedVar(cr.clone())
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, .. } if (idn.clone() == literal!("pre") || idn.clone() == literal!("previous") || idn.clone() == literal!("change") || idn.clone() == literal!("edge")) => {
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn selfGeneratedVar(mut inCref: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, .. } if (idn.clone() == literal!("$ZERO") || idn.clone() == literal!("$_DER") || idn.clone() == literal!("$pDER")) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

/* ********************************************************/
/* replace Equations  */
/* ********************************************************/
pub fn replaceEquationsArr(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut repl: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut replacementPerformed: bool = false;
    (outEqns, replacementPerformed) = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut replacementPerformed: bool = replacementPerformed.clone();
            let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let false = (isReplacementEmpty(repl.clone())) else { bail!("pattern mismatch") };
            (_, _, eqns, replacementPerformed) = BackendEquation::traverseEquationArray(inEqns.clone(), Arc::new(replaceEquationTraverser), (repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false))?;
            outEqns = if (replacementPerformed.clone()) {BackendEquation::listEquation(eqns.clone())?} else {inEqns.clone()};
            Ok((outEqns.clone(), replacementPerformed.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inEqns.clone(), false))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqns, replacementPerformed))
}

fn replaceEquationTraverser(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (VariableReplacements, Option<FuncTypeExp_ExpToBoolean>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)) -> Result<(Arc<BackendDAE::Equation>, (VariableReplacements, Option<FuncTypeExp_ExpToBoolean>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool))> {
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (VariableReplacements, Option<FuncTypeExp_ExpToBoolean>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool);
    let mut repl: VariableReplacements;
    let mut optfunc: Option<FuncTypeExp_ExpToBoolean> = None;
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut b: bool = false;
    e = inEq.clone();
    (repl, optfunc, eqns, b) = inTpl.clone();
    (eqns, b) = replaceEquation(e.clone(), repl.clone(), optfunc.clone(), eqns.clone(), b.clone())?;
    outTpl = (repl.clone(), optfunc.clone(), eqns.clone(), b.clone());
    Ok((e, outTpl))
}

pub fn replaceEquations(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut repl: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    if isReplacementEmpty(repl.clone()) {
        outEqns = inEqns.clone();
        replacementPerformed = false;
    } else {
        (outEqns, replacementPerformed) = replaceEquations2(inEqns.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
        if replacementPerformed.clone() && false {
            (outEqns, _) = BackendDAETransform::traverseBackendDAEExpsEqnLstWithSymbolicOperation(outEqns.clone(), Arc::new(BackendDAETransform::collapseArrayCrefExp), 0, metamodelica::nil())?;
        }
    }
    Ok((outEqns, replacementPerformed))
}

fn replaceEquations2(mut inBackendDAEEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut inAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iReplacementPerformed: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outBackendDAEEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    (outBackendDAEEquationLst, replacementPerformed) = (::match_deref::match_deref! { match &((inBackendDAEEquationLst.clone(), inVariableReplacements.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (inAcc.clone().reverse(), iReplacementPerformed.clone())
        },
        (Deref @ metamodelica::List::Cons { head: a, tail: es }, _) => {
            let mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut es = (*es).clone();
            (acc, b) = replaceEquation(a.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc.clone(), iReplacementPerformed.clone())?;
            (es, b) = replaceEquations2(es.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone(), acc.clone(), b.clone())?;
            (es.clone(), b.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBackendDAEEquationLst, replacementPerformed))
}

fn replaceEquation(mut inBackendDAEEquation: Arc<BackendDAE::Equation>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut inAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iReplacementPerformed: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outBackendDAEEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    (outBackendDAEEquationLst, replacementPerformed) = 'mc: {
        let __mc_input = (inBackendDAEEquation.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc.clone(), iReplacementPerformed.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { recordSize, attr: eqAttr, source, right: e2, left: e1, dimSize }, repl, _, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e2_2: Arc<DAE::Exp>;
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut source = (*source).clone();
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    e2_2 = __pa1.clone();
                    source = __pa2.clone();
                    Ok((cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_2.clone(), right: e2_2.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), inAcc.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: eqAttr, source, right: e2, left: e1, size }, repl, _, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e2_2: Arc<DAE::Exp>;
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut source = (*source).clone();
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    e2_2 = __pa1.clone();
                    source = __pa2.clone();
                    Ok((cons(Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_2.clone(), right: e2_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: e2, exp: e1 }, repl, _, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e2_2: Arc<DAE::Exp>;
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut source = (*source).clone();
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_2 = __pa0.clone();
                    e2_2 = __pa1.clone();
                    source = __pa2.clone();
                    Ok((cons(Arc::new(BackendDAE::Equation::EQUATION { exp: e1_2.clone(), scalar: e2_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand: crefExpand, source, alg: Deref @ DAE::Algorithm { statementLst: stmts }, size }, repl, _, _, _) => {
                    let mut hasArrayCref: bool = false;
                    let mut alg: Arc<DAE::Algorithm>;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut crefExpand = (*crefExpand).clone();
                    crefs = Expression::getLhsCrefsFromStatements(stmts.clone());
                    hasArrayCref = List::any(crefs.clone(), Arc::new(fnptr!(ComponentReference::isArrayElement, Arc<DAE::ComponentRef>)));
                    crefExpand = if (hasArrayCref.clone()) {crefExpand.clone()} else {openmodelica_frontend_types::DAE::Expand::NOT_EXPAND};
                    let __pa0 = ::match_deref::match_deref! { match &(replaceStatementLst(stmts.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    stmts1 = __pa0.clone();
                    alg = Arc::new(DAE::Algorithm { statementLst: stmts1.clone() });
                    eqns = if (!(stmts1.clone().is_empty())) {cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inAcc.clone())} else {inAcc.clone()};
                    Ok((eqns.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: eqAttr, source, exp: e, componentRef: cr }, repl, _, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut source = (*source).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    (e_2, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e.clone(), e_2.clone())?;
                    Ok((cons(Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr.clone(), exp: e_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: eqAttr, source, exp: e }, repl, _, _, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut source = (*source).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    (e_2, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e.clone(), e_2.clone())?;
                    Ok((cons(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: whenEqn, source, attr: eqAttr }, repl, _, _, _) => {
                    let mut whenEqn1: Arc<BackendDAE::WhenEquation>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(replaceWhenEquation(whenEqn.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), source.clone())?) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    whenEqn1 = __pa0.clone();
                    source = __pa1.clone();
                    Ok((cons(Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqn1.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::IF_EQUATION { attr: eqAttr, source, eqnsfalse: eqns, eqnstrue: eqnslst, conditions: expl }, repl, _, _, _) => {
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut b3: bool = false;
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut source = (*source).clone();
                    let mut eqns = (*eqns).clone();
                    let mut eqnslst = (*eqnslst).clone();
                    (expl1, blst) = replaceExpList1(expl.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    b1 = List::any(blst.clone(), Arc::new(fnptr!(Util::id, _)));
                    source = ElementSource::addSymbolicTransformationSubstitutionLst(blst.clone(), source.clone(), expl.clone(), expl1.clone())?;
                    (expl2, blst) = ExpressionSimplify::condsimplifyList1(blst.clone(), expl1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplifyLst(blst.clone(), source.clone(), expl1.clone(), expl2.clone())?;
                    (eqnslst, b2) = List::map3Fold(eqnslst.clone(), Arc::new(replaceEquations2), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false);
                    (eqns, b3) = replaceEquations2(eqns.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    eqns = optimizeIfEquation(expl2.clone(), eqnslst.clone(), eqns.clone(), metamodelica::nil(), metamodelica::nil(), source.clone(), eqAttr.clone(), inAcc.clone())?;
                    Ok((eqns.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a, _, _, _, _) => {
                    Ok((cons(a.clone(), inAcc.clone()), iReplacementPerformed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outBackendDAEEquationLst, replacementPerformed))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn optimizeIfEquation(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut source: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqns = 'mc: {
        let __mc_input = (conditions.clone(), theneqns.clone(), elseenqs.clone(), conditions1.clone(), theneqns1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(listAppend(elseenqs.clone(), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut eqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
                    explst = conditions1.clone().reverse();
                    eqnslst = theneqns1.clone().reverse();
                    Ok(cons(Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: elseenqs.clone(), source: source.clone(), attr: inEqAttr.clone() }), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::List::Cons { head: eqns, tail: _ }, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(listAppend(eqns.clone(), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::List::Cons { head: eqns, tail: _ }, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut eqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
                    explst = conditions1.clone().reverse();
                    eqnslst = theneqns1.clone().reverse();
                    Ok(cons(Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: inEqAttr.clone() }), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: explst }, Deref @ metamodelica::List::Cons { head: _, tail: eqnslst }, _, _, _) => {
                    Ok(optimizeIfEquation(explst.clone(), eqnslst.clone(), elseenqs.clone(), conditions1.clone(), theneqns1.clone(), source.clone(), inEqAttr.clone(), inEqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }, _, _, _) => {
                    Ok(optimizeIfEquation(explst.clone(), eqnslst.clone(), elseenqs.clone(), cons(e.clone(), conditions1.clone()), cons(eqns.clone(), theneqns1.clone()), source.clone(), inEqAttr.clone(), inEqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEqns)
}

fn validWhenLeftHandSide(mut inLhs: Arc<DAE::Exp>, mut inRhs: Arc<DAE::Exp>, mut oldCr: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut oRhs: Arc<DAE::Exp>;
    (outCr, oRhs) = (::match_deref::match_deref! { match &(inLhs.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            (cr.clone(), inRhs.clone())
        },
        Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: op } => {
            (cr.clone(), Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: inRhs.clone() }))
        },
        Deref @ DAE::Exp::LUNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: op } => {
            (cr.clone(), Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: inRhs.clone() }))
        },
        _ => {
            let mut msg: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendVarTransform: failed to replace left hand side of when equation ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(oldCr.clone())?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inLhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            Debug::trace((msg.clone()).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCr, oRhs))
}

fn replaceWhenEquation(mut whenEqn: Arc<BackendDAE::WhenEquation>, mut repl: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut isource: Arc<DAE::ElementSource>) -> Result<(Arc<BackendDAE::WhenEquation>, Arc<DAE::ElementSource>, bool)> {
    let mut outWhenEqn: Arc<BackendDAE::WhenEquation>;
    let mut osource: Arc<DAE::ElementSource>;
    let mut replacementPerformed: bool = false;
    (outWhenEqn, osource, replacementPerformed) = (::match_deref::match_deref! { match &((whenEqn.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), isource.clone())) {
        (Deref @ BackendDAE::WhenEquation { elsewhenPart: oelsewhenPart, whenStmtLst, condition: cond }, _, _, _) => {
            let mut cond1: Arc<DAE::Exp>;
            let mut cond2: Arc<DAE::Exp>;
            let mut weqn: Arc<BackendDAE::WhenEquation>;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut b3: bool = false;
            let mut b4: bool = false;
            let mut source: Arc<DAE::ElementSource>;
            let mut elsewhenPart: Arc<BackendDAE::WhenEquation>;
            let mut oelsewhenPart = (*oelsewhenPart).clone();
            let mut whenStmtLst = (*whenStmtLst).clone();
            (cond1, b1) = replaceExp(cond.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (cond2, _) = ExpressionSimplify::condsimplify(b1.clone(), cond1.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), isource.clone(), cond.clone(), cond2.clone())?;
            (whenStmtLst, b2) = replaceWhenOperator(whenStmtLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), false, metamodelica::nil())?;
            if isSome(oelsewhenPart.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oelsewhenPart.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                elsewhenPart = __pa0.clone();
                (elsewhenPart, source, b3) = replaceWhenEquation(elsewhenPart.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), source.clone())?;
                oelsewhenPart = Some(elsewhenPart.clone());
            } else {
                oelsewhenPart = None;
                b3 = false;
            }
            b4 = b1.clone() || b2.clone() || b3.clone();
            weqn = if (b4.clone()) {Arc::new(BackendDAE::WhenEquation { condition: cond2.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: oelsewhenPart.clone() })} else {whenEqn.clone()};
            (weqn.clone(), source.clone(), b4.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outWhenEqn, osource, replacementPerformed))
}

fn replaceWhenOperator(mut inReinitStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut repl: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut replacementPerformed: bool, mut iAcc: Arc<metamodelica::List<BackendDAE::WhenOperator>>) -> Result<(Arc<metamodelica::List<BackendDAE::WhenOperator>>, bool)> {
    let mut oReinitStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut oReplacementPerformed: bool = false;
    (oReinitStmtLst, oReplacementPerformed) = (::match_deref::match_deref! { match &(inReinitStmtLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iAcc.clone().reverse(), replacementPerformed.clone())
        },
        Deref @ metamodelica::List::Cons { head: wop @ BackendDAE::WhenOperator::ASSIGN { source, right: exp, left: cre @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, tail: res } => {
            let mut res1: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
            let mut wop1: BackendDAE::WhenOperator;
            let mut cre1: Arc<DAE::Exp>;
            let mut exp1: Arc<DAE::Exp>;
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut source = (*source).clone();
            (cre1, b1) = replaceExp(cre.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (_, _) = validWhenLeftHandSide(cre1.clone(), cre.clone(), cr.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), cre.clone(), cre1.clone())?;
            (exp1, b2) = replaceExp(exp.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (exp1, _) = ExpressionSimplify::condsimplify(b2.clone(), exp1.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), exp.clone(), exp1.clone())?;
            b = b1.clone() || b2.clone();
            wop1 = if (b.clone()) {BackendDAE::WhenOperator::ASSIGN { left: cre1.clone(), right: exp1.clone(), source: source.clone() }} else {wop.clone()};
            (res1, b) = replaceWhenOperator(res.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), replacementPerformed.clone() || b.clone(), cons(wop1.clone(), iAcc.clone()))?;
            (res1.clone(), b.clone())
        },
        Deref @ metamodelica::List::Cons { head: wop @ BackendDAE::WhenOperator::ASSIGN { source, right: exp, left: cre }, tail: res } => {
            let mut res1: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
            let mut wop1: BackendDAE::WhenOperator;
            let mut cre1: Arc<DAE::Exp>;
            let mut exp1: Arc<DAE::Exp>;
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut source = (*source).clone();
            (cre1, b1) = replaceExp(cre.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), cre.clone(), cre1.clone())?;
            (exp1, b2) = replaceExp(exp.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (exp1, _) = ExpressionSimplify::condsimplify(b2.clone(), exp1.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), exp.clone(), exp1.clone())?;
            b = b1.clone() || b2.clone();
            wop1 = if (b.clone()) {BackendDAE::WhenOperator::ASSIGN { left: cre1.clone(), right: exp1.clone(), source: source.clone() }} else {wop.clone()};
            (res1, b) = replaceWhenOperator(res.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), replacementPerformed.clone() || b.clone(), cons(wop1.clone(), iAcc.clone()))?;
            (res1.clone(), b.clone())
        },
        Deref @ metamodelica::List::Cons { head: wop @ BackendDAE::WhenOperator::REINIT { source, value: cond, stateVar: cr }, tail: res } => {
            let mut res1: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
            let mut wop1: BackendDAE::WhenOperator;
            let mut cond1: Arc<DAE::Exp>;
            let mut cre: Arc<DAE::Exp>;
            let mut cre1: Arc<DAE::Exp>;
            let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut source = (*source).clone();
            cre = Expression::crefExp(cr.clone())?;
            (cre1, b1) = replaceExp(cre.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (cr1, _) = validWhenLeftHandSide(cre1.clone(), cre.clone(), cr.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), cre.clone(), cre1.clone())?;
            (cond1, b2) = replaceExp(cond.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (cond1, _) = ExpressionSimplify::condsimplify(b2.clone(), cond1.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), cond.clone(), cond1.clone())?;
            b = b1.clone() || b2.clone();
            wop1 = if (b.clone()) {BackendDAE::WhenOperator::REINIT { stateVar: cr1.clone(), value: cond1.clone(), source: source.clone() }} else {wop.clone()};
            (res1, b) = replaceWhenOperator(res.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), replacementPerformed.clone() || b.clone(), cons(wop1.clone(), iAcc.clone()))?;
            (res1.clone(), b.clone())
        },
        Deref @ metamodelica::List::Cons { head: wop @ BackendDAE::WhenOperator::ASSERT { source, level, message: exp, condition: cond }, tail: res } => {
            let mut res1: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
            let mut wop1: BackendDAE::WhenOperator;
            let mut cond1: Arc<DAE::Exp>;
            let mut exp1: Arc<DAE::Exp>;
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut source = (*source).clone();
            (cond1, b1) = replaceExp(cond.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (cond1, _) = ExpressionSimplify::condsimplify(b1.clone(), cond1.clone())?;
            (exp1, b2) = replaceExp(exp.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            b = b1.clone() || b2.clone();
            source = ElementSource::addSymbolicTransformationSubstitution(b.clone(), source.clone(), cond.clone(), cond1.clone())?;
            wop1 = if (b.clone()) {BackendDAE::WhenOperator::ASSERT { condition: cond1.clone(), message: exp1.clone(), level: level.clone(), source: source.clone() }} else {wop.clone()};
            (res1, b) = replaceWhenOperator(res.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), replacementPerformed.clone() || b.clone(), cons(wop1.clone(), iAcc.clone()))?;
            (res1.clone(), b.clone())
        },
        Deref @ metamodelica::List::Cons { head: wop @ BackendDAE::WhenOperator::TERMINATE { source, message: exp }, tail: res } => {
            let mut res1: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
            let mut wop1: BackendDAE::WhenOperator;
            let mut exp1: Arc<DAE::Exp>;
            let mut b: bool = false;
            let mut source = (*source).clone();
            (exp1, b) = replaceExp(exp.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b.clone(), source.clone(), exp.clone(), exp1.clone())?;
            wop1 = if (b.clone()) {BackendDAE::WhenOperator::TERMINATE { message: exp1.clone(), source: source.clone() }} else {wop.clone()};
            (res1, b) = replaceWhenOperator(res.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), replacementPerformed.clone() || b.clone(), cons(wop1.clone(), iAcc.clone()))?;
            (res1.clone(), b.clone())
        },
        Deref @ metamodelica::List::Cons { head: wop @ BackendDAE::WhenOperator::NORETCALL { source, exp }, tail: res } => {
            let mut res1: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
            let mut wop1: BackendDAE::WhenOperator;
            let mut exp1: Arc<DAE::Exp>;
            let mut b: bool = false;
            let mut source = (*source).clone();
            (exp1, b) = replaceExp(exp.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (exp1, _) = ExpressionSimplify::condsimplify(b.clone(), exp1.clone())?;
            source = ElementSource::addSymbolicTransformationSubstitution(b.clone(), source.clone(), exp.clone(), exp1.clone())?;
            wop1 = if (b.clone()) {BackendDAE::WhenOperator::NORETCALL { exp: exp1.clone(), source: source.clone() }} else {wop.clone()};
            (res1, b) = replaceWhenOperator(res.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), replacementPerformed.clone() || b.clone(), cons(wop1.clone(), iAcc.clone()))?;
            (res1.clone(), b.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oReinitStmtLst, oReplacementPerformed))
}

/* ********************************************************/
/* replace statements  */
/* ********************************************************/
pub fn replaceStatementLst(mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut inAcc: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inBAcc: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, bool)> {
    let mut outStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut replacementPerformed: bool = inBAcc.clone();
    let mut isCon: bool = false;
    let mut repl: VariableReplacements = inVariableReplacements.clone();
    let mut es: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut es_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut statementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut statement: Arc<DAE::Statement>;
    let mut statement_1: Arc<DAE::Statement>;
    let mut type_: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut e1_1: Arc<DAE::Exp>;
    let mut e2_1: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e1_2: Arc<DAE::Exp>;
    let mut e2_2: Arc<DAE::Exp>;
    let mut e3: Arc<DAE::Exp>;
    let mut e3_1: Arc<DAE::Exp>;
    let mut e3_2: Arc<DAE::Exp>;
    let mut expExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expExpLst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut else_: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut source: Arc<DAE::ElementSource>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut iterIsArray: bool = false;
    let mut ident: ArcStr = arcstr::literal!("");
    let mut conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut initialCall: bool = false;
    let mut b: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    let mut loopPrlVars: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    for mut stmt in &*inStatementLst.clone() {
        let mut stmt = stmt.clone();
        (outStatementLst, replacementPerformed) = 'mc: {
        let __mc_input = stmt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { source, exp: e2, exp1: e1, type_ } => {
                    let mut source = (*source).clone();
                    let mut b1: bool = b1.clone();
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b2: bool = b2.clone();
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e2_2: Arc<DAE::Exp>;
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (e1_2, _) = ExpressionSimplify::simplify(e1_1.clone())?;
                    (e2_2, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                    (e1_2, e2_2) = moveNegateRhs(e1_2.clone(), e2_2.clone());
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_2.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_2.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: e1_2.clone(), exp: e2_2.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, exp: e2, expExpLst, type_ } => {
                    let mut source = (*source).clone();
                    let mut e2_2: Arc<DAE::Exp>;
                    let mut b2: bool = b2.clone();
                    let mut b1: bool = b1.clone();
                    let mut expExpLst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = expExpLst_1.clone();
                    let mut e2_1: Arc<DAE::Exp>;
                    (expExpLst_1, b1) = replaceExpList(expExpLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_1.clone())?;
                    (e2_2, b1) = ExpressionSimplify::simplify(e2_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e2_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e2_2.clone() }))?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: type_.clone(), expExpLst: expExpLst_1.clone(), exp: e2_2.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: e2, lhs: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, type_ } => {
                    let mut source = (*source).clone();
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b2: bool = b2.clone();
                    let mut e2_2: Arc<DAE::Exp>;
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    e2_2 = __pa1.clone();
                    source = __pa2.clone();
                    Ok((validLhsArrayAssignSTMT(cr.clone(), e1_1.clone(), e2_2.clone(), type_.clone(), source.clone(), outStatementLst.clone())?, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_IF { source, else_, statementLst, exp: e1 } => {
                    let mut source = (*source).clone();
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e1_2, _) = ExpressionSimplify::condsimplify(b1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_2.clone())?;
                    Ok(replaceSTMT_IF(e1_2.clone(), statementLst.clone(), else_.clone(), source.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), outStatementLst.clone(), replacementPerformed.clone() || b1.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FOR { source, statementLst, range: e1, iter: ident, iterIsArray, type_ } => {
                    let mut source = (*source).clone();
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = statementLst_1.clone();
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    let mut repl: VariableReplacements = repl.clone();
                    let mut b2: bool = b2.clone();
                    repl = addIterationVar(repl.clone(), (ident.clone()).clone());
                    (statementLst_1, b1) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    (e1_1, b2) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    (e1_2, b1) = ExpressionSimplify::condsimplify(b2.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                    repl = removeIterationVar(repl.clone(), (ident.clone()).clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_FOR { type_: type_.clone(), iterIsArray: iterIsArray.clone(), iter: (ident.clone()).clone(), range: e1_2.clone(), statementLst: statementLst_1.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_PARFOR { source, loopPrlVars, statementLst, range: e1, iter: ident, iterIsArray, type_ } => {
                    let mut source = (*source).clone();
                    let mut b2: bool = b2.clone();
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = statementLst_1.clone();
                    (statementLst_1, b1) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    (e1_1, b2) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    (e1_2, b1) = ExpressionSimplify::condsimplify(b2.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_PARFOR { type_: type_.clone(), iterIsArray: iterIsArray.clone(), iter: (ident.clone()).clone(), range: e1_2.clone(), statementLst: statementLst_1.clone(), loopPrlVars: loopPrlVars.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHILE { source, statementLst, exp: e1 } => {
                    let mut source = (*source).clone();
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = statementLst_1.clone();
                    let mut b1: bool = b1.clone();
                    let mut b2: bool = b2.clone();
                    (statementLst_1, b1) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    (e1_1, b2) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    (e1_2, b1) = ExpressionSimplify::condsimplify(b2.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHILE { exp: e1_2.clone(), statementLst: statementLst_1.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: None, statementLst, initialCall, conditions, exp: e1 } => {
                    let mut source = (*source).clone();
                    let mut b2: bool = b2.clone();
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = statementLst_1.clone();
                    (statementLst_1, b1) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    (e1_1, b2) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    (e1_2, b1) = ExpressionSimplify::condsimplify(b2.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e1_2.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: statementLst_1.clone(), elseWhen: None, source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: Some(statement), statementLst, initialCall, conditions, exp: e1 } => {
                    let mut source = (*source).clone();
                    let mut b1: bool = b1.clone();
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = statementLst_1.clone();
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut statement_1: Arc<DAE::Statement>;
                    let mut b2: bool = b2.clone();
                    let mut b3: bool = b3.clone();
                    (statementLst_1, b1) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(replaceStatementLst(list![statement.clone()], repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    statement_1 = __pa0.clone();
                    b2 = __pa1.clone();
                    (e1_1, b3) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b3.clone(), source.clone(), e1.clone(), e1_1.clone())?;
                    (e1_2, b1) = ExpressionSimplify::condsimplify(b3.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e1_2.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: statementLst_1.clone(), elseWhen: Some(statement_1.clone()), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSERT { source, level: e3, msg: e2, cond: e1 } => {
                    let mut source = (*source).clone();
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut b3: bool = b3.clone();
                    let mut e2_2: Arc<DAE::Exp>;
                    let mut e3_1: Arc<DAE::Exp>;
                    let mut e3_2: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut b2: bool = b2.clone();
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e3_1, b3) = replaceExp(e3.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone() || b3.clone()) else { bail!("pattern mismatch") };
                    (e1_2, _) = ExpressionSimplify::condsimplify(b1.clone(), e1_1.clone())?;
                    (e2_2, _) = ExpressionSimplify::condsimplify(b2.clone(), e2_1.clone())?;
                    (e3_2, _) = ExpressionSimplify::condsimplify(b3.clone(), e3_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_2.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_2.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b3.clone(), source.clone(), e3.clone(), e3_2.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_ASSERT { cond: e1_2.clone(), msg: e2_2.clone(), level: e3_2.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TERMINATE { source, msg: e1 } => {
                    let mut source = (*source).clone();
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e1.clone(), e1_1.clone())?;
                    (e1_2, b1) = ExpressionSimplify::simplify(e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_TERMINATE { msg: e1_2.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_REINIT { source, value: e2, var: e1 } => {
                    let mut source = (*source).clone();
                    let mut b2: bool = b2.clone();
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e2_2: Arc<DAE::Exp>;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    (e1_1, b1) = replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e2_1, b2) = replaceExp(e2.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    (e1_2, _) = ExpressionSimplify::condsimplify(b1.clone(), e1_1.clone())?;
                    (e2_2, _) = ExpressionSimplify::condsimplify(b2.clone(), e2_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b1.clone(), source.clone(), e1.clone(), e1_2.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2.clone(), source.clone(), e2.clone(), e2_2.clone())?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_REINIT { var: e1_2.clone(), value: e2_2.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_NORETCALL { source, exp: e1 } => {
                    let mut source = (*source).clone();
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut b1: bool = b1.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e1.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1_1 = __pa0.clone();
                    source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e1.clone(), e1_1.clone())?;
                    (e1_2, b1) = ExpressionSimplify::simplify(e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplify(b1.clone(), source.clone(), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                    Ok((cons(Arc::new(DAE::Statement::STMT_NORETCALL { exp: e1_2.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FAILURE { source, body: statementLst } => {
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = statementLst_1.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    statementLst_1 = __pa0.clone();
                    Ok((cons(Arc::new(DAE::Statement::STMT_FAILURE { body: statementLst_1.clone(), source: source.clone() }), outStatementLst.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((cons(stmt.clone(), outStatementLst.clone()), replacementPerformed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    }
    outStatementLst = outStatementLst.clone().reverse();
    Ok((outStatementLst, replacementPerformed))
}

pub fn replaceStatementLstRHS(mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut inAcc: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inBAcc: bool) -> (Arc<metamodelica::List<Arc<DAE::Statement>>>, bool) {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut outStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    let (__pa0, (_, _, __pa1)) = DAEUtil::traverseDAEEquationsStmtsRhsOnly(inStatementLst.clone(), Arc::new(replaceExpWrapper), (inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone(), false));
    outStatementLst = __pa0.clone();
    replacementPerformed = __pa1.clone();
    (outStatementLst, replacementPerformed)
}

fn replaceExpWrapper(mut inExp: Arc<DAE::Exp>, mut inTpl: (VariableReplacements, Option<FuncTypeExp_ExpToBoolean>, bool)) -> Result<(Arc<DAE::Exp>, (VariableReplacements, Option<FuncTypeExp_ExpToBoolean>, bool))> {
    pub type FuncTypeExp_ExpToBoolean = fn(Arc<DAE::Exp>) -> Result<bool>;

    let mut exp: Arc<DAE::Exp>;
    let mut tpl: (VariableReplacements, Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>>, bool);
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut repl: VariableReplacements;
    let mut opt: Option<Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>> = None;
    exp = inExp.clone();
    tpl = inTpl.clone();
    (repl, opt, b1) = tpl.clone();
    (exp, b2) = replaceExp(exp.clone(), repl.clone(), opt.clone())?;
    b2 = b1.clone() || b2.clone();
    tpl = (repl.clone(), opt.clone(), b2.clone());
    Ok((exp, tpl))
}

fn moveNegateRhs(mut inLhs: Arc<DAE::Exp>, mut inRhs: Arc<DAE::Exp>) -> (Arc<DAE::Exp>, Arc<DAE::Exp>) {
    let mut outLhs: Arc<DAE::Exp>;
    let mut outRhs: Arc<DAE::Exp>;
    (outLhs, outRhs) = (::match_deref::match_deref! { match &((inLhs.clone(), inRhs.clone())) {
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty }, exp: e }, _) => {
            (e.clone(), Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: inRhs.clone() }))
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty }, exp: e }, _) => {
            (e.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: inRhs.clone() }))
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty }, exp: e }, _) => {
            (e.clone(), Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: inRhs.clone() }))
        },
        (_, _) => {
            (inLhs.clone(), inRhs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outLhs, outRhs)
}

fn validLhsArrayAssignSTMT(mut oldCr: Arc<DAE::ComponentRef>, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut type_: Arc<DAE::Type>, mut source: Arc<DAE::ElementSource>, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatementLst = 'mc: {
        let __mc_input = lhs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                crefexp => {
                    Ok(cons(Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: rhs.clone(), source: source.clone() }), inStatementLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: crefexp } => {
                    Ok(cons(Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp }, exp: crefexp } => {
                    Ok(cons(Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp }, exp: crefexp } => {
                    Ok(cons(Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: elst, .. } => {
                    let mut statementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e: Arc<DAE::Exp>;
                    let mut ds: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    ds = Expression::dimensionsSizes(Expression::arrayDimension(type_.clone()));
                    subslst = Expression::dimensionSizesSubscripts(ds.clone());
                    subslst = Expression::rangesToSubscripts(subslst.clone())?;
                    elst1 = List::map1r(subslst.clone(), Arc::new(Expression::applyExpSubscripts), rhs.clone());
                    e = listHead(elst1.clone())?;
                    tp = Expression::r#typeof(e.clone())?;
                    statementLst = List::threadFold2(elst.clone(), elst1.clone(), Arc::new(validLhsAssignSTMT), tp.clone(), source.clone(), inStatementLst.clone())?;
                    Ok(statementLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendVarTransform: failed to replace left hand side of array assign statement ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(oldCr.clone())?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Debug::trace((msg.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatementLst)
}

fn validLhsAssignSTMT(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut type_: Arc<DAE::Type>, mut source: Arc<DAE::ElementSource>, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatementLst = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ DAE::Exp::CREF { .. } => {
            cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() }), inStatementLst.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: Deref @ DAE::Exp::CREF { .. } } => {
            cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: lhs.clone(), exp: Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone())
        },
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp }, exp: Deref @ DAE::Exp::CREF { .. } } => {
            cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: lhs.clone(), exp: Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStatementLst)
}

fn replaceElse(mut inElse: Arc<DAE::Else>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<DAE::Else>, bool)> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut replacementPerformed: bool = false;
    (outElse, replacementPerformed) = 'mc: {
        let __mc_input = inElse.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSEIF { else_, statementLst, exp: e1 } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e1_2: Arc<DAE::Exp>;
                    let mut else_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (e1_1, b1) = replaceExp(e1.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    (e1_2, _) = ExpressionSimplify::condsimplify(b1.clone(), e1_1.clone())?;
                    (else_1, b2) = replaceElse1(e1_2.clone(), statementLst.clone(), else_.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((else_1.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSE { statementLst } => {
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceStatementLst(statementLst.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    statementLst_1 = __pa0.clone();
                    Ok((Arc::new(DAE::Else::ELSE { statementLst: statementLst_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inElse.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElse, replacementPerformed))
}

fn replaceElse1(mut inExp: Arc<DAE::Exp>, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inElse: Arc<DAE::Else>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<(Arc<DAE::Else>, bool)> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut replacementPerformed: bool = false;
    (outElse, replacementPerformed) = 'mc: {
        let __mc_input = (inExp.clone(), inStatementLst.clone(), inElse.clone(), inVariableReplacements.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, statementLst, _, repl) => {
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    (statementLst_1, _) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    Ok((Arc::new(DAE::Else::ELSE { statementLst: statementLst_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, else_, repl) => {
                    let mut else_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    (else_1, _) = replaceElse(else_.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    Ok((else_1.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, statementLst, else_, repl) => {
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut else_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (statementLst_1, b1) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    (else_1, b2) = replaceElse(else_.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Else::ELSEIF { exp: e1.clone(), statementLst: statementLst_1.clone(), else_: else_1.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, statementLst, else_, _) => {
                    Ok((Arc::new(DAE::Else::ELSEIF { exp: e1.clone(), statementLst: statementLst.clone(), else_: else_.clone() }), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElse, replacementPerformed))
}

fn replaceSTMT_IF(mut inExp: Arc<DAE::Exp>, mut inStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inElse: Arc<DAE::Else>, mut inSource: Arc<DAE::ElementSource>, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>, mut inAcc: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inBAcc: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, bool)> {
    let mut outStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut replacementPerformed: bool = false;
    (outStatementLst, replacementPerformed) = 'mc: {
        let __mc_input = (inExp.clone(), inStatementLst.clone(), inElse.clone(), inSource.clone(), inVariableReplacements.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, statementLst, _, _, repl) => {
                    Ok(replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::NOELSE, _, _) => {
                    Ok((inAcc.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::ELSEIF { else_: else_e, statementLst: statementLst_e, exp: exp_e }, source, repl) => {
                    Ok(replaceSTMT_IF(exp_e.clone(), statementLst_e.clone(), else_e.clone(), source.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::ELSE { statementLst: statementLst_e }, _, repl) => {
                    Ok(replaceStatementLst(statementLst_e.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, statementLst, else_, source, repl) => {
                    let mut statementLst_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut else_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    (statementLst_1, b1) = replaceStatementLst(statementLst.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    (else_1, b2) = replaceElse(else_.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((cons(Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: statementLst_1.clone(), else_: else_1.clone(), source: source.clone() }), inAcc.clone()), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, statementLst, else_, source, _) => {
                    Ok((cons(Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: statementLst.clone(), else_: else_.clone(), source: source.clone() }), inAcc.clone()), inBAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStatementLst, replacementPerformed))
}

/* ********************************************************/
/* variable replacements  */
/* ********************************************************/
pub fn replaceVarTraverser(mut inVar: BackendDAE::Var, mut inRepl: VariableReplacements) -> Result<(BackendDAE::Var, VariableReplacements)> {
    let mut outVar: BackendDAE::Var;
    let mut repl: VariableReplacements = inRepl.clone();
    outVar = replaceBindingExp(inVar.clone(), inRepl.clone())?;
    outVar = replaceVariableAttributesInVar(outVar.clone(), inRepl.clone())?;
    Ok((outVar, repl))
}

pub fn replaceBindingExp(mut varIn: BackendDAE::Var, mut repl: VariableReplacements) -> Result<BackendDAE::Var> {
    let mut varOut: BackendDAE::Var;
    varOut = (::match_deref::match_deref! { match &(varIn.clone()) {
        BackendDAE::Var { bindExp: Some(exp), .. } => {
            let mut exp = (*exp).clone();
            (exp, _) = replaceExp(exp.clone(), repl.clone(), None)?;
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            BackendVariable::setBindExp(varIn.clone(), Some(exp.clone()))
        },
        BackendDAE::Var { bindExp: None, .. } => {
            varIn.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(varOut)
}

pub fn replaceVariableAttributes(mut attrIn: Arc<DAE::VariableAttributes>, mut repl: VariableReplacements) -> Result<Arc<DAE::VariableAttributes>> {
    let mut attrOut: Arc<DAE::VariableAttributes>;
    attrOut = (::match_deref::match_deref! { match &(attrIn.clone()) {
        Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity, unit, displayUnit, min, max, start, fixed, nominal, stateSelectOption, uncertainOption, distributionOption, equationBound, isProtected, finalPrefix, startOrigin } => {
            let mut quantity = (*quantity).clone();
            let mut unit = (*unit).clone();
            let mut displayUnit = (*displayUnit).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut nominal = (*nominal).clone();
            let mut equationBound = (*equationBound).clone();
            let mut startOrigin = (*startOrigin).clone();
            quantity = replaceOptionExp(quantity.clone(), repl.clone())?;
            unit = replaceOptionExp(unit.clone(), repl.clone())?;
            displayUnit = replaceOptionExp(displayUnit.clone(), repl.clone())?;
            min = replaceOptionExp(min.clone(), repl.clone())?;
            max = replaceOptionExp(max.clone(), repl.clone())?;
            start = replaceOptionExp(start.clone(), repl.clone())?;
            fixed = replaceOptionExp(fixed.clone(), repl.clone())?;
            nominal = replaceOptionExp(nominal.clone(), repl.clone())?;
            equationBound = replaceOptionExp(equationBound.clone(), repl.clone())?;
            startOrigin = replaceOptionExp(startOrigin.clone(), repl.clone())?;
            Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelectOption: stateSelectOption.clone(), uncertainOption: uncertainOption.clone(), distributionOption: distributionOption.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() })
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity, min, max, start, fixed, uncertainOption, distributionOption, equationBound, isProtected, finalPrefix, startOrigin } => {
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            let mut startOrigin = (*startOrigin).clone();
            quantity = replaceOptionExp(quantity.clone(), repl.clone())?;
            min = replaceOptionExp(min.clone(), repl.clone())?;
            max = replaceOptionExp(max.clone(), repl.clone())?;
            start = replaceOptionExp(start.clone(), repl.clone())?;
            fixed = replaceOptionExp(fixed.clone(), repl.clone())?;
            equationBound = replaceOptionExp(equationBound.clone(), repl.clone())?;
            startOrigin = replaceOptionExp(startOrigin.clone(), repl.clone())?;
            Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), uncertainOption: uncertainOption.clone(), distributionOption: distributionOption.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() })
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity, start, fixed, equationBound, isProtected, finalPrefix, startOrigin } => {
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            let mut startOrigin = (*startOrigin).clone();
            quantity = replaceOptionExp(quantity.clone(), repl.clone())?;
            start = replaceOptionExp(start.clone(), repl.clone())?;
            fixed = replaceOptionExp(fixed.clone(), repl.clone())?;
            equationBound = replaceOptionExp(equationBound.clone(), repl.clone())?;
            startOrigin = replaceOptionExp(startOrigin.clone(), repl.clone())?;
            Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() })
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity, start, fixed, equationBound, isProtected, finalPrefix, startOrigin } => {
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            let mut startOrigin = (*startOrigin).clone();
            quantity = replaceOptionExp(quantity.clone(), repl.clone())?;
            start = replaceOptionExp(start.clone(), repl.clone())?;
            fixed = replaceOptionExp(fixed.clone(), repl.clone())?;
            equationBound = replaceOptionExp(equationBound.clone(), repl.clone())?;
            startOrigin = replaceOptionExp(startOrigin.clone(), repl.clone())?;
            Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() })
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity, min, max, start, fixed, equationBound, isProtected, finalPrefix, startOrigin } => {
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            let mut startOrigin = (*startOrigin).clone();
            quantity = replaceOptionExp(quantity.clone(), repl.clone())?;
            min = replaceOptionExp(min.clone(), repl.clone())?;
            max = replaceOptionExp(max.clone(), repl.clone())?;
            start = replaceOptionExp(start.clone(), repl.clone())?;
            fixed = replaceOptionExp(fixed.clone(), repl.clone())?;
            equationBound = replaceOptionExp(equationBound.clone(), repl.clone())?;
            startOrigin = replaceOptionExp(startOrigin.clone(), repl.clone())?;
            Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() })
        },
        _ => {
            attrIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attrOut)
}

pub fn replaceOptionExp(mut optIn: Option<Arc<DAE::Exp>>, mut repl: VariableReplacements) -> Result<Option<Arc<DAE::Exp>>> {
    let mut optOut: Option<Arc<DAE::Exp>> = None;
    let mut exp: Arc<DAE::Exp>;
    if isSome(optIn.clone()) {
        exp = Util::getOption(optIn.clone())?;
        (exp, _) = replaceExp(exp.clone(), repl.clone(), None)?;
        optOut = Some(exp.clone());
    } else {
        optOut = None;
    }
    Ok(optOut)
}

pub fn replaceVariableAttributesInVar(mut varIn: BackendDAE::Var, mut repl: VariableReplacements) -> Result<BackendDAE::Var> {
    let mut varOut: BackendDAE::Var;
    varOut = (::match_deref::match_deref! { match &(varIn.clone()) {
        BackendDAE::Var { values: Some(values), .. } => {
            let mut values = (*values).clone();
            values = replaceVariableAttributes(values.clone(), repl.clone())?;
            BackendVariable::setVarAttributes(varIn.clone(), Some(values.clone()))
        },
        _ => {
            varIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(varOut)
}

fn negateOperator(mut inOp: DAE::Operator) -> DAE::Operator {
    let mut outOp: DAE::Operator;
    outOp = (match inOp.clone() {
        DAE::Operator::UMINUS { ty: mut ty } => {
            DAE::Operator::ADD { ty: ty.clone() }
        },
        DAE::Operator::SUB { ty: mut ty } => {
            DAE::Operator::ADD { ty: ty.clone() }
        },
        DAE::Operator::ADD { ty: mut ty } => {
            DAE::Operator::SUB { ty: ty.clone() }
        },
        _ => {
            inOp.clone()
        },
    });
    outOp
}

pub fn replaceEventInfo(mut eInfoIn: BackendDAE::EventInfo, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<BackendDAE::EventInfo> {
    type Func = fn(BackendDAE::ZeroCrossing, Option<FuncTypeExp_ExpToBoolean>) -> Result<BackendDAE::ZeroCrossing>;

    let mut eInfoOut: BackendDAE::EventInfo;
    let mut numberMathEvents: i32 = 0;
    let mut timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>> = metamodelica::nil();
    let mut zeroCrossingLst: BackendDAE::ZeroCrossingSet;
    let mut sampleLst: BackendDAE::ZeroCrossingSet;
    let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
    let mut zc: Arc<dyn ::std::ops::Fn(BackendDAE::ZeroCrossing, Option<FuncTypeExp_ExpToBoolean>) -> Result<BackendDAE::ZeroCrossing> + 'static>;
    let BackendDAE::EVENT_INFO { timeEvents: __pa0, zeroCrossings: __pa1, relations: __pa2, samples: __pa3, numberMathEvents: __pa4 } = (eInfoIn.clone()) else { bail!("pattern mismatch") };
    timeEvents = __pa0.clone();
    zeroCrossingLst = __pa1.clone();
    relationsLst = __pa2.clone();
    sampleLst = __pa3.clone();
    numberMathEvents = __pa4.clone();
    timeEvents = List::map2(timeEvents.clone(), Arc::new(replaceTimeEvents), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone());
    zc = { let __pe_b1 = inVariableReplacements.clone(); move |__pe_a0, __pe_a2| replaceZeroCrossing(__pe_a0, __pe_b1.clone(), __pe_a2) };
    DoubleEnded::mapNoCopy_1(zeroCrossingLst.zc.clone(), zc.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
    DoubleEnded::mapNoCopy_1(sampleLst.zc.clone(), zc.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
    DoubleEnded::mapNoCopy_1(relationsLst.clone(), zc.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
    eInfoOut = BackendDAE::EventInfo { timeEvents: timeEvents.clone(), zeroCrossings: zeroCrossingLst.clone(), relations: relationsLst.clone(), samples: sampleLst.clone(), numberMathEvents: numberMathEvents.clone() };
    Ok(eInfoOut)
}

fn replaceTimeEvents(mut teIn: BackendDAE::TimeEvent, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<BackendDAE::TimeEvent> {
    let mut teOut: BackendDAE::TimeEvent = BackendDAE::TimeEvent::SIMPLE_TIME_EVENT;
    teOut = 'mc: {
        let __mc_input = teIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { intervalExp: mut intervalExp, startExp: mut startExp, index: mut index } = __mc_input.clone() else { bail!("nomatch") };
            let mut intervalExp = intervalExp.clone();
            let mut startExp = startExp.clone();
            (startExp, _) = replaceExp(startExp.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            (intervalExp, _) = replaceExp(intervalExp.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            Ok(BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { index: index.clone(), startExp: startExp.clone(), intervalExp: intervalExp.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(teIn.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(teOut)
}

fn replaceZeroCrossing(mut zcIn: BackendDAE::ZeroCrossing, mut inVariableReplacements: VariableReplacements, mut inFuncTypeExpExpToBooleanOption: Option<FuncTypeExp_ExpToBoolean>) -> Result<BackendDAE::ZeroCrossing> {
    let mut zcOut: BackendDAE::ZeroCrossing;
    zcOut = 'mc: {
        let __mc_input = zcIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::ZeroCrossing { relation_: mut relation_, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut relation_ = relation_.clone();
            (relation_, _) = replaceExp(relation_.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
            Ok(BackendDAE::ZeroCrossing { index: zcIn.index.clone(), relation_: relation_.clone(), occurEquLst: zcIn.occurEquLst.clone(), iter: zcIn.iter.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(zcIn.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(zcOut)
}

/* ********************************************************/
/* dump replacements  */
/* ********************************************************/
pub fn dumpReplacements(mut repl: VariableReplacements) -> Result<()> {
    let mut srcs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dsts: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
    (srcs, dsts) = getAllReplacements(repl.clone())?;
    tplLst = List::zip(srcs.clone(), dsts.clone());
    println!("{}", (literal!("\nReplacements: (")).clone());
    println!("{}", ArcStr::from(::std::format!("{}", (tplLst.clone().len() as i32))));
    println!("{}", (literal!(")\n")).clone());
    println!("{}", (literal!("========================================\n")).clone());
    println!("{}", stringDelimitList(List::map(tplLst.clone(), Arc::new(printReplacementTupleStr)), (literal!("\n")).clone()));
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpExtendReplacements(mut repl: VariableReplacements) -> Result<()> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crefs = UnorderedSet::toList(repl.extendhashTable.clone());
    println!("{}", (literal!("\nExtendReplacements: (")).clone());
    println!("{}", ArcStr::from(::std::format!("{}", (crefs.clone().len() as i32))));
    println!("{}", (literal!(")\n")).clone());
    println!("{}", (literal!("========================================\n")).clone());
    println!("{}", stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut c in (crefs.clone()).into_iter().cloned() {
            let __x = ComponentReferenceBasics::printComponentRefStr(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n")).clone()));
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpDerConstReplacements(mut repl: VariableReplacements) -> Result<()> {
    let mut srcs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dsts: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
    if isSome(repl.derConst.clone()) {
        (srcs, dsts) = getCrefExpTableEntries(Util::getOption(repl.derConst.clone())?)?;
        tplLst = List::zip(srcs.clone(), dsts.clone());
        println!("{}", (literal!("\nDerConstReplacements: (")).clone());
        println!("{}", ArcStr::from(::std::format!("{}", (tplLst.clone().len() as i32))));
        println!("{}", (literal!(")\n")).clone());
        println!("{}", (literal!("========================================\n")).clone());
        println!("{}", stringDelimitList(List::map(tplLst.clone(), Arc::new(printReplacementTupleStr)), (literal!("\n")).clone()));
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

fn printReplacementTupleStr(mut tpl: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(tpl.clone()))?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::tuple22(tpl.clone()))?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn getConstantReplacements(mut replIn: VariableReplacements) -> Result<VariableReplacements> {
    let mut replOut: VariableReplacements;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (crefs, exps) = getAllReplacements(replIn.clone())?;
    (exps, crefs) = List::filterOnTrueSync(exps.clone(), Arc::new(fnptr!(Expression::isEvaluatedConst, Arc<DAE::Exp>)), crefs.clone())?;
    replOut = emptyReplacements();
    replOut = addReplacements(replOut.clone(), crefs.clone(), exps.clone(), None)?;
    Ok(replOut)
}

