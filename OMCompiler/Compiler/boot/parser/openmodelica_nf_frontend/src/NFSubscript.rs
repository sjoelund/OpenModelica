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
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::JSON;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFSubscript {
    RAW_SUBSCRIPT {
        subscript: Arc<Absyn::Subscript>,
    },
    UNTYPED {
        exp: Arc<Expression::NFExpression>,
    },
    INDEX {
        index: Arc<Expression::NFExpression>,
    },
    SLICE {
        slice: Arc<Expression::NFExpression>,
    },
    EXPANDED_SLICE {
        indices: Arc<metamodelica::List<Arc<NFSubscript>>>,
    },
    WHOLE,
    SPLIT_PROXY {
        origin: Arc<InstNode::InstNode>,
        parent: Arc<InstNode::InstNode>,
    },
    SPLIT_INDEX {
        node: Arc<InstNode::InstNode>,
        dimIndex: i32,
    },
}
impl Default for NFSubscript {
    fn default() -> Self { Self::WHOLE }
}
pub use self::NFSubscript::{RAW_SUBSCRIPT,UNTYPED,INDEX,SLICE,EXPANDED_SLICE,WHOLE,SPLIT_PROXY,SPLIT_INDEX};
pub fn fromExp(mut exp: Arc<Expression::NFExpression>) -> Arc<NFSubscript> {
    let mut subscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    subscript = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => Arc::new(NFSubscript::INDEX { index: exp.clone() }),
        Deref @ Expression::BOOLEAN { .. } => Arc::new(NFSubscript::INDEX { index: exp.clone() }),
        Deref @ Expression::ENUM_LITERAL { .. } => Arc::new(NFSubscript::INDEX { index: exp.clone() }),
        _ => Arc::new(NFSubscript::UNTYPED { exp: exp.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subscript
}

pub fn fromTypedExp(mut exp: Arc<Expression::NFExpression>) -> Arc<NFSubscript> {
    let mut subscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    subscript = if (Type::isArray(Expression::typeOf(exp.clone()))) {Arc::new(NFSubscript::SLICE { slice: exp.clone() })} else {Arc::new(NFSubscript::INDEX { index: exp.clone() })};
    subscript
}

pub fn toExp(mut subscript: Arc<NFSubscript>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(),
        Deref @ INDEX { .. } => var_field!((*subscript).index, NFSubscript::INDEX).clone(),
        Deref @ SLICE { .. } => var_field!((*subscript).slice, NFSubscript::SLICE).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn toInteger(mut subscript: Arc<NFSubscript>) -> Result<i32> {
    let mut int: i32 = 0;
    int = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => Expression::toInteger(var_field!((*subscript).index, NFSubscript::INDEX).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(int)
}

pub fn toIntegerOpt(mut subscript: Arc<NFSubscript>) -> Result<Option<i32>> {
    let mut int: Option<i32> = None;
    int = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => Some(Expression::toInteger(var_field!((*subscript).index, NFSubscript::INDEX).clone())?),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(int)
}

pub fn toIndexList(mut subscript: Arc<NFSubscript>, mut length: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    indices = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => {
            list![toInteger(subscript.clone())?]
        },
        Deref @ WHOLE { .. } => {
            List::intRange2(1, length.clone())
        },
        Deref @ SLICE { slice: Deref @ Expression::ARRAY { elements: elems, .. } } => {
            ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (elems.clone()).borrow().iter() {
            let __x = Expression::toInteger(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ SLICE { slice: Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop }, step: Some(Deref @ Expression::INTEGER { value: step }), start: Deref @ Expression::INTEGER { value: start }, .. } } => {
            List::intRange3(start.clone(), step.clone(), stop.clone())?
        },
        Deref @ SLICE { slice: Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: stop }, step: None, start: Deref @ Expression::INTEGER { value: start }, .. } } => {
            List::intRange2(start.clone(), stop.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.toIndexList")); __mm_s.push_str(&*literal!(" got an incorrect subscript type ")); __mm_s.push_str(&*toString(subscript.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(indices)
}

fn isValidIndexType(mut ty: Arc<Type::NFType>) -> bool {
    let mut b: bool = Type::isInteger(ty.clone()) || Type::isBoolean(ty.clone()) || Type::isEnumeration(ty.clone());
    b
}

pub fn makeIndex(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<NFSubscript>> {
    let mut subscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = Expression::typeOf(exp.clone());
    if isValidIndexType(ty.clone()) {
        subscript = Arc::new(NFSubscript::INDEX { index: exp.clone() });
    } else {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.makeIndex")); __mm_s.push_str(&*literal!(" got a non integer type exp to make an index sub")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(subscript)
}

pub fn makeSplitIndex(mut node: Arc<InstNode::InstNode>, mut dimIndex: i32) -> Result<Arc<NFSubscript>> {
    let mut subscript: Arc<NFSubscript> = Arc::new(NFSubscript::SPLIT_INDEX { node: node.clone(), dimIndex: dimIndex.clone() });
    if dimIndex.clone() < 1 {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.makeSplitIndex")); __mm_s.push_str(&*literal!(" got invalid index ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", dimIndex.clone()))); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(subscript)
}

pub fn isIndex(mut sub: Arc<NFSubscript>) -> bool {
    let mut isIndex: bool = false;
    isIndex = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ INDEX { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isIndex
}

pub fn isWhole(mut sub: Arc<NFSubscript>) -> bool {
    let mut isWhole: bool = false;
    isWhole = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ WHOLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhole
}

pub fn isSimple(mut sub: Arc<NFSubscript>) -> bool {
    let mut isSimple: bool = isIndex(sub.clone()) || isWhole(sub.clone());
    isSimple
}

pub fn isSliced(mut sub: Arc<NFSubscript>) -> bool {
    let mut sliced: bool = false;
    sliced = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ SLICE { .. } => true,
        Deref @ WHOLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sliced
}

pub fn isScalar(mut sub: Arc<NFSubscript>) -> bool {
    let mut isScalar: bool = false;
    isScalar = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ INDEX { .. } => {
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ty = Expression::typeOf(var_field!((*sub).index, NFSubscript::INDEX).clone());
            isValidIndexType(ty.clone())
        },
        Deref @ SPLIT_INDEX { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isScalar
}

pub fn isScalarLiteral(mut sub: Arc<NFSubscript>) -> bool {
    let mut isScalarLiteral: bool = false;
    isScalarLiteral = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ INDEX { .. } => Expression::isScalarLiteral(var_field!((*sub).index, NFSubscript::INDEX).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isScalarLiteral
}

pub fn equalsIterator(mut sub: Arc<NFSubscript>, mut iterator: Arc<InstNode::InstNode>) -> Result<bool> {
    let mut res: bool = false;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    res = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ UNTYPED { exp: Deref @ Expression::CREF { cref, .. } } => InstNode::refEqual(iterator.clone(), ComponentRef::node(cref.clone())?),
        Deref @ INDEX { index: Deref @ Expression::CREF { cref, .. } } => InstNode::refEqual(iterator.clone(), ComponentRef::node(cref.clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isIterator(mut sub: Arc<NFSubscript>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ UNTYPED { .. } => Expression::isIterator(var_field!((*sub).exp, NFSubscript::UNTYPED).clone()),
        Deref @ INDEX { .. } => Expression::isIterator(var_field!((*sub).index, NFSubscript::INDEX).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn toIterator(mut sub: Arc<NFSubscript>) -> Result<Arc<InstNode::InstNode>> {
    let mut iterator: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    iterator = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ UNTYPED { exp: Deref @ Expression::CREF { cref, .. } } if (ComponentRef::isIterator(cref.clone())) => ComponentRef::node(cref.clone())?,
        Deref @ INDEX { index: Deref @ Expression::CREF { cref, .. } } if (ComponentRef::isIterator(cref.clone())) => ComponentRef::node(cref.clone())?,
        _ => Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub fn isBackendIterator(mut sub: Arc<NFSubscript>) -> bool {
    let mut res: bool = false;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    res = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ INDEX { index: Deref @ Expression::CREF { cref, .. } } => ComponentRef::isIterator(cref.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isEqual(mut subscript1: Arc<NFSubscript>, mut subscript2: Arc<NFSubscript>) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = (::match_deref::match_deref! { match &((subscript1.clone(), subscript2.clone())) {
        (Deref @ RAW_SUBSCRIPT { .. }, Deref @ RAW_SUBSCRIPT { .. }) => AbsynUtil::subscriptEqual(var_field!((*subscript1).subscript, NFSubscript::RAW_SUBSCRIPT).clone(), var_field!((*subscript2).subscript, NFSubscript::RAW_SUBSCRIPT).clone())?,
        (Deref @ UNTYPED { .. }, Deref @ UNTYPED { .. }) => Expression::isEqual(var_field!((*subscript1).exp, NFSubscript::UNTYPED).clone(), var_field!((*subscript2).exp, NFSubscript::UNTYPED).clone())?,
        (Deref @ INDEX { .. }, Deref @ INDEX { .. }) => Expression::isEqual(var_field!((*subscript1).index, NFSubscript::INDEX).clone(), var_field!((*subscript2).index, NFSubscript::INDEX).clone())?,
        (Deref @ SLICE { .. }, Deref @ SLICE { .. }) => Expression::isEqual(var_field!((*subscript1).slice, NFSubscript::SLICE).clone(), var_field!((*subscript2).slice, NFSubscript::SLICE).clone())?,
        (Deref @ WHOLE { .. }, Deref @ WHOLE { .. }) => true,
        (Deref @ SPLIT_INDEX { .. }, Deref @ SPLIT_INDEX { .. }) => var_field!((*subscript1).dimIndex, NFSubscript::SPLIT_INDEX).clone() == var_field!((*subscript2).dimIndex, NFSubscript::SPLIT_INDEX).clone() && InstNode::refEqual(var_field!((*subscript1).node, NFSubscript::SPLIT_INDEX).clone(), var_field!((*subscript2).node, NFSubscript::SPLIT_INDEX).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub fn isEqualList(mut subscripts1: Arc<metamodelica::List<Arc<NFSubscript>>>, mut subscripts2: Arc<metamodelica::List<Arc<NFSubscript>>>) -> Result<bool> {
    let mut isEqual: bool = false;
    let mut s2: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut rest: Arc<metamodelica::List<Arc<NFSubscript>>> = subscripts2.clone();
    for mut s1 in &*subscripts1.clone() {
        let mut s1 = s1.clone();
        if rest.clone().is_empty() {
            isEqual = false;
            return Ok(isEqual.clone());
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        s2 = __pa0.clone();
        rest = __pa1.clone();
        if !(self::isEqual(s1.clone(), s2.clone())?) {
            isEqual = false;
            return Ok(isEqual.clone());
        }
    }
    isEqual = rest.clone().is_empty();
    Ok(isEqual)
}

pub fn compare(mut subscript1: Arc<NFSubscript>, mut subscript2: Arc<NFSubscript>) -> Result<i32> {
    let mut comp: i32 = 0;
    if referenceEq(&subscript1.clone(),&subscript2.clone()) {
        comp = 0;
        return Ok(comp.clone());
    }
    comp = Util::intCompare(metamodelica::valueConstructor((&*subscript1.clone()))?, metamodelica::valueConstructor((&*subscript2.clone()))?);
    if comp.clone() != 0 {
        return Ok(comp.clone());
    }
    comp = (::match_deref::match_deref! { match &(subscript1.clone()) {
        Deref @ UNTYPED { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let __pa0 = ::match_deref::match_deref! { match &(subscript2.clone()) {
                Deref @ UNTYPED { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            Expression::compare(var_field!((*subscript1).exp, NFSubscript::UNTYPED).clone(), e.clone())?
        },
        Deref @ INDEX { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let __pa0 = ::match_deref::match_deref! { match &(subscript2.clone()) {
                Deref @ INDEX { index: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            Expression::compare(var_field!((*subscript1).index, NFSubscript::INDEX).clone(), e.clone())?
        },
        Deref @ SLICE { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let __pa0 = ::match_deref::match_deref! { match &(subscript2.clone()) {
                Deref @ SLICE { slice: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            Expression::compare(var_field!((*subscript1).slice, NFSubscript::SLICE).clone(), e.clone())?
        },
        Deref @ WHOLE { .. } => {
            0
        },
        Deref @ SPLIT_INDEX { .. } => {
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut index: i32 = 0;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(subscript2.clone()) {
                Deref @ SPLIT_INDEX { dimIndex: __pa0, node: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            index = __pa0.clone();
            node = __pa1.clone();
            comp = InstNode::refCompare(var_field!((*subscript1).node, NFSubscript::SPLIT_INDEX).clone(), node.clone())?;
            if (comp.clone() == 0) {Util::intCompare(var_field!((*subscript1).dimIndex, NFSubscript::SPLIT_INDEX).clone(), index.clone())} else {comp.clone()}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(comp)
}

pub fn compareList(mut subscripts1: Arc<metamodelica::List<Arc<NFSubscript>>>, mut subscripts2: Arc<metamodelica::List<Arc<NFSubscript>>>) -> Result<i32> {
    let mut comp: i32 = 0;
    let mut s2: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut rest_s2: Arc<metamodelica::List<Arc<NFSubscript>>> = subscripts2.clone();
    comp = Util::intCompare((subscripts1.clone().len() as i32), (subscripts2.clone().len() as i32));
    if comp.clone() != 0 {
        return Ok(comp.clone());
    }
    for mut s1 in &*subscripts1.clone() {
        let mut s1 = s1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_s2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        s2 = __pa0.clone();
        rest_s2 = __pa1.clone();
        comp = compare(s1.clone(), s2.clone())?;
        if comp.clone() != 0 {
            return Ok(comp.clone());
        }
    }
    comp = 0;
    Ok(comp)
}

pub fn containsExp(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => Expression::contains(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(), func.clone())?,
        Deref @ INDEX { .. } => Expression::contains(var_field!((*subscript).index, NFSubscript::INDEX).clone(), func.clone())?,
        Deref @ SLICE { .. } => Expression::contains(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), func.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn listContainsExp(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        if containsExp(s.clone(), func.clone())? {
            res = true;
            return Ok(res.clone());
        }
    }
    res = false;
    Ok(res)
}

pub fn containsExpShallow(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> bool {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => func(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone()).unwrap(),
        Deref @ INDEX { .. } => func(var_field!((*subscript).index, NFSubscript::INDEX).clone()).unwrap(),
        Deref @ SLICE { .. } => func(var_field!((*subscript).slice, NFSubscript::SLICE).clone()).unwrap(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn listContainsExpShallow(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> bool {
    pub type ContainsPred = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        if containsExpShallow(s.clone(), func.clone()) {
            res = true;
            return res.clone();
        }
    }
    res = false;
    res
}

pub fn applyExp(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => {
            Expression::apply(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(), func.clone())?;
            ()
        },
        Deref @ INDEX { .. } => {
            Expression::apply(var_field!((*subscript).index, NFSubscript::INDEX).clone(), func.clone())?;
            ()
        },
        Deref @ SLICE { .. } => {
            Expression::apply(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn applyExpShallow(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> () {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => {
            func(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone()).unwrap();
            ()
        },
        Deref @ INDEX { .. } => {
            func(var_field!((*subscript).index, NFSubscript::INDEX).clone()).unwrap();
            ()
        },
        Deref @ SLICE { .. } => {
            func(var_field!((*subscript).slice, NFSubscript::SLICE).clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn mapExp(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFSubscript>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { exp: e1 } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e2.clone())) {subscript.clone()} else {Arc::new(NFSubscript::UNTYPED { exp: e2.clone() })}
        },
        Deref @ INDEX { index: e1 } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e2.clone())) {subscript.clone()} else {fromTypedExp(e2.clone())}
        },
        Deref @ SLICE { slice: e1 } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&e1.clone(),&e2.clone())) {subscript.clone()} else {fromTypedExp(e2.clone())}
        },
        _ => {
            subscript.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub fn mapShallowExp(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFSubscript> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { exp: e1 } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = func(e1.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e2.clone())) {subscript.clone()} else {Arc::new(NFSubscript::UNTYPED { exp: e2.clone() })}
        },
        Deref @ INDEX { index: e1 } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = func(e1.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e2.clone())) {subscript.clone()} else {fromTypedExp(e2.clone())}
        },
        Deref @ SLICE { slice: e1 } => {
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e2 = func(e1.clone()).unwrap();
            if (referenceEq(&e1.clone(),&e2.clone())) {subscript.clone()} else {fromTypedExp(e2.clone())}
        },
        _ => {
            subscript.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSubscript
}

pub fn foldExp<ArgT: Clone + 'static>(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => Expression::fold(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(), func.clone(), arg.clone())?,
        Deref @ INDEX { .. } => Expression::fold(var_field!((*subscript).index, NFSubscript::INDEX).clone(), func.clone(), arg.clone())?,
        Deref @ SLICE { .. } => Expression::fold(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), func.clone(), arg.clone())?,
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn mapFoldExp<ArgT: Clone + 'static>(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<NFSubscript>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>;

    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut arg: ArgT = arg;
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, arg) = Expression::mapFold(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(),&exp.clone())) {subscript.clone()} else {Arc::new(NFSubscript::UNTYPED { exp: exp.clone() })}
        },
        Deref @ INDEX { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, arg) = Expression::mapFold(var_field!((*subscript).index, NFSubscript::INDEX).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*subscript).index, NFSubscript::INDEX).clone(),&exp.clone())) {subscript.clone()} else {fromTypedExp(exp.clone())}
        },
        Deref @ SLICE { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, arg) = Expression::mapFold(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), func.clone(), arg.clone())?;
            if (referenceEq(&var_field!((*subscript).slice, NFSubscript::SLICE).clone(),&exp.clone())) {subscript.clone()} else {fromTypedExp(exp.clone())}
        },
        _ => {
            subscript.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSubscript, arg))
}

pub fn mapFoldExpShallow<ArgT: Clone + 'static>(mut subscript: Arc<NFSubscript>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<NFSubscript>, ArgT) {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>;

    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut arg: ArgT = arg;
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, arg) = func(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(), arg.clone()).unwrap();
            if (referenceEq(&var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(),&exp.clone())) {subscript.clone()} else {Arc::new(NFSubscript::UNTYPED { exp: exp.clone() })}
        },
        Deref @ INDEX { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, arg) = func(var_field!((*subscript).index, NFSubscript::INDEX).clone(), arg.clone()).unwrap();
            if (referenceEq(&var_field!((*subscript).index, NFSubscript::INDEX).clone(),&exp.clone())) {subscript.clone()} else {fromTypedExp(exp.clone())}
        },
        Deref @ SLICE { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, arg) = func(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), arg.clone()).unwrap();
            if (referenceEq(&var_field!((*subscript).slice, NFSubscript::SLICE).clone(),&exp.clone())) {subscript.clone()} else {fromTypedExp(exp.clone())}
        },
        _ => {
            subscript.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outSubscript, arg)
}

pub fn toAbsyn(mut subscript: Arc<NFSubscript>) -> Result<Arc<Absyn::Subscript>> {
    let mut asubscript: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    asubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ RAW_SUBSCRIPT { .. } => var_field!((*subscript).subscript, NFSubscript::RAW_SUBSCRIPT).clone(),
        Deref @ UNTYPED { .. } => Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Expression::toAbsyn(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone())? }),
        Deref @ INDEX { .. } => Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Expression::toAbsyn(var_field!((*subscript).index, NFSubscript::INDEX).clone())? }),
        Deref @ SLICE { .. } => Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Expression::toAbsyn(var_field!((*subscript).slice, NFSubscript::SLICE).clone())? }),
        Deref @ WHOLE { .. } => Arc::new(openmodelica_ast::Absyn::Subscript::NOSUB),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.toAbsyn")); __mm_s.push_str(&*literal!(" failed on unknown subscript")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(asubscript)
}

pub fn toDAE(mut subscript: Arc<NFSubscript>) -> Result<Arc<DAE::Subscript>> {
    let mut daeSubscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    daeSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => Arc::new(DAE::Subscript::INDEX { exp: Expression::toDAE(var_field!((*subscript).index, NFSubscript::INDEX).clone(), false)? }),
        Deref @ SLICE { .. } => Arc::new(DAE::Subscript::SLICE { exp: Expression::toDAE(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), false)? }),
        Deref @ WHOLE { .. } => Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.toDAE")); __mm_s.push_str(&*literal!(" failed on unknown subscript ")); __mm_s.push_str(&*toString(subscript.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(daeSubscript)
}

pub fn toString(mut subscript: Arc<NFSubscript>) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ RAW_SUBSCRIPT { .. } => Dump::printSubscriptStr(var_field!((*subscript).subscript, NFSubscript::RAW_SUBSCRIPT).clone())?,
        Deref @ UNTYPED { .. } => Expression::toString(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone())?,
        Deref @ INDEX { .. } => Expression::toString(var_field!((*subscript).index, NFSubscript::INDEX).clone())?,
        Deref @ SLICE { .. } => Expression::toString(var_field!((*subscript).slice, NFSubscript::SLICE).clone())?,
        Deref @ EXPANDED_SLICE { .. } => List::toString(var_field!((*subscript).indices, NFSubscript::EXPANDED_SLICE).clone(), (std::sync::Arc::new(toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSubscript>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), false, 0)?,
        Deref @ WHOLE { .. } => literal!(":"),
        Deref @ SPLIT_PROXY { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*InstNode::name(var_field!((*subscript).origin, NFSubscript::SPLIT_PROXY).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*InstNode::name(var_field!((*subscript).parent, NFSubscript::SPLIT_PROXY).clone())?); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) },
        Deref @ SPLIT_INDEX { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*InstNode::name(var_field!((*subscript).node, NFSubscript::SPLIT_INDEX).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", var_field!((*subscript).dimIndex, NFSubscript::SPLIT_INDEX).clone()))); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(string)
}

pub fn toStringList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = (List::toString(subscripts.clone(), (std::sync::Arc::new(toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSubscript>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?).clone();
    Ok(string)
}

pub fn toFlatString(mut subscript: Arc<NFSubscript>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ RAW_SUBSCRIPT { .. } => Dump::printSubscriptStr(var_field!((*subscript).subscript, NFSubscript::RAW_SUBSCRIPT).clone())?,
        Deref @ UNTYPED { .. } => Expression::toFlatString(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone(), format.clone())?,
        Deref @ INDEX { .. } => Expression::toFlatString(var_field!((*subscript).index, NFSubscript::INDEX).clone(), format.clone())?,
        Deref @ SLICE { .. } => Expression::toFlatString(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), format.clone())?,
        Deref @ EXPANDED_SLICE { .. } => List::toString(var_field!((*subscript).indices, NFSubscript::EXPANDED_SLICE).clone(), (std::sync::Arc::new(toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSubscript>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), false, 0)?,
        Deref @ WHOLE { .. } => literal!(":"),
        Deref @ SPLIT_INDEX { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*InstNode::name(var_field!((*subscript).node, NFSubscript::SPLIT_INDEX).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", var_field!((*subscript).dimIndex, NFSubscript::SPLIT_INDEX).clone()))); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(string)
}

pub fn toFlatStringList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>, mut format: BaseModelica::OutputFormat, mut escapeQuotes: bool) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = (List::toString(subscripts.clone(), Arc::new({ let __pe_b1 = format.clone(); move |__pe_a0| toFlatString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("[")).clone(), (literal!(",")).clone(), (literal!("]")).clone(), false, 0)?).clone();
    if escapeQuotes.clone() {
        string = (Util::escapeQuotes((string.clone()).clone())?).clone();
    }
    Ok(string)
}

pub fn toJSON(mut subscript: Arc<NFSubscript>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => Expression::toJSON(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone())?,
        Deref @ INDEX { .. } => Expression::toJSON(var_field!((*subscript).index, NFSubscript::INDEX).clone())?,
        Deref @ SLICE { .. } => Expression::toJSON(var_field!((*subscript).slice, NFSubscript::SLICE).clone())?,
        _ => JSON::makeString((toString(subscript.clone())?).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn toJSONList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        json = JSON::addElement(toJSON(s.clone())?, json.clone())?;
    }
    Ok(json)
}

pub fn eval(mut subscript: Arc<NFSubscript>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<NFSubscript>> {
    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => Arc::new(NFSubscript::INDEX { index: Ceval::evalExp(var_field!((*subscript).index, NFSubscript::INDEX).clone(), target.clone())? }),
        Deref @ SLICE { .. } => Arc::new(NFSubscript::SLICE { slice: Ceval::evalExp(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), target.clone())? }),
        _ => subscript.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub fn simplify(mut subscript: Arc<NFSubscript>, mut dimension: Arc<Dimension::NFDimension>) -> Result<Arc<NFSubscript>> {
    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => Arc::new(NFSubscript::INDEX { index: SimplifyExp::simplify(var_field!((*subscript).index, NFSubscript::INDEX).clone(), false)? }),
        Deref @ SLICE { .. } => simplifySlice(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), dimension.clone())?,
        _ => subscript.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub fn simplifySlice(mut slice: Arc<Expression::NFExpression>, mut dimension: Arc<Dimension::NFDimension>) -> Result<Arc<NFSubscript>> {
    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = SimplifyExp::simplify(slice.clone(), false)?;
    outSubscript = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { .. } if ((isNone(var_field!((*exp).step, Expression::NFExpression::RANGE).clone()) || Expression::isOne(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?)) && Dimension::expIsLowerBound(var_field!((*exp).start, Expression::NFExpression::RANGE).clone()) && Dimension::expIsUpperBound(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), dimension.clone())) => Arc::new(crate::NFSubscript::WHOLE),
        _ => Arc::new(NFSubscript::SLICE { slice: exp.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub fn simplifyList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut trim: bool) -> Result<Arc<metamodelica::List<Arc<NFSubscript>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    let mut d: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_d: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = dimensions.clone();
    if dimensions.clone().is_empty() {
        outSubscripts = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
        for mut s in (subscripts.clone()).into_iter().cloned() {
            let __x = simplify(s.clone(), Arc::new(crate::NFDimension::UNKNOWN))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    } else {
        for mut s in &*subscripts.clone() {
            let mut s = s.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_d.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            d = __pa0.clone();
            rest_d = __pa1.clone();
            outSubscripts = cons(simplify(s.clone(), d.clone())?, outSubscripts.clone());
        }
        if trim.clone() {
            outSubscripts = List::trim(outSubscripts.clone(), (std::sync::Arc::new(fnptr!(isWhole, Arc<NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSubscript>) -> Result<bool> + 'static>))?.reverse();
        } else {
            outSubscripts = outSubscripts.clone().reverse();
        }
    }
    Ok(outSubscripts)
}

pub fn toDimension(mut subscript: Arc<NFSubscript>) -> Result<Arc<Dimension::NFDimension>> {
    let mut dimension: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    dimension = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone()),
        Deref @ SLICE { .. } => listHead(Type::arrayDims(Expression::typeOf(var_field!((*subscript).slice, NFSubscript::SLICE).clone())))?,
        Deref @ WHOLE { .. } => Arc::new(crate::NFDimension::UNKNOWN),
        Deref @ SPLIT_INDEX { .. } => Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.toDimension")); __mm_s.push_str(&*literal!(" got wrong subscript ")); __mm_s.push_str(&*toString(subscript.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dimension)
}

pub fn fromDimension(mut dimension: Arc<Dimension::NFDimension>) -> Result<Arc<NFSubscript>> {
    let mut subscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    subscript = (::match_deref::match_deref! { match &(dimension.clone()) {
        Deref @ Dimension::INTEGER { .. } => Arc::new(NFSubscript::SLICE { slice: Expression::makeIntegerRange(1, 1, var_field!((*dimension).size, Dimension::NFDimension::INTEGER).clone())? }),
        Deref @ Dimension::BOOLEAN => Arc::new(NFSubscript::SLICE { slice: Expression::makeRange(Arc::new(Expression::NFExpression::BOOLEAN { value: false }), None, Arc::new(Expression::NFExpression::BOOLEAN { value: true }))? }),
        Deref @ Dimension::ENUM { .. } => Arc::new(NFSubscript::SLICE { slice: Expression::makeRange(Expression::makeEnumLiteral(var_field!((*dimension).enumType, Dimension::NFDimension::ENUM).clone(), 1)?, None, Expression::makeEnumLiteral(var_field!((*dimension).enumType, Dimension::NFDimension::ENUM).clone(), Type::enumSize(var_field!((*dimension).enumType, Dimension::NFDimension::ENUM).clone())?)?)? }),
        Deref @ Dimension::EXP { .. } => Arc::new(NFSubscript::SLICE { slice: Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), None, var_field!((*dimension).exp, Dimension::NFDimension::EXP).clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(subscript)
}

pub fn scalarize(mut subscript: Arc<NFSubscript>, mut dimension: Arc<Dimension::NFDimension>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<NFSubscript>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    subscripts = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ INDEX { .. } => list![subscript.clone()],
        Deref @ SLICE { .. } => ({
        let mut __acc: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
        for mut e in (Expression::arrayElements((ExpandExp::expand(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), resize.clone(), false)?).0)?).borrow().iter() {
            let __x = Arc::new(NFSubscript::INDEX { index: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ WHOLE { .. } => RangeIterator::map(RangeIterator::fromDim(dimension.clone(), resize.clone())?, (std::sync::Arc::new(makeIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<NFSubscript>> + 'static>))?,
        _ => list![subscript.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subscripts)
}

pub fn scalarizeList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFSubscript>>>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<NFSubscript>>>>> = metamodelica::nil();
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = dimensions.clone();
    let mut subs: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim = __pa0.clone();
        rest_dims = __pa1.clone();
        subs = scalarize(s.clone(), dim.clone(), resize.clone())?;
        if subs.clone().is_empty() {
            outSubscripts = metamodelica::nil();
            return Ok(outSubscripts.clone());
        } else {
            outSubscripts = cons(subs.clone(), outSubscripts.clone());
        }
    }
    for mut d in &*rest_dims.clone() {
        let mut d = d.clone();
        subs = RangeIterator::map(RangeIterator::fromDim(d.clone(), resize.clone())?, (std::sync::Arc::new(makeIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<NFSubscript>> + 'static>))?;
        if subs.clone().is_empty() {
            outSubscripts = metamodelica::nil();
            return Ok(outSubscripts.clone());
        } else {
            outSubscripts = cons(subs.clone(), outSubscripts.clone());
        }
    }
    outSubscripts = outSubscripts.clone().reverse();
    Ok(outSubscripts)
}

pub fn expand(mut subscript: Arc<NFSubscript>, mut dimension: Arc<Dimension::NFDimension>, mut resize: bool) -> Result<(Arc<NFSubscript>, bool)> {
    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut expanded: bool = false;
    (outSubscript, expanded) = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ SLICE { .. } => {
            expandSlice(subscript.clone(), resize.clone())?
        },
        Deref @ WHOLE { .. } => {
            let mut iter: Arc<RangeIterator::NFRangeIterator>;
            iter = RangeIterator::fromDim(dimension.clone(), resize.clone())?;
            if RangeIterator::isValid(iter.clone()) {
                outSubscript = Arc::new(NFSubscript::EXPANDED_SLICE { indices: RangeIterator::map(iter.clone(), (std::sync::Arc::new(makeIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<NFSubscript>> + 'static>))? });
                expanded = true;
            } else {
                outSubscript = subscript.clone();
                expanded = false;
            }
            (outSubscript.clone(), expanded.clone())
        },
        _ => {
            (subscript.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSubscript, expanded))
}

pub fn expandSlice(mut subscript: Arc<NFSubscript>, mut resize: bool) -> Result<(Arc<NFSubscript>, bool)> {
    let mut outSubscript: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut expanded: bool = false;
    (outSubscript, expanded) = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ SLICE { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (exp, _) = ExpandExp::expand(var_field!((*subscript).slice, NFSubscript::SLICE).clone(), resize.clone(), false)?;
            if Expression::isArray(exp.clone()) {
                outSubscript = Arc::new(NFSubscript::EXPANDED_SLICE { indices: ({
        let mut __acc: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
        for mut e in (Expression::arrayElements(exp.clone())?).borrow().iter() {
            let __x = Arc::new(NFSubscript::INDEX { index: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) });
                expanded = true;
            } else {
                outSubscript = subscript.clone();
                expanded = false;
            }
            (outSubscript.clone(), expanded.clone())
        },
        _ => {
            (subscript.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSubscript, expanded))
}

pub fn expandList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>, mut dimensions: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<NFSubscript>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = dimensions.clone();
    let mut sub: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim = __pa0.clone();
        rest_dims = __pa1.clone();
        (sub, _) = expand(s.clone(), dim.clone(), resize.clone())?;
        outSubscripts = cons(sub.clone(), outSubscripts.clone());
    }
    for mut d in &*rest_dims.clone() {
        let mut d = d.clone();
        sub = Arc::new(NFSubscript::EXPANDED_SLICE { indices: RangeIterator::map(RangeIterator::fromDim(d.clone(), resize.clone())?, (std::sync::Arc::new(makeIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<NFSubscript>> + 'static>))? });
        outSubscripts = cons(sub.clone(), outSubscripts.clone());
    }
    outSubscripts = outSubscripts.clone().reverse();
    Ok(outSubscripts)
}

pub fn variability(mut subscript: Arc<NFSubscript>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => Expression::variability(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone())?,
        Deref @ INDEX { .. } => Expression::variability(var_field!((*subscript).index, NFSubscript::INDEX).clone())?,
        Deref @ SLICE { .. } => Expression::variability(var_field!((*subscript).slice, NFSubscript::SLICE).clone())?,
        _ => Variability::CONSTANT.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn variabilityList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT.clone();
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        var = Prefixes::variabilityMax(var.clone(), variability(s.clone())?);
    }
    Ok(var)
}

pub fn purity(mut subscript: Arc<NFSubscript>) -> Result<Purity> {
    let mut purity: Purity = Purity::PURE;
    purity = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ UNTYPED { .. } => Expression::purity(var_field!((*subscript).exp, NFSubscript::UNTYPED).clone())?,
        Deref @ INDEX { .. } => Expression::purity(var_field!((*subscript).index, NFSubscript::INDEX).clone())?,
        Deref @ SLICE { .. } => Expression::purity(var_field!((*subscript).slice, NFSubscript::SLICE).clone())?,
        _ => Purity::IMPURE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(purity)
}

pub fn purityList(mut subscripts: Arc<metamodelica::List<Arc<NFSubscript>>>) -> Result<Purity> {
    let mut pur: Purity = Purity::PURE.clone();
    for mut s in &*subscripts.clone() {
        let mut s = s.clone();
        pur = Prefixes::purityMin(pur.clone(), purity(s.clone())?);
    }
    Ok(pur)
}

pub fn mergeList(mut newSubs: Arc<metamodelica::List<Arc<NFSubscript>>>, mut oldSubs: Arc<metamodelica::List<Arc<NFSubscript>>>, mut dimensions: i32, mut backend: bool) -> Result<(Arc<metamodelica::List<Arc<NFSubscript>>>, Arc<metamodelica::List<Arc<NFSubscript>>>)> {
    let mut outSubs: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    let mut remainingSubs: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    let mut subs_count: i32 = 0;
    let mut new_sub: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut old_sub: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    let mut rest_old_subs: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    let mut merged: bool = true;
    if backend.clone() && (oldSubs.clone().len() as i32) >= dimensions.clone() && List::all(List::firstN(oldSubs.clone(), dimensions.clone())?, (std::sync::Arc::new(fnptr!(isBackendIterator, Arc<NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSubscript>) -> Result<bool> + 'static>)) {
        (_, remainingSubs) = List::split(newSubs.clone(), dimensions.clone())?;
        (outSubs, _) = List::split(oldSubs.clone(), dimensions.clone())?;
        return Ok((outSubs.clone(), remainingSubs.clone()));
    }
    if oldSubs.clone().is_empty() {
        if (newSubs.clone().len() as i32) <= dimensions.clone() {
            outSubs = newSubs.clone();
            remainingSubs = metamodelica::nil();
        } else {
            (outSubs, remainingSubs) = List::split(newSubs.clone(), dimensions.clone())?;
        }
        return Ok((outSubs.clone(), remainingSubs.clone()));
    }
    subs_count = (oldSubs.clone().len() as i32);
    remainingSubs = newSubs.clone();
    rest_old_subs = oldSubs.clone();
    outSubs = metamodelica::nil();
    while merged.clone() && !(remainingSubs.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(remainingSubs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        new_sub = __pa0.clone();
        remainingSubs = __pa1.clone();
        merged = false;
        while !(merged.clone()) {
            if rest_old_subs.clone().is_empty() {
                remainingSubs = cons(new_sub.clone(), remainingSubs.clone());
                break;
            } else {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_old_subs.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                old_sub = __pa2.clone();
                rest_old_subs = __pa3.clone();
                (merged, outSubs) = (::match_deref::match_deref! { match &(old_sub.clone()) {
        Deref @ SLICE { .. } => {
            if !(isWhole(new_sub.clone())) {
                outSubs = cons(Arc::new(NFSubscript::INDEX { index: Expression::applySubscript(new_sub.clone(), var_field!((*old_sub).slice, NFSubscript::SLICE).clone(), metamodelica::nil(), false)? }), outSubs.clone());
            } else {
                outSubs = cons(old_sub.clone(), outSubs.clone());
            }
            (true, outSubs.clone())
        },
        Deref @ WHOLE { .. } => (true, cons(new_sub.clone(), outSubs.clone())),
        _ => (false, cons(old_sub.clone(), outSubs.clone())),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
        }
    }
    for mut s in &*rest_old_subs.clone() {
        let mut s = s.clone();
        outSubs = cons(s.clone(), outSubs.clone());
    }
    while !(remainingSubs.clone().is_empty()) && subs_count.clone() < dimensions.clone() {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(remainingSubs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        new_sub = __pa4.clone();
        remainingSubs = __pa5.clone();
        outSubs = cons(new_sub.clone(), outSubs.clone());
        subs_count = subs_count.clone() + 1;
    }
    outSubs = outSubs.clone().reverse();
    Ok((outSubs, remainingSubs))
}

pub fn nth(mut dim: Arc<Dimension::NFDimension>, mut i: i32) -> Result<Arc<NFSubscript>> {
    let mut sub: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    sub = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::INTEGER { .. } => Arc::new(NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) }),
        Deref @ Dimension::BOOLEAN if (i.clone() == 1) => Arc::new(NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::BOOLEAN { value: false }) }),
        Deref @ Dimension::BOOLEAN if (i.clone() == 2) => Arc::new(NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::BOOLEAN { value: true }) }),
        Deref @ Dimension::ENUM { .. } => Arc::new(NFSubscript::INDEX { index: Expression::nthEnumLiteral(var_field!((*dim).enumType, Dimension::NFDimension::ENUM).clone(), i.clone())? }),
        Deref @ Dimension::RESIZABLE { .. } => Arc::new(NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }) }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.nth")); __mm_s.push_str(&*literal!(" got an incorrect dimension type ")); __mm_s.push_str(&*Dimension::toString(dim.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sub)
}

pub fn first(mut dim: Arc<Dimension::NFDimension>) -> Result<Arc<NFSubscript>> {
    let mut sub: Arc<NFSubscript> = Arc::new(NFSubscript::WHOLE);
    sub = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::INTEGER { .. } => Arc::new(NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) }),
        Deref @ Dimension::BOOLEAN => Arc::new(NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::BOOLEAN { value: false }) }),
        Deref @ Dimension::ENUM { .. } => Arc::new(NFSubscript::INDEX { index: Expression::nthEnumLiteral(var_field!((*dim).enumType, Dimension::NFDimension::ENUM).clone(), 1)? }),
        Deref @ Dimension::RESIZABLE { .. } => Arc::new(NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(sub)
}

pub fn isFirst(mut sub: Arc<NFSubscript>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ INDEX { index: Deref @ Expression::INTEGER { value: 1 } } => true,
        Deref @ INDEX { index: Deref @ Expression::BOOLEAN { value: false } } => true,
        Deref @ INDEX { index: Deref @ Expression::ENUM_LITERAL { index: 1, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isSplit(mut sub: Arc<NFSubscript>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ SPLIT_PROXY { .. } => true,
        Deref @ SPLIT_INDEX { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isSplitIndex(mut sub: Arc<NFSubscript>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ SPLIT_INDEX { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isSplitClassProxy(mut sub: Arc<NFSubscript>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ SPLIT_PROXY { .. } => InstNode::isClass(var_field!((*sub).origin, NFSubscript::SPLIT_PROXY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isSplitFromOrigin(mut sub: Arc<NFSubscript>, mut origin: Arc<InstNode::InstNode>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ SPLIT_PROXY { .. } => InstNode::refEqual(origin.clone(), var_field!((*sub).origin, NFSubscript::SPLIT_PROXY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn expandSplitIndices(mut subs: Arc<metamodelica::List<Arc<NFSubscript>>>, mut indicesToKeep: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<NFSubscript>>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<NFSubscript>>> = metamodelica::nil();
    let mut changed: bool = false;
    for mut s in &*subs.clone() {
        let mut s = s.clone();
        let () = (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ SPLIT_INDEX { .. } => {
            if List::isMemberOnTrue(var_field!((*s).node, NFSubscript::SPLIT_INDEX).clone(), indicesToKeep.clone(), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>)) {
                outSubs = cons(s.clone(), outSubs.clone());
            } else {
                outSubs = cons(Arc::new(crate::NFSubscript::WHOLE), outSubs.clone());
                changed = true;
            }
            ()
        },
        _ => {
            outSubs = cons(s.clone(), outSubs.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if changed.clone() {
        outSubs = List::trim(outSubs.clone(), (std::sync::Arc::new(fnptr!(isWhole, Arc<NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFSubscript>) -> Result<bool> + 'static>))?;
        outSubs = outSubs.clone().reverse();
    } else {
        outSubs = subs.clone();
    }
    Ok(outSubs)
}

pub fn hash(mut sub: Arc<NFSubscript>) -> i32 {
    let mut hash: i32 = hashContinue(sub.clone(), Util::HASH_SEED.clone()).unwrap();
    hash
}

pub fn hashContinue(mut sub: Arc<NFSubscript>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ RAW_SUBSCRIPT { .. } => stringHashDjb2Continue((Dump::printSubscriptStr(var_field!((*sub).subscript, NFSubscript::RAW_SUBSCRIPT).clone())?).clone(), hash.clone()),
        Deref @ UNTYPED { .. } => Expression::hashContinue(var_field!((*sub).exp, NFSubscript::UNTYPED).clone(), hash.clone())?,
        Deref @ INDEX { .. } => Expression::hashContinue(var_field!((*sub).index, NFSubscript::INDEX).clone(), hash.clone())?,
        Deref @ SLICE { .. } => Expression::hashContinue(var_field!((*sub).slice, NFSubscript::SLICE).clone(), hash.clone())?,
        Deref @ EXPANDED_SLICE { .. } => {
            hash = stringHashDjb2Continue((literal!("{")).clone(), hash.clone());
            for mut s in &*var_field!((*sub).indices, NFSubscript::EXPANDED_SLICE).clone() {
                let mut s = s.clone();
                hash = hashContinue(s.clone(), hash.clone())?;
                hash = stringHashDjb2Continue((literal!(", ")).clone(), hash.clone());
            }
            hash = stringHashDjb2Continue((literal!("}")).clone(), hash.clone());
            hash.clone()
        },
        Deref @ WHOLE { .. } => stringHashDjb2Continue((literal!(":")).clone(), hash.clone()),
        Deref @ SPLIT_PROXY { .. } => {
            hash = InstNode::hashContinue(var_field!((*sub).origin, NFSubscript::SPLIT_PROXY).clone(), hash.clone())?;
            hash = InstNode::hashContinue(var_field!((*sub).parent, NFSubscript::SPLIT_PROXY).clone(), hash.clone())?;
            hash.clone()
        },
        Deref @ SPLIT_INDEX { .. } => {
            hash = InstNode::hashContinue(var_field!((*sub).node, NFSubscript::SPLIT_INDEX).clone(), hash.clone())?;
            hash = stringHashDjb2Continue((intString(var_field!((*sub).dimIndex, NFSubscript::SPLIT_INDEX).clone())).clone(), hash.clone());
            hash.clone()
        },
        _ => hash.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hash)
}

pub fn splitIndexDimExp(mut sub: Arc<NFSubscript>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut index: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ SPLIT_INDEX { dimIndex: __pa0, node: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    index = __pa0.clone();
    node = __pa1.clone();
    exp = Dimension::sizeExp(Type::nthDimension(InstNode::getType(node.clone())?, index.clone())?)?;
    Ok(exp)
}

pub fn isLiteral(mut sub: Arc<NFSubscript>) -> bool {
    let mut literal: bool = false;
    literal = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ UNTYPED { .. } => Expression::isLiteral(var_field!((*sub).exp, NFSubscript::UNTYPED).clone()),
        Deref @ INDEX { .. } => Expression::isLiteral(var_field!((*sub).index, NFSubscript::INDEX).clone()),
        Deref @ SLICE { .. } => Expression::isLiteral(var_field!((*sub).slice, NFSubscript::SLICE).clone()),
        Deref @ WHOLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    literal
}

pub fn fillWithWholeLeft(mut subs: Arc<metamodelica::List<Arc<NFSubscript>>>, mut targetLength: i32) -> Arc<metamodelica::List<Arc<NFSubscript>>> {
    let mut subs: Arc<metamodelica::List<Arc<NFSubscript>>> = subs;
    subs = listAppend(List::fill(Arc::new(crate::NFSubscript::WHOLE), targetLength.clone() - (subs.clone().len() as i32)), subs.clone());
    subs
}


