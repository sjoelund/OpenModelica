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

use crate::BaseModelica;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFType as Type;
use openmodelica_ast::Absyn::Exp;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn::Subscript;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub enum NFDimension {
    RAW_DIM {
        dim: Arc<Subscript>,
        scope: Arc<InstNode::InstNode>,
    },
    UNTYPED {
        dimension: Arc<Expression::NFExpression>,
        isProcessing: bool,
    },
    INTEGER {
        size: i32,
        var: Variability,
    },
    BOOLEAN,
    ENUM {
        enumType: Arc<Type::NFType>,
    },
    EXP {
        exp: Arc<Expression::NFExpression>,
        var: Variability,
    },
    /// for all symbolic purposes this is INTEGER() for codegeneration it is EXP()
    ///    invoked by using annotation(__OpenModelica_resizable=true) on a parameter
    RESIZABLE {
        /// the actual size defined by the user
        size: i32,
        /// the optimal size determined by the backend
        opt_size: Option<i32>,
        /// the full expression (parameter)
        exp: Arc<Expression::NFExpression>,
        var: Variability,
    },
    UNKNOWN,
}
impl Default for NFDimension {
    fn default() -> Self { Self::BOOLEAN }
}
pub use self::NFDimension::{RAW_DIM,UNTYPED,INTEGER,BOOLEAN,ENUM,EXP,RESIZABLE,UNKNOWN};
// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn fromExp(mut exp: Arc<Expression::NFExpression>, mut var: Variability) -> Result<Arc<NFDimension>> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            Arc::new(NFDimension::INTEGER { size: var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(), var: var.clone() })
        },
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: ty, .. } } => {
            (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::BOOLEAN => Arc::new(crate::NFDimension::BOOLEAN),
        Deref @ Type::ENUMERATION { .. } => Arc::new(NFDimension::ENUM { enumType: ty.clone() }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.fromExp")); __mm_s.push_str(&*literal!(" got invalid typename")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ Expression::ARRAY { .. } if (Expression::arrayAllEqual(exp.clone())?) => {
            fromExp(Expression::arrayFirstScalar(exp.clone())?, var.clone())?
        },
        Deref @ Expression::SUBSCRIPTED_EXP { split: true, .. } if (Expression::isArray(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone()) && Expression::arrayAllEqual(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())?) => {
            fromExp(Expression::arrayFirstScalar(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())?, var.clone())?
        },
        _ => {
            let mut exp_simple: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut value: i32 = 0;
            let mut value_original: i32 = 0;
            exp_simple = SimplifyExp::simplify(exp.clone(), false)?;
            (::match_deref::match_deref! { match &(exp_simple.clone()) {
        Deref @ Expression::INTEGER { value } => Arc::new(NFDimension::INTEGER { size: value.clone(), var: var.clone() }),
        _ => {
            e1 = Expression::map(exp_simple.clone(), Arc::new(Expression::replaceResizableParameter))?;
            e1 = SimplifyExp::simplify(e1.clone(), false)?;
            (::match_deref::match_deref! { match &(e1.clone()) {
        Deref @ Expression::INTEGER { value } => {
            e2 = Expression::map(exp_simple.clone(), Arc::new(Expression::replaceResizableParameterWithOriginal))?;
            e2 = SimplifyExp::simplify(e2.clone(), false)?;
            (::match_deref::match_deref! { match &(e2.clone()) {
        Deref @ Expression::INTEGER { value: value_original } if (value.clone() != value_original.clone()) => Arc::new(NFDimension::RESIZABLE { size: value_original.clone(), opt_size: Some(value.clone()), exp: exp.clone(), var: var.clone() }),
        _ => Arc::new(NFDimension::RESIZABLE { size: value.clone(), opt_size: None, exp: exp.clone(), var: var.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => Arc::new(NFDimension::EXP { exp: exp.clone(), var: var.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn fromRange(mut range: Arc<Expression::NFExpression>) -> Result<Arc<NFDimension>> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::BOOLEAN);
    let mut start: i32 = 0;
    let mut step: i32 = 0;
    let mut stop: i32 = 0;
    (start, step, stop) = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop }, step: None, start: Deref @ Expression::INTEGER { value: start }, .. } => (start.clone(), 1, stop.clone()),
        Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop }, step: Some(Deref @ Expression::INTEGER { value: step }), start: Deref @ Expression::INTEGER { value: start }, .. } => (start.clone(), step.clone(), stop.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.fromRange")); __mm_s.push_str(&*literal!(" got non-range expression: ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dim = Arc::new(NFDimension::INTEGER { size: intDiv(stop.clone() - start.clone(), step.clone()) + 1, var: Prefixes::Variability::CONSTANT.clone() });
    Ok(dim)
}

pub fn fromInteger(mut n: i32, mut var: Variability) -> Arc<NFDimension> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::INTEGER { size: n.clone(), var: var.clone() });
    dim
}

pub fn fromExpArray(mut expl: metamodelica::Array<Arc<Expression::NFExpression>>) -> Arc<NFDimension> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::INTEGER { size: (expl.clone().borrow().len() as i32), var: Variability::CONSTANT.clone() });
    dim
}

pub fn fromExpList(mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Arc<NFDimension> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::INTEGER { size: (expl.clone().len() as i32), var: Variability::CONSTANT.clone() });
    dim
}

pub fn toRange(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    range = Arc::new(Expression::NFExpression::RANGE { ty: Type::liftArrayLeft(typeOf(dim.clone()), dim.clone()), start: lowerBoundExp(dim.clone())?, step: None, stop: upperBoundExp(dim.clone())? });
    Ok(range)
}

pub fn toDAE(mut dim: Arc<NFDimension>) -> Result<Arc<DAE::Dimension>> {
    let mut daeDim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    daeDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(DAE::Dimension::DIM_INTEGER { integer: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_BOOLEAN)
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Arc::new(DAE::Dimension::DIM_ENUM { enumTypeName: var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), literals: var_field!((**ty).literals, Type::NFType::ENUMERATION).clone(), size: (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32) })
        },
        Deref @ EXP { .. } => {
            Arc::new(DAE::Dimension::DIM_EXP { exp: Expression::toDAE(var_field!((*dim).exp, NFDimension::EXP).clone(), false)? })
        },
        Deref @ RESIZABLE { .. } => {
            Arc::new(DAE::Dimension::DIM_EXP { exp: Expression::toDAE(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), false)? })
        },
        Deref @ UNKNOWN { .. } => {
            Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(daeDim)
}

pub fn add(mut a: Arc<NFDimension>, mut b: Arc<NFDimension>) -> Arc<NFDimension> {
    fn addExp(mut e1: Arc<Expression::NFExpression>, mut e2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
        let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: Arc::new(Operator::NFOperator { ty: Arc::new(crate::NFType::INTEGER), op: Operator::Op::ADD.clone() }), exp2: e2.clone() });
        res
    }

    fn addOpt(mut s1: Option<i32>, mut s2: Option<i32>) -> Option<i32> {
        let mut res: Option<i32> = None;
        res = (match (s1.clone(), s2.clone()) {
        (Some(mut i1), Some(mut i2)) => {
            Some(i1.clone() + i2.clone())
        },
        _ => {
            None
        },
    });
        res
    }

    let mut c: Arc<NFDimension> = Arc::new(NFDimension::BOOLEAN);
    c = (::match_deref::match_deref! { match &((a.clone(), b.clone())) {
        (Deref @ UNKNOWN { .. }, _) => Arc::new(crate::NFDimension::UNKNOWN),
        (_, Deref @ UNKNOWN { .. }) => Arc::new(crate::NFDimension::UNKNOWN),
        (Deref @ INTEGER { .. }, Deref @ INTEGER { .. }) => Arc::new(NFDimension::INTEGER { size: var_field!((*a).size, NFDimension::INTEGER).clone() + var_field!((*b).size, NFDimension::INTEGER).clone(), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::INTEGER).clone(), var_field!((*b).var, NFDimension::INTEGER).clone()) }),
        (Deref @ INTEGER { .. }, Deref @ EXP { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*b).exp, NFDimension::EXP).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*a).size, NFDimension::INTEGER).clone() })), var: var_field!((*b).var, NFDimension::EXP).clone() }),
        (Deref @ EXP { .. }, Deref @ INTEGER { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::EXP).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*b).size, NFDimension::INTEGER).clone() })), var: var_field!((*a).var, NFDimension::EXP).clone() }),
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::EXP).clone(), var_field!((*b).exp, NFDimension::EXP).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::EXP).clone(), var_field!((*b).var, NFDimension::EXP).clone()) }),
        (Deref @ INTEGER { .. }, Deref @ RESIZABLE { .. }) => Arc::new(NFDimension::RESIZABLE { size: var_field!((*a).size, NFDimension::INTEGER).clone() + var_field!((*b).size, NFDimension::RESIZABLE).clone(), opt_size: addOpt(Some(var_field!((*a).size, NFDimension::INTEGER).clone()), var_field!((*b).opt_size, NFDimension::RESIZABLE).clone()), exp: addExp(var_field!((*b).exp, NFDimension::RESIZABLE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*a).size, NFDimension::INTEGER).clone() })), var: var_field!((*b).var, NFDimension::RESIZABLE).clone() }),
        (Deref @ RESIZABLE { .. }, Deref @ INTEGER { .. }) => Arc::new(NFDimension::RESIZABLE { size: var_field!((*a).size, NFDimension::RESIZABLE).clone() + var_field!((*b).size, NFDimension::INTEGER).clone(), opt_size: addOpt(var_field!((*a).opt_size, NFDimension::RESIZABLE).clone(), Some(var_field!((*b).size, NFDimension::INTEGER).clone())), exp: addExp(var_field!((*a).exp, NFDimension::RESIZABLE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*b).size, NFDimension::INTEGER).clone() })), var: var_field!((*a).var, NFDimension::RESIZABLE).clone() }),
        (Deref @ EXP { .. }, Deref @ RESIZABLE { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::EXP).clone(), var_field!((*b).exp, NFDimension::RESIZABLE).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::EXP).clone(), var_field!((*b).var, NFDimension::RESIZABLE).clone()) }),
        (Deref @ RESIZABLE { .. }, Deref @ EXP { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::RESIZABLE).clone(), var_field!((*b).exp, NFDimension::EXP).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::RESIZABLE).clone(), var_field!((*b).var, NFDimension::EXP).clone()) }),
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Arc::new(NFDimension::RESIZABLE { size: var_field!((*a).size, NFDimension::RESIZABLE).clone() + var_field!((*b).size, NFDimension::RESIZABLE).clone(), opt_size: addOpt(var_field!((*a).opt_size, NFDimension::RESIZABLE).clone(), var_field!((*b).opt_size, NFDimension::RESIZABLE).clone()), exp: addExp(var_field!((*a).exp, NFDimension::RESIZABLE).clone(), var_field!((*b).exp, NFDimension::RESIZABLE).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::RESIZABLE).clone(), var_field!((*b).var, NFDimension::RESIZABLE).clone()) }),
        _ => Arc::new(crate::NFDimension::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    c
}

