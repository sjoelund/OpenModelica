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
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendUtil;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::InlineArrayEquations;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::HashTableCG;
use openmodelica_frontend::Inline;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// late inline functions stuff
//
// =============================================================================
pub fn lateInlineFunction(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = inlineCalls(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE], inDAE.clone())?;
    Ok(outDAE)
}

// =============================================================================
// normal inline functions stuff
//
// =============================================================================
pub fn normalInlineFunction(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    if Flags::getConfigEnum(Flags::INLINE_METHOD.clone())? == 1 {
        outDAE = inlineCalls(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE], inDAE.clone())?;
    } else {
        outDAE = inlineCallsBDAE(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE], inDAE.clone())?;
    }
    Ok(outDAE)
}

// =============================================================================
// inline calls stuff
//
// =============================================================================
// =============================================================================
// inline append functions
//
// =============================================================================
fn addNoEvent(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool = inB.clone();
    outExp = Expression::addNoEventToRelationsAndConds(inExp.clone())?;
    outExp = Expression::addNoEventToEventTriggeringFunctions(outExp.clone())?;
    Ok((outExp, outB))
}

fn createReplacementVariables(mut inCref: Arc<DAE::ComponentRef>, mut funcName: ArcStr, mut inRepls: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> {
    let mut crVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outRepls: BackendVarTransform::VariableReplacements = inRepls.clone();
    let mut eVar: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut arrExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: BackendDAE::Var;
    var = BackendVariable::createTmpVar(inCref.clone(), (funcName.clone()).clone())?;
    crVar = BackendVariable::varCref(var.clone())?;
    eVar = Expression::crefExp(crVar.clone())?;
    let false = (Expression::isRecord(eVar.clone())) else { bail!("pattern mismatch") };
    outRepls = BackendVarTransform::addReplacement(outRepls.clone(), inCref.clone(), eVar.clone(), None)?;
    crefs = ComponentReference::expandCref(inCref.clone(), false)?;
    crefs1 = ComponentReference::expandCref(crVar.clone(), false)?;
    match '__try0: {
        arrExp = unwrap_break_err!(Expression::getArrayOrRangeContents(eVar.clone()), '__try0);
        Ok::<_, anyhow::Error>((arrExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            arrExp = __try0_o0;
        }
        Err(_) => {
            arrExp = list![eVar.clone()];
        }
    }
    if (crefs.clone().len() as i32) != (arrExp.clone().len() as i32) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendInline.createReplacementVariables failed with array handling ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eVar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    for mut c in &*crefs.clone() {
        let mut c = c.clone();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(crefs1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa1.clone();
        crefs1 = __pa2.clone();
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(arrExp.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa3.clone();
        arrExp = __pa4.clone();
        var.varName = cr.clone();
        outVars = cons(var.clone(), outVars.clone());
        outRepls = BackendVarTransform::addReplacement(outRepls.clone(), c.clone(), e.clone(), None)?;
    }
    outVars = outVars.clone().reverse();
    Ok((crVar, outVars, outRepls))
}

fn createEqnSysfromFunction(mut fns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut funcname: ArcStr) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<BackendDAE::EqSystem>)> {
    let mut oOutput: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outEqs: Arc<BackendDAE::EqSystem>;
    let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = inArgs.clone();
    let mut left_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut fnInputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut argmap: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
    let mut checkcr: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: BackendDAE::Var;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut eqlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ncreate EqnSys from function: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
    }
    outEqs = BackendDAEUtil::createEqSystem(BackendVariable::listVar(metamodelica::nil()), BackendEquation::listEquation(metamodelica::nil())?, metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    repl = BackendVarTransform::emptyReplacements();
    for mut r#fn in &*fns.clone() {
        let mut r#fn = r#fn.clone();
        let _ = (::match_deref::match_deref! { match &(r#fn.clone()) {
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::VARIABLE, direction: DAE::VarDirection::INPUT, componentRef: cr, .. } => {
            fnInputs = cons(cr.clone(), fnInputs.clone());
            ()
        },
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::VARIABLE, direction: DAE::VarDirection::OUTPUT, componentRef: cr, .. } if (!(Expression::isRecordType(ComponentReference::crefTypeFull(cr.clone())?)) && ComponentReference::crefDepth(cr.clone())? > 0) => {
            let mut crVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            (crVar, varLst, repl) = createReplacementVariables(cr.clone(), (funcname.clone()).clone(), repl.clone())?;
            outEqs = BackendVariable::addVarsDAE(varLst.clone(), outEqs.clone());
            oOutput = cons(crVar.clone(), oOutput.clone());
            ()
        },
        Deref @ DAE::Element::VAR { binding: None, protection: DAE::VarVisibility::PROTECTED, componentRef: cr, .. } if (!(Expression::isRecordType(ComponentReference::crefTypeFull(cr.clone())?))) => {
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            (_, varLst, repl) = createReplacementVariables(cr.clone(), (funcname.clone()).clone(), repl.clone())?;
            varLst = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut _var in (varLst.clone()).into_iter().cloned() {
            let __x = BackendVariable::setVarTS(_var.clone(), Some(crate::BackendDAE::TearingSelect::AVOID));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            outEqs = BackendVariable::addVarsDAE(varLst.clone(), outEqs.clone());
            ()
        },
        Deref @ DAE::Element::VAR { binding: Some(eBind), protection: DAE::VarVisibility::PROTECTED, componentRef: cr, .. } if (!(Expression::isRecordType(ComponentReference::crefTypeFull(cr.clone())?))) => {
            let mut crVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut eVar: Arc<DAE::Exp>;
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            (crVar, varLst, repl) = createReplacementVariables(cr.clone(), (funcname.clone()).clone(), repl.clone())?;
            eVar = Expression::crefExp(crVar.clone())?;
            varLst = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut _var in (varLst.clone()).into_iter().cloned() {
            let __x = BackendVariable::setVarTS(_var.clone(), Some(crate::BackendDAE::TearingSelect::AVOID));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            outEqs = BackendVariable::addVarsDAE(varLst.clone(), outEqs.clone());
            eq = BackendEquation::generateEquation(eVar.clone(), eBind.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
            outEqs = BackendEquation::equationAddDAE(eq.clone(), outEqs.clone())?;
            ()
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: st }, .. } => {
            eqlst = List::map(st.clone(), (std::sync::Arc::new(BackendEquation::statementEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<Arc<BackendDAE::Equation>> + 'static>));
            outEqs = BackendEquation::equationsAddDAE(eqlst.clone(), outEqs.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    oOutput = oOutput.clone().reverse();
    if BackendDAEUtil::systemSize(outEqs.clone()) != BackendVariable::daenumVariables(outEqs.clone())? {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("newBackendInline.createEqnSysfromFunction failed for function ")); __mm_s.push_str(&*funcname.clone()); __mm_s.push_str(&*literal!("with different sizes\n")); ArcStr::from(__mm_s) }).clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(outEqs.clone()))); __mm_s.push_str(&*literal!(" <> ")); __mm_s.push_str(&*intString(BackendVariable::daenumVariables(outEqs.clone())?)); ArcStr::from(__mm_s) }).clone());
        }
        bail!("fail");
    }
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\noriginal function body of: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
        BackendDump::printEqSystem(outEqs.clone())?;
        println!("{}", (literal!("\nDump replacements: ")).clone());
        BackendVarTransform::dumpReplacements(repl.clone())?;
    }
    assign_field!(outEqs.orderedEqs = BackendEquation::listEquation((InlineArrayEquations::getScalarArrayEqns(BackendEquation::equationList(outEqs.orderedEqs.clone()))?).0)?);
    outEqs = BackendVarTransform::performReplacementsEqSystem(outEqs.clone(), repl.clone())?;
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n replaced protected and output for: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
        BackendDump::printEqSystem(outEqs.clone())?;
    }
    argmap = List::zip(fnInputs.clone().reverse(), args.clone());
    (argmap, checkcr) = Inline::extendCrefRecords(argmap.clone(), HashTableCG::emptyHashTable())?;
    BackendDAEUtil::traverseBackendDAEExpsEqSystemWithUpdate(outEqs.clone(), (std::sync::Arc::new(replaceArgs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool))> + 'static>), (argmap.clone(), checkcr.clone(), true))?;
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nreplaced input arguments for: ")); __mm_s.push_str(&*funcname.clone()); ArcStr::from(__mm_s) }).clone());
        BackendDump::printEqSystem(outEqs.clone())?;
    }
    Ok((oOutput, outEqs))
}

