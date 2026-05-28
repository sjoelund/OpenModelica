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
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFDimension as Dimension;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes;
use crate::NFRecord as Record;
use crate::NFSubscript as Subscript;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFType {
    INTEGER,
    REAL,
    STRING,
    BOOLEAN,
    CLOCK,
    ENUMERATION {
        typePath: Arc<Absyn::Path>,
        literals: Arc<metamodelica::List<ArcStr>>,
    },
    __ENUMERATION_ANY_NOT_USED__,
    ARRAY {
        elementType: Arc<NFType>,
        dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>,
    },
    TUPLE {
        types: Arc<metamodelica::List<Arc<NFType>>>,
        names: Option<Arc<metamodelica::List<ArcStr>>>,
    },
    NORETCALL,
    UNKNOWN,
    COMPLEX {
        cls: Arc<InstNode::InstNode>,
        complexTy: Arc<ComplexType::NFComplexType>,
    },
    FUNCTION {
        r#fn: Arc<Function::Function>,
        fnType: FunctionType,
    },
    /// Used for MetaModelica generic types
    METABOXED {
        ty: Arc<NFType>,
    },
    POLYMORPHIC {
        name: ArcStr,
    },
    ANY,
    /// A type that might be one of two types depending on a condition.
    ///     The two types are assumed to be array types with equal number of dimensions.
    CONDITIONAL_ARRAY {
        trueType: Arc<NFType>,
        falseType: Arc<NFType>,
        matchedBranch: Branch,
    },
    /// Used by untyped components to store type information needed during typing.
    UNTYPED {
        typeNode: Arc<InstNode::InstNode>,
        dimensions: metamodelica::Array<Arc<Dimension::NFDimension>>,
    },
}
impl Default for NFType {
    fn default() -> Self { Self::INTEGER }
}
pub use self::NFType::{INTEGER,REAL,STRING,BOOLEAN,CLOCK,ENUMERATION,__ENUMERATION_ANY_NOT_USED__,ARRAY,TUPLE,NORETCALL,UNKNOWN,COMPLEX,FUNCTION,METABOXED,POLYMORPHIC,ANY,CONDITIONAL_ARRAY,UNTYPED};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FunctionType {
    /// Function parameter of function type.
    FUNCTIONAL_PARAMETER = 1,
    /// Function name used to reference a function.
    FUNCTION_REFERENCE = 2,
    /// A variable that contains a function reference.
    FUNCTIONAL_VARIABLE = 3,
}
impl PartialOrd for FunctionType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for FunctionType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Branch {
    NONE = 1,
    TRUE = 2,
    FALSE = 3,
}
impl PartialOrd for Branch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Branch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn liftArrayLeft(mut ty: Arc<NFType>, mut dim: Arc<Dimension::NFDimension>) -> Arc<NFType> {
    let mut ty: Arc<NFType> = ty;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => Arc::new(NFType::ARRAY { elementType: var_field!((*ty).elementType, NFType::ARRAY).clone(), dimensions: cons(dim.clone(), var_field!((*ty).dimensions, NFType::ARRAY).clone()) }),
        Deref @ CONDITIONAL_ARRAY { .. } => Arc::new(NFType::CONDITIONAL_ARRAY { trueType: liftArrayLeft(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), dim.clone()), falseType: liftArrayLeft(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), dim.clone()), matchedBranch: var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() }),
        _ => Arc::new(NFType::ARRAY { elementType: ty.clone(), dimensions: list![dim.clone()] }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn liftArrayLeftList(mut ty: Arc<NFType>, mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Arc<NFType> {
    let mut ty: Arc<NFType> = ty;
    if dims.clone().is_empty() {
        return ty.clone();
    }
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => Arc::new(NFType::ARRAY { elementType: var_field!((*ty).elementType, NFType::ARRAY).clone(), dimensions: listAppend(dims.clone(), var_field!((*ty).dimensions, NFType::ARRAY).clone()) }),
        Deref @ CONDITIONAL_ARRAY { .. } => Arc::new(NFType::CONDITIONAL_ARRAY { trueType: liftArrayLeftList(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), dims.clone()), falseType: liftArrayLeftList(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), dims.clone()), matchedBranch: var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() }),
        _ => Arc::new(NFType::ARRAY { elementType: ty.clone(), dimensions: dims.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn liftArrayRightList(mut ty: Arc<NFType>, mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Arc<NFType> {
    let mut ty: Arc<NFType> = ty;
    if dims.clone().is_empty() {
        return ty.clone();
    }
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => Arc::new(NFType::ARRAY { elementType: var_field!((*ty).elementType, NFType::ARRAY).clone(), dimensions: listAppend(var_field!((*ty).dimensions, NFType::ARRAY).clone(), dims.clone()) }),
        Deref @ CONDITIONAL_ARRAY { .. } => Arc::new(NFType::CONDITIONAL_ARRAY { trueType: liftArrayRightList(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), dims.clone()), falseType: liftArrayRightList(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), dims.clone()), matchedBranch: var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() }),
        _ => Arc::new(NFType::ARRAY { elementType: ty.clone(), dimensions: dims.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn unliftArray(mut ty: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut ty: Arc<NFType> = ty;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: _, tail: dims }, .. } => {
            if (dims.clone().is_empty()) {var_field!((*ty).elementType, NFType::ARRAY).clone()} else {Arc::new(NFType::ARRAY { elementType: var_field!((*ty).elementType, NFType::ARRAY).clone(), dimensions: dims.clone() })}
        },
        Deref @ CONDITIONAL_ARRAY { .. } => {
            let mut tty: Arc<NFType> = Arc::new(NFType::ANY);
            let mut fty: Arc<NFType> = Arc::new(NFType::ANY);
            tty = unliftArray(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone())?;
            fty = unliftArray(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone())?;
            if (isEqual(tty.clone(), fty.clone())) {tty.clone()} else {Arc::new(NFType::CONDITIONAL_ARRAY { trueType: tty.clone(), falseType: fty.clone(), matchedBranch: var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() })}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ty)
}

pub fn unliftArrayN(mut N: i32, mut ty: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut ty: Arc<NFType> = ty;
    if N.clone() == 0 {
        return Ok(ty.clone());
    }
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { dimensions: dims, .. } => {
            let mut dims = (*dims).clone();
            for mut i in 1..=N.clone() {
                dims = listRest(dims.clone())?;
            }
            if (dims.clone().is_empty()) {var_field!((*ty).elementType, NFType::ARRAY).clone()} else {Arc::new(NFType::ARRAY { elementType: var_field!((*ty).elementType, NFType::ARRAY).clone(), dimensions: dims.clone() })}
        },
        Deref @ CONDITIONAL_ARRAY { .. } => {
            let mut tty: Arc<NFType> = Arc::new(NFType::ANY);
            let mut fty: Arc<NFType> = Arc::new(NFType::ANY);
            tty = unliftArrayN(N.clone(), var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone())?;
            fty = unliftArrayN(N.clone(), var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone())?;
            if (isEqual(tty.clone(), fty.clone())) {tty.clone()} else {Arc::new(NFType::CONDITIONAL_ARRAY { trueType: tty.clone(), falseType: fty.clone(), matchedBranch: var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() })}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ty)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isInteger(mut ty: Arc<NFType>) -> bool {
    let mut isInteger: bool = false;
    isInteger = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ METABOXED { .. } => self::isInteger(var_field!((*ty).ty, NFType::METABOXED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isInteger
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isReal(mut ty: Arc<NFType>) -> bool {
    let mut isReal: bool = false;
    isReal = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ REAL { .. } => true,
        Deref @ METABOXED { .. } => self::isReal(var_field!((*ty).ty, NFType::METABOXED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isReal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isBoolean(mut ty: Arc<NFType>) -> bool {
    let mut isBool: bool = false;
    isBool = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ BOOLEAN { .. } => true,
        Deref @ METABOXED { .. } => isBoolean(var_field!((*ty).ty, NFType::METABOXED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBool
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isString(mut ty: Arc<NFType>) -> bool {
    let mut isString: bool = false;
    isString = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ STRING { .. } => true,
        Deref @ METABOXED { .. } => self::isString(var_field!((*ty).ty, NFType::METABOXED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isString
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isClock(mut ty: Arc<NFType>) -> bool {
    let mut isClock: bool = false;
    isClock = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ CLOCK { .. } => true,
        Deref @ METABOXED { .. } => self::isClock(var_field!((*ty).ty, NFType::METABOXED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isClock
}

pub fn isContinuous(mut ty: Arc<NFType>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { complexTy: ct @ Deref @ ComplexType::RECORD { .. }, .. } => {
            List::all(({
        let mut __acc: Arc<metamodelica::List<Arc<NFType>>> = metamodelica::nil();
        for mut field in (var_field!((**ct).fields, ComplexType::NFComplexType::RECORD).clone()).borrow().iter() {
            let __x = lookupRecordFieldType((Record::Field::name(field.clone())?).clone(), ty.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(isContinuous) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFType>) -> Result<bool> + 'static>))
        },
        _ => {
            isReal(elementType(ty.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isScalar(mut ty: Arc<NFType>) -> bool {
    let mut isScalar: bool = false;
    isScalar = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => false,
        Deref @ CONDITIONAL_ARRAY { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isScalar
}

pub fn isArray(mut ty: Arc<NFType>) -> bool {
    let mut isArray: bool = false;
    isArray = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => true,
        Deref @ CONDITIONAL_ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}

pub fn isConditionalArray(mut ty: Arc<NFType>) -> bool {
    let mut isConditionalArray: bool = false;
    isConditionalArray = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ CONDITIONAL_ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConditionalArray
}

pub fn isResizable(mut ty: Arc<NFType>) -> bool {
    let mut b: bool = List::any(arrayDims(ty.clone()), (std::sync::Arc::new(fnptr!(Dimension::isResizable, Arc<Dimension::NFDimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<bool> + 'static>));
    b
}

pub fn sizeKnown(mut ty: Arc<NFType>) -> bool {
    let mut b: bool = !(List::any(arrayDims(ty.clone()), (std::sync::Arc::new(fnptr!(Dimension::isUnknown, Arc<Dimension::NFDimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<bool> + 'static>)));
    b
}

pub fn isAny(mut ty: Arc<NFType>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ANY => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn setConditionalArrayTypes(mut condType: Arc<NFType>, mut trueType: Arc<NFType>, mut falseType: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut outType: Arc<NFType> = Arc::new(NFType::ANY);
    let mut matched_branch: Branch = Branch::NONE;
    let __pa0 = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ CONDITIONAL_ARRAY { matchedBranch: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    matched_branch = __pa0.clone();
    outType = Arc::new(NFType::CONDITIONAL_ARRAY { trueType: trueType.clone(), falseType: falseType.clone(), matchedBranch: matched_branch.clone() });
    Ok(outType)
}

pub fn removeSizeOneArraysAndRecords(mut ty: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut ty: Arc<NFType> = ty;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut dim in (var_field!((*ty).dimensions, NFType::ARRAY).clone()).into_iter().cloned() {
            if !(!(Dimension::isOne(dim.clone())?)) { continue; }
            let __x = dim.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            if (var_field!((*ty).dimensions, NFType::ARRAY).clone().is_empty()) {removeSizeOneArraysAndRecords(var_field!((*ty).elementType, NFType::ARRAY).clone())?} else {ty.clone()}
        },
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { fields, .. }, .. } if ((fields.clone().borrow().len() as i32) == 1) => {
            removeSizeOneArraysAndRecords(lookupRecordFieldType((Record::Field::name(fields.borrow()[(1-1) as usize].clone())?).clone(), ty.clone())?)?
        },
        _ => {
            ty.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn isMatchedBranch(mut condition: bool, mut condType: Arc<NFType>) -> Result<bool> {
    let mut isMatched: bool = true;
    let mut matched_branch: Branch = Branch::NONE;
    let __pa0 = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ CONDITIONAL_ARRAY { matchedBranch: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    matched_branch = __pa0.clone();
    if condition.clone() && matched_branch.clone() == Branch::FALSE.clone() || !(condition.clone()) && matched_branch.clone() == Branch::TRUE.clone() {
        isMatched = false;
    }
    Ok(isMatched)
}

pub fn simplifyConditionalArray(mut ty: Arc<NFType>) -> Arc<NFType> {
    let mut outType: Arc<NFType> = Arc::new(NFType::ANY);
    outType = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ CONDITIONAL_ARRAY { .. } => (match var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() {
        Branch::TRUE => var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(),
        Branch::FALSE => var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(),
        _ => ty.clone(),
    }),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isVector(mut ty: Arc<NFType>) -> bool {
    let mut isVector: bool = false;
    isVector = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. } => true,
        Deref @ CONDITIONAL_ARRAY { .. } => self::isVector(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isVector
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isMatrix(mut ty: Arc<NFType>) -> bool {
    let mut isMatrix: bool = false;
    isMatrix = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. } => true,
        Deref @ CONDITIONAL_ARRAY { .. } => self::isMatrix(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isMatrix
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isSquareMatrix(mut ty: Arc<NFType>) -> Result<bool> {
    let mut isSquareMatrix: bool = false;
    isSquareMatrix = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: d1, tail: Deref @ metamodelica::List::Cons { head: d2, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            Dimension::isEqualKnown(d1.clone(), d2.clone())?
        },
        Deref @ CONDITIONAL_ARRAY { .. } => {
            self::isSquareMatrix(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isSquareMatrix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isEmptyArray(mut ty: Arc<NFType>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => List::any(var_field!((*ty).dimensions, NFType::ARRAY).clone(), (std::sync::Arc::new(Dimension::isZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<bool> + 'static>)),
        Deref @ CONDITIONAL_ARRAY { .. } => isEmptyArray(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn isSingleElementArray(mut ty: Arc<NFType>) -> Result<bool> {
    let mut isSingleElement: bool = false;
    isSingleElement = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil }, .. } => {
            Dimension::isKnown(d.clone(), false) && Dimension::size(d.clone(), false)? == 1
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isSingleElement)
}

pub fn isEnumeration(mut ty: Arc<NFType>) -> bool {
    let mut isEnum: bool = false;
    isEnum = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEnum
}

pub fn isBuiltinEnumeration(mut ty: Arc<NFType>) -> bool {
    let mut isBuiltin: bool = false;
    let mut name: ArcStr = arcstr::literal!("");
    isBuiltin = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { typePath: Deref @ Absyn::Path::IDENT { name }, .. } => (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "StateSelect" => true,
        Deref @ "AssertionLevel" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBuiltin
}

pub fn isUnspecifiedEnumeration(mut ty: Arc<NFType>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { literals: Deref @ metamodelica::List::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isComplex(mut ty: Arc<NFType>) -> bool {
    let mut isComplex: bool = false;
    isComplex = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isComplex
}

pub fn isComplexArray(mut ty: Arc<NFType>) -> bool {
    let mut isComplex: bool = false;
    isComplex = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => self::isComplex(var_field!((*ty).elementType, NFType::ARRAY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isComplex
}

pub fn complexNode(mut ty: Arc<NFType>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    node = __pa0.clone();
    Ok(node)
}

pub fn complexComponents(mut ty: Arc<NFType>) -> Result<metamodelica::Array<Arc<InstNode::InstNode>>> {
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    comps = ClassTree::getComponents(Class::classTree(InstNode::getClass(complexNode(ty.clone())?)?)?)?;
    Ok(comps)
}

pub fn isConnector(mut ty: Arc<NFType>) -> bool {
    let mut isConnector: bool = false;
    isConnector = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnector
}

pub fn isStreamConnector(mut ty: Arc<NFType>) -> bool {
    let mut isStreamConnector: bool = false;
    isStreamConnector = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::CONNECTOR { streams: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isStreamConnector
}

pub fn isExpandableConnector(mut ty: Arc<NFType>) -> bool {
    let mut isExpandable: bool = false;
    isExpandable = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::EXPANDABLE_CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExpandable
}

pub fn isExternalObject(mut ty: Arc<NFType>) -> bool {
    let mut isEO: bool = false;
    isEO = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEO
}

pub fn isRecord(mut ty: Arc<NFType>) -> bool {
    let mut isRecord: bool = false;
    isRecord = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecord
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isBasic(mut ty: Arc<NFType>) -> bool {
    let mut isNumeric: bool = false;
    isNumeric = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ REAL { .. } => true,
        Deref @ INTEGER { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ STRING { .. } => true,
        Deref @ ENUMERATION { .. } => true,
        Deref @ CLOCK { .. } => true,
        Deref @ FUNCTION { .. } => isBasic(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNumeric
}

pub fn isBasicNumeric(mut ty: Arc<NFType>) -> bool {
    let mut isNumeric: bool = false;
    isNumeric = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ REAL { .. } => true,
        Deref @ INTEGER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNumeric
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isNumeric(mut ty: Arc<NFType>) -> bool {
    let mut isNumeric: bool = false;
    isNumeric = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => isBasicNumeric(var_field!((*ty).elementType, NFType::ARRAY).clone()),
        Deref @ CONDITIONAL_ARRAY { .. } => self::isNumeric(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        _ => isBasicNumeric(ty.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNumeric
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isScalarBuiltin(mut ty: Arc<NFType>) -> bool {
    let mut isScalarBuiltin: bool = false;
    isScalarBuiltin = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ REAL { .. } => true,
        Deref @ STRING { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ CLOCK { .. } => true,
        Deref @ ENUMERATION { .. } => true,
        Deref @ FUNCTION { .. } => self::isScalarBuiltin(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isScalarBuiltin
}

pub fn isTuple(mut ty: Arc<NFType>) -> bool {
    let mut isTuple: bool = false;
    isTuple = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ TUPLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTuple
}

pub fn isUnknown(mut ty: Arc<NFType>) -> bool {
    let mut isUnknown: bool = false;
    isUnknown = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ UNKNOWN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isUnknown
}

pub fn isKnown(mut ty: Arc<NFType>) -> bool {
    let mut isKnown: bool = false;
    isKnown = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ UNKNOWN { .. } => false,
        Deref @ UNTYPED { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isKnown
}

pub fn isPolymorphic(mut ty: Arc<NFType>) -> bool {
    let mut isPolymorphic: bool = false;
    isPolymorphic = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ POLYMORPHIC { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isPolymorphic
}

pub fn isPolymorphicNamed(mut ty: Arc<NFType>, mut name: ArcStr) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ POLYMORPHIC { .. } => name.clone() == var_field!((*ty).name, NFType::POLYMORPHIC).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn firstTupleType(mut ty: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut outTy: Arc<NFType> = Arc::new(NFType::ANY);
    outTy = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ TUPLE { .. } => listHead(var_field!((*ty).types, NFType::TUPLE).clone())?,
        Deref @ ARRAY { .. } => Arc::new(NFType::ARRAY { elementType: firstTupleType(var_field!((*ty).elementType, NFType::ARRAY).clone())?, dimensions: var_field!((*ty).dimensions, NFType::ARRAY).clone() }),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTy)
}

pub fn nthTupleType(mut ty: Arc<NFType>, mut n: i32) -> Arc<NFType> {
    let mut outTy: Arc<NFType> = Arc::new(NFType::ANY);
    outTy = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ TUPLE { .. } => (var_field!((*ty).types, NFType::TUPLE).clone()).get(n.clone()).unwrap(),
        Deref @ ARRAY { .. } => Arc::new(NFType::ARRAY { elementType: nthTupleType(var_field!((*ty).elementType, NFType::ARRAY).clone(), n.clone()), dimensions: var_field!((*ty).dimensions, NFType::ARRAY).clone() }),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTy
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn arrayElementType(mut ty: Arc<NFType>) -> Arc<NFType> {
    let mut elementTy: Arc<NFType> = Arc::new(NFType::ANY);
    elementTy = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => var_field!((*ty).elementType, NFType::ARRAY).clone(),
        Deref @ CONDITIONAL_ARRAY { .. } => arrayElementType(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        Deref @ UNTYPED { .. } if (!(var_field!((*ty).dimensions, NFType::UNTYPED).clone().borrow().is_empty())) => Arc::new(NFType::UNTYPED { typeNode: var_field!((*ty).typeNode, NFType::UNTYPED).clone(), dimensions: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()) }),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elementTy
}

pub fn setArrayElementType(mut arrayTy: Arc<NFType>, mut elementTy: Arc<NFType>) -> Arc<NFType> {
    let mut ty: Arc<NFType> = Arc::new(NFType::ANY);
    ty = (::match_deref::match_deref! { match &(arrayTy.clone()) {
        Deref @ ARRAY { .. } => liftArrayLeftList(elementTy.clone(), var_field!((*arrayTy).dimensions, NFType::ARRAY).clone()),
        Deref @ CONDITIONAL_ARRAY { .. } => Arc::new(NFType::CONDITIONAL_ARRAY { trueType: setArrayElementType(var_field!((*arrayTy).trueType, NFType::CONDITIONAL_ARRAY).clone(), elementTy.clone()), falseType: setArrayElementType(var_field!((*arrayTy).falseType, NFType::CONDITIONAL_ARRAY).clone(), elementTy.clone()), matchedBranch: var_field!((*arrayTy).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() }),
        _ => elementTy.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn elementType(mut ty: Arc<NFType>) -> Arc<NFType> {
    let mut elementTy: Arc<NFType> = Arc::new(NFType::ANY);
    elementTy = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => var_field!((*ty).elementType, NFType::ARRAY).clone(),
        Deref @ CONDITIONAL_ARRAY { .. } => elementType(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        Deref @ FUNCTION { .. } => elementType(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone())),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elementTy
}

pub fn copyElementType(mut dstType: Arc<NFType>, mut srcType: Arc<NFType>) -> Arc<NFType> {
    let mut ty: Arc<NFType> = Arc::new(NFType::ANY);
    ty = setArrayElementType(dstType.clone(), arrayElementType(srcType.clone()));
    ty
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn arrayDims(mut ty: Arc<NFType>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    dims = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => var_field!((*ty).dimensions, NFType::ARRAY).clone(),
        Deref @ FUNCTION { .. } => arrayDims(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone())),
        Deref @ METABOXED { .. } => arrayDims(var_field!((*ty).ty, NFType::METABOXED).clone()),
        Deref @ CONDITIONAL_ARRAY { .. } => List::fill(Arc::new(crate::NFDimension::UNKNOWN), dimensionCount(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone())),
        Deref @ UNTYPED { .. } => Arc::new(var_field!((*ty).dimensions, NFType::UNTYPED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dims
}

pub fn copyDims(mut srcType: Arc<NFType>, mut dstType: Arc<NFType>) -> Arc<NFType> {
    let mut ty: Arc<NFType> = Arc::new(NFType::ANY);
    if arrayDims(srcType.clone()).is_empty() {
        ty = arrayElementType(dstType.clone());
    } else {
        ty = (::match_deref::match_deref! { match &(dstType.clone()) {
        Deref @ ARRAY { .. } => Arc::new(NFType::ARRAY { elementType: var_field!((*dstType).elementType, NFType::ARRAY).clone(), dimensions: arrayDims(srcType.clone()) }),
        _ => Arc::new(NFType::ARRAY { elementType: dstType.clone(), dimensions: arrayDims(srcType.clone()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    ty
}

pub fn applyToDims(mut ty: Arc<NFType>, mut func: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static>) -> Result<Arc<NFType>> {
    pub type dimFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static>;

    let mut ty: Arc<NFType> = ty;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (var_field!((*ty).dimensions, NFType::ARRAY).clone()).into_iter().cloned() {
            let __x = func(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ty.clone()
        },
        Deref @ FUNCTION { r#fn, .. } => {
            let mut r#fn = (*r#fn).clone();
            assign_field!(r#fn.returnType = applyToDims(r#fn.returnType.clone(), func.clone())?);
            assign_variant_field!(ty => NFType::FUNCTION; r#fn = r#fn.clone());
            ty.clone()
        },
        Deref @ METABOXED { .. } => {
            assign_variant_field!(ty => NFType::METABOXED; ty = applyToDims(var_field!((*ty).ty, NFType::METABOXED).clone(), func.clone())?);
            ty.clone()
        },
        Deref @ CONDITIONAL_ARRAY { .. } => {
            assign_variant_field!(ty => NFType::CONDITIONAL_ARRAY; trueType = applyToDims(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), func.clone())?);
            ty.clone()
        },
        Deref @ UNTYPED { .. } => {
            let __range0 = 1..=(var_field!((*ty).dimensions, NFType::UNTYPED).clone().borrow().len() as i32);
            for mut i in __range0 {
                {let _arr = var_field!((*ty).dimensions, NFType::UNTYPED).clone(); let _val = func(var_field!((*ty).dimensions, NFType::UNTYPED).borrow()[(i.clone()-1) as usize].clone())?; _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
            }
            ty.clone()
        },
        _ => {
            ty.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn nthDimension(mut ty: Arc<NFType>, mut index: i32) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dim = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => (var_field!((*ty).dimensions, NFType::ARRAY).clone()).get(index.clone())?,
        Deref @ FUNCTION { .. } => nthDimension(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone()), index.clone())?,
        Deref @ METABOXED { .. } => nthDimension(var_field!((*ty).ty, NFType::METABOXED).clone(), index.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(dim)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dimensionCount(mut ty: Arc<NFType>) -> i32 {
    let mut dimCount: i32 = 0;
    dimCount = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => (var_field!((*ty).dimensions, NFType::ARRAY).clone().len() as i32),
        Deref @ CONDITIONAL_ARRAY { .. } => dimensionCount(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        Deref @ FUNCTION { .. } => dimensionCount(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone())),
        Deref @ METABOXED { .. } => dimensionCount(var_field!((*ty).ty, NFType::METABOXED).clone()),
        Deref @ UNTYPED { .. } => (var_field!((*ty).dimensions, NFType::UNTYPED).clone().borrow().len() as i32),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dimCount
}

pub fn dimensionDiff(mut ty1: Arc<NFType>, mut ty2: Arc<NFType>) -> i32 {
    let mut diff: i32 = dimensionCount(ty1.clone()) - dimensionCount(ty2.clone());
    diff
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn hasKnownSize(mut ty: Arc<NFType>) -> bool {
    let mut isKnown: bool = false;
    isKnown = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => List::all(var_field!((*ty).dimensions, NFType::ARRAY).clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| Ok(Dimension::isKnown(__pe_a0, __pe_b1.clone())) })),
        Deref @ CONDITIONAL_ARRAY { .. } => false,
        Deref @ FUNCTION { .. } => hasKnownSize(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone())),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isKnown
}

pub fn hasZeroDimension(mut ty: Arc<NFType>) -> bool {
    let mut hasZero: bool = false;
    hasZero = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => List::any(var_field!((*ty).dimensions, NFType::ARRAY).clone(), (std::sync::Arc::new(Dimension::isZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<bool> + 'static>)),
        Deref @ CONDITIONAL_ARRAY { .. } => hasZeroDimension(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()) && hasZeroDimension(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasZero
}

pub fn mapDims(mut ty: Arc<NFType>, mut func: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static>) -> Arc<NFType> {
    pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static>;

    let mut ty: Arc<NFType> = ty;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (var_field!((*ty).dimensions, NFType::ARRAY).clone()).into_iter().cloned() {
            let __x = func(d.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        Deref @ TUPLE { .. } => {
            assign_variant_field!(ty => NFType::TUPLE; types = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFType>>> = metamodelica::nil();
        for mut t in (var_field!((*ty).types, NFType::TUPLE).clone()).into_iter().cloned() {
            let __x = mapDims(t.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        Deref @ FUNCTION { r#fn, .. } => {
            assign_variant_field!(ty => NFType::FUNCTION; r#fn = Function::setReturnType(mapDims(Function::returnType(r#fn.clone()), func.clone()), r#fn.clone()));
            ()
        },
        Deref @ METABOXED { .. } => {
            assign_variant_field!(ty => NFType::METABOXED; ty = mapDims(var_field!((*ty).ty, NFType::METABOXED).clone(), func.clone()));
            ()
        },
        Deref @ CONDITIONAL_ARRAY { .. } => {
            assign_variant_field!(ty => NFType::CONDITIONAL_ARRAY;
                trueType = mapDims(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), func.clone()),
                falseType = mapDims(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), func.clone())
            );
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn foldDims<ArgT: Clone + 'static>(mut ty: Arc<NFType>, mut func: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> ArgT {
    pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    arg = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => List::fold(var_field!((*ty).dimensions, NFType::ARRAY).clone(), func.clone(), arg.clone()),
        Deref @ TUPLE { .. } => List::fold(var_field!((*ty).types, NFType::TUPLE).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Ok(foldDims(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone()),
        Deref @ FUNCTION { .. } => foldDims(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone()), func.clone(), arg.clone()),
        Deref @ METABOXED { .. } => foldDims(var_field!((*ty).ty, NFType::METABOXED).clone(), func.clone(), arg.clone()),
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    arg
}

pub fn nthEnumLiteral(mut ty: Arc<NFType>, mut index: i32) -> Result<ArcStr> {
    let mut literal: ArcStr = arcstr::literal!("");
    let mut literals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    literals = __pa0.clone();
    literal = ((literals.clone()).get(index.clone())?).clone();
    Ok(literal)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toString(mut ty: Arc<NFType>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER => literal!("Integer"),
        Deref @ REAL => literal!("Real"),
        Deref @ STRING => literal!("String"),
        Deref @ BOOLEAN => literal!("Boolean"),
        Deref @ CLOCK => literal!("Clock"),
        Deref @ ENUMERATION { .. } => if (var_field!((*ty).literals, NFType::ENUMERATION).clone().is_empty()) {literal!("enumeration(:)")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("enumeration ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*ty).typePath, NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(var_field!((*ty).literals, NFType::ENUMERATION).clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }},
        Deref @ ARRAY { .. } => List::toString(var_field!((*ty).dimensions, NFType::ARRAY).clone(), (std::sync::Arc::new(Dimension::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<ArcStr> + 'static>), (toString(var_field!((*ty).elementType, NFType::ARRAY).clone())?).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?,
        Deref @ TUPLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(List::map(var_field!((*ty).types, NFType::TUPLE).clone(), (std::sync::Arc::new(toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFType>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ NORETCALL => literal!("()"),
        Deref @ UNKNOWN => literal!("unknown()"),
        Deref @ COMPLEX { .. } => AbsynUtil::pathString(InstNode::scopePath(var_field!((*ty).cls, NFType::COMPLEX).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?,
        Deref @ FUNCTION { .. } => Function::typeString(var_field!((*ty).r#fn, NFType::FUNCTION).clone())?,
        Deref @ METABOXED { .. } => toString(var_field!((*ty).ty, NFType::METABOXED).clone())?,
        Deref @ POLYMORPHIC { .. } => if (StringUtil::startsWith((var_field!((*ty).name, NFType::POLYMORPHIC).clone()).clone(), (literal!("__")).clone())) {substring((var_field!((*ty).name, NFType::POLYMORPHIC).clone()).clone(), 3, ((var_field!((*ty).name, NFType::POLYMORPHIC).clone()).clone().len() as i32))?} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*var_field!((*ty).name, NFType::POLYMORPHIC).clone()); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) }},
        Deref @ ANY => literal!("$ANY$"),
        Deref @ CONDITIONAL_ARRAY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*toString(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone())?); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*toString(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone())?); ArcStr::from(__mm_s) },
        Deref @ UNTYPED { .. } => Array::toString(var_field!((*ty).dimensions, NFType::UNTYPED).clone(), (std::sync::Arc::new(Dimension::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<ArcStr> + 'static>), (InstNode::name(var_field!((*ty).typeNode, NFType::UNTYPED).clone())?).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFType.toString")); __mm_s.push_str(&*literal!(" got unknown type: ")); __mm_s.push_str(&*anyString(ty.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toFlatString(mut ty: Arc<NFType>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER => literal!("Integer"),
        Deref @ REAL => literal!("Real"),
        Deref @ STRING => literal!("String"),
        Deref @ BOOLEAN => literal!("Boolean"),
        Deref @ CLOCK => literal!("Clock"),
        Deref @ ENUMERATION { .. } => if (var_field!((*ty).literals, NFType::ENUMERATION).clone().is_empty()) {literal!("enumeration(:)")} else if (isBuiltinEnumeration(ty.clone())) {AbsynUtil::pathString(var_field!((*ty).typePath, NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?} else {Util::makeQuotedIdentifier((AbsynUtil::pathString(var_field!((*ty).typePath, NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?).clone())?},
        Deref @ ARRAY { .. } => Dimension::toFlatStringList(var_field!((*ty).dimensions, NFType::ARRAY).clone(), format.clone(), (toFlatString(var_field!((*ty).elementType, NFType::ARRAY).clone(), format.clone())?).clone())?,
        Deref @ TUPLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(List::map(var_field!((*ty).types, NFType::TUPLE).clone(), Arc::new({ let __pe_b1 = format.clone(); move |__pe_a0| toFlatString(__pe_a0, __pe_b1.clone()) })), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ NORETCALL => literal!("()"),
        Deref @ UNKNOWN => literal!("unknown()"),
        Deref @ COMPLEX { .. } => Util::makeQuotedIdentifier((AbsynUtil::pathString(InstNode::scopePath(var_field!((*ty).cls, NFType::COMPLEX).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone())?,
        Deref @ FUNCTION { .. } => Util::makeQuotedIdentifier((AbsynUtil::pathString(InstNode::scopePath(var_field!((*ty).r#fn, NFType::FUNCTION).node.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone())?,
        Deref @ METABOXED { .. } => toFlatString(var_field!((*ty).ty, NFType::METABOXED).clone(), format.clone())?,
        Deref @ POLYMORPHIC { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*var_field!((*ty).name, NFType::POLYMORPHIC).clone()); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) },
        Deref @ ANY => literal!("$ANY$"),
        Deref @ CONDITIONAL_ARRAY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*toFlatString(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), format.clone())?); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*toFlatString(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), format.clone())?); ArcStr::from(__mm_s) },
        Deref @ UNTYPED { .. } => Array::toString(var_field!((*ty).dimensions, NFType::UNTYPED).clone(), Arc::new({ let __pe_b1 = format.clone(); move |__pe_a0| Dimension::toFlatString(__pe_a0, __pe_b1.clone()) }), (InstNode::name(var_field!((*ty).typeNode, NFType::UNTYPED).clone())?).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFType.toFlatString")); __mm_s.push_str(&*literal!(" got unknown type: ")); __mm_s.push_str(&*anyString(ty.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn dimensionsToFlatString(mut ty: Arc<NFType>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => stringDelimitList(List::map(var_field!((*ty).dimensions, NFType::ARRAY).clone(), Arc::new({ let __pe_b1 = format.clone(); move |__pe_a0| Dimension::toFlatString(__pe_a0, __pe_b1.clone()) })), (literal!(", ")).clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFType.dimensionsToFlatString")); __mm_s.push_str(&*literal!(" got unknown or not array type: ")); __mm_s.push_str(&*anyString(ty.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

pub fn toFlatDeclarationStream(mut ty: Arc<NFType>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut name: ArcStr = arcstr::literal!("");
    let mut complexTy: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut path: Arc<Absyn::Path>;
    let mut constructor: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut destructor: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut f: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    s = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { .. } => {
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("type ")).clone())?;
            s = IOStream::append(s.clone(), (Util::makeQuotedIdentifier((AbsynUtil::pathString(var_field!((*ty).typePath, NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(" = enumeration(")).clone())?;
            s = IOStream::append(s.clone(), stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut l in (var_field!((*ty).literals, NFType::ENUMERATION).clone()).into_iter().cloned() {
            let __x = Util::makeQuotedIdentifier((l.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone()))?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => Record::toFlatDeclarationStream(var_field!((*ty).cls, NFType::COMPLEX).clone(), format.clone(), (indent.clone()).clone(), s.clone())?,
        Deref @ COMPLEX { complexTy: complexTy @ Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => {
            path = InstNode::scopePath(var_field!((*ty).cls, NFType::COMPLEX).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?;
            name = (Util::makeQuotedIdentifier((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone())?).clone();
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("class ")).clone())?;
            s = IOStream::append(s.clone(), (name.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("\n  extends ExternalObject;\n\n")).clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(var_field!((**complexTy).constructor, ComplexType::NFComplexType::EXTERNAL_OBJECT).clone(), NFInstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            f = __pa0.clone();
            s = Function::toFlatStream(f.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone(), (literal!("constructor")).clone())?;
            s = IOStream::append(s.clone(), (literal!(";\n\n")).clone())?;
            let __pa2 = ::match_deref::match_deref! { match &(Function::typeNodeCache(var_field!((**complexTy).destructor, ComplexType::NFComplexType::EXTERNAL_OBJECT).clone(), NFInstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            f = __pa2.clone();
            s = Function::toFlatStream(f.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone(), (literal!("destructor")).clone())?;
            s = IOStream::append(s.clone(), (literal!(";\n\nend ")).clone())?;
            s = IOStream::append(s.clone(), (name.clone()).clone())?;
            s.clone()
        },
        Deref @ FUNCTION { .. } => Function::toFlatStream(var_field!((*ty).r#fn, NFType::FUNCTION).clone(), format.clone(), (indent.clone()).clone(), s.clone(), (Util::makeQuotedIdentifier((AbsynUtil::pathString(InstNode::scopePath(var_field!((*ty).r#fn, NFType::FUNCTION).node.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone())?).clone())?,
        _ => s.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn typenameString(mut ty: Arc<NFType>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { .. } => AbsynUtil::pathString(var_field!((*ty).typePath, NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?,
        _ => toString(ty.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn toDAE(mut ty: Arc<NFType>, mut makeTypeVars: bool) -> Result<Arc<DAE::Type>> {
    let mut daeTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    daeTy = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER => DAE::T_INTEGER_DEFAULT().clone(),
        Deref @ REAL => DAE::T_REAL_DEFAULT().clone(),
        Deref @ STRING => DAE::T_STRING_DEFAULT().clone(),
        Deref @ BOOLEAN => DAE::T_BOOL_DEFAULT().clone(),
        Deref @ ENUMERATION { .. } => Arc::new(DAE::Type::T_ENUMERATION { index: None, path: var_field!((*ty).typePath, NFType::ENUMERATION).clone(), names: var_field!((*ty).literals, NFType::ENUMERATION).clone(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }),
        Deref @ CLOCK => DAE::T_CLOCK_DEFAULT().clone(),
        Deref @ ARRAY { .. } => Arc::new(DAE::Type::T_ARRAY { ty: toDAE(var_field!((*ty).elementType, NFType::ARRAY).clone(), makeTypeVars.clone())?, dims: ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for mut d in (var_field!((*ty).dimensions, NFType::ARRAY).clone()).into_iter().cloned() {
            let __x = Dimension::toDAE(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) }),
        Deref @ TUPLE { .. } => Arc::new(DAE::Type::T_TUPLE { types: ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut t in (var_field!((*ty).types, NFType::TUPLE).clone()).into_iter().cloned() {
            let __x = toDAE(t.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), names: var_field!((*ty).names, NFType::TUPLE).clone() }),
        Deref @ FUNCTION { .. } => (match var_field!((*ty).fnType, NFType::FUNCTION).clone() {
        FunctionType::FUNCTIONAL_PARAMETER => Function::makeDAEType(var_field!((*ty).r#fn, NFType::FUNCTION).clone(), false)?,
        FunctionType::FUNCTION_REFERENCE => Arc::new(DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: Function::isBuiltin(var_field!((*ty).r#fn, NFType::FUNCTION).clone()), functionType: Function::makeDAEType(var_field!((*ty).r#fn, NFType::FUNCTION).clone(), false)? }),
        FunctionType::FUNCTIONAL_VARIABLE => Arc::new(DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: Function::makeDAEType(var_field!((*ty).r#fn, NFType::FUNCTION).clone(), true)? }),
        _ => bail!("match: no arm matched"),
    }),
        Deref @ NORETCALL => DAE::T_NORETCALL_DEFAULT().clone(),
        Deref @ UNKNOWN => DAE::T_UNKNOWN_DEFAULT().clone(),
        Deref @ COMPLEX { .. } => if (makeTypeVars.clone()) {InstNode::toFullDAEType(var_field!((*ty).cls, NFType::COMPLEX).clone())?} else {InstNode::toPartialDAEType(var_field!((*ty).cls, NFType::COMPLEX).clone())?},
        Deref @ METABOXED { .. } => Arc::new(DAE::Type::T_METABOXED { ty: toDAE(var_field!((*ty).ty, NFType::METABOXED).clone(), true)? }),
        Deref @ POLYMORPHIC { .. } => Arc::new(DAE::Type::T_METAPOLYMORPHIC { name: (var_field!((*ty).name, NFType::POLYMORPHIC).clone()).clone() }),
        Deref @ ANY => Arc::new(DAE::Type::T_ANYTYPE { anyClassType: None }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFType.toDAE")); __mm_s.push_str(&*literal!(" got unknown type: ")); __mm_s.push_str(&*anyString(ty.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(daeTy)
}

pub fn subscript(mut ty: Arc<NFType>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut failOnError: bool) -> Result<Arc<NFType>> {
    let mut ty: Arc<NFType> = ty;
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut subbed_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut el_ty: Arc<NFType> = Arc::new(NFType::ANY);
    if subs.clone().is_empty() {
        return Ok(ty.clone());
    }
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { dimensions: dims, .. } if (!(failOnError.clone()) && (subs.clone().len() as i32) > (dims.clone().len() as i32)) => Arc::new(crate::NFType::UNKNOWN),
        Deref @ ARRAY { dimensions: dims, .. } => {
            let mut dims = (*dims).clone();
            for mut sub in &*subs.clone() {
                let mut sub = sub.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                dim = __pa0.clone();
                dims = __pa1.clone();
                subbed_dims = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::INDEX { .. } => subbed_dims.clone(),
        Deref @ Subscript::SLICE { .. } => cons(Subscript::toDimension(sub.clone())?, subbed_dims.clone()),
        Deref @ Subscript::WHOLE => cons(dim.clone(), subbed_dims.clone()),
        Deref @ Subscript::SPLIT_INDEX { .. } => subbed_dims.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFType.subscript")); __mm_s.push_str(&*literal!(" got wrong subscript ")); __mm_s.push_str(&*Subscript::toString(sub.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            el_ty = arrayElementType(ty.clone());
            if (!(subbed_dims.clone().is_empty() && dims.clone().is_empty())) {Arc::new(NFType::ARRAY { elementType: el_ty.clone(), dimensions: listAppend(subbed_dims.clone().reverse(), dims.clone()) })} else {el_ty.clone()}
        },
        Deref @ CONDITIONAL_ARRAY { .. } => Arc::new(NFType::CONDITIONAL_ARRAY { trueType: subscript(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), subs.clone(), true)?, falseType: subscript(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), subs.clone(), true)?, matchedBranch: var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() }),
        Deref @ METABOXED { .. } => Arc::new(NFType::METABOXED { ty: subscript(var_field!((*ty).ty, NFType::METABOXED).clone(), subs.clone(), true)? }),
        Deref @ UNKNOWN { .. } => ty.clone(),
        _ => {
            if failOnError.clone() {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFType.subscript")); __mm_s.push_str(&*literal!(" got unsubscriptable type ")); __mm_s.push_str(&*toString(ty.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("fail");
            }
            Arc::new(crate::NFType::UNKNOWN)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn isEqual(mut ty1: Arc<NFType>, mut ty2: Arc<NFType>) -> bool {
    let mut equal: bool = false;
    if referenceEq(&ty1.clone(),&ty2.clone()) {
        equal = true;
        return equal.clone();
    }
    if metamodelica::valueConstructor((&*ty1.clone())).unwrap() != metamodelica::valueConstructor((&*ty2.clone())).unwrap() {
        equal = false;
        return equal.clone();
    }
    equal = (::match_deref::match_deref! { match &((ty1.clone(), ty2.clone())) {
        (Deref @ ENUMERATION { .. }, Deref @ ENUMERATION { .. }) => {
            List::isEqualOnTrue(var_field!((*ty1).literals, NFType::ENUMERATION).clone(), var_field!((*ty2).literals, NFType::ENUMERATION).clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))
        },
        (Deref @ ARRAY { .. }, Deref @ ARRAY { .. }) => {
            isEqual(var_field!((*ty1).elementType, NFType::ARRAY).clone(), var_field!((*ty2).elementType, NFType::ARRAY).clone()) && List::isEqualOnTrue(var_field!((*ty1).dimensions, NFType::ARRAY).clone(), var_field!((*ty2).dimensions, NFType::ARRAY).clone(), (std::sync::Arc::new(Dimension::isEqualKnown) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, Arc<Dimension::NFDimension>) -> Result<bool> + 'static>))
        },
        (Deref @ CONDITIONAL_ARRAY { .. }, Deref @ CONDITIONAL_ARRAY { .. }) => {
            isEqual(var_field!((*ty1).trueType, NFType::CONDITIONAL_ARRAY).clone(), var_field!((*ty2).trueType, NFType::CONDITIONAL_ARRAY).clone()) && isEqual(var_field!((*ty1).falseType, NFType::CONDITIONAL_ARRAY).clone(), var_field!((*ty2).falseType, NFType::CONDITIONAL_ARRAY).clone())
        },
        (Deref @ TUPLE { names: Some(names1), .. }, Deref @ TUPLE { names: Some(names2), .. }) => {
            List::isEqualOnTrue(names1.clone(), names2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)) && List::isEqualOnTrue(var_field!((*ty1).types, NFType::TUPLE).clone(), var_field!((*ty2).types, NFType::TUPLE).clone(), (std::sync::Arc::new(fnptr!(isEqual, Arc<NFType>, Arc<NFType>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFType>, Arc<NFType>) -> Result<bool> + 'static>))
        },
        (Deref @ TUPLE { names: None, .. }, Deref @ TUPLE { names: None, .. }) => {
            List::isEqualOnTrue(var_field!((*ty1).types, NFType::TUPLE).clone(), var_field!((*ty2).types, NFType::TUPLE).clone(), (std::sync::Arc::new(fnptr!(isEqual, Arc<NFType>, Arc<NFType>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFType>, Arc<NFType>) -> Result<bool> + 'static>))
        },
        (Deref @ TUPLE { .. }, Deref @ TUPLE { .. }) => {
            false
        },
        (Deref @ COMPLEX { .. }, Deref @ COMPLEX { .. }) => {
            InstNode::isSame(var_field!((*ty1).cls, NFType::COMPLEX).clone(), var_field!((*ty2).cls, NFType::COMPLEX).clone())
        },
        (Deref @ UNTYPED { .. }, Deref @ UNTYPED { .. }) => {
            InstNode::isSame(var_field!((*ty1).typeNode, NFType::UNTYPED).clone(), var_field!((*ty2).typeNode, NFType::UNTYPED).clone()) && Array::isEqualOnTrue(var_field!((*ty1).dimensions, NFType::UNTYPED).clone(), var_field!((*ty2).dimensions, NFType::UNTYPED).clone(), (std::sync::Arc::new(Dimension::isEqualKnown) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, Arc<Dimension::NFDimension>) -> Result<bool> + 'static>))
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

pub fn hashContinue(mut ty: Arc<NFType>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER => stringHashDjb2Continue((literal!("Integer")).clone(), hash.clone()),
        Deref @ REAL => stringHashDjb2Continue((literal!("Real")).clone(), hash.clone()),
        Deref @ STRING => stringHashDjb2Continue((literal!("String")).clone(), hash.clone()),
        Deref @ BOOLEAN => stringHashDjb2Continue((literal!("Boolean")).clone(), hash.clone()),
        Deref @ CLOCK => stringHashDjb2Continue((literal!("Clock")).clone(), hash.clone()),
        Deref @ ENUMERATION { .. } => {
            if var_field!((*ty).literals, NFType::ENUMERATION).clone().is_empty() {
                hash = stringHashDjb2Continue((literal!("enumeration(:)")).clone(), hash.clone());
            } else {
                hash = stringHashDjb2Continue((literal!("enumeration")).clone(), hash.clone());
                hash = AbsynUtil::pathHashContinue(var_field!((*ty).typePath, NFType::ENUMERATION).clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
                for mut lit in &*var_field!((*ty).literals, NFType::ENUMERATION).clone() {
                    let mut lit = lit.clone();
                    hash = stringHashDjb2Continue((lit.clone()).clone(), hash.clone());
                    hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
                }
                hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            }
            hash.clone()
        },
        Deref @ ARRAY { .. } => {
            hash = hashContinue(var_field!((*ty).elementType, NFType::ARRAY).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!("[")).clone(), hash.clone());
            for mut dim in &*var_field!((*ty).dimensions, NFType::ARRAY).clone() {
                let mut dim = dim.clone();
                hash = stringHashDjb2Continue((Dimension::toString(dim.clone())?).clone(), hash.clone());
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ TUPLE { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            for mut t in &*var_field!((*ty).types, NFType::TUPLE).clone() {
                let mut t = t.clone();
                hash = hashContinue(t.clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ NORETCALL => stringHashDjb2Continue((literal!("()")).clone(), hash.clone()),
        Deref @ UNKNOWN => stringHashDjb2Continue((literal!("unknown()")).clone(), hash.clone()),
        Deref @ COMPLEX { .. } => AbsynUtil::pathHashContinue(InstNode::scopePath(var_field!((*ty).cls, NFType::COMPLEX).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, hash.clone())?,
        Deref @ FUNCTION { .. } => stringHashDjb2Continue((Function::typeString(var_field!((*ty).r#fn, NFType::FUNCTION).clone())?).clone(), hash.clone()),
        Deref @ METABOXED { .. } => hashContinue(var_field!((*ty).ty, NFType::METABOXED).clone(), hash.clone())?,
        Deref @ POLYMORPHIC { .. } => stringHashDjb2Continue((var_field!((*ty).name, NFType::POLYMORPHIC).clone()).clone(), hash.clone()),
        Deref @ ANY => stringHashDjb2Continue((literal!("$ANY$")).clone(), hash.clone()),
        Deref @ CONDITIONAL_ARRAY { .. } => {
            hash = hashContinue(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone(), hash.clone())?;
            hash = hashContinue(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ UNTYPED { .. } => {
            hash = InstNode::hashContinue(var_field!((*ty).typeNode, NFType::UNTYPED).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!("[")).clone(), hash.clone());
            let __range0 = var_field!((*ty).dimensions, NFType::UNTYPED).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut dim in __range0 {
                hash = stringHashDjb2Continue((Dimension::toString(dim.clone())?).clone(), hash.clone());
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash.clone());
            hash.clone()
        },
        _ => hash.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hash)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isDiscrete(mut ty: Arc<NFType>) -> bool {
    let mut isDiscrete: bool = false;
    isDiscrete = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ STRING { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ ENUMERATION { .. } => true,
        Deref @ ARRAY { .. } => self::isDiscrete(var_field!((*ty).elementType, NFType::ARRAY).clone()),
        Deref @ CONDITIONAL_ARRAY { .. } => self::isDiscrete(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()),
        Deref @ FUNCTION { .. } => self::isDiscrete(Function::returnType(var_field!((*ty).r#fn, NFType::FUNCTION).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDiscrete
}

pub fn lookupRecordFieldType(mut name: ArcStr, mut recordType: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut fieldType: Arc<NFType> = Arc::new(NFType::ANY);
    fieldType = (::match_deref::match_deref! { match &(recordType.clone()) {
        Deref @ COMPLEX { .. } => InstNode::getType((Class::lookupElement((name.clone()).clone(), InstNode::getClass(var_field!((*recordType).cls, NFType::COMPLEX).clone())?)?).0)?,
        Deref @ ARRAY { .. } => liftArrayLeftList(lookupRecordFieldType((name.clone()).clone(), var_field!((*recordType).elementType, NFType::ARRAY).clone())?, var_field!((*recordType).dimensions, NFType::ARRAY).clone()),
        Deref @ CONDITIONAL_ARRAY { .. } => Arc::new(NFType::CONDITIONAL_ARRAY { trueType: lookupRecordFieldType((name.clone()).clone(), var_field!((*recordType).trueType, NFType::CONDITIONAL_ARRAY).clone())?, falseType: lookupRecordFieldType((name.clone()).clone(), var_field!((*recordType).falseType, NFType::CONDITIONAL_ARRAY).clone())?, matchedBranch: var_field!((*recordType).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(fieldType)
}

pub fn recordFieldCount(mut recordType: Arc<NFType>) -> i32 {
    let mut fieldCount: i32 = 0;
    let mut fields: metamodelica::Array<Arc<Record::Field::Field>>;
    fieldCount = (::match_deref::match_deref! { match &(recordType.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { fields, .. }, .. } => (fields.clone().borrow().len() as i32),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    fieldCount
}

pub fn recordFields(mut recordType: Arc<NFType>) -> Arc<metamodelica::List<Arc<Record::Field::Field>>> {
    let mut field_lst: Arc<metamodelica::List<Arc<Record::Field::Field>>> = metamodelica::nil();
    field_lst = (::match_deref::match_deref! { match &(recordType.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { fields, .. }, .. } => {
            Arc::new(fields.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    field_lst
}

pub fn setRecordFields(mut field_lst: Arc<metamodelica::List<Arc<Record::Field::Field>>>, mut recordType: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut recordType: Arc<NFType> = recordType;
    recordType = ({
        let mut fields: metamodelica::Array<Arc<Record::Field::Field>> = metamodelica::arrayFromVec(field_lst.clone().into_iter().cloned().collect());
        (::match_deref::match_deref! { match &(recordType.clone()) {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, .. } => {
            let mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>;
            indexMap = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (fields.clone().borrow().len() as i32));
            updateRecordFieldsIndexMap(fields.clone(), indexMap.clone())?;
            Arc::new(NFType::COMPLEX { cls: var_field!((*recordType).cls, NFType::COMPLEX).clone(), complexTy: Arc::new(ComplexType::NFComplexType::RECORD { constructor: rec_node.clone(), fields: fields.clone(), indexMap: indexMap.clone() }) })
        },
        _ => {
            recordType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(recordType)
}

pub fn updateRecordFieldsIndexMap(mut fields: metamodelica::Array<Arc<Record::Field::Field>>, mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>) -> Result<()> {
    let __range0 = 1..=(fields.clone().borrow().len() as i32);
    for mut i in __range0 {
        UnorderedMap::add((Record::Field::name(fields.borrow()[(i.clone()-1) as usize].clone())?).clone(), i.clone(), indexMap.clone())?;
    }
    Ok(())
}

pub fn tupleFieldCount(mut tupleType: Arc<NFType>) -> i32 {
    let mut fieldCount: i32 = 0;
    fieldCount = (::match_deref::match_deref! { match &(tupleType.clone()) {
        Deref @ TUPLE { .. } => (var_field!((*tupleType).types, NFType::TUPLE).clone().len() as i32),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    fieldCount
}

pub fn enumName(mut ty: Arc<NFType>) -> Result<Arc<Absyn::Path>> {
    let mut name: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { typePath: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn enumSize(mut ty: Arc<NFType>) -> Result<i32> {
    let mut size: i32 = 0;
    let mut literals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    literals = __pa0.clone();
    size = (literals.clone().len() as i32);
    Ok(size)
}

pub fn r#box(mut ty: Arc<NFType>) -> Arc<NFType> {
    let mut boxedType: Arc<NFType> = Arc::new(NFType::ANY);
    boxedType = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ STRING { .. } => ty.clone(),
        Deref @ TUPLE { .. } => Arc::new(NFType::TUPLE { types: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFType>>> = metamodelica::nil();
        for mut t in (var_field!((*ty).types, NFType::TUPLE).clone()).into_iter().cloned() {
            let __x = r#box(t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), names: var_field!((*ty).names, NFType::TUPLE).clone() }),
        Deref @ FUNCTION { .. } => ty.clone(),
        Deref @ METABOXED { .. } => ty.clone(),
        Deref @ POLYMORPHIC { .. } => ty.clone(),
        Deref @ ANY { .. } => ty.clone(),
        Deref @ CONDITIONAL_ARRAY { .. } => Arc::new(NFType::CONDITIONAL_ARRAY { trueType: r#box(var_field!((*ty).trueType, NFType::CONDITIONAL_ARRAY).clone()), falseType: r#box(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone()), matchedBranch: var_field!((*ty).matchedBranch, NFType::CONDITIONAL_ARRAY).clone() }),
        _ => Arc::new(NFType::METABOXED { ty: ty.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    boxedType
}

pub fn unbox(mut ty: Arc<NFType>) -> Arc<NFType> {
    let mut unboxedType: Arc<NFType> = Arc::new(NFType::ANY);
    unboxedType = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ METABOXED { .. } => var_field!((*ty).ty, NFType::METABOXED).clone(),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    unboxedType
}

pub fn isBoxed(mut ty: Arc<NFType>) -> bool {
    let mut isBoxed: bool = false;
    isBoxed = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ METABOXED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBoxed
}

pub fn sizeType(mut arrayTy: Arc<NFType>) -> Arc<NFType> {
    let mut sizeTy: Arc<NFType> = Arc::new(NFType::ANY);
    if isUnknown(arrayTy.clone()) {
        sizeTy = Arc::new(crate::NFType::UNKNOWN);
    } else {
        sizeTy = Arc::new(NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![Dimension::fromInteger(dimensionCount(arrayTy.clone()), NFPrefixes::Variability::CONSTANT.clone())] });
    }
    sizeTy
}

pub fn simplify(mut ty: Arc<NFType>) -> Result<Arc<NFType>> {
    let mut ty: Arc<NFType> = ty;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (var_field!((*ty).dimensions, NFType::ARRAY).clone()).into_iter().cloned() {
            let __x = Dimension::simplify(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn sizeOf(mut ty: Arc<NFType>, mut resize: bool) -> Result<i32> {
    pub fn fold_comp_size(mut comp: Arc<InstNode::InstNode>, mut sz: i32) -> i32 {
        let mut outSize: i32 = sz.clone() + sizeOf(InstNode::getType(comp.clone()).unwrap(), false).unwrap();
        outSize
    }

    let mut sz: i32 = 0;
    sz = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ INTEGER { .. } => 1,
        Deref @ REAL { .. } => 1,
        Deref @ STRING { .. } => 1,
        Deref @ BOOLEAN { .. } => 1,
        Deref @ CLOCK { .. } => 1,
        Deref @ ENUMERATION { .. } => 1,
        Deref @ ARRAY { .. } => sizeOf(var_field!((*ty).elementType, NFType::ARRAY).clone(), false)? * Dimension::sizesProduct(var_field!((*ty).dimensions, NFType::ARRAY).clone(), resize.clone()),
        Deref @ TUPLE { .. } => ({
        let mut __acc: i32 = 0;
        for mut t in (var_field!((*ty).types, NFType::TUPLE).clone()).into_iter().cloned() {
            let __x = sizeOf(t.clone(), false)?;
            __acc += __x;
        }
        __acc
    }),
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => 1,
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => ClassTree::foldComponents(Class::classTree(InstNode::getClass(var_field!((*ty).cls, NFType::COMPLEX).clone())?)?, (std::sync::Arc::new(fnptr!(fold_comp_size, Arc<InstNode::InstNode>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, i32) -> Result<i32> + 'static>), 0),
        Deref @ COMPLEX { .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sz)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn complexSize(mut ty: Arc<NFType>, mut resize: bool) -> Result<Option<i32>> {
    let mut sz: Option<i32> = None;
    sz = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ ARRAY { .. } => complexSize(var_field!((*ty).elementType, NFType::ARRAY).clone(), resize.clone())?,
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => Some(sizeOf(ty.clone(), resize.clone())?),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sz)
}


