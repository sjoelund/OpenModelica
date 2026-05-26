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
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::RemoveSimpleEquations;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashSetExp;
use openmodelica_frontend::HashTable2;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// TODO:
// - evaluation of for-loops
// - evaluation of while-loops
// - evaluation of xOut := funcCall1(funcCall2(xIn[1]));  with funcCall2(xIn[1]) = xIn[1,2] for example have a look at Media.Examples.ReferenceAir.MoistAir
// - evaluation of BackendDAE.ARRAY_EQUATION
// =============================================================================
// =============================================================================
// type definitions
//
// =============================================================================
/// store informations when traversing the statements and evaluate the function calls
#[derive(Clone, Debug, PartialEq)]
pub struct FuncInfo {
    pub repl: BackendVarTransform::VariableReplacements,
    pub funcTree: /* ? */,
    pub idx: i32,
}

pub type FUNCINFO = FuncInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variability {
    CONST,
    VARIABLE,
}
pub use self::Variability::{CONST,VARIABLE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallSignature {
    pub path: Arc<Absyn::Path>,
    pub inputsVari: Arc<metamodelica::List<Variability>>,
    pub canBeEvaluated: bool,
}

pub type SIGNATURE = CallSignature;


// =============================================================================
// caching of already evaluated functions
//
// =============================================================================
fn checkCallSignatureForExp(mut expIn: Arc<DAE::Exp>, mut signLst: Arc<metamodelica::List<CallSignature>>) -> Result<bool> {
    let mut continueEval: bool = false;
    let mut signature: CallSignature;
    continueEval = true;
    signature = getCallSignatureForCall(expIn.clone())?;
    if List::isMemberOnTrue(signature.clone(), signLst.clone(), Arc::new(callSignatureIsEqual)) {
        let CallSignature { canBeEvaluated: __pa0, .. } = (List::getMemberOnTrue(signature.clone(), signLst.clone(), Arc::new(callSignatureIsEqual))?) else { bail!("pattern mismatch") };
        continueEval = __pa0.clone();
    }
    Ok(continueEval)
}

fn callSignatureStr(mut signat: CallSignature) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut path: Arc<Absyn::Path>;
    let mut varis: Arc<metamodelica::List<Variability>> = metamodelica::nil();
    let mut b: bool = false;
    let CallSignature { canBeEvaluated: __pa0, inputsVari: __pa1, path: __pa2 } = (signat.clone()) else { bail!("pattern mismatch") };
    b = __pa0.clone();
    varis = __pa1.clone();
    path = __pa2.clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("[ ")); __mm_s.push_str(&*stringDelimitList(List::map(varis.clone(), Arc::new(fnptr!(VariabilityString, Variability))), (literal!(" | ")).clone())); __mm_s.push_str(&*literal!(" ] ")); __mm_s.push_str(&*boolString(b.clone())); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn VariabilityString(mut var: Variability) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match var.clone() {
        Variability::CONST { .. } => literal!("CONST"),
        _ => literal!("VARIABLE"),
    })).clone();
    r#str
}

fn callSignatureIsEqual(mut signat1: CallSignature, mut signat2: CallSignature) -> Result<bool> {
    let mut isEqual: bool = false;
    let mut path1: Arc<Absyn::Path>;
    let mut path2: Arc<Absyn::Path>;
    let mut vari1: Arc<metamodelica::List<Variability>> = metamodelica::nil();
    let mut vari2: Arc<metamodelica::List<Variability>> = metamodelica::nil();
    let CallSignature { inputsVari: __pa0, path: __pa1, .. } = (signat1.clone()) else { bail!("pattern mismatch") };
    vari1 = __pa0.clone();
    path1 = __pa1.clone();
    let CallSignature { inputsVari: __pa2, path: __pa3, .. } = (signat2.clone()) else { bail!("pattern mismatch") };
    vari2 = __pa2.clone();
    path2 = __pa3.clone();
    isEqual = false;
    if AbsynUtil::pathEqual(path1.clone(), path2.clone()) {
        if List::isEqualOnTrue(vari1.clone(), vari2.clone(), Arc::new(fnptr!(VariabilityIsEqual, Variability, Variability))) {
            isEqual = true;
        }
    }
    Ok(isEqual)
}

fn VariabilityIsEqual(mut vari1: Variability, mut vari2: Variability) -> bool {
    let mut isEqual: bool = false;
    isEqual = (match (vari1.clone(), vari2.clone()) {
        (Variability::CONST { .. }, Variability::CONST { .. }) => true,
        (Variability::VARIABLE { .. }, Variability::VARIABLE { .. }) => true,
        _ => false,
    });
    isEqual
}

fn getCallSignatureForCall(mut callExpIn: Arc<DAE::Exp>) -> Result<CallSignature> {
    let mut signatureOut: CallSignature;
    let mut path: Arc<Absyn::Path>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut vari: Arc<metamodelica::List<Variability>> = metamodelica::nil();
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(callExpIn.clone()) {
            Deref @ DAE::Exp::CALL { expLst: __pa1, path: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expLst = __pa1.clone();
        path = __pa2.clone();
        vari = List::map(expLst.clone(), Arc::new(getVariabilityForExp));
        signatureOut = CallSignature { path: path.clone(), inputsVari: vari.clone(), canBeEvaluated: true };
        Ok::<_, anyhow::Error>((expLst.clone(), path.clone(), signatureOut.clone(), vari.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            expLst = __try0_o0;
            path = __try0_o1;
            signatureOut = __try0_o2;
            vari = __try0_o3;
        }
        Err(_) => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evalFunc.getCallSignatureForCall failed for :\n")); __mm_s.push_str(&*ExpressionBasics::printExpStr(callExpIn.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail");
        }
    }
    Ok(signatureOut)
}

