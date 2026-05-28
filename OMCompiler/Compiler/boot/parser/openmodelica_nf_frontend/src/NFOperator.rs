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

use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::JSON;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NFOperator {
    pub ty: Arc<Type::NFType>,
    pub op: Op,
}

impl Default for NFOperator {
    fn default() -> Self {
        Self {
            ty: Default::default(),
            op: Default::default(),
        }
    }
}

pub type OPERATOR = NFOperator;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Op {
    ADD = 1,
    SUB = 2,
    MUL = 3,
    DIV = 4,
    POW = 5,
    ADD_EW = 6,
    SUB_EW = 7,
    MUL_EW = 8,
    DIV_EW = 9,
    POW_EW = 10,
    ADD_SCALAR_ARRAY = 11,
    ADD_ARRAY_SCALAR = 12,
    SUB_SCALAR_ARRAY = 13,
    SUB_ARRAY_SCALAR = 14,
    MUL_SCALAR_ARRAY = 15,
    MUL_ARRAY_SCALAR = 16,
    MUL_VECTOR_MATRIX = 17,
    MUL_MATRIX_VECTOR = 18,
    SCALAR_PRODUCT = 19,
    MATRIX_PRODUCT = 20,
    DIV_SCALAR_ARRAY = 21,
    DIV_ARRAY_SCALAR = 22,
    POW_SCALAR_ARRAY = 23,
    POW_ARRAY_SCALAR = 24,
    POW_MATRIX = 25,
    UMINUS = 26,
    AND = 27,
    OR = 28,
    NOT = 29,
    LESS = 30,
    LESSEQ = 31,
    GREATER = 32,
    GREATEREQ = 33,
    EQUAL = 34,
    NEQUAL = 35,
    USERDEFINED = 36,
}
impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Op {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Op {
    fn default() -> Self { Self::ADD }
}

pub fn compare(mut op1: Arc<NFOperator>, mut op2: Arc<NFOperator>) -> i32 {
    let mut comp: i32 = 0;
    let mut o1: Op = op1.op.clone();
    let mut o2: Op = op2.op.clone();
    comp = Util::intCompare(((o1.clone()) as i32), ((o2.clone()) as i32));
    comp
}

pub fn invert(mut operator: Arc<NFOperator>) -> Result<Arc<NFOperator>> {
    let mut operator: Arc<NFOperator> = operator;
    assign_field!(operator.op = (match operator.op.clone() {
        Op::ADD => Op::SUB.clone(),
        Op::SUB => Op::ADD.clone(),
        Op::MUL => Op::DIV.clone(),
        Op::DIV => Op::MUL.clone(),
        Op::ADD_EW => Op::SUB_EW.clone(),
        Op::SUB_EW => Op::ADD_EW.clone(),
        Op::MUL_EW => Op::DIV_EW.clone(),
        Op::DIV_EW => Op::MUL_EW.clone(),
        Op::ADD_SCALAR_ARRAY => Op::SUB_SCALAR_ARRAY.clone(),
        Op::ADD_ARRAY_SCALAR { .. } => Op::SUB_ARRAY_SCALAR.clone(),
        Op::SUB_SCALAR_ARRAY { .. } => Op::ADD_SCALAR_ARRAY.clone(),
        Op::SUB_ARRAY_SCALAR => Op::ADD_ARRAY_SCALAR.clone(),
        Op::MUL_SCALAR_ARRAY => Op::DIV_SCALAR_ARRAY.clone(),
        Op::MUL_ARRAY_SCALAR { .. } => Op::DIV_ARRAY_SCALAR.clone(),
        Op::DIV_SCALAR_ARRAY { .. } => Op::MUL_SCALAR_ARRAY.clone(),
        Op::DIV_ARRAY_SCALAR { .. } => Op::MUL_ARRAY_SCALAR.clone(),
        Op::LESS => Op::GREATEREQ.clone(),
        Op::LESSEQ => Op::GREATER.clone(),
        Op::GREATER => Op::LESSEQ.clone(),
        Op::GREATEREQ => Op::LESS.clone(),
        Op::EQUAL => Op::EQUAL.clone(),
        Op::NEQUAL => Op::NEQUAL.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.invert")); __mm_s.push_str(&*literal!("Failed! Don't know how to invert: ")); __mm_s.push_str(&*symbol(operator.clone(), (literal!(" ")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    }));
    Ok(operator)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum TypeRestriction {
    SCALAR = 1,
    VECTOR = 2,
    MATRIX = 3,
    ARRAY = 4,
    OTHER = 5,
}
impl PartialOrd for TypeRestriction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for TypeRestriction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn typeRestriction(mut ty: Arc<Type::NFType>) -> TypeRestriction {
    let mut restriction: TypeRestriction = TypeRestriction::SCALAR;
    if Type::isScalar(ty.clone()) {
        restriction = TypeRestriction::SCALAR.clone();
    } else if Type::isVector(ty.clone()) {
        restriction = TypeRestriction::VECTOR.clone();
    } else if Type::isMatrix(ty.clone()) {
        restriction = TypeRestriction::MATRIX.clone();
    } else if Type::isArray(ty.clone()) {
        restriction = TypeRestriction::ARRAY.clone();
    } else {
        restriction = TypeRestriction::OTHER.clone();
    }
    restriction
}

pub fn repairMultary(mut operator: Arc<NFOperator>, mut types: Arc<metamodelica::List<Arc<Type::NFType>>>) -> Result<Arc<NFOperator>> {
    fn tplLt(mut tpl1: (TypeRestriction, Arc<Type::NFType>), mut tpl2: (TypeRestriction, Arc<Type::NFType>)) -> bool {
        let mut b: bool = Util::tuple21(tpl1.clone()) < Util::tuple21(tpl2.clone());
        b
    }

    let mut operator: Arc<NFOperator> = operator;
    let mut mc: MathClassification = getMathClassification(operator.clone())?;
    let mut sc: SizeClassification = SizeClassification::SCALAR;
    let mut lst: Arc<metamodelica::List<(TypeRestriction, Arc<Type::NFType>)>> = metamodelica::nil();
    let mut min_: (TypeRestriction, Arc<Type::NFType>);
    let mut max_: (TypeRestriction, Arc<Type::NFType>);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    lst = {
        let mut __acc: Arc<metamodelica::List<(TypeRestriction, Arc<Type::NFType>)>> = metamodelica::nil();
        for mut t in (types.clone()).into_iter().cloned() {
            let __x = (typeRestriction(t.clone()), t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    min_ = List::minElement(lst.clone(), (std::sync::Arc::new(fnptr!(tplLt, (TypeRestriction, Arc<Type::NFType>), (TypeRestriction, Arc<Type::NFType>))) as std::sync::Arc<dyn ::std::ops::Fn((TypeRestriction, Arc<Type::NFType>), (TypeRestriction, Arc<Type::NFType>)) -> Result<bool> + 'static>))?;
    max_ = List::maxElement(lst.clone(), (std::sync::Arc::new(fnptr!(tplLt, (TypeRestriction, Arc<Type::NFType>), (TypeRestriction, Arc<Type::NFType>))) as std::sync::Arc<dyn ::std::ops::Fn((TypeRestriction, Arc<Type::NFType>), (TypeRestriction, Arc<Type::NFType>)) -> Result<bool> + 'static>))?;
    (sc, ty) = (::match_deref::match_deref! { match &((min_.clone(), max_.clone())) {
        ((TypeRestriction::SCALAR, _), (TypeRestriction::SCALAR, ty)) => (SizeClassification::SCALAR.clone(), ty.clone()),
        ((TypeRestriction::SCALAR, _), (_, ty)) => (SizeClassification::SCALAR_ARRAY.clone(), ty.clone()),
        ((TypeRestriction::VECTOR { .. }, _), (TypeRestriction::VECTOR { .. }, ty)) => (SizeClassification::ELEMENT_WISE.clone(), ty.clone()),
        ((TypeRestriction::VECTOR { .. }, _), (TypeRestriction::MATRIX { .. }, ty)) => (SizeClassification::VECTOR_MATRIX.clone(), ty.clone()),
        ((TypeRestriction::MATRIX { .. }, _), (TypeRestriction::MATRIX { .. }, ty)) => (SizeClassification::ELEMENT_WISE.clone(), ty.clone()),
        ((TypeRestriction::ARRAY { .. }, _), (TypeRestriction::ARRAY { .. }, ty)) => (SizeClassification::ELEMENT_WISE.clone(), ty.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.repairMultary")); __mm_s.push_str(&*literal!(" failed because the multary arguments have incompatible sizes: ")); __mm_s.push_str(&*List::toString(types.clone(), (std::sync::Arc::new(Type::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    operator = fromClassification((mc.clone(), sc.clone()), ty.clone())?;
    Ok(operator)
}

pub fn repairBinary(mut operator: Arc<NFOperator>, mut ty1: Arc<Type::NFType>, mut ty2: Arc<Type::NFType>) -> Result<Arc<NFOperator>> {
    let mut operator: Arc<NFOperator> = operator;
    let mut mc: MathClassification = getMathClassification(operator.clone())?;
    let mut sc: SizeClassification = SizeClassification::SCALAR;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    (sc, ty) = (match (typeRestriction(ty1.clone()), typeRestriction(ty2.clone())) {
        (TypeRestriction::SCALAR, TypeRestriction::SCALAR) => {
            (SizeClassification::SCALAR.clone(), ty1.clone())
        },
        (TypeRestriction::SCALAR, mut r2) if (r2.clone() > TypeRestriction::SCALAR.clone()) => {
            (SizeClassification::SCALAR_ARRAY.clone(), ty2.clone())
        },
        (mut r1, TypeRestriction::SCALAR) if (r1.clone() > TypeRestriction::SCALAR.clone()) => {
            (SizeClassification::ARRAY_SCALAR.clone(), ty1.clone())
        },
        (TypeRestriction::VECTOR { .. }, TypeRestriction::MATRIX { .. }) => {
            (SizeClassification::VECTOR_MATRIX.clone(), ty1.clone())
        },
        (TypeRestriction::MATRIX { .. }, TypeRestriction::VECTOR { .. }) => {
            (SizeClassification::MATRIX_VECTOR.clone(), ty2.clone())
        },
        (mut r1, mut r2) if (r1.clone() == r2.clone()) => {
            (getSizeClassification(operator.clone())?, ty1.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.repairBinary")); __mm_s.push_str(&*literal!(" failed because the binary arguments have incompatible sizes: ")); __mm_s.push_str(&*Type::toString(ty1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Type::toString(ty2.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    operator = fromClassification((mc.clone(), sc.clone()), ty.clone())?;
    Ok(operator)
}

pub fn isLogical(mut operator: Arc<NFOperator>) -> bool {
    let mut b: bool = false;
    b = (match operator.op.clone() {
        Op::AND => true,
        Op::OR => true,
        Op::NOT => true,
        _ => false,
    });
    b
}

pub fn isRelational(mut operator: Arc<NFOperator>) -> bool {
    let mut b: bool = false;
    b = (match operator.op.clone() {
        Op::LESS => true,
        Op::LESSEQ => true,
        Op::GREATER => true,
        Op::GREATEREQ => true,
        Op::EQUAL => true,
        Op::NEQUAL => true,
        _ => false,
    });
    b
}

pub fn isScalarProduct(mut operator: Arc<NFOperator>) -> bool {
    let mut b: bool = false;
    b = (match operator.op.clone() {
        Op::SCALAR_PRODUCT => true,
        _ => false,
    });
    b
}

pub fn fromAbsyn(mut inOperator: Absyn::Operator) -> Result<Arc<NFOperator>> {
    let mut outOperator: Arc<NFOperator> = Arc::new(<NFOperator as ::std::default::Default>::default());
    let mut op: Op = Op::ADD;
    op = (match inOperator.clone() {
        Absyn::Operator::ADD => Op::ADD.clone(),
        Absyn::Operator::SUB => Op::SUB.clone(),
        Absyn::Operator::MUL => Op::MUL.clone(),
        Absyn::Operator::DIV => Op::DIV.clone(),
        Absyn::Operator::POW => Op::POW.clone(),
        Absyn::Operator::ADD_EW => Op::ADD_EW.clone(),
        Absyn::Operator::SUB_EW => Op::SUB_EW.clone(),
        Absyn::Operator::MUL_EW => Op::MUL_EW.clone(),
        Absyn::Operator::DIV_EW => Op::DIV_EW.clone(),
        Absyn::Operator::POW_EW => Op::POW_EW.clone(),
        Absyn::Operator::UPLUS => Op::ADD.clone(),
        Absyn::Operator::UPLUS_EW => Op::ADD.clone(),
        Absyn::Operator::UMINUS => Op::UMINUS.clone(),
        Absyn::Operator::UMINUS_EW => Op::UMINUS.clone(),
        Absyn::Operator::AND => Op::AND.clone(),
        Absyn::Operator::OR => Op::OR.clone(),
        Absyn::Operator::NOT => Op::NOT.clone(),
        Absyn::Operator::LESS => Op::LESS.clone(),
        Absyn::Operator::LESSEQ => Op::LESSEQ.clone(),
        Absyn::Operator::GREATER => Op::GREATER.clone(),
        Absyn::Operator::GREATEREQ => Op::GREATEREQ.clone(),
        Absyn::Operator::EQUAL => Op::EQUAL.clone(),
        Absyn::Operator::NEQUAL => Op::NEQUAL.clone(),
        _ => bail!("match: no arm matched"),
    });
    outOperator = Arc::new(NFOperator { ty: Arc::new(crate::NFType::UNKNOWN), op: op.clone() });
    Ok(outOperator)
}

pub fn toAbsyn(mut op: Arc<NFOperator>) -> Result<Absyn::Operator> {
    let mut aop: Absyn::Operator = Absyn::Operator::ADD;
    aop = (match op.op.clone() {
        Op::ADD => if (Type::isArray(op.ty.clone())) {openmodelica_ast::Absyn::Operator::ADD_EW} else {openmodelica_ast::Absyn::Operator::ADD},
        Op::SUB => if (Type::isArray(op.ty.clone())) {openmodelica_ast::Absyn::Operator::SUB_EW} else {openmodelica_ast::Absyn::Operator::SUB},
        Op::MUL => if (Type::isArray(op.ty.clone())) {openmodelica_ast::Absyn::Operator::MUL_EW} else {openmodelica_ast::Absyn::Operator::MUL},
        Op::DIV => if (Type::isArray(op.ty.clone())) {openmodelica_ast::Absyn::Operator::DIV_EW} else {openmodelica_ast::Absyn::Operator::DIV},
        Op::POW => if (Type::isArray(op.ty.clone())) {openmodelica_ast::Absyn::Operator::POW_EW} else {openmodelica_ast::Absyn::Operator::POW},
        Op::ADD_EW => openmodelica_ast::Absyn::Operator::ADD_EW,
        Op::SUB_EW => openmodelica_ast::Absyn::Operator::SUB_EW,
        Op::MUL_EW => openmodelica_ast::Absyn::Operator::MUL_EW,
        Op::DIV_EW => openmodelica_ast::Absyn::Operator::DIV_EW,
        Op::POW_EW => openmodelica_ast::Absyn::Operator::POW_EW,
        Op::ADD_SCALAR_ARRAY => openmodelica_ast::Absyn::Operator::ADD,
        Op::ADD_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::ADD,
        Op::SUB_SCALAR_ARRAY { .. } => openmodelica_ast::Absyn::Operator::SUB,
        Op::SUB_ARRAY_SCALAR => openmodelica_ast::Absyn::Operator::SUB,
        Op::MUL_SCALAR_ARRAY => openmodelica_ast::Absyn::Operator::MUL,
        Op::MUL_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::MUL,
        Op::MUL_VECTOR_MATRIX => openmodelica_ast::Absyn::Operator::MUL,
        Op::MUL_MATRIX_VECTOR => openmodelica_ast::Absyn::Operator::MUL,
        Op::SCALAR_PRODUCT => openmodelica_ast::Absyn::Operator::MUL,
        Op::MATRIX_PRODUCT => openmodelica_ast::Absyn::Operator::MUL,
        Op::DIV_SCALAR_ARRAY { .. } => openmodelica_ast::Absyn::Operator::DIV,
        Op::DIV_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::DIV,
        Op::POW_SCALAR_ARRAY { .. } => openmodelica_ast::Absyn::Operator::POW,
        Op::POW_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::POW,
        Op::POW_MATRIX => openmodelica_ast::Absyn::Operator::POW,
        Op::UMINUS => if (Type::isArray(op.ty.clone())) {openmodelica_ast::Absyn::Operator::UMINUS_EW} else {openmodelica_ast::Absyn::Operator::UMINUS},
        Op::AND => openmodelica_ast::Absyn::Operator::AND,
        Op::OR => openmodelica_ast::Absyn::Operator::OR,
        Op::NOT => openmodelica_ast::Absyn::Operator::NOT,
        Op::LESS => openmodelica_ast::Absyn::Operator::LESS,
        Op::LESSEQ => openmodelica_ast::Absyn::Operator::LESSEQ,
        Op::GREATER => openmodelica_ast::Absyn::Operator::GREATER,
        Op::EQUAL => openmodelica_ast::Absyn::Operator::EQUAL,
        Op::NEQUAL => openmodelica_ast::Absyn::Operator::NEQUAL,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.toAbsyn")); __mm_s.push_str(&*literal!(" got unknown type.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(aop)
}

pub fn toDAE(mut op: Arc<NFOperator>) -> Result<(DAE::Operator, bool, bool)> {
    let mut daeOp: DAE::Operator;
    let mut swapArguments: bool = false;
    let mut negate: bool = false;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Type::toDAE(op.ty.clone(), true)?;
    daeOp = (match op.op.clone() {
        Op::ADD => if (Type::isArray(op.ty.clone())) {DAE::Operator::ADD_ARR { ty: ty.clone() }} else {DAE::Operator::ADD { ty: ty.clone() }},
        Op::SUB => if (Type::isArray(op.ty.clone())) {DAE::Operator::SUB_ARR { ty: ty.clone() }} else {DAE::Operator::SUB { ty: ty.clone() }},
        Op::MUL => if (Type::isArray(op.ty.clone())) {DAE::Operator::MUL_ARR { ty: ty.clone() }} else {DAE::Operator::MUL { ty: ty.clone() }},
        Op::DIV => if (Type::isArray(op.ty.clone())) {DAE::Operator::DIV_ARR { ty: ty.clone() }} else {DAE::Operator::DIV { ty: ty.clone() }},
        Op::POW => if (Type::isArray(op.ty.clone())) {DAE::Operator::POW_ARR2 { ty: ty.clone() }} else {DAE::Operator::POW { ty: ty.clone() }},
        Op::ADD_SCALAR_ARRAY => {
            swapArguments = true;
            DAE::Operator::ADD_ARRAY_SCALAR { ty: ty.clone() }
        },
        Op::ADD_ARRAY_SCALAR { .. } => DAE::Operator::ADD_ARRAY_SCALAR { ty: ty.clone() },
        Op::SUB_SCALAR_ARRAY { .. } => DAE::Operator::SUB_SCALAR_ARRAY { ty: ty.clone() },
        Op::SUB_ARRAY_SCALAR => {
            negate = true;
            DAE::Operator::ADD_ARRAY_SCALAR { ty: ty.clone() }
        },
        Op::MUL_SCALAR_ARRAY => {
            swapArguments = true;
            DAE::Operator::MUL_ARRAY_SCALAR { ty: ty.clone() }
        },
        Op::MUL_ARRAY_SCALAR { .. } => DAE::Operator::MUL_ARRAY_SCALAR { ty: ty.clone() },
        Op::MUL_VECTOR_MATRIX => DAE::Operator::MUL_MATRIX_PRODUCT { ty: ty.clone() },
        Op::MUL_MATRIX_VECTOR => DAE::Operator::MUL_MATRIX_PRODUCT { ty: ty.clone() },
        Op::SCALAR_PRODUCT => DAE::Operator::MUL_SCALAR_PRODUCT { ty: ty.clone() },
        Op::ADD_EW => DAE::Operator::ADD_ARR { ty: ty.clone() },
        Op::SUB_EW => DAE::Operator::SUB_ARR { ty: ty.clone() },
        Op::MUL_EW => DAE::Operator::MUL_ARR { ty: ty.clone() },
        Op::DIV_EW => DAE::Operator::DIV_ARR { ty: ty.clone() },
        Op::MATRIX_PRODUCT => DAE::Operator::MUL_MATRIX_PRODUCT { ty: ty.clone() },
        Op::DIV_SCALAR_ARRAY { .. } => DAE::Operator::DIV_SCALAR_ARRAY { ty: ty.clone() },
        Op::DIV_ARRAY_SCALAR { .. } => DAE::Operator::DIV_ARRAY_SCALAR { ty: ty.clone() },
        Op::POW_SCALAR_ARRAY { .. } => DAE::Operator::POW_SCALAR_ARRAY { ty: ty.clone() },
        Op::POW_ARRAY_SCALAR { .. } => DAE::Operator::POW_ARRAY_SCALAR { ty: ty.clone() },
        Op::POW_MATRIX => DAE::Operator::POW_ARR { ty: ty.clone() },
        Op::UMINUS => if (Type::isArray(op.ty.clone())) {DAE::Operator::UMINUS_ARR { ty: ty.clone() }} else {DAE::Operator::UMINUS { ty: ty.clone() }},
        Op::AND => DAE::Operator::AND { ty: ty.clone() },
        Op::OR => DAE::Operator::OR { ty: ty.clone() },
        Op::NOT => DAE::Operator::NOT { ty: ty.clone() },
        Op::LESS => DAE::Operator::LESS { ty: ty.clone() },
        Op::LESSEQ => DAE::Operator::LESSEQ { ty: ty.clone() },
        Op::GREATER => DAE::Operator::GREATER { ty: ty.clone() },
        Op::GREATEREQ => DAE::Operator::GREATEREQ { ty: ty.clone() },
        Op::EQUAL => DAE::Operator::EQUAL { ty: ty.clone() },
        Op::NEQUAL => DAE::Operator::NEQUAL { ty: ty.clone() },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.toDAE")); __mm_s.push_str(&*literal!(" got unknown type: ")); __mm_s.push_str(&*opToString(op.op.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok((daeOp, swapArguments, negate))
}

pub fn typeOf(mut op: Arc<NFOperator>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = op.ty.clone();
    ty
}

pub fn setType(mut ty: Arc<Type::NFType>, mut op: Arc<NFOperator>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = op;
    assign_field!(op.ty = ty.clone());
    op
}

pub fn scalarize(mut op: Arc<NFOperator>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = op;
    assign_field!(op.ty = Type::arrayElementType(op.ty.clone()));
    op
}

pub fn unlift(mut op: Arc<NFOperator>) -> Result<Arc<NFOperator>> {
    let mut op: Arc<NFOperator> = op;
    assign_field!(op.ty = Type::unliftArray(op.ty.clone())?);
    Ok(op)
}

pub fn symbol(mut op: Arc<NFOperator>, mut spacing: ArcStr) -> Result<ArcStr> {
    let mut symbol: ArcStr = arcstr::literal!("");
    symbol = ((match op.op.clone() {
        Op::ADD => literal!("+"),
        Op::SUB => literal!("-"),
        Op::MUL => literal!("*"),
        Op::DIV => literal!("/"),
        Op::POW => literal!("^"),
        Op::ADD_EW => literal!(".+"),
        Op::SUB_EW => literal!(".-"),
        Op::MUL_EW => literal!(".*"),
        Op::DIV_EW => literal!("./"),
        Op::POW_EW => literal!(".^"),
        Op::ADD_SCALAR_ARRAY => literal!(".+"),
        Op::ADD_ARRAY_SCALAR { .. } => literal!(".+"),
        Op::SUB_SCALAR_ARRAY { .. } => literal!(".-"),
        Op::SUB_ARRAY_SCALAR => literal!(".-"),
        Op::MUL_SCALAR_ARRAY => literal!("*"),
        Op::MUL_ARRAY_SCALAR { .. } => literal!(".*"),
        Op::MUL_VECTOR_MATRIX => literal!("*"),
        Op::MUL_MATRIX_VECTOR => literal!("*"),
        Op::SCALAR_PRODUCT => literal!("*"),
        Op::MATRIX_PRODUCT => literal!("*"),
        Op::DIV_SCALAR_ARRAY { .. } => literal!("./"),
        Op::DIV_ARRAY_SCALAR { .. } => literal!("/"),
        Op::POW_SCALAR_ARRAY { .. } => literal!(".^"),
        Op::POW_ARRAY_SCALAR { .. } => literal!(".^"),
        Op::POW_MATRIX => literal!("^"),
        Op::UMINUS => literal!("-"),
        Op::AND => literal!("and"),
        Op::OR => literal!("or"),
        Op::NOT => literal!("not"),
        Op::LESS => literal!("<"),
        Op::LESSEQ => literal!("<="),
        Op::GREATER => literal!(">"),
        Op::GREATEREQ => literal!(">="),
        Op::EQUAL => literal!("=="),
        Op::NEQUAL => literal!("<>"),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.symbol")); __mm_s.push_str(&*literal!(" got unknown type.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    })).clone();
    symbol = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*spacing.clone()); __mm_s.push_str(&*symbol.clone()); __mm_s.push_str(&*spacing.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(symbol)
}

pub fn toJSON(mut operator: Arc<NFOperator>) -> Arc<JSON::JSON> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let symbols: metamodelica::Array<Arc<JSON::JSON>> = metamodelica::Dangerous::listArray(list![Arc::new(JSON::JSON::STRING { r#str: (literal!("+")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("-")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("/")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("^")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".+")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".-")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("./")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".^")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".+")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".+")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".-")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".-")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("*")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("./")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("/")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".^")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(".^")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("^")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("-")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("and")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("or")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("not")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("<")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("<=")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(">")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!(">=")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("==")).clone() }), Arc::new(JSON::JSON::STRING { r#str: (literal!("<>")).clone() })]);
    let mut op: Op = operator.op.clone();
    json = symbols.borrow()[(((op.clone()) as i32)-1) as usize].clone();
    json
}

pub fn priority(mut op: Arc<NFOperator>, mut lhs: bool) -> i32 {
    let mut priority: i32 = 0;
    priority = (match op.op.clone() {
        Op::ADD => if (lhs.clone()) {5} else {6},
        Op::SUB => 5,
        Op::MUL => 2,
        Op::DIV => 2,
        Op::POW => 1,
        Op::ADD_EW => if (lhs.clone()) {5} else {6},
        Op::SUB_EW => 5,
        Op::MUL_EW => if (lhs.clone()) {2} else {3},
        Op::DIV_EW => 2,
        Op::POW_EW => 1,
        Op::ADD_SCALAR_ARRAY => if (lhs.clone()) {5} else {6},
        Op::ADD_ARRAY_SCALAR { .. } => if (lhs.clone()) {5} else {6},
        Op::SUB_SCALAR_ARRAY { .. } => 5,
        Op::SUB_ARRAY_SCALAR => 5,
        Op::MUL_SCALAR_ARRAY => if (lhs.clone()) {2} else {3},
        Op::MUL_ARRAY_SCALAR { .. } => if (lhs.clone()) {2} else {3},
        Op::MUL_VECTOR_MATRIX => if (lhs.clone()) {2} else {3},
        Op::MUL_MATRIX_VECTOR => if (lhs.clone()) {2} else {3},
        Op::SCALAR_PRODUCT => if (lhs.clone()) {2} else {3},
        Op::MATRIX_PRODUCT => if (lhs.clone()) {2} else {3},
        Op::DIV_SCALAR_ARRAY { .. } => 2,
        Op::DIV_ARRAY_SCALAR { .. } => 2,
        Op::POW_SCALAR_ARRAY { .. } => 1,
        Op::POW_ARRAY_SCALAR { .. } => 1,
        Op::POW_MATRIX => 1,
        Op::AND => 8,
        Op::OR => 9,
        _ => 0,
    });
    priority
}

pub fn isAssociative(mut op: Arc<NFOperator>) -> bool {
    let mut isAssociative: bool = false;
    isAssociative = (match op.op.clone() {
        Op::ADD => true,
        Op::ADD_EW => true,
        Op::MUL_EW => true,
        _ => false,
    });
    isAssociative
}

pub fn isNonAssociative(mut op: Arc<NFOperator>) -> bool {
    let mut isNonAssociative: bool = false;
    isNonAssociative = (match op.op.clone() {
        Op::POW => true,
        Op::POW_EW => true,
        Op::POW_SCALAR_ARRAY { .. } => true,
        Op::POW_ARRAY_SCALAR { .. } => true,
        Op::POW_MATRIX => true,
        _ => false,
    });
    isNonAssociative
}

pub fn makeAdd(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::ADD.clone() });
    op
}

pub fn makeSub(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::SUB.clone() });
    op
}

pub fn makeMul(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::MUL.clone() });
    op
}

pub fn makeScalarProduct(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::SCALAR_PRODUCT.clone() });
    op
}

pub fn makeDiv(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::DIV.clone() });
    op
}

pub fn makePow(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::POW.clone() });
    op
}

pub fn makeAddEW(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::ADD_EW.clone() });
    op
}

pub fn makeSubEW(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::SUB_EW.clone() });
    op
}

pub fn makeMulEW(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::MUL_EW.clone() });
    op
}

pub fn makeDivEW(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::DIV_EW.clone() });
    op
}

pub fn makeUMinus(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::UMINUS.clone() });
    op
}

pub fn makeAnd(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::AND.clone() });
    op
}

pub fn makeOr(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::OR.clone() });
    op
}

pub fn makeNot(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::NOT.clone() });
    op
}

