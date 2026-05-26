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

use crate::NFClass as Class;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRestriction as Restriction;
use crate::NFType;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq)]
pub struct NFAttributes {
    pub connectorType: i32,
    pub parallelism: Prefixes::Parallelism,
    pub variability: Prefixes::Variability,
    pub direction: Prefixes::Direction,
    pub innerOuter: Prefixes::InnerOuter,
    pub isFinal: bool,
    pub isRedeclare: bool,
    pub isReplaceable: Arc<Prefixes::Replaceable>,
    pub isResizable: bool,
}

pub type ATTRIBUTES = NFAttributes;

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<NFAttributes>
// Expr: Constructor { name: 'NFAttributes.ATTRIBUTES', args: [Var { name: 'ConnectorType.NON_CONNECTOR', segments: [CrefSegment { name: 'ConnectorType', subscripts: [] }, CrefSegment { name: 'NON_CONNECTOR', subscripts: [] }], ty: I32 }, Var { name: 'Parallelism.NON_PARALLEL', segments: [CrefSegment { name: 'Parallelism', subscripts: [] }, CrefSegment { name: 'NON_PARALLEL', subscripts: [] }], ty: Enumeration('NFPrefixes.Parallelism') }, Var { name: 'Variability.CONTINUOUS', segments: [CrefSegment { name: 'Variability', subscripts: [] }, CrefSegment { name: 'CONTINUOUS', subscripts: [] }], ty: Enumeration('NFPrefixes.Variability') }, Var { name: 'Direction.NONE', segments: [CrefSegment { name: 'Direction', subscripts: [] }, CrefSegment { name: 'NONE', subscripts: [] }], ty: Enumeration('NFPrefixes.Direction') }, Var { name: 'InnerOuter.NOT_INNER_OUTER', segments: [CrefSegment { name: 'InnerOuter', subscripts: [] }, CrefSegment { name: 'NOT_INNER_OUTER', subscripts: [] }], ty: Enumeration('NFPrefixes.InnerOuter') }, Lit(Bool(false)), Lit(Bool(false)), Constructor { name: 'NFPrefixes.Replaceable.NOT_REPLACEABLE', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Lit(Bool(false))], named_args: [], ty: RustStruct('NFAttributes'), field_names: ['connectorType', 'parallelism', 'variability', 'direction', 'innerOuter', 'isFinal', 'isRedeclare', 'isReplaceable', 'isResizable'] }
pub fn DEFAULT_ATTR() -> Arc<NFAttributes> { todo!("non-Sync, non-const-emittable constant DEFAULT_ATTR — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<NFAttributes>
// Expr: Constructor { name: 'NFAttributes.ATTRIBUTES', args: [Var { name: 'ConnectorType.NON_CONNECTOR', segments: [CrefSegment { name: 'ConnectorType', subscripts: [] }, CrefSegment { name: 'NON_CONNECTOR', subscripts: [] }], ty: I32 }, Var { name: 'Parallelism.NON_PARALLEL', segments: [CrefSegment { name: 'Parallelism', subscripts: [] }, CrefSegment { name: 'NON_PARALLEL', subscripts: [] }], ty: Enumeration('NFPrefixes.Parallelism') }, Var { name: 'Variability.CONTINUOUS', segments: [CrefSegment { name: 'Variability', subscripts: [] }, CrefSegment { name: 'CONTINUOUS', subscripts: [] }], ty: Enumeration('NFPrefixes.Variability') }, Var { name: 'Direction.INPUT', segments: [CrefSegment { name: 'Direction', subscripts: [] }, CrefSegment { name: 'INPUT', subscripts: [] }], ty: Enumeration('NFPrefixes.Direction') }, Var { name: 'InnerOuter.NOT_INNER_OUTER', segments: [CrefSegment { name: 'InnerOuter', subscripts: [] }, CrefSegment { name: 'NOT_INNER_OUTER', subscripts: [] }], ty: Enumeration('NFPrefixes.InnerOuter') }, Lit(Bool(false)), Lit(Bool(false)), Constructor { name: 'NFPrefixes.Replaceable.NOT_REPLACEABLE', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Lit(Bool(false))], named_args: [], ty: RustStruct('NFAttributes'), field_names: ['connectorType', 'parallelism', 'variability', 'direction', 'innerOuter', 'isFinal', 'isRedeclare', 'isReplaceable', 'isResizable'] }
pub fn INPUT_ATTR() -> Arc<NFAttributes> { todo!("non-Sync, non-const-emittable constant INPUT_ATTR — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<NFAttributes>
// Expr: Constructor { name: 'NFAttributes.ATTRIBUTES', args: [Var { name: 'ConnectorType.NON_CONNECTOR', segments: [CrefSegment { name: 'ConnectorType', subscripts: [] }, CrefSegment { name: 'NON_CONNECTOR', subscripts: [] }], ty: I32 }, Var { name: 'Parallelism.NON_PARALLEL', segments: [CrefSegment { name: 'Parallelism', subscripts: [] }, CrefSegment { name: 'NON_PARALLEL', subscripts: [] }], ty: Enumeration('NFPrefixes.Parallelism') }, Var { name: 'Variability.CONTINUOUS', segments: [CrefSegment { name: 'Variability', subscripts: [] }, CrefSegment { name: 'CONTINUOUS', subscripts: [] }], ty: Enumeration('NFPrefixes.Variability') }, Var { name: 'Direction.OUTPUT', segments: [CrefSegment { name: 'Direction', subscripts: [] }, CrefSegment { name: 'OUTPUT', subscripts: [] }], ty: Enumeration('NFPrefixes.Direction') }, Var { name: 'InnerOuter.NOT_INNER_OUTER', segments: [CrefSegment { name: 'InnerOuter', subscripts: [] }, CrefSegment { name: 'NOT_INNER_OUTER', subscripts: [] }], ty: Enumeration('NFPrefixes.InnerOuter') }, Lit(Bool(false)), Lit(Bool(false)), Constructor { name: 'NFPrefixes.Replaceable.NOT_REPLACEABLE', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Lit(Bool(false))], named_args: [], ty: RustStruct('NFAttributes'), field_names: ['connectorType', 'parallelism', 'variability', 'direction', 'innerOuter', 'isFinal', 'isRedeclare', 'isReplaceable', 'isResizable'] }
pub fn OUTPUT_ATTR() -> Arc<NFAttributes> { todo!("non-Sync, non-const-emittable constant OUTPUT_ATTR — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<NFAttributes>
// Expr: Constructor { name: 'NFAttributes.ATTRIBUTES', args: [Var { name: 'ConnectorType.NON_CONNECTOR', segments: [CrefSegment { name: 'ConnectorType', subscripts: [] }, CrefSegment { name: 'NON_CONNECTOR', subscripts: [] }], ty: I32 }, Var { name: 'Parallelism.NON_PARALLEL', segments: [CrefSegment { name: 'Parallelism', subscripts: [] }, CrefSegment { name: 'NON_PARALLEL', subscripts: [] }], ty: Enumeration('NFPrefixes.Parallelism') }, Var { name: 'Variability.CONSTANT', segments: [CrefSegment { name: 'Variability', subscripts: [] }, CrefSegment { name: 'CONSTANT', subscripts: [] }], ty: Enumeration('NFPrefixes.Variability') }, Var { name: 'Direction.NONE', segments: [CrefSegment { name: 'Direction', subscripts: [] }, CrefSegment { name: 'NONE', subscripts: [] }], ty: Enumeration('NFPrefixes.Direction') }, Var { name: 'InnerOuter.NOT_INNER_OUTER', segments: [CrefSegment { name: 'InnerOuter', subscripts: [] }, CrefSegment { name: 'NOT_INNER_OUTER', subscripts: [] }], ty: Enumeration('NFPrefixes.InnerOuter') }, Lit(Bool(false)), Lit(Bool(false)), Constructor { name: 'NFPrefixes.Replaceable.NOT_REPLACEABLE', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Lit(Bool(false))], named_args: [], ty: RustStruct('NFAttributes'), field_names: ['connectorType', 'parallelism', 'variability', 'direction', 'innerOuter', 'isFinal', 'isRedeclare', 'isReplaceable', 'isResizable'] }
pub fn CONSTANT_ATTR() -> Arc<NFAttributes> { todo!("non-Sync, non-const-emittable constant CONSTANT_ATTR — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<NFAttributes>
// Expr: Constructor { name: 'NFAttributes.ATTRIBUTES', args: [Var { name: 'ConnectorType.NON_CONNECTOR', segments: [CrefSegment { name: 'ConnectorType', subscripts: [] }, CrefSegment { name: 'NON_CONNECTOR', subscripts: [] }], ty: I32 }, Var { name: 'Parallelism.NON_PARALLEL', segments: [CrefSegment { name: 'Parallelism', subscripts: [] }, CrefSegment { name: 'NON_PARALLEL', subscripts: [] }], ty: Enumeration('NFPrefixes.Parallelism') }, Var { name: 'Variability.IMPLICITLY_DISCRETE', segments: [CrefSegment { name: 'Variability', subscripts: [] }, CrefSegment { name: 'IMPLICITLY_DISCRETE', subscripts: [] }], ty: Enumeration('NFPrefixes.Variability') }, Var { name: 'Direction.NONE', segments: [CrefSegment { name: 'Direction', subscripts: [] }, CrefSegment { name: 'NONE', subscripts: [] }], ty: Enumeration('NFPrefixes.Direction') }, Var { name: 'InnerOuter.NOT_INNER_OUTER', segments: [CrefSegment { name: 'InnerOuter', subscripts: [] }, CrefSegment { name: 'NOT_INNER_OUTER', subscripts: [] }], ty: Enumeration('NFPrefixes.InnerOuter') }, Lit(Bool(false)), Lit(Bool(false)), Constructor { name: 'NFPrefixes.Replaceable.NOT_REPLACEABLE', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Lit(Bool(false))], named_args: [], ty: RustStruct('NFAttributes'), field_names: ['connectorType', 'parallelism', 'variability', 'direction', 'innerOuter', 'isFinal', 'isRedeclare', 'isReplaceable', 'isResizable'] }
pub fn IMPL_DISCRETE_ATTR() -> Arc<NFAttributes> { todo!("non-Sync, non-const-emittable constant IMPL_DISCRETE_ATTR — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<NFAttributes>
// Expr: Constructor { name: 'NFAttributes.ATTRIBUTES', args: [Var { name: 'ConnectorType.AUGMENTED', segments: [CrefSegment { name: 'ConnectorType', subscripts: [] }, CrefSegment { name: 'AUGMENTED', subscripts: [] }], ty: I32 }, Var { name: 'Parallelism.NON_PARALLEL', segments: [CrefSegment { name: 'Parallelism', subscripts: [] }, CrefSegment { name: 'NON_PARALLEL', subscripts: [] }], ty: Enumeration('NFPrefixes.Parallelism') }, Var { name: 'Variability.CONTINUOUS', segments: [CrefSegment { name: 'Variability', subscripts: [] }, CrefSegment { name: 'CONTINUOUS', subscripts: [] }], ty: Enumeration('NFPrefixes.Variability') }, Var { name: 'Direction.NONE', segments: [CrefSegment { name: 'Direction', subscripts: [] }, CrefSegment { name: 'NONE', subscripts: [] }], ty: Enumeration('NFPrefixes.Direction') }, Var { name: 'InnerOuter.NOT_INNER_OUTER', segments: [CrefSegment { name: 'InnerOuter', subscripts: [] }, CrefSegment { name: 'NOT_INNER_OUTER', subscripts: [] }], ty: Enumeration('NFPrefixes.InnerOuter') }, Lit(Bool(false)), Lit(Bool(false)), Constructor { name: 'NFPrefixes.Replaceable.NOT_REPLACEABLE', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Lit(Bool(false))], named_args: [], ty: RustStruct('NFAttributes'), field_names: ['connectorType', 'parallelism', 'variability', 'direction', 'innerOuter', 'isFinal', 'isRedeclare', 'isReplaceable', 'isResizable'] }
pub fn AUGMENTED_ATTR() -> Arc<NFAttributes> { todo!("non-Sync, non-const-emittable constant AUGMENTED_ATTR — extend codegen") }

pub fn fromSCode(mut compAttr: SCode::Attributes, mut compPrefs: Arc<SCode::Prefixes>) -> Result<Arc<NFAttributes>> {
    let mut attributes: Arc<NFAttributes>;
    let mut cty: i32 = 0;
    let mut par: Prefixes::Parallelism = Prefixes::Parallelism::NON_PARALLEL;
    let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut dir: Prefixes::Direction = Prefixes::Direction::NONE;
    let mut io: Prefixes::InnerOuter = Prefixes::InnerOuter::NOT_INNER_OUTER;
    let mut fin: bool = false;
    let mut redecl: bool = false;
    let mut repl: Arc<Prefixes::Replaceable> = Arc::new(Prefixes::Replaceable::NOT_REPLACEABLE);
    attributes = (::match_deref::match_deref! { match &((compAttr.clone(), compPrefs.clone())) {
        (SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, variability: SCode::Variability::VAR { .. }, parallelism: SCode::Parallelism::NON_PARALLEL { .. }, connectorType: SCode::ConnectorType::POTENTIAL { .. }, .. }, Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, finalPrefix: SCode::Final::NOT_FINAL { .. }, redeclarePrefix: SCode::Redeclare::NOT_REDECLARE { .. }, .. }) => DEFAULT_ATTR().clone(),
        _ => {
            cty = Prefixes::ConnectorType::fromSCode(compAttr.connectorType.clone())?;
            par = Prefixes::parallelismFromSCode(compAttr.parallelism.clone())?;
            var = Prefixes::variabilityFromSCode(compAttr.variability.clone())?;
            dir = Prefixes::directionFromSCode(compAttr.direction.clone());
            io = Prefixes::innerOuterFromSCode(compPrefs.innerOuter.clone())?;
            fin = SCodeUtil::finalBool(compPrefs.finalPrefix.clone())?;
            redecl = SCodeUtil::redeclareBool(compPrefs.redeclarePrefix.clone())?;
            repl = Arc::new(crate::NFPrefixes::Replaceable::NOT_REPLACEABLE);
            Arc::new(NFAttributes { connectorType: cty.clone(), parallelism: par.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: io.clone(), isFinal: fin.clone(), isRedeclare: redecl.clone(), isReplaceable: repl.clone(), isResizable: false })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attributes)
}

pub fn fromDerivedSCode(mut scodeAttr: SCode::Attributes) -> Result<Arc<NFAttributes>> {
    let mut attributes: Arc<NFAttributes>;
    let mut cty: i32 = 0;
    let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut dir: Prefixes::Direction = Prefixes::Direction::NONE;
    attributes = (match scodeAttr.clone() {
        SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, variability: SCode::Variability::VAR { .. }, connectorType: SCode::ConnectorType::POTENTIAL { .. }, .. } => DEFAULT_ATTR().clone(),
        _ => {
            cty = Prefixes::ConnectorType::fromSCode(scodeAttr.connectorType.clone())?;
            var = Prefixes::variabilityFromSCode(scodeAttr.variability.clone())?;
            dir = Prefixes::directionFromSCode(scodeAttr.direction.clone());
            Arc::new(NFAttributes { connectorType: cty.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: Arc::new(crate::NFPrefixes::Replaceable::NOT_REPLACEABLE), isResizable: false })
        },
    });
    Ok(attributes)
}

pub fn mergeComponentAttributes(mut outerAttr: Arc<NFAttributes>, mut innerAttr: Arc<NFAttributes>, mut node: Arc<InstNode::InstNode>, mut parentRestriction: Arc<Restriction::NFRestriction>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes>;
    let mut cty: i32 = 0;
    let mut par: Prefixes::Parallelism = Prefixes::Parallelism::NON_PARALLEL;
    let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut dir: Prefixes::Direction = Prefixes::Direction::NONE;
    let mut fin: bool = false;
    let mut redecl: bool = false;
    let mut resize: bool = false;
    let mut repl: Arc<Prefixes::Replaceable> = Arc::new(Prefixes::Replaceable::NOT_REPLACEABLE);
    if referenceEq(&outerAttr.clone(),&DEFAULT_ATTR().clone()) && innerAttr.connectorType.clone() == 0 {
        attr = innerAttr.clone();
    } else if referenceEq(&innerAttr.clone(),&DEFAULT_ATTR().clone()) {
        cty = Prefixes::ConnectorType::merge(outerAttr.connectorType.clone(), innerAttr.connectorType.clone(), node.clone(), false)?;
        attr = Arc::new(NFAttributes { connectorType: cty.clone(), parallelism: outerAttr.parallelism.clone(), variability: outerAttr.variability.clone(), direction: outerAttr.direction.clone(), innerOuter: innerAttr.innerOuter.clone(), isFinal: outerAttr.isFinal.clone(), isRedeclare: innerAttr.isRedeclare.clone(), isReplaceable: innerAttr.isReplaceable.clone(), isResizable: innerAttr.isResizable.clone() });
    } else {
        cty = Prefixes::ConnectorType::merge(outerAttr.connectorType.clone(), innerAttr.connectorType.clone(), node.clone(), false)?;
        par = Prefixes::mergeParallelism(outerAttr.parallelism.clone(), innerAttr.parallelism.clone(), node.clone())?;
        var = Prefixes::variabilityMin(outerAttr.variability.clone(), innerAttr.variability.clone());
        if Restriction::isFunction(parentRestriction.clone()) {
            dir = innerAttr.direction.clone();
        } else {
            dir = Prefixes::mergeDirection(outerAttr.direction.clone(), innerAttr.direction.clone(), node.clone(), false)?;
        }
        fin = outerAttr.isFinal.clone() || innerAttr.isFinal.clone();
        redecl = innerAttr.isRedeclare.clone();
        repl = innerAttr.isReplaceable.clone();
        resize = innerAttr.isResizable.clone();
        attr = Arc::new(NFAttributes { connectorType: cty.clone(), parallelism: par.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: innerAttr.innerOuter.clone(), isFinal: fin.clone(), isRedeclare: redecl.clone(), isReplaceable: repl.clone(), isResizable: resize.clone() });
    }
    Ok(attr)
}

pub fn mergeDerivedAttributes(mut outerAttr: Arc<NFAttributes>, mut innerAttr: Arc<NFAttributes>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes>;
    let mut cty: i32 = 0;
    let mut par: Prefixes::Parallelism = Prefixes::Parallelism::NON_PARALLEL;
    let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut dir: Prefixes::Direction = Prefixes::Direction::NONE;
    let mut io: Prefixes::InnerOuter = Prefixes::InnerOuter::NOT_INNER_OUTER;
    let mut fin: bool = false;
    let mut redecl: bool = false;
    let mut resize: bool = false;
    let mut repl: Arc<Prefixes::Replaceable> = Arc::new(Prefixes::Replaceable::NOT_REPLACEABLE);
    if referenceEq(&innerAttr.clone(),&DEFAULT_ATTR().clone()) && outerAttr.connectorType.clone() == 0 {
        attr = outerAttr.clone();
    } else if referenceEq(&outerAttr.clone(),&DEFAULT_ATTR().clone()) && innerAttr.connectorType.clone() == 0 {
        attr = innerAttr.clone();
    } else {
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(outerAttr.clone()) {
            Deref @ ATTRIBUTES { connectorType: __pa0, parallelism: __pa1, variability: __pa2, direction: __pa3, innerOuter: __pa4, isFinal: __pa5, isRedeclare: __pa6, isReplaceable: __pa7, isResizable: __pa8 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cty = __pa0.clone();
        par = __pa1.clone();
        var = __pa2.clone();
        dir = __pa3.clone();
        io = __pa4.clone();
        fin = __pa5.clone();
        redecl = __pa6.clone();
        repl = __pa7.clone();
        resize = __pa8.clone();
        cty = Prefixes::ConnectorType::merge(cty.clone(), innerAttr.connectorType.clone(), node.clone(), true)?;
        var = Prefixes::variabilityMin(var.clone(), innerAttr.variability.clone());
        dir = Prefixes::mergeDirection(dir.clone(), innerAttr.direction.clone(), node.clone(), true)?;
        attr = Arc::new(NFAttributes { connectorType: cty.clone(), parallelism: par.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: innerAttr.innerOuter.clone(), isFinal: fin.clone(), isRedeclare: redecl.clone(), isReplaceable: repl.clone(), isResizable: resize.clone() });
    }
    Ok(attr)
}

pub fn mergeRedeclaredComponentAttributes(mut origAttr: Arc<NFAttributes>, mut redeclAttr: Arc<NFAttributes>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes>;
    let mut cty: i32 = 0;
    let mut rcty: i32 = 0;
    let mut cty_fs: i32 = 0;
    let mut rcty_fs: i32 = 0;
    let mut par: Prefixes::Parallelism = Prefixes::Parallelism::NON_PARALLEL;
    let mut rpar: Prefixes::Parallelism = Prefixes::Parallelism::NON_PARALLEL;
    let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut rvar: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut dir: Prefixes::Direction = Prefixes::Direction::NONE;
    let mut rdir: Prefixes::Direction = Prefixes::Direction::NONE;
    let mut io: Prefixes::InnerOuter = Prefixes::InnerOuter::NOT_INNER_OUTER;
    let mut rio: Prefixes::InnerOuter = Prefixes::InnerOuter::NOT_INNER_OUTER;
    let mut fin: bool = false;
    let mut redecl: bool = false;
    let mut resize: bool = false;
    let mut repl: Arc<Prefixes::Replaceable> = Arc::new(Prefixes::Replaceable::NOT_REPLACEABLE);
    if referenceEq(&origAttr.clone(),&DEFAULT_ATTR().clone()) {
        attr = redeclAttr.clone();
    } else if referenceEq(&redeclAttr.clone(),&DEFAULT_ATTR().clone()) {
        attr = origAttr.clone();
    } else {
        let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(origAttr.clone()) {
            Deref @ ATTRIBUTES { connectorType: __pa0, parallelism: __pa1, variability: __pa2, direction: __pa3, innerOuter: __pa4, isFinal: _, isRedeclare: _, isReplaceable: _, isResizable: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cty = __pa0.clone();
        par = __pa1.clone();
        var = __pa2.clone();
        dir = __pa3.clone();
        io = __pa4.clone();
        let (__pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(redeclAttr.clone()) {
            Deref @ ATTRIBUTES { connectorType: __pa5, parallelism: __pa6, variability: __pa7, direction: __pa8, innerOuter: __pa9, isFinal: __pa10, isRedeclare: __pa11, isReplaceable: __pa12, isResizable: __pa13 } => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
            _ => bail!("pattern mismatch"),
        } };
        rcty = __pa5.clone();
        rpar = __pa6.clone();
        rvar = __pa7.clone();
        rdir = __pa8.clone();
        rio = __pa9.clone();
        fin = __pa10.clone();
        redecl = __pa11.clone();
        repl = __pa12.clone();
        resize = __pa13.clone();
        rcty_fs = intBitAnd(rcty.clone(), ConnectorType::FLOW_STREAM_MASK.clone());
        cty_fs = intBitAnd(cty.clone(), ConnectorType::FLOW_STREAM_MASK.clone());
        if rcty_fs.clone() > 0 {
            if cty_fs.clone() > 0 && rcty_fs.clone() != cty_fs.clone() {
                printRedeclarePrefixError(node.clone(), (Prefixes::ConnectorType::toString(rcty.clone())).clone(), (Prefixes::ConnectorType::toString(cty.clone())).clone())?;
            }
        }
        cty = intBitOr(rcty.clone(), cty_fs.clone());
        if rpar.clone() != Parallelism::NON_PARALLEL.clone() {
            if par.clone() != Parallelism::NON_PARALLEL.clone() && par.clone() != rpar.clone() {
                printRedeclarePrefixError(node.clone(), (Prefixes::parallelismString(rpar.clone())).clone(), (Prefixes::parallelismString(par.clone())).clone())?;
            }
            par = rpar.clone();
        }
        if rvar.clone() != Variability::CONTINUOUS.clone() {
            if rvar.clone() > var.clone() {
                printRedeclarePrefixError(node.clone(), (Prefixes::variabilityString(rvar.clone())?).clone(), (Prefixes::variabilityString(var.clone())?).clone())?;
            }
            var = rvar.clone();
        }
        if rdir.clone() != Direction::NONE.clone() {
            if dir.clone() != Direction::NONE.clone() && rdir.clone() != dir.clone() {
                printRedeclarePrefixError(node.clone(), (Prefixes::directionString(rdir.clone())).clone(), (Prefixes::directionString(dir.clone())).clone())?;
            }
            dir = rdir.clone();
        }
        if rio.clone() != InnerOuter::NOT_INNER_OUTER.clone() {
            if io.clone() != InnerOuter::NOT_INNER_OUTER.clone() && rio.clone() != io.clone() {
                printRedeclarePrefixError(node.clone(), (Prefixes::innerOuterString(rio.clone())).clone(), (Prefixes::innerOuterString(io.clone())).clone())?;
            }
            io = rio.clone();
        }
        attr = Arc::new(NFAttributes { connectorType: cty.clone(), parallelism: par.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: io.clone(), isFinal: fin.clone(), isRedeclare: redecl.clone(), isReplaceable: repl.clone(), isResizable: resize.clone() });
    }
    Ok(attr)
}

pub fn mergeRedeclaredClassPrefixes(mut origPrefs: Arc<Class::Prefixes::Prefixes>, mut redeclPrefs: Arc<Class::Prefixes::Prefixes>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<Class::Prefixes::Prefixes>> {
    let mut prefs: Arc<Class::Prefixes::Prefixes>;
    let mut enc: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    let mut par: SCode::Partial = SCode::Partial::NOT_PARTIAL;
    let mut fin: SCode::Final = SCode::Final::FINAL;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut rio: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let mut repl: Arc<SCode::Replaceable> = Arc::new(SCode::Replaceable::NOT_REPLACEABLE);
    if referenceEq(&origPrefs.clone(),&Class::DEFAULT_PREFIXES.clone()) {
        prefs = redeclPrefs.clone();
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(origPrefs.clone()) {
            Deref @ Class::Prefixes::PREFIXES { innerOuter: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        io = __pa0.clone();
        let (__pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(redeclPrefs.clone()) {
            Deref @ Class::Prefixes::PREFIXES { encapsulatedPrefix: __pa1, partialPrefix: __pa2, finalPrefix: __pa3, innerOuter: __pa4, replaceablePrefix: __pa5 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        enc = __pa1.clone();
        par = __pa2.clone();
        fin = __pa3.clone();
        rio = __pa4.clone();
        repl = __pa5.clone();
        io = (match (io.clone(), rio.clone()) {
        (Absyn::InnerOuter::NOT_INNER_OUTER { .. }, _) => rio.clone(),
        (_, Absyn::InnerOuter::NOT_INNER_OUTER { .. }) => io.clone(),
        (Absyn::InnerOuter::INNER { .. }, Absyn::InnerOuter::INNER { .. }) => io.clone(),
        (Absyn::InnerOuter::OUTER { .. }, Absyn::InnerOuter::OUTER { .. }) => io.clone(),
        (Absyn::InnerOuter::INNER_OUTER { .. }, Absyn::InnerOuter::INNER_OUTER { .. }) => io.clone(),
        _ => {
            printRedeclarePrefixError(node.clone(), (Prefixes::innerOuterString(Prefixes::innerOuterFromSCode(rio.clone())?)).clone(), (Prefixes::innerOuterString(Prefixes::innerOuterFromSCode(io.clone())?)).clone())?;
            bail!("fail")
        },
    });
        prefs = Arc::new(Class::Prefixes::Prefixes { encapsulatedPrefix: enc.clone(), partialPrefix: par.clone(), finalPrefix: fin.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() });
    }
    Ok(prefs)
}

pub fn printRedeclarePrefixError(mut node: Arc<InstNode::InstNode>, mut prefix1: ArcStr, mut prefix2: ArcStr) -> Result<()> {
    Error::addSourceMessageAndFail(Error::REDECLARE_MISMATCHED_PREFIX.clone(), list![(prefix1.clone()).clone(), (InstNode::name(node.clone())?).clone(), (prefix2.clone()).clone()], InstNode::info(node.clone())?)?;
    Ok(())
}

pub fn checkDeclaredComponentAttributes(mut attr: Arc<NFAttributes>, mut parentRestriction: Arc<Restriction::NFRestriction>, mut component: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes> = attr;
    let () = (::match_deref::match_deref! { match &(parentRestriction.clone()) {
        Deref @ Restriction::CONNECTOR { .. } => {
            assertNotInnerOuter(attr.innerOuter.clone(), component.clone(), parentRestriction.clone())?;
            if var_field!((*parentRestriction).isExpandable, Restriction::NFRestriction::CONNECTOR).clone() {
                assertNotFlowStream(attr.connectorType.clone(), component.clone(), parentRestriction.clone())?;
                assign_field!(attr.connectorType = intBitOr(attr.connectorType.clone(), ConnectorType::POTENTIALLY_PRESENT.clone()));
            }
            ()
        },
        Deref @ Restriction::RECORD { .. } => {
            assertNotInputOutput(attr.direction.clone(), component.clone(), parentRestriction.clone())?;
            assertNotInnerOuter(attr.innerOuter.clone(), component.clone(), parentRestriction.clone())?;
            assertNotFlowStream(attr.connectorType.clone(), component.clone(), parentRestriction.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attr)
}

pub fn invalidComponentPrefixError(mut prefix: ArcStr, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    Error::addSourceMessage(Error::INVALID_COMPONENT_PREFIX.clone(), list![(prefix.clone()).clone(), (InstNode::name(node.clone())?).clone(), (Restriction::toString(restriction.clone())).clone()], InstNode::info(node.clone())?)?;
    Ok(())
}

pub fn assertNotInputOutput(mut dir: Prefixes::Direction, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    if dir.clone() != Direction::NONE.clone() {
        invalidComponentPrefixError((Prefixes::directionString(dir.clone())).clone(), node.clone(), restriction.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn assertNotInnerOuter(mut io: Prefixes::InnerOuter, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    if io.clone() != InnerOuter::NOT_INNER_OUTER.clone() {
        invalidComponentPrefixError((Prefixes::innerOuterString(io.clone())).clone(), node.clone(), restriction.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn assertNotFlowStream(mut cty: i32, mut node: Arc<InstNode::InstNode>, mut restriction: Arc<Restriction::NFRestriction>) -> Result<()> {
    if Prefixes::ConnectorType::isFlowOrStream(cty.clone()) {
        invalidComponentPrefixError((Prefixes::ConnectorType::toString(cty.clone())).clone(), node.clone(), restriction.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn updateComponentConnectorType(mut attributes: Arc<NFAttributes>, mut restriction: Arc<Restriction::NFRestriction>, mut context: i32, mut component: Arc<InstNode::InstNode>) -> Result<Arc<NFAttributes>> {
    let mut attributes: Arc<NFAttributes> = attributes;
    let mut cty: i32 = attributes.connectorType.clone();
    if Prefixes::ConnectorType::isConnectorType(cty.clone()) {
        if Restriction::isConnector(restriction.clone()) {
            if attributes.variability.clone() < Variability::DISCRETE.clone() && !(InstContext::inRelaxed(context.clone())) && !(Class::isBuiltin(InstNode::getClass(component.clone())?)?) {
                Error::addSourceMessage(Error::INVALID_CONNECTOR_VARIABILITY.clone(), list![(Prefixes::variabilityString(attributes.variability.clone())?).clone(), (InstNode::name(component.clone())?).clone()], InstNode::info(component.clone())?)?;
                bail!("fail");
            }
            if Restriction::isExpandableConnector(restriction.clone()) {
                cty = Prefixes::ConnectorType::setPresent(cty.clone());
            } else {
                cty = intBitAnd(cty.clone(), intBitNot(ConnectorType::EXPANDABLE.clone()));
            }
        } else {
            cty = intBitAnd(cty.clone(), intBitNot(intBitOr(ConnectorType::CONNECTOR.clone(), ConnectorType::EXPANDABLE.clone())));
        }
        if !(Prefixes::ConnectorType::isFlowOrStream(cty.clone())) {
            cty = Prefixes::ConnectorType::setPotential(cty.clone());
        }
        if cty.clone() != attributes.connectorType.clone() {
            assign_field!(attributes.connectorType = cty.clone());
        }
    } else if Prefixes::ConnectorType::isFlowOrStream(cty.clone()) && !(InstContext::inRedeclared(context.clone())) {
        Error::addStrictMessage(Error::CONNECTOR_PREFIX_OUTSIDE_CONNECTOR.clone(), list![(Prefixes::ConnectorType::toString(cty.clone())).clone()], InstNode::info(component.clone())?)?;
        assign_field!(attributes.connectorType = Prefixes::ConnectorType::unsetFlowStream(cty.clone()));
    }
    Ok(attributes)
}

pub fn updateClassConnectorType(mut res: Arc<Restriction::NFRestriction>, mut attrs: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attrs: Arc<NFAttributes> = attrs;
    if Restriction::isExpandableConnector(res.clone()) {
        assign_field!(attrs.connectorType = Prefixes::ConnectorType::setExpandable(attrs.connectorType.clone()));
    } else if Restriction::isConnector(res.clone()) {
        assign_field!(attrs.connectorType = Prefixes::ConnectorType::setConnector(attrs.connectorType.clone()));
    }
    attrs
}

pub fn updateVariability(mut attr: Arc<NFAttributes>, mut cls: Arc<Class::NFClass>, mut clsNode: Arc<InstNode::InstNode>, mut compNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<NFAttributes>> {
    let mut attr: Arc<NFAttributes> = attr;
    let mut var: Prefixes::Variability = attr.variability.clone();
    if referenceEq(&attr.clone(),&DEFAULT_ATTR().clone()) && InstNode::isDiscreteClass(clsNode.clone())? {
        attr = IMPL_DISCRETE_ATTR().clone();
    } else if var.clone() == Variability::CONTINUOUS.clone() && InstNode::isDiscreteClass(clsNode.clone())? {
        assign_field!(attr.variability = Variability::IMPLICITLY_DISCRETE.clone());
    } else if var.clone() < Variability::CONTINUOUS.clone() && InstContext::inFunction(context.clone()) && attr.direction.clone() != Direction::NONE.clone() && SCodeUtil::isEmptyMod((InstNode::getAnnotation((literal!("__OpenModelica_functionVariability")).clone(), compNode.clone())?).0) {
        assign_field!(attr.variability = Variability::CONTINUOUS.clone());
    } else if var.clone() == Variability::PARAMETER.clone() && !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) && Util::getOptionOrDefault(SCodeUtil::lookupBooleanAnnotationMod((InstNode::getAnnotation((literal!("__OpenModelica_resizable")).clone(), compNode.clone())?).0), false) {
        assign_field!(
            attr.variability = Variability::NON_STRUCTURAL_PARAMETER.clone(),
            attr.isResizable = true
        );
    }
    Ok(attr)
}

pub fn setConnectorType(mut cty: i32, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.connectorType = cty.clone());
    attr
}

pub fn setVariability(mut var: Prefixes::Variability, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.variability = var.clone());
    attr
}

pub fn setDirection(mut dir: Prefixes::Direction, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.direction = dir.clone());
    attr
}

pub fn setInnerOuter(mut io: Prefixes::InnerOuter, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.innerOuter = io.clone());
    attr
}

pub fn setFinal(mut fin: bool, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.isFinal = fin.clone());
    attr
}

pub fn setRedeclare(mut redecl: bool, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.isRedeclare = redecl.clone());
    attr
}

pub fn setReplaceable(mut repl: Arc<Prefixes::Replaceable>, mut attr: Arc<NFAttributes>) -> Arc<NFAttributes> {
    let mut attr: Arc<NFAttributes> = attr;
    assign_field!(attr.isReplaceable = repl.clone());
    attr
}

pub fn toDAE(mut ina: Arc<NFAttributes>, mut vis: Prefixes::Visibility) -> Result<Arc<DAE::Attributes>> {
    let mut outa: Arc<DAE::Attributes>;
    outa = Arc::new(DAE::Attributes { connectorType: Prefixes::ConnectorType::toDAE(ina.connectorType.clone()), parallelism: parallelismToSCode(ina.parallelism.clone())?, variability: variabilityToSCode(ina.variability.clone()), direction: directionToAbsyn(ina.direction.clone()), innerOuter: innerOuterToAbsyn(ina.innerOuter.clone())?, visibility: visibilityToSCode(vis.clone()) });
    Ok(outa)
}

pub fn toString(mut attr: Arc<NFAttributes>, mut ty: Arc<NFType::NFType>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (attr.isRedeclare.clone()) {literal!("redeclare ")} else {literal!("")}); __mm_s.push_str(&*if (attr.isFinal.clone()) {literal!("final ")} else {literal!("")}); __mm_s.push_str(&*Prefixes::unparseInnerOuter(attr.innerOuter.clone())); __mm_s.push_str(&*Prefixes::unparseReplaceable(attr.isReplaceable.clone())); __mm_s.push_str(&*Prefixes::unparseParallelism(attr.parallelism.clone())); __mm_s.push_str(&*Prefixes::ConnectorType::unparse(attr.connectorType.clone())); __mm_s.push_str(&*Prefixes::unparseVariability(attr.variability.clone(), ty.clone())); __mm_s.push_str(&*Prefixes::unparseDirection(attr.direction.clone())); ArcStr::from(__mm_s) }).clone();
    r#str
}

pub fn toFlatStream(mut attr: Arc<NFAttributes>, mut ty: Arc<NFType::NFType>, mut s: IOStream::IOStream, mut isTopLevel: bool) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    s = IOStream::append(s.clone(), (Prefixes::unparseVariability(attr.variability.clone(), ty.clone())).clone())?;
    if isTopLevel.clone() {
        s = IOStream::append(s.clone(), (Prefixes::unparseDirection(attr.direction.clone())).clone())?;
    }
    Ok(s)
}


