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
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableKind;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClockKind as ClockKind;
use crate::NFComplexType as ComplexType;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction as Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::JSON;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Debug, PartialEq)]
pub enum NFExpression {
    INTEGER {
        value: i32,
    },
    REAL {
        value: metamodelica::Real,
    },
    STRING {
        value: ArcStr,
    },
    BOOLEAN {
        value: bool,
    },
    ENUM_LITERAL {
        ty: Arc<Type::NFType>,
        name: ArcStr,
        index: i32,
    },
    /// Clock constructors
    CLKCONST {
        /// Clock kinds
        clk: Arc<ClockKind::NFClockKind>,
    },
    CREF {
        ty: Arc<Type::NFType>,
        cref: Arc<ComponentRef::NFComponentRef>,
    },
    /// Represents a type used as a range, e.g. Boolean.
    TYPENAME {
        ty: Arc<Type::NFType>,
    },
    ARRAY {
        ty: Arc<Type::NFType>,
        elements: metamodelica::Array<Arc<NFExpression>>,
        /// True if the array is known to only contain literal expressions.
        literal: bool,
    },
    /// The array concatentation operator [a,b; c,d]; this should be removed during type-checking
    MATRIX {
        elements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>>,
    },
    RANGE {
        ty: Arc<Type::NFType>,
        start: Arc<NFExpression>,
        step: Option<Arc<NFExpression>>,
        stop: Arc<NFExpression>,
    },
    TUPLE {
        ty: Arc<Type::NFType>,
        elements: Arc<metamodelica::List<Arc<NFExpression>>>,
    },
    RECORD {
        path: Arc<Path>,
        ty: Arc<Type::NFType>,
        elements: Arc<metamodelica::List<Arc<NFExpression>>>,
    },
    CALL {
        call: Arc<Call::NFCall>,
    },
    SIZE {
        exp: Arc<NFExpression>,
        dimIndex: Option<Arc<NFExpression>>,
    },
    END,
    /// Binary operations, e.g. a+4
    BINARY {
        exp1: Arc<NFExpression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<NFExpression>,
    },
    /// Unary operations, -(4x)
    UNARY {
        operator: Arc<Operator::NFOperator>,
        exp: Arc<NFExpression>,
    },
    /// Logical binary operations: and, or
    LBINARY {
        exp1: Arc<NFExpression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<NFExpression>,
    },
    /// Logical unary operations: not
    LUNARY {
        operator: Arc<Operator::NFOperator>,
        exp: Arc<NFExpression>,
    },
    /// Relation, e.g. a <= 0
    RELATION {
        exp1: Arc<NFExpression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<NFExpression>,
        /// index for event codegen
        index: i32,
    },
    /// Multary expressions with the same operator, e.g. a+b+c
    ///    An empty list has to be interpreted as the neutral element of the operator space
    MULTARY {
        /// arguments that are chained with the operator (+, *)
        arguments: Arc<metamodelica::List<Arc<NFExpression>>>,
        /// arguments that are chained with the inverse operator (-, :)
        inv_arguments: Arc<metamodelica::List<Arc<NFExpression>>>,
        /// Can only be + or * (commutative)
        operator: Arc<Operator::NFOperator>,
    },
    IF {
        ty: Arc<Type::NFType>,
        condition: Arc<NFExpression>,
        trueBranch: Arc<NFExpression>,
        falseBranch: Arc<NFExpression>,
    },
    CAST {
        ty: Arc<Type::NFType>,
        exp: Arc<NFExpression>,
    },
    /// MetaModelica boxed value
    BOX {
        exp: Arc<NFExpression>,
    },
    /// MetaModelica value unboxing (similar to a cast)
    UNBOX {
        exp: Arc<NFExpression>,
        ty: Arc<Type::NFType>,
    },
    SUBSCRIPTED_EXP {
        exp: Arc<NFExpression>,
        subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>,
        ty: Arc<Type::NFType>,
        split: bool,
    },
    TUPLE_ELEMENT {
        tupleExp: Arc<NFExpression>,
        index: i32,
        ty: Arc<Type::NFType>,
    },
    RECORD_ELEMENT {
        recordExp: Arc<NFExpression>,
        index: i32,
        fieldName: ArcStr,
        ty: Arc<Type::NFType>,
    },
    MUTABLE {
        exp: Mutable::Mutable<Arc<NFExpression>>,
    },
    EMPTY {
        ty: Arc<Type::NFType>,
    },
    PARTIAL_FUNCTION_APPLICATION {
        r#fn: Arc<ComponentRef::NFComponentRef>,
        args: Arc<metamodelica::List<Arc<NFExpression>>>,
        argNames: Arc<metamodelica::List<ArcStr>>,
        ty: Arc<Type::NFType>,
    },
    FILENAME {
        filename: ArcStr,
    },
    /// Before code generation, we make a pass that replaces constant literals
    ///    with a SHARED_LITERAL expression. Any immutable type can be shared:
    ///    basic MetaModelica types and Modelica strings are fine. There is no point
    ///    to share Real, Integer, Boolean or Enum though.
    SHARED_LITERAL {
        /// A unique indexing that can be used to point to a single shared literal in generated code
        index: i32,
        /// For printing strings, code generators that do not support this kind of literal, or for getting the type in case the code generator needs that
        exp: Arc<NFExpression>,
    },
    INSTANCE_NAME {
        scope: Arc<InstNode::InstNode>,
    },
}
impl Default for NFExpression {
    fn default() -> Self { Self::END }
}
pub use self::NFExpression::{INTEGER,REAL,STRING,BOOLEAN,ENUM_LITERAL,CLKCONST,CREF,TYPENAME,ARRAY,MATRIX,RANGE,TUPLE,RECORD,CALL,SIZE,END,BINARY,UNARY,LBINARY,LUNARY,RELATION,MULTARY,IF,CAST,BOX,UNBOX,SUBSCRIPTED_EXP,TUPLE_ELEMENT,RECORD_ELEMENT,MUTABLE,EMPTY,PARTIAL_FUNCTION_APPLICATION,FILENAME,SHARED_LITERAL,INSTANCE_NAME};
pub fn isArray(mut exp: Arc<NFExpression>) -> bool {
    let mut isArray: bool = false;
    isArray = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}