pub fn makeLess(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::LESS.clone() });
    op
}

pub fn makeLessEq(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::LESSEQ.clone() });
    op
}

pub fn makeGreater(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::GREATER.clone() });
    op
}

pub fn makeGreaterEq(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::GREATEREQ.clone() });
    op
}

pub fn makeEqual(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::EQUAL.clone() });
    op
}

pub fn makeNotEqual(mut ty: Arc<Type::NFType>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = Arc::new(NFOperator { ty: ty.clone(), op: Op::NEQUAL.clone() });
    op
}

pub fn makeScalarArray(mut ty: Arc<Type::NFType>, mut op: Op) -> Result<Arc<NFOperator>> {
    let mut outOp: Arc<NFOperator> = Arc::new(<NFOperator as ::std::default::Default>::default());
    let mut o: Op = Op::ADD;
    o = (match op.clone() {
        Op::ADD => Op::ADD_SCALAR_ARRAY.clone(),
        Op::SUB => Op::SUB_SCALAR_ARRAY.clone(),
        Op::MUL => Op::MUL_SCALAR_ARRAY.clone(),
        Op::DIV => Op::DIV_SCALAR_ARRAY.clone(),
        Op::POW => Op::POW_SCALAR_ARRAY.clone(),
        _ => bail!("match: no arm matched"),
    });
    outOp = Arc::new(NFOperator { ty: ty.clone(), op: o.clone() });
    Ok(outOp)
}