fn getVariabilityForExp(mut expIn: Arc<DAE::Exp>) -> Result<Variability> {
    let mut variOut: Variability = Variability::CONST;
    variOut = (::match_deref::match_deref! { match &(expIn.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            crate::EvaluateFunctions::Variability::CONST
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            crate::EvaluateFunctions::Variability::CONST
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            crate::EvaluateFunctions::Variability::CONST
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            crate::EvaluateFunctions::Variability::CONST
        },
        Deref @ DAE::Exp::CLKCONST { .. } => {
            crate::EvaluateFunctions::Variability::CONST
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            crate::EvaluateFunctions::Variability::CONST
        },
        Deref @ DAE::Exp::CREF { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::BINARY { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::UNARY { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::LBINARY { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::LUNARY { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::RELATION { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::IFEXP { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::CALL { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::RECORD { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::ARRAY { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::MATRIX { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::TUPLE { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::CAST { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::ASUB { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::TSUB { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::RSUB { .. } => {
            let mut vari: Variability = Variability::CONST;
            if Expression::isConst(expIn.clone())? {
                vari = crate::EvaluateFunctions::Variability::CONST;
            } else {
                vari = crate::EvaluateFunctions::Variability::VARIABLE;
            }
            vari.clone()
        },
        Deref @ DAE::Exp::SIZE { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::CODE { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::EMPTY { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        _ => {
            crate::EvaluateFunctions::Variability::VARIABLE
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(variOut)
}

// =============================================================================
// evaluate functions
//
// =============================================================================
pub fn evalFunctions(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Arc<BackendDAE::BackendDAE> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut changed: bool = false;
    let mut eqSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inDAE.clone()) {
            Deref @ DAE { shared: __pa1, eqs: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        shared = __pa1.clone();
        eqSysts = __pa2.clone();
        let (__pa3, (__pa4, _, __pa5, _)) = List::mapFold(eqSysts.clone(), Arc::new(evalFunctions_main), (shared.clone(), 1, false, metamodelica::nil()));
        eqSysts = __pa3.clone();
        shared = __pa4.clone();
        changed = __pa5.clone();
        if changed.clone() {
            outDAE = unwrap_break_err!(updateVarKinds(RemoveSimpleEquations::fastAcausal(BackendDAE::DAE(eqSysts.clone(), shared.clone()).unwrap()).unwrap()), '__try0);
        } else {
            outDAE = inDAE.clone();
        }
        Ok::<_, anyhow::Error>((outDAE.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDAE = __try0_o0;
        }
        Err(_) => {
            outDAE = inDAE.clone();
        }
    }
    outDAE
}

fn evalFunctions_main(mut eqSysIn: Arc<BackendDAE::EqSystem>, mut tplIn: (Arc<BackendDAE::Shared>, i32, bool, Arc<metamodelica::List<CallSignature>>)) -> Result<(Arc<BackendDAE::EqSystem>, (Arc<BackendDAE::Shared>, i32, bool, Arc<metamodelica::List<CallSignature>>))> {
    let mut eqSysOut: Arc<BackendDAE::EqSystem>;
    let mut tplOut: (Arc<BackendDAE::Shared>, i32, bool, Arc<metamodelica::List<CallSignature>>);
    let mut changed: bool = false;
    let mut sysIdx: i32 = 0;
    let mut sharedIn: Arc<BackendDAE::Shared>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut callSign: Arc<metamodelica::List<CallSignature>> = metamodelica::nil();
    let mut recursion_limit: i32 = 0;
    (sharedIn, sysIdx, changed, callSign) = tplIn.clone();
    let __pa0 = ::match_deref::match_deref! { match &(eqSysIn.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    eqLst = BackendEquation::equationList(eqs.clone());
    recursion_limit = Flags::getConfigInt(Flags::EVAL_RECURSION_LIMIT.clone())?;
    (eqLst, shared, addEqs, _, changed, callSign) = List::mapFold5(eqLst.clone(), Arc::new({ let __pe_b6 = recursion_limit.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5| evalFunctions_findFuncs(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_b6.clone()) }), sharedIn.clone(), metamodelica::nil(), 1, changed.clone(), callSign.clone());
    eqs = BackendEquation::listEquation(listAppend(eqLst.clone(), addEqs.clone()))?;
    eqSysOut = BackendDAEUtil::setEqSystEqs(eqSysIn.clone(), eqs.clone());
    tplOut = (shared.clone(), sysIdx.clone() + 1, changed.clone(), callSign.clone());
    Ok((eqSysOut, tplOut))
}

fn evalFunctions_findFuncs(mut eqIn: Arc<BackendDAE::Equation>, mut shared: Arc<BackendDAE::Shared>, mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut idx: i32, mut changed: bool, mut callSign: Arc<metamodelica::List<CallSignature>>, mut recursionLimit: i32) -> Result<(Arc<BackendDAE::Equation>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, i32, bool, Arc<metamodelica::List<CallSignature>>)> {
    let mut eqIn: Arc<BackendDAE::Equation> = eqIn;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = addEqs;
    let mut idx: i32 = idx;
    let mut changed: bool = changed;
    let mut callSign: Arc<metamodelica::List<CallSignature>> = callSign;
    eqIn = 'mc: {
        let __mc_input = eqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { attr, source, scalar: exp2, exp: exp1 } => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut changed1: bool = false;
                    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut lhsExp: Arc<DAE::Exp>;
                    let mut rhsExp: Arc<DAE::Exp>;
                    let mut funcs; // TODO: local with unresolved type
                    let mut addEqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut addEqs2: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut idx: i32 = idx.clone();
                    let mut callSign: Arc<metamodelica::List<CallSignature>> = callSign.clone();
                    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = addEqs.clone();
                    let mut changed: bool = changed.clone();
                    b1 = Expression::containFunctioncall(exp1.clone());
                    b2 = Expression::containFunctioncall(exp2.clone());
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    funcs = BackendDAEUtil::getFunctions(shared.clone())?;
                    if b1.clone() {
                        (rhsExp, lhsExp, addEqs1, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp1.clone(), exp2.clone(), funcs.clone(), idx.clone(), callSign.clone(), recursionLimit.clone())?;
                        changed = changed.clone() || changed1.clone();
                        addEqs = listAppend(addEqs1.clone(), addEqs.clone());
                    }
                    if b2.clone() {
                        (rhsExp, lhsExp, addEqs2, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp2.clone(), exp1.clone(), funcs.clone(), idx.clone(), callSign.clone(), recursionLimit.clone())?;
                        changed = changed.clone() || changed1.clone();
                        addEqs = listAppend(addEqs2.clone(), addEqs.clone());
                    }
                    eq = BackendEquation::generateEquation(lhsExp.clone(), rhsExp.clone(), source.clone(), attr.clone())?;
                    idx = idx.clone() + 1;
                    Ok(eq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", (literal!("this is an array equation. update evalFunctions_findFuncs\n")).clone());
                    }
                    Ok(eqIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr, source, right: exp2, left: exp1, .. } => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut changed1: bool = false;
                    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut lhsExp: Arc<DAE::Exp>;
                    let mut rhsExp: Arc<DAE::Exp>;
                    let mut funcs; // TODO: local with unresolved type
                    let mut addEqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut idx: i32 = idx.clone();
                    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = addEqs.clone();
                    let mut changed: bool = changed.clone();
                    let mut shared: Arc<BackendDAE::Shared> = shared.clone();
                    let mut callSign: Arc<metamodelica::List<CallSignature>> = callSign.clone();
                    b1 = Expression::containFunctioncall(exp1.clone());
                    b2 = Expression::containFunctioncall(exp2.clone());
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    funcs = BackendDAEUtil::getFunctions(shared.clone())?;
                    if b1.clone() {
                        (rhsExp, lhsExp, addEqs1, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp1.clone(), exp2.clone(), funcs.clone(), idx.clone(), callSign.clone(), recursionLimit.clone())?;
                        changed = changed.clone() || changed1.clone();
                        addEqs = listAppend(addEqs1.clone(), addEqs.clone());
                    }
                    if b2.clone() {
                        (rhsExp, lhsExp, addEqs1, funcs, idx, changed1, callSign) = evaluateConstantFunction(exp2.clone(), exp1.clone(), funcs.clone(), idx.clone(), callSign.clone(), recursionLimit.clone())?;
                        changed = changed.clone() || changed1.clone();
                        addEqs = listAppend(addEqs1.clone(), addEqs.clone());
                    }
                    shared = BackendDAEUtil::setSharedFunctionTree(shared.clone(), funcs.clone());
                    eq = BackendEquation::generateEquation(lhsExp.clone(), rhsExp.clone(), source.clone(), attr.clone())?;
                    (eq, addEqs) = convertTupleEquations(eq.clone(), addEqs.clone())?;
                    idx = idx.clone() + 1;
                    Ok(eq.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(eqIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((eqIn, shared, addEqs, idx, changed, callSign))
}

fn hasUnknownType(mut eIn: Arc<DAE::Exp>) -> bool {
    let mut bOut: bool = false;
    bOut = (::match_deref::match_deref! { match &(eIn.clone()) {
        Deref @ DAE::Exp::TUPLE { PR: eLst } => {
            List::any(eLst.clone(), Arc::new(fnptr!(hasUnknownType, Arc<DAE::Exp>)))
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_UNKNOWN, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bOut
}

fn hasMultipleArrayDimensions(mut eIn: Arc<DAE::Exp>) -> Result<bool> {
    let mut bOut: bool = false;
    bOut = (::match_deref::match_deref! { match &(eIn.clone()) {
        Deref @ DAE::Exp::TUPLE { PR: eLst } => {
            List::any(eLst.clone(), Arc::new(hasMultipleArrayDimensions))
        },
        Deref @ DAE::Exp::CREF { ty, .. } => {
            let mut b: bool = false;
            if Types::isArray(ty.clone()) {
                b = intNe(1, (Types::getDimensionSizes(ty.clone())?.len() as i32));
            } else {
                b = false;
            }
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bOut)
}

fn doNotInline(mut func: DAE::Function) -> bool {
    let mut dontInline: bool = false;
    dontInline = (match func.clone() {
        DAE::Function::FUNCTION { inlineType: DAE::InlineType::NO_INLINE, .. } => true,
        _ => false,
    });
    dontInline
}

fn expandComplexElementsToCrefs(mut e: Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut eLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    eLst = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let false = (isNotComplexVar(e.clone())?) else { bail!("pattern mismatch") };
                    lst = getScalarsForComplexVar(e.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cref = DAEUtil::varCref(e.clone())?;
                    Ok(list![cref.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eLst)
}

fn hasAssertFold(mut stmt: Arc<DAE::Element>, mut bIn: bool) -> bool {
    let mut bOut: bool = false;
    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut stmtLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    match '__try0: {
        stmtLst = unwrap_break_err!(DAEUtil::getStatement(stmt.clone()), '__try0);
        bLst = List::map(stmtLst.clone(), Arc::new(fnptr!(DAEUtil::isStmtAssert, Arc<DAE::Statement>)));
        bOut = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), bIn.clone());
        Ok::<_, anyhow::Error>((bOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bOut = __try0_o0;
        }
        Err(_) => {
            bOut = false;
        }
    }
    bOut
}

fn hasReturnFold(mut stmt: Arc<DAE::Element>, mut bIn: bool) -> bool {
    let mut bOut: bool = false;
    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut stmtLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    match '__try0: {
        stmtLst = unwrap_break_err!(DAEUtil::getStatement(stmt.clone()), '__try0);
        bLst = List::map(stmtLst.clone(), Arc::new(fnptr!(DAEUtil::isStmtReturn, Arc<DAE::Statement>)));
        bOut = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), bIn.clone());
        Ok::<_, anyhow::Error>((bOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bOut = __try0_o0;
        }
        Err(_) => {
            bOut = false;
        }
    }
    bOut
}

fn hasReinitFold(mut stmt: Arc<DAE::Element>, mut bIn: bool) -> bool {
    let mut bOut: bool = false;
    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut stmtLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    match '__try0: {
        stmtLst = unwrap_break_err!(DAEUtil::getStatement(stmt.clone()), '__try0);
        bLst = List::map(stmtLst.clone(), Arc::new(fnptr!(DAEUtil::isStmtReturn, Arc<DAE::Statement>)));
        bOut = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), bIn.clone());
        Ok::<_, anyhow::Error>((bOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bOut = __try0_o0;
        }
        Err(_) => {
            bOut = false;
        }
    }
    bOut
}

fn setRecordTypes(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst, .. } => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp1: Arc<DAE::Exp>;
                    let true = (Expression::isCall(inExp.clone())) else { bail!("pattern mismatch") };
                    let true = ((expLst.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    exp1 = listHead(expLst.clone())?;
                    cref = Expression::expCref(exp1.clone())?;
                    exp1 = Expression::makeCrefExp(cref.clone(), ty.clone())?;
                    Ok(exp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expLst } => {
                    let mut expLst = (*expLst).clone();
                    expLst = List::map(expLst.clone(), Arc::new(setRecordTypes));
                    Ok(Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() }))
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

pub fn getCrefsForRecord(mut e: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    es = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crefs = ComponentReference::expandCref(cref.clone(), true)?;
            expLst = List::map(crefs.clone(), Arc::new(Expression::crefExp));
            expLst.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(es)
}

fn scalarRecExpForOneDimRec(mut expIn: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut expOut: Arc<DAE::Exp>;
    expOut = 'mc: {
        let __mc_input = expIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { .. }, .. }, componentRef: cref } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut cref = (*cref).clone();
                    let true = ((varLst.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    crefs = getRecordScalars(cref.clone());
                    let true = ((crefs.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    cref = listHead(crefs.clone())?;
                    exp = Expression::crefExp(cref.clone())?;
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(expIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(expOut)
}

fn scalarRecCrefsForOneDimRec(mut crefIn: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut crefOut: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefOut = 'mc: {
        let __mc_input = crefIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    crefs = getRecordScalars(crefIn.clone());
                    let true = ((crefs.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    cref = listHead(crefs.clone())?;
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(crefIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(crefOut)
}

fn partiallyConstantArrayNeedsExpansion(mut allOutputCrefsIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> {
    let mut bOut: bool = false;
    for mut cref in &*allOutputCrefsIn.clone() {
        let mut cref = cref.clone();
        if Types::isArray(ComponentReference::crefType(cref.clone())?) {
            if List::isMemberOnTrue(cref.clone(), constScalarCrefs.clone(), Arc::new(fnptr!(ComponentReferenceBasics::crefEqualWithoutSubs, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>))) {
                bOut = true;
            }
        }
    }
    Ok(bOut)
}

fn buildVariableFunctionParts(mut scalarOutputs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, mut constComplexCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut varComplexCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut varScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut allOutputs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut lhsExpIn: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outputExpOut: Arc<DAE::Exp>;
    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut pos: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut lhsCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut outputExp: Arc<DAE::Exp>;
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let mut varScalarCrefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outputSCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut allOutputCrefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut protCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut protSCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut funcSOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut funcSProts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (varOutputs, outputExpOut, varScalarCrefsInFunc) = 'mc: {
        let __mc_input = (scalarOutputs.clone(), constComplexCrefs.clone(), varComplexCrefs.clone(), constScalarCrefs.clone(), varScalarCrefs.clone(), allOutputs.clone(), lhsExpIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = varOutputs.clone();
                    let mut pos: Arc<metamodelica::List<i32>> = pos.clone();
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    let mut protCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = protCrefs.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefsInFunc.clone();
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    varScalarCrefsInFunc = metamodelica::nil();
                    allOutputCrefs = List::map(allOutputs.clone(), Arc::new(DAEUtil::varCref));
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(constComplexCrefs.clone(), allOutputCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual))?;
                    pos = List::map1(outputCrefs.clone(), Arc::new(List::position), allOutputCrefs.clone());
                    varScalarExps = List::map1(pos.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    outputExp = if (List::hasOneElement(varScalarExps.clone())) {listHead(varScalarExps.clone())?} else {Arc::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    funcOutputs = List::map2(outputCrefs.clone(), Arc::new(generateOutputElements), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(protCrefs.clone(), Arc::new(generateProtectedElements), allOutputs.clone(), lhsExpIn.clone());
                    varOutputs = listAppend(funcOutputs.clone(), funcProts.clone());
                    Ok((varOutputs.clone(), outputExp.clone(), varScalarCrefsInFunc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, Deref @ DAE::Exp::LBINARY { .. }) => {
                    Ok((metamodelica::nil(), lhsExpIn.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = varOutputs.clone();
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    let mut pos: Arc<metamodelica::List<i32>> = pos.clone();
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefsInFunc.clone();
                    let mut allOutputCrefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs2.clone();
                    let mut protCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = protCrefs.clone();
                    allOutputCrefs = List::map(allOutputs.clone(), Arc::new(DAEUtil::varCref));
                    allOutputCrefs2 = List::map(allOutputCrefs.clone(), Arc::new(scalarRecCrefsForOneDimRec));
                    (_, _, varScalarCrefsInFunc) = List::intersection1OnTrue(allOutputCrefs.clone(), allOutputCrefs2.clone(), Arc::new(ComponentReferenceBasics::crefEqual))?;
                    allOutputCrefs = allOutputCrefs2.clone();
                    if partiallyConstantArrayNeedsExpansion(allOutputCrefs.clone(), constScalarCrefs.clone())? {
                        if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                            println!("{}", (literal!("A partially constant array needs expansion. Thats not supported.\n")).clone());
                        }
                        bail!("fail");
                    }
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(listAppend(constComplexCrefs.clone(), constScalarCrefs.clone()), allOutputCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual))?;
                    funcOutputs = List::map2(outputCrefs.clone(), Arc::new(generateOutputElements), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(protCrefs.clone(), Arc::new(generateProtectedElements), allOutputs.clone(), lhsExpIn.clone());
                    varOutputs = listAppend(funcOutputs.clone(), funcProts.clone());
                    pos = List::map1(outputCrefs.clone(), Arc::new(List::position), allOutputCrefs.clone());
                    varScalarExps = List::map1(pos.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    varScalarExps = List::map(varScalarExps.clone(), Arc::new(scalarRecExpForOneDimRec));
                    outputExp = if (List::hasOneElement(varScalarExps.clone())) {listHead(varScalarExps.clone())?} else {Arc::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    Ok((varOutputs.clone(), outputExp.clone(), varScalarCrefsInFunc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefsInFunc.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = varOutputs.clone();
                    let mut pos: Arc<metamodelica::List<i32>> = pos.clone();
                    let mut protCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = protCrefs.clone();
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let true = (List::flatten(scalarOutputs.clone()).is_empty()) else { bail!("pattern mismatch") };
                    let true = (!(constScalarCrefs.clone().is_empty())) else { bail!("pattern mismatch") };
                    varScalarCrefsInFunc = metamodelica::nil();
                    allOutputCrefs = List::map(allOutputs.clone(), Arc::new(DAEUtil::varCref));
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(constScalarCrefs.clone(), allOutputCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual))?;
                    pos = List::map1(outputCrefs.clone(), Arc::new(List::position), allOutputCrefs.clone());
                    varScalarExps = List::map1(pos.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    outputExp = if (List::hasOneElement(varScalarExps.clone())) {listHead(varScalarExps.clone())?} else {Arc::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    funcOutputs = List::map2(outputCrefs.clone(), Arc::new(generateOutputElements), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(protCrefs.clone(), Arc::new(generateProtectedElements), allOutputs.clone(), lhsExpIn.clone());
                    varOutputs = listAppend(funcOutputs.clone(), funcProts.clone());
                    Ok((varOutputs.clone(), outputExp.clone(), varScalarCrefsInFunc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil, _, _) => {
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = expLst.clone();
                    let mut lhsCref: Arc<DAE::ComponentRef> = lhsCref.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    lhsCref = Expression::expCref(lhsExpIn.clone())?;
                    outputCrefs = List::map(constScalarCrefs.clone(), Arc::new(ComponentReference::crefStripFirstIdent));
                    outputCrefs = List::map1(outputCrefs.clone(), Arc::new(ComponentReference::joinCrefsR), lhsCref.clone());
                    expLst = List::map(outputCrefs.clone(), Arc::new(Expression::crefExp));
                    outputExp = Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() });
                    Ok((metamodelica::nil(), outputExp.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    let mut funcSOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcSOutputs.clone();
                    let mut funcSProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcSProts.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut lhsCref: Arc<DAE::ComponentRef> = lhsCref.clone();
                    let mut varScalarCrefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefs1.clone();
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = varOutputs.clone();
                    lhsCref = Expression::expCref(lhsExpIn.clone())?;
                    allOutputCrefs = List::map(allOutputs.clone(), Arc::new(DAEUtil::varCref));
                    funcOutputs = List::map2(varComplexCrefs.clone(), Arc::new(generateOutputElements), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(constComplexCrefs.clone(), Arc::new(generateProtectedElements), allOutputs.clone(), lhsExpIn.clone());
                    funcSOutputs = List::map2(varScalarCrefs.clone(), Arc::new(generateOutputElements), allOutputs.clone(), lhsExpIn.clone());
                    funcSProts = List::map2(constScalarCrefs.clone(), Arc::new(generateProtectedElements), allOutputs.clone(), lhsExpIn.clone());
                    varOutputs = List::flatten(list![funcOutputs.clone(), funcSOutputs.clone(), funcProts.clone(), funcSProts.clone()]);
                    varScalarCrefs1 = List::map(varScalarCrefs.clone(), Arc::new(ComponentReference::crefStripFirstIdent));
                    varScalarCrefs1 = List::map1(varScalarCrefs1.clone(), Arc::new(ComponentReference::joinCrefsR), lhsCref.clone());
                    varScalarExps = List::map(varScalarCrefs1.clone(), Arc::new(Expression::crefExp));
                    outputExp = if (List::hasOneElement(varScalarExps.clone())) {listHead(varScalarExps.clone())?} else {Arc::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    Ok((varOutputs.clone(), outputExp.clone(), varScalarCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", (literal!("buildVariableFunctionParts failed!\n")).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n scalarOutputs \n")); __mm_s.push_str(&*stringDelimitList(List::map(List::flatten(scalarOutputs.clone()), Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n constScalarCrefs \n")); __mm_s.push_str(&*stringDelimitList(List::map(constScalarCrefs.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n allOutputs ")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*DAEDump::dumpElementsStr(allOutputs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n lhsExpIn ")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(lhsExpIn.clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((varOutputs, outputExpOut, varScalarCrefsInFunc))
}

fn buildConstFunctionCrefs(mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut constComplCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut lhsExpIn: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut constScalarCrefsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constComplCrefsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (constScalarCrefsOut, constComplCrefsOut) = 'mc: {
        let __mc_input = (constScalarCrefs.clone(), constComplCrefs.clone(), allOutputCrefs.clone(), lhsExpIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _) => {
                    let mut lhsCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    lhsCref = Expression::expCref(lhsExpIn.clone())?;
                    constCrefs = List::map(constScalarCrefs.clone(), Arc::new(ComponentReference::crefStripFirstIdent));
                    constCrefs = List::map1(constCrefs.clone(), Arc::new(ComponentReference::joinCrefsR), lhsCref.clone());
                    Ok((constCrefs.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut pos: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lhsCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut constExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    pos = metamodelica::nil();
                    for mut lhsCref in &*constComplCrefs.clone() {
                        let mut lhsCref = lhsCref.clone();
                        pos = cons(List::position1OnTrue(allOutputCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual), lhsCref.clone()), pos.clone());
                    }
                    pos = pos.clone().reverse();
                    constExps = List::map1(pos.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    constCrefs = List::map(constExps.clone(), Arc::new(Expression::expCref));
                    Ok((metamodelica::nil(), constCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((constScalarCrefs.clone(), constComplCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((constScalarCrefsOut, constComplCrefsOut))
}

fn checkIfOutputIsEvaluatedConstant(mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut constComplexLstIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut varComplexLstIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut constScalarLstIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut varScalarLstIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut constComplexLstOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut varComplexLstOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constScalarLstOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut varScalarLstOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (constComplexLstOut, varComplexLstOut, constScalarLstOut, varScalarLstOut) = 'mc: {
        let __mc_input = (elements.clone(), constCrefs.clone(), constComplexLstIn.clone(), varComplexLstIn.clone(), constScalarLstIn.clone(), varScalarLstIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok((constComplexLstIn.clone(), varComplexLstIn.clone(), constScalarLstIn.clone(), varScalarLstIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: elem, tail: rest }, _, _, _, _, _) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut scalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constCrefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constCompl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varCompl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varScalar: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constScalar: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    cref = DAEUtil::varCref(elem.clone())?;
                    (constVars, varVars, constCrefs1) = List::intersection1OnTrue(list![cref.clone()], constCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual))?;
                    if constVars.clone().is_empty() {
                        scalars = getScalarsForComplexVar(elem.clone())?;
                        if scalars.clone().is_empty() {
                            (constCompl, varCompl, constScalar, varScalar) = (constComplexLstIn.clone(), listAppend(varVars.clone(), varComplexLstIn.clone()), constScalarLstIn.clone(), varScalarLstIn.clone());
                        } else {
                            (constVars, varVars, constCrefs1) = List::intersection1OnTrue(scalars.clone(), constCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual))?;
                            (constCompl, varCompl, constScalar, varScalar) = (constComplexLstIn.clone(), varComplexLstIn.clone(), listAppend(constVars.clone(), constScalarLstIn.clone()), listAppend(varVars.clone(), varScalarLstIn.clone()));
                        }
                    } else {
                        (constCompl, varCompl, constScalar, varScalar) = (listAppend(constVars.clone(), constComplexLstIn.clone()), varComplexLstIn.clone(), constScalarLstIn.clone(), varScalarLstIn.clone());
                    }
                    (constCompl, varCompl, constScalar, varScalar) = checkIfOutputIsEvaluatedConstant(rest.clone(), constCrefs1.clone(), constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone())?;
                    Ok((constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: elem, tail: rest }, _, _, _, _, _) => {
                    let mut r#const: bool = false;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut scalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constCompl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varCompl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varScalar: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constScalar: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    scalars = getScalarsForComplexVar(elem.clone())?;
                    let false = (scalars.clone().is_empty()) else { bail!("pattern mismatch") };
                    constVars = List::intersectionOnTrue(scalars.clone(), constCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual));
                    r#const = intEq((scalars.clone().len() as i32), (constVars.clone().len() as i32));
                    constScalarCrefs = List::filter1OnTrue(constCrefs.clone(), Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)), constVars.clone());
                    (_, varCrefs, _) = List::intersection1OnTrue(scalars.clone(), constScalarCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual))?;
                    constCompl = if (false) {cons(cref.clone(), constComplexLstIn.clone())} else {constComplexLstIn.clone()};
                    varCompl = varComplexLstIn.clone();
                    constScalar = if (true) {listAppend(constScalarCrefs.clone(), constScalarLstIn.clone())} else {constScalarLstIn.clone()};
                    varScalar = if (!(r#const.clone())) {listAppend(varCrefs.clone(), varScalarLstIn.clone())} else {varScalarLstIn.clone()};
                    (constCompl, varCompl, constScalar, varScalar) = checkIfOutputIsEvaluatedConstant(rest.clone(), constCrefs.clone(), constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone())?;
                    Ok((constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: elem, tail: rest }, _, _, _, _, _) => {
                    let mut r#const: bool = false;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut scalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constCompl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varCompl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varScalar: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constScalar: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    cref = DAEUtil::varCref(elem.clone())?;
                    scalars = getScalarsForComplexVar(elem.clone())?;
                    let true = (scalars.clone().is_empty()) else { bail!("pattern mismatch") };
                    r#const = listMember(cref.clone(), constCrefs.clone());
                    constCompl = if (r#const.clone()) {cons(cref.clone(), constComplexLstIn.clone())} else {constComplexLstIn.clone()};
                    varCompl = if (!(r#const.clone())) {cons(cref.clone(), varComplexLstIn.clone())} else {varComplexLstIn.clone()};
                    (constCompl, varCompl, constScalar, varScalar) = checkIfOutputIsEvaluatedConstant(rest.clone(), constCrefs.clone(), constCompl.clone(), varCompl.clone(), constScalarLstIn.clone(), varScalarLstIn.clone())?;
                    Ok((constCompl.clone(), varCompl.clone(), constScalar.clone(), varScalar.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("checkIfOutputIsEvaluatedConstant failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((constComplexLstOut, varComplexLstOut, constScalarLstOut, varScalarLstOut))
}

fn generateOutputElements(mut cref: Arc<DAE::ComponentRef>, mut inFuncOutputs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut recId: Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> {
    let mut newOutputs: Arc<DAE::Element>;
    newOutputs = (::match_deref::match_deref! { match &((cref.clone(), inFuncOutputs.clone(), recId.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: sl, .. }, _, _) => {
            let mut i1: ArcStr = arcstr::literal!("");
            let mut i2: ArcStr = arcstr::literal!("");
            let mut cref1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut var: Arc<DAE::Element>;
            let mut typ: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            typ = ComponentReference::crefLastType(cref.clone())?;
            cref1 = ComponentReference::crefStripLastIdent(cref.clone())?;
            crefs = getRecordScalars(cref.clone());
            cref1 = if (intEq((crefs.clone().len() as i32), 1)) {listHead(crefs.clone())?} else {cref1.clone()};
            i1 = (ComponentReferenceBasics::crefFirstIdent(cref.clone())?).clone();
            i2 = (ComponentReferenceBasics::crefLastIdent(cref.clone())?).clone();
            i1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*i1.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*i2.clone()); ArcStr::from(__mm_s) }).clone();
            cref1 = ComponentReferenceBasics::makeCrefIdent((i1.clone()).clone(), typ.clone(), sl.clone());
            var = listHead(inFuncOutputs.clone())?;
            var = DAEUtil::replaceCrefandTypeInVar(cref1.clone(), typ.clone(), var.clone())?;
            var.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { identType: typ, .. }, _, _) => {
            let mut var: Arc<DAE::Element>;
            var = listHead(inFuncOutputs.clone())?;
            var = DAEUtil::replaceCrefandTypeInVar(cref.clone(), typ.clone(), var.clone())?;
            var.clone()
        },
        _ => {
            println!("{}", (literal!("generateOutputElements failed!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(newOutputs)
}

fn generateProtectedElements(mut cref: Arc<DAE::ComponentRef>, mut inFuncOutputs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut recId: Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> {
    let mut newProts: Arc<DAE::Element>;
    newProts = (::match_deref::match_deref! { match &((cref.clone(), inFuncOutputs.clone(), recId.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: sl, .. }, _, _) => {
            let mut cref1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut i1: ArcStr = arcstr::literal!("");
            let mut i2: ArcStr = arcstr::literal!("");
            let mut var: Arc<DAE::Element>;
            let mut typ: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            typ = ComponentReference::crefLastType(cref.clone())?;
            let _ = Expression::crefExp(cref.clone())?;
            i1 = (ComponentReferenceBasics::crefFirstIdent(cref.clone())?).clone();
            i2 = (ComponentReferenceBasics::crefLastIdent(cref.clone())?).clone();
            i1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*i1.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*i2.clone()); ArcStr::from(__mm_s) }).clone();
            cref1 = ComponentReferenceBasics::makeCrefIdent((i1.clone()).clone(), typ.clone(), sl.clone());
            var = listHead(inFuncOutputs.clone())?;
            var = DAEUtil::replaceCrefandTypeInVar(cref1.clone(), typ.clone(), var.clone())?;
            var = DAEUtil::setElementVarVisibility(var.clone(), openmodelica_frontend_types::DAE::VarVisibility::PROTECTED);
            var = DAEUtil::setElementVarDirection(var.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
            var.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { identType: typ, .. }, _, _) => {
            let mut var: Arc<DAE::Element>;
            var = listHead(inFuncOutputs.clone())?;
            var = DAEUtil::replaceCrefandTypeInVar(cref.clone(), typ.clone(), var.clone())?;
            var = DAEUtil::setElementVarVisibility(var.clone(), openmodelica_frontend_types::DAE::VarVisibility::PROTECTED);
            var = DAEUtil::setElementVarDirection(var.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
            var.clone()
        },
        _ => {
            println!("{}", (literal!("generateProtectedElements failed!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(newProts)
}

fn updateFunctionBody(mut funcIn: DAE::Function, mut body: Arc<metamodelica::List<Arc<DAE::Element>>>, mut idx: i32, mut outputs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut origOutputs: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(DAE::Function, Arc<Absyn::Path>)> {
    let mut funcOut: DAE::Function = funcIn.clone();
    let mut pathOut: Arc<Absyn::Path>;
    (funcOut, pathOut) = (match funcOut.clone() {
        DAE::Function::FUNCTION { .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (AbsynUtil::pathLastIdent(var_field!(funcOut.path, DAE::Function::FUNCTION).clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("_eval")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone();
            let __owned_variant_path_0 = AbsynUtil::pathSetLastIdent(AbsynUtil::makeNotFullyQualified(var_field!(funcOut.path, DAE::Function::FUNCTION).clone()), (s.clone()).clone())?;
            let __owned_variant_type__1 = updateFunctionType(var_field!(funcOut.type_, DAE::Function::FUNCTION).clone(), outputs.clone(), origOutputs.clone())?;
            let __owned_variant_functions_2 = list![DAE::FunctionDefinition::FUNCTION_DEF { body: body.clone() }];
            if let DAE::Function::FUNCTION { path, type_, functions, .. } = &mut funcOut {
                *path = __owned_variant_path_0;
                *type_ = __owned_variant_type__1;
                *functions = __owned_variant_functions_2;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            (funcOut.clone(), var_field!(funcOut.path, DAE::Function::FUNCTION).clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EvaluateFunctions.updateFunctionBody")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok((funcOut, pathOut))
}

fn updateFunctionType(mut typIn: Arc<DAE::Type>, mut outputs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut originOutputs: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<DAE::Type>> {
    let mut typOut: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    typOut = 'mc: {
        let __mc_input = (typIn.clone(), outputs.clone(), originOutputs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty @ Deref @ DAE::Type::T_FUNCTION { .. }, _, _) => {
                    let mut outTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut ty = (*ty).clone();
                    outTypeLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut o in (outputs.clone()).into_iter().cloned() {
                    let __x = DAEUtil::getVariableType(o.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    outNames = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut o in (outputs.clone()).into_iter().cloned() {
                    let __x = DAEUtil::varName(o.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    let __owned_variant_funcResultType_0 = if (intEq((outTypeLst.clone().len() as i32), 1)) {listHead(outTypeLst.clone())?} else {Arc::new(DAE::Type::T_TUPLE { types: outTypeLst.clone(), names: Some(outNames.clone()) })};
                    if let DAE::Type::T_FUNCTION { funcResultType, .. } = &mut ty {
                        *funcResultType = __owned_variant_funcResultType_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than DAE::Type::T_FUNCTION"); }
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(typIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(typOut)
}

fn buildPartialFunction(mut varPart: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Element>>>), mut constPart: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), mut replIn: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut algsOut: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqs: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut varScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constComplCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constScalarCrefsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut funcIn: Arc<DAE::Exp>;
    let mut funcAlgs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut constComplExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut constScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lhsExps1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lhsExps2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lhsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    (varScalarCrefs, funcAlgs) = varPart.clone();
    (constScalarCrefs, constScalarExps, constComplCrefs, constComplExps, constScalarCrefsOut) = constPart.clone();
    funcAlgs = List::filterOnTrue(funcAlgs.clone(), Arc::new(fnptr!(DAEUtil::isAlgorithm, Arc<DAE::Element>)));
    lhsExps1 = List::map(constScalarCrefsOut.clone(), Arc::new(Expression::crefExp));
    lhsExps2 = List::map(constComplCrefs.clone(), Arc::new(Expression::crefExp));
    eqsOut = generateConstEqs(lhsExps1.clone(), constScalarExps.clone(), metamodelica::nil())?;
    eqsOut = generateConstEqs(lhsExps2.clone(), constComplExps.clone(), eqsOut.clone())?;
    stmts1 = List::mapFlatReverse(funcAlgs.clone(), Arc::new(DAEUtil::getStatement));
    (stmts1, _) = DAEUtil::traverseDAEEquationsStmts(stmts1.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (makeIdentCref, varScalarCrefs.clone()));
    (stmts1, _) = DAEUtil::traverseDAEEquationsStmts(stmts1.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (makeIdentCref, constScalarCrefs.clone()));
    algsOut = list![Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts1.clone() }), source: DAE::emptyElementSource.clone() })];
    Ok((algsOut, eqsOut))
}

fn stmtCanBeRemoved<Type_a: Clone + 'static>(mut stmtIn: Arc<DAE::Statement>, mut repl: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Statement>, bool)> {
    let mut tplOut: (Arc<DAE::Statement>, bool);
    tplOut = 'mc: {
        let __mc_input = (stmtIn.clone(), repl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { .. }, _) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut stmt: Arc<DAE::Statement>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceStatementLst(list![stmtIn.clone()], repl.clone(), None, metamodelica::nil(), false)?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    stmt = __pa0.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(stmt.clone()) {
                        Deref @ DAE::Statement::STMT_ASSIGN { exp: __pa2, exp1: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa2.clone();
                    e1 = __pa3.clone();
                    b1 = Expression::isConst(e1.clone())?;
                    b2 = Expression::isConst(e2.clone())?;
                    stmt = stmtIn.clone();
                    Ok((stmt.clone(), b1.clone() && b2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((stmtIn.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn traverseStmtsAndUpdate<Type_a: Clone + 'static>(mut stmtsIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Statement>, bool)> + 'static>, mut argIn: Type_a, mut stmtsFold: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    pub type FuncType<Type_a: Clone> = fn(Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Statement>, bool)>;

    let mut stmtsOut: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    stmtsOut = 'mc: {
        let __mc_input = (stmtsIn.clone(), func.clone(), argIn.clone(), stmtsFold.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    let _ = stmtsFold.clone().reverse();
                    Ok(stmtsFold.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { else_, statementLst: stmtLst, .. }, tail: rest }, _, _, _) => {
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut stmtLst = (*stmtLst).clone();
                    x = listHead(stmtsIn.clone())?;
                    stmtLstLst = getDAEelseStatemntLsts(else_.clone(), metamodelica::nil());
                    stmtLstLst = stmtLstLst.clone().reverse();
                    stmtLstLst = List::map3(stmtLstLst.clone(), Arc::new(traverseStmtsAndUpdate), func.clone(), argIn.clone(), metamodelica::nil());
                    stmtLst = traverseStmtsAndUpdate(stmtLst.clone(), func.clone(), argIn.clone(), metamodelica::nil())?;
                    stmtLstLst = cons(stmtLst.clone(), stmtLstLst.clone());
                    x = updateStatementsInIfStmt(stmtLstLst.clone(), x.clone())?;
                    xs = traverseStmtsAndUpdate(rest.clone(), func.clone(), argIn.clone(), cons(x.clone(), stmtsFold.clone()))?;
                    Ok(xs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: x, tail: rest }, _, _, _) => {
                    let mut b: bool = false;
                    let mut xs: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut x = (*x).clone();
                    (x, b) = func(x.clone(), argIn.clone())?;
                    xs = if (b.clone()) {stmtsFold.clone()} else {cons(x.clone(), stmtsFold.clone())};
                    xs = traverseStmtsAndUpdate(rest.clone(), func.clone(), argIn.clone(), xs.clone())?;
                    Ok(xs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(stmtsOut)
}

fn makeIdentCref(mut inExp: Arc<DAE::Exp>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outCrefs) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefs.clone())) {
        (Deref @ DAE::Exp::CREF { ty, componentRef: cref }, crefs) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cref = (*cref).clone();
            cref = makeIdentCref2(cref.clone(), crefs.clone())?;
            exp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
            (exp.clone(), crefs.clone())
        },
        _ => {
            (inExp.clone(), inCrefs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outCrefs))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeIdentCref2(mut crefIn: Arc<DAE::ComponentRef>, mut changeTheseCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut crefOut: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefOut = 'mc: {
        let __mc_input = (crefIn.clone(), changeTheseCrefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref1 @ Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref2, ident: i1, .. }, _) => {
                    let mut i2: ArcStr = arcstr::literal!("");
                    let mut cref2 = (*cref2).clone();
                    let mut i1 = (*i1).clone();
                    let true = (List::isMemberOnTrue(cref1.clone(), changeTheseCrefs.clone(), Arc::new(ComponentReferenceBasics::crefEqual))) else { bail!("pattern mismatch") };
                    i2 = (ComponentReferenceBasics::crefFirstIdent(cref2.clone())?).clone();
                    i1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*i1.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*i2.clone()); ArcStr::from(__mm_s) }).clone();
                    cref2 = replaceCrefIdent(cref2.clone(), (i1.clone()).clone());
                    cref2 = makeIdentCref2(cref2.clone(), changeTheseCrefs.clone())?;
                    Ok(cref2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref1 @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _) => {
                    Ok(cref1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(crefIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(crefOut)
}

fn replaceCrefIdent(mut crefIn: Arc<DAE::ComponentRef>, mut ident: ArcStr) -> Arc<DAE::ComponentRef> {
    let mut crefOut: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefOut = (::match_deref::match_deref! { match &((crefIn.clone(), ident.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref2, subscriptLst: sl, identType: typ, .. }, _) => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: typ.clone(), subscriptLst: sl.clone(), componentRef: cref2.clone() });
            cref.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: sl, identType: typ, .. }, _) => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: typ.clone(), subscriptLst: sl.clone() });
            cref.clone()
        },
        _ => {
            crefIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    crefOut
}

fn statementRHSIsNotConst(mut stmt: Arc<DAE::Statement>) -> Result<bool> {
    let mut notConst: bool = false;
    notConst = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, .. } => {
            let mut b: bool = false;
            b = Expression::isConst(rhs.clone())?;
            !(b.clone())
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(notConst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn generateConstEqs(mut lhsLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rhsLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    eqsOut = (::match_deref::match_deref! { match &((lhsLst.clone(), rhsLst.clone(), eqsIn.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            eqsIn.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, tail: lrest }, Deref @ metamodelica::List::Cons { head: _, tail: rrest }, _) => {
            let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            eqs = generateConstEqs(lrest.clone(), rrest.clone(), eqsIn.clone())?;
            eqs.clone()
        },
        (Deref @ metamodelica::List::Cons { head: lhs, tail: lrest }, Deref @ metamodelica::List::Cons { head: rhs, tail: rrest }, _) => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            eq = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            eqs = generateConstEqs(lrest.clone(), rrest.clone(), cons(eq.clone(), eqsIn.clone()))?;
            eqs.clone()
        },
        _ => {
            println!("{}", (literal!("generateConstEqs failed!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqsOut)
}

fn addReplacementRuleForAssignment(mut stmt: Arc<DAE::Statement>, mut replIn: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = (::match_deref::match_deref! { match &((stmt.clone(), replIn.clone())) {
        (Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: lhs, .. }, _) => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = Expression::expCref(lhs.clone())?;
            repl = BackendVarTransform::addReplacement(replIn.clone(), cref.clone(), rhs.clone(), None)?;
            repl.clone()
        },
        _ => {
            replIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(replOut)
}

fn unboxExp(mut ie: Arc<DAE::Exp>, mut bIn: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut bOut: bool = false;
    (outExp, bOut) = (::match_deref::match_deref! { match &((ie.clone(), bIn.clone())) {
        (Deref @ DAE::Exp::BOX { exp: e }, _) => {
            unboxExp(e.clone(), true)
        },
        _ => {
            (ie.clone(), bIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, bOut)
}

fn getRangeBounds(mut range: Arc<DAE::Exp>) -> Result<(i32, i32, i32)> {
    let mut start: i32 = 0;
    let mut stop: i32 = 0;
    let mut step: i32 = 0;
    (start, stop, step) = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::ICONST { integer: i2 }, step: None, start: Deref @ DAE::Exp::ICONST { integer: i1 }, .. } => {
            (i1.clone(), i2.clone(), 1)
        },
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::ICONST { integer: i2 }, step: Some(Deref @ DAE::Exp::ICONST { integer: i3 }), start: Deref @ DAE::Exp::ICONST { integer: i1 }, .. } => {
            (i1.clone(), i2.clone(), i3.clone())
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start, stop, step))
}

fn evaluateIfStatement(mut stmtIn: Arc<DAE::Statement>, mut info: FuncInfo, mut recursionLimit: i32) -> Result<(bool, Arc<metamodelica::List<Arc<DAE::Statement>>>, BackendVarTransform::VariableReplacements)> {
    let mut isEval: bool = false;
    let mut stmtsOut: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut replOut: BackendVarTransform::VariableReplacements;
    (isEval, stmtsOut, replOut) = 'mc: {
        let __mc_input = (stmtIn.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { else_, statementLst: stmtsIf, exp: expIf, .. }, FuncInfo { idx, funcTree, repl: replIn }) => {
                    let mut isIf: bool = false;
                    let mut isCon: bool = false;
                    let mut isElse: bool = false;
                    let mut eval: bool = false;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut exp1: Arc<DAE::Exp>;
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtsElse: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut idx = (*idx).clone();
                    let mut funcTree = (*funcTree).clone();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", (literal!("-->try to check if its the if case\n")).clone());
                    }
                    (exp1, _) = BackendVarTransform::replaceExp(expIf.clone(), replIn.clone(), None)?;
                    exp1 = evaluateConstantFunctionCall(exp1.clone(), exp1.clone(), funcTree.clone(), idx.clone(), recursionLimit.clone())?;
                    (exp1, _) = BackendVarTransform::replaceExp(exp1.clone(), replIn.clone(), None)?;
                    (exp1, _) = ExpressionSimplify::simplify(exp1.clone())?;
                    isCon = Expression::isConst(exp1.clone())?;
                    isIf = if (isCon.clone()) {Expression::toBool(exp1.clone())?} else {false};
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-->is the if const? ")); __mm_s.push_str(&*boolString(isCon.clone())); __mm_s.push_str(&*literal!(" and is it the if case ? ")); __mm_s.push_str(&*boolString(isIf.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if isIf.clone() && isCon.clone() {
                        (stmts1, funcTree, repl, idx) = evaluateFunctions_updateStatement(stmtsIf.clone(), funcTree.clone(), replIn.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
                    } else {
                        stmts1 = list![stmtIn.clone()];
                        repl = replIn.clone();
                    }
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isIf.clone()) {
                        println!("{}", (literal!("-->try to check if its another case\n")).clone());
                    }
                    if isCon.clone() && !(isIf.clone()) {
                        (stmtsElse, isElse) = evaluateElse(else_.clone(), info.clone(), recursionLimit.clone())?;
                    } else {
                        stmtsElse = list![stmtIn.clone()];
                        isElse = false;
                    }
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isIf.clone()) {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-->is it an other case? ")); __mm_s.push_str(&*boolString(isElse.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if isCon.clone() && isElse.clone() {
                        (stmts1, funcTree, repl, idx) = evaluateFunctions_updateStatement(stmtsElse.clone(), funcTree.clone(), replIn.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
                    }
                    eval = isCon.clone() && (isIf.clone() || isElse.clone());
                    Ok((eval.clone(), stmts1.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", (literal!("evaluateIfStatement failed \n")).clone());
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((isEval, stmtsOut, replOut))
}

fn evaluateElse(mut elseIn: Arc<DAE::Else>, mut info: FuncInfo, mut recursionLimit: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, bool)> {
    let mut stmtsOut: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut isElse: bool = false;
    (stmtsOut, isElse) = (::match_deref::match_deref! { match &((elseIn.clone(), info.clone())) {
        (Deref @ DAE::Else::ELSEIF { else_, statementLst: stmts, exp: expIf }, FuncInfo { idx, funcTree, repl: replIn }) => {
            let mut isCon: bool = false;
            let mut isElseIf: bool = false;
            let mut exp1: Arc<DAE::Exp>;
            let mut stmts = (*stmts).clone();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", (literal!("-->try to check if its the elseif case\n")).clone());
            }
            exp1 = evaluateConstantFunctionCall(expIf.clone(), expIf.clone(), funcTree.clone(), idx.clone(), recursionLimit.clone())?;
            (exp1, _) = BackendVarTransform::replaceExp(exp1.clone(), replIn.clone(), None)?;
            (exp1, _) = ExpressionSimplify::simplify(exp1.clone())?;
            isCon = Expression::isConst(exp1.clone())?;
            isElseIf = if (isCon.clone()) {Expression::toBool(exp1.clone())?} else {false};
            if isCon.clone() && !(isElseIf.clone()) {
                (stmts, isElseIf) = evaluateElse(else_.clone(), info.clone(), recursionLimit.clone())?;
            }
            (stmts.clone(), isElseIf.clone())
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, FuncInfo { .. }) => {
            (stmts.clone(), true)
        },
        (Deref @ DAE::Else::NOELSE, FuncInfo { .. }) => {
            (metamodelica::nil(), true)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((stmtsOut, isElse))
}

fn addTplReplacements(mut replIn: BackendVarTransform::VariableReplacements, mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = 'mc: {
        let __mc_input = (replIn.clone(), e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut tplLHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tplRHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut repl: BackendVarTransform::VariableReplacements;
                    tplRHS = DAEUtil::getTupleExps(e1.clone());
                    tplLHS = DAEUtil::getTupleExps(e2.clone());
                    crefs = List::map(tplLHS.clone(), Arc::new(Expression::expCref));
                    repl = BackendVarTransform::addReplacements(replIn.clone(), crefs.clone(), tplRHS.clone(), None)?;
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(replIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(replOut)
}

fn equationToStatement(mut eqIn: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Statement>> {
    let mut stmtOut: Arc<DAE::Statement>;
    stmtOut = (::match_deref::match_deref! { match &(eqIn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { source, scalar: rhs, exp: lhs, .. } => {
            let mut typ: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            typ = Expression::r#typeof(lhs.clone())?;
            Arc::new(DAE::Statement::STMT_ASSIGN { type_: typ.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() })
        },
        _ => {
            println!("{}", (literal!("equationToStatement failed!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmtOut)
}

fn replaceExps(mut replIn: BackendVarTransform::VariableReplacements, mut expsIn: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut expsOut: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (expsOut, _) = List::map2_2(expsIn.clone(), Arc::new(BackendVarTransform::replaceExp), replIn.clone(), None);
    expsOut
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getStatementLHS(mut stmt: Arc<DAE::Statement>, mut expsIn: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    lhs = (::match_deref::match_deref! { match &((stmt.clone(), expsIn.clone())) {
        (Deref @ DAE::Statement::STMT_ASSIGN { exp1: exp, .. }, _) => {
            cons(exp.clone(), expsIn.clone())
        },
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, .. }, _) => {
            listAppend(expLst.clone(), expsIn.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp, .. }, _) => {
            cons(exp.clone(), expsIn.clone())
        },
        (Deref @ DAE::Statement::STMT_IF { else_, statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut stmtLst2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut stmtLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
            stmtLstLst = getDAEelseStatemntLsts(else_.clone(), metamodelica::nil());
            stmtLst2 = List::flatten(stmtLstLst.clone());
            stmtLst2 = listAppend(stmtLst1.clone(), stmtLst2.clone());
            expLst = List::fold(stmtLst2.clone(), Arc::new(getStatementLHS), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_FOR { statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = List::fold(stmtLst1.clone(), Arc::new(getStatementLHS), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_PARFOR { statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = List::fold(stmtLst1.clone(), Arc::new(getStatementLHS), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_WHILE { statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = List::fold(stmtLst1.clone(), Arc::new(getStatementLHS), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(stmt1), statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" check getStatementLHS for WHEN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            expLst = List::fold(stmtLst1.clone(), Arc::new(getStatementLHS), expsIn.clone());
            expLst = getStatementLHS(stmt1.clone(), expLst.clone())?;
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" check getStatementLHS for WHEN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            expLst = List::fold(stmtLst1.clone(), Arc::new(getStatementLHS), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_ASSERT { .. }, _) => {
            expsIn.clone()
        },
        (Deref @ DAE::Statement::STMT_TERMINATE { .. }, _) => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for TERMINATE!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            bail!("fail")
        },
        (Deref @ DAE::Statement::STMT_REINIT { .. }, _) => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for REINIT!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            bail!("fail")
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { .. }, _) => {
            expsIn.clone()
        },
        (Deref @ DAE::Statement::STMT_RETURN { .. }, _) => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for RETURN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            bail!("fail")
        },
        (Deref @ DAE::Statement::STMT_BREAK { .. }, _) => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for BREAK!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            bail!("fail")
        },
        (Deref @ DAE::Statement::STMT_ARRAY_INIT { .. }, _) => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for ARRAY_INIT!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            bail!("fail")
        },
        _ => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getStatementLHS update for !\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lhs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDAEelseStatemntLsts(mut elseIn: Arc<DAE::Else>, mut stmtLstsIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> {
    let mut stmtLstsOut: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
    stmtLstsOut = (::match_deref::match_deref! { match &((elseIn.clone(), stmtLstsIn.clone())) {
        (Deref @ DAE::Else::ELSEIF { else_: else1, statementLst: stmts, .. }, _) => {
            let mut stmtsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
            stmtsLst = cons(stmts.clone(), stmtLstsIn.clone());
            stmtsLst = getDAEelseStatemntLsts(else1.clone(), stmtsLst.clone());
            stmtsLst.clone()
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, _) => {
            let mut stmtsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
            stmtsLst = cons(stmts.clone(), stmtLstsIn.clone());
            stmtsLst.clone()
        },
        _ => {
            stmtLstsIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stmtLstsOut
}

fn equationToStmt(mut eqIn: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Statement>> {
    let mut stmtOut: Arc<DAE::Statement>;
    stmtOut = 'mc: {
        let __mc_input = eqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { source, scalar: rhs, exp: lhs, .. } => {
                    let mut typ: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    typ = expType(lhs.clone())?;
                    Ok(Arc::new(DAE::Statement::STMT_ASSIGN { type_: typ.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("equationToStmt failed for: ")); __mm_s.push_str(&*BackendDump::dumpEqnsStr(list![eqIn.clone()])?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(stmtOut)
}

fn expType(mut eIn: Arc<DAE::Exp>) -> Result<Arc<DAE::Type>> {
    let mut tOut: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    tOut = 'mc: {
        let __mc_input = eIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty: t, .. } => {
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("expType failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eIn.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tOut)
}

fn getScalarsForComplexVar(mut inElem: Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crefsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crefsOut = 'mc: {
        let __mc_input = inElem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, componentRef: cref, .. } => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
                    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    names = List::map(varLst.clone(), Arc::new(DAEUtil::typeVarIdent));
                    types = List::map(varLst.clone(), Arc::new(DAEUtil::varType));
                    crefs = List::map1(names.clone(), Arc::new(ComponentReference::appendStringCref), cref.clone());
                    crefs = setTypesForScalarCrefs(crefs.clone(), types.clone())?;
                    crefLst = List::map1(crefs.clone(), Arc::new(ComponentReference::expandCref), true);
                    crefs = List::flatten(crefLst.clone());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { dims, ty: Deref @ DAE::Type::T_REAL { .. }, componentRef: cref, .. } => {
                    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    subslst = expandDimension(dims.clone(), metamodelica::nil())?;
                    crefs = List::map1r(subslst.clone(), Arc::new(ComponentReference::subscriptCref), cref.clone());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { dims, ty: Deref @ DAE::Type::T_INTEGER { .. }, componentRef: cref, .. } => {
                    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    subslst = expandDimension(dims.clone(), metamodelica::nil())?;
                    crefs = List::map1r(subslst.clone(), Arc::new(ComponentReference::subscriptCref), cref.clone());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_ARRAY { dims: dimensions, ty: Deref @ DAE::Type::T_ARRAY { dims: dimensions2, ty } }, componentRef: cref, .. } => {
                    let mut subslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    let mut subslst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    let mut subslst2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    subslst1 = expandDimension(dimensions.clone(), metamodelica::nil())?;
                    subslst2 = expandDimension(dimensions2.clone(), metamodelica::nil())?;
                    subslst = metamodelica::nil();
                    for mut subs in &*subslst1.clone() {
                        let mut subs = subs.clone();
                        for mut subs2 in &*subslst2.clone() {
                            let mut subs2 = subs2.clone();
                            subslst = cons(listAppend(subs.clone(), subs2.clone()), subslst.clone());
                        }
                    }
                    crefs = List::map1r(subslst.clone(), Arc::new(ComponentReference::subscriptCref), cref.clone());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_ARRAY { dims: dimensions, .. }, componentRef: cref, .. } => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the array cref before\n")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    crefs = ComponentReference::expandArrayCref(cref.clone(), dimensions.clone())?;
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_ENUMERATION { .. }, componentRef: cref, .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update getScalarsForComplexVar for enumerations: the enum cref is :")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_TUPLE { .. }, componentRef: cref, .. } => {
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update getScalarsForComplexVar for tuple types: the tupl cref is :\n")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], Arc::new(ComponentReferenceBasics::printComponentRefStr)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(crefsOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn expandDimension(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut subsIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut subsOut: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
    subsOut = (::match_deref::match_deref! { match &((dims.clone(), subsIn.clone())) {
        (Deref @ metamodelica::List::Cons { head: dim, tail: rest }, _) => {
            let mut size: i32 = 0;
            let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut sub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut subsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            let mut subsLst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            let mut subFold: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            size = Expression::dimensionSize(dim.clone())?;
            range = List::intRange(size.clone());
            subs = List::map(range.clone(), Arc::new(fnptr!(Expression::intSubscript, i32)));
            subsLst = List::map(subs.clone(), Arc::new(fnptr!(List::create, _)));
            for mut sub in &*subsIn.clone() {
                let mut sub = sub.clone();
                subsLst1 = List::map1r(subsLst.clone(), Arc::new(listAppend.clone()), sub.clone());
                subFold = listAppend(subFold.clone(), subsLst1.clone());
            }
            if subsIn.clone().is_empty() {
                subFold = subsLst.clone();
            }
            expandDimension(rest.clone(), subFold.clone())?
        },
        (Deref @ metamodelica::List::Nil, _) => {
            let mut subFold: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            subsIn.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(subsOut)
}

fn subsLstString(mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(subs.clone(), Arc::new(ExpressionDump::subscriptString)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    s
}

fn isNotComplexVar(mut inElem: Arc<DAE::Element>) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = inElem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, .. } => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { dims, ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. } => {
                    let mut dimints: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    dimints = List::map(dims.clone(), Arc::new(Expression::dimensionSize));
                    let true = (listHead(dimints.clone())? != 0) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { dims, ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. } => {
                    let mut dimints: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    dimints = List::map(dims.clone(), Arc::new(Expression::dimensionSize));
                    let true = (listHead(dimints.clone())? != 0) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. } => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(b)
}

fn setTypesForScalarCrefs(mut allCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut types: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crefsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crefsOut = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for (cr, ty) in (&(allCrefs.clone())).into_iter().zip((&(types.clone())).into_iter()) {
            let __x = ComponentReference::crefSetLastType(cr.clone(), ty.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(crefsOut)
}

pub fn getRecordScalars(mut crefIn: Arc<DAE::ComponentRef>) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut crefsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    match '__try0: {
        crefsOut = unwrap_break_err!(ComponentReference::expandCref(crefIn.clone(), true), '__try0);
        Ok::<_, anyhow::Error>((crefsOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            crefsOut = __try0_o0;
        }
        Err(_) => {
            crefsOut = metamodelica::nil();
        }
    }
    crefsOut
}

fn getScalarExpSize(mut inExp: Arc<DAE::Exp>) -> Result<i32> {
    let mut size: i32 = 0;
    size = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::TUPLE { PR: exps @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
            let mut exps_len: i32 = 0;
            exps_len = todo!("reduction intAdd: cannot resolve default value");
            size = todo!("reduction intAdd: cannot resolve default value");
            std::cmp::max(size.clone(), exps_len.clone())
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { varLst: vl @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. } => {
            todo!("reduction intAdd: cannot resolve default value")
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            size = if (ComponentReference::isArrayElement(cref.clone())) {(ComponentReference::expandCref(cref.clone(), true)?.len() as i32)} else {1};
            size.clone()
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst: vl @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, .. } => {
            todo!("reduction intAdd: cannot resolve default value")
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_TUPLE { types: tyl @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. }, .. } => {
            let mut vl: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            size = 0;
            for mut ty in &*tyl.clone() {
                let mut ty = ty.clone();
                vl = getVarLstFromType(ty.clone());
                if !(vl.clone().is_empty()) {
                    size = size.clone() + todo!("reduction intAdd: cannot resolve default value");
                }
            }
            size.clone()
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

fn getVarLstFromType(mut tyIn: Arc<DAE::Type>) -> Arc<metamodelica::List<Arc<DAE::Var>>> {
    let mut varsOut: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    varsOut = (::match_deref::match_deref! { match &(tyIn.clone()) {
        Deref @ DAE::Type::T_TUPLE { types: tyLst @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
            {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut ty in (tyLst.clone()).into_iter().cloned() {
            let __x = getVarLstFromType(ty.clone());
            __acc = __x.append(&__acc);
        }
        __acc
    }
        },
        Deref @ DAE::Type::T_COMPLEX { varLst, .. } => {
            varLst.clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst, .. } => {
            varLst.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    varsOut
}

fn getScalarVarSize(mut inVar: Arc<DAE::Var>) -> Result<i32> {
    let mut size: i32 = 0;
    size = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_COMPLEX { varLst: vl @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. } => {
            todo!("reduction intAdd: cannot resolve default value")
        },
        Deref @ DAE::Var { ty: ty @ Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. } => {
            todo!("reduction intMul: cannot resolve default value")
        },
        _ => {
            1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

// =============================================================================
// predict if statements
//
// =============================================================================
fn predictIfOutput(mut stmtIn: Arc<DAE::Statement>, mut infoIn: FuncInfo, mut recursionLimit: i32) -> Result<((Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Statement>>>), FuncInfo)> {
    let mut stmtsOut: (Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Statement>>>);
    let mut infoOut: FuncInfo;
    (stmtsOut, infoOut) = 'mc: {
        let __mc_input = (stmtIn.clone(), infoIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { else_, statementLst: stmts1, .. }, FuncInfo { repl: replIn, funcTree, idx }) => {
                    let mut constantOutputs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut replLst: Arc<metamodelica::List<BackendVarTransform::VariableReplacements>> = metamodelica::nil();
                    let mut stmtNew: Arc<DAE::Statement>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut allLHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut addStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
                    let mut elseStmtsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
                    let mut funcTree = (*funcTree).clone();
                    let mut idx = (*idx).clone();
                    elseStmtsLst = getDAEelseStatemntLsts(else_.clone(), metamodelica::nil());
                    elseStmtsLst = elseStmtsLst.clone().reverse();
                    (stmtsLst, funcTree, idx) = evaluateFunctions_updateAllStatements(stmts1.clone(), elseStmtsLst.clone(), replIn.clone(), funcTree.clone(), idx.clone(), recursionLimit.clone())?;
                    replLst = List::map(stmtsLst.clone(), Arc::new(collectReplacements));
                    expLst = List::fold(List::flatten(stmtsLst.clone()), Arc::new(getStatementLHS), metamodelica::nil());
                    expLst = List::unique(expLst.clone());
                    allLHS = expLst.clone().reverse();
                    expLstLst = List::map1(replLst.clone(), Arc::new(fnptr!(replaceExps, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<DAE::Exp>>>)), allLHS.clone());
                    constantOutputs = compareConstantExps(expLstLst.clone())?;
                    outExps = List::map1(constantOutputs.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), allLHS.clone());
                    let _ = List::map(outExps.clone(), Arc::new(Expression::expCref));
                    expLst = List::map1(constantOutputs.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), listHead(expLstLst.clone())?);
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--> the predicted const outputs:\n")); __mm_s.push_str(&*stringDelimitList(List::map(outExps.clone(), Arc::new(ExpressionBasics::printExpStr)), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone());
                    }
                    addStmts = makeAssignmentMap(outExps.clone(), expLst.clone())?;
                    stmtNew = updateStatementsInIfStmt(stmtsLst.clone(), stmtIn.clone())?;
                    Ok(((list![stmtNew.clone()], addStmts.clone()), FuncInfo { repl: replIn.clone(), funcTree: funcTree.clone(), idx: idx.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(((list![stmtIn.clone()], metamodelica::nil()), infoIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((stmtsOut, infoOut))
}

fn collectReplacements(mut stmtsIn: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut repl: BackendVarTransform::VariableReplacements;
    repl = BackendVarTransform::emptyReplacements();
    replOut = collectReplacements1(stmtsIn.clone(), repl.clone())?;
    Ok(replOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn collectReplacements1(mut stmtsIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut replIn: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = 'mc: {
        let __mc_input = (stmtsIn.clone(), replIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(replIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: lhs, .. }, tail: rest }, _) => {
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut rhs = (*rhs).clone();
                    (rhs, _) = BackendVarTransform::replaceExp(rhs.clone(), replIn.clone(), None)?;
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    let true = (Expression::isConst(rhs.clone())?) else { bail!("pattern mismatch") };
                    cref = Expression::expCref(lhs.clone())?;
                    repl = BackendVarTransform::addReplacement(replIn.clone(), cref.clone(), rhs.clone(), None)?;
                    repl = collectReplacements1(rest.clone(), repl.clone())?;
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: rhs, expExpLst: lhsLst, .. }, tail: rest }, _) => {
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut rhsLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut rhs = (*rhs).clone();
                    (rhs, _) = BackendVarTransform::replaceExp(rhs.clone(), replIn.clone(), None)?;
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    rhsLst = Expression::getComplexContents(rhs.clone())?;
                    crefs = List::map(lhsLst.clone(), Arc::new(Expression::expCref));
                    (constExps, constCrefs) = List::filterOnTrueSync(rhsLst.clone(), Arc::new(Expression::isConst), crefs.clone())?;
                    (_, varCrefs) = List::filterOnTrueSync(rhsLst.clone(), Arc::new(Expression::isNotConst), crefs.clone())?;
                    repl = BackendVarTransform::addReplacements(replIn.clone(), constCrefs.clone(), constExps.clone(), None)?;
                    BackendVarTransform::removeReplacements(repl.clone(), varCrefs.clone())?;
                    repl = collectReplacements1(rest.clone(), repl.clone())?;
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: stmt, tail: rest }, _) => {
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut lhsLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    lhsLst = getStatementLHS(stmt.clone(), metamodelica::nil())?;
                    crefs = List::map(lhsLst.clone(), Arc::new(Expression::expCref));
                    BackendVarTransform::removeReplacements(replIn.clone(), crefs.clone())?;
                    repl = collectReplacements1(rest.clone(), replIn.clone())?;
                    Ok(repl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("collectReplacements failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(replOut)
}

fn getOnlyConstantReplacements(mut replIn: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements;
    (crefs, exps) = BackendVarTransform::getAllReplacements(replIn.clone())?;
    (exps, crefs) = List::filterOnTrueSync(exps.clone(), Arc::new(Expression::isConst), crefs.clone())?;
    repl = BackendVarTransform::emptyReplacements();
    replOut = BackendVarTransform::addReplacements(repl.clone(), crefs.clone(), exps.clone(), None)?;
    Ok(replOut)
}

fn updateStatementsInIfStmt(mut stmtLstIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>>, mut origIf: Arc<DAE::Statement>) -> Result<Arc<DAE::Statement>> {
    let mut ifStmtOut: Arc<DAE::Statement>;
    ifStmtOut = (::match_deref::match_deref! { match &((stmtLstIn.clone(), origIf.clone())) {
        (Deref @ metamodelica::List::Cons { head: stmts, tail: rest }, Deref @ DAE::Statement::STMT_IF { source, else_: els, exp, .. }) => {
            let mut els = (*els).clone();
            els = updateStatementsInElse(rest.clone(), els.clone())?;
            Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: stmts.clone(), else_: els.clone(), source: source.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ifStmtOut)
}

fn updateStatementsInElse(mut stmtLstIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>>, mut origElse: Arc<DAE::Else>) -> Result<Arc<DAE::Else>> {
    let mut elseOut: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    elseOut = (::match_deref::match_deref! { match &((stmtLstIn.clone(), origElse.clone())) {
        (Deref @ metamodelica::List::Cons { head: stmts, tail: rest }, Deref @ DAE::Else::ELSEIF { else_: els, exp, .. }) => {
            let mut els = (*els).clone();
            els = updateStatementsInElse(rest.clone(), els.clone())?;
            Arc::new(DAE::Else::ELSEIF { exp: exp.clone(), statementLst: stmts.clone(), else_: els.clone() })
        },
        (Deref @ metamodelica::List::Cons { head: stmts, tail: _ }, Deref @ DAE::Else::ELSE { .. }) => {
            Arc::new(DAE::Else::ELSE { statementLst: stmts.clone() })
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ DAE::Else::NOELSE) => {
            Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(elseOut)
}

fn compareConstantExps(mut expLstLstIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut posLstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in 1..=(listHead(expLstLstIn.clone())?.len() as i32) {
        posLstOut = compareConstantExps2(i.clone(), expLstLstIn.clone(), posLstOut.clone())?;
    }
    Ok(posLstOut)
}

fn compareConstantExps2(mut idx: i32, mut expLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut pos: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut pos: Arc<metamodelica::List<i32>> = pos;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut firstExp: Arc<DAE::Exp>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    expLst = List::map1(expLstLst.clone(), Arc::new(listGet), idx.clone());
    b1 = List::all(expLst.clone(), Arc::new(Expression::isConst));
    if b1.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expLst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        firstExp = __pa0.clone();
        rest = __pa1.clone();
        b2 = List::all(rest.clone(), Arc::new({ let __pe_b1 = firstExp.clone(); move |__pe_a0| ExpressionBasics::expEqual(__pe_a0, __pe_b1.clone()) }));
        if b2.clone() {
            pos = cons(idx.clone(), pos.clone());
        }
    }
    Ok(pos)
}

fn makeAssignmentMap(mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rhs: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    stmts = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for (e1, e2) in (&(lhs.clone())).into_iter().zip((&(rhs.clone())).into_iter()) {
            let __x = makeAssignment(e1.clone(), e2.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(stmts)
}

fn makeAssignment(mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>) -> Result<Arc<DAE::Statement>> {
    let mut stmtOut: Arc<DAE::Statement>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Expression::r#typeof(rhs.clone())?;
    stmtOut = Arc::new(DAE::Statement::STMT_ASSIGN { type_: ty.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: DAE::emptyElementSource.clone() });
    Ok(stmtOut)
}

// =============================================================================
// redeclare the varKinds (maybe some state candidates are vanished)
//
// =============================================================================
fn updateVarKinds(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { shared: __pa0, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    shared = __pa0.clone();
    systs = __pa1.clone();
    systs = List::map1(systs.clone(), Arc::new(updateVarKinds_eqSys), shared.clone());
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

fn updateVarKinds_eqSys(mut sysIn: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut sysOut: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut states: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut ssVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut initEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut derVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut ssVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut derVarsInit: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(sysIn.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    vars = __pa1.clone();
    varLst = BackendVariable::varList(vars.clone())?;
    initEqs = BackendEquation::getInitialEqnsFromShared(shared.clone());
    states = List::filterOnTrue(varLst.clone(), Arc::new(fnptr!(BackendVariable::isStateorStateDerVar, BackendDAE::Var)));
    (_, derVarsInit) = BackendDAEUtil::traverseBackendDAEExpsEqns(initEqs.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (fnptr!(findDerVarCrefs, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), metamodelica::nil()))?;
    (_, derVars) = BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (fnptr!(findDerVarCrefs, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), derVarsInit.clone()))?;
    ssVarLst = List::filterOnTrue(varLst.clone(), Arc::new(varSSisPreferOrHigher));
    ssVars = List::map(ssVarLst.clone(), Arc::new(BackendVariable::varCref));
    derVars = List::unique(listAppend(derVars.clone(), ssVars.clone()));
    (vars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), Arc::new(setVarKindForStates), derVars.clone())?;
    sysOut = BackendDAEUtil::setEqSystVars(sysIn.clone(), vars.clone())?;
    Ok(sysOut)
}

fn varSSisPreferOrHigher(mut varIn: BackendDAE::Var) -> Result<bool> {
    let mut ssOut: bool = false;
    let mut i: i32 = 0;
    let mut ss: DAE::StateSelect = DAE::StateSelect::ALWAYS;
    ss = BackendVariable::varStateSelect(varIn.clone());
    i = BackendVariable::stateSelectToInteger(ss.clone())?;
    ssOut = intGe(i.clone(), 2);
    Ok(ssOut)
}

fn setVarKindForStates(mut inVar: BackendDAE::Var, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(BackendDAE::Var, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outVar: BackendDAE::Var;
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outVar, outCrefs) = 'mc: {
        let __mc_input = (inVar.clone(), inCrefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (varOld @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, varName: cr1, .. }, derVars) => {
                    let mut isState: bool = false;
                    let mut varNew: BackendDAE::Var;
                    isState = List::isMemberOnTrue(cr1.clone(), derVars.clone(), Arc::new(ComponentReferenceBasics::crefEqual));
                    varNew = if (!(isState.clone())) {BackendVariable::setVarKind(varOld.clone(), crate::BackendDAE::VarKind::VARIABLE)?} else {varOld.clone()};
                    Ok((varNew.clone(), derVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outCrefs))
}

fn findDerVarCrefs(mut exp: Arc<DAE::Exp>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outCrefs = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            cons(cr.clone(), inCrefs.clone())
        },
        _ => {
            inCrefs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, outCrefs)
}

// =============================================================================
// convert tuple equations to several single equations
//
// =============================================================================
fn convertTupleEquations(mut eqIn: Arc<BackendDAE::Equation>, mut addEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut eqOut: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut addEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (eqOut, addEqsOut) = (::match_deref::match_deref! { match &(eqIn.clone()) {
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: Deref @ DAE::Exp::TUPLE { PR: rhs }, left: Deref @ DAE::Exp::TUPLE { PR: lhs }, .. } => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut rhs = (*rhs).clone();
            let mut lhs = (*lhs).clone();
            lhs = List::mapFlat(lhs.clone(), Arc::new(Expression::getComplexContents));
            rhs = List::mapFlat(rhs.clone(), Arc::new(Expression::getComplexContents));
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for (lh, rh) in (&(lhs.clone())).into_iter().zip((&(rhs.clone())).into_iter()) {
            let __x = makeBackendEquation(lh.clone(), rh.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eq = __pa0.clone();
            eqs = __pa1.clone();
            (eq.clone(), listAppend(eqs.clone(), addEqsIn.clone()))
        },
        _ => {
            (eqIn.clone(), addEqsIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqOut, addEqsOut))
}

fn makeBackendEquation(mut ls: Arc<DAE::Exp>, mut rs: Arc<DAE::Exp>) -> Arc<BackendDAE::Equation> {
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    eq = Arc::new(BackendDAE::Equation::EQUATION { exp: rs.clone(), scalar: ls.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
    eq
}