fn addReplacement(mut iCr: Arc<DAE::ComponentRef>, mut iExp: Arc<DAE::Exp>, mut iRepl: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut oRepl: BackendVarTransform::VariableReplacements;
    oRepl = (::match_deref::match_deref! { match &((iCr.clone(), iExp.clone(), iRepl.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { identType: tp, .. }, _, _) if (!(Expression::isRecordType(tp.clone())) && !(Expression::isArrayType(tp.clone()))) => {
            BackendVarTransform::addReplacement(iRepl.clone(), iCr.clone(), iExp.clone(), None)?
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { identType: tp, .. }, _, _) if (Expression::isArrayType(tp.clone())) => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut arrExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut e: Arc<DAE::Exp>;
            crefs = ComponentReference::expandCref(iCr.clone(), false)?;
            repl = iRepl.clone();
            arrExp = Expression::getArrayOrRangeContents(iExp.clone())?;
            for mut c in &*crefs.clone() {
                let mut c = c.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(arrExp.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                arrExp = __pa1.clone();
                repl = BackendVarTransform::addReplacement(repl.clone(), c.clone(), e.clone(), None)?;
            }
            repl.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRepl)
}

fn replaceArgs(mut inExp: Arc<DAE::Exp>, mut inTuple: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool);
    (outExp, outTuple) = Expression::Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(Inline::replaceArgs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr)), bool))> + 'static>), inTuple.clone())?;
    if !(Util::tuple33(outTuple.clone())) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::traceln((literal!("BackendInline.replaceArgs failed")).clone())?;
        }
        bail!("fail");
    }
    Ok((outExp, outTuple))
}