pub fn isEmptyArray(mut exp: Arc<NFExpression>) -> bool {
    let mut emptyArray: bool = false;
    emptyArray = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().is_empty(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    emptyArray
}

pub fn isVector(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => Type::isVector(var_field!((*exp).ty, NFExpression::ARRAY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isCref(mut exp: Arc<NFExpression>) -> bool {
    let mut isCref: bool = false;
    isCref = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCref
}

pub fn isFunctionInputCref(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isInput(ComponentRef::last(var_field!((*exp).cref, NFExpression::CREF).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isWildCref(mut exp: Arc<NFExpression>) -> bool {
    let mut wild: bool = false;
    wild = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::WILD, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    wild
}

pub fn isCall(mut exp: Arc<NFExpression>) -> bool {
    let mut isCall: bool = false;
    isCall = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCall
}

pub fn isImpureCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut isImpure: bool = false;
    isImpure = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isImpure(var_field!((*exp).call, NFExpression::CALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isImpure)
}

pub fn isExternalCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isExternal(var_field!((*exp).call, NFExpression::CALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isCallNamed(mut exp: Arc<NFExpression>, mut name: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isNamed(var_field!((*exp).call, NFExpression::CALL).clone(), (name.clone()).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isConnectionCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut isConnection: bool = false;
    isConnection = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isConnectionsOperator(var_field!((*exp).call, NFExpression::CALL).clone())? || Call::isStreamOperator(var_field!((*exp).call, NFExpression::CALL).clone()) || Call::isCardinality(var_field!((*exp).call, NFExpression::CALL).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isConnection)
}

pub fn isTrue(mut exp: Arc<NFExpression>) -> bool {
    let mut isTrue: bool = false;
    isTrue = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BOOLEAN { value: true } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTrue
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isAllTrue(mut exp: Arc<NFExpression>) -> bool {
    let mut isTrue: bool = false;
    isTrue = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BOOLEAN { value: true } => {
            true
        },
        Deref @ ARRAY { .. } => {
            Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new(fnptr!(isAllTrue, Arc<NFExpression>)))
        },
        Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: e, .. } } => {
            isAllTrue(e.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTrue
}

pub fn isFalse(mut exp: Arc<NFExpression>) -> bool {
    let mut isTrue: bool = false;
    isTrue = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BOOLEAN { value: false } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTrue
}

pub fn isTrivialCref(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => true,
        Deref @ UNARY { exp: Deref @ CREF { .. }, .. } => true,
        Deref @ LUNARY { exp: Deref @ CREF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hash(mut exp: Arc<NFExpression>) -> i32 {
    let mut hash: i32 = hashContinue(exp.clone(), Util::HASH_SEED.clone()).unwrap();
    hash
}

pub fn hashContinue(mut exp: Arc<NFExpression>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            stringHashDjb2Continue((intString(var_field!((*exp).value, NFExpression::INTEGER).clone())).clone(), hash.clone())
        },
        Deref @ REAL { .. } => {
            stringHashDjb2Continue((realString(var_field!((*exp).value, NFExpression::REAL).clone())).clone(), hash.clone())
        },
        Deref @ STRING { .. } => {
            stringHashDjb2Continue((var_field!((*exp).value, NFExpression::STRING).clone()).clone(), hash.clone())
        },
        Deref @ BOOLEAN { .. } => {
            stringHashDjb2Continue((boolString(var_field!((*exp).value, NFExpression::BOOLEAN).clone())).clone(), hash.clone())
        },
        Deref @ ENUM_LITERAL { ty: Deref @ Type::ENUMERATION { typePath: path, .. }, .. } => {
            hash = AbsynUtil::pathHashContinue(path.clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(".")).clone(), hash.clone());
            hash = stringHashDjb2Continue((var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()).clone(), hash.clone());
            hash.clone()
        },
        Deref @ CLKCONST { .. } => {
            ClockKind::hashContinue(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), hash.clone())?
        },
        Deref @ CREF { .. } => {
            ComponentRef::hashContinue(var_field!((*exp).cref, NFExpression::CREF).clone(), false, hash.clone())?
        },
        Deref @ TYPENAME { .. } => {
            Type::hashContinue(Type::arrayElementType(var_field!((*exp).ty, NFExpression::TYPENAME).clone()), hash.clone())?
        },
        Deref @ ARRAY { .. } => {
            hash = stringHashDjb2Continue((literal!("{")).clone(), hash.clone());
            let __range0 = var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                hash = hashContinue(e.clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!("}")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ MATRIX { .. } => {
            hash = stringHashDjb2Continue((literal!("[")).clone(), hash.clone());
            for mut el in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut el = el.clone();
                for mut e in &*el.clone() {
                    let mut e = e.clone();
                    hash = hashContinue(e.clone(), hash.clone())?;
                    hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
                }
                hash = stringHashDjb2Continue((literal!("; ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ RANGE { .. } => {
            hash = hashContinue(var_field!((*exp).start, NFExpression::RANGE).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(":")).clone(), hash.clone());
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                hash = hashContinue(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, hash.clone())?;
                hash = stringHashDjb2Continue((literal!(":")).clone(), hash.clone());
            }
            hash = hashContinue(var_field!((*exp).stop, NFExpression::RANGE).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ TUPLE { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            for mut e in &*var_field!((*exp).elements, NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                hash = hashContinue(e.clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ RECORD { .. } => {
            hash = AbsynUtil::pathHashContinue(var_field!((*exp).path, NFExpression::RECORD).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            for mut e in &*var_field!((*exp).elements, NFExpression::RECORD).clone() {
                let mut e = e.clone();
                hash = hashContinue(e.clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ CALL { .. } => {
            stringHashDjb2Continue((Call::toString(var_field!((*exp).call, NFExpression::CALL).clone())?).clone(), hash.clone())
        },
        Deref @ SIZE { .. } => {
            hash = stringHashDjb2Continue((literal!("size(")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::SIZE).clone(), hash.clone())?;
            if isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone()) {
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
                hash = hashContinue(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?, hash.clone())?;
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ END { .. } => {
            stringHashDjb2Continue((literal!("end")).clone(), hash.clone())
        },
        Deref @ BINARY { .. } => {
            hash = hashContinue(var_field!((*exp).exp1, NFExpression::BINARY).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::BINARY).clone(), (literal!(" ")).clone())?).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp2, NFExpression::BINARY).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ UNARY { .. } => {
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::UNARY).clone(), (literal!("")).clone())?).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::UNARY).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ LBINARY { .. } => {
            hash = hashContinue(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::LBINARY).clone(), (literal!(" ")).clone())?).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ LUNARY { .. } => {
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::LUNARY).clone(), (literal!("")).clone())?).clone(), hash.clone());
            hash = stringHashDjb2Continue((literal!(" ")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::LUNARY).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ RELATION { .. } => {
            hash = hashContinue(var_field!((*exp).exp1, NFExpression::RELATION).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::RELATION).clone(), (literal!(" ")).clone())?).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp2, NFExpression::RELATION).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ MULTARY { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            for mut e in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut e = e.clone();
                hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::MULTARY).clone(), (literal!(" ")).clone())?).clone(), hash.clone());
                hash = hashContinue(e.clone(), hash.clone())?;
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash = stringHashDjb2Continue((Operator::symbol(Operator::invert(var_field!((*exp).operator, NFExpression::MULTARY).clone())?, (literal!(" ")).clone())?).clone(), hash.clone());
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            for mut e in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut e = e.clone();
                hash = stringHashDjb2Continue((Operator::symbol(var_field!((*exp).operator, NFExpression::MULTARY).clone(), (literal!(" ")).clone())?).clone(), hash.clone());
                hash = hashContinue(e.clone(), hash.clone())?;
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ IF { .. } => {
            hash = stringHashDjb2Continue((literal!("if ")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).condition, NFExpression::IF).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(" then ")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).trueBranch, NFExpression::IF).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(" else ")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).falseBranch, NFExpression::IF).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ CAST { .. } => {
            hash = stringHashDjb2Continue((literal!("CAST(")).clone(), hash.clone());
            hash = Type::hashContinue(var_field!((*exp).ty, NFExpression::CAST).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::CAST).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ BOX { .. } => {
            hash = stringHashDjb2Continue((literal!("BOX(")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::BOX).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ UNBOX { .. } => {
            hash = stringHashDjb2Continue((literal!("UNBOX(")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::UNBOX).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(")[")).clone(), hash.clone());
            for mut sub in &*var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut sub = sub.clone();
                hash = Subscript::hashContinue(sub.clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            hash = hashContinue(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!("[")).clone(), hash.clone());
            hash = stringHashDjb2Continue((intString(var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone())).clone(), hash.clone());
            hash = stringHashDjb2Continue((literal!("]")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(").")).clone(), hash.clone());
            hash = stringHashDjb2Continue((var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), hash.clone());
            hash.clone()
        },
        Deref @ MUTABLE { .. } => {
            hashContinue(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), hash.clone())?
        },
        Deref @ EMPTY { .. } => {
            stringHashDjb2Continue((literal!("#EMPTY#")).clone(), hash.clone())
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            hash = stringHashDjb2Continue((literal!("function ")).clone(), hash.clone());
            hash = ComponentRef::hashContinue(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), false, hash.clone())?;
            hash = stringHashDjb2Continue((literal!("(")).clone(), hash.clone());
            for mut n in &*var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() {
                let mut n = n.clone();
                hash = stringHashDjb2Continue((n.clone()).clone(), hash.clone());
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!(" = ")).clone(), hash.clone());
            for mut a in &*var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() {
                let mut a = a.clone();
                hash = hashContinue(a.clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ FILENAME { .. } => {
            stringHashDjb2Continue((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone(), hash.clone())
        },
        Deref @ SHARED_LITERAL { .. } => {
            hash = stringHashDjb2Continue((literal!("LITERAL(")).clone(), hash.clone());
            hash = stringHashDjb2Continue((intString(var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone())).clone(), hash.clone());
            hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            hash = hashContinue(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((literal!(")")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ INSTANCE_NAME { .. } => {
            stringHashDjb2Continue((literal!("getInstanceName()")).clone(), hash.clone())
        },
        _ => {
            hash.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hash)
}

pub fn isEqual(mut exp1: Arc<NFExpression>, mut exp2: Arc<NFExpression>) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = 0 == compare(exp1.clone(), exp2.clone())?;
    Ok(isEqual)
}

pub fn compare(mut exp1: Arc<NFExpression>, mut exp2: Arc<NFExpression>) -> Result<i32> {
    let mut comp: i32 = 0;
    if referenceEq(&exp1.clone(),&exp2.clone()) {
        comp = 0;
        return Ok(comp);
    }
    comp = Util::intCompare(metamodelica::valueConstructor((&*exp1.clone()))?, metamodelica::valueConstructor((&*exp2.clone()))?);
    if comp.clone() != 0 {
        return Ok(comp);
    }
    comp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ INTEGER { .. } => {
            let mut i: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ INTEGER { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            Util::intCompare(var_field!((*exp1).value, NFExpression::INTEGER).clone(), i.clone())
        },
        Deref @ REAL { .. } => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ REAL { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            Util::realCompare(var_field!((*exp1).value, NFExpression::REAL).clone(), r.clone())
        },
        Deref @ STRING { .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ STRING { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            s = __pa0.clone();
            stringCompare((var_field!((*exp1).value, NFExpression::STRING).clone()).clone(), (s.clone()).clone())
        },
        Deref @ BOOLEAN { .. } => {
            let mut b: bool = false;
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ BOOLEAN { value: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            b = __pa0.clone();
            Util::boolCompare(var_field!((*exp1).value, NFExpression::BOOLEAN).clone(), b.clone())
        },
        Deref @ ENUM_LITERAL { .. } => {
            let mut i: i32 = 0;
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ ENUM_LITERAL { index: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            ty = __pa1.clone();
            comp = AbsynUtil::pathCompare(Type::enumName(var_field!((*exp1).ty, NFExpression::ENUM_LITERAL).clone())?, Type::enumName(ty.clone())?)?;
            if comp.clone() == 0 {
                comp = Util::intCompare(var_field!((*exp1).index, NFExpression::ENUM_LITERAL).clone(), i.clone());
            }
            comp.clone()
        },
        Deref @ CLKCONST { .. } => {
            let mut clk: Arc<ClockKind::NFClockKind>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ CLKCONST { clk: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            clk = __pa0.clone();
            ClockKind::compare(var_field!((*exp1).clk, NFExpression::CLKCONST).clone(), clk.clone())?
        },
        Deref @ CREF { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            ComponentRef::compare(var_field!((*exp1).cref, NFExpression::CREF).clone(), cr.clone())?
        },
        Deref @ TYPENAME { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ TYPENAME { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            valueCompare(var_field!((*exp1).ty, NFExpression::TYPENAME).clone(), ty.clone())
        },
        Deref @ ARRAY { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ ARRAY { elements: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arr = __pa0.clone();
            ty = __pa1.clone();
            comp = valueCompare(ty.clone(), var_field!((*exp1).ty, NFExpression::ARRAY).clone());
            if (comp.clone() == 0) {Array::compare(var_field!((*exp1).elements, NFExpression::ARRAY).clone(), arr.clone(), Arc::new(compare))} else {comp.clone()}
        },
        Deref @ MATRIX { .. } => {
            let mut mat: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ MATRIX { elements: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            mat = __pa0.clone();
            List::compare(var_field!((*exp1).elements, NFExpression::MATRIX).clone(), mat.clone(), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static> = Arc::new(compare); move |__pe_a0, __pe_a1| List::compare(__pe_a0, __pe_a1, __pe_b2.clone()) }))?
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut oe: Option<Arc<NFExpression>> = None;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ RANGE { stop: __pa0, step: __pa1, start: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            oe = __pa1.clone();
            e1 = __pa2.clone();
            comp = compare(var_field!((*exp1).start, NFExpression::RANGE).clone(), e1.clone())?;
            if comp.clone() == 0 {
                comp = compare(var_field!((*exp1).stop, NFExpression::RANGE).clone(), e2.clone())?;
                if comp.clone() == 0 {
                    comp = compareOpt(var_field!((*exp1).step, NFExpression::RANGE).clone(), oe.clone())?;
                }
            }
            comp.clone()
        },
        Deref @ TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ TUPLE { elements: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            List::compare(var_field!((*exp1).elements, NFExpression::TUPLE).clone(), expl.clone(), Arc::new(compare))?
        },
        Deref @ RECORD { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            let mut p: Arc<Path>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ RECORD { elements: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            comp = AbsynUtil::pathCompare(var_field!((*exp1).path, NFExpression::RECORD).clone(), p.clone())?;
            if (comp.clone() == 0) {List::compare(var_field!((*exp1).elements, NFExpression::RECORD).clone(), expl.clone(), Arc::new(compare))?} else {comp.clone()}
        },
        Deref @ CALL { .. } => {
            let mut c: Arc<Call::NFCall>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ CALL { call: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            c = __pa0.clone();
            Call::compare(var_field!((*exp1).call, NFExpression::CALL).clone(), c.clone())?
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut oe: Option<Arc<NFExpression>> = None;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ SIZE { dimIndex: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            oe = __pa0.clone();
            e1 = __pa1.clone();
            comp = compareOpt(var_field!((*exp1).dimIndex, NFExpression::SIZE).clone(), oe.clone())?;
            if (comp.clone() == 0) {compare(var_field!((*exp1).exp, NFExpression::SIZE).clone(), e1.clone())?} else {comp.clone()}
        },
        Deref @ END { .. } => {
            0
        },
        Deref @ MULTARY { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            let mut inv_expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ MULTARY { operator: __pa0, inv_arguments: __pa1, arguments: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            op = __pa0.clone();
            inv_expl = __pa1.clone();
            expl = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::MULTARY).clone(), op.clone());
            if comp.clone() == 0 {
                comp = compareList(var_field!((*exp1).arguments, NFExpression::MULTARY).clone(), expl.clone());
            }
            if comp.clone() == 0 {
                comp = compareList(var_field!((*exp1).inv_arguments, NFExpression::MULTARY).clone(), inv_expl.clone());
            }
            comp.clone()
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::BINARY).clone(), op.clone());
            if comp.clone() == 0 {
                comp = compare(var_field!((*exp1).exp1, NFExpression::BINARY).clone(), e1.clone())?;
                if comp.clone() == 0 {
                    comp = compare(var_field!((*exp1).exp2, NFExpression::BINARY).clone(), e2.clone())?;
                }
            }
            comp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ UNARY { exp: __pa0, operator: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            op = __pa1.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::UNARY).clone(), op.clone());
            if (comp.clone() == 0) {compare(var_field!((*exp1).exp, NFExpression::UNARY).clone(), e1.clone())?} else {comp.clone()}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ LBINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::LBINARY).clone(), op.clone());
            if comp.clone() == 0 {
                comp = compare(var_field!((*exp1).exp1, NFExpression::LBINARY).clone(), e1.clone())?;
                if comp.clone() == 0 {
                    comp = compare(var_field!((*exp1).exp2, NFExpression::LBINARY).clone(), e2.clone())?;
                }
            }
            comp.clone()
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ LUNARY { exp: __pa0, operator: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            op = __pa1.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::LUNARY).clone(), op.clone());
            if (comp.clone() == 0) {compare(var_field!((*exp1).exp, NFExpression::LUNARY).clone(), e1.clone())?} else {comp.clone()}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut op: Arc<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ RELATION { exp2: __pa0, operator: __pa1, exp1: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            comp = Operator::compare(var_field!((*exp1).operator, NFExpression::RELATION).clone(), op.clone());
            if comp.clone() == 0 {
                comp = compare(var_field!((*exp1).exp1, NFExpression::RELATION).clone(), e1.clone())?;
                if comp.clone() == 0 {
                    comp = compare(var_field!((*exp1).exp2, NFExpression::RELATION).clone(), e2.clone())?;
                }
            }
            comp.clone()
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ IF { falseBranch: __pa0, trueBranch: __pa1, condition: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e3 = __pa0.clone();
            e2 = __pa1.clone();
            e1 = __pa2.clone();
            comp = compare(var_field!((*exp1).condition, NFExpression::IF).clone(), e1.clone())?;
            if comp.clone() == 0 {
                comp = compare(var_field!((*exp1).trueBranch, NFExpression::IF).clone(), e2.clone())?;
                if comp.clone() == 0 {
                    comp = compare(var_field!((*exp1).falseBranch, NFExpression::IF).clone(), e3.clone())?;
                }
            }
            comp.clone()
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = (::match_deref::match_deref! { match &(exp2.clone()) {
        Deref @ CAST { exp: e1, .. } => e1.clone(),
        e1 => e1.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            compare(var_field!((*exp1).exp, NFExpression::CAST).clone(), e1.clone())?
        },
        Deref @ BOX { .. } => {
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ BOX { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            compare(var_field!((*exp1).exp, NFExpression::BOX).clone(), e2.clone())?
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ UNBOX { exp: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            compare(var_field!((*exp1).exp, NFExpression::UNBOX).clone(), e1.clone())?
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ SUBSCRIPTED_EXP { subscripts: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            subs = __pa0.clone();
            e1 = __pa1.clone();
            comp = compare(var_field!((*exp1).exp, NFExpression::SUBSCRIPTED_EXP).clone(), e1.clone())?;
            if comp.clone() == 0 {
                comp = Subscript::compareList(var_field!((*exp1).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), subs.clone())?;
            }
            comp.clone()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut i: i32 = 0;
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ TUPLE_ELEMENT { index: __pa0, tupleExp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            e1 = __pa1.clone();
            comp = Util::intCompare(var_field!((*exp1).index, NFExpression::TUPLE_ELEMENT).clone(), i.clone());
            if comp.clone() == 0 {
                comp = compare(var_field!((*exp1).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), e1.clone())?;
            }
            comp.clone()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut i: i32 = 0;
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ RECORD_ELEMENT { index: __pa0, recordExp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            e1 = __pa1.clone();
            comp = Util::intCompare(var_field!((*exp1).index, NFExpression::RECORD_ELEMENT).clone(), i.clone());
            if comp.clone() == 0 {
                comp = compare(var_field!((*exp1).recordExp, NFExpression::RECORD_ELEMENT).clone(), e1.clone())?;
            }
            comp.clone()
        },
        Deref @ MUTABLE { .. } => {
            let mut me: Mutable::Mutable<Arc<NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            me = __pa0.clone();
            compare(Mutable::access(var_field!((*exp1).exp, NFExpression::MUTABLE).clone()), Mutable::access(me.clone()))?
        },
        Deref @ SHARED_LITERAL { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ SHARED_LITERAL { exp: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            compare(var_field!((*exp1).exp, NFExpression::SHARED_LITERAL).clone(), e1.clone())?
        },
        Deref @ EMPTY { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ EMPTY { ty: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            valueCompare(var_field!((*exp1).ty, NFExpression::EMPTY).clone(), ty.clone())
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ PARTIAL_FUNCTION_APPLICATION { args: __pa0, r#fn: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            cr = __pa1.clone();
            comp = ComponentRef::compare(var_field!((*exp1).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), cr.clone())?;
            if comp.clone() == 0 {
                comp = List::compare(var_field!((*exp1).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), expl.clone(), Arc::new(compare))?;
            }
            comp.clone()
        },
        Deref @ FILENAME { .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ FILENAME { filename: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            s = __pa0.clone();
            stringCompare((var_field!((*exp1).filename, NFExpression::FILENAME).clone()).clone(), (s.clone()).clone())
        },
        Deref @ INSTANCE_NAME { .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let __pa0 = ::match_deref::match_deref! { match &(exp2.clone()) {
                Deref @ INSTANCE_NAME { scope: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            InstNode::refCompare(var_field!((*exp1).scope, NFExpression::INSTANCE_NAME).clone(), node.clone())?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.compare")); __mm_s.push_str(&*literal!(" got unknown expression.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn compareOpt(mut expl1: Option<Arc<NFExpression>>, mut expl2: Option<Arc<NFExpression>>) -> Result<i32> {
    let mut comp: i32 = 0;
    let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
    comp = (::match_deref::match_deref! { match &((expl1.clone(), expl2.clone())) {
        (None, None) => 0,
        (None, _) => -1,
        (_, None) => 1,
        (Some(e1), Some(e2)) => compare(e1.clone(), e2.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(comp)
}

pub fn compareList(mut expl1: Arc<metamodelica::List<Arc<NFExpression>>>, mut expl2: Arc<metamodelica::List<Arc<NFExpression>>>) -> i32 {
    let mut comp: i32 = List::compare(expl1.clone(), expl2.clone(), Arc::new(compare)).unwrap();
    comp
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn typeOf(mut exp: Arc<NFExpression>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => Arc::new(crate::NFType::INTEGER),
        Deref @ REAL { .. } => Arc::new(crate::NFType::REAL),
        Deref @ STRING { .. } => Arc::new(crate::NFType::STRING),
        Deref @ BOOLEAN { .. } => Arc::new(crate::NFType::BOOLEAN),
        Deref @ ENUM_LITERAL { .. } => var_field!((*exp).ty, NFExpression::ENUM_LITERAL).clone(),
        Deref @ CLKCONST { .. } => Arc::new(crate::NFType::CLOCK),
        Deref @ CREF { .. } => var_field!((*exp).ty, NFExpression::CREF).clone(),
        Deref @ TYPENAME { .. } => var_field!((*exp).ty, NFExpression::TYPENAME).clone(),
        Deref @ ARRAY { .. } => var_field!((*exp).ty, NFExpression::ARRAY).clone(),
        Deref @ RANGE { .. } => var_field!((*exp).ty, NFExpression::RANGE).clone(),
        Deref @ TUPLE { .. } => var_field!((*exp).ty, NFExpression::TUPLE).clone(),
        Deref @ RECORD { .. } => var_field!((*exp).ty, NFExpression::RECORD).clone(),
        Deref @ CALL { .. } => Call::typeOf(var_field!((*exp).call, NFExpression::CALL).clone()),
        Deref @ SIZE { .. } => if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {Arc::new(crate::NFType::INTEGER)} else {Type::sizeType(typeOf(var_field!((*exp).exp, NFExpression::SIZE).clone()))},
        Deref @ END { .. } => Arc::new(crate::NFType::INTEGER),
        Deref @ MULTARY { .. } => Operator::typeOf(var_field!((*exp).operator, NFExpression::MULTARY).clone()),
        Deref @ BINARY { .. } => Operator::typeOf(var_field!((*exp).operator, NFExpression::BINARY).clone()),
        Deref @ UNARY { .. } => Operator::typeOf(var_field!((*exp).operator, NFExpression::UNARY).clone()),
        Deref @ LBINARY { .. } => Operator::typeOf(var_field!((*exp).operator, NFExpression::LBINARY).clone()),
        Deref @ LUNARY { .. } => Operator::typeOf(var_field!((*exp).operator, NFExpression::LUNARY).clone()),
        Deref @ RELATION { .. } => Type::copyDims(Operator::typeOf(var_field!((*exp).operator, NFExpression::RELATION).clone()), Arc::new(crate::NFType::BOOLEAN)),
        Deref @ IF { .. } => var_field!((*exp).ty, NFExpression::IF).clone(),
        Deref @ CAST { .. } => var_field!((*exp).ty, NFExpression::CAST).clone(),
        Deref @ BOX { .. } => Arc::new(Type::NFType::METABOXED { ty: typeOf(var_field!((*exp).exp, NFExpression::BOX).clone()) }),
        Deref @ UNBOX { .. } => var_field!((*exp).ty, NFExpression::UNBOX).clone(),
        Deref @ SUBSCRIPTED_EXP { .. } => var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(),
        Deref @ TUPLE_ELEMENT { .. } => var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone(),
        Deref @ RECORD_ELEMENT { .. } => var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone(),
        Deref @ MUTABLE { .. } => typeOf(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone())),
        Deref @ SHARED_LITERAL { .. } => typeOf(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone()),
        Deref @ EMPTY { .. } => var_field!((*exp).ty, NFExpression::EMPTY).clone(),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => var_field!((*exp).ty, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(),
        Deref @ FILENAME { .. } => Arc::new(crate::NFType::STRING),
        Deref @ INSTANCE_NAME { .. } => Arc::new(crate::NFType::STRING),
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn sizeOf(mut exp: Arc<NFExpression>) -> i32 {
    let mut sz: i32 = Type::sizeOf(typeOf(exp.clone()), false).unwrap();
    sz
}

pub fn sizeZero(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    match '__try0: {
        b = 0 == sizeOf(exp.clone());
        Ok::<_, anyhow::Error>((b.clone(),))
    } {
        Ok((__try0_o0,)) => {
            b = __try0_o0;
        }
        Err(_) => {
            b = false;
        }
    }
    b
}

pub fn setType(mut ty: Arc<Type::NFType>, mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ENUM_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::ENUM_LITERAL; ty = ty.clone());
            exp.clone()
        },
        Deref @ CREF { .. } => {
            assign_variant_field!(exp => NFExpression::CREF; ty = ty.clone());
            exp.clone()
        },
        Deref @ TYPENAME { .. } => {
            assign_variant_field!(exp => NFExpression::TYPENAME; ty = ty.clone());
            exp.clone()
        },
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; ty = ty.clone());
            exp.clone()
        },
        Deref @ RANGE { .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = ty.clone());
            exp.clone()
        },
        Deref @ TUPLE { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE; ty = ty.clone());
            exp.clone()
        },
        Deref @ RECORD { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD; ty = ty.clone());
            exp.clone()
        },
        Deref @ CALL { .. } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::setType(var_field!((*exp).call, NFExpression::CALL).clone(), ty.clone())?);
            exp.clone()
        },
        Deref @ BINARY { .. } => {
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::setType(ty.clone(), var_field!((*exp).operator, NFExpression::BINARY).clone()));
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            assign_variant_field!(exp => NFExpression::UNARY; operator = Operator::setType(ty.clone(), var_field!((*exp).operator, NFExpression::UNARY).clone()));
            exp.clone()
        },
        Deref @ LBINARY { .. } => {
            assign_variant_field!(exp => NFExpression::LBINARY; operator = Operator::setType(ty.clone(), var_field!((*exp).operator, NFExpression::LBINARY).clone()));
            exp.clone()
        },
        Deref @ LUNARY { .. } => {
            assign_variant_field!(exp => NFExpression::LUNARY; operator = Operator::setType(ty.clone(), var_field!((*exp).operator, NFExpression::LUNARY).clone()));
            exp.clone()
        },
        Deref @ RELATION { .. } => {
            assign_variant_field!(exp => NFExpression::RELATION; operator = Operator::setType(ty.clone(), var_field!((*exp).operator, NFExpression::RELATION).clone()));
            exp.clone()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(exp => NFExpression::IF; ty = ty.clone());
            exp.clone()
        },
        Deref @ CAST { .. } => {
            assign_variant_field!(exp => NFExpression::CAST; ty = ty.clone());
            exp.clone()
        },
        Deref @ UNBOX { .. } => {
            assign_variant_field!(exp => NFExpression::UNBOX; ty = ty.clone());
            exp.clone()
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            assign_variant_field!(exp => NFExpression::SUBSCRIPTED_EXP; ty = ty.clone());
            exp.clone()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE_ELEMENT; ty = ty.clone());
            exp.clone()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD_ELEMENT; ty = ty.clone());
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; ty = ty.clone());
            exp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn applyToType(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type typeFunc = fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>>;

    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ENUM_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::ENUM_LITERAL; ty = func(var_field!((*exp).ty, NFExpression::ENUM_LITERAL).clone())?);
            exp.clone()
        },
        Deref @ CREF { .. } => {
            assign_variant_field!(exp => NFExpression::CREF;
                ty = func(var_field!((*exp).ty, NFExpression::CREF).clone())?,
                cref = ComponentRef::applyToType(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())
            );
            exp.clone()
        },
        Deref @ TYPENAME { .. } => {
            assign_variant_field!(exp => NFExpression::TYPENAME; ty = func(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?);
            exp.clone()
        },
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; ty = func(var_field!((*exp).ty, NFExpression::ARRAY).clone())?);
            exp.clone()
        },
        Deref @ RANGE { .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = func(var_field!((*exp).ty, NFExpression::RANGE).clone())?);
            exp.clone()
        },
        Deref @ TUPLE { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE; ty = func(var_field!((*exp).ty, NFExpression::TUPLE).clone())?);
            exp.clone()
        },
        Deref @ RECORD { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD; ty = func(var_field!((*exp).ty, NFExpression::RECORD).clone())?);
            exp.clone()
        },
        Deref @ CALL { .. } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::setType(var_field!((*exp).call, NFExpression::CALL).clone(), func(Call::typeOf(var_field!((*exp).call, NFExpression::CALL).clone()))?)?);
            exp.clone()
        },
        Deref @ SIZE { .. } => {
            assign_variant_field!(exp => NFExpression::SIZE; exp = applyToType(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?);
            exp.clone()
        },
        Deref @ MULTARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::MULTARY; operator = o.clone());
            exp.clone()
        },
        Deref @ BINARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::BINARY; operator = o.clone());
            exp.clone()
        },
        Deref @ UNARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::UNARY; operator = o.clone());
            exp.clone()
        },
        Deref @ LBINARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::LBINARY; operator = o.clone());
            exp.clone()
        },
        Deref @ LUNARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::LUNARY; operator = o.clone());
            exp.clone()
        },
        Deref @ RELATION { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::RELATION; operator = o.clone());
            exp.clone()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(exp => NFExpression::IF; ty = func(var_field!((*exp).ty, NFExpression::IF).clone())?);
            exp.clone()
        },
        Deref @ CAST { .. } => {
            assign_variant_field!(exp => NFExpression::CAST; ty = func(var_field!((*exp).ty, NFExpression::CAST).clone())?);
            exp.clone()
        },
        Deref @ BOX { .. } => {
            assign_variant_field!(exp => NFExpression::BOX; exp = applyToType(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?);
            exp.clone()
        },
        Deref @ UNBOX { .. } => {
            assign_variant_field!(exp => NFExpression::UNBOX; ty = func(var_field!((*exp).ty, NFExpression::UNBOX).clone())?);
            exp.clone()
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            assign_variant_field!(exp => NFExpression::SUBSCRIPTED_EXP; ty = func(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone())?);
            exp.clone()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE_ELEMENT; ty = func(var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone())?);
            exp.clone()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD_ELEMENT; ty = func(var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone())?);
            exp.clone()
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), applyToType(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?);
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = applyToType(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?);
            exp.clone()
        },
        Deref @ EMPTY { .. } => {
            assign_variant_field!(exp => NFExpression::EMPTY; ty = func(var_field!((*exp).ty, NFExpression::EMPTY).clone())?);
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; ty = func(var_field!((*exp).ty, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?);
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn typeCastOpt(mut exp: Option<Arc<NFExpression>>, mut ty: Arc<Type::NFType>) -> Option<Arc<NFExpression>> {
    let mut outExp: Option<Arc<NFExpression>> = Util::applyOption(exp.clone(), Arc::new({ let __pe_b1 = ty.clone(); move |__pe_a0| typeCast(__pe_a0, __pe_b1.clone()) }));
    outExp
}

pub fn typeCast(mut exp: Arc<NFExpression>, mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut t2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ety: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut el: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut dim_diff: i32 = 0;
    let mut arr: metamodelica::Array<Arc<NFExpression>>;
    ety = Type::arrayElementType(ty.clone());
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => if (Type::isReal(ety.clone())) {Arc::new(NFExpression::REAL { value: intReal(var_field!((*exp).value, NFExpression::INTEGER).clone()) })} else if (Type::isEnumeration(ety.clone()) && Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdIntegersAsEnumeration")).clone())?) {Arc::new(NFExpression::ENUM_LITERAL { ty: ety.clone(), name: (Type::nthEnumLiteral(ety.clone(), var_field!((*exp).value, NFExpression::INTEGER).clone())?).clone(), index: var_field!((*exp).value, NFExpression::INTEGER).clone() })} else {typeCastGeneric(exp.clone(), ety.clone())},
        Deref @ ENUM_LITERAL { .. } if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdEnumerationAsIntegers")).clone())?) => if (Type::isInteger(ety.clone())) {Arc::new(NFExpression::INTEGER { value: toInteger(exp.clone())? })} else {typeCastGeneric(exp.clone(), ety.clone())},
        Deref @ BOOLEAN { .. } => if (Type::isReal(ety.clone()) && Flags::isSet(Flags::NF_API.clone())?) {Arc::new(NFExpression::REAL { value: if (var_field!((*exp).value, NFExpression::BOOLEAN).clone()) {metamodelica::OrderedFloat(1.0_f64)} else {metamodelica::OrderedFloat(0.0_f64)} })} else {typeCastGeneric(exp.clone(), ety.clone())},
        Deref @ REAL { .. } => if (Type::isReal(ety.clone())) {exp.clone()} else {typeCastGeneric(exp.clone(), ety.clone())},
        Deref @ ARRAY { elements: arr, ty: t, .. } => {
            let mut arr = (*arr).clone();
            let mut t = (*t).clone();
            arr = Array::map(arr.clone(), Arc::new({ let __pe_b1 = ety.clone(); move |__pe_a0| typeCast(__pe_a0, __pe_b1.clone()) }));
            t = Type::setArrayElementType(t.clone(), ety.clone());
            makeArray(t.clone(), arr.clone(), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ RANGE { ty: t, .. } => {
            let mut t = (*t).clone();
            t = Type::setArrayElementType(t.clone(), ety.clone());
            Arc::new(NFExpression::RANGE { ty: t.clone(), start: typeCast(var_field!((*exp).start, NFExpression::RANGE).clone(), ety.clone())?, step: typeCastOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), ety.clone()), stop: typeCast(var_field!((*exp).stop, NFExpression::RANGE).clone(), ety.clone())? })
        },
        Deref @ UNARY { .. } => {
            t = Type::setArrayElementType(Operator::typeOf(var_field!((*exp).operator, NFExpression::UNARY).clone()), ety.clone());
            Arc::new(NFExpression::UNARY { operator: Operator::setType(t.clone(), var_field!((*exp).operator, NFExpression::UNARY).clone()), exp: typeCast(var_field!((*exp).exp, NFExpression::UNARY).clone(), ety.clone())? })
        },
        Deref @ IF { .. } => {
            e1 = typeCast(var_field!((*exp).trueBranch, NFExpression::IF).clone(), ety.clone())?;
            e2 = typeCast(var_field!((*exp).falseBranch, NFExpression::IF).clone(), ety.clone())?;
            t = if (Type::isConditionalArray(ty.clone())) {Type::setConditionalArrayTypes(ty.clone(), typeOf(e1.clone()), typeOf(e2.clone()))?} else {typeOf(e1.clone())};
            Arc::new(NFExpression::IF { ty: t.clone(), condition: var_field!((*exp).condition, NFExpression::IF).clone(), trueBranch: e1.clone(), falseBranch: e2.clone() })
        },
        Deref @ CALL { .. } => Call::typeCast(exp.clone(), ety.clone())?,
        Deref @ CAST { .. } => typeCast(var_field!((*exp).exp, NFExpression::CAST).clone(), ty.clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => {
            e1 = typeCast(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), ety.clone())?;
            t = Type::setArrayElementType(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), ety.clone());
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e1.clone(), subscripts: var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), ty: t.clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        _ => typeCastGeneric(exp.clone(), ety.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn typeCastGeneric(mut exp: Arc<NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = exp;
    let mut exp_ty: Arc<Type::NFType> = typeOf(exp.clone());
    if !(Type::isEqual(ty.clone(), Type::arrayElementType(exp_ty.clone()))) {
        exp = Arc::new(NFExpression::CAST { ty: Type::setArrayElementType(exp_ty.clone(), ty.clone()), exp: exp.clone() });
    }
    exp
}

pub fn realValue(mut exp: Arc<NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone(),
        Deref @ INTEGER { .. } => intReal(var_field!((*exp).value, NFExpression::INTEGER).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

pub fn makeReal(mut value: metamodelica::Real) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::REAL { value: value.clone() });
    exp
}

pub fn integerValue(mut exp: Arc<NFExpression>) -> Result<i32> {
    let mut value: i32 = 0;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ INTEGER { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        value = __pa1.clone();
        Ok::<_, anyhow::Error>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.integerValue")); __mm_s.push_str(&*literal!(" failed because expression is not an integer:\n")); __mm_s.push_str(&*toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    Ok(value)
}

pub fn integerValueOrDefault(mut exp: Arc<NFExpression>, mut value: i32) -> i32 {
    let mut value: i32 = value;
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone(),
        _ => value.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

pub fn makeInteger(mut value: i32) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::INTEGER { value: value.clone() });
    exp
}

pub fn stringValue(mut exp: Arc<NFExpression>) -> ArcStr {
    let mut value: ArcStr = arcstr::literal!("");
    value = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ STRING { .. } => var_field!((*exp).value, NFExpression::STRING).clone(),
        Deref @ FILENAME { .. } => var_field!((*exp).filename, NFExpression::FILENAME).clone(),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    value
}

pub fn booleanValue(mut exp: Arc<NFExpression>) -> bool {
    let mut value: bool = false;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ BOOLEAN { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        value = __pa1.clone();
        Ok::<_, anyhow::Error>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(_) => {
            value = false;
        }
    }
    value
}

pub fn makeArray(mut ty: Arc<Type::NFType>, mut expl: metamodelica::Array<Arc<NFExpression>>, mut literal: bool) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = Arc::new(NFExpression::ARRAY { ty: ty.clone(), elements: expl.clone(), literal: literal.clone() });
    outExp
}

pub fn makeArrayCheckLiteral(mut ty: Arc<Type::NFType>, mut expl: metamodelica::Array<Arc<NFExpression>>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = Arc::new(NFExpression::ARRAY { ty: ty.clone(), elements: expl.clone(), literal: Array::all(expl.clone(), Arc::new(fnptr!(isLiteral, Arc<NFExpression>))) });
    outExp
}

pub fn makeEmptyArray(mut ty: Arc<Type::NFType>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = Arc::new(NFExpression::ARRAY { ty: ty.clone(), elements: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), literal: true });
    outExp
}

pub fn makeIntegerArray(mut values: Arc<metamodelica::List<i32>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = makeArray(Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![Dimension::fromInteger((values.clone().len() as i32), Prefixes::Variability::CONSTANT.clone())] }), Array::mapList(values.clone(), Arc::new(fnptr!(makeInteger, i32)))?, true);
    Ok(exp)
}

pub fn makeRealArray(mut values: Arc<metamodelica::List<metamodelica::Real>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = makeArray(Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::REAL), dimensions: list![Dimension::fromInteger((values.clone().len() as i32), Prefixes::Variability::CONSTANT.clone())] }), Array::mapList(values.clone(), Arc::new(fnptr!(makeReal, metamodelica::Real)))?, true);
    Ok(exp)
}

pub fn makeRealMatrix(mut values: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    if values.clone().is_empty() {
        ty = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::REAL), dimensions: list![Dimension::fromInteger(0, Prefixes::Variability::CONSTANT.clone()), Arc::new(crate::NFDimension::UNKNOWN)] });
        exp = makeEmptyArray(ty.clone());
    } else {
        ty = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::REAL), dimensions: list![Dimension::fromInteger((listHead(values.clone())?.len() as i32), Prefixes::Variability::CONSTANT.clone())] });
        expl = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut row in (values.clone()).into_iter().cloned() {
            let __x = makeArray(ty.clone(), metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut v in (row.clone()).into_iter().cloned() {
            let __x = Arc::new(NFExpression::REAL { value: v.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        ty = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger((expl.clone().len() as i32), Prefixes::Variability::CONSTANT.clone()));
        exp = makeArray(ty.clone(), metamodelica::arrayFromVec(expl.clone().into_iter().cloned().collect()), true);
    }
    Ok(exp)
}

pub fn makeExpArray(mut elements: metamodelica::Array<Arc<NFExpression>>, mut elementType: Arc<Type::NFType>, mut isLiteral: bool) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = Type::liftArrayLeft(elementType.clone(), Dimension::fromInteger((elements.clone().borrow().len() as i32), Prefixes::Variability::CONSTANT.clone()));
    exp = makeArray(ty.clone(), elements.clone(), isLiteral.clone());
    exp
}

pub fn makeRecord(mut recordName: Arc<Path>, mut recordType: Arc<Type::NFType>, mut fields: Arc<metamodelica::List<Arc<NFExpression>>>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = Arc::new(NFExpression::RECORD { path: recordName.clone(), ty: recordType.clone(), elements: fields.clone() });
    exp
}

pub fn makeRange(mut start: Arc<NFExpression>, mut step: Option<Arc<NFExpression>>, mut stop: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut rangeExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    rangeExp = Arc::new(NFExpression::RANGE { ty: TypeCheck::getRangeType(start.clone(), step.clone(), stop.clone(), typeOf(start.clone()), Absyn::dummyInfo.clone())?, start: start.clone(), step: step.clone(), stop: stop.clone() });
    Ok(rangeExp)
}

pub fn makeIntegerRange(mut start: i32, mut step: i32, mut stop: i32) -> Result<Arc<NFExpression>> {
    let mut rangeExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut start_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut stop_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut step_exp: Option<Arc<NFExpression>> = None;
    start_exp = Arc::new(NFExpression::INTEGER { value: start.clone() });
    stop_exp = Arc::new(NFExpression::INTEGER { value: stop.clone() });
    if start.clone() == stop.clone() || step.clone() == 1 && start.clone() <= stop.clone() || step.clone() == -1 && start.clone() >= stop.clone() {
        step_exp = None;
    } else {
        step_exp = Some(Arc::new(NFExpression::INTEGER { value: step.clone() }));
    }
    rangeExp = makeRange(start_exp.clone(), step_exp.clone(), stop_exp.clone())?;
    Ok(rangeExp)
}

pub fn getIntegerRange(mut range: Arc<NFExpression>, mut resize: bool) -> Result<(i32, i32, i32)> {
    let mut start: i32 = 0;
    let mut step: i32 = 0;
    let mut stop: i32 = 0;
    (start, step, stop) = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ RANGE { .. } => {
            match '__try0: {
                start = unwrap_break_err!(getInteger(var_field!((*range).start, NFExpression::RANGE).clone(), resize.clone()), '__try0);
                stop = unwrap_break_err!(getInteger(var_field!((*range).stop, NFExpression::RANGE).clone(), resize.clone()), '__try0);
                if isSome(var_field!((*range).step, NFExpression::RANGE).clone()) {
                    step = unwrap_break_err!(getInteger(Util::getOption(var_field!((*range).step, NFExpression::RANGE).clone())?, resize.clone()), '__try0);
                } else {
                    step = if (start.clone() > stop.clone()) {-1} else {1};
                }
                Ok::<_, anyhow::Error>((start.clone(), step.clone(), stop.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    start = __try0_o0;
                    step = __try0_o1;
                    stop = __try0_o2;
                }
                Err(_) => {
                    Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.getIntegerRange")); __mm_s.push_str(&*literal!(" range could not be parsed to integer values: ")); __mm_s.push_str(&*toString(range.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    bail!("fail");
                }
            }
            (start.clone(), step.clone(), stop.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.getIntegerRange")); __mm_s.push_str(&*literal!(" expression not RANGE(): ")); __mm_s.push_str(&*toString(range.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start, step, stop))
}

pub fn getInteger(mut exp: Arc<NFExpression>, mut resize: bool) -> Result<i32> {
    let mut i: i32 = 0;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    if resize.clone() {
        e = map(exp.clone(), Arc::new(replaceResizableParameter))?;
    } else {
        e = map(exp.clone(), Arc::new(replaceResizableParameterWithOriginal))?;
    }
    i = (::match_deref::match_deref! { match &(SimplifyExp::simplify(e.clone(), false)?) {
        Deref @ INTEGER { value: i } => i.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.getInteger")); __mm_s.push_str(&*literal!(" cannot be parsed to an integer: ")); __mm_s.push_str(&*toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

pub fn makeTuple(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<NFExpression>> {
    let mut tupleExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut tyl: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    if (expl.clone().len() as i32) == 1 {
        tupleExp = listHead(expl.clone())?;
    } else {
        tyl = {
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = typeOf(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        tupleExp = Arc::new(NFExpression::TUPLE { ty: Arc::new(Type::NFType::TUPLE { types: tyl.clone(), names: None }), elements: expl.clone() });
    }
    Ok(tupleExp)
}

pub fn rangeSize(mut range: Arc<NFExpression>, mut resize: bool) -> i32 {
    let mut size: i32 = Dimension::size(Type::nthDimension(typeOf(range.clone()), 1).unwrap(), resize.clone()).unwrap();
    size
}

pub fn rangeSizeExp(mut range: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut size: Arc<NFExpression> = Dimension::sizeExp(Type::nthDimension(typeOf(range.clone()), 1).unwrap()).unwrap();
    size
}

pub fn applySubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut exp: Arc<NFExpression>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    if subscripts.clone().is_empty() {
        outExp = exp.clone();
    } else {
        outExp = applySubscript(listHead(subscripts.clone())?, exp.clone(), listRest(subscripts.clone())?, applyToScope.clone())?;
    }
    Ok(outExp)
}

pub fn applySubscript(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => applySubscriptCref(subscript.clone(), var_field!((*exp).cref, NFExpression::CREF).clone(), restSubscripts.clone(), applyToScope.clone())?,
        Deref @ TYPENAME { .. } if (restSubscripts.clone().is_empty()) => applySubscriptTypename(subscript.clone(), var_field!((*exp).ty, NFExpression::TYPENAME).clone())?,
        Deref @ ARRAY { .. } => applySubscriptArray(subscript.clone(), exp.clone(), restSubscripts.clone(), applyToScope.clone())?,
        Deref @ RANGE { .. } if (restSubscripts.clone().is_empty()) => applySubscriptRange(subscript.clone(), exp.clone())?,
        Deref @ CALL { .. } => applySubscriptCall(subscript.clone(), exp.clone(), restSubscripts.clone(), applyToScope.clone())?,
        Deref @ IF { .. } => applySubscriptIf(subscript.clone(), exp.clone(), restSubscripts.clone(), applyToScope.clone())?,
        Deref @ UNBOX { .. } => {
            outExp = applySubscript(subscript.clone(), var_field!((*exp).exp, NFExpression::UNBOX).clone(), restSubscripts.clone(), applyToScope.clone())?;
            unbox(outExp.clone())
        },
        Deref @ BOX { .. } => r#box(applySubscript(subscript.clone(), var_field!((*exp).exp, NFExpression::BOX).clone(), restSubscripts.clone(), applyToScope.clone())?),
        Deref @ CAST { .. } => {
            outExp = applySubscript(subscript.clone(), var_field!((*exp).exp, NFExpression::CAST).clone(), restSubscripts.clone(), applyToScope.clone())?;
            Arc::new(NFExpression::CAST { ty: Type::copyElementType(typeOf(outExp.clone()), var_field!((*exp).ty, NFExpression::CAST).clone()), exp: outExp.clone() })
        },
        _ => makeSubscriptedExp(cons(subscript.clone(), restSubscripts.clone()), exp.clone(), false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn applySubscriptCref(mut subscript: Arc<Subscript::NFSubscript>, mut cref: Arc<ComponentRef::NFComponentRef>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    cr = ComponentRef::mergeSubscripts(cons(subscript.clone(), restSubscripts.clone()), cref.clone(), applyToScope.clone(), false, false)?;
    ty = ComponentRef::getSubscriptedType(cr.clone(), false)?;
    outExp = Arc::new(NFExpression::CREF { ty: ty.clone(), cref: cr.clone() });
    Ok(outExp)
}

pub fn applySubscriptTypename(mut subscript: Arc<Subscript::NFSubscript>, mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut index: i32 = 0;
    let mut expl: metamodelica::Array<Arc<NFExpression>>;
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::INDEX { .. } => applyIndexSubscriptTypename(ty.clone(), sub.clone())?,
        Deref @ Subscript::SLICE { .. } => Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: Arc::new(NFExpression::TYPENAME { ty: ty.clone() }), subscripts: list![subscript.clone()], ty: Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![Subscript::toDimension(sub.clone())?] }), split: false }),
        Deref @ Subscript::WHOLE => Arc::new(NFExpression::TYPENAME { ty: ty.clone() }),
        Deref @ Subscript::EXPANDED_SLICE { .. } => {
            expl = Array::mapList(var_field!((*sub).indices, Subscript::NFSubscript::EXPANDED_SLICE).clone(), Arc::new({ let __pe_b0 = ty.clone(); move |__pe_a1| applyIndexSubscriptTypename(__pe_b0.clone(), __pe_a1) }))?;
            makeArray(Type::liftArrayLeft(ty.clone(), Dimension::fromInteger((expl.clone().borrow().len() as i32), Prefixes::Variability::CONSTANT.clone())), expl.clone(), true)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn applyIndexSubscriptTypename(mut ty: Arc<Type::NFType>, mut index: Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> {
    let mut subscriptedExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut idx_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut idx: i32 = 0;
    idx_exp = Subscript::toExp(index.clone())?;
    if isScalarLiteral(idx_exp.clone()) {
        idx = toInteger(idx_exp.clone())?;
        subscriptedExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::BOOLEAN if (idx.clone() <= 2) => if (idx.clone() == 1) {Arc::new(NFExpression::BOOLEAN { value: false })} else {Arc::new(NFExpression::BOOLEAN { value: true })},
        Deref @ Type::ENUMERATION { .. } => nthEnumLiteral(ty.clone(), idx.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    } else {
        subscriptedExp = Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: Arc::new(NFExpression::TYPENAME { ty: ty.clone() }), subscripts: list![index.clone()], ty: ty.clone(), split: false });
    }
    Ok(subscriptedExp)
}

pub fn applySubscriptArray(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut s: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut rest_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut expl: metamodelica::Array<Arc<NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut el_count: i32 = 0;
    let mut literal: bool = false;
    let mut first_e: Arc<NFExpression> = Arc::new(NFExpression::END);
    if isEmptyArray(exp.clone()) {
        outExp = makeSubscriptedExp(cons(subscript.clone(), restSubscripts.clone()), exp.clone(), false)?;
        return Ok(outExp);
    }
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::INDEX { .. } => applyIndexSubscriptArray(exp.clone(), sub.clone(), restSubscripts.clone())?,
        Deref @ Subscript::SLICE { .. } => makeSubscriptedExp(cons(subscript.clone(), restSubscripts.clone()), exp.clone(), false)?,
        Deref @ Subscript::WHOLE => {
            if restSubscripts.clone().is_empty() {
                outExp = exp.clone();
            } else {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
                    Deref @ ARRAY { literal: __pa0, elements: __pa1, ty: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                literal = __pa0.clone();
                expl = __pa1.clone();
                ty = __pa2.clone();
                let (__pa3, __pa4) = ::match_deref::match_deref! { match &(restSubscripts.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                s = __pa3.clone();
                rest_subs = __pa4.clone();
                expl = Array::map(expl.clone(), Arc::new({ let __pe_b0 = s.clone(); let __pe_b2 = rest_subs.clone(); let __pe_b3 = applyToScope.clone(); move |__pe_a1| applySubscript(__pe_b0.clone(), __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }));
                (ty, literal) = typeSubscriptedArray(expl.clone(), restSubscripts.clone(), ty.clone(), literal.clone())?;
                outExp = makeArray(ty.clone(), expl.clone(), literal.clone());
            }
            outExp.clone()
        },
        Deref @ Subscript::EXPANDED_SLICE { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ ARRAY { literal: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            literal = __pa0.clone();
            ty = __pa1.clone();
            expl = Array::mapList(var_field!((*sub).indices, Subscript::NFSubscript::EXPANDED_SLICE).clone(), Arc::new({ let __pe_b0 = exp.clone(); let __pe_b2 = restSubscripts.clone(); move |__pe_a1| applyIndexSubscriptArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) }))?;
            (ty, literal) = typeSubscriptedArray(expl.clone(), restSubscripts.clone(), ty.clone(), literal.clone())?;
            makeArray(ty.clone(), expl.clone(), literal.clone())
        },
        Deref @ Subscript::SPLIT_INDEX { .. } => makeSubscriptedExp(cons(subscript.clone(), restSubscripts.clone()), exp.clone(), false)?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn typeSubscriptedArray(mut elements: metamodelica::Array<Arc<NFExpression>>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut ty: Arc<Type::NFType>, mut literal: bool) -> Result<(Arc<Type::NFType>, bool)> {
    let mut ty: Arc<Type::NFType> = ty;
    let mut literal: bool = literal;
    let mut count: i32 = 0;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    count = (elements.clone().borrow().len() as i32);
    if count.clone() > 0 {
        e = elements.borrow()[(1-1) as usize].clone();
        ty = typeOf(e.clone());
        literal = literal.clone() && isLiteral(e.clone());
    } else {
        ty = Type::subscript(Type::unliftArray(ty.clone())?, subscripts.clone(), true)?;
    }
    ty = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger(count.clone(), Prefixes::Variability::CONSTANT.clone()));
    Ok((ty, literal))
}

pub fn applyIndexSubscriptArray(mut exp: Arc<NFExpression>, mut index: Arc<Subscript::NFSubscript>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = applyIndexExpArray(exp.clone(), Subscript::toExp(index.clone())?, restSubscripts.clone())?;
    Ok(outExp)
}

pub fn applyIndexExpArray(mut exp: Arc<NFExpression>, mut index: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut expl: metamodelica::Array<Arc<NFExpression>>;
    let mut idx: i32 = 0;
    if isScalarLiteral(index.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ ARRAY { elements: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        expl = __pa0.clone();
        idx = toInteger(index.clone())?;
        if idx.clone() > 0 && idx.clone() <= (expl.clone().borrow().len() as i32) {
            outExp = applySubscripts(restSubscripts.clone(), expl.borrow()[(idx.clone()-1) as usize].clone(), false)?;
            return Ok(outExp);
        }
    }
    outExp = makeSubscriptedExp(cons(Arc::new(Subscript::NFSubscript::INDEX { index: index.clone() }), restSubscripts.clone()), exp.clone(), false)?;
    Ok(outExp)
}

pub fn applySubscriptRange(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut start_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut stop_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut step_exp: Option<Arc<NFExpression>> = None;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expl: metamodelica::Array<Arc<NFExpression>>;
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::INDEX { .. } => applyIndexSubscriptRange(exp.clone(), sub.clone())?,
        Deref @ Subscript::SLICE { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            ty = Arc::new(Type::NFType::ARRAY { elementType: Type::unliftArray(ty.clone())?, dimensions: list![Subscript::toDimension(sub.clone())?] });
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: exp.clone(), subscripts: list![subscript.clone()], ty: ty.clone(), split: false })
        },
        Deref @ Subscript::WHOLE => exp.clone(),
        Deref @ Subscript::EXPANDED_SLICE { .. } => {
            expl = Array::mapList(var_field!((*sub).indices, Subscript::NFSubscript::EXPANDED_SLICE).clone(), Arc::new({ let __pe_b0 = exp.clone(); move |__pe_a1| applyIndexSubscriptRange(__pe_b0.clone(), __pe_a1) }))?;
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            makeArray(Type::liftArrayLeft(ty.clone(), Dimension::fromInteger((expl.clone().borrow().len() as i32), Prefixes::Variability::CONSTANT.clone())), expl.clone(), false)
        },
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            ty = Type::unliftArray(ty.clone())?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: exp.clone(), subscripts: list![sub.clone()], ty: ty.clone(), split: true })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.applySubscriptRange")); __mm_s.push_str(&*literal!(" got unknown subscript '")); __mm_s.push_str(&*Subscript::toString(sub.clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn applyIndexSubscriptRange(mut rangeExp: Arc<NFExpression>, mut index: Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut index_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut start_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut stop_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut step_exp: Option<Arc<NFExpression>> = None;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(index.clone()) {
        Deref @ Subscript::INDEX { index: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    index_exp = __pa0.clone();
    if isScalarLiteral(index_exp.clone()) {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(rangeExp.clone()) {
            Deref @ RANGE { stop: __pa1, step: __pa2, start: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        stop_exp = __pa1.clone();
        step_exp = __pa2.clone();
        start_exp = __pa3.clone();
        outExp = applyIndexSubscriptRange2(start_exp.clone(), step_exp.clone(), stop_exp.clone(), toInteger(index_exp.clone())?)?;
    } else {
        let __pa4 = ::match_deref::match_deref! { match &(rangeExp.clone()) {
            Deref @ RANGE { ty: __pa4, .. } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa4.clone();
        subs = list![index.clone()];
        ty = Type::subscript(ty.clone(), subs.clone(), true)?;
        outExp = Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: rangeExp.clone(), subscripts: subs.clone(), ty: ty.clone(), split: false });
    }
    Ok(outExp)
}

pub fn applyIndexSubscriptRange2(mut startExp: Arc<NFExpression>, mut stepExp: Option<Arc<NFExpression>>, mut stopExp: Arc<NFExpression>, mut index: i32) -> Result<Arc<NFExpression>> {
    let mut subscriptedExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut iidx: i32 = 0;
    let mut ridx: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    subscriptedExp = (::match_deref::match_deref! { match &((startExp.clone(), stepExp.clone())) {
        (Deref @ INTEGER { .. }, Some(Deref @ INTEGER { value: iidx })) => Arc::new(NFExpression::INTEGER { value: var_field!((*startExp).value, NFExpression::INTEGER).clone() + (index.clone() - 1) * iidx.clone() }),
        (Deref @ INTEGER { .. }, _) => Arc::new(NFExpression::INTEGER { value: var_field!((*startExp).value, NFExpression::INTEGER).clone() + index.clone() - 1 }),
        (Deref @ REAL { .. }, Some(Deref @ REAL { value: ridx })) => Arc::new(NFExpression::REAL { value: var_field!((*startExp).value, NFExpression::REAL).clone() + (metamodelica::OrderedFloat((index.clone() - 1) as f64)) * ridx.clone() }),
        (Deref @ REAL { .. }, _) => Arc::new(NFExpression::REAL { value: var_field!((*startExp).value, NFExpression::REAL).clone() + metamodelica::OrderedFloat((index.clone()) as f64) - metamodelica::OrderedFloat(1.0_f64) }),
        (Deref @ BOOLEAN { .. }, _) => if (index.clone() == 1) {startExp.clone()} else {stopExp.clone()},
        (Deref @ ENUM_LITERAL { index: iidx, .. }, _) => {
            let mut iidx = (*iidx).clone();
            iidx = iidx.clone() + index.clone() - 1;
            nthEnumLiteral(var_field!((*startExp).ty, NFExpression::ENUM_LITERAL).clone(), iidx.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(subscriptedExp)
}

pub fn applySubscriptCall(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut call: Arc<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { call: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    call = __pa0.clone();
    outExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil }, .. } if (Function::Function::isSubscriptableBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone())?) => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut arg = (*arg).clone();
            arg = applySubscript(subscript.clone(), arg.clone(), restSubscripts.clone(), applyToScope.clone())?;
            ty = Type::copyDims(typeOf(arg.clone()), var_field!((*call).ty, Call::NFCall::TYPED_CALL).clone());
            Arc::new(NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_CALL { r#fn: var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone(), ty: ty.clone(), var: var_field!((*call).var, Call::NFCall::TYPED_CALL).clone(), purity: var_field!((*call).purity, Call::NFCall::TYPED_CALL).clone(), arguments: list![arg.clone()], attributes: var_field!((*call).attributes, Call::NFCall::TYPED_CALL).clone() }) })
        },
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
            applySubscriptArrayConstructor(subscript.clone(), call.clone(), restSubscripts.clone())?
        },
        _ => {
            makeSubscriptedExp(cons(subscript.clone(), restSubscripts.clone()), exp.clone(), false)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn applySubscriptArrayConstructor(mut subscript: Arc<Subscript::NFSubscript>, mut call: Arc<Call::NFCall>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    if Subscript::isIndex(subscript.clone()) && restSubscripts.clone().is_empty() {
        outExp = applyIndexSubscriptArrayConstructor(call.clone(), subscript.clone())?;
    } else {
        outExp = makeSubscriptedExp(cons(subscript.clone(), restSubscripts.clone()), Arc::new(NFExpression::CALL { call: call.clone() }), false)?;
    }
    Ok(outExp)
}

pub fn applyIndexSubscriptArrayConstructor(mut call: Arc<Call::NFCall>, mut index: Arc<Subscript::NFSubscript>) -> Result<Arc<NFExpression>> {
    let mut subscriptedExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut iter_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<NFExpression>)>> = metamodelica::nil();
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { ty: __pa0, var: __pa1, purity: __pa2, exp: __pa3, iters: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    var = __pa1.clone();
    pur = __pa2.clone();
    exp = __pa3.clone();
    iters = __pa4.clone();
    let ((__pa5, __pa6), __pa7) = List::splitLast(iters.clone())?;
    iter = __pa5.clone();
    iter_exp = __pa6.clone();
    iters = __pa7.clone();
    iter_exp = applySubscript(index.clone(), iter_exp.clone(), metamodelica::nil(), false)?;
    subscriptedExp = replaceIterator(exp.clone(), iter.clone(), iter_exp.clone())?;
    if !(iters.clone().is_empty()) {
        subscriptedExp = Arc::new(NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: Type::unliftArray(ty.clone())?, var: var.clone(), purity: pur.clone(), exp: subscriptedExp.clone(), iters: iters.clone() }) });
    }
    Ok(subscriptedExp)
}

pub fn applySubscriptIf(mut subscript: Arc<Subscript::NFSubscript>, mut exp: Arc<NFExpression>, mut restSubscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut applyToScope: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut cond: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut tb: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut fb: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    tb = __pa2.clone();
    fb = __pa3.clone();
    if Type::isConditionalArray(ty.clone()) {
        match '__try4: {
            tb = unwrap_break_err!(applySubscript(subscript.clone(), tb.clone(), restSubscripts.clone(), applyToScope.clone()), '__try4);
            fb = unwrap_break_err!(applySubscript(subscript.clone(), fb.clone(), restSubscripts.clone(), applyToScope.clone()), '__try4);
            ty = unwrap_break_err!(Type::setConditionalArrayTypes(ty.clone(), typeOf(tb.clone()), typeOf(fb.clone())), '__try4);
            outExp = Arc::new(NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: tb.clone(), falseBranch: fb.clone() });
            Ok::<_, anyhow::Error>((outExp.clone(),))
        } {
            Ok((__try4_o0,)) => {
                outExp = __try4_o0;
            }
            Err(_) => {
                outExp = makeSubscriptedExp(cons(subscript.clone(), restSubscripts.clone()), exp.clone(), false)?;
            }
        }
    } else {
        tb = applySubscript(subscript.clone(), tb.clone(), restSubscripts.clone(), applyToScope.clone())?;
        fb = applySubscript(subscript.clone(), fb.clone(), restSubscripts.clone(), applyToScope.clone())?;
        ty = typeOf(tb.clone());
        outExp = Arc::new(NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: tb.clone(), falseBranch: fb.clone() });
    }
    Ok(outExp)
}

pub fn makeSubscriptedExp(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut exp: Arc<NFExpression>, mut backend: bool) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut extra_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dim_count: i32 = 0;
    let mut split: bool = false;
    (e, subs, ty, split) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { .. } => (var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), typeOf(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone()), var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone()),
        _ => (exp.clone(), metamodelica::nil(), typeOf(exp.clone()), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(split.clone()) {
        split = List::any(subscripts.clone(), Arc::new(fnptr!(Subscript::isSplitIndex, Arc<Subscript::NFSubscript>)));
    }
    dim_count = Type::dimensionCount(ty.clone());
    (subs, extra_subs) = Subscript::mergeList(subscripts.clone(), subs.clone(), dim_count.clone(), backend.clone())?;
    if !(extra_subs.clone().is_empty()) {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeSubscriptedExp")); __mm_s.push_str(&*literal!(": too few dimensions in ")); __mm_s.push_str(&*toString(exp.clone())?); __mm_s.push_str(&*literal!(" to apply subscripts ")); __mm_s.push_str(&*Subscript::toStringList(subscripts.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    ty = Type::subscript(ty.clone(), subs.clone(), true)?;
    outExp = Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e.clone(), subscripts: subs.clone(), ty: ty.clone(), split: split.clone() });
    Ok(outExp)
}

pub fn replaceIterator(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut iteratorValue: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = map(exp.clone(), Arc::new({ let __pe_b1 = iterator.clone(); let __pe_b2 = iteratorValue.clone(); move |__pe_a0| replaceIterator2(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    Ok(exp)
}

pub fn replaceIterator2(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>, mut iteratorValue: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node, .. }, .. } if (ComponentRef::isSimple(var_field!((*exp).cref, NFExpression::CREF).clone())) => {
            if (InstNode::refEqual(iterator.clone(), node.clone())) {iteratorValue.clone()} else {exp.clone()}
        },
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut fields: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            node = ComponentRef::node(ComponentRef::last(var_field!((*exp).cref, NFExpression::CREF).clone()))?;
            if InstNode::refEqual(iterator.clone(), node.clone()) {
                outExp = iteratorValue.clone();
                fields = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (listRest(ComponentRef::nodes(var_field!((*exp).cref, NFExpression::CREF).clone(), metamodelica::nil()))?).into_iter().cloned() {
            let __x = InstNode::name(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                for mut f in &*fields.clone() {
                    let mut f = f.clone();
                    outExp = recordElement((f.clone()).clone(), outExp.clone())?;
                }
            } else {
                outExp = exp.clone();
            }
            outExp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn containsIterator(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>) -> Result<bool> {
    fn containsIterator2(mut exp: Arc<NFExpression>, mut iterator: Arc<InstNode::InstNode>) -> bool {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node, .. }, .. } => {
            InstNode::refEqual(node.clone(), iterator.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    let mut res: bool = false;
    res = contains(exp.clone(), Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0| Ok(containsIterator2(__pe_a0, __pe_b1.clone())) }))?;
    Ok(res)
}

pub fn arrayFromList(mut inExps: Arc<metamodelica::List<Arc<NFExpression>>>, mut elemTy: Arc<Type::NFType>, mut inDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = arrayFromList_impl(inExps.clone(), elemTy.clone(), inDims.clone().reverse())?;
    Ok(outExp)
}

pub fn arrayFromList_impl(mut inExps: Arc<metamodelica::List<Arc<NFExpression>>>, mut elemTy: Arc<Type::NFType>, mut inDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut ldim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut restdims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut newlst: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut partexps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
    let mut dimsize: i32 = 0;
    Error::assertion(!(inDims.clone().is_empty()), (literal!("Empty dimension list given in arrayFromList.")).clone(), metamodelica::sourceInfo!())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDims.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ldim = __pa0.clone();
    restdims = __pa1.clone();
    dimsize = Dimension::size(ldim.clone(), false)?;
    ty = Type::liftArrayLeft(elemTy.clone(), ldim.clone());
    if List::hasOneElement(inDims.clone()) {
        Error::assertion(dimsize.clone() == (inExps.clone().len() as i32), (literal!("Length mismatch in arrayFromList.")).clone(), metamodelica::sourceInfo!())?;
        outExp = makeArray(ty.clone(), metamodelica::arrayFromVec(inExps.clone().into_iter().cloned().collect()), false);
        return Ok(outExp);
    }
    partexps = List::partition(inExps.clone(), dimsize.clone())?;
    newlst = metamodelica::nil();
    for mut arrexp in &*partexps.clone() {
        let mut arrexp = arrexp.clone();
        newlst = cons(makeArray(ty.clone(), metamodelica::arrayFromVec(arrexp.clone().into_iter().cloned().collect()), false), newlst.clone());
    }
    newlst = newlst.clone().reverse();
    outExp = arrayFromList_impl(newlst.clone(), ty.clone(), restdims.clone())?;
    Ok(outExp)
}

pub fn makeEnumLiteral(mut enumType: Arc<Type::NFType>, mut index: i32) -> Result<Arc<NFExpression>> {
    let mut literal: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut literals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(enumType.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    literals = __pa0.clone();
    literal = Arc::new(NFExpression::ENUM_LITERAL { ty: enumType.clone(), name: ((literals.clone()).get(index.clone())?).clone(), index: index.clone() });
    Ok(literal)
}

pub fn makeEnumLiterals(mut enumType: Arc<Type::NFType>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut literals: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut lits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(enumType.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lits = __pa0.clone();
    literals = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for (l, i) in (&(lits.clone())).into_iter().zip((1..=(lits.clone().len() as i32)).into_iter()) {
            let __x = Arc::new(NFExpression::ENUM_LITERAL { ty: enumType.clone(), name: (l.clone()).clone(), index: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(literals)
}

pub fn isIntegerValue(mut exp: Arc<NFExpression>, mut value: i32) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() == value.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn toInteger(mut exp: Arc<NFExpression>) -> Result<i32> {
    let mut i: i32 = 0;
    i = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone(),
        Deref @ BOOLEAN { .. } => if (var_field!((*exp).value, NFExpression::BOOLEAN).clone()) {2} else {1},
        Deref @ ENUM_LITERAL { .. } => var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(i)
}

pub fn toStringTyped(mut exp: Arc<NFExpression>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/*")); __mm_s.push_str(&*Type::toString(typeOf(exp.clone()))?); __mm_s.push_str(&*literal!("*/ ")); __mm_s.push_str(&*toString(exp.clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toString(mut exp: Arc<NFExpression>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut first: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut first_inv: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut rest: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut rest_inv: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => intString(var_field!((*exp).value, NFExpression::INTEGER).clone()),
        Deref @ REAL { .. } => realString(var_field!((*exp).value, NFExpression::REAL).clone()),
        Deref @ STRING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*System::escapedString((var_field!((*exp).value, NFExpression::STRING).clone()).clone(), false)); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) },
        Deref @ BOOLEAN { .. } => boolString(var_field!((*exp).value, NFExpression::BOOLEAN).clone()),
        Deref @ ENUM_LITERAL { ty: t @ Deref @ Type::ENUMERATION { .. }, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((**t).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()); ArcStr::from(__mm_s) },
        Deref @ CLKCONST { .. } => ClockKind::toString(var_field!((*exp).clk, NFExpression::CLKCONST).clone())?,
        Deref @ CREF { .. } => ComponentRef::toString(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        Deref @ TYPENAME { .. } => Type::typenameString(Type::arrayElementType(var_field!((*exp).ty, NFExpression::TYPENAME).clone()))?,
        Deref @ ARRAY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) },
        Deref @ MATRIX { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut el in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("; ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) },
        Deref @ RANGE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).start, NFExpression::RANGE).clone(), exp.clone(), false)?); __mm_s.push_str(&*if (isSome(var_field!((*exp).step, NFExpression::RANGE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandString(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, exp.clone(), false)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandString(var_field!((*exp).stop, NFExpression::RANGE).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ TUPLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ RECORD { .. } => List::toString(var_field!((*exp).elements, NFExpression::RECORD).clone(), Arc::new(toString), (AbsynUtil::pathString(var_field!((*exp).path, NFExpression::RECORD).clone(), (literal!(".")).clone(), true, false)?).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?,
        Deref @ CALL { .. } => Call::toString(var_field!((*exp).call, NFExpression::CALL).clone())?,
        Deref @ SIZE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SIZE).clone())?); __mm_s.push_str(&*if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ END { .. } => literal!("end"),
        Deref @ MULTARY { .. } if (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone().is_empty()) => multaryString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), false)?,
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty() && Operator::isDashClassification(Operator::getMathClassification(var_field!((*exp).operator, NFExpression::MULTARY).clone())?)) => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*multaryString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); ArcStr::from(__mm_s) },
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty()) => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1/")); __mm_s.push_str(&*multaryString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); ArcStr::from(__mm_s) },
        Deref @ MULTARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*multaryString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); __mm_s.push_str(&*Operator::symbol(Operator::invert(var_field!((*exp).operator, NFExpression::MULTARY).clone())?, (literal!(" ")).clone())?); __mm_s.push_str(&*multaryString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), true)?); ArcStr::from(__mm_s) },
        Deref @ BINARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).exp1, NFExpression::BINARY).clone(), exp.clone(), true)?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::BINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp2, NFExpression::BINARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ UNARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::UNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp, NFExpression::UNARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ LBINARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), exp.clone(), true)?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LBINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ LUNARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LUNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*operandString(var_field!((*exp).exp, NFExpression::LUNARY).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ RELATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(var_field!((*exp).exp1, NFExpression::RELATION).clone(), exp.clone(), true)?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::RELATION).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandString(var_field!((*exp).exp2, NFExpression::RELATION).clone(), exp.clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ IF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*toString(var_field!((*exp).condition, NFExpression::IF).clone())?); __mm_s.push_str(&*literal!(" then ")); __mm_s.push_str(&*toString(var_field!((*exp).trueBranch, NFExpression::IF).clone())?); __mm_s.push_str(&*literal!(" else ")); __mm_s.push_str(&*toString(var_field!((*exp).falseBranch, NFExpression::IF).clone())?); ArcStr::from(__mm_s) },
        Deref @ CAST { .. } => if (Flags::isSet(Flags::NF_API.clone())?) {toString(var_field!((*exp).exp, NFExpression::CAST).clone())?} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CAST(")); __mm_s.push_str(&*Type::toString(var_field!((*exp).ty, NFExpression::CAST).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::CAST).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }},
        Deref @ BOX { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BOX(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::BOX).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ UNBOX { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNBOX(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::UNBOX).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ SUBSCRIPTED_EXP { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*Subscript::toStringList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?); ArcStr::from(__mm_s) },
        Deref @ TUPLE_ELEMENT { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*toString(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) },
        Deref @ RECORD_ELEMENT { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toString(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?); __mm_s.push_str(&*literal!(").")); __mm_s.push_str(&*var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()); ArcStr::from(__mm_s) },
        Deref @ MUTABLE { .. } => toString(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("LITERAL(")); __mm_s.push_str(&*intString(var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ EMPTY { .. } => literal!("#EMPTY#"),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for (a, n) in (&(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())).into_iter().zip((&(var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())).into_iter()) {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*toString(a.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ FILENAME { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*System::escapedString((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone(), false)); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) },
        Deref @ INSTANCE_NAME { .. } => literal!("getInstanceName()"),
        _ => anyString(exp.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toFlatString(mut exp: Arc<NFExpression>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut first: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut rest: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => intString(var_field!((*exp).value, NFExpression::INTEGER).clone()),
        Deref @ REAL { .. } => realString(var_field!((*exp).value, NFExpression::REAL).clone()),
        Deref @ STRING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Util::escapeModelicaStringToCString((var_field!((*exp).value, NFExpression::STRING).clone()).clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) },
        Deref @ BOOLEAN { .. } => boolString(var_field!((*exp).value, NFExpression::BOOLEAN).clone()),
        Deref @ ENUM_LITERAL { ty: t @ Deref @ Type::ENUMERATION { .. }, .. } => if (Type::isBuiltinEnumeration(t.clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((**t).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::makeQuotedIdentifier((AbsynUtil::pathString(var_field!((**t).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?).clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Util::makeQuotedIdentifier((var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()).clone())?); ArcStr::from(__mm_s) }},
        Deref @ CLKCONST { .. } => ClockKind::toFlatString(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), format.clone())?,
        Deref @ CREF { .. } => ComponentRef::toFlatString(var_field!((*exp).cref, NFExpression::CREF).clone(), format.clone())?,
        Deref @ TYPENAME { .. } => Type::typenameString(Type::arrayElementType(var_field!((*exp).ty, NFExpression::TYPENAME).clone()))?,
        Deref @ ARRAY { .. } => if (var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("fill(")); __mm_s.push_str(&*toFlatString(makeDefaultValue(Type::elementType(var_field!((*exp).ty, NFExpression::ARRAY).clone()), None, None)?, format.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Type::dimensionsToFlatString(var_field!((*exp).ty, NFExpression::ARRAY).clone(), format.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toFlatString(e.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }},
        Deref @ MATRIX { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut el in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = toFlatString(e.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("; ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) },
        Deref @ RANGE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).start, NFExpression::RANGE).clone(), exp.clone(), false, format.clone())?); __mm_s.push_str(&*if (isSome(var_field!((*exp).step, NFExpression::RANGE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandFlatString(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandFlatString(var_field!((*exp).stop, NFExpression::RANGE).clone(), exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) },
        Deref @ TUPLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toFlatString(e.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ RECORD { .. } => List::toString(var_field!((*exp).elements, NFExpression::RECORD).clone(), Arc::new({ let __pe_b1 = format.clone(); move |__pe_a0| toFlatString(__pe_a0, __pe_b1.clone()) }), (Type::toFlatString(var_field!((*exp).ty, NFExpression::RECORD).clone(), format.clone())?).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?,
        Deref @ CALL { .. } => Call::toFlatString(var_field!((*exp).call, NFExpression::CALL).clone(), format.clone())?,
        Deref @ SIZE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*toFlatString(var_field!((*exp).exp, NFExpression::SIZE).clone(), format.clone())?); __mm_s.push_str(&*if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toFlatString(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?, format.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ END { .. } => literal!("end"),
        Deref @ MULTARY { .. } if (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone().is_empty()) => multaryFlatString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format.clone(), false)?,
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty() && Operator::isDashClassification(Operator::getMathClassification(var_field!((*exp).operator, NFExpression::MULTARY).clone())?)) => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format.clone(), true)?); ArcStr::from(__mm_s) },
        Deref @ MULTARY { .. } if (var_field!((*exp).arguments, NFExpression::MULTARY).clone().is_empty()) => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1/")); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format.clone(), true)?); ArcStr::from(__mm_s) },
        Deref @ MULTARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format.clone(), true)?); __mm_s.push_str(&*Operator::symbol(Operator::invert(var_field!((*exp).operator, NFExpression::MULTARY).clone())?, (literal!(" ")).clone())?); __mm_s.push_str(&*multaryFlatString(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), exp.clone(), var_field!((*exp).operator, NFExpression::MULTARY).clone(), format.clone(), true)?); ArcStr::from(__mm_s) },
        Deref @ BINARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp1, NFExpression::BINARY).clone(), exp.clone(), true, format.clone())?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::BINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp2, NFExpression::BINARY).clone(), exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) },
        Deref @ UNARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::UNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp, NFExpression::UNARY).clone(), exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) },
        Deref @ LBINARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), exp.clone(), true, format.clone())?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LBINARY).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) },
        Deref @ LUNARY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::LUNARY).clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp, NFExpression::LUNARY).clone(), exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) },
        Deref @ RELATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp1, NFExpression::RELATION).clone(), exp.clone(), true, format.clone())?); __mm_s.push_str(&*Operator::symbol(var_field!((*exp).operator, NFExpression::RELATION).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*operandFlatString(var_field!((*exp).exp2, NFExpression::RELATION).clone(), exp.clone(), false, format.clone())?); ArcStr::from(__mm_s) },
        Deref @ IF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*toFlatString(var_field!((*exp).condition, NFExpression::IF).clone(), format.clone())?); __mm_s.push_str(&*literal!(" then ")); __mm_s.push_str(&*toFlatString(var_field!((*exp).trueBranch, NFExpression::IF).clone(), format.clone())?); __mm_s.push_str(&*literal!(" else ")); __mm_s.push_str(&*toFlatString(var_field!((*exp).falseBranch, NFExpression::IF).clone(), format.clone())?); ArcStr::from(__mm_s) },
        Deref @ CAST { .. } => toFlatString(var_field!((*exp).exp, NFExpression::CAST).clone(), format.clone())?,
        Deref @ UNBOX { .. } => toFlatString(var_field!((*exp).exp, NFExpression::UNBOX).clone(), format.clone())?,
        Deref @ BOX { .. } => toFlatString(var_field!((*exp).exp, NFExpression::BOX).clone(), format.clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toFlatString(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), format.clone())?); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*Subscript::toFlatStringList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), format.clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ TUPLE_ELEMENT { .. } => toFlatString(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), format.clone())?,
        Deref @ RECORD_ELEMENT { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toFlatString(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), format.clone())?); __mm_s.push_str(&*literal!(").")); __mm_s.push_str(&*var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()); ArcStr::from(__mm_s) },
        Deref @ MUTABLE { .. } => toFlatString(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), format.clone())?,
        Deref @ SHARED_LITERAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[literal: ")); __mm_s.push_str(&*intString(var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) },
        Deref @ EMPTY { .. } => literal!("#EMPTY#"),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ")); __mm_s.push_str(&*ComponentRef::toFlatString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), format.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for (a, n) in (&(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())).into_iter().zip((&(var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())).into_iter()) {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*n.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*toFlatString(a.clone(), format.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ FILENAME { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Util::escapeModelicaStringToCString((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) },
        Deref @ INSTANCE_NAME { .. } => literal!("getInstanceName()"),
        _ => anyString(exp.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn operandString(mut operand: Arc<NFExpression>, mut operator: Arc<NFExpression>, mut lhs: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut operand_prio: i32 = 0;
    let mut operator_prio: i32 = 0;
    let mut parenthesize: bool = false;
    r#str = (toString(operand.clone())?).clone();
    operand_prio = priority(operand.clone(), lhs.clone());
    if operand_prio.clone() == 4 {
        parenthesize = true;
    } else {
        operator_prio = priority(operator.clone(), lhs.clone());
        if operand_prio.clone() > operator_prio.clone() {
            parenthesize = true;
        } else if operand_prio.clone() == operator_prio.clone() {
            parenthesize = if (lhs.clone()) {isNonAssociativeExp(operand.clone())} else {!(isAssociativeExp(operand.clone()))};
        }
    }
    if parenthesize.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn operandFlatString(mut operand: Arc<NFExpression>, mut operator: Arc<NFExpression>, mut lhs: bool, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut operand_prio: i32 = 0;
    let mut operator_prio: i32 = 0;
    let mut parenthesize: bool = false;
    r#str = (toFlatString(operand.clone(), format.clone())?).clone();
    operand_prio = priority(operand.clone(), lhs.clone());
    if operand_prio.clone() == 4 {
        parenthesize = true;
    } else {
        operator_prio = priority(operator.clone(), lhs.clone());
        if operand_prio.clone() > operator_prio.clone() {
            parenthesize = true;
        } else if operand_prio.clone() == operator_prio.clone() {
            parenthesize = if (lhs.clone()) {isNonAssociativeExp(operand.clone())} else {!(isAssociativeExp(operand.clone()))};
        }
    }
    if parenthesize.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn multaryString(mut arguments: Arc<metamodelica::List<Arc<NFExpression>>>, mut exp: Arc<NFExpression>, mut operator: Arc<Operator::NFOperator>, mut parenthesize: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (arguments.clone()).into_iter().cloned() {
            let __x = operandString(e.clone(), exp.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (Operator::symbol(operator.clone(), (literal!(" ")).clone())?).clone());
    if parenthesize.clone() && (arguments.clone().len() as i32) > 1 {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn multaryFlatString(mut arguments: Arc<metamodelica::List<Arc<NFExpression>>>, mut exp: Arc<NFExpression>, mut operator: Arc<Operator::NFOperator>, mut format: BaseModelica::OutputFormat, mut parenthesize: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (arguments.clone()).into_iter().cloned() {
            let __x = operandFlatString(e.clone(), exp.clone(), false, format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (Operator::symbol(operator.clone(), (literal!(" ")).clone())?).clone());
    if parenthesize.clone() && (arguments.clone().len() as i32) > 1 {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn priority(mut exp: Arc<NFExpression>, mut lhs: bool) -> i32 {
    let mut priority: i32 = 0;
    priority = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => if (var_field!((*exp).value, NFExpression::INTEGER).clone() < 0) {4} else {0},
        Deref @ REAL { .. } => if (var_field!((*exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat(0.0_f64)) {4} else {0},
        Deref @ MULTARY { .. } => Operator::priority(var_field!((*exp).operator, NFExpression::MULTARY).clone(), lhs.clone()),
        Deref @ BINARY { .. } => Operator::priority(var_field!((*exp).operator, NFExpression::BINARY).clone(), lhs.clone()),
        Deref @ UNARY { .. } => 4,
        Deref @ LBINARY { .. } => Operator::priority(var_field!((*exp).operator, NFExpression::LBINARY).clone(), lhs.clone()),
        Deref @ LUNARY { .. } => 7,
        Deref @ RELATION { .. } => 6,
        Deref @ RANGE { .. } => 10,
        Deref @ IF { .. } => 11,
        Deref @ CAST { .. } => self::priority(var_field!((*exp).exp, NFExpression::CAST).clone(), lhs.clone()),
        Deref @ BOX { .. } => self::priority(var_field!((*exp).exp, NFExpression::BOX).clone(), lhs.clone()),
        Deref @ UNBOX { .. } => self::priority(var_field!((*exp).exp, NFExpression::UNBOX).clone(), lhs.clone()),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    priority
}

pub fn isAssociativeExp(mut exp: Arc<NFExpression>) -> bool {
    let mut isAssociative: bool = false;
    isAssociative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BINARY { .. } => Operator::isAssociative(var_field!((*exp).operator, NFExpression::BINARY).clone()),
        Deref @ LBINARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isAssociative
}

pub fn isNonAssociativeExp(mut exp: Arc<NFExpression>) -> bool {
    let mut isAssociative: bool = false;
    isAssociative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BINARY { .. } => Operator::isNonAssociative(var_field!((*exp).operator, NFExpression::BINARY).clone()),
        Deref @ LBINARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isAssociative
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getName(mut exp: Arc<NFExpression>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RECORD { .. } => AbsynUtil::pathString(var_field!((*exp).path, NFExpression::RECORD).clone(), (literal!(".")).clone(), true, false)?,
        Deref @ CALL { .. } => AbsynUtil::pathString(Call::functionName(var_field!((*exp).call, NFExpression::CALL).clone())?, (literal!(".")).clone(), true, false)?,
        Deref @ CAST { .. } => getName(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => getName(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => getName(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ MUTABLE { .. } => getName(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => getName(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?,
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => ComponentRef::toString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?,
        Deref @ INSTANCE_NAME { .. } => literal!("getInstanceName"),
        _ => toString(exp.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(name)
}

pub fn enumLiteralPath(mut exp: Arc<NFExpression>) -> Result<Arc<Path>> {
    let mut path: Arc<Path>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut ty_path: Arc<Path>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ENUM_LITERAL { ty: Deref @ Type::ENUMERATION { typePath: __pa0, .. }, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty_path = __pa0.clone();
    name = __pa1.clone();
    path = AbsynUtil::suffixPath(ty_path.clone(), (name.clone()).clone())?;
    Ok(path)
}

pub fn getNominal(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = map(exp.clone(), Arc::new(computeNominal))?;
    exp = SimplifyExp::simplify(exp.clone(), false)?;
    Ok(exp)
}

pub fn computeNominal(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node: Deref @ InstNode::VAR_NODE { varPointer, .. }, .. }, .. } => {
            let mut nominal: Option<Arc<NFExpression>> = None;
            nominal = Variable::getNominal(Pointer::access(varPointer.clone()));
            Util::getOptionOrDefault(nominal.clone(), exp.clone())
        },
        Deref @ INTEGER { .. } => {
            Arc::new(NFExpression::INTEGER { value: var_field!((*exp).value, NFExpression::INTEGER).clone().abs() })
        },
        Deref @ REAL { .. } => {
            Arc::new(NFExpression::REAL { value: var_field!((*exp).value, NFExpression::REAL).clone().abs() })
        },
        Deref @ UNARY { .. } => {
            var_field!((*exp).exp, NFExpression::UNARY).clone()
        },
        Deref @ BINARY { operator, .. } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::SUBTRACTION.clone()) => {
            let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
            (_, sizeClass) = Operator::classify(operator.clone())?;
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass.clone()), operator.ty.clone())?);
            exp.clone()
        },
        Deref @ MULTARY { operator, .. } if (Operator::getMathClassification(operator.clone())? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = listAppend(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()),
                inv_arguments = metamodelica::nil()
            );
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toAbsyn(mut exp: Arc<NFExpression>) -> Result<Arc<Absyn::Exp>> {
    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    aexp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Absyn::Exp::INTEGER { value: var_field!((*exp).value, NFExpression::INTEGER).clone() })
        },
        Deref @ REAL { .. } => {
            Arc::new(Absyn::Exp::REAL { value: ArcStr::from(::std::format!("{}", var_field!((*exp).value, NFExpression::REAL).clone())) })
        },
        Deref @ STRING { .. } => {
            Arc::new(Absyn::Exp::STRING { value: (var_field!((*exp).value, NFExpression::STRING).clone()).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Absyn::Exp::BOOL { value: var_field!((*exp).value, NFExpression::BOOLEAN).clone() })
        },
        Deref @ ENUM_LITERAL { ty: Deref @ Type::ENUMERATION { .. }, .. } => {
            Arc::new(Absyn::Exp::CREF { componentRef: AbsynUtil::pathToCref(enumLiteralPath(exp.clone())?)? })
        },
        Deref @ CLKCONST { .. } => {
            ClockKind::toAbsyn(var_field!((*exp).clk, NFExpression::CLKCONST).clone())?
        },
        Deref @ CREF { .. } => {
            Arc::new(Absyn::Exp::CREF { componentRef: ComponentRef::toAbsyn(var_field!((*exp).cref, NFExpression::CREF).clone())? })
        },
        Deref @ TYPENAME { .. } => {
            Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (Type::toString(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?).clone(), subscripts: metamodelica::nil() }) })
        },
        Deref @ ARRAY { .. } => {
            Arc::new(Absyn::Exp::ARRAY { arrayExp: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ MATRIX { .. } => {
            Arc::new(Absyn::Exp::MATRIX { matrix: {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
        for mut l in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (l.clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RANGE { .. } => {
            Arc::new(Absyn::Exp::RANGE { start: toAbsyn(var_field!((*exp).start, NFExpression::RANGE).clone())?, step: Util::applyOption(var_field!((*exp).step, NFExpression::RANGE).clone(), Arc::new(toAbsyn)), stop: toAbsyn(var_field!((*exp).stop, NFExpression::RANGE).clone())? })
        },
        Deref @ TUPLE { .. } => {
            Arc::new(Absyn::Exp::TUPLE { expressions: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RECORD { .. } => {
            AbsynUtil::makeCall(AbsynUtil::pathToCref(var_field!((*exp).path, NFExpression::RECORD).clone())?, {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, metamodelica::nil())
        },
        Deref @ CALL { .. } => {
            Call::toAbsyn(var_field!((*exp).call, NFExpression::CALL).clone())?
        },
        Deref @ SIZE { .. } => {
            AbsynUtil::makeCall(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("size")).clone(), subscripts: metamodelica::nil() }), if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {list![toAbsyn(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?]} else {metamodelica::nil()}, metamodelica::nil())
        },
        Deref @ END { .. } => {
            Arc::new(openmodelica_ast::Absyn::Exp::END)
        },
        Deref @ BINARY { .. } => {
            Arc::new(Absyn::Exp::BINARY { exp1: toAbsyn(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::BINARY).clone())?, exp2: toAbsyn(var_field!((*exp).exp2, NFExpression::BINARY).clone())? })
        },
        Deref @ UNARY { .. } => {
            Arc::new(Absyn::Exp::UNARY { op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::UNARY).clone())?, exp: toAbsyn(var_field!((*exp).exp, NFExpression::UNARY).clone())? })
        },
        Deref @ LBINARY { .. } => {
            Arc::new(Absyn::Exp::LBINARY { exp1: toAbsyn(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::LBINARY).clone())?, exp2: toAbsyn(var_field!((*exp).exp2, NFExpression::LBINARY).clone())? })
        },
        Deref @ LUNARY { .. } => {
            Arc::new(Absyn::Exp::LUNARY { op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::LUNARY).clone())?, exp: toAbsyn(var_field!((*exp).exp, NFExpression::LUNARY).clone())? })
        },
        Deref @ RELATION { .. } => {
            Arc::new(Absyn::Exp::RELATION { exp1: toAbsyn(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, op: Operator::toAbsyn(var_field!((*exp).operator, NFExpression::RELATION).clone())?, exp2: toAbsyn(var_field!((*exp).exp2, NFExpression::RELATION).clone())? })
        },
        Deref @ IF { .. } => {
            Arc::new(Absyn::Exp::IFEXP { ifExp: toAbsyn(var_field!((*exp).condition, NFExpression::IF).clone())?, trueBranch: toAbsyn(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, elseBranch: toAbsyn(var_field!((*exp).falseBranch, NFExpression::IF).clone())?, elseIfBranch: metamodelica::nil() })
        },
        Deref @ CAST { .. } => {
            toAbsyn(var_field!((*exp).exp, NFExpression::CAST).clone())?
        },
        Deref @ BOX { .. } => {
            toAbsyn(var_field!((*exp).exp, NFExpression::BOX).clone())?
        },
        Deref @ UNBOX { .. } => {
            toAbsyn(var_field!((*exp).exp, NFExpression::UNBOX).clone())?
        },
        Deref @ MUTABLE { .. } => {
            toAbsyn(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?
        },
        Deref @ SHARED_LITERAL { .. } => {
            toAbsyn(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            Arc::new(Absyn::Exp::PARTEVALFUNCTION { function_: ComponentRef::toAbsyn(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?, functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = toAbsyn(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, argNames: metamodelica::nil() }) })
        },
        Deref @ FILENAME { .. } => {
            Arc::new(Absyn::Exp::STRING { value: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() })
        },
        Deref @ INSTANCE_NAME { .. } => {
            AbsynUtil::makeCall(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("getInstanceName")).clone(), subscripts: metamodelica::nil() }), metamodelica::nil(), metamodelica::nil())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toAbsyn")); __mm_s.push_str(&*literal!(" got unknown expression '")); __mm_s.push_str(&*toString(exp.clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(aexp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toDAE(mut exp: Arc<NFExpression>, mut allowEmpty: bool) -> Result<Arc<DAE::Exp>> {
    let mut dexp: Arc<DAE::Exp>;
    let mut changed: bool = true;
    dexp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(DAE::Exp::ICONST { integer: var_field!((*exp).value, NFExpression::INTEGER).clone() })
        },
        Deref @ REAL { .. } => {
            Arc::new(DAE::Exp::RCONST { real: var_field!((*exp).value, NFExpression::REAL).clone() })
        },
        Deref @ STRING { .. } => {
            Arc::new(DAE::Exp::SCONST { string: (var_field!((*exp).value, NFExpression::STRING).clone()).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(DAE::Exp::BCONST { bool: var_field!((*exp).value, NFExpression::BOOLEAN).clone() })
        },
        Deref @ ENUM_LITERAL { .. } => {
            Arc::new(DAE::Exp::ENUM_LITERAL { name: enumLiteralPath(exp.clone())?, index: var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone() })
        },
        Deref @ CLKCONST { .. } => {
            Arc::new(DAE::Exp::CLKCONST { clk: ClockKind::toDAE(var_field!((*exp).clk, NFExpression::CLKCONST).clone())? })
        },
        Deref @ CREF { .. } => {
            Arc::new(DAE::Exp::CREF { componentRef: ComponentRef::toDAE(var_field!((*exp).cref, NFExpression::CREF).clone())?, ty: Type::toDAE(var_field!((*exp).ty, NFExpression::CREF).clone(), true)? })
        },
        Deref @ TYPENAME { .. } => {
            toDAE(ExpandExp::expandTypename(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?, false)?
        },
        Deref @ ARRAY { .. } => {
            Arc::new(DAE::Exp::ARRAY { ty: Type::toDAE(var_field!((*exp).ty, NFExpression::ARRAY).clone(), true)?, scalar: Type::isVector(var_field!((*exp).ty, NFExpression::ARRAY).clone()), array: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toDAE(e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RECORD { .. } => {
            toDAERecord(var_field!((*exp).ty, NFExpression::RECORD).clone(), var_field!((*exp).path, NFExpression::RECORD).clone(), var_field!((*exp).elements, NFExpression::RECORD).clone())?
        },
        Deref @ RANGE { .. } => {
            Arc::new(DAE::Exp::RANGE { ty: Type::toDAE(var_field!((*exp).ty, NFExpression::RANGE).clone(), true)?, start: toDAE(var_field!((*exp).start, NFExpression::RANGE).clone(), false)?, step: if (isSome(var_field!((*exp).step, NFExpression::RANGE).clone())) {Some(toDAE(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?, false)?)} else {None}, stop: toDAE(var_field!((*exp).stop, NFExpression::RANGE).clone(), false)? })
        },
        Deref @ TUPLE { .. } => {
            Arc::new(DAE::Exp::TUPLE { PR: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toDAE(e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ CALL { .. } => {
            Call::toDAE(var_field!((*exp).call, NFExpression::CALL).clone())?
        },
        Deref @ SIZE { .. } => {
            Arc::new(DAE::Exp::SIZE { exp: toDAE(var_field!((*exp).exp, NFExpression::SIZE).clone(), false)?, sz: if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {Some(toDAE(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?, false)?)} else {None} })
        },
        Deref @ MULTARY { .. } => {
            toDAE(SimplifyExp::splitMultary(exp.clone())?, false)?
        },
        Deref @ BINARY { .. } => {
            let mut daeOp: DAE::Operator;
            let mut swap: bool = false;
            let mut negate: bool = false;
            let mut dae1: Arc<DAE::Exp>;
            let mut dae2: Arc<DAE::Exp>;
            (daeOp, swap, negate) = Operator::toDAE(var_field!((*exp).operator, NFExpression::BINARY).clone())?;
            dae1 = toDAE(var_field!((*exp).exp1, NFExpression::BINARY).clone(), false)?;
            dae2 = toDAE(if (negate.clone()) {self::negate(var_field!((*exp).exp2, NFExpression::BINARY).clone())} else {var_field!((*exp).exp2, NFExpression::BINARY).clone()}, false)?;
            Arc::new(DAE::Exp::BINARY { exp1: if (swap.clone()) {dae2.clone()} else {dae1.clone()}, operator: daeOp.clone(), exp2: if (swap.clone()) {dae1.clone()} else {dae2.clone()} })
        },
        Deref @ UNARY { .. } => {
            Arc::new(DAE::Exp::UNARY { operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::UNARY).clone())?.0, exp: toDAE(var_field!((*exp).exp, NFExpression::UNARY).clone(), false)? })
        },
        Deref @ LBINARY { .. } => {
            Arc::new(DAE::Exp::LBINARY { exp1: toDAE(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), false)?, operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::LBINARY).clone())?.0, exp2: toDAE(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), false)? })
        },
        Deref @ LUNARY { .. } => {
            Arc::new(DAE::Exp::LUNARY { operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::LUNARY).clone())?.0, exp: toDAE(var_field!((*exp).exp, NFExpression::LUNARY).clone(), false)? })
        },
        Deref @ RELATION { .. } => {
            Arc::new(DAE::Exp::RELATION { exp1: toDAE(var_field!((*exp).exp1, NFExpression::RELATION).clone(), false)?, operator: Operator::toDAE(var_field!((*exp).operator, NFExpression::RELATION).clone())?.0, exp2: toDAE(var_field!((*exp).exp2, NFExpression::RELATION).clone(), false)?, index: var_field!((*exp).index, NFExpression::RELATION).clone(), optionExpisASUB: None })
        },
        Deref @ IF { .. } => {
            Arc::new(DAE::Exp::IFEXP { expCond: toDAE(var_field!((*exp).condition, NFExpression::IF).clone(), false)?, expThen: toDAE(var_field!((*exp).trueBranch, NFExpression::IF).clone(), false)?, expElse: toDAE(var_field!((*exp).falseBranch, NFExpression::IF).clone(), false)? })
        },
        Deref @ CAST { .. } => {
            Arc::new(DAE::Exp::CAST { ty: Type::toDAE(var_field!((*exp).ty, NFExpression::CAST).clone(), true)?, exp: toDAE(var_field!((*exp).exp, NFExpression::CAST).clone(), false)? })
        },
        Deref @ BOX { .. } => {
            Arc::new(DAE::Exp::BOX { exp: toDAE(var_field!((*exp).exp, NFExpression::BOX).clone(), false)? })
        },
        Deref @ UNBOX { .. } => {
            Arc::new(DAE::Exp::UNBOX { exp: toDAE(var_field!((*exp).exp, NFExpression::UNBOX).clone(), false)?, ty: Type::toDAE(var_field!((*exp).ty, NFExpression::UNBOX).clone(), true)? })
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            Arc::new(DAE::Exp::ASUB { exp: toDAE(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?, sub: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::toDAE(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            Arc::new(DAE::Exp::TSUB { exp: toDAE(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), false)?, ix: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: Type::toDAE(var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone(), true)? })
        },
        Deref @ RECORD_ELEMENT { .. } => {
            Arc::new(DAE::Exp::RSUB { exp: toDAE(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), false)?, ix: -1, fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: Type::toDAE(var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone(), true)? })
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut r#fn: Arc<Function::Function::Function>;
            let __pa0 = ::match_deref::match_deref! { match &(Function::Function::typeRefCache(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            Arc::new(DAE::Exp::PARTEVALFUNCTION { path: Function::Function::nameConsiderBuiltin(r#fn.clone())?, expList: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = toDAE(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, ty: Type::toDAE(var_field!((*exp).ty, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), true)?, origType: Type::toDAE(Arc::new(Type::NFType::FUNCTION { r#fn: r#fn.clone(), fnType: Type::FunctionType::FUNCTIONAL_VARIABLE.clone() }), true)? })
        },
        Deref @ MUTABLE { .. } => {
            toDAE(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), false)?
        },
        Deref @ EMPTY { .. } if (allowEmpty.clone()) => {
            let mut dty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            dty = Type::toDAE(var_field!((*exp).ty, NFExpression::EMPTY).clone(), true)?;
            Arc::new(DAE::Exp::EMPTY { scope: (literal!("")).clone(), name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$dummy")).clone(), identType: dty.clone(), subscriptLst: metamodelica::nil() }), ty: dty.clone(), tyStr: (Type::toString(var_field!((*exp).ty, NFExpression::EMPTY).clone())?).clone() })
        },
        Deref @ SHARED_LITERAL { .. } => {
            Arc::new(DAE::Exp::SHARED_LITERAL { index: var_field!((*exp).index, NFExpression::SHARED_LITERAL).clone(), exp: toDAE(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), false)? })
        },
        Deref @ FILENAME { .. } => {
            if (Flags::getConfigBool(Flags::BUILDING_FMU.clone())?) {Arc::new(DAE::Exp::CALL { path: Arc::new(Path::IDENT { name: (literal!("OpenModelica_fmuLoadResource")).clone() }), expLst: list![Arc::new(DAE::Exp::SCONST { string: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() })], attr: DAE::callAttrBuiltinImpureString.clone() })} else {Arc::new(DAE::Exp::SCONST { string: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() })}
        },
        Deref @ INSTANCE_NAME { .. } => {
            Arc::new(DAE::Exp::CALL { path: Arc::new(Path::IDENT { name: (literal!("getInstanceName")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinString.clone() })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toDAE")); __mm_s.push_str(&*literal!(" got unknown expression '")); __mm_s.push_str(&*toString(exp.clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dexp)
}

pub fn toDAERecord(mut ty: Arc<Type::NFType>, mut path: Arc<Path>, mut args: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut field_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut arg: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut rest_args: Arc<metamodelica::List<Arc<NFExpression>>> = args.clone();
    let mut dargs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    for mut field in &*Type::recordFields(Type::unbox(ty.clone())) {
        let mut field = field.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(field.clone()) {
        Deref @ Record::Field::INPUT { .. } => {
            field_names = cons(var_field!((*field).name, Record::Field::Field::INPUT).clone(), field_names.clone());
            dargs = cons(toDAE(arg.clone(), true)?, dargs.clone());
            ()
        },
        Deref @ Record::Field::LOCAL { .. } => {
            field_names = cons(var_field!((*field).name, Record::Field::Field::LOCAL).clone(), field_names.clone());
            dargs = cons(toDAE(arg.clone(), true)?, dargs.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    field_names = field_names.clone().reverse();
    dargs = dargs.clone().reverse();
    exp = if (Type::isBoxed(ty.clone())) {Arc::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: dargs.clone(), fieldNames: field_names.clone(), index: -1, typeVars: metamodelica::nil() })} else {Arc::new(DAE::Exp::RECORD { path: path.clone(), exps: dargs.clone(), comp: field_names.clone(), ty: Type::toDAE(ty.clone(), true)? })};
    Ok(exp)
}

pub fn toDAEValue(mut exp: Arc<NFExpression>) -> Result<Arc<Values::Value>> {
    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Values::Value::INTEGER { integer: var_field!((*exp).value, NFExpression::INTEGER).clone() })
        },
        Deref @ REAL { .. } => {
            Arc::new(Values::Value::REAL { real: var_field!((*exp).value, NFExpression::REAL).clone() })
        },
        Deref @ STRING { .. } => {
            Arc::new(Values::Value::STRING { string: (var_field!((*exp).value, NFExpression::STRING).clone()).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Values::Value::BOOL { boolean: var_field!((*exp).value, NFExpression::BOOLEAN).clone() })
        },
        Deref @ ENUM_LITERAL { ty: ty @ Deref @ Type::ENUMERATION { .. }, .. } => {
            Arc::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), (var_field!((*exp).name, NFExpression::ENUM_LITERAL).clone()).clone())?, index: var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone() })
        },
        Deref @ ARRAY { .. } => {
            ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toDAEValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?
        },
        Deref @ RECORD { .. } => {
            toDAEValueRecord(var_field!((*exp).ty, NFExpression::RECORD).clone(), var_field!((*exp).path, NFExpression::RECORD).clone(), var_field!((*exp).elements, NFExpression::RECORD).clone())?
        },
        Deref @ FILENAME { .. } => {
            Arc::new(Values::Value::STRING { string: (var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone() })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toDAEValue")); __mm_s.push_str(&*literal!(" got unhandled expression ")); __mm_s.push_str(&*toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub fn toDAEValueRecord(mut ty: Arc<Type::NFType>, mut path: Arc<Path>, mut args: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<Values::Value>> {
    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut field_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut arg: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut rest_args: Arc<metamodelica::List<Arc<NFExpression>>> = args.clone();
    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    for mut field in &*Type::recordFields(ty.clone()) {
        let mut field = field.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(field.clone()) {
        Deref @ Record::Field::INPUT { .. } => {
            field_names = cons(var_field!((*field).name, Record::Field::Field::INPUT).clone(), field_names.clone());
            values = cons(toDAEValue(arg.clone())?, values.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    field_names = field_names.clone().reverse();
    values = values.clone().reverse();
    value = Arc::new(Values::Value::RECORD { record_: path.clone(), orderd: values.clone(), comp: field_names.clone(), index: -1 });
    Ok(value)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dimensionCount(mut exp: Arc<NFExpression>) -> Result<i32> {
    let mut dimCount: i32 = 0;
    dimCount = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { ty: Deref @ Type::UNKNOWN, .. } => 1 + dimensionCount(var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow()[(1-1) as usize].clone())?,
        Deref @ ARRAY { .. } => Type::dimensionCount(var_field!((*exp).ty, NFExpression::ARRAY).clone()),
        Deref @ RANGE { .. } => Type::dimensionCount(var_field!((*exp).ty, NFExpression::RANGE).clone()),
        Deref @ SIZE { dimIndex: None, .. } => dimensionCount(var_field!((*exp).exp, NFExpression::SIZE).clone())?,
        Deref @ CAST { .. } => dimensionCount(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => Type::dimensionCount(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone()),
        Deref @ TUPLE_ELEMENT { .. } => Type::dimensionCount(var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone()),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dimCount)
}

pub fn dimensions(mut exp: Arc<NFExpression>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    dims = Type::arrayDims(typeOf(exp.clone()));
    dims
}

pub fn map(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            Arc::new(NFExpression::CLKCONST { clk: ClockKind::mapExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())? })
        },
        Deref @ CREF { .. } => {
            Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: ComponentRef::mapExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())? })
        },
        Deref @ ARRAY { .. } if (!(var_field!((*exp).literal, NFExpression::ARRAY).clone())) => {
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static> = func.clone(); move |__pe_a0| map(__pe_a0, __pe_b1.clone()) })), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            Arc::new(NFExpression::MATRIX { elements: {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e4: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e4 = map(e2.clone(), func.clone())?;
            e3 = map(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e4.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: Some(e4.clone()), stop: e3.clone() })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e3 = map(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: None, stop: e3.clone() })}
        },
        Deref @ TUPLE { .. } => {
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RECORD { .. } => {
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ CALL { .. } => {
            Arc::new(NFExpression::CALL { call: Call::mapExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            e3 = map(e2.clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: Some(e3.clone()) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())?;
            e2 = map(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::BINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::BINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                inv_arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1.clone() })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())?;
            e2 = map(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::LBINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::LBINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::LUNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1.clone() })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())?;
            e2 = map(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::RELATION).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::RELATION).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())?;
            e2 = map(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())?;
            e3 = map(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).condition, NFExpression::IF).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).trueBranch, NFExpression::IF).clone(),&e2.clone()) && referenceEq(&var_field!((*exp).falseBranch, NFExpression::IF).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1.clone(), trueBranch: e2.clone(), falseBranch: e3.clone() })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::CAST).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1.clone() })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::BOX).clone(),&e1.clone())) {exp.clone()} else {r#box(e1.clone())}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNBOX).clone(),&e1.clone())) {exp.clone()} else {unbox(e1.clone())}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: map(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?, subscripts: {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(s.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1.clone(), index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = map(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1.clone(), index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), map(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?);
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = map(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?);
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp = func(outExp.clone())?;
    Ok(outExp)
}

pub fn fakeMap(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Arc<NFExpression> {
    pub type MapFunc = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = func(exp.clone()).unwrap();
    outExp
}

pub fn mapOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Option<Arc<NFExpression>>> {
    pub type MapFunc = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Option<Arc<NFExpression>> = None;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(e) => Some(map(e.clone(), func.clone())?),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn mapReverse(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut exp: Arc<NFExpression> = exp;
    exp = func(exp.clone())?;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            Arc::new(NFExpression::CLKCONST { clk: ClockKind::mapExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())? })
        },
        Deref @ CREF { .. } => {
            Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: ComponentRef::mapExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())? })
        },
        Deref @ ARRAY { .. } if (!(var_field!((*exp).literal, NFExpression::ARRAY).clone())) => {
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static> = func.clone(); move |__pe_a0| mapReverse(__pe_a0, __pe_b1.clone()) })), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            Arc::new(NFExpression::MATRIX { elements: {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e4: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e4 = mapReverse(e2.clone(), func.clone())?;
            e3 = mapReverse(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e4.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: Some(e4.clone()), stop: e3.clone() })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            e3 = mapReverse(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: None, stop: e3.clone() })}
        },
        Deref @ TUPLE { .. } => {
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RECORD { .. } => {
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ CALL { .. } => {
            Arc::new(NFExpression::CALL { call: Call::mapExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            e3 = mapReverse(e2.clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: Some(e3.clone()) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::BINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::BINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = mapReverse(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                inv_arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = mapReverse(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1.clone() })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::LBINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::LBINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::LUNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1.clone() })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::RELATION).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::RELATION).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())?;
            e2 = mapReverse(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())?;
            e3 = mapReverse(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).condition, NFExpression::IF).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).trueBranch, NFExpression::IF).clone(),&e2.clone()) && referenceEq(&var_field!((*exp).falseBranch, NFExpression::IF).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1.clone(), trueBranch: e2.clone(), falseBranch: e3.clone() })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::CAST).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1.clone() })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::BOX).clone(),&e1.clone())) {exp.clone()} else {r#box(e1.clone())}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNBOX).clone(),&e1.clone())) {exp.clone()} else {unbox(e1.clone())}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: mapReverse(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?, subscripts: {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(s.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1.clone(), index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = mapReverse(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?;
            if (referenceEq(&var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1.clone(), index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), mapReverse(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?);
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = mapReverse(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?);
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn mapShallow(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFunc = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            Arc::new(NFExpression::CLKCONST { clk: ClockKind::mapExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone()) })
        },
        Deref @ CREF { .. } => {
            Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: ComponentRef::mapExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone()) })
        },
        Deref @ ARRAY { .. } if (!(var_field!((*exp).literal, NFExpression::ARRAY).clone())) => {
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone()), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            Arc::new(NFExpression::MATRIX { elements: {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).elements, NFExpression::MATRIX).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e4: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            e4 = func(e2.clone())?;
            e3 = func(var_field!((*exp).stop, NFExpression::RANGE).clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e4.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: Some(e4.clone()), stop: e3.clone() })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            e3 = func(var_field!((*exp).stop, NFExpression::RANGE).clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: None, stop: e3.clone() })}
        },
        Deref @ TUPLE { .. } => {
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ RECORD { .. } => {
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ CALL { .. } => {
            Arc::new(NFExpression::CALL { call: Call::mapExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp, NFExpression::SIZE).clone())?;
            e3 = func(e2.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: Some(e3.clone()) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp, NFExpression::SIZE).clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp1, NFExpression::BINARY).clone())?;
            e2 = func(var_field!((*exp).exp2, NFExpression::BINARY).clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::BINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::BINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                inv_arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp, NFExpression::UNARY).clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1.clone() })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?;
            e2 = func(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::LBINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::LBINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp, NFExpression::LUNARY).clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::LUNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1.clone() })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp1, NFExpression::RELATION).clone())?;
            e2 = func(var_field!((*exp).exp2, NFExpression::RELATION).clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::RELATION).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::RELATION).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).condition, NFExpression::IF).clone())?;
            e2 = func(var_field!((*exp).trueBranch, NFExpression::IF).clone())?;
            e3 = func(var_field!((*exp).falseBranch, NFExpression::IF).clone())?;
            if (referenceEq(&var_field!((*exp).condition, NFExpression::IF).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).trueBranch, NFExpression::IF).clone(),&e2.clone()) && referenceEq(&var_field!((*exp).falseBranch, NFExpression::IF).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1.clone(), trueBranch: e2.clone(), falseBranch: e3.clone() })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp, NFExpression::CAST).clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::CAST).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1.clone() })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp, NFExpression::BOX).clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::BOX).clone(),&e1.clone())) {exp.clone()} else {r#box(e1.clone())}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).exp, NFExpression::UNBOX).clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNBOX).clone(),&e1.clone())) {exp.clone()} else {unbox(e1.clone())}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, subscripts: {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(e.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?;
            if (referenceEq(&var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1.clone(), index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            e1 = func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?;
            if (referenceEq(&var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1.clone(), index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?);
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?);
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn mapShallowOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Option<Arc<NFExpression>> {
    pub type MapFunc = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Option<Arc<NFExpression>> = None;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(e) => Some(func(e.clone()).unwrap()),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn mapArrayElements(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Arc<NFExpression> {
    pub type MapFunc = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY;
                elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static> = func.clone(); move |__pe_a0| Ok(mapArrayElements(__pe_a0, __pe_b1.clone())) })),
                literal = Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new(fnptr!(isLiteral, Arc<NFExpression>)))
            );
            exp.clone()
        },
        _ => func(exp.clone()).unwrap(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn foldArray<ArgT: Clone + 'static>(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<ArgT>;

    let mut result: ArgT = arg.clone();
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        result = fold(e.clone(), func.clone(), result.clone())?;
    }
    Ok(result)
}

