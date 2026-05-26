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

use crate::ComponentReference;
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::HashSet;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn checkModel(mut inDAELst: DAE::DAElist) -> Result<(i32, i32, i32)> {
    let mut varSize: i32 = 0;
    let mut eqnSize: i32 = 0;
    let mut simpleEqnSize: i32 = 0;
    let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    ExecStat::execStat((literal!("CheckModel - start counting")).clone())?;
    let DAE::DAE { elementLst: __pa0 } = (inDAELst.clone()) else { bail!("pattern mismatch") };
    lst = __pa0.clone();
    hs = HashSet::emptyHashSet();
    (varSize, eqnSize, eqns, hs) = List::fold(lst.clone(), Arc::new(countVarEqnSize), (0, 0, metamodelica::nil(), hs.clone()));
    simpleEqnSize = countSimpleEqnSize(eqns.clone(), 0, hs.clone());
    ExecStat::execStat((literal!("CheckModel - end counting")).clone())?;
    Ok((varSize, eqnSize, simpleEqnSize))
}

pub type CountVarEqnFoldArg = (i32, i32, Arc<metamodelica::List<Arc<DAE::Element>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));

/*varSize*/
/*eqnSize*/
/*eqns*/
/*vars*/
fn countVarEqnSize(mut element: Arc<DAE::Element>, mut inArg: CountVarEqnFoldArg) -> Result<CountVarEqnFoldArg> {
    let mut outArg: CountVarEqnFoldArg;
    outArg = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::PARAM, .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::CONST, .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::VAR { componentRef: cr, .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            size = Expression::sizeOf(var_field!((*element).ty, DAE::Element::VAR).clone())?;
            varSize = varSize.clone() + size.clone();
            if DAEUtil::isInput(element.clone()) && DAEUtil::isPublicVar(element.clone()) {
                eqnSize = eqnSize.clone() + size.clone();
            } else {
                if isSome(var_field!((*element).binding, DAE::Element::VAR).clone()) {
                    eqnSize = eqnSize.clone() + size.clone();
                    eqns = cons(Arc::new(DAE::Element::EQUATION { exp: Expression::crefExp(var_field!((*element).componentRef, DAE::Element::VAR).clone())?, scalar: Util::getOption(var_field!((*element).binding, DAE::Element::VAR).clone())?, source: var_field!((*element).source, DAE::Element::VAR).clone() }), eqns.clone());
                }
                hs = BaseHashSet::add(cr.clone(), hs.clone())?;
            }
            (varSize.clone(), eqnSize.clone(), eqns.clone(), hs.clone())
        },
        Deref @ DAE::Element::EQUATION { exp: e, .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            size = Expression::sizeOf(Expression::r#typeof(e.clone())?)?;
            (varSize.clone(), eqnSize.clone() + size.clone(), cons(element.clone(), eqns.clone()), hs.clone())
        },
        Deref @ DAE::Element::INITIALEQUATION { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            tp = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
            size = Expression::sizeOf(tp.clone())?;
            (varSize.clone(), eqnSize.clone() + size.clone(), cons(element.clone(), eqns.clone()), hs.clone())
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr, .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            tp = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
            size = Expression::sizeOf(tp.clone())?;
            (varSize.clone(), eqnSize.clone() + size.clone(), cons(element.clone(), eqns.clone()), hs.clone())
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e, .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            size = Expression::sizeOf(Expression::r#typeof(e.clone())?)?;
            (varSize.clone(), eqnSize.clone() + size.clone(), cons(element.clone(), eqns.clone()), hs.clone())
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            size = Expression::sizeOf(Expression::r#typeof(var_field!((*element).exp, DAE::Element::ARRAY_EQUATION).clone())?)?;
            (varSize.clone(), eqnSize.clone() + size.clone(), cons(element.clone(), eqns.clone()), hs.clone())
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: daeElts, .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            (_, size, _, _) = List::fold(daeElts.clone(), Arc::new(countVarEqnSize), (0, 0, metamodelica::nil(), hs.clone()));
            (varSize.clone(), eqnSize.clone() + size.clone(), eqns.clone(), hs.clone())
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            (_, size, _, _) = List::fold(var_field!((*element).equations, DAE::Element::INITIAL_FOR_EQUATION).clone(), Arc::new(countVarEqnSize), (0, 0, metamodelica::nil(), hs.clone()));
            size = size.clone() * Expression::sizeOf(Expression::r#typeof(var_field!((*element).range, DAE::Element::INITIAL_FOR_EQUATION).clone())?)?;
            (varSize.clone(), eqnSize.clone() + size.clone(), eqns.clone(), hs.clone())
        },
        Deref @ DAE::Element::FOR_EQUATION { .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            (_, size, _, _) = List::fold(var_field!((*element).equations, DAE::Element::FOR_EQUATION).clone(), Arc::new(countVarEqnSize), (0, 0, metamodelica::nil(), hs.clone()));
            size = size.clone() * Expression::sizeOf(Expression::r#typeof(var_field!((*element).range, DAE::Element::FOR_EQUATION).clone())?)?;
            (varSize.clone(), eqnSize.clone() + size.clone(), eqns.clone(), hs.clone())
        },
        Deref @ DAE::Element::IF_EQUATION { equations3: Deref @ metamodelica::List::Nil, condition1: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::IF_EQUATION { equations2: Deref @ metamodelica::List::Cons { head: daeElts, tail: _ }, .. } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            (_, size, _, _) = List::fold(daeElts.clone(), Arc::new(countVarEqnSize), (0, 0, metamodelica::nil(), hs.clone()));
            (varSize.clone(), eqnSize.clone() + size.clone(), eqns.clone(), hs.clone())
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { equations3: Deref @ metamodelica::List::Nil, condition1: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::ALGORITHM { source, algorithm_: alg } => {
            let mut varSize: i32 = 0;
            let mut eqnSize: i32 = 0;
            let mut size: i32 = 0;
            let mut eqns: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            (varSize, eqnSize, eqns, hs) = inArg.clone();
            crlst = checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
            size = (crlst.clone().len() as i32);
            (varSize.clone(), eqnSize.clone() + size.clone(), eqns.clone(), hs.clone())
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::COMP { dAElist: daeElts, .. } => {
            List::fold(daeElts.clone(), Arc::new(countVarEqnSize), inArg.clone())
        },
        Deref @ DAE::Element::REINIT { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::CONSTRAINT { .. } => {
            inArg.clone()
        },
        Deref @ DAE::Element::FLAT_SM { dAElist: daeElts, .. } => {
            List::fold(daeElts.clone(), Arc::new(countVarEqnSize), inArg.clone())
        },
        Deref @ DAE::Element::SM_COMP { dAElist: daeElts, .. } => {
            List::fold(daeElts.clone(), Arc::new(countVarEqnSize), inArg.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CheckModel.countVarEqnSize failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![element.clone()])?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub fn checkAndGetAlgorithmOutputs(mut inAlgorithm: Arc<DAE::Algorithm>, mut inSource: Arc<DAE::ElementSource>, mut inCrefExpansionRule: DAE::Expand) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outCrefLst = 'mc: {
        let __mc_input = (inAlgorithm.clone(), inSource.clone(), inCrefExpansionRule.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::ElementSource { instance: Deref @ DAE::ComponentPrefix::NOCOMPPRE, .. }, _) => {
                    Ok(algorithmOutputs(inAlgorithm.clone(), inCrefExpansionRule.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::ElementSource { .. }, _) => {
                    Ok(if (PrefixUtil::hasSubs(inSource.instance.clone())) {algorithmOutputs(inAlgorithm.clone(), openmodelica_frontend_types::DAE::Expand::NOT_EXPAND)?} else {algorithmOutputs(inAlgorithm.clone(), inCrefExpansionRule.clone())?})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("checkAndGetAlgorithmOutputs failed.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefLst)
}

pub fn isCrefListAlgorithmOutput(mut crefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inAlgorithm: Arc<DAE::Algorithm>, mut inSource: Arc<DAE::ElementSource>, mut inCrefExpansionRule: DAE::Expand) -> Result<bool> {
    let mut outResult: bool = false;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = HashSet::emptyHashSet();
    let mut algOutCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    algOutCrefs = checkAndGetAlgorithmOutputs(inAlgorithm.clone(), inSource.clone(), inCrefExpansionRule.clone())?;
    ht = List::fold(algOutCrefs.clone(), Arc::new(BaseHashSet::add), ht.clone());
    for mut cr in &*crefList.clone() {
        let mut cr = cr.clone();
        if !(BaseHashSet::has(cr.clone(), ht.clone())?) {
            return Ok(outResult);
        }
    }
    outResult = true;
    Ok(outResult)
}

fn algorithmOutputs(mut inAlgorithm: Arc<DAE::Algorithm>, mut inCrefExpansion: DAE::Expand) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ DAE::Algorithm { statementLst: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts = __pa0.clone();
    outCrefLst = algorithmStatementListOutputs(stmts.clone(), inCrefExpansion.clone())?;
    Ok(outCrefLst)
}

pub fn algorithmStatementListOutputs(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCrefExpansion: DAE::Expand) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    hs = HashSet::emptyHashSet();
    hs = List::fold1(inStmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), hs.clone());
    outCrefLst = BaseHashSet::hashSetList(hs.clone())?;
    Ok(outCrefLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn statementOutputs(mut inStatement: Arc<DAE::Statement>, mut inCrefExpansion: DAE::Expand, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut oht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    oht = 'mc: {
        let __mc_input = (inStatement.clone(), inCrefExpansion.clone(), iht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { exp1, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let (_, (_, __pa0)) = Expression::traverseExpTopDown(exp1.clone(), Arc::new(statementOutputsCrefFinder), (inCrefExpansion.clone(), iht.clone()))?;
                    ht = __pa0.clone();
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expl, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let (_, (_, __pa0)) = Expression::traverseExpListTopDown(expl.clone(), Arc::new(statementOutputsCrefFinder), (inCrefExpansion.clone(), iht.clone()))?;
                    ht = __pa0.clone();
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp1, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    cr = Expression::expCref(exp1.clone())?;
                    subs = ComponentReference::crefLastSubs(cr.clone())?;
                    if !(subs.clone().is_empty()) {
                        subs = List::fill(Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM), (subs.clone().len() as i32));
                        cr = ComponentReference::crefSetLastSubs(cr.clone(), subs.clone())?;
                    }
                    crlst = ComponentReference::expandCref(cr.clone(), true)?;
                    ht = List::fold(crlst.clone(), Arc::new(BaseHashSet::add), iht.clone());
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { else_: elsebranch, statementLst: stmts, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), iht.clone());
                    ht = statementElseOutputs(elsebranch.clone(), inCrefExpansion.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { statementLst: stmts, range: e, iter: iteratorName, type_: tp, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut stmts = (*stmts).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent((iteratorName.clone()).clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (Expression::replaceCref, (cr.clone(), e.clone())));
                    ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), openmodelica_frontend_types::DAE::Expand::EXPAND, iht.clone());
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_PARFOR { statementLst: stmts, range: e, iter: iteratorName, type_: tp, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut stmts = (*stmts).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent((iteratorName.clone()).clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (Expression::replaceCref, (cr.clone(), e.clone())));
                    ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), openmodelica_frontend_types::DAE::Expand::EXPAND, iht.clone());
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { statementLst: stmts, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), iht.clone());
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst: stmts, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), iht.clone());
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(stmt), statementLst: stmts, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), iht.clone());
                    ht = statementOutputs(stmt.clone(), inCrefExpansion.clone(), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { .. }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TERMINATE { .. }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { .. }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { .. }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_RETURN { source: _ }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_BREAK { source: _ }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_CONTINUE { source: _ }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ARRAY_INIT { .. }, _, _) => {
                    Ok(iht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, _, _) => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), iht.clone());
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = (DAEDump::ppStatementStr(inStatement.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CheckModel.statementOutputs failed for ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oht)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn statementElseOutputs(mut inElseBranch: Arc<DAE::Else>, mut inCrefExpansion: DAE::Expand, mut iht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut oht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    oht = (::match_deref::match_deref! { match &((inElseBranch.clone(), inCrefExpansion.clone(), iht.clone())) {
        (Deref @ DAE::Else::NOELSE, _, _) => {
            iht.clone()
        },
        (Deref @ DAE::Else::ELSEIF { else_: elseBranch, statementLst: stmts, .. }, _, _) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), iht.clone());
            ht = statementElseOutputs(elseBranch.clone(), inCrefExpansion.clone(), ht.clone())?;
            ht.clone()
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, _, _) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            ht = List::fold1(stmts.clone(), Arc::new(statementOutputs), inCrefExpansion.clone(), iht.clone());
            ht.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oht)
}

fn statementOutputsCrefFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(Arc<DAE::Exp>, bool, (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, componentRef: cr }, (expand, ht)) => {
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefStripSubs(cr.clone())?;
                    crlst = ComponentReference::expandCref(cr.clone(), true)?;
                    (ht, _, _, _, _) = List::fold(crlst.clone(), Arc::new(BaseHashSet::add), ht.clone());
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (expand @ DAE::Expand::NOT_EXPAND, ht)) => {
                    let mut ht = (*ht).clone();
                    (ht, _, _, _, _) = List::fold(list![cr.clone()], Arc::new(BaseHashSet::add), ht.clone());
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (expand, ht)) => {
                    let mut first_cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut cr = (*cr).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefStripSubsExceptModelSubs(cr.clone());
                    first_cref = ComponentReference::crefArrayGetFirstCref(cr.clone())?;
                    if !(BaseHashSet::has(first_cref.clone(), ht.clone())?) {
                        crlst = ComponentReference::expandCref(cr.clone(), true)?;
                        (ht, _, _, _, _) = List::fold(crlst.clone(), Arc::new(BaseHashSet::add), ht.clone());
                    }
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ASUB { exp, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(exp.clone(), Arc::new(statementOutputsCrefFinder), inTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::TSUB { exp, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(exp.clone(), Arc::new(statementOutputsCrefFinder), inTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expElse: e2, expThen: e1, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(e1.clone(), Arc::new(statementOutputsCrefFinder), inTpl.clone())?;
                    (_, outTpl) = Expression::traverseExpTopDown(e2.clone(), Arc::new(statementOutputsCrefFinder), outTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    Ok((e.clone(), true, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

fn countSimpleEqnSize(mut inEqns: Arc<metamodelica::List<Arc<DAE::Element>>>, mut isimpleEqnSize: i32, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> i32 {
    let mut osimpleEqnSize: i32 = 0;
    osimpleEqnSize = List::applyAndFold1(inEqns.clone(), Arc::new(fnptr!(intAdd, i32, i32)), Arc::new(countSimpleEqnSizeWork), ihs.clone(), 0);
    osimpleEqnSize
}

fn countSimpleEqnSizeWork(mut inEqns: Arc<DAE::Element>, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<i32> {
    let mut osimpleEqnSize: i32 = 0;
    osimpleEqnSize = 'mc: {
        let __mc_input = (inEqns.clone(), ihs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::EQUATION { scalar: e2, exp: e1, .. }, _) => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. }, _) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    tp = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
                    Ok(Expression::sizeOf(tp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::DEFINE { exp: e2, componentRef: cr, .. }, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(cr.clone())?;
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::COMPLEX_EQUATION { rhs: e2, lhs: e1, .. }, _) => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::ARRAY_EQUATION { array: e2, exp: e1, .. }, _) => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(osimpleEqnSize)
}

fn simpleEquation(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<i32> {
    let mut osimpleEqnSize: i32 = 0;
    osimpleEqnSize = 'mc: {
        let __mc_input = (e1.clone(), e2.clone(), ihs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, _, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { .. }, _) => {
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::sizeOf(Expression::r#typeof(e2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut ea1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ea2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let true = (Expression::isArray(e1.clone()) || Expression::isMatrix(e1.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isArray(e2.clone()) || Expression::isMatrix(e2.clone())) else { bail!("pattern mismatch") };
                    ea1 = Expression::flattenArrayExpToList(e1.clone())?;
                    ea2 = Expression::flattenArrayExpToList(e2.clone())?;
                    Ok(simpleEquations(ea1.clone(), ea2.clone(), 0, ihs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(Expression::expSub(e1.clone(), e2.clone())?, Arc::new(traversingComponentRefFinder), (ihs.clone(), metamodelica::nil()))?) {
                        (_, (_, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil })) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(Expression::sizeOf(Expression::r#typeof(e1.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(osimpleEqnSize)
}

fn traversingComponentRefFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _) => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (hs, crefs)) => {
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefs = (*crefs).clone();
                    crlst = ComponentReference::expandCref(cr.clone(), true)?;
                    crefs = getcr(crlst.clone(), hs.clone(), crefs.clone())?;
                    Ok((e.clone(), (hs.clone(), crefs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getcr(mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut iAcc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut oAcc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    oAcc = (::match_deref::match_deref! { match &((crefs.clone(), hs.clone(), iAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            iAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, _, _) if (BaseHashSet::has(cr.clone(), hs.clone())?) => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crlst = List::unionEltOnTrue(cr.clone(), iAcc.clone(), Arc::new(ComponentReferenceBasics::crefEqual));
            getcr(rest.clone(), hs.clone(), crlst.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _) => {
            getcr(rest.clone(), hs.clone(), iAcc.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oAcc)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simpleEquations(mut e1lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut e2lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut isimpleEqnSize: i32, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<i32> {
    let mut osimpleEqnSize: i32 = 0;
    osimpleEqnSize = (::match_deref::match_deref! { match &((e1lst.clone(), e2lst.clone(), isimpleEqnSize.clone(), ihs.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            isimpleEqnSize.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: r1 }, Deref @ metamodelica::List::Cons { head: e2, tail: r2 }, _, _) => {
            let mut size: i32 = 0;
            size = simpleEquation(e1.clone(), e2.clone(), ihs.clone())?;
            simpleEquations(r1.clone(), r2.clone(), size.clone() + isimpleEqnSize.clone(), ihs.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osimpleEqnSize)
}