pub fn makeArrayScalar(mut ty: Arc<Type::NFType>, mut op: Op) -> Result<Arc<NFOperator>> {
    let mut outOp: Arc<NFOperator> = Arc::new(<NFOperator as ::std::default::Default>::default());
    let mut o: Op = Op::ADD;
    o = (match op.clone() {
        Op::ADD => Op::ADD_ARRAY_SCALAR.clone(),
        Op::SUB => Op::SUB_ARRAY_SCALAR.clone(),
        Op::MUL => Op::MUL_ARRAY_SCALAR.clone(),
        Op::DIV => Op::DIV_ARRAY_SCALAR.clone(),
        Op::POW => Op::POW_ARRAY_SCALAR.clone(),
        _ => bail!("match: no arm matched"),
    });
    outOp = Arc::new(NFOperator { ty: ty.clone(), op: o.clone() });
    Ok(outOp)
}

pub fn makeEW(mut op: Arc<NFOperator>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = op;
    let () = (match op.op.clone() {
        Op::ADD => {
            assign_field!(op.op = Op::ADD_EW.clone());
            ()
        },
        Op::SUB => {
            assign_field!(op.op = Op::SUB_EW.clone());
            ()
        },
        Op::MUL => {
            assign_field!(op.op = Op::MUL_EW.clone());
            ()
        },
        Op::DIV => {
            assign_field!(op.op = Op::DIV_EW.clone());
            ()
        },
        Op::POW => {
            assign_field!(op.op = Op::POW_EW.clone());
            ()
        },
        _ => (),
    });
    op
}

