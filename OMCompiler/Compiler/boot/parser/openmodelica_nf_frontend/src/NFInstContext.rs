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

use crate::NFInstNode::InstNode;
use crate::NFRestriction as Restriction;

pub type Type = i32;

// Flag values:
pub const NO_CONTEXT: i32 = 0;

// Global flags:
pub const RELAXED: i32 = intBitLShift(1, 0);

// Relaxed instantiation, used by e.g. checkModel.
pub const INSTANCE_API: i32 = intBitLShift(1, 1);

// Instantiation for the model instance API.
pub const FAST_LOOKUP: i32 = intBitLShift(1, 2);

// Only expand packages when doing lookup.
pub const GLOBAL_FLAGS: i32 = intBitOr(RELAXED, intBitOr(INSTANCE_API, FAST_LOOKUP));

// Scope flags:
pub const CLASS: i32 = intBitLShift(1, 3);

// In class.
pub const FUNCTION: i32 = intBitLShift(1, 4);

// In function.
pub const REDECLARED: i32 = intBitLShift(1, 5);

// In an element that will be replaced with a redeclare.
pub const ALGORITHM: i32 = intBitLShift(1, 6);

// In algorithm section.
pub const EQUATION: i32 = intBitLShift(1, 7);

// In equation section.
pub const INITIAL: i32 = intBitLShift(1, 8);

// In initial section.
pub const LHS: i32 = intBitLShift(1, 9);

// On left hand side of equality/assignment.
pub const RHS: i32 = intBitLShift(1, 10);

// On right hand side of equality/assignment.
pub const WHEN: i32 = intBitLShift(1, 11);

// In when equation/statement.
pub const CLOCKED: i32 = intBitLShift(1, 12);

// Part of a clocked when equation.
pub const FOR: i32 = intBitLShift(1, 13);

// In a for loop.
pub const IF: i32 = intBitLShift(1, 14);

// In an if equation/statement.
pub const WHILE: i32 = intBitLShift(1, 15);

// In a while loop.
pub const NONEXPANDABLE: i32 = intBitLShift(1, 16);

// In non-parameter if/for.
pub const ITERATION_RANGE: i32 = intBitLShift(1, 17);

// In range used for iteration.
pub const DIMENSION: i32 = intBitLShift(1, 18);

// In dimension.
pub const BINDING: i32 = intBitLShift(1, 19);

// In binding.
pub const CONDITION: i32 = intBitLShift(1, 20);

// In conditional expression.
pub const SUBSCRIPT: i32 = intBitLShift(1, 21);

// In subscript.
pub const SUBEXPRESSION: i32 = intBitLShift(1, 22);

// Part of a larger expression.
pub const CONNECT: i32 = intBitLShift(1, 23);

// Part of connect argument.
pub const NOEVENT: i32 = intBitLShift(1, 24);

// Part of noEvent argument.
pub const ASSERT: i32 = intBitLShift(1, 25);

// Part of assert argument.
pub const ANNOTATION: i32 = intBitLShift(1, 26);

// Part of an annotation.
// Combined flags:
pub const EQ_SUBEXPRESSION: i32 = intBitOr(EQUATION, SUBEXPRESSION);

pub const VALID_TYPENAME_SCOPE: i32 = intBitOr(ITERATION_RANGE, DIMENSION);

pub const DISCRETE_SCOPE: i32 = intBitOr(WHEN, intBitOr(INITIAL, FUNCTION));

pub const NON_EXP_FLAGS: i32 = intBitOr(GLOBAL_FLAGS, intBitOr(CLASS, FUNCTION));

pub fn set(mut context: Type, mut flag: Type) -> Type {
    let mut newOrigin: Type = 0;
    newOrigin = intBitOr(context.clone(), flag.clone());
    newOrigin
}

pub fn unset(mut context: Type, mut flag: Type) -> Type {
    let mut newOrigin: Type = 0;
    newOrigin = intBitAnd(context.clone(), intBitNot(flag.clone()));
    newOrigin
}

pub fn isSet(mut context: Type, mut flag: Type) -> bool {
    let mut set: bool = false;
    set = intBitAnd(context.clone(), flag.clone()) > 0;
    set
}

pub fn isNotSet(mut context: Type, mut flag: Type) -> bool {
    let mut notSet: bool = false;
    notSet = intBitAnd(context.clone(), flag.clone()) == 0;
    notSet
}

