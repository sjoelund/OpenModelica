// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::DAE;
use crate::NFInstNode::InstNode;
use crate::NFType as Type;
use crate::SCode;

pub enum AccessLevel {
    HIDE,
    ICON,
    DOCUMENTATION,
    DIAGRAM,
    NON_PACKAGE_TEXT,
    NON_PACKAGE_DUPLICATE,
    PACKAGE_TEXT,
    PACKAGE_DUPLICATE,
}

pub mod ConnectorType {
    use super::*;
    pub type Type = i32;

    pub fn fromSCode(scodeCty: SCode::ConnectorType) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn isAugmented(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isConnector(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isConnectorType(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isExpandable(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isFlow(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isFlowOrStream(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isPotential(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isPotentiallyPresent(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isStream(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isUndeclared(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn isVirtual(cty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    pub fn merge(outerCty: Arc<Type::NFType>, innerCty: Arc<Type::NFType>, node: Arc<InstNode::InstNode>, isClass: bool) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn setConnector(cty: Arc<Type::NFType>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn setExpandable(cty: Arc<Type::NFType>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn setPotential(cty: Arc<Type::NFType>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn setPresent(cty: Arc<Type::NFType>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn toDAE(cty: Arc<Type::NFType>) -> Arc<DAE::ConnectorType> {
        todo!()
    }

    pub fn toDebugString(cty: Arc<Type::NFType>) -> String {
        todo!()
    }

    pub fn toString(cty: Arc<Type::NFType>) -> String {
        todo!()
    }

    pub fn unparse(cty: Arc<Type::NFType>) -> String {
        todo!()
    }

    pub fn unsetFlowStream(cty: Arc<Type::NFType>) -> Arc<Type::NFType> {
        todo!()
    }

}

pub enum Direction {
    NONE,
    INPUT,
    OUTPUT,
}

pub enum InnerOuter {
    NOT_INNER_OUTER,
    INNER,
    OUTER,
    INNER_OUTER,
}

pub enum Parallelism {
    NON_PARALLEL,
    GLOBAL,
    LOCAL,
}

pub enum Purity {
    PURE,
    IMPURE,
}

pub enum Replaceable {
    REPLACEABLE {
        constrainingClass: Option<Arc<InstNode::InstNode>>,
    },
    NOT_REPLACEABLE,
}
pub use Replaceable::*;

pub enum Variability {
    CONSTANT,
    STRUCTURAL_PARAMETER,
    PARAMETER,
    NON_STRUCTURAL_PARAMETER,
    DISCRETE,
    IMPLICITLY_DISCRETE,
    CONTINUOUS,
}

pub enum Visibility {
    PUBLIC,
    PROTECTED,
}

pub fn accessLevelFromAbsyn(exp: Arc<Absyn::Exp>) -> Option<AccessLevel> {
    todo!()
}

pub fn directionFromSCode(scodeDir: Absyn::Direction) -> Direction {
    todo!()
}

pub fn directionString(dir: Direction) -> String {
    todo!()
}

pub fn directionToAbsyn(dir: Direction) -> Absyn::Direction {
    todo!()
}

pub fn directionToDAE(dir: Direction) -> DAE::VarDirection {
    todo!()
}

pub fn effectiveVariability(inVar: Variability) -> Variability {
    todo!()
}

pub fn innerOuterFromSCode(scodeIO: Absyn::InnerOuter) -> InnerOuter {
    todo!()
}

pub fn innerOuterString(io: InnerOuter) -> String {
    todo!()
}

pub fn innerOuterToAbsyn(inIO: InnerOuter) -> Absyn::InnerOuter {
    todo!()
}

pub fn isReplaceable(repl: Arc<Replaceable>) -> bool {
    todo!()
}

pub fn mergeDirection(outerDir: Direction, innerDir: Direction, node: Arc<InstNode::InstNode>, allowSame: bool) -> Direction {
    todo!()
}

pub fn mergeParallelism(outerPar: Parallelism, innerPar: Parallelism, node: Arc<InstNode::InstNode>) -> Parallelism {
    todo!()
}

pub fn mergeVisibility(outerVis: Visibility, innerVis: Visibility) -> Visibility {
    todo!()
}

pub fn parallelismFromSCode(scodePar: SCode::Parallelism) -> Parallelism {
    todo!()
}

pub fn parallelismString(par: Parallelism) -> String {
    todo!()
}

pub fn parallelismToDAE(par: Parallelism) -> DAE::VarParallelism {
    todo!()
}

pub fn parallelismToSCode(par: Parallelism) -> SCode::Parallelism {
    todo!()
}

pub fn printPrefixError(outerPrefix: String, innerPrefix: String, node: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn purityMin(p1: Purity, p2: Purity) -> Purity {
    todo!()
}

pub fn purityString(purity: Purity) -> String {
    todo!()
}

pub fn replaceableString(repl: Arc<Replaceable>) -> String {
    todo!()
}

pub fn unparseDirection(dir: Direction) -> String {
    todo!()
}

pub fn unparseInnerOuter(io: InnerOuter) -> String {
    todo!()
}

pub fn unparseParallelism(par: Parallelism) -> String {
    todo!()
}

pub fn unparseReplaceable(repl: Arc<Replaceable>) -> String {
    todo!()
}

pub fn unparseVariability(var: Variability, ty: Arc<Type::NFType>) -> String {
    todo!()
}

pub fn unparseVisibility(vis: Visibility) -> String {
    todo!()
}

pub fn variabilityFromSCode(scodeVar: SCode::Variability) -> Variability {
    todo!()
}

pub fn variabilityMax(var1: Variability, var2: Variability) -> Variability {
    todo!()
}

pub fn variabilityMin(var1: Variability, var2: Variability) -> Variability {
    todo!()
}

pub fn variabilityString(var: Variability) -> String {
    todo!()
}

pub fn variabilityToDAE(var: Variability) -> DAE::VarKind {
    todo!()
}

pub fn variabilityToDAEConst(var: Variability) -> DAE::Const {
    todo!()
}

pub fn variabilityToSCode(var: Variability) -> SCode::Variability {
    todo!()
}

pub fn visibilityFromSCode(scodeVis: SCode::Visibility) -> Visibility {
    todo!()
}

pub fn visibilityString(vis: Visibility) -> String {
    todo!()
}

pub fn visibilityToDAE(vis: Visibility) -> DAE::VarVisibility {
    todo!()
}

pub fn visibilityToSCode(vis: Visibility) -> SCode::Visibility {
    todo!()
}