pub fn stripEW(mut op: Arc<NFOperator>) -> Arc<NFOperator> {
    let mut op: Arc<NFOperator> = op;
    let () = (match op.op.clone() {
        Op::ADD_EW => {
            assign_field!(op.op = Op::ADD.clone());
            ()
        },
        Op::SUB_EW => {
            assign_field!(op.op = Op::SUB.clone());
            ()
        },
        Op::MUL_EW => {
            assign_field!(op.op = Op::MUL.clone());
            ()
        },
        Op::DIV_EW => {
            assign_field!(op.op = Op::DIV.clone());
            ()
        },
        Op::POW_EW => {
            assign_field!(op.op = Op::POW.clone());
            ()
        },
        _ => (),
    });
    op
}

pub fn isElementWise(mut op: Arc<NFOperator>) -> bool {
    let mut ew: bool = false;
    ew = (match op.op.clone() {
        Op::ADD_EW => true,
        Op::SUB_EW => true,
        Op::MUL_EW => true,
        Op::DIV_EW => true,
        Op::POW_EW => true,
        _ => false,
    });
    ew
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum MathClassification {
    ADDITION = 1,
    SUBTRACTION = 2,
    MULTIPLICATION = 3,
    DIVISION = 4,
    POWER = 5,
    LOGICAL = 6,
    RELATION = 7,
}
impl PartialOrd for MathClassification {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for MathClassification {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SizeClassification {
    SCALAR = 1,
    ELEMENT_WISE = 2,
    ARRAY_SCALAR = 3,
    SCALAR_ARRAY = 4,
    MATRIX = 5,
    VECTOR_MATRIX = 6,
    MATRIX_VECTOR = 7,
    LOGICAL = 8,
    RELATION = 9,
}
impl PartialOrd for SizeClassification {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SizeClassification {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub type Classification = (MathClassification, SizeClassification);

pub fn mathSymbol(mut mcl: MathClassification) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match mcl.clone() {
        MathClassification::ADDITION => literal!("+"),
        MathClassification::SUBTRACTION => literal!("-"),
        MathClassification::MULTIPLICATION => literal!("*"),
        MathClassification::DIVISION { .. } => literal!("/"),
        MathClassification::POWER => literal!("^"),
        MathClassification::LOGICAL => literal!("L"),
        MathClassification::RELATION { .. } => literal!("R"),
        _ => bail!("fail"),
    })).clone();
    Ok(r#str)
}

pub fn classificationString(mut cla: Classification) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut mcl: MathClassification = MathClassification::ADDITION;
    let mut scl: SizeClassification = SizeClassification::SCALAR;
    (mcl, scl) = cla.clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*mathClassificationString(mcl.clone())?); __mm_s.push_str(&*sizeClassificationString(scl.clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn mathClassificationString(mut mcl: MathClassification) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match mcl.clone() {
        MathClassification::ADDITION => literal!("[ADD]"),
        MathClassification::SUBTRACTION => literal!("[SUB]"),
        MathClassification::MULTIPLICATION => literal!("[MUL]"),
        MathClassification::DIVISION { .. } => literal!("[DIV]"),
        MathClassification::POWER => literal!("[POW]"),
        MathClassification::LOGICAL => literal!("[LOG]"),
        MathClassification::RELATION { .. } => literal!("[REL]"),
        _ => bail!("fail"),
    })).clone();
    Ok(r#str)
}

pub fn sizeClassificationString(mut scl: SizeClassification) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match scl.clone() {
        SizeClassification::SCALAR => literal!("[SCALAR]"),
        SizeClassification::ELEMENT_WISE => literal!("[ELMWIS]"),
        SizeClassification::ARRAY_SCALAR => literal!("[ARR-SC]"),
        SizeClassification::SCALAR_ARRAY => literal!("[SC-ARR]"),
        SizeClassification::MATRIX { .. } => literal!("[MATRIX]"),
        SizeClassification::VECTOR_MATRIX => literal!("[VEC-MA]"),
        SizeClassification::MATRIX_VECTOR => literal!("[MA-VEC]"),
        SizeClassification::LOGICAL => literal!("[LOGICL]"),
        SizeClassification::RELATION { .. } => literal!("[RELATN]"),
        _ => bail!("fail"),
    })).clone();
    Ok(r#str)
}