pub fn clearScopeFlags(mut context: Type) -> Type {
    let mut outContext: Type = 0;
    outContext = intBitAnd(context.clone(), GLOBAL_FLAGS.clone());
    outContext
}

pub fn clearExpFlags(mut context: Type) -> Type {
    let mut outContext: Type = 0;
    outContext = intBitAnd(context.clone(), NON_EXP_FLAGS.clone());
    outContext
}

pub fn inRelaxed(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), RELAXED.clone()) > 0;
    res
}

pub fn inInstanceAPI(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), INSTANCE_API.clone()) > 0;
    res
}

pub fn inFastLookup(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), FAST_LOOKUP.clone()) > 0;
    res
}

pub fn inClass(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), CLASS.clone()) > 0;
    res
}

pub fn inFunction(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), FUNCTION.clone()) > 0;
    res
}

pub fn inRedeclared(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), REDECLARED.clone()) > 0;
    res
}

pub fn inAlgorithm(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), ALGORITHM.clone()) > 0;
    res
}

pub fn inEquation(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), EQUATION.clone()) > 0;
    res
}

pub fn inInitial(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), INITIAL.clone()) > 0;
    res
}

pub fn onLHS(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), LHS.clone()) > 0;
    res
}

pub fn onRHS(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), RHS.clone()) > 0;
    res
}

pub fn inWhen(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), WHEN.clone()) > 0;
    res
}

pub fn inClocked(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), CLOCKED.clone()) > 0;
    res
}

pub fn inFor(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), FOR.clone()) > 0;
    res
}

pub fn inIf(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), IF.clone()) > 0;
    res
}

pub fn inWhile(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), WHILE.clone()) > 0;
    res
}

pub fn inNonexpandable(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), NONEXPANDABLE.clone()) > 0;
    res
}

pub fn inIterationRange(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), ITERATION_RANGE.clone()) > 0;
    res
}

pub fn inDimension(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), DIMENSION.clone()) > 0;
    res
}

pub fn inBinding(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), BINDING.clone()) > 0;
    res
}

pub fn inCondition(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), CONDITION.clone()) > 0;
    res
}

pub fn inSubscript(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), SUBSCRIPT.clone()) > 0;
    res
}

pub fn inSubexpression(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), SUBEXPRESSION.clone()) > 0;
    res
}

pub fn inConnect(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), CONNECT.clone()) > 0;
    res
}

pub fn inNoEvent(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), NOEVENT.clone()) > 0;
    res
}

pub fn inAssert(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), ASSERT.clone()) > 0;
    res
}

pub fn inAnnotation(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), ANNOTATION.clone()) > 0;
    res
}

pub fn inValidTypenameScope(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), intBitOr(ITERATION_RANGE.clone(), DIMENSION.clone())) > 0;
    res
}

pub fn inDiscreteScope(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), intBitOr(WHEN.clone(), intBitOr(INITIAL.clone(), FUNCTION.clone()))) > 0;
    res
}

pub fn inLoop(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), intBitOr(FOR.clone(), WHILE.clone())) > 0;
    res
}

pub fn inValidWhenScope(mut context: Type) -> bool {
    let mut res: bool = intBitAnd(context.clone(), intBitOr(intBitOr(FUNCTION.clone(), WHILE.clone()), intBitOr(IF.clone(), intBitOr(FOR.clone(), WHEN.clone())))) == 0;
    res
}

pub fn isSingleExpression(mut context: Type) -> bool {
    let mut isSingle: bool = context.clone() < ITERATION_RANGE.clone() - 1;
    isSingle
}

pub fn nodeContext(mut node: Arc<InstNode::InstNode>, mut currentContext: Type) -> Type {
    let mut nodeContext: Type = 0;
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut parent_res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    nodeContext = clearScopeFlags(currentContext.clone());
    parent = InstNode::explicitParent(node.clone());
    if !(InstNode::isRootClass(parent.clone())) {
        nodeContext = set(nodeContext.clone(), CLASS.clone());
        return nodeContext.clone();
    }
    parent_res = InstNode::restriction(parent.clone());
    nodeContext = if (Restriction::isFunction(parent_res.clone()) || Restriction::isRecord(parent_res.clone())) {set(nodeContext.clone(), FUNCTION.clone())} else {set(nodeContext.clone(), CLASS.clone())};
    nodeContext
}

