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

use crate::BackendDAE as OldBackendDAE;
use crate::NBBackendUtil as BackendUtil;
use crate::NBDetectStates as DetectStates;
use crate::NBEvaluation as Evaluation;
use crate::NBInline as Inline;
use crate::NBReplacements as Replacements;
use crate::NBResizable::EvalOrder;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension::OptimizerExpression;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComplexType as ComplexType;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstContext;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes::Purity;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSimplifyModel as SimplifyModel;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTyping as Typing;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// Old Frontend imports
// New Frontend imports
// Old Backend imports
// New Backend imports
// Util imports
pub const SIMULATION_STR: &'static str = "SIM";

pub const START_STR: &'static str = "SRT";

pub const PRE_STR: &'static str = "PRE";

pub const TMP_STR: &'static str = "TMP";

// mainly used for mapping purposes
pub type EquationPointer = Pointer::Pointer<Arc<Equation::Equation>>;

pub type EqnSlice = Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;

// used to process different outcomes of slicing from Util/Slice.mo
// have to be defined here and not in Util/Slice.mo because it is a uniontype and not a package
/// iterator-like tuple for array handling
pub type Frame = (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>);

/// sliced frame at specific sub locations
pub type FrameLocation = (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>));

/// final result of slicing
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SlicingStatus {
    UNCHANGED = 1,
    TRIVIAL = 2,
    NONTRIVIAL = 3,
    FAILURE = 4,
}
impl PartialOrd for SlicingStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SlicingStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

/// result of sub-routine recollect
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum RecollectStatus {
    SUCCESS = 1,
    FAILURE = 2,
}
impl PartialOrd for RecollectStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for RecollectStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

/// result of sub-routine frame ordering
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FrameOrderingStatus {
    UNCHANGED = 1,
    CHANGED = 2,
    FAILURE = 3,
}
impl PartialOrd for FrameOrderingStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for FrameOrderingStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

/// type for collecting data in hash maps
pub type CrefLst = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

pub type MapFuncEqn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>;

pub type MapFuncEqnPtr = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> + 'static>;

pub type MapFuncExp = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

pub type MapFuncExpWrapper = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>;

pub type MapFuncCref = std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> + 'static>;

pub type checkEqn = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>;

