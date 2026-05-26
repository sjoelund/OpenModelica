// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::AbsynUtil;
use crate::DAE;
use crate::ElementSource;
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::ScalarizeMode;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use crate::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::List;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub struct FLAT_SM_SEMANTICS {
    pub initStateRef: Arc<ComponentRef::NFComponentRef>,
    pub smComps: Vec<Arc<ComponentRef::NFComponentRef>>,
    pub t: metamodelica::List<Transition>,
    pub c: metamodelica::List<Arc<Expression::NFExpression>>,
    pub vars: metamodelica::List<Arc<Variable::NFVariable>>,
    pub knowns: metamodelica::List<Arc<Variable::NFVariable>>,
    pub eqs: metamodelica::List<Arc<Equation::NFEquation>>,
    pub pvars: metamodelica::List<Arc<Variable::NFVariable>>,
    pub peqs: metamodelica::List<Arc<Equation::NFEquation>>,
    pub enclosingState: Option<Arc<ComponentRef::NFComponentRef>>,
}

pub type FlatSmSemantics = FLAT_SM_SEMANTICS;

pub struct TRANSITION {
    pub from: i32,
    pub to: i32,
    pub condition: Arc<Expression::NFExpression>,
    pub immediate: bool,
    pub reset: bool,
    pub synchronize: bool,
    pub priority: i32,
}

pub type Transition = TRANSITION;

fn addHierarchicalPassThroughs(stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, allVariables: metamodelica::List<Arc<Variable::NFVariable>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>) -> () {
    todo!()
}

fn addPropagationEquations(inSem: FlatSmSemantics, enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>, enclosingSmSemOpt: Option<FlatSmSemantics>) -> FlatSmSemantics {
    todo!()
}

