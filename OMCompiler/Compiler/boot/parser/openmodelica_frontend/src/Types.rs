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
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionSimplify;
use crate::Patternm;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type Binding = Arc<DAE::Binding>;

pub type Const = DAE::Const;

pub type EqualityConstraint = Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>;

pub type FuncArg = Arc<DAE::FuncArg>;

pub type Properties = DAE::Properties;

pub type TupleConst = Arc<DAE::TupleConst>;

pub type Type = Arc<DAE::Type>;

pub type Var = Arc<DAE::Var>;

pub type EqMod = DAE::EqMod;

pub fn discreteType(mut inType: Arc<DAE::Type>) -> Result<()> {
    let true = (isDiscreteType(inType.clone())) else { bail!("pattern mismatch") };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isDiscreteType(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsDiscrete: bool = false;
    outIsDiscrete = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => true,
        Deref @ DAE::Type::T_STRING { .. } => true,
        Deref @ DAE::Type::T_BOOL { .. } => true,
        Deref @ DAE::Type::T_CLOCK { .. } => true,
        Deref @ DAE::Type::T_ENUMERATION { .. } => true,
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => isDiscreteType(var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsDiscrete
}

pub fn propsAnd(mut inProps: Arc<metamodelica::List<DAE::Properties>>) -> Result<DAE::Properties> {
    let mut outProp: DAE::Properties;
    outProp = 'mc: {
        let __mc_input = inProps.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: prop, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(prop.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: DAE::Properties::PROP { type_: ty, constFlag: c }, tail: props } => {
                    let mut c2: Const = DAE::Const::C_CONST;
                    let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c = (*c).clone();
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (propsAnd(props.clone())?) else { bail!("pattern mismatch") };
                    ty2 = __pa0.clone();
                    c2 = __pa1.clone();
                    c = constAnd(c.clone(), c2.clone());
                    let true = (equivtypes(ty.clone(), ty2.clone())?) else { bail!("pattern mismatch") };
                    Ok(DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProp)
}

pub fn makePropsNotConst(mut inProperties: DAE::Properties) -> Result<DAE::Properties> {
    let mut outProperties: DAE::Properties;
    outProperties = (match inProperties.clone() {
        DAE::Properties::PROP { type_: ref t, .. } => {
            DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outProperties)
}

// stefan
pub fn getConstList(mut inPropertiesList: Arc<metamodelica::List<DAE::Properties>>) -> Result<Arc<metamodelica::List<DAE::Const>>> {
    let mut outConstList: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
    outConstList = (::match_deref::match_deref! { match &(inPropertiesList.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: DAE::Properties::PROP { constFlag: c, .. }, tail: pcdr } => {
            let mut ccdr: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
            ccdr = getConstList(pcdr.clone())?;
            cons(c.clone(), ccdr.clone())
        },
        Deref @ metamodelica::List::Cons { head: DAE::Properties::PROP_TUPLE { tupleConst: tc, .. }, tail: pcdr } => {
            let mut c: Const = DAE::Const::C_CONST;
            let mut ccdr: Arc<metamodelica::List<DAE::Const>> = metamodelica::nil();
            c = propertiesListToConst2(tc.clone())?;
            ccdr = getConstList(pcdr.clone())?;
            cons(c.clone(), ccdr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outConstList)
}

pub fn propertiesListToConst(mut p: Arc<metamodelica::List<DAE::Properties>>) -> Result<DAE::Const> {
    let mut c: DAE::Const = DAE::Const::C_CONST;
    c = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ metamodelica::List::Nil => {
            openmodelica_frontend_types::DAE::Const::C_CONST
        },
        Deref @ metamodelica::List::Cons { head: DAE::Properties::PROP { type_: _, constFlag: c1 }, tail: pps } => {
            let mut c2: Const = DAE::Const::C_CONST;
            let mut c1 = (*c1).clone();
            c2 = propertiesListToConst(pps.clone())?;
            c1 = constAnd(c1.clone(), c2.clone());
            c1.clone()
        },
        Deref @ metamodelica::List::Cons { head: DAE::Properties::PROP_TUPLE { type_: _, tupleConst: tc1 }, tail: pps } => {
            let mut c1: Const = DAE::Const::C_CONST;
            let mut c2: Const = DAE::Const::C_CONST;
            c1 = propertiesListToConst2(tc1.clone())?;
            c2 = propertiesListToConst(pps.clone())?;
            c1 = constAnd(c1.clone(), c2.clone());
            c1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(c)
}

fn propertiesListToConst2(mut t: Arc<DAE::TupleConst>) -> Result<DAE::Const> {
    let mut c: DAE::Const = DAE::Const::C_CONST;
    c = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c1 } => {
            c1.clone()
        },
        Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: tc1, tail: tcxl } } => {
            let mut c1: Const = DAE::Const::C_CONST;
            let mut c2: Const = DAE::Const::C_CONST;
            c1 = propertiesListToConst2(tc1.clone())?;
            c2 = tupleConstListToConst(tcxl.clone())?;
            c1 = constAnd(c1.clone(), c2.clone());
            c1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(c)
}

pub fn tupleConstListToConst(mut t: Arc<metamodelica::List<Arc<DAE::TupleConst>>>) -> Result<DAE::Const> {
    let mut c: DAE::Const = DAE::Const::C_CONST;
    c = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ metamodelica::List::Nil => {
            openmodelica_frontend_types::DAE::Const::C_CONST
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c1 }, tail: tcxl } => {
            let mut c2: Const = DAE::Const::C_CONST;
            let mut c1 = (*c1).clone();
            c2 = tupleConstListToConst(tcxl.clone())?;
            c1 = constAnd(c1.clone(), c2.clone());
            c1.clone()
        },
        Deref @ metamodelica::List::Cons { head: p1 @ Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: _ }, tail: tcxl } => {
            let mut c1: Const = DAE::Const::C_CONST;
            let mut c2: Const = DAE::Const::C_CONST;
            c1 = propertiesListToConst2(p1.clone())?;
            c2 = tupleConstListToConst(tcxl.clone())?;
            c1 = constAnd(c1.clone(), c2.clone());
            c1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(c)
}

pub fn externalObjectType(mut inType: Arc<DAE::Type>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn varBinding(mut inVar: Arc<DAE::Var>) -> Result<Arc<DAE::Binding>> {
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let __pa0 = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { binding: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outBinding = __pa0.clone();
    Ok(outBinding)
}

pub fn varEqualName(mut inVar1: Arc<DAE::Var>, mut inVar2: Arc<DAE::Var>) -> Result<bool> {
    let mut outEqual: bool = false;
    let mut name1: ArcStr = arcstr::literal!("");
    let mut name2: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inVar1.clone()) {
        Deref @ DAE::Var { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(inVar2.clone()) {
        Deref @ DAE::Var { name: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name2 = __pa1.clone();
    outEqual = name1.clone() == name2.clone();
    Ok(outEqual)
}

pub fn externalObjectConstructorType(mut inType: Arc<DAE::Type>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcResultType: tp, .. } => {
            externalObjectType(tp.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn simpleType(mut inType: Arc<DAE::Type>) -> Result<()> {
    let true = (isSimpleType(inType.clone())) else { bail!("pattern mismatch") };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isSimpleType(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            true
        },
        Deref @ DAE::Type::T_INTEGER { .. } => {
            true
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            true
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            true
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            true
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            true
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
            isSimpleType(t.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: t, .. } => {
            isSimpleType(t.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isSimpleNumericType(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            true
        },
        Deref @ DAE::Type::T_INTEGER { .. } => {
            true
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
            isSimpleNumericType(t.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: t, .. } => {
            isSimpleNumericType(t.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isNumericType(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBool: bool = false;
    outBool = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, .. } => {
            isNumericType(ty.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            isNumericType(ty.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
            isNumericType(ty.clone())
        },
        _ => {
            isSimpleNumericType(inType.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBool
}

pub fn isConnector(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsConnector: bool = false;
    outIsConnector = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { .. }, .. } => true,
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ClassInf::State::CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsConnector
}

pub fn isComplexConnector(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsComplexConnector: bool = false;
    outIsComplexConnector = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsComplexConnector
}

pub fn isComplexExpandableConnector(mut inType: Arc<DAE::Type>) -> bool {
    let mut outResult: bool = false;
    outResult = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { isExpandable: true, .. }, .. } => true,
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ClassInf::State::CONNECTOR { isExpandable: true, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isComplexType(mut ity: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ity.clone()) {
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            isComplexType(ty.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
            isComplexType(ty.clone())
        },
        Deref @ DAE::Type::T_COMPLEX { varLst: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isExternalObject(mut tp: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn expTypetoTypesType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut oType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    oType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: at, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    ty = expTypetoTypesType(at.clone())?;
                    tty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] });
                    Ok(tty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: at, dims: Deref @ metamodelica::List::Cons { head: dim, tail: ad } } => {
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    ty = expTypetoTypesType(Arc::new(DAE::Type::T_ARRAY { ty: at.clone(), dims: ad.clone() }))?;
                    tty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] });
                    Ok(tty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: CIS, varLst: vars, equalityConstraint: ec, .. } => {
                    let mut vars = (*vars).clone();
                    vars = List::map(vars.clone(), (std::sync::Arc::new(convertFromExpToTypesVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Var>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: CIS.clone(), varLst: vars.clone(), equalityConstraint: ec.clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: CIS, varLst: vars, complexType: ty, equalityConstraint: ec } => {
                    let mut vars = (*vars).clone();
                    let mut ty = (*ty).clone();
                    vars = List::map(vars.clone(), (std::sync::Arc::new(convertFromExpToTypesVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Var>> + 'static>));
                    ty = expTypetoTypesType(ty.clone())?;
                    Ok(Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: CIS.clone(), varLst: vars.clone(), complexType: ty.clone(), equalityConstraint: ec.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty } => {
                    let mut ty = (*ty).clone();
                    ty = expTypetoTypesType(ty.clone())?;
                    Ok(Arc::new(DAE::Type::T_METABOXED { ty: ty.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oType)
}

fn convertFromExpToTypesVar(mut inVar: Arc<DAE::Var>) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    outVar = inVar.clone();
    assign_field!(outVar.ty = expTypetoTypesType(inVar.ty.clone())?);
    Ok(outVar)
}

pub fn isTuple(mut tp: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_TUPLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isMetaTuple(mut tp: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_METATUPLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isRecord(mut tp: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn recordHasConstVar(mut ty: Arc<DAE::Type>) -> Result<bool> {
    let mut hasConstType: bool = false;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } => {
            for mut var in &*var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone() {
                let mut var = var.clone();
                if DAEUtil::isConstVar(var.clone()) {
                    hasConstType = true;
                    break;
                }
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.recordHasConstVar")); __mm_s.push_str(&*literal!(" failed because input type is not a record.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasConstType)
}

pub fn getRecordPath(mut tp: Arc<DAE::Type>) -> Result<Arc<Absyn::Path>> {
    let mut p: Arc<Absyn::Path>;
    p = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p }, .. } => p.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(p)
}

pub fn isRecordWithOnlyReals(mut tp: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. } => {
            List::all(List::map(varLst.clone(), (std::sync::Arc::new(getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>)), (std::sync::Arc::new(fnptr!(isReal, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn getVarType(mut v: Arc<DAE::Var>) -> Result<Arc<DAE::Type>> {
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    tp = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Var { ty: tp, .. } => tp.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Types.getVarType failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tp)
}

pub fn varIsVariable(mut v: Arc<DAE::Var>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { variability: SCode::Variability::VAR, .. }, .. } => true,
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { variability: SCode::Variability::DISCRETE, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isReal(mut tp: Arc<DAE::Type>) -> bool {
    let mut res: bool = false;
    res = isScalarReal(arrayElementType(tp.clone()));
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isScalarReal(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsScalarReal: bool = false;
    outIsScalarReal = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            true
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            isScalarReal(ty.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsScalarReal
}

pub fn isRealOrSubTypeReal(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    let mut lb1: bool = false;
    let mut lb2: bool = false;
    lb1 = isReal(inType.clone());
    lb2 = equivtypes(inType.clone(), DAE::T_REAL_DEFAULT().clone())?;
    b = lb1.clone() || lb2.clone();
    Ok(b)
}

pub fn isIntegerOrSubTypeInteger(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    let mut lb1: bool = false;
    let mut lb2: bool = false;
    lb1 = isInteger(inType.clone());
    lb2 = equivtypes(inType.clone(), DAE::T_INTEGER_DEFAULT().clone())?;
    b = lb1.clone() || lb2.clone();
    Ok(b)
}

pub fn isEnumerationOrSubTypeEnumeration(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    let mut lb1: bool = false;
    let mut lb2: bool = false;
    lb1 = isEnumeration(inType.clone());
    lb2 = equivtypes(inType.clone(), DAE::T_ENUMERATION_DEFAULT().clone())?;
    b = lb1.clone() || lb2.clone();
    Ok(b)
}

fn isClockOrSubTypeClock1(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    let mut lb1: bool = false;
    let mut lb2: bool = false;
    let mut lb3: bool = false;
    lb1 = isClock(inType.clone());
    lb2 = equivtypes(inType.clone(), DAE::T_CLOCK_DEFAULT().clone())?;
    lb3 = !(equivtypes(inType.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?);
    b = lb1.clone() || lb2.clone() && lb3.clone();
    Ok(b)
}

pub fn isClockOrSubTypeClock(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
            isClockOrSubTypeClock1(ty.clone())?
        },
        _ => {
            isClockOrSubTypeClock1(inType.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isBooleanOrSubTypeBoolean(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    let mut lb1: bool = false;
    let mut lb2: bool = false;
    lb1 = isBoolean(inType.clone());
    lb2 = equivtypes(inType.clone(), DAE::T_BOOL_DEFAULT().clone())?;
    b = lb1.clone() || lb2.clone();
    Ok(b)
}

pub fn isStringOrSubTypeString(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    let mut lb1: bool = false;
    let mut lb2: bool = false;
    lb1 = isString(inType.clone());
    lb2 = equivtypes(inType.clone(), DAE::T_STRING_DEFAULT().clone())?;
    b = lb1.clone() || lb2.clone();
    Ok(b)
}

pub fn isIntegerOrRealOrSubTypeOfEither(mut t: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(t.clone()) {
        _ if (isRealOrSubTypeReal(t.clone())?) => true,
        _ if (isIntegerOrSubTypeInteger(t.clone())?) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isIntegerOrRealOrBooleanOrSubTypeOfEither(mut t: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(t.clone()) {
        _ if (isRealOrSubTypeReal(t.clone())?) => true,
        _ if (isIntegerOrSubTypeInteger(t.clone())?) => true,
        _ if (isBooleanOrSubTypeBoolean(t.clone())?) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isClock(mut tp: Arc<DAE::Type>) -> bool {
    let mut res: bool = false;
    res = isScalarClock(arrayElementType(tp.clone()));
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isScalarClock(mut inType: Arc<DAE::Type>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_CLOCK { .. } => {
            true
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            isScalarClock(ty.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isInteger(mut tp: Arc<DAE::Type>) -> bool {
    let mut res: bool = false;
    res = isScalarInteger(arrayElementType(tp.clone()));
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isScalarInteger(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsScalarInteger: bool = false;
    outIsScalarInteger = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            true
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            isScalarInteger(ty.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsScalarInteger
}

pub fn isBoolean(mut tp: Arc<DAE::Type>) -> bool {
    let mut res: bool = false;
    res = isScalarBoolean(arrayElementType(tp.clone()));
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isScalarBoolean(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsScalarBoolean: bool = false;
    outIsScalarBoolean = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_BOOL { .. } => {
            true
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            isScalarBoolean(ty.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsScalarBoolean
}

pub fn integerOrReal(mut inType: Arc<DAE::Type>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            ()
        },
        Deref @ DAE::Type::T_INTEGER { .. } => {
            ()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
            integerOrReal(tp.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isNonscalarArray(mut inType: Arc<DAE::Type>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inType.clone(), inDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { .. }, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, _) => {
                    Ok(isNonscalarArray(t.clone(), metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, _) => {
                    let mut b: bool = false;
                    b = List::applyAndFold1(tys.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(isNonscalarArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<bool> + 'static>), metamodelica::nil(), false);
                    Ok(b.clone())
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
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isArray(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsArray: bool = false;
    outIsArray = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => true,
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => isArray(var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone()),
        Deref @ DAE::Type::T_FUNCTION { .. } => isArray(var_field!((*inType).funcResultType, DAE::Type::T_FUNCTION).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsArray
}

pub fn isEmptyArray(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, tail: Deref @ metamodelica::List::Nil }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isString(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_STRING { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isEnumeration(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(arrayElementType(inType.clone())) {
        Deref @ DAE::Type::T_ENUMERATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isArrayOrString(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inType.clone()) {
        ty if (isArray(ty.clone())) => {
            true
        },
        ty if (isString(ty.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn numberOfDimensions(mut inType: Arc<DAE::Type>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims, ty: t } => {
                    let mut n: i32 = 0;
                    n = numberOfDimensions(t.clone())?;
                    n = n.clone() + (dims.clone().len() as i32);
                    Ok(n.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                    let mut n: i32 = 0;
                    n = numberOfDimensions(t.clone())?;
                    Ok(n.clone())
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
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dimensionsKnown(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut outRes: bool = false;
    outRes = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Cons { head: d, tail: dims } } => {
                    let true = (Expression::dimensionKnown(d.clone())) else { bail!("pattern mismatch") };
                    let true = (dimensionsKnown(Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: dims.clone() }))?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Nil } => {
                    let true = (dimensionsKnown(tp.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { .. } => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
                    Ok(dimensionsKnown(tp.clone())?)
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
    Ok(outRes)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getDimensionSizes(mut inType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIntegerLst = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Cons { head: d, tail: dims } } => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut i: i32 = 0;
                    i = Expression::dimensionSize(d.clone())?;
                    res = getDimensionSizes(Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: dims.clone() }))?;
                    Ok(cons(i.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Cons { head: _, tail: dims } } => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    res = getDimensionSizes(Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: dims.clone() }))?;
                    Ok(cons(0, res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Nil } => {
                    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    res = getDimensionSizes(tp.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
                    Ok(getDimensionSizes(tp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (arrayType(inType.clone())) else { bail!("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIntegerLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getDimensionProduct(mut inType: Arc<DAE::Type>) -> Result<i32> {
    let mut sz: i32 = 0;
    sz = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: tp, dims } => {
            {
        let mut __acc: i32 = 1;
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = Expression::dimensionSize(d.clone())?;
            __acc *= __x;
        }
        __acc
    } * getDimensionProduct(tp.clone())?
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
            getDimensionProduct(tp.clone())?
        },
        _ => {
            let false = (arrayType(inType.clone())) else { bail!("pattern mismatch") };
            1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sz)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getDimensionNth(mut inType: Arc<DAE::Type>, mut inDim: i32) -> Result<Arc<DAE::Dimension>> {
    let mut outDimension: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    outDimension = 'mc: {
        let __mc_input = (inType.clone(), inDim.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims, .. }, d) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    dim = (dims.clone()).get(d.clone())?;
                    Ok(dim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims, ty: t }, d) => {
                    let mut dc: i32 = 0;
                    dc = (dims.clone().len() as i32);
                    let true = (d.clone() > dc.clone()) else { bail!("pattern mismatch") };
                    Ok(getDimensionNth(t.clone(), d.clone() - dc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, d) => {
                    Ok(getDimensionNth(t.clone(), d.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDimension)
}

pub fn setDimensionNth(mut inType: Arc<DAE::Type>, mut inDim: Arc<DAE::Dimension>, mut inDimNth: i32) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inType.clone(), inDim.clone(), inDimNth.clone())) {
        (Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _, 1) => {
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![inDim.clone()] })
        },
        (Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, _, _) => {
            let mut ty = (*ty).clone();
            let true = (inDimNth.clone() > 1) else { bail!("pattern mismatch") };
            ty = setDimensionNth(ty.clone(), inDim.clone(), inDimNth.clone() - 1)?;
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

pub fn valuesToVars(mut inValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut inExpIdentLst: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut outVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    outVarLst = 'mc: {
        let __mc_input = (inValuesValueLst.clone(), inExpIdentLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v, tail: vs }, Deref @ metamodelica::List::Cons { head: id, tail: ids }) => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut rest: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    tp = typeOfValue(v.clone())?;
                    rest = valuesToVars(vs.clone(), ids.clone())?;
                    Ok(cons(Arc::new(DAE::Var { name: (id.clone()).clone(), attributes: DAE::dummyAttrVar().clone(), ty: tp.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-values_to_vars failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn typeOfValue(mut inValue: Arc<Values::Value>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::EMPTY { ty: valType, .. } => {
                    Ok(typeOfValue(valType.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::INTEGER { .. } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::REAL { .. } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::STRING { .. } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { .. } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { index, name: path } => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::pathPrefix(path.clone())?;
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: Some(index.clone()), path: path.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: v, tail: vs }, .. } => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dim1: i32 = 0;
                    tp = typeOfValue(v.clone())?;
                    dim1 = (cons(v.clone(), vs.clone()).len() as i32);
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim1.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. } => {
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::TUPLE { valueLst: vs } => {
                    let mut ts: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    ts = List::map(vs.clone(), (std::sync::Arc::new(typeOfValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_TUPLE { types: ts.clone(), names: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { index: (-1), comp: ids, orderd: vl, record_: cname } => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    vars = valuesToVars(vl.clone(), ids.clone())?;
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: cname.clone() }, varLst: vars.clone(), equalityConstraint: None, usedExternally: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { index, comp: ids, orderd: vl, record_: cname } => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut utPath: Arc<Absyn::Path>;
                    let true = (index.clone() >= 0) else { bail!("pattern mismatch") };
                    vars = valuesToVars(vl.clone(), ids.clone())?;
                    utPath = AbsynUtil::stripLast(cname.clone())?;
                    Ok(Arc::new(DAE::Type::T_METARECORD { path: cname.clone(), utPath: utPath.clone(), typeVars: metamodelica::nil(), index: index.clone(), fields: vars.clone(), knownSingleton: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::LIST { valueLst: vl } => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ts: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    explist = List::map(vl.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| ValuesUtil::valueExp(__pe_a0, __pe_b1.clone()) }));
                    ts = List::map(vl.clone(), (std::sync::Arc::new(typeOfValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>));
                    (_, tp) = listMatchSuperType(explist.clone(), ts.clone(), true)?;
                    tp = boxIfUnboxedType(tp.clone())?;
                    Ok(Arc::new(DAE::Type::T_METALIST { ty: tp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: None } => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    tp = Arc::new(DAE::Type::T_METAOPTION { ty: DAE::T_UNKNOWN_DEFAULT().clone() });
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: Some(v) } => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    tp = boxIfUnboxedType(typeOfValue(v.clone())?)?;
                    tp = Arc::new(DAE::Type::T_METAOPTION { ty: tp.clone() });
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_TUPLE { valueLst: vs } => {
                    let mut ts: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    ts = List::mapMap(vs.clone(), (std::sync::Arc::new(typeOfValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METATUPLE { types: ts.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: v, tail: _ } } => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    tp = boxIfUnboxedType(typeOfValue(v.clone())?)?;
                    tp = Arc::new(DAE::Type::T_METAARRAY { ty: tp.clone() });
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_ARRAY { valueLst: Deref @ metamodelica::List::Nil } => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    tp = Arc::new(DAE::Type::T_METAARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone() });
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_BOX { value: v } => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    tp = typeOfValue(v.clone())?;
                    Ok(boxIfUnboxedType(tp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::NORETCALL => {
                    Ok(DAE::T_NORETCALL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { .. } } => {
                    Ok(Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_TYPENAME }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { .. } } => {
                    Ok(Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { .. } } => {
                    Ok(Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { .. } } => {
                    Ok(Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_MODIFICATION }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.typeOfValue failed: ")); __mm_s.push_str(&*ValuesDump::valString(v.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn basicType(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => true,
        Deref @ DAE::Type::T_REAL { .. } => true,
        Deref @ DAE::Type::T_STRING { .. } => true,
        Deref @ DAE::Type::T_BOOL { .. } => true,
        Deref @ DAE::Type::T_CLOCK { .. } => true,
        Deref @ DAE::Type::T_ENUMERATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn extendsBasicType(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn derivedBasicType(mut inType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => derivedBasicType(var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone()),
        _ => inType.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn arrayType(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn setVarInput(mut var: Arc<DAE::Var>) -> Arc<DAE::Var> {
    let mut outV: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    let mut attrs: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    outV = var.clone();
    attrs = outV.attributes.clone();
    assign_field!(attrs.direction = openmodelica_ast::Absyn::Direction::INPUT);
    assign_field!(outV.attributes = attrs.clone());
    outV
}

pub fn setVarDefaultInput(mut var: Arc<DAE::Var>) -> Arc<DAE::Var> {
    let mut outV: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    let mut attrs: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    outV = var.clone();
    attrs = outV.attributes.clone();
    assign_field!(
        attrs.connectorType = Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR),
        attrs.variability = openmodelica_frontend_types::SCode::Variability::VAR,
        attrs.direction = openmodelica_ast::Absyn::Direction::INPUT,
        attrs.innerOuter = openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        attrs.visibility = openmodelica_frontend_types::SCode::Visibility::PUBLIC
    );
    assign_field!(outV.attributes = attrs.clone());
    outV
}

pub fn setVarProtected(mut var: Arc<DAE::Var>) -> Arc<DAE::Var> {
    let mut outV: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    let mut attrs: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    outV = var.clone();
    attrs = outV.attributes.clone();
    assign_field!(attrs.visibility = openmodelica_frontend_types::SCode::Visibility::PROTECTED);
    assign_field!(outV.attributes = attrs.clone());
    outV
}

fn setVarType(mut var: Arc<DAE::Var>, mut ty: Arc<DAE::Type>) -> Arc<DAE::Var> {
    let mut outV: Arc<DAE::Var> = var.clone();
    assign_field!(outV.ty = ty.clone());
    outV
}

pub fn semiEquivTypes(mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>) -> Result<bool> {
    let mut outEquiv: bool = false;
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    if arrayType(inType1.clone()) && arrayType(inType2.clone()) {
        (ty1, dims1) = TypesDump::flattenArrayType(inType1.clone());
        (ty2, dims2) = TypesDump::flattenArrayType(inType2.clone());
        outEquiv = equivtypes(inType1.clone(), inType2.clone())? && (dims1.clone().len() as i32) == (dims2.clone().len() as i32);
    } else if !(arrayType(inType1.clone())) && !(arrayType(inType2.clone())) {
        outEquiv = equivtypes(inType1.clone(), inType2.clone())?;
    } else {
        outEquiv = false;
    }
    Ok(outEquiv)
}

pub fn equivtypes(mut t1: Arc<DAE::Type>, mut t2: Arc<DAE::Type>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = subtype(t1.clone(), t2.clone(), true)? && subtype(t2.clone(), t1.clone(), true)?;
    Ok(outBoolean)
}

pub fn equivtypesOrRecordSubtypeOf(mut t1: Arc<DAE::Type>, mut t2: Arc<DAE::Type>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = subtype(t1.clone(), t2.clone(), false)? && subtype(t2.clone(), t1.clone(), false)?;
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn subtype(mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>, mut requireRecordNamesEqual: bool) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inType1.clone(), inType2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ANYTYPE { .. }, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_ANYTYPE { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_STRING { .. }, Deref @ DAE::Type::T_STRING { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_BOOL { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_CLOCK { .. }, Deref @ DAE::Type::T_CLOCK { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { names: Deref @ metamodelica::List::Nil, .. }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { .. }, Deref @ DAE::Type::T_ENUMERATION { names: Deref @ metamodelica::List::Nil, .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { names: names1, .. }, Deref @ DAE::Type::T_ENUMERATION { names: names2, .. }) => {
                    let mut res: bool = false;
                    res = List::isEqualOnTrue(names1.clone(), names2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>));
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, dims: dlst1 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } }, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: dlst2 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } }) => {
                    let true = (Expression::dimsEqual(dlst1.clone(), dlst2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: dlst2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } }) => {
                    let true = (Expression::dimensionsEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subtype(t1.clone(), Arc::new(DAE::Type::T_ARRAY { ty: t2.clone(), dims: dlst2.clone() }), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: dlst1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } }, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }) => {
                    let true = (Expression::dimensionsEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subtype(Arc::new(DAE::Type::T_ARRAY { ty: t1.clone(), dims: dlst1.clone() }), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil } }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: t2, .. }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: t2, .. }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }) => {
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: p1 }, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: p2 }, .. }) => {
                    Ok(AbsynUtil::pathEqual(p1.clone(), p2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst: els1, complexClassType: st1, .. }, Deref @ DAE::Type::T_COMPLEX { varLst: els2, complexClassType: st2, .. }) => {
                    let true = (classTypeEqualIfRecord(st1.clone(), st2.clone()) || !(requireRecordNamesEqual.clone())) else { bail!("pattern mismatch") };
                    let true = ((els1.clone().len() as i32) == (els2.clone().len() as i32)) else { bail!("pattern mismatch") };
                    let true = (subtypeVarlist(els1.clone(), els2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp1, .. }, tp2) => {
                    let mut res: bool = false;
                    res = subtype(tp1.clone(), tp2.clone(), requireRecordNamesEqual.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tp1, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp2, .. }) => {
                    let mut res: bool = false;
                    res = subtype(tp1.clone(), tp2.clone(), requireRecordNamesEqual.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: type_list1, .. }, Deref @ DAE::Type::T_TUPLE { types: type_list2, .. }) => {
                    let true = (subtypeTypelist(type_list1.clone(), type_list2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METALIST { ty: t1 }, Deref @ DAE::Type::T_METALIST { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { ty: t1 }, Deref @ DAE::Type::T_METAARRAY { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: tList1 }, Deref @ DAE::Type::T_METATUPLE { types: tList2 }) => {
                    let mut res: bool = false;
                    res = subtypeTypelist(tList1.clone(), tList2.clone(), requireRecordNamesEqual.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAOPTION { ty: t1 }, Deref @ DAE::Type::T_METAOPTION { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: t1 }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2) => {
                    let true = (isBoxedType(t2.clone())) else { bail!("pattern mismatch") };
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let true = (isBoxedType(t1.clone())) else { bail!("pattern mismatch") };
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAPOLYMORPHIC { name: l1 }, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: l2 }) => {
                    Ok(l1.clone() == l2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_UNKNOWN, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_UNKNOWN) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_NORETCALL, Deref @ DAE::Type::T_NORETCALL) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcResultType: t1, funcArg: farg1, .. }, Deref @ DAE::Type::T_FUNCTION { funcResultType: t2, funcArg: farg2, .. }) => {
                    let mut tList1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tList2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    tList1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut t in (farg1.clone()).into_iter().cloned() {
                    let __x = (traverseType(funcArgType(t.clone())?, 1, std::sync::Arc::new(fnptr!(unboxedTypeTraverseHelper, Arc<DAE::Type>, _)))?).0;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    tList2 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut t in (farg2.clone()).into_iter().cloned() {
                    let __x = (traverseType(funcArgType(t.clone())?, 1, std::sync::Arc::new(fnptr!(unboxedTypeTraverseHelper, Arc<DAE::Type>, _)))?).0;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    (t1, _) = traverseType(t1.clone(), 1, std::sync::Arc::new(fnptr!(unboxedTypeTraverseHelper, Arc<DAE::Type>, _)))?;
                    (t2, _) = traverseType(t2.clone(), 1, std::sync::Arc::new(fnptr!(unboxedTypeTraverseHelper, Arc<DAE::Type>, _)))?;
                    let true = (subtypeTypelist(tList1.clone(), tList2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: t1 }, Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METARECORD { path: p1, .. }, Deref @ DAE::Type::T_METARECORD { path: p2, .. }) => {
                    Ok(AbsynUtil::pathEqual(p1.clone(), p2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { path: p1, .. }, Deref @ DAE::Type::T_METARECORD { utPath: p2, .. }) => {
                    Ok(if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) {subtypeTypelist(var_field!((*inType1).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), var_field!((*inType2).typeVars, DAE::Type::T_METARECORD).clone(), requireRecordNamesEqual.clone())?} else {false})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METARECORD { utPath: p1, knownSingleton: b1, .. }, Deref @ DAE::Type::T_METAUNIONTYPE { path: p2, knownSingleton: b2, .. }) => {
                    Ok(if (AbsynUtil::pathEqual(p1.clone(), p2.clone()) && (b1.clone() || b2.clone())) {subtypeTypelist(var_field!((*inType1).typeVars, DAE::Type::T_METARECORD).clone(), var_field!((*inType2).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), requireRecordNamesEqual.clone())?} else {false})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { path: p1, .. }, Deref @ DAE::Type::T_METAUNIONTYPE { path: p2, .. }) => {
                    Ok(if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) {subtypeTypelist(var_field!((*inType1).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), var_field!((*inType2).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), requireRecordNamesEqual.clone())?} else {false})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_CODE { ty: c1 }, Deref @ DAE::Type::T_CODE { ty: c2 }) => {
                    Ok(c1.clone() == c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATYPE { ty: t1 }, Deref @ DAE::Type::T_METATYPE { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_METATYPE { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATYPE { ty: t1 }, t2) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?)
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
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subtypeTypelist(mut inTypeLst1: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inTypeLst2: Arc<metamodelica::List<Arc<DAE::Type>>>, mut requireRecordNamesEqual: bool) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inTypeLst1.clone(), inTypeLst2.clone(), requireRecordNamesEqual.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: t1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: t2, tail: rest2 }, _) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual.clone())?) else { bail!("pattern mismatch") };
                    Ok(subtypeTypelist(rest1.clone(), rest2.clone(), requireRecordNamesEqual.clone())?)
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
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subtypeVarlist(mut inVarLst1: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inVarLst2: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inVarLst1.clone(), inVarLst2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (l, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty: t2, name: n, .. }, tail: vs }) => {
                    let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let __pa0 = ::match_deref::match_deref! { match &(varlistLookup(l.clone(), (n.clone()).clone())?) {
                        Deref @ DAE::Var { ty: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    t1 = __pa0.clone();
                    let true = (subtype(t1.clone(), t2.clone(), false)?) else { bail!("pattern mismatch") };
                    Ok(subtypeVarlist(l.clone(), vs.clone())?)
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
    Ok(outBoolean)
}

pub fn varlistLookup(mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inIdent: ArcStr) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    let mut name: ArcStr = arcstr::literal!("");
    for mut var in &*inVarLst.clone() {
        let mut var = var.clone();
        let __pa0 = ::match_deref::match_deref! { match &(var.clone()) {
            Deref @ DAE::Var { name: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        if name.clone() == inIdent.clone() {
            outVar = var.clone();
            return Ok(outVar);
        }
    }
    bail!("fail");
    Ok(outVar)
}

pub fn lookupComponent(mut inType: Arc<DAE::Type>, mut inIdent: ArcStr) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    outVar = 'mc: {
        let __mc_input = (inType.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, n) => {
                    let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
                    let true = (basicType(t.clone())) else { bail!("pattern mismatch") };
                    v = lookupInBuiltin(t.clone(), (n.clone()).clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst: cs, .. }, id) => {
                    let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
                    v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst: cs, .. }, id) => {
                    let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
                    v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { varLst: cs, .. }, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, id) => {
                    let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
                    v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
                    assign_field!(v.ty = Arc::new(DAE::Type::T_ARRAY { ty: v.ty.clone(), dims: list![dim.clone()] }));
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst: cs, .. }, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, id) => {
                    let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
                    v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
                    assign_field!(v.ty = Arc::new(DAE::Type::T_ARRAY { ty: v.ty.clone(), dims: list![dim.clone()] }));
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

fn lookupInBuiltin(mut inType: Arc<DAE::Type>, mut inIdent: ArcStr) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    outVar = (::match_deref::match_deref! { match &((inType.clone(), inIdent.clone())) {
        (Deref @ DAE::Type::T_REAL { varLst: cs }, id) => {
            let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
            v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
            v.clone()
        },
        (Deref @ DAE::Type::T_INTEGER { varLst: cs }, id) => {
            let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
            v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
            v.clone()
        },
        (Deref @ DAE::Type::T_STRING { varLst: cs }, id) => {
            let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
            v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
            v.clone()
        },
        (Deref @ DAE::Type::T_BOOL { varLst: cs }, id) => {
            let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
            v = lookupComponent2(cs.clone(), (id.clone()).clone())?;
            v.clone()
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "quantity") => {
            Arc::new(DAE::Var { name: (literal!("quantity")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(DAE::Binding::VALBOUND { valBound: Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "min") => {
            Arc::new(DAE::Var { name: (literal!("min")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(1), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("min,max")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "max") => {
            Arc::new(DAE::Var { name: (literal!("max")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: Arc::new(DAE::Type::T_ENUMERATION { index: Some(2), path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: list![(literal!("min,max")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "start") => {
            Arc::new(DAE::Var { name: (literal!("start")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_BOOL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "fixed") => {
            Arc::new(DAE::Var { name: (literal!("fixed")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_BOOL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "enable") => {
            Arc::new(DAE::Var { name: (literal!("enable")).clone(), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_BOOL_DEFAULT().clone(), binding: Arc::new(DAE::Binding::VALBOUND { valBound: Arc::new(Values::Value::BOOL { boolean: true }), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lookupComponent2(mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inIdent: ArcStr) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
    outVar = 'mc: {
        let __mc_input = (inVarLst.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v @ Deref @ DAE::Var { name: n, .. }, tail: _ }, m) => {
                    let true = (stringEq((n.clone()).clone(), (m.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: vs }, n) => {
                    let mut v: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
                    v = lookupComponent2(vs.clone(), (n.clone()).clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVar)
}

pub fn makeArray(mut inType: Arc<DAE::Type>, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inType.clone(), inArrayDim.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Nil) => {
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, l) => {
                    let mut len: i32 = 0;
                    len = (l.clone().len() as i32);
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: len.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn makeArraySubscripts(mut inType: Arc<DAE::Type>, mut lst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inType.clone(), lst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Nil) => {
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), rest.clone())?;
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: _ }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), rest.clone())?;
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: _ }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), rest.clone())?;
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() })] }), rest.clone())?;
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: _ }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] }), rest.clone())?;
                    Ok(t.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn liftArray(mut inType: Arc<DAE::Type>, mut inDimension: Arc<DAE::Dimension>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = Arc::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![inDimension.clone()] });
    outType
}

pub fn liftList(mut inType: Arc<DAE::Type>, mut inDimension: Arc<DAE::Dimension>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = Arc::new(DAE::Type::T_METALIST { ty: inType.clone() });
    outType
}

pub fn liftArrayListDims(mut inType: Arc<DAE::Type>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = inType.clone();
    for mut dim in &*inDimensions.clone().reverse() {
        let mut dim = dim.clone();
        outType = Arc::new(DAE::Type::T_ARRAY { ty: outType.clone(), dims: list![dim.clone()] });
    }
    outType
}

pub fn liftArrayListDimsReverse(mut inType: Arc<DAE::Type>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<DAE::Type> {
    let mut ty: Arc<DAE::Type> = inType.clone();
    for mut dim in &*dims.clone() {
        let mut dim = dim.clone();
        ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] });
    }
    ty
}

pub fn liftTypeWithDims(mut inType: Arc<DAE::Type>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if inDims.clone().is_empty() {
        outType = inType.clone();
        return Ok(outType);
    }
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            println!("{}", (literal!("Can not handle this yet!!")).clone());
            bail!("fail")
        },
        Deref @ DAE::Type::T_ARRAY { ty, dims } => {
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: listAppend(dims.clone(), inDims.clone()) })
        },
        _ => {
            Arc::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: inDims.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub fn liftTypeWithDimExps(mut inType: Arc<DAE::Type>, mut inDimExps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inType.clone(), inDimExps.clone())) {
        (ty, Deref @ metamodelica::List::Nil) => {
            ty.clone()
        },
        (ty, Deref @ metamodelica::List::Cons { head: d, tail: rest }) => {
            liftArray(liftTypeWithDimExps(ty.clone(), rest.clone())?, Arc::new(DAE::Dimension::DIM_EXP { exp: d.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

pub fn liftArrayRight(mut inType: Arc<DAE::Type>, mut inIntegerOption: Arc<DAE::Dimension>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inType.clone(), inIntegerOption.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, d) => {
                    let mut ty_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    ty_1 = liftArrayRight(ty.clone(), d.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: ty_1.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci, varLst: varlst, complexType: ty, equalityConstraint: ec }, d) => {
                    let mut ty_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let false = (TypesDump::getDimensions(ty.clone()).is_empty()) else { bail!("pattern mismatch") };
                    ty_1 = liftArrayRight(ty.clone(), d.clone())?;
                    Ok(Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci.clone(), varLst: varlst.clone(), complexType: ty_1.clone(), equalityConstraint: ec.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tty, d) => {
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: tty.clone(), dims: list![d.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn unliftArray(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, .. } => {
            ty.clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            unliftArray(ty.clone())?
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
            unliftArray(ty.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

pub fn unliftArrayOrList(mut inType: Arc<DAE::Type>) -> Result<(Arc<DAE::Type>, Arc<DAE::Dimension>)> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    (outType, dim) = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_METALIST { ty } => {
            (boxIfUnboxedType(ty.clone())?, Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))
        },
        Deref @ DAE::Type::T_METAARRAY { ty } => {
            (boxIfUnboxedType(ty.clone())?, Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))
        },
        Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } } => {
            (ty.clone(), dim.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            let mut ty = (*ty).clone();
            (ty, dim) = unliftArrayOrList(ty.clone())?;
            (ty.clone(), dim.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
            unliftArrayOrList(ty.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outType, dim))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn arrayElementType(mut inType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => arrayElementType(var_field!((*inType).ty, DAE::Type::T_ARRAY).clone()),
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => if (TypesDump::getDimensions(var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone()).is_empty()) {inType.clone()} else {arrayElementType(var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone())},
        Deref @ DAE::Type::T_FUNCTION { .. } => arrayElementType(var_field!((*inType).funcResultType, DAE::Type::T_FUNCTION).clone()),
        _ => inType.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn setArrayElementType(mut inType: Arc<DAE::Type>, mut inBaseType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inType.clone(), inBaseType.clone())) {
        (Deref @ DAE::Type::T_ARRAY { ty, dims }, _) => {
            let mut ty = (*ty).clone();
            ty = setArrayElementType(ty.clone(), inBaseType.clone());
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() })
        },
        _ => {
            inBaseType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn makeFunctionType(mut p: Arc<Absyn::Path>, mut vl: Arc<metamodelica::List<Arc<DAE::Var>>>, mut functionAttributes: DAE::FunctionAttributes) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut invl: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outvl: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut rettype: Type = Arc::new(DAE::Type::T_NORETCALL);
    invl = getInputVars(vl.clone());
    outvl = getOutputVars(vl.clone());
    fargs = makeFargsList(invl.clone());
    rettype = makeReturnType(outvl.clone())?;
    outType = Arc::new(DAE::Type::T_FUNCTION { funcArg: fargs.clone(), funcResultType: rettype.clone(), functionAttributes: functionAttributes.clone(), path: p.clone() });
    Ok(outType)
}

pub fn extendsFunctionTypeArgs(mut inType: Arc<DAE::Type>, mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inOutputElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inBooltLst: Arc<metamodelica::List<bool>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tysrc: Arc<Absyn::Path>;
    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut fargs1: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut newfargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut rettype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut functionAttributes: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: __pa0, funcResultType: __pa1, functionAttributes: __pa2, path: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    fargs = __pa0.clone();
    rettype = __pa1.clone();
    functionAttributes = __pa2.clone();
    tysrc = __pa3.clone();
    (fargs1, _) = List::splitOnBoolList(fargs.clone(), inBooltLst.clone())?;
    newfargs = List::threadMap(inElementLst.clone(), fargs1.clone(), (std::sync::Arc::new(makeElementFarg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::FuncArg>) -> Result<Arc<DAE::FuncArg>> + 'static>));
    newfargs = listAppend(fargs.clone(), newfargs.clone());
    outType = Arc::new(DAE::Type::T_FUNCTION { funcArg: newfargs.clone(), funcResultType: rettype.clone(), functionAttributes: functionAttributes.clone(), path: tysrc.clone() });
    Ok(outType)
}

fn makeElementReturnType(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inElementLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            Arc::new(openmodelica_frontend_types::DAE::Type::T_NORETCALL)
        },
        Deref @ metamodelica::List::Cons { head: element, tail: Deref @ metamodelica::List::Nil } => {
            let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
            ty = makeElementReturnTypeSingle(element.clone())?;
            ty.clone()
        },
        elements => {
            let mut element: Arc<DAE::Element>;
            let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut namesOpt: Option<Arc<metamodelica::List<ArcStr>>> = None;
            types = metamodelica::nil();
            names = metamodelica::nil();
            for mut element in &*elements.clone() {
                let mut element = element.clone();
                types = cons(makeElementReturnTypeSingle(element.clone())?, types.clone());
                names = cons((DAEUtil::varName(element.clone())?).clone(), names.clone());
            }
            if names.clone().is_empty() {
                namesOpt = None;
            } else {
                namesOpt = Some(names.clone().reverse());
            }
            Arc::new(DAE::Type::T_TUPLE { types: types.clone().reverse(), names: namesOpt.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn makeElementReturnTypeSingle(mut inElement: Arc<DAE::Element>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { ty, .. } => {
            ty.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getNthEnumLiteral(mut ty: Arc<DAE::Type>, mut n: i32) -> Result<Arc<DAE::Exp>> {
    let mut literalExp: Arc<DAE::Exp>;
    literalExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_ENUMERATION { .. } => Arc::new(DAE::Exp::ENUM_LITERAL { name: AbsynUtil::joinPaths(var_field!((*ty).path, DAE::Type::T_ENUMERATION).clone(), Arc::new(Absyn::Path::IDENT { name: ((var_field!((*ty).names, DAE::Type::T_ENUMERATION).clone()).get(n.clone())?).clone() }))?, index: n.clone() }),
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => getNthEnumLiteral(var_field!((*ty).complexType, DAE::Type::T_SUBTYPE_BASIC).clone(), n.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(literalExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn makeEnumerationType(mut inPath: Arc<Absyn::Path>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inPath.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_ENUMERATION { attributeLst: attrs, literalVarLst: vars, names, path: p, index: None }) => {
                    let mut attr_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut attrs = (*attrs).clone();
                    let mut vars = (*vars).clone();
                    vars = makeEnumerationType1(p.clone(), vars.clone(), names.clone(), 1)?;
                    attr_names = List::map(vars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    attrs = makeEnumerationType1(p.clone(), attrs.clone(), attr_names.clone(), 1)?;
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: names.clone(), literalVarLst: vars.clone(), attributeLst: attrs.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_ARRAY { ty, .. }) => {
                    Ok(makeEnumerationType(inPath.clone(), ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.makeEnumerationType failed on ")); __mm_s.push_str(&*TypesDump::printTypeStr(inType.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn makeEnumerationType1(mut inPath: Arc<Absyn::Path>, mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inNames: Arc<metamodelica::List<ArcStr>>, mut inIdx: i32) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut outVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    outVarLst = (::match_deref::match_deref! { match &((inPath.clone(), inVarLst.clone(), inNames.clone(), inIdx.clone())) {
        (p, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name, attributes, ty: _, binding, bind_from_outside: bsrc, constOfForIteratorRange: cnstForRange }, tail: xs }, names, idx) => {
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut var: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
            vars = makeEnumerationType1(p.clone(), xs.clone(), names.clone(), idx.clone() + 1)?;
            t = Arc::new(DAE::Type::T_ENUMERATION { index: Some(idx.clone()), path: p.clone(), names: names.clone(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() });
            var = Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: attributes.clone(), ty: t.clone(), binding: binding.clone(), bind_from_outside: bsrc.clone(), constOfForIteratorRange: cnstForRange.clone() });
            cons(var.clone(), vars.clone())
        },
        (_, Deref @ metamodelica::List::Nil, _, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVarLst)
}

fn getInputVars(mut vl: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Arc<metamodelica::List<Arc<DAE::Var>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    vl_1 = List::select(vl.clone(), (std::sync::Arc::new(isInputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
    vl_1
}

fn getOutputVars(mut vl: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Arc<metamodelica::List<Arc<DAE::Var>>> {
    let mut vl_1: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    vl_1 = List::select(vl.clone(), (std::sync::Arc::new(isOutputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
    vl_1
}

pub fn getFixedVarAttributeParameterOrConstant(mut tp: Arc<DAE::Type>) -> bool {
    let mut fix: bool = false;
    match '__try0: {
        fix = unwrap_break_err!(getFixedVarAttribute(tp.clone()), '__try0);
        Ok::<_, anyhow::Error>((fix.clone(),))
    } {
        Ok((__try0_o0,)) => {
            fix = __try0_o0;
        }
        Err(_) => {
            fix = true;
        }
    }
    fix
}

pub fn getFixedVarAttribute(mut tp: Arc<DAE::Type>) -> Result<bool> {
    let mut fixed: bool = false;
    fixed = 'mc: {
        let __mc_input = tp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::VALBOUND { valBound: Deref @ Values::Value::BOOL { boolean: fixed }, .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::BOOL { boolean: fixed }), .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::BCONST { bool: fixed }, .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::List::Cons { head: _, tail: vars } } => {
                    let mut fixed: bool = fixed.clone();
                    fixed = getFixedVarAttribute(Arc::new(DAE::Type::T_REAL { varLst: vars.clone() }))?;
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::VALBOUND { valBound: Deref @ Values::Value::BOOL { boolean: fixed }, .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::BOOL { boolean: fixed }), .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::BCONST { bool: fixed }, .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::List::Cons { head: _, tail: vars } } => {
                    let mut fixed: bool = fixed.clone();
                    fixed = getFixedVarAttribute(Arc::new(DAE::Type::T_INTEGER { varLst: vars.clone() }))?;
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::VALBOUND { valBound: Deref @ Values::Value::BOOL { boolean: fixed }, .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::BOOL { boolean: fixed }), .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::BCONST { bool: fixed }, .. }, name: Deref @ "fixed", .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::List::Cons { head: _, tail: vars } } => {
                    let mut fixed: bool = fixed.clone();
                    fixed = getFixedVarAttribute(Arc::new(DAE::Type::T_BOOL { varLst: vars.clone() }))?;
                    Ok(fixed.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, .. } => {
                    let mut result: bool = false;
                    result = getFixedVarAttribute(ty.clone())?;
                    Ok(result.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(fixed)
}

pub fn getConnectorVars(mut inType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    outVars = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { varLst: vars, complexClassType: ClassInf::State::CONNECTOR { .. }, .. } => {
            vars.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVars)
}

pub fn isInputVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { attributes: attr, .. } => {
            isInputAttr(attr.clone()) && isPublicAttr(attr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub fn isOutputVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { attributes: attr, .. } => {
            isOutputAttr(attr.clone()) && isPublicAttr(attr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub fn isInputAttr(mut inAttributes: Arc<DAE::Attributes>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inAttributes.clone()) {
        Deref @ DAE::Attributes { direction: Absyn::Direction::INPUT, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isOutputAttr(mut inAttributes: Arc<DAE::Attributes>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inAttributes.clone()) {
        Deref @ DAE::Attributes { direction: Absyn::Direction::OUTPUT, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isBidirAttr(mut inAttributes: Arc<DAE::Attributes>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inAttributes.clone()) {
        Deref @ DAE::Attributes { direction: Absyn::Direction::BIDIR, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isPublicAttr(mut inAttributes: Arc<DAE::Attributes>) -> bool {
    let mut outIsPublic: bool = false;
    outIsPublic = (::match_deref::match_deref! { match &(inAttributes.clone()) {
        Deref @ DAE::Attributes { visibility: SCode::Visibility::PUBLIC, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsPublic
}

pub fn isConstAttr(mut inAttributes: Arc<DAE::Attributes>) -> bool {
    let mut outIsPublic: bool = false;
    outIsPublic = (::match_deref::match_deref! { match &(inAttributes.clone()) {
        Deref @ DAE::Attributes { variability: SCode::Variability::CONST, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsPublic
}

pub fn isPublicVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { .. } => isPublicAttr(inVar.attributes.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub fn isConstVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { .. } => isConstAttr(inVar.attributes.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

// This used in creation of record constructors to decide wether a variable should be
// part of the constructor signature or not. If a var is modifiable from outside then
// it is part of the construvtor signature.
pub fn isModifiableTypesVar(mut inVar: Arc<DAE::Var>) -> Result<bool> {
    let mut b: bool = false;
    if !(isPublicVar(inVar.clone())?) {
        if isNone(getBindingExpOptional(inVar.clone())) {
            Error::addSourceMessage(Error::MISSING_BINDING_PROTECTED_RECORD_VAR.clone(), list![(TypesDump::getVarName(inVar.clone())?).clone()], Absyn::dummyInfo.clone())?;
        }
        b = false;
        return Ok(b);
    }
    if isConstVar(inVar.clone())? && isSome(getBindingExpOptional(inVar.clone())) {
        b = false;
        return Ok(b);
    }
    b = true;
    Ok(b)
}

pub fn getBindingExpOptional(mut inVar: Arc<DAE::Var>) -> Option<Arc<DAE::Exp>> {
    let mut outExp: Option<Arc<DAE::Exp>> = None;
    outExp = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp, .. }, .. } => {
            Some(exp.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

// This should be removed. It is used in cevalScript now. cevalScript should be updated
// and this removed.
pub fn getBindingExp(mut inVar: Arc<DAE::Var>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inVar.clone(), inPath.clone())) {
        (Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp, .. }, .. }, _) => {
            exp.clone()
        },
        (Deref @ DAE::Var { binding: Deref @ DAE::Binding::UNBOUND, name, .. }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Record '")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("' member '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' has no default value and is not modifiable by a constructor function.\n")); ArcStr::from(__mm_s) }).clone();
            Error::addCompilerWarning((r#str.clone()).clone())?;
            Arc::new(DAE::Exp::ICONST { integer: 0 })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn makeFargsList(mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Arc<metamodelica::List<Arc<DAE::FuncArg>>> {
    let mut fargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    fargs = List::map(vars.clone(), (std::sync::Arc::new(makeFarg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::FuncArg>> + 'static>));
    fargs
}

fn makeFarg(mut variable: Arc<DAE::Var>) -> Result<Arc<DAE::FuncArg>> {
    let mut farg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    farg = (::match_deref::match_deref! { match &(variable.clone()) {
        Deref @ DAE::Var { binding: bnd, ty, attributes: Deref @ DAE::Attributes { parallelism: par, variability: var, .. }, name: n, .. } => {
            let mut c: DAE::Const = DAE::Const::C_CONST;
            let mut p: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
            let mut oexp: Option<Arc<DAE::Exp>> = None;
            c = variabilityToConst(var.clone())?;
            p = DAEUtil::scodePrlToDaePrl(par.clone())?;
            oexp = DAEUtil::bindingExp(bnd.clone())?;
            Arc::new(DAE::FuncArg { name: (n.clone()).clone(), ty: ty.clone(), r#const: c.clone(), par: p.clone(), defaultBinding: oexp.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(farg)
}

fn makeElementFarg(mut inElement: Arc<DAE::Element>, mut inFarg: Arc<DAE::FuncArg>) -> Result<Arc<DAE::FuncArg>> {
    let mut farg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    farg = (::match_deref::match_deref! { match &((inElement.clone(), inFarg.clone())) {
        (Deref @ DAE::Element::VAR { componentRef: cref, .. }, _) => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (ComponentReferenceBasics::crefLastIdent(cref.clone())?).clone();
            setFuncArgName(inFarg.clone(), (name.clone()).clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(farg)
}

fn makeReturnType(mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = inVarLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Type::T_NORETCALL))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: var, tail: Deref @ metamodelica::List::Nil } => {
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    ty = makeReturnTypeSingle(var.clone())?;
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                vl => {
                    Ok(Arc::new(DAE::Type::T_TUPLE { types: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut v in (vl.clone()).into_iter().cloned() {
            let __x = makeReturnTypeSingle(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, names: Some({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (vl.clone()).into_iter().cloned() {
            let __x = TypesDump::getVarName(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn makeReturnTypeSingle(mut inVar: Arc<DAE::Var>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { ty, .. } => {
            ty.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

pub fn isParameterVar(mut inVar: Arc<DAE::Var>) -> Result<()> {
    ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { visibility: SCode::Visibility::PUBLIC, variability: SCode::Variability::PARAM, .. }, .. } => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn isConstant(mut c: DAE::Const) -> bool {
    let mut b: bool = false;
    b = (match c.clone() {
        DAE::Const::C_CONST => true,
        _ => false,
    });
    b
}

pub fn isParameter(mut c: DAE::Const) -> bool {
    let mut b: bool = false;
    b = (match c.clone() {
        DAE::Const::C_PARAM => true,
        _ => false,
    });
    b
}

pub fn isParameterOrConstant(mut c: DAE::Const) -> bool {
    let mut b: bool = false;
    b = (match c.clone() {
        DAE::Const::C_CONST => true,
        DAE::Const::C_PARAM => true,
        _ => false,
    });
    b
}

pub fn isVar(mut inConst: DAE::Const) -> bool {
    let mut outIsVar: bool = false;
    outIsVar = (match inConst.clone() {
        DAE::Const::C_VAR => true,
        _ => false,
    });
    outIsVar
}

pub fn propsContainReal(mut inProperties: Arc<metamodelica::List<DAE::Properties>>) -> Result<bool> {
    let mut outHasReal: bool = false;
    for mut prop in &*inProperties.clone() {
        let mut prop = prop.clone();
        if isReal(getPropType(prop.clone())?) {
            outHasReal = true;
            break;
        }
    }
    Ok(outHasReal)
}

pub fn containReal(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> bool {
    let mut outHasReal: bool = false;
    for mut ty in &*inTypes.clone() {
        let mut ty = ty.clone();
        if isReal(ty.clone()) {
            outHasReal = true;
            return outHasReal;
        }
    }
    outHasReal = false;
    outHasReal
}

pub fn propAllConst(mut inProperties: DAE::Properties) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = 'mc: {
        let __mc_input = inProperties.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP { constFlag: mut c, .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(c.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE { tupleConst: ref constant_, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut res: DAE::Const = DAE::Const::C_CONST;
            res = propTupleAllConst(constant_.clone())?;
            Ok(res.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut prop = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- prop_all_const failed: ")).clone())?;
            r#str = (printPropStr(prop.clone())?).clone();
            Debug::traceln((r#str.clone()).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outConst)
}

pub fn propAnyConst(mut inProperties: DAE::Properties) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = 'mc: {
        let __mc_input = inProperties.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP { constFlag: mut constant_, .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(constant_.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE { tupleConst: ref tconstant_, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut res: DAE::Const = DAE::Const::C_CONST;
            res = propTupleAnyConst(tconstant_.clone())?;
            Ok(res.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut prop = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- prop_any_const failed: ")).clone())?;
            r#str = (printPropStr(prop.clone())?).clone();
            Debug::traceln((r#str.clone()).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outConst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn propTupleAnyConst(mut inTupleConst: Arc<DAE::TupleConst>) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = 'mc: {
        let __mc_input = inTupleConst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c } => {
                    Ok(c.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: _ } } => {
                    let DAE::C_CONST { .. } = (propTupleAnyConst(first.clone())?) else { bail!("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_CONST)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: Deref @ metamodelica::List::Nil } } => {
                    let DAE::C_PARAM { .. } = (propTupleAnyConst(first.clone())?) else { bail!("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_PARAM)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: Deref @ metamodelica::List::Nil } } => {
                    let DAE::C_VAR { .. } = (propTupleAnyConst(first.clone())?) else { bail!("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_VAR)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: rest } } => {
                    let mut res: DAE::Const = DAE::Const::C_CONST;
                    let DAE::C_PARAM { .. } = (propTupleAnyConst(first.clone())?) else { bail!("pattern mismatch") };
                    res = propTupleAnyConst(Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: rest.clone() }))?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: rest } } => {
                    let mut res: DAE::Const = DAE::Const::C_CONST;
                    let DAE::C_VAR { .. } = (propTupleAnyConst(first.clone())?) else { bail!("pattern mismatch") };
                    res = propTupleAnyConst(Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: rest.clone() }))?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                r#const => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- prop_tuple_any_const failed: ")).clone())?;
                    r#str = (TypesDump::printTupleConstStr(r#const.clone())?).clone();
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outConst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn propTupleAllConst(mut inTupleConst: Arc<DAE::TupleConst>) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = 'mc: {
        let __mc_input = inTupleConst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c } => {
                    Ok(c.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: _ } } => {
                    let DAE::C_PARAM { .. } = (propTupleAllConst(first.clone())?) else { bail!("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_PARAM)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: _ } } => {
                    let DAE::C_VAR { .. } = (propTupleAllConst(first.clone())?) else { bail!("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_VAR)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: Deref @ metamodelica::List::Nil } } => {
                    let DAE::C_CONST { .. } = (propTupleAllConst(first.clone())?) else { bail!("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_CONST)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: first, tail: rest } } => {
                    let mut res: DAE::Const = DAE::Const::C_CONST;
                    let DAE::C_CONST { .. } = (propTupleAllConst(first.clone())?) else { bail!("pattern mismatch") };
                    res = propTupleAllConst(Arc::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: rest.clone() }))?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                r#const => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- prop_tuple_all_const failed: ")).clone())?;
                    r#str = (TypesDump::printTupleConstStr(r#const.clone())?).clone();
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outConst)
}

pub fn isPropTupleArray(mut p: DAE::Properties) -> Result<bool> {
    let mut ob: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    b1 = isPropTuple(p.clone())?;
    b2 = isPropArray(p.clone())?;
    ob = boolOr(b1.clone(), b2.clone());
    Ok(ob)
}

pub fn isPropTuple(mut p: DAE::Properties) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = p.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(getPropType(p.clone())?) {
                Deref @ DAE::Type::T_TUPLE { .. } => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(b)
}

pub fn isPropArray(mut p: DAE::Properties) -> Result<bool> {
    let mut b: bool = false;
    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
    t = getPropType(p.clone())?;
    b = isArray(t.clone());
    Ok(b)
}

pub fn propTupleFirstProp(mut inTupleProp: DAE::Properties) -> Result<DAE::Properties> {
    let mut outFirstProp: DAE::Properties;
    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let DAE::PROP_TUPLE { tupleConst: __t2, type_: __t3 } = (inTupleProp.clone()) else { bail!("pattern mismatch") };
    let __pa0 = ::match_deref::match_deref! { match &(__t2.clone()) {
        Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: __pa0 }, tail: _ } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    let __pa1 = ::match_deref::match_deref! { match &(__t3.clone()) {
        Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    c = __pa0.clone();
    ty = __pa1.clone();
    outFirstProp = DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() };
    Ok(outFirstProp)
}

pub fn propTuplePropList(mut prop_tuple: DAE::Properties) -> Result<Arc<metamodelica::List<DAE::Properties>>> {
    let mut prop_list: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    prop_list = (::match_deref::match_deref! { match &(prop_tuple.clone()) {
        DAE::Properties::PROP_TUPLE { tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: cl }, type_: Deref @ DAE::Type::T_TUPLE { types: tl, .. } } => {
            let mut pl: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
            pl = propTuplePropList2(tl.clone(), cl.clone())?;
            pl.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(prop_list)
}

fn propTuplePropList2(mut tl: Arc<metamodelica::List<Arc<DAE::Type>>>, mut cl: Arc<metamodelica::List<Arc<DAE::TupleConst>>>) -> Result<Arc<metamodelica::List<DAE::Properties>>> {
    let mut pl: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    pl = (::match_deref::match_deref! { match &((tl.clone(), cl.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: t, tail: t_rest }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c }, tail: c_rest }) => {
            let mut p_rest: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
            p_rest = propTuplePropList2(t_rest.clone(), c_rest.clone())?;
            cons(DAE::Properties::PROP { type_: t.clone(), constFlag: c.clone() }, p_rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(pl)
}

pub fn getPropConst(mut inProperties: DAE::Properties) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    let DAE::PROP { constFlag: __pa0, .. } = (inProperties.clone()) else { bail!("pattern mismatch") };
    outConst = __pa0.clone();
    Ok(outConst)
}

pub fn getPropType(mut inProperties: DAE::Properties) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (match inProperties.clone() {
        DAE::Properties::PROP { .. } => var_field!(inProperties.type_, DAE::Properties::PROP).clone(),
        DAE::Properties::PROP_TUPLE { .. } => var_field!(inProperties.type_, DAE::Properties::PROP_TUPLE).clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outType)
}

pub fn setPropType(mut inProperties: DAE::Properties, mut ty: Arc<DAE::Type>) -> Result<DAE::Properties> {
    let mut outProperties: DAE::Properties;
    outProperties = (match inProperties.clone() {
        DAE::Properties::PROP { .. } => DAE::Properties::PROP { type_: ty.clone(), constFlag: var_field!(inProperties.constFlag, DAE::Properties::PROP).clone() },
        DAE::Properties::PROP_TUPLE { .. } => DAE::Properties::PROP_TUPLE { type_: ty.clone(), tupleConst: var_field!(inProperties.tupleConst, DAE::Properties::PROP_TUPLE).clone() },
        _ => bail!("match: no arm matched"),
    });
    Ok(outProperties)
}

pub fn createEmptyTypeMemory() -> metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Type>, Arc<DAE::Type>)>>> {
    let mut tyMemory: metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Type>, Arc<DAE::Type>)>>>;
    tyMemory = arrayCreate(30, metamodelica::nil());
    tyMemory
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn simplifyType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outExpType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outExpType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { .. } => {
                    Ok(Arc::new(DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METARECORD { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAARRAY { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAOPTION { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATUPLE { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN => {
                    Ok(DAE::T_UNKNOWN_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ANYTYPE { .. } => {
                    Ok(DAE::T_UNKNOWN_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t @ Deref @ DAE::Type::T_ARRAY { .. } => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut t = (*t).clone();
                    (t, dims) = TypesDump::flattenArrayType(t.clone());
                    t_1 = simplifyType(t.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: dims.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                    Ok(simplifyType(t.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(DAE::T_CLOCK_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_NORETCALL => {
                    Ok(DAE::T_NORETCALL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut tys = (*tys).clone();
                    tys = List::map(tys.clone(), (std::sync::Arc::new(simplifyType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: var_field!((*inType).names, DAE::Type::T_TUPLE).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: CIS, varLst, equalityConstraint: ec, .. } => {
                    let mut varLst = (*varLst).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    varLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = simplifyVar(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: CIS.clone(), varLst: varLst.clone(), equalityConstraint: ec.clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: CIS @ ClassInf::State::RECORD { .. }, varLst, equalityConstraint: ec, .. } => {
                    let mut varLst = (*varLst).clone();
                    varLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = simplifyVar(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: CIS.clone(), varLst: varLst.clone(), equalityConstraint: ec.clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { .. } => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty: t } => {
                    let mut t_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t_1 = simplifyType(t.clone())?;
                    Ok(Arc::new(DAE::Type::T_METABOXED { ty: t_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(DAE::T_UNKNOWN_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.simplifyType failed for: ")); __mm_s.push_str(&*TypesDump::unparseType(inType.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpType)
}

fn simplifyVar(mut inVar: Arc<DAE::Var>) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = inVar.clone();
    outVar = (::match_deref::match_deref! { match &(outVar.clone()) {
        Deref @ DAE::Var { .. } => {
            assign_field!(outVar.ty = simplifyType(outVar.ty.clone())?);
            outVar.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

pub fn complicateType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = inType.clone();
    outType = (::match_deref::match_deref! { match &(outType.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            (ty, dims) = TypesDump::flattenArrayType(outType.clone());
            liftArrayListDims(ty.clone(), dims.clone())
        },
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. } => {
            var_field!((*outType).functionType, DAE::Type::T_FUNCTION_REFERENCE_VAR).clone()
        },
        Deref @ DAE::Type::T_METATYPE { .. } => {
            var_field!((*outType).ty, DAE::Type::T_METATYPE).clone()
        },
        Deref @ DAE::Type::T_TUPLE { .. } => {
            assign_variant_field!(outType => DAE::Type::T_TUPLE; types = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut t in (var_field!((*outType).types, DAE::Type::T_TUPLE).clone()).into_iter().cloned() {
            let __x = complicateType(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outType.clone()
        },
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            if isRecord(inType.clone()) || Config::acceptMetaModelicaGrammar()? {
                assign_variant_field!(outType => DAE::Type::T_COMPLEX; varLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (var_field!((*outType).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = complicateVar(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            outType.clone()
        },
        Deref @ DAE::Type::T_METABOXED { .. } => {
            assign_variant_field!(outType => DAE::Type::T_METABOXED; ty = complicateType(var_field!((*outType).ty, DAE::Type::T_METABOXED).clone())?);
            outType.clone()
        },
        _ => {
            outType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn complicateVar(mut inVar: Arc<DAE::Var>) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var> = inVar.clone();
    outVar = (::match_deref::match_deref! { match &(outVar.clone()) {
        Deref @ DAE::Var { .. } => {
            assign_field!(outVar.ty = complicateType(outVar.ty.clone())?);
            outVar.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn typeMemoryEntryEq(mut inType1: Arc<DAE::Type>, mut inType2: (Arc<DAE::Type>, Arc<DAE::Type>)) -> bool {
    let mut outEq: bool = false;
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (ty2, _) = inType2.clone();
    outEq = typesElabEquivalent(inType1.clone(), ty2.clone());
    outEq
}

pub fn typesElabEquivalent(mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>) -> bool {
    let mut isEqual: bool = false;
    match '__try0: {
        isEqual = unwrap_break_err!(ttypesElabEquivalent(inType1.clone(), inType2.clone()), '__try0);
        Ok::<_, anyhow::Error>((isEqual.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isEqual = __try0_o0;
        }
        Err(_) => {
            isEqual = false;
        }
    }
    isEqual
}

fn ttypesElabEquivalent(mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = (::match_deref::match_deref! { match &((inType1.clone(), inType2.clone())) {
        (Deref @ DAE::Type::T_COMPLEX { varLst: vars1, complexClassType: cty1, .. }, Deref @ DAE::Type::T_COMPLEX { varLst: vars2, complexClassType: cty2, .. }) => {
            let true = (AbsynUtil::pathEqual(ClassInfUtil::getStateName(cty1.clone()), ClassInfUtil::getStateName(cty2.clone()))) else { bail!("pattern mismatch") };
            let true = (List::isEqualOnTrue(vars1.clone(), vars2.clone(), (std::sync::Arc::new(varsElabEquivalent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::Var>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
            true
        },
        (Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: ad1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: ad2, tail: Deref @ metamodelica::List::Nil } }) => {
            let true = (ad1.clone() == ad2.clone()) else { bail!("pattern mismatch") };
            let true = (typesElabEquivalent(ty1.clone(), ty2.clone())) else { bail!("pattern mismatch") };
            true
        },
        (Deref @ DAE::Type::T_ENUMERATION { names: names1, path: p1, .. }, Deref @ DAE::Type::T_ENUMERATION { names: names2, path: p2, .. }) => {
            let true = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
            let true = (List::isEqualOnTrue(names1.clone(), names2.clone(), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
            true
        },
        (Deref @ DAE::Type::T_TUPLE { types: types1, .. }, Deref @ DAE::Type::T_TUPLE { types: types2, .. }) => {
            List::isEqualOnTrue(types1.clone(), types2.clone(), (std::sync::Arc::new(fnptr!(typesElabEquivalent, Arc<DAE::Type>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<bool> + 'static>))
        },
        (Deref @ DAE::Type::T_METABOXED { ty: ty1 }, Deref @ DAE::Type::T_METABOXED { ty: ty2 }) => {
            typesElabEquivalent(ty1.clone(), ty2.clone())
        },
        _ => {
            inType1.clone() == inType2.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

fn varsElabEquivalent(mut inVar1: Arc<DAE::Var>, mut inVar2: Arc<DAE::Var>) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = 'mc: {
        let __mc_input = (inVar1.clone(), inVar2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Var { ty: ty1, name: id1, .. }, Deref @ DAE::Var { ty: ty2, name: id2, .. }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (typesElabEquivalent(ty1.clone(), ty2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
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
    Ok(isEqual)
}

pub fn matchProp(mut inExp: Arc<DAE::Exp>, mut inActualType: DAE::Properties, mut inExpectedType: DAE::Properties, mut printFailtrace: bool) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = 'mc: {
        let __mc_input = (inExp.clone(), inActualType.clone(), inExpectedType.clone(), printFailtrace.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { constFlag: c1, type_: gt }, DAE::Properties::PROP { constFlag: c2, type_: et }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace.clone())?;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok((e_1.clone(), DAE::Properties::PROP { type_: t_1.clone(), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { tupleConst: tc1, type_: gt }, DAE::Properties::PROP_TUPLE { tupleConst: tc2, type_: et }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tc: TupleConst;
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace.clone())?;
                    tc = constTupleAnd(tc1.clone(), tc2.clone());
                    Ok((e_1.clone(), DAE::Properties::PROP_TUPLE { type_: t_1.clone(), tupleConst: tc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { tupleConst: tc1, type_: gt @ Deref @ DAE::Type::T_TUPLE { .. } }, DAE::Properties::PROP { constFlag: c2, type_: et @ Deref @ DAE::Type::T_METATUPLE { .. } }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let mut c_1: Const = DAE::Const::C_CONST;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace.clone())?;
                    c_1 = propTupleAllConst(tc1.clone())?;
                    c = constAnd(c_1.clone(), c2.clone());
                    Ok((e_1.clone(), DAE::Properties::PROP { type_: t_1.clone(), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { tupleConst: tc1, type_: gt @ Deref @ DAE::Type::T_TUPLE { .. } }, DAE::Properties::PROP { constFlag: c2, type_: et @ Deref @ DAE::Type::T_METABOXED { .. } }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let mut c_1: Const = DAE::Const::C_CONST;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace.clone())?;
                    c_1 = propTupleAllConst(tc1.clone())?;
                    c = constAnd(c_1.clone(), c2.clone());
                    Ok((e_1.clone(), DAE::Properties::PROP { type_: t_1.clone(), constFlag: c.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { type_: gt, .. }, DAE::Properties::PROP_TUPLE { .. }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut prop: Properties;
                    let mut gt = (*gt).clone();
                    prop = propTupleFirstProp(inExpectedType.clone())?;
                    (e_1, prop) = matchProp(e.clone(), inActualType.clone(), prop.clone(), printFailtrace.clone())?;
                    gt = simplifyType(gt.clone())?;
                    e_1 = Arc::new(DAE::Exp::TSUB { exp: e_1.clone(), ix: 1, ty: gt.clone() });
                    Ok((e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { .. }, DAE::Properties::PROP { .. }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut gt: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop: Properties;
                    let ref __pa1 @ DAE::PROP { type_: ref __pa0, .. } = (propTupleFirstProp(inActualType.clone())?) else { bail!("pattern mismatch") };
                    gt = __pa0.clone();
                    prop = __pa1.clone();
                    (e_1, prop) = matchProp(e.clone(), prop.clone(), inExpectedType.clone(), printFailtrace.clone())?;
                    gt = simplifyType(gt.clone())?;
                    e_1 = Arc::new(DAE::Exp::TSUB { exp: e_1.clone(), ix: 1, ty: gt.clone() });
                    Ok((e_1.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _, _, true) => {
                    let true = (Flags::isSet(Flags::TYPES.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.matchProp failed on exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*printPropStr(inActualType.clone())?); __mm_s.push_str(&*literal!(" != ")); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln((printPropStr(inExpectedType.clone())?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outProperties))
}

pub fn matchTypeList(mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut expType: Arc<DAE::Type>, mut expectedType: Arc<DAE::Type>, mut printFailtrace: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut expLstNew: Arc<metamodelica::List<Arc<DAE::Exp>>> = exps.clone();
    let mut exp: Arc<DAE::Exp>;
    let mut e_1: Arc<DAE::Exp>;
    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
    while !(expLstNew.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expLstNew.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
        expLstNew = __pa1.clone();
        (e_1, tp) = matchType(exp.clone(), expType.clone(), expectedType.clone(), printFailtrace.clone())?;
        outExp = cons(e_1.clone(), outExp.clone());
        outTypeLst = cons(tp.clone(), outTypeLst.clone());
    }
    outExp = outExp.clone().reverse();
    outTypeLst = outTypeLst.clone().reverse();
    Ok((outExp, outTypeLst))
}

pub fn matchTypeTuple(mut inExp1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypeLst2: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inTypeLst3: Arc<metamodelica::List<Arc<DAE::Type>>>, mut printFailtrace: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outExp, outTypeLst) = 'mc: {
        let __mc_input = (inExp1.clone(), inTypeLst2.clone(), inTypeLst3.clone(), printFailtrace.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, Deref @ metamodelica::List::Cons { head: t1, tail: ts1 }, Deref @ metamodelica::List::Cons { head: t2, tail: ts2 }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut res: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    (e_1, tp) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    (e_2, res) = matchTypeTuple(rest.clone(), ts1.clone(), ts2.clone(), printFailtrace.clone())?;
                    Ok((cons(e_1.clone(), e_2.clone()), cons(tp.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: t1, tail: _ }, Deref @ metamodelica::List::Cons { head: t2, tail: _ }, true) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.matchTypeTuple failed:")); __mm_s.push_str(&*TypesDump::TypesDump::unparseType(t1.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*TypesDump::unparseType(t2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTypeLst))
}

pub fn matchTypeTupleCall(mut inExp1: Arc<DAE::Exp>, mut inTypeLst2: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inTypeLst3: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inExp1.clone(), inTypeLst2.clone(), inTypeLst3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ metamodelica::List::Cons { head: t1, tail: ts1 }, Deref @ metamodelica::List::Cons { head: t2, tail: ts2 }) => {
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    matchTypeTupleCall(e.clone(), ts1.clone(), ts2.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- matchTypeTupleCall failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn vectorizableType(mut inExp: Arc<DAE::Exp>, mut inExpType: Arc<DAE::Type>, mut inExpectedType: Arc<DAE::Type>, mut fnPath: Option<Arc<Absyn::Path>>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Dimension>>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outArrayDimLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut outBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (outExp, outType, outArrayDimLst, outBindings) = vectorizableType2(inExp.clone(), inExpType.clone(), inExpType.clone(), metamodelica::nil(), inExpectedType.clone(), fnPath.clone())?;
    Ok((outExp, outType, outArrayDimLst, outBindings))
}

fn vectorizableType2(mut inExp: Arc<DAE::Exp>, mut inExpType: Arc<DAE::Type>, mut inCurrentType: Arc<DAE::Type>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inExpectedType: Arc<DAE::Type>, mut fnPath: Option<Arc<Absyn::Path>>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Dimension>>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut outBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    let mut vec_type: Type = Arc::new(DAE::Type::T_NORETCALL);
    let mut cur_type: Type = Arc::new(DAE::Type::T_NORETCALL);
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    match '__try0: {
        vec_type = liftArrayListDimsReverse(inExpectedType.clone(), inDims.clone());
        (outExp, outType, outBindings) = unwrap_break_err!(matchTypePolymorphic(inExp.clone(), inExpType.clone(), vec_type.clone(), fnPath.clone(), metamodelica::nil(), true), '__try0);
        outDims = inDims.clone().reverse();
        Ok::<_, anyhow::Error>((outBindings.clone(), outDims.clone(), outExp.clone(), outType.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            outBindings = __try0_o0;
            outDims = __try0_o1;
            outExp = __try0_o2;
            outType = __try0_o3;
        }
        Err(_) => {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(inCurrentType.clone()) {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, ty: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dim = __pa1.clone();
            cur_type = __pa2.clone();
            (outExp, outType, outDims, outBindings) = vectorizableType2(inExp.clone(), inExpType.clone(), cur_type.clone(), cons(dim.clone(), inDims.clone()), inExpectedType.clone(), fnPath.clone())?;
        }
    }
    Ok((outExp, outType, outDims, outBindings))
}

pub fn unflattenArrayType(mut inTy: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outTy = unflattenArrayType2(inTy.clone(), false)?;
    Ok(outTy)
}

fn unflattenArrayType2(mut inTy: Arc<DAE::Type>, mut last: bool) -> Result<Arc<DAE::Type>> {
    let mut outTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outTy = 'mc: {
        let __mc_input = (inTy.clone(), last.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci, varLst: vl, complexType: ty, equalityConstraint: eqc }, _) => {
                    let mut ty = (*ty).clone();
                    ty = unflattenArrayType(ty.clone())?;
                    Ok(Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci.clone(), varLst: vl.clone(), complexType: ty.clone(), equalityConstraint: eqc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut t = (*t).clone();
                    t = unflattenArrayType(t.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Nil }, true) => {
                    Ok(unflattenArrayType(t.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: dim, tail: dims } }, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = unflattenArrayType2(Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: dims.clone() }), true)?;
                    ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] });
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, false) => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTy)
}

fn typeConvert(mut inExp1: Arc<DAE::Exp>, mut actual: Arc<DAE::Type>, mut expected: Arc<DAE::Type>, mut printFailtrace: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outExp, outType) = 'mc: {
        let __mc_input = (inExp1.clone(), actual.clone(), expected.clone(), printFailtrace.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, ty1, ty2, _) => {
                    let true = (subtype(ty1.clone(), ty2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok((e.clone(), ty2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: ty1, tail: _ }, .. }, ty2, _) => {
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e = (*e).clone();
                    let false = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let false = (isTuple(ty2.clone())) else { bail!("pattern mismatch") };
                    let true = (subtype(ty1.clone(), ty2.clone(), true)?) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::TSUB { exp: e.clone(), ix: 1, ty: ty2.clone() });
                    ty = ty2.clone();
                    Ok((e.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }, ty2, _) => {
                    let mut ty1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e = (*e).clone();
                    let mut ty2 = (*ty2).clone();
                    ty1 = unflattenArrayType(actual.clone())?;
                    ty2 = unflattenArrayType(ty2.clone())?;
                    (e, ty) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    Ok((e.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, ty1, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }, _) => {
                    let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e = (*e).clone();
                    let mut ty1 = (*ty1).clone();
                    ty1 = unflattenArrayType(ty1.clone())?;
                    ty2 = unflattenArrayType(expected.clone())?;
                    (e, ty) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    Ok((e.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: elist, .. }, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, ty0 @ Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sc: bool = false;
                    let mut a: bool = false;
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    elist_1 = typeConvertArray(elist.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    at = simplifyType(ty0.clone())?;
                    a = isArray(ty2.clone());
                    sc = boolNot(a.clone());
                    Ok((Arc::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: sc.clone(), array: elist_1.clone() }), Arc::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: elist, .. }, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut sc: bool = false;
                    let mut a: bool = false;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut ety1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2 = (*ty2).clone();
                    let true = (Expression::dimensionKnown(dim1.clone())) else { bail!("pattern mismatch") };
                    elist_1 = typeConvertArray(elist.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    dims = Expression::arrayDimension(simplifyType(ty1.clone())?);
                    a = isArray(ty2.clone());
                    sc = boolNot(a.clone());
                    dims = cons(dim1.clone(), dims.clone());
                    ty2 = arrayElementType(ty2.clone());
                    ety1 = simplifyType(ty2.clone())?;
                    ty2 = liftArrayListDims(ty2.clone(), dims.clone());
                    Ok((Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ety1.clone(), dims: dims.clone() }), scalar: sc.clone(), array: elist_1.clone() }), ty2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { stop, step: Some(step), start: begin, .. }, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut begin_1: Arc<DAE::Exp>;
                    let mut step_1: Arc<DAE::Exp>;
                    let mut stop_1: Arc<DAE::Exp>;
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    (begin_1, _) = typeConvert(begin.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    (step_1, _) = typeConvert(step.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    (stop_1, _) = typeConvert(stop.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    at = simplifyType(Arc::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] }))?;
                    Ok((Arc::new(DAE::Exp::RANGE { ty: at.clone(), start: begin_1.clone(), step: Some(step_1.clone()), stop: stop_1.clone() }), Arc::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { stop, step: None, start: begin, .. }, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut begin_1: Arc<DAE::Exp>;
                    let mut stop_1: Arc<DAE::Exp>;
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    (begin_1, _) = typeConvert(begin.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    (stop_1, _) = typeConvert(stop.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    at = simplifyType(Arc::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] }))?;
                    Ok((Arc::new(DAE::Exp::RANGE { ty: at.clone(), start: begin_1.clone(), step: None, stop: stop_1.clone() }), Arc::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: ell, integer: nmax, .. }, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Nil } }, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, ty0 @ Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim22, tail: Deref @ metamodelica::List::Nil } }, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ell_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::dimensionsKnownAndEqual(dim11.clone(), dim22.clone())?) else { bail!("pattern mismatch") };
                    ell_1 = typeConvertMatrix(ell.clone(), t1.clone(), t2.clone(), printFailtrace.clone());
                    at = simplifyType(ty0.clone())?;
                    Ok((Arc::new(DAE::Exp::MATRIX { ty: at.clone(), integer: nmax.clone(), matrix: ell_1.clone() }), Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: t2.clone(), dims: list![dim11.clone()] }), dims: list![dim1.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: ell, integer: nmax, .. }, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Nil } }, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim22, tail: Deref @ metamodelica::List::Nil } }, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    if !((!(Expression::dimensionKnown(dim2.clone())))) { bail!("guard") }
                    let mut at: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ell_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let true = (Expression::dimensionsKnownAndEqual(dim11.clone(), dim22.clone())?) else { bail!("pattern mismatch") };
                    ell_1 = typeConvertMatrix(ell.clone(), t1.clone(), t2.clone(), printFailtrace.clone());
                    ty = Arc::new(DAE::Type::T_ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: t2.clone(), dims: list![dim11.clone()] }), dims: list![dim1.clone()] });
                    at = simplifyType(ty.clone())?;
                    Ok((Arc::new(DAE::Exp::MATRIX { ty: at.clone(), integer: nmax.clone(), matrix: ell_1.clone() }), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t_2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    let true = (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) else { bail!("pattern mismatch") };
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    e_1 = liftExpType(e_1.clone(), dim1.clone())?;
                    t_2 = Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim2.clone()] });
                    Ok((e_1.clone(), t_2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    e_1 = liftExpType(e_1.clone(), Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))?;
                    Ok((e_1.clone(), Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    e_1 = liftExpType(e_1.clone(), dim1.clone())?;
                    Ok((e_1.clone(), Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim1.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    let false = (Expression::dimensionKnown(dim1.clone())) else { bail!("pattern mismatch") };
                    let false = (Expression::dimensionKnown(dim2.clone())) else { bail!("pattern mismatch") };
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
                    e_1 = liftExpType(e_1.clone(), Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))?;
                    Ok((e_1.clone(), Arc::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: elist }, Deref @ DAE::Type::T_TUPLE { types: tys1, .. }, Deref @ DAE::Type::T_TUPLE { types: tys2, .. }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tys_1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    (elist_1, tys_1) = typeConvertList(elist.clone(), tys1.clone(), tys2.clone(), printFailtrace.clone())?;
                    Ok((Arc::new(DAE::Exp::TUPLE { PR: elist_1.clone() }), Arc::new(DAE::Type::T_TUPLE { types: tys_1.clone(), names: var_field!((*expected).names, DAE::Type::T_TUPLE).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_INTEGER { .. }, t2 @ Deref @ DAE::Type::T_ENUMERATION { names: l, path: tp, .. }, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut tp = (*tp).clone();
                    let true = (Config::intEnumConversion()?) else { bail!("pattern mismatch") };
                    let true = (typeConvertIntToEnumCheck(exp.clone(), t2.clone())?) else { bail!("pattern mismatch") };
                    name = ((l.clone()).get(oi.clone())?).clone();
                    tp = AbsynUtil::joinPaths(tp.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
                    Ok((Arc::new(DAE::Exp::ENUM_LITERAL { name: tp.clone(), index: oi.clone() }), expected.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }, _) => {
                    Ok((Arc::new(DAE::Exp::CAST { ty: DAE::T_REAL_DEFAULT().clone(), exp: e.clone() }), expected.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t1, .. }, t2, _) => {
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    Ok((e_1.clone(), t_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t2, .. }, _) => {
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    Ok((e_1.clone(), t_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_COMPLEX { varLst: els1, complexClassType: ClassInf::State::RECORD { path: p1 }, .. }, t2 @ Deref @ DAE::Type::T_COMPLEX { varLst: els2, complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, _) => {
                    let mut e = (*e).clone();
                    let false = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
                    let true = (Flags::isSet(Flags::ALLOW_RECORD_TOO_MANY_FIELDS.clone())? || (els1.clone().len() as i32) == (els2.clone().len() as i32)) else { bail!("pattern mismatch") };
                    let true = (subtypeVarlist(els1.clone(), els2.clone())?) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::CAST { ty: t2.clone(), exp: e.clone() });
                    Ok((e.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: Some(e) }, Deref @ DAE::Type::T_METAOPTION { ty: t1 }, Deref @ DAE::Type::T_METAOPTION { ty: t2 }, _) => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { bail!("guard") }
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    (e_1, t_1) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    Ok((Arc::new(DAE::Exp::META_OPTION { exp: Some(e_1.clone()) }), Arc::new(DAE::Type::T_METAOPTION { ty: t_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: None }, _, Deref @ DAE::Type::T_METAOPTION { ty: t2 }, _) => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { bail!("guard") }
                    Ok((Arc::new(DAE::Exp::META_OPTION { exp: None }), Arc::new(DAE::Type::T_METAOPTION { ty: t2.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: elist }, Deref @ DAE::Type::T_TUPLE { types: tys1, .. }, Deref @ DAE::Type::T_METATUPLE { types: tys2 }, _) => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { bail!("guard") }
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tys_1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tys2 = (*tys2).clone();
                    tys2 = List::map(tys2.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    (elist_1, tys_1) = matchTypeTuple(elist.clone(), tys1.clone(), tys2.clone(), printFailtrace.clone())?;
                    Ok((Arc::new(DAE::Exp::META_TUPLE { listExp: elist_1.clone() }), Arc::new(DAE::Type::T_METATUPLE { types: tys_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchTy, inputs, aliases, localDecls, cases, et }, _, _, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut elist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut cases = (*cases).clone();
                    let mut et = (*et).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    elist = Patternm::resultExps(cases.clone());
                    (elist_1, _) = matchTypeList(elist.clone(), actual.clone(), expected.clone(), printFailtrace.clone())?;
                    cases = Patternm::fixCaseReturnTypes2(cases.clone(), elist_1.clone(), Absyn::dummyInfo.clone())?;
                    et = simplifyType(expected.clone())?;
                    Ok((Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: matchTy.clone(), inputs: inputs.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases.clone(), et: et.clone() }), expected.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_TUPLE { listExp: elist }, Deref @ DAE::Type::T_METATUPLE { types: tys1 }, Deref @ DAE::Type::T_METATUPLE { types: tys2 }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tys_1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tys2 = (*tys2).clone();
                    tys2 = List::map(tys2.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    (elist_1, tys_1) = matchTypeTuple(elist.clone(), tys1.clone(), tys2.clone(), printFailtrace.clone())?;
                    Ok((Arc::new(DAE::Exp::META_TUPLE { listExp: elist_1.clone() }), Arc::new(DAE::Type::T_METATUPLE { types: tys_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: elist }, Deref @ DAE::Type::T_TUPLE { types: tys1, .. }, ty2 @ Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_UNKNOWN }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tys_1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    tys2 = List::fill(ty2.clone(), (tys1.clone().len() as i32));
                    (elist_1, tys_1) = matchTypeTuple(elist.clone(), tys1.clone(), tys2.clone(), printFailtrace.clone())?;
                    Ok((Arc::new(DAE::Exp::META_TUPLE { listExp: elist_1.clone() }), Arc::new(DAE::Type::T_METATUPLE { types: tys_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, scalar: _, array: elist }, Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_METALIST { ty: t2 }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t2 = (*t2).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    t2 = boxIfUnboxedType(t2.clone())?;
                    (elist_1, _) = matchTypeList(elist.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    e_1 = Arc::new(DAE::Exp::LIST { valList: elist_1.clone() });
                    t2 = Arc::new(DAE::Type::T_METALIST { ty: t2.clone() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, scalar: _, array: elist }, Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut t2 = (*t2).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    (elist_1, tys1) = matchTypeList(elist.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    (elist_1, t2) = listMatchSuperType(elist_1.clone(), tys1.clone(), printFailtrace.clone())?;
                    t2 = boxIfUnboxedType(t2.clone())?;
                    (elist_1, _) = matchTypeList(elist_1.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    e_1 = Arc::new(DAE::Exp::LIST { valList: elist_1.clone() });
                    t2 = Arc::new(DAE::Type::T_METALIST { ty: t2.clone() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { .. }, integer: _, matrix: elist_big }, t1, t2, _) => {
                    let mut elist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    (elist, ty2) = typeConvertMatrixToList(elist_big.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    e_1 = Arc::new(DAE::Exp::LIST { valList: elist.clone() });
                    Ok((e_1.clone(), ty2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: elist }, Deref @ DAE::Type::T_METALIST { ty: t1 }, Deref @ DAE::Type::T_METALIST { ty: t2 }, _) => {
                    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut t2 = (*t2).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    (elist_1, tys1) = matchTypeList(elist.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    (elist_1, t2) = listMatchSuperType(elist_1.clone(), tys1.clone(), printFailtrace.clone())?;
                    e_1 = Arc::new(DAE::Exp::LIST { valList: elist_1.clone() });
                    t2 = Arc::new(DAE::Type::T_METALIST { ty: t2.clone() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace.clone())?;
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace.clone())?;
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace.clone())?;
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_ENUMERATION { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace.clone())?;
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_ARRAY { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace.clone())?;
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: elist, path: path1, .. }, t1 @ Deref @ DAE::Type::T_COMPLEX { varLst: v, complexClassType: ClassInf::State::RECORD { path: path2 }, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut l: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut elist = (*elist).clone();
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    l = List::map(v.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    tys1 = List::map(v.clone(), (std::sync::Arc::new(getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>));
                    tys2 = List::map(tys1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    (elist, _) = matchTypeTuple(elist.clone(), tys1.clone(), tys2.clone(), printFailtrace.clone())?;
                    e_1 = Arc::new(DAE::Exp::METARECORDCALL { path: path1.clone(), args: elist.clone(), fieldNames: l.clone(), index: -1, typeVars: metamodelica::nil() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { exps: elist, path: path1, .. }, t1 @ Deref @ DAE::Type::T_COMPLEX { varLst: v, complexClassType: ClassInf::State::RECORD { path: path2 }, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut l: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut elist = (*elist).clone();
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    l = List::map(v.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    tys1 = List::map(v.clone(), (std::sync::Arc::new(getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>));
                    tys2 = List::map(tys1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    (elist, _) = matchTypeTuple(elist.clone(), tys1.clone(), tys2.clone(), printFailtrace.clone())?;
                    e_1 = Arc::new(DAE::Exp::METARECORDCALL { path: path1.clone(), args: elist.clone(), fieldNames: l.clone(), index: -1, typeVars: metamodelica::nil() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, ty: _ }, t1 @ Deref @ DAE::Type::T_COMPLEX { varLst: v, complexClassType: ClassInf::State::RECORD { path }, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }, _) => {
                    let mut elist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut l: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut pathList: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut crefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    t2 = Arc::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    l = List::map(v.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    tys1 = List::map(v.clone(), (std::sync::Arc::new(getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>));
                    tys2 = List::map(tys1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    expTypes = List::map(tys1.clone(), (std::sync::Arc::new(simplifyType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    pathList = List::map(l.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>));
                    crefList = List::map(pathList.clone(), (std::sync::Arc::new(ComponentReference::pathToCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    crefList = List::map1r(crefList.clone(), (std::sync::Arc::new(ComponentReference::joinCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cref.clone());
                    elist = List::threadMap(crefList.clone(), expTypes.clone(), (std::sync::Arc::new(Expression::makeCrefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>));
                    (elist, _) = matchTypeTuple(elist.clone(), tys1.clone(), tys2.clone(), printFailtrace.clone())?;
                    e_1 = Arc::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: elist.clone(), fieldNames: l.clone(), index: -1, typeVars: metamodelica::nil() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, Deref @ DAE::Type::T_METABOXED { .. }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Not yet implemented: Converting record into boxed records: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BOX { exp: e }, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2, _) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    (e_1, t2) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_INTEGER { .. }, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    (_, _) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    t = simplifyType(t2.clone())?;
                    Ok((Arc::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_REAL { .. }, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    (_, _) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    t = simplifyType(t2.clone())?;
                    Ok((Arc::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_BOOL { .. }, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    (_, _) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    t = simplifyType(t2.clone())?;
                    Ok((Arc::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_ENUMERATION { .. }, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    t = simplifyType(t2.clone())?;
                    Ok((Arc::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, _) => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e_1: Arc<DAE::Exp>;
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    (e_1, _) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    t = simplifyType(t2.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("mmc_unbox_record")).clone() }), expLst: list![e_1.clone()], attr: Arc::new(DAE::CallAttributes { ty: t.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), t2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outType))
}

fn liftExpType(mut ie: Arc<DAE::Exp>, mut dim: Arc<DAE::Dimension>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    res = 'mc: {
        let __mc_input = (ie.clone(), dim.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty, exp: e }, _) => {
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty1 = Expression::liftArrayR(ty.clone(), dim.clone());
                    Ok(Arc::new(DAE::Exp::CAST { ty: ty1.clone(), exp: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn typeConvertArray(mut inArray: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inActualType: Arc<DAE::Type>, mut inExpectedType: Arc<DAE::Type>, mut inPrintFailtrace: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outArray: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outArray = (::match_deref::match_deref! { match &((inArray.clone(), inActualType.clone(), inExpectedType.clone(), inPrintFailtrace.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            let mut e: Arc<DAE::Exp>;
            e = makeDummyExpFromType(inActualType.clone())?;
            (_, _) = typeConvert(e.clone(), inActualType.clone(), inExpectedType.clone(), inPrintFailtrace.clone())?;
            metamodelica::nil()
        },
        _ => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (expl, _) = List::map_2(inArray.clone(), Arc::new({ let __pe_b1 = inActualType.clone(); let __pe_b2 = inExpectedType.clone(); let __pe_b3 = inPrintFailtrace.clone(); move |__pe_a0| typeConvert(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }));
            expl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArray)
}

fn typeConvertMatrix(mut inMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inActualType: Arc<DAE::Type>, mut inExpectedType: Arc<DAE::Type>, mut printFailtrace: bool) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> {
    let mut outMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    outMatrix = List::map3(inMatrix.clone(), (std::sync::Arc::new(typeConvertArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, Arc<DAE::Type>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), inActualType.clone(), inExpectedType.clone(), printFailtrace.clone());
    outMatrix
}

fn typeConvertList(mut inExpExpLst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypeLst2: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inTypeLst3: Arc<metamodelica::List<Arc<DAE::Type>>>, mut printFailtrace: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outExpExpLst, outTypeLst) = (::match_deref::match_deref! { match &((inExpExpLst1.clone(), inTypeLst2.clone(), inTypeLst3.clone(), printFailtrace.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: first, tail: rest }, Deref @ metamodelica::List::Cons { head: ty1, tail: ty1rest }, Deref @ metamodelica::List::Cons { head: ty2, tail: ty2rest }, _) => {
            let mut rest_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tyrest_1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut first_1: Arc<DAE::Exp>;
            let mut ty_1: Type = Arc::new(DAE::Type::T_NORETCALL);
            (rest_1, tyrest_1) = typeConvertList(rest.clone(), ty1rest.clone(), ty2rest.clone(), printFailtrace.clone())?;
            (first_1, ty_1) = typeConvert(first.clone(), ty1.clone(), ty2.clone(), printFailtrace.clone())?;
            (cons(first_1.clone(), rest_1.clone()), cons(ty_1.clone(), tyrest_1.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExpExpLst, outTypeLst))
}

fn typeConvertMatrixToList(mut melist: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inType: Arc<DAE::Type>, mut outType: Arc<DAE::Type>, mut printFailtrace: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>)> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut actualOutType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outExp, actualOutType) = 'mc: {
        let __mc_input = (melist.clone(), inType.clone(), outType.clone(), printFailtrace.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: expl, tail: rest }, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, .. }, Deref @ DAE::Type::T_METALIST { ty: Deref @ DAE::Type::T_METALIST { ty: t2 } }, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut expl = (*expl).clone();
                    let mut t1 = (*t1).clone();
                    (e, t1) = typeConvertMatrixRowToList(expl.clone(), t1.clone(), t2.clone(), printFailtrace.clone())?;
                    (expl, _) = typeConvertMatrixToList(rest.clone(), inType.clone(), outType.clone(), printFailtrace.clone())?;
                    Ok((cons(e.clone(), expl.clone()), Arc::new(DAE::Type::T_METALIST { ty: t1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::TYPES.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- typeConvertMatrixToList failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, actualOutType))
}

fn typeConvertMatrixRowToList(mut elist: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inType: Arc<DAE::Type>, mut outType: Arc<DAE::Type>, mut printFailtrace: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut out: Arc<DAE::Exp>;
    let mut t1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut exp: Arc<DAE::Exp>;
    let mut elist_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matchTypeList(elist.clone(), inType.clone(), outType.clone(), printFailtrace.clone())?) {
        (__pa0, Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    elist_1 = __pa0.clone();
    t1 = __pa1.clone();
    out = Arc::new(DAE::Exp::LIST { valList: elist_1.clone() });
    t1 = Arc::new(DAE::Type::T_METALIST { ty: t1.clone() });
    Ok((out, t1))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn matchWithPromote(mut inProperties1: DAE::Properties, mut inProperties2: DAE::Properties, mut inBoolean3: bool) -> Result<DAE::Properties> {
    let mut outProperties: DAE::Properties;
    outProperties = 'mc: {
        let __mc_input = (inProperties1.clone(), inProperties2.clone(), inBoolean3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t1, .. }, constFlag: c1 }, DAE::Properties::PROP { type_: t2, constFlag: c2 }, havereal) => {
                    Ok(matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t1, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t2, .. }, constFlag: c2 }, havereal) => {
                    Ok(matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Nil } } }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, havereal) => {
                    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    c = __pa1.clone();
                    dim = dim1.clone();
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: t1 }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 1 }, tail: Deref @ metamodelica::List::Nil } } }, havereal) => {
                    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let false = (isArray(t1.clone())) else { bail!("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    c = __pa1.clone();
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: t1 }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim @ Deref @ DAE::Dimension::DIM_ENUM { size: 1, .. }, tail: Deref @ metamodelica::List::Nil } } }, havereal) => {
                    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let false = (isArray(t1.clone())) else { bail!("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    c = __pa1.clone();
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: t1 }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_ARRAY { ty: t2, dims: Deref @ metamodelica::List::Cons { head: dim @ Deref @ DAE::Dimension::DIM_BOOLEAN, tail: Deref @ metamodelica::List::Nil } } }, havereal) => {
                    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let false = (isArray(t1.clone())) else { bail!("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    c = __pa1.clone();
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 1 }, tail: Deref @ metamodelica::List::Nil } } }, DAE::Properties::PROP { constFlag: c2, type_: t2 }, havereal) => {
                    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let false = (isArray(t2.clone())) else { bail!("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    c = __pa1.clone();
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim @ Deref @ DAE::Dimension::DIM_ENUM { size: 1, .. }, tail: Deref @ metamodelica::List::Nil } } }, DAE::Properties::PROP { constFlag: c2, type_: t2 }, havereal) => {
                    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let false = (isArray(t2.clone())) else { bail!("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    c = __pa1.clone();
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_ARRAY { ty: t1, dims: Deref @ metamodelica::List::Cons { head: dim @ Deref @ DAE::Dimension::DIM_BOOLEAN, tail: Deref @ metamodelica::List::Nil } } }, DAE::Properties::PROP { constFlag: c2, type_: t2 }, havereal) => {
                    let mut t: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut c: Const = DAE::Const::C_CONST;
                    let false = (isArray(t2.clone())) else { bail!("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }, DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }, havereal.clone())?) else { bail!("pattern mismatch") };
                    t = __pa0.clone();
                    c = __pa1.clone();
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: t1 }, DAE::Properties::PROP { constFlag: c2, type_: t2 }, false) => {
                    let mut c: Const = DAE::Const::C_CONST;
                    let false = (isArray(t1.clone())) else { bail!("pattern mismatch") };
                    let false = (isArray(t2.clone())) else { bail!("pattern mismatch") };
                    let true = (equivtypes(t1.clone(), t2.clone())?) else { bail!("pattern mismatch") };
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: t1.clone(), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: t @ Deref @ DAE::Type::T_ENUMERATION { .. } }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_ENUMERATION { .. } }, false) => {
                    let mut c: Const = DAE::Const::C_CONST;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: t.clone(), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_REAL { varLst: v } }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_REAL { .. } }, true) => {
                    let mut c: Const = DAE::Const::C_CONST;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_REAL { varLst: v.clone() }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_INTEGER { .. } }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_REAL { varLst: v } }, true) => {
                    let mut c: Const = DAE::Const::C_CONST;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_REAL { varLst: v.clone() }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_REAL { varLst: v } }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_INTEGER { .. } }, true) => {
                    let mut c: Const = DAE::Const::C_CONST;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: Arc::new(DAE::Type::T_REAL { varLst: v.clone() }), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { constFlag: c1, type_: Deref @ DAE::Type::T_INTEGER { .. } }, DAE::Properties::PROP { constFlag: c2, type_: Deref @ DAE::Type::T_INTEGER { .. } }, true) => {
                    let mut c: Const = DAE::Const::C_CONST;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: c.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.matchWithPromote failed on: ")); __mm_s.push_str(&*literal!("\nprop1: ")); __mm_s.push_str(&*printPropStr(inProperties1.clone())?); __mm_s.push_str(&*literal!("\nprop2: ")); __mm_s.push_str(&*printPropStr(inProperties2.clone())?); __mm_s.push_str(&*literal!("\nhaveReal: ")); __mm_s.push_str(&*boolString(inBoolean3.clone())); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProperties)
}

pub fn constAnd(mut inConst1: DAE::Const, mut inConst2: DAE::Const) -> DAE::Const {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = (match (inConst1.clone(), inConst2.clone()) {
        (DAE::Const::C_CONST, DAE::Const::C_CONST) => openmodelica_frontend_types::DAE::Const::C_CONST,
        (DAE::Const::C_CONST, DAE::Const::C_PARAM) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_PARAM, DAE::Const::C_CONST) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_PARAM, DAE::Const::C_PARAM) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_UNKNOWN, _) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        (_, DAE::Const::C_UNKNOWN) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        _ => openmodelica_frontend_types::DAE::Const::C_VAR,
    });
    outConst
}

fn constTupleAnd(mut inTupleConst1: Arc<DAE::TupleConst>, mut inTupleConst2: Arc<DAE::TupleConst>) -> Arc<DAE::TupleConst> {
    let mut outTupleConst: Arc<DAE::TupleConst>;
    outTupleConst = (::match_deref::match_deref! { match &((inTupleConst1.clone(), inTupleConst2.clone())) {
        (c1, _) => {
            c1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTupleConst
}

pub fn constOr(mut inConst1: DAE::Const, mut inConst2: DAE::Const) -> DAE::Const {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = (match (inConst1.clone(), inConst2.clone()) {
        (DAE::Const::C_CONST, _) => openmodelica_frontend_types::DAE::Const::C_CONST,
        (_, DAE::Const::C_CONST) => openmodelica_frontend_types::DAE::Const::C_CONST,
        (DAE::Const::C_PARAM, _) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (_, DAE::Const::C_PARAM) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_UNKNOWN, _) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        (_, DAE::Const::C_UNKNOWN) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        _ => openmodelica_frontend_types::DAE::Const::C_VAR,
    });
    outConst
}

pub fn boolConst(mut inBoolean: bool) -> DAE::Const {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = (match inBoolean.clone() {
        false => openmodelica_frontend_types::DAE::Const::C_VAR,
        true => openmodelica_frontend_types::DAE::Const::C_CONST,
    });
    outConst
}

pub fn boolConstSize(mut inBoolean: bool) -> DAE::Const {
    let mut outConst: DAE::Const = DAE::Const::C_CONST;
    outConst = (match inBoolean.clone() {
        false => openmodelica_frontend_types::DAE::Const::C_PARAM,
        true => openmodelica_frontend_types::DAE::Const::C_CONST,
    });
    outConst
}

pub fn constEqualOrHigher(mut c1: DAE::Const, mut c2: DAE::Const) -> bool {
    let mut b: bool = false;
    b = (match (c1.clone(), c2.clone()) {
        (DAE::Const::C_CONST, _) => true,
        (_, DAE::Const::C_CONST) => false,
        (DAE::Const::C_PARAM, _) => true,
        (_, DAE::Const::C_PARAM) => false,
        _ => true,
    });
    b
}

pub fn constEqual(mut c1: DAE::Const, mut c2: DAE::Const) -> bool {
    let mut b: bool = false;
    b = c1.clone() == c2.clone();
    b
}

pub fn constIsVariable(mut c: DAE::Const) -> bool {
    let mut b: bool = false;
    b = constEqual(c.clone(), openmodelica_frontend_types::DAE::Const::C_VAR);
    b
}

pub fn constIsParameter(mut c: DAE::Const) -> bool {
    let mut b: bool = false;
    b = constEqual(c.clone(), openmodelica_frontend_types::DAE::Const::C_PARAM);
    b
}

pub fn constIsConst(mut c: DAE::Const) -> bool {
    let mut b: bool = false;
    b = constEqual(c.clone(), openmodelica_frontend_types::DAE::Const::C_CONST);
    b
}

pub fn printPropStr(mut inProperties: DAE::Properties) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inProperties.clone() {
        DAE::Properties::PROP { constFlag: mut r#const, type_: ref ty } => {
            let mut ty_str: ArcStr = arcstr::literal!("");
            let mut const_str: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            ty_str = (TypesDump::unparseType(ty.clone())?).clone();
            const_str = (TypesDump::printConstStr(r#const.clone())?).clone();
            res = stringAppendList(list![(literal!("DAE.PROP(")).clone(), (ty_str.clone()).clone(), (literal!(", ")).clone(), (const_str.clone()).clone(), (literal!(")")).clone()]);
            res.clone()
        },
        DAE::Properties::PROP_TUPLE { tupleConst: ref tconst, type_: ref ty } => {
            let mut ty_str: ArcStr = arcstr::literal!("");
            let mut const_str: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            ty_str = (TypesDump::unparseType(ty.clone())?).clone();
            const_str = (TypesDump::printTupleConstStr(tconst.clone())?).clone();
            res = stringAppendList(list![(literal!("DAE.PROP_TUPLE(")).clone(), (ty_str.clone()).clone(), (literal!(", ")).clone(), (const_str.clone()).clone(), (literal!(")")).clone()]);
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn printProp(mut p: DAE::Properties) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (printPropStr(p.clone())?).clone();
    Print::printErrorBuf((r#str.clone()).clone())?;
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn flowVariables(mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outExpComponentRefLst = 'mc: {
        let __mc_input = (inVarLst.clone(), inComponentRef.clone());
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
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::FLOW, .. }, name: id, .. }, tail: vs }, cr) => {
                    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty2 = simplifyType(ty.clone())?;
                    cr_1 = ComponentReference::crefPrependIdent(cr.clone(), (id.clone()).clone(), metamodelica::nil(), ty2.clone())?;
                    res = flowVariables(vs.clone(), cr.clone())?;
                    Ok(cons(cr_1.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: vs }, cr) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res = flowVariables(vs.clone(), cr.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpComponentRefLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn streamVariables(mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outExpComponentRefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    outExpComponentRefLst = 'mc: {
        let __mc_input = (inVarLst.clone(), inComponentRef.clone());
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
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. }, name: id, .. }, tail: vs }, cr) => {
                    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty2 = simplifyType(ty.clone())?;
                    cr_1 = ComponentReference::crefPrependIdent(cr.clone(), (id.clone()).clone(), metamodelica::nil(), ty2.clone())?;
                    res = streamVariables(vs.clone(), cr.clone())?;
                    Ok(cons(cr_1.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: vs }, cr) => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    res = streamVariables(vs.clone(), cr.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpComponentRefLst)
}

pub fn getAllExps(mut inType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpExpLst = getAllExpsTt(inType.clone())?;
    Ok(outExpExpLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAllExpsTt(mut inType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpExpLst = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { attributeLst: attrs, literalVarLst: vars, .. } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tyexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    exps = getAllExpsVars(vars.clone());
                    tyexps = getAllExpsVars(attrs.clone());
                    exps = listAppend(tyexps.clone(), exps.clone());
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, .. } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. } => {
                    Ok(getAllExpsVars(vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst: vars, .. } => {
                    Ok(getAllExpsVars(vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, funcArg: fargs, .. } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tyexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut explists: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    explists = List::mapMap(fargs.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>), (std::sync::Arc::new(getAllExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
                    tyexps = getAllExps(ty.clone())?;
                    exps = List::flatten(cons(tyexps.clone(), explists.clone()));
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut explist: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    explist = List::map(tys.clone(), (std::sync::Arc::new(getAllExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
                    exps = List::flatten(explist.clone());
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATUPLE { types: tys } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    exps = getAllExpsTt(Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }))?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAOPTION { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAARRAY { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_NORETCALL => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tty => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = (TypesDump::unparseType(tty.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Types.getAllExpsTt failed ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExpExpLst)
}

fn getAllExpsVars(mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut explist: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    explist = List::map(vars.clone(), (std::sync::Arc::new(getAllExpsVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
    exps = List::flatten(explist.clone());
    exps
}

fn getAllExpsVar(mut inVar: Arc<DAE::Var>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpExpLst = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { binding: bnd, ty, .. } => {
            let mut tyexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut bndexp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            tyexps = getAllExps(ty.clone())?;
            bndexp = getAllExpsBinding(bnd.clone())?;
            exps = listAppend(tyexps.clone(), bndexp.clone());
            exps.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpExpLst)
}

fn getAllExpsBinding(mut inBinding: Arc<DAE::Binding>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpExpLst = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::EQBOUND { exp, .. } => {
            list![exp.clone()]
        },
        Deref @ DAE::Binding::UNBOUND => {
            metamodelica::nil()
        },
        Deref @ DAE::Binding::VALBOUND { .. } => {
            metamodelica::nil()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-- Types.getAllExpsBinding failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpExpLst)
}

pub fn isBoxedType(mut ty: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_STRING { .. } => true,
        Deref @ DAE::Type::T_METAOPTION { .. } => true,
        Deref @ DAE::Type::T_METALIST { .. } => true,
        Deref @ DAE::Type::T_METATUPLE { .. } => true,
        Deref @ DAE::Type::T_METAUNIONTYPE { .. } => true,
        Deref @ DAE::Type::T_METARECORD { .. } => true,
        Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => true,
        Deref @ DAE::Type::T_METAARRAY { .. } => true,
        Deref @ DAE::Type::T_FUNCTION { .. } => true,
        Deref @ DAE::Type::T_METABOXED { .. } => true,
        Deref @ DAE::Type::T_ANYTYPE { .. } => true,
        Deref @ DAE::Type::T_UNKNOWN => true,
        Deref @ DAE::Type::T_METATYPE { .. } => true,
        Deref @ DAE::Type::T_NORETCALL => true,
        Deref @ DAE::Type::T_CODE { .. } => true,
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isMetaBoxedType(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsMetaBoxed: bool = false;
    outIsMetaBoxed = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_METABOXED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsMetaBoxed
}

pub fn boxIfUnboxedType(mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = ty.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { .. } => {
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    tys = List::map(var_field!((*ty).types, DAE::Type::T_TUPLE).clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METATUPLE { types: tys.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(if (isBoxedType(ty.clone())) {ty.clone()} else {Arc::new(DAE::Type::T_METABOXED { ty: ty.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn unboxedType(mut ity: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut out: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    out = (::match_deref::match_deref! { match &(ity.clone()) {
        Deref @ DAE::Type::T_METABOXED { .. } => {
            unboxedType(var_field!((*ity).ty, DAE::Type::T_METABOXED).clone())?
        },
        Deref @ DAE::Type::T_METAOPTION { .. } => {
            let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
            ty = unboxedType(var_field!((*ity).ty, DAE::Type::T_METAOPTION).clone())?;
            ty = boxIfUnboxedType(ty.clone())?;
            Arc::new(DAE::Type::T_METAOPTION { ty: ty.clone() })
        },
        Deref @ DAE::Type::T_METALIST { .. } => {
            let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
            ty = unboxedType(var_field!((*ity).ty, DAE::Type::T_METALIST).clone())?;
            ty = boxIfUnboxedType(ty.clone())?;
            Arc::new(DAE::Type::T_METALIST { ty: ty.clone() })
        },
        Deref @ DAE::Type::T_METATUPLE { .. } => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            tys = List::mapMap(var_field!((*ity).types, DAE::Type::T_METATUPLE).clone(), (std::sync::Arc::new(unboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
            Arc::new(DAE::Type::T_METATUPLE { types: tys.clone() })
        },
        Deref @ DAE::Type::T_METAARRAY { .. } => {
            let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
            ty = unboxedType(var_field!((*ity).ty, DAE::Type::T_METAARRAY).clone())?;
            ty = boxIfUnboxedType(ty.clone())?;
            Arc::new(DAE::Type::T_METAARRAY { ty: ty.clone() })
        },
        t @ Deref @ DAE::Type::T_ARRAY { .. } => {
            let mut t = (*t).clone();
            assign_variant_field!(t => DAE::Type::T_ARRAY; ty = unboxedType(var_field!((*t).ty, DAE::Type::T_ARRAY).clone())?);
            t.clone()
        },
        _ => {
            ity.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub fn listMatchSuperType(mut ielist: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut typeList: Arc<metamodelica::List<Arc<DAE::Type>>>, mut printFailtrace: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>)> {
    let mut out: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (out, t) = 'mc: {
        let __mc_input = (ielist.clone(), typeList.clone(), printFailtrace.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok((metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                    let mut st: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut elist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    st = List::reduce(typeList.clone(), (std::sync::Arc::new(superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    st = superType(st.clone(), st.clone())?;
                    st = unboxedType(st.clone())?;
                    elist = listMatchSuperType2(ielist.clone(), typeList.clone(), st.clone(), printFailtrace.clone())?;
                    Ok((elist.clone(), st.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Types.listMatchSuperType failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((out, t))
}

fn listMatchSuperType2(mut elist: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut typeList: Arc<metamodelica::List<Arc<DAE::Type>>>, mut st: Arc<DAE::Type>, mut printFailtrace: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut out: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    out = 'mc: {
        let __mc_input = (elist.clone(), typeList.clone(), st.clone(), printFailtrace.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: erest }, Deref @ metamodelica::List::Cons { head: t, tail: trest }, _, _) => {
                    let mut e = (*e).clone();
                    let mut erest = (*erest).clone();
                    let mut t = (*t).clone();
                    (e, t) = matchType(e.clone(), t.clone(), st.clone(), printFailtrace.clone())?;
                    erest = listMatchSuperType2(erest.clone(), trest.clone(), st.clone(), printFailtrace.clone())?;
                    Ok(cons(e.clone(), erest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: _ }, _, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.listMatchSuperType2 failed: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

pub fn superType(mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut out: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    out = 'mc: {
        let __mc_input = (inType1.clone(), inType2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ANYTYPE { .. }, t2) => {
                    Ok(t2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_ANYTYPE { .. }) => {
                    Ok(t1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_UNKNOWN, t2) => {
                    Ok(t2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_UNKNOWN) => {
                    Ok(t1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, t2 @ Deref @ DAE::Type::T_METAPOLYMORPHIC { .. }) => {
                    Ok(t2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: type_list1, .. }, Deref @ DAE::Type::T_TUPLE { types: type_list2, .. }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list2 = List::map(type_list2.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), (std::sync::Arc::new(superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: type_list1, .. }, Deref @ DAE::Type::T_METATUPLE { types: type_list2 }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list2 = List::map(type_list2.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), (std::sync::Arc::new(superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: type_list1 }, Deref @ DAE::Type::T_TUPLE { types: type_list2, .. }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list2 = List::map(type_list2.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), (std::sync::Arc::new(superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: type_list1 }, Deref @ DAE::Type::T_METATUPLE { types: type_list2 }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list2 = List::map(type_list2.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), (std::sync::Arc::new(superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METALIST { ty: t1 }, Deref @ DAE::Type::T_METALIST { ty: t2 }) => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    t1 = boxIfUnboxedType(t1.clone())?;
                    t2 = boxIfUnboxedType(t2.clone())?;
                    tp = superType(t1.clone(), t2.clone())?;
                    Ok(Arc::new(DAE::Type::T_METALIST { ty: tp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAOPTION { ty: t1 }, Deref @ DAE::Type::T_METAOPTION { ty: t2 }) => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    t1 = boxIfUnboxedType(t1.clone())?;
                    t2 = boxIfUnboxedType(t2.clone())?;
                    tp = superType(t1.clone(), t2.clone())?;
                    Ok(Arc::new(DAE::Type::T_METAOPTION { ty: tp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { ty: t1 }, Deref @ DAE::Type::T_METAARRAY { ty: t2 }) => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    t1 = boxIfUnboxedType(t1.clone())?;
                    t2 = boxIfUnboxedType(t2.clone())?;
                    tp = superType(t1.clone(), t2.clone())?;
                    Ok(Arc::new(DAE::Type::T_METAARRAY { ty: tp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1 @ Deref @ DAE::Type::T_METAUNIONTYPE { path: path1, .. }, Deref @ DAE::Type::T_METARECORD { utPath: path2, .. }) => {
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    Ok(t1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METARECORD { utPath: path1, knownSingleton: false, .. }, Deref @ DAE::Type::T_METARECORD { utPath: path2, knownSingleton: false, .. }) => {
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Type::T_METAUNIONTYPE { paths: metamodelica::nil(), typeVars: var_field!((*inType1).typeVars, DAE::Type::T_METARECORD).clone(), knownSingleton: false, singletonType: Arc::new(openmodelica_frontend_types::DAE::EvaluateSingletonType::NOT_SINGLETON), path: path1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, t2) => {
                    let true = (subtype(t1.clone(), t2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(t2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, t2) => {
                    let true = (subtype(t2.clone(), t1.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(t1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out)
}

pub fn matchTypePolymorphic(mut iexp: Arc<DAE::Exp>, mut iactual: Arc<DAE::Type>, mut expected: Arc<DAE::Type>, mut envPath: Option<Arc<Absyn::Path>>, mut ipolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut printFailtrace: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut exp: Arc<DAE::Exp> = iexp.clone();
    let mut actual: Arc<DAE::Type> = iactual.clone();
    let mut polymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = ipolymorphicBindings.clone();
    let debug: bool = false;
    if getAllInnerTypesOfType(expected.clone(), (std::sync::Arc::new(fnptr!(isPolymorphic, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>)).is_empty() {
        (exp, actual) = matchType(exp.clone(), actual.clone(), expected.clone(), printFailtrace.clone())?;
    } else {
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("match type: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!(" of ")); __mm_s.push_str(&*TypesDump::unparseType(actual.clone())?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*TypesDump::unparseType(expected.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        (exp, actual) = matchType(exp.clone(), actual.clone(), Arc::new(DAE::Type::T_METABOXED { ty: DAE::T_UNKNOWN_DEFAULT().clone() }), printFailtrace.clone())?;
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("matched type: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!(" of ")); __mm_s.push_str(&*TypesDump::unparseType(actual.clone())?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*TypesDump::unparseType(expected.clone())?); __mm_s.push_str(&*literal!(" (boxed)\n")); ArcStr::from(__mm_s) }).clone());
        }
        polymorphicBindings = subtypePolymorphic(getUniontypeIfMetarecordReplaceAllSubtypes(actual.clone())?, getUniontypeIfMetarecordReplaceAllSubtypes(expected.clone())?, envPath.clone(), polymorphicBindings.clone())?;
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("match type: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!(" of ")); __mm_s.push_str(&*TypesDump::unparseType(actual.clone())?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*TypesDump::unparseType(expected.clone())?); __mm_s.push_str(&*literal!(" and bindings ")); __mm_s.push_str(&*polymorphicBindingsStr(polymorphicBindings.clone())); __mm_s.push_str(&*literal!(" (OK)\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok((exp, actual, polymorphicBindings))
}

pub fn matchTypePolymorphicWithError(mut iexp: Arc<DAE::Exp>, mut iactual: Arc<DAE::Type>, mut iexpected: Arc<DAE::Type>, mut envPath: Option<Arc<Absyn::Path>>, mut ipolymorphicBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut info: SourceInfo) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (outExp, outType, outBindings) = 'mc: {
        let __mc_input = (iexp.clone(), iactual.clone(), iexpected.clone(), envPath.clone(), ipolymorphicBindings.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, actual, expected, _, polymorphicBindings, _) => {
                    let mut exp = (*exp).clone();
                    let mut actual = (*actual).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    (exp, actual, polymorphicBindings) = matchTypePolymorphic(exp.clone(), actual.clone(), expected.clone(), envPath.clone(), polymorphicBindings.clone(), false)?;
                    Ok((exp.clone(), actual.clone(), polymorphicBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    str1 = (ExpressionBasics::printExpStr(iexp.clone())?).clone();
                    str2 = (TypesDump::unparseType(iactual.clone())?).clone();
                    str3 = (TypesDump::unparseType(iexpected.clone())?).clone();
                    Error::addSourceMessage(Error::EXP_TYPE_MISMATCH.clone(), list![(str1.clone()).clone(), (str3.clone()).clone(), (str2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outType, outBindings))
}

pub fn matchType(mut inExp: Arc<DAE::Exp>, mut inActualType: Arc<DAE::Type>, mut inExpectedType: Arc<DAE::Type>, mut inPrintFailtrace: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if subtype(inExpectedType.clone(), inActualType.clone(), true)? {
        outExp = inExp.clone();
        outType = inActualType.clone();
    } else {
        match '__try0: {
            let false = (unwrap_break_err!(subtype(inActualType.clone(), inExpectedType.clone(), true), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            (outExp, outType) = unwrap_break_err!(typeConvert(inExp.clone(), inActualType.clone(), inExpectedType.clone(), inPrintFailtrace.clone()), '__try0);
            (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify1(outExp.clone()), '__try0);
            Ok::<_, anyhow::Error>((outExp.clone(), outType.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                outExp = __try0_o0;
                outType = __try0_o1;
            }
            Err(_) => {
                printFailure(Flags::TYPES.clone(), (literal!("matchType")).clone(), inExp.clone(), inActualType.clone(), inExpectedType.clone())?;
                bail!("fail");
            }
        }
    }
    Ok((outExp, outType))
}

pub fn matchTypeNoFail(mut inExp: Arc<DAE::Exp>, mut inActualType: Arc<DAE::Type>, mut inExpectedType: Arc<DAE::Type>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outMatch: bool = false;
    if subtype(inExpectedType.clone(), inActualType.clone(), true)? {
        outExp = inExp.clone();
        outType = inActualType.clone();
        outMatch = true;
    } else {
        match '__try0: {
            (outExp, outType) = unwrap_break_err!(typeConvert(inExp.clone(), inActualType.clone(), inExpectedType.clone(), false), '__try0);
            (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify1(outExp.clone()), '__try0);
            outMatch = true;
            Ok::<_, anyhow::Error>((outExp.clone(), outMatch.clone(), outType.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                outExp = __try0_o0;
                outMatch = __try0_o1;
                outType = __try0_o2;
            }
            Err(_) => {
                outExp = inExp.clone();
                outType = inActualType.clone();
                outMatch = true;
            }
        }
    }
    Ok((outExp, outType, outMatch))
}

pub fn matchTypes(mut iexps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut itys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut expected: Arc<DAE::Type>, mut printFailtrace: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outExps, outTys) = matchTypes_tail(iexps.clone(), itys.clone(), expected.clone(), printFailtrace.clone(), metamodelica::nil(), metamodelica::nil())?;
    Ok((outExps, outTys))
}

fn matchTypes_tail(mut iexps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut itys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut expected: Arc<DAE::Type>, mut printFailtrace: bool, mut inAccumExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAccumTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outExps, outTys) = (::match_deref::match_deref! { match &((iexps.clone(), itys.clone(), expected.clone(), printFailtrace.clone(), inAccumExps.clone(), inAccumTypes.clone())) {
        (Deref @ metamodelica::List::Cons { head: e, tail: exps }, Deref @ metamodelica::List::Cons { head: ty, tail: tys }, _, _, _, _) => {
            let mut e = (*e).clone();
            let mut exps = (*exps).clone();
            let mut ty = (*ty).clone();
            let mut tys = (*tys).clone();
            (e, ty) = matchTypes2(e.clone(), ty.clone(), expected.clone(), printFailtrace.clone())?;
            (exps, tys) = matchTypes_tail(exps.clone(), tys.clone(), expected.clone(), printFailtrace.clone(), cons(e.clone(), inAccumExps.clone()), cons(ty.clone(), inAccumTypes.clone()))?;
            (exps.clone(), tys.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (inAccumExps.clone().reverse(), inAccumTypes.clone().reverse())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExps, outTys))
}

fn matchTypes2(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inExpected: Arc<DAE::Type>, mut inPrintFailtrace: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outExp, outType) = 'mc: {
        let __mc_input = (inExp.clone(), inType.clone(), inExpected.clone(), inPrintFailtrace.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut expected_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = getUniontypeIfMetarecordReplaceAllSubtypes(inType.clone())?;
                    expected_ty = getUniontypeIfMetarecordReplaceAllSubtypes(inExpected.clone())?;
                    (e, ty) = matchType(inExp.clone(), ty.clone(), expected_ty.clone(), inPrintFailtrace.clone())?;
                    Ok((e.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.matchTypes failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" from ")); __mm_s.push_str(&*TypesDump::unparseType(inType.clone())?); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*TypesDump::unparseType(inExpected.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outType))
}

fn printFailure(mut flag: Flags::DebugFlag, mut source: ArcStr, mut e: Arc<DAE::Exp>, mut e_type: Arc<DAE::Type>, mut expected_type: Arc<DAE::Type>) -> Result<()> {
    if Flags::isSet(flag.clone())? {
        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.")); __mm_s.push_str(&*source.clone()); __mm_s.push_str(&*literal!(" failed on:")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone())?;
        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  type:")); __mm_s.push_str(&*TypesDump::unparseType(e_type.clone())?); __mm_s.push_str(&*literal!(" differs from expected\n  type:")); __mm_s.push_str(&*TypesDump::unparseType(expected_type.clone())?); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(())
}

fn polymorphicBindingStr(mut binding: (ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (r#str, tys) = binding.clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(":\n")); __mm_s.push_str(&*stringDelimitList(List::map1r(List::map(tys.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("      ")).clone()), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
    r#str
}

pub fn polymorphicBindingsStr(mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList(List::map(bindings.clone(), (std::sync::Arc::new(fnptr!(polymorphicBindingStr, (ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone());
    r#str
}

pub fn fixPolymorphicRestype(mut ty: Arc<DAE::Type>, mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut info: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    resType = fixPolymorphicRestype2(ty.clone(), (literal!("$")).clone(), bindings.clone(), info.clone())?;
    Ok(resType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fixPolymorphicRestype2(mut ty: Arc<DAE::Type>, mut prefix: ArcStr, mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut info: SourceInfo) -> Result<Arc<DAE::Type>> {
    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    resType = 'mc: {
        let __mc_input = (ty.clone(), prefix.clone(), bindings.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, _, _, _) => {
                    let mut t1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let __pa0 = ::match_deref::match_deref! { match &(polymorphicBindingsLookup(({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone(), bindings.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    t1 = __pa0.clone();
                    t1 = fixPolymorphicRestype2(t1.clone(), (literal!("")).clone(), bindings.clone(), info.clone())?;
                    Ok(t1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METALIST { ty: t1 }, _, _, _) => {
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    t2 = fixPolymorphicRestype2(t1.clone(), (prefix.clone()).clone(), bindings.clone(), info.clone())?;
                    t2 = boxIfUnboxedType(t2.clone())?;
                    Ok(Arc::new(DAE::Type::T_METALIST { ty: t2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { ty: t1 }, _, _, _) => {
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    t2 = fixPolymorphicRestype2(t1.clone(), (prefix.clone()).clone(), bindings.clone(), info.clone())?;
                    t2 = boxIfUnboxedType(t2.clone())?;
                    Ok(Arc::new(DAE::Type::T_METAARRAY { ty: t2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAOPTION { ty: t1 }, _, _, _) => {
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    t2 = fixPolymorphicRestype2(t1.clone(), (prefix.clone()).clone(), bindings.clone(), info.clone())?;
                    t2 = boxIfUnboxedType(t2.clone())?;
                    Ok(Arc::new(DAE::Type::T_METAOPTION { ty: t2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { typeVars: Deref @ metamodelica::List::Nil, .. }, _, _, _) => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { typeVars: tys, .. }, _, _, _) => {
                    let mut tys = (*tys).clone();
                    tys = List::map3(tys.clone(), (std::sync::Arc::new(fixPolymorphicRestype2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>), (prefix.clone()).clone(), bindings.clone(), info.clone());
                    tys = List::map(tys.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METAUNIONTYPE { paths: var_field!((*ty).paths, DAE::Type::T_METAUNIONTYPE).clone(), typeVars: tys.clone(), knownSingleton: var_field!((*ty).knownSingleton, DAE::Type::T_METAUNIONTYPE).clone(), singletonType: var_field!((*ty).singletonType, DAE::Type::T_METAUNIONTYPE).clone(), path: var_field!((*ty).path, DAE::Type::T_METAUNIONTYPE).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: tys }, _, _, _) => {
                    let mut tys = (*tys).clone();
                    tys = List::map3(tys.clone(), (std::sync::Arc::new(fixPolymorphicRestype2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>), (prefix.clone()).clone(), bindings.clone(), info.clone());
                    tys = List::map(tys.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_METATUPLE { types: tys.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1 @ Deref @ DAE::Type::T_ARRAY { .. }, _, _, _) => {
                    let mut t1 = (*t1).clone();
                    assign_variant_field!(t1 => DAE::Type::T_ARRAY; ty = fixPolymorphicRestype2(var_field!((*t1).ty, DAE::Type::T_ARRAY).clone(), (prefix.clone()).clone(), bindings.clone(), info.clone())?);
                    Ok(t1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1 @ Deref @ DAE::Type::T_TUPLE { .. }, _, _, _) => {
                    let mut t1 = (*t1).clone();
                    assign_variant_field!(t1 => DAE::Type::T_TUPLE; types = List::map3(var_field!((*t1).types, DAE::Type::T_TUPLE).clone(), (std::sync::Arc::new(fixPolymorphicRestype2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>), (prefix.clone()).clone(), bindings.clone(), info.clone()));
                    Ok(t1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: args1, funcResultType: ty1, functionAttributes, path }, _, _, _) => {
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut args1 = (*args1).clone();
                    let mut ty1 = (*ty1).clone();
                    tys1 = List::map(args1.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                    tys1 = List::map3(tys1.clone(), (std::sync::Arc::new(fixPolymorphicRestype2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, SourceInfo) -> Result<Arc<DAE::Type>> + 'static>), (prefix.clone()).clone(), bindings.clone(), info.clone());
                    ty1 = fixPolymorphicRestype2(ty1.clone(), (prefix.clone()).clone(), bindings.clone(), info.clone())?;
                    args1 = List::threadMap(args1.clone(), tys1.clone(), (std::sync::Arc::new(setFuncArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Arc<DAE::Type>) -> Result<Arc<DAE::FuncArg>> + 'static>));
                    ty1 = Arc::new(DAE::Type::T_FUNCTION { funcArg: args1.clone(), funcResultType: ty1.clone(), functionAttributes: functionAttributes.clone(), path: path.clone() });
                    Ok(ty1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut bstr: ArcStr = arcstr::literal!("");
                    let mut tstr: ArcStr = arcstr::literal!("");
                    tstr = (TypesDump::unparseType(ty.clone())?).clone();
                    bstr = (polymorphicBindingsStr(bindings.clone())).clone();
                    id = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.fixPolymorphicRestype failed for type: ")); __mm_s.push_str(&*tstr.clone()); __mm_s.push_str(&*literal!(" using bindings: ")); __mm_s.push_str(&*bstr.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(id.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(resType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn polymorphicBindingsLookup(mut id: ArcStr, mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> {
    let mut resType: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    resType = 'mc: {
        let __mc_input = (id.clone(), bindings.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: (id2, tys), tail: _ }) => {
                    let true = (id.clone() == id2.clone()) else { bail!("pattern mismatch") };
                    Ok(List::map(tys.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    tys = polymorphicBindingsLookup((id.clone()).clone(), rest.clone())?;
                    Ok(tys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(resType)
}

pub fn getAllInnerTypesOfType(mut inType: Arc<DAE::Type>, mut inFn: Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>) -> Arc<metamodelica::List<Arc<DAE::Type>>> {
    pub type TypeFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>;

    let mut outTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    outTypes = getAllInnerTypes(list![inType.clone()], metamodelica::nil(), inFn.clone());
    outTypes
}

fn getAllInnerTypes(mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inAccum: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>) -> Arc<metamodelica::List<Arc<DAE::Type>>> {
    pub type MatchFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>;

    let mut outTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = inAccum.clone();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    for mut t in &*inTypes.clone() {
        let mut t = t.clone();
        if inFunc(t.clone()).unwrap() {
            outTypes = cons(t.clone(), outTypes.clone());
        }
        tys = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, .. } => {
            list![ty.clone()]
        },
        Deref @ DAE::Type::T_METALIST { ty } => {
            list![ty.clone()]
        },
        Deref @ DAE::Type::T_METAARRAY { ty } => {
            list![ty.clone()]
        },
        Deref @ DAE::Type::T_METABOXED { ty } => {
            list![ty.clone()]
        },
        Deref @ DAE::Type::T_METAOPTION { ty } => {
            list![ty.clone()]
        },
        Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
            tys.clone()
        },
        Deref @ DAE::Type::T_METATUPLE { types: tys } => {
            tys.clone()
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { typeVars: tys, .. } => {
            tys.clone()
        },
        Deref @ DAE::Type::T_METARECORD { fields, typeVars: tys, .. } => {
            listAppend(tys.clone(), List::map(fields.clone(), (std::sync::Arc::new(getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>)))
        },
        Deref @ DAE::Type::T_COMPLEX { varLst: fields, .. } => {
            List::map(fields.clone(), (std::sync::Arc::new(getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>))
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst: fields, .. } => {
            List::map(fields.clone(), (std::sync::Arc::new(getVarType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Arc<DAE::Type>> + 'static>))
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, funcArg: funcArgs, .. } => {
            cons(ty.clone(), List::map(funcArgs.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>)))
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outTypes = getAllInnerTypes(tys.clone(), outTypes.clone(), inFunc.clone());
    }
    outTypes
}

pub fn uniontypeFilter(mut ty: Arc<DAE::Type>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn metarecordFilter(mut ty: Arc<DAE::Type>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METARECORD { path: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn getUniontypePaths(mut ty: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outPaths = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAUNIONTYPE { paths, .. } => {
            paths.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPaths)
}

pub fn makeFunctionPolymorphicReference(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: funcArgs1, funcResultType: resType1, functionAttributes, path } => {
            let mut funcArgs2: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
            let mut funcArgTypes1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut funcArgTypes2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut dummyBoxedTypeList: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut dummyExpList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut resType2: Type = Arc::new(DAE::Type::T_NORETCALL);
            funcArgTypes1 = List::map(funcArgs1.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
            (dummyExpList, dummyBoxedTypeList) = makeDummyExpAndTypeLists(funcArgTypes1.clone())?;
            (_, funcArgTypes2) = matchTypeTuple(dummyExpList.clone(), funcArgTypes1.clone(), dummyBoxedTypeList.clone(), false)?;
            funcArgs2 = List::threadMap(funcArgs1.clone(), funcArgTypes2.clone(), (std::sync::Arc::new(setFuncArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Arc<DAE::Type>) -> Result<Arc<DAE::FuncArg>> + 'static>));
            resType2 = makeFunctionPolymorphicReferenceResType(resType1.clone())?;
            ty2 = Arc::new(DAE::Type::T_FUNCTION { funcArg: funcArgs2.clone(), funcResultType: resType2.clone(), functionAttributes: functionAttributes.clone(), path: path.clone() });
            ty2.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn makeFunctionPolymorphicReferenceResType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty @ Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut dummyBoxedTypeList: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut dummyExpList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ty = (*ty).clone();
                    let mut tys = (*tys).clone();
                    (dummyExpList, dummyBoxedTypeList) = makeDummyExpAndTypeLists(tys.clone())?;
                    (_, tys) = matchTypeTuple(dummyExpList.clone(), tys.clone(), dummyBoxedTypeList.clone(), false)?;
                    assign_variant_field!(ty => DAE::Type::T_TUPLE; types = tys.clone());
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty @ Deref @ DAE::Type::T_NORETCALL => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty1 => {
                    let mut e: Arc<DAE::Exp>;
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(makeDummyExpAndTypeLists(list![ty1.clone()])?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    ty2 = __pa1.clone();
                    (_, ty) = matchType(e.clone(), ty1.clone(), ty2.clone(), false)?;
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn makeDummyExpAndTypeLists(mut lst: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    (outExps, outTypes) = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut restExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut restType: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut crefExp: Arc<DAE::Exp>;
            (restExp, restType) = makeDummyExpAndTypeLists(rest.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent((literal!("#DummyExp#")).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
            crefExp = Expression::crefExp(cref_.clone())?;
            (cons(crefExp.clone(), restExp.clone()), cons(Arc::new(DAE::Type::T_METABOXED { ty: DAE::T_UNKNOWN_DEFAULT().clone() }), restType.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExps, outTypes))
}

pub fn resTypeToListTypes(mut inType: Arc<DAE::Type>) -> Arc<metamodelica::List<Arc<DAE::Type>>> {
    let mut outType: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
            tys.clone()
        },
        Deref @ DAE::Type::T_NORETCALL => {
            metamodelica::nil()
        },
        ty => {
            list![ty.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getRealOrIntegerDimensions(mut inType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    outDims = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Type::T_INTEGER { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            getRealOrIntegerDimensions(ty.clone())?
        },
        Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: d @ Deref @ DAE::Dimension::DIM_INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            dims = getRealOrIntegerDimensions(ty.clone())?;
            cons(d.clone(), dims.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDims)
}

pub fn isPolymorphic(mut ty: Arc<DAE::Type>) -> bool {
    let mut outMatch: bool = false;
    outMatch = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAPOLYMORPHIC { name: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

pub fn polymorphicTypeName(mut ty: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAPOLYMORPHIC { name: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn addPolymorphicBinding(mut id: ArcStr, mut ity: Arc<DAE::Type>, mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>> {
    let mut outBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    outBindings = 'mc: {
        let __mc_input = (id.clone(), ity.clone(), bindings.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ty, Deref @ metamodelica::List::Nil) => {
                    let mut ty = (*ty).clone();
                    ty = unboxedType(ty.clone())?;
                    ty = boxIfUnboxedType(ty.clone())?;
                    Ok(list![(id.clone(), list![ty.clone()])])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id1, ty, Deref @ metamodelica::List::Cons { head: (id2, tys), tail: rest }) => {
                    let mut ty = (*ty).clone();
                    let true = (id1.clone() == id2.clone()) else { bail!("pattern mismatch") };
                    ty = unboxedType(ty.clone())?;
                    ty = boxIfUnboxedType(ty.clone())?;
                    Ok(cons((id2.clone(), cons(ty.clone(), tys.clone())), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ty, Deref @ metamodelica::List::Cons { head: first, tail: rest }) => {
                    let mut rest = (*rest).clone();
                    rest = addPolymorphicBinding((id.clone()).clone(), ty.clone(), rest.clone())?;
                    Ok(cons(first.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBindings)
}

pub fn solvePolymorphicBindings(mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut info: SourceInfo, mut path: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>> {
    let mut solvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    let mut unsolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(bindings.clone(), metamodelica::nil(), metamodelica::nil())?;
    checkValidBindings(bindings.clone(), solvedBindings.clone(), unsolvedBindings.clone(), info.clone(), path.clone())?;
    Ok(solvedBindings)
}

fn checkValidBindings(mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut solvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut unsolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut info: SourceInfo, mut path: Arc<Absyn::Path>) -> Result<()> {
    let mut bindingsStr: ArcStr = arcstr::literal!("");
    let mut solvedBindingsStr: ArcStr = arcstr::literal!("");
    let mut unsolvedBindingsStr: ArcStr = arcstr::literal!("");
    let mut pathStr: ArcStr = arcstr::literal!("");
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    if !(unsolvedBindings.clone().is_empty()) {
        pathStr = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
        bindingsStr = (polymorphicBindingsStr(bindings.clone())).clone();
        solvedBindingsStr = (polymorphicBindingsStr(solvedBindings.clone())).clone();
        unsolvedBindingsStr = (polymorphicBindingsStr(unsolvedBindings.clone())).clone();
        Error::addSourceMessage(Error::META_UNSOLVED_POLYMORPHIC_BINDINGS.clone(), list![(pathStr.clone()).clone(), (bindingsStr.clone()).clone(), (solvedBindingsStr.clone()).clone(), (unsolvedBindingsStr.clone()).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

fn solvePolymorphicBindingsLoop(mut ibindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut isolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut iunsolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<(Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outSolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    let mut outUnsolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (outSolvedBindings, outUnsolvedBindings) = 'mc: {
        let __mc_input = (ibindings.clone(), isolvedBindings.clone(), iunsolvedBindings.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, solvedBindings, unsolvedBindings) => {
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (id, Deref @ metamodelica::List::Cons { head: ty, tail: Deref @ metamodelica::List::Nil }), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut ty = (*ty).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    ty = boxIfUnboxedType(ty.clone())?;
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(listAppend(unsolvedBindings.clone(), rest.clone()), cons((id.clone(), list![ty.clone()]), solvedBindings.clone()), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (id, tys), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut tys = (*tys).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    tys = replaceSolvedBindings(tys.clone(), solvedBindings.clone(), false)?;
                    tys = List::unionOnTrue(tys.clone(), metamodelica::nil(), (std::sync::Arc::new(equivtypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<bool> + 'static>));
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(listAppend(cons((id.clone(), tys.clone()), unsolvedBindings.clone()), rest.clone()), solvedBindings.clone(), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (id, tys), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut tys = (*tys).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    (tys, solvedBindings) = solveBindings(tys.clone(), tys.clone(), solvedBindings.clone())?;
                    tys = List::unionOnTrue(tys.clone(), metamodelica::nil(), (std::sync::Arc::new(equivtypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<bool> + 'static>));
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(listAppend(cons((id.clone(), tys.clone()), unsolvedBindings.clone()), rest.clone()), solvedBindings.clone(), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (id, tys), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut len1: i32 = 0;
                    let mut len2: i32 = 0;
                    let mut tys = (*tys).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    len1 = (tys.clone().len() as i32);
                    let true = (len1.clone() > 1) else { bail!("pattern mismatch") };
                    tys = List::unionOnTrue(tys.clone(), metamodelica::nil(), (std::sync::Arc::new(equivtypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<bool> + 'static>));
                    len2 = (tys.clone().len() as i32);
                    let false = (len1.clone() == len2.clone()) else { bail!("pattern mismatch") };
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(listAppend(cons((id.clone(), tys.clone()), unsolvedBindings.clone()), rest.clone()), solvedBindings.clone(), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: first, tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(rest.clone(), solvedBindings.clone(), cons(first.clone(), unsolvedBindings.clone()))?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSolvedBindings, outUnsolvedBindings))
}

fn solveBindings(mut itys1: Arc<metamodelica::List<Arc<DAE::Type>>>, mut itys2: Arc<metamodelica::List<Arc<DAE::Type>>>, mut isolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outTys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut outSolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (outTys, outSolvedBindings) = 'mc: {
        let __mc_input = (itys1.clone(), itys2.clone(), isolvedBindings.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ty1 @ Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id1 }, tail: _ }, Deref @ metamodelica::List::Cons { head: ty2 @ Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut fromOtherFunction: bool = false;
                    let mut solvedBindings = (*solvedBindings).clone();
                    let false = (id1.clone() == id2.clone()) else { bail!("pattern mismatch") };
                    fromOtherFunction = System::stringFind((id1.clone()).clone(), (literal!("$")).clone())? != -1;
                    id = (if (fromOtherFunction.clone()) {id1.clone()} else {id2.clone()}).clone();
                    ty = if (fromOtherFunction.clone()) {ty2.clone()} else {ty1.clone()};
                    if '__try0: {
                        let _ = unwrap_break_err!(polymorphicBindingsLookup((id.clone()).clone(), solvedBindings.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    solvedBindings = addPolymorphicBinding((id.clone()).clone(), ty.clone(), solvedBindings.clone())?;
                    Ok((cons(ty.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, tail: _ }, Deref @ metamodelica::List::Cons { head: ty2, tail: tys2 }, solvedBindings) => {
                    let mut solvedBindings = (*solvedBindings).clone();
                    let false = (isPolymorphic(ty2.clone())) else { bail!("pattern mismatch") };
                    if '__try0: {
                        let _ = unwrap_break_err!(polymorphicBindingsLookup((id.clone()).clone(), solvedBindings.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    solvedBindings = addPolymorphicBinding((id.clone()).clone(), ty2.clone(), solvedBindings.clone())?;
                    Ok((cons(ty2.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ty1, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, tail: tys2 }, solvedBindings) => {
                    let mut solvedBindings = (*solvedBindings).clone();
                    let false = (isPolymorphic(ty1.clone())) else { bail!("pattern mismatch") };
                    if '__try0: {
                        let _ = unwrap_break_err!(polymorphicBindingsLookup((id.clone()).clone(), solvedBindings.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    solvedBindings = addPolymorphicBinding((id.clone()).clone(), ty1.clone(), solvedBindings.clone())?;
                    Ok((cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METAOPTION { ty: ty1 }, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METAOPTION { ty: ty2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], list![ty2.clone()], solvedBindings.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty1 = __pa0.clone();
                    solvedBindings = __pa1.clone();
                    ty1 = Arc::new(DAE::Type::T_METAOPTION { ty: ty1.clone() });
                    Ok((cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METALIST { ty: ty1 }, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METALIST { ty: ty2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], list![ty2.clone()], solvedBindings.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty1 = __pa0.clone();
                    solvedBindings = __pa1.clone();
                    ty1 = Arc::new(DAE::Type::T_METALIST { ty: ty1.clone() });
                    Ok((cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METAARRAY { ty: ty1 }, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METAARRAY { ty: ty2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], list![ty2.clone()], solvedBindings.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty1 = __pa0.clone();
                    solvedBindings = __pa1.clone();
                    ty1 = Arc::new(DAE::Type::T_METAARRAY { ty: ty1.clone() });
                    Ok((cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METATUPLE { types: tys1 }, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METATUPLE { types: tys2 }, tail: rest }, solvedBindings) => {
                    let mut ty1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tys1 = (*tys1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    (tys1, solvedBindings) = solveBindingsThread(tys1.clone(), tys2.clone(), false, solvedBindings.clone())?;
                    ty1 = Arc::new(DAE::Type::T_METATUPLE { types: tys1.clone() });
                    Ok((cons(ty1.clone(), rest.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: args1, funcResultType: ty1, functionAttributes: functionAttributes1, path }, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: args2, funcResultType: ty2, functionAttributes: _, path: _ }, tail: rest }, solvedBindings) => {
                    let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut args1 = (*args1).clone();
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    tys1 = List::map(args1.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                    tys2 = List::map(args2.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(solveBindingsThread(cons(ty1.clone(), tys1.clone()), cons(ty2.clone(), tys2.clone()), false, solvedBindings.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty1 = __pa0.clone();
                    tys1 = __pa1.clone();
                    solvedBindings = __pa2.clone();
                    tys1 = List::map(tys1.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
                    args1 = List::threadMap(args1.clone(), tys1.clone(), (std::sync::Arc::new(setFuncArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Arc<DAE::Type>) -> Result<Arc<DAE::FuncArg>> + 'static>));
                    args1 = List::map(args1.clone(), (std::sync::Arc::new(clearDefaultBinding) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::FuncArg>> + 'static>));
                    ty1 = Arc::new(DAE::Type::T_FUNCTION { funcArg: args1.clone(), funcResultType: ty1.clone(), functionAttributes: functionAttributes1.clone(), path: path.clone() });
                    Ok((cons(ty1.clone(), rest.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tys1, Deref @ metamodelica::List::Cons { head: ty, tail: tys2 }, solvedBindings) => {
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut solvedBindings = (*solvedBindings).clone();
                    (tys, solvedBindings) = solveBindings(tys1.clone(), tys2.clone(), solvedBindings.clone())?;
                    Ok((cons(ty.clone(), tys.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outTys, outSolvedBindings))
}

fn solveBindingsThread(mut itys1: Arc<metamodelica::List<Arc<DAE::Type>>>, mut itys2: Arc<metamodelica::List<Arc<DAE::Type>>>, mut changed: bool, mut isolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Type>>>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>)> {
    let mut outTys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut outSolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    (outTys, outSolvedBindings) = 'mc: {
        let __mc_input = (itys1.clone(), itys2.clone(), changed.clone(), isolvedBindings.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ty1, tail: tys1 }, Deref @ metamodelica::List::Cons { head: ty2, tail: tys2 }, _, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut tys2 = (*tys2).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], list![ty2.clone()], solvedBindings.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty1 = __pa0.clone();
                    solvedBindings = __pa1.clone();
                    (tys2, solvedBindings) = solveBindingsThread(tys1.clone(), tys2.clone(), true, solvedBindings.clone())?;
                    Ok((cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ty1, tail: tys1 }, Deref @ metamodelica::List::Cons { head: _, tail: tys2 }, _, solvedBindings) => {
                    let mut tys2 = (*tys2).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    (tys2, solvedBindings) = solveBindingsThread(tys1.clone(), tys2.clone(), changed.clone(), solvedBindings.clone())?;
                    Ok((cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, true, solvedBindings) => {
                    Ok((metamodelica::nil(), solvedBindings.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outTys, outSolvedBindings))
}

fn replaceSolvedBindings(mut itys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut isolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>, mut changed: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Type>>>> {
    let mut outTys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    outTys = 'mc: {
        let __mc_input = (itys.clone(), isolvedBindings.clone(), changed.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, true) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ty, tail: tys }, solvedBindings, _) => {
                    let mut ty = (*ty).clone();
                    let mut tys = (*tys).clone();
                    ty = replaceSolvedBinding(ty.clone(), solvedBindings.clone())?;
                    tys = replaceSolvedBindings(tys.clone(), solvedBindings.clone(), true)?;
                    Ok(cons(ty.clone(), tys.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ty, tail: tys }, solvedBindings, _) => {
                    let mut tys = (*tys).clone();
                    tys = replaceSolvedBindings(tys.clone(), solvedBindings.clone(), changed.clone())?;
                    Ok(cons(ty.clone(), tys.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTys)
}

fn replaceSolvedBinding(mut ity: Arc<DAE::Type>, mut isolvedBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<Arc<DAE::Type>> {
    let mut outTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outTy = (::match_deref::match_deref! { match &((ity.clone(), isolvedBindings.clone())) {
        (Deref @ DAE::Type::T_METALIST { ty }, solvedBindings) => {
            let mut ty = (*ty).clone();
            ty = replaceSolvedBinding(ty.clone(), solvedBindings.clone())?;
            ty = Arc::new(DAE::Type::T_METALIST { ty: ty.clone() });
            ty.clone()
        },
        (Deref @ DAE::Type::T_METAARRAY { ty }, solvedBindings) => {
            let mut ty = (*ty).clone();
            ty = replaceSolvedBinding(ty.clone(), solvedBindings.clone())?;
            ty = Arc::new(DAE::Type::T_METAARRAY { ty: ty.clone() });
            ty.clone()
        },
        (Deref @ DAE::Type::T_METAOPTION { ty }, solvedBindings) => {
            let mut ty = (*ty).clone();
            ty = replaceSolvedBinding(ty.clone(), solvedBindings.clone())?;
            ty = Arc::new(DAE::Type::T_METAOPTION { ty: ty.clone() });
            ty.clone()
        },
        (Deref @ DAE::Type::T_METATUPLE { types: tys }, solvedBindings) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut tys = (*tys).clone();
            tys = replaceSolvedBindings(tys.clone(), solvedBindings.clone(), false)?;
            ty = Arc::new(DAE::Type::T_METATUPLE { types: tys.clone() });
            ty.clone()
        },
        (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, solvedBindings) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut tys = (*tys).clone();
            tys = replaceSolvedBindings(tys.clone(), solvedBindings.clone(), false)?;
            ty = Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: var_field!((*ity).names, DAE::Type::T_TUPLE).clone() });
            ty.clone()
        },
        (Deref @ DAE::Type::T_FUNCTION { funcArg: args, funcResultType: resType, functionAttributes, path }, solvedBindings) => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut args = (*args).clone();
            tys = List::map(args.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
            tys = replaceSolvedBindings(cons(resType.clone(), tys.clone()), solvedBindings.clone(), false)?;
            tys = List::map(tys.clone(), (std::sync::Arc::new(unboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::map(tys.clone(), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>))) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            tys = __pa1.clone();
            args = List::threadMap(args.clone(), tys.clone(), (std::sync::Arc::new(setFuncArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Arc<DAE::Type>) -> Result<Arc<DAE::FuncArg>> + 'static>));
            ty = makeRegularTupleFromMetaTupleOnTrue(isTuple(resType.clone()), ty.clone())?;
            ty = Arc::new(DAE::Type::T_FUNCTION { funcArg: args.clone(), funcResultType: ty.clone(), functionAttributes: functionAttributes.clone(), path: path.clone() });
            ty.clone()
        },
        (Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, solvedBindings) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(polymorphicBindingsLookup((id.clone()).clone(), solvedBindings.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            ty.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTy)
}

fn subtypePolymorphic(mut actual: Arc<DAE::Type>, mut expected: Arc<DAE::Type>, mut envPath: Option<Arc<Absyn::Path>>, mut inBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>> {
    let mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    bindings = 'mc: {
        let __mc_input = (actual.clone(), expected.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }) => {
                    Ok(addPolymorphicBinding(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone(), actual.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, _) => {
                    if stringGet((id.clone()).clone(),1)? != stringCharInt((literal!("$")).clone())? {
                        bail!("fail");
                    }
                    Ok(addPolymorphicBinding(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$$")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone(), expected.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: ty1 }, ty2) => {
                    let mut ty1 = (*ty1).clone();
                    ty1 = unboxedType(ty1.clone())?;
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, Deref @ DAE::Type::T_METABOXED { ty: ty2 }) => {
                    let mut ty2 = (*ty2).clone();
                    ty2 = unboxedType(ty2.clone())?;
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_NORETCALL, Deref @ DAE::Type::T_NORETCALL) => {
                    Ok(inBindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_STRING { .. }, Deref @ DAE::Type::T_STRING { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_BOOL { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { names: names1, .. }, Deref @ DAE::Type::T_ENUMERATION { names: names2, .. }) => {
                    let true = (List::isEqualOnTrue(names1.clone(), names2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(inBindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: ty1, .. }, Deref @ DAE::Type::T_ARRAY { ty: ty2, .. }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { ty: ty1 }, Deref @ DAE::Type::T_METAARRAY { ty: ty2 }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METALIST { ty: ty1 }, Deref @ DAE::Type::T_METALIST { ty: ty2 }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAOPTION { ty: ty1 }, Deref @ DAE::Type::T_METAOPTION { ty: ty2 }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: tList1 }, Deref @ DAE::Type::T_METATUPLE { types: tList2 }) => {
                    Ok(subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tList1, .. }, Deref @ DAE::Type::T_TUPLE { types: tList2, .. }) => {
                    Ok(subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { .. }, Deref @ DAE::Type::T_METAUNIONTYPE { .. }) => {
                    let true = (AbsynUtil::pathEqual(var_field!((*actual).path, DAE::Type::T_METAUNIONTYPE).clone(), var_field!((*expected).path, DAE::Type::T_METAUNIONTYPE).clone())) else { bail!("pattern mismatch") };
                    Ok(subtypePolymorphicList(var_field!((*actual).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), var_field!((*expected).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: path1 }, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: path2 }, .. }) => {
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    Ok(inBindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: farg1, funcResultType: ty1, functionAttributes: _, path: path1 }, Deref @ DAE::Type::T_FUNCTION { funcArg: farg2, funcResultType: ty2, functionAttributes: _, path: _ }) => {
                    let mut prefix: ArcStr = arcstr::literal!("");
                    let mut tList1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut tList2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut farg1 = (*farg1).clone();
                    let mut ty1 = (*ty1).clone();
                    let mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = bindings.clone();
                    if AbsynUtil::pathPrefixOf(Util::getOptionOrDefault(envPath.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("$TOP$")).clone() })), path1.clone()) {
                        tList1 = List::map(farg1.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                        tList2 = List::map(farg2.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                        bindings = subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?;
                        bindings = subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), bindings.clone())?;
                    } else {
                        prefix = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*AbsynUtil::pathString(path1.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone();
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseType(actual.clone(), (prefix.clone()).clone(), (std::sync::Arc::new(fnptr!(prefixTraversedPolymorphicType, Arc<DAE::Type>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, ArcStr) -> Result<(Arc<DAE::Type>, ArcStr)> + 'static>))?) {
                            (Deref @ DAE::Type::T_FUNCTION { funcArg: __pa0, funcResultType: __pa1, functionAttributes: _, path: _ }, _) => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        farg1 = __pa0.clone();
                        ty1 = __pa1.clone();
                        tList1 = List::map(farg1.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                        tList2 = List::map(farg2.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>));
                        bindings = subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?;
                        bindings = subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), bindings.clone())?;
                    }
                    Ok(bindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_UNKNOWN, ty2) => {
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut ids: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = bindings.clone();
                    tys = getAllInnerTypesOfType(ty2.clone(), (std::sync::Arc::new(fnptr!(isPolymorphic, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>));
                    ids = List::map(tys.clone(), (std::sync::Arc::new(polymorphicTypeName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>));
                    bindings = List::fold1(ids.clone(), (std::sync::Arc::new(addPolymorphicBinding) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>> + 'static>), actual.clone(), inBindings.clone());
                    Ok(bindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ANYTYPE { .. }, ty2) => {
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut ids: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut bindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = bindings.clone();
                    tys = getAllInnerTypesOfType(ty2.clone(), (std::sync::Arc::new(fnptr!(isPolymorphic, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>));
                    ids = List::map(tys.clone(), (std::sync::Arc::new(polymorphicTypeName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>));
                    bindings = List::fold1(ids.clone(), (std::sync::Arc::new(addPolymorphicBinding) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>> + 'static>), actual.clone(), inBindings.clone());
                    Ok(bindings.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(bindings)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subtypePolymorphicList(mut actual: Arc<metamodelica::List<Arc<DAE::Type>>>, mut expected: Arc<metamodelica::List<Arc<DAE::Type>>>, mut envPath: Option<Arc<Absyn::Path>>, mut ibindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>>> {
    let mut outBindings: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<Arc<DAE::Type>>>)>> = metamodelica::nil();
    outBindings = (::match_deref::match_deref! { match &((actual.clone(), expected.clone(), envPath.clone(), ibindings.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, bindings) => {
            bindings.clone()
        },
        (Deref @ metamodelica::List::Cons { head: ty1, tail: tList1 }, Deref @ metamodelica::List::Cons { head: ty2, tail: tList2 }, _, bindings) => {
            let mut bindings = (*bindings).clone();
            bindings = subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), bindings.clone())?;
            bindings = subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), bindings.clone())?;
            bindings.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBindings)
}

pub fn boxVarLst(mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut ovars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    ovars = (::match_deref::match_deref! { match &(vars.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name, attributes, ty: type_, binding, bind_from_outside: bdsrc, constOfForIteratorRange }, tail: rest } => {
            let mut type_ = (*type_).clone();
            let mut rest = (*rest).clone();
            type_ = boxIfUnboxedType(type_.clone())?;
            rest = boxVarLst(rest.clone())?;
            cons(Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: attributes.clone(), ty: type_.clone(), binding: binding.clone(), bind_from_outside: bdsrc.clone(), constOfForIteratorRange: constOfForIteratorRange.clone() }), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ovars)
}

pub fn liftArraySubscript(mut inType: Arc<DAE::Type>, mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inType.clone(), inSubscript.clone())) {
        (ty, Deref @ DAE::Subscript::WHOLE_NONEXP { exp: Deref @ DAE::Exp::ICONST { integer: i } }) => {
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() })] })
        },
        (ty, Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e }) => {
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_EXP { exp: e.clone() })] })
        },
        (ty, _) => {
            ty.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub fn liftArraySubscriptList(mut inType: Arc<DAE::Type>, mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &((inType.clone(), inSubscriptLst.clone())) {
        (ty, Deref @ metamodelica::List::Nil) => {
            ty.clone()
        },
        (ty, Deref @ metamodelica::List::Cons { head: sub, tail: rest }) => {
            liftArraySubscript(liftArraySubscriptList(ty.clone(), rest.clone())?, sub.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

pub fn convertTupleToMetaTuple(mut exp: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut oexp: Arc<DAE::Exp>;
    let mut oty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (oexp, oty) = (::match_deref::match_deref! { match &((exp.clone(), ty.clone())) {
        (Deref @ DAE::Exp::TUPLE { PR: _ }, _) => {
            (oexp, oty) = matchType(exp.clone(), ty.clone(), DAE::T_METABOXED_DEFAULT().clone(), false)?;
            (oexp.clone(), oty.clone())
        },
        _ => (exp.clone(), ty.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oexp, oty))
}

pub fn isFunctionType(mut ty: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn prefixTraversedPolymorphicType(mut ty: Type, mut prefix: ArcStr) -> (Type, ArcStr) {
    let mut oty: Type = ty.clone();
    let mut r#str: ArcStr = arcstr::literal!("");
    (oty, r#str) = (::match_deref::match_deref! { match &(oty.clone()) {
        Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
            assign_variant_field!(oty => DAE::Type::T_METAPOLYMORPHIC; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*var_field!((*oty).name, DAE::Type::T_METAPOLYMORPHIC).clone()); ArcStr::from(__mm_s) });
            (oty.clone(), prefix.clone())
        },
        _ => (ty.clone(), prefix.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, r#str)
}

pub fn makeExpDimensionsUnknown(mut ty: Arc<DAE::Type>, mut dummy: i32) -> (Arc<DAE::Type>, i32) {
    let mut oty: Arc<DAE::Type> = ty.clone();
    let mut odummy: i32 = dummy.clone();
    oty = (::match_deref::match_deref! { match &(oty.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)]);
            oty.clone()
        },
        _ => oty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, odummy)
}

pub fn makeKnownDimensionsInteger(mut ty: Arc<DAE::Type>, mut dummy: i32) -> (Arc<DAE::Type>, i32) {
    let mut oty: Arc<DAE::Type> = ty.clone();
    let mut odummy: i32 = dummy.clone();
    oty = (::match_deref::match_deref! { match &(oty.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_BOOLEAN, tail: Deref @ metamodelica::List::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 })]);
            oty.clone()
        },
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_ENUM { size, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: size.clone() })]);
            oty.clone()
        },
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { integer: size } }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: size.clone() })]);
            oty.clone()
        },
        _ => {
            oty.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, odummy)
}

pub fn traverseType<A: Clone + 'static>(mut ty: Arc<DAE::Type>, mut arg: A, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>) -> Result<(Arc<DAE::Type>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>;

    let mut oty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut a: A = arg.clone();
    (oty, a) = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_NORETCALL => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_UNKNOWN => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
            (ty.clone(), a.clone())
        },
        Deref @ DAE::Type::T_CODE { .. } => {
            (ty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_METABOXED { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METABOXED).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_METABOXED; ty = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_ARRAY { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_ARRAY).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_ARRAY; ty = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_METATYPE { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METATYPE).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_METATYPE; ty = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_METALIST { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METALIST).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_METALIST; ty = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_METAOPTION { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METAOPTION).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_METAOPTION; ty = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_METAARRAY { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METAARRAY).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_METAARRAY; ty = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).functionType, DAE::Type::T_FUNCTION_REFERENCE_VAR).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION_REFERENCE_VAR; functionType = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut oty = (*oty).clone();
            (tyInner, a) = traverseType(var_field!((*oty).functionType, DAE::Type::T_FUNCTION_REFERENCE_FUNC).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION_REFERENCE_FUNC; functionType = tyInner.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_METATUPLE { .. } => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut oty = (*oty).clone();
            (tys, a) = traverseTupleType(var_field!((*oty).types, DAE::Type::T_METATUPLE).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_METATUPLE; types = tys.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_TUPLE { .. } => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut oty = (*oty).clone();
            (tys, a) = traverseTupleType(var_field!((*oty).types, DAE::Type::T_TUPLE).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_TUPLE; types = tys.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_METARECORD { .. } => {
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut oty = (*oty).clone();
            (vars, a) = traverseVarTypes(var_field!((*oty).fields, DAE::Type::T_METARECORD).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_METARECORD; fields = vars.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut oty = (*oty).clone();
            (vars, a) = traverseVarTypes(var_field!((*oty).varLst, DAE::Type::T_COMPLEX).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_COMPLEX; varLst = vars.clone());
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut oty = (*oty).clone();
            (vars, a) = traverseVarTypes(var_field!((*oty).varLst, DAE::Type::T_SUBTYPE_BASIC).clone(), a.clone(), r#fn.clone())?;
            (tyInner, a) = traverseType(var_field!((*oty).complexType, DAE::Type::T_SUBTYPE_BASIC).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_SUBTYPE_BASIC;
                varLst = vars.clone(),
                complexType = tyInner.clone()
            );
            (oty.clone(), a.clone())
        },
        oty @ Deref @ DAE::Type::T_FUNCTION { .. } => {
            let mut tyInner: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut farg: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
            let mut oty = (*oty).clone();
            (farg, a) = traverseFuncArg(var_field!((*oty).funcArg, DAE::Type::T_FUNCTION).clone(), a.clone(), r#fn.clone())?;
            (tyInner, a) = traverseType(var_field!((*oty).funcResultType, DAE::Type::T_FUNCTION).clone(), a.clone(), r#fn.clone())?;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION;
                funcArg = farg.clone(),
                funcResultType = tyInner.clone()
            );
            (oty.clone(), a.clone())
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.traverseType not implemented correctly: ")); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, a) = r#fn(oty.clone(), a.clone())?;
    Ok((oty, a))
}

fn traverseTupleType<A: Clone + 'static>(mut itys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut ia: A, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>) -> Result<(Arc<metamodelica::List<Arc<DAE::Type>>>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>;

    let mut otys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut oa: A;
    (otys, oa) = (::match_deref::match_deref! { match &((itys.clone(), ia.clone(), r#fn.clone())) {
        (Deref @ metamodelica::List::Nil, a, _) => {
            (metamodelica::nil(), a.clone())
        },
        (Deref @ metamodelica::List::Cons { head: ty, tail: tys }, a, _) => {
            let mut ty = (*ty).clone();
            let mut tys = (*tys).clone();
            let mut a = (*a).clone();
            (ty, a) = traverseType(ty.clone(), a.clone(), r#fn.clone())?;
            (tys, a) = traverseTupleType(tys.clone(), a.clone(), r#fn.clone())?;
            (cons(ty.clone(), tys.clone()), a.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((otys, oa))
}

fn traverseVarTypes<A: Clone + 'static>(mut ivars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut ia: A, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>) -> Result<(Arc<metamodelica::List<Arc<DAE::Var>>>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>;

    let mut ovars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut oa: A;
    (ovars, oa) = (::match_deref::match_deref! { match &((ivars.clone(), ia.clone(), r#fn.clone())) {
        (Deref @ metamodelica::List::Nil, a, _) => {
            (metamodelica::nil(), a.clone())
        },
        (Deref @ metamodelica::List::Cons { head: var, tail: vars }, a, _) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut var = (*var).clone();
            let mut vars = (*vars).clone();
            let mut a = (*a).clone();
            ty = getVarType(var.clone())?;
            (ty, a) = traverseType(ty.clone(), a.clone(), r#fn.clone())?;
            var = setVarType(var.clone(), ty.clone());
            (vars, a) = traverseVarTypes(vars.clone(), a.clone(), r#fn.clone())?;
            (cons(var.clone(), vars.clone()), a.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((ovars, oa))
}

fn traverseFuncArg<A: Clone + 'static>(mut iargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut ia: A, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>) -> Result<(Arc<metamodelica::List<Arc<DAE::FuncArg>>>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, A) -> Result<(Arc<DAE::Type>, A)> + 'static>;

    let mut oargs: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let mut oa: A;
    (oargs, oa) = (::match_deref::match_deref! { match &((iargs.clone(), ia.clone())) {
        (Deref @ metamodelica::List::Nil, a) => {
            (metamodelica::nil(), a.clone())
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ DAE::FuncArg { .. }, tail: args }, a) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut arg = (*arg).clone();
            let mut args = (*args).clone();
            let mut a = (*a).clone();
            (ty, a) = traverseType(arg.ty.clone(), a.clone(), r#fn.clone())?;
            assign_field!(arg.ty = ty.clone());
            (args, a) = traverseFuncArg(args.clone(), a.clone(), r#fn.clone())?;
            (cons(arg.clone(), args.clone()), a.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oargs, oa))
}

pub fn makeRegularTupleFromMetaTupleOnTrue(mut b: bool, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut out: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    out = (::match_deref::match_deref! { match &((b.clone(), ty.clone())) {
        (true, Deref @ DAE::Type::T_METATUPLE { types: tys }) => {
            let mut tys = (*tys).clone();
            tys = List::mapMap(tys.clone(), (std::sync::Arc::new(unboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>), (std::sync::Arc::new(boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
            tys = List::map(tys.clone(), (std::sync::Arc::new(unboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
            Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None })
        },
        (false, _) => {
            ty.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn allTuple(mut itys: Arc<metamodelica::List<Arc<DAE::Type>>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(itys.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_TUPLE { .. }, tail: tys } => {
            allTuple(tys.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn unboxedFunctionType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: args1, funcResultType: ty1, functionAttributes, path } => {
            let mut tys1: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut args1 = (*args1).clone();
            let mut ty1 = (*ty1).clone();
            tys1 = List::mapMap(args1.clone(), (std::sync::Arc::new(funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>), (std::sync::Arc::new(unboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>));
            ty1 = unboxedType(ty1.clone())?;
            args1 = List::threadMap(args1.clone(), tys1.clone(), (std::sync::Arc::new(setFuncArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>, Arc<DAE::Type>) -> Result<Arc<DAE::FuncArg>> + 'static>));
            Arc::new(DAE::Type::T_FUNCTION { funcArg: args1.clone(), funcResultType: ty1.clone(), functionAttributes: functionAttributes.clone(), path: path.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

pub fn varHasMetaRecordType(mut var: Arc<DAE::Var>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_METARECORD { .. } }, .. } => true,
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_METARECORD { .. }, .. } => true,
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::META_RECORD { path: _ }, .. } }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn optInteger(mut inInt: Option<i32>) -> i32 {
    let mut outInt: i32 = 0;
    outInt = (match inInt.clone() {
        Some(mut i) => {
            i.clone()
        },
        _ => {
            -1
        },
    });
    outInt
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn typeToValue(mut inType: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut defaultValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    defaultValue = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("<EMPTY>")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { path, index: iOpt, .. } => {
                    let mut i: i32 = 0;
                    i = optInteger(iOpt.clone());
                    Ok(Arc::new(Values::Value::ENUM_LITERAL { name: path.clone(), index: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { varLst: vars, complexClassType: st, .. } => {
                    let mut comp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut path: Arc<Absyn::Path>;
                    let mut ordered: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    (ordered, comp) = varsToValues(vars.clone())?;
                    path = ClassInfUtil::getStateName(st.clone());
                    Ok(Arc::new(Values::Value::RECORD { record_: path.clone(), orderd: ordered.clone(), comp: comp.clone(), index: -1 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    v = typeToValue(t.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut valueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    v = typeToValue(t.clone())?;
                    valueLst = List::fill(v.clone(), i.clone());
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: valueLst.clone(), dimLst: list![i.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut valueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    valueLst = List::map(tys.clone(), (std::sync::Arc::new(typeToValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<Values::Value>> + 'static>));
                    v = Arc::new(Values::Value::TUPLE { valueLst: valueLst.clone() });
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN => {
                    Ok(Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Types.typeToValue failed on unhandled Type ")).clone())?;
                    s1 = (TypesDump::printTypeStr(inType.clone())?).clone();
                    Debug::traceln((s1.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(defaultValue)
}

pub fn varsToValues(mut inVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<(Arc<metamodelica::List<Arc<Values::Value>>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut outExpIdentLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outValuesValueLst, outExpIdentLst) = 'mc: {
        let __mc_input = inVarLst.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty: tp, name: id, .. }, tail: rest } => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut restVals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut restIds: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    v = typeToValue(tp.clone())?;
                    (restVals, restIds) = varsToValues(rest.clone())?;
                    Ok((cons(v.clone(), restVals.clone()), cons((id.clone()).clone(), restIds.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Types.varsToValues failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValuesValueLst, outExpIdentLst))
}

pub fn makeNthDimUnknown(mut ty: Arc<DAE::Type>, mut dim: i32) -> Result<Arc<DAE::Type>> {
    let mut oty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    oty = (::match_deref::match_deref! { match &((ty.clone(), dim.clone())) {
        (Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, 1) => {
            Arc::new(DAE::Type::T_ARRAY { ty: ty1.clone(), dims: list![Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)] })
        },
        (Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::List::Cons { head: ad, tail: Deref @ metamodelica::List::Nil } }, _) => {
            let mut ty1 = (*ty1).clone();
            ty1 = makeNthDimUnknown(ty1.clone(), dim.clone() - 1)?;
            Arc::new(DAE::Type::T_ARRAY { ty: ty1.clone(), dims: list![ad.clone()] })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oty)
}

pub fn arraySuperType(mut ity1: Arc<DAE::Type>, mut info: SourceInfo, mut ity2: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = 'mc: {
        let __mc_input = (ity1.clone(), info.clone(), ity2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, _, ty2) => {
                    let mut ty1 = (*ty1).clone();
                    let true = (isInteger(arrayElementType(ty1.clone()))) else { bail!("pattern mismatch") };
                    let true = (isReal(arrayElementType(ty2.clone()))) else { bail!("pattern mismatch") };
                    (ty1, _) = traverseType(ty1.clone(), -1, (std::sync::Arc::new(fnptr!(replaceIntegerTypeWithReal, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
                    let true = (subtype(ty1.clone(), ty2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(ty1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, _, ty2) => {
                    let mut ty2 = (*ty2).clone();
                    let true = (isInteger(arrayElementType(ty2.clone()))) else { bail!("pattern mismatch") };
                    let true = (isReal(arrayElementType(ty1.clone()))) else { bail!("pattern mismatch") };
                    (ty2, _) = traverseType(ty2.clone(), -1, (std::sync::Arc::new(fnptr!(replaceIntegerTypeWithReal, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
                    let true = (subtype(ty1.clone(), ty2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(ty1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, _, ty2) => {
                    let true = (subtype(ty1.clone(), ty2.clone(), true)?) else { bail!("pattern mismatch") };
                    Ok(ty1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, _, ty2) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    str1 = (TypesDump::unparseType(ty1.clone())?).clone();
                    str2 = (TypesDump::unparseType(ty2.clone())?).clone();
                    typeErrorSanityCheck((str1.clone()).clone(), (str2.clone()).clone(), info.clone())?;
                    Error::addSourceMessage(Error::ARRAY_TYPE_MISMATCH.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ty)
}

fn replaceIntegerTypeWithReal(mut ty: Type, mut dummy: i32) -> (Type, i32) {
    let mut oty: Type = Arc::new(DAE::Type::T_NORETCALL);
    let mut odummy: i32 = dummy.clone();
    oty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => DAE::T_REAL_DEFAULT().clone(),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, odummy)
}

pub fn isZeroLengthArray(mut ty: Arc<DAE::Type>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims, .. } => {
            res = List::fold(dims.clone(), (std::sync::Arc::new(fnptr!(isZeroDim, Arc<DAE::Dimension>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, bool) -> Result<bool> + 'static>), false);
            res.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isZeroDim(mut dim: Arc<DAE::Dimension>, mut acc: bool) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((dim.clone(), acc.clone())) {
        (Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, _) => true,
        (Deref @ DAE::Dimension::DIM_ENUM { size: 0, .. }, _) => true,
        _ => acc.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn variabilityToConst(mut variability: SCode::Variability) -> Result<DAE::Const> {
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    r#const = (match variability.clone() {
        SCode::Variability::VAR => openmodelica_frontend_types::DAE::Const::C_VAR,
        SCode::Variability::DISCRETE => openmodelica_frontend_types::DAE::Const::C_VAR,
        SCode::Variability::PARAM => openmodelica_frontend_types::DAE::Const::C_PARAM,
        SCode::Variability::CONST => openmodelica_frontend_types::DAE::Const::C_CONST,
        _ => bail!("match: no arm matched"),
    });
    Ok(r#const)
}

pub fn varKindToConst(mut varKind: DAE::VarKind) -> Result<DAE::Const> {
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    r#const = (match varKind.clone() {
        DAE::VarKind::VARIABLE => openmodelica_frontend_types::DAE::Const::C_VAR,
        DAE::VarKind::DISCRETE => openmodelica_frontend_types::DAE::Const::C_VAR,
        DAE::VarKind::PARAM => openmodelica_frontend_types::DAE::Const::C_PARAM,
        DAE::VarKind::CONST => openmodelica_frontend_types::DAE::Const::C_CONST,
        _ => bail!("match: no arm matched"),
    });
    Ok(r#const)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isValidFunctionVarType(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsValid: bool = false;
    outIsValid = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: state, .. } => {
            isValidFunctionVarState(state.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            isValidFunctionVarType(ty.clone())
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsValid
}

fn isValidFunctionVarState(mut inState: ClassInf::State) -> bool {
    let mut outIsValid: bool = false;
    outIsValid = (match inState.clone() {
        ClassInf::State::MODEL { .. } => false,
        ClassInf::State::BLOCK { .. } => false,
        ClassInf::State::CONNECTOR { .. } => false,
        ClassInf::State::OPTIMIZATION { .. } => false,
        ClassInf::State::PACKAGE { .. } => false,
        _ => true,
    });
    outIsValid
}

fn makeDummyExpFromType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            Arc::new(DAE::Exp::ICONST { integer: 0 })
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            getNthEnumLiteral(inType.clone(), 1)?
        },
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, ty } => {
            let mut idim: i32 = 0;
            let mut exp: Arc<DAE::Exp>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            idim = Expression::dimensionSize(dim.clone())?;
            exp = makeDummyExpFromType(ty.clone())?;
            ety = Expression::r#typeof(exp.clone())?;
            ety = Expression::liftArrayLeft(ety.clone(), dim.clone());
            expl = List::fill(exp.clone(), idim.clone());
            Arc::new(DAE::Exp::ARRAY { ty: ety.clone(), scalar: true, array: expl.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn printExpTypeStr(mut iet: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (TypesDump::printTypeStr(expTypetoTypesType(iet.clone())?)?).clone();
    Ok(r#str)
}

pub fn isUnknownType(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_UNKNOWN => true,
        Deref @ DAE::Type::T_ANYTYPE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isOverdeterminedType(mut inType: Arc<DAE::Type>) -> Result<bool> {
    let mut outIsOverdetermined: bool = false;
    outIsOverdetermined = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { equalityConstraint: Some(_), complexClassType: cct, .. } => {
            ClassInfUtil::isTypeOrRecord(cct.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
            true
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIsOverdetermined)
}

pub fn hasMetaArray(mut ty: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    (_, b) = traverseType(ty.clone(), false, (std::sync::Arc::new(fnptr!(hasMetaArrayWork, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, bool) -> Result<(Arc<DAE::Type>, bool)> + 'static>))?;
    Ok(b)
}

fn hasMetaArrayWork(mut ty: Type, mut b: bool) -> (Type, bool) {
    let mut oty: Type = ty.clone();
    let mut ob: bool = b.clone();
    if !(b.clone()) {
        ob = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    (oty, ob)
}

fn classTypeEqualIfRecord(mut st1: ClassInf::State, mut st2: ClassInf::State) -> bool {
    let mut b: bool = false;
    b = (match (st1.clone(), st2.clone()) {
        (ClassInf::State::RECORD { path: ref p1 }, ClassInf::State::RECORD { path: ref p2 }) => {
            AbsynUtil::pathEqual(p1.clone(), p2.clone())
        },
        _ => {
            true
        },
    });
    b
}

pub fn ifExpMakeDimsUnknown(mut ty1: Arc<DAE::Type>, mut ty2: Arc<DAE::Type>) -> (Arc<DAE::Type>, Arc<DAE::Type>) {
    let mut oty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut oty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (oty1, oty2) = (::match_deref::match_deref! { match &((ty1.clone(), ty2.clone())) {
        (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil }, ty: inner1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, ty: inner2 }) => {
            (oty1, oty2) = ifExpMakeDimsUnknown(inner1.clone(), inner2.clone());
            (Arc::new(DAE::Type::T_ARRAY { ty: inner1.clone(), dims: cons(Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN), metamodelica::nil()) }), Arc::new(DAE::Type::T_ARRAY { ty: inner2.clone(), dims: cons(Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN), metamodelica::nil()) }))
        },
        (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, ty: inner1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil }, ty: inner2 }) => {
            (oty1, oty2) = ifExpMakeDimsUnknown(inner1.clone(), inner2.clone());
            (Arc::new(DAE::Type::T_ARRAY { ty: inner1.clone(), dims: cons(Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN), metamodelica::nil()) }), Arc::new(DAE::Type::T_ARRAY { ty: inner2.clone(), dims: cons(Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN), metamodelica::nil()) }))
        },
        (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d1, tail: Deref @ metamodelica::List::Nil }, ty: inner1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: d2, tail: Deref @ metamodelica::List::Nil }, ty: inner2 }) => {
            (oty1, oty2) = ifExpMakeDimsUnknown(inner1.clone(), inner2.clone());
            (Arc::new(DAE::Type::T_ARRAY { ty: inner1.clone(), dims: list![d1.clone()] }), Arc::new(DAE::Type::T_ARRAY { ty: inner2.clone(), dims: list![d2.clone()] }))
        },
        _ => {
            (ty1.clone(), ty2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty1, oty2)
}

pub fn isFixedWithNoBinding(mut inTy: Arc<DAE::Type>, mut inVariability: SCode::Variability) -> Result<bool> {
    let mut outFixed: bool = false;
    outFixed = 'mc: {
        let __mc_input = (inTy.clone(), inVariability.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut b: bool = false;
                    b = getFixedVarAttribute(inTy.clone())?;
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst: vl, .. }, _) => {
                    let true = (allHaveBindings(vl.clone())?) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool = false;
                    b = listMember(inVariability.clone(), list![openmodelica_frontend_types::SCode::Variability::PARAM, openmodelica_frontend_types::SCode::Variability::CONST]);
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFixed)
}

pub fn allHaveBindings(mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = inVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: _ } => {
                    let false = (hasBinding(v.clone())) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: rest } => {
                    let true = (hasBinding(v.clone())) else { bail!("pattern mismatch") };
                    let true = (allHaveBindings(rest.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(b)
}

pub fn hasBinding(mut inVar: Arc<DAE::Var>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::UNBOUND, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn typeErrorSanityCheck(mut inType1: ArcStr, mut inType2: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inType1.clone(), inType2.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (stringEq((inType1.clone()).clone(), (inType2.clone()).clone())) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addSourceMessage(Error::ERRONEOUS_TYPE_ERROR.clone(), list![(inType1.clone()).clone()], inInfo.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dimNotFixed(mut dim: Arc<DAE::Dimension>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_UNKNOWN => true,
        Deref @ DAE::Dimension::DIM_EXP { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isArrayWithUnknownDimension(mut ty: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => {
        let mut __acc: Option<bool> = None;
        for mut d in (TypesDump::getDimensions(ty.clone())).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(d.clone()) {
        Deref @ DAE::Dimension::DIM_UNKNOWN => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction")).unwrap()
    },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn setTypeVars(mut ty: Arc<DAE::Type>, mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Arc<DAE::Type>> {
    let mut ty: Arc<DAE::Type> = ty;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            assign_variant_field!(ty => DAE::Type::T_REAL; varLst = inVars.clone());
            ty.clone()
        },
        Deref @ DAE::Type::T_INTEGER { .. } => {
            assign_variant_field!(ty => DAE::Type::T_INTEGER; varLst = inVars.clone());
            ty.clone()
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            assign_variant_field!(ty => DAE::Type::T_STRING; varLst = inVars.clone());
            ty.clone()
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            assign_variant_field!(ty => DAE::Type::T_BOOL; varLst = inVars.clone());
            ty.clone()
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            assign_variant_field!(ty => DAE::Type::T_CLOCK; varLst = inVars.clone());
            ty.clone()
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            assign_variant_field!(ty => DAE::Type::T_ENUMERATION; attributeLst = inVars.clone());
            ty.clone()
        },
        Deref @ DAE::Type::T_ARRAY { .. } => {
            assign_variant_field!(ty => DAE::Type::T_ARRAY; ty = setTypeVars(var_field!((*ty).ty, DAE::Type::T_ARRAY).clone(), inVars.clone())?);
            ty.clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => {
            assign_variant_field!(ty => DAE::Type::T_SUBTYPE_BASIC; complexType = setTypeVars(var_field!((*ty).complexType, DAE::Type::T_SUBTYPE_BASIC).clone(), inVars.clone())?);
            ty.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ty)
}

pub fn isEmptyOrNoRetcall(mut ty: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Nil, .. } => true,
        Deref @ DAE::Type::T_METATUPLE { types: Deref @ metamodelica::List::Nil } => true,
        Deref @ DAE::Type::T_NORETCALL => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn typeConvertIntToEnumCheck(mut exp: Arc<DAE::Exp>, mut expected: Arc<DAE::Type>) -> Result<bool> {
    let mut conversionOK: bool = false;
    conversionOK = 'mc: {
        let __mc_input = (exp.clone(), expected.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_ENUMERATION { names: l, path: tp, .. }) => {
                    let mut pathStr: ArcStr = arcstr::literal!("");
                    let mut intStr: ArcStr = arcstr::literal!("");
                    let mut enumConst: ArcStr = arcstr::literal!("");
                    let true = (1 <= oi.clone() && oi.clone() <= (l.clone().len() as i32)) else { bail!("pattern mismatch") };
                    pathStr = (AbsynUtil::pathString(tp.clone(), (literal!(".")).clone(), true, false)?).clone();
                    intStr = (intString(oi.clone())).clone();
                    enumConst = ((l.clone()).get(oi.clone())?).clone();
                    Error::addMessage(Error::INTEGER_ENUMERATION_CONVERSION_WARNING.clone(), list![(intStr.clone()).clone(), (pathStr.clone()).clone(), (enumConst.clone()).clone()])?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_ENUMERATION { names: l, path: tp, .. }) => {
                    let mut pathStr: ArcStr = arcstr::literal!("");
                    let mut intStr: ArcStr = arcstr::literal!("");
                    let mut lengthStr: ArcStr = arcstr::literal!("");
                    pathStr = (AbsynUtil::pathString(tp.clone(), (literal!(".")).clone(), true, false)?).clone();
                    let false = (stringEq((pathStr.clone()).clone(), (literal!("")).clone())) else { bail!("pattern mismatch") };
                    intStr = (intString(oi.clone())).clone();
                    lengthStr = (intString((l.clone().len() as i32))).clone();
                    Error::addMessage(Error::INTEGER_ENUMERATION_OUT_OF_RANGE.clone(), list![(pathStr.clone()).clone(), (intStr.clone()).clone(), (lengthStr.clone()).clone()])?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_ENUMERATION { path: tp, .. }) => {
                    let mut pathStr: ArcStr = arcstr::literal!("");
                    let mut intStr: ArcStr = arcstr::literal!("");
                    pathStr = (AbsynUtil::pathString(tp.clone(), (literal!(".")).clone(), true, false)?).clone();
                    let true = (stringEq((pathStr.clone()).clone(), (literal!("")).clone())) else { bail!("pattern mismatch") };
                    intStr = (intString(oi.clone())).clone();
                    Error::addMessage(Error::INTEGER_TO_UNKNOWN_ENUMERATION.clone(), list![(intStr.clone()).clone()])?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(conversionOK)
}

pub fn findVarIndex(mut id: ArcStr, mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>) -> i32 {
    let mut index: i32 = 0;
    index = List::position1OnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(selectVar, Arc<DAE::Var>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, ArcStr) -> Result<bool> + 'static>), (id.clone()).clone()) - 1;
    index
}

fn selectVar(mut var: Arc<DAE::Var>, mut id: ArcStr) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { name: id1, .. } => {
            stringEq((id.clone()).clone(), (id1.clone()).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn getUniontypeIfMetarecord(mut inTy: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = (::match_deref::match_deref! { match &(inTy.clone()) {
        Deref @ DAE::Type::T_METARECORD { knownSingleton: b, utPath: p, .. } => {
            Arc::new(DAE::Type::T_METAUNIONTYPE { paths: metamodelica::nil(), typeVars: var_field!((*inTy).typeVars, DAE::Type::T_METARECORD).clone(), knownSingleton: b.clone(), singletonType: if (b.clone()) {Arc::new(DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: inTy.clone() })} else {Arc::new(openmodelica_frontend_types::DAE::EvaluateSingletonType::NOT_SINGLETON)}, path: p.clone() })
        },
        _ => {
            inTy.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn getUniontypeIfMetarecordReplaceAllSubtypes(mut inTy: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (ty, _) = traverseType(inTy.clone(), 1, (std::sync::Arc::new(fnptr!(getUniontypeIfMetarecordTraverse, Arc<DAE::Type>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, i32) -> Result<(Arc<DAE::Type>, i32)> + 'static>))?;
    Ok(ty)
}

fn getUniontypeIfMetarecordTraverse(mut ty: Arc<DAE::Type>, mut dummy: i32) -> (Arc<DAE::Type>, i32) {
    let mut oty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut odummy: i32 = dummy.clone();
    oty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METARECORD { .. } => Arc::new(DAE::Type::T_METAUNIONTYPE { paths: metamodelica::nil(), typeVars: var_field!((*ty).typeVars, DAE::Type::T_METARECORD).clone(), knownSingleton: var_field!((*ty).knownSingleton, DAE::Type::T_METARECORD).clone(), singletonType: if (var_field!((*ty).knownSingleton, DAE::Type::T_METARECORD).clone()) {Arc::new(DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: ty.clone() })} else {Arc::new(openmodelica_frontend_types::DAE::EvaluateSingletonType::NOT_SINGLETON)}, path: var_field!((*ty).utPath, DAE::Type::T_METARECORD).clone() }),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, odummy)
}

fn isBuiltin(mut a: DAE::FunctionBuiltin) -> bool {
    let mut b: bool = false;
    b = (match a.clone() {
        DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN => false,
        _ => true,
    });
    b
}

pub fn makeCallAttr(mut ty: Arc<DAE::Type>, mut attr: DAE::FunctionAttributes) -> Arc<DAE::CallAttributes> {
    let mut callAttr: Arc<DAE::CallAttributes> = Arc::new(<DAE::CallAttributes as ::std::default::Default>::default());
    let mut isImpure: bool = false;
    let mut isT: bool = false;
    let mut isB: bool = false;
    isT = isTuple(ty.clone());
    isB = isBuiltin(attr.isBuiltin.clone());
    isImpure = attr.purity.clone() == DAE::Purity::IMPURE.clone();
    callAttr = Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: isT.clone(), builtin: isB.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: attr.inline.clone(), tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    callAttr
}

pub fn builtinName(mut isbuiltin: DAE::FunctionBuiltin) -> Option<ArcStr> {
    let mut name: Option<ArcStr> = None;
    name = (match isbuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. } => var_field!(isbuiltin.name, DAE::FunctionBuiltin::FUNCTION_BUILTIN).clone(),
        _ => None,
    });
    name
}

pub fn getFuncArg(mut ty: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::FuncArg>>>> {
    let mut args: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    Ok(args)
}

pub fn isArray1D(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, .. } => {
            !(arrayType(ty.clone()))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isArray2D(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, .. }, .. } => {
            !(arrayType(ty.clone()))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn funcArgName(mut arg: Arc<DAE::FuncArg>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::FuncArg { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn funcArgType(mut arg: Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let __pa0 = ::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::FuncArg { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    Ok(ty)
}

pub fn funcArgDefaultBinding(mut arg: Arc<DAE::FuncArg>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut defaultBinding: Option<Arc<DAE::Exp>> = None;
    let __pa0 = ::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::FuncArg { defaultBinding: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    defaultBinding = __pa0.clone();
    Ok(defaultBinding)
}

pub fn setFuncArgType(mut arg: Arc<DAE::FuncArg>, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::FuncArg>> {
    let mut outArg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let mut name: ArcStr = arcstr::literal!("");
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    let mut par: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    let mut defaultBinding: Option<Arc<DAE::Exp>> = None;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::FuncArg { name: __pa0, ty: _, r#const: __pa1, par: __pa2, defaultBinding: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    r#const = __pa1.clone();
    par = __pa2.clone();
    defaultBinding = __pa3.clone();
    outArg = Arc::new(DAE::FuncArg { name: (name.clone()).clone(), ty: ty.clone(), r#const: r#const.clone(), par: par.clone(), defaultBinding: defaultBinding.clone() });
    Ok(outArg)
}

pub fn setFuncArgName(mut arg: Arc<DAE::FuncArg>, mut name: ArcStr) -> Result<Arc<DAE::FuncArg>> {
    let mut outArg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    let mut par: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    let mut defaultBinding: Option<Arc<DAE::Exp>> = None;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::FuncArg { name: _, ty: __pa0, r#const: __pa1, par: __pa2, defaultBinding: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    r#const = __pa1.clone();
    par = __pa2.clone();
    defaultBinding = __pa3.clone();
    outArg = Arc::new(DAE::FuncArg { name: (name.clone()).clone(), ty: ty.clone(), r#const: r#const.clone(), par: par.clone(), defaultBinding: defaultBinding.clone() });
    Ok(outArg)
}

pub fn clearDefaultBinding(mut arg: Arc<DAE::FuncArg>) -> Result<Arc<DAE::FuncArg>> {
    let mut outArg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    let mut name: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    let mut par: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::FuncArg { name: __pa0, ty: __pa1, r#const: __pa2, par: __pa3, defaultBinding: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    r#const = __pa2.clone();
    par = __pa3.clone();
    outArg = Arc::new(DAE::FuncArg { name: (name.clone()).clone(), ty: ty.clone(), r#const: r#const.clone(), par: par.clone(), defaultBinding: None });
    Ok(outArg)
}

pub fn makeDefaultFuncArg(mut name: ArcStr, mut ty: Arc<DAE::Type>) -> Arc<DAE::FuncArg> {
    let mut arg: Arc<DAE::FuncArg> = Arc::new(<DAE::FuncArg as ::std::default::Default>::default());
    arg = Arc::new(DAE::FuncArg { name: (name.clone()).clone(), ty: ty.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None });
    arg
}

pub fn setIsFunctionPointer(mut ty: Arc<DAE::Type>, mut dummy: i32) -> (Arc<DAE::Type>, i32) {
    let mut oty: Arc<DAE::Type> = ty.clone();
    let mut odummy: i32 = dummy.clone();
    oty = (::match_deref::match_deref! { match &(oty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { functionAttributes: attr @ DAE::FunctionAttributes { isFunctionPointer: false, .. }, .. } => {
            let mut attr = (*attr).clone();
            attr.isFunctionPointer = true;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION; functionAttributes = attr.clone());
            oty.clone()
        },
        _ => {
            oty.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, odummy)
}

pub fn isFunctionReferenceVar(mut ty: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isFunctionPointer(mut inType: Arc<DAE::Type>) -> bool {
    let mut outIsFunPtr: bool = false;
    outIsFunPtr = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isFunctionPointer: true, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsFunPtr
}

pub fn filterRecordComponents(mut inRecordVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut outRecordVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    outRecordVars = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
        for mut v in (inRecordVars.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Var { .. } => {
            if !(allowedInRecord(v.ty.clone())?) {
                Error::addSourceMessage(Error::ILLEGAL_RECORD_COMPONENT.clone(), list![(TypesDump::unparseVar(v.clone())?).clone()], inInfo.clone())?;
                bail!("fail");
            }
            v.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outRecordVars)
}

pub fn allowedInRecord(mut ty: Arc<DAE::Type>) -> Result<bool> {
    let mut yes: bool = false;
    yes = 'mc: {
        let __mc_input = ty.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    t = arrayElementType(ty.clone());
                    let true = (basicType(t.clone()) || isRecord(t.clone()) || extendsBasicType(t.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
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
    Ok(yes)
}

pub fn lookupIndexInMetaRecord(mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut name: ArcStr) -> i32 {
    let mut index: i32 = 0;
    index = List::position1OnTrue(vars.clone(), (std::sync::Arc::new(DAEUtil::typeVarIdentEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone());
    index
}

pub fn checkEnumDuplicateLiterals(mut names: Arc<metamodelica::List<ArcStr>>, mut info: SourceInfo) -> Result<()> {
    let mut sortedNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    sortedNames = List::sort(names.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
    if !(List::sortedListAllUnique(sortedNames.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?) {
        Error::addSourceMessage(Error::ENUM_DUPLICATES.clone(), list![stringDelimitList(List::sortedUniqueOnlyDuplicates(sortedNames.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, (literal!(",")).clone()), stringDelimitList(names.clone(), (literal!(",")).clone())], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn checkTypeCompat(mut inExp1: Arc<DAE::Exp>, mut inType1: Arc<DAE::Type>, mut inExp2: Arc<DAE::Exp>, mut inType2: Arc<DAE::Type>, mut inAllowUnknown: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Type>, bool)> {
    let mut outExp1: Arc<DAE::Exp> = inExp1.clone();
    let mut outExp2: Arc<DAE::Exp> = inExp2.clone();
    let mut outCompatType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outCompatible: bool = true;
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if referenceEq(&inType1.clone(),&inType2.clone()) {
        outCompatType = inType1.clone();
        return Ok((outExp1, outExp2, outCompatType, outCompatible));
    }
    if metamodelica::valueConstructor((&*inType1.clone()))? != metamodelica::valueConstructor((&*inType2.clone()))? {
        if extendsBasicType(inType1.clone()) || extendsBasicType(inType2.clone()) {
            ty1 = derivedBasicType(inType1.clone());
            ty2 = derivedBasicType(inType2.clone());
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), ty1.clone(), inExp2.clone(), ty2.clone(), false)?;
        } else {
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat_cast(inExp1.clone(), inType1.clone(), inExp2.clone(), inType2.clone(), inAllowUnknown.clone())?;
        }
        return Ok((outExp1, outExp2, outCompatType, outCompatible));
    }
    outCompatType = (::match_deref::match_deref! { match &(inType1.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            DAE::T_INTEGER_DEFAULT().clone()
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            DAE::T_REAL_DEFAULT().clone()
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            DAE::T_STRING_DEFAULT().clone()
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            DAE::T_BOOL_DEFAULT().clone()
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            DAE::T_CLOCK_DEFAULT().clone()
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*inType1).complexType, DAE::Type::T_SUBTYPE_BASIC).clone(), inExp2.clone(), ty.clone(), false)?;
            outCompatType.clone()
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_ENUMERATION { names: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            names = __pa0.clone();
            outCompatible = List::isEqualOnTrue(var_field!((*inType1).names, DAE::Type::T_ENUMERATION).clone(), names.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>));
            inType1.clone()
        },
        Deref @ DAE::Type::T_ARRAY { .. } => {
            let mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut ety1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ety2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ety1 = arrayElementType(inType1.clone());
            ety2 = arrayElementType(inType2.clone());
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), ety1.clone(), inExp2.clone(), ety2.clone(), false)?;
            if outCompatible.clone() {
                dims1 = TypesDump::getDimensions(inType1.clone());
                dims2 = TypesDump::getDimensions(inType2.clone());
                if (dims1.clone().len() as i32) == (dims2.clone().len() as i32) {
                    dims1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for (dim1, dim2) in (&(dims1.clone())).into_iter().zip((&(dims2.clone())).into_iter()) {
            let __x = if (Expression::dimensionsKnownAndEqual(dim1.clone(), dim2.clone())?) {dim1.clone()} else {Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    outCompatType = liftArrayListDims(outCompatType.clone(), dims1.clone());
                } else {
                    outCompatible = false;
                }
            }
            outCompatType.clone()
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => {
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            vars = __pa0.clone();
            outCompatible = List::isEqualOnTrue(var_field!((*inType1).varLst, DAE::Type::T_COMPLEX).clone(), vars.clone(), (std::sync::Arc::new(varEqualName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::Var>) -> Result<bool> + 'static>));
            inType1.clone()
        },
        Deref @ DAE::Type::T_FUNCTION { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut args: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut tys2: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_FUNCTION { funcArg: __pa0, funcResultType: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            args = __pa0.clone();
            ty = __pa1.clone();
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*inType1).funcResultType, DAE::Type::T_FUNCTION).clone(), inExp2.clone(), ty.clone(), false)?;
            if outCompatible.clone() {
                tys = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut arg in (var_field!((*inType1).funcArg, DAE::Type::T_FUNCTION).clone()).into_iter().cloned() {
            let __x = funcArgType(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                tys2 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            let __x = funcArgType(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                (_, outCompatible) = checkTypeCompatList(inExp1.clone(), tys.clone(), inExp2.clone(), tys2.clone())?;
            }
            inType1.clone()
        },
        Deref @ DAE::Type::T_TUPLE { .. } => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_TUPLE { types: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            tys = __pa0.clone();
            (tys, outCompatible) = checkTypeCompatList(inExp1.clone(), var_field!((*inType1).types, DAE::Type::T_TUPLE).clone(), inExp2.clone(), tys.clone())?;
            Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: var_field!((*inType1).names, DAE::Type::T_TUPLE).clone() })
        },
        Deref @ DAE::Type::T_METALIST { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METALIST { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*inType1).ty, DAE::Type::T_METALIST).clone(), inExp2.clone(), ty.clone(), true)?;
            Arc::new(DAE::Type::T_METALIST { ty: outCompatType.clone() })
        },
        Deref @ DAE::Type::T_METAARRAY { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METAARRAY { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*inType1).ty, DAE::Type::T_METAARRAY).clone(), inExp2.clone(), ty.clone(), true)?;
            Arc::new(DAE::Type::T_METAARRAY { ty: outCompatType.clone() })
        },
        Deref @ DAE::Type::T_METAOPTION { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METAOPTION { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*inType1).ty, DAE::Type::T_METAOPTION).clone(), inExp2.clone(), ty.clone(), true)?;
            Arc::new(DAE::Type::T_METAOPTION { ty: outCompatType.clone() })
        },
        Deref @ DAE::Type::T_METATUPLE { .. } => {
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METATUPLE { types: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            tys = __pa0.clone();
            (tys, outCompatible) = checkTypeCompatList(inExp1.clone(), var_field!((*inType1).types, DAE::Type::T_METATUPLE).clone(), inExp2.clone(), tys.clone())?;
            Arc::new(DAE::Type::T_METATUPLE { types: tys.clone() })
        },
        Deref @ DAE::Type::T_METABOXED { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METABOXED { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*inType1).ty, DAE::Type::T_METABOXED).clone(), inExp2.clone(), ty.clone(), false)?;
            Arc::new(DAE::Type::T_METABOXED { ty: outCompatType.clone() })
        },
        Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { name: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            outCompatible = var_field!((*inType1).name, DAE::Type::T_METAPOLYMORPHIC).clone() == name.clone();
            inType1.clone()
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { path: p1, .. } => {
            let mut p2: Arc<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METAUNIONTYPE { path: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p2 = __pa0.clone();
            outCompatible = AbsynUtil::pathEqual(p1.clone(), p2.clone());
            inType1.clone()
        },
        Deref @ DAE::Type::T_METARECORD { utPath: p1, .. } => {
            let mut p2: Arc<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_METARECORD { utPath: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p2 = __pa0.clone();
            outCompatible = AbsynUtil::pathEqual(p1.clone(), p2.clone());
            inType1.clone()
        },
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. } => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let __pa0 = ::match_deref::match_deref! { match &(inType2.clone()) {
                Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*inType1).functionType, DAE::Type::T_FUNCTION_REFERENCE_VAR).clone(), inExp2.clone(), ty.clone(), false)?;
            Arc::new(DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: outCompatType.clone() })
        },
        _ => {
            outCompatible = false;
            DAE::T_UNKNOWN_DEFAULT().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp1, outExp2, outCompatType, outCompatible))
}

fn checkTypeCompatList(mut inExp1: Arc<DAE::Exp>, mut inTypes1: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inExp2: Arc<DAE::Exp>, mut inTypes2: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Type>>>, bool)> {
    let mut outCompatibleTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut outCompatible: bool = true;
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut rest_ty2: Arc<metamodelica::List<Arc<DAE::Type>>> = inTypes2.clone();
    let mut compat: bool = false;
    if (inTypes1.clone().len() as i32) != (inTypes2.clone().len() as i32) {
        outCompatible = false;
        return Ok((outCompatibleTypes, outCompatible));
    }
    for mut ty1 in &*inTypes1.clone() {
        let mut ty1 = ty1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_ty2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty2 = __pa0.clone();
        rest_ty2 = __pa1.clone();
        (_, _, ty2, compat) = checkTypeCompat(inExp1.clone(), ty1.clone(), inExp2.clone(), ty2.clone(), false)?;
        if !(compat.clone()) {
            outCompatible = false;
            return Ok((outCompatibleTypes, outCompatible));
        }
        outCompatibleTypes = cons(ty2.clone(), outCompatibleTypes.clone());
    }
    outCompatibleTypes = outCompatibleTypes.clone().reverse();
    Ok((outCompatibleTypes, outCompatible))
}

fn checkTypeCompat_cast(mut inExp1: Arc<DAE::Exp>, mut inType1: Arc<DAE::Type>, mut inExp2: Arc<DAE::Exp>, mut inType2: Arc<DAE::Type>, mut inAllowUnknown: bool) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Type>, bool)> {
    let mut outExp1: Arc<DAE::Exp> = inExp1.clone();
    let mut outExp2: Arc<DAE::Exp> = inExp2.clone();
    let mut outCompatType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outCompatible: bool = true;
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut path: Arc<Absyn::Path>;
    ty1 = derivedBasicType(inType1.clone());
    ty2 = derivedBasicType(inType2.clone());
    outCompatType = (::match_deref::match_deref! { match &((ty1.clone(), ty2.clone())) {
        (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            outExp2 = Expression::typeCastElements(inExp2.clone(), DAE::T_REAL_DEFAULT().clone())?;
            DAE::T_REAL_DEFAULT().clone()
        },
        (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
            outExp1 = Expression::typeCastElements(inExp1.clone(), DAE::T_REAL_DEFAULT().clone())?;
            DAE::T_REAL_DEFAULT().clone()
        },
        (Deref @ DAE::Type::T_METABOXED { .. }, _) => {
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), var_field!((*ty1).ty, DAE::Type::T_METABOXED).clone(), inExp2.clone(), ty2.clone(), inAllowUnknown.clone())?;
            outExp1 = if (isBoxedType(ty2.clone())) {outExp1.clone()} else {Arc::new(DAE::Exp::UNBOX { exp: outExp1.clone(), ty: outCompatType.clone() })};
            ty2.clone()
        },
        (_, Deref @ DAE::Type::T_METABOXED { .. }) => {
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1.clone(), ty1.clone(), inExp2.clone(), var_field!((*ty2).ty, DAE::Type::T_METABOXED).clone(), inAllowUnknown.clone())?;
            outExp2 = if (isBoxedType(ty1.clone())) {outExp2.clone()} else {Arc::new(DAE::Exp::UNBOX { exp: outExp2.clone(), ty: outCompatType.clone() })};
            ty1.clone()
        },
        (Deref @ DAE::Type::T_METARECORD { .. }, Deref @ DAE::Type::T_METAUNIONTYPE { .. }) => {
            outCompatible = AbsynUtil::pathEqual(var_field!((*ty1).utPath, DAE::Type::T_METARECORD).clone(), var_field!((*ty2).path, DAE::Type::T_METAUNIONTYPE).clone());
            ty2.clone()
        },
        (Deref @ DAE::Type::T_METAUNIONTYPE { .. }, Deref @ DAE::Type::T_METARECORD { .. }) => {
            outCompatible = AbsynUtil::pathEqual(var_field!((*ty1).path, DAE::Type::T_METAUNIONTYPE).clone(), var_field!((*ty2).utPath, DAE::Type::T_METARECORD).clone());
            ty1.clone()
        },
        (Deref @ DAE::Type::T_UNKNOWN, _) => {
            outCompatible = inAllowUnknown.clone();
            ty2.clone()
        },
        (_, Deref @ DAE::Type::T_UNKNOWN) => {
            outCompatible = inAllowUnknown.clone();
            ty1.clone()
        },
        _ => {
            outCompatible = false;
            DAE::T_UNKNOWN_DEFAULT().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp1, outExp2, outCompatType, outCompatible))
}

pub fn arrayHasUnknownDims(mut inType: Arc<DAE::Type>) -> bool {
    let mut outUnknownDims: bool = false;
    outUnknownDims = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => List::any(var_field!((*inType).dims, DAE::Type::T_ARRAY).clone(), (std::sync::Arc::new(fnptr!(Expression::dimensionUnknown, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<bool> + 'static>)) || arrayHasUnknownDims(var_field!((*inType).ty, DAE::Type::T_ARRAY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outUnknownDims
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn metaArrayElementType(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_METAARRAY { .. } => var_field!((*inType).ty, DAE::Type::T_METAARRAY).clone(),
        Deref @ DAE::Type::T_METATYPE { .. } => metaArrayElementType(var_field!((*inType).ty, DAE::Type::T_METATYPE).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isMetaArray(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_METAARRAY { .. } => true,
        Deref @ DAE::Type::T_METATYPE { .. } => isMetaArray(var_field!((*inType).ty, DAE::Type::T_METATYPE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getAttributes(mut inType: Arc<DAE::Type>) -> Arc<metamodelica::List<Arc<DAE::Var>>> {
    let mut outAttributes: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    outAttributes = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => var_field!((*inType).varLst, DAE::Type::T_REAL).clone(),
        Deref @ DAE::Type::T_INTEGER { .. } => var_field!((*inType).varLst, DAE::Type::T_INTEGER).clone(),
        Deref @ DAE::Type::T_STRING { .. } => var_field!((*inType).varLst, DAE::Type::T_STRING).clone(),
        Deref @ DAE::Type::T_BOOL { .. } => var_field!((*inType).varLst, DAE::Type::T_BOOL).clone(),
        Deref @ DAE::Type::T_ENUMERATION { .. } => var_field!((*inType).attributeLst, DAE::Type::T_ENUMERATION).clone(),
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => getAttributes(var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAttributes
}

pub fn lookupAttributeValue(mut inAttributes: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inName: ArcStr) -> Result<Option<Arc<Values::Value>>> {
    let mut outValue: Option<Arc<Values::Value>> = None;
    for mut attr in &*inAttributes.clone() {
        let mut attr = attr.clone();
        if inName.clone() == TypesDump::getVarName(attr.clone())? {
            outValue = DAEUtil::bindingValue(varBinding(attr.clone())?);
            break;
        }
    }
    Ok(outValue)
}

pub fn lookupAttributeExp(mut inAttributes: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inName: ArcStr) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExp: Option<Arc<DAE::Exp>> = None;
    for mut attr in &*inAttributes.clone() {
        let mut attr = attr.clone();
        if inName.clone() == TypesDump::getVarName(attr.clone())? {
            outExp = DAEUtil::bindingExp(varBinding(attr.clone())?)?;
            break;
        }
    }
    Ok(outExp)
}

fn unboxedTypeTraverseHelper<T: Clone + 'static>(mut ty: Arc<DAE::Type>, mut dummy: T) -> (Arc<DAE::Type>, T) {
    let mut oty: Arc<DAE::Type> = unboxedType(ty.clone()).unwrap();
    let mut odummy: T = dummy.clone();
    (oty, odummy)
}

pub fn getMetaRecordFields(mut ty: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Var>>>> {
    let mut fields: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    fields = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METARECORD { fields, .. } => {
            fields.clone()
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: false, .. } => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.getMetaRecordFields")); __mm_s.push_str(&*literal!(" called on a non-singleton uniontype: ")); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: Deref @ DAE::Type::T_METARECORD { fields, .. } }, .. } => {
            fields.clone()
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_TYPE_FUNCTION { fun }, .. } => {
            let DAE::T_METARECORD { fields: __pa0, .. } = (fun()?) else { bail!("pattern mismatch") };
            fields = __pa0.clone();
            fields.clone()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.getMetaRecordFields")); __mm_s.push_str(&*literal!(" called on a non-singleton uniontype: ")); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(fields)
}

pub fn getMetaRecordIfSingleton(mut ty: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut oty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    oty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: false, .. } => {
            ty.clone()
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: oty }, .. } => {
            setTypeVariables(oty.clone(), var_field!((*ty).typeVars, DAE::Type::T_METAUNIONTYPE).clone())
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_TYPE_FUNCTION { fun }, .. } => {
            oty = fun().unwrap();
            setTypeVariables(oty.clone(), var_field!((*ty).typeVars, DAE::Type::T_METAUNIONTYPE).clone())
        },
        _ => {
            ty.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oty
}

pub fn setTypeVariables(mut ty: Arc<DAE::Type>, mut typeVars: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Arc<DAE::Type> {
    let mut oty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    oty = (::match_deref::match_deref! { match &(ty.clone()) {
        oty @ Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
            let mut oty = (*oty).clone();
            assign_variant_field!(oty => DAE::Type::T_METAUNIONTYPE; typeVars = typeVars.clone());
            oty.clone()
        },
        oty @ Deref @ DAE::Type::T_METARECORD { .. } => {
            let mut oty = (*oty).clone();
            assign_variant_field!(oty => DAE::Type::T_METARECORD; typeVars = typeVars.clone());
            oty.clone()
        },
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oty
}

pub fn isExpandableConnector(mut ty: Arc<DAE::Type>) -> bool {
    let mut isExpandable: bool = false;
    isExpandable = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: true }, .. } => true,
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: true }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExpandable
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getBasicType(mut ty: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => getBasicType(var_field!((*ty).ty, DAE::Type::T_ARRAY).clone()),
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => getBasicType(var_field!((*ty).complexType, DAE::Type::T_SUBTYPE_BASIC).clone()),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