pub fn classify(mut op: Arc<NFOperator>) -> Result<Classification> {
    let mut cl: Classification;
    cl = (match op.op.clone() {
        Op::ADD => (MathClassification::ADDITION.clone(), SizeClassification::SCALAR.clone()),
        Op::SUB => (MathClassification::SUBTRACTION.clone(), SizeClassification::SCALAR.clone()),
        Op::MUL => (MathClassification::MULTIPLICATION.clone(), SizeClassification::SCALAR.clone()),
        Op::DIV => (MathClassification::DIVISION.clone(), SizeClassification::SCALAR.clone()),
        Op::POW => (MathClassification::POWER.clone(), SizeClassification::SCALAR.clone()),
        Op::ADD_EW => (MathClassification::ADDITION.clone(), SizeClassification::ELEMENT_WISE.clone()),
        Op::SUB_EW => (MathClassification::SUBTRACTION.clone(), SizeClassification::ELEMENT_WISE.clone()),
        Op::MUL_EW => (MathClassification::MULTIPLICATION.clone(), SizeClassification::ELEMENT_WISE.clone()),
        Op::DIV_EW => (MathClassification::DIVISION.clone(), SizeClassification::ELEMENT_WISE.clone()),
        Op::POW_EW => (MathClassification::POWER.clone(), SizeClassification::ELEMENT_WISE.clone()),
        Op::MUL_ARRAY_SCALAR { .. } => (MathClassification::MULTIPLICATION.clone(), SizeClassification::ARRAY_SCALAR.clone()),
        Op::MUL_SCALAR_ARRAY => (MathClassification::MULTIPLICATION.clone(), SizeClassification::SCALAR_ARRAY.clone()),
        Op::ADD_ARRAY_SCALAR { .. } => (MathClassification::ADDITION.clone(), SizeClassification::ARRAY_SCALAR.clone()),
        Op::ADD_SCALAR_ARRAY => (MathClassification::ADDITION.clone(), SizeClassification::SCALAR_ARRAY.clone()),
        Op::SUB_ARRAY_SCALAR => (MathClassification::SUBTRACTION.clone(), SizeClassification::ARRAY_SCALAR.clone()),
        Op::SUB_SCALAR_ARRAY { .. } => (MathClassification::SUBTRACTION.clone(), SizeClassification::SCALAR_ARRAY.clone()),
        Op::SCALAR_PRODUCT => (MathClassification::MULTIPLICATION.clone(), SizeClassification::SCALAR.clone()),
        Op::MATRIX_PRODUCT => (MathClassification::MULTIPLICATION.clone(), SizeClassification::MATRIX.clone()),
        Op::MUL_VECTOR_MATRIX => (MathClassification::MULTIPLICATION.clone(), SizeClassification::VECTOR_MATRIX.clone()),
        Op::MUL_MATRIX_VECTOR => (MathClassification::MULTIPLICATION.clone(), SizeClassification::MATRIX_VECTOR.clone()),
        Op::DIV_ARRAY_SCALAR { .. } => (MathClassification::DIVISION.clone(), SizeClassification::ARRAY_SCALAR.clone()),
        Op::DIV_SCALAR_ARRAY { .. } => (MathClassification::DIVISION.clone(), SizeClassification::SCALAR_ARRAY.clone()),
        Op::POW_ARRAY_SCALAR { .. } => (MathClassification::POWER.clone(), SizeClassification::ARRAY_SCALAR.clone()),
        Op::POW_SCALAR_ARRAY { .. } => (MathClassification::POWER.clone(), SizeClassification::SCALAR_ARRAY.clone()),
        Op::POW_MATRIX => (MathClassification::POWER.clone(), SizeClassification::MATRIX.clone()),
        Op::AND => (MathClassification::LOGICAL.clone(), SizeClassification::LOGICAL.clone()),
        Op::OR => (MathClassification::LOGICAL.clone(), SizeClassification::LOGICAL.clone()),
        Op::NOT => (MathClassification::LOGICAL.clone(), SizeClassification::LOGICAL.clone()),
        Op::LESS => (MathClassification::RELATION.clone(), SizeClassification::RELATION.clone()),
        Op::LESSEQ => (MathClassification::RELATION.clone(), SizeClassification::RELATION.clone()),
        Op::GREATER => (MathClassification::RELATION.clone(), SizeClassification::RELATION.clone()),
        Op::GREATEREQ => (MathClassification::RELATION.clone(), SizeClassification::RELATION.clone()),
        Op::EQUAL => (MathClassification::RELATION.clone(), SizeClassification::RELATION.clone()),
        Op::NEQUAL => (MathClassification::RELATION.clone(), SizeClassification::RELATION.clone()),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.classify")); __mm_s.push_str(&*literal!(": Don't know how to handle ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", op.op.clone()))); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(cl)
}

pub fn classifyAddition(mut op: Arc<NFOperator>) -> SizeClassification {
    let mut sz: SizeClassification = if (Type::isScalar(op.ty.clone())) {SizeClassification::SCALAR.clone()} else {SizeClassification::ELEMENT_WISE.clone()};
    sz
}

pub fn fromClassification(mut cl: Classification, mut ty: Arc<Type::NFType>) -> Result<Arc<NFOperator>> {
    let mut result: Arc<NFOperator> = Arc::new(<NFOperator as ::std::default::Default>::default());
    let mut op: Op = Op::ADD;
    op = (match cl.clone() {
        (MathClassification::ADDITION, SizeClassification::SCALAR) => Op::ADD.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::SCALAR) => Op::SUB.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::SCALAR) => Op::MUL.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::SCALAR) => Op::DIV.clone(),
        (MathClassification::POWER, SizeClassification::SCALAR) => Op::POW.clone(),
        (MathClassification::ADDITION, SizeClassification::ELEMENT_WISE) => Op::ADD_EW.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::ELEMENT_WISE) => Op::SUB_EW.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::ELEMENT_WISE) => Op::MUL_EW.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::ELEMENT_WISE) => Op::DIV_EW.clone(),
        (MathClassification::POWER, SizeClassification::ELEMENT_WISE) => Op::POW_EW.clone(),
        (MathClassification::ADDITION, SizeClassification::ARRAY_SCALAR) => Op::ADD_ARRAY_SCALAR.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::ARRAY_SCALAR) => Op::SUB_ARRAY_SCALAR.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::ARRAY_SCALAR) => Op::MUL_ARRAY_SCALAR.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::ARRAY_SCALAR) => Op::DIV_ARRAY_SCALAR.clone(),
        (MathClassification::POWER, SizeClassification::ARRAY_SCALAR) => Op::POW_ARRAY_SCALAR.clone(),
        (MathClassification::ADDITION, SizeClassification::SCALAR_ARRAY) => Op::ADD_SCALAR_ARRAY.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::SCALAR_ARRAY) => Op::SUB_SCALAR_ARRAY.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::SCALAR_ARRAY) => Op::MUL_SCALAR_ARRAY.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::SCALAR_ARRAY) => Op::DIV_SCALAR_ARRAY.clone(),
        (MathClassification::POWER, SizeClassification::SCALAR_ARRAY) => Op::POW_SCALAR_ARRAY.clone(),
        (MathClassification::ADDITION, SizeClassification::MATRIX { .. }) => Op::ADD_EW.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::MATRIX { .. }) => Op::SUB_EW.clone(),
        (MathClassification::POWER, SizeClassification::MATRIX { .. }) => Op::POW_MATRIX.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::MATRIX { .. }) => Op::MATRIX_PRODUCT.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::VECTOR_MATRIX) => Op::MUL_VECTOR_MATRIX.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::MATRIX_VECTOR) => Op::MUL_MATRIX_VECTOR.clone(),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.fromClassification")); __mm_s.push_str(&*literal!(": Don't know how to handle math class and size class combination: ")); __mm_s.push_str(&*classificationString(cl.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    result = Arc::new(NFOperator { ty: ty.clone(), op: op.clone() });
    Ok(result)
}