fn addStateActivationAndReset(inEq: Arc<Equation::NFEquation>, stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, crToStart: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>>, accEqs: metamodelica::List<Arc<Equation::NFEquation>>, accVars: metamodelica::List<Arc<Variable::NFVariable>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn addStateActivationAndReset1(inEq: Arc<Equation::NFEquation>, stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, crToStart: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>>, accEqs: metamodelica::List<Arc<Equation::NFEquation>>, accVars: metamodelica::List<Arc<Variable::NFVariable>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn basicFlatSmSemantics(initStateCref: Arc<ComponentRef::NFComponentRef>, stateCrefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, transitionEqs: metamodelica::List<Arc<Equation::NFEquation>>) -> FlatSmSemantics {
    todo!()
}

fn collectReachableStates(initCref: Arc<ComponentRef::NFComponentRef>, froms: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, tos: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> metamodelica::List<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

fn createActiveIndicator(stateRef: Arc<ComponentRef::NFComponentRef>, preRef: Arc<ComponentRef::NFComponentRef>, i: i32) -> (Arc<Variable::NFVariable>, Arc<Equation::NFEquation>) {
    todo!()
}

fn createResetEquation(lhsCref: Arc<ComponentRef::NFComponentRef>, lhsTy: Arc<Type::NFType>, stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, crToStart: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>>) -> Arc<Equation::NFEquation> {
    todo!()
}

fn createTandC(stateCrefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, transitionEqs: metamodelica::List<Arc<Equation::NFEquation>>) -> (metamodelica::List<Transition>, metamodelica::List<Arc<Expression::NFExpression>>) {
    todo!()
}

fn createTicksInStateIndicator(stateRef: Arc<ComponentRef::NFComponentRef>, stateActiveRef: Arc<ComponentRef::NFComponentRef>) -> (Arc<Variable::NFVariable>, Arc<Equation::NFEquation>) {
    todo!()
}

fn createTimeEnteredStateIndicator(stateRef: Arc<ComponentRef::NFComponentRef>, stateActiveRef: Arc<ComponentRef::NFComponentRef>) -> (Arc<Variable::NFVariable>, Arc<Equation::NFEquation>) {
    todo!()
}

fn createTimeInStateIndicator(stateRef: Arc<ComponentRef::NFComponentRef>, stateActiveRef: Arc<ComponentRef::NFComponentRef>, timeEnteredVar: Arc<Variable::NFVariable>) -> (Arc<Variable::NFVariable>, Arc<Equation::NFEquation>) {
    todo!()
}

fn crefHasPrefix(prefix: Arc<ComponentRef::NFComponentRef>, cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn elabXInStateOps(sem: FlatSmSemantics, enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>) -> FlatSmSemantics {
    todo!()
}

fn equationHasPrevious(eq: Arc<Equation::NFEquation>, varCref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn extractTransition(eq: Arc<Equation::NFEquation>, stateCrefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> Transition {
    todo!()
}

fn flatSmToDataFlow(initStateCref: Arc<ComponentRef::NFComponentRef>, stateCrefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, allEquations: metamodelica::List<Arc<Equation::NFEquation>>, allVariables: metamodelica::List<Arc<Variable::NFVariable>>, enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>, enclosingSmSemOpt: Option<FlatSmSemantics>, accEqs: metamodelica::List<Arc<Equation::NFEquation>>, accVars: metamodelica::List<Arc<Variable::NFVariable>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Variable::NFVariable>>, FlatSmSemantics) {
    todo!()
}

pub fn flatten(flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    todo!()
}

fn generateMergeEquation(outerVarCref: Arc<ComponentRef::NFComponentRef>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>, allVariables: metamodelica::List<Arc<Variable::NFVariable>>, accEqs: metamodelica::List<Arc<Equation::NFEquation>>, accVars: metamodelica::List<Arc<Variable::NFVariable>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn getDefaultStart(ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn getStartValue(var: Arc<Variable::NFVariable>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn groupStateMachines(equations: metamodelica::List<Arc<Equation::NFEquation>>, initialEquations: metamodelica::List<Arc<Equation::NFEquation>>) -> (metamodelica::List<Arc<ComponentRef::NFComponentRef>>, metamodelica::List<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) {
    todo!()
}

fn isEquationOfState(eq: Arc<Equation::NFEquation>, stateCref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn isInitialStateForGroup(eq: Arc<Equation::NFEquation>, initStateCref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn isOuterStateEquation(eq: Arc<Equation::NFEquation>, stateCrefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn isPreviousOfCref(e: Arc<Expression::NFExpression>, varCref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn isSimpleVarNamed(v: Arc<Variable::NFVariable>, name: String) -> bool {
    todo!()
}

fn isTransitionForGroup(eq: Arc<Equation::NFEquation>, stateCrefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

fn isTransitionOrInitialState(eq: Arc<Equation::NFEquation>) -> bool {
    todo!()
}

fn isVariableOfState(var: Arc<Variable::NFVariable>, stateCref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn makeCrefExp(cref: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeEq(lhs: Arc<Expression::NFExpression>, rhs: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Equation::NFEquation> {
    todo!()
}

fn makeIfExp(cond: Arc<Expression::NFExpression>, thenExp: Arc<Expression::NFExpression>, elseExp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeInitialCall() -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeMaxIntArrCall(exps: metamodelica::List<Arc<Expression::NFExpression>>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makePreviousCall(exp: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeRelationEq(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeRelationGt(exp1: Arc<Expression::NFExpression>, exp2: Arc<Expression::NFExpression>, ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeSMSPrefix(initStateCref: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

fn makeSampleTimeCall() -> Arc<Expression::NFExpression> {
    todo!()
}

fn makeVar(name: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, var: Variability) -> Arc<Variable::NFVariable> {
    todo!()
}

fn makeVarWithBinding(name: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, var: Variability, bindExp: Arc<Expression::NFExpression>) -> Arc<Variable::NFVariable> {
    todo!()
}

fn makeVarWithStart(name: Arc<ComponentRef::NFComponentRef>, ty: Arc<Type::NFType>, var: Variability, startExp: Arc<Expression::NFExpression>) -> Arc<Variable::NFVariable> {
    todo!()
}

fn priorityGt(t1: Transition, t2: Transition) -> bool {
    todo!()
}

fn qCref(name: String, ty: Arc<Type::NFType>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>, prefixCr: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

fn qualifyOuterVarCref(e: Arc<Expression::NFExpression>, parentPrefix: Arc<ComponentRef::NFComponentRef>, varCrefStrings: metamodelica::List<String>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn qualifyOuterVarExpr(e: Arc<Expression::NFExpression>, parentPrefix: Arc<ComponentRef::NFComponentRef>, varCrefStrings: metamodelica::List<String>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn smCompToDataFlow(stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, allEquations: metamodelica::List<Arc<Equation::NFEquation>>, allVariables: metamodelica::List<Arc<Variable::NFVariable>>, accEqs: metamodelica::List<Arc<Equation::NFEquation>>, accVars: metamodelica::List<Arc<Variable::NFVariable>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn smGroupDepthLt(g1: (metamodelica::List<Arc<ComponentRef::NFComponentRef>>, Arc<ComponentRef::NFComponentRef>), g2: (metamodelica::List<Arc<ComponentRef::NFComponentRef>>, Arc<ComponentRef::NFComponentRef>)) -> bool {
    todo!()
}

fn smeqsSubsXInState(eq: Arc<Equation::NFEquation>, initStateComp: Arc<ComponentRef::NFComponentRef>, i: i32, nTransitions: i32, substExp: Arc<Expression::NFExpression>, xInState: String) -> Arc<Equation::NFEquation> {
    todo!()
}

fn statePriorityGt(cr1: Arc<ComponentRef::NFComponentRef>, cr2: Arc<ComponentRef::NFComponentRef>, initCref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

fn subsActiveStateHelper(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn subsActiveStateInEq(eq: Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation> {
    todo!()
}

fn subsActiveStateInExp(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

fn subsPreviousCrefs(exp: Arc<Expression::NFExpression>, stateVarCrefs: metamodelica::List<Arc<ComponentRef::NFComponentRef>>, found: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

fn subsXInState(inExp: Arc<Expression::NFExpression>, funcName: String, substExp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

fn subsXInStateHelper(exp: Arc<Expression::NFExpression>, funcName: String, substExp: Arc<Expression::NFExpression>, found: bool) -> (Arc<Expression::NFExpression>, bool) {
    todo!()
}

fn transformWhenBranches(whenEq: Arc<Equation::NFEquation>, stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, crToStart: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>) -> (Arc<Equation::NFEquation>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn transformWhenBranchesAndAccumulate(whenEq: Arc<Equation::NFEquation>, stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, crToStart: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>, accEqs: metamodelica::List<Arc<Equation::NFEquation>>, accVars: metamodelica::List<Arc<Variable::NFVariable>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn transformWhenInnerAsPlain(whenEq: Arc<Equation::NFEquation>, stateCref: Arc<ComponentRef::NFComponentRef>, sem: FlatSmSemantics, crToStart: UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<ComponentRef::NFComponentRef>>, outerVarMap: UnorderedMap::UnorderedMap<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>, Arc<ComponentRef::NFComponentRef>>) -> (metamodelica::List<Arc<Equation::NFEquation>>, metamodelica::List<Arc<Variable::NFVariable>>) {
    todo!()
}

fn wrapInStateActivationConditional(inEq: Arc<Equation::NFEquation>, stateCref: Arc<ComponentRef::NFComponentRef>, isResetEquation: bool) -> Arc<Equation::NFEquation> {
    todo!()
}