pub fn foldList<ArgT: Clone + 'static>(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<ArgT>;

    let mut result: ArgT = arg.clone();
    for mut e in &*expl.clone() {
        let mut e = e.clone();
        result = fold(e.clone(), func.clone(), result.clone())?;
    }
    Ok(result)
}

pub fn foldOpt<ArgT: Clone + 'static>(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> ArgT {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<ArgT>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(e) => {
            func(e.clone(), arg.clone()).unwrap()
        },
        _ => {
            arg.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub fn fold<ArgT: Clone + 'static>(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<ArgT>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            ClockKind::foldExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone(), arg.clone())?
        },
        Deref @ CREF { .. } => {
            ComponentRef::foldExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone(), arg.clone())
        },
        Deref @ ARRAY { .. } => {
            foldArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone(), arg.clone())?
        },
        Deref @ MATRIX { .. } => {
            result = arg.clone();
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                result = foldList(row.clone(), func.clone(), result.clone())?;
            }
            result.clone()
        },
        Deref @ RANGE { .. } => {
            result = fold(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone(), arg.clone())?;
            result = foldOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone(), result.clone());
            fold(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone(), result.clone())?
        },
        Deref @ TUPLE { .. } => {
            foldList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone(), arg.clone())?
        },
        Deref @ RECORD { .. } => {
            foldList(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone(), arg.clone())?
        },
        Deref @ CALL { .. } => {
            Call::foldExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone(), arg.clone())?
        },
        Deref @ SIZE { dimIndex: Some(e), .. } => {
            result = fold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg.clone())?;
            fold(e.clone(), func.clone(), result.clone())?
        },
        Deref @ SIZE { .. } => {
            fold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg.clone())?
        },
        Deref @ BINARY { .. } => {
            result = fold(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone(), arg.clone())?;
            fold(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone(), result.clone())?
        },
        Deref @ MULTARY { .. } => {
            result = arg.clone();
            for mut argument in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                result = fold(argument.clone(), func.clone(), result.clone())?;
            }
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                result = fold(argument.clone(), func.clone(), result.clone())?;
            }
            result.clone()
        },
        Deref @ UNARY { .. } => {
            fold(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone(), arg.clone())?
        },
        Deref @ LBINARY { .. } => {
            result = fold(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone(), arg.clone())?;
            fold(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone(), result.clone())?
        },
        Deref @ LUNARY { .. } => {
            fold(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone(), arg.clone())?
        },
        Deref @ RELATION { .. } => {
            result = fold(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone(), arg.clone())?;
            fold(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone(), result.clone())?
        },
        Deref @ IF { .. } => {
            result = fold(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone(), arg.clone())?;
            result = fold(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone(), result.clone())?;
            fold(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone(), result.clone())?
        },
        Deref @ CAST { .. } => {
            fold(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone(), arg.clone())?
        },
        Deref @ BOX { .. } => {
            fold(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone(), arg.clone())?
        },
        Deref @ UNBOX { .. } => {
            fold(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone(), arg.clone())?
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            result = fold(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone(), arg.clone())?;
            List::fold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::foldExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), result.clone())
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            fold(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone(), arg.clone())?
        },
        Deref @ RECORD_ELEMENT { .. } => {
            fold(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone(), arg.clone())?
        },
        Deref @ MUTABLE { .. } => {
            fold(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone(), arg.clone())?
        },
        Deref @ SHARED_LITERAL { .. } => {
            fold(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone(), arg.clone())?
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            foldList(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone(), arg.clone())?
        },
        _ => {
            arg.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = func(exp.clone(), result.clone())?;
    Ok(result)
}