pub fn size(mut dim: Arc<NFDimension>, mut resize: bool) -> Result<i32> {
    let mut size: i32 = 0;
    size = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            var_field!((*dim).size, NFDimension::INTEGER).clone()
        },
        Deref @ RESIZABLE { .. } => {
            if (resize.clone()) {Util::getOptionOrDefault(var_field!((*dim).opt_size, NFDimension::RESIZABLE).clone(), var_field!((*dim).size, NFDimension::RESIZABLE).clone())} else {var_field!((*dim).size, NFDimension::RESIZABLE).clone()}
        },
        Deref @ BOOLEAN { .. } => {
            2
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32)
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.size")); __mm_s.push_str(&*literal!(" could not get size of: ")); __mm_s.push_str(&*toString(dim.clone())?); ArcStr::from(__mm_s) }).clone())?;
            }
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

pub fn sizes(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut resize: bool) -> Arc<metamodelica::List<i32>> {
    let mut outSizes: Arc<metamodelica::List<i32>> = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = size(d.clone(), resize.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outSizes
}

pub fn sizesProduct(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut resize: bool) -> i32 {
    let mut outSize: i32 = {
        let mut __acc: i32 = 1;
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = size(d.clone(), resize.clone()).unwrap();
            __acc *= __x;
        }
        __acc
    };
    outSize
}