pub mod Iterator {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Iterator {
        SINGLE {
            /// the name of the iterator
            name: Arc<ComponentRef::NFComponentRef>,
            /// range as <start, step, stop>
            range: Arc<Expression::NFExpression>,
            /// maps to a second iterator if derived from a for-expression
            map: Option<Arc<Iterator>>,
        },
        NESTED {
            /// sorted iterator names
            names: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>,
            /// sorted ranges as <start, step, stop>
            ranges: metamodelica::Array<Arc<Expression::NFExpression>>,
            /// maps to a second iterator if derived from a for-expression
            maps: metamodelica::Array<Option<Arc<Iterator>>>,
        },
        EMPTY,
    }
    impl Default for Iterator {
        fn default() -> Self { Self::EMPTY }
    }
    pub use self::Iterator::{SINGLE,NESTED,EMPTY};
    pub fn createFrame(mut iter: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>), mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>)> {
        let mut frame: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>);
        frame = (::match_deref::match_deref! { match &(iter.clone()) {
        (node, range @ Deref @ Expression::RANGE { .. }) => {
            (ComponentRef::makeIterator(node.clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER)), range.clone(), None)
        },
        (node, range @ Deref @ Expression::ARRAY { .. }) => {
            let mut node2: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut range2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut map: Arc<Iterator> = Arc::new(Iterator::EMPTY);
            let mut iter_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut iter_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            node2 = InstNode::newIterator(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*InstNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), metamodelica::sourceInfo!());
            range2 = Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), None, Arc::new(Expression::NFExpression::INTEGER { value: Type::sizeOf(Expression::typeOf(range.clone()), false)? }))?;
            map = fromFrames(list![(ComponentRef::makeIterator(node.clone(), Type::arrayElementType(Expression::typeOf(range.clone()))), range.clone(), None)]);
            iter_cref = ComponentRef::makeIterator(node2.clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER));
            iter_var = BackendDAE::lowerIterator(iter_cref.clone());
            iter_cref = BVariable::getVarName(iter_var.clone());
            UnorderedSet::add(iter_var.clone(), set.clone())?;
            (iter_cref.clone(), range2.clone(), Some(map.clone()))
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.createFrame")); __mm_s.push_str(&*literal!(" failed to inline iterator expression: ")); __mm_s.push_str(&*InstNode::toString(Util::tuple21(iter.clone()))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(frame)
    }

    pub fn fromFrames(mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>)>>) -> Arc<Iterator> {
        let mut iter: Arc<Iterator> = Arc::new(Iterator::EMPTY);
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut map: Option<Arc<Iterator>> = None;
        if frames.clone().is_empty() {
            iter = Arc::new(crate::NBEquation::Iterator::EMPTY);
        } else {
            (names, ranges, maps) = List::unzip3(frames.clone());
            iter = (::match_deref::match_deref! { match &((names.clone(), ranges.clone(), maps.clone())) {
        (Deref @ metamodelica::List::Cons { head: name, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: range, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: map, tail: Deref @ metamodelica::List::Nil }) => Arc::new(Iterator::SINGLE { name: name.clone(), range: range.clone(), map: map.clone() }),
        _ => Arc::new(Iterator::NESTED { names: metamodelica::arrayFromVec(names.clone().into_iter().cloned().collect()), ranges: metamodelica::arrayFromVec(ranges.clone().into_iter().cloned().collect()), maps: metamodelica::arrayFromVec(maps.clone().into_iter().cloned().collect()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        iter
    }

    pub fn addFrames(mut iter: Arc<Iterator>, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>)>>) -> Result<Arc<Iterator>> {
        let mut iter: Arc<Iterator> = iter;
        let mut names1: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut names2: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges1: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut ranges2: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps1: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        let mut maps2: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        if !(frames.clone().is_empty()) {
            (names1, ranges1, maps1) = getFrames(iter.clone())?;
            (names2, ranges2, maps2) = List::unzip3(frames.clone());
            iter = fromFrames(List::zip3(listAppend(names1.clone(), names2.clone()), listAppend(ranges1.clone(), ranges2.clone()), listAppend(maps1.clone(), maps2.clone())));
        }
        Ok(iter)
    }

    pub fn getFrames(mut iter: Arc<Iterator>) -> Result<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Option<Arc<Iterator>>>>)> {
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        (names, ranges, maps) = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => (list![var_field!((*iter).name, Iterator::SINGLE).clone()], list![var_field!((*iter).range, Iterator::SINGLE).clone()], list![var_field!((*iter).map, Iterator::SINGLE).clone()]),
        Deref @ NESTED { .. } => (Arc::new(var_field!((*iter).names, Iterator::NESTED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(var_field!((*iter).ranges, Iterator::NESTED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(var_field!((*iter).maps, Iterator::NESTED).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())),
        Deref @ EMPTY { .. } => (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
        _ => bail!("match: no arm matched"),
    } });
        Ok((names, ranges, maps))
    }

    pub fn merge(mut iterators: Arc<metamodelica::List<Arc<Iterator>>>) -> Result<Arc<Iterator>> {
        let mut result: Arc<Iterator> = Arc::new(Iterator::EMPTY);
        let mut tmp_names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut tmp_ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut tmp_maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        if List::hasOneElement(iterators.clone()) {
            result = listHead(iterators.clone())?;
        } else {
            for mut iter in &*iterators.clone().reverse() {
                let mut iter = iter.clone();
                (tmp_names, tmp_ranges, tmp_maps) = getFrames(iter.clone())?;
                names = listAppend(tmp_names.clone(), names.clone());
                ranges = listAppend(tmp_ranges.clone(), ranges.clone());
                maps = listAppend(tmp_maps.clone(), maps.clone());
            }
            result = Arc::new(Iterator::NESTED { names: metamodelica::arrayFromVec(names.clone().into_iter().cloned().collect()), ranges: metamodelica::arrayFromVec(ranges.clone().into_iter().cloned().collect()), maps: metamodelica::arrayFromVec(maps.clone().into_iter().cloned().collect()) });
        }
        Ok(result)
    }

    pub fn split(mut iterator: Arc<Iterator>) -> Result<Arc<metamodelica::List<Arc<Iterator>>>> {
        let mut result: Arc<metamodelica::List<Arc<Iterator>>> = metamodelica::nil();
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        (names, ranges, maps) = getFrames(iterator.clone())?;
        for mut tpl in &*List::zip3(names.clone(), ranges.clone(), maps.clone()) {
            let mut tpl = tpl.clone();
            result = cons(fromFrames(list![tpl.clone()]), result.clone());
        }
        Ok(result)
    }

    pub fn rename(mut iter: Arc<Iterator>, mut newBaseName: ArcStr, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<Iterator>> {
        let mut iter: Arc<Iterator> = iter;
        iter = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => {
            let mut replacor: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            replacor = ComponentRef::rename(({ let mut __mm_s = String::new(); __mm_s.push_str(&*newBaseName.clone()); __mm_s.push_str(&*intString(1)); ArcStr::from(__mm_s) }).clone(), var_field!((*iter).name, Iterator::SINGLE).clone())?;
            UnorderedMap::add(var_field!((*iter).name, Iterator::SINGLE).clone(), Expression::fromCref(replacor.clone(), false)?, replacements.clone())?;
            assign_variant_field!(iter => Iterator::SINGLE; name = replacor.clone());
            iter.clone()
        },
        Deref @ NESTED { .. } => {
            let mut replacor: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let __range0 = 1..=(var_field!((*iter).names, Iterator::NESTED).clone().borrow().len() as i32);
            for mut i in __range0 {
                replacor = ComponentRef::rename(({ let mut __mm_s = String::new(); __mm_s.push_str(&*newBaseName.clone()); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), var_field!((*iter).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone())?;
                UnorderedMap::add(var_field!((*iter).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), Expression::fromCref(replacor.clone(), false)?, replacements.clone())?;
                {
                    let __cell1 = replacor.clone();
                    var_field!((*iter).names, Iterator::NESTED).clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
            iter.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(iter)
    }

    pub fn isEqual(mut iter1: Arc<Iterator>, mut iter2: Arc<Iterator>) -> Result<bool> {
        let mut b: bool = true;
        b = (::match_deref::match_deref! { match &((iter1.clone(), iter2.clone())) {
        (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => true,
        (Deref @ SINGLE { .. }, Deref @ SINGLE { .. }) => Expression::isEqual(var_field!((*iter1).range, Iterator::SINGLE).clone(), var_field!((*iter2).range, Iterator::SINGLE).clone())? && Util::optionEqual(var_field!((*iter1).map, Iterator::SINGLE).clone(), var_field!((*iter2).map, Iterator::SINGLE).clone(), (std::sync::Arc::new(isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Iterator>, Arc<Iterator>) -> Result<bool> + 'static>)),
        (Deref @ NESTED { .. }, Deref @ NESTED { .. }) => {
            if (var_field!((*iter1).ranges, Iterator::NESTED).clone().borrow().len() as i32) == (var_field!((*iter2).ranges, Iterator::NESTED).clone().borrow().len() as i32) && (var_field!((*iter1).maps, Iterator::NESTED).clone().borrow().len() as i32) == (var_field!((*iter2).maps, Iterator::NESTED).clone().borrow().len() as i32) {
                let __range0 = 1..=(var_field!((*iter1).ranges, Iterator::NESTED).clone().borrow().len() as i32);
                for mut i in __range0 {
                    b = Expression::isEqual(var_field!((*iter1).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*iter2).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone())?;
                    if !(b.clone()) {
                        break;
                    }
                }
                let __range1 = 1..=(var_field!((*iter1).maps, Iterator::NESTED).clone().borrow().len() as i32);
                for mut i in __range1 {
                    b = Util::optionEqual(var_field!((*iter1).maps, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*iter2).maps, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), (std::sync::Arc::new(isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Iterator>, Arc<Iterator>) -> Result<bool> + 'static>));
                    if !(b.clone()) {
                        break;
                    }
                }
            } else {
                b = false;
            }
            b.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(b)
    }

    pub fn isEmpty(mut iter: Arc<Iterator>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isResizable(mut iter: Arc<Iterator>) -> Result<bool> {
        let mut b: bool = false;
        b = List::any(types(iter.clone())?, (std::sync::Arc::new(fnptr!(Type::isResizable, Arc<Type::NFType>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<bool> + 'static>));
        Ok(b)
    }

    pub fn intersect(mut iter1: Arc<Iterator>, mut iter2: Arc<Iterator>) -> Result<(Arc<Iterator>, (Arc<Iterator>, Arc<Iterator>), (Arc<Iterator>, Arc<Iterator>))> {
        let mut intersection: Arc<Iterator> = Arc::new(Iterator::EMPTY);
        let mut rest1: (Arc<Iterator>, Arc<Iterator>);
        let mut rest2: (Arc<Iterator>, Arc<Iterator>);
        (intersection, rest1, rest2) = (::match_deref::match_deref! { match &((iter1.clone(), iter2.clone())) {
        (Deref @ SINGLE { range: Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop1 }, step: Some(Deref @ Expression::INTEGER { value: step1 }), start: Deref @ Expression::INTEGER { value: start1 }, .. }, .. }, Deref @ SINGLE { range: Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop2 }, step: Some(Deref @ Expression::INTEGER { value: step2 }), start: Deref @ Expression::INTEGER { value: start2 }, .. }, .. }) if (step1.clone() == step2.clone() && intMod(start1.clone(), step1.clone()) == intMod(start2.clone(), step2.clone())) => {
            let mut start_max: i32 = 0;
            let mut stop_min: i32 = 0;
            let _ = intMin(start1.clone(), start2.clone());
            start_max = intMax(start1.clone(), start2.clone());
            stop_min = intMin(stop1.clone(), stop2.clone());
            let _ = intMax(stop1.clone(), stop2.clone());
            if start_max.clone() >= stop_min.clone() {
                intersection = Arc::new(crate::NBEquation::Iterator::EMPTY);
            } else {
                intersection = Arc::new(Iterator::SINGLE { map: var_field!((*iter1).map, Iterator::SINGLE).clone(), range: Arc::new(Expression::NFExpression::RANGE { stop: Arc::new(Expression::NFExpression::INTEGER { value: stop_min.clone() }), step: Some(Arc::new(Expression::NFExpression::INTEGER { value: step1.clone() })), start: Arc::new(Expression::NFExpression::INTEGER { value: start_max.clone() }), ty: Expression::typeOf(var_field!((*iter1).range, Iterator::SINGLE).clone()) }), name: var_field!((*iter1).name, Iterator::SINGLE).clone() });
            }
            rest1 = intersectRest(var_field!((*iter1).name, Iterator::SINGLE).clone(), start1.clone(), step1.clone(), stop1.clone(), start_max.clone() - step1.clone(), stop_min.clone() + step1.clone(), var_field!((*iter1).map, Iterator::SINGLE).clone())?;
            rest2 = intersectRest(var_field!((*iter2).name, Iterator::SINGLE).clone(), start2.clone(), step2.clone(), stop2.clone(), start_max.clone() - step2.clone(), stop_min.clone() + step2.clone(), var_field!((*iter2).map, Iterator::SINGLE).clone())?;
            (intersection.clone(), rest1.clone(), rest2.clone())
        },
        _ => {
            (Arc::new(crate::NBEquation::Iterator::EMPTY), (iter1.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY)), (Arc::new(crate::NBEquation::Iterator::EMPTY), iter2.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((intersection, rest1, rest2))
    }

    pub fn intersectRest(mut name: Arc<ComponentRef::NFComponentRef>, mut start: i32, mut step: i32, mut stop: i32, mut start_max: i32, mut stop_min: i32, mut map: Option<Arc<Iterator>>) -> Result<(Arc<Iterator>, Arc<Iterator>)> {
        let mut rest: (Arc<Iterator>, Arc<Iterator>);
        let mut rest_left: Arc<Iterator> = Arc::new(Iterator::EMPTY);
        let mut rest_right: Arc<Iterator> = Arc::new(Iterator::EMPTY);
        if start.clone() > start_max.clone() {
            rest_left = Arc::new(crate::NBEquation::Iterator::EMPTY);
        } else {
            rest_left = Arc::new(Iterator::SINGLE { map: map.clone(), range: Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: start.clone() }), Some(Arc::new(Expression::NFExpression::INTEGER { value: step.clone() })), Arc::new(Expression::NFExpression::INTEGER { value: start_max.clone() }))?, name: name.clone() });
        }
        if stop_min.clone() > stop.clone() {
            rest_right = Arc::new(crate::NBEquation::Iterator::EMPTY);
        } else {
            rest_right = Arc::new(Iterator::SINGLE { map: map.clone(), range: Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: stop_min.clone() }), Some(Arc::new(Expression::NFExpression::INTEGER { value: step.clone() })), Arc::new(Expression::NFExpression::INTEGER { value: stop.clone() }))?, name: name.clone() });
        }
        rest = (rest_left.clone(), rest_right.clone());
        Ok(rest)
    }

    pub fn types(mut iter: Arc<Iterator>) -> Result<Arc<metamodelica::List<Arc<Type::NFType>>>> {
        let mut t: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        t = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => list![Expression::typeOf(var_field!((*iter).range, Iterator::SINGLE).clone())],
        Deref @ NESTED { .. } => {
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut i in (1..=(var_field!((*iter).ranges, Iterator::NESTED).clone().borrow().len() as i32)).into_iter() {
            let __x = Expression::typeOf(var_field!((*iter).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        Deref @ EMPTY { .. } => metamodelica::nil(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.types")); __mm_s.push_str(&*literal!(" could not get types for: ")); __mm_s.push_str(&*toString(iter.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(t)
    }

    pub fn sizes(mut iter: Arc<Iterator>, mut resize: bool) -> Result<Arc<metamodelica::List<i32>>> {
        let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
        sizes = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => list![Expression::rangeSize(var_field!((*iter).range, Iterator::SINGLE).clone(), resize.clone())],
        Deref @ NESTED { .. } => {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(var_field!((*iter).ranges, Iterator::NESTED).clone().borrow().len() as i32)).into_iter() {
            let __x = Expression::rangeSize(var_field!((*iter).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), resize.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        Deref @ EMPTY { .. } => metamodelica::nil(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.sizes")); __mm_s.push_str(&*literal!(" could not get sizes for: ")); __mm_s.push_str(&*toString(iter.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(sizes)
    }

    pub fn size(mut iter: Arc<Iterator>, mut resize: bool) -> i32 {
        let mut size: i32 = {
        let mut __acc: i32 = 1;
        for mut i in (cons(1, sizes(iter.clone(), resize.clone()).unwrap())).into_iter().cloned() {
            let __x = i.clone();
            __acc *= __x;
        }
        __acc
    };
        size
    }

    pub fn dimensions(mut iter: Arc<Iterator>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
        let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>>> = metamodelica::nil();
        for mut t in (types(iter.clone()).unwrap()).into_iter().cloned() {
            let __x = Type::arrayDims(t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        dims
    }

    pub fn numDimensions(mut iter: Arc<Iterator>) -> i32 {
        let mut num: i32 = 0;
        num = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => 1,
        Deref @ NESTED { .. } => (var_field!((*iter).names, Iterator::NESTED).clone().borrow().len() as i32),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        num
    }

    pub fn dummy(mut iter: Arc<Iterator>) -> Result<Arc<Iterator>> {
        fn dummyRange(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
            let mut exp: Arc<Expression::NFExpression> = exp;
            exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { .. } => Expression::makeRange(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), None, var_field!((*exp).start, Expression::NFExpression::RANGE).clone())?,
        Deref @ Expression::ARRAY { .. } => if ((var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32) > 0) {Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: Arc::new(openmodelica_nf_frontend::NFType::INTEGER), dimensions: list![Dimension::fromInteger(1, Variability::CONSTANT.clone())] }), arrayCreate(1, var_field!((*exp).elements, Expression::NFExpression::ARRAY).borrow()[(1-1) as usize].clone()), Expression::isLiteral(var_field!((*exp).elements, Expression::NFExpression::ARRAY).borrow()[(1-1) as usize].clone()))} else {exp.clone()},
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok(exp)
        }

        let mut iter: Arc<Iterator> = iter;
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        (names, ranges, maps) = getFrames(iter.clone())?;
        ranges = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (ranges.clone()).into_iter().cloned() {
            let __x = dummyRange(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        iter = fromFrames(List::zip3(names.clone(), ranges.clone(), maps.clone()));
        Ok(iter)
    }

    pub fn createLocationReplacements(mut iter: Arc<Iterator>, mut location: metamodelica::Array<i32>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<()> {
        let () = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } if ((location.clone().borrow().len() as i32) == 1) => {
            let mut start: i32 = 0;
            let mut step: i32 = 0;
            (start, step, _) = Expression::getIntegerRange(var_field!((*iter).range, Iterator::SINGLE).clone(), true)?;
            UnorderedMap::add(var_field!((*iter).name, Iterator::SINGLE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: start.clone() + location.borrow()[(1-1) as usize].clone() * step.clone() }), replacements.clone())?;
            createMappedLocationReplacement(var_field!((*iter).map, Iterator::SINGLE).clone(), location.borrow()[(1-1) as usize].clone(), replacements.clone())?;
            ()
        },
        Deref @ NESTED { .. } if ((location.clone().borrow().len() as i32) == (var_field!((*iter).ranges, Iterator::NESTED).clone().borrow().len() as i32)) => {
            let mut start: i32 = 0;
            let mut step: i32 = 0;
            let __range0 = 1..=(location.clone().borrow().len() as i32);
            for mut i in __range0 {
                (start, step, _) = Expression::getIntegerRange(var_field!((*iter).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), true)?;
                UnorderedMap::add(var_field!((*iter).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), Arc::new(Expression::NFExpression::INTEGER { value: start.clone() + location.borrow()[(i.clone()-1) as usize].clone() * step.clone() }), replacements.clone())?;
                createMappedLocationReplacement(var_field!((*iter).maps, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), location.borrow()[(i.clone()-1) as usize].clone(), replacements.clone())?;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.createLocationReplacements")); __mm_s.push_str(&*literal!(" could not create replacements for location: ")); __mm_s.push_str(&*Array::toString(location.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" and iterator: ")); __mm_s.push_str(&*toString(iter.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn createMappedLocationReplacement(mut map: Option<Arc<Iterator>>, mut location: i32, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<()> {
        let _ = (::match_deref::match_deref! { match &(map.clone()) {
        Some(Deref @ SINGLE { range: arr @ Deref @ Expression::ARRAY { .. }, name, .. }) => {
            UnorderedMap::add(name.clone(), var_field!((**arr).elements, Expression::NFExpression::ARRAY).borrow()[(location.clone()-1) as usize].clone(), replacements.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn createReplacement(mut replacor: Arc<Iterator>, mut replacee: Arc<Iterator>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<()> {
        let mut failed: bool = false;
        failed = (::match_deref::match_deref! { match &((replacor.clone(), replacee.clone())) {
        (Deref @ SINGLE { .. }, Deref @ SINGLE { .. }) => {
            failed = createSingleReplacement(var_field!((*replacor).name, Iterator::SINGLE).clone(), var_field!((*replacor).range, Iterator::SINGLE).clone(), var_field!((*replacee).name, Iterator::SINGLE).clone(), var_field!((*replacee).range, Iterator::SINGLE).clone(), replacements.clone())?;
            failed.clone()
        },
        (Deref @ NESTED { .. }, Deref @ NESTED { .. }) => {
            if (var_field!((*replacor).names, Iterator::NESTED).clone().borrow().len() as i32) == (var_field!((*replacee).names, Iterator::NESTED).clone().borrow().len() as i32) {
                let __range0 = 1..=(var_field!((*replacor).names, Iterator::NESTED).clone().borrow().len() as i32);
                for mut i in __range0 {
                    failed = createSingleReplacement(var_field!((*replacor).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*replacor).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*replacee).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*replacee).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), replacements.clone())?;
                    if failed.clone() {
                        break;
                    }
                }
            } else {
                failed = true;
            }
            failed.clone()
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if failed.clone() {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.createReplacement")); __mm_s.push_str(&*literal!(" could not create replacements for replacor: ")); __mm_s.push_str(&*toString(replacor.clone())?); __mm_s.push_str(&*literal!(" and replacee: ")); __mm_s.push_str(&*toString(replacee.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(())
    }

    pub fn createSingleReplacement(mut replacor_cref: Arc<ComponentRef::NFComponentRef>, mut replacor_range: Arc<Expression::NFExpression>, mut replacee_cref: Arc<ComponentRef::NFComponentRef>, mut replacee_range: Arc<Expression::NFExpression>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<bool> {
        let mut failed: bool = false;
        let mut or_start: i32 = 0;
        let mut or_step: i32 = 0;
        let mut or_stop: i32 = 0;
        let mut ee_start: i32 = 0;
        let mut ee_step: i32 = 0;
        let mut ee_stop: i32 = 0;
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        (or_start, or_step, or_stop) = Expression::getIntegerRange(replacor_range.clone(), true)?;
        (ee_start, ee_step, ee_stop) = Expression::getIntegerRange(replacee_range.clone(), true)?;
        if (or_stop.clone() - or_start.clone() + 1) / or_step.clone() == (ee_stop.clone() - ee_start.clone() + 1) / ee_step.clone() {
            exp = Arc::new(Expression::NFExpression::MULTARY { operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::REAL)), inv_arguments: metamodelica::nil(), arguments: list![Arc::new(Expression::NFExpression::REAL { value: intReal(ee_start.clone()) }), Arc::new(Expression::NFExpression::MULTARY { operator: Operator::makeMul(Arc::new(openmodelica_nf_frontend::NFType::REAL)), inv_arguments: metamodelica::nil(), arguments: list![Arc::new(Expression::NFExpression::REAL { value: intReal(ee_step.clone()) / intReal(or_step.clone()) }), Arc::new(Expression::NFExpression::MULTARY { operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::REAL)), inv_arguments: list![Arc::new(Expression::NFExpression::REAL { value: intReal(or_start.clone()) })], arguments: list![Expression::fromCref(replacor_cref.clone(), false)?] })] })] });
            UnorderedMap::add(replacee_cref.clone(), exp.clone(), replacements.clone())?;
        } else {
            failed = true;
        }
        Ok(failed)
    }

    pub fn expand(mut iter: Arc<Iterator>, mut call: Arc<Call::NFCall>) -> Result<Arc<Iterator>> {
        let mut iter: Arc<Iterator> = iter;
        let mut new_iters: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
        iter = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
            (names, ranges, maps) = getFrames(iter.clone())?;
            fromFrames(listAppend({
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>)>> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()).into_iter().cloned() {
            let __x = createFrame(tpl.clone(), new_iters.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, List::zip3(names.clone(), ranges.clone(), maps.clone())))
        },
        Deref @ Call::TYPED_REDUCTION { .. } => {
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
            (names, ranges, maps) = getFrames(iter.clone())?;
            fromFrames(listAppend({
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>)>> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = createFrame(tpl.clone(), new_iters.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, List::zip3(names.clone(), ranges.clone(), maps.clone())))
        },
        _ => {
            iter.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(iter)
    }

    pub fn extract(mut exp: Arc<Expression::NFExpression>, mut new_iters: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut dims_map: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<(Arc<Iterator>, Arc<Expression::NFExpression>)> {
        let mut iter: Arc<Iterator> = Arc::new(Iterator::EMPTY);
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        (exp, iter) = extractFromCall(exp.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), replacements.clone(), new_iters.clone(), dims_map.clone())?;
        exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
        (exp, _, _, _) = Typing::typeExp(exp.clone(), NFInstContext::RHS.clone(), metamodelica::sourceInfo!(), true)?;
        Ok((iter, exp))
    }

    pub fn extractFromCall(mut exp: Arc<Expression::NFExpression>, mut iter: Arc<Iterator>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut new_iters: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut dims_map: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<(Arc<Expression::NFExpression>, Arc<Iterator>)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut iter: Arc<Iterator> = iter;
        (exp, iter) = ({
        let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>)>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut tmp: Arc<Iterator> = Arc::new(Iterator::EMPTY);
            let mut full_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            for mut tpl in &*var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone().reverse() {
                let mut tpl = tpl.clone();
                frames = cons(createFrame(tpl.clone(), new_iters.clone())?, frames.clone());
            }
            tmp = fromFrames(frames.clone());
            if !(isEmpty(iter.clone())) {
                createReplacement(iter.clone(), tmp.clone(), replacements.clone())?;
            } else {
                iter = tmp.clone();
            }
            full_dims = Type::arrayDims(Expression::typeOf(exp.clone()));
            full_dims = List::firstN(full_dims.clone(), (full_dims.clone().len() as i32) - Type::dimensionCount(Expression::typeOf(var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())))?;
            UnorderedMap::tryAdd(full_dims.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut f in (frames.clone()).into_iter().cloned() {
            let __x = Util::tuple31(f.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, dims_map.clone())?;
            (var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), iter.clone())
        },
        Deref @ Expression::CALL { .. } => {
            (exp.clone(), iter.clone())
        },
        _ => {
            (exp, iter) = Expression::mapFoldShallow(exp.clone(), Arc::new({ let __pe_b2 = replacements.clone(); let __pe_b3 = new_iters.clone(); let __pe_b4 = dims_map.clone(); move |__pe_a0, __pe_a1| extractFromCall(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }), iter.clone())?;
            (exp.clone(), iter.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok((exp, iter))
    }

    pub fn normalizedSubscripts(mut iter: Arc<Iterator>, mut iter_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Subscript::NFSubscript>>>) -> Result<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>> {
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        (names, ranges, _) = getFrames(iter.clone())?;
        subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for (name, range) in (&(names.clone())).into_iter().zip((&(ranges.clone())).into_iter()) {
            let __x = normalizedSubscript(name.clone(), range.clone(), iter_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        Ok(subs)
    }

    pub fn normalizedSubscript(mut iter_name: Arc<ComponentRef::NFComponentRef>, mut range: Arc<Expression::NFExpression>, mut iter_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Subscript::NFSubscript>>>) -> Result<Arc<Subscript::NFSubscript>> {
        let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
        let mut step: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut sub_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        sub = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { .. } => {
            step = Util::getOptionOrDefault(var_field!((*range).step, Expression::NFExpression::RANGE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }));
            sub_exp = Expression::fromCref(iter_name.clone(), false)?;
            if !(Expression::isOne(var_field!((*range).start, Expression::NFExpression::RANGE).clone())) {
                sub_exp = Arc::new(Expression::NFExpression::MULTARY { operator: Operator::makeAdd(Arc::new(openmodelica_nf_frontend::NFType::INTEGER)), inv_arguments: list![var_field!((*range).start, Expression::NFExpression::RANGE).clone()], arguments: list![sub_exp.clone()] });
            }
            if !(Expression::isOne(step.clone())) {
                sub_exp = Arc::new(Expression::NFExpression::MULTARY { operator: Operator::makeMul(Arc::new(openmodelica_nf_frontend::NFType::REAL)), inv_arguments: list![step.clone()], arguments: list![sub_exp.clone()] });
            }
            if !(Expression::isOne(var_field!((*range).start, Expression::NFExpression::RANGE).clone())) {
                sub_exp = Arc::new(Expression::NFExpression::MULTARY { operator: Operator::makeAdd(Expression::typeOf(sub_exp.clone())), inv_arguments: metamodelica::nil(), arguments: list![sub_exp.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 })] });
            }
            sub_exp = SimplifyExp::simplifyDump(sub_exp.clone(), true, (literal!("NBEquation.Iterator.normalizedSubscript")).clone(), (literal!("")).clone())?;
            if !(Type::isInteger(Expression::typeOf(sub_exp.clone()))) {
                sub_exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INTEGER_REAL().clone(), list![sub_exp.clone()], Variability::DISCRETE.clone(), Purity::PURE.clone(), NFBuiltinFuncs::INTEGER_REAL().returnType.clone()) });
            }
            Arc::new(Subscript::NFSubscript::INDEX { index: sub_exp.clone() })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.normalizedSubscript")); __mm_s.push_str(&*literal!(" failed because range is no range: ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(iter_name.clone(), sub.clone(), iter_map.clone())?;
        Ok(sub)
    }

    pub fn simplifyRangeCondition(mut iter: Arc<Iterator>, mut condition: Arc<Expression::NFExpression>) -> Result<(Arc<Iterator>, Solve::Status)> {
        let mut iter: Arc<Iterator> = iter;
        let mut status: Solve::Status = Solve::Status::UNPROCESSED;
        let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut maps: Arc<metamodelica::List<Option<Arc<Iterator>>>> = metamodelica::nil();
        let mut iter_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut opt_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Option<Arc<Iterator>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        (iter, status) = (::match_deref::match_deref! { match &(condition.clone()) {
        Deref @ Expression::RELATION { .. } => {
            let mut tmpEqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut occs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut invert: Solve::RelationInversion = Solve::RelationInversion::TRUE;
            let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut operator: Arc<Operator::NFOperator>;
            (names, ranges, maps) = getFrames(iter.clone())?;
            for mut frame in &*List::zip3(names.clone(), ranges.clone(), maps.clone()) {
                let mut frame = frame.clone();
                UnorderedMap::add(Util::tuple31(frame.clone()), Util::tuple32(frame.clone()), iter_map.clone())?;
                UnorderedMap::add(Util::tuple31(frame.clone()), Util::tuple33(frame.clone()), opt_map.clone())?;
            }
            tmpEqn = Pointer::access(Equation::makeAssignment(var_field!((*condition).exp1, Expression::NFExpression::RELATION).clone(), var_field!((*condition).exp2, Expression::NFExpression::RELATION).clone(), Pointer::create(0), (arcstr::literal!(BVariable::TEMPORARY_STR)).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), default(EquationKind::UNKNOWN.clone(), false, None, None))?);
            occs = Equation::collectCrefs(tmpEqn.clone(), Arc::new(todo!("PARTEVALFUNCTION of Equation.collectFromMap: function signature not resolved")), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            if List::hasOneElement(occs.clone()) {
                cref = listHead(occs.clone())?;
                (tmpEqn, status, invert) = Solve::solveBody(tmpEqn.clone(), cref.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Path>, Arc<Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Path>, Arc<Path>) -> Result<bool> + 'static>), 1))?;
                operator = if (invert.clone() == Solve::RelationInversion::TRUE.clone()) {Operator::invert(var_field!((*condition).operator, Expression::NFExpression::RELATION).clone())?} else {var_field!((*condition).operator, Expression::NFExpression::RELATION).clone()};
                if status.clone() == Solve::Status::EXPLICIT.clone() && invert.clone() != Solve::RelationInversion::UNKNOWN.clone() {
                    range = UnorderedMap::getSafe(cref.clone(), iter_map.clone(), metamodelica::sourceInfo!())?;
                    match '__try0: {
                        (range, status) = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { .. } => (unwrap_break_err!(adaptRange(UnorderedMap::getSafe(cref.clone(), iter_map.clone(), metamodelica::sourceInfo!())?, Util::getOption(Equation::getRHS(tmpEqn.clone())?)?, operator.clone()), '__try0), status.clone()),
        Deref @ Expression::ARRAY { .. } => (unwrap_break_err!(adaptArray(UnorderedMap::getSafe(cref.clone(), iter_map.clone(), metamodelica::sourceInfo!())?, Util::getOption(Equation::getRHS(tmpEqn.clone())?)?, operator.clone()), '__try0), status.clone()),
        _ => (range.clone(), Solve::Status::UNSOLVABLE.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        Ok::<_, anyhow::Error>((range.clone(), status.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1)) => {
                            range = __try0_o0;
                            status = __try0_o1;
                        }
                        Err(_) => {
                            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.simplifyRangeCondition")); __mm_s.push_str(&*literal!(" failed to combine iterator: ")); __mm_s.push_str(&*toString(iter.clone())?); __mm_s.push_str(&*literal!(" with condition ")); __mm_s.push_str(&*Expression::toString(condition.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
                            bail!("fail");
                        }
                    }
                    UnorderedMap::add(cref.clone(), range.clone(), iter_map.clone())?;
                } else {
                    status = Solve::Status::UNSOLVABLE.clone();
                }
            }
            if status.clone() == Solve::Status::EXPLICIT.clone() {
                iter = fromFrames({
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator>>)>> = metamodelica::nil();
        for mut name in (names.clone()).into_iter().cloned() {
            let __x = (name.clone(), UnorderedMap::getSafe(name.clone(), iter_map.clone(), metamodelica::sourceInfo!())?, UnorderedMap::getSafe(name.clone(), opt_map.clone(), metamodelica::sourceInfo!())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            (iter.clone(), status.clone())
        },
        _ => {
            (iter.clone(), Solve::Status::UNSOLVABLE.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((iter, status))
    }

    pub fn adaptRange(mut range: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut operator: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
        let mut range: Arc<Expression::NFExpression> = range;
        let mut thresh: i32 = 0;
        let mut start: i32 = 0;
        let mut step: i32 = 0;
        let mut stop: i32 = 0;
        let mut within_range: bool = false;
        (thresh, start, step, stop) = (::match_deref::match_deref! { match &((rhs.clone(), range.clone())) {
        (Deref @ Expression::INTEGER { value: thresh }, range @ Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop }, step: Some(Deref @ Expression::INTEGER { value: step }), start: Deref @ Expression::INTEGER { value: start }, .. }) => (thresh.clone(), start.clone(), step.clone(), stop.clone()),
        (Deref @ Expression::INTEGER { value: thresh }, range @ Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop }, start: Deref @ Expression::INTEGER { value: start }, .. }) => (thresh.clone(), start.clone(), 1, stop.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptRange")); __mm_s.push_str(&*literal!(" failed because range could not be evaluated: ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        within_range = thresh.clone() * sign(metamodelica::OrderedFloat((step.clone()) as f64)) > start.clone() * sign(metamodelica::OrderedFloat((step.clone()) as f64)) && thresh.clone() * sign(metamodelica::OrderedFloat((step.clone()) as f64)) < stop.clone() * sign(metamodelica::OrderedFloat((step.clone()) as f64));
        range = (match operator.op.clone() {
        Operator::Op::EQUAL => if (within_range.clone()) {Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: thresh.clone() }), None, Arc::new(Expression::NFExpression::INTEGER { value: thresh.clone() }))?} else {Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Some(Arc::new(Expression::NFExpression::INTEGER { value: 0 })), Arc::new(Expression::NFExpression::INTEGER { value: 0 }))?},
        Operator::Op::NEQUAL => if (within_range.clone()) {Expression::makeExpArray(metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (List::intRange3(start.clone(), step.clone(), stop.clone())?).into_iter().cloned() {
            if !(i.clone() != thresh.clone()) { continue; }
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), true)} else {range.clone()},
        Operator::Op::LESS => interceptRange(thresh.clone() - 1, start.clone(), step.clone(), stop.clone(), within_range.clone(), sign(metamodelica::OrderedFloat((step.clone()) as f64)) > 0, range.clone(), (std::sync::Arc::new(fnptr!(intLe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?,
        Operator::Op::LESSEQ => interceptRange(thresh.clone(), start.clone(), step.clone(), stop.clone(), within_range.clone(), sign(metamodelica::OrderedFloat((step.clone()) as f64)) > 0, range.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?,
        Operator::Op::GREATER => interceptRange(thresh.clone() + 1, start.clone(), step.clone(), stop.clone(), within_range.clone(), sign(metamodelica::OrderedFloat((step.clone()) as f64)) < 0, range.clone(), (std::sync::Arc::new(fnptr!(intGe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?,
        Operator::Op::GREATEREQ => interceptRange(thresh.clone(), start.clone(), step.clone(), stop.clone(), within_range.clone(), sign(metamodelica::OrderedFloat((step.clone()) as f64)) < 0, range.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptRange")); __mm_s.push_str(&*literal!(" failed for operator: ")); __mm_s.push_str(&*Operator::toDebugString(operator.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Ok(range)
    }

    pub fn interceptRange(mut thresh: i32, mut start: i32, mut step: i32, mut stop: i32, mut within_range: bool, mut at_end: bool, mut range: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>) -> Result<Arc<Expression::NFExpression>> {
        type intComp = std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>;

        fn lowerBoundary(mut thresh: i32, mut start: i32, mut step: i32) -> i32 {
            let mut boundary: i32 = thresh.clone() + intMod(start.clone() - thresh.clone(), step.clone());
            boundary
        }

        let mut range: Arc<Expression::NFExpression> = range;
        if within_range.clone() {
            if at_end.clone() {
                range = Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: start.clone() }), Some(Arc::new(Expression::NFExpression::INTEGER { value: step.clone() })), Arc::new(Expression::NFExpression::INTEGER { value: thresh.clone() }))?;
            } else {
                range = Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: lowerBoundary(thresh.clone(), start.clone(), step.clone()) }), Some(Arc::new(Expression::NFExpression::INTEGER { value: step.clone() })), Arc::new(Expression::NFExpression::INTEGER { value: stop.clone() }))?;
            }
        } else if func(if (at_end.clone()) {stop.clone()} else {start.clone()}, thresh.clone())? {
            range = Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Some(Arc::new(Expression::NFExpression::INTEGER { value: 0 })), Arc::new(Expression::NFExpression::INTEGER { value: 0 }))?;
        }
        Ok(range)
    }

    pub fn adaptArray(mut array: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut operator: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
        let mut array: Arc<Expression::NFExpression> = array;
        let mut thresh: i32 = 0;
        let mut elems: Arc<metamodelica::List<i32>> = metamodelica::nil();
        (thresh, elems) = (::match_deref::match_deref! { match &((rhs.clone(), array.clone())) {
        (Deref @ Expression::INTEGER { value: thresh }, Deref @ Expression::ARRAY { literal: true, .. }) => (thresh.clone(), {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (var_field!((*array).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::integerValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptArray")); __mm_s.push_str(&*literal!(" failed because array range is non literal: ")); __mm_s.push_str(&*Expression::toString(array.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        array = (match operator.op.clone() {
        Operator::Op::EQUAL => if (List::contains(elems.clone(), thresh.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) {Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: thresh.clone() }), None, Arc::new(Expression::NFExpression::INTEGER { value: thresh.clone() }))?} else {Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Some(Arc::new(Expression::NFExpression::INTEGER { value: 0 })), Arc::new(Expression::NFExpression::INTEGER { value: 0 }))?},
        Operator::Op::NEQUAL => Expression::makeExpArray(metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (elems.clone()).into_iter().cloned() {
            if !(i.clone() != thresh.clone()) { continue; }
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), true),
        Operator::Op::LESS => Expression::makeExpArray(metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (elems.clone()).into_iter().cloned() {
            if !(i.clone() < thresh.clone()) { continue; }
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), true),
        Operator::Op::LESSEQ => Expression::makeExpArray(metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (elems.clone()).into_iter().cloned() {
            if !(i.clone() <= thresh.clone()) { continue; }
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), true),
        Operator::Op::GREATER => Expression::makeExpArray(metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (elems.clone()).into_iter().cloned() {
            if !(i.clone() > thresh.clone()) { continue; }
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), true),
        Operator::Op::GREATEREQ => Expression::makeExpArray(metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (elems.clone()).into_iter().cloned() {
            if !(i.clone() >= thresh.clone()) { continue; }
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), true),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptArray")); __mm_s.push_str(&*literal!(" failed for operator: ")); __mm_s.push_str(&*Operator::toDebugString(operator.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Ok(array)
    }

    pub fn applyOrder(mut iter: Arc<Iterator>, mut order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, EvalOrder>>) -> Result<Arc<Iterator>> {
        pub fn applySingleOrder(mut name: Arc<ComponentRef::NFComponentRef>, mut range: Arc<Expression::NFExpression>, mut order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, EvalOrder>>) -> Result<Arc<Expression::NFExpression>> {
            let mut range: Arc<Expression::NFExpression> = range;
            let mut eo: EvalOrder = UnorderedMap::getOrDefault(name.clone(), order.clone(), EvalOrder::INDEPENDENT.clone());
            let mut step: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut elements: Arc<metamodelica::List<i32>> = metamodelica::nil();
            range = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { .. } => {
            step = Util::getOptionOrDefault(var_field!((*range).step, Expression::NFExpression::RANGE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }));
            if Expression::isNegative(step.clone()) && eo.clone() == EvalOrder::FORWARD.clone() || Expression::isPositive(step.clone()) && eo.clone() == EvalOrder::BACKWARD.clone() {
                res = Expression::revertRange(range.clone())?;
            } else {
                res = range.clone();
            }
            res.clone()
        },
        Deref @ Expression::ARRAY { literal: true, .. } => {
            if eo.clone() == EvalOrder::FORWARD.clone() {
                elements = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (var_field!((*range).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::getInteger(e.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                assign_variant_field!(range => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (List::sort(elements.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?).into_iter().cloned() {
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()));
            } else if eo.clone() == EvalOrder::BACKWARD.clone() {
                elements = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (var_field!((*range).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::getInteger(e.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                assign_variant_field!(range => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (List::sort(elements.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?).into_iter().cloned() {
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()));
            }
            range.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.applyOrder.applySingleOrder")); __mm_s.push_str(&*literal!(" failed for unhandled range expression: ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok(range)
        }

        let mut iter: Arc<Iterator> = iter;
        iter = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => {
            assign_variant_field!(iter => Iterator::SINGLE; range = applySingleOrder(var_field!((*iter).name, Iterator::SINGLE).clone(), var_field!((*iter).range, Iterator::SINGLE).clone(), order.clone())?);
            iter.clone()
        },
        Deref @ NESTED { .. } => {
            let __range0 = 1..=(var_field!((*iter).names, Iterator::NESTED).clone().borrow().len() as i32);
            for mut i in __range0 {
                {
                    let __cell1 = applySingleOrder(var_field!((*iter).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*iter).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), order.clone())?;
                    var_field!((*iter).ranges, Iterator::NESTED).clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
            iter.clone()
        },
        _ => iter.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(iter)
    }

    pub fn toString(mut iter: Arc<Iterator>) -> Result<ArcStr> {
        fn singleStr(mut name: Arc<ComponentRef::NFComponentRef>, mut range: Arc<Expression::NFExpression>, mut map: Option<Arc<Iterator>>) -> Result<ArcStr> {
            let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(name.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) };
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            if Util::isSome(map.clone()) {
                (names, _, _) = getFrames(Util::getOption(map.clone())?)?;
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*ComponentRef::toString(listHead(names.clone())?)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            }
            Ok(r#str)
        }

        let mut r#str: ArcStr = literal!("");
        r#str = ((::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => singleStr(var_field!((*iter).name, Iterator::SINGLE).clone(), var_field!((*iter).range, Iterator::SINGLE).clone(), var_field!((*iter).map, Iterator::SINGLE).clone())?,
        Deref @ NESTED { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (1..=(var_field!((*iter).names, Iterator::NESTED).clone().borrow().len() as i32)).into_iter() {
            let __x = singleStr(var_field!((*iter).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*iter).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), var_field!((*iter).maps, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) },
        Deref @ EMPTY { .. } => literal!("<EMPTY ITERATOR>"),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.toString")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn map(mut iter: Arc<Iterator>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Iterator>> {
        let mut iter: Arc<Iterator> = iter;
        let mut funcCref: MapFuncCref;
        iter = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ SINGLE { .. } => {
            if Util::isSome(funcCrefOpt.clone()) {
                funcCref = Util::getOption(funcCrefOpt.clone())?;
                assign_variant_field!(iter => Iterator::SINGLE; name = funcCref(var_field!((*iter).name, Iterator::SINGLE).clone())?);
            }
            assign_variant_field!(iter => Iterator::SINGLE; range = mapFunc(var_field!((*iter).range, Iterator::SINGLE).clone(), funcExp.clone())?);
            iter.clone()
        },
        Deref @ NESTED { .. } => {
            if Util::isSome(funcCrefOpt.clone()) {
                funcCref = Util::getOption(funcCrefOpt.clone())?;
                let __range0 = 1..=(var_field!((*iter).names, Iterator::NESTED).clone().borrow().len() as i32);
                for mut i in __range0 {
                    {
                        let __cell1 = funcCref(var_field!((*iter).names, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone())?;
                        var_field!((*iter).names, Iterator::NESTED).clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                    }
                }
            }
            let __range2 = 1..=(var_field!((*iter).ranges, Iterator::NESTED).clone().borrow().len() as i32);
            for mut i in __range2 {
                {
                    let __cell3 = mapFunc(var_field!((*iter).ranges, Iterator::NESTED).borrow()[(i.clone()-1) as usize].clone(), funcExp.clone())?;
                    var_field!((*iter).ranges, Iterator::NESTED).clone().borrow_mut()[(i.clone()-1) as usize] = __cell3;
                }
            }
            iter.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.map")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(iter)
    }

}

pub mod Equation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Equation {
        SCALAR_EQUATION {
            /// equality type
            ty: Arc<Type::NFType>,
            /// left hand side expression
            lhs: Arc<Expression::NFExpression>,
            /// right hand side expression
            rhs: Arc<Expression::NFExpression>,
            /// origin of equation
            source: Arc<DAE::ElementSource>,
            /// Additional Attributes
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        ARRAY_EQUATION {
            /// equality type containing dimensions
            ty: Arc<Type::NFType>,
            /// left hand side expression
            lhs: Arc<Expression::NFExpression>,
            /// right hand side expression
            rhs: Arc<Expression::NFExpression>,
            /// origin of equation
            source: Arc<DAE::ElementSource>,
            /// Additional Attributes
            attr: Arc<EquationAttributes::EquationAttributes>,
            /// NONE() if not a record
            recordSize: Option<i32>,
        },
        RECORD_EQUATION {
            /// equality type
            ty: Arc<Type::NFType>,
            /// left hand side expression
            lhs: Arc<Expression::NFExpression>,
            /// right hand side expression
            rhs: Arc<Expression::NFExpression>,
            /// origin of equation
            source: Arc<DAE::ElementSource>,
            /// Additional Attributes
            attr: Arc<EquationAttributes::EquationAttributes>,
            /// size of the record
            recordSize: i32,
        },
        ALGORITHM {
            /// output size
            size: i32,
            /// Algorithm statements
            alg: Arc<Algorithm::NFAlgorithm>,
            /// origin of algorithm
            source: Arc<DAE::ElementSource>,
            /// this algorithm was translated from an equation. we should not expand array crefs!
            expand: DAE::Expand,
            /// Additional Attributes
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        IF_EQUATION {
            /// size of equation
            size: i32,
            /// Actual equation body
            body: Arc<IfEquationBody::IfEquationBody>,
            /// origin of equation
            source: Arc<DAE::ElementSource>,
            /// Additional Attributes
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        FOR_EQUATION {
            /// size of equation
            size: i32,
            /// list of all: <iterator, range>
            iter: Arc<Iterator::Iterator>,
            /// iterated equations (only multiples if entwined)
            body: Arc<metamodelica::List<Arc<Equation>>>,
            /// origin of equation
            source: Arc<DAE::ElementSource>,
            /// Additional Attributes
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        WHEN_EQUATION {
            /// size of equation
            size: i32,
            /// Actual equation body
            body: Arc<WhenEquationBody::WhenEquationBody>,
            /// origin of equation
            source: Arc<DAE::ElementSource>,
            /// Additional Attributes
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// Auxiliary equations are generated when auxiliary variables are generated
        ///      that are known to always be solved in this specific equation. E.G. $CSE
        ///      The variable binding contains the equation, but this equation is also
        ///      allowed to have a body for special cases.
        AUX_EQUATION {
            /// Corresponding auxiliary variable
            auxiliary: Pointer::Pointer<Arc<Variable::NFVariable>>,
            /// Optional body equation
            body: Option<Arc<Equation>>,
        },
        DUMMY_EQUATION,
    }
    impl Default for Equation {
        fn default() -> Self { Self::DUMMY_EQUATION }
    }
    pub use self::Equation::{SCALAR_EQUATION,ARRAY_EQUATION,RECORD_EQUATION,ALGORITHM,IF_EQUATION,FOR_EQUATION,WHEN_EQUATION,AUX_EQUATION,DUMMY_EQUATION};
    pub fn toString(mut eq: Arc<Equation>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut s: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(size(Pointer::create(eq.clone()), true)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
        let mut tupl_recd_str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[SCAL] ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Expression::toString(var_field!((*eq).lhs, Equation::SCALAR_EQUATION).clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(var_field!((*eq).rhs, Equation::SCALAR_EQUATION).clone())?); __mm_s.push_str(&*EquationAttributes::toString(var_field!((*eq).attr, Equation::SCALAR_EQUATION).clone(), (literal!(" ")).clone())?); ArcStr::from(__mm_s) },
        Deref @ ARRAY_EQUATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[ARRY] ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Expression::toString(var_field!((*eq).lhs, Equation::ARRAY_EQUATION).clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(var_field!((*eq).rhs, Equation::ARRAY_EQUATION).clone())?); __mm_s.push_str(&*EquationAttributes::toString(var_field!((*eq).attr, Equation::ARRAY_EQUATION).clone(), (literal!(" ")).clone())?); ArcStr::from(__mm_s) },
        Deref @ RECORD_EQUATION { .. } => {
            tupl_recd_str = (if (Type::isTuple(var_field!((*eq).ty, Equation::RECORD_EQUATION).clone())) {literal!("[TUPL] ")} else {literal!("[RECD] ")}).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*tupl_recd_str.clone()); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Expression::toString(var_field!((*eq).lhs, Equation::RECORD_EQUATION).clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(var_field!((*eq).rhs, Equation::RECORD_EQUATION).clone())?); __mm_s.push_str(&*EquationAttributes::toString(var_field!((*eq).attr, Equation::RECORD_EQUATION).clone(), (literal!(" ")).clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ ALGORITHM { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[ALGO] ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*EquationAttributes::toString(var_field!((*eq).attr, Equation::ALGORITHM).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Algorithm::toString(var_field!((*eq).alg, Equation::ALGORITHM).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[----] ")); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) },
        Deref @ IF_EQUATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*IfEquationBody::toString(var_field!((*eq).body, Equation::IF_EQUATION).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[----] ")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[-IF-] ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*EquationAttributes::toString(var_field!((*eq).attr, Equation::IF_EQUATION).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ FOR_EQUATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*forEquationToString(var_field!((*eq).iter, Equation::FOR_EQUATION).clone(), var_field!((*eq).body, Equation::FOR_EQUATION).clone(), (literal!("")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[----] ")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[FOR-] ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*EquationAttributes::toString(var_field!((*eq).attr, Equation::FOR_EQUATION).clone(), (literal!(" ")).clone())?); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) },
        Deref @ WHEN_EQUATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*WhenEquationBody::toString(var_field!((*eq).body, Equation::WHEN_EQUATION).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[----] ")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[WHEN] ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*EquationAttributes::toString(var_field!((*eq).attr, Equation::WHEN_EQUATION).clone(), (literal!(" ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ AUX_EQUATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[AUX-] ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("Auxiliary equation for ")); __mm_s.push_str(&*Variable::toString(Pointer::access(var_field!((*eq).auxiliary, Equation::AUX_EQUATION).clone()), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) },
        Deref @ DUMMY_EQUATION { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[DUMY] (0) Dummy equation.")); ArcStr::from(__mm_s) },
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("[FAIL] (0) ")); __mm_s.push_str(&*literal!("NBEquation.Equation.toString")); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn pointerToString(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = (toString(Pointer::access(eqn_ptr.clone()), (r#str.clone()).clone())?).clone();
        Ok(r#str)
    }

    pub fn source(mut eq: Arc<Equation>) -> Result<Arc<DAE::ElementSource>> {
        let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
        src = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => var_field!((*eq).source, Equation::SCALAR_EQUATION).clone(),
        Deref @ ARRAY_EQUATION { .. } => var_field!((*eq).source, Equation::ARRAY_EQUATION).clone(),
        Deref @ RECORD_EQUATION { .. } => var_field!((*eq).source, Equation::RECORD_EQUATION).clone(),
        Deref @ ALGORITHM { .. } => var_field!((*eq).source, Equation::ALGORITHM).clone(),
        Deref @ IF_EQUATION { .. } => var_field!((*eq).source, Equation::IF_EQUATION).clone(),
        Deref @ FOR_EQUATION { .. } => var_field!((*eq).source, Equation::FOR_EQUATION).clone(),
        Deref @ WHEN_EQUATION { .. } => var_field!((*eq).source, Equation::WHEN_EQUATION).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.source")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(src)
    }

    pub fn info(mut eq: Arc<Equation>) -> SourceInfo {
        let mut info: SourceInfo = ElementSource::getInfo(source(eq.clone()).unwrap());
        info
    }

    pub fn size(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut resize: bool) -> Result<i32> {
        let mut s: i32 = 0;
        let mut eqn: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        eqn = Pointer::access(eqn_ptr.clone());
        s = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            1
        },
        Deref @ ARRAY_EQUATION { .. } => {
            Type::sizeOf(var_field!((*eqn).ty, Equation::ARRAY_EQUATION).clone(), resize.clone())?
        },
        Deref @ RECORD_EQUATION { .. } => {
            Type::sizeOf(var_field!((*eqn).ty, Equation::RECORD_EQUATION).clone(), resize.clone())?
        },
        Deref @ ALGORITHM { .. } => {
            var_field!((*eqn).size, Equation::ALGORITHM).clone()
        },
        Deref @ IF_EQUATION { .. } => {
            if (resize.clone()) {IfEquationBody::size(var_field!((*eqn).body, Equation::IF_EQUATION).clone(), resize.clone())} else {var_field!((*eqn).size, Equation::IF_EQUATION).clone()}
        },
        Deref @ FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            if (resize.clone()) {Iterator::size(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone(), resize.clone()) * size(Pointer::create(body.clone()), resize.clone())?} else {var_field!((*eqn).size, Equation::FOR_EQUATION).clone()}
        },
        Deref @ WHEN_EQUATION { .. } => {
            if (resize.clone()) {WhenEquationBody::size(var_field!((*eqn).body, Equation::WHEN_EQUATION).clone(), resize.clone())} else {var_field!((*eqn).size, Equation::WHEN_EQUATION).clone()}
        },
        Deref @ AUX_EQUATION { .. } => {
            Variable::size(Pointer::access(var_field!((*eqn).auxiliary, Equation::AUX_EQUATION).clone()), resize.clone())
        },
        Deref @ DUMMY_EQUATION { .. } => {
            0
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.size")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(s)
    }

    pub fn sizes(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut resize: bool) -> Result<Arc<metamodelica::List<i32>>> {
        let mut size_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut eqn: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        eqn = Pointer::access(eqn_ptr.clone());
        size_lst = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { .. } => list![1],
        Deref @ ARRAY_EQUATION { .. } => list![Type::sizeOf(var_field!((*eqn).ty, Equation::ARRAY_EQUATION).clone(), resize.clone())?],
        Deref @ RECORD_EQUATION { .. } => list![Type::sizeOf(var_field!((*eqn).ty, Equation::RECORD_EQUATION).clone(), resize.clone())?],
        Deref @ ALGORITHM { .. } => list![var_field!((*eqn).size, Equation::ALGORITHM).clone()],
        Deref @ IF_EQUATION { .. } => list![var_field!((*eqn).size, Equation::IF_EQUATION).clone()],
        Deref @ FOR_EQUATION { .. } => Iterator::sizes(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone(), resize.clone())?.reverse(),
        Deref @ WHEN_EQUATION { .. } => list![var_field!((*eqn).size, Equation::WHEN_EQUATION).clone()],
        Deref @ AUX_EQUATION { .. } => list![Variable::size(Pointer::access(var_field!((*eqn).auxiliary, Equation::AUX_EQUATION).clone()), resize.clone())],
        Deref @ DUMMY_EQUATION { .. } => metamodelica::nil(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.sizes")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(size_lst)
    }

    pub fn applyToType(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>) -> Pointer::Pointer<Arc<Equation>> {
        pub type typeFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static>;

        let mut eqn_ptr: Pointer::Pointer<Arc<Equation>> = eqn_ptr;
        let mut new: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut eqn: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        new = (::match_deref::match_deref! { match &(eqn.clone()) {
        new @ Deref @ ARRAY_EQUATION { .. } => {
            let mut new = (*new).clone();
            assign_variant_field!(new => Equation::ARRAY_EQUATION; ty = func(var_field!((*new).ty, Equation::ARRAY_EQUATION).clone()).unwrap());
            new.clone()
        },
        new @ Deref @ RECORD_EQUATION { .. } => {
            let mut new = (*new).clone();
            assign_variant_field!(new => Equation::RECORD_EQUATION; ty = func(var_field!((*new).ty, Equation::RECORD_EQUATION).clone()).unwrap());
            new.clone()
        },
        _ => eqn.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(referenceEq(&eqn.clone(),&new.clone())) {
            Pointer::update(eqn_ptr.clone(), new.clone());
        }
        eqn_ptr
    }

    pub fn hash(mut eqn: Pointer::Pointer<Arc<Equation>>) -> i32 {
        let mut i: i32 = if (isDummy(Pointer::access(eqn.clone()))) {0} else {ComponentRef::hash(getEqnName(eqn.clone()).unwrap())};
        i
    }

    pub fn equalName(mut eqn1: Pointer::Pointer<Arc<Equation>>, mut eqn2: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut b: bool = ComponentRef::isEqual(getEqnName(eqn1.clone()).unwrap(), getEqnName(eqn2.clone()).unwrap()).unwrap();
        b
    }

    pub fn isEqualPtrTpl(mut tpl: (Pointer::Pointer<Arc<Equation>>, Pointer::Pointer<Arc<Equation>>)) -> bool {
        let mut b: bool = false;
        let mut eqn1: Pointer::Pointer<Arc<Equation>>;
        let mut eqn2: Pointer::Pointer<Arc<Equation>>;
        (eqn1, eqn2) = tpl.clone();
        b = isEqualPtr(eqn1.clone(), eqn2.clone());
        b
    }

    pub fn isEqualPtr(mut eqn1: Pointer::Pointer<Arc<Equation>>, mut eqn2: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut b: bool = isEqual(Pointer::access(eqn1.clone()), Pointer::access(eqn2.clone())).unwrap();
        b
    }

    pub fn isEqualTpl(mut tpl: (Arc<Equation>, Arc<Equation>)) -> Result<bool> {
        let mut b: bool = false;
        let mut eqn1: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut eqn2: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        (eqn1, eqn2) = tpl.clone();
        b = isEqual(eqn1.clone(), eqn2.clone())?;
        Ok(b)
    }

    pub fn isEqual(mut eqn1: Arc<Equation>, mut eqn2: Arc<Equation>) -> Result<bool> {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &((eqn1.clone(), eqn2.clone())) {
        (Deref @ SCALAR_EQUATION { .. }, Deref @ SCALAR_EQUATION { .. }) => Expression::isEqual(var_field!((*eqn1).lhs, Equation::SCALAR_EQUATION).clone(), var_field!((*eqn2).lhs, Equation::SCALAR_EQUATION).clone())? && Expression::isEqual(var_field!((*eqn1).rhs, Equation::SCALAR_EQUATION).clone(), var_field!((*eqn2).rhs, Equation::SCALAR_EQUATION).clone())?,
        (Deref @ ARRAY_EQUATION { .. }, Deref @ ARRAY_EQUATION { .. }) => Expression::isEqual(var_field!((*eqn1).lhs, Equation::ARRAY_EQUATION).clone(), var_field!((*eqn2).lhs, Equation::ARRAY_EQUATION).clone())? && Expression::isEqual(var_field!((*eqn1).rhs, Equation::ARRAY_EQUATION).clone(), var_field!((*eqn2).rhs, Equation::ARRAY_EQUATION).clone())?,
        (Deref @ RECORD_EQUATION { .. }, Deref @ RECORD_EQUATION { .. }) => Expression::isEqual(var_field!((*eqn1).lhs, Equation::RECORD_EQUATION).clone(), var_field!((*eqn2).lhs, Equation::RECORD_EQUATION).clone())? && Expression::isEqual(var_field!((*eqn1).rhs, Equation::RECORD_EQUATION).clone(), var_field!((*eqn2).rhs, Equation::RECORD_EQUATION).clone())?,
        (Deref @ ALGORITHM { .. }, Deref @ ALGORITHM { .. }) => Algorithm::isEqual(var_field!((*eqn1).alg, Equation::ALGORITHM).clone(), var_field!((*eqn2).alg, Equation::ALGORITHM).clone()),
        (Deref @ IF_EQUATION { .. }, Deref @ IF_EQUATION { .. }) => IfEquationBody::isEqual(var_field!((*eqn1).body, Equation::IF_EQUATION).clone(), var_field!((*eqn2).body, Equation::IF_EQUATION).clone()),
        (Deref @ FOR_EQUATION { .. }, Deref @ FOR_EQUATION { .. }) => Iterator::isEqual(var_field!((*eqn1).iter, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).iter, Equation::FOR_EQUATION).clone())? && List::all({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for (b1, b2) in (&(var_field!((*eqn1).body, Equation::FOR_EQUATION).clone())).into_iter().zip((&(var_field!((*eqn2).body, Equation::FOR_EQUATION).clone())).into_iter()) {
            let __x = isEqual(b1.clone(), b2.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, std::sync::Arc::new(fnptr!(Util::id, _))),
        (Deref @ WHEN_EQUATION { .. }, Deref @ WHEN_EQUATION { .. }) => WhenEquationBody::isEqual(var_field!((*eqn1).body, Equation::WHEN_EQUATION).clone(), var_field!((*eqn2).body, Equation::WHEN_EQUATION).clone())?,
        (Deref @ AUX_EQUATION { .. }, Deref @ AUX_EQUATION { .. }) => BVariable::equalName(var_field!((*eqn1).auxiliary, Equation::AUX_EQUATION).clone(), var_field!((*eqn2).auxiliary, Equation::AUX_EQUATION).clone()) && Util::optionEqual(var_field!((*eqn1).body, Equation::AUX_EQUATION).clone(), var_field!((*eqn2).body, Equation::AUX_EQUATION).clone(), (std::sync::Arc::new(isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation>, Arc<Equation>) -> Result<bool> + 'static>)),
        (Deref @ DUMMY_EQUATION { .. }, Deref @ DUMMY_EQUATION { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(b)
    }

    pub fn getEqnName(mut eqn: Pointer::Pointer<Arc<Equation>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut residualVar: Pointer::Pointer<Arc<Variable::NFVariable>>;
        if isDummy(Pointer::access(eqn.clone())) {
            name = Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY);
        } else {
            residualVar = getResidualVar(eqn.clone())?;
            name = BVariable::getVarName(residualVar.clone());
        }
        Ok(name)
    }

    pub fn getResidualVar(mut eqn: Pointer::Pointer<Arc<Equation>>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
        let mut residualVar: Pointer::Pointer<Arc<Variable::NFVariable>>;
        if let Ok(__iflet0) = EquationAttributes::getResidualVar(getAttributes(Pointer::access(eqn.clone()))) {
            residualVar = __iflet0;
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.getResidualVar")); __mm_s.push_str(&*literal!(" failed because of missing residual variable.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(residualVar)
    }

    pub fn getSolvedVar(mut eqn: Arc<Equation>) -> Result<Arc<Variable::NFVariable>> {
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        var = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            BVariable::getVar(cref.clone(), metamodelica::sourceInfo!())?
        },
        Deref @ ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            BVariable::getVar(cref.clone(), metamodelica::sourceInfo!())?
        },
        Deref @ RECORD_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            BVariable::getVar(cref.clone(), metamodelica::sourceInfo!())?
        },
        _ => {
            BVariable::DUMMY_VARIABLE().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(var)
    }

    pub fn makeAssignment(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut idx: Pointer::Pointer<i32>, mut r#str: ArcStr, mut iter: Arc<Iterator::Iterator>, mut attr: Arc<EquationAttributes::EquationAttributes>) -> Result<Pointer::Pointer<Arc<Equation>>> {
        let mut eq: Pointer::Pointer<Arc<Equation>>;
        let mut e: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        e = makeAssignmentEqn(lhs.clone(), rhs.clone(), iter.clone(), attr.clone())?;
        eq = Pointer::create(e.clone());
        createName(eq.clone(), idx.clone(), (r#str.clone()).clone())?;
        Ok(eq)
    }

    pub fn makeAssignmentUpdate(mut eq: Arc<Equation>, mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut attr: Arc<EquationAttributes::EquationAttributes>) -> Result<Arc<Equation>> {
        let mut eq: Arc<Equation> = eq;
        let mut res_var: Pointer::Pointer<Arc<Variable::NFVariable>> = getResidualVar(Pointer::create(eq.clone()))?;
        eq = makeAssignmentEqn(lhs.clone(), rhs.clone(), iter.clone(), attr.clone())?;
        eq = setResidualVar(eq.clone(), res_var.clone())?;
        Ok(eq)
    }

    pub fn makeAssignmentEqn(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut attr: Arc<EquationAttributes::EquationAttributes>) -> Result<Arc<Equation>> {
        let mut e: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut ty: Arc<Type::NFType> = Expression::typeOf(lhs.clone());
        e = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { .. } => {
            Arc::new(Equation::ARRAY_EQUATION { recordSize: None, attr: attr.clone(), source: DAE::emptyElementSource().clone(), rhs: rhs.clone(), lhs: lhs.clone(), ty: ty.clone() })
        },
        Deref @ Type::TUPLE { .. } => {
            Arc::new(Equation::RECORD_EQUATION { recordSize: Type::sizeOf(ty.clone(), false)?, attr: attr.clone(), source: DAE::emptyElementSource().clone(), rhs: rhs.clone(), lhs: lhs.clone(), ty: ty.clone() })
        },
        Deref @ Type::COMPLEX { complexTy: ct @ Deref @ ComplexType::RECORD { .. }, .. } => {
            Arc::new(Equation::RECORD_EQUATION { recordSize: (var_field!((**ct).fields, ComplexType::NFComplexType::RECORD).clone().borrow().len() as i32), attr: attr.clone(), source: DAE::emptyElementSource().clone(), rhs: rhs.clone(), lhs: lhs.clone(), ty: ty.clone() })
        },
        _ => {
            Arc::new(Equation::SCALAR_EQUATION { attr: attr.clone(), source: DAE::emptyElementSource().clone(), rhs: rhs.clone(), lhs: lhs.clone(), ty: ty.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(Iterator::isEmpty(iter.clone())) {
            e = Arc::new(Equation::FOR_EQUATION { attr: attr.clone(), source: DAE::emptyElementSource().clone(), body: list![e.clone()], iter: iter.clone(), size: Type::sizeOf(ty.clone(), false)? * Iterator::size(iter.clone(), false) });
            e = Inline::inlineForEquation(e.clone())?;
        }
        Ok(e)
    }

    pub fn makeAlgorithm(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut init: bool) -> Result<Pointer::Pointer<Arc<Equation>>> {
        let mut eqn: Pointer::Pointer<Arc<Equation>>;
        let mut alg: Arc<Algorithm::NFAlgorithm>;
        alg = Arc::new(Algorithm::NFAlgorithm { statements: stmts.clone(), inputs: metamodelica::nil(), outputs: metamodelica::nil(), stmtDiffInfo: None, scope: Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), source: DAE::emptyElementSource().clone() });
        alg = Algorithm::setInputsOutputs(alg.clone())?;
        eqn = BackendDAE::lowerAlgorithm(alg.clone(), init.clone());
        Ok(eqn)
    }

    pub fn forEquationToString(mut iter: Arc<Iterator::Iterator>, mut body: Arc<metamodelica::List<Arc<Equation>>>, mut r#str: ArcStr, mut indent: ArcStr, mut indicator: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut iterators: ArcStr = arcstr::literal!("");
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indicator.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("for ")); __mm_s.push_str(&*Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!(" loop\n")); ArcStr::from(__mm_s) }).clone();
        for mut eqn in &*body.clone() {
            let mut eqn = eqn.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toString(eqn.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("end for;")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getAttributes(mut eq: Arc<Equation>) -> Arc<EquationAttributes::EquationAttributes> {
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        attr = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            var_field!((*eq).attr, Equation::SCALAR_EQUATION).clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            var_field!((*eq).attr, Equation::ARRAY_EQUATION).clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            var_field!((*eq).attr, Equation::RECORD_EQUATION).clone()
        },
        Deref @ ALGORITHM { .. } => {
            var_field!((*eq).attr, Equation::ALGORITHM).clone()
        },
        Deref @ IF_EQUATION { .. } => {
            var_field!((*eq).attr, Equation::IF_EQUATION).clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            var_field!((*eq).attr, Equation::FOR_EQUATION).clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            var_field!((*eq).attr, Equation::WHEN_EQUATION).clone()
        },
        Deref @ AUX_EQUATION { body: Some(body), .. } => {
            getAttributes(body.clone())
        },
        _ => {
            default(EquationKind::UNKNOWN.clone(), false, None, None)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        attr
    }

    pub fn setAttributes(mut eq: Arc<Equation>, mut attr: Arc<EquationAttributes::EquationAttributes>) -> Result<Arc<Equation>> {
        let mut eq: Arc<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::SCALAR_EQUATION; attr = attr.clone());
            eq.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::ARRAY_EQUATION; attr = attr.clone());
            eq.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::RECORD_EQUATION; attr = attr.clone());
            eq.clone()
        },
        Deref @ ALGORITHM { .. } => {
            assign_variant_field!(eq => Equation::ALGORITHM; attr = attr.clone());
            eq.clone()
        },
        Deref @ IF_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::IF_EQUATION; attr = attr.clone());
            eq.clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::FOR_EQUATION; attr = attr.clone());
            eq.clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::WHEN_EQUATION; attr = attr.clone());
            eq.clone()
        },
        Deref @ AUX_EQUATION { body: Some(body), .. } => {
            assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(setAttributes(body.clone(), attr.clone())?));
            eq.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(eq)
    }

    pub fn setKind(mut eq: Arc<Equation>, mut kind: EquationKind, mut clock_idx: Option<i32>) -> Result<Arc<Equation>> {
        let mut eq: Arc<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::SCALAR_EQUATION; attr = EquationAttributes::setKind(var_field!((*eq).attr, Equation::SCALAR_EQUATION).clone(), kind.clone(), clock_idx.clone()));
            eq.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::ARRAY_EQUATION; attr = EquationAttributes::setKind(var_field!((*eq).attr, Equation::ARRAY_EQUATION).clone(), kind.clone(), clock_idx.clone()));
            eq.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::RECORD_EQUATION; attr = EquationAttributes::setKind(var_field!((*eq).attr, Equation::RECORD_EQUATION).clone(), kind.clone(), clock_idx.clone()));
            eq.clone()
        },
        Deref @ ALGORITHM { .. } => {
            assign_variant_field!(eq => Equation::ALGORITHM; attr = EquationAttributes::setKind(var_field!((*eq).attr, Equation::ALGORITHM).clone(), kind.clone(), clock_idx.clone()));
            eq.clone()
        },
        Deref @ IF_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::IF_EQUATION; attr = EquationAttributes::setKind(var_field!((*eq).attr, Equation::IF_EQUATION).clone(), kind.clone(), clock_idx.clone()));
            eq.clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::FOR_EQUATION; attr = EquationAttributes::setKind(var_field!((*eq).attr, Equation::FOR_EQUATION).clone(), kind.clone(), clock_idx.clone()));
            eq.clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::WHEN_EQUATION; attr = EquationAttributes::setKind(var_field!((*eq).attr, Equation::WHEN_EQUATION).clone(), kind.clone(), clock_idx.clone()));
            eq.clone()
        },
        Deref @ AUX_EQUATION { body: Some(body), .. } => {
            assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(setKind(body.clone(), kind.clone(), clock_idx.clone())?));
            eq.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(eq)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getSource(mut eq: Arc<Equation>) -> Arc<DAE::ElementSource> {
        let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
        source = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            var_field!((*eq).source, Equation::SCALAR_EQUATION).clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            var_field!((*eq).source, Equation::ARRAY_EQUATION).clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            var_field!((*eq).source, Equation::RECORD_EQUATION).clone()
        },
        Deref @ ALGORITHM { .. } => {
            var_field!((*eq).source, Equation::ALGORITHM).clone()
        },
        Deref @ IF_EQUATION { .. } => {
            var_field!((*eq).source, Equation::IF_EQUATION).clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            var_field!((*eq).source, Equation::FOR_EQUATION).clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            var_field!((*eq).source, Equation::WHEN_EQUATION).clone()
        },
        Deref @ AUX_EQUATION { body: Some(body), .. } => {
            getSource(body.clone())
        },
        _ => {
            DAE::emptyElementSource().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        source
    }

    pub fn setDerivative(mut eq: Arc<Equation>, mut derivative: Pointer::Pointer<Arc<Equation>>) -> Result<Arc<Equation>> {
        let mut eq: Arc<Equation> = eq;
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        attr = getAttributes(eq.clone());
        assign_field!(attr.derivative = Some(derivative.clone()));
        eq = setAttributes(eq.clone(), attr.clone())?;
        Ok(eq)
    }

    pub fn map(mut eq: Arc<Equation>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Equation>> {
        let mut eq: Arc<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            lhs = mapFunc(var_field!((*eq).lhs, Equation::SCALAR_EQUATION).clone(), funcExp.clone())?;
            rhs = mapFunc(var_field!((*eq).rhs, Equation::SCALAR_EQUATION).clone(), funcExp.clone())?;
            if !(referenceEq(&lhs.clone(),&var_field!((*eq).lhs, Equation::SCALAR_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::SCALAR_EQUATION; lhs = lhs.clone());
            }
            if !(referenceEq(&rhs.clone(),&var_field!((*eq).rhs, Equation::SCALAR_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::SCALAR_EQUATION; rhs = rhs.clone());
            }
            eq.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            lhs = mapFunc(var_field!((*eq).lhs, Equation::ARRAY_EQUATION).clone(), funcExp.clone())?;
            rhs = mapFunc(var_field!((*eq).rhs, Equation::ARRAY_EQUATION).clone(), funcExp.clone())?;
            if !(referenceEq(&lhs.clone(),&var_field!((*eq).lhs, Equation::ARRAY_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::ARRAY_EQUATION; lhs = lhs.clone());
            }
            if !(referenceEq(&rhs.clone(),&var_field!((*eq).rhs, Equation::ARRAY_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::ARRAY_EQUATION; rhs = rhs.clone());
            }
            eq.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            lhs = mapFunc(var_field!((*eq).lhs, Equation::RECORD_EQUATION).clone(), funcExp.clone())?;
            rhs = mapFunc(var_field!((*eq).rhs, Equation::RECORD_EQUATION).clone(), funcExp.clone())?;
            if !(referenceEq(&lhs.clone(),&var_field!((*eq).lhs, Equation::RECORD_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::RECORD_EQUATION; lhs = lhs.clone());
            }
            if !(referenceEq(&rhs.clone(),&var_field!((*eq).rhs, Equation::RECORD_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::RECORD_EQUATION; rhs = rhs.clone());
            }
            eq.clone()
        },
        Deref @ ALGORITHM { .. } => {
            let mut alg: Arc<Algorithm::NFAlgorithm>;
            alg = Algorithm::mapExp(var_field!((*eq).alg, Equation::ALGORITHM).clone(), Arc::new({ let __pe_b1 = funcExp.clone(); move |__pe_a0| mapFunc(__pe_a0, __pe_b1.clone()) }));
            if !(referenceEq(&alg.clone(),&var_field!((*eq).alg, Equation::ALGORITHM).clone())) {
                assign_variant_field!(eq => Equation::ALGORITHM; alg = Algorithm::setInputsOutputs(alg.clone())?);
            }
            eq.clone()
        },
        Deref @ IF_EQUATION { .. } => {
            let mut ifEqBody: Arc<IfEquationBody::IfEquationBody>;
            ifEqBody = IfEquationBody::map(var_field!((*eq).body, Equation::IF_EQUATION).clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
            if !(referenceEq(&ifEqBody.clone(),&var_field!((*eq).body, Equation::IF_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::IF_EQUATION; body = ifEqBody.clone());
            }
            eq.clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
            iter = Iterator::map(var_field!((*eq).iter, Equation::FOR_EQUATION).clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
            if !(referenceEq(&iter.clone(),&var_field!((*eq).iter, Equation::FOR_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::FOR_EQUATION; iter = iter.clone());
            }
            assign_variant_field!(eq => Equation::FOR_EQUATION; body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        for mut body_eqn in (var_field!((*eq).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = map(body_eqn.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eq.clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            let mut whenEqBody: Arc<WhenEquationBody::WhenEquationBody>;
            whenEqBody = WhenEquationBody::map(var_field!((*eq).body, Equation::WHEN_EQUATION).clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone());
            if !(referenceEq(&whenEqBody.clone(),&var_field!((*eq).body, Equation::WHEN_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::WHEN_EQUATION; body = whenEqBody.clone());
            }
            eq.clone()
        },
        Deref @ AUX_EQUATION { body: Some(body), .. } => {
            let mut new_body: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
            new_body = map(body.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
            if !(referenceEq(&new_body.clone(),&body.clone())) {
                assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(new_body.clone()));
            }
            eq.clone()
        },
        Deref @ DUMMY_EQUATION { .. } => {
            eq.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.map")); __mm_s.push_str(&*literal!(" failed because there was no suitable case for: ")); __mm_s.push_str(&*toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eq)
    }

    pub fn mapCondition(mut eq: Arc<Equation>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<Equation> {
        let mut eq: Arc<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ IF_EQUATION { .. } => {
            let mut ifEqBody: Arc<IfEquationBody::IfEquationBody>;
            ifEqBody = IfEquationBody::mapCondition(var_field!((*eq).body, Equation::IF_EQUATION).clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone());
            if !(referenceEq(&ifEqBody.clone(),&var_field!((*eq).body, Equation::IF_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::IF_EQUATION; body = ifEqBody.clone());
            }
            eq.clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::FOR_EQUATION; body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        for mut body_eqn in (var_field!((*eq).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = mapCondition(body_eqn.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eq.clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            let mut whenEqBody: Arc<WhenEquationBody::WhenEquationBody>;
            whenEqBody = WhenEquationBody::mapCondition(var_field!((*eq).body, Equation::WHEN_EQUATION).clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone());
            if !(referenceEq(&whenEqBody.clone(),&var_field!((*eq).body, Equation::WHEN_EQUATION).clone())) {
                assign_variant_field!(eq => Equation::WHEN_EQUATION; body = whenEqBody.clone());
            }
            eq.clone()
        },
        Deref @ AUX_EQUATION { body: Some(body), .. } => {
            let mut new_body: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
            new_body = mapCondition(body.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone());
            if !(referenceEq(&new_body.clone(),&body.clone())) {
                assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(new_body.clone()));
            }
            eq.clone()
        },
        _ => {
            eq.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        eq
    }

    pub fn collectCrefs(mut eq: Arc<Equation>, mut filter: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> + 'static>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
        let mut cref_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
        let _ = map(eq.clone(), Arc::new({ let __pe_b1 = filter.clone(); let __pe_b2 = acc.clone(); move |__pe_a0| Slice::filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), Some({ let __pe_b1 = acc.clone(); move |__pe_a0| filter(__pe_a0, __pe_b1.clone()) }), mapFunc.clone())?;
        cref_lst = UnorderedSet::toList(acc.clone());
        Ok(cref_lst)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getLHS(mut eq: Arc<Equation>) -> Result<Option<Arc<Expression::NFExpression>>> {
        let mut lhs: Option<Arc<Expression::NFExpression>> = None;
        let mut success: bool = false;
        lhs = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            Some(var_field!((*eq).lhs, Equation::SCALAR_EQUATION).clone())
        },
        Deref @ ARRAY_EQUATION { .. } => {
            Some(var_field!((*eq).lhs, Equation::ARRAY_EQUATION).clone())
        },
        Deref @ RECORD_EQUATION { .. } => {
            Some(var_field!((*eq).lhs, Equation::RECORD_EQUATION).clone())
        },
        Deref @ FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. } => {
            getLHS(listHead(var_field!((*eq).body, Equation::FOR_EQUATION).clone())?)?
        },
        Deref @ IF_EQUATION { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, success) = IfEquationBody::getLHS(var_field!((*eq).body, Equation::IF_EQUATION).clone(), Arc::new(openmodelica_nf_frontend::NFExpression::END))?;
            if (success.clone()) {Some(exp.clone())} else {None}
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(lhs)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getRHS(mut eq: Arc<Equation>) -> Result<Option<Arc<Expression::NFExpression>>> {
        let mut rhs: Option<Arc<Expression::NFExpression>> = None;
        rhs = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => Some(var_field!((*eq).rhs, Equation::SCALAR_EQUATION).clone()),
        Deref @ ARRAY_EQUATION { .. } => Some(var_field!((*eq).rhs, Equation::ARRAY_EQUATION).clone()),
        Deref @ RECORD_EQUATION { .. } => Some(var_field!((*eq).rhs, Equation::RECORD_EQUATION).clone()),
        Deref @ FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. } => getRHS(listHead(var_field!((*eq).body, Equation::FOR_EQUATION).clone())?)?,
        Deref @ IF_EQUATION { .. } => Some(IfEquationBody::getRHS(var_field!((*eq).body, Equation::IF_EQUATION).clone())?),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(rhs)
    }

    pub fn setLHS(mut eq: Arc<Equation>, mut lhs: Arc<Expression::NFExpression>) -> Result<Arc<Equation>> {
        let mut eq: Arc<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::SCALAR_EQUATION; lhs = lhs.clone());
            eq.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::ARRAY_EQUATION; lhs = lhs.clone());
            eq.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::RECORD_EQUATION; lhs = lhs.clone());
            eq.clone()
        },
        Deref @ FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. } => {
            assign_variant_field!(eq => Equation::FOR_EQUATION; body = list![setLHS(listHead(var_field!((*eq).body, Equation::FOR_EQUATION).clone())?, lhs.clone())?]);
            eq.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.setLHS")); __mm_s.push_str(&*literal!(" failed because LHS ")); __mm_s.push_str(&*Expression::toString(lhs.clone())?); __mm_s.push_str(&*literal!(" could not be set for:\n ")); __mm_s.push_str(&*toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eq)
    }

    pub fn setRHS(mut eq: Arc<Equation>, mut rhs: Arc<Expression::NFExpression>) -> Result<Arc<Equation>> {
        let mut eq: Arc<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::SCALAR_EQUATION; rhs = rhs.clone());
            eq.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::ARRAY_EQUATION; rhs = rhs.clone());
            eq.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            assign_variant_field!(eq => Equation::RECORD_EQUATION; rhs = rhs.clone());
            eq.clone()
        },
        Deref @ FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. } => {
            assign_variant_field!(eq => Equation::FOR_EQUATION; body = list![setRHS(listHead(var_field!((*eq).body, Equation::FOR_EQUATION).clone())?, rhs.clone())?]);
            eq.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.setRHS")); __mm_s.push_str(&*literal!(" failed because RHS could not be set for: ")); __mm_s.push_str(&*toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eq)
    }

    pub fn updateLHSandRHS(mut eqn: Arc<Equation>, mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>) -> Result<Arc<Equation>> {
        let mut eqn: Arc<Equation> = eqn;
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut attr: Arc<EquationAttributes::EquationAttributes> = getAttributes(eqn.clone());
        let mut src: Arc<DAE::ElementSource> = source(eqn.clone())?;
        let mut opt_rec_size: Option<i32> = None;
        let mut rec_size: i32 = 0;
        ty = Expression::typeOf(lhs.clone());
        opt_rec_size = Type::complexSize(ty.clone(), false)?;
        eqn = (::match_deref::match_deref! { match &((ty.clone(), opt_rec_size.clone())) {
        (Deref @ Type::ARRAY { .. }, _) => Arc::new(Equation::ARRAY_EQUATION { ty: ty.clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: src.clone(), attr: attr.clone(), recordSize: opt_rec_size.clone() }),
        (Deref @ Type::COMPLEX { .. }, Some(rec_size)) => Arc::new(Equation::RECORD_EQUATION { ty: ty.clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: src.clone(), attr: attr.clone(), recordSize: rec_size.clone() }),
        _ => Arc::new(Equation::SCALAR_EQUATION { ty: ty.clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: src.clone(), attr: attr.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqn)
    }

    pub fn swapLHSandRHS(mut eqn: Arc<Equation>) -> Result<Arc<Equation>> {
        let mut eqn: Arc<Equation> = eqn;
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            let mut tmpExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            tmpExp = var_field!((*eqn).rhs, Equation::SCALAR_EQUATION).clone();
            assign_variant_field!(eqn => Equation::SCALAR_EQUATION;
                rhs = var_field!((*eqn).lhs, Equation::SCALAR_EQUATION).clone(),
                lhs = tmpExp.clone()
            );
            eqn.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            let mut tmpExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            tmpExp = var_field!((*eqn).rhs, Equation::ARRAY_EQUATION).clone();
            assign_variant_field!(eqn => Equation::ARRAY_EQUATION;
                rhs = var_field!((*eqn).lhs, Equation::ARRAY_EQUATION).clone(),
                lhs = tmpExp.clone()
            );
            eqn.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            let mut tmpExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            tmpExp = var_field!((*eqn).rhs, Equation::RECORD_EQUATION).clone();
            assign_variant_field!(eqn => Equation::RECORD_EQUATION;
                rhs = var_field!((*eqn).lhs, Equation::RECORD_EQUATION).clone(),
                lhs = tmpExp.clone()
            );
            eqn.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.swapLHSandRHS")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqn)
    }

    pub fn simplify(mut eq: Arc<Equation>, mut name: ArcStr, mut indent: ArcStr, mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut simplifyExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Equation>> {
        pub type SimplifyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

        fn apply(mut e: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<Expression::NFExpression> {
            e = func(e.clone()).unwrap();
            ()
        }

        let mut eq: Arc<Equation> = eq;
        let mut old_eq: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        if Flags::isSet(Flags::DUMP_SIMPLIFY.clone())? && !(stringEqual((indent.clone()).clone(), (literal!("")).clone())) {
            println!("{}", (literal!("\n")).clone());
        }
        eq = map(eq.clone(), simplifyExp.clone(), None, (std::sync::Arc::new(fnptr!(apply, Arc<Expression::NFExpression>, MapFuncExp)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        old_eq = eq.clone();
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            if Expression::isEqual(var_field!((*eq).lhs, Equation::SCALAR_EQUATION).clone(), var_field!((*eq).rhs, Equation::SCALAR_EQUATION).clone())? {
                assign_variant_field!(eq => Equation::SCALAR_EQUATION;
                    lhs = Expression::makeZero(var_field!((*eq).ty, Equation::SCALAR_EQUATION).clone())?,
                    rhs = Expression::makeZero(var_field!((*eq).ty, Equation::SCALAR_EQUATION).clone())?
                );
            }
            eq.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            if Expression::isEqual(var_field!((*eq).lhs, Equation::ARRAY_EQUATION).clone(), var_field!((*eq).rhs, Equation::ARRAY_EQUATION).clone())? {
                assign_variant_field!(eq => Equation::ARRAY_EQUATION;
                    lhs = Expression::makeZero(var_field!((*eq).ty, Equation::ARRAY_EQUATION).clone())?,
                    rhs = Expression::makeZero(var_field!((*eq).ty, Equation::ARRAY_EQUATION).clone())?
                );
            }
            eq.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            if Expression::isEqual(var_field!((*eq).lhs, Equation::RECORD_EQUATION).clone(), var_field!((*eq).rhs, Equation::RECORD_EQUATION).clone())? {
                assign_variant_field!(eq => Equation::RECORD_EQUATION;
                    lhs = Expression::makeZero(var_field!((*eq).ty, Equation::RECORD_EQUATION).clone())?,
                    rhs = Expression::makeZero(var_field!((*eq).ty, Equation::RECORD_EQUATION).clone())?
                );
            }
            eq.clone()
        },
        Deref @ ALGORITHM { .. } => {
            assign_variant_field!(eq => Equation::ALGORITHM; alg = SimplifyModel::simplifyAlgorithm(var_field!((*eq).alg, Equation::ALGORITHM).clone())?);
            if (Algorithm::isEmpty(var_field!((*eq).alg, Equation::ALGORITHM).clone())) {Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION)} else {eq.clone()}
        },
        Deref @ WHEN_EQUATION { .. } => {
            let mut new_eq: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut when_body: Arc<WhenEquationBody::WhenEquationBody>;
            new_eq = (::match_deref::match_deref! { match &(WhenEquationBody::simplify(Some(var_field!((*eq).body, Equation::WHEN_EQUATION).clone()))?) {
        Some(when_body) => {
            assign_variant_field!(eq => Equation::WHEN_EQUATION; body = when_body.clone());
            eq.clone()
        },
        _ => {
            DetectStates::findDiscreteStatesFromWhenBody(var_field!((*eq).body, Equation::WHEN_EQUATION).clone(), acc_discrete_states.clone(), acc_previous.clone())?;
            Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_eq.clone()
        },
        Deref @ IF_EQUATION { .. } => {
            let mut new_eq: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut if_body: Arc<IfEquationBody::IfEquationBody>;
            new_eq = (::match_deref::match_deref! { match &(IfEquationBody::simplify(Some(var_field!((*eq).body, Equation::IF_EQUATION).clone()))) {
        Some(if_body) => {
            if isNone(if_body.else_if.clone()) && !(List::hasSeveralElements(if_body.then_eqns.clone())) {
                new_eq = Pointer::access(listHead(if_body.then_eqns.clone())?);
            } else {
                assign_variant_field!(eq => Equation::IF_EQUATION; body = if_body.clone());
                match '__try0: {
                    new_eq = unwrap_break_err!(IfEquationBody::inline(if_body.clone(), eq.clone()), '__try0);
                    Ok::<_, anyhow::Error>((new_eq.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        new_eq = __try0_o0;
                    }
                    Err(_) => {
                        new_eq = eq.clone();
                    }
                }
            }
            new_eq.clone()
        },
        _ => Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_eq.clone()
        },
        Deref @ FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: Deref @ IF_EQUATION { body: if_body @ Deref @ IfEquationBody::IF_EQUATION_BODY { else_if: None, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
            let mut status: Solve::Status = Solve::Status::UNPROCESSED;
            (iter, status) = Iterator::simplifyRangeCondition(var_field!((*eq).iter, Equation::FOR_EQUATION).clone(), if_body.condition.clone())?;
            if status.clone() == Solve::Status::EXPLICIT.clone() {
                assign_variant_field!(eq => Equation::FOR_EQUATION;
                    iter = iter.clone(),
                    body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        for mut be in (if_body.then_eqns.clone()).into_iter().cloned() {
            let __x = Pointer::access(be.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                    size = size(Pointer::create(eq.clone()), true)?
                );
            }
            Inline::inlineForEquation(eq.clone())?
        },
        Deref @ FOR_EQUATION { .. } => {
            Inline::inlineForEquation(eq.clone())?
        },
        Deref @ AUX_EQUATION { .. } => {
            eq.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.simplify")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if Flags::isSet(Flags::DUMP_SIMPLIFY.clone())? && !(isEqual(old_eq.clone(), eq.clone())?) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("### dumpSimplify | ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ###\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("[BEFORE]\n")); __mm_s.push_str(&*toString(old_eq.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("[AFTER ]\n")); __mm_s.push_str(&*toString(eq.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok(eq)
    }

    pub fn createName(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut idx: Pointer::Pointer<i32>, mut context: ArcStr) -> Result<()> {
        let mut eqn: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        let mut residualVar: Pointer::Pointer<Arc<Variable::NFVariable>>;
        let mut dummy_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>> = metamodelica::nil();
        (residualVar, _) = BVariable::makeResidualVar((context.clone()).clone(), Pointer::access(idx.clone()), getType(eqn.clone(), false)?)?;
        Pointer::update(idx.clone(), Pointer::access(idx.clone()) + 1);
        eqn = setResidualVar(eqn.clone(), residualVar.clone())?;
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ IF_EQUATION { .. } => {
            IfEquationBody::createNames(var_field!((*eqn).body, Equation::IF_EQUATION).clone(), idx.clone(), (context.clone()).clone())?;
            eqn.clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            dummy_eqns = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>> = metamodelica::nil();
        for mut body_eqn in (var_field!((*eqn).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = Pointer::create(body_eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            for mut body_eqn in &*dummy_eqns.clone() {
                let mut body_eqn = body_eqn.clone();
                createName(body_eqn.clone(), idx.clone(), (context.clone()).clone())?;
            }
            assign_variant_field!(eqn => Equation::FOR_EQUATION; body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        for mut body_eqn in (dummy_eqns.clone()).into_iter().cloned() {
            let __x = Pointer::access(body_eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eqn.clone()
        },
        _ => eqn.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Pointer::update(eqn_ptr.clone(), eqn.clone());
        Ok(())
    }

    pub fn setResidualVar(mut eqn: Arc<Equation>, mut residualVar: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<Arc<Equation>> {
        let mut eqn: Arc<Equation> = eqn;
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::SCALAR_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::SCALAR_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::ARRAY_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::ARRAY_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::RECORD_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::RECORD_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ ALGORITHM { .. } => {
            assign_variant_field!(eqn => Equation::ALGORITHM; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::ALGORITHM).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ IF_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::IF_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::IF_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::FOR_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::FOR_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::WHEN_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::WHEN_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.setResidualVar")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqn)
    }

    pub fn subIdxName(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut idx: Pointer::Pointer<i32>) -> Result<()> {
        let mut eqn: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        let mut residualVar: Pointer::Pointer<Arc<Variable::NFVariable>>;
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            residualVar = EquationAttributes::getResidualVar(var_field!((*eqn).attr, Equation::SCALAR_EQUATION).clone())?;
            residualVar = BVariable::subIdxName(residualVar.clone(), idx.clone())?;
            assign_variant_field!(eqn => Equation::SCALAR_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::SCALAR_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ ARRAY_EQUATION { .. } => {
            residualVar = EquationAttributes::getResidualVar(var_field!((*eqn).attr, Equation::ARRAY_EQUATION).clone())?;
            residualVar = BVariable::subIdxName(residualVar.clone(), idx.clone())?;
            assign_variant_field!(eqn => Equation::ARRAY_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::ARRAY_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            residualVar = EquationAttributes::getResidualVar(var_field!((*eqn).attr, Equation::RECORD_EQUATION).clone())?;
            residualVar = BVariable::subIdxName(residualVar.clone(), idx.clone())?;
            assign_variant_field!(eqn => Equation::RECORD_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::RECORD_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ ALGORITHM { .. } => {
            residualVar = EquationAttributes::getResidualVar(var_field!((*eqn).attr, Equation::ALGORITHM).clone())?;
            residualVar = BVariable::subIdxName(residualVar.clone(), idx.clone())?;
            assign_variant_field!(eqn => Equation::ALGORITHM; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::ALGORITHM).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ IF_EQUATION { .. } => {
            residualVar = EquationAttributes::getResidualVar(var_field!((*eqn).attr, Equation::IF_EQUATION).clone())?;
            residualVar = BVariable::subIdxName(residualVar.clone(), idx.clone())?;
            assign_variant_field!(eqn => Equation::IF_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::IF_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ FOR_EQUATION { .. } => {
            residualVar = EquationAttributes::getResidualVar(var_field!((*eqn).attr, Equation::FOR_EQUATION).clone())?;
            residualVar = BVariable::subIdxName(residualVar.clone(), idx.clone())?;
            assign_variant_field!(eqn => Equation::FOR_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::FOR_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        Deref @ WHEN_EQUATION { .. } => {
            residualVar = EquationAttributes::getResidualVar(var_field!((*eqn).attr, Equation::WHEN_EQUATION).clone())?;
            residualVar = BVariable::subIdxName(residualVar.clone(), idx.clone())?;
            assign_variant_field!(eqn => Equation::WHEN_EQUATION; attr = EquationAttributes::setResidualVar(var_field!((*eqn).attr, Equation::WHEN_EQUATION).clone(), residualVar.clone()));
            eqn.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.subIdxName")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Pointer::update(idx.clone(), Pointer::access(idx.clone()) + 1);
        Pointer::update(eqn_ptr.clone(), eqn.clone());
        Ok(())
    }

    pub fn createResidual(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut residualCref_opt: Option<Arc<ComponentRef::NFComponentRef>>, mut new: bool, mut allowFail: bool) -> Result<Pointer::Pointer<Arc<Equation>>> {
        let mut eqn_ptr: Pointer::Pointer<Arc<Equation>> = eqn_ptr;
        let mut eqn: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        let mut residualCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut failed: bool = false;
        if isResidual(eqn_ptr.clone()) {
            return Ok(eqn_ptr);
        }
        residualCref = (::match_deref::match_deref! { match &((eqn.clone(), residualCref_opt.clone())) {
        (_, Some(residualCref)) => {
            residualCref.clone()
        },
        (Deref @ FOR_EQUATION { .. }, None) => {
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            residualCref = getEqnName(eqn_ptr.clone())?;
            subs = Iterator::normalizedSubscripts(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?;
            subs = listAppend(List::fill(Arc::new(openmodelica_nf_frontend::NFSubscript::WHOLE), Type::dimensionCount(getType(listHead(var_field!((*eqn).body, Equation::FOR_EQUATION).clone())?, false)?)), subs.clone());
            residualCref = ComponentRef::setSubscripts(subs.clone(), residualCref.clone())?;
            residualCref.clone()
        },
        _ => {
            getEqnName(eqn_ptr.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (eqn, failed) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ IF_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::IF_EQUATION; body = IfEquationBody::createResidual(var_field!((*eqn).body, Equation::IF_EQUATION).clone(), residualCref.clone(), new.clone(), allowFail.clone())?);
            (IfEquationBody::inline(var_field!((*eqn).body, Equation::IF_EQUATION).clone(), eqn.clone())?, false)
        },
        Deref @ FOR_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::FOR_EQUATION; body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        for mut body_eqn in (var_field!((*eqn).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = Pointer::access(createResidual(Pointer::create(body_eqn.clone()), Some(residualCref.clone()), new.clone(), allowFail.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (eqn.clone(), false)
        },
        _ => {
            lhs = Expression::fromCref(residualCref.clone(), false)?;
            match '__try0: {
                rhs = unwrap_break_err!(getResidualExp(eqn.clone(), !(allowFail.clone())), '__try0);
                eqn = unwrap_break_err!(setLHS(eqn.clone(), lhs.clone()), '__try0);
                eqn = unwrap_break_err!(setRHS(eqn.clone(), rhs.clone()), '__try0);
                failed = false;
                Ok::<_, anyhow::Error>((failed.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    failed = __try0_o0;
                }
                Err(_) => {
                    failed = true;
                    if !(allowFail.clone()) {
                        bail!("fail");
                    }
                }
            }
            (eqn.clone(), failed.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(failed.clone()) {
            attr = getAttributes(eqn.clone());
            assign_field!(attr.residual = true);
            eqn = setAttributes(eqn.clone(), attr.clone())?;
        }
        if new.clone() {
            eqn_ptr = Pointer::create(eqn.clone());
        } else {
            Pointer::update(eqn_ptr.clone(), eqn.clone());
        }
        Ok(eqn_ptr)
    }

    pub fn getResidualExp(mut eqn: Arc<Equation>, mut throwOnFail: bool) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        exp = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            let mut operator: Arc<Operator::NFOperator>;
            operator = Arc::new(Operator::NFOperator { ty: Expression::typeOf(var_field!((*eqn).lhs, Equation::SCALAR_EQUATION).clone()), op: Operator::Op::ADD.clone() });
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![var_field!((*eqn).rhs, Equation::SCALAR_EQUATION).clone()], inv_arguments: list![var_field!((*eqn).lhs, Equation::SCALAR_EQUATION).clone()], operator: operator.clone() })
        },
        Deref @ ARRAY_EQUATION { .. } => {
            let mut operator: Arc<Operator::NFOperator>;
            operator = Arc::new(Operator::NFOperator { ty: Expression::typeOf(var_field!((*eqn).lhs, Equation::ARRAY_EQUATION).clone()), op: Operator::Op::ADD_EW.clone() });
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![var_field!((*eqn).rhs, Equation::ARRAY_EQUATION).clone()], inv_arguments: list![var_field!((*eqn).lhs, Equation::ARRAY_EQUATION).clone()], operator: operator.clone() })
        },
        Deref @ RECORD_EQUATION { ty: Deref @ Type::COMPLEX { cls: cls_node, .. }, .. } => {
            let mut operator: Arc<Operator::NFOperator>;
            let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            cls = InstNode::getClass(cls_node.clone())?;
            for mut op in &*list![(literal!("'+'")).clone(), (literal!("'0'")).clone(), (literal!("'-'")).clone()] {
                let mut op = op.clone();
                if !(Class::hasOperator((op.clone()).clone(), cls.clone())) {
                    if throwOnFail.clone() {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Trying to construct residual expression of type ")); __mm_s.push_str(&*Type::toString(var_field!((*eqn).ty, Equation::RECORD_EQUATION).clone())?); __mm_s.push_str(&*literal!(" for equation ")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!(" but operator ")); __mm_s.push_str(&*op.clone()); __mm_s.push_str(&*literal!(" is not defined.")); ArcStr::from(__mm_s) }).clone()])?;
                    }
                    bail!("fail");
                }
            }
            operator = Arc::new(Operator::NFOperator { ty: Expression::typeOf(var_field!((*eqn).lhs, Equation::RECORD_EQUATION).clone()), op: Operator::Op::ADD.clone() });
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![var_field!((*eqn).rhs, Equation::RECORD_EQUATION).clone()], inv_arguments: list![var_field!((*eqn).lhs, Equation::RECORD_EQUATION).clone()], operator: operator.clone() })
        },
        Deref @ FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. } => {
            getResidualExp(listHead(var_field!((*eqn).body, Equation::FOR_EQUATION).clone())?, true)?
        },
        _ => {
            if throwOnFail.clone() {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.getResidualExp")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            }
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        exp = SimplifyExp::simplifyDump(exp.clone(), true, (literal!("NBEquation.Equation.getResidualExp")).clone(), (literal!("")).clone())?;
        Ok(exp)
    }

    pub fn getType(mut eq: Arc<Equation>, mut skipIterator: bool) -> Result<Arc<Type::NFType>> {
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        ty = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => var_field!((*eq).ty, Equation::SCALAR_EQUATION).clone(),
        Deref @ ARRAY_EQUATION { .. } => var_field!((*eq).ty, Equation::ARRAY_EQUATION).clone(),
        Deref @ RECORD_EQUATION { .. } => var_field!((*eq).ty, Equation::RECORD_EQUATION).clone(),
        Deref @ FOR_EQUATION { .. } => {
            ty = getType(listHead(var_field!((*eq).body, Equation::FOR_EQUATION).clone())?, false)?;
            if !(skipIterator.clone()) {
                ty = Type::liftArrayRightList(ty.clone(), Iterator::dimensions(var_field!((*eq).iter, Equation::FOR_EQUATION).clone()));
            }
            ty.clone()
        },
        Deref @ WHEN_EQUATION { .. } => WhenEquationBody::getType(var_field!((*eq).body, Equation::WHEN_EQUATION).clone())?,
        _ => Arc::new(openmodelica_nf_frontend::NFType::REAL),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(ty)
    }

    pub fn getForIterator(mut eqn: Arc<Equation>) -> Arc<Iterator::Iterator> {
        let mut iterator: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        iterator = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ FOR_EQUATION { .. } => var_field!((*eqn).iter, Equation::FOR_EQUATION).clone(),
        _ => Arc::new(crate::NBEquation::Iterator::EMPTY),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        iterator
    }

    pub fn getForFrames(mut eqn: Arc<Equation>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>> {
        let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
        frames = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ FOR_EQUATION { .. } => {
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
            (names, ranges, maps) = Iterator::getFrames(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone())?;
            List::zip3(names.clone(), ranges.clone(), maps.clone())
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(frames)
    }

    pub fn applyForOrder(mut eqn: Arc<Equation>, mut order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, EvalOrder>>) -> Result<Arc<Equation>> {
        let mut eqn: Arc<Equation> = eqn;
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ FOR_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::FOR_EQUATION; iter = Iterator::applyOrder(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone(), order.clone())?);
            eqn.clone()
        },
        _ => eqn.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqn)
    }

    pub fn isDummy(mut eqn: Arc<Equation>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ DUMMY_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isResidual(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr.clone()));
        let b = attr.residual.clone();
        ()
    }

    pub fn isDiscrete(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr.clone()));
        let b = attr.kind.clone() == EquationKind::DISCRETE.clone();
        ()
    }

    pub fn isContinuous(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr.clone()));
        let b = attr.kind.clone() == EquationKind::CONTINUOUS.clone();
        ()
    }

    pub fn isDiscontinuous(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = !(isContinuous(eqn_ptr.clone()));
        ()
    }

    pub fn isContinousRecordAware(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> Result<bool> {
        let mut eqn: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        let b = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ RECORD_EQUATION { .. } => Type::isContinuous(var_field!((*eqn).ty, Equation::RECORD_EQUATION).clone())?,
        _ => isContinuous(eqn_ptr.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn isInitial(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr.clone()));
        let b = attr.exclusively_initial.clone();
        ()
    }

    pub fn isWhenEquation(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut eqn: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        let b = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ WHEN_EQUATION { .. } => true,
        Deref @ FOR_EQUATION { .. } => List::any({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>> = metamodelica::nil();
        for mut e in (var_field!((*eqn).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = Pointer::create(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(isWhenEquation, Pointer::Pointer<Arc<Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation>>) -> Result<bool> + 'static>)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isIfEquation(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        Deref @ IF_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isForEquation(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        Deref @ FOR_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isArrayEquation(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        Deref @ ARRAY_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isRecordOrTupleEquation(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        Deref @ RECORD_EQUATION { .. } => {
            true
        },
        Deref @ ARRAY_EQUATION { recordSize: Some(_), .. } => {
            true
        },
        Deref @ WHEN_EQUATION { body: when_body, .. } => {
            WhenEquationBody::isRecordOrTupleEquation(when_body.clone())
        },
        Deref @ IF_EQUATION { body: if_body, .. } => {
            IfEquationBody::isRecordOrTupleEquation(if_body.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isRecordEquation(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        e @ Deref @ RECORD_EQUATION { .. } => {
            !(Type::isTuple(var_field!((**e).ty, Equation::RECORD_EQUATION).clone()))
        },
        Deref @ ARRAY_EQUATION { recordSize: Some(_), .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isTupleEquation(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        e @ Deref @ RECORD_EQUATION { .. } => {
            Type::isTuple(var_field!((**e).ty, Equation::RECORD_EQUATION).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isAlgorithm(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        Deref @ ALGORITHM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isParameterEquation(mut eqn: Arc<Equation>) -> Result<bool> {
        let mut b: bool = true;
        let mut b_ptr: Pointer::Pointer<bool> = Pointer::create(b.clone());
        map(eqn.clone(), Arc::new({ let __pe_b1 = b_ptr.clone(); move |__pe_a0| expIsParamOrConst(__pe_a0, __pe_b1.clone()) }), Some({ let __pe_b1 = b_ptr.clone(); move |__pe_a0| crefIsParamOrConst(__pe_a0, __pe_b1.clone()) }), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        b = Pointer::access(b_ptr.clone());
        Ok(b)
    }

    pub fn isClocked(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(getAttributes(Pointer::access(eqn_ptr.clone()))) {
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { kind: EquationKind::CLOCKED { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isTypeClock(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let mut eq: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        let b = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCALAR_EQUATION { .. } => Type::isClock(var_field!((*eq).ty, Equation::SCALAR_EQUATION).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isCompound(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        Deref @ ALGORITHM { .. } => true,
        Deref @ IF_EQUATION { .. } => true,
        Deref @ WHEN_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn isResizable(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> Result<bool> {
        let b = Type::isResizable(getType(Pointer::access(eqn_ptr.clone()), false)?);
        Ok(())
    }

    pub fn hasDerivative(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>) -> bool {
        let b = (::match_deref::match_deref! { match &(getAttributes(Pointer::access(eqn_ptr.clone()))) {
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { derivative: Some(_), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn expIsParamOrConst(mut exp: Arc<Expression::NFExpression>, mut b_ptr: Pointer::Pointer<bool>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        if Pointer::access(b_ptr.clone()) {
            let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            crefIsParamOrConst(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), b_ptr.clone())?;
            ()
        },
        Deref @ Expression::CALL { .. } => {
            Pointer::update(b_ptr.clone(), Call::isImpure(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(exp)
    }

    pub fn crefIsParamOrConst(mut cref: Arc<ComponentRef::NFComponentRef>, mut b_ptr: Pointer::Pointer<bool>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
        if Pointer::access(b_ptr.clone()) {
            Pointer::update(b_ptr.clone(), BVariable::isParamOrConst(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?));
        }
        Ok(cref)
    }

    pub fn generateBindingEquation(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut idx: Pointer::Pointer<i32>, mut initial_: bool, mut new_iters: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<Pointer::Pointer<Arc<Equation>>> {
        let mut eqn: Pointer::Pointer<Arc<Equation>>;
        let mut context: ArcStr = literal!("BND");
        let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
        let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut eqnAttr: Arc<EquationAttributes::EquationAttributes>;
        let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let mut dims_map: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = UnorderedMap::new((std::sync::Arc::new(Dimension::hashList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<i32> + 'static>), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(Dimension::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, Arc<Dimension::NFDimension>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| Ok(List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone())) }), 1);
        let mut iter_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Subscript::NFSubscript>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        var = Pointer::access(var_ptr.clone());
        rhs = (::match_deref::match_deref! { match &(var.binding.clone()) {
        qual @ Deref @ Binding::TYPED_BINDING { .. } => {
            var_field!((**qual).bindingExp, Binding::NFBinding::TYPED_BINDING).clone()
        },
        qual @ Deref @ Binding::UNTYPED_BINDING { .. } => {
            var_field!((**qual).bindingExp, Binding::NFBinding::UNTYPED_BINDING).clone()
        },
        qual @ Deref @ Binding::FLAT_BINDING { .. } => {
            var_field!((**qual).bindingExp, Binding::NFBinding::FLAT_BINDING).clone()
        },
        Deref @ Binding::UNBOUND => {
            let mut start: Option<Arc<Expression::NFExpression>> = None;
            start = VariableAttributes::getStartAttribute(var.backendinfo.attributes.clone());
            (::match_deref::match_deref! { match &(start.clone()) {
        Some(start_exp) => {
            start_exp.clone()
        },
        _ => {
            Expression::makeZero(ComponentRef::getSubscriptedType(var.name.clone(), true)?)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.generateBindingEquation")); __mm_s.push_str(&*literal!(" failed because of wrong binding type: ")); __mm_s.push_str(&*Binding::toDebugString(var.binding.clone())); __mm_s.push_str(&*literal!(" for variable ")); __mm_s.push_str(&*Variable::toString(Pointer::access(var_ptr.clone()), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if BVariable::isClock(var_ptr.clone()) {
            eqnAttr = default(EquationKind::CLOCKED.clone(), initial_.clone(), Some(-1), None);
        } else if BVariable::isContinuous(var_ptr.clone(), initial_.clone())? {
            eqnAttr = default(EquationKind::CONTINUOUS.clone(), initial_.clone(), None, var.backendinfo.annotations.optimizerExpression.clone());
        } else {
            eqnAttr = default(EquationKind::DISCRETE.clone(), initial_.clone(), None, None);
        }
        (iter, rhs) = Iterator::extract(rhs.clone(), new_iters.clone(), dims_map.clone())?;
        rhs = SimplifyExp::simplifyDump(rhs.clone(), true, (literal!("NBEquation.Equation.generateBindingEquation")).clone(), (literal!("")).clone())?;
        if Iterator::isEmpty(iter.clone()) {
            lhs = Expression::fromCref(var.name.clone(), false)?;
            eqn = makeAssignment(lhs.clone(), rhs.clone(), idx.clone(), (context.clone()).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), eqnAttr.clone())?;
        } else {
            rhs = Expression::map(rhs.clone(), (std::sync::Arc::new(Expression::repairOperator) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            subs = Iterator::normalizedSubscripts(iter.clone(), iter_map.clone())?;
            lhs = Expression::fromCref(ComponentRef::mergeSubscriptsMapped(var.name.clone(), dims_map.clone(), iter_map.clone())?, false)?;
            eqn = makeAssignment(lhs.clone(), rhs.clone(), idx.clone(), (context.clone()).clone(), iter.clone(), eqnAttr.clone())?;
            renameIterators(eqn.clone(), (literal!("$i")).clone())?;
        }
        Ok(eqn)
    }

    pub fn mergeIterators(mut eq: Arc<Equation>, mut top_level: bool) -> Result<(Arc<Equation>, Arc<metamodelica::List<Arc<Iterator::Iterator>>>)> {
        let mut eq: Arc<Equation> = eq;
        let mut acc: Arc<metamodelica::List<Arc<Iterator::Iterator>>> = metamodelica::nil();
        (eq, acc) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ FOR_EQUATION { .. } => {
            let mut body: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
            (body, acc) = mergeIterators(listHead(var_field!((*eq).body, Equation::FOR_EQUATION).clone())?, false)?;
            acc = cons(var_field!((*eq).iter, Equation::FOR_EQUATION).clone(), acc.clone());
            (if (top_level.clone()) {Arc::new(Equation::FOR_EQUATION { size: var_field!((*eq).size, Equation::FOR_EQUATION).clone(), iter: Iterator::merge(acc.clone())?, body: list![body.clone()], source: var_field!((*eq).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eq).attr, Equation::FOR_EQUATION).clone() })} else {body.clone()}, acc.clone())
        },
        _ => {
            (eq.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((eq, acc))
    }

    pub fn splitIterators(mut eqn: Arc<Equation>) -> Result<Arc<Equation>> {
        let mut eqn: Arc<Equation> = eqn;
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ FOR_EQUATION { .. } => {
            let mut iterators: Arc<metamodelica::List<Arc<Iterator::Iterator>>> = metamodelica::nil();
            let mut body: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
            iterators = Iterator::split(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone())?;
            body = listHead(var_field!((*eqn).body, Equation::FOR_EQUATION).clone())?;
            for mut iter in &*iterators.clone() {
                let mut iter = iter.clone();
                body = Arc::new(Equation::FOR_EQUATION { size: var_field!((*eqn).size, Equation::FOR_EQUATION).clone(), iter: iter.clone(), body: list![body.clone()], source: var_field!((*eqn).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::FOR_EQUATION).clone() });
            }
            body.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqn)
    }

    pub fn renameIterators(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut newBaseName: ArcStr) -> Result<()> {
        let mut eqn: Arc<Equation> = Pointer::access(eqn_ptr.clone());
        let _ = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ FOR_EQUATION { .. } => {
            let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
            replacements = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            assign_variant_field!(eqn => Equation::FOR_EQUATION;
                iter = Iterator::rename(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone(), (newBaseName.clone()).clone(), replacements.clone())?,
                body = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        for mut body_eqn in (var_field!((*eqn).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = map(body_eqn.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            Pointer::update(eqn_ptr.clone(), eqn.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn entwine(mut eqn_lst: Arc<metamodelica::List<Arc<Equation>>>, mut nesting_level: i32) -> Result<Arc<metamodelica::List<Arc<Equation>>>> {
        let mut entwined: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        let mut eqn1: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut eqn2: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut next: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut rest: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        let mut tmp: Arc<metamodelica::List<Arc<Equation>>> = metamodelica::nil();
        let mut intersection: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        let mut rest1_left: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        let mut rest1_right: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        let mut rest2_left: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        let mut rest2_right: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        let mut shift: ArcStr = StringUtil::repeat((literal!("  ")).clone(), nesting_level.clone());
        if Flags::isSet(Flags::DUMP_SLICE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shift.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(nesting_level.clone())); __mm_s.push_str(&*literal!("] ### Entwining following equations:\n")); __mm_s.push_str(&*List::toString(eqn_lst.clone(), Arc::new({ let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shift.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("\n\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone());
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn_lst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqn1 = __pa0.clone();
        rest = __pa1.clone();
        while !(rest.clone().is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eqn2 = __pa2.clone();
            rest = __pa3.clone();
            eqn1 = (::match_deref::match_deref! { match &((eqn1.clone(), eqn2.clone())) {
        (Deref @ FOR_EQUATION { .. }, Deref @ FOR_EQUATION { .. }) if (Iterator::isEqual(var_field!((*eqn1).iter, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).iter, Equation::FOR_EQUATION).clone())?) => {
            assign_variant_field!(eqn1 => Equation::FOR_EQUATION; body = entwine(listAppend(var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).body, Equation::FOR_EQUATION).clone()), nesting_level.clone() + 1)?);
            eqn1.clone()
        },
        (Deref @ FOR_EQUATION { .. }, Deref @ FOR_EQUATION { .. }) => {
            let (__pa0, (__pa1, __pa2), (__pa3, __pa4)) = Iterator::intersect(var_field!((*eqn1).iter, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).iter, Equation::FOR_EQUATION).clone())?;
            intersection = __pa0.clone();
            rest1_left = __pa1.clone();
            rest1_right = __pa2.clone();
            rest2_left = __pa3.clone();
            rest2_right = __pa4.clone();
            tmp = metamodelica::nil();
            if !(Iterator::isEmpty(rest1_left.clone())) {
                tmp = cons(Arc::new(Equation::FOR_EQUATION { size: var_field!((*eqn1).size, Equation::FOR_EQUATION).clone(), iter: rest1_left.clone(), body: var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn1).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn1).attr, Equation::FOR_EQUATION).clone() }), tmp.clone());
            }
            if !(Iterator::isEmpty(rest2_left.clone())) {
                tmp = cons(Arc::new(Equation::FOR_EQUATION { size: var_field!((*eqn2).size, Equation::FOR_EQUATION).clone(), iter: rest2_left.clone(), body: var_field!((*eqn2).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn2).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn2).attr, Equation::FOR_EQUATION).clone() }), tmp.clone());
            }
            if !(Iterator::isEmpty(intersection.clone())) {
                tmp = cons(Arc::new(Equation::FOR_EQUATION { attr: var_field!((*eqn1).attr, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn1).source, Equation::FOR_EQUATION).clone(), body: entwine(listAppend(var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).body, Equation::FOR_EQUATION).clone()), nesting_level.clone() + 1)?, iter: intersection.clone(), size: var_field!((*eqn1).size, Equation::FOR_EQUATION).clone() }), tmp.clone());
            }
            if !(Iterator::isEmpty(rest1_right.clone())) {
                tmp = cons(Arc::new(Equation::FOR_EQUATION { size: var_field!((*eqn1).size, Equation::FOR_EQUATION).clone(), iter: rest1_right.clone(), body: var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn1).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn1).attr, Equation::FOR_EQUATION).clone() }), tmp.clone());
            }
            if !(Iterator::isEmpty(rest2_right.clone())) {
                tmp = cons(Arc::new(Equation::FOR_EQUATION { size: var_field!((*eqn2).size, Equation::FOR_EQUATION).clone(), iter: rest2_right.clone(), body: var_field!((*eqn2).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn2).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn2).attr, Equation::FOR_EQUATION).clone() }), tmp.clone());
            }
            let (__pa5, __pa6) = ::match_deref::match_deref! { match &(tmp.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            next = __pa5.clone();
            tmp = __pa6.clone();
            entwined = listAppend(tmp.clone(), entwined.clone());
            next.clone()
        },
        _ => {
            entwined = cons(eqn1.clone(), entwined.clone());
            eqn2.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        entwined = cons(eqn1.clone(), entwined.clone()).reverse();
        if Flags::isSet(Flags::DUMP_SLICE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shift.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(nesting_level.clone())); __mm_s.push_str(&*literal!("] +++ Result of entwining:\n")); __mm_s.push_str(&*List::toString(entwined.clone(), Arc::new({ let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*shift.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("\n\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone());
        }
        Ok(entwined)
    }

    pub fn slice(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut indices: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>>, SlicingStatus)> {
        let mut sliced_eqn: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>> = metamodelica::nil();
        let mut slicing_status: SlicingStatus = SlicingStatus::UNCHANGED;
        let mut eqn: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
        eqn = Pointer::access(eqn_ptr.clone());
        (sliced_eqn, slicing_status) = (::match_deref::match_deref! { match &(eqn.clone()) {
        _ if (indices.clone().is_empty()) => (list![Pointer::create(eqn.clone())], SlicingStatus::UNCHANGED.clone()),
        Deref @ RECORD_EQUATION { .. } => {
            slicing_status = if (size(eqn_ptr.clone(), false)? == (indices.clone().len() as i32)) {SlicingStatus::TRIVIAL.clone()} else {SlicingStatus::NONTRIVIAL.clone()};
            (list![Pointer::create(eqn.clone())], slicing_status.clone())
        },
        Deref @ ARRAY_EQUATION { .. } => {
            slicing_status = if (size(eqn_ptr.clone(), false)? == (indices.clone().len() as i32)) {SlicingStatus::TRIVIAL.clone()} else {SlicingStatus::NONTRIVIAL.clone()};
            (list![Pointer::create(eqn.clone())], slicing_status.clone())
        },
        Deref @ FOR_EQUATION { .. } => {
            dims = Type::arrayDims(getType(eqn.clone(), false)?);
            sizes = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            slicing_status = if (size(eqn_ptr.clone(), false)? == (indices.clone().len() as i32)) {SlicingStatus::TRIVIAL.clone()} else {SlicingStatus::NONTRIVIAL.clone()};
            if slicing_status.clone() == SlicingStatus::NONTRIVIAL.clone() {
                sliced_eqn = sliceFor(listHead(var_field!((*eqn).body, Equation::FOR_EQUATION).clone())?, getForIterator(eqn.clone()), sizes.clone(), getForFrames(eqn.clone())?.reverse(), indices.clone(), false)?;
            }
            (sliced_eqn.clone(), slicing_status.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.slice")); __mm_s.push_str(&*literal!(" failed because slicing is not yet supported for:\n")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((sliced_eqn, slicing_status))
    }

    pub fn sliceFor(mut body: Arc<Equation>, mut iter: Arc<Iterator::Iterator>, mut sizes: Arc<metamodelica::List<i32>>, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut indices: Arc<metamodelica::List<i32>>, mut naive: bool) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>>> {
        let mut result: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>> = metamodelica::nil();
        let mut location: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut new_frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
        let mut locations: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        let mut locations_T: Arc<metamodelica::List<metamodelica::Array<i32>>> = metamodelica::nil();
        let mut frame_locations: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>> = metamodelica::nil();
        let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
        let mut frame_status: FrameOrderingStatus = FrameOrderingStatus::UNCHANGED;
        let mut recollect_status: RecollectStatus = RecollectStatus::SUCCESS;
        let mut tmp: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut removed_diagonals_opt: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>> = None;
        let mut size: i32 = 0;
        let mut new_iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
        if List::hasOneElement(indices.clone()) {
            location = Slice::indexToLocation(listHead(indices.clone())?, sizes.clone());
            replacements = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            Iterator::createLocationReplacements(iter.clone(), metamodelica::arrayFromVec(location.clone().into_iter().cloned().collect()), replacements.clone())?;
            tmp = map(body.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            result = list![Pointer::create(tmp.clone())];
        } else {
            locations = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut idx in (indices.clone()).into_iter().cloned() {
            let __x = Slice::indexToLocation(idx.clone(), sizes.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            locations_T = Slice::transposeLocations(locations.clone(), (sizes.clone().len() as i32));
            frame_locations = List::zip(locations_T.clone(), frames.clone());
            (frame_locations, replacements, frame_status) = Slice::orderTransposedFrameLocations(frame_locations.clone())?;
            if frame_status.clone() == FrameOrderingStatus::FAILURE.clone() {
                if naive.clone() {
                    result = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>>>> = metamodelica::nil();
        for mut i in (indices.clone()).into_iter().cloned() {
            let __x = sliceFor(body.clone(), iter.clone(), sizes.clone(), frames.clone(), list![i.clone()], true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                } else {
                    result = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>>>> = metamodelica::nil();
        for mut subset in (Slice::naiveSeparation(indices.clone())?).into_iter().cloned() {
            let __x = sliceFor(body.clone(), iter.clone(), sizes.clone(), frames.clone(), subset.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                }
            } else {
                (new_frames, removed_diagonals_opt, recollect_status) = Slice::recollectRangesHeuristic(frame_locations.clone())?;
                if recollect_status.clone() == RecollectStatus::FAILURE.clone() || isSome(removed_diagonals_opt.clone()) {
                    if naive.clone() {
                        result = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>>>> = metamodelica::nil();
        for mut i in (indices.clone()).into_iter().cloned() {
            let __x = sliceFor(body.clone(), iter.clone(), sizes.clone(), frames.clone(), list![i.clone()], true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    } else {
                        result = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation>>>>>> = metamodelica::nil();
        for mut subset in (Slice::naiveSeparation(indices.clone())?).into_iter().cloned() {
            let __x = sliceFor(body.clone(), iter.clone(), sizes.clone(), frames.clone(), subset.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    }
                } else {
                    tmp = map(body.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                    new_iter = Iterator::fromFrames(new_frames.clone());
                    size = Iterator::size(new_iter.clone(), false) * self::size(Pointer::create(tmp.clone()), false)?;
                    tmp = Arc::new(Equation::FOR_EQUATION { attr: getAttributes(body.clone()), source: getSource(body.clone()), body: list![tmp.clone()], iter: new_iter.clone(), size: size.clone() });
                    result = list![Pointer::create(tmp.clone())];
                }
            }
        }
        Ok(result)
    }

    pub fn singleSlice(mut eqn_ptr: Pointer::Pointer<Arc<Equation>>, mut scal_idx: i32, mut sizes: Arc<metamodelica::List<i32>>, mut cref_to_solve: Arc<ComponentRef::NFComponentRef>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Equation>, Solve::Status)> {
        let mut sliced_eqn: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut solve_status: Solve::Status = Solve::Status::UNPROCESSED;
        let mut eqn: Arc<Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut location: Arc<metamodelica::List<i32>> = metamodelica::nil();
        eqn = Pointer::access(eqn_ptr.clone());
        (sliced_eqn, solve_status) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ FOR_EQUATION { .. } => {
            location = Slice::indexToLocation(scal_idx.clone(), sizes.clone());
            Iterator::createLocationReplacements(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone(), metamodelica::arrayFromVec(location.clone().into_iter().cloned().collect()), replacements.clone())?;
            sliced_eqn = map(listHead(var_field!((*eqn).body, Equation::FOR_EQUATION).clone())?, Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            if !(ComponentRef::isEmpty(cref_to_solve.clone())) {
                (sliced_eqn, solve_status, _) = Solve::solveBody(sliced_eqn.clone(), cref_to_solve.clone(), funcMap.clone())?;
            }
            (sliced_eqn.clone(), solve_status.clone())
        },
        _ => (eqn.clone(), Solve::Status::UNPROCESSED.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((sliced_eqn, solve_status))
    }

    fn makeInequality(mut tpl: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)) -> Result<Arc<Expression::NFExpression>> {
        let mut equality_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        (cref, exp) = tpl.clone();
        equality_exp = Arc::new(Expression::NFExpression::RELATION { index: -1, exp2: SimplifyExp::simplifyDump(exp.clone(), true, (literal!("NBEquation.Equation.makeInequality")).clone(), (literal!("")).clone())?, operator: Arc::new(Operator::NFOperator { ty: ComponentRef::nodeType(cref.clone())?, op: Operator::Op::NEQUAL.clone() }), exp1: Expression::fromCref(cref.clone(), false)? });
        Ok(equality_exp)
    }

    pub fn toStatement(mut eqn: Arc<Equation>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
        let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        stmts = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ SCALAR_EQUATION { .. } => {
            list![Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: var_field!((*eqn).lhs, Equation::SCALAR_EQUATION).clone(), rhs: var_field!((*eqn).rhs, Equation::SCALAR_EQUATION).clone(), ty: var_field!((*eqn).ty, Equation::SCALAR_EQUATION).clone(), source: var_field!((*eqn).source, Equation::SCALAR_EQUATION).clone() })]
        },
        Deref @ ARRAY_EQUATION { .. } => {
            list![Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: var_field!((*eqn).lhs, Equation::ARRAY_EQUATION).clone(), rhs: var_field!((*eqn).rhs, Equation::ARRAY_EQUATION).clone(), ty: var_field!((*eqn).ty, Equation::ARRAY_EQUATION).clone(), source: var_field!((*eqn).source, Equation::ARRAY_EQUATION).clone() })]
        },
        Deref @ RECORD_EQUATION { rhs: Deref @ Expression::CREF { cref: rhs_rec, .. }, lhs: Deref @ Expression::CREF { cref: lhs_rec, .. }, .. } => {
            let mut lhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut lhs: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut rhs: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut lhs_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut rhs_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut lhs_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            let mut rhs_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            lhs_lst = BVariable::getRecordChildren(BVariable::getVarPointer(lhs_rec.clone(), metamodelica::sourceInfo!())?);
            rhs_lst = BVariable::getRecordChildren(BVariable::getVarPointer(rhs_rec.clone(), metamodelica::sourceInfo!())?);
            lhs_subs = ComponentRef::subscriptsAllFlat(lhs_rec.clone());
            rhs_subs = ComponentRef::subscriptsAllFlat(rhs_rec.clone());
            if List::compareLength(lhs_lst.clone(), rhs_lst.clone())? == 0 && !(Type::isExternalObject(Type::arrayElementType(Expression::typeOf(var_field!((*eqn).lhs, Equation::RECORD_EQUATION).clone())))) {
                for mut tpl in &*List::zip(lhs_lst.clone(), rhs_lst.clone()) {
                    let mut tpl = tpl.clone();
                    (lhs, rhs) = tpl.clone();
                    lhs_exp = Expression::fromCref(ComponentRef::mergeSubscripts(lhs_subs.clone(), BVariable::getVarName(lhs.clone()), true, false, false)?, false)?;
                    rhs_exp = Expression::fromCref(ComponentRef::mergeSubscripts(rhs_subs.clone(), BVariable::getVarName(rhs.clone()), true, false, false)?, false)?;
                    stmts = cons(Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: lhs_exp.clone(), rhs: rhs_exp.clone(), ty: Expression::typeOf(lhs_exp.clone()), source: var_field!((*eqn).source, Equation::RECORD_EQUATION).clone() }), stmts.clone());
                }
            } else {
                stmts = list![Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: var_field!((*eqn).lhs, Equation::RECORD_EQUATION).clone(), rhs: var_field!((*eqn).rhs, Equation::RECORD_EQUATION).clone(), ty: var_field!((*eqn).ty, Equation::RECORD_EQUATION).clone(), source: var_field!((*eqn).source, Equation::RECORD_EQUATION).clone() })];
            }
            stmts.clone()
        },
        Deref @ RECORD_EQUATION { .. } => {
            list![Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: var_field!((*eqn).lhs, Equation::RECORD_EQUATION).clone(), rhs: var_field!((*eqn).rhs, Equation::RECORD_EQUATION).clone(), ty: var_field!((*eqn).ty, Equation::RECORD_EQUATION).clone(), source: var_field!((*eqn).source, Equation::RECORD_EQUATION).clone() })]
        },
        Deref @ FOR_EQUATION { .. } => {
            let mut iter_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut range_lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut iter: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            (iter_lst, range_lst, _) = Iterator::getFrames(var_field!((*eqn).iter, Equation::FOR_EQUATION).clone())?;
            body = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
        for mut body_eqn in (var_field!((*eqn).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = toStatement(body_eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut tpl in &*List::zip(iter_lst.clone(), range_lst.clone()).reverse() {
                let mut tpl = tpl.clone();
                (iter, range) = tpl.clone();
                body = list![Arc::new(Statement::NFStatement::FOR { source: var_field!((*eqn).source, Equation::FOR_EQUATION).clone(), forType: Arc::new(openmodelica_nf_frontend::NFStatement::ForType::NORMAL), body: body.clone(), range: Some(range.clone()), iterator: ComponentRef::node(iter.clone())? })];
            }
            body.clone()
        },
        Deref @ IF_EQUATION { .. } => {
            list![Arc::new(Statement::NFStatement::IF { branches: IfEquationBody::toStatement(var_field!((*eqn).body, Equation::IF_EQUATION).clone())?, source: var_field!((*eqn).source, Equation::IF_EQUATION).clone() })]
        },
        Deref @ WHEN_EQUATION { .. } => {
            list![Arc::new(Statement::NFStatement::WHEN { branches: WhenEquationBody::toStatement(var_field!((*eqn).body, Equation::WHEN_EQUATION).clone())?, source: var_field!((*eqn).source, Equation::WHEN_EQUATION).clone() })]
        },
        Deref @ ALGORITHM { .. } => {
            var_field!((*eqn).alg, Equation::ALGORITHM).statements.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.toStatement")); __mm_s.push_str(&*literal!(" failed it is not yet supported for:\n")); __mm_s.push_str(&*toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(stmts)
    }

}

pub mod IfEquationBody {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct IfEquationBody {
        /// the if-condition
        pub condition: Arc<Expression::NFExpression>,
        /// body equations
        pub then_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>,
        /// optional elseif equation
        pub else_if: Option<Arc<IfEquationBody>>,
    }

    pub type IF_EQUATION_BODY = IfEquationBody;

    pub fn toEquation(mut body: Arc<IfEquationBody>, mut source: Arc<DAE::ElementSource>, mut init: bool) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
        let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut attr: Arc<EquationAttributes::EquationAttributes>;
        let mut isAlgorithm: bool = false;
        let mut e: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut alg: Arc<Algorithm::NFAlgorithm>;
        let mut size: i32 = 0;
        (attr, isAlgorithm) = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
        Deref @ metamodelica::List::Cons { head: then_eqn, tail: Deref @ metamodelica::List::Nil } => {
            (if (Equation::isDiscrete(then_eqn.clone())) {default(EquationKind::DISCRETE.clone(), init.clone(), None, None)} else {default(EquationKind::CONTINUOUS.clone(), init.clone(), None, None)}, Equation::isAlgorithm(then_eqn.clone()))
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.toEquation")); __mm_s.push_str(&*literal!(": Creating if-equation with multiple body equations. Unsure of type:\n")); __mm_s.push_str(&*toString(body.clone(), (literal!("")).clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone()])?;
            }
            (default(EquationKind::CONTINUOUS.clone(), init.clone(), None, None), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        e = Arc::new(Equation::Equation::IF_EQUATION { size: self::size(body.clone(), false), body: body.clone(), source: source.clone(), attr: attr.clone() });
        if isAlgorithm.clone() {
            alg = Arc::new(Algorithm::NFAlgorithm { statements: Equation::toStatement(e.clone())?, inputs: metamodelica::nil(), outputs: metamodelica::nil(), stmtDiffInfo: None, scope: Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), source: source.clone() });
            alg = Algorithm::setInputsOutputs(alg.clone())?;
            size = {
        let mut __acc: i32 = 0;
        for mut out in (alg.outputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::size(out.clone(), false, false);
            __acc += __x;
        }
        __acc
    };
            eqn = Pointer::create(Arc::new(Equation::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: alg.source.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: attr.clone() }));
        } else {
            eqn = Pointer::create(e.clone());
        }
        Ok(eqn)
    }

    pub fn makeIfEquation(mut body: Arc<IfEquationBody>, mut idx: Pointer::Pointer<i32>, mut r#str: ArcStr, mut iter: Arc<Iterator::Iterator>, mut source: Arc<DAE::ElementSource>, mut attr: Arc<EquationAttributes::EquationAttributes>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
        let mut eq: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut e: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        e = makeIfEquationEqn(body.clone(), iter.clone(), source.clone(), attr.clone())?;
        eq = Pointer::create(e.clone());
        Equation::createName(eq.clone(), idx.clone(), (r#str.clone()).clone())?;
        Ok(eq)
    }

    fn makeIfEquationEqn(mut body: Arc<IfEquationBody>, mut iter: Arc<Iterator::Iterator>, mut source: Arc<DAE::ElementSource>, mut attr: Arc<EquationAttributes::EquationAttributes>) -> Result<Arc<Equation::Equation>> {
        let mut e: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        e = Arc::new(Equation::Equation::IF_EQUATION { attr: attr.clone(), source: source.clone(), body: body.clone(), size: size(body.clone(), false) });
        if !(Iterator::isEmpty(iter.clone())) {
            e = Arc::new(Equation::Equation::FOR_EQUATION { attr: attr.clone(), source: source.clone(), body: list![e.clone()], iter: iter.clone(), size: size(body.clone(), false) * Iterator::size(iter.clone(), false) });
            e = Inline::inlineForEquation(e.clone())?;
        }
        Ok(e)
    }

    pub fn toString(mut body: Arc<IfEquationBody>, mut indent: ArcStr, mut elseStr: ArcStr, mut selfCall: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (elseStr.clone()).clone();
        if !(selfCall.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); ArcStr::from(__mm_s) }).clone();
        }
        if !(Expression::isEnd(body.condition.clone())) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*Expression::toString(body.condition.clone())?); __mm_s.push_str(&*literal!(" then\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        for mut eqn in &*body.then_eqns.clone() {
            let mut eqn = eqn.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Equation::toString(Pointer::access(eqn.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        if isSome(body.else_if.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toString(Util::getOption(body.else_if.clone())?, (indent.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("else")); ArcStr::from(__mm_s) }).clone(), true)?); ArcStr::from(__mm_s) }).clone();
        }
        if !(selfCall.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("end if;")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn map(mut ifBody: Arc<IfEquationBody>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<IfEquationBody>> {
        let mut ifBody: Arc<IfEquationBody> = ifBody;
        ifBody = mapEqnExpCref(ifBody.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static> = Arc::new({ let __pe_b1 = funcExp.clone(); let __pe_b2 = funcCrefOpt.clone(); let __pe_b3 = mapFunc.clone(); move |__pe_a0| Equation::map(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }); move |__pe_a0| Ok(Pointer::apply(__pe_a0, __pe_b1.clone())) }), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
        Ok(ifBody)
    }

    pub fn mapCondition(mut ifBody: Arc<IfEquationBody>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<IfEquationBody> {
        let mut ifBody: Arc<IfEquationBody> = ifBody;
        let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        condition = mapFunc(ifBody.condition.clone(), funcExp.clone()).unwrap();
        if !(referenceEq(&condition.clone(),&ifBody.condition.clone())) {
            assign_field!(ifBody.condition = condition.clone());
        }
        assign_field!(ifBody.else_if = Util::applyOption(ifBody.else_if.clone(), Arc::new({ let __pe_b1 = funcExp.clone(); let __pe_b2 = funcCrefOpt.clone(); let __pe_b3 = mapFunc.clone(); move |__pe_a0| Ok(mapCondition(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())) })));
        ifBody
    }

    pub fn mapEqnExpCref(mut ifBody: Arc<IfEquationBody>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> + 'static>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<IfEquationBody>> {
        let mut ifBody: Arc<IfEquationBody> = ifBody;
        let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut else_if: Arc<IfEquationBody>;
        let mut old_else_if: Arc<IfEquationBody>;
        condition = mapFunc(ifBody.condition.clone(), funcExp.clone())?;
        if !(referenceEq(&condition.clone(),&ifBody.condition.clone())) {
            assign_field!(ifBody.condition = condition.clone());
        }
        assign_field!(ifBody.then_eqns = List::map(ifBody.then_eqns.clone(), func.clone()));
        if Util::isSome(ifBody.else_if.clone()) {
            old_else_if = Util::getOption(ifBody.else_if.clone())?;
            else_if = mapEqnExpCref(old_else_if.clone(), func.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
            if !(referenceEq(&else_if.clone(),&old_else_if.clone())) {
                assign_field!(ifBody.else_if = Some(else_if.clone()));
            }
        }
        Ok(ifBody)
    }

    pub fn size(mut body: Arc<IfEquationBody>, mut resize: bool) -> i32 {
        let mut size: i32 = {
        let mut __acc: i32 = 0;
        for mut eqn in (body.then_eqns.clone()).into_iter().cloned() {
            let __x = Equation::size(eqn.clone(), resize.clone()).unwrap();
            __acc += __x;
        }
        __acc
    };
        size
    }

    pub fn isEqual(mut body1: Arc<IfEquationBody>, mut body2: Arc<IfEquationBody>) -> bool {
        let mut b: bool = false;
        b = List::all({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for (b1, b2) in (&(body1.then_eqns.clone())).into_iter().zip((&(body2.then_eqns.clone())).into_iter()) {
            let __x = Equation::isEqualPtr(b1.clone(), b2.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, std::sync::Arc::new(fnptr!(Util::id, _))) && Util::optionEqual(body1.else_if.clone(), body2.else_if.clone(), (std::sync::Arc::new(fnptr!(isEqual, Arc<IfEquationBody>, Arc<IfEquationBody>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<IfEquationBody>, Arc<IfEquationBody>) -> Result<bool> + 'static>));
        b
    }

    pub fn createNames(mut body: Arc<IfEquationBody>, mut idx: Pointer::Pointer<i32>, mut context: ArcStr) -> Result<()> {
        for mut eqn in &*body.then_eqns.clone() {
            let mut eqn = eqn.clone();
            Equation::createName(eqn.clone(), idx.clone(), (context.clone()).clone())?;
        }
        if Util::isSome(body.else_if.clone()) {
            createNames(Util::getOption(body.else_if.clone())?, idx.clone(), (context.clone()).clone())?;
        }
        Ok(())
    }

    pub fn toStatement(mut body: Arc<IfEquationBody>) -> Result<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>> {
        let mut stmts: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        let mut stmt: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>);
        let mut condition: Arc<Expression::NFExpression> = if (Expression::isEnd(body.condition.clone())) {Arc::new(Expression::NFExpression::BOOLEAN { value: true })} else {body.condition.clone()};
        stmt = (condition.clone(), List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Statement::NFStatement>>>>> = metamodelica::nil();
        for mut eqn in (body.then_eqns.clone()).into_iter().cloned() {
            let __x = Equation::toStatement(Pointer::access(eqn.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
        if Util::isSome(body.else_if.clone()) {
            stmts = cons(stmt.clone(), toStatement(Util::getOption(body.else_if.clone())?)?);
        } else {
            stmts = list![stmt.clone()];
        }
        Ok(stmts)
    }

    pub fn createResidual(mut body: Arc<IfEquationBody>, mut res: Arc<ComponentRef::NFComponentRef>, mut new: bool, mut allowFail: bool) -> Result<Arc<IfEquationBody>> {
        let mut body_res: Arc<IfEquationBody>;
        let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        body_res = Arc::new(IfEquationBody { condition: body.condition.clone(), then_eqns: metamodelica::nil(), else_if: Util::applyOption(body.else_if.clone(), Arc::new({ let __pe_b1 = res.clone(); let __pe_b2 = new.clone(); let __pe_b3 = allowFail.clone(); move |__pe_a0| createResidual(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) })) });
        body_res = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
        Deref @ metamodelica::List::Cons { head: eqn_ptr, tail: Deref @ metamodelica::List::Nil } => {
            assign_field!(body_res.then_eqns = cons(Equation::createResidual(eqn_ptr.clone(), Some(res.clone()), new.clone(), allowFail.clone())?, body_res.then_eqns.clone()));
            body_res.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.createResidual")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(body.clone(), (literal!("")).clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(body_res)
    }

    pub fn inline(mut body: Arc<IfEquationBody>, mut eqn: Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> {
        let mut eqn: Arc<Equation::Equation> = eqn;
        let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut success: bool = false;
        (lhs, success) = getLHS(body.clone(), Arc::new(openmodelica_nf_frontend::NFExpression::END))?;
        if success.clone() {
            rhs = SimplifyExp::simplify(getRHS(body.clone())?, false)?;
            eqn = Equation::makeAssignmentUpdate(eqn.clone(), lhs.clone(), rhs.clone(), Equation::getForIterator(eqn.clone()), Equation::getAttributes(eqn.clone()))?;
        }
        Ok(eqn)
    }

    pub fn getLHS(mut body: Arc<IfEquationBody>, mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut success: bool = true;
        let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        exp = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
        Deref @ metamodelica::List::Cons { head: eqn_ptr, tail: Deref @ metamodelica::List::Nil } => {
            let __pa0 = ::match_deref::match_deref! { match &(Equation::getLHS(Pointer::access(eqn_ptr.clone()))?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            new_exp = __pa0.clone();
            if Expression::isEnd(exp.clone()) || Expression::isEqual(exp.clone(), new_exp.clone())? {
                if Util::isSome(body.else_if.clone()) {
                    (new_exp, success) = getLHS(Util::getOption(body.else_if.clone())?, new_exp.clone())?;
                }
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.getLHS")); __mm_s.push_str(&*literal!(" failed because of ambiguous LHS for:\n")); __mm_s.push_str(&*toString(body.clone(), (literal!("")).clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone())?;
                }
                success = false;
            }
            new_exp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.getLHS")); __mm_s.push_str(&*literal!(" failed because of un-split if-equation:\n")); __mm_s.push_str(&*toString(body.clone(), (literal!("")).clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((exp, success))
    }

    pub fn getRHS(mut body: Arc<IfEquationBody>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        exp = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
        Deref @ metamodelica::List::Cons { head: eqn_ptr, tail: Deref @ metamodelica::List::Nil } => {
            let __pa0 = ::match_deref::match_deref! { match &(Equation::getRHS(Pointer::access(eqn_ptr.clone()))?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            new_exp = __pa0.clone();
            if Util::isSome(body.else_if.clone()) {
                new_exp = Arc::new(Expression::NFExpression::IF { ty: Expression::typeOf(new_exp.clone()), condition: body.condition.clone(), trueBranch: new_exp.clone(), falseBranch: getRHS(Util::getOption(body.else_if.clone())?)? });
            }
            new_exp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.getRHS")); __mm_s.push_str(&*literal!(" failed because of un-split if-equation:\n")); __mm_s.push_str(&*toString(body.clone(), (literal!("")).clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    pub fn split(mut body: Arc<IfEquationBody>) -> Result<Arc<metamodelica::List<Arc<IfEquationBody>>>> {
        let mut bodies: Arc<metamodelica::List<Arc<IfEquationBody>>> = metamodelica::nil();
        let mut conditions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut then_eqns: metamodelica::Array<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = arrayCreate((body.then_eqns.clone().len() as i32), metamodelica::nil());
        let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut tmp: Option<Arc<IfEquationBody>> = None;
        (conditions, then_eqns) = splitCollect(sortForSplit(body.clone())?, conditions.clone(), then_eqns.clone())?;
        let __range0 = 1..=(then_eqns.clone().borrow().len() as i32);
        for mut i in __range0 {
            tmp = None;
            let __range1 = &*List::zip(conditions.clone(), then_eqns.borrow()[(i.clone()-1) as usize].clone());
            for mut tpl in __range1 {
                let mut tpl = tpl.clone();
                (condition, eqn) = tpl.clone();
                tmp = Some(Arc::new(IfEquationBody { condition: condition.clone(), then_eqns: list![eqn.clone()], else_if: tmp.clone() }));
            }
            bodies = cons(Util::getOption(tmp.clone())?, bodies.clone());
        }
        Ok(bodies)
    }

    pub fn simplify(mut body: Option<Arc<IfEquationBody>>) -> Option<Arc<IfEquationBody>> {
        let mut body: Option<Arc<IfEquationBody>> = body;
        body = (::match_deref::match_deref! { match &(body.clone()) {
        Some(b) => {
            let mut b = (*b).clone();
            if Expression::isTrue(b.condition.clone()) {
                assign_field!(
                    b.condition = Arc::new(openmodelica_nf_frontend::NFExpression::END),
                    b.else_if = None
                );
            } else {
                assign_field!(b.else_if = simplify(b.else_if.clone()));
            }
            if Expression::isFalse(b.condition.clone()) {
                body = b.else_if.clone();
            } else {
                body = Some(b.clone());
            }
            body.clone()
        },
        _ => {
            body.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        body
    }

    pub fn isRecordOrTupleEquation(mut body: Arc<IfEquationBody>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
        Deref @ metamodelica::List::Cons { head: eqn_ptr, tail: Deref @ metamodelica::List::Nil } => {
            Equation::isRecordOrTupleEquation(eqn_ptr.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    fn sortForSplit(mut body: Arc<IfEquationBody>) -> Result<Arc<IfEquationBody>> {
        fn compareLHS(mut eqn1: Pointer::Pointer<Arc<Equation::Equation>>, mut eqn2: Pointer::Pointer<Arc<Equation::Equation>>) -> bool {
            let mut b: bool = 0 < Expression::compare(Util::getOption(Equation::getLHS(Pointer::access(eqn1.clone())).unwrap()).unwrap(), Util::getOption(Equation::getLHS(Pointer::access(eqn2.clone())).unwrap()).unwrap()).unwrap();
            b
        }

        let mut body: Arc<IfEquationBody> = body;
        let mut discretes: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut continuous: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        (discretes, continuous) = List::splitOnTrue(body.then_eqns.clone(), (std::sync::Arc::new(fnptr!(Equation::isDiscrete, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>));
        discretes = List::sort(discretes.clone(), (std::sync::Arc::new(fnptr!(compareLHS, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>))?;
        continuous = List::sort(continuous.clone(), (std::sync::Arc::new(fnptr!(compareLHS, Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>))?;
        assign_field!(
            body.then_eqns = listAppend(discretes.clone(), continuous.clone()),
            body.else_if = Util::applyOption(body.else_if.clone(), (std::sync::Arc::new(sortForSplit) as std::sync::Arc<dyn ::std::ops::Fn(Arc<IfEquationBody>) -> Result<Arc<IfEquationBody>> + 'static>))
        );
        Ok(body)
    }

    fn splitCollect(mut body: Arc<IfEquationBody>, mut conditions: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut then_eqns: metamodelica::Array<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, metamodelica::Array<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>)> {
        let mut conditions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = conditions;
        let mut then_eqns: metamodelica::Array<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = then_eqns;
        let mut i: i32 = 1;
        conditions = cons(body.condition.clone(), conditions.clone());
        for mut eqn in &*body.then_eqns.clone() {
            let mut eqn = eqn.clone();
            {
                let __cell0 = cons(eqn.clone(), then_eqns.borrow()[(i.clone()-1) as usize].clone());
                then_eqns.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
            }
            i = i.clone() + 1;
        }
        if Util::isSome(body.else_if.clone()) {
            (conditions, then_eqns) = splitCollect(Util::getOption(body.else_if.clone())?, conditions.clone(), then_eqns.clone())?;
        }
        Ok((conditions, then_eqns))
    }

}

pub mod WhenEquationBody {
    use super::*;
    /// equation when condition then cr = exp, reinit(...), terminate(...) or assert(...)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct WhenEquationBody {
        /// the when-condition
        pub condition: Arc<Expression::NFExpression>,
        /// body statements
        pub when_stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>,
        /// optional elsewhen body
        pub else_when: Option<Arc<WhenEquationBody>>,
    }

    pub type WHEN_EQUATION_BODY = WhenEquationBody;

    pub fn fromFlatList(mut flat_list: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>)>>, mut body: Option<Arc<WhenEquationBody>>) -> Option<Arc<WhenEquationBody>> {
        let mut body: Option<Arc<WhenEquationBody>> = body;
        body = (::match_deref::match_deref! { match &(flat_list.clone()) {
        Deref @ metamodelica::List::Cons { head: (condition, stmts), tail: tail } => {
            fromFlatList(tail.clone(), Some(Arc::new(WhenEquationBody { condition: condition.clone(), when_stmts: stmts.clone(), else_when: body.clone() })))
        },
        _ => {
            body.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        body
    }

    pub fn toString(mut body: Arc<WhenEquationBody>, mut indent: ArcStr, mut elseStr: ArcStr, mut selfCall: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (elseStr.clone()).clone();
        if !(selfCall.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("when ")); __mm_s.push_str(&*Expression::toString(body.condition.clone())?); __mm_s.push_str(&*literal!(" then\n")); ArcStr::from(__mm_s) }).clone();
        for mut stmt in &*body.when_stmts.clone() {
            let mut stmt = stmt.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*WhenStatement::toString(stmt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        if isSome(body.else_when.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toString(Util::getOption(body.else_when.clone())?, (indent.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("else")); ArcStr::from(__mm_s) }).clone(), true)?); ArcStr::from(__mm_s) }).clone();
        }
        if !(selfCall.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("end when;")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn size(mut body: Arc<WhenEquationBody>, mut resize: bool) -> i32 {
        let mut s: i32 = {
        let mut __acc: i32 = 0;
        for mut stmt in (body.when_stmts.clone()).into_iter().cloned() {
            let __x = WhenStatement::size(stmt.clone(), resize.clone()).unwrap();
            __acc += __x;
        }
        __acc
    };
        s
    }

    pub fn getType(mut body: Arc<WhenEquationBody>) -> Result<Arc<Type::NFType>> {
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        ty = (::match_deref::match_deref! { match &(body.when_stmts.clone()) {
        Deref @ metamodelica::List::Cons { head: stmt, tail: Deref @ metamodelica::List::Nil } => {
            WhenStatement::getType(stmt.clone())
        },
        _ if (List::all({
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut st in (body.when_stmts.clone()).into_iter().cloned() {
            let __x = WhenStatement::getType(st.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(Type::isAny, Arc<Type::NFType>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<bool> + 'static>))) => {
            Arc::new(openmodelica_nf_frontend::NFType::ANY)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.getType")); __mm_s.push_str(&*literal!(" failed because of not properly split up when equation body: ")); __mm_s.push_str(&*toString(body.clone(), (literal!("")).clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(ty)
    }

    pub fn isEqual(mut body1: Arc<WhenEquationBody>, mut body2: Arc<WhenEquationBody>) -> Result<bool> {
        let mut b: bool = false;
        b = Expression::isEqual(body1.condition.clone(), body2.condition.clone())? && List::all({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for (b1, b2) in (&(body1.when_stmts.clone())).into_iter().zip((&(body2.when_stmts.clone())).into_iter()) {
            let __x = WhenStatement::isEqual(b1.clone(), b2.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, std::sync::Arc::new(fnptr!(Util::id, _))) && Util::optionEqual(body1.else_when.clone(), body2.else_when.clone(), (std::sync::Arc::new(isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<WhenEquationBody>, Arc<WhenEquationBody>) -> Result<bool> + 'static>));
        Ok(b)
    }

    pub fn getBodyAttributes(mut body: Arc<WhenEquationBody>) -> Result<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>, Option<Arc<WhenEquationBody>>)> {
        fn getConditions(mut cond: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
            let mut conditions: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            conditions = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::CREF { cref, .. } => {
            list![cref.clone()]
        },
        Deref @ Expression::ARRAY { .. } => {
            List::flatten({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut elem in (var_field!((*cond).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = getConditions(elem.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ Expression::CALL { .. } if (Call::isNamed(var_field!((*cond).call, Expression::NFExpression::CALL).clone(), (literal!("initial")).clone())?) => {
            metamodelica::nil()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.getBodyAttributes.getConditions")); __mm_s.push_str(&*literal!(" failed for condition: ")); __mm_s.push_str(&*Expression::toString(cond.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok(conditions)
        }

        let mut conditions: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut when_stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>> = body.when_stmts.clone();
        let mut else_when: Option<Arc<WhenEquationBody>> = body.else_when.clone();
        conditions = getConditions(body.condition.clone())?;
        Ok((conditions, when_stmts, else_when))
    }

    pub fn toStatement(mut body: Arc<WhenEquationBody>) -> Result<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>> {
        let mut stmts: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        let mut stmt: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>);
        stmt = (body.condition.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut st in (body.when_stmts.clone()).into_iter().cloned() {
            let __x = WhenStatement::toStatement(st.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if Util::isSome(body.else_when.clone()) {
            stmts = cons(stmt.clone(), toStatement(Util::getOption(body.else_when.clone())?)?);
        } else {
            stmts = list![stmt.clone()];
        }
        Ok(stmts)
    }

    pub fn map(mut whenBody: Arc<WhenEquationBody>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<WhenEquationBody> {
        let mut whenBody: Arc<WhenEquationBody> = whenBody;
        let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        condition = mapFunc(whenBody.condition.clone(), funcExp.clone()).unwrap();
        if !(referenceEq(&condition.clone(),&whenBody.condition.clone())) {
            assign_field!(whenBody.condition = condition.clone());
        }
        assign_field!(
            whenBody.when_stmts = List::map(whenBody.when_stmts.clone(), Arc::new({ let __pe_b1 = funcExp.clone(); let __pe_b2 = funcCrefOpt.clone(); let __pe_b3 = mapFunc.clone(); move |__pe_a0| WhenStatement::map(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) })),
            whenBody.else_when = Util::applyOption(whenBody.else_when.clone(), Arc::new({ let __pe_b1 = funcExp.clone(); let __pe_b2 = funcCrefOpt.clone(); let __pe_b3 = mapFunc.clone(); move |__pe_a0| Ok(map(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())) }))
        );
        whenBody
    }

    pub fn mapCondition(mut whenBody: Arc<WhenEquationBody>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<WhenEquationBody> {
        let mut whenBody: Arc<WhenEquationBody> = whenBody;
        let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        condition = mapFunc(whenBody.condition.clone(), funcExp.clone()).unwrap();
        if !(referenceEq(&condition.clone(),&whenBody.condition.clone())) {
            assign_field!(whenBody.condition = condition.clone());
        }
        assign_field!(whenBody.else_when = Util::applyOption(whenBody.else_when.clone(), Arc::new({ let __pe_b1 = funcExp.clone(); let __pe_b2 = funcCrefOpt.clone(); let __pe_b3 = mapFunc.clone(); move |__pe_a0| Ok(mapCondition(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())) })));
        whenBody
    }

    pub fn split(mut body: Arc<WhenEquationBody>) -> Result<Arc<metamodelica::List<Arc<WhenEquationBody>>>> {
        let mut bodies: Arc<metamodelica::List<Arc<WhenEquationBody>>> = metamodelica::nil();
        let mut discr_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut state_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
        let mut discr_marks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
        let mut flat_when: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>)>> = metamodelica::nil();
        let mut flat_new: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>)>> = metamodelica::nil();
        let mut discretes: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut states: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut set: CrefSet;
        let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut acc_condition: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(openmodelica_nf_frontend::NFType::INTEGER) });
        let mut stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>> = metamodelica::nil();
        let mut assigns: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>> = metamodelica::nil();
        let mut stmt: Option<Arc<WhenStatement::WhenStatement>> = None;
        let mut new_body: Option<Arc<WhenEquationBody>> = None;
        flat_when = collectForSplit(Some(body.clone()), discr_map.clone(), state_set.clone())?;
        discretes = UnorderedMap::keyList(discr_map.clone());
        states = UnorderedSet::toList(state_set.clone());
        for mut disc in &*discretes.clone() {
            let mut disc = disc.clone();
            if !(UnorderedSet::contains(disc.clone(), discr_marks.clone())?) {
                set = UnorderedMap::getSafe(disc.clone(), discr_map.clone(), metamodelica::sourceInfo!())?;
                for mut marked in &*UnorderedSet::toList(set.clone()) {
                    let mut marked = marked.clone();
                    UnorderedSet::add(marked.clone(), discr_marks.clone())?;
                }
                flat_new = metamodelica::nil();
                for mut tpl in &*flat_when.clone() {
                    let mut tpl = tpl.clone();
                    (condition, stmts) = tpl.clone();
                    assigns = getAssignments(set.clone(), stmts.clone())?;
                    if !(assigns.clone().is_empty()) {
                        condition = combineConditions(acc_condition.clone(), condition.clone(), false);
                        acc_condition = Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(openmodelica_nf_frontend::NFType::INTEGER) });
                        flat_new = cons((condition.clone(), assigns.clone()), flat_new.clone());
                    } else {
                        acc_condition = combineConditions(acc_condition.clone(), condition.clone(), true);
                    }
                }
                new_body = fromFlatList(flat_new.clone(), None);
                if Util::isSome(new_body.clone()) {
                    bodies = cons(Util::getOption(new_body.clone())?, bodies.clone());
                } else {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.split")); __mm_s.push_str(&*literal!(" failed because when partition for: ")); __mm_s.push_str(&*ComponentRef::toString(disc.clone())?); __mm_s.push_str(&*literal!(" could not be recovered.")); ArcStr::from(__mm_s) }).clone()])?;
                }
            }
        }
        for mut state in &*states.clone() {
            let mut state = state.clone();
            flat_new = metamodelica::nil();
            for mut tpl in &*flat_when.clone() {
                let mut tpl = tpl.clone();
                (condition, stmts) = tpl.clone();
                stmt = getFirstReinit(state.clone(), stmts.clone())?;
                if Util::isSome(stmt.clone()) {
                    condition = combineConditions(acc_condition.clone(), condition.clone(), false);
                    acc_condition = Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(openmodelica_nf_frontend::NFType::INTEGER) });
                    flat_new = cons((condition.clone(), list![Util::getOption(stmt.clone())?]), flat_new.clone());
                } else {
                    acc_condition = combineConditions(acc_condition.clone(), condition.clone(), true);
                }
            }
            new_body = fromFlatList(flat_new.clone(), None);
            if Util::isSome(new_body.clone()) {
                bodies = cons(Util::getOption(new_body.clone())?, bodies.clone());
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.split")); __mm_s.push_str(&*literal!(" failed because when partition for: ")); __mm_s.push_str(&*ComponentRef::toString(state.clone())?); __mm_s.push_str(&*literal!(" could not be recovered.")); ArcStr::from(__mm_s) }).clone()])?;
            }
        }
        flat_new = metamodelica::nil();
        for mut tpl in &*flat_when.clone() {
            let mut tpl = tpl.clone();
            (condition, stmts) = tpl.clone();
            stmts = {
        let mut __acc: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>> = metamodelica::nil();
        for mut stmt in (stmts.clone()).into_iter().cloned() {
            if !(!(WhenStatement::isAssignOrReinit(stmt.clone()))) { continue; }
            let __x = stmt.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if !(stmts.clone().is_empty()) {
                condition = combineConditions(acc_condition.clone(), condition.clone(), false);
                acc_condition = Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(openmodelica_nf_frontend::NFType::INTEGER) });
                flat_new = cons((condition.clone(), stmts.clone()), flat_new.clone());
                new_body = fromFlatList(flat_new.clone(), None);
                if Util::isSome(new_body.clone()) {
                    bodies = cons(Util::getOption(new_body.clone())?, bodies.clone());
                }
            } else {
                acc_condition = combineConditions(acc_condition.clone(), condition.clone(), true);
            }
        }
        bodies = bodies.clone().reverse();
        Ok(bodies)
    }

    pub fn simplify(mut body: Option<Arc<WhenEquationBody>>) -> Result<Option<Arc<WhenEquationBody>>> {
        let mut body: Option<Arc<WhenEquationBody>> = body;
        body = (::match_deref::match_deref! { match &(body.clone()) {
        Some(b @ Deref @ WhenEquationBody { condition: condition @ Deref @ Expression::ARRAY { .. }, .. }) => {
            let mut conditions: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut b = (*b).clone();
            assign_field!(b.else_when = simplify(b.else_when.clone())?);
            conditions = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut elem in (var_field!((**condition).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            if !(!(Expression::isBoolean(elem.clone()))) { continue; }
            let __x = elem.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if conditions.clone().is_empty() {
                body = b.else_when.clone();
            } else if List::hasOneElement(conditions.clone()) {
                assign_field!(b.condition = listHead(conditions.clone())?);
                body = Some(b.clone());
            } else {
                assign_field!(b.condition = Expression::makeArrayCheckLiteral(Arc::new(Type::NFType::ARRAY { elementType: Arc::new(openmodelica_nf_frontend::NFType::BOOLEAN), dimensions: list![Dimension::fromInteger((conditions.clone().len() as i32), Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec(conditions.clone().into_iter().cloned().collect())));
                body = Some(b.clone());
            }
            body.clone()
        },
        Some(b) => {
            let mut b = (*b).clone();
            assign_field!(b.else_when = simplify(b.else_when.clone())?);
            if Expression::isBoolean(b.condition.clone()) {
                body = b.else_when.clone();
            } else {
                body = Some(b.clone());
            }
            body.clone()
        },
        _ => {
            body.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(body)
    }

    pub fn getAllAssigned(mut body: Arc<WhenEquationBody>) -> Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> {
        let mut assigned: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut stmt in &*body.when_stmts.clone() {
            let mut stmt = stmt.clone();
            assigned = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: lhs, .. }, .. } => {
            cons(lhs.clone(), assigned.clone())
        },
        _ => {
            assigned.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        assigned
    }

    pub fn isRecordOrTupleEquation(mut body: Arc<WhenEquationBody>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(body.when_stmts.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::TUPLE { .. }, .. }, tail: Deref @ metamodelica::List::Nil } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::RECORD { .. }, .. }, tail: Deref @ metamodelica::List::Nil } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref, .. }, .. }, tail: Deref @ metamodelica::List::Nil } => {
            BVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::isRecord, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!())
        },
        _ if (List::count(body.when_stmts.clone(), (std::sync::Arc::new(fnptr!(WhenStatement::isAssign, Arc<WhenStatement::WhenStatement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<WhenStatement::WhenStatement>) -> Result<bool> + 'static>)) > 1) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub type CrefSet = Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;

    fn collectForSplit(mut body_opt: Option<Arc<WhenEquationBody>>, mut discr_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>>, mut state_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>)>>> {
        let mut flat_when: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>)>> = metamodelica::nil();
        let mut body: Arc<WhenEquationBody>;
        if Util::isSome(body_opt.clone()) {
            body = Util::getOption(body_opt.clone())?;
            for mut stmt in &*body.when_stmts.clone() {
                let mut stmt = stmt.clone();
                let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            addCrefsMap(discr_map.clone(), list![cref.clone()])?;
            ()
        },
        Deref @ WhenStatement::ASSIGN { lhs: tpl @ Deref @ Expression::TUPLE { .. }, .. } => {
            addCrefsMap(discr_map.clone(), UnorderedSet::toList(Expression::extractCrefs(tpl.clone())))?;
            ()
        },
        Deref @ WhenStatement::REINIT { stateVar: cref, .. } => {
            UnorderedSet::add(cref.clone(), state_set.clone())?;
            ()
        },
        Deref @ WhenStatement::ASSIGN { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.collectForSplit")); __mm_s.push_str(&*literal!(" failed because lhs of statement is not a cref: ")); __mm_s.push_str(&*WhenStatement::toString(stmt.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            flat_when = cons((body.condition.clone(), body.when_stmts.clone()), collectForSplit(body.else_when.clone(), discr_map.clone(), state_set.clone())?);
        } else {
            flat_when = metamodelica::nil();
        }
        Ok(flat_when)
    }

    fn addCrefsMap(mut discr_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>>, mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        let mut set_new: CrefSet;
        let mut set: CrefSet = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
        for mut c in &*crefs.clone() {
            let mut c = c.clone();
            if UnorderedMap::contains(c.clone(), discr_map.clone()) {
                set_new = UnorderedMap::getSafe(c.clone(), discr_map.clone(), metamodelica::sourceInfo!())?;
                if !(referenceEq(&set.clone(),&set_new.clone())) {
                    set = UnorderedSet::union(set.clone(), set_new.clone())?;
                }
            } else {
                UnorderedSet::add(c.clone(), set.clone())?;
            }
        }
        for mut c in &*crefs.clone() {
            let mut c = c.clone();
            UnorderedMap::add(c.clone(), set.clone(), discr_map.clone())?;
        }
        Ok(())
    }

    fn getAssignments(mut crefSet: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>) -> Result<Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>> {
        let mut assigns: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>> = metamodelica::nil();
        for mut stmt in &*stmts.clone() {
            let mut stmt = stmt.clone();
            let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref, .. }, .. } if (UnorderedSet::contains(cref.clone(), crefSet.clone())?) => {
            assigns = cons(stmt.clone(), assigns.clone());
            ()
        },
        Deref @ WhenStatement::ASSIGN { lhs: tpl @ Deref @ Expression::TUPLE { .. }, .. } if (List::any({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut c in (UnorderedSet::toList(Expression::extractCrefs(tpl.clone()))).into_iter().cloned() {
            let __x = UnorderedSet::contains(c.clone(), crefSet.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, std::sync::Arc::new(fnptr!(Util::id, _)))) => {
            assigns = cons(stmt.clone(), assigns.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(assigns)
    }

    fn getFirstReinit(mut cref: Arc<ComponentRef::NFComponentRef>, mut stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>) -> Result<Option<Arc<WhenStatement::WhenStatement>>> {
        let mut assign: Option<Arc<WhenStatement::WhenStatement>> = None;
        for mut stmt in &*stmts.clone() {
            let mut stmt = stmt.clone();
            let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ WhenStatement::REINIT { .. } if (ComponentRef::isEqual(cref.clone(), var_field!((*stmt).stateVar, WhenStatement::WhenStatement::REINIT).clone())?) => {
            assign = Some(stmt.clone());
            break;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(assign)
    }

    fn combineConditions(mut acc_condition: Arc<Expression::NFExpression>, mut condition: Arc<Expression::NFExpression>, mut invert: bool) -> Arc<Expression::NFExpression> {
        let mut condition: Arc<Expression::NFExpression> = condition;
        if invert.clone() {
            condition = Expression::logicNegate(condition.clone());
        }
        if !(Expression::isEmpty(acc_condition.clone())) {
            condition = Arc::new(Expression::NFExpression::LBINARY { exp1: acc_condition.clone(), operator: Operator::makeAnd(Arc::new(openmodelica_nf_frontend::NFType::BOOLEAN)), exp2: condition.clone() });
        }
        condition
    }

}

pub mod WhenStatement {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum WhenStatement {
        /// left_cr = right_exp
        ASSIGN {
            /// left hand side of assignment
            lhs: Arc<Expression::NFExpression>,
            /// right hand side of assignment
            rhs: Arc<Expression::NFExpression>,
            /// origin of assignment
            source: Arc<DAE::ElementSource>,
        },
        /// Reinit Statement
        REINIT {
            /// State variable to reinit
            stateVar: Arc<ComponentRef::NFComponentRef>,
            /// Value after reinit
            value: Arc<Expression::NFExpression>,
            /// origin of statement
            source: Arc<DAE::ElementSource>,
        },
        ASSERT {
            condition: Arc<Expression::NFExpression>,
            message: Arc<Expression::NFExpression>,
            level: Arc<Expression::NFExpression>,
            /// origin of statement
            source: Arc<DAE::ElementSource>,
        },
        /// The Modelica built-in terminate(msg)
        TERMINATE {
            message: Arc<Expression::NFExpression>,
            /// the origin of the component/equation/algorithm
            source: Arc<DAE::ElementSource>,
        },
        /// call with no return value, i.e. no equation.
        ///      Typically side effect call of external function but also
        ///      Connections.* i.e. Connections.root(...) functions.
        NORETCALL {
            exp: Arc<Expression::NFExpression>,
            /// the origin of the component/equation/algorithm
            source: Arc<DAE::ElementSource>,
        },
    }
    pub use self::WhenStatement::{ASSIGN,REINIT,ASSERT,TERMINATE,NORETCALL};
    pub fn toString(mut stmt: Arc<WhenStatement>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ((::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { rhs, lhs, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Expression::toString(lhs.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*Expression::toString(rhs.clone())?); ArcStr::from(__mm_s) }
        },
        Deref @ REINIT { value, stateVar, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("reinit(")); __mm_s.push_str(&*ComponentRef::toString(stateVar.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(value.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ ASSERT { level, message, condition, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("assert(")); __mm_s.push_str(&*Expression::toString(condition.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(message.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(level.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ TERMINATE { message, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("terminate(")); __mm_s.push_str(&*Expression::toString(message.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ NORETCALL { exp: value, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Expression::toString(value.clone())?); ArcStr::from(__mm_s) }
        },
        _ => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("NBEquation.WhenStatement.toString")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn isEqualTpl(mut tpl: (Arc<WhenStatement>, Arc<WhenStatement>)) -> Result<bool> {
        let mut b: bool = false;
        let mut stmt1: Arc<WhenStatement>;
        let mut stmt2: Arc<WhenStatement>;
        (stmt1, stmt2) = tpl.clone();
        b = isEqual(stmt1.clone(), stmt2.clone())?;
        Ok(b)
    }

    pub fn isEqual(mut stmt1: Arc<WhenStatement>, mut stmt2: Arc<WhenStatement>) -> Result<bool> {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &((stmt1.clone(), stmt2.clone())) {
        (Deref @ ASSIGN { .. }, Deref @ ASSIGN { .. }) => Expression::isEqual(var_field!((*stmt1).lhs, WhenStatement::ASSIGN).clone(), var_field!((*stmt2).lhs, WhenStatement::ASSIGN).clone())? && Expression::isEqual(var_field!((*stmt1).rhs, WhenStatement::ASSIGN).clone(), var_field!((*stmt2).rhs, WhenStatement::ASSIGN).clone())?,
        (Deref @ REINIT { .. }, Deref @ REINIT { .. }) => ComponentRef::isEqual(var_field!((*stmt1).stateVar, WhenStatement::REINIT).clone(), var_field!((*stmt2).stateVar, WhenStatement::REINIT).clone())? && Expression::isEqual(var_field!((*stmt1).value, WhenStatement::REINIT).clone(), var_field!((*stmt2).value, WhenStatement::REINIT).clone())?,
        (Deref @ ASSERT { .. }, Deref @ ASSERT { .. }) => Expression::isEqual(var_field!((*stmt1).condition, WhenStatement::ASSERT).clone(), var_field!((*stmt2).condition, WhenStatement::ASSERT).clone())? && Expression::isEqual(var_field!((*stmt1).message, WhenStatement::ASSERT).clone(), var_field!((*stmt2).message, WhenStatement::ASSERT).clone())? && Expression::isEqual(var_field!((*stmt1).level, WhenStatement::ASSERT).clone(), var_field!((*stmt2).level, WhenStatement::ASSERT).clone())?,
        (Deref @ TERMINATE { .. }, Deref @ TERMINATE { .. }) => Expression::isEqual(var_field!((*stmt1).message, WhenStatement::TERMINATE).clone(), var_field!((*stmt2).message, WhenStatement::TERMINATE).clone())?,
        (Deref @ NORETCALL { .. }, Deref @ NORETCALL { .. }) => Expression::isEqual(var_field!((*stmt1).exp, WhenStatement::NORETCALL).clone(), var_field!((*stmt2).exp, WhenStatement::NORETCALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(b)
    }

    pub fn toStatement(mut wstmt: Arc<WhenStatement>) -> Result<Arc<Statement::NFStatement>> {
        let mut stmt: Arc<Statement::NFStatement>;
        stmt = (::match_deref::match_deref! { match &(wstmt.clone()) {
        Deref @ ASSIGN { .. } => Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: var_field!((*wstmt).lhs, WhenStatement::ASSIGN).clone(), rhs: var_field!((*wstmt).rhs, WhenStatement::ASSIGN).clone(), ty: Expression::typeOf(var_field!((*wstmt).lhs, WhenStatement::ASSIGN).clone()), source: var_field!((*wstmt).source, WhenStatement::ASSIGN).clone() }),
        Deref @ REINIT { .. } => Arc::new(Statement::NFStatement::REINIT { cref: Expression::fromCref(var_field!((*wstmt).stateVar, WhenStatement::REINIT).clone(), false)?, reinitExp: var_field!((*wstmt).value, WhenStatement::REINIT).clone(), source: var_field!((*wstmt).source, WhenStatement::REINIT).clone() }),
        Deref @ ASSERT { .. } => Arc::new(Statement::NFStatement::ASSERT { condition: var_field!((*wstmt).condition, WhenStatement::ASSERT).clone(), message: var_field!((*wstmt).message, WhenStatement::ASSERT).clone(), level: var_field!((*wstmt).level, WhenStatement::ASSERT).clone(), source: var_field!((*wstmt).source, WhenStatement::ASSERT).clone() }),
        Deref @ TERMINATE { .. } => Arc::new(Statement::NFStatement::TERMINATE { message: var_field!((*wstmt).message, WhenStatement::TERMINATE).clone(), source: var_field!((*wstmt).source, WhenStatement::TERMINATE).clone() }),
        Deref @ NORETCALL { .. } => Arc::new(Statement::NFStatement::NORETCALL { exp: var_field!((*wstmt).exp, WhenStatement::NORETCALL).clone(), source: var_field!((*wstmt).source, WhenStatement::NORETCALL).clone() }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenStatement.toStatement")); __mm_s.push_str(&*literal!(" failed because of unrecognized statement: ")); __mm_s.push_str(&*toString(wstmt.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(stmt)
    }

    pub fn toEquation(mut stmt: Arc<WhenStatement>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut init: bool) -> Result<Arc<Equation::Equation>> {
        let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        eqn = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { .. } => Equation::makeAssignmentEqn(var_field!((*stmt).lhs, WhenStatement::ASSIGN).clone(), var_field!((*stmt).rhs, WhenStatement::ASSIGN).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), attr.clone())?,
        _ => Equation::setAttributes(Pointer::access(Equation::makeAlgorithm(list![toStatement(stmt.clone())?], init.clone())?), attr.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqn)
    }

    pub fn size(mut stmt: Arc<WhenStatement>, mut resize: bool) -> Result<i32> {
        let mut s: i32 = 0;
        s = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { .. } => Type::sizeOf(Expression::typeOf(var_field!((*stmt).lhs, WhenStatement::ASSIGN).clone()), resize.clone())?,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(s)
    }

    pub fn isAssign(mut stmt: Arc<WhenStatement>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isAssignOrReinit(mut stmt: Arc<WhenStatement>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { .. } => true,
        Deref @ REINIT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn getType(mut stmt: Arc<WhenStatement>) -> Arc<Type::NFType> {
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        ty = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { .. } => Expression::typeOf(var_field!((*stmt).lhs, WhenStatement::ASSIGN).clone()),
        _ => Arc::new(openmodelica_nf_frontend::NFType::ANY),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ty
    }

    pub fn map(mut stmt: Arc<WhenStatement>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<WhenStatement>> {
        let mut stmt: Arc<WhenStatement> = stmt;
        stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { .. } => {
            let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            lhs = mapFunc(var_field!((*stmt).lhs, WhenStatement::ASSIGN).clone(), funcExp.clone())?;
            rhs = mapFunc(var_field!((*stmt).rhs, WhenStatement::ASSIGN).clone(), funcExp.clone())?;
            if !(referenceEq(&lhs.clone(),&var_field!((*stmt).lhs, WhenStatement::ASSIGN).clone())) {
                assign_variant_field!(stmt => WhenStatement::ASSIGN; lhs = lhs.clone());
            }
            if !(referenceEq(&rhs.clone(),&var_field!((*stmt).rhs, WhenStatement::ASSIGN).clone())) {
                assign_variant_field!(stmt => WhenStatement::ASSIGN; rhs = rhs.clone());
            }
            stmt.clone()
        },
        Deref @ REINIT { .. } => {
            let mut funcCref: MapFuncCref;
            let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut stateVar: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            if isSome(funcCrefOpt.clone()) {
                let Some(__pa0) = (funcCrefOpt.clone()) else { bail!("pattern mismatch") };
                funcCref = __pa0.clone();
                stateVar = funcCref(var_field!((*stmt).stateVar, WhenStatement::REINIT).clone())?;
                if !(referenceEq(&stateVar.clone(),&var_field!((*stmt).stateVar, WhenStatement::REINIT).clone())) {
                    assign_variant_field!(stmt => WhenStatement::REINIT; stateVar = stateVar.clone());
                }
            }
            value = mapFunc(var_field!((*stmt).value, WhenStatement::REINIT).clone(), funcExp.clone())?;
            if !(referenceEq(&value.clone(),&var_field!((*stmt).value, WhenStatement::REINIT).clone())) {
                assign_variant_field!(stmt => WhenStatement::REINIT; value = value.clone());
            }
            stmt.clone()
        },
        Deref @ ASSERT { .. } => {
            let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut message: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            condition = mapFunc(var_field!((*stmt).condition, WhenStatement::ASSERT).clone(), funcExp.clone())?;
            if !(referenceEq(&condition.clone(),&var_field!((*stmt).condition, WhenStatement::ASSERT).clone())) {
                assign_variant_field!(stmt => WhenStatement::ASSERT; condition = condition.clone());
            }
            message = mapFunc(var_field!((*stmt).message, WhenStatement::ASSERT).clone(), funcExp.clone())?;
            if !(referenceEq(&message.clone(),&var_field!((*stmt).message, WhenStatement::ASSERT).clone())) {
                assign_variant_field!(stmt => WhenStatement::ASSERT; message = message.clone());
            }
            stmt.clone()
        },
        Deref @ TERMINATE { .. } => {
            stmt.clone()
        },
        Deref @ NORETCALL { .. } => {
            let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            value = mapFunc(var_field!((*stmt).exp, WhenStatement::NORETCALL).clone(), funcExp.clone())?;
            if !(referenceEq(&value.clone(),&var_field!((*stmt).exp, WhenStatement::NORETCALL).clone())) {
                assign_variant_field!(stmt => WhenStatement::NORETCALL; exp = value.clone());
            }
            stmt.clone()
        },
        _ => {
            stmt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(stmt)
    }

    pub fn convert(mut stmt: Arc<WhenStatement>) -> Result<OldBackendDAE::WhenOperator> {
        let mut oldStmt: OldBackendDAE::WhenOperator;
        oldStmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ ASSIGN { .. } => OldBackendDAE::WhenOperator::ASSIGN { source: var_field!((*stmt).source, WhenStatement::ASSIGN).clone(), right: Expression::toDAE(var_field!((*stmt).rhs, WhenStatement::ASSIGN).clone(), false)?, left: Expression::toDAE(var_field!((*stmt).lhs, WhenStatement::ASSIGN).clone(), false)? },
        Deref @ REINIT { .. } => OldBackendDAE::WhenOperator::REINIT { source: var_field!((*stmt).source, WhenStatement::REINIT).clone(), value: Expression::toDAE(var_field!((*stmt).value, WhenStatement::REINIT).clone(), false)?, stateVar: ComponentRef::toDAE(var_field!((*stmt).stateVar, WhenStatement::REINIT).clone())? },
        Deref @ ASSERT { .. } => OldBackendDAE::WhenOperator::ASSERT { source: var_field!((*stmt).source, WhenStatement::ASSERT).clone(), level: Expression::toDAE(var_field!((*stmt).level, WhenStatement::ASSERT).clone(), false)?, message: Expression::toDAE(var_field!((*stmt).message, WhenStatement::ASSERT).clone(), false)?, condition: Expression::toDAE(var_field!((*stmt).condition, WhenStatement::ASSERT).clone(), false)? },
        Deref @ TERMINATE { .. } => OldBackendDAE::WhenOperator::TERMINATE { source: var_field!((*stmt).source, WhenStatement::TERMINATE).clone(), message: Expression::toDAE(var_field!((*stmt).message, WhenStatement::TERMINATE).clone(), false)? },
        Deref @ NORETCALL { .. } => OldBackendDAE::WhenOperator::NORETCALL { source: var_field!((*stmt).source, WhenStatement::NORETCALL).clone(), exp: Expression::toDAE(var_field!((*stmt).exp, WhenStatement::NORETCALL).clone(), false)? },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenStatement.convert")); __mm_s.push_str(&*literal!(" failed because of unrecognized statement: ")); __mm_s.push_str(&*toString(stmt.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldStmt)
    }

}

pub mod EquationAttributes {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EquationAttributes {
        /// if the equation has been differentiated w.r.t time already
        pub derivative: Option<Pointer::Pointer<Arc<Equation::Equation>>>,
        /// also used to represent the equation itself
        pub residualVar: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        /// only set if clocked eq
        pub clock_idx: Option<i32>,
        /// true if in residual form
        pub residual: bool,
        /// true if in initial equation block
        pub exclusively_initial: bool,
        /// evaluation stages (prior used for DAE mode, still necessary?)
        pub evalStages: Arc<Evaluation::Stages::Stages>,
        /// continuous, clocked, discrete, empty
        pub kind: EquationKind,
        /// dynamic optimization component: Mayer, Lagrange, Path, Boundary
        pub optimizerExpression: Option<OptimizerExpression>,
    }

    pub type EQUATION_ATTRIBUTES = EquationAttributes;

    pub fn toString(mut attr: Arc<EquationAttributes>, mut indent: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(attr.clone()) {
        Deref @ EquationAttributes { residualVar: Some(residualVar), .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(BVariable::getVarName(residualVar.clone()))?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn setKind(mut attr: Arc<EquationAttributes>, mut kind: EquationKind, mut clock_idx: Option<i32>) -> Arc<EquationAttributes> {
        let mut attr: Arc<EquationAttributes> = attr;
        assign_field!(
            attr.kind = kind.clone(),
            attr.clock_idx = clock_idx.clone()
        );
        attr
    }

    pub fn setResidualVar(mut attr: Arc<EquationAttributes>, mut residualVar: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<EquationAttributes> {
        let mut attr: Arc<EquationAttributes> = attr;
        assign_field!(attr.residualVar = Some(residualVar.clone()));
        attr
    }

    pub fn getResidualVar(mut attr: Arc<EquationAttributes>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
        let mut residualVar: Pointer::Pointer<Arc<Variable::NFVariable>>;
        match '__try0: {
            let Some(__pa1) = (attr.residualVar.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            residualVar = __pa1.clone();
            Ok::<_, anyhow::Error>((residualVar.clone(),))
        } {
            Ok((__try0_o0,)) => {
                residualVar = __try0_o0;
            }
            Err(_) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EquationAttributes.getResidualVar")); __mm_s.push_str(&*literal!(" failed because of missing residualVar!")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("try/else: outputs not set in else branch");
            }
        }
        Ok(residualVar)
    }

    pub fn convert(mut attributes: Arc<EquationAttributes>) -> Result<OldBackendDAE::EquationAttributes> {
        let mut oldAttributes: OldBackendDAE::EquationAttributes;
        oldAttributes = OldBackendDAE::EquationAttributes { evalStages: Evaluation::Stages::convert(attributes.evalStages.clone()), kind: convertEquationKind(attributes.kind.clone(), attributes.clock_idx.clone(), attributes.exclusively_initial.clone())?, differentiated: Util::isSome(attributes.derivative.clone()) };
        Ok(oldAttributes)
    }

}

pub fn default(mut kind: EquationKind, mut exclusively_initial: bool, mut clock_idx: Option<i32>, mut optimizerExpression: Option<OptimizerExpression>) -> Arc<EquationAttributes::EquationAttributes> {
    let mut attr: Arc<EquationAttributes::EquationAttributes>;
    attr = Arc::new(EquationAttributes::EquationAttributes { optimizerExpression: optimizerExpression.clone(), kind: kind.clone(), evalStages: Evaluation::DEFAULT_STAGES.clone(), exclusively_initial: exclusively_initial.clone(), residual: false, clock_idx: clock_idx.clone(), residualVar: None, derivative: None });
    attr
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum EquationKind {
    CONTINUOUS = 1,
    DISCRETE = 2,
    CLOCKED = 3,
    EMPTY = 4,
    UNKNOWN = 5,
}
impl PartialOrd for EquationKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for EquationKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for EquationKind {
    fn default() -> Self { Self::CONTINUOUS }
}

pub fn convertEquationKind(mut eqKind: EquationKind, mut clock_idx: Option<i32>, mut exclusively_initial: bool) -> Result<OldBackendDAE::EquationKind> {
    let mut oldEqKind: OldBackendDAE::EquationKind = OldBackendDAE::EquationKind::AUX_EQUATION;
    oldEqKind = (match (eqKind.clone(), clock_idx.clone()) {
        (_, _) if (exclusively_initial.clone()) => {
            crate::BackendDAE::EquationKind::INITIAL_EQUATION
        },
        (EquationKind::CONTINUOUS { .. }, None) => {
            crate::BackendDAE::EquationKind::DYNAMIC_EQUATION
        },
        (EquationKind::CLOCKED { .. }, Some(mut clk)) => {
            OldBackendDAE::EquationKind::CLOCKED_EQUATION { clk: clk.clone() }
        },
        (EquationKind::DISCRETE, None) => {
            crate::BackendDAE::EquationKind::DISCRETE_EQUATION
        },
        (EquationKind::EMPTY, None) => {
            crate::BackendDAE::EquationKind::AUX_EQUATION
        },
        (EquationKind::UNKNOWN { .. }, None) => {
            crate::BackendDAE::EquationKind::UNKNOWN_EQUATION_KIND
        },
        (_, Some(_)) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.convertEquationKind")); __mm_s.push_str(&*literal!(" failed because the non-clock equation kind ")); __mm_s.push_str(&*equationKindString(eqKind.clone(), clock_idx.clone(), exclusively_initial.clone())?); __mm_s.push_str(&*literal!(" has a clock index.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        (EquationKind::CLOCKED { .. }, None) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.convertEquationKind")); __mm_s.push_str(&*literal!(" failed because no clock index was provided for clocked equation.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.convertEquationKind")); __mm_s.push_str(&*literal!(" for an unknown reason.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
    Ok(oldEqKind)
}

pub fn equationKindString(mut eqKind: EquationKind, mut clock_idx: Option<i32>, mut exclusively_initial: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match eqKind.clone() {
        EquationKind::CONTINUOUS { .. } => literal!("[CONT"),
        EquationKind::CLOCKED { .. } => literal!("[CLCK"),
        EquationKind::DISCRETE => literal!("[DISC"),
        EquationKind::EMPTY => literal!("[EMTY"),
        _ => literal!("[UKWN"),
    })).clone();
    r#str = (if (exclusively_initial.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[INI]")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAE]")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }}).clone();
    if Util::isSome(clock_idx.clone()) {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(Util::getOption(clock_idx.clone())?)); __mm_s.push_str(&*literal!(")]")); ArcStr::from(__mm_s) }).clone();
    } else {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub mod EquationPointers {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EquationPointers {
        /// Map for cref->index
        pub map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>,
        pub eqArr: Arc<ExpandableArray::ExpandableArray<Pointer::Pointer<Arc<Equation::Equation>>>>,
    }

    impl Default for EquationPointers {
        fn default() -> Self {
            Self {
                map: Default::default(),
                eqArr: Default::default(),
            }
        }
    }

    pub type EQUATION_POINTERS = EquationPointers;

    pub fn toString(mut equations: Arc<EquationPointers>, mut r#str: ArcStr, mut mapping_opt: Option<metamodelica::Array<(i32, i32)>>, mut printEmpty: bool, mut filter_opt: Option<Arc<UnorderedSet::UnorderedSet<ArcStr>>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut luI: i32 = lastUsedIndex(equations.clone());
        let mut length: i32 = 0;
        let mut scal_start: i32 = 0;
        let mut current_index: i32 = 1;
        let mut index: ArcStr = arcstr::literal!("");
        let mut useMapping: bool = Util::isSome(mapping_opt.clone());
        let mut filterEqs: bool = Util::isSome(filter_opt.clone());
        let mut mapping: metamodelica::Array<(i32, i32)>;
        let mut filter: Arc<UnorderedSet::UnorderedSet<ArcStr>>;
        let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
        if useMapping.clone() {
            length = 15;
            mapping = Util::getOption(mapping_opt.clone())?;
        } else {
            length = 10;
        }
        if filterEqs.clone() {
            filter = Util::getOption(filter_opt.clone())?;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Filtered ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
        }
        if printEmpty.clone() || luI.clone() > 0 {
            r#str = (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" Equations (")); __mm_s.push_str(&*intString(size(equations.clone()))); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(scalarSize(equations.clone(), true)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            for mut i in 1..=luI.clone() {
                if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                    eqn = ExpandableArray::get(i.clone(), equations.eqArr.clone())?;
                    if !(filterEqs.clone()) || UnorderedSet::contains((ComponentRef::toString(Equation::getEqnName(eqn.clone())?)?).clone(), filter.clone())? {
                        if useMapping.clone() {
                            (scal_start, _) = mapping.borrow()[(current_index.clone()-1) as usize].clone();
                            index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(current_index.clone())); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*intString(scal_start.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                        } else {
                            index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(current_index.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                        }
                        index = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*index.clone()); __mm_s.push_str(&*StringUtil::repeat((literal!(" ")).clone(), length.clone() - ((index.clone()).clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*Equation::toString(Pointer::access(eqn.clone()), (index.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                    current_index = current_index.clone() + 1;
                }
            }
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = (literal!("")).clone();
        }
        Ok(r#str)
    }

    pub fn empty(mut size: i32) -> Arc<EquationPointers> {
        let mut equationPointers: Arc<EquationPointers> = Arc::new(<EquationPointers as ::std::default::Default>::default());
        let mut arr_size: i32 = 0;
        let mut bucketSize: i32 = 0;
        arr_size = std::cmp::max(size.clone(), BaseHashTable::lowBucketSize.clone());
        bucketSize = Util::nextPrime(arr_size.clone());
        equationPointers = Arc::new(EquationPointers { map: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), bucketSize.clone()), eqArr: ExpandableArray::new(arr_size.clone(), Pointer::create(Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION))) });
        equationPointers
    }

    pub fn clone(mut equations: Arc<EquationPointers>, mut shallow: bool) -> Result<Arc<EquationPointers>> {
        let mut new: Arc<EquationPointers> = Arc::new(<EquationPointers as ::std::default::Default>::default());
        if shallow.clone() {
            new = fromList(toList(equations.clone())?);
        } else {
            new = fromList({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (toList(equations.clone())?).into_iter().cloned() {
            let __x = Pointer::create(Pointer::access(eqn.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        }
        Ok(new)
    }

    pub fn size(mut equations: Arc<EquationPointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getNumberOfElements(equations.eqArr.clone());
        sz
    }

    pub fn scalarSize(mut equations: Arc<EquationPointers>, mut resize: bool) -> Result<i32> {
        let mut sz: i32 = 0;
        for mut eqn_ptr in &*toList(equations.clone())? {
            let mut eqn_ptr = eqn_ptr.clone();
            sz = sz.clone() + Equation::size(eqn_ptr.clone(), resize.clone())?;
        }
        Ok(sz)
    }

    pub fn lastUsedIndex(mut equations: Arc<EquationPointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getLastUsedIndex(equations.eqArr.clone());
        sz
    }

    pub fn toList(mut equations: Arc<EquationPointers>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
        let mut eqn_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        eqn_lst = ExpandableArray::toList(equations.eqArr.clone())?;
        Ok(eqn_lst)
    }

    pub fn fromList(mut eq_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Arc<EquationPointers> {
        let mut equations: Arc<EquationPointers> = Arc::new(<EquationPointers as ::std::default::Default>::default());
        equations = empty((eq_lst.clone().len() as i32));
        equations = addList(eq_lst.clone(), equations.clone());
        equations
    }

    pub fn addList(mut eq_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut equations: Arc<EquationPointers>) -> Arc<EquationPointers> {
        let mut equations: Arc<EquationPointers> = equations;
        equations = List::fold(eq_lst.clone(), Arc::new(move |__pe_a0, __pe_a1| add(__pe_a0, __pe_a1)), equations.clone());
        equations
    }

    pub fn removeList(mut eq_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut equations: Arc<EquationPointers>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        equations = List::fold(eq_lst.clone(), Arc::new(move |__pe_a0, __pe_a1| remove(__pe_a0, __pe_a1)), equations.clone());
        equations = compress(equations.clone())?;
        Ok(equations)
    }

    pub fn removeCheck(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        eqns = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (toList(equations.clone())?).into_iter().cloned() {
            if !(!(func(eqn.clone())?)) { continue; }
            let __x = eqn.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        equations = fromList(eqns.clone());
        Ok(equations)
    }

    pub fn add(mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut equations: Arc<EquationPointers>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut index: i32 = 0;
        name = Equation::getEqnName(eqn.clone())?;
        let () = (match UnorderedMap::get(name.clone(), equations.map.clone()) {
        Some(mut index) if (index.clone() > 0) => {
            ExpandableArray::update(index.clone(), eqn.clone(), equations.eqArr.clone())?;
            ()
        },
        _ => {
            (_, index) = ExpandableArray::add(eqn.clone(), equations.eqArr.clone())?;
            UnorderedMap::add(name.clone(), index.clone(), equations.map.clone())?;
            ()
        },
    });
        Ok(equations)
    }

    pub fn remove(mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut equations: Arc<EquationPointers>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut index: i32 = 0;
        name = Equation::getEqnName(eqn.clone())?;
        let () = (match UnorderedMap::get(name.clone(), equations.map.clone()) {
        Some(mut index) if (index.clone() > 0) => {
            ExpandableArray::delete(index.clone(), equations.eqArr.clone())?;
            UnorderedMap::add(name.clone(), -1, equations.map.clone())?;
            ()
        },
        _ => (),
    });
        Ok(equations)
    }

    pub fn map(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut eq_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut new_eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut followEquations: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::DEBUG_FOLLOW_EQUATIONS.clone())?;
        let mut debug: bool = !(followEquations.clone().is_empty());
        let mut debug_eqns: Arc<UnorderedSet::UnorderedSet<ArcStr>>;
        if debug.clone() {
            debug_eqns = UnorderedSet::fromList(followEquations.clone(), (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
        }
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i.clone(), equations.eqArr.clone())?;
                eq = Pointer::access(eq_ptr.clone());
                new_eq = func(eq.clone())?;
                if !(referenceEq(&eq.clone(),&new_eq.clone())) {
                    if debug.clone() && (UnorderedSet::contains((ComponentRef::toString(Equation::getEqnName(eq_ptr.clone())?)?).clone(), debug_eqns.clone())? || UnorderedSet::contains((ComponentRef::toString(Equation::getEqnName(Pointer::create(new_eq.clone()))?)?).clone(), debug_eqns.clone())?) && !(Equation::equalName(Pointer::create(eq.clone()), Pointer::create(new_eq.clone()))) {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[debugFollowEquations] The equation:\n")); __mm_s.push_str(&*Equation::toString(eq.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\nGets replaced by:\n")); __mm_s.push_str(&*Equation::toString(new_eq.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Pointer::update(eq_ptr.clone(), new_eq.clone());
                }
            }
        }
        Ok(equations)
    }

    pub fn mapPtr(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> + 'static>) -> Result<()> {
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                func(ExpandableArray::get(i.clone(), equations.eqArr.clone())?)?;
            }
        }
        Ok(())
    }

    pub fn mapExp(mut equations: Arc<EquationPointers>, mut funcExp: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut funcCrefOpt: Option<MapFuncCref>, mut mapFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, MapFuncExp) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut eq_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut new_eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i.clone(), equations.eqArr.clone())?;
                eq = Pointer::access(eq_ptr.clone());
                new_eq = Equation::map(eq.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
                if !(referenceEq(&eq.clone(),&new_eq.clone())) {
                    Pointer::update(eq_ptr.clone(), new_eq.clone());
                }
            }
        }
        Ok(equations)
    }

    pub fn mapRemovePtr(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut eq_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i.clone(), equations.eqArr.clone())?;
                if func(eq_ptr.clone())? {
                    equations = remove(eq_ptr.clone(), equations.clone())?;
                }
            }
        }
        equations = compress(equations.clone())?;
        Ok(equations)
    }

    pub fn mapRes(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>) -> Result<()> {
        pub type mapFunc = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>;

        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                func(Equation::getResidualVar(ExpandableArray::get(i.clone(), equations.eqArr.clone())?)?)?;
            }
        }
        Ok(())
    }

    pub fn fold<T: Clone + 'static>(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>, T) -> Result<T> + 'static>, mut extArg: T) -> Result<T> {
        pub type MapFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>, T) -> Result<T> + 'static>;

        let mut extArg: T = extArg;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                extArg = func(Pointer::access(ExpandableArray::get(i.clone(), equations.eqArr.clone())?), extArg.clone())?;
            }
        }
        Ok(extArg)
    }

    pub fn foldPtr<T: Clone + 'static>(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, T) -> Result<T> + 'static>, mut extArg: T) -> Result<T> {
        pub type MapFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, T) -> Result<T> + 'static>;

        let mut extArg: T = extArg;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                extArg = func(ExpandableArray::get(i.clone(), equations.eqArr.clone())?, extArg.clone())?;
            }
        }
        Ok(extArg)
    }

    pub fn foldRemovePtr<T: Clone + 'static>(mut equations: Arc<EquationPointers>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, T) -> Result<(T, bool)> + 'static>, mut extArg: T) -> Result<(Arc<EquationPointers>, T)> {
        pub type MapFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, T) -> Result<(T, bool)> + 'static>;

        let mut equations: Arc<EquationPointers> = equations;
        let mut extArg: T = extArg;
        let mut eq_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut delete: bool = false;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i.clone(), equations.eqArr.clone())?;
                (extArg, delete) = func(eq_ptr.clone(), extArg.clone())?;
                if delete.clone() {
                    Pointer::update(eq_ptr.clone(), Arc::new(crate::NBEquation::Equation::DUMMY_EQUATION));
                    assign_field!(equations.eqArr = ExpandableArray::delete(i.clone(), equations.eqArr.clone())?);
                }
            }
        }
        Ok((equations, extArg))
    }

    pub fn getEqnAt(mut equations: Arc<EquationPointers>, mut index: i32) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
        let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
        eqn = ExpandableArray::get(index.clone(), equations.eqArr.clone())?;
        Ok(eqn)
    }

    pub fn getEqnByName(mut equations: Arc<EquationPointers>, mut name: Arc<ComponentRef::NFComponentRef>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
        let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
        eqn = (match UnorderedMap::get(name.clone(), equations.map.clone()) {
        Some(mut index) if (index.clone() > 0) => {
            getEqnAt(equations.clone(), index.clone())?
        },
        Some(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EquationPointers.getEqnByName")); __mm_s.push_str(&*literal!(" failed because the equation with the name ")); __mm_s.push_str(&*ComponentRef::toString(name.clone())?); __mm_s.push_str(&*literal!(" has already been deleted.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EquationPointers.getEqnByName")); __mm_s.push_str(&*literal!(" failed because there is no equation with the name ")); __mm_s.push_str(&*ComponentRef::toString(name.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Ok(eqn)
    }

    pub fn getEqnIndex(mut equations: Arc<EquationPointers>, mut name: Arc<ComponentRef::NFComponentRef>) -> i32 {
        let mut index: i32 = UnorderedMap::getOrDefault(name.clone(), equations.map.clone(), -1);
        index
    }

    pub fn compress(mut equations: Arc<EquationPointers>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
        let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut i in (1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone())).rev() {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                eqn = ExpandableArray::get(i.clone(), equations.eqArr.clone())?;
                let () = (::match_deref::match_deref! { match &(Pointer::access(eqn.clone())) {
        Deref @ Equation::DUMMY_EQUATION => {
            ()
        },
        Deref @ Equation::FOR_EQUATION { body, .. } if (List::all(body.clone(), (std::sync::Arc::new(fnptr!(Equation::isDummy, Arc<Equation::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<bool> + 'static>))) => {
            ()
        },
        _ => {
            eqns = cons(eqn.clone(), eqns.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
        }
        equations = fromList(eqns.clone());
        Ok(equations)
    }

    pub fn sort(mut equations: Arc<EquationPointers>) -> Result<Arc<EquationPointers>> {
        let mut equations: Arc<EquationPointers> = equations;
        let mut size: i32 = 0;
        let mut hash_lst: Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Equation::Equation>>)>> = metamodelica::nil();
        let mut hash_lst_ptr: Pointer::Pointer<Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Equation::Equation>>)>>> = Pointer::create(metamodelica::nil());
        let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
        size = ExpandableArray::getNumberOfElements(equations.eqArr.clone());
        mapPtr(equations.clone(), Arc::new({ let __pe_b1 = ((metamodelica::OrderedFloat((size.clone()) as f64) * (size.clone()).ln()).0 as i32); let __pe_b2 = hash_lst_ptr.clone(); move |__pe_a0| createSortHashTpl(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
        hash_lst = List::sort(Pointer::access(hash_lst_ptr.clone()), std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)))?;
        equations = empty(size.clone());
        for mut tpl in &*hash_lst.clone() {
            let mut tpl = tpl.clone();
            (_, eqn_ptr) = tpl.clone();
            assign_field!(equations.eqArr = ExpandableArray::add(eqn_ptr.clone(), equations.eqArr.clone())?.0);
        }
        Ok(equations)
    }

    pub fn getResiduals(mut equations: Arc<EquationPointers>) -> Result<Arc<VariablePointers::VariablePointers>> {
        let mut residuals: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
        residuals = BVariable::VariablePointers::fromList({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut eqn in (toList(equations.clone())?).into_iter().cloned() {
            let __x = Equation::getResidualVar(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, false);
        Ok(residuals)
    }

    fn createSortHashTpl(mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>, mut r#mod: i32, mut hash_lst_ptr: Pointer::Pointer<Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Equation::Equation>>)>>>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
        let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>> = eqn_ptr;
        let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut hash: i32 = 0;
        eqn = Pointer::access(eqn_ptr.clone());
        hash = BackendUtil::noNameHashEq(eqn.clone(), r#mod.clone())?;
        Pointer::update(hash_lst_ptr.clone(), cons((hash.clone(), eqn_ptr.clone()), Pointer::access(hash_lst_ptr.clone())));
        Ok(eqn_ptr)
    }

}

pub mod EqData {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum EqData {
        EQ_DATA_SIM {
            /// current index to be used for new identifier
            uniqueIndex: Pointer::Pointer<i32>,
            /// All equations
            equations: Arc<EquationPointers::EquationPointers>,
            /// All equations for simulation (without initial)
            simulation: Arc<EquationPointers::EquationPointers>,
            /// Continuous equations
            continuous: Arc<EquationPointers::EquationPointers>,
            /// Clocked equations
            clocked: Arc<EquationPointers::EquationPointers>,
            /// Discrete equations
            discretes: Arc<EquationPointers::EquationPointers>,
            /// (Exclusively) Initial equations
            initials: Arc<EquationPointers::EquationPointers>,
            /// Auxiliary equations
            auxiliaries: Arc<EquationPointers::EquationPointers>,
            /// Removed equations (alias and no return value)
            removed: Arc<EquationPointers::EquationPointers>,
        },
        EQ_DATA_JAC {
            /// current index to be used for new identifier
            uniqueIndex: Pointer::Pointer<i32>,
            /// All equations
            equations: Arc<EquationPointers::EquationPointers>,
            /// Result equations
            results: Arc<EquationPointers::EquationPointers>,
            /// Temporary inner equations
            temporary: Arc<EquationPointers::EquationPointers>,
            /// Auxiliary equations
            auxiliaries: Arc<EquationPointers::EquationPointers>,
            /// Removed equations (alias and no return value)
            removed: Arc<EquationPointers::EquationPointers>,
        },
        EQ_DATA_HES {
            /// current index to be used for new identifier
            uniqueIndex: Pointer::Pointer<i32>,
            /// All equations
            equations: Arc<EquationPointers::EquationPointers>,
            /// Result equation
            result: Pointer::Pointer<Arc<Equation::Equation>>,
            /// Temporary inner equations
            temporary: Arc<EquationPointers::EquationPointers>,
            /// Auxiliary equations
            auxiliaries: Arc<EquationPointers::EquationPointers>,
            /// Removed equations (alias and no return value)
            removed: Arc<EquationPointers::EquationPointers>,
        },
        EQ_DATA_EMPTY,
    }
    pub use self::EqData::{EQ_DATA_SIM,EQ_DATA_JAC,EQ_DATA_HES,EQ_DATA_EMPTY};
    pub fn size(mut eqData: Arc<EqData>) -> Result<i32> {
        let mut s: i32 = 0;
        s = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => EquationPointers::size(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone()),
        Deref @ EQ_DATA_JAC { .. } => EquationPointers::size(var_field!((*eqData).equations, EqData::EQ_DATA_JAC).clone()),
        Deref @ EQ_DATA_HES { .. } => EquationPointers::size(var_field!((*eqData).equations, EqData::EQ_DATA_HES).clone()),
        _ => bail!("match: no arm matched"),
    } });
        Ok(s)
    }

    pub fn scalarSize(mut eqData: Arc<EqData>, mut resize: bool) -> Result<i32> {
        let mut s: i32 = 0;
        s = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => EquationPointers::scalarSize(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), resize.clone())?,
        Deref @ EQ_DATA_JAC { .. } => EquationPointers::scalarSize(var_field!((*eqData).equations, EqData::EQ_DATA_JAC).clone(), resize.clone())?,
        Deref @ EQ_DATA_HES { .. } => EquationPointers::scalarSize(var_field!((*eqData).equations, EqData::EQ_DATA_HES).clone(), resize.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(s)
    }

    pub fn map(mut eqData: Arc<EqData>, mut func: Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                simulation = EquationPointers::map(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                continuous = EquationPointers::map(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                clocked = EquationPointers::map(var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                discretes = EquationPointers::map(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                initials = EquationPointers::map(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                auxiliaries = EquationPointers::map(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone(), func.clone())?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_JAC { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                results = EquationPointers::map(var_field!((*eqData).results, EqData::EQ_DATA_JAC).clone(), func.clone())?,
                temporary = EquationPointers::map(var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone(), func.clone())?,
                auxiliaries = EquationPointers::map(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone(), func.clone())?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_HES { .. } => {
            Pointer::update(var_field!((*eqData).result, EqData::EQ_DATA_HES).clone(), func(Pointer::access(var_field!((*eqData).result, EqData::EQ_DATA_HES).clone()))?);
            assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                temporary = EquationPointers::map(var_field!((*eqData).temporary, EqData::EQ_DATA_HES).clone(), func.clone())?,
                auxiliaries = EquationPointers::map(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone(), func.clone())?
            );
            eqData.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(eqData)
    }

    pub fn mapExp(mut eqData: Arc<EqData>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                simulation = EquationPointers::mapExp(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                continuous = EquationPointers::mapExp(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                clocked = EquationPointers::mapExp(var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                discretes = EquationPointers::mapExp(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                initials = EquationPointers::mapExp(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                auxiliaries = EquationPointers::mapExp(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                removed = EquationPointers::mapExp(var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_JAC { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                results = EquationPointers::mapExp(var_field!((*eqData).results, EqData::EQ_DATA_JAC).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                temporary = EquationPointers::mapExp(var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                auxiliaries = EquationPointers::mapExp(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                removed = EquationPointers::mapExp(var_field!((*eqData).removed, EqData::EQ_DATA_JAC).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_HES { .. } => {
            Pointer::update(var_field!((*eqData).result, EqData::EQ_DATA_HES).clone(), Equation::map(Pointer::access(var_field!((*eqData).result, EqData::EQ_DATA_HES).clone()), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?);
            assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                temporary = EquationPointers::mapExp(var_field!((*eqData).temporary, EqData::EQ_DATA_HES).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                auxiliaries = EquationPointers::mapExp(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                removed = EquationPointers::mapExp(var_field!((*eqData).removed, EqData::EQ_DATA_HES).clone(), func.clone(), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
            );
            eqData.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(eqData)
    }

    pub fn toString(mut eqData: Arc<EqData>, mut level: i32, mut filter_opt: Option<Arc<UnorderedSet::UnorderedSet<ArcStr>>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => {
            let mut tmp: ArcStr = arcstr::literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Equation Data Simulation (scalar simulation equations: ")); __mm_s.push_str(&*intString(EquationPointers::scalarSize(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), true)?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((tmp.clone()).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            if level.clone() == 0 {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(), (literal!("Simulation")).clone(), None, false, filter_opt.clone())?); ArcStr::from(__mm_s) }).clone();
            } else {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone(), (literal!("Continuous")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone(), (literal!("Clocked")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone(), (literal!("Discrete")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone(), (literal!("(Exclusively) Initial")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone(), (literal!("Auxiliary")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone(), (literal!("Removed")).clone(), None, false, filter_opt.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            tmp.clone()
        },
        Deref @ EQ_DATA_JAC { .. } => {
            let mut tmp: ArcStr = arcstr::literal!("");
            if level.clone() == 0 {
                tmp = (EquationPointers::toString(var_field!((*eqData).equations, EqData::EQ_DATA_JAC).clone(), (literal!("Jacobian")).clone(), None, false, filter_opt.clone())?).clone();
            } else {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).results, EqData::EQ_DATA_JAC).clone(), (literal!("Residual")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone(), (literal!("Inner")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone(), (literal!("Auxiliary")).clone(), None, false, filter_opt.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            tmp.clone()
        },
        Deref @ EQ_DATA_HES { .. } => {
            let mut tmp: ArcStr = arcstr::literal!("");
            if level.clone() == 0 {
                tmp = (EquationPointers::toString(var_field!((*eqData).equations, EqData::EQ_DATA_HES).clone(), (literal!("Hessian")).clone(), None, false, filter_opt.clone())?).clone();
            } else {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_4((literal!("Result Equation")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Equation::toString(Pointer::access(var_field!((*eqData).result, EqData::EQ_DATA_HES).clone()), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).temporary, EqData::EQ_DATA_HES).clone(), (literal!("Temporary Inner")).clone(), None, false, filter_opt.clone())?); __mm_s.push_str(&*EquationPointers::toString(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone(), (literal!("Auxiliary")).clone(), None, false, filter_opt.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            tmp.clone()
        },
        Deref @ EQ_DATA_EMPTY { .. } => {
            literal!("Empty equation Data!\n")
        },
        _ => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.toString")); __mm_s.push_str(&*literal!(" failed!\n")); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn getUniqueIndex(mut eqData: Arc<EqData>) -> Result<Pointer::Pointer<i32>> {
        let mut uniqueIndex: Pointer::Pointer<i32>;
        uniqueIndex = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(),
        Deref @ EQ_DATA_JAC { .. } => var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_JAC).clone(),
        Deref @ EQ_DATA_HES { .. } => var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_HES).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.getUniqueIndex")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(uniqueIndex)
    }

    pub fn getEquations(mut eqData: Arc<EqData>) -> Result<Arc<EquationPointers::EquationPointers>> {
        let mut equations: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
        equations = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(),
        Deref @ EQ_DATA_JAC { .. } => var_field!((*eqData).equations, EqData::EQ_DATA_JAC).clone(),
        Deref @ EQ_DATA_HES { .. } => var_field!((*eqData).equations, EqData::EQ_DATA_HES).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.getEquations")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(equations)
    }

    pub fn setEquations(mut eqData: Arc<EqData>, mut equations: Arc<EquationPointers::EquationPointers>) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM; equations = equations.clone());
            eqData.clone()
        },
        Deref @ EQ_DATA_JAC { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_JAC; equations = equations.clone());
            eqData.clone()
        },
        Deref @ EQ_DATA_HES { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_HES; equations = equations.clone());
            eqData.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(eqData)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    #[repr(i32)]
    pub enum EqType {
        CONTINUOUS = 1,
        DISCRETE = 2,
        CLOCKED = 3,
        INITIAL = 4,
    }
    impl PartialOrd for EqType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl Ord for EqType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
    }

    pub fn addTypedList(mut eqData: Arc<EqData>, mut eq_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut eqType: EqType, mut newName: bool) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &((eqData.clone(), eqType.clone())) {
        (Deref @ EQ_DATA_SIM { .. }, EqType::CONTINUOUS { .. }) => {
            if newName.clone() {
                for mut eqn_ptr in &*eq_lst.clone() {
                    let mut eqn_ptr = eqn_ptr.clone();
                    Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), (arcstr::literal!(SIMULATION_STR)).clone())?;
                }
            }
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone()),
                simulation = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone()),
                continuous = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone())
            );
            eqData.clone()
        },
        (Deref @ EQ_DATA_SIM { .. }, EqType::DISCRETE) => {
            if newName.clone() {
                for mut eqn_ptr in &*eq_lst.clone() {
                    let mut eqn_ptr = eqn_ptr.clone();
                    Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), (arcstr::literal!(SIMULATION_STR)).clone())?;
                }
            }
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone()),
                simulation = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone()),
                discretes = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone())
            );
            eqData.clone()
        },
        (Deref @ EQ_DATA_SIM { .. }, EqType::CLOCKED { .. }) => {
            if newName.clone() {
                for mut eqn_ptr in &*eq_lst.clone() {
                    let mut eqn_ptr = eqn_ptr.clone();
                    Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), (arcstr::literal!(SIMULATION_STR)).clone())?;
                }
            }
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM; clocked = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone()));
            eqData.clone()
        },
        (Deref @ EQ_DATA_SIM { .. }, EqType::INITIAL) => {
            if newName.clone() {
                for mut eqn_ptr in &*eq_lst.clone() {
                    let mut eqn_ptr = eqn_ptr.clone();
                    Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), (arcstr::literal!(SIMULATION_STR)).clone())?;
                }
            }
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone()),
                initials = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone())
            );
            eqData.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.addTypedList")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqData)
    }

    pub fn addUntypedList(mut eqData: Arc<EqData>, mut eq_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut newName: bool) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        let mut equation_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut continuous_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut clocked_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut discretes_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut initials_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut auxiliaries_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut simulation_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut removed_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => {
            if newName.clone() {
                for mut eqn_ptr in &*eq_lst.clone() {
                    let mut eqn_ptr = eqn_ptr.clone();
                    Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), (arcstr::literal!(SIMULATION_STR)).clone())?;
                }
            }
            (simulation_lst, continuous_lst, clocked_lst, discretes_lst, initials_lst, auxiliaries_lst, removed_lst) = typeList(eq_lst.clone())?;
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::addList(eq_lst.clone(), var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone()),
                simulation = EquationPointers::addList(simulation_lst.clone(), var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone()),
                continuous = EquationPointers::addList(continuous_lst.clone(), var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone()),
                clocked = EquationPointers::addList(clocked_lst.clone(), var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone()),
                discretes = EquationPointers::addList(discretes_lst.clone(), var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone()),
                initials = EquationPointers::addList(initials_lst.clone(), var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone()),
                auxiliaries = EquationPointers::addList(auxiliaries_lst.clone(), var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone()),
                removed = EquationPointers::addList(removed_lst.clone(), var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone())
            );
            eqData.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.addUntypedList")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqData)
    }

    pub fn removeList(mut eq_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut eqData: Arc<EqData>) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone())?,
                simulation = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone())?,
                continuous = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone())?,
                discretes = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone())?,
                clocked = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone())?,
                initials = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone())?,
                auxiliaries = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone())?,
                removed = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone())?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_JAC { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                equations = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).equations, EqData::EQ_DATA_JAC).clone())?,
                results = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).results, EqData::EQ_DATA_JAC).clone())?,
                temporary = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone())?,
                auxiliaries = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone())?,
                removed = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).removed, EqData::EQ_DATA_JAC).clone())?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_HES { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                equations = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).equations, EqData::EQ_DATA_HES).clone())?,
                temporary = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).temporary, EqData::EQ_DATA_HES).clone())?,
                auxiliaries = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone())?,
                removed = EquationPointers::removeList(eq_lst.clone(), var_field!((*eqData).removed, EqData::EQ_DATA_HES).clone())?
            );
            eqData.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.removeList")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqData)
    }

    pub fn removeTypedCheck(mut eqData: Arc<EqData>, mut func: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>, mut eqType: EqType) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &((eqData.clone(), eqType.clone())) {
        (Deref @ EQ_DATA_SIM { .. }, EqType::CONTINUOUS { .. }) => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::removeCheck(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                simulation = EquationPointers::removeCheck(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                continuous = EquationPointers::removeCheck(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone(), func.clone())?
            );
            eqData.clone()
        },
        (Deref @ EQ_DATA_SIM { .. }, EqType::DISCRETE) => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::removeCheck(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                simulation = EquationPointers::removeCheck(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                discretes = EquationPointers::removeCheck(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone(), func.clone())?
            );
            eqData.clone()
        },
        (Deref @ EQ_DATA_SIM { .. }, EqType::CLOCKED { .. }) => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM; clocked = EquationPointers::removeCheck(var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone(), func.clone())?);
            eqData.clone()
        },
        (Deref @ EQ_DATA_SIM { .. }, EqType::INITIAL) => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::removeCheck(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(), func.clone())?,
                initials = EquationPointers::removeCheck(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone(), func.clone())?
            );
            eqData.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.removeTypedCheck")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqData)
    }

    pub fn compress(mut eqData: Arc<EqData>) -> Result<Arc<EqData>> {
        let mut eqData: Arc<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ EQ_DATA_SIM { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                equations = EquationPointers::compress(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone())?,
                simulation = EquationPointers::compress(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone())?,
                continuous = EquationPointers::compress(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone())?,
                discretes = EquationPointers::compress(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone())?,
                initials = EquationPointers::compress(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone())?,
                auxiliaries = EquationPointers::compress(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone())?,
                removed = EquationPointers::compress(var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone())?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_JAC { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                equations = EquationPointers::compress(var_field!((*eqData).equations, EqData::EQ_DATA_JAC).clone())?,
                results = EquationPointers::compress(var_field!((*eqData).results, EqData::EQ_DATA_JAC).clone())?,
                temporary = EquationPointers::compress(var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone())?,
                auxiliaries = EquationPointers::compress(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone())?,
                removed = EquationPointers::compress(var_field!((*eqData).removed, EqData::EQ_DATA_JAC).clone())?
            );
            eqData.clone()
        },
        Deref @ EQ_DATA_HES { .. } => {
            assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                equations = EquationPointers::compress(var_field!((*eqData).equations, EqData::EQ_DATA_HES).clone())?,
                temporary = EquationPointers::compress(var_field!((*eqData).temporary, EqData::EQ_DATA_HES).clone())?,
                auxiliaries = EquationPointers::compress(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone())?,
                removed = EquationPointers::compress(var_field!((*eqData).removed, EqData::EQ_DATA_HES).clone())?
            );
            eqData.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.compress")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(eqData)
    }

}

pub fn typeList(mut equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> {
    let mut simulation_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut continuous_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut clocked_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut discretes_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut initials_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut auxiliaries_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut removed_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    for mut eq in &*equations.clone() {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(Equation::getAttributes(Pointer::access(eq.clone()))) {
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { exclusively_initial: true, .. } => {
            initials_lst = cons(eq.clone(), initials_lst.clone());
            ()
        },
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { kind: EquationKind::CONTINUOUS { .. }, .. } => {
            continuous_lst = cons(eq.clone(), continuous_lst.clone());
            simulation_lst = cons(eq.clone(), simulation_lst.clone());
            ()
        },
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { kind: EquationKind::CLOCKED { .. }, .. } => {
            clocked_lst = cons(eq.clone(), clocked_lst.clone());
            ()
        },
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { kind: EquationKind::DISCRETE, .. } => {
            discretes_lst = cons(eq.clone(), discretes_lst.clone());
            simulation_lst = cons(eq.clone(), simulation_lst.clone());
            ()
        },
        Deref @ EquationAttributes::EQUATION_ATTRIBUTES { kind: EquationKind::EMPTY, .. } => {
            removed_lst = cons(eq.clone(), removed_lst.clone());
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.typeList")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Equation::toString(Pointer::access(eq.clone()), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((simulation_lst, continuous_lst, clocked_lst, discretes_lst, initials_lst, auxiliaries_lst, removed_lst))
}