pub fn applyArray(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        apply(e.clone(), func.clone())?;
    }
    Ok(())
}

pub fn applyList(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    for mut e in &*expl.clone() {
        let mut e = e.clone();
        apply(e.clone(), func.clone())?;
    }
    Ok(())
}

pub fn applyOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    if isSome(exp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        apply(e.clone(), func.clone())?;
    }
    Ok(())
}

pub fn apply(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            ClockKind::applyExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())?;
            ()
        },
        Deref @ CREF { .. } => {
            ComponentRef::applyExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())?;
            ()
        },
        Deref @ ARRAY { .. } => {
            applyArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone())?;
            ()
        },
        Deref @ MATRIX { .. } => {
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                applyList(row.clone(), func.clone())?;
            }
            ()
        },
        Deref @ RANGE { .. } => {
            apply(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())?;
            applyOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone())?;
            apply(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?;
            ()
        },
        Deref @ TUPLE { .. } => {
            applyList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone())?;
            ()
        },
        Deref @ RECORD { .. } => {
            applyList(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone())?;
            ()
        },
        Deref @ CALL { .. } => {
            Call::applyExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?;
            ()
        },
        Deref @ SIZE { .. } => {
            apply(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?;
            applyOpt(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone())?;
            ()
        },
        Deref @ BINARY { .. } => {
            apply(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())?;
            apply(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?;
            ()
        },
        Deref @ MULTARY { .. } => {
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                apply(arg.clone(), func.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                apply(arg.clone(), func.clone())?;
            }
            ()
        },
        Deref @ UNARY { .. } => {
            apply(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?;
            ()
        },
        Deref @ LBINARY { .. } => {
            apply(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())?;
            apply(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?;
            ()
        },
        Deref @ LUNARY { .. } => {
            apply(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?;
            ()
        },
        Deref @ RELATION { .. } => {
            apply(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())?;
            apply(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?;
            ()
        },
        Deref @ IF { .. } => {
            apply(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())?;
            apply(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())?;
            apply(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?;
            ()
        },
        Deref @ CAST { .. } => {
            apply(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?;
            ()
        },
        Deref @ BOX { .. } => {
            apply(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?;
            ()
        },
        Deref @ UNBOX { .. } => {
            apply(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?;
            ()
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            apply(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?;
            for mut s in &*var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut s = s.clone();
                Subscript::applyExp(s.clone(), func.clone())?;
            }
            ()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            apply(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?;
            ()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            apply(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?;
            ()
        },
        Deref @ MUTABLE { .. } => {
            apply(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?;
            ()
        },
        Deref @ SHARED_LITERAL { .. } => {
            apply(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?;
            ()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            applyList(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    func(exp.clone())?;
    Ok(())
}

pub fn applyArrayShallow(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> () {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        func(e.clone()).unwrap();
    }
    ()
}

pub fn applyListShallow(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> () {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    for mut e in &*expl.clone() {
        let mut e = e.clone();
        func(e.clone()).unwrap();
    }
    ()
}

pub fn applyShallow(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            ClockKind::applyExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone());
            ()
        },
        Deref @ CREF { .. } => {
            ComponentRef::applyExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone());
            ()
        },
        Deref @ ARRAY { .. } => {
            applyArrayShallow(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone());
            ()
        },
        Deref @ MATRIX { .. } => {
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                applyListShallow(row.clone(), func.clone());
            }
            ()
        },
        Deref @ RANGE { .. } => {
            func(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            applyShallowOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone())?;
            func(var_field!((*exp).stop, NFExpression::RANGE).clone())?;
            ()
        },
        Deref @ TUPLE { .. } => {
            applyListShallow(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone());
            ()
        },
        Deref @ RECORD { .. } => {
            applyListShallow(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone());
            ()
        },
        Deref @ CALL { .. } => {
            Call::applyExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?;
            ()
        },
        Deref @ SIZE { .. } => {
            func(var_field!((*exp).exp, NFExpression::SIZE).clone())?;
            applyShallowOpt(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone())?;
            ()
        },
        Deref @ BINARY { .. } => {
            func(var_field!((*exp).exp1, NFExpression::BINARY).clone())?;
            func(var_field!((*exp).exp2, NFExpression::BINARY).clone())?;
            ()
        },
        Deref @ MULTARY { .. } => {
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                func(arg.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                func(arg.clone())?;
            }
            ()
        },
        Deref @ UNARY { .. } => {
            func(var_field!((*exp).exp, NFExpression::UNARY).clone())?;
            ()
        },
        Deref @ LBINARY { .. } => {
            func(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?;
            func(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?;
            ()
        },
        Deref @ LUNARY { .. } => {
            func(var_field!((*exp).exp, NFExpression::LUNARY).clone())?;
            ()
        },
        Deref @ RELATION { .. } => {
            func(var_field!((*exp).exp1, NFExpression::RELATION).clone())?;
            func(var_field!((*exp).exp2, NFExpression::RELATION).clone())?;
            ()
        },
        Deref @ IF { .. } => {
            func(var_field!((*exp).condition, NFExpression::IF).clone())?;
            func(var_field!((*exp).trueBranch, NFExpression::IF).clone())?;
            func(var_field!((*exp).falseBranch, NFExpression::IF).clone())?;
            ()
        },
        Deref @ CAST { .. } => {
            func(var_field!((*exp).exp, NFExpression::CAST).clone())?;
            ()
        },
        Deref @ BOX { .. } => {
            func(var_field!((*exp).exp, NFExpression::BOX).clone())?;
            ()
        },
        Deref @ UNBOX { .. } => {
            func(var_field!((*exp).exp, NFExpression::UNBOX).clone())?;
            ()
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?;
            for mut s in &*var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone() {
                let mut s = s.clone();
                Subscript::applyExpShallow(s.clone(), func.clone());
            }
            ()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?;
            ()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?;
            ()
        },
        Deref @ MUTABLE { .. } => {
            func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?;
            ()
        },
        Deref @ SHARED_LITERAL { .. } => {
            func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?;
            ()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            applyListShallow(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn applyShallowOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<NFExpression>) -> Result<()>;

    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    if isSome(exp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        func(e.clone())?;
    }
    Ok(())
}

pub fn mapFold<ArgT: Clone + 'static>(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<NFExpression>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut arg: ArgT = arg;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            let mut ck: Arc<ClockKind::NFClockKind>;
            (ck, arg) = ClockKind::mapFoldExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).clk, NFExpression::CLKCONST).clone(),&ck.clone())) {exp.clone()} else {Arc::new(NFExpression::CLKCONST { clk: ck.clone() })}
        },
        Deref @ CREF { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            (cr, arg) = ComponentRef::mapFoldExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone(), arg.clone());
            if (referenceEq(&var_field!((*exp).cref, NFExpression::CREF).clone(),&cr.clone())) {exp.clone()} else {Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: cr.clone() })}
        },
        Deref @ ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            (arr, arg) = Array::mapFold(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| mapFold(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), arr.clone(), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            let mut mat: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
            (mat, arg) = List::mapFoldList(var_field!((*exp).elements, NFExpression::MATRIX).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| mapFold(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
            Arc::new(NFExpression::MATRIX { elements: mat.clone() })
        },
        Deref @ RANGE { step: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e4: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone(), arg.clone())?;
            (e4, arg) = mapFold(e2.clone(), func.clone(), arg.clone())?;
            (e3, arg) = mapFold(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e4.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: Some(e4.clone()), stop: e3.clone() })}
        },
        Deref @ RANGE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone(), arg.clone())?;
            (e3, arg) = mapFold(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).start, NFExpression::RANGE).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).stop, NFExpression::RANGE).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: None, stop: e3.clone() })}
        },
        Deref @ TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            (expl, arg) = List::map1Fold(var_field!((*exp).elements, NFExpression::TUPLE).clone(), Arc::new(mapFold), func.clone(), arg.clone());
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: expl.clone() })
        },
        Deref @ RECORD { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            (expl, arg) = List::map1Fold(var_field!((*exp).elements, NFExpression::RECORD).clone(), Arc::new(mapFold), func.clone(), arg.clone());
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: expl.clone() })
        },
        Deref @ CALL { .. } => {
            let mut call: Arc<Call::NFCall>;
            (call, arg) = Call::mapFoldExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).call, NFExpression::CALL).clone(),&call.clone())) {exp.clone()} else {Arc::new(NFExpression::CALL { call: call.clone() })}
        },
        Deref @ SIZE { dimIndex: Some(e2), .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg.clone())?;
            (e3, arg) = mapFold(e2.clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone()) && referenceEq(&e2.clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: Some(e3.clone()) })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: None })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone(), arg.clone())?;
            (e2, arg) = mapFold(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::BINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::BINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = mapFold(argument.clone(), func.clone(), arg.clone())?;
                expl = cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; arguments = expl.clone().reverse());
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = mapFold(argument.clone(), func.clone(), arg.clone())?;
                expl = cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; inv_arguments = expl.clone().reverse());
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1.clone() })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone(), arg.clone())?;
            (e2, arg) = mapFold(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::LBINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::LBINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::LUNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1.clone() })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone(), arg.clone())?;
            (e2, arg) = mapFold(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::RELATION).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::RELATION).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone(), arg.clone())?;
            (e2, arg) = mapFold(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone(), arg.clone())?;
            (e3, arg) = mapFold(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).condition, NFExpression::IF).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).trueBranch, NFExpression::IF).clone(),&e2.clone()) && referenceEq(&var_field!((*exp).falseBranch, NFExpression::IF).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1.clone(), trueBranch: e2.clone(), falseBranch: e3.clone() })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::CAST).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1.clone() })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::BOX).clone(),&e1.clone())) {exp.clone()} else {r#box(e1.clone())}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNBOX).clone(),&e1.clone())) {exp.clone()} else {unbox(e1.clone())}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone(), arg.clone())?;
            (subs, arg) = List::mapFold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::mapFoldExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e1.clone(), subscripts: subs.clone(), ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1.clone(), index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1.clone(), index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone(), arg.clone())?;
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), e1.clone());
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = mapFold(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone(), arg.clone())?;
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = e1.clone());
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            (expl, arg) = List::map1Fold(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), Arc::new(mapFold), func.clone(), arg.clone());
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = expl.clone());
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, arg) = func(outExp.clone(), arg.clone())?;
    Ok((outExp, arg))
}