pub fn getMathClassification(mut op: Arc<NFOperator>) -> Result<MathClassification> {
    let mut mcl: MathClassification = MathClassification::ADDITION;
    (mcl, _) = classify(op.clone())?;
    Ok(mcl)
}

pub fn getSizeClassification(mut op: Arc<NFOperator>) -> Result<SizeClassification> {
    let mut scl: SizeClassification = SizeClassification::SCALAR;
    (_, scl) = classify(op.clone())?;
    Ok(scl)
}

pub fn combineSizeClassification(mut scl1: SizeClassification, mut scl2: SizeClassification) -> SizeClassification {
    let mut scl: SizeClassification = SizeClassification::SCALAR;
    scl = (match (scl1.clone(), scl2.clone()) {
        (SizeClassification::ELEMENT_WISE, SizeClassification::SCALAR) => SizeClassification::ARRAY_SCALAR.clone(),
        (SizeClassification::SCALAR, SizeClassification::ELEMENT_WISE) => SizeClassification::SCALAR_ARRAY.clone(),
        _ => scl1.clone(),
    });
    scl
}

pub fn isDashClassification(mut mcl: MathClassification) -> bool {
    let mut b: bool = false;
    b = (match mcl.clone() {
        MathClassification::ADDITION => true,
        MathClassification::SUBTRACTION => true,
        _ => false,
    });
    b
}

