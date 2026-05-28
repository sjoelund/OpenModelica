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
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncInfo {
    pub repl: BackendVarTransform::VariableReplacements,
    pub funcTree: Arc<AvlTreePathFunction::Tree>,
    pub idx: i32,
}

impl Default for FuncInfo {
    fn default() -> Self {
        Self {
            repl: Default::default(),
            funcTree: Default::default(),
            idx: Default::default(),
        }
    }
}

pub type FUNCINFO = FuncInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variability {
    CONST,
    VARIABLE,
}
impl Default for Variability {
    fn default() -> Self { Self::CONST }
}
pub use self::Variability::{CONST,VARIABLE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallSignature {
    pub path: Arc<Absyn::Path>,
    pub inputsVari: Arc<metamodelica::List<Variability>>,
    pub canBeEvaluated: bool,
}

impl Default for CallSignature {
    fn default() -> Self {
        Self {
            path: Default::default(),
            inputsVari: Default::default(),
            canBeEvaluated: Default::default(),
        }
    }
}

pub type SIGNATURE = CallSignature;


// =============================================================================
// caching of already evaluated functions
//
// =============================================================================
fn checkCallSignatureForExp(mut expIn: Arc<DAE::Exp>, mut signLst: Arc<metamodelica::List<CallSignature>>) -> Result<bool> {
    let mut continueEval: bool = false;
    let mut signature: CallSignature = <CallSignature as ::std::default::Default>::default();
    continueEval = true;
    signature = getCallSignatureForCall(expIn.clone())?;
    if List::isMemberOnTrue(signature.clone(), signLst.clone(), (std::sync::Arc::new(callSignatureIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(CallSignature, CallSignature) -> Result<bool> + 'static>)) {
        let CallSignature { canBeEvaluated: __pa0, .. } = (List::getMemberOnTrue(signature.clone(), signLst.clone(), (std::sync::Arc::new(callSignatureIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(CallSignature, CallSignature) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
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
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("[ ")); __mm_s.push_str(&*stringDelimitList(List::map(varis.clone(), (std::sync::Arc::new(fnptr!(VariabilityString, Variability)) as std::sync::Arc<dyn ::std::ops::Fn(Variability) -> Result<ArcStr> + 'static>)), (literal!(" | ")).clone())); __mm_s.push_str(&*literal!(" ] ")); __mm_s.push_str(&*boolString(b.clone())); ArcStr::from(__mm_s) }).clone();
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
        if List::isEqualOnTrue(vari1.clone(), vari2.clone(), (std::sync::Arc::new(fnptr!(VariabilityIsEqual, Variability, Variability)) as std::sync::Arc<dyn ::std::ops::Fn(Variability, Variability) -> Result<bool> + 'static>)) {
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
    let mut signatureOut: CallSignature = <CallSignature as ::std::default::Default>::default();
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
        vari = List::map(expLst.clone(), (std::sync::Arc::new(getVariabilityForExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Variability> + 'static>));
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
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut changed: bool = false;
    let mut eqSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inDAE.clone()) {
            Deref @ DAE { shared: __pa1, eqs: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        shared = __pa1.clone();
        eqSysts = __pa2.clone();
        let (__pa3, (__pa4, _, __pa5, _)) = List::mapFold(eqSysts.clone(), (std::sync::Arc::new(evalFunctions_main) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Arc<BackendDAE::Shared>, i32, bool, Arc<metamodelica::List<CallSignature>>)) -> Result<(Arc<BackendDAE::EqSystem>, (Arc<BackendDAE::Shared>, i32, bool, Arc<metamodelica::List<CallSignature>>))> + 'static>), (shared.clone(), 1, false, metamodelica::nil()));
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
    let mut eqSysOut: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut tplOut: (Arc<BackendDAE::Shared>, i32, bool, Arc<metamodelica::List<CallSignature>>);
    let mut changed: bool = false;
    let mut sysIdx: i32 = 0;
    let mut sharedIn: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
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
                    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut addEqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut addEqs2: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut callSign: Arc<metamodelica::List<CallSignature>> = callSign.clone();
                    let mut idx: i32 = idx.clone();
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
                    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    let mut addEqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut callSign: Arc<metamodelica::List<CallSignature>> = callSign.clone();
                    let mut idx: i32 = idx.clone();
                    let mut changed: bool = changed.clone();
                    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = addEqs.clone();
                    let mut shared: Arc<BackendDAE::Shared> = shared.clone();
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
                    shared = BackendDAEUtil::setSharedFunctionTree(shared.clone(), funcs.clone())?;
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

pub fn evaluateConstantFunctionCallExp(mut expIn: Arc<DAE::Exp>, mut funcsIn: Arc<AvlTreePathFunction::Tree>, mut evalConstArgsOnly: bool, mut recursionLimit: i32) -> Result<Arc<DAE::Exp>> {
    let mut expOut: Arc<DAE::Exp>;
    expOut = 'mc: {
        let __mc_input = (expIn.clone(), funcsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: exps0, path, .. }, _) => {
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut func: DAE::Function;
                    let mut allInputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constInputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constComplexCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut scalarInputs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
                    let mut scalarOutputs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
                    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut protectVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut algs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut allInputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut allOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut allInputExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constInputExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constComplexExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut scalarExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut expOut: Arc<DAE::Exp>;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart constant evaluation of expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(expIn.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if evalConstArgsOnly.clone() {
                        let true = (Expression::isConstWorkList(exps0.clone())?) else { bail!("pattern mismatch") };
                    }
                    let Some(__pa0) = (AvlTreePathFunction::get(funcsIn.clone(), path.clone())?) else { bail!("pattern mismatch") };
                    func = __pa0.clone();
                    let false = (DAEUtil::isExtFunction(func.clone())) else { bail!("pattern mismatch") };
                    elements = DAEUtil::getFunctionElements(func.clone())?;
                    exps = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (exps0.clone()).into_iter().cloned() {
                    let __x = evaluateConstantFunctionCallExp(e.clone(), funcsIn.clone(), evalConstArgsOnly.clone(), recursionLimit.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    scalarExp = List::map1(exps.clone(), (std::sync::Arc::new(expandComplexExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), funcsIn.clone());
                    allInputExps = List::flatten(scalarExp.clone());
                    if elements.clone().is_empty() && DAEUtil::funcIsRecord(func.clone()) {
                        expOut = Arc::new(DAE::Exp::TUPLE { PR: allInputExps.clone() });
                        if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                            println!("{}", (literal!("\nIts a record.\n")).clone());
                        }
                    } else {
                        allInputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isInputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                        scalarInputs = List::map(allInputs.clone(), (std::sync::Arc::new(expandComplexElementsToCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>));
                        allInputCrefs = List::flatten(scalarInputs.clone());
                        protectVars = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isProtectedVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                        algs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isAlgorithm, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                        algs = listAppend(protectVars.clone(), algs.clone());
                        allOutputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                        outputCrefs = List::map(allOutputs.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                        scalarOutputs = List::map(allOutputs.clone(), (std::sync::Arc::new(getScalarsForComplexVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>));
                        allOutputCrefs = listAppend(outputCrefs.clone(), List::flatten(scalarOutputs.clone()));
                        (constInputExps, constInputCrefs) = List::filterOnTrueSync(allInputExps.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), allInputCrefs.clone())?;
                        repl = BackendVarTransform::emptyReplacements();
                        repl = BackendVarTransform::addReplacements(repl.clone(), constInputCrefs.clone(), constInputExps.clone(), None)?;
                        let _ = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasAssertFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                        let _ = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasReturnFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                        let _ = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasReturnFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                        let _ = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasReinitFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                        (algs, _, repl, _) = List::mapFold3(algs.clone(), Arc::new({ let __pe_b4 = recursionLimit.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| evaluateFunctions_updateAlgElements(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), funcsIn.clone(), repl.clone(), 1);
                        (constCrefs, constExps) = BackendVarTransform::getAllReplacements(repl.clone())?;
                        (constCrefs, constExps) = List::filter1OnTrueSync(constCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), allOutputCrefs.clone(), constExps.clone())?;
                        (constExps, constCrefs) = List::filterOnTrueSync(constExps.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), constCrefs.clone())?;
                        (constComplexCrefs, _, constScalarCrefs, varScalarCrefs) = checkIfOutputIsEvaluatedConstant(allOutputs.clone(), constCrefs.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                        constScalarExps = List::map1r(constScalarCrefs.clone(), (std::sync::Arc::new(BackendVarTransform::getReplacement) as std::sync::Arc<dyn ::std::ops::Fn(BackendVarTransform::VariableReplacements, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), repl.clone());
                        constComplexExps = List::map1r(constComplexCrefs.clone(), (std::sync::Arc::new(BackendVarTransform::getReplacement) as std::sync::Arc<dyn ::std::ops::Fn(BackendVarTransform::VariableReplacements, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), repl.clone());
                        (constScalarCrefs, constScalarExps) = List::filter1OnTrueSync(constCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), constScalarCrefs.clone(), constExps.clone())?;
                        (constComplexCrefs, constComplexExps) = List::filter1OnTrueSync(constCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), constComplexCrefs.clone(), constExps.clone())?;
                        if varScalarCrefs.clone().is_empty() && varScalarCrefs.clone().is_empty() && constComplexCrefs.clone().is_empty() && !(constScalarExps.clone().is_empty()) {
                            if (constScalarCrefs.clone().len() as i32) == 1 {
                                        expOut = listHead(constScalarExps.clone())?;
                            } else {
                                        expOut = Arc::new(DAE::Exp::TUPLE { PR: constScalarExps.clone() });
                            }
                        } else if varScalarCrefs.clone().is_empty() && varScalarCrefs.clone().is_empty() && constScalarCrefs.clone().is_empty() && !(constComplexExps.clone().is_empty()) {
                            if (constComplexCrefs.clone().len() as i32) == 1 {
                                        expOut = listHead(constComplexExps.clone())?;
                            } else {
                                        expOut = Arc::new(DAE::Exp::TUPLE { PR: constComplexExps.clone() });
                            }
                        } else {
                            expOut = expIn.clone();
                        }
                    }
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nevaluated to: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(expOut.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(expOut.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { attr: attr1, expLst: exps, path }, sub }, _) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = evaluateConstantFunctionCallExp(Arc::new(DAE::Exp::CALL { attr: attr1.clone(), expLst: exps.clone(), path: path.clone() }), funcsIn.clone(), evalConstArgsOnly.clone(), recursionLimit.clone())?;
                    (exp, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::ASUB { exp: exp.clone(), sub: sub.clone() }))?;
                    if !(Expression::isConst(exp.clone())?) {
                        exp = expIn.clone();
                    }
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

fn hasUnknownType(mut eIn: Arc<DAE::Exp>) -> bool {
    let mut bOut: bool = false;
    bOut = (::match_deref::match_deref! { match &(eIn.clone()) {
        Deref @ DAE::Exp::TUPLE { PR: eLst } => {
            List::any(eLst.clone(), (std::sync::Arc::new(fnptr!(hasUnknownType, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))
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
            List::any(eLst.clone(), (std::sync::Arc::new(hasMultipleArrayDimensions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))
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

pub fn evaluateConstantFunction(mut rhsExpIn: Arc<DAE::Exp>, mut lhsExpIn: Arc<DAE::Exp>, mut funcsIn: Arc<AvlTreePathFunction::Tree>, mut eqIdx: i32, mut callSignLstIn: Arc<metamodelica::List<CallSignature>>, mut recursionLimit: i32) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<AvlTreePathFunction::Tree>, i32, bool, Arc<metamodelica::List<CallSignature>>)> {
    let mut rhsExpOut: Arc<DAE::Exp>;
    let mut lhsExpOut: Arc<DAE::Exp>;
    let mut addedEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut funcsOut: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut eqIdxOut: i32 = 0;
    let mut changed: bool = false;
    let mut callSignLstOut: Arc<metamodelica::List<CallSignature>> = metamodelica::nil();
    let mut funcIsConst: bool = false;
    let mut funcIsPartConst: bool = false;
    let mut isConstRec: bool = false;
    let mut hasAssert: bool = false;
    let mut hasReturn: bool = false;
    let mut hasTerminate: bool = false;
    let mut hasReinit: bool = false;
    let mut abort: bool = false;
    let mut isUnknownType: bool = false;
    let mut isNDimArray: bool = false;
    let mut idx: i32 = 0;
    let mut bList: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut constIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut path: Arc<Absyn::Path>;
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut attr1: Arc<DAE::CallAttributes> = Arc::new(<DAE::CallAttributes as ::std::default::Default>::default());
    let mut attr2: Arc<DAE::CallAttributes> = Arc::new(<DAE::CallAttributes as ::std::default::Default>::default());
    let mut constCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut lhsCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut exp: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let mut constExp: Arc<DAE::Exp>;
    let mut outputExp: Arc<DAE::Exp>;
    let mut func: DAE::Function;
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut singleOutputType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut constEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut inputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut allInputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constInputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constScalarCrefsLhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constComplexCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut varComplexCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut varScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut algs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut allInputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut protectVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut allOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut updatedVarOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut newOutputVars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expsIn: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut inputExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut complexExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut allInputExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut constInputExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut constExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut constComplexExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut constScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut sub: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut scalarExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outputVarTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut outputVarNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut scalarInputs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let mut scalarOutputs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let mut signature: CallSignature = <CallSignature as ::std::default::Default>::default();
    let mut callSignLst: Arc<metamodelica::List<CallSignature>> = metamodelica::nil();
    let mut continueEval: bool = false;
    let true = (recursionLimit.clone() > 0) else { bail!("pattern mismatch") };
    (rhsExpOut, lhsExpOut, addedEquations, funcsOut, eqIdxOut, changed, callSignLstOut) = 'mc: {
        let __mc_input = (rhsExpIn.clone(), lhsExpIn.clone(), funcsIn.clone(), eqIdx.clone(), callSignLstIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: attr1, expLst: expsIn, path }, _, _, _, callSignLst) => {
                    let mut attr1 = (*attr1).clone();
                    let mut path = (*path).clone();
                    let mut callSignLst = (*callSignLst).clone();
                    let mut constInputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = constInputCrefs.clone();
                    let mut protectVars: Arc<metamodelica::List<Arc<DAE::Element>>> = protectVars.clone();
                    let mut isUnknownType: bool = isUnknownType.clone();
                    let mut allInputExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = allInputExps.clone();
                    let mut func: DAE::Function;
                    let mut hasTerminate: bool = hasTerminate.clone();
                    let mut varComplexCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varComplexCrefs.clone();
                    let mut constScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = constScalarExps.clone();
                    let mut signature: CallSignature = signature.clone();
                    let mut hasAssert: bool = hasAssert.clone();
                    let mut lhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = lhsExps.clone();
                    let mut constInputExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = constInputExps.clone();
                    let mut outputVarNames: Arc<metamodelica::List<ArcStr>> = outputVarNames.clone();
                    let mut attr2: Arc<DAE::CallAttributes> = attr2.clone();
                    let mut continueEval: bool = continueEval.clone();
                    let mut updatedVarOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = updatedVarOutputs.clone();
                    let mut singleOutputType: Arc<DAE::Type> = singleOutputType.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut hasReinit: bool = hasReinit.clone();
                    let mut constComplexCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = constComplexCrefs.clone();
                    let mut scalarInputs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = scalarInputs.clone();
                    let mut constScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = constScalarCrefs.clone();
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = exps.clone();
                    let mut allOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = allOutputs.clone();
                    let mut funcIsPartConst: bool = funcIsPartConst.clone();
                    let mut funcIsConst: bool = funcIsConst.clone();
                    let mut constScalarCrefsLhs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = constScalarCrefsLhs.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut exp: Arc<DAE::Exp>;
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut abort: bool = abort.clone();
                    let mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = constCrefs.clone();
                    let mut constComplexExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = constComplexExps.clone();
                    let mut algs: Arc<metamodelica::List<Arc<DAE::Element>>> = algs.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let mut changed: bool = changed.clone();
                    let mut allInputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allInputCrefs.clone();
                    let mut isConstRec: bool = isConstRec.clone();
                    let mut constEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = constEqs.clone();
                    let mut newOutputVars: Arc<metamodelica::List<Arc<DAE::Element>>> = newOutputVars.clone();
                    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefsInFunc.clone();
                    let mut outputVarTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = outputVarTypes.clone();
                    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements.clone();
                    let mut constExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = constExps.clone();
                    let mut funcs: Arc<AvlTreePathFunction::Tree> = funcs.clone();
                    let mut varScalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefs.clone();
                    let mut scalarExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = scalarExp.clone();
                    let mut allInputs: Arc<metamodelica::List<Arc<DAE::Element>>> = allInputs.clone();
                    let mut scalarOutputs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = scalarOutputs.clone();
                    let mut hasReturn: bool = hasReturn.clone();
                    let mut idx: i32 = idx.clone();
                    let mut isNDimArray: bool = isNDimArray.clone();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nStart function evaluation of:\n")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhsExpIn.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhsExpIn.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    continueEval = checkCallSignatureForExp(rhsExpIn.clone(), callSignLst.clone())?;
                    isUnknownType = hasUnknownType(lhsExpIn.clone());
                    isNDimArray = hasMultipleArrayDimensions(lhsExpIn.clone())?;
                    if !(continueEval.clone()) && Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", (literal!("THIS FUNCTION CALL WITH THIS SPECIFIC SIGNATURE CANNOT BE EVALUTED\n")).clone());
                    }
                    if !(continueEval.clone()) || isUnknownType.clone() || isNDimArray.clone() {
                        bail!("fail");
                    }
                    let Some(__pa0) = (AvlTreePathFunction::get(funcsIn.clone(), path.clone())?) else { bail!("pattern mismatch") };
                    func = __pa0.clone();
                    let false = (doNotInline(func.clone())) else { bail!("pattern mismatch") };
                    elements = DAEUtil::getFunctionElements(func.clone())?;
                    protectVars = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isProtectedVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    algs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isAlgorithm, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && elements.clone().is_empty() {
                        println!("{}", (literal!("Its a Record!\n")).clone());
                        let false = (true) else { bail!("pattern mismatch") };
                    } else if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && (protectVars.clone().is_empty() && algs.clone().is_empty()) {
                        println!("{}", (literal!("Its a Built-In!\n")).clone());
                        let false = (true) else { bail!("pattern mismatch") };
                    }
                    let false = (elements.clone().is_empty()) else { bail!("pattern mismatch") };
                    let false = (algs.clone().is_empty()) else { bail!("pattern mismatch") };
                    exps = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (expsIn.clone()).into_iter().cloned() {
                    let __x = evaluateConstantFunctionCallExp(e.clone(), funcsIn.clone(), false, recursionLimit.clone() - 1)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    scalarExp = List::map1(exps.clone(), (std::sync::Arc::new(expandComplexExpressions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), funcsIn.clone());
                    allInputExps = List::flatten(scalarExp.clone());
                    allInputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isInputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    scalarInputs = List::map(allInputs.clone(), (std::sync::Arc::new(expandComplexElementsToCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>));
                    allInputCrefs = List::flatten(scalarInputs.clone());
                    allOutputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    outputCrefs = List::map(allOutputs.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    scalarOutputs = List::map(allOutputs.clone(), (std::sync::Arc::new(getScalarsForComplexVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>));
                    allOutputCrefs = listAppend(outputCrefs.clone(), List::flatten(scalarOutputs.clone()));
                    (constInputExps, constInputCrefs) = List::filterOnTrueSync(allInputExps.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), allInputCrefs.clone())?;
                    repl = BackendVarTransform::emptyReplacements();
                    repl = BackendVarTransform::addReplacements(repl.clone(), constInputCrefs.clone(), constInputExps.clone(), None)?;
                    hasAssert = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasAssertFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                    hasReturn = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasReturnFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                    hasTerminate = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasReturnFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                    hasReinit = List::fold(algs.clone(), (std::sync::Arc::new(fnptr!(hasReinitFold, Arc<DAE::Element>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, bool) -> Result<bool> + 'static>), false);
                    abort = hasReturn.clone() || hasTerminate.clone() || hasReinit.clone();
                    (algs, funcs, repl, idx) = List::mapFold3(algs.clone(), Arc::new({ let __pe_b4 = recursionLimit.clone() - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| evaluateFunctions_updateAlgElements(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone()) }), funcsIn.clone(), repl.clone(), eqIdx.clone());
                    (constCrefs, constExps) = BackendVarTransform::getAllReplacements(repl.clone())?;
                    (constCrefs, constExps) = List::filter1OnTrueSync(constCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), allOutputCrefs.clone(), constExps.clone())?;
                    (constExps, constCrefs) = List::filterOnTrueSync(constExps.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), constCrefs.clone())?;
                    (constComplexCrefs, varComplexCrefs, constScalarCrefs, varScalarCrefs) = checkIfOutputIsEvaluatedConstant(allOutputs.clone(), constCrefs.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                    (constScalarCrefs, constScalarExps) = List::filter1OnTrueSync(constCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), constScalarCrefs.clone(), constExps.clone())?;
                    (constComplexCrefs, constComplexExps) = List::filter1OnTrueSync(constCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), constComplexCrefs.clone(), constExps.clone())?;
                    funcIsConst = varScalarCrefs.clone().is_empty() && varComplexCrefs.clone().is_empty() && (!(constScalarCrefs.clone().is_empty()) || !(constComplexCrefs.clone().is_empty()));
                    funcIsPartConst = (!(varScalarCrefs.clone().is_empty()) || !(varComplexCrefs.clone().is_empty())) && (!(constScalarCrefs.clone().is_empty()) || !(constComplexCrefs.clone().is_empty())) && !(funcIsConst.clone());
                    isConstRec = intEq((constScalarCrefs.clone().len() as i32), (List::flatten(scalarOutputs.clone()).len() as i32)) && varScalarCrefs.clone().is_empty() && varComplexCrefs.clone().is_empty() && constComplexCrefs.clone().is_empty();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        if funcIsConst.clone() {
                            if hasAssert.clone() {
                                        println!("{}", (literal!("the function output is completely constant but there is an assertion\n")).clone());
                            } else {
                                        println!("{}", (literal!("the function output is completely constant\n")).clone());
                            }
                        } else if !(funcIsPartConst.clone()) {
                            println!("{}", (literal!("the function output is not constant in any case\n")).clone());
                        }
                        if abort.clone() {
                            println!("{}", (literal!("the evaluated function is not used because there is a return or a terminate or a reinit statement\n")).clone());
                        }
                    }
                    funcIsConst = if (hasAssert.clone() && funcIsConst.clone() || abort.clone()) {false} else {funcIsConst.clone()};
                    funcIsPartConst = if (hasAssert.clone() && funcIsConst.clone()) {true} else {funcIsPartConst.clone()};
                    funcIsPartConst = if (abort.clone()) {false} else {funcIsPartConst.clone()};
                    let true = (funcIsPartConst.clone() || funcIsConst.clone()) else { bail!("pattern mismatch") };
                    signature = getCallSignatureForCall(rhsExpIn.clone())?;
                    signature.canBeEvaluated = true;
                    callSignLst = cons(signature.clone(), callSignLst.clone());
                    changed = funcIsPartConst.clone() || funcIsConst.clone();
                    (updatedVarOutputs, outputExp, varScalarCrefsInFunc) = buildVariableFunctionParts(scalarOutputs.clone(), constComplexCrefs.clone(), varComplexCrefs.clone(), constScalarCrefs.clone(), varScalarCrefs.clone(), allOutputs.clone(), lhsExpIn.clone())?;
                    (constScalarCrefsLhs, constComplexCrefs) = buildConstFunctionCrefs(constScalarCrefs.clone(), constComplexCrefs.clone(), allOutputCrefs.clone(), lhsExpIn.clone())?;
                    if !(funcIsConst.clone()) {
                        (algs, constEqs) = buildPartialFunction((varScalarCrefsInFunc.clone(), algs.clone()), (constScalarCrefs.clone(), constScalarExps.clone(), constComplexCrefs.clone(), constComplexExps.clone(), constScalarCrefsLhs.clone()), repl.clone())?;
                    } else {
                        constEqs = metamodelica::nil();
                    }
                    elements = listAppend(protectVars.clone(), algs.clone());
                    elements = listAppend(updatedVarOutputs.clone(), elements.clone());
                    elements = listAppend(allInputs.clone(), elements.clone());
                    elements = List::unique(elements.clone());
                    (func, path) = updateFunctionBody(func.clone(), elements.clone(), idx.clone(), updatedVarOutputs.clone(), allOutputs.clone())?;
                    funcs = if (funcIsPartConst.clone()) {DAEUtil::addDaeFunction(list![func.clone()], funcs.clone())?} else {funcs.clone()};
                    idx = if (funcIsPartConst.clone() || funcIsConst.clone()) {idx.clone() + 1} else {idx.clone()};
                    outputExp = if (funcIsPartConst.clone()) {outputExp.clone()} else {lhsExpIn.clone()};
                    lhsExps = getCrefsForRecord(lhsExpIn.clone())?;
                    outputExp = if (isConstRec.clone()) {Arc::new(DAE::Exp::TUPLE { PR: lhsExps.clone() })} else {outputExp.clone()};
                    newOutputVars = List::filterOnTrue(updatedVarOutputs.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    outputVarTypes = List::map(newOutputVars.clone(), (std::sync::Arc::new(DAEUtil::getVariableType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::Type>> + 'static>));
                    outputVarNames = List::map(newOutputVars.clone(), (std::sync::Arc::new(DAEUtil::varName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<ArcStr> + 'static>));
                    attr2 = DAEUtil::replaceCallAttrType(attr1.clone(), Arc::new(DAE::Type::T_TUPLE { types: outputVarTypes.clone(), names: Some(outputVarNames.clone()) }));
                    let __pa1 = ::match_deref::match_deref! { match &(attr1.clone()) {
                        Deref @ DAE::CallAttributes { ty: __pa1, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    singleOutputType = __pa1.clone();
                    singleOutputType = if (!(newOutputVars.clone().is_empty())) {listHead(outputVarTypes.clone())?} else {singleOutputType.clone()};
                    attr1 = DAEUtil::replaceCallAttrType(attr1.clone(), singleOutputType.clone());
                    attr2 = if (intEq((newOutputVars.clone().len() as i32), 1)) {attr1.clone()} else {attr2.clone()};
                    if List::hasOneElement(listAppend(constComplexExps.clone(), constScalarExps.clone())) && funcIsConst.clone() {
                        exp = listHead(listAppend(constComplexExps.clone(), constScalarExps.clone()))?;
                    } else if funcIsConst.clone() && !(List::hasOneElement(listAppend(constComplexExps.clone(), constScalarExps.clone()))) {
                        exp = Arc::new(DAE::Exp::TUPLE { PR: listAppend(constComplexExps.clone(), constScalarExps.clone()) });
                    } else {
                        exp = rhsExpIn.clone();
                    }
                    exp = if (funcIsPartConst.clone()) {Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expsIn.clone(), attr: attr2.clone() })} else {exp.clone()};
                    exp = if (isConstRec.clone()) {Arc::new(DAE::Exp::TUPLE { PR: constScalarExps.clone() })} else {exp.clone()};
                    outputExp = setRecordTypes(outputExp.clone())?;
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Finish evaluation of:\n")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhsExpIn.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhsExpIn.clone())?); __mm_s.push_str(&*literal!("\nto:\n")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outputExp.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        if !(constEqs.clone().is_empty()) {
                            BackendDump::dumpEquationList(constEqs.clone(), (literal!("including the additional equations:\n")).clone());
                        }
                    }
                    Ok((exp.clone(), outputExp.clone(), constEqs.clone(), funcs.clone(), idx.clone(), changed.clone(), callSignLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { attr: attr1, expLst: exps, path }, sub }, _, _, _, callSignLst) => {
                    let mut changed: bool = changed.clone();
                    let mut exp: Arc<DAE::Exp>;
                    let mut continueEval: bool = continueEval.clone();
                    exp = Arc::new(DAE::Exp::CALL { attr: attr1.clone(), expLst: exps.clone(), path: path.clone() });
                    continueEval = checkCallSignatureForExp(exp.clone(), callSignLst.clone())?;
                    if !(continueEval.clone()) && Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", (literal!("THIS FUNCTION CALL WITH THIS SPECIFIC SIGNATURE CANNOT BE EVALUTED\n")).clone());
                    }
                    if !(continueEval.clone()) {
                        bail!("fail");
                    }
                    exp = evaluateConstantFunctionCallExp(exp.clone(), funcsIn.clone(), false, recursionLimit.clone())?;
                    (exp, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::ASUB { exp: exp.clone(), sub: sub.clone() }))?;
                    changed = true;
                    if !(Expression::isConst(exp.clone())?) {
                        exp = rhsExpIn.clone();
                        changed = false;
                    }
                    Ok((exp.clone(), lhsExpIn.clone(), metamodelica::nil(), funcsIn.clone(), eqIdx.clone(), changed.clone(), callSignLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut callSignLst: Arc<metamodelica::List<CallSignature>> = callSignLst.clone();
                    let mut signature: CallSignature = signature.clone();
                    callSignLst = callSignLstIn.clone();
                    if Expression::isCall(rhsExpIn.clone()) {
                        signature = getCallSignatureForCall(rhsExpIn.clone())?;
                        signature.canBeEvaluated = false;
                        if !(List::isMemberOnTrue(signature.clone(), callSignLstIn.clone(), (std::sync::Arc::new(callSignatureIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(CallSignature, CallSignature) -> Result<bool> + 'static>))) {
                            callSignLst = cons(signature.clone(), callSignLst.clone());
                        }
                    }
                    Ok((rhsExpIn.clone(), lhsExpIn.clone(), metamodelica::nil(), funcsIn.clone(), eqIdx.clone(), false, callSignLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((rhsExpOut, lhsExpOut, addedEquations, funcsOut, eqIdxOut, changed, callSignLstOut))
}

fn expandComplexExpressions(mut e: Arc<DAE::Exp>, mut funcs: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut eLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    eLst = 'mc: {
        let __mc_input = (e.clone(), funcs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: lst, path, .. }, _) => {
                    let mut func: DAE::Function;
                    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut allOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut lst = (*lst).clone();
                    let Some(__pa0) = (AvlTreePathFunction::get(funcs.clone(), path.clone())?) else { bail!("pattern mismatch") };
                    func = __pa0.clone();
                    elements = DAEUtil::getFunctionElements(func.clone())?;
                    if elements.clone().is_empty() {
                    } else {
                        let Some(__pa1) = (AvlTreePathFunction::get(funcs.clone(), path.clone())?) else { bail!("pattern mismatch") };
                        func = __pa1.clone();
                        elements = DAEUtil::getFunctionElements(func.clone())?;
                        allOutputs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                        lst = List::map(List::flatten(List::map(allOutputs.clone(), (std::sync::Arc::new(getScalarsForComplexVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>))), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    }
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    lst = Expression::getComplexContents(e.clone())?;
                    let false = (lst.clone().is_empty()) else { bail!("pattern mismatch") };
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![e.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eLst)
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
        bLst = List::map(stmtLst.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isStmtAssert, Arc<DAE::Statement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<bool> + 'static>));
        bOut = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), bIn.clone());
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
        bLst = List::map(stmtLst.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isStmtReturn, Arc<DAE::Statement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<bool> + 'static>));
        bOut = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), bIn.clone());
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
        bLst = List::map(stmtLst.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isStmtReturn, Arc<DAE::Statement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<bool> + 'static>));
        bOut = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), bIn.clone());
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
                    expLst = List::map(expLst.clone(), (std::sync::Arc::new(setRecordTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
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
            expLst = List::map(crefs.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
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
            if List::isMemberOnTrue(cref.clone(), constScalarCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefEqualWithoutSubs, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>)) {
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
                    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefsInFunc.clone();
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    let mut protCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = protCrefs.clone();
                    let mut pos: Arc<metamodelica::List<i32>> = pos.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    varScalarCrefsInFunc = metamodelica::nil();
                    allOutputCrefs = List::map(allOutputs.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(constComplexCrefs.clone(), allOutputCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    pos = List::map1(outputCrefs.clone(), (std::sync::Arc::new(List::position) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static>), allOutputCrefs.clone());
                    varScalarExps = List::map1(pos.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    outputExp = if (List::hasOneElement(varScalarExps.clone())) {listHead(varScalarExps.clone())?} else {Arc::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    funcOutputs = List::map2(outputCrefs.clone(), (std::sync::Arc::new(generateOutputElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(protCrefs.clone(), (std::sync::Arc::new(generateProtectedElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
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
                    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = varOutputs.clone();
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let mut protCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = protCrefs.clone();
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut pos: Arc<metamodelica::List<i32>> = pos.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefsInFunc.clone();
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    let mut allOutputCrefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs2.clone();
                    allOutputCrefs = List::map(allOutputs.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    allOutputCrefs2 = List::map(allOutputCrefs.clone(), (std::sync::Arc::new(scalarRecCrefsForOneDimRec) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    (_, _, varScalarCrefsInFunc) = List::intersection1OnTrue(allOutputCrefs.clone(), allOutputCrefs2.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    allOutputCrefs = allOutputCrefs2.clone();
                    if partiallyConstantArrayNeedsExpansion(allOutputCrefs.clone(), constScalarCrefs.clone())? {
                        if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                            println!("{}", (literal!("A partially constant array needs expansion. Thats not supported.\n")).clone());
                        }
                        bail!("fail");
                    }
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(listAppend(constComplexCrefs.clone(), constScalarCrefs.clone()), allOutputCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    funcOutputs = List::map2(outputCrefs.clone(), (std::sync::Arc::new(generateOutputElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(protCrefs.clone(), (std::sync::Arc::new(generateProtectedElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    varOutputs = listAppend(funcOutputs.clone(), funcProts.clone());
                    pos = List::map1(outputCrefs.clone(), (std::sync::Arc::new(List::position) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static>), allOutputCrefs.clone());
                    varScalarExps = List::map1(pos.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    varScalarExps = List::map(varScalarExps.clone(), (std::sync::Arc::new(scalarRecExpForOneDimRec) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    outputExp = if (List::hasOneElement(varScalarExps.clone())) {listHead(varScalarExps.clone())?} else {Arc::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    Ok((varOutputs.clone(), outputExp.clone(), varScalarCrefsInFunc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, Deref @ DAE::Exp::TUPLE { PR: expLst }) => {
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = varOutputs.clone();
                    let mut varScalarCrefsInFunc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefsInFunc.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut pos: Arc<metamodelica::List<i32>> = pos.clone();
                    let mut protCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = protCrefs.clone();
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let true = (List::flatten(scalarOutputs.clone()).is_empty()) else { bail!("pattern mismatch") };
                    let true = (!(constScalarCrefs.clone().is_empty())) else { bail!("pattern mismatch") };
                    varScalarCrefsInFunc = metamodelica::nil();
                    allOutputCrefs = List::map(allOutputs.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    (protCrefs, _, outputCrefs) = List::intersection1OnTrue(constScalarCrefs.clone(), allOutputCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    pos = List::map1(outputCrefs.clone(), (std::sync::Arc::new(List::position) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static>), allOutputCrefs.clone());
                    varScalarExps = List::map1(pos.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    outputExp = if (List::hasOneElement(varScalarExps.clone())) {listHead(varScalarExps.clone())?} else {Arc::new(DAE::Exp::TUPLE { PR: varScalarExps.clone() })};
                    funcOutputs = List::map2(outputCrefs.clone(), (std::sync::Arc::new(generateOutputElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(protCrefs.clone(), (std::sync::Arc::new(generateProtectedElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
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
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outputCrefs.clone();
                    let mut lhsCref: Arc<DAE::ComponentRef> = lhsCref.clone();
                    lhsCref = Expression::expCref(lhsExpIn.clone())?;
                    outputCrefs = List::map(constScalarCrefs.clone(), (std::sync::Arc::new(ComponentReference::crefStripFirstIdent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    outputCrefs = List::map1(outputCrefs.clone(), (std::sync::Arc::new(ComponentReference::joinCrefsR) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), lhsCref.clone());
                    expLst = List::map(outputCrefs.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    outputExp = Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() });
                    Ok((metamodelica::nil(), outputExp.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut funcOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcOutputs.clone();
                    let mut lhsCref: Arc<DAE::ComponentRef> = lhsCref.clone();
                    let mut varOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = varOutputs.clone();
                    let mut funcProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcProts.clone();
                    let mut outputExp: Arc<DAE::Exp>;
                    let mut varScalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = varScalarExps.clone();
                    let mut funcSOutputs: Arc<metamodelica::List<Arc<DAE::Element>>> = funcSOutputs.clone();
                    let mut allOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = allOutputCrefs.clone();
                    let mut varScalarCrefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = varScalarCrefs1.clone();
                    let mut funcSProts: Arc<metamodelica::List<Arc<DAE::Element>>> = funcSProts.clone();
                    lhsCref = Expression::expCref(lhsExpIn.clone())?;
                    allOutputCrefs = List::map(allOutputs.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    funcOutputs = List::map2(varComplexCrefs.clone(), (std::sync::Arc::new(generateOutputElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    funcProts = List::map2(constComplexCrefs.clone(), (std::sync::Arc::new(generateProtectedElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    funcSOutputs = List::map2(varScalarCrefs.clone(), (std::sync::Arc::new(generateOutputElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    funcSProts = List::map2(constScalarCrefs.clone(), (std::sync::Arc::new(generateProtectedElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> + 'static>), allOutputs.clone(), lhsExpIn.clone());
                    varOutputs = List::flatten(list![funcOutputs.clone(), funcSOutputs.clone(), funcProts.clone(), funcSProts.clone()]);
                    varScalarCrefs1 = List::map(varScalarCrefs.clone(), (std::sync::Arc::new(ComponentReference::crefStripFirstIdent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    varScalarCrefs1 = List::map1(varScalarCrefs1.clone(), (std::sync::Arc::new(ComponentReference::joinCrefsR) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), lhsCref.clone());
                    varScalarExps = List::map(varScalarCrefs1.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
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
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n scalarOutputs \n")); __mm_s.push_str(&*stringDelimitList(List::map(List::flatten(scalarOutputs.clone()), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n constScalarCrefs \n")); __mm_s.push_str(&*stringDelimitList(List::map(constScalarCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
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
                    constCrefs = List::map(constScalarCrefs.clone(), (std::sync::Arc::new(ComponentReference::crefStripFirstIdent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    constCrefs = List::map1(constCrefs.clone(), (std::sync::Arc::new(ComponentReference::joinCrefsR) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), lhsCref.clone());
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
                        pos = cons(List::position1OnTrue(allOutputCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), lhsCref.clone()), pos.clone());
                    }
                    pos = pos.clone().reverse();
                    constExps = List::map1(pos.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), expLst.clone());
                    constCrefs = List::map(constExps.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
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
                    (constVars, varVars, constCrefs1) = List::intersection1OnTrue(list![cref.clone()], constCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    if constVars.clone().is_empty() {
                        scalars = getScalarsForComplexVar(elem.clone())?;
                        if scalars.clone().is_empty() {
                            (constCompl, varCompl, constScalar, varScalar) = (constComplexLstIn.clone(), listAppend(varVars.clone(), varComplexLstIn.clone()), constScalarLstIn.clone(), varScalarLstIn.clone());
                        } else {
                            (constVars, varVars, constCrefs1) = List::intersection1OnTrue(scalars.clone(), constCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
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
                    constVars = List::intersectionOnTrue(scalars.clone(), constCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>));
                    r#const = intEq((scalars.clone().len() as i32), (constVars.clone().len() as i32));
                    constScalarCrefs = List::filter1OnTrue(constCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), constVars.clone());
                    (_, varCrefs, _) = List::intersection1OnTrue(scalars.clone(), constScalarCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
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
                    assign_variant_field!(ty => DAE::Type::T_FUNCTION; funcResultType = if (intEq((outTypeLst.clone().len() as i32), 1)) {listHead(outTypeLst.clone())?} else {Arc::new(DAE::Type::T_TUPLE { types: outTypeLst.clone(), names: Some(outNames.clone()) })});
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
    funcAlgs = List::filterOnTrue(funcAlgs.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isAlgorithm, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
    lhsExps1 = List::map(constScalarCrefsOut.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
    lhsExps2 = List::map(constComplCrefs.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
    eqsOut = generateConstEqs(lhsExps1.clone(), constScalarExps.clone(), metamodelica::nil())?;
    eqsOut = generateConstEqs(lhsExps2.clone(), constComplExps.clone(), eqsOut.clone())?;
    stmts1 = List::mapFlatReverse(funcAlgs.clone(), (std::sync::Arc::new(DAEUtil::getStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> + 'static>));
    (stmts1, _) = DAEUtil::traverseDAEEquationsStmts(stmts1.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(makeIdentCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), varScalarCrefs.clone()));
    (stmts1, _) = DAEUtil::traverseDAEEquationsStmts(stmts1.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(makeIdentCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), constScalarCrefs.clone()));
    algsOut = list![Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts1.clone() }), source: DAE::emptyElementSource().clone() })];
    Ok((algsOut, eqsOut))
}

fn stmtCanBeRemoved(mut stmtIn: Arc<DAE::Statement>, mut repl: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Statement>, bool)> {
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
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Statement>, bool)> + 'static>;

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
                    stmtLstLst = List::map3(stmtLstLst.clone(), (std::sync::Arc::new(traverseStmtsAndUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Statement>>>, _, _, Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> + 'static>), func.clone(), argIn.clone(), metamodelica::nil());
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
                    let true = (List::isMemberOnTrue(cref1.clone(), changeTheseCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
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
            eq = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
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
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    replOut = (::match_deref::match_deref! { match &((stmt.clone(), replIn.clone())) {
        (Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: lhs, .. }, _) => {
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
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

fn evaluateFunctions_updateAlgElements(mut element: Arc<DAE::Element>, mut funcTree: Arc<AvlTreePathFunction::Tree>, mut repl: BackendVarTransform::VariableReplacements, mut idx: i32, mut recursionLimit: i32) -> Result<(Arc<DAE::Element>, Arc<AvlTreePathFunction::Tree>, BackendVarTransform::VariableReplacements, i32)> {
    let mut element: Arc<DAE::Element> = element;
    let mut funcTree: Arc<AvlTreePathFunction::Tree> = funcTree;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut idx: i32 = idx;
    element = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut alg = (*alg).clone();
            stmts = DAEUtil::getStatement(element.clone())?;
            (stmts, funcTree, repl, idx) = evaluateFunctions_updateStatement(stmts.clone(), funcTree.clone(), repl.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
            alg = Arc::new(DAE::Algorithm { statementLst: stmts.clone() });
            Arc::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::VAR { binding: Some(exp), componentRef: cref, .. } => {
            let mut scalarExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut scalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut exp = (*exp).clone();
            (exp, _) = BackendVarTransform::replaceExp(exp.clone(), repl.clone(), None)?;
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            if Expression::isConst(exp.clone())? {
                repl = BackendVarTransform::addReplacement(repl.clone(), cref.clone(), exp.clone(), None)?;
                scalars = ComponentReference::expandCref(cref.clone(), false)?;
                scalarExps = Expression::getComplexContents(exp.clone())?;
                if (scalars.clone().len() as i32) == (scalarExps.clone().len() as i32) {
                    repl = BackendVarTransform::addReplacements(repl.clone(), scalars.clone(), scalarExps.clone(), None)?;
                }
            }
            DAEUtil::replaceBindungInVar(exp.clone(), element.clone())?
        },
        _ => {
            element.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((element, funcTree, repl, idx))
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

fn evaluateFunctions_updateStatement(mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut funcTree: Arc<AvlTreePathFunction::Tree>, mut repl: BackendVarTransform::VariableReplacements, mut idx: i32, mut lstIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut recursionLimit: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<AvlTreePathFunction::Tree>, BackendVarTransform::VariableReplacements, i32)> {
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts;
    let mut funcTree: Arc<AvlTreePathFunction::Tree> = funcTree;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut idx: i32 = idx;
    let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
    stmtsList = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
        for mut stmt in (stmts.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { source, exp: exp2, exp1, type_: typ } => {
            let mut isCon: bool = false;
            let mut isRec: bool = false;
            let mut isTpl: bool = false;
            let mut eqDim: bool = false;
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut stmt1: Arc<DAE::Statement>;
            let mut scalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut varScalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut constScalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut outputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut addStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut tplStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tplExpsLHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tplExpsRHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut lhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut exp2 = (*exp2).clone();
            let mut exp1 = (*exp1).clone();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("assignment:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            cref = Expression::expCref(exp1.clone())?;
            scalars = getRecordScalars(cref.clone());
            (exp2, _) = BackendVarTransform::replaceExp(exp2.clone(), repl.clone(), None)?;
            (exp2, _) = ExpressionSimplify::simplify(exp2.clone())?;
            (exp2, exp1, funcTree, idx, addStmts) = evaluateConstantFunctionCall(exp2.clone(), exp1.clone(), funcTree.clone(), idx.clone(), recursionLimit.clone())?;
            (exp2, _) = ExpressionSimplify::simplify(exp2.clone())?;
            (exp2, _) = Expression::traverseExpBottomUp(exp2.clone(), (std::sync::Arc::new(fnptr!(unboxExp, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            expLst = Expression::getComplexContents(exp2.clone())?;
            repl = List::fold(addStmts.clone(), (std::sync::Arc::new(addReplacementRuleForAssignment) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> + 'static>), repl.clone());
            lhsExps = Expression::getComplexContents(exp1.clone())?;
            outputs = List::map(lhsExps.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
            BackendVarTransform::removeReplacements(repl.clone(), outputs.clone())?;
            isCon = Expression::isConst(exp2.clone())? && !(Expression::isCall(exp2.clone()));
            eqDim = (scalars.clone().len() as i32) == (expLst.clone().len() as i32);
            isRec = ComponentReference::isRecord(cref.clone()) || Expression::isRecordCall(exp2.clone(), funcTree.clone())?;
            isTpl = Expression::isTuple(exp1.clone()) && Expression::isTuple(exp2.clone());
            scalars = if (isRec.clone() && eqDim.clone()) {scalars.clone()} else {metamodelica::nil()};
            expLst = if (isRec.clone() && eqDim.clone()) {expLst.clone()} else {metamodelica::nil()};
            (_, varScalars) = List::filterOnTrueSync(expLst.clone(), (std::sync::Arc::new(Expression::isNotConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), scalars.clone())?;
            (expLst, constScalars) = List::filterOnTrueSync(expLst.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), scalars.clone())?;
            repl = if (isCon.clone() && !(isRec.clone())) {BackendVarTransform::addReplacement(repl.clone(), cref.clone(), exp2.clone(), None)?} else {repl.clone()};
            repl = if (isCon.clone() && isRec.clone()) {BackendVarTransform::addReplacements(repl.clone(), scalars.clone(), expLst.clone(), None)?} else {repl.clone()};
            if !(isCon.clone()) {
                if !(isRec.clone()) {
                    BackendVarTransform::removeReplacement(repl.clone(), cref.clone())?;
                } else {
                    BackendVarTransform::removeReplacements(repl.clone(), varScalars.clone())?;
                    repl = BackendVarTransform::addReplacements(repl.clone(), constScalars.clone(), expLst.clone(), None)?;
                }
            }
            stmt1 = if (isCon.clone()) {Arc::new(DAE::Statement::STMT_ASSIGN { type_: typ.clone(), exp1: exp1.clone(), exp: exp2.clone(), source: source.clone() })} else {stmt.clone()};
            tplExpsLHS = if (isTpl.clone()) {Expression::getComplexContents(exp1.clone())?} else {metamodelica::nil()};
            tplExpsRHS = if (isTpl.clone()) {Expression::getComplexContents(exp2.clone())?} else {metamodelica::nil()};
            tplStmts = makeAssignmentMap(tplExpsLHS.clone(), tplExpsRHS.clone())?;
            stmts1 = if (isTpl.clone()) {tplStmts.clone()} else {list![stmt1.clone()]};
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluated assignment to:\n")); __mm_s.push_str(&*stringDelimitList(List::map(stmts1.clone(), (std::sync::Arc::new(DAEDump::ppStatementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            stmts1.clone()
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: exp2, lhs: exp1, type_: typ } => {
            let mut isCon: bool = false;
            let mut isRec: bool = false;
            let mut isTpl: bool = false;
            let mut eqDim: bool = false;
            let mut isArr: bool = false;
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut stmt1: Arc<DAE::Statement>;
            let mut scalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut varScalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut constScalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut outputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut addStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut tplStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tplExpsLHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tplExpsRHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut lhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut exp2 = (*exp2).clone();
            let mut exp1 = (*exp1).clone();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Array assignment:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            cref = Expression::expCref(exp1.clone())?;
            scalars = getRecordScalars(cref.clone());
            (exp2, _) = BackendVarTransform::replaceExp(exp2.clone(), repl.clone(), None)?;
            (exp2, exp1, funcTree, idx, addStmts) = evaluateConstantFunctionCall(exp2.clone(), exp1.clone(), funcTree.clone(), idx.clone(), recursionLimit.clone())?;
            (exp2, _) = ExpressionSimplify::simplify(exp2.clone())?;
            expLst = Expression::getComplexContents(exp2.clone())?;
            repl = List::fold(addStmts.clone(), (std::sync::Arc::new(addReplacementRuleForAssignment) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> + 'static>), repl.clone());
            lhsExps = Expression::getComplexContents(exp1.clone())?;
            outputs = List::map(lhsExps.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
            BackendVarTransform::removeReplacements(repl.clone(), outputs.clone())?;
            isCon = Expression::isConst(exp2.clone())? && !(Expression::isCall(exp2.clone()));
            eqDim = (scalars.clone().len() as i32) == (expLst.clone().len() as i32);
            isRec = ComponentReference::isRecord(cref.clone());
            isArr = ComponentReference::isArrayElement(cref.clone());
            isTpl = Expression::isTuple(exp1.clone()) && Expression::isTuple(exp2.clone());
            scalars = if ((isRec.clone() || isArr.clone()) && eqDim.clone()) {scalars.clone()} else {metamodelica::nil()};
            expLst = if ((isRec.clone() || isArr.clone()) && eqDim.clone()) {expLst.clone()} else {metamodelica::nil()};
            (_, varScalars) = List::filterOnTrueSync(expLst.clone(), (std::sync::Arc::new(Expression::isNotConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), scalars.clone())?;
            (expLst, constScalars) = List::filterOnTrueSync(expLst.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), scalars.clone())?;
            repl = if (isCon.clone() && !(isRec.clone())) {BackendVarTransform::addReplacement(repl.clone(), cref.clone(), exp2.clone(), None)?} else {repl.clone()};
            repl = if (isCon.clone() && isRec.clone()) {BackendVarTransform::addReplacements(repl.clone(), scalars.clone(), expLst.clone(), None)?} else {repl.clone()};
            repl = if (isCon.clone() && isArr.clone()) {BackendVarTransform::addReplacements(repl.clone(), scalars.clone(), expLst.clone(), None)?} else {repl.clone()};
            if !(isCon.clone()) {
                if !(isRec.clone()) {
                    BackendVarTransform::removeReplacement(repl.clone(), cref.clone())?;
                } else {
                    BackendVarTransform::removeReplacements(repl.clone(), varScalars.clone())?;
                    repl = BackendVarTransform::addReplacements(repl.clone(), constScalars.clone(), expLst.clone(), None)?;
                }
            }
            stmt1 = if (isCon.clone()) {Arc::new(DAE::Statement::STMT_ASSIGN { type_: typ.clone(), exp1: exp1.clone(), exp: exp2.clone(), source: source.clone() })} else {stmt.clone()};
            tplExpsLHS = if (isTpl.clone()) {Expression::getComplexContents(exp1.clone())?} else {metamodelica::nil()};
            tplExpsRHS = if (isTpl.clone()) {Expression::getComplexContents(exp2.clone())?} else {metamodelica::nil()};
            tplStmts = makeAssignmentMap(tplExpsLHS.clone(), tplExpsRHS.clone())?;
            stmts1 = if (isTpl.clone()) {tplStmts.clone()} else {list![stmt1.clone()]};
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluated array assignment to:\n")); __mm_s.push_str(&*stringDelimitList(List::map(stmts1.clone(), (std::sync::Arc::new(DAEDump::ppStatementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            stmts1.clone()
        },
        Deref @ DAE::Statement::STMT_IF { else_, statementLst: stmtsIf, .. } => {
            let mut predicted: bool = false;
            let mut isEval: bool = false;
            let mut outputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut addStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut stmtsNew: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut allStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IF-statement:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            stmtsList = getDAEelseStatemntLsts(else_.clone(), metamodelica::nil());
            stmtsList = stmtsList.clone().reverse();
            stmtsList = cons(stmtsIf.clone(), stmtsList.clone());
            allStmts = List::flatten(stmtsList.clone());
            outputs = getStatementsOutputs(allStmts.clone(), funcTree.clone())?;
            (isEval, stmts1, repl) = evaluateIfStatement(stmt.clone(), FuncInfo { repl: repl.clone(), funcTree: funcTree.clone(), idx: idx.clone() }, recursionLimit.clone())?;
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isEval.clone()) {
                println!("{}", (literal!("-->try to predict the outputs \n")).clone());
            }
            if !(isEval.clone()) {
                let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(predictIfOutput(stmt.clone(), FuncInfo { repl: repl.clone(), funcTree: funcTree.clone(), idx: idx.clone() }, recursionLimit.clone())?) {
                    ((__pa0, __pa1), FuncInfo { repl: __pa2, funcTree: __pa3, idx: __pa4 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                stmtsNew = __pa0.clone();
                addStmts = __pa1.clone();
                repl = __pa2.clone();
                funcTree = __pa3.clone();
                idx = __pa4.clone();
            } else {
                stmtsNew = stmts1.clone();
                addStmts = metamodelica::nil();
            }
            predicted = !(addStmts.clone().is_empty()) || stmtsNew.clone().is_empty() && !(isEval.clone());
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? && !(isEval.clone()) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("could it be predicted? ")); __mm_s.push_str(&*boolString(predicted.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            if !(predicted.clone()) && !(isEval.clone()) {
                BackendVarTransform::removeReplacements(repl.clone(), outputs.clone())?;
            }
            stmts1 = if (predicted.clone()) {stmtsNew.clone()} else {stmts1.clone()};
            (addStmts, funcTree, repl, idx) = evaluateFunctions_updateStatement(addStmts.clone(), funcTree.clone(), repl.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluated IF-statements to:\n")); __mm_s.push_str(&*stringDelimitList(List::map(listAppend(stmts1.clone(), addStmts.clone()), (std::sync::Arc::new(DAEDump::ppStatementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            listAppend(stmts1.clone(), addStmts.clone())
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: exp0, expExpLst: expLst, .. } => {
            let mut isCon: bool = false;
            let mut size: i32 = 0;
            let mut exp1: Arc<DAE::Exp>;
            let mut exp2: Arc<DAE::Exp>;
            let mut typ: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut varScalars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut stmtsNew: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut tplExpsLHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tplExpsRHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tuple-statement:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            (exp1, _) = BackendVarTransform::replaceExp(exp0.clone(), repl.clone(), None)?;
            exp2 = Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() });
            (exp1, exp2, addEqs, funcTree, idx, _, _) = evaluateConstantFunction(exp1.clone(), exp2.clone(), funcTree.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
            isCon = Expression::isConst(exp1.clone())?;
            exp1 = if (isCon.clone()) {exp1.clone()} else {exp0.clone()};
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--> is the tuple const? ")); __mm_s.push_str(&*boolString(isCon.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            varScalars = List::map(expLst.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
            if !(isCon.clone()) {
                BackendVarTransform::removeReplacements(repl.clone(), varScalars.clone())?;
            } else {
                repl = addTplReplacements(repl.clone(), exp1.clone(), exp2.clone())?;
            }
            size = DAEUtil::getTupleSize(exp2.clone());
            typ = Expression::r#typeof(exp2.clone())?;
            tplExpsLHS = DAEUtil::getTupleExps(exp2.clone());
            tplExpsLHS = if (isCon.clone()) {tplExpsLHS.clone()} else {metamodelica::nil()};
            tplExpsRHS = DAEUtil::getTupleExps(exp1.clone());
            tplExpsRHS = if (isCon.clone()) {tplExpsRHS.clone()} else {metamodelica::nil()};
            stmtsNew = makeAssignmentMap(tplExpsLHS.clone(), tplExpsRHS.clone())?;
            stmtsNew = if (isCon.clone()) {stmtsNew.clone()} else {list![stmt.clone()]};
            stmts2 = if (intEq(size.clone(), 0)) {list![Arc::new(DAE::Statement::STMT_ASSIGN { type_: typ.clone(), exp1: exp2.clone(), exp: exp1.clone(), source: DAE::emptyElementSource().clone() })]} else {stmtsNew.clone()};
            stmts1 = List::map(addEqs.clone(), (std::sync::Arc::new(equationToStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Statement>> + 'static>));
            stmts1 = listAppend(stmts2.clone(), stmts1.clone());
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluated Tuple-statements to (incl. addEqs):\n")); __mm_s.push_str(&*stringDelimitList(List::map(stmts1.clone(), (std::sync::Arc::new(DAEDump::ppStatementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            stmts1.clone().reverse()
        },
        Deref @ DAE::Statement::STMT_FOR { statementLst: stmts1, .. } => {
            let mut stmts1 = (*stmts1).clone();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("For-statement:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            (stmts1, funcTree, repl, idx) = evaluateForStatement(stmt.clone(), funcTree.clone(), repl.clone(), idx.clone(), recursionLimit.clone())?;
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluated for-statements to:\n")); __mm_s.push_str(&*stringDelimitList(List::map(stmts1.clone(), (std::sync::Arc::new(DAEDump::ppStatementStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            stmts1.clone().reverse()
        },
        Deref @ DAE::Statement::STMT_WHILE { statementLst: stmts1, .. } => {
            let mut outputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("While-statement (not evaluated):\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            outputs = getStatementsOutputs(stmts1.clone(), funcTree.clone())?;
            BackendVarTransform::removeReplacements(repl.clone(), outputs.clone())?;
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluated While-statement to:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            list![stmt.clone()]
        },
        Deref @ DAE::Statement::STMT_ASSERT { level: lvl, msg, cond, .. } => {
            let mut msg = (*msg).clone();
            let mut cond = (*cond).clone();
            (cond, _) = BackendVarTransform::replaceExp(cond.clone(), repl.clone(), None)?;
            cond = evaluateConstantFunctionCallExp(cond.clone(), funcTree.clone(), false, recursionLimit.clone())?;
            (cond, _) = ExpressionSimplify::simplify(cond.clone())?;
            (msg, _) = BackendVarTransform::replaceExp(msg.clone(), repl.clone(), None)?;
            msg = evaluateConstantFunctionCallExp(msg.clone(), funcTree.clone(), false, recursionLimit.clone())?;
            (msg, _) = ExpressionSimplify::simplify(msg.clone())?;
            if ExpressionBasics::expEqual(cond.clone(), Arc::new(DAE::Exp::BCONST { bool: false }))? && Expression::sconstEnumNameString(lvl.clone())? == literal!("AssertionLevel.error") {
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ERROR: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(msg.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                bail!("fail");
            } else if ExpressionBasics::expEqual(cond.clone(), Arc::new(DAE::Exp::BCONST { bool: false }))? && Expression::sconstEnumNameString(lvl.clone())? == literal!("AssertionLevel.warning") {
                if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WARNING: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(msg.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                bail!("fail");
            }
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("assert-statement:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            list![stmt.clone()]
        },
        Deref @ DAE::Statement::STMT_TERMINATE { .. } => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("terminate-statement:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            list![stmt.clone()]
        },
        Deref @ DAE::Statement::STMT_REINIT { .. } => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("reinit-statement:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            list![stmt.clone()]
        },
        Deref @ DAE::Statement::STMT_NORETCALL { .. } => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("noretcall-statement (not evaluated):\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            list![stmt.clone()]
        },
        Deref @ DAE::Statement::STMT_RETURN { .. } => {
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("return-statement:\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            list![stmt.clone()]
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    stmts = List::flatten(stmtsList.clone());
    Ok((stmts, funcTree, repl, idx))
}

fn evaluateForStatement(mut stmtIn: Arc<DAE::Statement>, mut funcTreeIn: Arc<AvlTreePathFunction::Tree>, mut replIn: BackendVarTransform::VariableReplacements, mut idxIn: i32, mut recursionLimit: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<AvlTreePathFunction::Tree>, BackendVarTransform::VariableReplacements, i32)> {
    let mut stmtsOut: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut funcTreeOut: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut idxOut: i32 = 0;
    let mut hasNoRepl: bool = false;
    let mut i: i32 = 0;
    let mut start: i32 = 0;
    let mut stop: i32 = 0;
    let mut step: i32 = 0;
    let mut iter: ArcStr = arcstr::literal!("");
    let mut range: Arc<DAE::Exp>;
    let mut outputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut lhsExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lhsExpLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut stmtsIn: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(stmtIn.clone()) {
        Deref @ DAE::Statement::STMT_FOR { statementLst: __pa0, range: __pa1, iter: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stmtsIn = __pa0.clone();
    range = __pa1.clone();
    iter = __pa2.clone();
    match '__try3: {
        (range, _) = unwrap_break_err!(BackendVarTransform::replaceExp(range.clone(), replIn.clone(), None), '__try3);
        (start, stop, step) = unwrap_break_err!(getRangeBounds(range.clone()), '__try3);
        let true = (intEq(step.clone(), 1)) else { break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        let true = (intGe(stop.clone(), start.clone())) else { break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        repl = replIn.clone();
        for mut i in start.clone()..=stop.clone() {
            repl = unwrap_break_err!(BackendVarTransform::addReplacement(repl.clone(), ComponentReferenceBasics::makeCrefIdent((iter.clone()).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil()), Arc::new(DAE::Exp::ICONST { integer: i.clone() }), None), '__try3);
            (stmts, _, repl, _) = unwrap_break_err!(evaluateFunctions_updateStatement(stmtsIn.clone(), funcTreeIn.clone(), repl.clone(), i.clone(), metamodelica::nil(), recursionLimit.clone()), '__try3);
            outputs = unwrap_break_err!(getStatementsOutputs(stmts.clone(), funcTreeIn.clone()), '__try3);
            hasNoRepl = List::applyAndFold1(outputs.clone(), (std::sync::Arc::new(fnptr!(boolAnd, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(BackendVarTransform::hasNoReplacement, Arc<DAE::ComponentRef>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, BackendVarTransform::VariableReplacements) -> Result<bool> + 'static>), repl.clone(), true);
            if hasNoRepl.clone() {
                if unwrap_break_err!(Flags::isSet(Flags::EVAL_FUNC_DUMP.clone()), '__try3) {
                    println!("{}", (literal!("For-loop evaluation is skipped, since the first loop evaluated nothing.\n")).clone());
                }
                break '__try3 Err::<_, _>(anyhow::anyhow!("fail"));
            }
        }
        unwrap_break_err!(BackendVarTransform::removeReplacement(repl.clone(), ComponentReferenceBasics::makeCrefIdent((iter.clone()).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil())), '__try3);
        funcTreeOut = funcTreeIn.clone();
        idxOut = idxIn.clone();
        stmtsOut = stmts.clone();
        Ok::<_, anyhow::Error>((funcTreeOut.clone(), idxOut.clone(), repl.clone(), stmtsOut.clone()))
    } {
        Ok((__try3_o0, __try3_o1, __try3_o2, __try3_o3)) => {
            funcTreeOut = __try3_o0;
            idxOut = __try3_o1;
            repl = __try3_o2;
            stmtsOut = __try3_o3;
        }
        Err(_) => {
            lhsExps = List::fold(stmtsIn.clone(), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), metamodelica::nil());
            lhsExps = List::unique(lhsExps.clone());
            lhsExpLst = List::map(lhsExps.clone(), (std::sync::Arc::new(Expression::getComplexContents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
            lhsExps = listAppend(List::flatten(lhsExpLst.clone()), lhsExps.clone());
            outputs = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut e in (lhsExps.clone()).into_iter().cloned() {
            if !(Expression::isCref(e.clone())) { continue; }
            let __x = Expression::expCref(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            repl = replIn.clone();
            BackendVarTransform::removeReplacements(repl.clone(), outputs.clone())?;
            stmtsOut = list![stmtIn.clone()];
            funcTreeOut = funcTreeIn.clone();
            idxOut = idxIn.clone();
        }
    }
    Ok((stmtsOut, funcTreeOut, repl, idxOut))
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
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (isEval, stmtsOut, replOut) = 'mc: {
        let __mc_input = (stmtIn.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { else_, statementLst: stmtsIf, exp: expIf, .. }, FuncInfo { idx, funcTree, repl: replIn }) => {
                    let mut isIf: bool = false;
                    let mut isCon: bool = false;
                    let mut isElse: bool = false;
                    let mut eval: bool = false;
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut exp1: Arc<DAE::Exp>;
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtsElse: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut idx = (*idx).clone();
                    let mut funcTree = (*funcTree).clone();
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", (literal!("-->try to check if its the if case\n")).clone());
                    }
                    (exp1, _) = BackendVarTransform::replaceExp(expIf.clone(), replIn.clone(), None)?;
                    (exp1, _, _, _, _) = evaluateConstantFunctionCall(exp1.clone(), exp1.clone(), funcTree.clone(), idx.clone(), recursionLimit.clone())?;
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
            (exp1, _, _, _, _) = evaluateConstantFunctionCall(expIf.clone(), expIf.clone(), funcTree.clone(), idx.clone(), recursionLimit.clone())?;
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
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    replOut = 'mc: {
        let __mc_input = (replIn.clone(), e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut tplLHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tplRHS: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    tplRHS = DAEUtil::getTupleExps(e1.clone());
                    tplLHS = DAEUtil::getTupleExps(e2.clone());
                    crefs = List::map(tplLHS.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
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
    (expsOut, _) = List::map2_2(expsIn.clone(), (std::sync::Arc::new(BackendVarTransform::replaceExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendVarTransform::VariableReplacements, Option<BackendVarTransform::FuncTypeExp_ExpToBoolean>) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), replIn.clone(), None);
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
            expLst = List::fold(stmtLst2.clone(), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_FOR { statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = List::fold(stmtLst1.clone(), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_PARFOR { statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = List::fold(stmtLst1.clone(), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_WHILE { statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = List::fold(stmtLst1.clone(), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), expsIn.clone());
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(stmt1), statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" check getStatementLHS for WHEN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            expLst = List::fold(stmtLst1.clone(), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), expsIn.clone());
            expLst = getStatementLHS(stmt1.clone(), expLst.clone())?;
            expLst.clone()
        },
        (Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst: stmtLst1, .. }, _) => {
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" check getStatementLHS for WHEN!\n")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone());
            }
            expLst = List::fold(stmtLst1.clone(), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), expsIn.clone());
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

fn getStatementLHSScalar(mut stmt: Arc<DAE::Statement>, mut funcTree: Arc<AvlTreePathFunction::Tree>, mut expsIn: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    lhs = 'mc: {
        let __mc_input = (stmt.clone(), funcTree.clone(), expsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { exp: Deref @ DAE::Exp::CALL { path, .. }, exp1: exp, .. }, _, _) => {
                    let mut lhsCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut func: DAE::Function;
                    let mut outputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut algs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut stmtLst1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmtLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
                    let Some(__pa0) = (AvlTreePathFunction::get(funcTree.clone(), path.clone())?) else { bail!("pattern mismatch") };
                    func = __pa0.clone();
                    elements = DAEUtil::getFunctionElements(func.clone())?;
                    algs = List::filterOnTrue(elements.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isAlgorithm, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
                    stmtLstLst = List::map(algs.clone(), (std::sync::Arc::new(DAEUtil::getStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> + 'static>));
                    stmtLst1 = List::flatten(stmtLstLst.clone());
                    expLst = List::fold1(stmtLst1.clone(), (std::sync::Arc::new(getStatementLHSScalar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), funcTree.clone(), metamodelica::nil());
                    outputCrefs = List::map(expLst.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    lhsCref = Expression::expCref(exp.clone())?;
                    outputCrefs = List::filterOnTrue(outputCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReference::crefIsNotIdent, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>));
                    outputCrefs = List::map(outputCrefs.clone(), (std::sync::Arc::new(ComponentReference::crefStripFirstIdent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    outputCrefs = List::map1(outputCrefs.clone(), (std::sync::Arc::new(ComponentReference::joinCrefsR) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), lhsCref.clone());
                    expLst = List::map(outputCrefs.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>));
                    Ok(listAppend(expLst.clone(), expsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp, .. }, _, _) => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expLst = Expression::getComplexContents(exp.clone())?;
                    Ok(listAppend(expLst.clone(), expsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expLst = getStatementLHS(stmt.clone(), metamodelica::nil())?;
                    Ok(listAppend(expLst.clone(), expsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(lhs)
}

fn getStatementsOutputs(mut statements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut funcTree: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outputs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut lhs_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lhs_set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::Exp>>>), i32, i32, (HashSetExp::FuncHashCref, HashSetExp::FuncCrefEqual, HashSetExp::FuncCrefStr));
    lhs_expl = List::fold1(statements.clone(), (std::sync::Arc::new(getStatementLHSScalar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), funcTree.clone(), metamodelica::nil());
    lhs_set = HashSetExp::emptyHashSetSized(Util::nextPrime((lhs_expl.clone().len() as i32)));
    lhs_set = List::fold(lhs_expl.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), lhs_set.clone());
    outputs = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut e in (BaseHashSet::hashSetList(lhs_set.clone())?).into_iter().cloned() {
            let __x = Expression::expCref(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outputs)
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

fn evaluateConstantFunctionCall(mut exp: Arc<DAE::Exp>, mut lhs: Arc<DAE::Exp>, mut funcs: Arc<AvlTreePathFunction::Tree>, mut eqIdx: i32, mut recursionLimit: i32) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>, i32, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outLhs: Arc<DAE::Exp>;
    let mut outFuncs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut outEqIdx: i32 = 0;
    let mut addedStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let (__pa0, (__pa1, __pa2, __pa3, __pa4)) = Expression::traverseExpTopDown(exp.clone(), Arc::new({ let __pe_b2 = recursionLimit.clone(); move |__pe_a0, __pe_a1| evaluateConstantFunction_traverser(__pe_a0, __pe_a1, __pe_b2.clone()) }), (lhs.clone(), funcs.clone(), eqIdx.clone(), metamodelica::nil()))?;
    outExp = __pa0.clone();
    outLhs = __pa1.clone();
    outFuncs = __pa2.clone();
    outEqIdx = __pa3.clone();
    addedStmts = __pa4.clone();
    Ok((outExp, outLhs, outFuncs, outEqIdx, addedStmts))
}

fn evaluateConstantFunction_traverser(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>, i32, Arc<metamodelica::List<Arc<DAE::Statement>>>), mut recursionLimit: i32) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>, i32, Arc<metamodelica::List<Arc<DAE::Statement>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>, i32, Arc<metamodelica::List<Arc<DAE::Statement>>>);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { .. }, (lhs, funcs, idx, stmtsIn)) => {
                    let mut rhs: Arc<DAE::Exp>;
                    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut lhs = (*lhs).clone();
                    let mut funcs = (*funcs).clone();
                    let mut idx = (*idx).clone();
                    (rhs, lhs, addEqs, funcs, idx, _, _) = evaluateConstantFunction(inExp.clone(), lhs.clone(), funcs.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
                    stmts = List::map(addEqs.clone(), (std::sync::Arc::new(equationToStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Statement>> + 'static>));
                    Ok((rhs.clone(), true, (lhs.clone(), funcs.clone(), idx.clone(), listAppend(stmts.clone(), stmtsIn.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: rhs, .. }, _) => {
                    let mut tpl: (Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>, i32, Arc<metamodelica::List<Arc<DAE::Statement>>>);
                    let mut rhs = (*rhs).clone();
                    (rhs, _, tpl) = evaluateConstantFunction_traverser(rhs.clone(), inTpl.clone(), recursionLimit.clone())?;
                    Ok((rhs.clone(), true, tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
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
                    names = List::map(varLst.clone(), (std::sync::Arc::new(DAEUtil::typeVarIdent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    types = List::map(varLst.clone(), (std::sync::Arc::new(DAEUtil::varType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>));
                    crefs = List::map1(names.clone(), (std::sync::Arc::new(ComponentReference::appendStringCref) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cref.clone());
                    crefs = setTypesForScalarCrefs(crefs.clone(), types.clone())?;
                    crefLst = List::map1(crefs.clone(), (std::sync::Arc::new(ComponentReference::expandCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), true);
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
                    crefs = List::map1r(subslst.clone(), (std::sync::Arc::new(ComponentReference::subscriptCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cref.clone());
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
                    crefs = List::map1r(subslst.clone(), (std::sync::Arc::new(ComponentReference::subscriptCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cref.clone());
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
                    crefs = List::map1r(subslst.clone(), (std::sync::Arc::new(ComponentReference::subscriptCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cref.clone());
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
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the array cref before\n")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
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
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update getScalarsForComplexVar for enumerations: the enum cref is :")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
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
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update getScalarsForComplexVar for tuple types: the tupl cref is :\n")); __mm_s.push_str(&*stringDelimitList(List::map(list![cref.clone()], (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
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
    subsOut = ({
        let mut subFold: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &((dims.clone(), subsIn.clone())) {
        (Deref @ metamodelica::List::Cons { head: dim, tail: rest }, _) => {
            let mut size: i32 = 0;
            let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut sub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut subsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            let mut subsLst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
            size = Expression::dimensionSize(dim.clone())?;
            range = List::intRange(size.clone());
            subs = List::map(range.clone(), (std::sync::Arc::new(fnptr!(Expression::intSubscript, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Subscript>> + 'static>));
            subsLst = List::map(subs.clone(), std::sync::Arc::new(fnptr!(List::create, _)));
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
            subsIn.clone()
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok(subsOut)
}

fn subsLstString(mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(subs.clone(), (std::sync::Arc::new(ExpressionDump::subscriptString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
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
                    dimints = List::map(dims.clone(), (std::sync::Arc::new(Expression::dimensionSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>));
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
                    dimints = List::map(dims.clone(), (std::sync::Arc::new(Expression::dimensionSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>));
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
fn evaluateFunctions_updateStatementEmptyRepl(mut algsIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inFuncTree: Arc<AvlTreePathFunction::Tree>, mut inIndex: i32, mut recursionLimit: i32) -> Result<((Arc<metamodelica::List<Arc<DAE::Statement>>>, BackendVarTransform::VariableReplacements), Arc<AvlTreePathFunction::Tree>, i32)> {
    let mut mapTplOut: (Arc<metamodelica::List<Arc<DAE::Statement>>>, BackendVarTransform::VariableReplacements);
    let mut outFuncTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut outIndex: i32 = 0;
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut algsOut: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    repl = BackendVarTransform::emptyReplacements();
    (algsOut, outFuncTree, repl, outIndex) = evaluateFunctions_updateStatement(algsIn.clone(), inFuncTree.clone(), repl.clone(), inIndex.clone(), metamodelica::nil(), recursionLimit.clone())?;
    mapTplOut = (algsOut.clone(), repl.clone());
    Ok((mapTplOut, outFuncTree, outIndex))
}

fn evaluateFunctions_updateAllStatements(mut stmtsIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut elseStmtsLstIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>>, mut replIn: BackendVarTransform::VariableReplacements, mut funcTree: Arc<AvlTreePathFunction::Tree>, mut idx: i32, mut recursionLimit: i32) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>>, Arc<AvlTreePathFunction::Tree>, i32)> {
    let mut stmtsLstOut: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
    let mut funcTree: Arc<AvlTreePathFunction::Tree> = funcTree;
    let mut idx: i32 = idx;
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    repl = getOnlyConstantReplacements(replIn.clone())?;
    (stmts, funcTree, _, idx) = evaluateFunctions_updateStatement(stmtsIn.clone(), funcTree.clone(), repl.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
    stmtsLstOut = list![stmts.clone()];
    for mut elseStmts in &*elseStmtsLstIn.clone() {
        let mut elseStmts = elseStmts.clone();
        repl = getOnlyConstantReplacements(replIn.clone())?;
        (stmts, funcTree, _, idx) = evaluateFunctions_updateStatement(elseStmts.clone(), funcTree.clone(), repl.clone(), idx.clone(), metamodelica::nil(), recursionLimit.clone())?;
        stmtsLstOut = cons(stmts.clone(), stmtsLstOut.clone());
    }
    stmtsLstOut = stmtsLstOut.clone().reverse();
    Ok((stmtsLstOut, funcTree, idx))
}

fn predictIfOutput(mut stmtIn: Arc<DAE::Statement>, mut infoIn: FuncInfo, mut recursionLimit: i32) -> Result<((Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Statement>>>), FuncInfo)> {
    let mut stmtsOut: (Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Statement>>>);
    let mut infoOut: FuncInfo = <FuncInfo as ::std::default::Default>::default();
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
                    replLst = List::map(stmtsLst.clone(), (std::sync::Arc::new(collectReplacements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<BackendVarTransform::VariableReplacements> + 'static>));
                    expLst = List::fold(List::flatten(stmtsLst.clone()), (std::sync::Arc::new(getStatementLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), metamodelica::nil());
                    expLst = List::unique(expLst.clone());
                    allLHS = expLst.clone().reverse();
                    expLstLst = List::map1(replLst.clone(), (std::sync::Arc::new(fnptr!(replaceExps, BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<DAE::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(BackendVarTransform::VariableReplacements, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), allLHS.clone());
                    constantOutputs = compareConstantExps(expLstLst.clone())?;
                    outExps = List::map1(constantOutputs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), allLHS.clone());
                    let _ = List::map(outExps.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    expLst = List::map1(constantOutputs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), listHead(expLstLst.clone())?);
                    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--> the predicted const outputs:\n")); __mm_s.push_str(&*stringDelimitList(List::map(outExps.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone());
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
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    repl = BackendVarTransform::emptyReplacements();
    replOut = collectReplacements1(stmtsIn.clone(), repl.clone())?;
    Ok(replOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn collectReplacements1(mut stmtsIn: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut replIn: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
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
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
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
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut varCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut rhsLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut constExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut rhs = (*rhs).clone();
                    (rhs, _) = BackendVarTransform::replaceExp(rhs.clone(), replIn.clone(), None)?;
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    rhsLst = Expression::getComplexContents(rhs.clone())?;
                    crefs = List::map(lhsLst.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    (constExps, constCrefs) = List::filterOnTrueSync(rhsLst.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), crefs.clone())?;
                    (_, varCrefs) = List::filterOnTrueSync(rhsLst.clone(), (std::sync::Arc::new(Expression::isNotConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), crefs.clone())?;
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
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut lhsLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    lhsLst = getStatementLHS(stmt.clone(), metamodelica::nil())?;
                    crefs = List::map(lhsLst.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
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
    let mut replOut: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (crefs, exps) = BackendVarTransform::getAllReplacements(replIn.clone())?;
    (exps, crefs) = List::filterOnTrueSync(exps.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), crefs.clone())?;
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
    expLst = List::map1(expLstLst.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), idx.clone());
    b1 = List::all(expLst.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
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
    stmtOut = Arc::new(DAE::Statement::STMT_ASSIGN { type_: ty.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: DAE::emptyElementSource().clone() });
    Ok(stmtOut)
}

// =============================================================================
// redeclare the varKinds (maybe some state candidates are vanished)
//
// =============================================================================
fn updateVarKinds(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { shared: __pa0, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    shared = __pa0.clone();
    systs = __pa1.clone();
    systs = List::map1(systs.clone(), (std::sync::Arc::new(updateVarKinds_eqSys) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), shared.clone());
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

fn updateVarKinds_eqSys(mut sysIn: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut sysOut: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
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
    states = List::filterOnTrue(varLst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateorStateDerVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    (_, derVarsInit) = BackendDAEUtil::traverseBackendDAEExpsEqns(initEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(findDerVarCrefs, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil()))?;
    (_, derVars) = BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(findDerVarCrefs, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), derVarsInit.clone()))?;
    ssVarLst = List::filterOnTrue(varLst.clone(), (std::sync::Arc::new(varSSisPreferOrHigher) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    ssVars = List::map(ssVarLst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>));
    derVars = List::unique(listAppend(derVars.clone(), ssVars.clone()));
    (vars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(setVarKindForStates) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(BackendDAE::Var, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), derVars.clone())?;
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
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outVar, outCrefs) = 'mc: {
        let __mc_input = (inVar.clone(), inCrefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (varOld @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, varName: cr1, .. }, derVars) => {
                    let mut isState: bool = false;
                    let mut varNew: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    isState = List::isMemberOnTrue(cr1.clone(), derVars.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>));
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
            lhs = List::mapFlat(lhs.clone(), (std::sync::Arc::new(Expression::getComplexContents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
            rhs = List::mapFlat(rhs.clone(), (std::sync::Arc::new(Expression::getComplexContents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
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
    eq = Arc::new(BackendDAE::Equation::EQUATION { exp: rs.clone(), scalar: ls.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
    eq
}

