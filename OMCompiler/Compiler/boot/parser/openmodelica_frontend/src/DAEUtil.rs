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

use crate::Algorithm;
use crate::AvlSetCR;
use crate::Ceval;
use crate::ComponentReference;
use crate::ConnectUtil;
use crate::DAEDump;
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::FCore;
use crate::HashTable2;
use crate::HashTable;
use crate::StateMachineFlatten;
use crate::Types;
use crate::ValuesUtil;
use crate::VarTransform;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub fn constStr(mut r#const: DAE::Const) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match r#const.clone() {
        DAE::Const::C_VAR => literal!("VAR"),
        DAE::Const::C_PARAM => literal!("PARAM"),
        DAE::Const::C_CONST => literal!("CONST"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn const2VarKind(mut r#const: DAE::Const) -> Result<DAE::VarKind> {
    let mut kind: DAE::VarKind = DAE::VarKind::CONST;
    kind = (match r#const.clone() {
        DAE::Const::C_VAR => openmodelica_frontend_types::DAE::VarKind::VARIABLE,
        DAE::Const::C_PARAM => openmodelica_frontend_types::DAE::VarKind::PARAM,
        DAE::Const::C_CONST => openmodelica_frontend_types::DAE::VarKind::CONST,
        _ => bail!("match: no arm matched"),
    });
    Ok(kind)
}

pub fn topLevelInput(mut componentRef: Arc<DAE::ComponentRef>, mut varDirection: DAE::VarDirection, mut connectorType: Arc<DAE::ConnectorType>, mut visibility: DAE::VarVisibility) -> Result<bool> {
    let mut isTopLevel: bool = false;
    let mut newInst: bool = Flags::isSet(Flags::SCODE_INST.clone())?;
    isTopLevel = (::match_deref::match_deref! { match &((varDirection.clone(), componentRef.clone(), visibility.clone(), newInst.clone())) {
        (_, _, DAE::VarVisibility::PROTECTED, _) => false,
        (DAE::VarDirection::INPUT, _, _, true) => true,
        (DAE::VarDirection::INPUT, Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, _) => true,
        (DAE::VarDirection::INPUT, _, _, _) if (ConnectUtil::faceEqual(ConnectUtil::componentFaceType(componentRef.clone())?, openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE)) => topLevelConnectorType(connectorType.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isTopLevel)
}

pub fn topLevelOutput(mut componentRef: Arc<DAE::ComponentRef>, mut varDirection: DAE::VarDirection, mut connectorType: Arc<DAE::ConnectorType>) -> Result<bool> {
    let mut isTopLevel: bool = false;
    isTopLevel = (::match_deref::match_deref! { match &((varDirection.clone(), componentRef.clone())) {
        (DAE::VarDirection::OUTPUT, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => true,
        (DAE::VarDirection::OUTPUT, _) if (ConnectUtil::faceEqual(ConnectUtil::componentFaceType(componentRef.clone())?, openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE)) => topLevelConnectorType(connectorType.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isTopLevel)
}

fn topLevelConnectorType(mut inConnectorType: Arc<DAE::ConnectorType>) -> bool {
    let mut isTopLevel: bool = false;
    isTopLevel = (::match_deref::match_deref! { match &(inConnectorType.clone()) {
        Deref @ DAE::ConnectorType::FLOW => true,
        Deref @ DAE::ConnectorType::POTENTIAL => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTopLevel
}

pub fn expTypeSimple(mut tp: Arc<DAE::Type>) -> bool {
    let mut isSimple: bool = false;
    isSimple = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => true,
        Deref @ DAE::Type::T_INTEGER { .. } => true,
        Deref @ DAE::Type::T_STRING { .. } => true,
        Deref @ DAE::Type::T_BOOL { .. } => true,
        Deref @ DAE::Type::T_CLOCK { .. } => true,
        Deref @ DAE::Type::T_ENUMERATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isSimple
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn expTypeElementType(mut tp: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut eltTp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    eltTp = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, .. } => {
            expTypeElementType(ty.clone())
        },
        _ => {
            tp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eltTp
}

pub fn expTypeComplex(mut tp: Arc<DAE::Type>) -> bool {
    let mut isComplex: bool = false;
    isComplex = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_COMPLEX { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isComplex
}

pub fn expTypeArray(mut tp: Arc<DAE::Type>) -> bool {
    let mut isArray: bool = false;
    isArray = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}

pub fn expTypeTuple(mut tp: Arc<DAE::Type>) -> bool {
    let mut isTuple: bool = false;
    isTuple = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_TUPLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTuple
}

pub fn expTypeArrayDimensions(mut tp: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    dims = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: array_dims, .. } => {
            dims = List::map(array_dims.clone(), Arc::new(Expression::dimensionSize));
            dims.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(dims)
}

pub fn typeExp(mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: rest }, .. } => {
            exp = dimExp(dim.clone())?;
            for mut d in &*rest.clone() {
                let mut d = d.clone();
                exp = Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: DAE::T_INTEGER_DEFAULT.clone() }, exp2: dimExp(d.clone())? });
            }
            exp.clone()
        },
        _ => {
            Arc::new(DAE::Exp::ICONST { integer: 1 })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn dimExp(mut dim: Arc<DAE::Dimension>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: iconst } => {
            Arc::new(DAE::Exp::ICONST { integer: iconst.clone() })
        },
        Deref @ DAE::Dimension::DIM_EXP { exp } => {
            exp.clone()
        },
        _ => {
            Error::addMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(anyString(dim.clone())).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn derivativeOrder(mut e1: (i32, DAE::derivativeCond), mut e2: (i32, DAE::derivativeCond)) -> bool {
    let mut b: bool = false;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    b = (match (e1.clone(), e2.clone()) {
        ((mut i1, _), (mut i2, _)) => Util::isIntGreater(i1.clone(), i2.clone()),
    });
    b
}

pub fn getDerivativePaths(mut inFuncDefs: Arc<metamodelica::List<DAE::FunctionDefinition>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = 'mc: {
        let __mc_input = inFuncDefs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DER_MAPPER { lowerOrderDerivatives: pLst1, defaultDerivative: Some(p2), derivativeFunction: p1, .. }, tail: funcDefs } => {
                    let mut pLst2: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = paths.clone();
                    pLst2 = getDerivativePaths(funcDefs.clone())?;
                    paths = List::union(cons(p1.clone(), cons(p2.clone(), pLst1.clone())), pLst2.clone());
                    Ok(paths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DER_MAPPER { lowerOrderDerivatives: pLst1, defaultDerivative: None, derivativeFunction: p1, .. }, tail: funcDefs } => {
                    let mut pLst2: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = paths.clone();
                    pLst2 = getDerivativePaths(funcDefs.clone())?;
                    paths = List::union(cons(p1.clone(), pLst1.clone()), pLst2.clone());
                    Ok(paths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: funcDefs } => {
                    Ok(getDerivativePaths(funcDefs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(paths)
}

pub fn addEquationBoundString(mut bindExp: Arc<DAE::Exp>, mut attr: Option<Arc<DAE::VariableAttributes>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut oattr: Option<Arc<DAE::VariableAttributes>> = None;
    oattr = (::match_deref::match_deref! { match &((bindExp.clone(), attr.clone())) {
        (_, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: e1, unit: e2, displayUnit: e3, min, max, start: e4, fixed: e5, nominal: e6, stateSelectOption: sSelectOption, uncertainOption: unc, distributionOption: distOption, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so })) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: e1.clone(), unit: e2.clone(), displayUnit: e3.clone(), min: min.clone(), max: max.clone(), start: e4.clone(), fixed: e5.clone(), nominal: e6.clone(), stateSelectOption: sSelectOption.clone(), uncertainOption: unc.clone(), distributionOption: distOption.clone(), equationBound: Some(bindExp.clone()), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (_, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: e1, min, max, start: e2, fixed: e3, uncertainOption: unc, distributionOption: distOption, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so })) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: e1.clone(), min: min.clone(), max: max.clone(), start: e2.clone(), fixed: e3.clone(), uncertainOption: unc.clone(), distributionOption: distOption.clone(), equationBound: Some(bindExp.clone()), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (_, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: e1, start: e2, fixed: e3, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so })) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: e1.clone(), start: e2.clone(), fixed: e3.clone(), equationBound: Some(bindExp.clone()), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (_, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: e1, start: e2, fixed: e3, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so })) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: e1.clone(), start: e2.clone(), fixed: e3.clone(), equationBound: Some(bindExp.clone()), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (_, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: e1, min, max, start: e2, fixed: e3, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so })) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: e1.clone(), min: min.clone(), max: max.clone(), start: e2.clone(), fixed: e3.clone(), equationBound: Some(bindExp.clone()), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        _ => {
            println!("{}", (literal!("-failure in DAEUtil.addEquationBoundString\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oattr)
}

pub fn getClassList(mut v: Arc<DAE::Element>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    lst = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Element::VAR { source: Deref @ DAE::ElementSource { typeLst: lst, .. }, .. } => lst.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub fn getEmptyVarAttr(mut ty: Arc<DAE::Type>) -> Option<Arc<DAE::VariableAttributes>> {
    let mut attr: Option<Arc<DAE::VariableAttributes>> = None;
    attr = (::match_deref::match_deref! { match &(Types::getBasicType(ty.clone())) {
        Deref @ DAE::Type::T_REAL { .. } => Some(DAE::emptyVarAttrReal.clone()),
        Deref @ DAE::Type::T_INTEGER { .. } => Some(DAE::emptyVarAttrInt.clone()),
        Deref @ DAE::Type::T_BOOL { .. } => Some(DAE::emptyVarAttrBool.clone()),
        Deref @ DAE::Type::T_STRING { .. } => Some(DAE::emptyVarAttrString.clone()),
        Deref @ DAE::Type::T_ENUMERATION { .. } => Some(DAE::emptyVarAttrEnum.clone()),
        Deref @ DAE::Type::T_CLOCK { .. } => Some(DAE::emptyVarAttrClock.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attr
}

pub fn getBoundStartEquation(mut attr: Arc<DAE::VariableAttributes>) -> Result<Arc<DAE::Exp>> {
    let mut oe: Arc<DAE::Exp>;
    oe = (::match_deref::match_deref! { match &(attr.clone()) {
        Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_INT { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oe)
}

pub fn splitDAEIntoVarsAndEquations(mut inDae: DAE::DAElist) -> Result<(DAE::DAElist, DAE::DAElist)> {
    let mut allVars: DAE::DAElist;
    let mut allEqs: DAE::DAElist;
    let mut rest: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut vars: DoubleEnded::MutableList<Arc<DAE::Element>>;
    let mut eqs: DoubleEnded::MutableList<Arc<DAE::Element>>;
    let DAE::DAE { elementLst: __pa0 } = (inDae.clone()) else { bail!("pattern mismatch") };
    rest = __pa0.clone();
    vars = DoubleEnded::fromList(metamodelica::nil())?;
    eqs = DoubleEnded::fromList(metamodelica::nil())?;
    for mut elt in &*rest.clone() {
        let mut elt = elt.clone();
        let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            DoubleEnded::push_back(vars.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::COMP { ident: id, dAElist: elts1, source, comment: cmt } => {
            let mut elts11: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut elts3: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let (DAE::DAE { elementLst: __pa0 }, DAE::DAE { elementLst: __pa1 }) = (splitDAEIntoVarsAndEquations(DAE::DAElist { elementLst: elts1.clone() })?) else { bail!("pattern mismatch") };
            elts11 = __pa0.clone();
            elts3 = __pa1.clone();
            DoubleEnded::push_back(vars.clone(), Arc::new(DAE::Element::COMP { ident: (id.clone()).clone(), dAElist: elts11.clone(), source: source.clone(), comment: cmt.clone() }));
            DoubleEnded::push_list_back(eqs.clone(), elts3.clone());
            ()
        },
        Deref @ DAE::Element::EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::EQUEQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIALEQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIALDEFINE { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::DEFINE { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::WHEN_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::FOR_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::IF_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            DoubleEnded::push_back(vars.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::REINIT { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            DoubleEnded::push_back(eqs.clone(), elt.clone());
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.splitDAEIntoVarsAndEquations")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*DAEDump::dumpDAEElementsStr(DAE::DAElist { elementLst: list![elt.clone()] })?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    allVars = DAE::DAElist { elementLst: DoubleEnded::toListAndClear(vars.clone(), metamodelica::nil()) };
    allEqs = DAE::DAElist { elementLst: DoubleEnded::toListAndClear(eqs.clone(), metamodelica::nil()) };
    Ok((allVars, allEqs))
}

pub fn removeVariables(mut dae: DAE::DAElist, mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((dae.clone(), vars.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            dae.clone()
        },
        (DAE::DAElist { elementLst: elements }, _) => {
            let mut elements = (*elements).clone();
            elements = removeVariablesFromElements(elements.clone(), vars.clone());
            DAE::DAElist { elementLst: elements.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

fn removeVariablesFromElements(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut variableNames: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut outElements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    if variableNames.clone().is_empty() {
        outElements = inElements.clone();
        return outElements;
    }
    for mut el in &*inElements.clone() {
        let mut el = el.clone();
        let _ = (::match_deref::match_deref! { match &(el.clone()) {
        v @ Deref @ DAE::Element::VAR { componentRef: cr, .. } => {
            if List::select1(variableNames.clone(), Arc::new(ComponentReferenceBasics::crefEqual), cr.clone()).is_empty() {
                outElements = cons(v.clone(), outElements.clone());
            }
            ()
        },
        Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt } => {
            let mut elist = (*elist).clone();
            elist = removeVariablesFromElements(elist.clone(), variableNames.clone());
            outElements = cons(Arc::new(DAE::Element::COMP { ident: (id.clone()).clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() }), outElements.clone());
            ()
        },
        _ => {
            outElements = cons(el.clone(), outElements.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outElements = metamodelica::Dangerous::listReverseInPlace(outElements.clone());
    outElements
}

fn removeVariable(mut var: Arc<DAE::ComponentRef>, mut dae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = (var.clone(), dae.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Nil }) => {
                    Ok(DAE::DAElist { elementLst: metamodelica::nil() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, .. }, tail: elist } }) => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(var.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    Ok(DAE::DAElist { elementLst: elist.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt }, tail: elist2 } }) => {
                    let mut elist = (*elist).clone();
                    let mut elist2 = (*elist2).clone();
                    let DAE::DAE { elementLst: __pa0 } = (removeVariable(var.clone(), DAE::DAElist { elementLst: elist.clone() })?) else { bail!("pattern mismatch") };
                    elist = __pa0.clone();
                    let DAE::DAE { elementLst: __pa1 } = (removeVariable(var.clone(), DAE::DAElist { elementLst: elist2.clone() })?) else { bail!("pattern mismatch") };
                    elist2 = __pa1.clone();
                    Ok(DAE::DAElist { elementLst: cons(Arc::new(DAE::Element::COMP { ident: (id.clone()).clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() }), elist2.clone()) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: e, tail: elist } }) => {
                    let mut elist = (*elist).clone();
                    let DAE::DAE { elementLst: __pa0 } = (removeVariable(var.clone(), DAE::DAElist { elementLst: elist.clone() })?) else { bail!("pattern mismatch") };
                    elist = __pa0.clone();
                    Ok(DAE::DAElist { elementLst: cons(e.clone(), elist.clone()) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDae)
}

pub fn removeInnerAttrs(mut dae: DAE::DAElist, mut vars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> DAE::DAElist {
    let mut outDae: DAE::DAElist;
    outDae = List::fold(vars.clone(), Arc::new(removeInnerAttr), dae.clone());
    outDae
}

pub fn removeInnerAttr(mut var: Arc<DAE::ComponentRef>, mut dae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((var.clone(), dae.clone())) {
        (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Nil }) => {
            DAE::DAElist { elementLst: metamodelica::nil() }
        },
        (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { componentRef: oldVar, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: bind, dims: dim, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: Absyn::InnerOuter::INNER_OUTER, encrypted: ie }, tail: elist } }) if (compareUniquedVarWithNonUnique(var.clone(), oldVar.clone())?) => {
            let mut newVar: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut u: Arc<DAE::Element>;
            let mut o: Arc<DAE::Element>;
            let mut elist = (*elist).clone();
            newVar = nameInnerouterUniqueCref(oldVar.clone())?;
            o = Arc::new(DAE::Element::VAR { componentRef: oldVar.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: None, dims: dim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::OUTER, encrypted: ie.clone() });
            u = Arc::new(DAE::Element::VAR { componentRef: newVar.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: bind.clone(), dims: dim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: ie.clone() });
            elist = cons(u.clone(), cons(o.clone(), elist.clone()));
            DAE::DAElist { elementLst: elist.clone() }
        },
        (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: bind, dims: dim, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: ie }, tail: elist } }) if (ComponentReferenceBasics::crefEqualNoStringCompare(var.clone(), cr.clone())?) => {
            let mut io2: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
            io2 = removeInnerAttribute(io.clone());
            DAE::DAElist { elementLst: cons(Arc::new(DAE::Element::VAR { componentRef: cr.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: bind.clone(), dims: dim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io2.clone(), encrypted: ie.clone() }), elist.clone()) }
        },
        (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt }, tail: elist2 } }) => {
            let mut elist = (*elist).clone();
            let mut elist2 = (*elist2).clone();
            let DAE::DAE { elementLst: __pa0 } = (removeInnerAttr(var.clone(), DAE::DAElist { elementLst: elist.clone() })?) else { bail!("pattern mismatch") };
            elist = __pa0.clone();
            let DAE::DAE { elementLst: __pa1 } = (removeInnerAttr(var.clone(), DAE::DAElist { elementLst: elist2.clone() })?) else { bail!("pattern mismatch") };
            elist2 = __pa1.clone();
            DAE::DAElist { elementLst: cons(Arc::new(DAE::Element::COMP { ident: (id.clone()).clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() }), elist2.clone()) }
        },
        (_, DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: e, tail: elist } }) => {
            let mut elist = (*elist).clone();
            let DAE::DAE { elementLst: __pa0 } = (removeInnerAttr(var.clone(), DAE::DAElist { elementLst: elist.clone() })?) else { bail!("pattern mismatch") };
            elist = __pa0.clone();
            DAE::DAElist { elementLst: cons(e.clone(), elist.clone()) }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

fn compareUniquedVarWithNonUnique(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut equal: bool = false;
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    s1 = (ComponentReferenceBasics::printComponentRefStr(cr1.clone())?).clone();
    s2 = (ComponentReferenceBasics::printComponentRefStr(cr2.clone())?).clone();
    s1 = (System::stringReplace((s1.clone()).clone(), (arcstr::literal!(DAE::UNIQUEIO)).clone(), (literal!("")).clone())?).clone();
    s2 = (System::stringReplace((s2.clone()).clone(), (arcstr::literal!(DAE::UNIQUEIO)).clone(), (literal!("")).clone())?).clone();
    equal = stringEq((s1.clone()).clone(), (s2.clone()).clone());
    Ok(equal)
}

pub fn nameInnerouterUniqueCref(mut inCr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCr = (::match_deref::match_deref! { match &(inCr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: idt, subscriptLst: subs } => {
            let mut id = (*id).clone();
            id = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(DAE::UNIQUEIO)); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone();
            ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), idt.clone(), subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: idt, subscriptLst: subs, componentRef: child } => {
            let mut newChild: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            newChild = nameInnerouterUniqueCref(child.clone())?;
            ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), idt.clone(), subs.clone(), newChild.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCr)
}

pub fn unNameInnerouterUniqueCref(mut cr: Arc<DAE::ComponentRef>, mut removalString: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut ocr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    ocr = 'mc: {
        let __mc_input = (cr.clone(), removalString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: r#str, identType: ty, subscriptLst: subs }, _) => {
                    let mut str2: ArcStr = arcstr::literal!("");
                    str2 = (System::stringReplace((r#str.clone()).clone(), (removalString.clone()).clone(), (literal!("")).clone())?).clone();
                    Ok(ComponentReferenceBasics::makeCrefIdent((str2.clone()).clone(), ty.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: r#str, identType: ty, subscriptLst: subs, componentRef: child }, _) => {
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut child_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    child_2 = unNameInnerouterUniqueCref(child.clone(), (removalString.clone()).clone())?;
                    str2 = (System::stringReplace((r#str.clone()).clone(), (removalString.clone()).clone(), (literal!("")).clone())?).clone();
                    Ok(ComponentReferenceBasics::makeCrefQual((str2.clone()).clone(), ty.clone(), subs.clone(), child_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::WILD, _) => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (child, _) => {
                    println!("{}", (literal!(" failure unNameInnerouterUniqueCref: ")).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(child.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ocr)
}

fn removeInnerAttribute(mut io: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut ioOut: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    ioOut = (match io.clone() {
        Absyn::InnerOuter::INNER => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        Absyn::InnerOuter::INNER_OUTER => openmodelica_ast::Absyn::InnerOuter::OUTER,
        _ => io.clone(),
    });
    ioOut
}

pub fn varCref(mut elt: Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> {
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let __pa0 = ::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    Ok(cr)
}

pub fn getVariableAttributes(mut elt: Arc<DAE::Element>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut variableAttributesOption: Option<Arc<DAE::VariableAttributes>> = None;
    let __pa0 = ::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    variableAttributesOption = __pa0.clone();
    Ok(variableAttributesOption)
}

pub fn getUnitAttr(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Arc<DAE::Exp> {
    let mut start: Arc<DAE::Exp>;
    start = (::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(u), .. }) => {
            u.clone()
        },
        _ => {
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    start
}

pub fn getStartAttrEmpty(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>, mut optExp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut start: Arc<DAE::Exp>;
    start = (::match_deref::match_deref! { match &((inVariableAttributesOption.clone(), optExp.clone())) {
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(r), .. }), _) => {
            r.clone()
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(r), .. }), _) => {
            r.clone()
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(r), .. }), _) => {
            r.clone()
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(r), .. }), _) => {
            r.clone()
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { start: Some(r), .. }), _) => {
            r.clone()
        },
        _ => {
            optExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    start
}

pub fn getMinMax(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Arc<metamodelica::List<Option<Arc<DAE::Exp>>>> {
    let mut oExps: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>> = metamodelica::nil();
    oExps = (::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { max: e2, min: e1, .. }) => {
            list![e1.clone(), e2.clone()]
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { max: e2, min: e1, .. }) => {
            list![e1.clone(), e2.clone()]
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { max: e2, min: e1, .. }) => {
            list![e1.clone(), e2.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oExps
}

pub fn getMinMaxValues(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>) {
    let mut outMinValue: Option<Arc<DAE::Exp>> = None;
    let mut outMaxValue: Option<Arc<DAE::Exp>> = None;
    (outMinValue, outMaxValue) = (::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { max: maxValue, min: minValue, .. }) => {
            (minValue.clone(), maxValue.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { max: maxValue, min: minValue, .. }) => {
            (minValue.clone(), maxValue.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { max: maxValue, min: minValue, .. }) => {
            (minValue.clone(), maxValue.clone())
        },
        _ => {
            (None, None)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outMinValue, outMaxValue)
}

pub fn setMinMax(mut inAttr: Option<Arc<DAE::VariableAttributes>>, mut inMin: Option<Arc<DAE::Exp>>, mut inMax: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &((inAttr.clone(), inMin.clone(), inMax.clone())) {
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, _) => {
            if (referenceEq(&min.clone(),&inMin.clone()) && referenceEq(&max.clone(),&inMax.clone())) {inAttr.clone()} else {Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: inMin.clone(), max: inMax.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))}
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, _) => {
            if (referenceEq(&min.clone(),&inMin.clone()) && referenceEq(&max.clone(),&inMax.clone())) {inAttr.clone()} else {Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: inMin.clone(), max: inMax.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))}
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, _) => {
            if (referenceEq(&min.clone(),&inMin.clone()) && referenceEq(&max.clone(),&inMax.clone())) {inAttr.clone()} else {Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: inMin.clone(), max: inMax.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))}
        },
        (None, _, _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: inMin.clone(), max: inMax.clone(), start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getStartAttr(mut inAttributes: Option<Arc<DAE::VariableAttributes>>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut start: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    start = (::match_deref::match_deref! { match &(inAttributes.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(e), .. }) => e.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(e), .. }) => e.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(e), .. }) => e.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(e), .. }) => e.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { start: Some(e), .. }) => e.clone(),
        _ => (::match_deref::match_deref! { match &(Types::getBasicType(inType.clone())) {
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::ICONST { integer: 0 }),
        Deref @ DAE::Type::T_STRING { .. } => Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }),
        Deref @ DAE::Type::T_BOOL { .. } => Arc::new(DAE::Exp::BCONST { bool: false }),
        Deref @ DAE::Type::T_ENUMERATION { .. } => Types::getNthEnumLiteral(inType.clone(), 1)?,
        _ => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(start)
}

pub fn getStartOrigin(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut startOrigin: Option<Arc<DAE::Exp>> = None;
    startOrigin = (::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { startOrigin: so, .. }) => {
            so.clone()
        },
        None => {
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(startOrigin)
}

pub fn getStartAttrFail(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Arc<DAE::Exp>> {
    let mut start: Arc<DAE::Exp>;
    start = (::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { start: Some(r), .. }) => {
            r.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(start)
}

pub fn getNominalAttrFail(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Arc<DAE::Exp>> {
    let mut nominal: Arc<DAE::Exp>;
    nominal = (::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { nominal: Some(r), .. }) => {
            r.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(nominal)
}

pub fn getMinAttrFail(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Arc<DAE::Exp>> {
    let mut outMin: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: Some(__pa0), .. }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outMin = __pa0.clone();
    Ok(outMin)
}

pub fn getMaxAttrFail(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Arc<DAE::Exp>> {
    let mut outMax: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { max: Some(__pa0), .. }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outMax = __pa0.clone();
    Ok(outMax)
}

pub fn setVariableAttributes(mut var: Arc<DAE::Element>, mut varOpt: Option<Arc<DAE::VariableAttributes>>) -> Result<Arc<DAE::Element>> {
    let mut v: Arc<DAE::Element> = var.clone();
    v = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            assign_variant_field!(v => DAE::Element::VAR; variableAttributesOption = varOpt.clone());
            v.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(v)
}

pub fn setStateSelect(mut attr: Option<Arc<DAE::VariableAttributes>>, mut s: DAE::StateSelect) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; stateSelectOption = Some(s.clone()));
            Some(va.clone())
        },
        None => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: Some(s.clone()), uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setStartAttr(mut attr: Option<Arc<DAE::VariableAttributes>>, mut start: Arc<DAE::Exp>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = setStartAttrOption(attr.clone(), Some(start.clone()))?;
    Ok(outAttr)
}

pub fn setStartAttrOption(mut attr: Option<Arc<DAE::VariableAttributes>>, mut start: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut at: Option<Arc<DAE::VariableAttributes>> = None;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_REAL).clone() == start.clone() {
                at = attr.clone();
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; start = start.clone());
                at = Some(va.clone());
            }
            at.clone()
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }) => {
            let mut at: Option<Arc<DAE::VariableAttributes>> = None;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_INT).clone() == start.clone() {
                at = attr.clone();
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_INT; start = start.clone());
                at = Some(va.clone());
            }
            at.clone()
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }) => {
            let mut at: Option<Arc<DAE::VariableAttributes>> = None;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_BOOL).clone() == start.clone() {
                at = attr.clone();
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_BOOL; start = start.clone());
                at = Some(va.clone());
            }
            at.clone()
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }) => {
            let mut at: Option<Arc<DAE::VariableAttributes>> = None;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_STRING).clone() == start.clone() {
                at = attr.clone();
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_STRING; start = start.clone());
                at = Some(va.clone());
            }
            at.clone()
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }) => {
            let mut at: Option<Arc<DAE::VariableAttributes>> = None;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone() == start.clone() {
                at = attr.clone();
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_ENUMERATION; start = start.clone());
                at = Some(va.clone());
            }
            at.clone()
        },
        None => {
            if (isNone(start.clone())) {None} else {Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: start.clone(), fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setStartOrigin(mut attr: Option<Arc<DAE::VariableAttributes>>, mut startOrigin: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; startOrigin = startOrigin.clone());
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_INT; startOrigin = startOrigin.clone());
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_BOOL; startOrigin = startOrigin.clone());
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_STRING; startOrigin = startOrigin.clone());
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_ENUMERATION; startOrigin = startOrigin.clone());
            Some(va.clone())
        },
        None => {
            if (isNone(startOrigin.clone())) {None} else {Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin.clone() }))}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getNominalAttr(mut attr: Option<Arc<DAE::VariableAttributes>>) -> Arc<DAE::Exp> {
    let mut nominal: Arc<DAE::Exp>;
    nominal = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { nominal: Some(n), .. }) => {
            n.clone()
        },
        _ => {
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    nominal
}

pub fn setNominalAttr(mut attr: Option<Arc<DAE::VariableAttributes>>, mut nominal: Arc<DAE::Exp>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; nominal = Some(nominal.clone()));
            Some(va.clone())
        },
        None => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: Some(nominal.clone()), stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setUnitAttr(mut attr: Option<Arc<DAE::VariableAttributes>>, mut unit: Arc<DAE::Exp>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &((attr.clone(), unit.clone())) {
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: _, displayUnit: du, min, max, start: s, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: Some(unit.clone()), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: s.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (None, _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: Some(unit.clone()), displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setElementVarVisibility(mut elt: Arc<DAE::Element>, mut visibility: DAE::VarVisibility) -> Arc<DAE::Element> {
    let mut e: Arc<DAE::Element> = elt.clone();
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            assign_variant_field!(e => DAE::Element::VAR; protection = visibility.clone());
            e.clone()
        },
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    e
}

pub fn setElementVarDirection(mut elt: Arc<DAE::Element>, mut direction: DAE::VarDirection) -> Arc<DAE::Element> {
    let mut e: Arc<DAE::Element> = elt.clone();
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            assign_variant_field!(e => DAE::Element::VAR; direction = direction.clone());
            e.clone()
        },
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    e
}

pub fn setElementVarBinding(mut elt: Arc<DAE::Element>, mut binding: Option<Arc<DAE::Exp>>) -> Arc<DAE::Element> {
    let mut e: Arc<DAE::Element> = elt.clone();
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            assign_variant_field!(e => DAE::Element::VAR; binding = binding.clone());
            e.clone()
        },
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    e
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn setProtectedAttr(mut attr: Option<Arc<DAE::VariableAttributes>>, mut isProtected: bool) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: Some(isProtected.clone()), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: Some(isProtected.clone()), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: Some(isProtected.clone()), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: Some(isProtected.clone()), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: Some(isProtected.clone()), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: r#fn, finalPrefix: _ }) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: r#fn.clone(), finalPrefix: Some(isProtected.clone()) }))
        },
        None => {
            setProtectedAttr(Some(DAE::emptyVarAttrReal.clone()), isProtected.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getProtectedAttr(mut attr: Option<Arc<DAE::VariableAttributes>>) -> bool {
    let mut isProtected: bool = false;
    isProtected = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { isProtected: Some(isProtected), .. }) => isProtected.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { isProtected: Some(isProtected), .. }) => isProtected.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { isProtected: Some(isProtected), .. }) => isProtected.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { isProtected: Some(isProtected), .. }) => isProtected.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { isProtected: Some(isProtected), .. }) => isProtected.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: Some(isProtected), .. }) => isProtected.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isProtected
}

pub fn setFixedAttr(mut attr: Option<Arc<DAE::VariableAttributes>>, mut fixed: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &((attr.clone(), fixed.clone())) {
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: ini, fixed: _, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: ini.clone(), fixed: fixed.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: ini, fixed: _, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: ini.clone(), fixed: fixed.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: ini, fixed: _, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: ini.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: ini, fixed: _, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: ini.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: _, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getFixedAttr(mut attr: Option<Arc<DAE::VariableAttributes>>) -> Option<Arc<DAE::Exp>> {
    let mut isFixed: Option<Arc<DAE::Exp>> = None;
    isFixed = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: isFixed, .. }) => isFixed.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: isFixed, .. }) => isFixed.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: isFixed, .. }) => isFixed.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { fixed: isFixed, .. }) => isFixed.clone(),
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: isFixed, .. }) => isFixed.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isFixed
}

pub fn setFinalAttr(mut attr: Option<Arc<DAE::VariableAttributes>>, mut finalPrefix: bool) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outAttr: Option<Arc<DAE::VariableAttributes>> = None;
    outAttr = (::match_deref::match_deref! { match &((attr.clone(), finalPrefix.clone())) {
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix.clone()), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix.clone()), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix.clone()), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: ip, finalPrefix: _ }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: ip.clone(), finalPrefix: Some(finalPrefix.clone()) }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix.clone()), startOrigin: so.clone() }))
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }), _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix.clone()), startOrigin: so.clone() }))
        },
        (None, _) => {
            Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: Some(finalPrefix.clone()), startOrigin: None }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getFinalAttr(mut attr: Option<Arc<DAE::VariableAttributes>>) -> bool {
    let mut finalPrefix: bool = false;
    finalPrefix = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    finalPrefix
}