pub fn isEqual(mut dim1: Arc<NFDimension>, mut dim2: Arc<NFDimension>) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ UNKNOWN { .. }, _) => true,
        (_, Deref @ UNKNOWN { .. }) => true,
        (Deref @ EXP { .. }, _) => true,
        (_, Deref @ EXP { .. }) => true,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((*dim2).exp, NFDimension::RESIZABLE).clone())?,
        _ => size(dim1.clone(), false)? == size(dim2.clone(), false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub fn isEqualKnown(mut dim1: Arc<NFDimension>, mut dim2: Arc<NFDimension>) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ UNKNOWN { .. }, _) => false,
        (_, Deref @ UNKNOWN { .. }) => false,
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::EXP).clone(), var_field!((*dim2).exp, NFDimension::EXP).clone())?,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((*dim2).exp, NFDimension::RESIZABLE).clone())?,
        (Deref @ EXP { .. }, _) => false,
        (_, Deref @ EXP { .. }) => false,
        _ => size(dim1.clone(), false)? == size(dim2.clone(), false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub fn isEqualKnownSize(mut dim1: Arc<NFDimension>, mut node1: Arc<InstNode::InstNode>, mut index1: i32, mut dim2: Arc<NFDimension>, mut node2: Arc<InstNode::InstNode>, mut index2: i32) -> Result<bool> {
    let mut isEqual: bool = false;
    let mut cref_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    isEqual = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ EXP { .. }, _) if (isSizeOf(dim1.clone(), node2.clone(), index2.clone())?) => true,
        (_, Deref @ EXP { .. }) if (isSizeOf(dim2.clone(), node1.clone(), index1.clone())?) => true,
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::EXP).clone(), var_field!((*dim2).exp, NFDimension::EXP).clone())?,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((*dim2).exp, NFDimension::RESIZABLE).clone())?,
        (Deref @ UNKNOWN { .. }, _) => false,
        (_, Deref @ UNKNOWN { .. }) => false,
        _ => size(dim1.clone(), false)? == size(dim2.clone(), false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub fn isSizeOf(mut dim: Arc<NFDimension>, mut node: Arc<InstNode::InstNode>, mut index: i32) -> Result<bool> {
    let mut res: bool = false;
    let mut cref_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    res = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ EXP { exp: Deref @ Expression::SIZE { dimIndex: Some(index_exp), exp: cref_exp @ Deref @ Expression::CREF { .. } }, .. } => InstNode::refEqual(ComponentRef::node(var_field!((**cref_exp).cref, Expression::NFExpression::CREF).clone())?, node.clone()) && Expression::isEqual(index_exp.clone(), Arc::new(Expression::NFExpression::INTEGER { value: index.clone() }))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isResizable(mut dim: Arc<NFDimension>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ RESIZABLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn allEqualKnown(mut dims1: Arc<metamodelica::List<Arc<NFDimension>>>, mut dims2: Arc<metamodelica::List<Arc<NFDimension>>>) -> bool {
    let mut allEqual: bool = List::isEqualOnTrue(dims1.clone(), dims2.clone(), Arc::new(isEqualKnown));
    allEqual
}

pub fn isKnown(mut dim: Arc<NFDimension>, mut allowExp: bool) -> bool {
    let mut known: bool = false;
    known = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ ENUM { .. } => true,
        Deref @ RESIZABLE { .. } => true,
        Deref @ EXP { .. } => allowExp.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    known
}

pub fn isUnknown(mut dim: Arc<NFDimension>) -> bool {
    let mut isUnknown: bool = false;
    isUnknown = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ UNKNOWN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isUnknown
}

pub fn isZero(mut dim: Arc<NFDimension>) -> Result<bool> {
    let mut isZero: bool = false;
    isZero = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => var_field!((*dim).size, NFDimension::INTEGER).clone() == 0,
        Deref @ ENUM { .. } => Type::enumSize(var_field!((*dim).enumType, NFDimension::ENUM).clone())? == 0,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isZero)
}

pub fn isOne(mut dim: Arc<NFDimension>) -> Result<bool> {
    let mut isOne: bool = false;
    isOne = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => var_field!((*dim).size, NFDimension::INTEGER).clone() == 1,
        Deref @ ENUM { .. } => Type::enumSize(var_field!((*dim).enumType, NFDimension::ENUM).clone())? == 1,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isOne)
}

pub fn subscriptType(mut dim: Arc<NFDimension>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => Arc::new(crate::NFType::INTEGER),
        Deref @ BOOLEAN { .. } => Arc::new(crate::NFType::BOOLEAN),
        Deref @ ENUM { .. } => var_field!((*dim).enumType, NFDimension::ENUM).clone(),
        Deref @ EXP { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::EXP).clone()),
        Deref @ RESIZABLE { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::RESIZABLE).clone()),
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn toString(mut dim: Arc<NFDimension>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ RAW_DIM { .. } => {
            Dump::printSubscriptStr(var_field!((*dim).dim, NFDimension::RAW_DIM).clone())?
        },
        Deref @ INTEGER { .. } => {
            ArcStr::from(::std::format!("{}", var_field!((*dim).size, NFDimension::INTEGER).clone()))
        },
        Deref @ BOOLEAN { .. } => {
            literal!("Boolean")
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            AbsynUtil::pathString(var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?
        },
        Deref @ EXP { .. } => {
            Expression::toString(var_field!((*dim).exp, NFDimension::EXP).clone())?
        },
        Deref @ RESIZABLE { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(var_field!((*dim).exp, NFDimension::RESIZABLE).clone())?); __mm_s.push_str(&*literal!("(R)")); ArcStr::from(__mm_s) }
        },
        Deref @ UNKNOWN { .. } => {
            literal!(":")
        },
        Deref @ UNTYPED { .. } => {
            Expression::toString(var_field!((*dim).dimension, NFDimension::UNTYPED).clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn hashList(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>) -> Result<i32> {
    let mut hash: i32 = Util::HASH_SEED.clone();
    for mut dim in &*dims.clone() {
        let mut dim = dim.clone();
        hash = stringHashDjb2Continue((toString(dim.clone())?).clone(), hash.clone());
    }
    Ok(hash)
}

pub fn toStringList(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut brackets: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = toString(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
    if brackets.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn toFlatString(mut dim: Arc<NFDimension>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => ArcStr::from(::std::format!("{}", var_field!((*dim).size, NFDimension::INTEGER).clone())),
        Deref @ BOOLEAN { .. } => literal!("Boolean"),
        Deref @ ENUM { .. } => Type::toFlatString(var_field!((*dim).enumType, NFDimension::ENUM).clone(), format.clone())?,
        Deref @ EXP { .. } => Expression::toFlatString(var_field!((*dim).exp, NFDimension::EXP).clone(), format.clone())?,
        Deref @ RESIZABLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toFlatString(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), format.clone())?); __mm_s.push_str(&*literal!("(R)")); ArcStr::from(__mm_s) },
        Deref @ UNKNOWN { .. } => literal!(":"),
        Deref @ UNTYPED { .. } => Expression::toFlatString(var_field!((*dim).dimension, NFDimension::UNTYPED).clone(), format.clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn toFlatStringList(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut format: BaseModelica::OutputFormat, mut name: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (List::toString(dims.clone(), Arc::new({ let __pe_b1 = format.clone(); move |__pe_a0| toFlatString(__pe_a0, __pe_b1.clone()) }), (name.clone()).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?).clone();
    Ok(r#str)
}

pub fn endExp(mut dim: Arc<NFDimension>, mut subscriptedExp: Arc<Expression::NFExpression>, mut index: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut sizeExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    sizeExp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Expression::NFExpression::BOOLEAN { value: true })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Expression::makeEnumLiteral(ty.clone(), (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32))?
        },
        Deref @ EXP { .. } => {
            var_field!((*dim).exp, NFDimension::EXP).clone()
        },
        Deref @ RESIZABLE { .. } => {
            var_field!((*dim).exp, NFDimension::RESIZABLE).clone()
        },
        Deref @ UNKNOWN { .. } => {
            (::match_deref::match_deref! { match &(subscriptedExp.clone()) {
        Deref @ Expression::CREF { .. } => Arc::new(Expression::NFExpression::SIZE { exp: Expression::fromCref((ComponentRef::stripSubscripts(var_field!((*subscriptedExp).cref, Expression::NFExpression::CREF).clone())).0, false)?, dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: index.clone() })) }),
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => Arc::new(Expression::NFExpression::SIZE { exp: var_field!((*subscriptedExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: index.clone() })) }),
        _ => bail!("match: no arm matched"),
    } })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub fn sizeExp(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut sizeExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    sizeExp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: 2 })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Arc::new(Expression::NFExpression::INTEGER { value: (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32) })
        },
        Deref @ EXP { .. } => {
            var_field!((*dim).exp, NFDimension::EXP).clone()
        },
        Deref @ RESIZABLE { .. } => {
            var_field!((*dim).exp, NFDimension::RESIZABLE).clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub fn lowerBoundExp(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ BOOLEAN { .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ ENUM { .. } => Expression::makeEnumLiteral(var_field!((*dim).enumType, NFDimension::ENUM).clone(), 1)?,
        _ => Arc::new(Expression::NFExpression::INTEGER { value: 1 }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn expIsLowerBound(mut exp: Arc<Expression::NFExpression>) -> bool {
    let mut isStart: bool = false;
    isStart = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => var_field!((*exp).value, Expression::NFExpression::INTEGER).clone() == 1,
        Deref @ Expression::BOOLEAN { .. } => var_field!((*exp).value, Expression::NFExpression::BOOLEAN).clone() == false,
        Deref @ Expression::ENUM_LITERAL { .. } => var_field!((*exp).index, Expression::NFExpression::ENUM_LITERAL).clone() == 1,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isStart
}

pub fn upperBoundExp(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Expression::NFExpression::BOOLEAN { value: true })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Expression::makeEnumLiteral(ty.clone(), (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32))?
        },
        Deref @ EXP { .. } => {
            var_field!((*dim).exp, NFDimension::EXP).clone()
        },
        Deref @ RESIZABLE { .. } => {
            var_field!((*dim).exp, NFDimension::RESIZABLE).clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn expIsUpperBound(mut exp: Arc<Expression::NFExpression>, mut dim: Arc<NFDimension>) -> bool {
    let mut isEnd: bool = false;
    isEnd = (::match_deref::match_deref! { match &((exp.clone(), dim.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ INTEGER { .. }) => {
            var_field!((*exp).value, Expression::NFExpression::INTEGER).clone() == var_field!((*dim).size, NFDimension::INTEGER).clone()
        },
        (Deref @ Expression::BOOLEAN { .. }, _) => {
            var_field!((*exp).value, Expression::NFExpression::BOOLEAN).clone() == true
        },
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } }) => {
            var_field!((*exp).index, Expression::NFExpression::ENUM_LITERAL).clone() == (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEnd
}

pub fn variability(mut dim: Arc<NFDimension>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => var_field!((*dim).var, NFDimension::INTEGER).clone(),
        Deref @ BOOLEAN { .. } => Variability::CONSTANT.clone(),
        Deref @ ENUM { .. } => Variability::CONSTANT.clone(),
        Deref @ EXP { .. } => var_field!((*dim).var, NFDimension::EXP).clone(),
        Deref @ RESIZABLE { .. } => var_field!((*dim).var, NFDimension::RESIZABLE).clone(),
        Deref @ UNKNOWN { .. } => Variability::CONTINUOUS.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub fn mapExp(mut dim: Arc<NFDimension>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFDimension>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outDim: Arc<NFDimension> = Arc::new(NFDimension::BOOLEAN);
    outDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ UNTYPED { dimension: e1, .. } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e2.clone())) {dim.clone()} else {Arc::new(NFDimension::UNTYPED { dimension: e2.clone(), isProcessing: var_field!((*dim).isProcessing, NFDimension::UNTYPED).clone() })}
        },
        Deref @ EXP { exp: e1, .. } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e2.clone())) {dim.clone()} else {fromExp(e2.clone(), var_field!((*dim).var, NFDimension::EXP).clone())?}
        },
        Deref @ RESIZABLE { exp: e1, .. } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e2.clone())) {dim.clone()} else {fromExp(e2.clone(), var_field!((*dim).var, NFDimension::RESIZABLE).clone())?}
        },
        _ => {
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

pub fn foldExp<ArgT: Clone + 'static>(mut dim: Arc<NFDimension>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut outArg: ArgT;
    outArg = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ UNTYPED { .. } => Expression::fold(var_field!((*dim).dimension, NFDimension::UNTYPED).clone(), func.clone(), arg.clone())?,
        Deref @ EXP { .. } => Expression::fold(var_field!((*dim).exp, NFDimension::EXP).clone(), func.clone(), arg.clone())?,
        Deref @ RESIZABLE { .. } => Expression::fold(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), func.clone(), arg.clone())?,
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub fn foldExpList<ArgT: Clone + 'static>(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    for mut dim in &*dims.clone() {
        let mut dim = dim.clone();
        arg = foldExp(dim.clone(), func.clone(), arg.clone())?;
    }
    Ok(arg)
}

