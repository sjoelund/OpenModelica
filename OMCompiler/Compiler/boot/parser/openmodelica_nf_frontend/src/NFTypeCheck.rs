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

use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::Slot;
use crate::NFFunction::TypedArg;
use crate::NFInline as Inline;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRestriction as Restriction;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum MatchKind {
    /// Exact match
    EXACT = 1,
    /// Matched by casting, e.g. Integer to Real
    CAST = 2,
    /// The expected type was unknown
    UNKNOWN_EXPECTED = 3,
    /// The actual type was unknown
    UNKNOWN_ACTUAL = 4,
    /// Matched with a generic type e.g. function F<T> input T i; end F; F(1)
    GENERIC = 5,
    /// Component by component matching, e.g. class A R r; end A; is plug compatible with class B R r; end B;
    PLUG_COMPATIBLE = 6,
    NOT_COMPATIBLE = 7,
}
impl PartialOrd for MatchKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for MatchKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn isCompatibleMatch(mut kind: MatchKind) -> bool {
    let mut isCompatible: bool = kind.clone() != MatchKind::NOT_COMPATIBLE.clone();
    isCompatible
}

pub fn isIncompatibleMatch(mut kind: MatchKind) -> bool {
    let mut isIncompatible: bool = kind.clone() == MatchKind::NOT_COMPATIBLE.clone();
    isIncompatible
}

pub fn isExactMatch(mut kind: MatchKind) -> bool {
    let mut isCompatible: bool = kind.clone() == MatchKind::EXACT.clone();
    isCompatible
}

pub fn isCastMatch(mut kind: MatchKind) -> bool {
    let mut isCast: bool = kind.clone() == MatchKind::CAST.clone();
    isCast
}

pub fn isGenericMatch(mut kind: MatchKind) -> bool {
    let mut isCast: bool = kind.clone() == MatchKind::GENERIC.clone();
    isCast
}

pub fn isValidAssignmentMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind.clone() == MatchKind::EXACT.clone() || kind.clone() == MatchKind::CAST.clone() || kind.clone() == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub fn isValidArgumentMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind.clone() == MatchKind::EXACT.clone() || kind.clone() == MatchKind::CAST.clone() || kind.clone() == MatchKind::GENERIC.clone() || kind.clone() == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub fn isValidPlugCompatibleMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind.clone() == MatchKind::EXACT.clone() || kind.clone() == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub type MatchOptions = i32;

pub const DEFAULT_OPTIONS: i32 = 0;

pub const ALLOW_UNKNOWN: i32 = intBitLShift(1, 0);

pub const IGNORE_DIMENSIONS: i32 = intBitLShift(1, 1);

pub const IGNORE_DIMENSIONS_IN_RECORDS: i32 = intBitLShift(1, 2);

pub fn setOption(mut currentOptions: MatchOptions, mut newOption: MatchOptions) -> MatchOptions {
    let mut newOptions: MatchOptions = intBitOr(currentOptions.clone(), newOption.clone());
    newOptions
}

pub fn getOption(mut options: MatchOptions, mut option: MatchOptions) -> bool {
    let mut isSet: bool = intBitAnd(options.clone(), option.clone()) > 0;
    isSet
}

pub fn checkBinaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    if Type::isConditionalArray(type1.clone()) || Type::isConditionalArray(type2.clone()) {
        (binaryExp, resultType) = checkConditionalBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), operator.clone(), exp2.clone(), type2.clone(), var2.clone(), context.clone(), info.clone(), retype.clone())?;
    } else if Type::isComplex(Type::arrayElementType(type1.clone())) || Type::isComplex(Type::arrayElementType(type2.clone())) {
        (binaryExp, resultType) = checkOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), operator.clone(), exp2.clone(), type2.clone(), var2.clone(), context.clone(), info.clone())?;
    } else if Type::isBoxed(type1.clone()) && Type::isBoxed(type2.clone()) {
        (binaryExp, resultType) = checkBinaryOperationBoxed(exp1.clone(), type1.clone(), var1.clone(), operator.clone(), exp2.clone(), type2.clone(), var2.clone(), context.clone(), info.clone(), retype.clone())?;
    } else {
        (binaryExp, resultType) = (match operator.op.clone() {
        Operator::Op::ADD => checkBinaryOperationAdd(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::SUB => checkBinaryOperationSub(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::MUL => checkBinaryOperationMul(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::DIV => checkBinaryOperationDiv(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone(), retype.clone())?,
        Operator::Op::POW => checkBinaryOperationPow(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::ADD_EW => checkBinaryOperationEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), Op::ADD.clone(), info.clone())?,
        Operator::Op::SUB_EW => checkBinaryOperationEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), Op::SUB.clone(), info.clone())?,
        Operator::Op::MUL_EW => checkBinaryOperationEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), Op::MUL.clone(), info.clone())?,
        Operator::Op::DIV_EW => checkBinaryOperationDiv(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone(), true)?,
        Operator::Op::POW_EW => checkBinaryOperationPowEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::ADD_SCALAR_ARRAY => checkBinaryOperationEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), Op::ADD.clone(), info.clone())?,
        Operator::Op::ADD_ARRAY_SCALAR { .. } => checkBinaryOperationEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), Op::ADD.clone(), info.clone())?,
        Operator::Op::SUB_SCALAR_ARRAY { .. } => checkBinaryOperationEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), Op::SUB.clone(), info.clone())?,
        Operator::Op::SUB_ARRAY_SCALAR => checkBinaryOperationEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), Op::SUB.clone(), info.clone())?,
        Operator::Op::MUL_SCALAR_ARRAY => checkBinaryOperationMul(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::MUL_ARRAY_SCALAR { .. } => checkBinaryOperationMul(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::MUL_VECTOR_MATRIX => checkBinaryOperationMul(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::MUL_MATRIX_VECTOR => checkBinaryOperationMul(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::SCALAR_PRODUCT => checkBinaryOperationMul(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::MATRIX_PRODUCT => checkBinaryOperationMul(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::DIV_SCALAR_ARRAY { .. } => checkBinaryOperationDiv(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone(), retype.clone())?,
        Operator::Op::DIV_ARRAY_SCALAR { .. } => checkBinaryOperationDiv(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone(), retype.clone())?,
        Operator::Op::POW_SCALAR_ARRAY { .. } => checkBinaryOperationPowEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::POW_ARRAY_SCALAR { .. } => checkBinaryOperationPowEW(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        Operator::Op::POW_MATRIX => checkBinaryOperationPow(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), info.clone())?,
        _ => bail!("match: no arm matched"),
    });
    }
    Ok((binaryExp, resultType))
}

pub fn checkOverloadedBinaryOperator(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut op_str: ArcStr = arcstr::literal!("");
    let mut candidates: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut ety1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ety2: Arc<Type::NFType> = Arc::new(Type::ANY);
    op_str = (Operator::symbol(Operator::stripEW(op.clone()), (literal!("'")).clone())?).clone();
    ety1 = Type::arrayElementType(type1.clone());
    ety2 = Type::arrayElementType(type2.clone());
    candidates = OperatorOverloading::lookupOperatorFunctionsInType((op_str.clone()).clone(), ety1.clone())?;
    if !(Type::isEqual(ety1.clone(), ety2.clone())) {
        candidates = listAppend(OperatorOverloading::lookupOperatorFunctionsInType((op_str.clone()).clone(), ety2.clone())?, candidates.clone());
    }
    if candidates.clone().is_empty() {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    if Operator::isElementWise(op.clone()) {
        (outExp, outType) = checkOverloadedBinaryArrayEW(exp1.clone(), type1.clone(), var1.clone(), Operator::stripEW(op.clone()), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
    } else {
        (outExp, outType) = matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), true)?;
    }
    outExp = Inline::inlineCallExp(outExp.clone(), false)?;
    Ok((outExp, outType))
}

pub fn matchOverloadedBinaryOperator(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo, mut showErrors: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut matchedFunc: Arc<MatchedFunction::MatchedFunction> = Arc::new(<MatchedFunction::MatchedFunction as ::std::default::Default>::default());
    let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let mut exactMatches: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    args = list![Arc::new(TypedArg { name: None, value: exp1.clone(), ty: type1.clone(), var: var1.clone(), purity: Purity::PURE.clone() }), Arc::new(TypedArg { name: None, value: exp2.clone(), ty: type2.clone(), var: var2.clone(), purity: Purity::PURE.clone() })];
    matchedFunctions = Function::matchFunctionsSilent(candidates.clone(), args.clone(), metamodelica::nil(), context.clone(), info.clone(), true)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if exactMatches.clone().is_empty() {
        ErrorExt::setCheckpoint((literal!("NFTypeCheck:implicitConstruction")).clone());
        match '__try0: {
            (outExp, outType) = unwrap_break_err!(implicitConstructAndMatch(candidates.clone(), exp1.clone(), type1.clone(), op.clone(), exp2.clone(), type2.clone(), info.clone()), '__try0);
            if showErrors.clone() {
                ErrorExt::delCheckpoint((literal!("NFTypeCheck:implicitConstruction")).clone());
            } else {
                ErrorExt::rollBack((literal!("NFTypeCheck:implicitConstruction")).clone());
            }
            Ok::<_, anyhow::Error>((outExp.clone(), outType.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                outExp = __try0_o0;
                outType = __try0_o1;
            }
            Err(_) => {
                ErrorExt::rollBack((literal!("NFTypeCheck:implicitConstruction")).clone());
                if Type::isArray(type1.clone()) || Type::isArray(type2.clone()) {
                    (outExp, outType) = (match op.op.clone() {
        Operator::Op::ADD => checkOverloadedBinaryArrayAddSub(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?,
        Operator::Op::SUB => checkOverloadedBinaryArrayAddSub(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?,
        Operator::Op::MUL => checkOverloadedBinaryArrayMul(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?,
        Operator::Op::DIV => checkOverloadedBinaryArrayDiv(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?,
        _ => {
            printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), showErrors.clone())?;
            bail!("fail")
        },
    });
                } else {
                    printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), showErrors.clone())?;
                }
                bail!("try/else: outputs not set in else branch");
            }
        }
    } else if (exactMatches.clone().len() as i32) == 1 {
        let __pa1 = ::match_deref::match_deref! { match &(exactMatches.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        matchedFunc = __pa1.clone();
        r#fn = matchedFunc.func.clone();
        outType = Function::returnType(r#fn.clone());
        outExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(matchedFunc.func.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut a in (matchedFunc.args.clone()).into_iter().cloned() {
            let __x = a.value.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), Prefixes::variabilityMax(var1.clone(), var2.clone()), Purity::PURE.clone(), outType.clone()) });
    } else {
        if showErrors.clone() {
            Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }))?).clone(), (Function::candidateFuncListString(({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut mfn in (matchedFunctions.clone()).into_iter().cloned() {
            let __x = mfn.func.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))).clone()], info.clone())?;
        }
        bail!("fail");
    }
    Ok((outExp, outType))
}

pub fn checkBinaryOperationBoxed(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    (e1, ty1, _) = matchTypes(type1.clone(), Type::unbox(type1.clone()), exp1.clone(), DEFAULT_OPTIONS.clone())?;
    (e2, ty2, _) = matchTypes(type2.clone(), Type::unbox(type2.clone()), exp2.clone(), DEFAULT_OPTIONS.clone())?;
    (outExp, outType) = checkBinaryOperation(e1.clone(), ty1.clone(), var1.clone(), op.clone(), e2.clone(), ty2.clone(), var2.clone(), context.clone(), info.clone(), retype.clone())?;
    Ok((outExp, outType))
}