pub fn mapFoldOpt<ArgT: Clone + 'static>(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Option<Arc<NFExpression>>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)>;

    let mut outExp: Option<Arc<NFExpression>> = None;
    let mut arg: ArgT = arg;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(e) => {
            let mut e = (*e).clone();
            (e, arg) = mapFold(e.clone(), func.clone(), arg.clone())?;
            Some(e.clone())
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

pub fn mapFoldShallow<ArgT: Clone + 'static>(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<NFExpression>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut arg: ArgT = arg;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            let mut ck: Arc<ClockKind::NFClockKind>;
            (ck, arg) = ClockKind::mapFoldExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).clk, NFExpression::CLKCONST).clone(),&ck.clone())) {exp.clone()} else {Arc::new(NFExpression::CLKCONST { clk: ck.clone() })}
        },
        Deref @ CREF { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            (cr, arg) = ComponentRef::mapFoldExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone(), arg.clone());
            if (referenceEq(&var_field!((*exp).cref, NFExpression::CREF).clone(),&cr.clone())) {exp.clone()} else {Arc::new(NFExpression::CREF { ty: var_field!((*exp).ty, NFExpression::CREF).clone(), cref: cr.clone() })}
        },
        Deref @ ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            (arr, arg) = Array::mapFold(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone(), arg.clone());
            makeArray(var_field!((*exp).ty, NFExpression::ARRAY).clone(), arr.clone(), var_field!((*exp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ MATRIX { .. } => {
            let mut mat: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
            (mat, arg) = List::mapFoldList(var_field!((*exp).elements, NFExpression::MATRIX).clone(), func.clone(), arg.clone());
            Arc::new(NFExpression::MATRIX { elements: mat.clone() })
        },
        Deref @ RANGE { step: oe, .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut oe = (*oe).clone();
            (e1, arg) = func(var_field!((*exp).start, NFExpression::RANGE).clone(), arg.clone())?;
            (oe, arg) = mapFoldOptShallow(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone(), arg.clone());
            (e3, arg) = func(var_field!((*exp).stop, NFExpression::RANGE).clone(), arg.clone())?;
            if (referenceEq(&e1.clone(),&var_field!((*exp).start, NFExpression::RANGE).clone()) && referenceEq(&oe.clone(),&var_field!((*exp).step, NFExpression::RANGE).clone()) && referenceEq(&e3.clone(),&var_field!((*exp).stop, NFExpression::RANGE).clone())) {exp.clone()} else {Arc::new(NFExpression::RANGE { ty: var_field!((*exp).ty, NFExpression::RANGE).clone(), start: e1.clone(), step: oe.clone(), stop: e3.clone() })}
        },
        Deref @ TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            (expl, arg) = List::mapFold(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone(), arg.clone());
            Arc::new(NFExpression::TUPLE { ty: var_field!((*exp).ty, NFExpression::TUPLE).clone(), elements: expl.clone() })
        },
        Deref @ RECORD { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            (expl, arg) = List::mapFold(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone(), arg.clone());
            Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD).clone(), elements: expl.clone() })
        },
        Deref @ CALL { .. } => {
            let mut call: Arc<Call::NFCall>;
            (call, arg) = Call::mapFoldExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).call, NFExpression::CALL).clone(),&call.clone())) {exp.clone()} else {Arc::new(NFExpression::CALL { call: call.clone() })}
        },
        Deref @ SIZE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut oe: Option<Arc<NFExpression>> = None;
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::SIZE).clone(), arg.clone())?;
            (oe, arg) = mapFoldOptShallow(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone(), arg.clone());
            if (referenceEq(&var_field!((*exp).exp, NFExpression::SIZE).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).dimIndex, NFExpression::SIZE).clone(),&oe.clone())) {exp.clone()} else {Arc::new(NFExpression::SIZE { exp: e1.clone(), dimIndex: oe.clone() })}
        },
        Deref @ BINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp1, NFExpression::BINARY).clone(), arg.clone())?;
            (e2, arg) = func(var_field!((*exp).exp2, NFExpression::BINARY).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::BINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::BINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::BINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::BINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ MULTARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = func(argument.clone(), arg.clone())?;
                expl = cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; arguments = expl.clone().reverse());
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut argument = argument.clone();
                (e1, arg) = func(argument.clone(), arg.clone())?;
                expl = cons(e1.clone(), expl.clone());
            }
            assign_variant_field!(exp => NFExpression::MULTARY; inv_arguments = expl.clone().reverse());
            exp.clone()
        },
        Deref @ UNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::UNARY).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::UNARY { operator: var_field!((*exp).operator, NFExpression::UNARY).clone(), exp: e1.clone() })}
        },
        Deref @ LBINARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), arg.clone())?;
            (e2, arg) = func(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::LBINARY).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::LBINARY).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::LBINARY { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::LBINARY).clone(), exp2: e2.clone() })}
        },
        Deref @ LUNARY { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::LUNARY).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::LUNARY).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::LUNARY { operator: var_field!((*exp).operator, NFExpression::LUNARY).clone(), exp: e1.clone() })}
        },
        Deref @ RELATION { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp1, NFExpression::RELATION).clone(), arg.clone())?;
            (e2, arg) = func(var_field!((*exp).exp2, NFExpression::RELATION).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp1, NFExpression::RELATION).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).exp2, NFExpression::RELATION).clone(),&e2.clone())) {exp.clone()} else {Arc::new(NFExpression::RELATION { exp1: e1.clone(), operator: var_field!((*exp).operator, NFExpression::RELATION).clone(), exp2: e2.clone(), index: var_field!((*exp).index, NFExpression::RELATION).clone() })}
        },
        Deref @ IF { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut e3: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).condition, NFExpression::IF).clone(), arg.clone())?;
            (e2, arg) = func(var_field!((*exp).trueBranch, NFExpression::IF).clone(), arg.clone())?;
            (e3, arg) = func(var_field!((*exp).falseBranch, NFExpression::IF).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).condition, NFExpression::IF).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).trueBranch, NFExpression::IF).clone(),&e2.clone()) && referenceEq(&var_field!((*exp).falseBranch, NFExpression::IF).clone(),&e3.clone())) {exp.clone()} else {Arc::new(NFExpression::IF { ty: var_field!((*exp).ty, NFExpression::IF).clone(), condition: e1.clone(), trueBranch: e2.clone(), falseBranch: e3.clone() })}
        },
        Deref @ CAST { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::CAST).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::CAST).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: e1.clone() })}
        },
        Deref @ BOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::BOX).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::BOX).clone(),&e1.clone())) {exp.clone()} else {r#box(e1.clone())}
        },
        Deref @ UNBOX { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::UNBOX).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).exp, NFExpression::UNBOX).clone(),&e1.clone())) {exp.clone()} else {unbox(e1.clone())}
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), arg.clone())?;
            (subs, arg) = List::mapFold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Ok(Subscript::mapFoldExpShallow(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: e1.clone(), subscripts: subs.clone(), ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: var_field!((*exp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1.clone(), index: var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone(), ty: var_field!((*exp).ty, NFExpression::TUPLE_ELEMENT).clone() })}
        },
        Deref @ RECORD_ELEMENT { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), arg.clone())?;
            if (referenceEq(&var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(NFExpression::RECORD_ELEMENT { recordExp: e1.clone(), index: var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone(), fieldName: (var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone(), ty: var_field!((*exp).ty, NFExpression::RECORD_ELEMENT).clone() })}
        },
        Deref @ MUTABLE { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), arg.clone())?;
            Mutable::update(var_field!((*exp).exp, NFExpression::MUTABLE).clone(), e1.clone());
            exp.clone()
        },
        Deref @ SHARED_LITERAL { .. } => {
            let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
            (e1, arg) = func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), arg.clone())?;
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = e1.clone());
            exp.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
            (expl, arg) = List::mapFold(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone(), arg.clone());
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = expl.clone());
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

pub fn mapFoldOptShallow<ArgT: Clone + 'static>(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> (Option<Arc<NFExpression>>, ArgT) {
    pub type MapFunc<ArgT: Clone> = fn(Arc<NFExpression>, ArgT) -> Result<(Arc<NFExpression>, ArgT)>;

    let mut outExp: Option<Arc<NFExpression>> = None;
    let mut arg: ArgT = arg;
    let mut e1: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut e2: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(e1) => {
            (e2, arg) = func(e1.clone(), arg.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e2.clone())) {exp.clone()} else {Some(e2.clone())}
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, arg)
}

