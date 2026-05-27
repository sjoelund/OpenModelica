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
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;

pub mod ConnectorType {
    use super::*;
    pub type Type = i32;

    pub const NON_CONNECTOR: i32 = 0;

    pub const POTENTIAL: i32 = intBitLShift(1, 0);

    pub const FLOW: i32 = intBitLShift(1, 1);

    pub const STREAM: i32 = intBitLShift(1, 2);

    pub const POTENTIALLY_PRESENT: i32 = intBitLShift(1, 3);

    pub const VIRTUAL: i32 = intBitLShift(1, 4);

    pub const CONNECTOR: i32 = intBitLShift(1, 5);

    pub const EXPANDABLE: i32 = intBitLShift(1, 6);

    pub const AUGMENTED: i32 = intBitLShift(1, 7);

    // flow/stream
    pub const FLOW_STREAM_MASK: i32 = intBitOr(FLOW, STREAM);

    // potential/flow/stream
    pub const PREFIX_MASK: i32 = intBitOr(POTENTIAL, FLOW_STREAM_MASK);

    // Some kind of connector, where anything inside an expandable connector also counts.
    pub const CONNECTOR_MASK: i32 = intBitOr(CONNECTOR, intBitOr(EXPANDABLE, POTENTIALLY_PRESENT));

    // An element in an expandable connector.
    pub const UNDECLARED_MASK: i32 = intBitOr(VIRTUAL, POTENTIALLY_PRESENT);

    pub fn fromSCode(mut scodeCty: SCode::ConnectorType) -> Result<Type> {
        let mut cty: Type = 0;
        cty = (match scodeCty.clone() {
        SCode::ConnectorType::POTENTIAL { .. } => 0,
        SCode::ConnectorType::FLOW { .. } => FLOW.clone(),
        SCode::ConnectorType::STREAM { .. } => STREAM.clone(),
        _ => bail!("match: no arm matched"),
    });
        Ok(cty)
    }