pub fn boolVarVisibility(mut vp: DAE::VarVisibility) -> Result<bool> {
    let mut prot: bool = false;
    prot = (match vp.clone() {
        DAE::VarVisibility::PUBLIC => false,
        DAE::VarVisibility::PROTECTED => true,
        _ => {
            println!("{}", (literal!("- DAEUtil.boolVarVisibility failed\n")).clone());
            bail!("fail")
        },
    });
    Ok(prot)
}

pub fn hasStartAttr(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> bool {
    let mut hasStart: bool = false;
    hasStart = (::match_deref::match_deref! { match &(inVariableAttributesOption.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(_), .. }) => {
            true
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(_), .. }) => {
            true
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(_), .. }) => {
            true
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(_), .. }) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasStart
}

pub fn getStartAttrString(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inVariableAttributesOption.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(r), .. }) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ExpressionBasics::printExpStr(r.clone())?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(r), .. }) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ExpressionBasics::printExpStr(r.clone())?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn getMatchingElements(mut elist: Arc<metamodelica::List<Arc<DAE::Element>>>, mut cond: Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    pub type FuncTypeElementTo = fn(Arc<DAE::Element>) -> Result<bool>;

    let mut oelist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    oelist = List::filterOnTrue(elist.clone(), cond.clone());
    oelist
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getAllMatchingElements(mut elist: Arc<metamodelica::List<Arc<DAE::Element>>>, mut cond: Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<()> + 'static>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    pub type FuncTypeElementTo = fn(Arc<DAE::Element>) -> Result<()>;

    let mut outElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outElist = 'mc: {
        let __mc_input = (elist.clone(), cond.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { dAElist: elist1, .. }, tail: elist2 }, _) => {
                    let mut elist1 = (*elist1).clone();
                    let mut elist2 = (*elist2).clone();
                    elist1 = getAllMatchingElements(elist1.clone(), cond.clone())?;
                    elist2 = getAllMatchingElements(elist2.clone(), cond.clone())?;
                    Ok(listAppend(elist1.clone(), elist2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: elist2 }, _) => {
                    let mut elist2 = (*elist2).clone();
                    cond(e.clone())?;
                    elist2 = getAllMatchingElements(elist2.clone(), cond.clone())?;
                    Ok(cons(e.clone(), elist2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: elist2 }, _) => {
                    Ok(getAllMatchingElements(elist2.clone(), cond.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElist)
}

pub fn findAllMatchingElements(mut dae: DAE::DAElist, mut cond1: Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>, mut cond2: Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>) -> Result<(DAE::DAElist, DAE::DAElist)> {
    pub type CondFunc = fn(Arc<DAE::Element>) -> Result<bool>;

    let mut firstList: DAE::DAElist;
    let mut secondList: DAE::DAElist;
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut el1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut el2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (dae.clone()) else { bail!("pattern mismatch") };
    elements = __pa0.clone();
    (el1, el2) = findAllMatchingElements2(elements.clone(), cond1.clone(), cond2.clone(), metamodelica::nil(), metamodelica::nil());
    firstList = DAE::DAElist { elementLst: metamodelica::Dangerous::listReverseInPlace(el1.clone()) };
    secondList = DAE::DAElist { elementLst: metamodelica::Dangerous::listReverseInPlace(el2.clone()) };
    Ok((firstList, secondList))
}

fn findAllMatchingElements2(mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut cond1: Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>, mut cond2: Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>, mut accumFirst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut accumSecond: Arc<metamodelica::List<Arc<DAE::Element>>>) -> (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>) {
    pub type CondFunc = fn(Arc<DAE::Element>) -> Result<bool>;

    let mut firstList: Arc<metamodelica::List<Arc<DAE::Element>>> = accumFirst.clone();
    let mut secondList: Arc<metamodelica::List<Arc<DAE::Element>>> = accumSecond.clone();
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        let _ = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::COMP { .. } => {
            (firstList, secondList) = findAllMatchingElements2(var_field!((*e).dAElist, DAE::Element::COMP).clone(), cond1.clone(), cond2.clone(), firstList.clone(), secondList.clone());
            ()
        },
        _ => {
            if cond1(e.clone()).unwrap() {
                firstList = cons(e.clone(), firstList.clone());
            }
            if cond2(e.clone()).unwrap() {
                secondList = cons(e.clone(), secondList.clone());
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    (firstList, secondList)
}

pub fn isAfterIndexInlineFunc(mut inElem: DAE::Function) -> bool {
    let mut b: bool = false;
    b = (match inElem.clone() {
        DAE::Function::FUNCTION { inlineType: DAE::InlineType::AFTER_INDEX_RED_INLINE, .. } => true,
        _ => false,
    });
    b
}

pub fn isParameter(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::PARAM, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isParameterOrConstant(mut inElement: Arc<DAE::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::CONST, .. } => true,
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::PARAM, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isParamOrConstVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut outIsParamOrConst: bool = false;
    let mut var: SCode::Variability = SCode::Variability::CONST;
    let __pa0 = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { variability: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    outIsParamOrConst = SCodeUtil::isParameterOrConst(var.clone());
    Ok(outIsParamOrConst)
}

pub fn isConstVar(mut var: Arc<DAE::Var>) -> bool {
    let mut isConstVar: bool = false;
    isConstVar = (match var.attributes.variability.clone() {
        SCode::Variability::CONST => true,
        _ => false,
    });
    isConstVar
}

pub fn isNotParamOrConstVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut outIsNotParamOrConst: bool = false;
    outIsNotParamOrConst = !(isParamOrConstVar(inVar.clone())?);
    Ok(outIsNotParamOrConst)
}

pub fn isParamConstOrComplexVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut outIsParamConstComplex: bool = false;
    outIsParamConstComplex = isParamOrConstVar(inVar.clone())? || isComplexVar(inVar.clone())?;
    Ok(outIsParamConstComplex)
}

pub fn isParamOrConstVarKind(mut inVarKind: DAE::VarKind) -> bool {
    let mut outIsParamOrConst: bool = false;
    outIsParamOrConst = (match inVarKind.clone() {
        DAE::VarKind::PARAM => true,
        DAE::VarKind::CONST => true,
        _ => false,
    });
    outIsParamOrConst
}

pub fn isInnerVar(mut element: Arc<DAE::Element>) -> bool {
    let mut isInner: bool = false;
    isInner = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::VAR { .. } => AbsynUtil::isInner(var_field!((*element).innerOuter, DAE::Element::VAR).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isInner
}

pub fn isOuterVar(mut element: Arc<DAE::Element>) -> bool {
    let mut isOuter: bool = false;
    isOuter = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::VAR { innerOuter: Absyn::InnerOuter::OUTER, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOuter
}

pub fn isComp(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::COMP { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn getOutputVars(mut vl: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    vl_1 = getMatchingElements(vl.clone(), Arc::new(fnptr!(isOutputVar, Arc<DAE::Element>)));
    vl_1
}

pub fn getOutputElements(mut vl: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    vl_1 = getMatchingElements(vl.clone(), Arc::new(fnptr!(isOutputElement, Arc<DAE::Element>)));
    vl_1
}

pub fn getProtectedVars(mut vl: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    vl_1 = getMatchingElements(vl.clone(), Arc::new(fnptr!(isProtectedVar, Arc<DAE::Element>)));
    vl_1
}

pub fn getBidirVars(mut vl: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    vl_1 = getMatchingElements(vl.clone(), Arc::new(fnptr!(isBidirVar, Arc<DAE::Element>)));
    vl_1
}

pub fn getBidirElements(mut vl: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    vl_1 = getMatchingElements(vl.clone(), Arc::new(fnptr!(isBidirElement, Arc<DAE::Element>)));
    vl_1
}

pub fn getInputVars(mut vl: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    vl_1 = getMatchingElements(vl.clone(), Arc::new(fnptr!(isInput, Arc<DAE::Element>)));
    vl_1
}

pub fn isFlowVar(mut inElement: Arc<DAE::Element>) -> Result<()> {
    ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { connectorType: Deref @ DAE::ConnectorType::FLOW, kind: DAE::VarKind::VARIABLE, .. } => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn isStreamVar(mut inElement: Arc<DAE::Element>) -> Result<()> {
    ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, kind: DAE::VarKind::VARIABLE, .. } => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn isFlow(mut inFlow: Arc<DAE::ConnectorType>) -> bool {
    let mut outIsFlow: bool = false;
    outIsFlow = (::match_deref::match_deref! { match &(inFlow.clone()) {
        Deref @ DAE::ConnectorType::FLOW => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsFlow
}

pub fn isStream(mut inStream: Arc<DAE::ConnectorType>) -> bool {
    let mut outIsStream: bool = false;
    outIsStream = (::match_deref::match_deref! { match &(inStream.clone()) {
        Deref @ DAE::ConnectorType::STREAM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsStream
}

pub fn isOutputVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT, kind: DAE::VarKind::VARIABLE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isOutputElement(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn assertProtectedVar(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { protection: DAE::VarVisibility::PROTECTED, .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn isProtectedVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { protection: DAE::VarVisibility::PROTECTED, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isPublicVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { protection: DAE::VarVisibility::PUBLIC, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isBidirVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::BIDIR, kind: DAE::VarKind::VARIABLE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isBidirElement(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::BIDIR, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isInputVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, kind: DAE::VarKind::VARIABLE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isInput(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isNotVar(mut e: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::VAR { componentRef: _, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { componentRef: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isFunctionRefVar(mut inElem: Arc<DAE::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inElem.clone()) {
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_FUNCTION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isComment(mut elt: Arc<DAE::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::COMMENT { cmt: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isAlgorithm(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isStmtAssert(mut stmt: Arc<DAE::Statement>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSERT { cond: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStmtReturn(mut stmt: Arc<DAE::Statement>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_RETURN { source: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStmtReinit(mut stmt: Arc<DAE::Statement>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_REINIT { var: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStmtTerminate(mut stmt: Arc<DAE::Statement>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_TERMINATE { msg: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isComplexEquation(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn isFunctionInlineFalse(mut inElement: DAE::Function) -> bool {
    let mut res: bool = false;
    res = (match inElement.clone() {
        DAE::Function::FUNCTION { inlineType: DAE::InlineType::NO_INLINE, .. } => true,
        _ => false,
    });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn findElement(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inFuncTypeElementTo: Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<()> + 'static>) -> Result<Option<Arc<DAE::Element>>> {
    pub type FuncTypeElementTo = fn(Arc<DAE::Element>) -> Result<()>;

    let mut outElementOption: Option<Arc<DAE::Element>> = None;
    outElementOption = (::match_deref::match_deref! { match &((inElementLst.clone(), inFuncTypeElementTo.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            None
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, f) => {
            let mut e_1: Option<Arc<DAE::Element>> = None;
            e_1 = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            f(e.clone())?;
            Ok(Some(e.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut e_1: Option<Arc<DAE::Element>>;
            if '__try0: {
                unwrap_break_err!(f(e.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            e_1 = findElement(rest.clone(), f.clone())?;
            Ok(e_1.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
            e_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElementOption)
}

pub fn getVariableBindingsStr(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut varlst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(elts.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { dAElist: els, .. }, tail: Deref @ metamodelica::List::Nil } => getVariableBindingsStr(els.clone())?,
        _ => {
            varlst = getVariableList(elts.clone());
            r#str = (getBindingsStr(varlst.clone())?).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

fn getVariableList(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outElementLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut e in (inElementLst.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. } => false,
        Deref @ DAE::Element::VAR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outElementLst
}

pub fn getVariableType(mut inElement: Arc<DAE::Element>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { ty: tp, .. } => {
            tp.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn getBindingsStr(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inElementLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: Some(e), .. }, tail: lst @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
            let mut expstr: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            expstr = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s3 = (stringAppend((expstr.clone()).clone(), (literal!(",")).clone())).clone();
            s4 = (getBindingsStr(lst.clone())?).clone();
            r#str = (stringAppend((s3.clone()).clone(), (s4.clone()).clone())).clone();
            r#str.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: None, .. }, tail: lst @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (literal!("-,")).clone();
            s2 = (getBindingsStr(lst.clone())?).clone();
            r#str = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            r#str.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: Some(e), .. }, tail: Deref @ metamodelica::List::Nil } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: None, .. }, tail: Deref @ metamodelica::List::Nil } => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn getBindings(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut outc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut oute: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (outc, oute) = 'mc: {
        let __mc_input = inElementLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: Some(e), componentRef: cr, .. }, tail: rest } => {
                    let mut outc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outc.clone();
                    let mut oute: Arc<metamodelica::List<Arc<DAE::Exp>>> = oute.clone();
                    (outc, oute) = getBindings(rest.clone())?;
                    Ok((cons(cr.clone(), outc.clone()), cons(e.clone(), oute.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: None, .. }, tail: rest } => {
                    let mut outc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = outc.clone();
                    let mut oute: Arc<metamodelica::List<Arc<DAE::Exp>>> = oute.clone();
                    (outc, oute) = getBindings(rest.clone())?;
                    Ok((outc.clone(), oute.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!(" error in getBindings \n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outc, oute))
}

pub fn toConnectorType(mut inConnectorType: SCode::ConnectorType, mut inState: ClassInf::State) -> Arc<DAE::ConnectorType> {
    let mut outConnectorType: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    outConnectorType = (match (inConnectorType.clone(), inState.clone()) {
        (SCode::ConnectorType::FLOW, _) => Arc::new(openmodelica_frontend_types::DAE::ConnectorType::FLOW),
        (SCode::ConnectorType::STREAM, _) => Arc::new(DAE::ConnectorType::STREAM { associatedFlow: None }),
        (_, ClassInf::State::CONNECTOR { .. }) => Arc::new(openmodelica_frontend_types::DAE::ConnectorType::POTENTIAL),
        _ => Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR),
    });
    outConnectorType
}

pub fn toConnectorTypeNoState(mut scodeConnectorType: SCode::ConnectorType, mut flowName: Option<Arc<DAE::ComponentRef>>) -> Arc<DAE::ConnectorType> {
    let mut daeConnectorType: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    daeConnectorType = (match scodeConnectorType.clone() {
        SCode::ConnectorType::FLOW => Arc::new(openmodelica_frontend_types::DAE::ConnectorType::FLOW),
        SCode::ConnectorType::STREAM => Arc::new(DAE::ConnectorType::STREAM { associatedFlow: flowName.clone() }),
        _ => Arc::new(openmodelica_frontend_types::DAE::ConnectorType::POTENTIAL),
    });
    daeConnectorType
}

pub fn toDaeParallelism(mut inCref: Arc<DAE::ComponentRef>, mut inParallelism: SCode::Parallelism, mut inState: ClassInf::State, mut inInfo: SourceInfo) -> Result<DAE::VarParallelism> {
    let mut outParallelism: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    outParallelism = 'mc: {
        let __mc_input = (inCref.clone(), inParallelism.clone(), inState.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Parallelism::NON_PARALLEL, _, _) => {
                    Ok(openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Parallelism::PARGLOBAL, ClassInf::State::FUNCTION { path: _, isImpure: _ }, _) => {
                    Ok(openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Parallelism::PARLOCAL, ClassInf::State::FUNCTION { path: _, isImpure: _ }, _) => {
                    Ok(openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Parallelism::PARGLOBAL, _, _) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut path: Arc<Absyn::Path>;
                    path = ClassInfUtil::getStateName(inState.clone());
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- DAEUtil.toDaeParallelism: parglobal component '")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("' in non-function class: ")); __mm_s.push_str(&*ClassInfUtil::printStateStr(inState.clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_WARNING.clone(), list![(str1.clone()).clone()], inInfo.clone())?;
                    Ok(openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Parallelism::PARLOCAL, _, _) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut path: Arc<Absyn::Path>;
                    path = ClassInfUtil::getStateName(inState.clone());
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- DAEUtil.toDaeParallelism: parlocal component '")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("' in non-function class: ")); __mm_s.push_str(&*ClassInfUtil::printStateStr(inState.clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_WARNING.clone(), list![(str1.clone()).clone()], inInfo.clone())?;
                    Ok(openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outParallelism)
}

pub fn scodePrlToDaePrl(mut inParallelism: SCode::Parallelism) -> Result<DAE::VarParallelism> {
    let mut outVarParallelism: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    outVarParallelism = (match inParallelism.clone() {
        SCode::Parallelism::NON_PARALLEL => openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        SCode::Parallelism::PARGLOBAL => openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL,
        SCode::Parallelism::PARLOCAL => openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL,
        _ => bail!("match: no arm matched"),
    });
    Ok(outVarParallelism)
}

pub fn daeParallelismEqual(mut inParallelism1: DAE::VarParallelism, mut inParallelism2: DAE::VarParallelism) -> bool {
    let mut equal: bool = false;
    equal = (match (inParallelism1.clone(), inParallelism2.clone()) {
        (DAE::VarParallelism::NON_PARALLEL, DAE::VarParallelism::NON_PARALLEL) => true,
        (DAE::VarParallelism::PARGLOBAL, DAE::VarParallelism::PARGLOBAL) => true,
        (DAE::VarParallelism::PARLOCAL, DAE::VarParallelism::PARLOCAL) => true,
        _ => false,
    });
    equal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getFlowVariables(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outExpComponentRefLst = 'mc: {
        let __mc_input = inElementLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { connectorType: Deref @ DAE::ConnectorType::FLOW, componentRef: cr, .. }, tail: xs } => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res = getFlowVariables(xs.clone())?;
                    Ok(cons(cr.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { dAElist: lst, ident: id, .. }, tail: xs } => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res1_1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res1 = getFlowVariables(lst.clone())?;
                    res1_1 = getFlowVariables2(res1.clone(), (id.clone()).clone())?;
                    res2 = getFlowVariables(xs.clone())?;
                    res = listAppend(res1_1.clone(), res2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res = getFlowVariables(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpComponentRefLst)
}

fn getFlowVariables2(mut inExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inIdent: ArcStr) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outExpComponentRefLst = (::match_deref::match_deref! { match &((inExpComponentRefLst.clone(), inIdent.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: xs }, id) => {
            let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            res = getFlowVariables2(xs.clone(), (id.clone()).clone())?;
            cr_1 = ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil(), cr.clone());
            cons(cr_1.clone(), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpComponentRefLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getStreamVariables(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outExpComponentRefLst = 'mc: {
        let __mc_input = inElementLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, componentRef: cr, .. }, tail: xs } => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res = getStreamVariables(xs.clone())?;
                    Ok(cons(cr.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { dAElist: lst, ident: id, .. }, tail: xs } => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res1_1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res1 = getStreamVariables(lst.clone())?;
                    res1_1 = getStreamVariables2(res1.clone(), (id.clone()).clone())?;
                    res2 = getStreamVariables(xs.clone())?;
                    res = listAppend(res1_1.clone(), res2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res = getStreamVariables(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpComponentRefLst)
}

fn getStreamVariables2(mut inExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inIdent: ArcStr) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outExpComponentRefLst = (::match_deref::match_deref! { match &((inExpComponentRefLst.clone(), inIdent.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: xs }, id) => {
            let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            res = getStreamVariables2(xs.clone(), (id.clone()).clone())?;
            cr_1 = ComponentReferenceBasics::makeCrefQual((id.clone()).clone(), DAE::T_UNKNOWN_DEFAULT.clone(), metamodelica::nil(), cr.clone());
            cons(cr_1.clone(), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpComponentRefLst)
}

pub fn daeToRecordValue(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inBoolean: bool) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone(), inElementLst.clone(), inBoolean.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, cname, Deref @ metamodelica::List::Nil, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::RECORD { record_: cname.clone(), orderd: metamodelica::nil(), comp: metamodelica::nil(), index: -1 })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cname, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { source, binding: Some(rhs), componentRef: cr, .. }, tail: rest }, r#impl) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cr_str: ArcStr = arcstr::literal!("");
                    let mut ix: i32 = 0;
                    let mut info: SourceInfo;
                    let mut cache = (*cache).clone();
                    let mut cname = (*cname).clone();
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    (cache, value) = Ceval::ceval(cache.clone(), env.clone(), rhs.clone(), r#impl.clone(), Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(daeToRecordValue(cache.clone(), env.clone(), cname.clone(), rest.clone(), r#impl.clone())?) {
                        (__pa0, Deref @ Values::Value::RECORD { record_: __pa1, orderd: __pa2, comp: __pa3, index: __pa4 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    cname = __pa1.clone();
                    vals = __pa2.clone();
                    names = __pa3.clone();
                    ix = __pa4.clone();
                    cr_str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    Ok((cache.clone(), Arc::new(Values::Value::RECORD { record_: cname.clone(), orderd: cons(value.clone(), vals.clone()), comp: cons(cr_str.clone(), names.clone()), index: ix.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Cons { head: el, tail: _ }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = (DAEDump::dumpDebugDAE(DAE::DAElist { elementLst: list![el.clone()] })?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- DAEUtil.daeToRecordValue failed on: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn toModelicaForm(mut inDAElist: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDAElist: DAE::DAElist;
    outDAElist = (match inDAElist.clone() {
        DAE::DAElist { elementLst: ref elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            elts_1 = toModelicaFormElts(elts.clone())?;
            DAE::DAElist { elementLst: elts_1.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outDAElist)
}

fn toModelicaFormElts(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outElementLst = (::match_deref::match_deref! { match &(inElementLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { encrypted, innerOuter: io, comment, variableAttributesOption: dae_var_attr, source, connectorType: ct, dims: instDim, binding: d, ty: t, protection: prot, parallelism: prl, direction: b, kind: a, componentRef: cr }, tail: elts } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str_1: ArcStr = arcstr::literal!("");
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut d_1: Option<Arc<DAE::Exp>> = None;
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            r#str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            str_1 = (Util::stringReplaceChar((r#str.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            elts_1 = toModelicaFormElts(elts.clone())?;
            d_1 = toModelicaFormExpOpt(d.clone())?;
            ty = ComponentReference::crefLastType(cr.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent((str_1.clone()).clone(), ty.clone(), metamodelica::nil());
            cons(Arc::new(DAE::Element::VAR { componentRef: cref_.clone(), kind: a.clone(), direction: b.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: t.clone(), binding: d_1.clone(), dims: instDim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: encrypted.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { source, exp: e, componentRef: cr }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e_1: Arc<DAE::Exp>;
            e_1 = toModelicaFormExp(e.clone())?;
            cr_1 = toModelicaFormCref(cr.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::DEFINE { componentRef: cr_1.clone(), exp: e_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIALDEFINE { source, exp: e, componentRef: cr }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut e_1: Arc<DAE::Exp>;
            e_1 = toModelicaFormExp(e.clone())?;
            cr_1 = toModelicaFormCref(cr.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::INITIALDEFINE { componentRef: cr_1.clone(), exp: e_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { source, scalar: e2, exp: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone())?;
            e2_1 = toModelicaFormExp(e2.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::EQUATION { exp: e1_1.clone(), scalar: e2_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { source, rhs: e2, lhs: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone())?;
            e2_1 = toModelicaFormExp(e2.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::COMPLEX_EQUATION { lhs: e1_1.clone(), rhs: e2_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { source, rhs: e2, lhs: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone())?;
            e2_1 = toModelicaFormExp(e2.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1_1.clone(), rhs: e2_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { source, cr2, cr1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut cr2 = (*cr2).clone();
            let mut cr1 = (*cr1).clone();
            let __pa0 = ::match_deref::match_deref! { match &(toModelicaFormExp(Expression::crefExp(cr1.clone())?)?) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr1 = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(toModelicaFormExp(Expression::crefExp(cr2.clone())?)?) {
                Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ } => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr2 = __pa1.clone();
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::EQUEQUATION { cr1: cr1.clone(), cr2: cr2.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { source, elsewhen_: Some(elt), equations: welts, condition: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut welts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut elt_1: Arc<DAE::Element>;
            let mut e1_1: Arc<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone())?;
            welts_1 = toModelicaFormElts(welts.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(toModelicaFormElts(list![elt.clone()])?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elt_1 = __pa0.clone();
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::WHEN_EQUATION { condition: e1_1.clone(), equations: welts_1.clone(), elsewhen_: Some(elt_1.clone()), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { source, elsewhen_: None, equations: welts, condition: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut welts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e1_1: Arc<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone())?;
            welts_1 = toModelicaFormElts(welts.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::WHEN_EQUATION { condition: e1_1.clone(), equations: welts_1.clone(), elsewhen_: None, source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { source, equations3: eelts, equations2: trueBranches, condition1: conds }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eelts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut conds_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut trueBranches_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            conds_1 = List::map(conds.clone(), Arc::new(toModelicaFormExp));
            trueBranches_1 = List::map(trueBranches.clone(), Arc::new(toModelicaFormElts));
            eelts_1 = toModelicaFormElts(eelts.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::IF_EQUATION { condition1: conds_1.clone(), equations2: trueBranches_1.clone(), equations3: eelts_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_IF_EQUATION { source, equations3: eelts, equations2: trueBranches, condition1: conds }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut eelts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut conds_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut trueBranches_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            conds_1 = List::map(conds.clone(), Arc::new(toModelicaFormExp));
            trueBranches_1 = List::map(trueBranches.clone(), Arc::new(toModelicaFormElts));
            eelts_1 = toModelicaFormElts(eelts.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::INITIAL_IF_EQUATION { condition1: conds_1.clone(), equations2: trueBranches_1.clone(), equations3: eelts_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIALEQUATION { source, exp2: e2, exp1: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone())?;
            e2_1 = toModelicaFormExp(e2.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::INITIALEQUATION { exp1: e1_1.clone(), exp2: e2_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { source, algorithm_: alg }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            println!("{}", (literal!("to_modelica_form_elts(ALGORITHM) not impl. yet\n")).clone());
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIALALGORITHM { source, algorithm_: alg }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            println!("{}", (literal!("to_modelica_form_elts(INITIALALGORITHM) not impl. yet\n")).clone());
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: alg.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { comment, source, dAElist: elts2, ident: id }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut elts2 = (*elts2).clone();
            elts2 = toModelicaFormElts(elts2.clone())?;
            elts_1 = toModelicaFormElts(elts.clone())?;
            cons(Arc::new(DAE::Element::COMP { ident: (id.clone()).clone(), dAElist: elts2.clone(), source: source.clone(), comment: comment.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { source, level: e3, message: e2, condition: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            let mut e_3: Arc<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts.clone())?;
            e_1 = toModelicaFormExp(e1.clone())?;
            e_2 = toModelicaFormExp(e2.clone())?;
            e_3 = toModelicaFormExp(e3.clone())?;
            cons(Arc::new(DAE::Element::ASSERT { condition: e_1.clone(), message: e_2.clone(), level: e_3.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_ASSERT { source, level: e3, message: e2, condition: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            let mut e_3: Arc<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts.clone())?;
            e_1 = toModelicaFormExp(e1.clone())?;
            e_2 = toModelicaFormExp(e2.clone())?;
            e_3 = toModelicaFormExp(e3.clone())?;
            cons(Arc::new(DAE::Element::INITIAL_ASSERT { condition: e_1.clone(), message: e_2.clone(), level: e_3.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { source, message: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e_1: Arc<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts.clone())?;
            e_1 = toModelicaFormExp(e1.clone())?;
            cons(Arc::new(DAE::Element::TERMINATE { message: e_1.clone(), source: source.clone() }), elts_1.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_TERMINATE { source, message: e1 }, tail: elts } => {
            let mut elts_1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e_1: Arc<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts.clone())?;
            e_1 = toModelicaFormExp(e1.clone())?;
            cons(Arc::new(DAE::Element::INITIAL_TERMINATE { message: e_1.clone(), source: source.clone() }), elts_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElementLst)
}

pub fn replaceCrefInVar(mut newCr: Arc<DAE::ComponentRef>, mut inelem: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outelem: Arc<DAE::Element>;
    outelem = (::match_deref::match_deref! { match &((newCr.clone(), inelem.clone())) {
        (_, Deref @ DAE::Element::VAR { componentRef: _, kind: a2, direction: a3, parallelism: prl, protection: a4, ty: a5, binding: a6, dims: a7, connectorType: ct, source, variableAttributesOption: a11, comment: a12, innerOuter: a13, encrypted: e }) => {
            Arc::new(DAE::Element::VAR { componentRef: newCr.clone(), kind: a2.clone(), direction: a3.clone(), parallelism: prl.clone(), protection: a4.clone(), ty: a5.clone(), binding: a6.clone(), dims: a7.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: a11.clone(), comment: a12.clone(), innerOuter: a13.clone(), encrypted: e.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outelem)
}

pub fn replaceTypeInVar(mut newType: Arc<DAE::Type>, mut inelem: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outelem: Arc<DAE::Element>;
    outelem = (::match_deref::match_deref! { match &((newType.clone(), inelem.clone())) {
        (_, Deref @ DAE::Element::VAR { componentRef: a1, kind: a2, direction: a3, parallelism: prl, protection: a4, ty: _, binding: a6, dims: a7, connectorType: ct, source, variableAttributesOption: a11, comment: a12, innerOuter: a13, encrypted: e }) => {
            Arc::new(DAE::Element::VAR { componentRef: a1.clone(), kind: a2.clone(), direction: a3.clone(), parallelism: prl.clone(), protection: a4.clone(), ty: newType.clone(), binding: a6.clone(), dims: a7.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: a11.clone(), comment: a12.clone(), innerOuter: a13.clone(), encrypted: e.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outelem)
}

pub fn replaceCrefandTypeInVar(mut newCr: Arc<DAE::ComponentRef>, mut newType: Arc<DAE::Type>, mut inelem: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outelem: Arc<DAE::Element>;
    outelem = (::match_deref::match_deref! { match &((newCr.clone(), newType.clone(), inelem.clone())) {
        (_, _, Deref @ DAE::Element::VAR { componentRef: _, kind: a2, direction: a3, parallelism: prl, protection: a4, ty: _, binding: a6, dims: a7, connectorType: ct, source, variableAttributesOption: a11, comment: a12, innerOuter: a13, encrypted: e }) => {
            outelem = Arc::new(DAE::Element::VAR { componentRef: newCr.clone(), kind: a2.clone(), direction: a3.clone(), parallelism: prl.clone(), protection: a4.clone(), ty: newType.clone(), binding: a6.clone(), dims: a7.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: a11.clone(), comment: a12.clone(), innerOuter: a13.clone(), encrypted: e.clone() });
            outelem.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outelem)
}

pub fn replaceBindungInVar(mut newBindung: Arc<DAE::Exp>, mut inelem: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outelem: Arc<DAE::Element>;
    outelem = (::match_deref::match_deref! { match &((newBindung.clone(), inelem.clone())) {
        (_, Deref @ DAE::Element::VAR { componentRef: a1, kind: a2, direction: a3, parallelism: prl, protection: a4, ty: a5, binding: _, dims: a7, connectorType: ct, source, variableAttributesOption: a11, comment: a12, innerOuter: a13, encrypted: e }) => {
            Arc::new(DAE::Element::VAR { componentRef: a1.clone(), kind: a2.clone(), direction: a3.clone(), parallelism: prl.clone(), protection: a4.clone(), ty: a5.clone(), binding: Some(newBindung.clone()), dims: a7.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: a11.clone(), comment: a12.clone(), innerOuter: a13.clone(), encrypted: e.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outelem)
}

fn toModelicaFormExpOpt(mut inExpExpOption: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExpExpOption: Option<Arc<DAE::Exp>> = None;
    outExpExpOption = (::match_deref::match_deref! { match &(inExpExpOption.clone()) {
        Some(e) => {
            let mut e_1: Arc<DAE::Exp>;
            e_1 = toModelicaFormExp(e.clone())?;
            Some(e_1.clone())
        },
        None => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpExpOption)
}

fn toModelicaFormCref(mut cr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut str_1: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    r#str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
    ty = ComponentReference::crefLastType(cr.clone())?;
    str_1 = (Util::stringReplaceChar((r#str.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
    outComponentRef = ComponentReferenceBasics::makeCrefIdent((str_1.clone()).clone(), ty.clone(), metamodelica::nil());
    Ok(outComponentRef)
}

fn toModelicaFormExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty: t, componentRef: cr } => {
                    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cr_1 = toModelicaFormCref(cr.clone())?;
                    Ok(Expression::makeCrefExp(cr_1.clone(), t.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone())?;
                    e2_1 = toModelicaFormExp(e2.clone())?;
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone())?;
                    e2_1 = toModelicaFormExp(e2.clone())?;
                    Ok(Arc::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { exp: e, operator: op } => {
                    let mut e_1: Arc<DAE::Exp>;
                    e_1 = toModelicaFormExp(e.clone())?;
                    Ok(Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { exp: e, operator: op } => {
                    let mut e_1: Arc<DAE::Exp>;
                    e_1 = toModelicaFormExp(e.clone())?;
                    Ok(Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { optionExpisASUB, index: i, exp2: e2, operator: op, exp1: e1 } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone())?;
                    e2_1 = toModelicaFormExp(e2.clone())?;
                    Ok(Arc::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone(), index: i.clone(), optionExpisASUB: optionExpisASUB.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 } => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut e3_1: Arc<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone())?;
                    e2_1 = toModelicaFormExp(e2.clone())?;
                    e3_1 = toModelicaFormExp(e3.clone())?;
                    Ok(Arc::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr, expLst: expl, path: f } => {
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expl_1 = List::map(expl.clone(), Arc::new(toModelicaFormExp));
                    Ok(Arc::new(DAE::Exp::CALL { path: f.clone(), expLst: expl_1.clone(), attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, scalar: b, ty: t } => {
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expl_1 = List::map(expl.clone(), Arc::new(toModelicaFormExp));
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: b.clone(), array: expl_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expl } => {
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    expl_1 = List::map(expl.clone(), Arc::new(toModelicaFormExp));
                    Ok(Arc::new(DAE::Exp::TUPLE { PR: expl_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp: e, ty: t } => {
                    let mut e_1: Arc<DAE::Exp>;
                    e_1 = toModelicaFormExp(e.clone())?;
                    Ok(Arc::new(DAE::Exp::CAST { ty: t.clone(), exp: e_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { sub: subs, exp: e } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    e_1 = toModelicaFormExp(e.clone())?;
                    expl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    Ok(Expression::makeASUB(e_1.clone(), expl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { sz: eopt, exp: e } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut eopt_1: Option<Arc<DAE::Exp>> = None;
                    e_1 = toModelicaFormExp(e.clone())?;
                    eopt_1 = toModelicaFormExpOpt(eopt.clone())?;
                    Ok(Arc::new(DAE::Exp::SIZE { exp: e_1.clone(), sz: eopt_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn getNamedFunctionFromList(mut ipath: Arc<Absyn::Path>, mut ifns: Arc<metamodelica::List<DAE::Function>>) -> Result<DAE::Function> {
    let mut r#fn: DAE::Function;
    r#fn = 'mc: {
        let __mc_input = (ipath.clone(), ifns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ metamodelica::List::Cons { head: r#fn, tail: _ }) => {
                    let true = (AbsynUtil::pathEqual(functionName(r#fn.clone())?, path.clone())) else { bail!("pattern mismatch") };
                    Ok(r#fn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ metamodelica::List::Cons { head: _, tail: fns }) => {
                    Ok(getNamedFunctionFromList(path.clone(), fns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ metamodelica::List::Nil) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- DAEUtil.getNamedFunctionFromList failed ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(r#fn)
}

pub fn getFunctionVisibility(mut r#fn: DAE::Function) -> SCode::Visibility {
    let mut visibility: SCode::Visibility = SCode::Visibility::PROTECTED;
    visibility = (match r#fn.clone() {
        DAE::Function::FUNCTION { visibility: mut visibility, .. } => visibility.clone(),
        _ => openmodelica_frontend_types::SCode::Visibility::PUBLIC,
    });
    visibility
}

fn getFunctionsElements(mut elements: Arc<metamodelica::List<DAE::Function>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
    elsList = List::map(elements.clone(), Arc::new(getFunctionElements));
    els = List::flatten(elsList.clone());
    els
}

pub fn getFunctionElements(mut r#fn: DAE::Function) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    els = (::match_deref::match_deref! { match &(r#fn.clone()) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: elements }, tail: _ }, .. } => {
            elements.clone()
        },
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { body: elements, .. }, tail: _ }, .. } => {
            elements.clone()
        },
        DAE::Function::RECORD_CONSTRUCTOR { .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(els)
}

pub fn getFunctionType(mut r#fn: DAE::Function) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (match r#fn.clone() {
        DAE::Function::FUNCTION { type_: ref outType, .. } => {
            outType.clone()
        },
        DAE::Function::FUNCTION { type_: ref outType, .. } => {
            outType.clone()
        },
        DAE::Function::RECORD_CONSTRUCTOR { type_: ref outType, .. } => {
            outType.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outType)
}

pub fn getFunctionImpureAttribute(mut r#fn: DAE::Function) -> Result<bool> {
    let mut outImpure: bool = false;
    outImpure = (match r#fn.clone() {
        DAE::Function::FUNCTION { isImpure: mut outImpure, .. } => outImpure.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outImpure)
}

pub fn getFunctionInlineType(mut r#fn: DAE::Function) -> Result<DAE::InlineType> {
    let mut outInlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    outInlineType = (match r#fn.clone() {
        DAE::Function::FUNCTION { inlineType: mut outInlineType, .. } => outInlineType.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outInlineType)
}

pub fn getFunctionInputVars(mut r#fn: DAE::Function) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elements = getFunctionElements(r#fn.clone())?;
    outEls = List::filterOnTrue(elements.clone(), Arc::new(fnptr!(isInputVar, Arc<DAE::Element>)));
    Ok(outEls)
}

pub fn getFunctionOutputVars(mut r#fn: DAE::Function) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elements = getFunctionElements(r#fn.clone())?;
    outEls = List::filterOnTrue(elements.clone(), Arc::new(fnptr!(isOutputElement, Arc<DAE::Element>)));
    Ok(outEls)
}

pub fn getFunctionProtectedVars(mut r#fn: DAE::Function) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elements = getFunctionElements(r#fn.clone())?;
    outEls = List::filterOnTrue(elements.clone(), Arc::new(fnptr!(isProtectedVar, Arc<DAE::Element>)));
    Ok(outEls)
}

pub fn getFunctionAlgorithms(mut r#fn: DAE::Function) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elements = getFunctionElements(r#fn.clone())?;
    outEls = List::filterOnTrue(elements.clone(), Arc::new(fnptr!(isAlgorithm, Arc<DAE::Element>)));
    Ok(outEls)
}

pub fn getFunctionAlgorithmStmts(mut r#fn: DAE::Function) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut bodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elements = getFunctionElements(r#fn.clone())?;
    bodyStmts = List::mapFlat(List::filterOnTrue(elements.clone(), Arc::new(fnptr!(isAlgorithm, Arc<DAE::Element>))), Arc::new(getStatement));
    Ok(bodyStmts)
}

pub fn getStatement(mut inElement: Arc<DAE::Element>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outStatements = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
                    Ok(stmts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Differentiatte.getStatement failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStatements)
}

pub fn getTupleSize(mut inExp: Arc<DAE::Exp>) -> i32 {
    let mut size: i32 = 0;
    size = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::TUPLE { PR: exps } => {
            size = (exps.clone().len() as i32);
            size.clone()
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    size
}

pub fn getTupleExps(mut inExp: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    exps = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::TUPLE { PR: exps } => exps.clone(),
        _ => list![inExp.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exps
}

fn crefToExp(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Expression::makeCrefExp(inComponentRef.clone(), DAE::T_UNKNOWN_DEFAULT.clone())?;
    Ok(outExp)
}

pub fn verifyEquationsDAE(mut dae: DAE::DAElist) -> Result<()> {
    let mut cond: Arc<DAE::Exp>;
    let mut dae_elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut ew: Option<Arc<DAE::Element>> = None;
    let mut source: Arc<DAE::ElementSource>;
    let mut el: Arc<DAE::Element>;
    let mut info: SourceInfo;
    let DAE::DAE { elementLst: __pa0 } = (dae.clone()) else { bail!("pattern mismatch") };
    dae_elts = __pa0.clone();
    for mut el in &*dae_elts.clone() {
        let mut el = el.clone();
        let () = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqs, elsewhen_: ew, source } => {
            verifyWhenEquation(cond.clone(), eqs.clone(), ew.clone(), source.clone())?;
            ()
        },
        Deref @ DAE::Element::REINIT { .. } => {
            info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(el.clone())?);
            Error::addSourceMessageAndFail(Error::REINIT_NOT_IN_WHEN.clone(), metamodelica::nil(), info.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn verifyWhenEquation(mut cond: Arc<DAE::Exp>, mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut ew: Option<Arc<DAE::Element>>, mut source: Arc<DAE::ElementSource>) -> Result<()> {
    if Types::isClockOrSubTypeClock(Expression::r#typeof(cond.clone())?)? {
        verifyClockWhenEquation(cond.clone(), eqs.clone(), ew.clone(), source.clone())?;
    } else {
        verifyBoolWhenEquation(cond.clone(), eqs.clone(), ew.clone(), source.clone())?;
    }
    Ok(())
}

fn verifyClockWhenEquation(mut cond: Arc<DAE::Exp>, mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut ew: Option<Arc<DAE::Element>>, mut source: Arc<DAE::ElementSource>) -> Result<()> {
    let mut info: SourceInfo;
    if isSome(ew.clone()) {
        info = ElementSource::getElementSourceFileInfo(source.clone());
        Error::addSourceMessageAndFail(Error::ELSE_WHEN_CLOCK.clone(), metamodelica::nil(), info.clone())?;
    }
    verifyClockWhenEquation1(eqs.clone())?;
    Ok(())
}

fn verifyClockWhenEquation1(mut inEqs: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<()> {
    let mut el: Arc<DAE::Element>;
    for mut el in &*inEqs.clone() {
        let mut el = el.clone();
        let () = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ DAE::Element::REINIT { .. } => {
            let mut info: SourceInfo;
            info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(el.clone())?);
            Error::addSourceMessageAndFail(Error::REINIT_NOT_IN_WHEN.clone(), metamodelica::nil(), info.clone())?;
            ()
        },
        Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqs, elsewhen_: ew, source } => {
            let mut info: SourceInfo;
            if Types::isClockOrSubTypeClock(Expression::r#typeof(cond.clone())?)? {
                info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(el.clone())?);
                Error::addSourceMessageAndFail(Error::NESTED_CLOCKED_WHEN.clone(), metamodelica::nil(), info.clone())?;
            }
            verifyBoolWhenEquation(cond.clone(), eqs.clone(), ew.clone(), source.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn verifyBoolWhenEquation(mut inCond: Arc<DAE::Exp>, mut inEqs: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inElseWhen: Option<Arc<DAE::Element>>, mut source: Arc<DAE::ElementSource>) -> Result<()> {
    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut whenBranches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)>> = metamodelica::nil();
    let mut whenBranch: (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>);
    let mut cond: Arc<DAE::Exp>;
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut info: SourceInfo;
    crefs1 = verifyBoolWhenEquationBranch(inCond.clone(), inEqs.clone())?;
    whenBranches = collectWhenEquationBranches(inElseWhen.clone(), metamodelica::nil())?;
    for mut whenBranch in &*whenBranches.clone() {
        let mut whenBranch = whenBranch.clone();
        (cond, eqs) = whenBranch.clone();
        if Types::isClockOrSubTypeClock(Expression::r#typeof(cond.clone())?)? {
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessageAndFail(Error::CLOCKED_WHEN_BRANCH.clone(), metamodelica::nil(), info.clone())?;
        }
        crefs2 = verifyBoolWhenEquationBranch(cond.clone(), eqs.clone())?;
        crefs2 = List::unionOnTrue(crefs1.clone(), crefs2.clone(), Arc::new(ComponentReferenceBasics::crefEqual));
        if (crefs2.clone().len() as i32) != (crefs1.clone().len() as i32) {
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessageAndFail(Error::DIFFERENT_VARIABLES_SOLVED_IN_ELSEWHEN.clone(), metamodelica::nil(), info.clone())?;
        }
    }
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn collectWhenEquationBranches(mut inElseWhen: Option<Arc<DAE::Element>>, mut inWhenBranches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)>>> {
    let mut outWhenBranches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)>> = metamodelica::nil();
    outWhenBranches = (::match_deref::match_deref! { match &(inElseWhen.clone()) {
        None => {
            inWhenBranches.clone()
        },
        Some(Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqs, elsewhen_: ew, source: _ }) => {
            collectWhenEquationBranches(ew.clone(), cons((cond.clone(), eqs.clone()), inWhenBranches.clone()))?
        },
        Some(el) => {
            let mut info: SourceInfo;
            let mut msg: ArcStr = arcstr::literal!("");
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- DAEUtil.collectWhenEquationBranches failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![el.clone()])?); ArcStr::from(__mm_s) }).clone();
            info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(el.clone())?);
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outWhenBranches)
}

fn verifyBoolWhenEquationBranch(mut inCond: Arc<DAE::Exp>, mut inEqs: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut initCond: bool = Expression::containsInitialCall(inCond.clone());
    crefs = verifyBoolWhenEquation1(inEqs.clone(), initCond.clone(), metamodelica::nil())?;
    Ok(crefs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn verifyBoolWhenEquation1(mut inElems: Arc<metamodelica::List<Arc<DAE::Element>>>, mut initCond: bool, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outCrefs = (::match_deref::match_deref! { match &(inElems.clone()) {
        Deref @ metamodelica::List::Nil => {
            inCrefs.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { .. }, tail: rest } => {
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), inCrefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, .. }, tail: rest } => {
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), cons(cr.clone(), inCrefs.clone()))?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { source, exp: e, .. }, tail: rest } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crefs = collectWhenCrefs1(e.clone(), source.clone(), inCrefs.clone())?;
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), crefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { source, exp: e, .. }, tail: rest } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crefs = collectWhenCrefs1(e.clone(), source.clone(), inCrefs.clone())?;
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), crefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { source, lhs: e, .. }, tail: rest } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crefs = collectWhenCrefs1(e.clone(), source.clone(), inCrefs.clone())?;
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), crefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. }, tail: rest } => {
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), cons(cr.clone(), inCrefs.clone()))?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { source, equations3: falseEqs, equations2: trueEqs, .. }, tail: rest } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut crefsLists: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
            let mut info: SourceInfo;
            let mut b: bool = false;
            let mut msg: ArcStr = arcstr::literal!("");
            crefsLists = List::map2(trueEqs.clone(), Arc::new(verifyBoolWhenEquation1), initCond.clone(), metamodelica::nil());
            crefs = verifyBoolWhenEquation1(falseEqs.clone(), initCond.clone(), metamodelica::nil())?;
            crefsLists = cons(crefs.clone(), crefsLists.clone());
            (crefs, b) = compareCrefList(crefsLists.clone())?;
            if !(b.clone()) {
                info = ElementSource::getElementSourceFileInfo(source.clone());
                msg = (literal!("All branches must write to the same variable")).clone();
                Error::addSourceMessage(Error::WHEN_EQ_LHS.clone(), list![(msg.clone()).clone()], info.clone())?;
                bail!("fail");
            }
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), listAppend(crefs.clone(), inCrefs.clone()))?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { .. }, tail: rest } => {
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), inCrefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { .. }, tail: rest } => {
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), inCrefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::REINIT { source, .. }, tail: rest } => {
            let mut info: SourceInfo;
            if initCond.clone() {
                info = ElementSource::getElementSourceFileInfo(source.clone());
                Error::addSourceMessage(Error::REINIT_IN_WHEN_INITIAL.clone(), metamodelica::nil(), info.clone())?;
                bail!("fail");
            }
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), inCrefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::NORETCALL { .. }, tail: rest } => {
            verifyBoolWhenEquation1(rest.clone(), initCond.clone(), inCrefs.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { source, condition: e, .. }, tail: _ } => {
            let mut info: SourceInfo;
            info = ElementSource::getElementSourceFileInfo(source.clone());
            if Types::isClockOrSubTypeClock(Expression::r#typeof(e.clone())?)? {
                Error::addSourceMessage(Error::CLOCKED_WHEN_IN_WHEN_EQ.clone(), metamodelica::nil(), info.clone())?;
            } else {
                Error::addSourceMessage(Error::NESTED_WHEN.clone(), metamodelica::nil(), info.clone())?;
            }
            bail!("fail")
        },
        Deref @ metamodelica::List::Cons { head: el, tail: _ } => {
            let mut info: SourceInfo;
            let mut msg: ArcStr = arcstr::literal!("");
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- DAEUtil.verifyWhenEquationStatements failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![el.clone()])?); ArcStr::from(__mm_s) }).clone();
            info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(el.clone())?);
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefs)
}

fn collectWhenCrefs(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut source: Arc<DAE::ElementSource>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Arc<metamodelica::List<Arc<DAE::ComponentRef>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outCrefs = List::fold1(inExps.clone(), Arc::new(collectWhenCrefs1), source.clone(), inCrefs.clone());
    outCrefs
}

fn collectWhenCrefs1(mut inExp: Arc<DAE::Exp>, mut source: Arc<DAE::ElementSource>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCrefs = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ } => {
            cons(cr.clone(), inCrefs.clone())
        },
        Deref @ DAE::Exp::TUPLE { PR: exps } => {
            collectWhenCrefs(exps.clone(), source.clone(), inCrefs.clone())
        },
        _ => {
            let mut msg: ArcStr = arcstr::literal!("");
            let mut info: SourceInfo;
            msg = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessage(Error::WHEN_EQ_LHS.clone(), list![(msg.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefs)
}

fn compareCrefList(mut inCrefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool)> {
    let mut outrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut matching: bool = false;
    (outrefs, matching) = (::match_deref::match_deref! { match &(inCrefs.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), true)
        },
        Deref @ metamodelica::List::Cons { head: crefs, tail: Deref @ metamodelica::List::Nil } => {
            (crefs.clone(), true)
        },
        Deref @ metamodelica::List::Cons { head: crefs, tail: llrefs } => {
            let mut recRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut i: i32 = 0;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut b3: bool = false;
            let mut crefs = (*crefs).clone();
            (recRefs, b3) = compareCrefList(llrefs.clone())?;
            i = (recRefs.clone().len() as i32);
            if intGt(i.clone(), 0) {
                b1 = 0 == intMod((crefs.clone().len() as i32), i.clone());
                crefs = List::unionOnTrueList(list![recRefs.clone(), crefs.clone()], Arc::new(ComponentReferenceBasics::crefEqual))?;
                b2 = intEq((crefs.clone().len() as i32), i.clone());
                b1 = boolAnd(b1.clone(), boolAnd(b2.clone(), b3.clone()));
            } else {
                let true = (intEq(i.clone(), 0)) else { bail!("pattern mismatch") };
                let true = (crefs.clone().is_empty()) else { bail!("pattern mismatch") };
                b1 = true;
            }
            (crefs.clone(), b1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outrefs, matching))
}

pub fn evaluateAnnotation(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inDAElist: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDAElist: DAE::DAElist;
    outDAElist = 'mc: {
        let __mc_input = (inCache.clone(), env.clone(), inDAElist.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, ref dae @ DAE::DAElist { elementLst: ref elts }) = __mc_input.clone() else { bail!("nomatch") };
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut pv: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut elts2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            pv = getParameterVars(dae.clone(), HashTable2::emptyHashTable())?;
            let (__pa0, true) = (evaluateAnnotation1(dae.clone(), pv.clone(), HashTable2::emptyHashTable())?) else { bail!("pattern mismatch") };
            ht = __pa0.clone();
            (_, ht1, _) = evaluateAnnotation2_loop(inCache.clone(), env.clone(), dae.clone(), ht.clone(), BaseHashTable::hashTableCurrentSize(ht.clone()))?;
            (elts2, _) = traverseDAEElementList(elts.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (evaluateAnnotationTraverse, (ht1.clone(), 0, 0)));
            Ok(DAE::DAElist { elementLst: elts2.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inDAElist.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDAElist)
}

fn evaluateAnnotationTraverse(mut inExp: Arc<DAE::Exp>, mut itpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), i32, i32)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), i32, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut otpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), i32, i32);
    (outExp, otpl) = 'mc: {
        let __mc_input = (inExp.clone(), itpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, (ht, i, j)) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut k: i32 = 0;
                    let mut ht = (*ht).clone();
                    let mut i = (*i).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(exp.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    let (__pa1, (__pa2, __pa3, __pa4)) = Expression::traverseExpBottomUp(e1.clone(), Arc::new(evaluateAnnotationTraverse), itpl.clone())?;
                    e1 = __pa1.clone();
                    ht = __pa2.clone();
                    i = __pa3.clone();
                    k = __pa4.clone();
                    let true = (intGt(k.clone(), j.clone())) else { bail!("pattern mismatch") };
                    Ok((e1.clone(), (ht.clone(), i.clone(), k.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, (ht, i, j)) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut k: i32 = 0;
                    let mut ht = (*ht).clone();
                    let mut i = (*i).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(exp.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    let (__pa1, (__pa2, __pa3, __pa4)) = Expression::traverseExpBottomUp(e1.clone(), Arc::new(evaluateAnnotationTraverse), itpl.clone())?;
                    e1 = __pa1.clone();
                    ht = __pa2.clone();
                    i = __pa3.clone();
                    k = __pa4.clone();
                    let true = (intGt(k.clone(), j.clone())) else { bail!("pattern mismatch") };
                    Ok((e1.clone(), (ht.clone(), i.clone(), k.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { .. }, (ht, i, j)) => {
                    let mut e1: Arc<DAE::Exp>;
                    e1 = replaceCrefInAnnotation(exp.clone(), ht.clone())?;
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok((e1.clone(), (ht.clone(), i.clone(), j.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { .. }, (ht, i, j)) => {
                    Ok((exp.clone(), (ht.clone(), i.clone() + 1, j.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, otpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn replaceCrefInAnnotation(mut inExp: Arc<DAE::Exp>, mut inTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp.clone(), inTable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = BaseHashTable::get(cr.clone(), inTable.clone())?;
                    Ok(replaceCrefInAnnotation(exp.clone(), inTable.clone())?)
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

pub fn getParameterVars(mut dae: DAE::DAElist, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))> {
    let mut oht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (dae.clone()) else { bail!("pattern mismatch") };
    elts = __pa0.clone();
    oht = List::fold(elts.clone(), Arc::new(getParameterVars2), ht.clone());
    Ok(oht)
}

fn getParameterVars2(mut elt: Arc<DAE::Element>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))> {
    let mut ouHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    ouHt = 'mc: {
        let __mc_input = (elt.clone(), ht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::COMP { dAElist: elts, .. }, _) => {
                    Ok(List::fold(elts.clone(), Arc::new(getParameterVars2), ht.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { binding: Some(e), kind: DAE::VarKind::PARAM, componentRef: cr, .. }, _) => {
                    Ok(BaseHashTable::add((cr.clone(), e.clone()), ht.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { variableAttributesOption: dae_var_attr, kind: DAE::VarKind::PARAM, componentRef: cr, .. }, _) => {
                    let mut e: Arc<DAE::Exp>;
                    e = getStartAttrFail(dae_var_attr.clone())?;
                    Ok(BaseHashTable::add((cr.clone(), e.clone()), ht.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ht.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ouHt)
}

pub fn evaluateAnnotation1(mut dae: DAE::DAElist, mut pv: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), bool)> {
    let mut oht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut hasEvaluate: bool = false;
    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (dae.clone()) else { bail!("pattern mismatch") };
    elts = __pa0.clone();
    (oht, hasEvaluate) = List::fold1r(elts.clone(), Arc::new(evaluateAnnotation1Fold), pv.clone(), (ht.clone(), false));
    Ok((oht, hasEvaluate))
}

fn evaluateAnnotation1Fold(mut tpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), bool), mut el: Arc<DAE::Element>, mut inPV: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), bool)> {
    let mut otpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), bool);
    otpl = 'mc: {
        let __mc_input = (tpl.clone(), el.clone(), inPV.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Element::COMP { dAElist: sublist, .. }, pv) => {
                    Ok(List::fold1r(sublist.clone(), Arc::new(evaluateAnnotation1Fold), pv.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((ht, _), Deref @ DAE::Element::VAR { comment: Some(comment), binding: Some(e), kind: DAE::VarKind::PARAM, componentRef: cr, .. }, pv) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut anno: Arc<SCode::Annotation>;
                    let mut e1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(comment.clone()) {
                        Deref @ SCode::Comment { annotation_: Some(__pa0), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    anno = __pa0.clone();
                    let true = (SCodeUtil::hasBooleanNamedAnnotation(anno.clone(), (literal!("Evaluate")).clone())?) else { bail!("pattern mismatch") };
                    e1 = evaluateParameter(e.clone(), pv.clone())?;
                    ht1 = BaseHashTable::add((cr.clone(), e1.clone()), ht.clone())?;
                    Ok((ht1.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(otpl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn evaluateParameter(mut inExp: Arc<DAE::Exp>, mut inPV: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp.clone(), inPV.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let true = (Expression::isConst(e.clone())?) else { bail!("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let false = (Expression::expHasCrefs(e.clone())?) else { bail!("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, pv) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut i: i32 = 0;
                    let (__pa0, (_, __pa1, _)) = Expression::traverseExpBottomUp(e.clone(), Arc::new(evaluateAnnotationTraverse), (pv.clone(), 0, 0))?;
                    e1 = __pa0.clone();
                    i = __pa1.clone();
                    let true = (intEq(i.clone(), 0)) else { bail!("pattern mismatch") };
                    e2 = evaluateParameter(e1.clone(), pv.clone())?;
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn evaluateAnnotation2_loop(mut cache: FCore::Cache, mut env: FCore::Graph, mut inDAElist: DAE::DAElist, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), mut sizeBefore: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache)> {
    let mut outDAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut newsize: i32 = 0;
    (outDAElist, outHt, outCache) = evaluateAnnotation2(cache.clone(), env.clone(), inDAElist.clone(), inHt.clone())?;
    newsize = BaseHashTable::hashTableCurrentSize(outHt.clone());
    (outDAElist, outHt, outCache) = evaluateAnnotation2_loop1(intEq(newsize.clone(), sizeBefore.clone()), outCache.clone(), env.clone(), DAE::DAElist { elementLst: outDAElist.clone() }, outHt.clone(), newsize.clone())?;
    Ok((outDAElist, outHt, outCache))
}

fn evaluateAnnotation2_loop1(mut finish: bool, mut inCache: FCore::Cache, mut env: FCore::Graph, mut inDAElist: DAE::DAElist, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), mut sizeBefore: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache)> {
    let mut outDAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outDAElist, outHt, outCache) = (match (finish.clone(), inCache.clone(), env.clone(), inDAElist.clone(), inHt.clone(), sizeBefore.clone()) {
        (true, _, _, DAE::DAElist { elementLst: ref elst }, _, _) => {
            (elst.clone(), inHt.clone(), inCache.clone())
        },
        _ => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut elst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            (elst, ht, cache) = evaluateAnnotation2_loop(inCache.clone(), env.clone(), inDAElist.clone(), inHt.clone(), sizeBefore.clone())?;
            (elst.clone(), ht.clone(), cache.clone())
        },
    });
    Ok((outDAElist, outHt, outCache))
}

fn evaluateAnnotation2(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inDAElist: DAE::DAElist, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache)> {
    let mut outDAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outDAElist, outHt, outCache) = 'mc: {
        let __mc_input = (inCache.clone(), env.clone(), inDAElist.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, DAE::DAElist { elementLst: Deref @ metamodelica::List::Nil }, ht) => {
                    Ok((metamodelica::nil(), ht.clone(), inCache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, DAE::DAElist { elementLst }, ht) => {
                    let mut elementLst1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let (__pa0, (__pa1, __pa2, _)) = List::mapFold(elementLst.clone(), Arc::new(evaluateAnnotation3), (ht.clone(), inCache.clone(), env.clone()));
                    elementLst1 = __pa0.clone();
                    ht1 = __pa1.clone();
                    cache = __pa2.clone();
                    Ok((elementLst1.clone(), ht1.clone(), cache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDAElist, outHt, outCache))
}

fn evaluateAnnotation3(mut iel: Arc<DAE::Element>, mut inHt: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache, FCore::Graph)) -> Result<(Arc<DAE::Element>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache, FCore::Graph))> {
    let mut oel: Arc<DAE::Element>;
    let mut outHt: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache, FCore::Graph);
    (oel, outHt) = 'mc: {
        let __mc_input = (iel.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::COMP { comment, source, dAElist: sublist, ident }, _) => {
                    let mut httpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache, FCore::Graph);
                    let mut sublist1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    (sublist1, httpl) = List::mapFold(sublist.clone(), Arc::new(evaluateAnnotation3), inHt.clone());
                    Ok((Arc::new(DAE::Element::COMP { ident: (ident.clone()).clone(), dAElist: sublist1.clone(), source: source.clone(), comment: comment.clone() }), httpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { encrypted, innerOuter, comment: absynCommentOption, variableAttributesOption, source, connectorType: ct, dims, binding: Some(e), ty, protection, parallelism, direction, kind: DAE::VarKind::PARAM, componentRef: cr }, (ht, cache, env)) => {
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut i: i32 = 0;
                    let mut j: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, (_, __pa1, __pa2)) = Expression::traverseExpBottomUp(e.clone(), Arc::new(evaluateAnnotationTraverse), (ht.clone(), 0, 0))?;
                    e1 = __pa0.clone();
                    i = __pa1.clone();
                    j = __pa2.clone();
                    (e2, ht1, cache) = evaluateAnnotation4(cache.clone(), env.clone(), cr.clone(), e1.clone(), i.clone(), j.clone(), ht.clone())?;
                    Ok((Arc::new(DAE::Element::VAR { componentRef: cr.clone(), kind: openmodelica_frontend_types::DAE::VarKind::PARAM, direction: direction.clone(), parallelism: parallelism.clone(), protection: protection.clone(), ty: ty.clone(), binding: Some(e2.clone()), dims: dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: variableAttributesOption.clone(), comment: absynCommentOption.clone(), innerOuter: innerOuter.clone(), encrypted: encrypted.clone() }), (ht1.clone(), cache.clone(), env.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iel.clone(), inHt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oel, outHt))
}

fn evaluateAnnotation4(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inCr: Arc<DAE::ComponentRef>, mut inExp: Arc<DAE::Exp>, mut inInteger1: i32, mut inInteger2: i32, mut inHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), FCore::Cache)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outExp, outHt, outCache) = 'mc: {
        let __mc_input = (inCache.clone(), env.clone(), inCr.clone(), inExp.clone(), inInteger1.clone(), inInteger2.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cr, e, i, j, ht) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut ht = (*ht).clone();
                    let true = (intGt(j.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intEq(i.clone(), 0)) else { bail!("pattern mismatch") };
                    let (__pa0, (__pa1, _, _)) = Expression::traverseExpBottomUp(e.clone(), Arc::new(evaluateAnnotationTraverse), (ht.clone(), 0, 0))?;
                    e1 = __pa0.clone();
                    ht = __pa1.clone();
                    (cache, value) = Ceval::ceval(inCache.clone(), env.clone(), e1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    ht1 = BaseHashTable::add((cr.clone(), e1.clone()), ht.clone())?;
                    Ok((e1.clone(), ht1.clone(), cache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, e, _, _, ht) => {
                    Ok((e.clone(), ht.clone(), inCache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outHt, outCache))
}

pub fn renameUniqueOuterVars(mut dae: DAE::DAElist) -> DAE::DAElist {
    let mut odae: DAE::DAElist;
    (odae, _, _) = traverseDAE(dae.clone(), AvlTreePathFunction::Tree::EMPTY().unwrap(), Arc::new(Expression::traverseSubexpressionsHelper), (removeUniqieIdentifierFromCref, metamodelica::nil()));
    odae
}

fn removeUniqieIdentifierFromCref<Type_a: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut oarg: Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDummy: Type_a;
    (outExp, outDummy) = 'mc: {
        let __mc_input = (inExp.clone(), oarg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty }, _) => {
                    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp: Arc<DAE::Exp>;
                    cr2 = unNameInnerouterUniqueCref(cr.clone(), (arcstr::literal!(DAE::UNIQUEIO)).clone())?;
                    exp = Expression::makeCrefExp(cr2.clone(), ty.clone())?;
                    Ok((exp.clone(), oarg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), oarg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outDummy))
}

pub fn nameUniqueOuterVars(mut dae: DAE::DAElist) -> DAE::DAElist {
    let mut odae: DAE::DAElist;
    (odae, _, _) = traverseDAE(dae.clone(), AvlTreePathFunction::Tree::EMPTY().unwrap(), Arc::new(Expression::traverseSubexpressionsHelper), (addUniqueIdentifierToCref, metamodelica::nil()));
    odae
}

fn addUniqueIdentifierToCref<Type_a: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut oarg: Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDummy: Type_a;
    (outExp, outDummy) = 'mc: {
        let __mc_input = (inExp.clone(), oarg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty }, _) => {
                    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut exp: Arc<DAE::Exp>;
                    cr2 = nameInnerouterUniqueCref(cr.clone())?;
                    exp = Expression::makeCrefExp(cr2.clone(), ty.clone())?;
                    Ok((exp.clone(), oarg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), oarg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outDummy))
}

// helper functions for traverseDAE
fn traverseDAEOptExp<Type_a: Clone + 'static>(mut oexp: Option<Arc<DAE::Exp>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iextraArg: Type_a) -> Result<(Option<Arc<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut ooexp: Option<Arc<DAE::Exp>> = None;
    let mut oextraArg: Type_a;
    (ooexp, oextraArg) = (::match_deref::match_deref! { match &((oexp.clone(), func.clone(), iextraArg.clone())) {
        (None, _, extraArg) => {
            (None, extraArg.clone())
        },
        (Some(e), _, extraArg) => {
            let mut e = (*e).clone();
            let mut extraArg = (*extraArg).clone();
            (e, extraArg) = func(e.clone(), extraArg.clone())?;
            (Some(e.clone()), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((ooexp, oextraArg))
}

fn traverseDAEExpList<Type_a: Clone + 'static>(mut iexps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iextraArg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut oexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut oextraArg: Type_a;
    (oexps, oextraArg) = (::match_deref::match_deref! { match &((iexps.clone(), func.clone(), iextraArg.clone())) {
        (Deref @ metamodelica::List::Nil, _, extraArg) => {
            (metamodelica::nil(), extraArg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: exps }, _, extraArg) => {
            let mut e = (*e).clone();
            let mut extraArg = (*extraArg).clone();
            (e, extraArg) = func(e.clone(), extraArg.clone())?;
            (oexps, extraArg) = traverseDAEExpList(exps.clone(), func.clone(), extraArg.clone())?;
            (cons(e.clone(), oexps.clone()), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oexps, oextraArg))
}

fn traverseDAEList<Type_a: Clone + 'static>(mut idaeList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iextraArg: Type_a) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut traversedDaeList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
    let mut oextraArg: Type_a;
    (traversedDaeList, oextraArg) = (::match_deref::match_deref! { match &((idaeList.clone(), func.clone(), iextraArg.clone())) {
        (Deref @ metamodelica::List::Nil, _, extraArg) => {
            (metamodelica::nil(), extraArg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: branch, tail: daeList }, _, extraArg) => {
            let mut branch2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut recRes: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut extraArg = (*extraArg).clone();
            (branch2, extraArg) = traverseDAEElementList(branch.clone(), func.clone(), extraArg.clone());
            (recRes, extraArg) = traverseDAEList(daeList.clone(), func.clone(), extraArg.clone())?;
            (cons(branch2.clone(), recRes.clone()), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((traversedDaeList, oextraArg))
}

pub fn traverseDAEFunctions<ArgT: Clone + 'static>(mut functions: Arc<metamodelica::List<DAE::Function>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<metamodelica::List<DAE::Function>>, ArgT) {
    pub type FuncExpType<ArgT: Clone> = fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)>;

    let mut functions: Arc<metamodelica::List<DAE::Function>> = functions;
    let mut arg: ArgT = arg;
    (functions, arg) = List::mapFold(functions.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseDAEFunc(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
    (functions, arg)
}

fn traverseDAEFunc<ArgT: Clone + 'static>(mut daeFunction: DAE::Function, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(DAE::Function, ArgT)> {
    pub type FuncExpType<ArgT: Clone> = fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)>;

    let mut daeFunction: DAE::Function = daeFunction;
    let mut arg: ArgT = arg;
    let _ = (::match_deref::match_deref! { match &(daeFunction.clone()) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: fdef @ DAE::FunctionDefinition::FUNCTION_DEF { .. }, tail: rest_defs }, .. } => {
            let mut el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut fdef = (*fdef).clone();
            (el, arg) = traverseDAEElementList(fdef.body.clone(), func.clone(), arg.clone());
            if !(referenceEq(&fdef.body.clone(),&el.clone())) {
                todo!("unhandled field-assign shape: fdef.body");
                let __owned_variant_functions_0 = cons(fdef.clone(), rest_defs.clone());
                if let DAE::Function::FUNCTION { functions, .. } = &mut daeFunction {
                    *functions = __owned_variant_functions_0;
                } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            }
            ()
        },
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: fdef @ DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: rest_defs }, .. } => {
            let mut el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut fdef = (*fdef).clone();
            (el, arg) = traverseDAEElementList(fdef.body.clone(), func.clone(), arg.clone());
            if !(referenceEq(&fdef.body.clone(),&el.clone())) {
                todo!("unhandled field-assign shape: fdef.body");
                let __owned_variant_functions_0 = cons(fdef.clone(), rest_defs.clone());
                if let DAE::Function::FUNCTION { functions, .. } = &mut daeFunction {
                    *functions = __owned_variant_functions_0;
                } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            }
            ()
        },
        DAE::Function::RECORD_CONSTRUCTOR { .. } => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((daeFunction, arg))
}

pub fn traverseDAEElementList<ArgT: Clone + 'static>(mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<metamodelica::List<Arc<DAE::Element>>>, ArgT) {
    pub type FuncExpType<ArgT: Clone> = fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)>;

    let mut elements: Arc<metamodelica::List<Arc<DAE::Element>>> = elements;
    let mut arg: ArgT = arg;
    (elements, arg) = List::mapFold(elements.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseDAEElement(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
    (elements, arg)
}

fn traverseDAEElement<ArgT: Clone + 'static>(mut element: Arc<DAE::Element>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<DAE::Element>, ArgT)> {
    pub type FuncExpType<ArgT: Clone> = fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)>;

    let mut element: Arc<DAE::Element> = element;
    let mut arg: ArgT = arg;
    let _ = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: attr, binding, componentRef: cr1, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut new_binding: Option<Arc<DAE::Exp>> = None;
            let mut new_attr: Option<Arc<DAE::VariableAttributes>> = None;
            let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut daebinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut new_daebinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let mut new_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            (e1, arg) = func(Expression::crefExp(cr1.clone())?, arg.clone())?;
            if Expression::isCref(e1.clone()) {
                new_cr1 = Expression::expCref(e1.clone())?;
                if !(referenceEq(&cr1.clone(),&new_cr1.clone())) {
                    assign_variant_field!(element => DAE::Element::VAR; componentRef = new_cr1.clone());
                }
            }
            assign_variant_field!(element => DAE::Element::VAR; dims = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for mut d in (var_field!((*element).dims, DAE::Element::VAR).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(d.clone()) {
        Deref @ DAE::Dimension::DIM_EXP { exp: e1 } => {
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&new_e1.clone())) {d.clone()} else {Arc::new(DAE::Dimension::DIM_EXP { exp: new_e1.clone() })}
        },
        _ => d.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            new_ty = { let ty = var_field!((*element).ty, DAE::Element::VAR).clone(); (::match_deref::match_deref! { match &(ty) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => {
            changed = false;
            varLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Var { binding: daebinding @ Deref @ DAE::Binding::EQBOUND { .. }, .. } => {
            let mut daebinding = (*daebinding).clone();
            (e2, arg) = func(var_field!((*daebinding).exp, DAE::Binding::EQBOUND).clone(), arg.clone())?;
            if !(referenceEq(&var_field!((*daebinding).exp, DAE::Binding::EQBOUND).clone(),&e2.clone())) {
                daebinding = Arc::new(DAE::Binding::EQBOUND { exp: e2.clone(), evaluatedExp: None, constant_: var_field!((*daebinding).constant_, DAE::Binding::EQBOUND).clone(), source: var_field!((*daebinding).source, DAE::Binding::EQBOUND).clone() });
                assign_field!(v.binding = daebinding.clone());
                changed = true;
            }
            v.clone()
        },
        Deref @ DAE::Var { binding: daebinding @ Deref @ DAE::Binding::VALBOUND { .. }, .. } => {
            e1 = ValuesUtil::valueExp(var_field!((**daebinding).valBound, DAE::Binding::VALBOUND).clone(), None)?;
            (e2, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                new_daebinding = Arc::new(DAE::Binding::EQBOUND { exp: e2.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: var_field!((**daebinding).source, DAE::Binding::VALBOUND).clone() });
                assign_field!(v.binding = new_daebinding.clone());
                changed = true;
            }
            v.clone()
        },
        _ => v.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if !(referenceEq(&varLst.clone(),&var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone())) {
                assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = varLst.clone());
            }
            ty.clone()
        },
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) };
            if !(referenceEq(&var_field!((*element).ty, DAE::Element::VAR).clone(),&new_ty.clone())) {
                assign_variant_field!(element => DAE::Element::VAR; ty = new_ty.clone());
            }
            (new_binding, arg) = traverseDAEOptExp(binding.clone(), func.clone(), arg.clone())?;
            if !(referenceEq(&binding.clone(),&new_binding.clone())) {
                assign_variant_field!(element => DAE::Element::VAR; binding = new_binding.clone());
            }
            (new_attr, arg) = traverseDAEVarAttr(attr.clone(), func.clone(), arg.clone())?;
            if !(referenceEq(&attr.clone(),&new_attr.clone())) {
                assign_variant_field!(element => DAE::Element::VAR; variableAttributesOption = new_attr.clone());
            }
            ()
        },
        Deref @ DAE::Element::DEFINE { exp: e1, componentRef: cr1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::DEFINE; exp = new_e1.clone());
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cr1 = __pa0.clone();
            arg = __pa1.clone();
            if !(referenceEq(&cr1.clone(),&new_cr1.clone())) {
                assign_variant_field!(element => DAE::Element::DEFINE; componentRef = new_cr1.clone());
            }
            ()
        },
        Deref @ DAE::Element::INITIALDEFINE { exp: e1, componentRef: cr1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::INITIALDEFINE; exp = new_e1.clone());
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cr1 = __pa0.clone();
            arg = __pa1.clone();
            if !(referenceEq(&cr1.clone(),&new_cr1.clone())) {
                assign_variant_field!(element => DAE::Element::INITIALDEFINE; componentRef = new_cr1.clone());
            }
            ()
        },
        Deref @ DAE::Element::EQUEQUATION { cr2, cr1, .. } => {
            let mut new_cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut new_cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cr1 = __pa0.clone();
            arg = __pa1.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr2.clone())?, arg.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa2, .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cr2 = __pa2.clone();
            arg = __pa3.clone();
            if !(referenceEq(&cr1.clone(),&new_cr1.clone())) || !(referenceEq(&cr2.clone(),&new_cr2.clone())) {
                element = Arc::new(DAE::Element::EQUEQUATION { cr1: new_cr1.clone(), cr2: new_cr2.clone(), source: var_field!((*element).source, DAE::Element::EQUEQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::EQUATION { scalar: e2, exp: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) || !(referenceEq(&e2.clone(),&new_e2.clone())) {
                element = Arc::new(DAE::Element::EQUATION { exp: new_e1.clone(), scalar: new_e2.clone(), source: var_field!((*element).source, DAE::Element::EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::INITIALEQUATION { exp2: e2, exp1: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) || !(referenceEq(&e2.clone(),&new_e2.clone())) {
                element = Arc::new(DAE::Element::INITIALEQUATION { exp1: new_e1.clone(), exp2: new_e2.clone(), source: var_field!((*element).source, DAE::Element::INITIALEQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { rhs: e2, lhs: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) || !(referenceEq(&e2.clone(),&new_e2.clone())) {
                element = Arc::new(DAE::Element::COMPLEX_EQUATION { lhs: new_e1.clone(), rhs: new_e2.clone(), source: var_field!((*element).source, DAE::Element::COMPLEX_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { rhs: e2, lhs: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) || !(referenceEq(&e2.clone(),&new_e2.clone())) {
                element = Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: new_e1.clone(), rhs: new_e2.clone(), source: var_field!((*element).source, DAE::Element::INITIAL_COMPLEX_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::ARRAY_EQUATION { array: e2, exp: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) || !(referenceEq(&e2.clone(),&new_e2.clone())) {
                element = Arc::new(DAE::Element::ARRAY_EQUATION { dimension: var_field!((*element).dimension, DAE::Element::ARRAY_EQUATION).clone(), exp: new_e1.clone(), array: new_e2.clone(), source: var_field!((*element).source, DAE::Element::ARRAY_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { array: e2, exp: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) || !(referenceEq(&e2.clone(),&new_e2.clone())) {
                element = Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: var_field!((*element).dimension, DAE::Element::INITIAL_ARRAY_EQUATION).clone(), exp: new_e1.clone(), array: new_e2.clone(), source: var_field!((*element).source, DAE::Element::INITIAL_ARRAY_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: el, condition: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e: Arc<DAE::Element>;
            let mut new_e: Arc<DAE::Element>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::WHEN_EQUATION; condition = new_e1.clone());
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::WHEN_EQUATION; equations = new_el.clone());
            }
            if isSome(var_field!((*element).elsewhen_, DAE::Element::WHEN_EQUATION).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*element).elsewhen_, DAE::Element::WHEN_EQUATION).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                (new_e, arg) = traverseDAEElement(e.clone(), func.clone(), arg.clone())?;
                if !(referenceEq(&e.clone(),&new_e.clone())) {
                    assign_variant_field!(element => DAE::Element::WHEN_EQUATION; elsewhen_ = Some(new_e.clone()));
                }
            }
            ()
        },
        Deref @ DAE::Element::FOR_EQUATION { equations: el, range: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::FOR_EQUATION; range = new_e1.clone());
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::FOR_EQUATION; equations = new_el.clone());
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { equations: el, range: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_FOR_EQUATION; range = new_e1.clone());
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_FOR_EQUATION; equations = new_el.clone());
            }
            ()
        },
        Deref @ DAE::Element::COMP { dAElist: el, .. } => {
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::COMP; dAElist = new_el.clone());
            }
            ()
        },
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            ()
        },
        Deref @ DAE::Element::ASSERT { level: e3, message: e2, condition: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            let mut new_e3: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::ASSERT; condition = new_e1.clone());
            }
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e2.clone(),&new_e2.clone())) {
                assign_variant_field!(element => DAE::Element::ASSERT; message = new_e2.clone());
            }
            (new_e3, arg) = func(e3.clone(), arg.clone())?;
            if !(referenceEq(&e3.clone(),&new_e3.clone())) {
                assign_variant_field!(element => DAE::Element::ASSERT; level = new_e3.clone());
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_ASSERT { level: e3, message: e2, condition: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_e2: Arc<DAE::Exp>;
            let mut new_e3: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_ASSERT; condition = new_e1.clone());
            }
            (new_e2, arg) = func(e2.clone(), arg.clone())?;
            if !(referenceEq(&e2.clone(),&new_e2.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_ASSERT; message = new_e2.clone());
            }
            (new_e3, arg) = func(e3.clone(), arg.clone())?;
            if !(referenceEq(&e3.clone(),&new_e3.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_ASSERT; level = new_e3.clone());
            }
            ()
        },
        Deref @ DAE::Element::TERMINATE { message: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::TERMINATE; message = new_e1.clone());
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_TERMINATE; message = new_e1.clone());
            }
            ()
        },
        Deref @ DAE::Element::NORETCALL { exp: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::NORETCALL; exp = new_e1.clone());
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { exp: e1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_NORETCALL; exp = new_e1.clone());
            }
            ()
        },
        Deref @ DAE::Element::REINIT { exp: e1, componentRef: cr1, .. } => {
            let mut new_e1: Arc<DAE::Exp>;
            let mut new_cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&e1.clone(),&new_e1.clone())) {
                assign_variant_field!(element => DAE::Element::REINIT; exp = new_e1.clone());
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg.clone())?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            new_cr1 = __pa0.clone();
            arg = __pa1.clone();
            if !(referenceEq(&cr1.clone(),&new_cr1.clone())) {
                assign_variant_field!(element => DAE::Element::REINIT; componentRef = new_cr1.clone());
            }
            ()
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut new_stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (new_stmts, arg) = traverseDAEEquationsStmts(stmts.clone(), func.clone(), arg.clone());
            if !(referenceEq(&stmts.clone(),&new_stmts.clone())) {
                assign_variant_field!(element => DAE::Element::ALGORITHM; algorithm_ = Arc::new(DAE::Algorithm { statementLst: new_stmts.clone() }));
            }
            ()
        },
        Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut new_stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (new_stmts, arg) = traverseDAEEquationsStmts(stmts.clone(), func.clone(), arg.clone());
            if !(referenceEq(&stmts.clone(),&new_stmts.clone())) {
                assign_variant_field!(element => DAE::Element::INITIALALGORITHM; algorithm_ = Arc::new(DAE::Algorithm { statementLst: new_stmts.clone() }));
            }
            ()
        },
        Deref @ DAE::Element::CONSTRAINT { constraints: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: expl }, .. } => {
            let mut new_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (new_expl, arg) = traverseDAEExpList(expl.clone(), func.clone(), arg.clone())?;
            if !(referenceEq(&expl.clone(),&new_expl.clone())) {
                assign_variant_field!(element => DAE::Element::CONSTRAINT; constraints = Arc::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: new_expl.clone() }));
            }
            ()
        },
        Deref @ DAE::Element::CLASS_ATTRIBUTES { .. } => {
            ()
        },
        Deref @ DAE::Element::IF_EQUATION { equations3: el, equations2: eqll, condition1: expl, .. } => {
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut new_eqll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut new_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (new_expl, arg) = traverseDAEExpList(expl.clone(), func.clone(), arg.clone())?;
            if !(referenceEq(&expl.clone(),&new_expl.clone())) {
                assign_variant_field!(element => DAE::Element::IF_EQUATION; condition1 = new_expl.clone());
            }
            (new_eqll, arg) = traverseDAEList(eqll.clone(), func.clone(), arg.clone())?;
            if !(referenceEq(&eqll.clone(),&new_eqll.clone())) {
                assign_variant_field!(element => DAE::Element::IF_EQUATION; equations2 = new_eqll.clone());
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::IF_EQUATION; equations3 = new_el.clone());
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { equations3: el, equations2: eqll, condition1: expl, .. } => {
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut new_eqll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut new_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (new_expl, arg) = traverseDAEExpList(expl.clone(), func.clone(), arg.clone())?;
            if !(referenceEq(&expl.clone(),&new_expl.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_IF_EQUATION; condition1 = new_expl.clone());
            }
            (new_eqll, arg) = traverseDAEList(eqll.clone(), func.clone(), arg.clone())?;
            if !(referenceEq(&eqll.clone(),&new_eqll.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_IF_EQUATION; equations2 = new_eqll.clone());
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::INITIAL_IF_EQUATION; equations3 = new_el.clone());
            }
            ()
        },
        Deref @ DAE::Element::FLAT_SM { dAElist: el, .. } => {
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::FLAT_SM; dAElist = new_el.clone());
            }
            ()
        },
        Deref @ DAE::Element::SM_COMP { dAElist: el, .. } => {
            let mut new_el: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg.clone());
            if !(referenceEq(&el.clone(),&new_el.clone())) {
                assign_variant_field!(element => DAE::Element::SM_COMP; dAElist = new_el.clone());
            }
            ()
        },
        Deref @ DAE::Element::COMMENT { .. } => {
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.traverseDAEElement not implemented correctly for element: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(list![element.clone()])?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((element, arg))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TraverseStatementsOptions {
    TRAVERSE_ALL,
    TRAVERSE_RHS_ONLY,
}
pub use self::TraverseStatementsOptions::{TRAVERSE_ALL,TRAVERSE_RHS_ONLY};

pub fn traverseAlgorithmExps<Type_a: Clone + 'static>(mut inAlgorithm: Arc<DAE::Algorithm>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outTypeA: Type_a;
    outTypeA = (::match_deref::match_deref! { match &((inAlgorithm.clone(), func.clone(), inTypeA.clone())) {
        (Deref @ DAE::Algorithm { statementLst: stmts }, _, _) => {
            let mut ext_arg_1: Type_a;
            (_, ext_arg_1) = traverseDAEEquationsStmts(stmts.clone(), func.clone(), inTypeA.clone());
            ext_arg_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTypeA)
}

pub fn traverseDAEEquationsStmts<Type_a: Clone + 'static>(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iextraArg: Type_a) -> (Arc<metamodelica::List<Arc<DAE::Statement>>>, Type_a) {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut oextraArg: Type_a;
    (outStmts, oextraArg) = traverseDAEEquationsStmtsList(inStmts.clone(), func.clone(), crate::DAEUtil::TraverseStatementsOptions::TRAVERSE_ALL, iextraArg.clone());
    (outStmts, oextraArg)
}

pub fn traverseDAEEquationsStmtsRhsOnly<Type_a: Clone + 'static>(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iextraArg: Type_a) -> (Arc<metamodelica::List<Arc<DAE::Statement>>>, Type_a) {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut oextraArg: Type_a;
    (outStmts, oextraArg) = traverseDAEEquationsStmtsList(inStmts.clone(), func.clone(), crate::DAEUtil::TraverseStatementsOptions::TRAVERSE_RHS_ONLY, iextraArg.clone());
    (outStmts, oextraArg)
}

fn traverseDAEEquationsStmtsList<Type_a: Clone + 'static>(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut opt: TraverseStatementsOptions, mut iextraArg: Type_a) -> (Arc<metamodelica::List<Arc<DAE::Statement>>>, Type_a) {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut oextraArg: Type_a;
    let mut outStmtsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Statement>>>>> = metamodelica::nil();
    let mut b: bool = false;
    (outStmtsLst, oextraArg) = List::map2Fold(inStmts.clone(), Arc::new(traverseDAEEquationsStmtsWork), func.clone(), opt.clone(), iextraArg.clone(), metamodelica::nil());
    outStmts = List::flatten(outStmtsLst.clone());
    b = List::allReferenceEq(inStmts.clone(), outStmts.clone());
    outStmts = if (b.clone()) {inStmts.clone()} else {outStmts.clone()};
    (outStmts, oextraArg)
}

fn traverseStatementsOptionsEvalLhs<Type_a: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut inA: Type_a, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut opt: TraverseStatementsOptions) -> (Arc<DAE::Exp>, Type_a) {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outA: Type_a;
    (outExp, outA) = (::match_deref::match_deref! { match &((inExp.clone(), inA.clone(), func.clone(), opt.clone())) {
        (_, _, _, TraverseStatementsOptions::TRAVERSE_ALL { .. }) => {
            (outExp, outA) = func(inExp.clone(), inA.clone()).unwrap();
            (outExp.clone(), outA.clone())
        },
        _ => (inExp.clone(), inA.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outA)
}

fn traverseDAEEquationsStmtsWork<Type_a: Clone + 'static>(mut inStmt: Arc<DAE::Statement>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut opt: TraverseStatementsOptions, mut iextraArg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut oextraArg: Type_a;
    (outStmts, oextraArg) = 'mc: {
        let __mc_input = (inStmt.clone(), func.clone(), opt.clone(), iextraArg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { source, exp: e2, exp1: e, type_: tp }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = traverseStatementsOptionsEvalLhs(e.clone(), extraArg.clone(), func.clone(), opt.clone());
                    (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&e2.clone(),&e_2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_1.clone(), exp: e_2.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, exp: e, expExpLst: expl1, type_: tp }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseStatementsOptionsEvalLhs(Arc::new(DAE::Exp::TUPLE { PR: expl1.clone() }), extraArg.clone(), func.clone(), opt.clone())) {
                        (Deref @ DAE::Exp::TUPLE { PR: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expl2 = __pa0.clone();
                    extraArg = __pa1.clone();
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&expl1.clone(),&expl2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2.clone(), exp: e_1.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: e2, lhs: e, type_: tp }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                    let _ = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
                    let () = __mc_input.clone() else { bail!("nomatch") };
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Statement>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseStatementsOptionsEvalLhs(e.clone(), extraArg.clone(), func.clone(), opt.clone())) {
                        (__pa0 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    extraArg = __pa1.clone();
                    x = if (referenceEq(&e2.clone(),&e_2.clone()) && referenceEq(&e.clone(),&e_1.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_1.clone(), exp: e_2.clone(), source: source.clone() })};
                    Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut x: Arc<DAE::Statement>;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(func(e.clone(), extraArg.clone()), '__try0)) {
                            (Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, _) => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    x = if (referenceEq(&e2.clone(),&e_2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e.clone(), exp: e_2.clone(), source: source.clone() })};
                    Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { source, else_: algElse, statementLst: stmts, exp: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut algElse1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
                    let mut b: bool = false;
                    let mut extraArg = (*extraArg).clone();
                    (algElse1, extraArg) = traverseDAEEquationsStmtsElse(algElse.clone(), func.clone(), opt.clone(), extraArg.clone())?;
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt.clone(), extraArg.clone());
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    (stmts1, b) = Algorithm::optimizeIf(e_1.clone(), stmts2.clone(), algElse1.clone(), source.clone());
                    stmts1 = if (!(b.clone()) && referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone()) && referenceEq(&algElse.clone(),&algElse1.clone())) {cons(inStmt.clone(), metamodelica::nil())} else {stmts1.clone()};
                    Ok((stmts1.clone(), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { source, statementLst: stmts, range: e, iter: id1, iterIsArray: b1, type_: tp }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt.clone(), extraArg.clone());
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: (id1.clone()).clone(), range: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_PARFOR { source, loopPrlVars, statementLst: stmts, range: e, iter: id1, iterIsArray: b1, type_: tp }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt.clone(), extraArg.clone());
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_PARFOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: (id1.clone()).clone(), range: e_1.clone(), statementLst: stmts2.clone(), loopPrlVars: loopPrlVars.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { source, statementLst: stmts, exp: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt.clone(), extraArg.clone());
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: None, statementLst: stmts, initialCall, conditions, exp: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt.clone(), extraArg.clone());
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: None, source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: Some(ew), statementLst: stmts, initialCall, conditions, exp: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut ew_1: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseDAEEquationsStmtsList(list![ew.clone()], func.clone(), opt.clone(), extraArg.clone())) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ew_1 = __pa0.clone();
                    extraArg = __pa1.clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt.clone(), extraArg.clone());
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&ew.clone(),&ew_1.clone()) && referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: Some(ew_1.clone()), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { source, level: e3, msg: e2, cond: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut e_3: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                    (e_3, extraArg) = func(e3.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&e2.clone(),&e_2.clone()) && referenceEq(&e3.clone(),&e_3.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_ASSERT { cond: e_1.clone(), msg: e_2.clone(), level: e_3.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TERMINATE { source, msg: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_TERMINATE { msg: e_1.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { source, value: e2, var: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&e2.clone(),&e_2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_REINIT { var: e_1.clone(), value: e_2.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { source, exp: e }, _, _, extraArg) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&e.clone(),&e_1.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x @ Deref @ DAE::Statement::STMT_RETURN { .. }, _, _, extraArg) => {
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x @ Deref @ DAE::Statement::STMT_BREAK { .. }, _, _, extraArg) => {
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x @ Deref @ DAE::Statement::STMT_CONTINUE { .. }, _, _, extraArg) => {
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { source, body: stmts }, _, _, extraArg) => {
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
                    let mut x: Arc<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt.clone(), extraArg.clone());
                    x = if (referenceEq(&stmts.clone(),&stmts2.clone())) {inStmt.clone()} else {Arc::new(DAE::Statement::STMT_FAILURE { body: stmts2.clone(), source: source.clone() })};
                    Ok((cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x, _, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (DAEDump::ppStatementStr(x.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.traverseDAEEquationsStmts not implemented correctly: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, oextraArg))
}

fn traverseDAEEquationsStmtsElse<Type_a: Clone + 'static>(mut inElse: Arc<DAE::Else>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut opt: TraverseStatementsOptions, mut iextraArg: Type_a) -> Result<(Arc<DAE::Else>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut oextraArg: Type_a;
    (outElse, oextraArg) = (::match_deref::match_deref! { match &((inElse.clone(), func.clone(), opt.clone(), iextraArg.clone())) {
        (Deref @ DAE::Else::NOELSE, _, _, extraArg) => {
            (Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), extraArg.clone())
        },
        (Deref @ DAE::Else::ELSEIF { exp: e, statementLst: st, else_: el }, _, _, extraArg) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut el_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            let mut b: bool = false;
            let mut extraArg = (*extraArg).clone();
            (el_1, extraArg) = traverseDAEEquationsStmtsElse(el.clone(), func.clone(), opt.clone(), extraArg.clone())?;
            (st_1, extraArg) = traverseDAEEquationsStmtsList(st.clone(), func.clone(), opt.clone(), extraArg.clone());
            (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
            outElse = Algorithm::optimizeElseIf(e_1.clone(), st_1.clone(), el_1.clone());
            b = referenceEq(&el.clone(),&el_1.clone()) && referenceEq(&st.clone(),&st_1.clone()) && referenceEq(&e.clone(),&e_1.clone());
            outElse = if (b.clone()) {inElse.clone()} else {outElse.clone()};
            (outElse.clone(), extraArg.clone())
        },
        (Deref @ DAE::Else::ELSE { statementLst: st }, _, _, extraArg) => {
            let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut extraArg = (*extraArg).clone();
            (st_1, extraArg) = traverseDAEEquationsStmtsList(st.clone(), func.clone(), opt.clone(), extraArg.clone());
            outElse = if (referenceEq(&st.clone(),&st_1.clone())) {inElse.clone()} else {Arc::new(DAE::Else::ELSE { statementLst: st_1.clone() })};
            (outElse.clone(), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outElse, oextraArg))
}

pub fn traverseDAEStmts<Type_a: Clone + 'static>(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iextraArg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut extraArg: Type_a = iextraArg.clone();
    let mut e_1: Arc<DAE::Exp>;
    let mut e_2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e3: Arc<DAE::Exp>;
    let mut e_3: Arc<DAE::Exp>;
    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ew: Arc<DAE::Statement>;
    let mut ew_1: Arc<DAE::Statement>;
    let mut b1: bool = false;
    let mut id1: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut source: Arc<DAE::ElementSource>;
    let mut algElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut loopPrlVars: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    let mut conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut initialCall: bool = false;
    for mut stmt in &*inStmts.clone() {
        let mut stmt = stmt.clone();
        outStmts = 'mc: {
        let __mc_input = stmt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { source, exp: e, exp1: e2, type_: tp } => {
                    let mut e_2: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    let mut extraArg: Type_a = extraArg.clone();
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&e2.clone(),&e_2.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_2.clone(), exp: e_1.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, exp: e, expExpLst: expl1, type_: tp } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = expl2.clone();
                    let mut extraArg: Type_a = extraArg.clone();
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    (expl2, extraArg) = traverseDAEExpListStmt(expl1.clone(), func.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&expl2.clone(),&expl1.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2.clone(), exp: e_1.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: e2, lhs: e, type_: tp } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut extraArg: Type_a = extraArg.clone();
                    (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                    match '__try0: {
                        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(func(e.clone(), stmt.clone(), extraArg.clone()), '__try0)) {
                            (__pa1 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, __pa2) => (__pa1.clone(), __pa2.clone()),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        e_1 = __pa1.clone();
                        extraArg = __pa2.clone();
                        Ok::<_, anyhow::Error>((e_1.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            e_1 = __try0_o0;
                        }
                        Err(_) => {
                            e_1 = e.clone();
                        }
                    }
                    Ok(if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&e2.clone(),&e_2.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_1.clone(), exp: e_2.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_IF { source, else_: algElse, statementLst: stmts, exp: e } => {
                    let mut algElse = (*algElse).clone();
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts2.clone();
                    let mut stmts1: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts1.clone();
                    let mut e_1: Arc<DAE::Exp>;
                    (algElse, extraArg) = traverseDAEStmtsElse(algElse.clone(), func.clone(), stmt.clone(), extraArg.clone())?;
                    (stmts2, extraArg) = traverseDAEStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    (stmts1, _) = Algorithm::optimizeIf(e_1.clone(), stmts2.clone(), algElse.clone(), source.clone());
                    Ok(List::append_reverse(stmts1.clone(), outStmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FOR { source, statementLst: stmts, range: e, iter: id1, iterIsArray: b1, type_: tp } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts2.clone();
                    (stmts2, extraArg) = traverseDAEStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: (id1.clone()).clone(), range: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_PARFOR { source, loopPrlVars, statementLst: stmts, range: e, iter: id1, iterIsArray: b1, type_: tp } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts2.clone();
                    let mut e_1: Arc<DAE::Exp>;
                    (stmts2, extraArg) = traverseDAEStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_PARFOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: (id1.clone()).clone(), range: e_1.clone(), statementLst: stmts2.clone(), loopPrlVars: loopPrlVars.clone(), source: source.clone() }), outStmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHILE { source, statementLst: stmts, exp: e } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts2.clone();
                    (stmts2, extraArg) = traverseDAEStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&stmts.clone(),&stmts2.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: None, statementLst: stmts, initialCall, conditions, exp: e } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts2.clone();
                    let mut extraArg: Type_a = extraArg.clone();
                    (stmts2, extraArg) = traverseDAEStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: None, source: source.clone() }), outStmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: Some(ew), statementLst: stmts, initialCall, conditions, exp: e } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts2.clone();
                    let mut e_1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(traverseDAEStmts(list![ew.clone()], func.clone(), extraArg.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    extraArg = __pa0.clone();
                    (stmts2, extraArg) = traverseDAEStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(cons(Arc::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: Some(ew.clone()), source: source.clone() }), outStmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSERT { source, level: e3, msg: e2, cond: e } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut e_3: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                    (e_3, extraArg) = func(e3.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&e2.clone(),&e_2.clone()) && referenceEq(&e3.clone(),&e_3.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_ASSERT { cond: e_1.clone(), msg: e_2.clone(), level: e_3.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TERMINATE { source, msg: e } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut extraArg: Type_a = extraArg.clone();
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_TERMINATE { msg: e_1.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_REINIT { source, value: e2, var: e } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut e_2: Arc<DAE::Exp>;
                    let mut e_1: Arc<DAE::Exp>;
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone()) && referenceEq(&e2.clone(),&e_2.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_REINIT { var: e_1.clone(), value: e_2.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_NORETCALL { source, exp: e } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut extraArg: Type_a = extraArg.clone();
                    (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&e.clone(),&e_1.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_RETURN { .. } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    (_, extraArg) = func(Arc::new(DAE::Exp::ICONST { integer: -1 }), stmt.clone(), extraArg.clone())?;
                    Ok(cons(stmt.clone(), outStmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_BREAK { .. } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    (_, extraArg) = func(Arc::new(DAE::Exp::ICONST { integer: -1 }), stmt.clone(), extraArg.clone())?;
                    Ok(cons(stmt.clone(), outStmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_CONTINUE { .. } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    (_, extraArg) = func(Arc::new(DAE::Exp::ICONST { integer: -1 }), stmt.clone(), extraArg.clone())?;
                    Ok(cons(stmt.clone(), outStmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FAILURE { source, body: stmts } => {
                    let mut extraArg: Type_a = extraArg.clone();
                    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = stmts2.clone();
                    (stmts2, extraArg) = traverseDAEStmts(stmts.clone(), func.clone(), extraArg.clone())?;
                    Ok(if (referenceEq(&stmts.clone(),&stmts2.clone())) {cons(stmt.clone(), outStmts.clone())} else {cons(Arc::new(DAE::Statement::STMT_FAILURE { body: stmts2.clone(), source: source.clone() }), outStmts.clone())})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (DAEDump::ppStatementStr(stmt.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.traverseDAEStmts not implemented correctly: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    }
    outStmts = outStmts.clone().reverse();
    Ok((outStmts, extraArg))
}

fn traverseDAEStmtsElse<Type_a: Clone + 'static>(mut inElse: Arc<DAE::Else>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut istmt: Arc<DAE::Statement>, mut iextraArg: Type_a) -> Result<(Arc<DAE::Else>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut oextraArg: Type_a;
    (outElse, oextraArg) = (::match_deref::match_deref! { match &((inElse.clone(), func.clone(), istmt.clone(), iextraArg.clone())) {
        (Deref @ DAE::Else::NOELSE, _, _, extraArg) => {
            (Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), extraArg.clone())
        },
        (Deref @ DAE::Else::ELSEIF { exp: e, statementLst: st, else_: el }, _, _, extraArg) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut el_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            let mut extraArg = (*extraArg).clone();
            (el_1, extraArg) = traverseDAEStmtsElse(el.clone(), func.clone(), istmt.clone(), extraArg.clone())?;
            (st_1, extraArg) = traverseDAEStmts(st.clone(), func.clone(), extraArg.clone())?;
            (e_1, extraArg) = func(e.clone(), istmt.clone(), extraArg.clone())?;
            (Algorithm::optimizeElseIf(e_1.clone(), st_1.clone(), el_1.clone()), extraArg.clone())
        },
        (Deref @ DAE::Else::ELSE { statementLst: st }, _, _, extraArg) => {
            let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut extraArg = (*extraArg).clone();
            (st_1, extraArg) = traverseDAEStmts(st.clone(), func.clone(), extraArg.clone())?;
            (Arc::new(DAE::Else::ELSE { statementLst: st_1.clone() }), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outElse, oextraArg))
}

fn traverseDAEExpListStmt<Type_a: Clone + 'static>(mut iexps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut istmt: Arc<DAE::Statement>, mut iextraArg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Arc<DAE::Statement>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut oexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut oextraArg: Type_a;
    (oexps, oextraArg) = (::match_deref::match_deref! { match &((iexps.clone(), func.clone(), istmt.clone(), iextraArg.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, extraArg) => {
            (metamodelica::nil(), extraArg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: exps }, _, _, extraArg) => {
            let mut e = (*e).clone();
            let mut extraArg = (*extraArg).clone();
            (e, extraArg) = func(e.clone(), istmt.clone(), extraArg.clone())?;
            (oexps, extraArg) = traverseDAEExpListStmt(exps.clone(), func.clone(), istmt.clone(), extraArg.clone())?;
            (cons(e.clone(), oexps.clone()), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oexps, oextraArg))
}

fn traverseDAEVarAttr<Type_a: Clone + 'static>(mut attr: Option<Arc<DAE::VariableAttributes>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iextraArg: Type_a) -> Result<(Option<Arc<DAE::VariableAttributes>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone> = fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)>;

    let mut traversedDaeList: Option<Arc<DAE::VariableAttributes>> = None;
    let mut oextraArg: Type_a;
    (traversedDaeList, oextraArg) = (::match_deref::match_deref! { match &((attr.clone(), func.clone(), iextraArg.clone())) {
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity, unit, displayUnit, min, max, start, fixed, nominal, stateSelectOption: stateSelect, uncertainOption: uncertainty, distributionOption: distribution, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, extraArg) => {
            let mut quantity = (*quantity).clone();
            let mut unit = (*unit).clone();
            let mut displayUnit = (*displayUnit).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut nominal = (*nominal).clone();
            let mut extraArg = (*extraArg).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func.clone(), extraArg.clone())?;
            (unit, extraArg) = traverseDAEOptExp(unit.clone(), func.clone(), extraArg.clone())?;
            (displayUnit, extraArg) = traverseDAEOptExp(displayUnit.clone(), func.clone(), extraArg.clone())?;
            (min, extraArg) = traverseDAEOptExp(min.clone(), func.clone(), extraArg.clone())?;
            (max, extraArg) = traverseDAEOptExp(max.clone(), func.clone(), extraArg.clone())?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func.clone(), extraArg.clone())?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func.clone(), extraArg.clone())?;
            (nominal, extraArg) = traverseDAEOptExp(nominal.clone(), func.clone(), extraArg.clone())?;
            (Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelectOption: stateSelect.clone(), uncertainOption: uncertainty.clone(), distributionOption: distribution.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg.clone())
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity, min, max, start, fixed, uncertainOption: uncertainty, distributionOption: distribution, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, extraArg) => {
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut extraArg = (*extraArg).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func.clone(), extraArg.clone())?;
            (min, extraArg) = traverseDAEOptExp(min.clone(), func.clone(), extraArg.clone())?;
            (max, extraArg) = traverseDAEOptExp(max.clone(), func.clone(), extraArg.clone())?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func.clone(), extraArg.clone())?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func.clone(), extraArg.clone())?;
            (Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), uncertainOption: uncertainty.clone(), distributionOption: distribution.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg.clone())
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity, start, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, extraArg) => {
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut extraArg = (*extraArg).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func.clone(), extraArg.clone())?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func.clone(), extraArg.clone())?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func.clone(), extraArg.clone())?;
            (Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg.clone())
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: _, finalPrefix: _ }), _, extraArg) => {
            (attr.clone(), extraArg.clone())
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity, start, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, extraArg) => {
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut extraArg = (*extraArg).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func.clone(), extraArg.clone())?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func.clone(), extraArg.clone())?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func.clone(), extraArg.clone())?;
            (Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg.clone())
        },
        (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity, min, max, start, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }), _, extraArg) => {
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut extraArg = (*extraArg).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func.clone(), extraArg.clone())?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func.clone(), extraArg.clone())?;
            (Some(Arc::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg.clone())
        },
        (None, _, extraArg) => {
            (None, extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((traversedDaeList, oextraArg))
}

pub fn addComponentTypeOpt(mut inDae: DAE::DAElist, mut inPath: Option<Arc<Absyn::Path>>) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inDae.clone(), inPath.clone())) {
        (dae, Some(p)) => {
            let mut dae = (*dae).clone();
            dae = addComponentType(dae.clone(), p.clone())?;
            dae.clone()
        },
        (dae, None) => {
            dae.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

pub fn addComponentType(mut dae: DAE::DAElist, mut newtype: Arc<Absyn::Path>) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist = dae;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(dae);
    }
    dae = (match dae.clone() {
        DAE::DAElist { elementLst: ref elts } => {
            let mut elts = elts.clone();
            elts = List::map1(elts.clone(), Arc::new(addComponentType2), newtype.clone());
            DAE::DAElist { elementLst: elts.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(dae)
}

fn addComponentType2(mut elt: Arc<DAE::Element>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<DAE::Element>> {
    let mut elt: Arc<DAE::Element> = elt;
    elt = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            assign_variant_field!(elt => DAE::Element::VAR; source = ElementSource::addElementSourceType(var_field!((*elt).source, DAE::Element::VAR).clone(), inPath.clone())?);
            elt.clone()
        },
        _ => {
            elt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elt)
}

pub fn isExtFunction(mut elt: DAE::Function) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(elt.clone()) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn functionName(mut elt: DAE::Function) -> Result<Arc<Absyn::Path>> {
    let mut name: Arc<Absyn::Path>;
    name = (match elt.clone() {
        DAE::Function::FUNCTION { path: ref name, .. } => name.clone(),
        DAE::Function::RECORD_CONSTRUCTOR { path: ref name, .. } => name.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(name)
}

pub fn convertInlineTypeToBool(mut it: DAE::InlineType) -> bool {
    let mut b: bool = false;
    b = (match it.clone() {
        DAE::InlineType::NO_INLINE => false,
        _ => true,
    });
    b
}

pub fn inlineTypeEqual(mut it1: DAE::InlineType, mut it2: DAE::InlineType) -> bool {
    let mut b: bool = false;
    b = (match (it1.clone(), it2.clone()) {
        (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
        (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true,
        (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
        (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
        (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
        (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => true,
        _ => false,
    });
    b
}

pub fn daeElements(mut dae: DAE::DAElist) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    elts = (match dae.clone() {
        DAE::DAElist { elementLst: ref elts } => elts.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(elts)
}

pub fn joinDaes(mut dae1: DAE::DAElist, mut dae2: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (match (dae1.clone(), dae2.clone()) {
        (DAE::DAElist { elementLst: ref elts1 }, DAE::DAElist { elementLst: ref elts2 }) => {
            let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            elts = listAppend(elts1.clone(), elts2.clone());
            DAE::DAElist { elementLst: elts.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outDae)
}

pub fn joinDaeLst(mut idaeLst: Arc<metamodelica::List<DAE::DAElist>>) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = idaeLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: dae, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: dae, tail: daeLst } => {
                    let mut dae1: DAE::DAElist;
                    let mut dae = (*dae).clone();
                    dae1 = joinDaeLst(daeLst.clone())?;
                    dae = joinDaes(dae.clone(), dae1.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDae)
}

pub fn splitElements(mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>>, Arc<metamodelica::List<Arc<SCode::Comment>>>)> {
    let mut variables: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut initialEquations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut initialAlgorithms: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut algorithms: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut classAttributes: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut constraints: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut externalObjects: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut stateMachineComps: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>> = metamodelica::nil();
    let mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
    let mut split_comp: Arc<DAEDumpTypes::compWithSplitElements>;
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        let _ = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::VAR { .. } => {
            variables = cons(e.clone(), variables.clone());
            ()
        },
        Deref @ DAE::Element::INITIALEQUATION { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIALDEFINE { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            initialEquations = cons(e.clone(), initialEquations.clone());
            ()
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            initialAlgorithms = cons(e.clone(), initialAlgorithms.clone());
            ()
        },
        Deref @ DAE::Element::EQUATION { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::EQUEQUATION { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::DEFINE { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::IF_EQUATION { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::FOR_EQUATION { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::WHEN_EQUATION { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::REINIT { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            equations = cons(e.clone(), equations.clone());
            ()
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            algorithms = cons(e.clone(), algorithms.clone());
            ()
        },
        Deref @ DAE::Element::CONSTRAINT { .. } => {
            constraints = cons(e.clone(), constraints.clone());
            ()
        },
        Deref @ DAE::Element::CLASS_ATTRIBUTES { .. } => {
            classAttributes = cons(e.clone(), classAttributes.clone());
            ()
        },
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            externalObjects = cons(e.clone(), externalObjects.clone());
            ()
        },
        Deref @ DAE::Element::COMP { .. } => {
            variables = listAppend(var_field!((*e).dAElist, DAE::Element::COMP).clone(), variables.clone());
            ()
        },
        Deref @ DAE::Element::FLAT_SM { .. } => {
            split_comp = splitComponent(Arc::new(DAE::Element::COMP { ident: (var_field!((*e).ident, DAE::Element::FLAT_SM).clone()).clone(), dAElist: var_field!((*e).dAElist, DAE::Element::FLAT_SM).clone(), source: DAE::emptyElementSource.clone(), comment: Some(Arc::new(SCode::Comment { annotation_: None, comment: Some((literal!("stateMachine")).clone()) })) }))?;
            stateMachineComps = cons(split_comp.clone(), stateMachineComps.clone());
            ()
        },
        Deref @ DAE::Element::SM_COMP { .. } => {
            split_comp = splitComponent(Arc::new(DAE::Element::COMP { ident: (ComponentReference::crefStr(var_field!((*e).componentRef, DAE::Element::SM_COMP).clone())?).clone(), dAElist: var_field!((*e).dAElist, DAE::Element::SM_COMP).clone(), source: DAE::emptyElementSource.clone(), comment: Some(Arc::new(SCode::Comment { annotation_: None, comment: Some((literal!("state")).clone()) })) }))?;
            stateMachineComps = cons(split_comp.clone(), stateMachineComps.clone());
            ()
        },
        Deref @ DAE::Element::COMMENT { .. } => {
            comments = cons(var_field!((*e).cmt, DAE::Element::COMMENT).clone(), comments.clone());
            ()
        },
        _ => {
            Error::addInternalError((literal!("DAEUtil.splitElements got unknown element.")).clone(), Absyn::dummyInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    variables = variables.clone().reverse();
    initialEquations = initialEquations.clone().reverse();
    initialAlgorithms = initialAlgorithms.clone().reverse();
    equations = equations.clone().reverse();
    algorithms = algorithms.clone().reverse();
    classAttributes = classAttributes.clone().reverse();
    constraints = constraints.clone().reverse();
    externalObjects = externalObjects.clone().reverse();
    stateMachineComps = stateMachineComps.clone().reverse();
    Ok((variables, initialEquations, initialAlgorithms, equations, algorithms, classAttributes, constraints, externalObjects, stateMachineComps, comments))
}

pub fn splitComponent(mut component: Arc<DAE::Element>) -> Result<Arc<DAEDumpTypes::compWithSplitElements>> {
    let mut splitComponent: Arc<DAEDumpTypes::compWithSplitElements>;
    let mut v: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut ie: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut ia: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut e: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut a: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut co: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut o: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut ca: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut sm: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>> = metamodelica::nil();
    let mut split_el: Arc<DAEDumpTypes::splitElements>;
    splitComponent = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ DAE::Element::COMP { .. } => {
            (v, ie, ia, e, a, co, o, ca, sm, _) = splitElements(var_field!((*component).dAElist, DAE::Element::COMP).clone())?;
            split_el = Arc::new(DAEDumpTypes::splitElements { v: v.clone(), ie: ie.clone(), ia: ia.clone(), e: e.clone(), a: a.clone(), co: co.clone(), o: o.clone(), ca: ca.clone(), sm: sm.clone() });
            Arc::new(DAEDumpTypes::compWithSplitElements { name: (var_field!((*component).ident, DAE::Element::COMP).clone()).clone(), spltElems: split_el.clone(), comment: var_field!((*component).comment, DAE::Element::COMP).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(splitComponent)
}

fn isIfEquation(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::IF_EQUATION { .. } => (),
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn collectLocalDecls(mut e: Arc<DAE::Exp>, mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outElements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (outExp, outElements) = (::match_deref::match_deref! { match &((e.clone(), inElements.clone())) {
        (Deref @ DAE::Exp::MATCHEXPRESSION { localDecls: ld1, .. }, ld2) => {
            let mut ld: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            ld = listAppend(ld1.clone(), ld2.clone());
            (e.clone(), ld.clone())
        },
        _ => {
            (e.clone(), inElements.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outElements)
}

pub fn getUniontypePaths(mut funcs: Arc<metamodelica::List<DAE::Function>>, mut els: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut paths1: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut paths2: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outPaths = 'mc: {
        let __mc_input = (funcs.clone(), els.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let false = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut paths2: Arc<metamodelica::List<Arc<Absyn::Path>>> = paths2.clone();
                    let mut paths1: Arc<metamodelica::List<Arc<Absyn::Path>>> = paths1.clone();
                    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = outPaths.clone();
                    paths1 = getUniontypePathsFunctions(funcs.clone())?;
                    paths2 = getUniontypePathsElements(els.clone(), metamodelica::nil())?;
                    outPaths = listAppend(paths1.clone(), paths2.clone());
                    Ok(outPaths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPaths)
}

fn getUniontypePathsFunctions(mut elements: Arc<metamodelica::List<DAE::Function>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outPaths = (::match_deref::match_deref! { match &(elements.clone()) {
        _ => {
            let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut els1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut els2: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let (_, (_, __pa0)) = traverseDAEFunctions(elements.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (fnptr!(collectLocalDecls, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>), metamodelica::nil()));
            els1 = __pa0.clone();
            els2 = getFunctionsElements(elements.clone());
            els = listAppend(els1.clone(), els2.clone());
            outPaths = getUniontypePathsElements(els.clone(), metamodelica::nil())?;
            outPaths.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPaths)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getUniontypePathsElements(mut elements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut acc: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outPaths = (::match_deref::match_deref! { match &((elements.clone(), acc.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            List::applyAndFold(acc.clone(), Arc::new(listAppend.clone()), Arc::new(Types::getUniontypePaths), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { ty: ft, .. }, tail: rest }, _) => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            tys = Types::getAllInnerTypesOfType(ft.clone(), Arc::new(fnptr!(Types::uniontypeFilter, Arc<DAE::Type>)));
            getUniontypePathsElements(rest.clone(), listAppend(tys.clone(), acc.clone()))?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            getUniontypePathsElements(rest.clone(), acc.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPaths)
}

fn getDAEDeclsFromValueblocks(mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut outEls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    for mut ex in &*exps.clone() {
        let mut ex = ex.clone();
        let _ = (::match_deref::match_deref! { match &(ex.clone()) {
        Deref @ DAE::Exp::MATCHEXPRESSION { localDecls: els1, .. } => {
            outEls = List::append_reverse(els1.clone(), outEls.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outEls = metamodelica::Dangerous::listReverseInPlace(outEls.clone());
    outEls
}

// protected function transformDerInline "This is not used.
//   Simple euler inline of the equation system; only does explicit euler, and only der(cref)"
//   input DAE.DAElist dae;
//   output DAE.DAElist d;
// algorithm
//   d := matchcontinue (dae)
//     local
//       HashTable.HashTable ht;
//     case _
//       equation
//         false = Flags.isSet(Flags.FRONTEND_INLINE_EULER);
//       then dae;
//     case _
//       equation
//         ht = HashTable.emptyHashTable();
//         (d,_,ht) = traverseDAE(dae,AvlTreePathFunction.Tree.EMPTY(),simpleInlineDerEuler,ht);
//       then d;
//   end matchcontinue;
// end transformDerInline;
//
// protected function simpleInlineDerEuler "This is not used.
//   Helper function of transformDerInline."
//   input tuple<DAE.Exp,HashTable.HashTable> itpl;
//   output tuple<DAE.Exp,HashTable.HashTable> otpl;
// algorithm
//   otpl := matchcontinue (itpl)
//     local
//       DAE.ComponentRef cr,cref_1,cref_2;
//       HashTable.HashTable crs0,crs1;
//       DAE.Exp exp,e1,e2;
//
//     case ((DAE.CALL(path=Absyn.IDENT("der"),expLst={exp as DAE.CREF(componentRef = cr, ty = DAE.T_REAL(varLst = _))}),crs0))
//       equation
//         cref_1 = ComponentReferenceBasics.makeCrefQual("$old",DAE.T_REAL_DEFAULT,{},cr);
//         cref_2 = ComponentReferenceBasics.makeCrefIdent("$current_step_size",DAE.T_REAL_DEFAULT,{});
//         e1 = Expression.makeCrefExp(cref_1,DAE.T_REAL_DEFAULT);
//         e2 = Expression.makeCrefExp(cref_2,DAE.T_REAL_DEFAULT);
//         exp = DAE.BINARY(
//                 DAE.BINARY(exp, DAE.SUB(DAE.T_REAL_DEFAULT), e1),
//                 DAE.DIV(DAE.T_REAL_DEFAULT),
//                 e2);
//         crs1 = BaseHashTable.add((cr,0),crs0);
//       then
//         ((exp,crs1));
//
//     case ((exp,crs0)) then ((exp,crs0));
//
//   end matchcontinue;
// end simpleInlineDerEuler;
pub fn transformationsBeforeBackend(mut cache: FCore::Cache, mut env: FCore::Graph, mut inDAElist: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDAElist: DAE::DAElist;
    let mut dAElist: DAE::DAElist;
    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut ht: Arc<AvlSetCR::Tree> = Arc::new(AvlSetCR::Tree::EMPTY);
    dAElist = StateMachineFlatten::stateMachineToDataFlow(cache.clone(), env.clone(), inDAElist.clone())?;
    if Flags::isSet(Flags::SCODE_INST.clone())? {
        let DAE::DAE { elementLst: __pa0 } = (dAElist.clone()) else { bail!("pattern mismatch") };
        elts = __pa0.clone();
        outDAElist = DAE::DAElist { elementLst: elts.clone() };
    } else {
        let DAE::DAE { elementLst: __pa1 } = (dAElist.clone()) else { bail!("pattern mismatch") };
        elts = __pa1.clone();
        ht = FCore::getEvaluatedParams(cache.clone())?;
        elts = List::map1(elts.clone(), Arc::new(makeEvaluatedParamFinal), ht.clone());
        if Flags::isSet(Flags::PRINT_STRUCTURAL.clone())? {
            transformationsBeforeBackendNotification(ht.clone())?;
        }
        outDAElist = DAE::DAElist { elementLst: elts.clone() };
    }
    Ok(outDAElist)
}

fn transformationsBeforeBackendNotification(mut ht: Arc<AvlSetCR::Tree>) -> Result<()> {
    let mut crs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut r#str: ArcStr = arcstr::literal!("");
    crs = AvlSetCR::listKeys(ht.clone(), metamodelica::nil());
    if !(crs.clone().is_empty()) {
        strs = List::map(crs.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr));
        r#str = stringDelimitList(strs.clone(), (literal!(", ")).clone());
        Error::addMessage(Error::NOTIFY_FRONTEND_STRUCTURAL_PARAMETERS.clone(), list![(r#str.clone()).clone()])?;
    }
    Ok(())
}

fn makeEvaluatedParamFinal(mut inElement: Arc<DAE::Element>, mut ht: Arc<AvlSetCR::Tree>) -> Result<Arc<DAE::Element>> {
    let mut outElement: Arc<DAE::Element>;
    outElement = (::match_deref::match_deref! { match &((inElement.clone(), ht.clone())) {
        (Deref @ DAE::Element::VAR { variableAttributesOption: varOpt, kind: DAE::VarKind::PARAM, componentRef: cr, .. }, _) => {
            let mut elt: Arc<DAE::Element>;
            elt = if (AvlSetCR::hasKey(ht.clone(), cr.clone())?) {setVariableAttributes(inElement.clone(), setFinalAttr(varOpt.clone(), true)?)?} else {inElement.clone()};
            elt.clone()
        },
        (Deref @ DAE::Element::COMP { ident: id, dAElist: elts, source, comment: cmt }, _) => {
            let mut elts = (*elts).clone();
            elts = List::map1(elts.clone(), Arc::new(makeEvaluatedParamFinal), ht.clone());
            Arc::new(DAE::Element::COMP { ident: (id.clone()).clone(), dAElist: elts.clone(), source: source.clone(), comment: cmt.clone() })
        },
        _ => {
            inElement.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

pub fn setBindingSource(mut inBinding: Arc<DAE::Binding>, mut bindingSource: DAE::BindingSource) -> Result<Arc<DAE::Binding>> {
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    outBinding = (::match_deref::match_deref! { match &((inBinding.clone(), bindingSource.clone())) {
        (Deref @ DAE::Binding::UNBOUND, _) => {
            inBinding.clone()
        },
        (Deref @ DAE::Binding::EQBOUND { exp, evaluatedExp, constant_: cnst, source: _ }, _) => {
            Arc::new(DAE::Binding::EQBOUND { exp: exp.clone(), evaluatedExp: evaluatedExp.clone(), constant_: cnst.clone(), source: bindingSource.clone() })
        },
        (Deref @ DAE::Binding::VALBOUND { valBound, source: _ }, _) => {
            Arc::new(DAE::Binding::VALBOUND { valBound: valBound.clone(), source: bindingSource.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinding)
}

pub fn printBindingExpStr(mut binding: Arc<DAE::Binding>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ DAE::Binding::UNBOUND => {
            literal!("")
        },
        Deref @ DAE::Binding::EQBOUND { exp: e, .. } => {
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str.clone()
        },
        Deref @ DAE::Binding::VALBOUND { valBound: v, .. } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ValuesDump::valString(v.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn collectValueblockFunctionRefVars(mut exp: Arc<DAE::Exp>, mut acc: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<Absyn::Path>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outAcc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    (outExp, outAcc) = (::match_deref::match_deref! { match &((exp.clone(), acc.clone())) {
        (Deref @ DAE::Exp::MATCHEXPRESSION { localDecls: decls, .. }, _) => {
            outAcc = List::fold(decls.clone(), Arc::new(fnptr!(collectFunctionRefVarPaths, Arc<DAE::Element>, Arc<metamodelica::List<Arc<Absyn::Path>>>)), acc.clone());
            (exp.clone(), outAcc.clone())
        },
        _ => {
            (exp.clone(), acc.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outAcc)
}

pub fn collectFunctionRefVarPaths(mut inElem: Arc<DAE::Element>, mut acc: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut outAcc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outAcc = (::match_deref::match_deref! { match &(inElem.clone()) {
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_FUNCTION { path, .. }, .. } => {
            cons(path.clone(), acc.clone())
        },
        _ => {
            acc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAcc
}

pub fn addFunctionDefinition(mut ifunc: DAE::Function, mut iFuncDef: DAE::FunctionDefinition) -> DAE::Function {
    let mut func: DAE::Function = ifunc.clone();
    let _ = (match func.clone() {
        DAE::Function::FUNCTION { .. } => {
            let __owned_variant_functions_0 = List::appendElt(iFuncDef.clone(), var_field!(func.functions, DAE::Function::FUNCTION).clone());
            if let DAE::Function::FUNCTION { functions, .. } = &mut func {
                *functions = __owned_variant_functions_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            ()
        },
        _ => (),
    });
    func
}

pub fn setAttrVariability(mut attr: Arc<DAE::Attributes>, mut var: SCode::Variability) -> Arc<DAE::Attributes> {
    let mut attr: Arc<DAE::Attributes> = attr;
    assign_field!(attr.variability = var.clone());
    attr
}

pub fn getAttrVariability(mut attr: Arc<DAE::Attributes>) -> SCode::Variability {
    let mut var: SCode::Variability = attr.variability.clone();
    var
}

pub fn setAttrDirection(mut attr: Arc<DAE::Attributes>, mut dir: Absyn::Direction) -> Arc<DAE::Attributes> {
    let mut attr: Arc<DAE::Attributes> = attr;
    assign_field!(attr.direction = dir.clone());
    attr
}

pub fn getAttrDirection(mut attr: Arc<DAE::Attributes>) -> Absyn::Direction {
    let mut dir: Absyn::Direction = attr.direction.clone();
    dir
}

pub fn setAttrInnerOuter(mut attr: Arc<DAE::Attributes>, mut io: Absyn::InnerOuter) -> Arc<DAE::Attributes> {
    let mut attr: Arc<DAE::Attributes> = attr;
    assign_field!(attr.innerOuter = io.clone());
    attr
}

pub fn getAttrInnerOuter(mut attr: Arc<DAE::Attributes>) -> Absyn::InnerOuter {
    let mut io: Absyn::InnerOuter = attr.innerOuter.clone();
    io
}

pub fn translateSCodeAttrToDAEAttr(mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>) -> Result<Arc<DAE::Attributes>> {
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut ct: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    let mut prl: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    let mut var: SCode::Variability = SCode::Variability::CONST;
    let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
    let SCode::ATTR { direction: __pa0, variability: __pa1, parallelism: __pa2, connectorType: __pa3, .. } = (inAttributes.clone()) else { bail!("pattern mismatch") };
    dir = __pa0.clone();
    var = __pa1.clone();
    prl = __pa2.clone();
    ct = __pa3.clone();
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::Prefixes { visibility: __pa4, innerOuter: __pa5, .. } => (__pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vis = __pa4.clone();
    io = __pa5.clone();
    outAttributes = Arc::new(DAE::Attributes { connectorType: toConnectorTypeNoState(ct.clone(), None), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: vis.clone() });
    Ok(outAttributes)
}

pub fn varName(mut var: Arc<DAE::Element>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn typeVarIdent(mut var: Arc<DAE::Var>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn typeVarIdentEqual(mut var: Arc<DAE::Var>, mut name: ArcStr) -> Result<bool> {
    let mut b: bool = false;
    let mut name2: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name2 = __pa0.clone();
    b = stringEq((name.clone()).clone(), (name2.clone()).clone());
    Ok(b)
}

pub fn varType(mut var: Arc<DAE::Var>) -> Result<Arc<DAE::Type>> {
    let mut type_: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    type_ = __pa0.clone();
    Ok(type_)
}

pub fn bindingExp(mut bind: Arc<DAE::Binding>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut exp: Option<Arc<DAE::Exp>> = None;
    exp = (::match_deref::match_deref! { match &(bind.clone()) {
        Deref @ DAE::Binding::UNBOUND => {
            None
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(v), .. } => {
            let mut e: Arc<DAE::Exp>;
            e = ValuesUtil::valueExp(v.clone(), None)?;
            Some(e.clone())
        },
        Deref @ DAE::Binding::EQBOUND { exp: e, .. } => {
            Some(e.clone())
        },
        Deref @ DAE::Binding::VALBOUND { valBound: v, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = ValuesUtil::valueExp(v.clone(), None)?;
            Some(e.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn isBound(mut inBinding: Arc<DAE::Binding>) -> bool {
    let mut outIsBound: bool = false;
    outIsBound = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::UNBOUND => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsBound
}

pub fn isCompleteFunction(mut f: DAE::Function) -> Result<bool> {
    let mut isComplete: bool = false;
    isComplete = (match f.clone() {
        DAE::Function::RECORD_CONSTRUCTOR { .. } => {
            true
        },
        DAE::Function::FUNCTION { functions: mut functions, .. } => {
            isCompleteFunctionBody(functions.clone())?
        },
        _ => {
            false
        },
    });
    Ok(isComplete)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isCompleteFunctionBody(mut functions: Arc<metamodelica::List<DAE::FunctionDefinition>>) -> Result<bool> {
    let mut isComplete: bool = false;
    isComplete = 'mc: {
        let __mc_input = functions.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: _ } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: els }, tail: _ } => {
                    let mut a: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    (_, _, _, _, a, _, _, _, _, _) = splitElements(els.clone())?;
                    let false = (a.clone().is_empty()) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. }, tail: rest } => {
                    Ok(isCompleteFunctionBody(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isComplete)
}

pub fn isNotCompleteFunction(mut f: DAE::Function) -> Result<bool> {
    let mut isNotComplete: bool = false;
    isNotComplete = !(isCompleteFunction(f.clone())?);
    Ok(isNotComplete)
}

pub fn setAttributeDirection(mut inDirection: Absyn::Direction, mut inAttributes: Arc<DAE::Attributes>) -> Result<Arc<DAE::Attributes>> {
    let mut outAttributes: Arc<DAE::Attributes>;
    let mut ct: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    let mut p: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    let mut var: SCode::Variability = SCode::Variability::CONST;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inAttributes.clone()) {
        Deref @ DAE::Attributes { connectorType: __pa0, parallelism: __pa1, variability: __pa2, direction: _, innerOuter: __pa3, visibility: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ct = __pa0.clone();
    p = __pa1.clone();
    var = __pa2.clone();
    io = __pa3.clone();
    vis = __pa4.clone();
    outAttributes = Arc::new(DAE::Attributes { connectorType: ct.clone(), parallelism: p.clone(), variability: var.clone(), direction: inDirection.clone(), innerOuter: io.clone(), visibility: vis.clone() });
    Ok(outAttributes)
}

pub fn varKindEqual(mut inVariability1: DAE::VarKind, mut inVariability2: DAE::VarKind) -> Result<bool> {
    let mut outIsEqual: bool = false;
    outIsEqual = (match (inVariability1.clone(), inVariability2.clone()) {
        (DAE::VarKind::VARIABLE, DAE::VarKind::VARIABLE) => true,
        (DAE::VarKind::DISCRETE, DAE::VarKind::DISCRETE) => true,
        (DAE::VarKind::CONST, DAE::VarKind::CONST) => true,
        (DAE::VarKind::PARAM, DAE::VarKind::PARAM) => true,
        _ => bail!("match: no arm matched"),
    });
    Ok(outIsEqual)
}

pub fn varDirectionEqual(mut inDirection1: DAE::VarDirection, mut inDirection2: DAE::VarDirection) -> bool {
    let mut outIsEqual: bool = false;
    outIsEqual = (match (inDirection1.clone(), inDirection2.clone()) {
        (DAE::VarDirection::BIDIR, DAE::VarDirection::BIDIR) => true,
        (DAE::VarDirection::INPUT, DAE::VarDirection::INPUT) => true,
        (DAE::VarDirection::OUTPUT, DAE::VarDirection::OUTPUT) => true,
        _ => false,
    });
    outIsEqual
}

pub fn isComplexVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut outIsComplex: bool = false;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    outIsComplex = Types::isComplexType(ty.clone());
    Ok(outIsComplex)
}

pub fn getElements(mut inDAE: DAE::DAElist) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (inDAE.clone()) else { bail!("pattern mismatch") };
    outElements = __pa0.clone();
    Ok(outElements)
}

pub fn mkEmptyVar(mut name: ArcStr) -> Arc<DAE::Var> {
    let mut outVar: Arc<DAE::Var>;
    outVar = Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: DAE::dummyAttrVar.clone(), ty: DAE::T_UNKNOWN_DEFAULT.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None });
    outVar
}

pub fn sortDAEInModelicaCodeOrder(mut inShouldSort: bool, mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inDae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inShouldSort.clone(), inElements.clone(), inDae.clone())) {
        (false, _, _) => {
            inDae.clone()
        },
        (true, Deref @ metamodelica::List::Nil, _) => {
            inDae.clone()
        },
        (true, _, DAE::DAElist { elementLst: els }) => {
            let mut els = (*els).clone();
            els = sortDAEElementsInModelicaCodeOrder(inElements.clone(), els.clone())?;
            DAE::DAElist { elementLst: els.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDae)
}

fn sortDAEElementsInModelicaCodeOrder(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inDaeEls: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outDaeEls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<DAE::Element>>> = inDaeEls.clone();
    for mut e in &*inElements.clone() {
        let mut e = e.clone();
        let _ = (::match_deref::match_deref! { match &(e.clone()) {
        (Deref @ SCode::Element::COMPONENT { name, .. }, _) => {
            let mut named: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            (named, rest) = splitVariableNamed(rest.clone(), (name.clone()).clone(), metamodelica::nil(), metamodelica::nil())?;
            outDaeEls = List::append_reverse(named.clone(), outDaeEls.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outDaeEls = List::append_reverse(inDaeEls.clone(), outDaeEls.clone());
    outDaeEls = metamodelica::Dangerous::listReverseInPlace(outDaeEls.clone());
    Ok(outDaeEls)
}

fn splitVariableNamed(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inName: ArcStr, mut inAccNamed: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inAccRest: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outNamed: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outRest: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (outNamed, outRest) = (::match_deref::match_deref! { match &((inElementLst.clone(), inName.clone(), inAccNamed.clone(), inAccRest.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            (inAccNamed.clone().reverse(), inAccRest.clone().reverse())
        },
        (Deref @ metamodelica::List::Cons { head: x @ Deref @ DAE::Element::VAR { componentRef: cr, .. }, tail: lst }, _, accNamed, accRest) => {
            let mut equal: bool = false;
            let mut accNamed = (*accNamed).clone();
            let mut accRest = (*accRest).clone();
            equal = stringEq((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (inName.clone()).clone());
            accNamed = List::consOnTrue(equal.clone(), x.clone(), accNamed.clone());
            accRest = List::consOnTrue(boolNot(equal.clone()), x.clone(), accRest.clone());
            (accNamed, accRest) = splitVariableNamed(lst.clone(), (inName.clone()).clone(), accNamed.clone(), accRest.clone())?;
            (accNamed.clone(), accRest.clone())
        },
        (Deref @ metamodelica::List::Cons { head: x, tail: lst }, _, accNamed, accRest) => {
            let mut accNamed = (*accNamed).clone();
            let mut accRest = (*accRest).clone();
            (accNamed, accRest) = splitVariableNamed(lst.clone(), (inName.clone()).clone(), accNamed.clone(), cons(x.clone(), accRest.clone()))?;
            (accNamed.clone(), accRest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNamed, outRest))
}

pub fn getAllExpandableCrefsFromDAE(mut inDAE: DAE::DAElist) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (inDAE.clone()) else { bail!("pattern mismatch") };
    elts = __pa0.clone();
    let (_, (_, __pa1)) = traverseDAEElementList(elts.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (fnptr!(collectAllExpandableCrefsInExp, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), metamodelica::nil()));
    outCrefs = __pa1.clone();
    Ok(outCrefs)
}

fn collectAllExpandableCrefsInExp(mut exp: Arc<DAE::Exp>, mut acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outCrefs) = (::match_deref::match_deref! { match &((exp.clone(), acc.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
            (exp.clone(), List::consOnTrue(ConnectUtil::isExpandable(cr.clone()), cr.clone(), acc.clone()))
        },
        _ => {
            (exp.clone(), acc.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outCrefs)
}

pub fn daeDescription(mut inDAE: DAE::DAElist) -> ArcStr {
    let mut comment: ArcStr = arcstr::literal!("");
    comment = ((::match_deref::match_deref! { match &(inDAE.clone()) {
        DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { comment: Some(Deref @ SCode::Comment { comment: Some(comment), .. }), .. }, tail: _ } } => comment.clone(),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    comment
}

pub fn replaceCallAttrType(mut caIn: Arc<DAE::CallAttributes>, mut typeIn: Arc<DAE::Type>) -> Arc<DAE::CallAttributes> {
    let mut caOut: Arc<DAE::CallAttributes>;
    caOut = caIn.clone();
    assign_field!(caOut.ty = typeIn.clone());
    if Types::isTuple(typeIn.clone()) {
        assign_field!(caOut.tuple_ = true);
    }
    caOut
}

pub fn funcIsRecord(mut func: DAE::Function) -> bool {
    let mut isRec: bool = false;
    isRec = (match func.clone() {
        DAE::Function::RECORD_CONSTRUCTOR { .. } => true,
        _ => false,
    });
    isRec
}

pub fn funcArgDim(mut argIn: Arc<DAE::FuncArg>) -> i32 {
    let mut dim: i32 = 0;
    dim = (::match_deref::match_deref! { match &(argIn.clone()) {
        Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_ARRAY { dims: arrayDims, .. }, .. } => {
            List::applyAndFold(arrayDims.clone(), Arc::new(fnptr!(intAdd, i32, i32)), Arc::new(Expression::dimensionSize), 0)
        },
        Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_ENUMERATION { names, .. }, .. } => {
            (names.clone().len() as i32)
        },
        _ => {
            1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dim
}

pub fn toDAEInnerOuter(mut ioIn: Absyn::InnerOuter) -> Result<DAE::VarInnerOuter> {
    let mut ioOut: DAE::VarInnerOuter = DAE::VarInnerOuter::INNER;
    ioOut = (match ioIn.clone() {
        Absyn::InnerOuter::INNER => openmodelica_frontend_types::DAE::VarInnerOuter::INNER,
        Absyn::InnerOuter::OUTER => openmodelica_frontend_types::DAE::VarInnerOuter::OUTER,
        Absyn::InnerOuter::INNER_OUTER => openmodelica_frontend_types::DAE::VarInnerOuter::INNER_OUTER,
        Absyn::InnerOuter::NOT_INNER_OUTER => openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        _ => bail!("match: no arm matched"),
    });
    Ok(ioOut)
}

pub fn getAssertConditionCrefs(mut stmt: Arc<DAE::Statement>, mut crefsIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut crefsOut: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    crefsOut = (::match_deref::match_deref! { match &((stmt.clone(), crefsIn.clone())) {
        (Deref @ DAE::Statement::STMT_ASSERT { cond, .. }, _) => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crefs = Expression::extractCrefsFromExp(cond.clone())?;
            listAppend(crefsIn.clone(), crefs.clone())
        },
        _ => {
            crefsIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefsOut)
}

pub fn getSubscriptIndex(mut iSubscript: Arc<DAE::Subscript>) -> i32 {
    let mut oIndex: i32 = 0;
    let mut index: i32 = 0;
    let mut exp: Arc<DAE::Exp>;
    oIndex = (::match_deref::match_deref! { match &(iSubscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: index } } => index.clone(),
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { index, .. } } => index.clone(),
        _ => -1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oIndex
}

pub fn bindingValue(mut inBinding: Arc<DAE::Binding>) -> Option<Arc<Values::Value>> {
    let mut outValue: Option<Arc<Values::Value>> = None;
    outValue = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::EQBOUND { .. } => var_field!((*inBinding).evaluatedExp, DAE::Binding::EQBOUND).clone(),
        Deref @ DAE::Binding::VALBOUND { .. } => Some(var_field!((*inBinding).valBound, DAE::Binding::VALBOUND).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

pub fn statementsContainReturn(mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<bool> {
    let mut b: bool = false;
    (_, b) = traverseDAEStmts(stmts.clone(), Arc::new(statementsContainReturn2), false)?;
    Ok(b)
}

pub fn statementsContainTryBlock(mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<bool> {
    let mut b: bool = false;
    (_, b) = traverseDAEStmts(stmts.clone(), Arc::new(fnptr!(statementsContainTryBlock2, Arc<DAE::Exp>, Arc<DAE::Statement>, bool)), false)?;
    Ok(b)
}

fn statementsContainReturn2(mut inExp: Arc<DAE::Exp>, mut inStmt: Arc<DAE::Statement>, mut b: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut ob: bool = b.clone();
    if !(b.clone()) {
        ob = (::match_deref::match_deref! { match &(inStmt.clone()) {
        Deref @ DAE::Statement::STMT_RETURN { .. } => true,
        _ => (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. } => {
            let mut body: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            for mut c in &*cases.clone() {
                let mut c = c.clone();
                if !(ob.clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(c.clone()) {
                        Deref @ DAE::MatchCase { body: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    body = __pa0.clone();
                    ob = statementsContainReturn(body.clone())?;
                }
            }
            ob.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((outExp, ob))
}

fn statementsContainTryBlock2(mut inExp: Arc<DAE::Exp>, mut inStmt: Arc<DAE::Statement>, mut b: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut ob: bool = b.clone();
    if !(b.clone()) {
        ob = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: DAE::MatchType::MATCHCONTINUE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    (outExp, ob)
}

pub fn getVarBinding(mut iels: Arc<metamodelica::List<Arc<DAE::Element>>>, mut icr: Arc<DAE::ComponentRef>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut obnd: Option<Arc<DAE::Exp>> = None;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut e: Arc<DAE::Exp>;
    let mut lst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    obnd = None;
    for mut i in &*iels.clone() {
        let mut i = i.clone();
        obnd = (::match_deref::match_deref! { match &(i.clone()) {
        Deref @ DAE::Element::VAR { binding: obnd, componentRef: cr, .. } => {
            if ComponentReferenceBasics::crefEqualNoStringCompare(icr.clone(), cr.clone())? {
                return Ok(obnd);
            }
            obnd.clone()
        },
        Deref @ DAE::Element::DEFINE { exp: e, componentRef: cr, .. } => {
            obnd = Some(e.clone());
            if ComponentReferenceBasics::crefEqualNoStringCompare(icr.clone(), cr.clone())? {
                return Ok(obnd);
            }
            obnd.clone()
        },
        Deref @ DAE::Element::INITIALDEFINE { exp: e, componentRef: cr, .. } => {
            obnd = Some(e.clone());
            if ComponentReferenceBasics::crefEqualNoStringCompare(icr.clone(), cr.clone())? {
                return Ok(obnd);
            }
            obnd.clone()
        },
        Deref @ DAE::Element::EQUATION { scalar: e, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            obnd = Some(e.clone());
            if ComponentReferenceBasics::crefEqualNoStringCompare(icr.clone(), cr.clone())? {
                return Ok(obnd);
            }
            obnd.clone()
        },
        Deref @ DAE::Element::EQUATION { scalar: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, exp: e, .. } => {
            obnd = Some(e.clone());
            if ComponentReferenceBasics::crefEqualNoStringCompare(icr.clone(), cr.clone())? {
                return Ok(obnd);
            }
            obnd.clone()
        },
        Deref @ DAE::Element::INITIALEQUATION { exp2: e, exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            obnd = Some(e.clone());
            if ComponentReferenceBasics::crefEqualNoStringCompare(icr.clone(), cr.clone())? {
                return Ok(obnd);
            }
            obnd.clone()
        },
        Deref @ DAE::Element::INITIALEQUATION { exp2: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, exp1: e, .. } => {
            obnd = Some(e.clone());
            if ComponentReferenceBasics::crefEqualNoStringCompare(icr.clone(), cr.clone())? {
                return Ok(obnd);
            }
            obnd.clone()
        },
        _ => obnd.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(obnd)
}

pub fn evaluateCref(mut icr: Arc<DAE::ComponentRef>, mut iels: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut oexp: Option<Arc<DAE::Exp>> = None;
    let mut e: Arc<DAE::Exp>;
    let mut ee: Arc<DAE::Exp>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut oexps: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>> = metamodelica::nil();
    let mut o: Option<Arc<DAE::Exp>> = None;
    oexp = getVarBinding(iels.clone(), icr.clone())?;
    if isSome(oexp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(oexp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        (e, _) = ExpressionSimplify::simplify(e.clone())?;
        if Expression::isConst(e.clone())? {
            oexp = Some(e.clone());
            return Ok(oexp);
        }
        crefs = Expression::getAllCrefs(e.clone())?;
        oexps = List::map1(crefs.clone(), Arc::new(evaluateCref), iels.clone());
        for mut c in &*crefs.clone() {
            let mut c = c.clone();
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(oexps.clone()) {
                Deref @ metamodelica::List::Cons { head: Some(__pa1), tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ee = __pa1.clone();
            oexps = __pa2.clone();
            (e, _) = Expression::replaceCref(e.clone(), (c.clone(), ee.clone()))?;
            (e, _) = ExpressionSimplify::simplify(e.clone())?;
        }
        oexp = Some(e.clone());
    }
    Ok(oexp)
}

pub fn evaluateExp(mut iexp: Arc<DAE::Exp>, mut iels: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut oexp: Option<Arc<DAE::Exp>> = None;
    let mut e: Arc<DAE::Exp>;
    let mut ee: Arc<DAE::Exp>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut oexps: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>> = metamodelica::nil();
    let mut o: Option<Arc<DAE::Exp>> = None;
    if Expression::isConst(iexp.clone())? {
        oexp = Some(iexp.clone());
        return Ok(oexp);
    }
    match '__try0: {
        e = iexp.clone();
        crefs = unwrap_break_err!(Expression::getAllCrefs(e.clone()), '__try0);
        oexps = List::map1(crefs.clone(), Arc::new(evaluateCref), iels.clone());
        for mut c in &*crefs.clone() {
            let mut c = c.clone();
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(oexps.clone()) {
                Deref @ metamodelica::List::Cons { head: Some(__pa1), tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            ee = __pa1.clone();
            oexps = __pa2.clone();
            e = unwrap_break_err!(Expression::replaceCrefBottomUp(e.clone(), c.clone(), ee.clone()), '__try0);
            (e, _) = unwrap_break_err!(ExpressionSimplify::simplify(e.clone()), '__try0);
        }
        oexp = Some(e.clone());
        Ok::<_, anyhow::Error>((oexp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            oexp = __try0_o0;
        }
        Err(_) => {
            oexp = None;
        }
    }
    Ok(oexp)
}

pub fn replaceCrefInDAEElements(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inCref: Arc<DAE::ComponentRef>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut repl: VarTransform::VariableReplacements;
    repl = VarTransform::emptyReplacements();
    repl = VarTransform::addReplacement(repl.clone(), inCref.clone(), inExp.clone())?;
    (outElements, _) = traverseDAEElementList(inElements.clone(), Arc::new(replaceCrefBottomUp), repl.clone());
    Ok(outElements)
}

pub fn replaceCrefBottomUp(mut inExp: Arc<DAE::Exp>, mut replIn: VarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, VarTransform::VariableReplacements)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut replOut: VarTransform::VariableReplacements;
    replOut = replIn.clone();
    (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(replaceCompRef), replIn.clone())?;
    Ok((outExp, replOut))
}

fn replaceCompRef(mut inExp: Arc<DAE::Exp>, mut replIn: VarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, VarTransform::VariableReplacements)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut replOut: VarTransform::VariableReplacements;
    replOut = replIn.clone();
    (outExp, _) = VarTransform::replaceExp(inExp.clone(), replIn.clone(), None)?;
    Ok((outExp, replOut))
}

pub fn connectorTypeStr(mut connectorType: Arc<DAE::ConnectorType>) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(connectorType.clone()) {
        Deref @ DAE::ConnectorType::POTENTIAL => {
            literal!("")
        },
        Deref @ DAE::ConnectorType::FLOW => {
            literal!("flow")
        },
        Deref @ DAE::ConnectorType::STREAM { associatedFlow: None } => {
            literal!("stream()")
        },
        Deref @ DAE::ConnectorType::STREAM { associatedFlow: Some(cref) } => {
            let mut cref_str: ArcStr = arcstr::literal!("");
            cref_str = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("stream(")); __mm_s.push_str(&*cref_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("non connector")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(string)
}

pub fn streamBool(mut inStream: Arc<DAE::ConnectorType>) -> bool {
    let mut bStream: bool = false;
    bStream = (::match_deref::match_deref! { match &(inStream.clone()) {
        Deref @ DAE::ConnectorType::STREAM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bStream
}

pub fn potentialBool(mut inConnectorType: Arc<DAE::ConnectorType>) -> bool {
    let mut outPotential: bool = false;
    outPotential = (::match_deref::match_deref! { match &(inConnectorType.clone()) {
        Deref @ DAE::ConnectorType::POTENTIAL => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPotential
}

pub fn connectorTypeEqual(mut inConnectorType1: Arc<DAE::ConnectorType>, mut inConnectorType2: Arc<DAE::ConnectorType>) -> Result<bool> {
    let mut outEqual: bool = false;
    outEqual = (::match_deref::match_deref! { match &((inConnectorType1.clone(), inConnectorType2.clone())) {
        (Deref @ DAE::ConnectorType::POTENTIAL, Deref @ DAE::ConnectorType::POTENTIAL) => true,
        (Deref @ DAE::ConnectorType::FLOW, Deref @ DAE::ConnectorType::FLOW) => true,
        (Deref @ DAE::ConnectorType::STREAM { associatedFlow: _ }, Deref @ DAE::ConnectorType::STREAM { associatedFlow: _ }) => true,
        (Deref @ DAE::ConnectorType::NON_CONNECTOR, Deref @ DAE::ConnectorType::NON_CONNECTOR) => true,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqual)
}

pub fn toSCodeConnectorType(mut daeConnectorType: Arc<DAE::ConnectorType>) -> Result<SCode::ConnectorType> {
    let mut scodeConnectorType: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    scodeConnectorType = (::match_deref::match_deref! { match &(daeConnectorType.clone()) {
        Deref @ DAE::ConnectorType::FLOW => openmodelica_frontend_types::SCode::ConnectorType::FLOW,
        Deref @ DAE::ConnectorType::STREAM { associatedFlow: _ } => openmodelica_frontend_types::SCode::ConnectorType::STREAM,
        Deref @ DAE::ConnectorType::POTENTIAL => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        Deref @ DAE::ConnectorType::NON_CONNECTOR => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        _ => bail!("match: no arm matched"),
    } });
    Ok(scodeConnectorType)
}

pub fn mergeAlgorithmSections(mut inDae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut newEls: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut istmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut s: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource>;
    let mut src: Arc<DAE::ElementSource>;
    let mut ident: ArcStr = arcstr::literal!("");
    let mut comment: Option<Arc<SCode::Comment>> = None;
    if !(Flags::isSet(Flags::MERGE_ALGORITHM_SECTIONS.clone())?) {
        outDae = inDae.clone();
        return Ok(outDae);
    }
    let DAE::DAE { elementLst: __pa0 } = (inDae.clone()) else { bail!("pattern mismatch") };
    els = __pa0.clone();
    for mut e in &*els.clone() {
        let mut e = e.clone();
        let _ = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Element::COMP { ident, dAElist, source: src, comment } => {
            let mut dAElist = (*dAElist).clone();
            let DAE::DAE { elementLst: __pa0 } = (mergeAlgorithmSections(DAE::DAElist { elementLst: dAElist.clone() })?) else { bail!("pattern mismatch") };
            dAElist = __pa0.clone();
            newEls = cons(Arc::new(DAE::Element::COMP { ident: (ident.clone()).clone(), dAElist: dAElist.clone(), source: src.clone(), comment: comment.clone() }), newEls.clone());
            ()
        },
        Deref @ DAE::Element::ALGORITHM { source, algorithm_: Deref @ DAE::Algorithm { statementLst: s } } => {
            stmts = List::append_reverse(s.clone(), stmts.clone());
            ()
        },
        Deref @ DAE::Element::INITIALALGORITHM { source, algorithm_: Deref @ DAE::Algorithm { statementLst: s } } => {
            istmts = List::append_reverse(s.clone(), istmts.clone());
            ()
        },
        _ => {
            newEls = cons(e.clone(), newEls.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if !(istmts.clone().is_empty()) {
        newEls = cons(Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: istmts.clone().reverse() }), source: source.clone() }), newEls.clone());
    }
    if !(stmts.clone().is_empty()) {
        newEls = cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone().reverse() }), source: source.clone() }), newEls.clone());
    }
    newEls = newEls.clone().reverse();
    outDae = DAE::DAElist { elementLst: newEls.clone() };
    Ok(outDae)
}

pub fn moveElementToInitialSection(mut elt: Arc<DAE::Element>) -> Arc<DAE::Element> {
    let mut elt: Arc<DAE::Element> = elt;
    elt = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::EQUATION { .. } => Arc::new(DAE::Element::INITIALEQUATION { exp1: var_field!((*elt).exp, DAE::Element::EQUATION).clone(), exp2: var_field!((*elt).scalar, DAE::Element::EQUATION).clone(), source: var_field!((*elt).source, DAE::Element::EQUATION).clone() }),
        Deref @ DAE::Element::DEFINE { .. } => Arc::new(DAE::Element::INITIALDEFINE { componentRef: var_field!((*elt).componentRef, DAE::Element::DEFINE).clone(), exp: var_field!((*elt).exp, DAE::Element::DEFINE).clone(), source: var_field!((*elt).source, DAE::Element::DEFINE).clone() }),
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: var_field!((*elt).dimension, DAE::Element::ARRAY_EQUATION).clone(), exp: var_field!((*elt).exp, DAE::Element::ARRAY_EQUATION).clone(), array: var_field!((*elt).array, DAE::Element::ARRAY_EQUATION).clone(), source: var_field!((*elt).source, DAE::Element::ARRAY_EQUATION).clone() }),
        Deref @ DAE::Element::COMPLEX_EQUATION { .. } => Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: var_field!((*elt).lhs, DAE::Element::COMPLEX_EQUATION).clone(), rhs: var_field!((*elt).rhs, DAE::Element::COMPLEX_EQUATION).clone(), source: var_field!((*elt).source, DAE::Element::COMPLEX_EQUATION).clone() }),
        Deref @ DAE::Element::IF_EQUATION { .. } => Arc::new(DAE::Element::INITIAL_IF_EQUATION { condition1: var_field!((*elt).condition1, DAE::Element::IF_EQUATION).clone(), equations2: var_field!((*elt).equations2, DAE::Element::IF_EQUATION).clone(), equations3: var_field!((*elt).equations3, DAE::Element::IF_EQUATION).clone(), source: var_field!((*elt).source, DAE::Element::IF_EQUATION).clone() }),
        Deref @ DAE::Element::ALGORITHM { .. } => Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: var_field!((*elt).algorithm_, DAE::Element::ALGORITHM).clone(), source: var_field!((*elt).source, DAE::Element::ALGORITHM).clone() }),
        Deref @ DAE::Element::ASSERT { .. } => Arc::new(DAE::Element::INITIAL_ASSERT { condition: var_field!((*elt).condition, DAE::Element::ASSERT).clone(), message: var_field!((*elt).message, DAE::Element::ASSERT).clone(), level: var_field!((*elt).level, DAE::Element::ASSERT).clone(), source: var_field!((*elt).source, DAE::Element::ASSERT).clone() }),
        Deref @ DAE::Element::TERMINATE { .. } => Arc::new(DAE::Element::INITIAL_TERMINATE { message: var_field!((*elt).message, DAE::Element::TERMINATE).clone(), source: var_field!((*elt).source, DAE::Element::TERMINATE).clone() }),
        Deref @ DAE::Element::NORETCALL { .. } => Arc::new(DAE::Element::INITIAL_NORETCALL { exp: var_field!((*elt).exp, DAE::Element::NORETCALL).clone(), source: var_field!((*elt).source, DAE::Element::NORETCALL).clone() }),
        _ => elt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elt
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getParameters(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut acc: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut params: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    params = (::match_deref::match_deref! { match &((elts.clone(), acc.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            acc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { dAElist: celts, .. }, tail: rest }, _) => {
            let mut a: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            a = getParameters(celts.clone(), acc.clone())?;
            a = getParameters(rest.clone(), a.clone())?;
            a.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Element::VAR { .. }, tail: rest }, _) => {
            if (isParameterOrConstant(e.clone())) {cons(e.clone(), getParameters(rest.clone(), acc.clone())?)} else {getParameters(rest.clone(), acc.clone())?}
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            getParameters(rest.clone(), acc.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(params)
}

pub fn getInteger(mut exp: Arc<DAE::Exp>) -> Result<i32> {
    let mut i: i32 = 0;
    i = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: i } => i.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.getInteger")); __mm_s.push_str(&*literal!(" failed because expression is not an ICONST: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

pub fn optimizeMetaRecordFieldAssigns(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = tempVars;
    let mut s2: Arc<DAE::Statement>;
    outStmts = metamodelica::nil();
    for mut s in &*inStmts.clone() {
        let mut s = s.clone();
        (s2, tempVars) = optMRFAInStmt(s.clone(), tempVars.clone())?;
        outStmts = cons(s2.clone(), outStmts.clone());
    }
    outStmts = outStmts.clone().reverse();
    (outStmts, tempVars) = optMRFAMergeList(outStmts.clone(), tempVars.clone())?;
    Ok((outStmts, tempVars))
}

fn optMRFAInStmt(mut stmt: Arc<DAE::Statement>, mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<DAE::Statement>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut stmt: Arc<DAE::Statement> = stmt;
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = tempVars;
    (stmt, tempVars) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_IF { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut els: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*stmt).statementLst, DAE::Statement::STMT_IF).clone(), tempVars.clone())?;
            assign_variant_field!(stmt => DAE::Statement::STMT_IF; statementLst = stmts.clone());
            (els, tempVars) = optMRFAInElse(var_field!((*stmt).else_, DAE::Statement::STMT_IF).clone(), tempVars.clone())?;
            assign_variant_field!(stmt => DAE::Statement::STMT_IF; else_ = els.clone());
            (stmt.clone(), tempVars.clone())
        },
        Deref @ DAE::Statement::STMT_FOR { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*stmt).statementLst, DAE::Statement::STMT_FOR).clone(), tempVars.clone())?;
            assign_variant_field!(stmt => DAE::Statement::STMT_FOR; statementLst = stmts.clone());
            (stmt.clone(), tempVars.clone())
        },
        Deref @ DAE::Statement::STMT_PARFOR { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*stmt).statementLst, DAE::Statement::STMT_PARFOR).clone(), tempVars.clone())?;
            assign_variant_field!(stmt => DAE::Statement::STMT_PARFOR; statementLst = stmts.clone());
            (stmt.clone(), tempVars.clone())
        },
        Deref @ DAE::Statement::STMT_WHILE { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*stmt).statementLst, DAE::Statement::STMT_WHILE).clone(), tempVars.clone())?;
            assign_variant_field!(stmt => DAE::Statement::STMT_WHILE; statementLst = stmts.clone());
            (stmt.clone(), tempVars.clone())
        },
        Deref @ DAE::Statement::STMT_WHEN { .. } => {
            let mut ew: Arc<DAE::Statement>;
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*stmt).statementLst, DAE::Statement::STMT_WHEN).clone(), tempVars.clone())?;
            assign_variant_field!(stmt => DAE::Statement::STMT_WHEN;
                statementLst = stmts.clone(),
                elseWhen = (::match_deref::match_deref! { match &(var_field!((*stmt).elseWhen, DAE::Statement::STMT_WHEN).clone()) {
        Some(ew) => {
            let mut ew = (*ew).clone();
            (ew, tempVars) = optMRFAInStmt(ew.clone(), tempVars.clone())?;
            Some(ew.clone())
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
            );
            (stmt.clone(), tempVars.clone())
        },
        Deref @ DAE::Statement::STMT_FAILURE { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*stmt).body, DAE::Statement::STMT_FAILURE).clone(), tempVars.clone())?;
            assign_variant_field!(stmt => DAE::Statement::STMT_FAILURE; body = stmts.clone());
            (stmt.clone(), tempVars.clone())
        },
        _ => {
            (stmt.clone(), tempVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmt, tempVars))
}

fn optMRFAInElse(mut els: Arc<DAE::Else>, mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<DAE::Else>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut els: Arc<DAE::Else> = els;
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = tempVars;
    (els, tempVars) = (::match_deref::match_deref! { match &(els.clone()) {
        Deref @ DAE::Else::ELSEIF { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut nestedEls: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*els).statementLst, DAE::Else::ELSEIF).clone(), tempVars.clone())?;
            assign_variant_field!(els => DAE::Else::ELSEIF; statementLst = stmts.clone());
            (nestedEls, tempVars) = optMRFAInElse(var_field!((*els).else_, DAE::Else::ELSEIF).clone(), tempVars.clone())?;
            assign_variant_field!(els => DAE::Else::ELSEIF; else_ = nestedEls.clone());
            (els.clone(), tempVars.clone())
        },
        Deref @ DAE::Else::ELSE { .. } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(var_field!((*els).statementLst, DAE::Else::ELSE).clone(), tempVars.clone())?;
            assign_variant_field!(els => DAE::Else::ELSE; statementLst = stmts.clone());
            (els.clone(), tempVars.clone())
        },
        _ => {
            (els.clone(), tempVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((els, tempVars))
}

// (field name, rhs expression, originating statement -- kept so we can reuse
// its ElementSource when the group is merged into a single statement).
pub type MRFAUpdate = (ArcStr, Arc<DAE::Exp>, Arc<DAE::Statement>);

fn optMRFAMergeList(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = tempVars;
    let mut rest: Arc<metamodelica::List<Arc<DAE::Statement>>> = inStmts.clone();
    let mut stmt: Arc<DAE::Statement>;
    let mut m: Option<(Arc<DAE::Exp>, Arc<DAE::Type>, ArcStr, Arc<DAE::Exp>)> = None;
    let mut base: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut mrecTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut field: ArcStr = arcstr::literal!("");
    let mut group: Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>, Arc<DAE::Statement>)>> = metamodelica::nil();
    let mut writtenFields: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut extended: bool = false;
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        stmt = __pa0.clone();
        rest = __pa1.clone();
        m = optMRFAMatch(stmt.clone());
        (outStmts, rest, tempVars) = (::match_deref::match_deref! { match &(m.clone()) {
        Some((base, mrecTy, field, rhs)) => {
            group = list![(field.clone(), rhs.clone(), stmt.clone())];
            writtenFields = list![(field.clone()).clone()];
            extended = true;
            while extended.clone() {
                (writtenFields, group, extended) = optMRFATryExtend(rest.clone(), base.clone(), mrecTy.clone(), writtenFields.clone(), group.clone())?;
                if extended.clone() {
                    rest = listRest(rest.clone())?;
                }
            }
            (outStmts, tempVars) = optMRFACommitGroup(group.clone().reverse(), base.clone(), mrecTy.clone(), outStmts.clone(), tempVars.clone())?;
            (outStmts.clone(), rest.clone(), tempVars.clone())
        },
        _ => (cons(stmt.clone(), outStmts.clone()), rest.clone(), tempVars.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outStmts = outStmts.clone().reverse();
    Ok((outStmts, tempVars))
}

fn optMRFACommitGroup(mut group: Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>, Arc<DAE::Statement>)>>, mut baseExp: Arc<DAE::Exp>, mut mrecTy: Arc<DAE::Type>, mut acc: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outAcc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = tempVars;
    let mut original: Arc<DAE::Statement>;
    let mut mergedStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    (outAcc, tempVars) = (::match_deref::match_deref! { match &(group.clone()) {
        Deref @ metamodelica::List::Cons { head: (_, _, original), tail: Deref @ metamodelica::List::Nil } => (cons(original.clone(), acc.clone()), tempVars.clone()),
        _ => {
            (mergedStmts, tempVars) = optMRFABuildMerged(group.clone(), baseExp.clone(), mrecTy.clone(), tempVars.clone())?;
            (List::append_reverse(mergedStmts.clone(), acc.clone()), tempVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outAcc, tempVars))
}

fn optMRFABuildMerged(mut group: Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>, Arc<DAE::Statement>)>>, mut baseExp: Arc<DAE::Exp>, mut mrecTy: Arc<DAE::Type>, mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut tempVars: Arc<metamodelica::List<Arc<DAE::Element>>> = tempVars;
    let mut path: Arc<Absyn::Path>;
    let mut index: i32 = 0;
    let mut typeVars: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut fields: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut fieldNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fv: Arc<DAE::Var>;
    let mut fname: ArcStr = arcstr::literal!("");
    let mut gname: ArcStr = arcstr::literal!("");
    let mut tname: ArcStr = arcstr::literal!("");
    let mut fty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut gty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut grhs: Arc<DAE::Exp>;
    let mut pos: i32 = 1;
    let mut arg: Arc<DAE::Exp>;
    let mut src: Arc<DAE::ElementSource>;
    let mut baseTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tempAssigns: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut fieldRefs: Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>)>> = metamodelica::nil();
    let mut tcref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tref: Arc<DAE::Exp>;
    let mut tvar: Arc<DAE::Element>;
    (path, index, typeVars) = optMRFAMetaRecordInfo(mrecTy.clone())?;
    fields = Types::getMetaRecordFields(mrecTy.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(Util::tuple33(listHead(group.clone())?)) {
        Deref @ DAE::Statement::STMT_ASSIGN { source: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    src = __pa0.clone();
    for mut upd in &*group.clone() {
        let mut upd = upd.clone();
        (gname, grhs, _) = upd.clone();
        gty = optMRFAFieldType(fields.clone(), (gname.clone()).clone())?;
        tname = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$mrfa_")); __mm_s.push_str(&*Util::tickStr()); ArcStr::from(__mm_s) }).clone();
        tcref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (tname.clone()).clone(), identType: gty.clone(), subscriptLst: metamodelica::nil() });
        tref = Arc::new(DAE::Exp::CREF { componentRef: tcref.clone(), ty: gty.clone() });
        tempAssigns = cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: gty.clone(), exp1: tref.clone(), exp: grhs.clone(), source: src.clone() }), tempAssigns.clone());
        tvar = Arc::new(DAE::Element::VAR { componentRef: tcref.clone(), kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE, direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, protection: openmodelica_frontend_types::DAE::VarVisibility::PROTECTED, ty: gty.clone(), binding: None, dims: metamodelica::nil(), connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), source: src.clone(), variableAttributesOption: None, comment: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: false });
        tempVars = cons(tvar.clone(), tempVars.clone());
        fieldRefs = cons((gname.clone(), tref.clone()), fieldRefs.clone());
    }
    for mut fv in &*fields.clone() {
        let mut fv = fv.clone();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(fv.clone()) {
            Deref @ DAE::Var { ty: __pa1, name: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        fty = __pa1.clone();
        fname = __pa2.clone();
        arg = (::match_deref::match_deref! { match &(optMRFALookupRef(fieldRefs.clone(), (fname.clone()).clone())) {
        Some(arg) => arg.clone(),
        _ => Arc::new(DAE::Exp::RSUB { exp: baseExp.clone(), ix: pos.clone(), fieldName: (fname.clone()).clone(), ty: fty.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        args = cons(arg.clone(), args.clone());
        fieldNames = cons(fname.clone(), fieldNames.clone());
        pos = pos.clone() + 1;
    }
    args = args.clone().reverse();
    fieldNames = fieldNames.clone().reverse();
    baseTy = Expression::r#typeof(baseExp.clone())?;
    tempAssigns = cons(Arc::new(DAE::Statement::STMT_ASSIGN { type_: baseTy.clone(), exp1: baseExp.clone(), exp: Arc::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: args.clone(), fieldNames: fieldNames.clone(), index: index.clone(), typeVars: typeVars.clone() }), source: src.clone() }), tempAssigns.clone());
    outStmts = tempAssigns.clone().reverse();
    Ok((outStmts, tempVars))
}

fn optMRFAFieldType(mut fields: Arc<metamodelica::List<Arc<DAE::Var>>>, mut fname: ArcStr) -> Result<Arc<DAE::Type>> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut n: ArcStr = arcstr::literal!("");
    for mut fv in &*fields.clone() {
        let mut fv = fv.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(fv.clone()) {
            Deref @ DAE::Var { ty: __pa0, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa0.clone();
        n = __pa1.clone();
        if stringEq((n.clone()).clone(), (fname.clone()).clone()) {
            return Ok(ty);
        }
    }
    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("optMRFAFieldType: field ")); __mm_s.push_str(&*fname.clone()); __mm_s.push_str(&*literal!(" not found in metarecord")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    bail!("fail");
    Ok(ty)
}

fn optMRFALookupRef(mut fieldRefs: Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>)>>, mut fname: ArcStr) -> Option<Arc<DAE::Exp>> {
    let mut outRef: Option<Arc<DAE::Exp>> = None;
    let mut n: ArcStr = arcstr::literal!("");
    let mut r: Arc<DAE::Exp>;
    outRef = None;
    for mut fr in &*fieldRefs.clone() {
        let mut fr = fr.clone();
        (n, r) = fr.clone();
        if stringEq((n.clone()).clone(), (fname.clone()).clone()) {
            outRef = Some(r.clone());
            return outRef;
        }
    }
    outRef
}

fn optMRFAMetaRecordInfo(mut ty: Arc<DAE::Type>) -> Result<(Arc<Absyn::Path>, i32, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut path: Arc<Absyn::Path>;
    let mut index: i32 = 0;
    let mut typeVars: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (path, index, typeVars) = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METARECORD { .. } => {
            (var_field!((*ty).path, DAE::Type::T_METARECORD).clone(), var_field!((*ty).index, DAE::Type::T_METARECORD).clone(), var_field!((*ty).typeVars, DAE::Type::T_METARECORD).clone())
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: rec }, .. } => {
            optMRFAMetaRecordPieces(rec.clone())?
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_TYPE_FUNCTION { fun: r#fn }, .. } => {
            optMRFAMetaRecordPieces(r#fn()?)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((path, index, typeVars))
}

fn optMRFAMetaRecordPieces(mut ty: Arc<DAE::Type>) -> Result<(Arc<Absyn::Path>, i32, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut path: Arc<Absyn::Path>;
    let mut index: i32 = 0;
    let mut typeVars: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METARECORD { typeVars: __pa0, index: __pa1, path: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    typeVars = __pa0.clone();
    index = __pa1.clone();
    path = __pa2.clone();
    Ok((path, index, typeVars))
}

fn optMRFAMatch(mut stmt: Arc<DAE::Statement>) -> Option<(Arc<DAE::Exp>, Arc<DAE::Type>, ArcStr, Arc<DAE::Exp>)> {
    let mut outMatch: Option<(Arc<DAE::Exp>, Arc<DAE::Type>, ArcStr, Arc<DAE::Exp>)> = None;
    outMatch = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: Deref @ DAE::Exp::RSUB { fieldName: fname, exp: baseExp @ Deref @ DAE::Exp::CREF { ty: t1 @ Deref @ DAE::Type::T_METARECORD { .. }, .. }, .. }, .. } => {
            Some((baseExp.clone(), t1.clone(), fname.clone(), rhs.clone()))
        },
        Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: Deref @ DAE::Exp::RSUB { fieldName: fname, exp: baseExp @ Deref @ DAE::Exp::CREF { ty: t1 @ Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: true, .. }, .. }, .. }, .. } if (optMRFAResolvableSingleton(t1.clone())) => {
            Some((baseExp.clone(), t1.clone(), fname.clone(), rhs.clone()))
        },
        Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: fname, .. }, subscriptLst: subs, identType: identTy @ Deref @ DAE::Type::T_METATYPE { ty: t1 @ Deref @ DAE::Type::T_METARECORD { .. } }, ident }, .. }, .. } => {
            let mut baseExp: Arc<DAE::Exp>;
            let mut topCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            topCref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identTy.clone(), subscriptLst: subs.clone() });
            baseExp = Arc::new(DAE::Exp::CREF { componentRef: topCref.clone(), ty: identTy.clone() });
            Some((baseExp.clone(), t1.clone(), fname.clone(), rhs.clone()))
        },
        Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: fname, .. }, subscriptLst: subs, identType: identTy @ Deref @ DAE::Type::T_METATYPE { ty: t1 @ Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: true, .. } }, ident }, .. }, .. } if (optMRFAResolvableSingleton(t1.clone())) => {
            let mut baseExp: Arc<DAE::Exp>;
            let mut topCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            topCref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identTy.clone(), subscriptLst: subs.clone() });
            baseExp = Arc::new(DAE::Exp::CREF { componentRef: topCref.clone(), ty: identTy.clone() });
            Some((baseExp.clone(), t1.clone(), fname.clone(), rhs.clone()))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

fn optMRFAResolvableSingleton(mut ty: Arc<DAE::Type>) -> bool {
    let mut ok: bool = false;
    ok = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: Deref @ DAE::Type::T_METARECORD { .. } }, .. } => true,
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_TYPE_FUNCTION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ok
}

fn optMRFATryExtend(mut rest: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut baseExp: Arc<DAE::Exp>, mut mrecTy: Arc<DAE::Type>, mut writtenFields: Arc<metamodelica::List<ArcStr>>, mut group: Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>, Arc<DAE::Statement>)>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>, Arc<DAE::Statement>)>>, bool)> {
    let mut writtenFields: Arc<metamodelica::List<ArcStr>> = writtenFields;
    let mut group: Arc<metamodelica::List<(ArcStr, Arc<DAE::Exp>, Arc<DAE::Statement>)>> = group;
    let mut extended: bool = false;
    let mut stmt: Arc<DAE::Statement>;
    let mut m: Option<(Arc<DAE::Exp>, Arc<DAE::Type>, ArcStr, Arc<DAE::Exp>)> = None;
    let mut base2: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut t2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut f: ArcStr = arcstr::literal!("");
    if rest.clone().is_empty() {
        extended = false;
        return Ok((writtenFields, group, extended));
    }
    stmt = listHead(rest.clone())?;
    m = optMRFAMatch(stmt.clone());
    extended = (::match_deref::match_deref! { match &(m.clone()) {
        Some((base2, t2, f, rhs)) => optMRFACheckExtend(base2.clone(), t2.clone(), (f.clone()).clone(), rhs.clone(), baseExp.clone(), mrecTy.clone(), writtenFields.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if extended.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(m.clone()) {
            Some((_, _, __pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        f = __pa0.clone();
        rhs = __pa1.clone();
        group = cons((f.clone(), rhs.clone(), stmt.clone()), group.clone());
        writtenFields = cons(f.clone(), writtenFields.clone());
    }
    Ok((writtenFields, group, extended))
}

fn optMRFACheckExtend(mut newBase: Arc<DAE::Exp>, mut newMrecTy: Arc<DAE::Type>, mut newField: ArcStr, mut newRhs: Arc<DAE::Exp>, mut baseExp: Arc<DAE::Exp>, mut mrecTy: Arc<DAE::Type>, mut writtenFields: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut ok: bool = false;
    ok = ExpressionBasics::expEqual(newBase.clone(), baseExp.clone())? && !(listMember((newField.clone()).clone(), writtenFields.clone())) && optMRFARhsSafe(newRhs.clone(), baseExp.clone(), writtenFields.clone())?;
    Ok(ok)
}

fn optMRFARhsSafe(mut rhs: Arc<DAE::Exp>, mut baseExp: Arc<DAE::Exp>, mut writtenFields: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut safe: bool = false;
    let mut acc: (Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>, bool);
    let (_, (_, _, __pa0)) = Expression::traverseExpTopDown(rhs.clone(), Arc::new(optMRFARhsCheck), (baseExp.clone(), writtenFields.clone(), true))?;
    safe = __pa0.clone();
    Ok(safe)
}

fn optMRFARhsCheck(mut inExp: Arc<DAE::Exp>, mut inAcc: (Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>, bool))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool = false;
    let mut outAcc: (Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>, bool);
    let mut baseExp: Arc<DAE::Exp>;
    let mut innerExp: Arc<DAE::Exp>;
    let mut writtenFields: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut safe: bool = false;
    let mut handled: bool = false;
    let mut fname: ArcStr = arcstr::literal!("");
    (baseExp, writtenFields, safe) = inAcc.clone();
    if !(safe.clone()) {
        cont = false;
        outAcc = inAcc.clone();
        return Ok((outExp, cont, outAcc));
    }
    handled = false;
    (safe, handled) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::RSUB { fieldName: fname, exp: innerExp, .. } if (ExpressionBasics::expEqual(innerExp.clone(), baseExp.clone())?) => (!(listMember((fname.clone()).clone(), writtenFields.clone())), true),
        Deref @ DAE::Exp::CREF { .. } => (optMRFACheckCrefRead(inExp.clone(), baseExp.clone(), writtenFields.clone())?, false),
        _ => (safe.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cont = safe.clone() && !(handled.clone());
    outAcc = (baseExp.clone(), writtenFields.clone(), safe.clone());
    Ok((outExp, cont, outAcc))
}

fn optMRFACheckCrefRead(mut crefExp: Arc<DAE::Exp>, mut baseExp: Arc<DAE::Exp>, mut writtenFields: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut safe: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut baseCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut baseIdent: ArcStr = arcstr::literal!("");
    let mut headIdent: ArcStr = arcstr::literal!("");
    let mut fname: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(crefExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cref = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(baseExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    baseCref = __pa1.clone();
    safe = 'mc: {
        let __mc_input = (cref.clone(), baseCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: headIdent, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: baseIdent, .. }) => {
                    Ok(!(stringEq((headIdent.clone()).clone(), (baseIdent.clone()).clone()) && !(writtenFields.clone().is_empty())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: fname, .. }, ident: headIdent, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: baseIdent, .. }) => {
                    Ok(!(stringEq((headIdent.clone()).clone(), (baseIdent.clone()).clone()) && listMember((fname.clone()).clone(), writtenFields.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: headIdent, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: baseIdent, .. }) => {
                    Ok(!(stringEq((headIdent.clone()).clone(), (baseIdent.clone()).clone()) && !(writtenFields.clone().is_empty())))
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
    Ok(safe)
}