pub fn eval(mut dim: Arc<NFDimension>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<NFDimension>> {
    let mut outDim: Arc<NFDimension> = Arc::new(NFDimension::BOOLEAN);
    outDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ EXP { .. } => fromExp(Ceval::evalExp(var_field!((*dim).exp, NFDimension::EXP).clone(), target.clone())?, var_field!((*dim).var, NFDimension::EXP).clone())?,
        Deref @ RESIZABLE { .. } => {
            assign_variant_field!(dim => NFDimension::RESIZABLE; exp = Ceval::evalExp(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), target.clone())?);
            dim.clone()
        },
        _ => dim.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

pub fn simplify(mut dim: Arc<NFDimension>) -> Result<Arc<NFDimension>> {
    let mut dim: Arc<NFDimension> = dim;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ EXP { .. } => {
            let mut simple: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            simple = SimplifyExp::simplify(var_field!((*dim).exp, NFDimension::EXP).clone(), false)?;
            fromExp(simple.clone(), Expression::variability(simple.clone())?)?
        },
        Deref @ RESIZABLE { .. } => {
            assign_variant_field!(dim => NFDimension::RESIZABLE; exp = SimplifyExp::simplify(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), false)?);
            dim.clone()
        },
        _ => {
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn typeOf(mut dim: Arc<NFDimension>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => Arc::new(crate::NFType::INTEGER),
        Deref @ BOOLEAN { .. } => Arc::new(crate::NFType::BOOLEAN),
        Deref @ ENUM { .. } => var_field!((*dim).enumType, NFDimension::ENUM).clone(),
        Deref @ EXP { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::EXP).clone()),
        Deref @ RESIZABLE { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::RESIZABLE).clone()),
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}