pub fn containsOpt(mut exp: Option<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(e) => contains(e.clone(), func.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn contains(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    if func(exp.clone())? {
        res = true;
        return Ok(res);
    }
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => {
            ClockKind::containsExp(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone())?
        },
        Deref @ CREF { .. } => {
            ComponentRef::containsExp(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone())?
        },
        Deref @ ARRAY { .. } => {
            arrayContains(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone())?
        },
        Deref @ MATRIX { .. } => {
            res = false;
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                if listContains(row.clone(), func.clone())? {
                    res = true;
                    break;
                }
            }
            res.clone()
        },
        Deref @ RANGE { .. } => {
            contains(var_field!((*exp).start, NFExpression::RANGE).clone(), func.clone())? || containsOpt(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone())? || contains(var_field!((*exp).stop, NFExpression::RANGE).clone(), func.clone())?
        },
        Deref @ TUPLE { .. } => {
            listContains(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone())?
        },
        Deref @ RECORD { .. } => {
            listContains(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone())?
        },
        Deref @ CALL { .. } => {
            Call::containsExp(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?
        },
        Deref @ SIZE { .. } => {
            containsOpt(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone())? || contains(var_field!((*exp).exp, NFExpression::SIZE).clone(), func.clone())?
        },
        Deref @ BINARY { .. } => {
            contains(var_field!((*exp).exp1, NFExpression::BINARY).clone(), func.clone())? || contains(var_field!((*exp).exp2, NFExpression::BINARY).clone(), func.clone())?
        },
        Deref @ MULTARY { .. } => {
            res = false;
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res.clone() {
                    break;
                }
                res = contains(arg.clone(), func.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res.clone() {
                    break;
                }
                res = contains(arg.clone(), func.clone())?;
            }
            res.clone()
        },
        Deref @ UNARY { .. } => {
            contains(var_field!((*exp).exp, NFExpression::UNARY).clone(), func.clone())?
        },
        Deref @ LBINARY { .. } => {
            contains(var_field!((*exp).exp1, NFExpression::LBINARY).clone(), func.clone())? || contains(var_field!((*exp).exp2, NFExpression::LBINARY).clone(), func.clone())?
        },
        Deref @ LUNARY { .. } => {
            contains(var_field!((*exp).exp, NFExpression::LUNARY).clone(), func.clone())?
        },
        Deref @ RELATION { .. } => {
            contains(var_field!((*exp).exp1, NFExpression::RELATION).clone(), func.clone())? || contains(var_field!((*exp).exp2, NFExpression::RELATION).clone(), func.clone())?
        },
        Deref @ IF { .. } => {
            contains(var_field!((*exp).condition, NFExpression::IF).clone(), func.clone())? || contains(var_field!((*exp).trueBranch, NFExpression::IF).clone(), func.clone())? || contains(var_field!((*exp).falseBranch, NFExpression::IF).clone(), func.clone())?
        },
        Deref @ CAST { .. } => {
            contains(var_field!((*exp).exp, NFExpression::CAST).clone(), func.clone())?
        },
        Deref @ BOX { .. } => {
            contains(var_field!((*exp).exp, NFExpression::BOX).clone(), func.clone())?
        },
        Deref @ UNBOX { .. } => {
            contains(var_field!((*exp).exp, NFExpression::UNBOX).clone(), func.clone())?
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            contains(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())? || Subscript::listContainsExp(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone())?
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            contains(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone(), func.clone())?
        },
        Deref @ RECORD_ELEMENT { .. } => {
            contains(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone(), func.clone())?
        },
        Deref @ MUTABLE { .. } => {
            contains(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()), func.clone())?
        },
        Deref @ SHARED_LITERAL { .. } => {
            contains(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone(), func.clone())?
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            listContains(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn arrayContains(mut expl: metamodelica::Array<Arc<NFExpression>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if contains(e.clone(), func.clone())? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub fn listContains(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    for mut e in &*expl.clone() {
        let mut e = e.clone();
        if contains(e.clone(), func.clone())? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub fn containsShallow(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { .. } => ClockKind::containsExpShallow(var_field!((*exp).clk, NFExpression::CLKCONST).clone(), func.clone()),
        Deref @ CREF { .. } => ComponentRef::containsExpShallow(var_field!((*exp).cref, NFExpression::CREF).clone(), func.clone()),
        Deref @ ARRAY { .. } => Array::any(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone()),
        Deref @ MATRIX { .. } => {
            res = false;
            for mut row in &*var_field!((*exp).elements, NFExpression::MATRIX).clone() {
                let mut row = row.clone();
                if List::any(row.clone(), func.clone()) {
                    res = true;
                    break;
                }
            }
            res.clone()
        },
        Deref @ RANGE { .. } => func(var_field!((*exp).start, NFExpression::RANGE).clone())? || Util::applyOptionOrDefault(var_field!((*exp).step, NFExpression::RANGE).clone(), func.clone(), false) || func(var_field!((*exp).stop, NFExpression::RANGE).clone())?,
        Deref @ TUPLE { .. } => List::any(var_field!((*exp).elements, NFExpression::TUPLE).clone(), func.clone()),
        Deref @ RECORD { .. } => List::any(var_field!((*exp).elements, NFExpression::RECORD).clone(), func.clone()),
        Deref @ CALL { .. } => Call::containsExpShallow(var_field!((*exp).call, NFExpression::CALL).clone(), func.clone())?,
        Deref @ SIZE { .. } => Util::applyOptionOrDefault(var_field!((*exp).dimIndex, NFExpression::SIZE).clone(), func.clone(), false) || func(var_field!((*exp).exp, NFExpression::SIZE).clone())?,
        Deref @ BINARY { .. } => func(var_field!((*exp).exp1, NFExpression::BINARY).clone())? || func(var_field!((*exp).exp2, NFExpression::BINARY).clone())?,
        Deref @ MULTARY { .. } => {
            res = false;
            for mut arg in &*var_field!((*exp).arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res.clone() {
                    break;
                }
                res = func(arg.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                if res.clone() {
                    break;
                }
                res = func(arg.clone())?;
            }
            res.clone()
        },
        Deref @ UNARY { .. } => func(var_field!((*exp).exp, NFExpression::UNARY).clone())?,
        Deref @ LBINARY { .. } => func(var_field!((*exp).exp1, NFExpression::LBINARY).clone())? || func(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?,
        Deref @ LUNARY { .. } => func(var_field!((*exp).exp, NFExpression::LUNARY).clone())?,
        Deref @ RELATION { .. } => func(var_field!((*exp).exp1, NFExpression::RELATION).clone())? || func(var_field!((*exp).exp2, NFExpression::RELATION).clone())?,
        Deref @ IF { .. } => func(var_field!((*exp).condition, NFExpression::IF).clone())? || func(var_field!((*exp).trueBranch, NFExpression::IF).clone())? || func(var_field!((*exp).falseBranch, NFExpression::IF).clone())?,
        Deref @ CAST { .. } => func(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => func(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => func(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => func(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())? || Subscript::listContainsExpShallow(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), func.clone()),
        Deref @ TUPLE_ELEMENT { .. } => func(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?,
        Deref @ RECORD_ELEMENT { .. } => func(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?,
        Deref @ MUTABLE { .. } => func(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => func(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?,
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => listContains(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone(), func.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn arrayFirstScalar(mut arrayExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ ARRAY { .. } => arrayFirstScalar(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone().borrow()[(1-1) as usize].clone())?,
        _ => arrayExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn arrayAllEqual(mut arrayExp: Arc<NFExpression>) -> Result<bool> {
    let mut allEqual: bool = false;
    allEqual = 'mc: {
        let __mc_input = arrayExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ ARRAY { .. } => {
                    Ok(arrayAllEqual2(arrayExp.clone(), arrayFirstScalar(arrayExp.clone())?)?)
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
    Ok(allEqual)
}

pub fn arrayAllEqual2(mut arrayExp: Arc<NFExpression>, mut element: Arc<NFExpression>) -> Result<bool> {
    let mut allEqual: bool = false;
    allEqual = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ ARRAY { .. } if (!(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) && isArray(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone().borrow()[(1-1) as usize].clone())) => Array::all(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = element.clone(); move |__pe_a0| arrayAllEqual2(__pe_a0, __pe_b1.clone()) })),
        Deref @ ARRAY { .. } => Array::all(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = element.clone(); move |__pe_a0| isEqual(__pe_a0, __pe_b1.clone()) })),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(allEqual)
}

pub fn fromCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut includeScope: bool) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = Arc::new(NFExpression::CREF { ty: ComponentRef::getSubscriptedType(cref.clone(), includeScope.clone())?, cref: cref.clone() });
    Ok(exp)
}

pub fn fromTypedCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::CREF { ty: ty.clone(), cref: cref.clone() });
    exp
}

pub fn toCref(mut exp: Arc<NFExpression>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cref = __pa0.clone();
    Ok(cref)
}

pub fn extractCrefs(mut exp: Arc<NFExpression>) -> Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> {
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = fold(exp.clone(), Arc::new(extractCref), UnorderedSet::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 13)).unwrap();
    crefs
}

pub fn extractCref(mut exp: Arc<NFExpression>, mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = crefs;
    crefs = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => {
            UnorderedSet::add(var_field!((*exp).cref, NFExpression::CREF).clone(), crefs.clone())?;
            crefs.clone()
        },
        _ => crefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

pub fn isResizableCref(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isIterator(mut exp: Arc<NFExpression>) -> bool {
    let mut isIterator: bool = false;
    isIterator = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isIterator(var_field!((*exp).cref, NFExpression::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isIterator
}

pub fn containsAnyIterator(mut exp: Arc<NFExpression>, mut context: i32) -> Result<bool> {
    let mut iter: bool = false;
    if InstContext::inFor(context.clone()) {
        iter = contains(exp.clone(), Arc::new(fnptr!(isIterator, Arc<NFExpression>)))?;
    } else {
        iter = false;
    }
    Ok(iter)
}

pub fn isTime(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isTime(var_field!((*exp).cref, NFExpression::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isSubstitute(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isSubstitute(var_field!((*exp).cref, NFExpression::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isZero(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() == 0,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64),
        Deref @ CAST { .. } => isZero(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => isZero(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        Deref @ ARRAY { .. } => Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new(fnptr!(isZero, Arc<NFExpression>))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isNonZero(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool = isPositive(exp.clone()) || isNegative(exp.clone());
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isOne(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() == 1,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(1.0_f64),
        Deref @ CAST { .. } => isOne(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => isMinusOne(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        Deref @ ARRAY { .. } => Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new(fnptr!(isOne, Arc<NFExpression>))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isMinusOne(mut exp: Arc<NFExpression>) -> bool {
    let mut isOne: bool = false;
    isOne = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() == -1,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(-1.0_f64),
        Deref @ CAST { .. } => isMinusOne(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => self::isOne(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOne
}

pub fn isNaN(mut nan: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(nan.clone()) {
        Deref @ BINARY { .. } => Operator::getMathClassification(var_field!((*nan).operator, NFExpression::BINARY).clone())? == Operator::MathClassification::DIVISION.clone() && isZero(var_field!((*nan).exp1, NFExpression::BINARY).clone()) && isZero(var_field!((*nan).exp2, NFExpression::BINARY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isPositive(mut exp: Arc<NFExpression>) -> bool {
    let mut positive: bool = false;
    positive = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() > 0,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() > metamodelica::OrderedFloat(0.0_f64),
        Deref @ CAST { .. } => isPositive(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => isNegative(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        Deref @ CREF { .. } => Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("min")).clone()), Arc::new(fnptr!(isPositive, Arc<NFExpression>)), false),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    positive
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isNegative(mut exp: Arc<NFExpression>) -> bool {
    let mut negative: bool = false;
    negative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() < 0,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat(0.0_f64),
        Deref @ CAST { .. } => isNegative(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => isPositive(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        Deref @ CREF { .. } => Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("max")).clone()), Arc::new(fnptr!(isNegative, Arc<NFExpression>)), false),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    negative
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isNonPositive(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() <= 0,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() <= metamodelica::OrderedFloat(0.0_f64),
        Deref @ CAST { .. } => isNonPositive(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => isNonNegative(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        Deref @ CREF { .. } => Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("max")).clone()), Arc::new(fnptr!(isNonPositive, Arc<NFExpression>)), false),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isNonNegative(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() >= 0,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() >= metamodelica::OrderedFloat(0.0_f64),
        Deref @ CAST { .. } => isNonNegative(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => isNonPositive(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        Deref @ CREF { .. } => Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*exp).cref, NFExpression::CREF).clone(), (literal!("min")).clone()), Arc::new(fnptr!(isNonNegative, Arc<NFExpression>)), false),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isGreaterOrEqual(mut lhs: Arc<NFExpression>, mut rhs: Arc<NFExpression>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ REAL { .. }, Deref @ REAL { .. }) => var_field!((*lhs).value, NFExpression::REAL).clone() >= var_field!((*rhs).value, NFExpression::REAL).clone(),
        (Deref @ CREF { .. }, _) => Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*lhs).cref, NFExpression::CREF).clone(), (literal!("min")).clone()), Arc::new({ let __pe_b1 = rhs.clone(); move |__pe_a0| Ok(isGreaterOrEqual(__pe_a0, __pe_b1.clone())) }), false),
        (_, Deref @ CREF { .. }) => Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*rhs).cref, NFExpression::CREF).clone(), (literal!("max")).clone()), Arc::new({ let __pe_b0 = lhs.clone(); move |__pe_a1| Ok(isGreaterOrEqual(__pe_b0.clone(), __pe_a1)) }), false),
        (Deref @ UNARY { exp: Deref @ CREF { .. }, .. }, _) => isGreaterOrEqual(negate(rhs.clone()), var_field!((*lhs).exp, NFExpression::UNARY).clone()),
        (_, Deref @ UNARY { exp: Deref @ CREF { .. }, .. }) => isGreaterOrEqual(var_field!((*rhs).exp, NFExpression::UNARY).clone(), negate(lhs.clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn hasArrayType(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = Type::isArray(typeOf(exp.clone()));
    b
}

pub fn isScalar(mut exp: Arc<NFExpression>) -> bool {
    let mut scalar: bool = Type::isScalar(typeOf(exp.clone()));
    scalar
}

pub fn isScalarLiteral(mut exp: Arc<NFExpression>) -> bool {
    let mut literal: bool = false;
    literal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ REAL { .. } => true,
        Deref @ STRING { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ ENUM_LITERAL { .. } => true,
        Deref @ FILENAME { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    literal
}

pub fn isLiteral(mut exp: Arc<NFExpression>) -> bool {
    let mut literal: bool = false;
    literal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ REAL { .. } => true,
        Deref @ STRING { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ ENUM_LITERAL { .. } => true,
        Deref @ ARRAY { .. } => var_field!((*exp).literal, NFExpression::ARRAY).clone() || Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new(fnptr!(isLiteral, Arc<NFExpression>))),
        Deref @ RECORD { .. } => List::all(var_field!((*exp).elements, NFExpression::RECORD).clone(), Arc::new(fnptr!(isLiteral, Arc<NFExpression>))),
        Deref @ RANGE { .. } => isLiteral(var_field!((*exp).start, NFExpression::RANGE).clone()) && isLiteral(var_field!((*exp).stop, NFExpression::RANGE).clone()) && Util::applyOptionOrDefault(var_field!((*exp).step, NFExpression::RANGE).clone(), Arc::new(fnptr!(isLiteral, Arc<NFExpression>)), true),
        Deref @ FILENAME { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    literal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isLiteralXML(mut exp: Arc<NFExpression>) -> bool {
    let mut literal: bool = false;
    literal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => {
            true
        },
        Deref @ REAL { .. } => {
            true
        },
        Deref @ STRING { .. } => {
            true
        },
        Deref @ BOOLEAN { .. } => {
            true
        },
        Deref @ ENUM_LITERAL { .. } => {
            true
        },
        Deref @ ARRAY { .. } => {
            Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new(fnptr!(isLiteralXML, Arc<NFExpression>)))
        },
        Deref @ RECORD { .. } => {
            List::all(var_field!((*exp).elements, NFExpression::RECORD).clone(), Arc::new(fnptr!(isLiteralXML, Arc<NFExpression>)))
        },
        Deref @ RANGE { .. } => {
            isLiteralXML(var_field!((*exp).start, NFExpression::RANGE).clone()) && isLiteralXML(var_field!((*exp).stop, NFExpression::RANGE).clone()) && Util::applyOptionOrDefault(var_field!((*exp).step, NFExpression::RANGE).clone(), Arc::new(fnptr!(isLiteralXML, Arc<NFExpression>)), true)
        },
        Deref @ FILENAME { .. } => {
            true
        },
        Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: call_exp, .. } } => {
            isLiteralXML(call_exp.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    literal
}

pub fn isLiteralReplace(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ STRING { .. } => true,
        Deref @ BOX { exp: Deref @ STRING { .. } } => true,
        Deref @ RECORD { .. } => isLiteral(exp.clone()),
        Deref @ ARRAY { .. } => isLiteral(exp.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isKnownSizeFill(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut literal: bool = false;
    literal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => Call::isKnownSizeFill(var_field!((*exp).call, NFExpression::CALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(literal)
}

pub fn isInteger(mut exp: Arc<NFExpression>) -> bool {
    let mut isInteger: bool = false;
    isInteger = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isInteger
}

pub fn isReal(mut exp: Arc<NFExpression>) -> bool {
    let mut isReal: bool = false;
    isReal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ REAL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isReal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isConstNumber(mut exp: Arc<NFExpression>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => true,
        Deref @ REAL { .. } => true,
        Deref @ CAST { .. } => isConstNumber(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => isConstNumber(var_field!((*exp).exp, NFExpression::UNARY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isBoolean(mut exp: Arc<NFExpression>) -> bool {
    let mut isBool: bool = false;
    isBool = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BOOLEAN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBool
}

pub fn isRecord(mut exp: Arc<NFExpression>) -> bool {
    let mut isRecord: bool = false;
    isRecord = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecord
}

pub fn isRecordOrRecordArray(mut exp: Arc<NFExpression>) -> bool {
    let mut isRecord: bool = false;
    isRecord = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RECORD { .. } => true,
        Deref @ ARRAY { .. } => Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new(fnptr!(isRecordOrRecordArray, Arc<NFExpression>))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecord
}

pub fn fillType(mut ty: Arc<Type::NFType>, mut fillExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = fillExp.clone();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = Type::arrayDims(ty.clone());
    let mut arr_ty: Arc<Type::NFType> = Type::arrayElementType(ty.clone());
    let mut is_literal: bool = isLiteral(exp.clone());
    for mut dim in &*dims.clone().reverse() {
        let mut dim = dim.clone();
        (exp, arr_ty) = fillArray_impl(Dimension::size(dim.clone(), false)?, exp.clone(), arr_ty.clone(), is_literal.clone());
    }
    Ok(exp)
}

pub fn fillArgs(mut fillExp: Arc<NFExpression>, mut dims: Arc<metamodelica::List<Arc<NFExpression>>>) -> Result<Arc<NFExpression>> {
    let mut result: Arc<NFExpression> = fillExp.clone();
    let mut arr_ty: Arc<Type::NFType> = typeOf(result.clone());
    let mut is_literal: bool = isLiteral(fillExp.clone());
    let mut d_resizable: Arc<NFExpression> = Arc::new(NFExpression::END);
    for mut d in &*dims.clone().reverse() {
        let mut d = d.clone();
        d_resizable = map(d.clone(), Arc::new(move |__pe_a0| replaceResizableParameter(__pe_a0)))?;
        (result, arr_ty) = fillArray_impl(toInteger(d_resizable.clone())?, result.clone(), arr_ty.clone(), is_literal.clone());
    }
    Ok(result)
}

pub fn fillArray(mut n: i32, mut fillExp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut result: Arc<NFExpression> = Arc::new(NFExpression::END);
    (result, _) = fillArray_impl(n.clone(), fillExp.clone(), typeOf(fillExp.clone()), isLiteral(fillExp.clone()));
    result
}

pub fn fillArray_impl(mut n: i32, mut fillExp: Arc<NFExpression>, mut ty: Arc<Type::NFType>, mut isLiteral: bool) -> (Arc<NFExpression>, Arc<Type::NFType>) {
    let mut result: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut resultType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<NFExpression>>;
    arr = Array::generate(n.clone(), Arc::new({ let __pe_b0 = fillExp.clone(); move || Ok(clone(__pe_b0.clone())) }));
    resultType = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger(n.clone(), Prefixes::Variability::CONSTANT.clone()));
    result = makeArray(resultType.clone(), arr.clone(), isLiteral.clone());
    (result, resultType)
}

pub fn liftArray(mut dim: Arc<Dimension::NFDimension>, mut exp: Arc<NFExpression>) -> Result<(Arc<NFExpression>, Arc<Type::NFType>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut arrayType: Arc<Type::NFType> = typeOf(exp.clone());
    (exp, arrayType) = fillArray_impl(Dimension::size(dim.clone(), false)?, exp.clone(), arrayType.clone(), isLiteral(exp.clone()));
    Ok((exp, arrayType))
}

pub fn liftArrayList(mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut exp: Arc<NFExpression>) -> Result<(Arc<NFExpression>, Arc<Type::NFType>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut arrayType: Arc<Type::NFType> = typeOf(exp.clone());
    let mut is_literal: bool = isLiteral(exp.clone());
    for mut dim in &*dims.clone().reverse() {
        let mut dim = dim.clone();
        (exp, arrayType) = fillArray_impl(Dimension::size(dim.clone(), false)?, exp.clone(), arrayType.clone(), is_literal.clone());
    }
    Ok((exp, arrayType))
}

pub fn makeZero(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut zeroExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    zeroExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: 0 }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: false }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeZero(Type::arrayElementType(ty.clone()))?)?,
        Deref @ Type::COMPLEX { .. } => makeOperatorRecordZero(var_field!((*ty).cls, Type::NFType::COMPLEX).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeZero")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Type::toString(ty.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(zeroExp)
}

pub fn makeOperatorRecordZero(mut recordNode: Arc<InstNode::InstNode>) -> Result<Arc<NFExpression>> {
    let mut zeroExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut op_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut r#fn: Arc<Function::Function::Function>;
    match '__try0: {
        (op_node, _) = unwrap_break_err!(Class::lookupElement((literal!("'0'")).clone(), InstNode::getClass(recordNode.clone())?), '__try0);
        unwrap_break_err!(Function::Function::instFunctionNode(op_node.clone(), InstContext::NO_CONTEXT.clone(), InstNode::info(InstNode::parent(op_node.clone()))?), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Function::Function::typeNodeCache(op_node.clone(), InstContext::FUNCTION.clone()), '__try0)) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        r#fn = __pa1.clone();
        zeroExp = Arc::new(NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), metamodelica::nil(), Variability::CONSTANT.clone(), Purity::PURE.clone(), r#fn.returnType.clone()) });
        zeroExp = unwrap_break_err!(Ceval::evalExp(zeroExp.clone(), Ceval::noTarget().clone()), '__try0);
        Ok::<_, anyhow::Error>((r#fn.clone(), op_node.clone(), zeroExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            r#fn = __try0_o0;
            op_node = __try0_o1;
            zeroExp = __try0_o2;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeOperatorRecordZero")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*InstNode::toString(recordNode.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    Ok(zeroExp)
}

pub fn makeOne(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut oneExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    oneExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: 1 }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeOne(Type::arrayElementType(ty.clone()))?)?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeOne")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Type::toString(ty.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oneExp)
}

pub fn makeMinusOne(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut oneExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    oneExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(-1.0_f64) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: -1 }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeMinusOne(Type::arrayElementType(ty.clone()))?)?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.makeMinusOne")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Type::toString(ty.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oneExp)
}

pub fn makeNaN(mut ty: Arc<Type::NFType>) -> Arc<NFExpression> {
    let mut nan: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut zero: Arc<NFExpression> = makeZero(ty.clone()).unwrap();
    nan = Arc::new(NFExpression::BINARY { exp1: zero.clone(), operator: Operator::makeDiv(ty.clone()), exp2: zero.clone() });
    nan
}

pub fn makeMaxValue(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: System::realMaxLit() }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: System::intMaxLit() }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: true }),
        Deref @ Type::ENUMERATION { .. } => Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (List::last(var_field!((*ty).literals, Type::NFType::ENUMERATION).clone())?).clone(), index: (var_field!((*ty).literals, Type::NFType::ENUMERATION).clone().len() as i32) }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeMaxValue(Type::arrayElementType(ty.clone()))?)?,
        _ => Arc::new(NFExpression::REAL { value: System::realMaxLit() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn makeMinValue(mut ty: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => Arc::new(NFExpression::REAL { value: -(System::realMaxLit()) }),
        Deref @ Type::INTEGER => Arc::new(NFExpression::INTEGER { value: -(System::intMaxLit()) }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: false }),
        Deref @ Type::ENUMERATION { .. } => Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (listHead(var_field!((*ty).literals, Type::NFType::ENUMERATION).clone())?).clone(), index: 1 }),
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeMinValue(Type::arrayElementType(ty.clone()))?)?,
        _ => Arc::new(NFExpression::REAL { value: -(System::realMaxLit()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn makeDefaultValue(mut ty: Arc<Type::NFType>, mut min: Option<Arc<NFExpression>>, mut max: Option<Arc<NFExpression>>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => {
            if isSome(min.clone()) && isNonNegative(Util::getOption(min.clone())?) {
                let __pa0 = ::match_deref::match_deref! { match &(min.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
            } else if isSome(max.clone()) && isNonPositive(Util::getOption(max.clone())?) {
                let __pa1 = ::match_deref::match_deref! { match &(max.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa1.clone();
            } else {
                exp = Arc::new(NFExpression::INTEGER { value: 0 });
            }
            exp.clone()
        },
        Deref @ Type::REAL => {
            if isSome(min.clone()) && isNonNegative(Util::getOption(min.clone())?) {
                let __pa0 = ::match_deref::match_deref! { match &(min.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
            } else if isSome(max.clone()) && isNonPositive(Util::getOption(max.clone())?) {
                let __pa1 = ::match_deref::match_deref! { match &(max.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa1.clone();
            } else {
                exp = Arc::new(NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) });
            }
            exp.clone()
        },
        Deref @ Type::STRING => Arc::new(NFExpression::STRING { value: (literal!("")).clone() }),
        Deref @ Type::BOOLEAN => Arc::new(NFExpression::BOOLEAN { value: false }),
        Deref @ Type::ENUMERATION { .. } => {
            if isSome(min.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(min.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
            } else {
                exp = Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (listHead(var_field!((*ty).literals, Type::NFType::ENUMERATION).clone())?).clone(), index: 1 });
            }
            exp.clone()
        },
        Deref @ Type::ARRAY { .. } => fillType(ty.clone(), makeDefaultValue(Type::arrayElementType(ty.clone()), None, None)?)?,
        Deref @ Type::TUPLE { .. } => Arc::new(NFExpression::TUPLE { ty: ty.clone(), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut t in (var_field!((*ty).types, Type::NFType::TUPLE).clone()).into_iter().cloned() {
            let __x = makeDefaultValue(t.clone(), None, None)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn r#box(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut boxedExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    boxedExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ STRING { .. } => exp.clone(),
        Deref @ RECORD { .. } => Arc::new(NFExpression::RECORD { path: var_field!((*exp).path, NFExpression::RECORD).clone(), ty: Type::r#box(var_field!((*exp).ty, NFExpression::RECORD).clone()), elements: {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = r#box(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } }),
        Deref @ BOX { .. } => exp.clone(),
        Deref @ FILENAME { .. } => exp.clone(),
        _ => Arc::new(NFExpression::BOX { exp: exp.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    boxedExp
}

pub fn unbox(mut boxedExp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(boxedExp.clone()) {
        Deref @ BOX { .. } => {
            var_field!((*boxedExp).exp, NFExpression::BOX).clone()
        },
        _ => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ty = typeOf(boxedExp.clone());
            if (Type::isBoxed(ty.clone())) {Arc::new(NFExpression::UNBOX { exp: boxedExp.clone(), ty: Type::unbox(ty.clone()) })} else {boxedExp.clone()}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isNegated(mut exp: Arc<NFExpression>) -> bool {
    let mut negated: bool = false;
    negated = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => var_field!((*exp).value, NFExpression::INTEGER).clone() < 0,
        Deref @ REAL { .. } => var_field!((*exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64),
        Deref @ CAST { .. } => isNegated(var_field!((*exp).exp, NFExpression::CAST).clone()),
        Deref @ UNARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    negated
}

pub fn negate(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => Arc::new(NFExpression::INTEGER { value: -(var_field!((*exp).value, NFExpression::INTEGER).clone()) }),
        Deref @ REAL { .. } => Arc::new(NFExpression::REAL { value: -(var_field!((*exp).value, NFExpression::REAL).clone()) }),
        Deref @ CAST { .. } => Arc::new(NFExpression::CAST { ty: var_field!((*exp).ty, NFExpression::CAST).clone(), exp: negate(var_field!((*exp).exp, NFExpression::CAST).clone()) }),
        Deref @ UNARY { .. } => var_field!((*exp).exp, NFExpression::UNARY).clone(),
        _ => Arc::new(NFExpression::UNARY { operator: Operator::makeUMinus(typeOf(exp.clone())), exp: exp.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn logicNegate(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BOOLEAN { .. } => Arc::new(NFExpression::BOOLEAN { value: !(var_field!((*exp).value, NFExpression::BOOLEAN).clone()) }),
        Deref @ LUNARY { .. } => var_field!((*exp).exp, NFExpression::LUNARY).clone(),
        _ => Arc::new(NFExpression::LUNARY { operator: Operator::makeNot(typeOf(exp.clone())), exp: exp.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn revertRange(mut range: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut range: Arc<NFExpression> = range;
    range = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ RANGE { step: Some(step), .. } => {
            Arc::new(NFExpression::RANGE { ty: var_field!((*range).ty, NFExpression::RANGE).clone(), start: var_field!((*range).stop, NFExpression::RANGE).clone(), step: Some(negate(step.clone())), stop: var_field!((*range).start, NFExpression::RANGE).clone() })
        },
        Deref @ RANGE { .. } => {
            Arc::new(NFExpression::RANGE { ty: var_field!((*range).ty, NFExpression::RANGE).clone(), start: var_field!((*range).stop, NFExpression::RANGE).clone(), step: Some(Arc::new(NFExpression::INTEGER { value: -1 })), stop: var_field!((*range).start, NFExpression::RANGE).clone() })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.revertRange")); __mm_s.push_str(&*literal!(" failed because expression is not a range:\n")); __mm_s.push_str(&*toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(range)
}

pub fn sliceRange(mut range: Arc<NFExpression>, mut slice: (i32, i32, i32)) -> Result<Arc<NFExpression>> {
    let mut range: Arc<NFExpression> = range;
    range = (::match_deref::match_deref! { match &((range.clone(), slice.clone())) {
        (Deref @ RANGE { .. }, (slice_start, slice_step, slice_stop)) => {
            let mut start: i32 = 0;
            let mut step: i32 = 0;
            let mut stop: i32 = 0;
            step = Util::applyOptionOrDefault(var_field!((*range).step, NFExpression::RANGE).clone(), Arc::new(integerValue), 1);
            start = integerValue(var_field!((*range).start, NFExpression::RANGE).clone())?;
            stop = start.clone() + slice_stop.clone() * step.clone();
            start = start.clone() + slice_start.clone() * step.clone();
            step = slice_step.clone() * step.clone();
            range = Arc::new(NFExpression::RANGE { ty: var_field!((*range).ty, NFExpression::RANGE).clone(), start: Arc::new(NFExpression::INTEGER { value: start.clone() }), step: Some(Arc::new(NFExpression::INTEGER { value: step.clone() })), stop: Arc::new(NFExpression::INTEGER { value: stop.clone() }) });
            retype(range.clone())?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.sliceRange")); __mm_s.push_str(&*literal!(" failed because expression is not a range:\n")); __mm_s.push_str(&*toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(range)
}

pub fn arrayElements(mut array: Arc<NFExpression>) -> Result<metamodelica::Array<Arc<NFExpression>>> {
    let mut elements: metamodelica::Array<Arc<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &(array.clone()) {
        Deref @ ARRAY { elements: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    elements = __pa0.clone();
    Ok(elements)
}

pub fn arrayElementList(mut array: Arc<NFExpression>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    elements = (::match_deref::match_deref! { match &(array.clone()) {
        Deref @ ARRAY { .. } => Arc::new(var_field!((*array).elements, NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(elements)
}

pub fn arrayScalarElements(mut exp: Arc<NFExpression>) -> Arc<metamodelica::List<Arc<NFExpression>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    elements = arrayScalarElements_impl(exp.clone(), metamodelica::nil()).reverse();
    elements
}

pub fn arrayScalarElements_impl(mut exp: Arc<NFExpression>, mut elements: Arc<metamodelica::List<Arc<NFExpression>>>) -> Arc<metamodelica::List<Arc<NFExpression>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>> = elements;
    elements = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            let __range0 = var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range0 {
                elements = arrayScalarElements_impl(e.clone(), elements.clone());
            }
            elements.clone()
        },
        _ => cons(exp.clone(), elements.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elements
}

pub fn arrayScalarElement(mut arrayExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut scalarExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    scalarExp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ ARRAY { .. } if ((var_field!((*arrayExp).elements, NFExpression::ARRAY).clone().borrow().len() as i32) == 1) => var_field!((*arrayExp).elements, NFExpression::ARRAY).clone().borrow()[(1-1) as usize].clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(scalarExp)
}

pub fn hasArrayCall(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut hasArrayCall: bool = false;
    hasArrayCall = contains(exp.clone(), Arc::new(fnptr!(hasArrayCall2, Arc<NFExpression>)))?;
    Ok(hasArrayCall)
}

pub fn hasArrayCall2(mut exp: Arc<NFExpression>) -> bool {
    let mut hasArrayCall: bool = false;
    let mut call: Arc<Call::NFCall>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    hasArrayCall = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { call } => {
            ty = Call::typeOf(call.clone());
            Type::isArray(ty.clone()) && Call::isVectorizeable(call.clone())
        },
        Deref @ TUPLE_ELEMENT { tupleExp: Deref @ CALL { call }, .. } => {
            ty = Type::nthTupleType(Call::typeOf(call.clone()), var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone());
            Type::isArray(ty.clone()) && Call::isVectorizeable(call.clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasArrayCall
}

pub fn transposeArray(mut arrayExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut dim1: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFExpression>>>>> = metamodelica::nil();
    let mut literal: bool = false;
    let mut arr: metamodelica::Array<Arc<NFExpression>>;
    let mut matrix_arr: metamodelica::Array<metamodelica::Array<Arc<NFExpression>>>;
    outExp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: ty, dimensions: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Cons { head: dim2, tail: rest_dims } } }, elements: arr, literal } => {
            let mut arr = (*arr).clone();
            if !(arr.clone().borrow().is_empty()) {
                row_ty = Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: cons(dim1.clone(), rest_dims.clone()) });
                matrix_arr = Array::map(arr.clone(), Arc::new(arrayElements));
                matrix_arr = Array::transpose(matrix_arr.clone());
                arr = Array::map(matrix_arr.clone(), Arc::new({ let __pe_b0 = row_ty.clone(); let __pe_b2 = literal.clone(); move |__pe_a1| Ok(makeArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone())) }));
            }
            makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: cons(dim2.clone(), cons(dim1.clone(), rest_dims.clone())) }), arr.clone(), literal.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn makeIdentityMatrix(mut n: i32, mut elementType: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut matrix: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut row: metamodelica::Array<Arc<NFExpression>>;
    let mut rows: metamodelica::Array<Arc<NFExpression>>;
    let mut zero: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut one: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    zero = makeZero(elementType.clone())?;
    one = makeOne(elementType.clone())?;
    rows = metamodelica::arrayCreate(n.clone(), zero.clone());
    row_ty = Arc::new(Type::NFType::ARRAY { elementType: elementType.clone(), dimensions: list![Dimension::fromInteger(n.clone(), Prefixes::Variability::CONSTANT.clone())] });
    for mut i in 1..=n.clone() {
        row = metamodelica::arrayCreate(n.clone(), zero.clone());
        for mut j in 1..=n.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(row.clone(), j.clone(), if (i.clone() == j.clone()) {one.clone()} else {zero.clone()}) };
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(rows.clone(), i.clone(), makeArray(row_ty.clone(), row.clone(), true)) };
    }
    matrix = makeExpArray(rows.clone(), row_ty.clone(), true);
    Ok(matrix)
}

pub fn makeTriuMask(mut n: i32, mut elTy: Arc<Type::NFType>) -> Result<Arc<NFExpression>> {
    let mut mask: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut row: metamodelica::Array<Arc<NFExpression>>;
    let mut rows: metamodelica::Array<Arc<NFExpression>>;
    let mut zero: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut one: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    zero = makeZero(elTy.clone())?;
    one = makeOne(elTy.clone())?;
    rows = metamodelica::arrayCreate(n.clone(), zero.clone());
    row_ty = Arc::new(Type::NFType::ARRAY { elementType: elTy.clone(), dimensions: list![Dimension::fromInteger(n.clone(), Prefixes::Variability::CONSTANT.clone())] });
    for mut i in 1..=n.clone() {
        row = metamodelica::arrayCreate(n.clone(), zero.clone());
        for mut j in 1..=n.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(row.clone(), j.clone(), if (i.clone() <= j.clone()) {one.clone()} else {zero.clone()}) };
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(rows.clone(), i.clone(), makeArray(row_ty.clone(), row.clone(), true)) };
    }
    mask = makeExpArray(rows.clone(), row_ty.clone(), true);
    Ok(mask)
}

pub fn promote(mut e: Arc<NFExpression>, mut ty: Arc<Type::NFType>, mut n: i32) -> Result<(Arc<NFExpression>, Arc<Type::NFType>)> {
    let mut e: Arc<NFExpression> = e;
    let mut ty: Arc<Type::NFType> = ty;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut ety: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut is_array: bool = false;
    dims = {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut i in (Type::dimensionCount(ty.clone())..=n.clone() - 1).into_iter() {
            let __x = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if !(dims.clone().is_empty()) {
        dims = listAppend(Type::arrayDims(ty.clone()), dims.clone());
        is_array = Type::isArray(ty.clone());
        ety = Type::arrayElementType(ty.clone());
        ty = Type::liftArrayLeftList(ety.clone(), dims.clone());
        while !(dims.clone().is_empty()) {
            tys = cons(Type::liftArrayLeftList(ety.clone(), dims.clone()), tys.clone());
            dims = listRest(dims.clone())?;
        }
        e = promote2(e.clone(), is_array.clone(), n.clone(), tys.clone().reverse())?;
    }
    Ok((e, ty))
}

pub fn promote2(mut exp: Arc<NFExpression>, mut isArray: bool, mut dims: i32, mut types: Arc<metamodelica::List<Arc<Type::NFType>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &((exp.clone(), types.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            exp.clone()
        },
        (Deref @ ARRAY { .. }, Deref @ metamodelica::List::Cons { head: ty, tail: rest_ty }) => {
            makeArray(ty.clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = false; let __pe_b2 = dims.clone(); let __pe_b3 = rest_ty.clone(); move |__pe_a0| promote2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) })), false)
        },
        (_, _) if (isArray.clone()) => {
            let mut expanded: bool = false;
            if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? && !(isLiteral(exp.clone())) {
                expanded = false;
            } else {
                (outExp, expanded) = ExpandExp::expand(exp.clone(), false, false)?;
            }
            if expanded.clone() {
                outExp = promote2(outExp.clone(), true, dims.clone(), types.clone())?;
            } else {
                outExp = Arc::new(NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::PROMOTE().clone(), list![exp.clone(), Arc::new(NFExpression::INTEGER { value: dims.clone() })], variability(exp.clone())?, purity(exp.clone())?, listHead(types.clone())?) });
            }
            outExp.clone()
        },
        _ => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            outExp = exp.clone();
            for mut ty in &*types.clone().reverse() {
                let mut ty = ty.clone();
                outExp = makeArray(ty.clone(), arrayCreate(1, outExp.clone()), false);
            }
            outExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn variability(mut exp: Arc<NFExpression>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => Variability::CONSTANT.clone(),
        Deref @ REAL { .. } => Variability::CONSTANT.clone(),
        Deref @ STRING { .. } => Variability::CONSTANT.clone(),
        Deref @ BOOLEAN { .. } => Variability::CONSTANT.clone(),
        Deref @ ENUM_LITERAL { .. } => Variability::CONSTANT.clone(),
        Deref @ CLKCONST { .. } => Variability::DISCRETE.clone(),
        Deref @ CREF { .. } => ComponentRef::variability(var_field!((*exp).cref, NFExpression::CREF).clone()),
        Deref @ TYPENAME { .. } => Variability::CONSTANT.clone(),
        Deref @ ARRAY { .. } => variabilityArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Prefixes::Variability::CONSTANT.clone())?,
        Deref @ MATRIX { .. } => List::fold(var_field!((*exp).elements, NFExpression::MATRIX).clone(), Arc::new(variabilityList), Variability::CONSTANT.clone()),
        Deref @ RANGE { .. } => {
            var = variability(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            var = Prefixes::variabilityMax(var.clone(), variability(var_field!((*exp).stop, NFExpression::RANGE).clone())?);
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                var = Prefixes::variabilityMax(var.clone(), variability(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?)?);
            }
            var.clone()
        },
        Deref @ TUPLE { .. } => variabilityList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), Prefixes::Variability::CONSTANT.clone())?,
        Deref @ RECORD { .. } => variabilityList(var_field!((*exp).elements, NFExpression::RECORD).clone(), Prefixes::Variability::CONSTANT.clone())?,
        Deref @ CALL { .. } => Call::variability(var_field!((*exp).call, NFExpression::CALL).clone())?,
        Deref @ SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone()) {
                var = Prefixes::variabilityMax(Variability::PARAMETER.clone(), variability(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?);
            } else {
                var = Variability::PARAMETER.clone();
            }
            var.clone()
        },
        Deref @ END { .. } => Variability::PARAMETER.clone(),
        Deref @ MULTARY { .. } => Prefixes::variabilityMax(variabilityList(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), Prefixes::Variability::CONSTANT.clone())?, variabilityList(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), Prefixes::Variability::CONSTANT.clone())?),
        Deref @ BINARY { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, variability(var_field!((*exp).exp2, NFExpression::BINARY).clone())?),
        Deref @ UNARY { .. } => variability(var_field!((*exp).exp, NFExpression::UNARY).clone())?,
        Deref @ LBINARY { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, variability(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?),
        Deref @ LUNARY { .. } => variability(var_field!((*exp).exp, NFExpression::LUNARY).clone())?,
        Deref @ RELATION { .. } => Prefixes::variabilityMin(Prefixes::variabilityMax(variability(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, variability(var_field!((*exp).exp2, NFExpression::RELATION).clone())?), Variability::DISCRETE.clone()),
        Deref @ IF { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).condition, NFExpression::IF).clone())?, Prefixes::variabilityMax(variability(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, variability(var_field!((*exp).falseBranch, NFExpression::IF).clone())?)),
        Deref @ CAST { .. } => variability(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => variability(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => variability(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => Prefixes::variabilityMax(variability(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, Subscript::variabilityList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?),
        Deref @ TUPLE_ELEMENT { .. } => variability(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?,
        Deref @ RECORD_ELEMENT { .. } => variability(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?,
        Deref @ MUTABLE { .. } => variability(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => variability(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?,
        Deref @ EMPTY { .. } => Variability::CONSTANT.clone(),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => Variability::CONTINUOUS.clone(),
        Deref @ FILENAME { .. } => Variability::CONSTANT.clone(),
        Deref @ INSTANCE_NAME { .. } => Variability::CONSTANT.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.variability")); __mm_s.push_str(&*literal!(" got unknown expression.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn variabilityArray(mut expl: metamodelica::Array<Arc<NFExpression>>, mut var: Variability) -> Result<Variability> {
    let mut var: Variability = var;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        var = Prefixes::variabilityMax(var.clone(), variability(e.clone())?);
    }
    Ok(var)
}

pub fn variabilityList(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut var: Variability) -> Result<Variability> {
    let mut var: Variability = var;
    for mut e in &*expl.clone() {
        let mut e = e.clone();
        var = Prefixes::variabilityMax(var.clone(), variability(e.clone())?);
    }
    Ok(var)
}

pub fn purity(mut exp: Arc<NFExpression>) -> Result<Purity> {
    let mut pur: Purity = Purity::PURE;
    pur = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => Purity::PURE.clone(),
        Deref @ REAL { .. } => Purity::PURE.clone(),
        Deref @ STRING { .. } => Purity::PURE.clone(),
        Deref @ BOOLEAN { .. } => Purity::PURE.clone(),
        Deref @ ENUM_LITERAL { .. } => Purity::PURE.clone(),
        Deref @ CLKCONST { .. } => Purity::PURE.clone(),
        Deref @ CREF { .. } => ComponentRef::purity(var_field!((*exp).cref, NFExpression::CREF).clone()),
        Deref @ TYPENAME { .. } => Purity::PURE.clone(),
        Deref @ ARRAY { .. } => purityArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Prefixes::Purity::PURE.clone())?,
        Deref @ MATRIX { .. } => List::fold(var_field!((*exp).elements, NFExpression::MATRIX).clone(), Arc::new(purityList), Purity::PURE.clone()),
        Deref @ RANGE { .. } => {
            pur = purity(var_field!((*exp).start, NFExpression::RANGE).clone())?;
            pur = Prefixes::purityMin(pur.clone(), purity(var_field!((*exp).stop, NFExpression::RANGE).clone())?);
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                pur = Prefixes::purityMin(pur.clone(), purity(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?)?);
            }
            pur.clone()
        },
        Deref @ TUPLE { .. } => purityList(var_field!((*exp).elements, NFExpression::TUPLE).clone(), Prefixes::Purity::PURE.clone())?,
        Deref @ RECORD { .. } => purityList(var_field!((*exp).elements, NFExpression::RECORD).clone(), Prefixes::Purity::PURE.clone())?,
        Deref @ CALL { .. } => Call::purity(var_field!((*exp).call, NFExpression::CALL).clone()),
        Deref @ SIZE { .. } => if (isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())) {purity(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?} else {Purity::PURE.clone()},
        Deref @ END { .. } => Purity::PURE.clone(),
        Deref @ BINARY { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, purity(var_field!((*exp).exp2, NFExpression::BINARY).clone())?),
        Deref @ UNARY { .. } => purity(var_field!((*exp).exp, NFExpression::UNARY).clone())?,
        Deref @ LBINARY { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, purity(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?),
        Deref @ LUNARY { .. } => purity(var_field!((*exp).exp, NFExpression::LUNARY).clone())?,
        Deref @ RELATION { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, purity(var_field!((*exp).exp2, NFExpression::RELATION).clone())?),
        Deref @ MULTARY { .. } => Prefixes::purityMin(purityList(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), Prefixes::Purity::PURE.clone())?, purityList(var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone(), Prefixes::Purity::PURE.clone())?),
        Deref @ IF { .. } => Prefixes::purityMin(purity(var_field!((*exp).condition, NFExpression::IF).clone())?, Prefixes::purityMin(purity(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, purity(var_field!((*exp).falseBranch, NFExpression::IF).clone())?)),
        Deref @ CAST { .. } => purity(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => purity(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => purity(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => Prefixes::purityMin(purity(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, Subscript::purityList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?),
        Deref @ TUPLE_ELEMENT { .. } => purity(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?,
        Deref @ RECORD_ELEMENT { .. } => purity(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?,
        Deref @ MUTABLE { .. } => purity(Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()))?,
        Deref @ SHARED_LITERAL { .. } => purity(var_field!((*exp).exp, NFExpression::SHARED_LITERAL).clone())?,
        Deref @ EMPTY { .. } => Purity::PURE.clone(),
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => Purity::PURE.clone(),
        Deref @ FILENAME { .. } => Purity::PURE.clone(),
        Deref @ INSTANCE_NAME { .. } => Purity::PURE.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.purity")); __mm_s.push_str(&*literal!(" got unknown expression.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(pur)
}

pub fn purityArray(mut expl: metamodelica::Array<Arc<NFExpression>>, mut pur: Purity) -> Result<Purity> {
    let mut pur: Purity = pur;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        pur = Prefixes::purityMin(pur.clone(), purity(e.clone())?);
    }
    Ok(pur)
}

pub fn purityList(mut expl: Arc<metamodelica::List<Arc<NFExpression>>>, mut pur: Purity) -> Result<Purity> {
    let mut pur: Purity = pur;
    for mut e in &*expl.clone() {
        let mut e = e.clone();
        pur = Prefixes::purityMin(pur.clone(), purity(e.clone())?);
    }
    Ok(pur)
}

pub fn makeMutable(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = Arc::new(NFExpression::MUTABLE { exp: Mutable::create(exp.clone()) });
    outExp
}

pub fn makeImmutable(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ MUTABLE { .. } => Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn isMutable(mut exp: Arc<NFExpression>) -> bool {
    let mut isMutable: bool = false;
    isMutable = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ MUTABLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isMutable
}

pub fn updateMutable(mut mutableExp: Arc<NFExpression>, mut value: Arc<NFExpression>) -> Result<()> {
    let mut exp_ptr: Mutable::Mutable<Arc<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &(mutableExp.clone()) {
        Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp_ptr = __pa0.clone();
    Mutable::update(exp_ptr.clone(), value.clone());
    Ok(())
}

pub fn applyMutable(mut mutableExp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<()> {
    pub type FuncType = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut exp_ptr: Mutable::Mutable<Arc<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &(mutableExp.clone()) {
        Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp_ptr = __pa0.clone();
    Mutable::update(exp_ptr.clone(), func(Mutable::access(exp_ptr.clone()))?);
    Ok(())
}

pub fn isEmpty(mut exp: Arc<NFExpression>) -> bool {
    let mut empty: bool = false;
    empty = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    empty
}

pub fn isEnd(mut exp: Arc<NFExpression>) -> bool {
    let mut isend: bool = false;
    isend = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ END { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isend
}

pub fn enumIndexExp(mut enumExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut indexExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    indexExp = (::match_deref::match_deref! { match &(enumExp.clone()) {
        Deref @ ENUM_LITERAL { .. } => Arc::new(NFExpression::INTEGER { value: var_field!((*enumExp).index, NFExpression::ENUM_LITERAL).clone() }),
        _ => Arc::new(NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INTEGER_ENUM().clone(), list![enumExp.clone()], variability(enumExp.clone())?, Purity::PURE.clone(), NFBuiltinFuncs::INTEGER_ENUM().returnType.clone()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(indexExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toScalar(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } if ((var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().len() as i32) == 1) => toScalar(var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow()[(1-1) as usize].clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn tupleElement(mut exp: Arc<NFExpression>, mut ty: Arc<Type::NFType>, mut index: i32) -> Result<Arc<NFExpression>> {
    let mut tupleElem: Arc<NFExpression> = Arc::new(NFExpression::END);
    tupleElem = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ TUPLE { .. } => {
            (var_field!((*exp).elements, NFExpression::TUPLE).clone()).get(index.clone())?
        },
        Deref @ ARRAY { .. } => {
            let mut ety: Arc<Type::NFType> = Arc::new(Type::ANY);
            ety = Type::unliftArray(ty.clone())?;
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = ety.clone(); let __pe_b2 = index.clone(); move |__pe_a0| tupleElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })));
            exp.clone()
        },
        Deref @ SUBSCRIPTED_EXP { split: true, .. } => {
            mapSplitExpressions(exp.clone(), Arc::new({ let __pe_b1 = ty.clone(); let __pe_b2 = index.clone(); move |__pe_a0| tupleElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?
        },
        _ => {
            Arc::new(NFExpression::TUPLE_ELEMENT { tupleExp: exp.clone(), index: index.clone(), ty: ty.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tupleElem)
}

pub fn recordElement(mut elementName: ArcStr, mut recordExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(recordExp.clone()) {
        Deref @ RECORD { ty: Deref @ Type::COMPLEX { cls: node, .. }, .. } => {
            let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            let mut index: i32 = 0;
            cls = InstNode::getClass(node.clone())?;
            index = Class::lookupComponentIndex((elementName.clone()).clone(), cls.clone())?;
            (var_field!((*recordExp).elements, NFExpression::RECORD).clone()).get(index.clone())?
        },
        Deref @ CREF { .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let __pa0 = ::match_deref::match_deref! { match &(Type::arrayElementType(var_field!((*recordExp).ty, NFExpression::CREF).clone())) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            cls_tree = Class::classTree(InstNode::getClass(node.clone())?)?;
            let __pa1 = ::match_deref::match_deref! { match &(ClassTree::lookupElement((elementName.clone()).clone(), cls_tree.clone())?) {
                (__pa1, false) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa1.clone();
            ty = InstNode::getType(node.clone())?;
            cref = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), var_field!((*recordExp).cref, NFExpression::CREF).clone());
            ty = Type::liftArrayLeftList(ty.clone(), Type::arrayDims(var_field!((*recordExp).ty, NFExpression::CREF).clone()));
            Arc::new(NFExpression::CREF { ty: ty.clone(), cref: cref.clone() })
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } if (var_field!((*recordExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) => {
            let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut index: i32 = 0;
            cls = InstNode::getClass(node.clone())?;
            index = Class::lookupComponentIndex((elementName.clone()).clone(), cls.clone())?;
            ty = InstNode::getType(Class::nthComponent(index.clone(), cls.clone())?)?;
            ty = Type::liftArrayLeftList(ty.clone(), Type::arrayDims(var_field!((*recordExp).ty, NFExpression::ARRAY).clone()));
            makeEmptyArray(ty.clone())
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut index: i32 = 0;
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            index = Class::lookupComponentIndex((elementName.clone()).clone(), InstNode::getClass(node.clone())?)?;
            arr = Array::map(var_field!((*recordExp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = index.clone(); move |__pe_a1| nthRecordElement(__pe_b0.clone(), __pe_a1) }));
            ty = Type::liftArrayLeft(typeOf(arr.clone().borrow()[(1-1) as usize].clone()), Dimension::fromInteger((arr.clone().borrow().len() as i32), Prefixes::Variability::CONSTANT.clone()));
            makeArray(ty.clone(), arr.clone(), var_field!((*recordExp).literal, NFExpression::ARRAY).clone())
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            outExp = recordElement((elementName.clone()).clone(), var_field!((*recordExp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?;
            ty = Type::subscript(typeOf(outExp.clone()), var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), true)?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: outExp.clone(), subscripts: var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), ty: ty.clone(), split: var_field!((*recordExp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ EMPTY { .. } => {
            bail!("fail")
        },
        _ => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut index: i32 = 0;
            ty = typeOf(recordExp.clone());
            let __pa0 = ::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            cls = InstNode::getClass(node.clone())?;
            index = Class::lookupComponentIndex((elementName.clone()).clone(), cls.clone())?;
            ty = Type::liftArrayLeftList(InstNode::getType(Class::nthComponent(index.clone(), cls.clone())?)?, Type::arrayDims(ty.clone()));
            Arc::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index.clone(), fieldName: (elementName.clone()).clone(), ty: ty.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn nthRecordElement(mut index: i32, mut recordExp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(recordExp.clone()) {
        Deref @ RECORD { .. } => {
            (var_field!((*recordExp).elements, NFExpression::RECORD).clone()).get(index.clone())?
        },
        Deref @ CREF { .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let __pa0 = ::match_deref::match_deref! { match &(Type::arrayElementType(typeOf(recordExp.clone()))) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            node = Class::nthComponent(index.clone(), InstNode::getClass(node.clone())?)?;
            fromCref(ComponentRef::prefixCref(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), var_field!((*recordExp).cref, NFExpression::CREF).clone()), false)?
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } if (var_field!((*recordExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) => {
            makeEmptyArray(InstNode::getType(Class::nthComponent(index.clone(), InstNode::getClass(node.clone())?)?)?)
        },
        Deref @ ARRAY { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut arr: metamodelica::Array<Arc<NFExpression>>;
            arr = Array::map(var_field!((*recordExp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = index.clone(); move |__pe_a1| nthRecordElement(__pe_b0.clone(), __pe_a1) }));
            ty = Type::liftArrayLeft(typeOf(arr.clone().borrow()[(1-1) as usize].clone()), listHead(Type::arrayDims(var_field!((*recordExp).ty, NFExpression::ARRAY).clone()))?);
            makeArray(ty.clone(), arr.clone(), false)
        },
        Deref @ RECORD_ELEMENT { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { cls: node, .. }, .. }, .. } => {
            let mut node = (*node).clone();
            node = Class::nthComponent(index.clone(), InstNode::getClass(node.clone())?)?;
            Arc::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index.clone(), fieldName: (InstNode::name(node.clone())?).clone(), ty: Type::liftArrayLeftList(InstNode::getType(node.clone())?, Type::arrayDims(var_field!((*recordExp).ty, NFExpression::RECORD_ELEMENT).clone())) })
        },
        Deref @ SUBSCRIPTED_EXP { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            outExp = nthRecordElement(index.clone(), var_field!((*recordExp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?;
            ty = Type::subscript(typeOf(outExp.clone()), var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), true)?;
            Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: outExp.clone(), subscripts: var_field!((*recordExp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), ty: ty.clone(), split: var_field!((*recordExp).split, NFExpression::SUBSCRIPTED_EXP).clone() })
        },
        Deref @ IF { .. } => {
            let mut trueBranch: Arc<NFExpression> = Arc::new(NFExpression::END);
            let mut falseBranch: Arc<NFExpression> = Arc::new(NFExpression::END);
            trueBranch = nthRecordElement(index.clone(), var_field!((*recordExp).trueBranch, NFExpression::IF).clone())?;
            falseBranch = nthRecordElement(index.clone(), var_field!((*recordExp).falseBranch, NFExpression::IF).clone())?;
            Arc::new(NFExpression::IF { ty: typeOf(trueBranch.clone()), condition: var_field!((*recordExp).condition, NFExpression::IF).clone(), trueBranch: trueBranch.clone(), falseBranch: falseBranch.clone() })
        },
        _ => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let __pa0 = ::match_deref::match_deref! { match &(typeOf(recordExp.clone())) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            node = Class::nthComponent(index.clone(), InstNode::getClass(node.clone())?)?;
            Arc::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index.clone(), fieldName: (InstNode::name(node.clone())?).clone(), ty: InstNode::getType(node.clone())? })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn getRecordElements(mut exp: Arc<NFExpression>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut elements: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Type::arrayElementType(typeOf(exp.clone()));
    elements = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::COMPLEX { complexTy: complexTy @ Deref @ ComplexType::RECORD { .. }, .. } => {
            let __range0 = (1..=(var_field!((**complexTy).fields, ComplexType::NFComplexType::RECORD).clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                elements = cons(recordElement((Record::Field::name(var_field!((**complexTy).fields, ComplexType::NFComplexType::RECORD).borrow()[(i.clone()-1) as usize].clone())?).clone(), exp.clone())?, elements.clone());
            }
            elements.clone()
        },
        _ => {
            elements.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

pub fn retype(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RANGE { .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = TypeCheck::getRangeType(var_field!((*exp).start, NFExpression::RANGE).clone(), var_field!((*exp).step, NFExpression::RANGE).clone(), var_field!((*exp).stop, NFExpression::RANGE).clone(), typeOf(var_field!((*exp).start, NFExpression::RANGE).clone()), Absyn::dummyInfo.clone())?);
            ()
        },
        Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::retype(var_field!((*exp).call, NFExpression::CALL).clone()));
            ()
        },
        _ => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ty = typeOf(exp.clone());
            if Type::isConditionalArray(ty.clone()) {
                ty = Type::simplifyConditionalArray(ty.clone());
                exp = setType(ty.clone(), exp.clone())?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn nthEnumLiteral(mut ty: Arc<Type::NFType>, mut n: i32) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = Arc::new(NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (Type::nthEnumLiteral(ty.clone(), n.clone())?).clone(), index: n.clone() });
    Ok(exp)
}

pub fn createIterationRanges(mut exp: Arc<NFExpression>, mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<NFExpression>)>>) -> Result<(Arc<NFExpression>, Arc<metamodelica::List<Arc<NFExpression>>>, Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut ranges: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut iters: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>> = metamodelica::nil();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut range: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut iter: Mutable::Mutable<Arc<NFExpression>>;
    for mut i in &*iterators.clone() {
        let mut i = i.clone();
        (node, range) = i.clone();
        iter = Mutable::create(Arc::new(NFExpression::INTEGER { value: 0 }));
        ranges = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut r in (ranges.clone()).into_iter().cloned() {
            let __x = replaceIterator(r.clone(), node.clone(), Arc::new(NFExpression::MUTABLE { exp: iter.clone() }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        exp = replaceIterator(exp.clone(), node.clone(), Arc::new(NFExpression::MUTABLE { exp: iter.clone() }))?;
        iters = cons(iter.clone(), iters.clone());
        ranges = cons(range.clone(), ranges.clone());
    }
    Ok((exp, ranges, iters))
}

pub fn foldReduction(mut exp: Arc<NFExpression>, mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<NFExpression>)>>, mut foldExp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    pub type FoldFn = fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut result: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut range: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut iter: Mutable::Mutable<Arc<NFExpression>>;
    let mut ranges: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut iters: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>> = metamodelica::nil();
    (e, ranges, iters) = createIterationRanges(exp.clone(), iterators.clone())?;
    result = foldReduction2(e.clone(), ranges.clone(), iters.clone(), foldExp.clone(), mapFn.clone(), foldFn.clone())?;
    Ok(result)
}

pub fn foldReduction2(mut exp: Arc<NFExpression>, mut ranges: Arc<metamodelica::List<Arc<NFExpression>>>, mut iterators: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>>, mut foldExp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    pub type FoldFn = fn(Arc<NFExpression>, Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut result: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut range: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut value: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut ranges_rest: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut el: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut iter: Mutable::Mutable<Arc<NFExpression>>;
    let mut iters_rest: Arc<metamodelica::List<Mutable::Mutable<Arc<NFExpression>>>> = metamodelica::nil();
    let mut range_iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
    if ranges.clone().is_empty() {
        result = foldFn(foldExp.clone(), mapFn(exp.clone())?)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ranges.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa0.clone();
        ranges_rest = __pa1.clone();
        range = Ceval::evalExp(range.clone(), Ceval::noTarget().clone())?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(iterators.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        iter = __pa2.clone();
        iters_rest = __pa3.clone();
        range_iter = ExpressionIterator::fromExp(range.clone(), false, false)?;
        result = foldExp.clone();
        while ExpressionIterator::hasNext(range_iter.clone())? {
            (range_iter, value) = ExpressionIterator::next(range_iter.clone())?;
            Mutable::update(iter.clone(), value.clone());
            result = foldReduction2(exp.clone(), ranges_rest.clone(), iters_rest.clone(), result.clone(), mapFn.clone(), foldFn.clone())?;
        }
    }
    Ok(result)
}

pub fn isPure(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut isPure: bool = false;
    isPure = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => !(ComponentRef::isIterator(var_field!((*exp).cref, NFExpression::CREF).clone())),
        Deref @ CALL { .. } => (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(Call::functionName(var_field!((*exp).call, NFExpression::CALL).clone())?)?) {
        Deref @ "Connections" => false,
        Deref @ "cardinality" => false,
        _ => !(Call::isImpure(var_field!((*exp).call, NFExpression::CALL).clone())?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isPure)
}

pub fn containsCref(mut exp: Arc<NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    b = fold(exp.clone(), Arc::new({ let __pe_b2 = cref.clone(); move |__pe_a0, __pe_a1| isCrefEqual(__pe_a0, __pe_a1, __pe_b2.clone()) }), false)?;
    Ok(b)
}

pub fn isCrefEqual(mut exp: Arc<NFExpression>, mut b: bool, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = b;
    b = (::match_deref::match_deref! { match &((b.clone(), exp.clone())) {
        (false, Deref @ CREF { .. }) => ComponentRef::isEqual(var_field!((*exp).cref, NFExpression::CREF).clone(), cref.clone())?,
        _ => b.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn containsCrefSet(mut exp: Arc<NFExpression>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut b: bool = false;
    b = fold(exp.clone(), Arc::new({ let __pe_b2 = set.clone(); move |__pe_a0, __pe_a1| isCrefEqualSet(__pe_a0, __pe_a1, __pe_b2.clone()) }), false)?;
    Ok(b)
}

pub fn isCrefEqualSet(mut exp: Arc<NFExpression>, mut b: bool, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut b: bool = b;
    b = (::match_deref::match_deref! { match &((b.clone(), exp.clone())) {
        (false, Deref @ CREF { .. }) => UnorderedSet::contains(var_field!((*exp).cref, NFExpression::CREF).clone(), set.clone())?,
        _ => b.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn filterSplitIndices(mut exp: Arc<NFExpression>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { subscripts: subs, exp: e, .. } => {
            let mut subs = (*subs).clone();
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            if !(!(filterSplitIndices2(s.clone(), node.clone()))) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if (subs.clone().is_empty()) {var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone()} else if (Type::isUnknown(var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone())) {Arc::new(NFExpression::SUBSCRIPTED_EXP { exp: var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), subscripts: subs.clone(), ty: var_field!((*exp).ty, NFExpression::SUBSCRIPTED_EXP).clone(), split: List::any(subs.clone(), Arc::new(fnptr!(Subscript::isSplit, Arc<Subscript::NFSubscript>))) })} else {applySubscripts(subs.clone(), var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?}
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn filterSplitIndices2(mut sub: Arc<Subscript::NFSubscript>, mut node: Arc<InstNode::InstNode>) -> bool {
    let mut matching: bool = false;
    matching = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => InstNode::refEqual(var_field!((*sub).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), node.clone()),
        Deref @ Subscript::SPLIT_PROXY { .. } => InstNode::refEqual(var_field!((*sub).parent, Subscript::NFSubscript::SPLIT_PROXY).clone(), node.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    matching
}

pub fn expandSplitIndices(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { .. } => applySubscripts(Subscript::expandSplitIndices(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), metamodelica::nil())?, var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?,
        Deref @ CREF { .. } => {
            assign_variant_field!(exp => NFExpression::CREF; cref = ComponentRef::expandSplitSubscripts(var_field!((*exp).cref, NFExpression::CREF).clone())?);
            exp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn expandNonListedSplitIndices(mut exp: Arc<NFExpression>, mut indicesToKeep: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<NFExpression>> {
    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { split: true, .. } => applySubscripts(Subscript::expandSplitIndices(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), indicesToKeep.clone())?, var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn isSplitSubscriptedExp(mut exp: Arc<NFExpression>) -> bool {
    let mut split: bool = false;
    split = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { split, .. } => split.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    split
}

pub fn mapSplitExpressions(mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type Func = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut osub_repls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>> = None;
    let mut sub_repls: Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut sub_exps: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut dim_sizes: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    (outExp, osub_repls) = mapFold(exp.clone(), Arc::new(replaceSplitSubscripts), None)?;
    if isNone(osub_repls.clone()) {
        outExp = func(exp.clone())?;
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(osub_repls.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        sub_repls = __pa0.clone();
        subs = UnorderedMap::keyList(sub_repls.clone());
        sub_exps = UnorderedMap::valueList(sub_repls.clone());
        dim_sizes = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Subscript::splitIndexDimExp(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        dim_sizes = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut d in (dim_sizes.clone()).into_iter().cloned() {
            let __x = (replaceSplitSubscripts(d.clone(), Some(sub_repls.clone()))?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        outExp = mapSplitExpressions2(outExp.clone(), dim_sizes.clone(), sub_exps.clone(), func.clone())?;
        outExp = applySubscripts(subs.clone(), outExp.clone(), false)?;
    }
    Ok(outExp)
}

pub fn replaceSplitSubscripts(mut exp: Arc<NFExpression>, mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>) -> Result<(Arc<NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>)> {
    let mut exp: Arc<NFExpression> = exp;
    let mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>> = subRepls;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SUBSCRIPTED_EXP { split: true, .. } => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            (subs, subRepls) = List::mapFold(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone(), Arc::new(replaceSplitSubscripts2), subRepls.clone());
            applySubscripts(subs.clone(), var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, subRepls))
}

pub fn replaceSplitSubscripts2(mut subscript: Arc<Subscript::NFSubscript>, mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>) -> Result<(Arc<Subscript::NFSubscript>, Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>>)> {
    let mut subscript: Arc<Subscript::NFSubscript> = subscript;
    let mut subRepls: Option<Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>> = subRepls;
    let mut sub_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut sub_repls: Arc<UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<NFExpression>>>;
    subscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            if isSome(subRepls.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(subRepls.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                sub_repls = __pa0.clone();
            } else {
                sub_repls = UnorderedMap::new(fnptr!(Subscript::hash, Arc<Subscript::NFSubscript>), Subscript::isEqual, 1);
                subRepls = Some(sub_repls.clone());
            }
            sub_exp = makeMutable(Arc::new(NFExpression::INTEGER { value: 0 }));
            sub_exp = UnorderedMap::tryAdd(subscript.clone(), sub_exp.clone(), sub_repls.clone())?;
            Arc::new(Subscript::NFSubscript::INDEX { index: sub_exp.clone() })
        },
        _ => subscript.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((subscript, subRepls))
}

pub fn mapSplitExpressions2(mut exp: Arc<NFExpression>, mut dimSizes: Arc<metamodelica::List<Arc<NFExpression>>>, mut subExps: Arc<metamodelica::List<Arc<NFExpression>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type Func = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut dim_size: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut rest_dims: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut dim_size_int: i32 = 0;
    let mut sub_exp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut rest_subs: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut expl: metamodelica::Array<Arc<NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if dimSizes.clone().is_empty() {
        outExp = map(exp.clone(), Arc::new(mapSplitExpressions3))?;
        outExp = func(outExp.clone())?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dimSizes.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim_size = __pa0.clone();
        rest_dims = __pa1.clone();
        dim_size_int = toInteger(Ceval::evalExp(dim_size.clone(), Ceval::noTarget().clone())?)?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(subExps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        sub_exp = __pa2.clone();
        rest_subs = __pa3.clone();
        expl = metamodelica::arrayCreate(dim_size_int.clone(), exp.clone());
        for mut i in 1..=dim_size_int.clone() {
            updateMutable(sub_exp.clone(), Arc::new(NFExpression::INTEGER { value: i.clone() }))?;
            unsafe { metamodelica::Dangerous::arrayInitSlot(expl.clone(), i.clone(), mapSplitExpressions2(exp.clone(), rest_dims.clone(), rest_subs.clone(), func.clone())?) };
        }
        ty = typeOf(if (expl.clone().borrow().is_empty()) {exp.clone()} else {expl.clone().borrow()[(1-1) as usize].clone()});
        outExp = makeExpArray(expl.clone(), ty.clone(), Array::all(expl.clone(), Arc::new(fnptr!(isLiteral, Arc<NFExpression>))));
    }
    Ok(outExp)
}

pub fn mapSplitExpressions3(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ MUTABLE { .. } => Mutable::access(var_field!((*exp).exp, NFExpression::MUTABLE).clone()),
        Deref @ SUBSCRIPTED_EXP { subscripts: subs, .. } => applySubscripts(subs.clone(), var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone(), false)?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn mapCrefScalars(mut crefExp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    (outExp, _) = ExpandExp::expand(crefExp.clone(), false, false)?;
    outExp = mapCrefScalars2(outExp.clone(), mapFn.clone())?;
    Ok(outExp)
}

pub fn mapCrefScalars2(mut exp: Arc<NFExpression>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type MapFn = fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>>;

    let mut outExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut literal: bool = false;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut arr: metamodelica::Array<Arc<NFExpression>>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } if (!(var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().is_empty())) => {
            arr = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<NFExpression>> + 'static> = mapFn.clone(); move |__pe_a0| mapCrefScalars2(__pe_a0, __pe_b1.clone()) }));
            ty = typeOf(arr.clone().borrow()[(1-1) as usize].clone());
            literal = Array::all(arr.clone(), Arc::new(fnptr!(isLiteral, Arc<NFExpression>)));
            makeExpArray(arr.clone(), ty.clone(), literal.clone())
        },
        Deref @ CREF { .. } => mapFn(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn isFunctionPointer(mut exp: Arc<NFExpression>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { ty: Deref @ Type::FUNCTION { .. }, .. } => true,
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isClockOrSampleFunction(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: _ }, .. } } => {
            (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "sample" => !(isLiteral(arg.clone())),
        Deref @ "subSample" => true,
        Deref @ "superSample" => true,
        Deref @ "shiftSample" => true,
        Deref @ "backSample" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ CLKCONST { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isConnector(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => {
            node = ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF).clone())?;
            InstNode::isComponent(node.clone()) && InstNode::isConnector(node.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isComponentExpression(mut exp: Arc<NFExpression>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } => ComponentRef::isCref(var_field!((*exp).cref, NFExpression::CREF).clone()) && InstNode::isComponent(ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF).clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn clone(mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut exp: Arc<NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = metamodelica::arrayFromVec(var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn toJSON(mut exp: Arc<NFExpression>) -> Result<Arc<JSON::JSON>> {
    fn dump_arg(mut name: ArcStr, mut arg: Arc<NFExpression>) -> Result<Arc<JSON::JSON>> {
        let mut json: Arc<JSON::JSON> = JSON::emptyListObject();
        json = JSON::addPair((literal!("name")).clone(), JSON::makeString((name.clone()).clone()), json.clone())?;
        json = JSON::addPair((literal!("value")).clone(), toJSON(arg.clone())?, json.clone())?;
        Ok(json)
    }

    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ INTEGER { .. } => JSON::makeInteger(var_field!((*exp).value, NFExpression::INTEGER).clone()),
        Deref @ REAL { .. } => JSON::makeNumber(var_field!((*exp).value, NFExpression::REAL).clone()),
        Deref @ STRING { .. } => JSON::makeString((var_field!((*exp).value, NFExpression::STRING).clone()).clone()),
        Deref @ BOOLEAN { .. } => JSON::makeBoolean(var_field!((*exp).value, NFExpression::BOOLEAN).clone()),
        Deref @ ENUM_LITERAL { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("enum")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((toString(exp.clone())?).clone()), json.clone())?;
            json = JSON::addPair((literal!("index")).clone(), JSON::makeInteger(var_field!((*exp).index, NFExpression::ENUM_LITERAL).clone()), json.clone())?;
            json.clone()
        },
        Deref @ CLKCONST { .. } => ClockKind::toJSON(var_field!((*exp).clk, NFExpression::CLKCONST).clone())?,
        Deref @ CREF { .. } => ComponentRef::toJSON(var_field!((*exp).cref, NFExpression::CREF).clone())?,
        Deref @ TYPENAME { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("typename")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((Type::toString(var_field!((*exp).ty, NFExpression::TYPENAME).clone())?).clone()), json.clone())?;
            json.clone()
        },
        Deref @ ARRAY { .. } => JSON::makeList({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toJSON(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ RANGE { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("range")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("start")).clone(), toJSON(var_field!((*exp).start, NFExpression::RANGE).clone())?, json.clone())?;
            if isSome(var_field!((*exp).step, NFExpression::RANGE).clone()) {
                json = JSON::addPair((literal!("step")).clone(), toJSON(Util::getOption(var_field!((*exp).step, NFExpression::RANGE).clone())?)?, json.clone())?;
            }
            json = JSON::addPair((literal!("stop")).clone(), toJSON(var_field!((*exp).stop, NFExpression::RANGE).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ TUPLE { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("tuple")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("elements")).clone(), JSON::makeList({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = toJSON(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), json.clone())?;
            json.clone()
        },
        Deref @ RECORD { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("record")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((AbsynUtil::pathString(var_field!((*exp).path, NFExpression::RECORD).clone(), (literal!(".")).clone(), true, false)?).clone()), json.clone())?;
            json = JSON::addPair((literal!("elements")).clone(), JSON::makeList({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = toJSON(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), json.clone())?;
            json.clone()
        },
        Deref @ CALL { .. } => Call::toJSON(var_field!((*exp).call, NFExpression::CALL).clone())?,
        Deref @ SIZE { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("call")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("size")).clone() }), json.clone())?;
            if isSome(var_field!((*exp).dimIndex, NFExpression::SIZE).clone()) {
                json = JSON::addPair((literal!("arguments")).clone(), JSON::makeList(list![toJSON(var_field!((*exp).exp, NFExpression::SIZE).clone())?, toJSON(Util::getOption(var_field!((*exp).dimIndex, NFExpression::SIZE).clone())?)?]), json.clone())?;
            } else {
                json = JSON::addPair((literal!("arguments")).clone(), JSON::makeArray(list![toJSON(var_field!((*exp).exp, NFExpression::SIZE).clone())?]), json.clone())?;
            }
            json.clone()
        },
        Deref @ BINARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("binary_op")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("lhs")).clone(), toJSON(var_field!((*exp).exp1, NFExpression::BINARY).clone())?, json.clone())?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::BINARY).clone()), json.clone())?;
            json = JSON::addPair((literal!("rhs")).clone(), toJSON(var_field!((*exp).exp2, NFExpression::BINARY).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ UNARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("unary_op")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::UNARY).clone()), json.clone())?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).exp, NFExpression::UNARY).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ LBINARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("binary_op")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("lhs")).clone(), toJSON(var_field!((*exp).exp1, NFExpression::LBINARY).clone())?, json.clone())?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::LBINARY).clone()), json.clone())?;
            json = JSON::addPair((literal!("rhs")).clone(), toJSON(var_field!((*exp).exp2, NFExpression::LBINARY).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ LUNARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("unary_op")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::LUNARY).clone()), json.clone())?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).exp, NFExpression::LUNARY).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ RELATION { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("binary_op")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("lhs")).clone(), toJSON(var_field!((*exp).exp1, NFExpression::RELATION).clone())?, json.clone())?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::RELATION).clone()), json.clone())?;
            json = JSON::addPair((literal!("rhs")).clone(), toJSON(var_field!((*exp).exp2, NFExpression::RELATION).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ MULTARY { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("multary_op")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("args")).clone(), JSON::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut a in (var_field!((*exp).arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = toJSON(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), json.clone())?;
            json = JSON::addPair((literal!("inv_args")).clone(), JSON::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut a in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = toJSON(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), json.clone())?;
            json = JSON::addPair((literal!("op")).clone(), Operator::toJSON(var_field!((*exp).operator, NFExpression::MULTARY).clone()), json.clone())?;
            json.clone()
        },
        Deref @ IF { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("if")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("condition")).clone(), toJSON(var_field!((*exp).condition, NFExpression::IF).clone())?, json.clone())?;
            json = JSON::addPair((literal!("true")).clone(), toJSON(var_field!((*exp).trueBranch, NFExpression::IF).clone())?, json.clone())?;
            json = JSON::addPair((literal!("false")).clone(), toJSON(var_field!((*exp).falseBranch, NFExpression::IF).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ CAST { .. } => toJSON(var_field!((*exp).exp, NFExpression::CAST).clone())?,
        Deref @ BOX { .. } => toJSON(var_field!((*exp).exp, NFExpression::BOX).clone())?,
        Deref @ UNBOX { .. } => toJSON(var_field!((*exp).exp, NFExpression::UNBOX).clone())?,
        Deref @ SUBSCRIPTED_EXP { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("sub")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).exp, NFExpression::SUBSCRIPTED_EXP).clone())?, json.clone())?;
            json = JSON::addPair((literal!("subscripts")).clone(), Subscript::toJSONList(var_field!((*exp).subscripts, NFExpression::SUBSCRIPTED_EXP).clone())?, json.clone())?;
            json.clone()
        },
        Deref @ TUPLE_ELEMENT { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("tuple_element")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).tupleExp, NFExpression::TUPLE_ELEMENT).clone())?, json.clone())?;
            json = JSON::addPair((literal!("index")).clone(), JSON::makeInteger(var_field!((*exp).index, NFExpression::TUPLE_ELEMENT).clone()), json.clone())?;
            json.clone()
        },
        Deref @ RECORD_ELEMENT { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("record_element")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("exp")).clone(), toJSON(var_field!((*exp).recordExp, NFExpression::RECORD_ELEMENT).clone())?, json.clone())?;
            json = JSON::addPair((literal!("index")).clone(), JSON::makeInteger(var_field!((*exp).index, NFExpression::RECORD_ELEMENT).clone()), json.clone())?;
            json = JSON::addPair((literal!("field")).clone(), JSON::makeString((var_field!((*exp).fieldName, NFExpression::RECORD_ELEMENT).clone()).clone()), json.clone())?;
            json.clone()
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("function")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((ComponentRef::toString(var_field!((*exp).r#fn, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())?).clone()), json.clone())?;
            json = JSON::addPair((literal!("arguments")).clone(), JSON::makeList({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for (arg, name) in (&(var_field!((*exp).args, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())).into_iter().zip((&(var_field!((*exp).argNames, NFExpression::PARTIAL_FUNCTION_APPLICATION).clone())).into_iter()) {
            let __x = dump_arg((name.clone()).clone(), arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), json.clone())?;
            json.clone()
        },
        Deref @ FILENAME { .. } => JSON::makeString((var_field!((*exp).filename, NFExpression::FILENAME).clone()).clone()),
        _ => JSON::makeString((toString(exp.clone())?).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn tupleElements(mut exp: Arc<NFExpression>) -> Arc<metamodelica::List<Arc<NFExpression>>> {
    let mut expl: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    expl = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ TUPLE { .. } => var_field!((*exp).elements, NFExpression::TUPLE).clone(),
        _ => list![exp.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    expl
}

pub fn wrapCall(mut exp: Arc<NFExpression>, mut fun: Arc<dyn ::std::ops::Fn(Arc<Call::NFCall>) -> Result<Arc<Call::NFCall>> + 'static>) -> Arc<NFExpression> {
    pub type callFun = fn(Arc<Call::NFCall>) -> Result<Arc<Call::NFCall>>;

    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { .. } => {
            assign_variant_field!(exp => NFExpression::CALL; call = fun(var_field!((*exp).call, NFExpression::CALL).clone()).unwrap());
            exp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn repairOperator(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BINARY { .. } => {
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::repairBinary(var_field!((*exp).operator, NFExpression::BINARY).clone(), typeOf(var_field!((*exp).exp1, NFExpression::BINARY).clone()), typeOf(var_field!((*exp).exp2, NFExpression::BINARY).clone()))?);
            exp.clone()
        },
        Deref @ MULTARY { .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY; operator = Operator::repairMultary(var_field!((*exp).operator, NFExpression::MULTARY).clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut e in (listAppend(var_field!((*exp).arguments, NFExpression::MULTARY).clone(), var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone())).into_iter().cloned() {
            let __x = typeOf(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?);
            exp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn makeUnary(mut op: Arc<Operator::NFOperator>, mut exp: Arc<NFExpression>) -> Arc<NFExpression> {
    let mut unaryExp: Arc<NFExpression> = Arc::new(NFExpression::END);
    if op.op.clone() == Operator::Op::ADD.clone() {
        unaryExp = exp.clone();
    } else if op.op.clone() == Operator::Op::UMINUS.clone() {
        unaryExp = negate(exp.clone());
    } else {
        unaryExp = Arc::new(NFExpression::UNARY { operator: op.clone(), exp: exp.clone() });
    }
    unaryExp
}

pub fn replaceLiteral(mut exp: Arc<NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<NFExpression>, i32>>, mut idx_ptr: Pointer::Pointer<i32>) -> Result<Arc<NFExpression>> {
    fn replace(mut exp: Arc<NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<NFExpression>, i32>>, mut idx_ptr: Pointer::Pointer<i32>) -> Result<Arc<NFExpression>> {
        let mut exp: Arc<NFExpression> = exp;
        let mut idx: i32 = 0;
        let mut idx_opt: Option<i32> = None;
        idx_opt = UnorderedMap::get(exp.clone(), map.clone());
        if isSome(idx_opt.clone()) {
            idx = Util::getOption(idx_opt.clone())?;
        } else {
            idx = Pointer::access(idx_ptr.clone());
            Pointer::update(idx_ptr.clone(), idx.clone() + 1);
            UnorderedMap::add(exp.clone(), idx.clone(), map.clone())?;
        }
        exp = Arc::new(NFExpression::SHARED_LITERAL { index: idx.clone(), exp: exp.clone() });
        Ok(exp)
    }

    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ SHARED_LITERAL { .. } => exp.clone(),
        Deref @ ARRAY { .. } if (isLiteralReplace(exp.clone())) => replace(replaceLiteralArrayElements(exp.clone(), map.clone(), idx_ptr.clone())?, map.clone(), idx_ptr.clone())?,
        Deref @ RECORD { .. } if (isLiteralReplace(exp.clone())) => {
            assign_variant_field!(exp => NFExpression::RECORD; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
        for mut elem in (var_field!((*exp).elements, NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = replaceLiteral(elem.clone(), map.clone(), idx_ptr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            replace(exp.clone(), map.clone(), idx_ptr.clone())?
        },
        _ if (isLiteralReplace(exp.clone())) => replace(exp.clone(), map.clone(), idx_ptr.clone())?,
        _ => mapShallow(exp.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = idx_ptr.clone(); move |__pe_a0| replaceLiteral(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn replaceLiteralArrayElements(mut exp: Arc<NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<NFExpression>, i32>>, mut idx_ptr: Pointer::Pointer<i32>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = idx_ptr.clone(); move |__pe_a0| replaceLiteralArrayElements(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) })));
            exp.clone()
        },
        _ => replaceLiteral(exp.clone(), map.clone(), idx_ptr.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn replaceCrefWithBinding(mut cref: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFExpression>) -> Result<Arc<NFExpression>> + 'static>) -> Result<Arc<NFExpression>> {
    pub type recurse = fn(Arc<NFExpression>) -> Result<Arc<NFExpression>>;

    let mut exp: Arc<NFExpression> = exp;
    let mut e: Arc<NFExpression> = Arc::new(NFExpression::END);
    exp = (::match_deref::match_deref! { match &(InstNode::getBindingExpOpt(ComponentRef::node(cref.clone())?)) {
        Some(e @ Deref @ INTEGER { .. }) => e.clone(),
        Some(e @ Deref @ CREF { .. }) => replaceCrefWithBinding(var_field!((*e).cref, NFExpression::CREF).clone(), e.clone(), func.clone())?,
        Some(Deref @ SUBSCRIPTED_EXP { exp: e @ Deref @ INTEGER { .. }, .. }) => e.clone(),
        Some(Deref @ SUBSCRIPTED_EXP { exp: e @ Deref @ CREF { .. }, .. }) => replaceCrefWithBinding(var_field!((**e).cref, NFExpression::CREF).clone(), e.clone(), func.clone())?,
        Some(e) => {
            let mut e = (*e).clone();
            e = map(e.clone(), func.clone())?;
            e.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn replaceResizableParameterWithOriginal(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { .. } if (ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?) => replaceCrefWithBinding(var_field!((*exp).cref, NFExpression::CREF).clone(), exp.clone(), Arc::new(replaceResizableParameterWithOriginal))?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn replaceResizableParameter(mut exp: Arc<NFExpression>) -> Result<Arc<NFExpression>> {
    let mut exp: Arc<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { node: Deref @ InstNode::VAR_NODE { varPointer: var, .. }, .. }, .. } if (ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?) => {
            let mut v: i32 = 0;
            (::match_deref::match_deref! { match &(Pointer::access(var.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { resize_value: Some(v) }, .. }, .. } => Arc::new(NFExpression::INTEGER { value: v.clone() }),
        _ => replaceCrefWithBinding(var_field!((*exp).cref, NFExpression::CREF).clone(), exp.clone(), Arc::new(replaceResizableParameter))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ CREF { .. } if (ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF).clone())?) => {
            replaceCrefWithBinding(var_field!((*exp).cref, NFExpression::CREF).clone(), exp.clone(), Arc::new(replaceResizableParameter))?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn mulResultType(mut tl: Arc<Type::NFType>, mut tr: Arc<Type::NFType>) -> Arc<Type::NFType> {
    let mut tres: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut el: Arc<Type::NFType> = Arc::new(Type::ANY);
    if Type::isArray(tl.clone()) && Type::isArray(tr.clone()) {
        tres = tl.clone();
    } else if Type::isArray(tl.clone()) {
        tres = tl.clone();
    } else if Type::isArray(tr.clone()) {
        tres = tr.clone();
    } else {
        tres = tl.clone();
    }
    tres
}

pub fn mmul(mut lhs: Arc<NFExpression>, mut rhs: Arc<NFExpression>, mut baseOp: Arc<Operator::NFOperator>) -> Result<Arc<NFExpression>> {
    let mut prod: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut tl: Arc<Type::NFType> = typeOf(lhs.clone());
    let mut tr: Arc<Type::NFType> = typeOf(rhs.clone());
    let mut lArr: bool = Type::isArray(tl.clone());
    let mut rArr: bool = Type::isArray(tr.clone());
    let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR;
    let mut resTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut op: Arc<Operator::NFOperator>;
    if !(lArr.clone()) && !(rArr.clone()) {
        sizeClass = Operator::SizeClassification::SCALAR.clone();
    } else if !(lArr.clone()) && rArr.clone() {
        sizeClass = Operator::SizeClassification::SCALAR_ARRAY.clone();
    } else if lArr.clone() && !(rArr.clone()) {
        sizeClass = Operator::SizeClassification::ARRAY_SCALAR.clone();
    } else {
        sizeClass = Operator::SizeClassification::ELEMENT_WISE.clone();
    }
    resTy = mulResultType(tl.clone(), tr.clone());
    op = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass.clone()), resTy.clone())?;
    prod = Arc::new(NFExpression::BINARY { exp1: lhs.clone(), operator: op.clone(), exp2: rhs.clone() });
    Ok(prod)
}

pub fn productOfListExceptSelf(mut arguments: Arc<metamodelica::List<Arc<NFExpression>>>, mut mulOp: Arc<Operator::NFOperator>) -> Result<Arc<metamodelica::List<Arc<NFExpression>>>> {
    let mut products: Arc<metamodelica::List<Arc<NFExpression>>> = metamodelica::nil();
    let mut n: i32 = (arguments.clone().len() as i32);
    let mut argsArr: metamodelica::Array<Arc<NFExpression>>;
    let mut pref: metamodelica::Array<Arc<NFExpression>>;
    let mut res: metamodelica::Array<Arc<NFExpression>>;
    let mut i: i32 = 0;
    let mut rightProd: Arc<NFExpression> = Arc::new(NFExpression::END);
    let mut baseTy: Arc<Type::NFType> = mulOp.ty.clone();
    let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
    if n.clone() == 0 {
        products = metamodelica::nil();
        return Ok(products);
    }
    elTy = if (Type::isArray(baseTy.clone())) {Type::arrayElementType(baseTy.clone())} else {baseTy.clone()};
    argsArr = arrayCreate(n.clone(), makeOne(elTy.clone())?);
    i = 1;
    for mut a in &*arguments.clone() {
        let mut a = a.clone();
        {
            let __cell0 = a.clone();
            argsArr.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
        i = i.clone() + 1;
    }
    pref = arrayCreate(n.clone(), makeOne(elTy.clone())?);
    res = arrayCreate(n.clone(), makeOne(elTy.clone())?);
    for mut i in 2..=n.clone() {
        {
            let __cell1 = mmul(pref.borrow()[(i.clone() - 1-1) as usize].clone(), argsArr.borrow()[(i.clone() - 1-1) as usize].clone(), mulOp.clone())?;
            pref.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
        }
    }
    rightProd = makeOne(elTy.clone())?;
    for mut i in (1..=n.clone()).rev() {
        {
            let __cell2 = mmul(pref.borrow()[(i.clone()-1) as usize].clone(), rightProd.clone(), mulOp.clone())?;
            res.clone().borrow_mut()[(i.clone()-1) as usize] = __cell2;
        }
        rightProd = mmul(rightProd.clone(), argsArr.borrow()[(i.clone()-1) as usize].clone(), mulOp.clone())?;
    }
    products = metamodelica::nil();
    for mut i in (1..=n.clone()).rev() {
        products = cons(SimplifyExp::simplify(res.borrow()[(i.clone()-1) as usize].clone(), false)?, products.clone());
    }
    Ok(products)
}