    pub fn toDAE(mut cty: Type) -> Arc<DAE::ConnectorType> {
        let mut dcty: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
        if intBitAnd(cty.clone(), POTENTIAL.clone()) > 0 {
            dcty = Arc::new(openmodelica_frontend_types::DAE::ConnectorType::POTENTIAL);
        } else if intBitAnd(cty.clone(), FLOW.clone()) > 0 {
            dcty = Arc::new(openmodelica_frontend_types::DAE::ConnectorType::FLOW);
        } else if intBitAnd(cty.clone(), STREAM.clone()) > 0 {
            dcty = Arc::new(DAE::ConnectorType::STREAM { associatedFlow: None });
        } else {
            dcty = Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR);
        }
        dcty
    }

    pub fn merge(mut outerCty: Type, mut innerCty: Type, mut node: Arc<InstNode::InstNode>, mut isClass: bool) -> Result<Type> {
        let mut cty: Type = 0;
        if intBitAnd(outerCty.clone(), FLOW_STREAM_MASK.clone()) > 0 && intBitAnd(innerCty.clone(), FLOW_STREAM_MASK.clone()) > 0 {
            printPrefixError((toString(outerCty.clone())).clone(), (toString(innerCty.clone())).clone(), node.clone())?;
        }
        cty = intBitOr(outerCty.clone(), innerCty.clone());
        Ok(cty)
    }

    pub fn isPotential(mut cty: Type) -> bool {
        let mut isPotential: bool = false;
        isPotential = intBitAnd(cty.clone(), POTENTIAL.clone()) > 0;
        isPotential
    }

    pub fn setPotential(mut cty: Type) -> Type {
        let mut cty: Type = cty;
        cty = intBitOr(cty.clone(), POTENTIAL.clone());
        cty
    }

    pub fn isFlow(mut cty: Type) -> bool {
        let mut isFlow: bool = false;
        isFlow = intBitAnd(cty.clone(), FLOW.clone()) > 0;
        isFlow
    }

    pub fn isStream(mut cty: Type) -> bool {
        let mut isStream: bool = false;
        isStream = intBitAnd(cty.clone(), STREAM.clone()) > 0;
        isStream
    }

    pub fn isFlowOrStream(mut cty: Type) -> bool {
        let mut isFlowOrStream: bool = false;
        isFlowOrStream = intBitAnd(cty.clone(), FLOW_STREAM_MASK.clone()) > 0;
        isFlowOrStream
    }

    pub fn unsetFlowStream(mut cty: Type) -> Type {
        let mut cty: Type = cty;
        cty = intBitAnd(cty.clone(), intBitNot(FLOW_STREAM_MASK.clone()));
        cty
    }

    pub fn isConnector(mut cty: Type) -> bool {
        let mut isConnector: bool = false;
        isConnector = intBitAnd(cty.clone(), CONNECTOR.clone()) > 0;
        isConnector
    }

    pub fn setConnector(mut cty: Type) -> Type {
        let mut cty: Type = cty;
        cty = intBitOr(cty.clone(), CONNECTOR.clone());
        cty
    }

    pub fn isConnectorType(mut cty: Type) -> bool {
        let mut isConnector: bool = false;
        isConnector = intBitAnd(cty.clone(), CONNECTOR_MASK.clone()) > 0;
        isConnector
    }

    pub fn isExpandable(mut cty: Type) -> bool {
        let mut isExpandable: bool = false;
        isExpandable = intBitAnd(cty.clone(), EXPANDABLE.clone()) > 0;
        isExpandable
    }

    pub fn setExpandable(mut cty: Type) -> Type {
        let mut cty: Type = cty;
        cty = intBitOr(cty.clone(), EXPANDABLE.clone());
        cty
    }

    pub fn isUndeclared(mut cty: Type) -> bool {
        let mut isExpandableElement: bool = false;
        isExpandableElement = intBitAnd(cty.clone(), UNDECLARED_MASK.clone()) > 0;
        isExpandableElement
    }

    pub fn isVirtual(mut cty: Type) -> bool {
        let mut isVirtual: bool = false;
        isVirtual = intBitAnd(cty.clone(), VIRTUAL.clone()) > 0;
        isVirtual
    }

    pub fn isPotentiallyPresent(mut cty: Type) -> bool {
        let mut isPotentiallyPresent: bool = false;
        isPotentiallyPresent = intBitAnd(cty.clone(), POTENTIALLY_PRESENT.clone()) > 0;
        isPotentiallyPresent
    }

    pub fn setPresent(mut cty: Type) -> Type {
        let mut cty: Type = cty;
        cty = intBitAnd(cty.clone(), intBitNot(POTENTIALLY_PRESENT.clone()));
        cty
    }

    pub fn isAugmented(mut cty: Type) -> bool {
        let mut augmented: bool = false;
        augmented = intBitAnd(cty.clone(), AUGMENTED.clone()) > 0;
        augmented
    }

    pub fn toString(mut cty: Type) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        if intBitAnd(cty.clone(), FLOW.clone()) > 0 {
            r#str = (literal!("flow")).clone();
        } else if intBitAnd(cty.clone(), STREAM.clone()) > 0 {
            r#str = (literal!("stream")).clone();
        } else if intBitAnd(cty.clone(), EXPANDABLE.clone()) > 0 {
            r#str = (literal!("expandable")).clone();
        } else {
            r#str = (literal!("")).clone();
        }
        r#str
    }

    pub fn unparse(mut cty: Type) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        if intBitAnd(cty.clone(), FLOW.clone()) > 0 {
            r#str = (literal!("flow ")).clone();
        } else if intBitAnd(cty.clone(), STREAM.clone()) > 0 {
            r#str = (literal!("stream ")).clone();
        } else {
            r#str = (literal!("")).clone();
        }
        r#str
    }

    pub fn toDebugString(mut cty: Type) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        if intBitAnd(cty.clone(), POTENTIAL.clone()) > 0 {
            strl = cons((literal!("potential")).clone(), strl.clone());
        }
        if intBitAnd(cty.clone(), FLOW.clone()) > 0 {
            strl = cons((literal!("flow")).clone(), strl.clone());
        }
        if intBitAnd(cty.clone(), STREAM.clone()) > 0 {
            strl = cons((literal!("stream")).clone(), strl.clone());
        }
        if intBitAnd(cty.clone(), POTENTIALLY_PRESENT.clone()) > 0 {
            strl = cons((literal!("potentially present")).clone(), strl.clone());
        }
        if intBitAnd(cty.clone(), VIRTUAL.clone()) > 0 {
            strl = cons((literal!("virtual")).clone(), strl.clone());
        }
        if intBitAnd(cty.clone(), CONNECTOR.clone()) > 0 {
            strl = cons((literal!("connector")).clone(), strl.clone());
        }
        if intBitAnd(cty.clone(), EXPANDABLE.clone()) > 0 {
            strl = cons((literal!("expandable")).clone(), strl.clone());
        }
        r#str = stringDelimitList(strl.clone(), (literal!(" ")).clone());
        r#str
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Parallelism {
    NON_PARALLEL = 1,
    GLOBAL = 2,
    LOCAL = 3,
}
impl PartialOrd for Parallelism {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Parallelism {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Parallelism {
    fn default() -> Self { Self::NON_PARALLEL }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Variability {
    CONSTANT = 1,
    STRUCTURAL_PARAMETER = 2,
    PARAMETER = 3,
    NON_STRUCTURAL_PARAMETER = 4,
    DISCRETE = 5,
    IMPLICITLY_DISCRETE = 6,
    CONTINUOUS = 7,
}
impl PartialOrd for Variability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Variability {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Variability {
    fn default() -> Self { Self::CONSTANT }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Purity {
    PURE = 1,
    IMPURE = 2,
}
impl PartialOrd for Purity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Purity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Purity {
    fn default() -> Self { Self::PURE }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Direction {
    NONE = 1,
    INPUT = 2,
    OUTPUT = 3,
}
impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Direction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Direction {
    fn default() -> Self { Self::NONE }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum InnerOuter {
    NOT_INNER_OUTER = 1,
    INNER = 2,
    OUTER = 3,
    INNER_OUTER = 4,
}
impl PartialOrd for InnerOuter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for InnerOuter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for InnerOuter {
    fn default() -> Self { Self::NOT_INNER_OUTER }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Visibility {
    PUBLIC = 1,
    PROTECTED = 2,
}
impl PartialOrd for Visibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Visibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Visibility {
    fn default() -> Self { Self::PUBLIC }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum AccessLevel {
    HIDE = 1,
    ICON = 2,
    DOCUMENTATION = 3,
    DIAGRAM = 4,
    NON_PACKAGE_TEXT = 5,
    NON_PACKAGE_DUPLICATE = 6,
    PACKAGE_TEXT = 7,
    PACKAGE_DUPLICATE = 8,
}
impl PartialOrd for AccessLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for AccessLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Replaceable {
    REPLACEABLE {
        constrainingClass: Option<Arc<InstNode::InstNode>>,
    },
    NOT_REPLACEABLE,
}
impl Default for Replaceable {
    fn default() -> Self { Self::NOT_REPLACEABLE }
}
pub use self::Replaceable::{REPLACEABLE,NOT_REPLACEABLE};

pub fn parallelismFromSCode(mut scodePar: SCode::Parallelism) -> Result<Parallelism> {
    let mut par: Parallelism = Parallelism::NON_PARALLEL;
    par = (match scodePar.clone() {
        SCode::Parallelism::PARGLOBAL { .. } => Parallelism::GLOBAL.clone(),
        SCode::Parallelism::PARLOCAL { .. } => Parallelism::LOCAL.clone(),
        SCode::Parallelism::NON_PARALLEL { .. } => Parallelism::NON_PARALLEL.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(par)
}

pub fn parallelismToSCode(mut par: Parallelism) -> Result<SCode::Parallelism> {
    let mut scodePar: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    scodePar = (match par.clone() {
        Parallelism::GLOBAL => openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL,
        Parallelism::LOCAL { .. } => openmodelica_frontend_types::SCode::Parallelism::PARLOCAL,
        Parallelism::NON_PARALLEL => openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
        _ => bail!("match: no arm matched"),
    });
    Ok(scodePar)
}

pub fn parallelismToDAE(mut par: Parallelism) -> Result<DAE::VarParallelism> {
    let mut dpar: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    dpar = (match par.clone() {
        Parallelism::GLOBAL => openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL,
        Parallelism::LOCAL { .. } => openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL,
        Parallelism::NON_PARALLEL => openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        _ => bail!("match: no arm matched"),
    });
    Ok(dpar)
}

pub fn parallelismString(mut par: Parallelism) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match par.clone() {
        Parallelism::GLOBAL => literal!("parglobal"),
        Parallelism::LOCAL { .. } => literal!("parlocal"),
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn unparseParallelism(mut par: Parallelism) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match par.clone() {
        Parallelism::GLOBAL => literal!("parglobal "),
        Parallelism::LOCAL { .. } => literal!("parlocal "),
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn mergeParallelism(mut outerPar: Parallelism, mut innerPar: Parallelism, mut node: Arc<InstNode::InstNode>) -> Result<Parallelism> {
    let mut par: Parallelism = Parallelism::NON_PARALLEL;
    if outerPar.clone() == Parallelism::NON_PARALLEL.clone() {
        par = innerPar.clone();
    } else if innerPar.clone() == Parallelism::NON_PARALLEL.clone() {
        par = outerPar.clone();
    } else if innerPar.clone() == outerPar.clone() {
        par = innerPar.clone();
    } else {
        printPrefixError((parallelismString(outerPar.clone())).clone(), (parallelismString(innerPar.clone())).clone(), node.clone())?;
    }
    Ok(par)
}

pub fn variabilityFromSCode(mut scodeVar: SCode::Variability) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (match scodeVar.clone() {
        SCode::Variability::CONST { .. } => Variability::CONSTANT.clone(),
        SCode::Variability::PARAM { .. } => Variability::PARAMETER.clone(),
        SCode::Variability::DISCRETE { .. } => Variability::DISCRETE.clone(),
        SCode::Variability::VAR { .. } => Variability::CONTINUOUS.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(var)
}

pub fn variabilityToSCode(mut var: Variability) -> SCode::Variability {
    let mut scodeVar: SCode::Variability = SCode::Variability::CONST;
    scodeVar = (match var.clone() {
        Variability::CONSTANT => openmodelica_frontend_types::SCode::Variability::CONST,
        Variability::STRUCTURAL_PARAMETER => openmodelica_frontend_types::SCode::Variability::PARAM,
        Variability::PARAMETER { .. } => openmodelica_frontend_types::SCode::Variability::PARAM,
        Variability::NON_STRUCTURAL_PARAMETER => openmodelica_frontend_types::SCode::Variability::PARAM,
        Variability::DISCRETE => openmodelica_frontend_types::SCode::Variability::DISCRETE,
        _ => openmodelica_frontend_types::SCode::Variability::VAR,
    });
    scodeVar
}

pub fn variabilityToDAE(mut var: Variability) -> DAE::VarKind {
    let mut varKind: DAE::VarKind = DAE::VarKind::CONST;
    varKind = (match var.clone() {
        Variability::CONSTANT => openmodelica_frontend_types::DAE::VarKind::CONST,
        Variability::STRUCTURAL_PARAMETER => openmodelica_frontend_types::DAE::VarKind::PARAM,
        Variability::PARAMETER { .. } => openmodelica_frontend_types::DAE::VarKind::PARAM,
        Variability::NON_STRUCTURAL_PARAMETER => openmodelica_frontend_types::DAE::VarKind::PARAM,
        Variability::DISCRETE => openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        _ => openmodelica_frontend_types::DAE::VarKind::VARIABLE,
    });
    varKind
}

pub fn variabilityToDAEConst(mut var: Variability) -> DAE::Const {
    let mut r#const: DAE::Const = DAE::Const::C_CONST;
    r#const = (match var.clone() {
        Variability::CONSTANT => openmodelica_frontend_types::DAE::Const::C_CONST,
        Variability::STRUCTURAL_PARAMETER => openmodelica_frontend_types::DAE::Const::C_PARAM,
        Variability::PARAMETER { .. } => openmodelica_frontend_types::DAE::Const::C_PARAM,
        Variability::NON_STRUCTURAL_PARAMETER => openmodelica_frontend_types::DAE::Const::C_PARAM,
        _ => openmodelica_frontend_types::DAE::Const::C_VAR,
    });
    r#const
}

pub fn variabilityString(mut var: Variability) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match var.clone() {
        Variability::CONSTANT => literal!("constant"),
        Variability::STRUCTURAL_PARAMETER => literal!("parameter"),
        Variability::PARAMETER { .. } => literal!("parameter"),
        Variability::NON_STRUCTURAL_PARAMETER => literal!("parameter"),
        Variability::DISCRETE => literal!("discrete"),
        Variability::IMPLICITLY_DISCRETE => literal!("discrete"),
        Variability::CONTINUOUS { .. } => literal!("continuous"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn unparseVariability(mut var: Variability, mut ty: Arc<Type::NFType>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match var.clone() {
        Variability::CONSTANT => literal!("constant "),
        Variability::STRUCTURAL_PARAMETER => literal!("parameter "),
        Variability::PARAMETER { .. } => literal!("parameter "),
        Variability::NON_STRUCTURAL_PARAMETER => literal!("parameter "),
        Variability::DISCRETE => if (Type::isDiscrete(ty.clone())) {literal!("")} else {literal!("discrete ")},
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn variabilityMax(mut var1: Variability, mut var2: Variability) -> Variability {
    let mut var: Variability = if (var1.clone() > var2.clone()) {var1.clone()} else {var2.clone()};
    var
}

pub fn variabilityMin(mut var1: Variability, mut var2: Variability) -> Variability {
    let mut var: Variability = if (var1.clone() > var2.clone()) {var2.clone()} else {var1.clone()};
    var
}

pub fn effectiveVariability(mut inVar: Variability) -> Variability {
    let mut outVar: Variability = Variability::CONSTANT;
    outVar = (match inVar.clone() {
        Variability::STRUCTURAL_PARAMETER => Variability::PARAMETER.clone(),
        Variability::NON_STRUCTURAL_PARAMETER => Variability::PARAMETER.clone(),
        Variability::IMPLICITLY_DISCRETE => Variability::DISCRETE.clone(),
        _ => inVar.clone(),
    });
    outVar
}

pub fn purityString(mut purity: Purity) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match purity.clone() {
        Purity::PURE => literal!("pure"),
        Purity::IMPURE => literal!("impure"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn purityMin(mut p1: Purity, mut p2: Purity) -> Purity {
    let mut p: Purity = if (p1.clone() == Purity::IMPURE.clone()) {p1.clone()} else {p2.clone()};
    p
}

pub fn directionFromSCode(mut scodeDir: Absyn::Direction) -> Direction {
    let mut dir: Direction = Direction::NONE;
    dir = (match scodeDir.clone() {
        Absyn::Direction::INPUT { .. } => Direction::INPUT.clone(),
        Absyn::Direction::OUTPUT { .. } => Direction::OUTPUT.clone(),
        _ => Direction::NONE.clone(),
    });
    dir
}

pub fn directionToDAE(mut dir: Direction) -> DAE::VarDirection {
    let mut ddir: DAE::VarDirection = DAE::VarDirection::BIDIR;
    ddir = (match dir.clone() {
        Direction::INPUT => openmodelica_frontend_types::DAE::VarDirection::INPUT,
        Direction::OUTPUT => openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        _ => openmodelica_frontend_types::DAE::VarDirection::BIDIR,
    });
    ddir
}

pub fn directionToAbsyn(mut dir: Direction) -> Absyn::Direction {
    let mut adir: Absyn::Direction = Absyn::Direction::BIDIR;
    adir = (match dir.clone() {
        Direction::INPUT => openmodelica_ast::Absyn::Direction::INPUT,
        Direction::OUTPUT => openmodelica_ast::Absyn::Direction::OUTPUT,
        _ => openmodelica_ast::Absyn::Direction::BIDIR,
    });
    adir
}

pub fn directionString(mut dir: Direction) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match dir.clone() {
        Direction::INPUT => literal!("input"),
        Direction::OUTPUT => literal!("output"),
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn unparseDirection(mut dir: Direction) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match dir.clone() {
        Direction::INPUT => literal!("input "),
        Direction::OUTPUT => literal!("output "),
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn mergeDirection(mut outerDir: Direction, mut innerDir: Direction, mut node: Arc<InstNode::InstNode>, mut allowSame: bool) -> Result<Direction> {
    let mut dir: Direction = Direction::NONE;
    if outerDir.clone() == Direction::NONE.clone() {
        dir = innerDir.clone();
    } else if innerDir.clone() == Direction::NONE.clone() {
        dir = outerDir.clone();
    } else if allowSame.clone() && outerDir.clone() == innerDir.clone() {
        dir = innerDir.clone();
    } else {
        printPrefixError((directionString(outerDir.clone())).clone(), (directionString(innerDir.clone())).clone(), node.clone())?;
    }
    Ok(dir)
}

pub fn innerOuterFromSCode(mut scodeIO: Absyn::InnerOuter) -> Result<InnerOuter> {
    let mut io: InnerOuter = InnerOuter::NOT_INNER_OUTER;
    io = (match scodeIO.clone() {
        Absyn::InnerOuter::NOT_INNER_OUTER => InnerOuter::NOT_INNER_OUTER.clone(),
        Absyn::InnerOuter::INNER => InnerOuter::INNER.clone(),
        Absyn::InnerOuter::OUTER => InnerOuter::OUTER.clone(),
        Absyn::InnerOuter::INNER_OUTER => InnerOuter::INNER_OUTER.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(io)
}

pub fn innerOuterToAbsyn(mut inIO: InnerOuter) -> Result<Absyn::InnerOuter> {
    let mut outIO: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    outIO = (match inIO.clone() {
        InnerOuter::NOT_INNER_OUTER => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        InnerOuter::INNER => openmodelica_ast::Absyn::InnerOuter::INNER,
        InnerOuter::OUTER { .. } => openmodelica_ast::Absyn::InnerOuter::OUTER,
        InnerOuter::INNER_OUTER => openmodelica_ast::Absyn::InnerOuter::INNER_OUTER,
        _ => bail!("match: no arm matched"),
    });
    Ok(outIO)
}

pub fn innerOuterString(mut io: InnerOuter) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match io.clone() {
        InnerOuter::INNER => literal!("inner"),
        InnerOuter::OUTER { .. } => literal!("outer"),
        InnerOuter::INNER_OUTER => literal!("inner outer"),
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn unparseInnerOuter(mut io: InnerOuter) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match io.clone() {
        InnerOuter::INNER => literal!("inner "),
        InnerOuter::OUTER { .. } => literal!("outer "),
        InnerOuter::INNER_OUTER => literal!("inner outer "),
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn visibilityFromSCode(mut scodeVis: SCode::Visibility) -> Visibility {
    let mut vis: Visibility = Visibility::PUBLIC;
    vis = (match scodeVis.clone() {
        SCode::Visibility::PUBLIC { .. } => Visibility::PUBLIC.clone(),
        _ => Visibility::PROTECTED.clone(),
    });
    vis
}

pub fn visibilityToDAE(mut vis: Visibility) -> DAE::VarVisibility {
    let mut dvis: DAE::VarVisibility = if (vis.clone() == Visibility::PUBLIC.clone()) {openmodelica_frontend_types::DAE::VarVisibility::PUBLIC} else {openmodelica_frontend_types::DAE::VarVisibility::PROTECTED};
    dvis
}

pub fn visibilityToSCode(mut vis: Visibility) -> SCode::Visibility {
    let mut scodeVis: SCode::Visibility = if (vis.clone() == Visibility::PUBLIC.clone()) {openmodelica_frontend_types::SCode::Visibility::PUBLIC} else {openmodelica_frontend_types::SCode::Visibility::PROTECTED};
    scodeVis
}

pub fn visibilityString(mut vis: Visibility) -> ArcStr {
    let mut r#str: ArcStr = if (vis.clone() == Visibility::PUBLIC.clone()) {literal!("public")} else {literal!("protected")};
    r#str
}

pub fn unparseVisibility(mut vis: Visibility) -> ArcStr {
    let mut r#str: ArcStr = if (vis.clone() == Visibility::PROTECTED.clone()) {literal!("protected ")} else {literal!("")};
    r#str
}

pub fn mergeVisibility(mut outerVis: Visibility, mut innerVis: Visibility) -> Visibility {
    let mut vis: Visibility = if (outerVis.clone() == Visibility::PROTECTED.clone()) {outerVis.clone()} else {innerVis.clone()};
    vis
}

pub fn isReplaceable(mut repl: Arc<Replaceable>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ Replaceable::REPLACEABLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn replaceableString(mut repl: Arc<Replaceable>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ Replaceable::REPLACEABLE { .. } => literal!("replaceable"),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    r#str
}

pub fn unparseReplaceable(mut repl: Arc<Replaceable>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ Replaceable::REPLACEABLE { .. } => literal!("replaceable "),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    r#str
}

pub fn printPrefixError(mut outerPrefix: ArcStr, mut innerPrefix: ArcStr, mut node: Arc<InstNode::InstNode>) -> Result<()> {
    Error::addSourceMessage(Error::INVALID_TYPE_PREFIX.clone(), list![(outerPrefix.clone()).clone(), (InstNode::typeName(node.clone())?).clone(), (InstNode::name(node.clone())?).clone(), (innerPrefix.clone()).clone()], InstNode::info(node.clone())?)?;
    bail!("fail");
    Ok(())
}

pub fn accessLevelFromAbsyn(mut exp: Arc<Absyn::Exp>) -> Option<AccessLevel> {
    let mut access: Option<AccessLevel> = None;
    let mut name: ArcStr = arcstr::literal!("");
    access = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. }, name: Deref @ "Access", .. } } => (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "hide" => Some(AccessLevel::HIDE.clone()),
        Deref @ "icon" => Some(AccessLevel::ICON.clone()),
        Deref @ "documentation" => Some(AccessLevel::DOCUMENTATION.clone()),
        Deref @ "diagram" => Some(AccessLevel::DIAGRAM.clone()),
        Deref @ "nonPackageText" => Some(AccessLevel::NON_PACKAGE_TEXT.clone()),
        Deref @ "nonPackageDuplicate" => Some(AccessLevel::NON_PACKAGE_DUPLICATE.clone()),
        Deref @ "packageText" => Some(AccessLevel::PACKAGE_TEXT.clone()),
        Deref @ "packageDuplicate" => Some(AccessLevel::PACKAGE_DUPLICATE.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    access
}