fn checkConditionalBinaryOperator(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut fty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut fty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut valid1: bool = false;
    let mut valid2: bool = false;
    let mut branch: Type::Branch = Type::Branch::NONE;
    (tty1, fty1, tty2, fty2, branch) = (::match_deref::match_deref! { match &((type1.clone(), type2.clone())) {
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => (var_field!((*type1).trueType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type1).falseType, Type::NFType::CONDITIONAL_ARRAY).clone(), type2.clone(), type2.clone(), var_field!((*type1).matchedBranch, Type::NFType::CONDITIONAL_ARRAY).clone()),
        (_, Deref @ Type::CONDITIONAL_ARRAY { .. }) => (type1.clone(), type1.clone(), var_field!((*type2).trueType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type2).falseType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type2).matchedBranch, Type::NFType::CONDITIONAL_ARRAY).clone()),
        _ => bail!("match: no arm matched"),
    } });
    ErrorExt::setCheckpoint((literal!("NFTypeCheck.checkConditionalBinaryOperator")).clone());
    match '__try0: {
        (e1, ty1) = unwrap_break_err!(checkBinaryOperation(exp1.clone(), tty1.clone(), var1.clone(), op.clone(), exp2.clone(), tty2.clone(), var2.clone(), context.clone(), info.clone(), retype.clone()), '__try0);
        valid1 = true;
        Ok::<_, anyhow::Error>((valid1.clone(),))
    } {
        Ok((__try0_o0,)) => {
            valid1 = __try0_o0;
        }
        Err(_) => {
            valid1 = false;
        }
    }
    match '__try1: {
        (e2, ty2) = unwrap_break_err!(checkBinaryOperation(exp1.clone(), fty1.clone(), var1.clone(), op.clone(), exp2.clone(), fty2.clone(), var2.clone(), context.clone(), info.clone(), retype.clone()), '__try1);
        valid2 = true;
        Ok::<_, anyhow::Error>((valid2.clone(),))
    } {
        Ok((__try1_o0,)) => {
            valid2 = __try1_o0;
        }
        Err(_) => {
            valid2 = false;
        }
    }
    ErrorExt::rollBack((literal!("NFTypeCheck.checkConditionalBinaryOperator")).clone());
    if valid1.clone() && valid2.clone() {
        outType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: ty1.clone(), falseType: ty2.clone(), matchedBranch: branch.clone() });
        outExp = e1.clone();
    } else if valid1.clone() {
        outType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: ty1.clone(), falseType: Arc::new(crate::NFType::UNKNOWN), matchedBranch: Type::Branch::TRUE.clone() });
        outExp = e1.clone();
    } else if valid2.clone() {
        outType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: Arc::new(crate::NFType::UNKNOWN), falseType: ty2.clone(), matchedBranch: Type::Branch::FALSE.clone() });
        outExp = e2.clone();
    } else {
        printUnresolvableTypeError(exp1.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    outExp = Expression::setType(outType.clone(), outExp.clone())?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayAddSub(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    (e1, e2, _, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), ALLOW_UNKNOWN.clone())?;
    if !(isCompatibleMatch(mk.clone())) {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    (e1, _) = ExpandExp::expand(e1.clone(), false, false)?;
    (e2, _) = ExpandExp::expand(e2.clone(), false, false)?;
    (outExp, outType) = checkOverloadedBinaryArrayAddSub2(e1.clone(), type1.clone(), var1.clone(), op.clone(), e2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayAddSub2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    (outExp, outType) = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::ARRAY { elements: arr1, .. }, Deref @ Expression::ARRAY { elements: arr2, .. }) => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            if arr1.clone().borrow().is_empty() {
                ty1 = Type::arrayElementType(type1.clone());
                ty2 = Type::arrayElementType(type2.clone());
                arr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                if '__try0: {
                    (_, ty) = unwrap_break_err!(matchOverloadedBinaryOperator(Arc::new(Expression::NFExpression::EMPTY { ty: ty1.clone() }), ty1.clone(), var1.clone(), op.clone(), Arc::new(Expression::NFExpression::EMPTY { ty: ty2.clone() }), ty2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), false), '__try0);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
                }
            } else {
                ty1 = Type::unliftArray(type1.clone())?;
                ty2 = Type::unliftArray(type2.clone())?;
                arr = metamodelica::arrayCreateDefault((arr1.clone().borrow().len() as i32));
                let __range1 = 1..=(arr1.clone().borrow().len() as i32);
                for mut i in __range1 {
                    e1 = arr1.clone().borrow()[(i.clone()-1) as usize].clone();
                    e2 = arr2.clone().borrow()[(i.clone()-1) as usize].clone();
                    (e, ty) = checkOverloadedBinaryArrayAddSub2(e1.clone(), ty1.clone(), var1.clone(), op.clone(), e2.clone(), ty2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
                    unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), e.clone()) };
                }
            }
            outType = Type::setArrayElementType(type1.clone(), ty.clone());
            outExp = Expression::makeArray(outType.clone(), arr.clone(), false);
            (outExp.clone(), outType.clone())
        },
        _ => {
            matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), true)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayMul(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut valid: bool = false;
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim11: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim12: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim21: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dims1 = Type::arrayDims(type1.clone());
    dims2 = Type::arrayDims(type2.clone());
    (valid, outExp) = (::match_deref::match_deref! { match &((dims1.clone(), dims2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }) => {
            (outExp, _) = checkOverloadedBinaryScalarArray(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
            (true, outExp.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) => {
            (outExp, _) = checkOverloadedBinaryArrayScalar(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
            (true, outExp.clone())
        },
        (Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Cons { head: dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: dim21, tail: Deref @ metamodelica::List::Nil }) => {
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            outExp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
            valid = false;
            (valid.clone(), outExp.clone())
        },
        (Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Cons { head: dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: dim21, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            outExp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
            valid = false;
            (valid.clone(), outExp.clone())
        },
        _ => (false, Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(valid.clone()) {
        printUnresolvableTypeError(outExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    outType = Expression::typeOf(outExp.clone());
    Ok((outExp, outType))
}

fn checkOverloadedBinaryScalarArray(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    (outExp, outType) = checkOverloadedBinaryScalarArray2(exp1.clone(), type1.clone(), var1.clone(), op.clone(), (ExpandExp::expand(exp2.clone(), false, false)?).0, type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryScalarArray2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (outExp, outType) = (::match_deref::match_deref! { match &(exp2.clone()) {
        Deref @ Expression::ARRAY { .. } if (var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty()) => {
            match '__try0: {
                ty = unwrap_break_err!(Type::unliftArray(type2.clone()), '__try0);
                (_, outType) = unwrap_break_err!(matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), op.clone(), Arc::new(Expression::NFExpression::EMPTY { ty: type2.clone() }), ty.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), false), '__try0);
                Ok::<_, anyhow::Error>((outType.clone(), ty.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    outType = __try0_o0;
                    ty = __try0_o1;
                }
                Err(_) => {
                    printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone()], info.clone(), true)?;
                    bail!("try/else: outputs not set in else branch");
                }
            }
            outType = Type::setArrayElementType(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), outType.clone());
            (Expression::makeEmptyArray(outType.clone()), outType.clone())
        },
        Deref @ Expression::ARRAY { .. } => {
            ty = Type::unliftArray(type2.clone())?;
            arr = metamodelica::arrayCreate((var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32), exp2.clone());
            let __range0 = 1..=(arr.clone().borrow().len() as i32);
            for mut i in __range0 {
                e2 = var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone().borrow()[(i.clone()-1) as usize].clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), (checkOverloadedBinaryScalarArray2(exp1.clone(), type1.clone(), var1.clone(), op.clone(), e2.clone(), ty.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?).0) };
            }
            outType = Type::setArrayElementType(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Expression::typeOf(arr.borrow()[(1-1) as usize].clone()));
            (Expression::makeArray(outType.clone(), arr.clone(), false), outType.clone())
        },
        _ => matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), true)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayScalar(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    (outExp, outType) = checkOverloadedBinaryArrayScalar2((ExpandExp::expand(exp1.clone(), false, false)?).0, type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayScalar2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    (outExp, outType) = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::ARRAY { .. } if (var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty()) => {
            match '__try0: {
                ty = unwrap_break_err!(Type::unliftArray(type1.clone()), '__try0);
                (_, outType) = unwrap_break_err!(matchOverloadedBinaryOperator(Arc::new(Expression::NFExpression::EMPTY { ty: type1.clone() }), ty.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), false), '__try0);
                Ok::<_, anyhow::Error>((outType.clone(), ty.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    outType = __try0_o0;
                    ty = __try0_o1;
                }
                Err(_) => {
                    printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone()], info.clone(), true)?;
                    bail!("try/else: outputs not set in else branch");
                }
            }
            outType = Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), outType.clone());
            (Expression::makeEmptyArray(outType.clone()), outType.clone())
        },
        Deref @ Expression::ARRAY { .. } => {
            ty = Type::unliftArray(type1.clone())?;
            arr = metamodelica::arrayCreate((var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32), exp1.clone());
            let __range0 = 1..=(arr.clone().borrow().len() as i32);
            for mut i in __range0 {
                e1 = var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone().borrow()[(i.clone()-1) as usize].clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), (checkOverloadedBinaryArrayScalar2(e1.clone(), ty.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?).0) };
            }
            outType = Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Expression::typeOf(arr.borrow()[(1-1) as usize].clone()));
            (Expression::makeArray(outType.clone(), arr.clone(), false), outType.clone())
        },
        _ => matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), true)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayDiv(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    if Type::isArray(type1.clone()) && Type::isScalar(type2.clone()) {
        (outExp, outType) = checkOverloadedBinaryArrayScalar(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
    } else {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayEW(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    if Type::isArray(type1.clone()) && Type::isArray(type2.clone()) {
        (e1, e2, _, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), ALLOW_UNKNOWN.clone())?;
    } else {
        (e1, e2, _, mk) = matchExpressions(exp1.clone(), Type::arrayElementType(type1.clone()), exp2.clone(), Type::arrayElementType(type2.clone()), ALLOW_UNKNOWN.clone())?;
    }
    if !(isCompatibleMatch(mk.clone())) {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    (e1, _) = ExpandExp::expand(exp1.clone(), false, false)?;
    (e2, _) = ExpandExp::expand(exp2.clone(), false, false)?;
    (outExp, outType) = checkOverloadedBinaryArrayEW2(e1.clone(), type1.clone(), var1.clone(), op.clone(), e2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayEW2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut expl1: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut expl2: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut is_array1: bool = false;
    let mut is_array2: bool = false;
    is_array1 = Type::isArray(type1.clone());
    is_array2 = Type::isArray(type2.clone());
    if is_array1.clone() || is_array2.clone() {
        expl = metamodelica::nil();
        if Expression::isEmptyArray(exp1.clone()) || Expression::isEmptyArray(exp2.clone()) {
            ty1 = Type::arrayElementType(type1.clone());
            ty2 = Type::arrayElementType(type2.clone());
            if '__try0: {
                (_, ty) = unwrap_break_err!(matchOverloadedBinaryOperator(Arc::new(Expression::NFExpression::EMPTY { ty: ty1.clone() }), ty1.clone(), var1.clone(), op.clone(), Arc::new(Expression::NFExpression::EMPTY { ty: ty2.clone() }), ty2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), true), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                printUnresolvableTypeError(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info.clone(), true)?;
            }
        } else if is_array1.clone() && is_array2.clone() {
            ty1 = Type::unliftArray(type1.clone())?;
            ty2 = Type::unliftArray(type2.clone())?;
            expl1 = Expression::arrayElements(exp1.clone())?;
            expl2 = Expression::arrayElements(exp2.clone())?;
            if (expl1.clone().borrow().len() as i32) > (expl2.clone().borrow().len() as i32) {
                bail!("fail");
            }
            let __range1 = 1..=(expl1.clone().borrow().len() as i32);
            for mut i in __range1 {
                e1 = expl1.clone().borrow()[(i.clone()-1) as usize].clone();
                e2 = expl2.clone().borrow()[(i.clone()-1) as usize].clone();
                (e1, ty) = checkOverloadedBinaryArrayEW2(e1.clone(), ty1.clone(), var1.clone(), op.clone(), e2.clone(), ty2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
                expl = cons(e1.clone(), expl.clone());
            }
        } else if is_array1.clone() {
            ty1 = Type::unliftArray(type1.clone())?;
            expl1 = Expression::arrayElements(exp1.clone())?;
            let __range2 = expl1.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range2 {
                (e, ty) = checkOverloadedBinaryArrayEW2(e.clone(), ty1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
                expl = cons(e.clone(), expl.clone());
            }
        } else if is_array2.clone() {
            ty2 = Type::unliftArray(type2.clone())?;
            expl2 = Expression::arrayElements(exp2.clone())?;
            let __range3 = expl2.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range3 {
                (e, ty) = checkOverloadedBinaryArrayEW2(exp1.clone(), type1.clone(), var1.clone(), op.clone(), e.clone(), ty2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone())?;
                expl = cons(e.clone(), expl.clone());
            }
        }
        outType = Type::setArrayElementType(type1.clone(), ty.clone());
        outExp = Expression::makeArray(outType.clone(), metamodelica::arrayFromVec(expl.clone().reverse().into_iter().cloned().collect()), false);
    } else {
        (outExp, outType) = matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), op.clone(), exp2.clone(), type2.clone(), var2.clone(), candidates.clone(), context.clone(), info.clone(), true)?;
    }
    Ok((outExp, outType))
}

fn implicitConstructAndMatch(mut candidates: Arc<metamodelica::List<Arc<Function::Function>>>, mut inExp1: Arc<Expression::NFExpression>, mut inType1: Arc<Type::NFType>, mut op: Arc<Operator::NFOperator>, mut inExp2: Arc<Expression::NFExpression>, mut inType2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut in1: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut in2: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut operfn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut matchedfuncs: Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>> = metamodelica::nil();
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg1_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arg2_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut matched: bool = false;
    let mut arg1_info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut arg2_info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    exp1 = inExp1.clone();
    exp2 = inExp2.clone();
    for mut r#fn in &*candidates.clone() {
        let mut r#fn = r#fn.clone();
        if (r#fn.inputs.clone().len() as i32) != 2 {
            continue;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(r#fn.inputs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        in1 = __pa0.clone();
        in2 = __pa1.clone();
        arg1_ty = InstNode::getType(in1.clone())?;
        arg2_ty = InstNode::getType(in2.clone())?;
        arg1_info = InstNode::info(in1.clone())?;
        arg2_info = InstNode::info(in2.clone())?;
        (matchedfuncs, matched) = implicitConstructAndMatch2(inExp1.clone(), inType1.clone(), inExp2.clone(), arg1_ty.clone(), arg1_info.clone(), arg2_ty.clone(), arg2_info.clone(), InstNode::classScope(in2.clone()), r#fn.clone(), false, matchedfuncs.clone())?;
        if matched.clone() {
            continue;
        }
        (matchedfuncs, matched) = implicitConstructAndMatch2(inExp2.clone(), inType2.clone(), inExp1.clone(), arg2_ty.clone(), arg2_info.clone(), arg1_ty.clone(), arg1_info.clone(), InstNode::classScope(in1.clone()), r#fn.clone(), true, matchedfuncs.clone())?;
    }
    if (matchedfuncs.clone().len() as i32) == 1 {
        let (__pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(matchedfuncs.clone()) {
            Deref @ metamodelica::List::Cons { head: (__pa3, Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } }, __pa6), tail: _ } => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        operfn = __pa3.clone();
        exp1 = __pa4.clone();
        exp2 = __pa5.clone();
        var = __pa6.clone();
        outType = Function::returnType(operfn.clone());
        outExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(operfn.clone(), list![exp1.clone(), exp2.clone()], var.clone(), Purity::PURE.clone(), outType.clone()) });
    } else {
        Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }))?).clone(), (Function::candidateFuncListString(({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut r#fn in (matchedfuncs.clone()).into_iter().cloned() {
            let __x = Util::tuple31(r#fn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((outExp, outType))
}

fn implicitConstructAndMatch2(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut paramType1: Arc<Type::NFType>, mut paramInfo1: SourceInfo, mut paramType2: Arc<Type::NFType>, mut paramInfo2: SourceInfo, mut scope: Arc<InstNode::InstNode>, mut r#fn: Arc<Function::Function>, mut reverseArgs: bool, mut matchedFns: Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>>) -> Result<(Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>>, bool)> {
    let mut matchedFns: Arc<metamodelica::List<(Arc<Function::Function>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Variability)>> = matchedFns;
    let mut matched: bool = false;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut var: Variability = Variability::CONSTANT;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    (e1, _, mk) = matchTypes(paramType1.clone(), type1.clone(), exp1.clone(), DEFAULT_OPTIONS.clone())?;
    if mk.clone() == MatchKind::EXACT.clone() {
        (fn_ref, _, _) = Function::instFunction(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("'constructor'")).clone(), subscripts: metamodelica::nil() }), scope.clone(), InstContext::NO_CONTEXT.clone(), paramInfo2.clone())?;
        e2 = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: fn_ref.clone(), arguments: list![exp2.clone()], named_args: metamodelica::nil(), call_scope: scope.clone() }) });
        (e2, ty, var, _) = Call::typeCall(e2.clone(), 0, paramInfo1.clone(), false)?;
        (_, _, mk) = matchTypes(paramType2.clone(), ty.clone(), e2.clone(), DEFAULT_OPTIONS.clone())?;
        if mk.clone() == MatchKind::EXACT.clone() {
            matchedFns = cons((r#fn.clone(), if (reverseArgs.clone()) {list![e2.clone(), e1.clone()]} else {list![e1.clone(), e2.clone()]}, var.clone()), matchedFns.clone());
            matched = true;
        } else {
            matched = false;
        }
    } else {
        matched = false;
    }
    Ok((matchedFns, matched))
}

fn checkBinaryOperationAdd(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut valid: bool = false;
    (e1, e2, resultType, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk.clone());
    valid = (::match_deref::match_deref! { match &(Type::arrayElementType(resultType.clone())) {
        Deref @ Type::INTEGER => valid.clone(),
        Deref @ Type::REAL => valid.clone(),
        Deref @ Type::STRING => valid.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: Operator::makeAdd(resultType.clone()), exp2: e2.clone() });
    if !(valid.clone()) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationSub(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut valid: bool = false;
    (e1, e2, resultType, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk.clone());
    valid = (::match_deref::match_deref! { match &(Type::arrayElementType(resultType.clone())) {
        Deref @ Type::INTEGER => valid.clone(),
        Deref @ Type::REAL => valid.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: Operator::makeSub(resultType.clone()), exp2: e2.clone() });
    if !(valid.clone()) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationMul(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim11: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim12: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim21: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim22: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut op: Op = Op::ADD;
    let mut valid: bool = false;
    ty1 = Type::arrayElementType(type1.clone());
    ty2 = Type::arrayElementType(type2.clone());
    (e1, e2, resultType, mk) = matchExpressions(exp1.clone(), ty1.clone(), exp2.clone(), ty2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk.clone());
    valid = (::match_deref::match_deref! { match &(resultType.clone()) {
        Deref @ Type::INTEGER => valid.clone(),
        Deref @ Type::REAL => valid.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dims1 = Type::arrayDims(type1.clone());
    dims2 = Type::arrayDims(type2.clone());
    (resultType, op) = (::match_deref::match_deref! { match &((dims1.clone(), dims2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => (resultType.clone(), Op::MUL.clone()),
        (Deref @ metamodelica::List::Nil, _) => (Arc::new(Type::NFType::ARRAY { elementType: resultType.clone(), dimensions: dims2.clone() }), Op::MUL_SCALAR_ARRAY.clone()),
        (_, Deref @ metamodelica::List::Nil) => (Arc::new(Type::NFType::ARRAY { elementType: resultType.clone(), dimensions: dims1.clone() }), Op::MUL_ARRAY_SCALAR.clone()),
        (Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: dim21, tail: Deref @ metamodelica::List::Nil }) => {
            valid = Dimension::isEqual(dim11.clone(), dim21.clone())?;
            (resultType.clone(), Op::SCALAR_PRODUCT.clone())
        },
        (Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: dim21, tail: Deref @ metamodelica::List::Cons { head: dim22, tail: Deref @ metamodelica::List::Nil } }) => {
            valid = Dimension::isEqual(dim11.clone(), dim21.clone())?;
            (Arc::new(Type::NFType::ARRAY { elementType: resultType.clone(), dimensions: list![dim22.clone()] }), Op::MUL_VECTOR_MATRIX.clone())
        },
        (Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Cons { head: dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: dim21, tail: Deref @ metamodelica::List::Nil }) => {
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            (Arc::new(Type::NFType::ARRAY { elementType: resultType.clone(), dimensions: list![dim11.clone()] }), Op::MUL_MATRIX_VECTOR.clone())
        },
        (Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Cons { head: dim12, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Cons { head: dim21, tail: Deref @ metamodelica::List::Cons { head: dim22, tail: Deref @ metamodelica::List::Nil } }) => {
            valid = Dimension::isEqual(dim12.clone(), dim21.clone())?;
            (Arc::new(Type::NFType::ARRAY { elementType: resultType.clone(), dimensions: list![dim11.clone(), dim22.clone()] }), Op::MATRIX_PRODUCT.clone())
        },
        _ => {
            valid = false;
            (resultType.clone(), Op::MUL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: Arc::new(Operator::NFOperator { ty: resultType.clone(), op: op.clone() }), exp2: e2.clone() });
    if !(valid.clone()) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationDiv(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo, mut isElementWise: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut valid: bool = false;
    let mut op: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    (e1, ty1, mk) = matchTypes(type1.clone(), Type::setArrayElementType(type1.clone(), Arc::new(crate::NFType::REAL)), exp1.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk.clone());
    (e2, ty2, mk) = matchTypes(type2.clone(), Type::setArrayElementType(type2.clone(), Arc::new(crate::NFType::REAL)), exp2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = valid.clone() && isCompatibleMatch(mk.clone());
    (resultType, op) = (match (Type::isArray(ty1.clone()), Type::isArray(ty2.clone()), isElementWise.clone()) {
        (false, false, _) => (ty1.clone(), Operator::makeDiv(ty1.clone())),
        (_, false, _) => (ty1.clone(), Arc::new(Operator::NFOperator { ty: ty1.clone(), op: Op::DIV_ARRAY_SCALAR.clone() })),
        (false, _, true) => (ty2.clone(), Arc::new(Operator::NFOperator { ty: ty2.clone(), op: Op::DIV_SCALAR_ARRAY.clone() })),
        (true, _, true) => {
            (_, _, mk) = matchArrayTypes(ty1.clone(), ty2.clone(), e1.clone(), ALLOW_UNKNOWN.clone())?;
            valid = valid.clone() && isCompatibleMatch(mk.clone());
            (ty1.clone(), Operator::makeDiv(ty1.clone()))
        },
        _ => {
            valid = false;
            (ty1.clone(), Operator::makeDiv(ty1.clone()))
        },
    });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
    if !(valid.clone()) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationPow(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut valid: bool = false;
    let mut op: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    (e1, resultType, mk) = matchTypes(type1.clone(), Type::setArrayElementType(type1.clone(), Arc::new(crate::NFType::REAL)), exp1.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk.clone());
    if Type::isArray(resultType.clone()) {
        valid = valid.clone() && Type::isSquareMatrix(resultType.clone())?;
        valid = valid.clone() && Type::isInteger(type2.clone());
        valid = valid.clone() && !(Expression::isNegative(exp2.clone()));
        op = Arc::new(Operator::NFOperator { ty: resultType.clone(), op: Op::POW_MATRIX.clone() });
        e2 = exp2.clone();
    } else {
        (e2, _, mk) = matchTypes(type2.clone(), Arc::new(crate::NFType::REAL), exp2.clone(), ALLOW_UNKNOWN.clone())?;
        valid = valid.clone() && isCompatibleMatch(mk.clone());
        op = Arc::new(Operator::NFOperator { ty: resultType.clone(), op: Op::POW.clone() });
    }
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
    if !(valid.clone()) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationPowEW(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut valid: bool = false;
    let mut op: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    (e1, ty1, mk) = matchTypes(type1.clone(), Type::setArrayElementType(type1.clone(), Arc::new(crate::NFType::REAL)), exp1.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk.clone());
    (e2, ty2, mk) = matchTypes(type2.clone(), Type::setArrayElementType(type2.clone(), Arc::new(crate::NFType::REAL)), exp2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = valid.clone() && isCompatibleMatch(mk.clone());
    (resultType, op) = (match (Type::isArray(ty1.clone()), Type::isArray(ty2.clone())) {
        (false, false) => (ty1.clone(), Operator::makePow(ty1.clone())),
        (_, false) => (ty1.clone(), Arc::new(Operator::NFOperator { ty: ty1.clone(), op: Op::POW_ARRAY_SCALAR.clone() })),
        (false, _) => (ty2.clone(), Arc::new(Operator::NFOperator { ty: ty2.clone(), op: Op::POW_SCALAR_ARRAY.clone() })),
        _ => {
            (_, _, mk) = matchArrayTypes(ty1.clone(), ty2.clone(), e1.clone(), ALLOW_UNKNOWN.clone())?;
            valid = valid.clone() && isCompatibleMatch(mk.clone());
            (ty1.clone(), Operator::makePow(ty1.clone()))
        },
    });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
    if !(valid.clone()) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationEW(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut elemOp: Op, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut binaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut valid: bool = false;
    let mut is_arr1: bool = false;
    let mut is_arr2: bool = false;
    let mut op: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    is_arr1 = Type::isArray(type1.clone());
    is_arr2 = Type::isArray(type2.clone());
    if is_arr1.clone() && is_arr2.clone() {
        (e1, e2, resultType, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), ALLOW_UNKNOWN.clone())?;
    } else {
        ty1 = Type::arrayElementType(type1.clone());
        ty2 = Type::arrayElementType(type2.clone());
        (e1, e2, resultType, mk) = matchExpressions(exp1.clone(), ty1.clone(), exp2.clone(), ty2.clone(), ALLOW_UNKNOWN.clone())?;
    }
    valid = isCompatibleMatch(mk.clone());
    valid = (::match_deref::match_deref! { match &((Type::arrayElementType(resultType.clone()), elemOp.clone())) {
        (Deref @ Type::INTEGER, _) => valid.clone(),
        (Deref @ Type::REAL, _) => valid.clone(),
        (Deref @ Type::STRING, Operator::Op::ADD) => valid.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (resultType, op) = (match (is_arr1.clone(), is_arr2.clone()) {
        (true, false) => {
            resultType = Type::copyDims(type1.clone(), resultType.clone());
            op = Operator::makeArrayScalar(resultType.clone(), elemOp.clone())?;
            (resultType.clone(), op.clone())
        },
        (false, true) => {
            resultType = Type::copyDims(type2.clone(), resultType.clone());
            op = Operator::makeScalarArray(resultType.clone(), elemOp.clone())?;
            (resultType.clone(), op.clone())
        },
        (true, true) => (resultType.clone(), Operator::makeEW(Arc::new(Operator::NFOperator { ty: resultType.clone(), op: elemOp.clone() }))),
        _ => (resultType.clone(), Arc::new(Operator::NFOperator { ty: resultType.clone(), op: elemOp.clone() })),
    });
    binaryExp = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
    if !(valid.clone()) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((binaryExp, resultType))
}

pub fn checkUnaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut unaryExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut unaryType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut op: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    if Type::isComplex(Type::arrayElementType(type1.clone())) {
        (unaryExp, unaryType) = checkOverloadedUnaryOperator(exp1.clone(), type1.clone(), var1.clone(), operator.clone(), context.clone(), info.clone())?;
        return Ok((unaryExp.clone(), unaryType.clone()));
    }
    unaryType = type1.clone();
    op = Operator::setType(unaryType.clone(), operator.clone());
    unaryExp = (match operator.op.clone() {
        Operator::Op::ADD => exp1.clone(),
        _ => Arc::new(Expression::NFExpression::UNARY { operator: op.clone(), exp: exp1.clone() }),
    });
    if !(Type::isNumeric(type1.clone())) {
        printUnresolvableTypeError(unaryExp.clone(), list![type1.clone()], info.clone(), true)?;
    }
    Ok((unaryExp, unaryType))
}

pub fn checkOverloadedUnaryOperator(mut inExp1: Arc<Expression::NFExpression>, mut inType1: Arc<Type::NFType>, mut var: Variability, mut inOp: Arc<Operator::NFOperator>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut opstr: ArcStr = arcstr::literal!("");
    let mut candidates: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut matchedFunc: Arc<MatchedFunction::MatchedFunction> = Arc::new(<MatchedFunction::MatchedFunction as ::std::default::Default>::default());
    let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let mut exactMatches: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    opstr = (Operator::symbol(inOp.clone(), (literal!("'")).clone())?).clone();
    candidates = OperatorOverloading::lookupOperatorFunctionsInType((opstr.clone()).clone(), inType1.clone())?;
    args = list![Arc::new(TypedArg { name: None, value: inExp1.clone(), ty: inType1.clone(), var: var.clone(), purity: Purity::PURE.clone() })];
    matchedFunctions = Function::matchFunctionsSilent(candidates.clone(), args.clone(), metamodelica::nil(), context.clone(), info.clone(), false)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if exactMatches.clone().is_empty() {
        printUnresolvableTypeError(Arc::new(Expression::NFExpression::UNARY { operator: inOp.clone(), exp: inExp1.clone() }), list![inType1.clone()], info.clone(), true)?;
        bail!("fail");
    }
    if (exactMatches.clone().len() as i32) == 1 {
        let __pa0 = ::match_deref::match_deref! { match &(exactMatches.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        matchedFunc = __pa0.clone();
        outType = Function::returnType(matchedFunc.func.clone());
        outExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(matchedFunc.func.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut a in (matchedFunc.args.clone()).into_iter().cloned() {
            let __x = a.value.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), var.clone(), Purity::PURE.clone(), outType.clone()) });
    } else {
        Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone(), list![(Expression::toString(Arc::new(Expression::NFExpression::UNARY { operator: inOp.clone(), exp: inExp1.clone() }))?).clone(), (Function::candidateFuncListString(({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut mfn in (matchedFunctions.clone()).into_iter().cloned() {
            let __x = mfn.func.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))).clone()], info.clone())?;
        bail!("fail");
    }
    outExp = Inline::inlineCallExp(outExp.clone(), false)?;
    Ok((outExp, outType))
}

pub fn checkLogicalBinaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    if Type::isComplex(Type::arrayElementType(type1.clone())) || Type::isComplex(Type::arrayElementType(type2.clone())) {
        (outExp, resultType) = checkOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), operator.clone(), exp2.clone(), type2.clone(), var2.clone(), context.clone(), info.clone())?;
        return Ok((outExp.clone(), resultType.clone()));
    }
    (e1, e2, resultType, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), ALLOW_UNKNOWN.clone())?;
    outExp = Arc::new(Expression::NFExpression::LBINARY { exp1: e1.clone(), operator: Operator::setType(resultType.clone(), operator.clone()), exp2: e2.clone() });
    if !(isCompatibleMatch(mk.clone())) || !(Type::isBoolean(Type::arrayElementType(resultType.clone()))) {
        printUnresolvableTypeError(outExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((outExp, resultType))
}

pub fn checkLogicalUnaryOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = type1.clone();
    if Type::isComplex(Type::arrayElementType(type1.clone())) {
        (outExp, resultType) = checkOverloadedUnaryOperator(exp1.clone(), type1.clone(), var1.clone(), operator.clone(), context.clone(), info.clone())?;
        return Ok((outExp.clone(), resultType.clone()));
    }
    outExp = Arc::new(Expression::NFExpression::LUNARY { operator: Operator::setType(type1.clone(), operator.clone()), exp: exp1.clone() });
    if !(Type::isBoolean(Type::arrayElementType(type1.clone()))) {
        printUnresolvableTypeError(outExp.clone(), list![type1.clone()], info.clone(), true)?;
    }
    Ok((outExp, resultType))
}

pub fn checkRelationOperation(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut var1: Variability, mut operator: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut var2: Variability, mut index: i32, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut valid: bool = false;
    let mut o: Op = Op::ADD;
    if Type::isComplex(Type::arrayElementType(type1.clone())) || Type::isComplex(Type::arrayElementType(type2.clone())) {
        (outExp, resultType) = checkOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1.clone(), operator.clone(), exp2.clone(), type2.clone(), var2.clone(), context.clone(), info.clone())?;
        return Ok((outExp.clone(), resultType.clone()));
    }
    (e1, e2, ty, mk) = matchExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), DEFAULT_OPTIONS.clone())?;
    valid = isCompatibleMatch(mk.clone());
    resultType = Arc::new(crate::NFType::BOOLEAN);
    outExp = Arc::new(Expression::NFExpression::RELATION { exp1: e1.clone(), operator: Operator::setType(ty.clone(), operator.clone()), exp2: e2.clone(), index: index.clone() });
    valid = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => valid.clone(),
        Deref @ Type::REAL => {
            o = operator.op.clone();
            if !(InstContext::inFunction(context.clone())) && (o.clone() == Op::EQUAL.clone() || o.clone() == Op::NEQUAL.clone()) {
                Error::addStrictMessage(Error::WARNING_RELATION_ON_REAL.clone(), list![(Expression::toString(outExp.clone())?).clone(), (Operator::symbol(operator.clone(), (literal!("")).clone())?).clone()], info.clone())?;
            }
            valid.clone()
        },
        Deref @ Type::STRING => valid.clone(),
        Deref @ Type::BOOLEAN => valid.clone(),
        Deref @ Type::ENUMERATION { .. } => valid.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(valid.clone()) {
        printUnresolvableTypeError(outExp.clone(), list![type1.clone(), type2.clone()], info.clone(), true)?;
    }
    Ok((outExp, resultType))
}

pub fn printUnresolvableTypeError(mut exp: Arc<Expression::NFExpression>, mut types: Arc<metamodelica::List<Arc<Type::NFType>>>, mut info: SourceInfo, mut printError: bool) -> Result<()> {
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut ty_str: ArcStr = arcstr::literal!("");
    if printError.clone() {
        exp_str = (Expression::toString(exp.clone())?).clone();
        ty_str = (List::toString(types.clone(), (std::sync::Arc::new(Type::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), false, 0)?).clone();
        Error::addSourceMessage(Error::UNRESOLVABLE_TYPE.clone(), list![(exp_str.clone()).clone(), (ty_str.clone()).clone(), (literal!("<NO_COMPONENT>")).clone()], info.clone())?;
    }
    bail!("fail");
    Ok(())
}

pub fn matchExpressions(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    if referenceEq(&type1.clone(),&type2.clone()) {
        compatibleType = type1.clone();
        matchKind = MatchKind::EXACT.clone();
        return Ok((exp1.clone(), exp2.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if metamodelica::valueConstructor((&*type1.clone()))? != metamodelica::valueConstructor((&*type2.clone()))? {
        (exp1, exp2, compatibleType, matchKind) = matchExpressions_cast(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), options.clone())?;
        return Ok((exp1.clone(), exp2.clone(), compatibleType.clone(), matchKind.clone()));
    }
    matchKind = MatchKind::EXACT.clone();
    compatibleType = (::match_deref::match_deref! { match &(type1.clone()) {
        Deref @ Type::INTEGER => type1.clone(),
        Deref @ Type::REAL => type1.clone(),
        Deref @ Type::STRING => type1.clone(),
        Deref @ Type::BOOLEAN => type1.clone(),
        Deref @ Type::CLOCK => type1.clone(),
        Deref @ Type::ENUMERATION { .. } => {
            matchKind = matchEnumerationTypes(type1.clone(), type2.clone())?;
            type1.clone()
        },
        Deref @ Type::ARRAY { .. } => {
            (exp1, exp2, compatibleType, matchKind) = matchArrayExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), options.clone())?;
            compatibleType.clone()
        },
        Deref @ Type::TUPLE { .. } => {
            (exp1, compatibleType, matchKind) = matchTupleTypes(type1.clone(), type2.clone(), exp1.clone(), options.clone())?;
            compatibleType.clone()
        },
        Deref @ Type::UNKNOWN => {
            matchKind = if (getOption(options.clone(), ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            type1.clone()
        },
        Deref @ Type::COMPLEX { .. } => {
            (exp1, compatibleType, matchKind) = matchComplexTypes(type1.clone(), type2.clone(), exp1.clone(), options.clone())?;
            compatibleType.clone()
        },
        Deref @ Type::METABOXED { .. } => {
            (exp1, exp2, compatibleType, matchKind) = matchBoxedExpressions(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), options.clone())?;
            compatibleType.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTypeCheck.matchExpressions")); __mm_s.push_str(&*literal!(" got unknown type.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub fn matchTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    if referenceEq(&actualType.clone(),&expectedType.clone()) {
        compatibleType = actualType.clone();
        matchKind = MatchKind::EXACT.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if metamodelica::valueConstructor((&*actualType.clone()))? != metamodelica::valueConstructor((&*expectedType.clone()))? {
        (expression, compatibleType, matchKind) = matchTypes_cast(actualType.clone(), expectedType.clone(), expression.clone(), options.clone())?;
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    matchKind = MatchKind::EXACT.clone();
    compatibleType = (::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::INTEGER => actualType.clone(),
        Deref @ Type::REAL => actualType.clone(),
        Deref @ Type::STRING => actualType.clone(),
        Deref @ Type::BOOLEAN => actualType.clone(),
        Deref @ Type::CLOCK => actualType.clone(),
        Deref @ Type::ENUMERATION { .. } => {
            if Type::isUnspecifiedEnumeration(expectedType.clone()) {
                matchKind = MatchKind::EXACT.clone();
            } else {
                matchKind = matchEnumerationTypes(actualType.clone(), expectedType.clone())?;
            }
            actualType.clone()
        },
        Deref @ Type::ARRAY { .. } => {
            (expression, compatibleType, matchKind) = matchArrayTypes(actualType.clone(), expectedType.clone(), expression.clone(), options.clone())?;
            compatibleType.clone()
        },
        Deref @ Type::TUPLE { .. } => {
            (expression, compatibleType, matchKind) = matchTupleTypes(actualType.clone(), expectedType.clone(), expression.clone(), options.clone())?;
            compatibleType.clone()
        },
        Deref @ Type::UNKNOWN => {
            matchKind = if (getOption(options.clone(), ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            actualType.clone()
        },
        Deref @ Type::COMPLEX { .. } => {
            (expression, compatibleType, matchKind) = matchComplexTypes(actualType.clone(), expectedType.clone(), expression.clone(), options.clone())?;
            compatibleType.clone()
        },
        Deref @ Type::FUNCTION { .. } => {
            (expression, compatibleType, matchKind) = matchFunctionTypes(actualType.clone(), expectedType.clone(), expression.clone(), options.clone())?;
            compatibleType.clone()
        },
        Deref @ Type::METABOXED { .. } => {
            (expression, compatibleType, matchKind) = matchTypes(var_field!((*actualType).ty, Type::NFType::METABOXED).clone(), Type::unbox(expectedType.clone()), Expression::unbox(expression.clone()), options.clone())?;
            expression = Expression::r#box(expression.clone());
            compatibleType = Type::r#box(compatibleType.clone());
            compatibleType.clone()
        },
        Deref @ Type::CONDITIONAL_ARRAY { .. } => {
            (expression, compatibleType, matchKind) = matchConditionalArrayTypes(actualType.clone(), expectedType.clone(), expression.clone(), options.clone())?;
            compatibleType.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTypeCheck.matchTypes")); __mm_s.push_str(&*literal!(" got unknown type.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub fn matchExpressions_cast(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut before: Arc<Expression::NFExpression> = exp1.clone();
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((type1.clone(), type2.clone())) {
        (Deref @ Type::INTEGER, Deref @ Type::REAL) => {
            exp1 = Expression::typeCast(exp1.clone(), type2.clone())?;
            (type2.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::ENUMERATION { .. }, Deref @ Type::INTEGER) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdEnumerationAsIntegers")).clone())?) => {
            exp1 = Expression::typeCast(exp1.clone(), type2.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of enumeration expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" to Integer: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use Integer(")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(") instead!")); ArcStr::from(__mm_s) }).clone())?;
            (type2.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::INTEGER, Deref @ Type::ENUMERATION { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdIntegersAsEnumeration")).clone())?) => {
            exp1 = Expression::typeCast(exp1.clone(), type2.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Integer expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" to enumeration: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use the actual enumeration instead!")); ArcStr::from(__mm_s) }).clone())?;
            (type2.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::REAL, Deref @ Type::INTEGER) => {
            exp2 = Expression::typeCast(exp2.clone(), type1.clone())?;
            (type1.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::BOOLEAN, Deref @ Type::REAL) if (Flags::isSet(Flags::NF_API.clone())?) => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Boolean expression: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(" to Real.")); ArcStr::from(__mm_s) }).clone())?;
            exp1 = Expression::typeCast(exp1.clone(), type2.clone())?;
            (type2.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::REAL, Deref @ Type::BOOLEAN) if (Flags::isSet(Flags::NF_API.clone())?) => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Boolean expression: ")); __mm_s.push_str(&*Expression::toString(exp2.clone())?); __mm_s.push_str(&*literal!(" to Real.")); ArcStr::from(__mm_s) }).clone())?;
            exp2 = Expression::typeCast(exp2.clone(), type1.clone())?;
            (type1.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::TUPLE { types: Deref @ metamodelica::List::Cons { head: compatibleType, tail: _ }, .. }, _) => {
            let mut compatibleType = (*compatibleType).clone();
            exp1 = Expression::tupleElement(exp1.clone(), compatibleType.clone(), 1)?;
            (exp1, compatibleType, matchKind) = matchTypes(compatibleType.clone(), type2.clone(), exp1.clone(), options.clone())?;
            if isCompatibleMatch(matchKind.clone()) {
                matchKind = MatchKind::CAST.clone();
            }
            (compatibleType.clone(), matchKind.clone())
        },
        (Deref @ Type::UNKNOWN, _) => (type2.clone(), if (getOption(options.clone(), ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (_, Deref @ Type::UNKNOWN) => (type1.clone(), if (getOption(options.clone(), ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (Deref @ Type::METABOXED { .. }, _) => {
            (exp1, exp2, compatibleType, matchKind) = matchExpressions(Expression::unbox(exp1.clone()), var_field!((*type1).ty, Type::NFType::METABOXED).clone(), exp2.clone(), type2.clone(), options.clone())?;
            (compatibleType.clone(), matchKind.clone())
        },
        (_, Deref @ Type::METABOXED { .. }) => {
            (exp1, exp2, compatibleType, matchKind) = matchExpressions(exp1.clone(), type1.clone(), Expression::unbox(exp2.clone()), var_field!((*type2).ty, Type::NFType::METABOXED).clone(), options.clone())?;
            (compatibleType.clone(), matchKind.clone())
        },
        (_, Deref @ Type::POLYMORPHIC { .. }) => {
            exp1 = Expression::r#box(exp1.clone());
            (Type::r#box(type1.clone()), MatchKind::GENERIC.clone())
        },
        (Deref @ Type::POLYMORPHIC { .. }, _) => {
            exp2 = Expression::r#box(exp2.clone());
            (Type::r#box(type2.clone()), MatchKind::GENERIC.clone())
        },
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => {
            (exp1, exp2, compatibleType, matchKind) = matchConditionalArrayExp(exp1.clone(), type1.clone(), exp2.clone(), type2.clone(), options.clone())?;
            (compatibleType.clone(), matchKind.clone())
        },
        (_, Deref @ Type::CONDITIONAL_ARRAY { .. }) => {
            (exp2, exp1, compatibleType, matchKind) = matchConditionalArrayExp(exp2.clone(), type2.clone(), exp1.clone(), type1.clone(), options.clone())?;
            (compatibleType.clone(), matchKind.clone())
        },
        _ => (Arc::new(crate::NFType::UNKNOWN), MatchKind::NOT_COMPATIBLE.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub fn matchComplexTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = actualType.clone();
    let mut matchKind: MatchKind = MatchKind::NOT_COMPATIBLE.clone();
    let mut cls1: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls2: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut ctree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut anode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut enode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comps1: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut comps2: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cty1: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut cty2: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut matched_elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut elem_arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut opt: MatchOptions = options.clone();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    anode = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(expectedType.clone()) {
        Deref @ Type::COMPLEX { cls: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    enode = __pa1.clone();
    if InstNode::isSame(anode.clone(), enode.clone()) {
        matchKind = MatchKind::EXACT.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    cls1 = InstNode::getClass(anode.clone())?;
    cls2 = InstNode::getClass(enode.clone())?;
    if getOption(opt.clone(), IGNORE_DIMENSIONS_IN_RECORDS.clone()) {
        opt = setOption(opt.clone(), IGNORE_DIMENSIONS.clone());
    }
    let () = (::match_deref::match_deref! { match &((cls1.clone(), actualType.clone(), cls2.clone(), expectedType.clone())) {
        (_, Deref @ Type::COMPLEX { complexTy: cty1 @ Deref @ ComplexType::CONNECTOR { .. }, .. }, _, Deref @ Type::COMPLEX { complexTy: cty2 @ Deref @ ComplexType::CONNECTOR { .. }, .. }) => {
            matchKind = matchComponentList(var_field!((**cty1).potentials, ComplexType::NFComplexType::CONNECTOR).clone(), var_field!((**cty2).potentials, ComplexType::NFComplexType::CONNECTOR).clone(), options.clone())?;
            if matchKind.clone() != MatchKind::NOT_COMPATIBLE.clone() {
                matchKind = matchComponentList(var_field!((**cty1).flows, ComplexType::NFComplexType::CONNECTOR).clone(), var_field!((**cty2).flows, ComplexType::NFComplexType::CONNECTOR).clone(), options.clone())?;
                if matchKind.clone() != MatchKind::NOT_COMPATIBLE.clone() {
                    matchKind = matchComponentList(var_field!((**cty1).streams, ComplexType::NFComplexType::CONNECTOR).clone(), var_field!((**cty2).streams, ComplexType::NFComplexType::CONNECTOR).clone(), options.clone())?;
                }
            }
            if matchKind.clone() != MatchKind::NOT_COMPATIBLE.clone() {
                matchKind = MatchKind::PLUG_COMPATIBLE.clone();
            }
            ()
        },
        (Deref @ Class::INSTANCED_CLASS { elements: ctree @ Deref @ ClassTree::FLAT_TREE { components: comps1, .. }, .. }, _, Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: comps2, .. }, .. }, _) => {
            if (comps1.clone().borrow().len() as i32) != (comps2.clone().borrow().len() as i32) {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
            }
            matchKind = MatchKind::PLUG_COMPATIBLE.clone();
            elem_arr = (::match_deref::match_deref! { match &(expression.clone()) {
        Deref @ Expression::RECORD { .. } => metamodelica::arrayFromVec(var_field!((*expression).elements, Expression::NFExpression::RECORD).clone().into_iter().cloned().collect()),
        _ => {
            elem_arr = metamodelica::arrayCreate((comps1.clone().borrow().len() as i32), Arc::new(Expression::NFExpression::INTEGER { value: 0 }));
            dims = Type::arrayDims(Expression::typeOf(expression.clone()));
            let __range0 = (1..=(comps1.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                ty = Component::getType(InstNode::component(comps1.borrow()[(i.clone()-1) as usize].clone())?)?;
                ty = Type::liftArrayRightList(ty.clone(), dims.clone());
                {
                    let __cell1 = Arc::new(Expression::NFExpression::RECORD_ELEMENT { recordExp: expression.clone(), index: i.clone(), fieldName: (InstNode::name(comps1.borrow()[(i.clone()-1) as usize].clone())?).clone(), ty: ty.clone() });
                    unsafe { metamodelica::Dangerous::arrayInitSlot(elem_arr.clone().clone(), i.clone(), __cell1); }
                }
            }
            elem_arr.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (matched_elements, matchKind) = matchComplexComponents(comps1.clone(), comps2.clone(), elem_arr.clone(), ctree.clone(), opt.clone())?;
            if matchKind.clone() == MatchKind::CAST.clone() {
                expression = typeCastRecord(matched_elements.clone(), enode.clone(), expectedType.clone(), expression.clone())?;
            }
            ()
        },
        _ => {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub fn matchComplexComponents(mut actualComponents: metamodelica::Array<Arc<InstNode::InstNode>>, mut expectedComponents: metamodelica::Array<Arc<InstNode::InstNode>>, mut expressions: metamodelica::Array<Arc<Expression::NFExpression>>, mut classTree: Arc<ClassTree::ClassTree>, mut options: MatchOptions) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, MatchKind)> {
    let mut matchedExpressions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut matchKind: MatchKind = MatchKind::PLUG_COMPATIBLE.clone();
    let mut anode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut enode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut acomp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut ecomp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut idx: i32 = 0;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut mk: MatchKind = MatchKind::EXACT;
    if (actualComponents.clone().borrow().len() as i32) != (expectedComponents.clone().borrow().len() as i32) || (actualComponents.clone().borrow().len() as i32) != (expressions.clone().borrow().len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((matchedExpressions.clone(), matchKind.clone()));
    }
    let __range0 = 1..=(actualComponents.clone().borrow().len() as i32);
    for mut i in __range0 {
        enode = expectedComponents.borrow()[(i.clone()-1) as usize].clone();
        ecomp = InstNode::component(enode.clone())?;
        anode = actualComponents.borrow()[(i.clone()-1) as usize].clone();
        if InstNode::name(anode.clone())? == InstNode::name(enode.clone())? {
            idx = i.clone();
        } else {
            if let Ok(__iflet1) = ClassTree::lookupComponentIndex((InstNode::name(enode.clone())?).clone(), classTree.clone()) {
                idx = __iflet1;
            } else {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok((matchedExpressions.clone(), matchKind.clone()));
            }
            anode = actualComponents.borrow()[(idx.clone()-1) as usize].clone();
        }
        if i.clone() != idx.clone() {
            matchKind = MatchKind::CAST.clone();
        }
        acomp = InstNode::component(anode.clone())?;
        e = expressions.borrow()[(idx.clone()-1) as usize].clone();
        (e, _, mk) = matchTypes(Component::getType(acomp.clone())?, Component::getType(ecomp.clone())?, e.clone(), options.clone())?;
        matchedExpressions = cons(e.clone(), matchedExpressions.clone());
        if mk.clone() == MatchKind::CAST.clone() {
            matchKind = mk.clone();
        } else if !(isValidPlugCompatibleMatch(mk.clone())) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            break;
        }
    }
    matchedExpressions = matchedExpressions.clone().reverse();
    Ok((matchedExpressions, matchKind))
}

pub fn typeCastRecord(mut expressions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut node: Arc<InstNode::InstNode>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut iters: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut i: i32 = 0;
    ty = Expression::typeOf(expression.clone());
    if Type::isArray(ty.clone()) {
        dims = Type::arrayDims(ty.clone());
        ranges = metamodelica::nil();
        iters = metamodelica::nil();
        subs = metamodelica::nil();
        i = 1;
        for mut d in &*dims.clone().reverse() {
            let mut d = d.clone();
            if Dimension::isUnknown(d.clone()) {
                ranges = cons(Arc::new(Expression::NFExpression::RANGE { ty: Arc::new(crate::NFType::INTEGER), start: Arc::new(Expression::NFExpression::INTEGER { value: 1 }), step: None, stop: Arc::new(Expression::NFExpression::SIZE { exp: expression.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() })) }) }), ranges.clone());
            } else {
                ranges = cons(Dimension::toRange(d.clone())?, ranges.clone());
            }
            iter = InstNode::newUniqueIterator(InstNode::info(node.clone())?, Arc::new(crate::NFType::INTEGER));
            iters = cons(iter.clone(), iters.clone());
            sub = Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::INTEGER), cref: ComponentRef::makeIterator(iter.clone(), Arc::new(crate::NFType::INTEGER)) }) });
            subs = cons(sub.clone(), subs.clone());
            i = i.clone() + 1;
        }
        expression = Arc::new(Expression::NFExpression::RECORD { path: InstNode::scopePath(node.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, ty: expectedType.clone(), elements: ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (expressions.clone()).into_iter().cloned() {
            let __x = Expression::applySubscripts(subs.clone(), e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) });
        expression = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: Expression::variability(expression.clone())?, purity: Expression::purity(expression.clone())?, exp: expression.clone(), iters: ({
        let mut __acc: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for (i, r) in (&(iters.clone())).into_iter().zip((&(ranges.clone())).into_iter()) {
            let __x = (i.clone(), r.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }) });
    } else {
        expression = Arc::new(Expression::NFExpression::RECORD { path: InstNode::scopePath(node.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, ty: expectedType.clone(), elements: expressions.clone() });
    }
    Ok(expression)
}

pub fn matchComponentList(mut comps1: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut comps2: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut options: MatchOptions) -> Result<MatchKind> {
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut c2: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rest_c2: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = comps2.clone();
    let mut dummy: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    if (comps1.clone().len() as i32) != (comps2.clone().len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
    } else {
        for mut c1 in &*comps1.clone() {
            let mut c1 = c1.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_c2.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            c2 = __pa0.clone();
            rest_c2 = __pa1.clone();
            if InstNode::name(c1.clone())? != InstNode::name(c2.clone())? {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok(matchKind.clone());
            }
            (_, _, matchKind) = matchTypes(InstNode::getType(c1.clone())?, InstNode::getType(c2.clone())?, dummy.clone(), options.clone())?;
            if matchKind.clone() == MatchKind::NOT_COMPATIBLE.clone() {
                return Ok(matchKind.clone());
            }
        }
    }
    matchKind = MatchKind::PLUG_COMPATIBLE.clone();
    Ok(matchKind)
}

pub fn matchFunctionTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = actualType.clone();
    let mut matchKind: MatchKind = MatchKind::EXACT.clone();
    let mut inputs1: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut inputs2: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut outputs1: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut outputs2: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut slots1: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
    let mut slots2: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
    let mut slot1: Arc<Slot::Slot> = Arc::new(<Slot::Slot as ::std::default::Default>::default());
    let mut slot2: Arc<Slot::Slot> = Arc::new(<Slot::Slot as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::FUNCTION { r#fn: Deref @ Function::FUNCTION { slots: __pa0, outputs: __pa1, inputs: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    slots1 = __pa0.clone();
    outputs1 = __pa1.clone();
    inputs1 = __pa2.clone();
    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(expectedType.clone()) {
        Deref @ Type::FUNCTION { r#fn: Deref @ Function::FUNCTION { slots: __pa4, outputs: __pa5, inputs: __pa6, .. }, .. } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    slots2 = __pa4.clone();
    outputs2 = __pa5.clone();
    inputs2 = __pa6.clone();
    if (outputs1.clone().len() as i32) != (outputs2.clone().len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if !(matchFunctionParameters(outputs1.clone(), outputs2.clone(), options.clone())?) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    if !(matchFunctionParameters(inputs1.clone(), inputs2.clone(), options.clone())?) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    for mut i in &*inputs2.clone() {
        let mut i = i.clone();
        let (__pa8, __pa9) = ::match_deref::match_deref! { match &(slots1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
            _ => bail!("pattern mismatch"),
        } };
        slot1 = __pa8.clone();
        slots1 = __pa9.clone();
        let (__pa10, __pa11) = ::match_deref::match_deref! { match &(slots2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
            _ => bail!("pattern mismatch"),
        } };
        slot2 = __pa10.clone();
        slots2 = __pa11.clone();
        if isSome(slot2.default.clone()) && isNone(slot1.default.clone()) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
        }
    }
    for mut slot in &*slots1.clone() {
        let mut slot = slot.clone();
        if isNone(slot.default.clone()) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
        }
    }
    Ok((expression, compatibleType, matchKind))
}

pub fn matchFunctionParameters(mut params1: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut params2: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut options: MatchOptions) -> Result<bool> {
    let mut matching: bool = true;
    let mut pl1: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = params1.clone();
    let mut pl2: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = params2.clone();
    let mut p1: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut dummy: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    let mut mk: MatchKind = MatchKind::EXACT;
    for mut p2 in &*pl2.clone() {
        let mut p2 = p2.clone();
        if pl1.clone().is_empty() {
            matching = false;
            break;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pl1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        p1 = __pa0.clone();
        pl1 = __pa1.clone();
        if InstNode::name(p1.clone())? != InstNode::name(p2.clone())? {
            matching = false;
            break;
        }
        (_, _, mk) = matchTypes(Type::unbox(InstNode::getType(p1.clone())?), Type::unbox(InstNode::getType(p2.clone())?), dummy.clone(), options.clone())?;
        if mk.clone() != MatchKind::EXACT.clone() {
            matching = false;
            break;
        }
    }
    Ok(matching)
}

pub fn matchEnumerationTypes(mut type1: Arc<Type::NFType>, mut type2: Arc<Type::NFType>) -> Result<MatchKind> {
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut lits1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut lits2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(type1.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lits1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(type2.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lits2 = __pa1.clone();
    matchKind = if (List::isEqualOnTrue(lits1.clone(), lits2.clone(), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
    Ok(matchKind)
}

pub fn matchArrayExpressions(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut ety1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ety2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(type1.clone()) {
        Deref @ Type::ARRAY { dimensions: __pa0, elementType: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dims1 = __pa0.clone();
    ety1 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(type2.clone()) {
        Deref @ Type::ARRAY { dimensions: __pa2, elementType: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dims2 = __pa2.clone();
    ety2 = __pa3.clone();
    (exp1, exp2, compatibleType, matchKind) = matchExpressions(exp1.clone(), ety1.clone(), exp2.clone(), ety2.clone(), options.clone())?;
    (compatibleType, matchKind) = matchArrayDims(dims1.clone(), dims2.clone(), compatibleType.clone(), matchKind.clone(), options.clone())?;
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub fn matchArrayTypes(mut arrayType1: Arc<Type::NFType>, mut arrayType2: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut ety1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ety2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(arrayType1.clone()) {
        Deref @ Type::ARRAY { dimensions: __pa0, elementType: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dims1 = __pa0.clone();
    ety1 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(arrayType2.clone()) {
        Deref @ Type::ARRAY { dimensions: __pa2, elementType: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dims2 = __pa2.clone();
    ety2 = __pa3.clone();
    (expression, compatibleType, matchKind) = matchTypes(ety1.clone(), ety2.clone(), expression.clone(), options.clone())?;
    (compatibleType, matchKind) = matchArrayDims(dims1.clone(), dims2.clone(), compatibleType.clone(), matchKind.clone(), options.clone())?;
    Ok((expression, compatibleType, matchKind))
}

pub fn matchArrayDims(mut dims1: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut ty: Arc<Type::NFType>, mut matchKind: MatchKind, mut options: MatchOptions) -> Result<(Arc<Type::NFType>, MatchKind)> {
    let mut ty: Arc<Type::NFType> = ty;
    let mut matchKind: MatchKind = matchKind;
    let mut rest_dims2: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = dims2.clone();
    let mut cdims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut compat: bool = false;
    if !(isCompatibleMatch(matchKind.clone())) {
        return Ok((ty.clone(), matchKind.clone()));
    }
    if (dims1.clone().len() as i32) != (dims2.clone().len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((ty.clone(), matchKind.clone()));
    }
    for mut dim1 in &*dims1.clone() {
        let mut dim1 = dim1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim2 = __pa0.clone();
        rest_dims2 = __pa1.clone();
        (dim1, compat) = matchDimensions(dim1.clone(), dim2.clone())?;
        if !(compat.clone()) && !(getOption(options.clone(), IGNORE_DIMENSIONS.clone())) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            break;
        }
        cdims = cons(dim1.clone(), cdims.clone());
    }
    ty = Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: cdims.clone().reverse() });
    Ok((ty, matchKind))
}

pub fn matchDimensions(mut dim1: Arc<Dimension::NFDimension>, mut dim2: Arc<Dimension::NFDimension>) -> Result<(Arc<Dimension::NFDimension>, bool)> {
    let mut compatibleDim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut compatible: bool = false;
    if Dimension::isEqualKnown(dim1.clone(), dim2.clone())? {
        compatibleDim = dim1.clone();
        compatible = true;
    } else {
        if !(Dimension::isKnown(dim1.clone(), false)) {
            compatibleDim = dim2.clone();
            compatible = true;
        } else if !(Dimension::isKnown(dim2.clone(), false)) {
            compatibleDim = dim1.clone();
            compatible = true;
        } else if Dimension::isResizable(dim1.clone()) && Dimension::isResizable(dim2.clone()) {
            compatibleDim = dim1.clone();
            compatible = true;
        } else {
            compatibleDim = dim1.clone();
            compatible = false;
        }
    }
    Ok((compatibleDim, compatible))
}

pub fn matchTupleTypes(mut tupleType1: Arc<Type::NFType>, mut tupleType2: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = tupleType1.clone();
    let mut matchKind: MatchKind = MatchKind::EXACT.clone();
    let mut tyl1: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut tyl2: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let __pa0 = ::match_deref::match_deref! { match &(tupleType1.clone()) {
        Deref @ Type::TUPLE { types: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tyl1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(tupleType2.clone()) {
        Deref @ Type::TUPLE { types: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tyl2 = __pa1.clone();
    if (tyl1.clone().len() as i32) < (tyl2.clone().len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression.clone(), compatibleType.clone(), matchKind.clone()));
    }
    for mut ty2 in &*tyl2.clone() {
        let mut ty2 = ty2.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tyl1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty1 = __pa2.clone();
        tyl1 = __pa3.clone();
        if Type::isUnknown(ty2.clone()) {
            continue;
        }
        (_, _, matchKind) = matchTypes(ty1.clone(), ty2.clone(), expression.clone(), options.clone())?;
        if matchKind.clone() != MatchKind::EXACT.clone() {
            break;
        }
    }
    Ok((expression, compatibleType, matchKind))
}

pub fn matchBoxedExpressions(mut exp1: Arc<Expression::NFExpression>, mut type1: Arc<Type::NFType>, mut exp2: Arc<Expression::NFExpression>, mut type2: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp1: Arc<Expression::NFExpression> = exp1;
    let mut exp2: Arc<Expression::NFExpression> = exp2;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    e1 = Expression::unbox(exp1.clone());
    e2 = Expression::unbox(exp2.clone());
    (e1, e2, compatibleType, matchKind) = matchExpressions(e1.clone(), Type::unbox(type1.clone()), e2.clone(), Type::unbox(type2.clone()), options.clone())?;
    if isCastMatch(matchKind.clone()) {
        exp1 = Expression::r#box(e1.clone());
        exp2 = Expression::r#box(e2.clone());
    }
    compatibleType = Type::r#box(compatibleType.clone());
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub fn matchConditionalArrayExp(mut condExp: Arc<Expression::NFExpression>, mut condType: Arc<Type::NFType>, mut otherExp: Arc<Expression::NFExpression>, mut otherType: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut condExp: Arc<Expression::NFExpression> = condExp;
    let mut otherExp: Arc<Expression::NFExpression> = otherExp;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut true_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut false_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut comp_ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut comp_ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1_1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2_1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1_2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2_2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut branch: Type::Branch = Type::Branch::NONE;
    let mut mk1: MatchKind = MatchKind::EXACT;
    let mut mk2: MatchKind = MatchKind::EXACT;
    let mut compat1: bool = false;
    let mut compat2: bool = false;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { matchedBranch: __pa0, falseType: __pa1, trueType: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    branch = __pa0.clone();
    false_ty = __pa1.clone();
    true_ty = __pa2.clone();
    if branch.clone() == Type::Branch::NONE.clone() {
        (e1_1, e2_1, comp_ty1, mk1) = matchExpressions(condExp.clone(), true_ty.clone(), otherExp.clone(), otherType.clone(), options.clone())?;
        (e1_2, e2_2, comp_ty2, mk2) = matchExpressions(condExp.clone(), false_ty.clone(), otherExp.clone(), otherType.clone(), options.clone())?;
        compat1 = isCompatibleMatch(mk1.clone());
        compat2 = isCompatibleMatch(mk2.clone());
        (compatibleType, otherExp, matchKind) = (match (isCompatibleMatch(mk1.clone()), isCompatibleMatch(mk2.clone())) {
        (true, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: comp_ty2.clone(), matchedBranch: Type::Branch::NONE.clone() });
            condExp = Expression::typeCast(condExp.clone(), cond_ty.clone())?;
            (comp_ty1.clone(), otherExp.clone(), mk1.clone())
        },
        (true, _) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: comp_ty2.clone(), matchedBranch: Type::Branch::TRUE.clone() });
            condExp = Expression::typeCast(e1_1.clone(), cond_ty.clone())?;
            (comp_ty1.clone(), e2_1.clone(), mk1.clone())
        },
        (_, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: comp_ty2.clone(), matchedBranch: Type::Branch::FALSE.clone() });
            condExp = Expression::typeCast(e1_2.clone(), cond_ty.clone())?;
            (comp_ty2.clone(), e2_2.clone(), mk2.clone())
        },
        _ => (condType.clone(), condExp.clone(), mk1.clone()),
    });
    } else {
        if branch.clone() == Type::Branch::TRUE.clone() {
            (condExp, otherExp, compatibleType, matchKind) = matchExpressions(condExp.clone(), true_ty.clone(), otherExp.clone(), otherType.clone(), options.clone())?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: compatibleType.clone(), falseType: false_ty.clone(), matchedBranch: branch.clone() });
        } else {
            (condExp, otherExp, compatibleType, matchKind) = matchExpressions(condExp.clone(), false_ty.clone(), otherExp.clone(), otherType.clone(), options.clone())?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty.clone(), falseType: compatibleType.clone(), matchedBranch: branch.clone() });
        }
        if isCompatibleMatch(matchKind.clone()) {
            condExp = Expression::typeCast(condExp.clone(), cond_ty.clone())?;
        }
    }
    Ok((condExp, otherExp, compatibleType, matchKind))
}

pub fn matchConditionalArrayTypes(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut actual_true_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut actual_false_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expected_true_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expected_false_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut true_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut false_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut true_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut false_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { falseType: __pa0, trueType: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    actual_false_ty = __pa0.clone();
    actual_true_ty = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(expectedType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { falseType: __pa2, trueType: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expected_false_ty = __pa2.clone();
    expected_true_ty = __pa3.clone();
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { .. } => {
            (true_exp, true_ty, matchKind) = matchTypes(actual_true_ty.clone(), expected_true_ty.clone(), var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), options.clone())?;
            if !(isCompatibleMatch(matchKind.clone())) {
                compatibleType = actualType.clone();
                return Ok((exp.clone(), compatibleType.clone(), matchKind.clone()));
            }
            (false_exp, false_ty, matchKind) = matchTypes(actual_false_ty.clone(), expected_false_ty.clone(), var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), options.clone())?;
            if !(isCompatibleMatch(matchKind.clone())) {
                compatibleType = actualType.clone();
                return Ok((exp.clone(), compatibleType.clone(), matchKind.clone()));
            }
            compatibleType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty.clone(), falseType: false_ty.clone(), matchedBranch: Type::Branch::NONE.clone() });
            exp = Arc::new(Expression::NFExpression::IF { ty: compatibleType.clone(), condition: var_field!((*exp).condition, Expression::NFExpression::IF).clone(), trueBranch: true_exp.clone(), falseBranch: false_exp.clone() });
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((exp, compatibleType, matchKind))
}

pub fn matchConditionalArrayTypes_cast(mut condType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut true_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut false_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut comp_ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut comp_ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut branch: Type::Branch = Type::Branch::NONE;
    let mut mk1: MatchKind = MatchKind::EXACT;
    let mut mk2: MatchKind = MatchKind::EXACT;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { matchedBranch: __pa0, falseType: __pa1, trueType: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    branch = __pa0.clone();
    false_ty = __pa1.clone();
    true_ty = __pa2.clone();
    if branch.clone() == Type::Branch::NONE.clone() {
        (e1, comp_ty1, mk1) = matchTypes(true_ty.clone(), expectedType.clone(), exp.clone(), options.clone())?;
        (e2, comp_ty2, mk2) = matchTypes(false_ty.clone(), expectedType.clone(), exp.clone(), options.clone())?;
        (compatibleType, matchKind) = (match (isCompatibleMatch(mk1.clone()), isCompatibleMatch(mk2.clone())) {
        (true, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: comp_ty2.clone(), matchedBranch: Type::Branch::NONE.clone() });
            exp = Expression::typeCast(exp.clone(), cond_ty.clone())?;
            (comp_ty1.clone(), mk1.clone())
        },
        (true, _) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: comp_ty1.clone(), falseType: false_ty.clone(), matchedBranch: Type::Branch::TRUE.clone() });
            exp = Expression::typeCast(e1.clone(), cond_ty.clone())?;
            (comp_ty1.clone(), mk1.clone())
        },
        (_, true) => {
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty.clone(), falseType: comp_ty2.clone(), matchedBranch: Type::Branch::FALSE.clone() });
            exp = Expression::typeCast(e2.clone(), cond_ty.clone())?;
            (comp_ty2.clone(), mk2.clone())
        },
        _ => (condType.clone(), mk1.clone()),
    });
    } else {
        if branch.clone() == Type::Branch::TRUE.clone() {
            (exp, compatibleType, matchKind) = matchTypes(true_ty.clone(), expectedType.clone(), exp.clone(), options.clone())?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: compatibleType.clone(), falseType: false_ty.clone(), matchedBranch: branch.clone() });
        } else {
            (exp, compatibleType, matchKind) = matchTypes(false_ty.clone(), expectedType.clone(), exp.clone(), options.clone())?;
            cond_ty = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: true_ty.clone(), falseType: compatibleType.clone(), matchedBranch: branch.clone() });
        }
        if isCompatibleMatch(matchKind.clone()) {
            exp = Expression::typeCast(exp.clone(), cond_ty.clone())?;
        }
    }
    Ok((exp, compatibleType, matchKind))
}

pub fn matchTypes_cast(mut actualType: Arc<Type::NFType>, mut expectedType: Arc<Type::NFType>, mut expression: Arc<Expression::NFExpression>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut expression: Arc<Expression::NFExpression> = expression;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut before: Arc<Expression::NFExpression> = expression.clone();
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((actualType.clone(), expectedType.clone())) {
        (Deref @ Type::INTEGER, Deref @ Type::REAL) => {
            expression = Expression::typeCast(expression.clone(), expectedType.clone())?;
            (expectedType.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::ENUMERATION { .. }, Deref @ Type::INTEGER) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdEnumerationAsIntegers")).clone())?) => {
            expression = Expression::typeCast(expression.clone(), expectedType.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing usage of enumeration expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" as Integer: ")); __mm_s.push_str(&*Expression::toString(expression.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use Integer(")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(") instead!")); ArcStr::from(__mm_s) }).clone())?;
            (expectedType.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::INTEGER, Deref @ Type::ENUMERATION { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdIntegersAsEnumeration")).clone())?) => {
            expression = Expression::typeCast(expression.clone(), expectedType.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing usage of Integer expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" as enumeration: ")); __mm_s.push_str(&*Expression::toString(expression.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use the actual enumeration instead!")); ArcStr::from(__mm_s) }).clone())?;
            (expectedType.clone(), MatchKind::CAST.clone())
        },
        (Deref @ Type::TUPLE { types: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, _) => {
            (expression, compatibleType, matchKind) = matchTypes(listHead(var_field!((*actualType).types, Type::NFType::TUPLE).clone())?, expectedType.clone(), expression.clone(), options.clone())?;
            if isCompatibleMatch(matchKind.clone()) {
                expression = (::match_deref::match_deref! { match &(expression.clone()) {
        Deref @ Expression::TUPLE { .. } => listHead(var_field!((*expression).elements, Expression::NFExpression::TUPLE).clone())?,
        _ => Arc::new(Expression::NFExpression::TUPLE_ELEMENT { tupleExp: expression.clone(), index: 1, ty: Type::setArrayElementType(Expression::typeOf(expression.clone()), compatibleType.clone()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                matchKind = MatchKind::CAST.clone();
            }
            (compatibleType.clone(), matchKind.clone())
        },
        (Deref @ Type::UNKNOWN, _) => (expectedType.clone(), if (getOption(options.clone(), ALLOW_UNKNOWN.clone())) {MatchKind::UNKNOWN_ACTUAL.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (_, Deref @ Type::UNKNOWN) => (actualType.clone(), if (getOption(options.clone(), ALLOW_UNKNOWN.clone())) {MatchKind::UNKNOWN_EXPECTED.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (Deref @ Type::METABOXED { .. }, _) => {
            expression = Expression::unbox(expression.clone());
            (expression, compatibleType, matchKind) = matchTypes(var_field!((*actualType).ty, Type::NFType::METABOXED).clone(), expectedType.clone(), expression.clone(), options.clone())?;
            (compatibleType.clone(), if (isCompatibleMatch(matchKind.clone())) {MatchKind::CAST.clone()} else {matchKind.clone()})
        },
        (_, Deref @ Type::METABOXED { .. }) => {
            (expression, compatibleType, matchKind) = matchTypes(actualType.clone(), var_field!((*expectedType).ty, Type::NFType::METABOXED).clone(), expression.clone(), options.clone())?;
            expression = Expression::r#box(expression.clone());
            compatibleType = Type::r#box(compatibleType.clone());
            (compatibleType.clone(), if (isCompatibleMatch(matchKind.clone())) {MatchKind::CAST.clone()} else {matchKind.clone()})
        },
        (_, Deref @ Type::POLYMORPHIC { .. }) => {
            (expression, compatibleType, matchKind) = matchPolymorphic((var_field!((*expectedType).name, Type::NFType::POLYMORPHIC).clone()).clone(), actualType.clone(), expression.clone())?;
            (compatibleType.clone(), matchKind.clone())
        },
        (Deref @ Type::POLYMORPHIC { .. }, _) => (expectedType.clone(), MatchKind::GENERIC.clone()),
        (_, Deref @ Type::ANY) => (expectedType.clone(), MatchKind::EXACT.clone()),
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => {
            (expression, compatibleType, matchKind) = matchConditionalArrayTypes_cast(actualType.clone(), expectedType.clone(), expression.clone(), options.clone())?;
            (compatibleType.clone(), matchKind.clone())
        },
        _ => (Arc::new(crate::NFType::UNKNOWN), MatchKind::NOT_COMPATIBLE.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub fn matchPolymorphic(mut polymorphicName: ArcStr, mut actualType: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &(polymorphicName.clone()) {
        Deref @ "__Any" => (actualType.clone(), MatchKind::GENERIC.clone()),
        Deref @ "__Scalar" => {
            matchKind = if (Type::isScalar(actualType.clone())) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType.clone(), matchKind.clone())
        },
        Deref @ "__Array" => {
            matchKind = if (Type::isArray(actualType.clone())) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType.clone(), matchKind.clone())
        },
        Deref @ "__Connector" => {
            matchKind = if (Type::isScalar(actualType.clone()) && Expression::isConnector(exp.clone())?) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType.clone(), matchKind.clone())
        },
        Deref @ "__ComponentExpression" => {
            matchKind = if (Type::isScalar(actualType.clone()) && Expression::isComponentExpression(exp.clone())?) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType.clone(), matchKind.clone())
        },
        Deref @ "__Block" => {
            matchKind = if (Type::isComplex(actualType.clone())) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType.clone(), matchKind.clone())
        },
        _ => {
            exp = Expression::r#box(exp.clone());
            (Arc::new(Type::NFType::METABOXED { ty: actualType.clone() }), MatchKind::GENERIC.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, compatibleType, matchKind))
}

pub fn getRangeType(mut startExp: Arc<Expression::NFExpression>, mut stepExp: Option<Arc<Expression::NFExpression>>, mut stopExp: Arc<Expression::NFExpression>, mut rangeElemType: Arc<Type::NFType>, mut info: SourceInfo) -> Result<Arc<Type::NFType>> {
    let mut rangeType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &(rangeElemType.clone()) {
        Deref @ Type::INTEGER => getRangeTypeInt(startExp.clone(), stepExp.clone(), stopExp.clone(), info.clone())?,
        Deref @ Type::REAL => getRangeTypeReal(startExp.clone(), stepExp.clone(), stopExp.clone(), info.clone())?,
        Deref @ Type::BOOLEAN => {
            if isSome(stepExp.clone()) {
                Error::addSourceMessageAndFail(Error::RANGE_INVALID_STEP.clone(), list![(Type::toString(rangeElemType.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            getRangeTypeBool(startExp.clone(), stopExp.clone())?
        },
        Deref @ Type::ENUMERATION { .. } => {
            if isSome(stepExp.clone()) {
                Error::addSourceMessageAndFail(Error::RANGE_INVALID_STEP.clone(), list![(Type::toString(rangeElemType.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            getRangeTypeEnum(startExp.clone(), stopExp.clone())?
        },
        _ => {
            Error::addSourceMessage(Error::RANGE_INVALID_TYPE.clone(), list![(Type::toString(rangeElemType.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    rangeType = Arc::new(Type::NFType::ARRAY { elementType: rangeElemType.clone(), dimensions: list![dim.clone()] });
    Ok(rangeType)
}

pub fn getRangeTypeInt(mut startExp: Arc<Expression::NFExpression>, mut stepExp: Option<Arc<Expression::NFExpression>>, mut stopExp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stepExp.clone(), stopExp.clone())) {
        (Deref @ Expression::INTEGER { .. }, None, Deref @ Expression::INTEGER { .. }) => {
            Dimension::fromInteger(std::cmp::max(var_field!((*stopExp).value, Expression::NFExpression::INTEGER).clone() - var_field!((*startExp).value, Expression::NFExpression::INTEGER).clone() + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::INTEGER { .. }, Some(Deref @ Expression::INTEGER { value: step }), Deref @ Expression::INTEGER { .. }) => {
            if step.clone() == 0 {
                Error::addSourceMessageAndFail(Error::RANGE_TOO_SMALL_STEP.clone(), list![ArcStr::from(::std::format!("{}", step.clone()))], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            Dimension::fromInteger(std::cmp::max(intDiv(var_field!((*stopExp).value, Expression::NFExpression::INTEGER).clone() - var_field!((*startExp).value, Expression::NFExpression::INTEGER).clone(), step.clone()) + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::INTEGER { value: 1 }, None, _) => {
            let mut dim_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            dim_exp = SimplifyExp::simplify(stopExp.clone(), false)?;
            Dimension::fromExp(dim_exp.clone(), Expression::variability(dim_exp.clone())?)?
        },
        (_, None, _) if (Expression::isEqual(startExp.clone(), stopExp.clone())?) => {
            Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut step_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut dim_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var: Variability = Variability::CONSTANT;
            let mut pur: Purity = Purity::PURE;
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: stopExp.clone(), operator: Operator::makeSub(Arc::new(crate::NFType::INTEGER)), exp2: startExp.clone() });
            var = Prefixes::variabilityMax(Expression::variability(stopExp.clone())?, Expression::variability(startExp.clone())?);
            pur = Prefixes::purityMin(Expression::purity(stopExp.clone())?, Expression::purity(startExp.clone())?);
            if isSome(stepExp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(stepExp.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                step_exp = __pa0.clone();
                var = Prefixes::variabilityMax(var.clone(), Expression::variability(step_exp.clone())?);
                pur = Prefixes::purityMin(pur.clone(), Expression::purity(step_exp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::DIV_INT().clone(), list![dim_exp.clone(), step_exp.clone()], var.clone(), pur.clone(), NFBuiltinFuncs::DIV_INT().returnType.clone()) });
            }
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::INTEGER)), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
            dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_INT().clone(), list![dim_exp.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 })], var.clone(), pur.clone(), NFBuiltinFuncs::MAX_INT().returnType.clone()) });
            dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
            Dimension::fromExp(dim_exp.clone(), var.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn getRangeTypeReal(mut startExp: Arc<Expression::NFExpression>, mut stepExp: Option<Arc<Expression::NFExpression>>, mut stopExp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stepExp.clone(), stopExp.clone())) {
        (Deref @ Expression::REAL { .. }, None, Deref @ Expression::REAL { .. }) => {
            Dimension::fromInteger(Util::realRangeSize(var_field!((*startExp).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat(1.0_f64), var_field!((*stopExp).value, Expression::NFExpression::REAL).clone()), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::REAL { value: start }, Some(Deref @ Expression::REAL { value: step }), Deref @ Expression::REAL { .. }) => {
            if start.clone() == start.clone() + step.clone() {
                Error::addSourceMessageAndFail(Error::RANGE_TOO_SMALL_STEP.clone(), list![ArcStr::from(::std::format!("{}", step.clone()))], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            Dimension::fromInteger(Util::realRangeSize(var_field!((*startExp).value, Expression::NFExpression::REAL).clone(), step.clone(), var_field!((*stopExp).value, Expression::NFExpression::REAL).clone()), Prefixes::Variability::CONSTANT.clone())
        },
        (_, None, _) if (Expression::isEqual(startExp.clone(), stopExp.clone())?) => {
            Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut dim_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut step_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var: Variability = Variability::CONSTANT;
            let mut pur: Purity = Purity::PURE;
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: stopExp.clone(), operator: Operator::makeSub(Arc::new(crate::NFType::REAL)), exp2: startExp.clone() });
            var = Prefixes::variabilityMax(Expression::variability(stopExp.clone())?, Expression::variability(startExp.clone())?);
            pur = Prefixes::purityMin(Expression::purity(stopExp.clone())?, Expression::purity(startExp.clone())?);
            if isSome(stepExp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(stepExp.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                step_exp = __pa0.clone();
                var = Prefixes::variabilityMax(var.clone(), Expression::variability(step_exp.clone())?);
                pur = Prefixes::purityMin(pur.clone(), Expression::purity(step_exp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeDiv(Arc::new(crate::NFType::REAL)), exp2: step_exp.clone() });
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::REAL)), exp2: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(5e-15_f64) }) });
            }
            dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FLOOR().clone(), list![dim_exp.clone()], var.clone(), pur.clone(), NFBuiltinFuncs::FLOOR().returnType.clone()) });
            dim_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INTEGER_REAL().clone(), list![dim_exp.clone()], var.clone(), pur.clone(), NFBuiltinFuncs::INTEGER_REAL().returnType.clone()) });
            dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::INTEGER)), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
            dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
            Dimension::fromExp(dim_exp.clone(), var.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn getRangeTypeBool(mut startExp: Arc<Expression::NFExpression>, mut stopExp: Arc<Expression::NFExpression>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stopExp.clone())) {
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => {
            let mut sz: i32 = 0;
            sz = if (var_field!((*startExp).value, Expression::NFExpression::BOOLEAN).clone() == var_field!((*stopExp).value, Expression::NFExpression::BOOLEAN).clone()) {1} else if (var_field!((*startExp).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*stopExp).value, Expression::NFExpression::BOOLEAN).clone()) {2} else {0};
            Dimension::fromInteger(sz.clone(), Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut dim_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var: Variability = Variability::CONSTANT;
            if Expression::isEqual(startExp.clone(), stopExp.clone())? {
                dim = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            } else {
                var = Prefixes::variabilityMax(Expression::variability(startExp.clone())?, Expression::variability(stopExp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::IF { ty: Arc::new(crate::NFType::INTEGER), condition: Arc::new(Expression::NFExpression::RELATION { exp1: startExp.clone(), operator: Operator::makeEqual(Arc::new(crate::NFType::BOOLEAN)), exp2: stopExp.clone(), index: -1 }), trueBranch: Arc::new(Expression::NFExpression::INTEGER { value: 1 }), falseBranch: Arc::new(Expression::NFExpression::IF { ty: Arc::new(crate::NFType::INTEGER), condition: Arc::new(Expression::NFExpression::RELATION { exp1: startExp.clone(), operator: Operator::makeLess(Arc::new(crate::NFType::BOOLEAN)), exp2: stopExp.clone(), index: -1 }), trueBranch: Arc::new(Expression::NFExpression::INTEGER { value: 2 }), falseBranch: Arc::new(Expression::NFExpression::INTEGER { value: 0 }) }) });
                dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
                dim = Dimension::fromExp(dim_exp.clone(), var.clone())?;
            }
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn getRangeTypeEnum(mut startExp: Arc<Expression::NFExpression>, mut stopExp: Arc<Expression::NFExpression>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stopExp.clone())) {
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => {
            Dimension::fromInteger(std::cmp::max(var_field!((*stopExp).index, Expression::NFExpression::ENUM_LITERAL).clone() - var_field!((*startExp).index, Expression::NFExpression::ENUM_LITERAL).clone() + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::ENUM_LITERAL { index: 1, .. }, _) => {
            Dimension::fromExp(stopExp.clone(), Expression::variability(stopExp.clone())?)?
        },
        _ => {
            let mut dim_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut var: Variability = Variability::CONSTANT;
            if Expression::isEqual(startExp.clone(), stopExp.clone())? {
                dim = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            } else {
                var = Prefixes::variabilityMax(Expression::variability(startExp.clone())?, Expression::variability(stopExp.clone())?);
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: Expression::enumIndexExp(startExp.clone())?, operator: Operator::makeSub(Arc::new(crate::NFType::INTEGER)), exp2: Expression::enumIndexExp(stopExp.clone())? });
                dim_exp = Arc::new(Expression::NFExpression::BINARY { exp1: dim_exp.clone(), operator: Operator::makeAdd(Arc::new(crate::NFType::INTEGER)), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
                dim_exp = SimplifyExp::simplify(dim_exp.clone(), false)?;
                dim = Dimension::fromExp(dim_exp.clone(), var.clone())?;
            }
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn matchBinding(mut binding: Arc<Binding::NFBinding>, mut componentType: Arc<Type::NFType>, mut name: ArcStr, mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { bindingExp: exp, .. } => {
            let mut ty_match: MatchKind = MatchKind::EXACT;
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut bind_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut comp_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut exp = (*exp).clone();
            (bind_ty, comp_ty) = elaborateBindingType(exp.clone(), component.clone(), var_field!((*binding).bindingType, Binding::NFBinding::TYPED_BINDING).clone(), componentType.clone())?;
            (exp, ty, ty_match) = matchTypes(bind_ty.clone(), comp_ty.clone(), exp.clone(), ALLOW_UNKNOWN.clone())?;
            if !(isValidAssignmentMatch(ty_match.clone())) {
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = Expression::expandSplitIndices(exp.clone())?);
                printBindingTypeError((name.clone()).clone(), binding.clone(), comp_ty.clone(), bind_ty.clone(), component.clone(), context.clone())?;
                if !(InstContext::inInstanceAPI(context.clone())) {
                    bail!("fail");
                }
            } else if isCastMatch(ty_match.clone()) {
                binding = Arc::new(Binding::NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: ty.clone(), variability: var_field!((*binding).variability, Binding::NFBinding::TYPED_BINDING).clone(), purity: var_field!((*binding).purity, Binding::NFBinding::TYPED_BINDING).clone(), eachType: var_field!((*binding).eachType, Binding::NFBinding::TYPED_BINDING).clone(), evalState: var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), isFlattened: var_field!((*binding).isFlattened, Binding::NFBinding::TYPED_BINDING).clone(), source: var_field!((*binding).source, Binding::NFBinding::TYPED_BINDING).clone(), confidence: var_field!((*binding).confidence, Binding::NFBinding::TYPED_BINDING).clone(), info: var_field!((*binding).info, Binding::NFBinding::TYPED_BINDING).clone() });
            }
            ()
        },
        Deref @ Binding::UNBOUND => {
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFTypeCheck.matchBinding")); __mm_s.push_str(&*literal!(" got untyped binding ")); __mm_s.push_str(&*Binding::toString(binding.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub fn elaborateBindingType(mut bindingExp: Arc<Expression::NFExpression>, mut component: Arc<InstNode::InstNode>, mut bindingType: Arc<Type::NFType>, mut componentType: Arc<Type::NFType>) -> Result<(Arc<Type::NFType>, Arc<Type::NFType>)> {
    fn isParent(mut parent: Arc<InstNode::InstNode>, mut node: Arc<InstNode::InstNode>) -> bool {
        let mut res: bool = false;
        let mut n: Arc<InstNode::InstNode> = InstNode::getDerivedNode(node.clone(), true);
        let mut p: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        res = (::match_deref::match_deref! { match &(n.clone()) {
        Deref @ InstNode::COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent: p }, .. } => InstNode::refEqual(parent.clone(), n.clone()) || isParent(parent.clone(), p.clone()),
        Deref @ InstNode::COMPONENT_NODE { .. } => InstNode::refEqual(parent.clone(), n.clone()) || isParent(parent.clone(), var_field!((*n).parent, InstNode::InstNode::COMPONENT_NODE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    let mut bindingType: Arc<Type::NFType> = bindingType;
    let mut componentType: Arc<Type::NFType> = componentType;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(bindingExp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            bindingType = Expression::typeOf(var_field!((*bindingExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone());
            dims = metamodelica::nil();
            for mut s in &*var_field!((*bindingExp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut s = s.clone();
                dims = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            if isParent(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), component.clone()) {
                dims = cons(Type::nthDimension(InstNode::getType(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone())?, var_field!((*s).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone())?, dims.clone());
            }
            dims.clone()
        },
        _ => cons(Arc::new(crate::NFDimension::UNKNOWN), dims.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            dims = dims.clone().reverse();
            componentType = Type::liftArrayLeftList(componentType.clone(), dims.clone());
            ()
        },
        Deref @ Expression::CREF { .. } => {
            bindingType = ComponentRef::getSubscriptedType(ComponentRef::expandSplitSubscripts(var_field!((*bindingExp).cref, Expression::NFExpression::CREF).clone())?, false)?;
            dims = metamodelica::nil();
            for mut s in &*ComponentRef::subscriptsAllFlat(var_field!((*bindingExp).cref, Expression::NFExpression::CREF).clone()) {
                let mut s = s.clone();
                dims = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            if isParent(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), component.clone()) {
                dims = cons(Type::nthDimension(InstNode::getType(var_field!((*s).node, Subscript::NFSubscript::SPLIT_INDEX).clone())?, var_field!((*s).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone())?, dims.clone());
            }
            dims.clone()
        },
        _ => dims.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            dims = dims.clone().reverse();
            componentType = Type::liftArrayLeftList(componentType.clone(), dims.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((bindingType, componentType))
}

pub fn printBindingTypeError(mut name: ArcStr, mut binding: Arc<Binding::NFBinding>, mut componentType: Arc<Type::NFType>, mut bindingType: Arc<Type::NFType>, mut component: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut binding_info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut comp_info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut mk: MatchKind = MatchKind::EXACT;
    binding_info = Binding::getInfo(binding.clone());
    comp_info = InstNode::info(component.clone())?;
    if Type::isScalar(bindingType.clone()) && Type::isArray(componentType.clone()) {
        Error::addMultiSourceMessage(Error::MODIFIER_NON_ARRAY_TYPE_ERROR.clone(), list![(Binding::toString(binding.clone(), (literal!("")).clone())?).clone(), (name.clone()).clone()], list![binding_info.clone(), comp_info.clone()])?;
    } else {
        (_, _, mk) = matchTypes(Type::arrayElementType(bindingType.clone()), Type::arrayElementType(componentType.clone()), Arc::new(Expression::NFExpression::EMPTY { ty: bindingType.clone() }), ALLOW_UNKNOWN.clone())?;
        if !(InstContext::inAnnotation(context.clone())) {
            if isValidAssignmentMatch(mk.clone()) {
                Error::addMultiSourceMessage(Error::VARIABLE_BINDING_DIMS_MISMATCH.clone(), list![(name.clone()).clone(), (Binding::toString(binding.clone(), (literal!("")).clone())?).clone(), (Dimension::toStringList(Type::arrayDims(componentType.clone()), true)?).clone(), (Dimension::toStringList(Type::arrayDims(bindingType.clone()), true)?).clone()], list![binding_info.clone(), comp_info.clone()])?;
            } else {
                Error::addMultiSourceMessage(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone(), list![(name.clone()).clone(), (Binding::toString(binding.clone(), (literal!("")).clone())?).clone(), (Type::toString(componentType.clone())?).clone(), (Type::toString(bindingType.clone())?).clone()], list![binding_info.clone(), comp_info.clone()])?;
            }
        }
    }
    Ok(())
}

pub fn checkDimensionType(mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut info: SourceInfo) -> Result<()> {
    if !(Type::isInteger(ty.clone())) {
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::BOOLEAN, .. } } => (),
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::ENUMERATION { .. }, .. } } => (),
        _ => {
            Error::addSourceMessage(Error::INVALID_DIMENSION_TYPE.clone(), list![(Expression::toString(exp.clone())?).clone(), (Type::toString(ty.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

pub fn checkReductionType(mut ty: Arc<Type::NFType>, mut name: Arc<Absyn::Path>, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<()> {
    let mut err: ArcStr = arcstr::literal!("");
    err = ((::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } => (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        Deref @ Type::COMPLEX { .. } if (checkSumComplexType(ty.clone(), exp.clone(), info.clone())?) => literal!(""),
        _ => literal!("Integer or Real, or operator record"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "product" } => (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        _ => literal!("scalar Integer or Real"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "min" } => (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        Deref @ Type::BOOLEAN => literal!(""),
        Deref @ Type::ENUMERATION { .. } => literal!(""),
        _ => literal!("scalar enumeration, Boolean, Integer, or Real"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "max" } => (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => literal!(""),
        Deref @ Type::REAL => literal!(""),
        Deref @ Type::BOOLEAN => literal!(""),
        Deref @ Type::ENUMERATION { .. } => literal!(""),
        _ => literal!("scalar enumeration, Boolean, Integer, or Real"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    if !(stringEmpty((err.clone()).clone())) {
        Error::addSourceMessageAndFail(Error::INVALID_REDUCTION_TYPE.clone(), list![(Expression::toString(exp.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (AbsynUtil::pathString(name.clone(), (literal!(".")).clone(), true, false)?).clone(), (err.clone()).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok(())
}

pub fn checkSumComplexType(mut ty: Arc<Type::NFType>, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<bool> {
    let mut valid: bool = true;
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls_node = __pa0.clone();
    cls = InstNode::getClass(cls_node.clone())?;
    for mut op in &*list![(literal!("'+'")).clone(), (literal!("'0'")).clone()] {
        let mut op = op.clone();
        if !(Class::hasOperator((op.clone()).clone(), cls.clone())) {
            Error::addSourceMessage(Error::OPERATOR_RECORD_MISSING_OPERATOR.clone(), list![(Type::toString(ty.clone())?).clone(), (Expression::toString(exp.clone())?).clone(), (literal!("sum")).clone(), (op.clone()).clone()], info.clone())?;
            valid = false;
        }
    }
    Ok(valid)
}

pub fn matchIfBranches(mut trueBranch: Arc<Expression::NFExpression>, mut trueType: Arc<Type::NFType>, mut falseBranch: Arc<Expression::NFExpression>, mut falseType: Arc<Type::NFType>, mut options: MatchOptions) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, MatchKind)> {
    let mut trueBranch: Arc<Expression::NFExpression> = trueBranch;
    let mut falseBranch: Arc<Expression::NFExpression> = falseBranch;
    let mut compatibleType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((trueType.clone(), falseType.clone())) {
        (Deref @ Type::ARRAY { .. }, Deref @ Type::ARRAY { .. }) => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch.clone(), var_field!((*trueType).elementType, Type::NFType::ARRAY).clone(), falseBranch.clone(), var_field!((*falseType).elementType, Type::NFType::ARRAY).clone(), options.clone())?;
            if isIncompatibleMatch(matchKind.clone()) {
                return Ok((trueBranch.clone(), falseBranch.clone(), compatibleType.clone(), matchKind.clone()));
            }
            (compatibleType, matchKind) = matchArrayDims(var_field!((*trueType).dimensions, Type::NFType::ARRAY).clone(), var_field!((*falseType).dimensions, Type::NFType::ARRAY).clone(), compatibleType.clone(), matchKind.clone(), options.clone())?;
            if isIncompatibleMatch(matchKind.clone()) && (var_field!((*trueType).dimensions, Type::NFType::ARRAY).clone().len() as i32) == (var_field!((*falseType).dimensions, Type::NFType::ARRAY).clone().len() as i32) {
                compatibleType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: Type::copyElementType(trueType.clone(), compatibleType.clone()), falseType: Type::copyElementType(falseType.clone(), compatibleType.clone()), matchedBranch: Type::Branch::NONE.clone() });
                matchKind = MatchKind::EXACT.clone();
            }
            (compatibleType.clone(), matchKind.clone())
        },
        (_, _) if (Type::isConditionalArray(trueType.clone()) || Type::isConditionalArray(falseType.clone())) => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch.clone(), Type::arrayElementType(trueType.clone()), falseBranch.clone(), Type::arrayElementType(falseType.clone()), options.clone())?;
            if isIncompatibleMatch(matchKind.clone()) {
                return Ok((trueBranch.clone(), falseBranch.clone(), compatibleType.clone(), matchKind.clone()));
            }
            compatibleType = Arc::new(Type::NFType::CONDITIONAL_ARRAY { trueType: Type::copyElementType(trueType.clone(), compatibleType.clone()), falseType: Type::copyElementType(falseType.clone(), compatibleType.clone()), matchedBranch: Type::Branch::NONE.clone() });
            (compatibleType.clone(), matchKind.clone())
        },
        _ => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch.clone(), trueType.clone(), falseBranch.clone(), falseType.clone(), options.clone())?;
            (compatibleType.clone(), matchKind.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((trueBranch, falseBranch, compatibleType, matchKind))
}