pub fn isCommutative(mut operator: Arc<NFOperator>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(Type::arrayElementType(operator.ty.clone())) {
        Deref @ Type::INTEGER => true,
        Deref @ Type::REAL => true,
        Deref @ Type::BOOLEAN => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(b.clone()) {
        return b;
    }
    b = (match operator.op.clone() {
        Op::ADD => true,
        Op::MUL => true,
        Op::ADD_EW => true,
        Op::MUL_EW => true,
        Op::ADD_SCALAR_ARRAY => true,
        Op::ADD_ARRAY_SCALAR { .. } => true,
        Op::MUL_SCALAR_ARRAY => true,
        Op::MUL_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    b
}

pub fn isSoftCommutative(mut operator: Arc<NFOperator>) -> bool {
    let mut b: bool = false;
    b = (match operator.op.clone() {
        Op::SUB => true,
        Op::DIV => true,
        Op::SUB_EW => true,
        Op::DIV_EW => true,
        Op::SUB_SCALAR_ARRAY { .. } => true,
        Op::SUB_ARRAY_SCALAR => true,
        Op::DIV_SCALAR_ARRAY { .. } => true,
        Op::DIV_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    b
}

pub fn repetition(mut operator: Arc<NFOperator>) -> (bool, bool) {
    let mut b: (bool, bool);
    b = (match operator.op.clone() {
        Op::ADD_SCALAR_ARRAY => (true, false),
        Op::ADD_ARRAY_SCALAR { .. } => (false, true),
        Op::MUL_SCALAR_ARRAY => (true, false),
        Op::MUL_ARRAY_SCALAR { .. } => (false, true),
        Op::MUL_VECTOR_MATRIX => (true, true),
        Op::MUL_MATRIX_VECTOR => (true, true),
        Op::MATRIX_PRODUCT => (true, true),
        _ => (false, false),
    });
    b
}

pub fn reduction(mut operator: Arc<NFOperator>) -> bool {
    let mut b: bool = false;
    b = (match operator.op.clone() {
        Op::MUL_MATRIX_VECTOR => true,
        Op::MUL_VECTOR_MATRIX => true,
        Op::MATRIX_PRODUCT => true,
        Op::SCALAR_PRODUCT => true,
        _ => false,
    });
    b
}

pub fn isCombineable(mut op1: Arc<NFOperator>, mut op2: Arc<NFOperator>) -> Result<bool> {
    let mut b: bool = false;
    let mut mcl1: MathClassification = MathClassification::ADDITION;
    let mut mcl2: MathClassification = MathClassification::ADDITION;
    let mut scl1: SizeClassification = SizeClassification::SCALAR;
    let mut scl2: SizeClassification = SizeClassification::SCALAR;
    (mcl1, scl1) = classify(op1.clone())?;
    (mcl2, scl2) = classify(op2.clone())?;
    b = isCombineableMath(mcl1.clone(), mcl2.clone()) && isCombineableSize(scl1.clone(), scl2.clone());
    if b.clone() {
        b = !(isScalarProduct(op1.clone()) || isScalarProduct(op2.clone()));
    }
    Ok(b)
}

pub fn isCombineableMath(mut mcl1: MathClassification, mut mcl2: MathClassification) -> bool {
    let mut b: bool = false;
    b = mcl1.clone() == mcl2.clone() || isDashClassification(mcl1.clone()) && isDashClassification(mcl2.clone());
    b
}

pub fn isCombineableSize(mut scl1: SizeClassification, mut scl2: SizeClassification) -> bool {
    let mut b: bool = false;
    b = scl1.clone() == scl2.clone();
    b
}

pub fn toDebugString(mut op: Arc<NFOperator>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OPERATOR(")); __mm_s.push_str(&*Type::toString(op.ty.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*opToString(op.op.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn opToString(mut op: Op) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match op.clone() {
        Op::ADD => literal!("ADD"),
        Op::SUB => literal!("SUB"),
        Op::MUL => literal!("MUL"),
        Op::DIV => literal!("DIV"),
        Op::POW => literal!("POW"),
        Op::ADD_EW => literal!("ADD_EW"),
        Op::SUB_EW => literal!("SUB_EW"),
        Op::MUL_EW => literal!("MUL_EW"),
        Op::DIV_EW => literal!("DIV_EW"),
        Op::POW_EW => literal!("POW_EW"),
        Op::ADD_SCALAR_ARRAY => literal!("ADD_SCALAR_ARRAY"),
        Op::ADD_ARRAY_SCALAR { .. } => literal!("ADD_ARRAY_SCALAR"),
        Op::SUB_SCALAR_ARRAY { .. } => literal!("SUB_SCALAR_ARRAY"),
        Op::SUB_ARRAY_SCALAR => literal!("SUB_ARRAY_SCALAR"),
        Op::MUL_SCALAR_ARRAY => literal!("MUL_SCALAR_ARRAY"),
        Op::MUL_ARRAY_SCALAR { .. } => literal!("MUL_ARRAY_SCALAR"),
        Op::MUL_VECTOR_MATRIX => literal!("MUL_VECTOR_MATRIX"),
        Op::MUL_MATRIX_VECTOR => literal!("MUL_MATRIX_VECTOR"),
        Op::SCALAR_PRODUCT => literal!("SCALAR_PRODUCT"),
        Op::MATRIX_PRODUCT => literal!("MATRIX_PRODUCT"),
        Op::DIV_SCALAR_ARRAY { .. } => literal!("DIV_SCALAR_ARRAY"),
        Op::DIV_ARRAY_SCALAR { .. } => literal!("DIV_ARRAY_SCALAR"),
        Op::POW_SCALAR_ARRAY { .. } => literal!("POW_SCALAR_ARRAY"),
        Op::POW_ARRAY_SCALAR { .. } => literal!("POW_ARRAY_SCALAR"),
        Op::POW_MATRIX => literal!("POW_MATRIX"),
        Op::UMINUS => literal!("UMINUS"),
        Op::AND => literal!("AND"),
        Op::OR => literal!("OR"),
        Op::NOT => literal!("NOT"),
        Op::LESS => literal!("LESS"),
        Op::LESSEQ => literal!("LESSEQ"),
        Op::GREATER => literal!("GREATER"),
        Op::GREATEREQ => literal!("GREATEREQ"),
        Op::EQUAL => literal!("EQUAL"),
        Op::NEQUAL => literal!("NEQUAL"),
        Op::USERDEFINED { .. } => literal!("USERDEFINED"),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.opToString")); __mm_s.push_str(&*literal!("failed. Unhanded enumeration.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(r#str)
}


