// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::DAE;
use crate::Dump;
use crate::NFAttributes as Attributes;
use crate::NFBinding::Binding;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComplexType as ComplexType;
use crate::NFComponent::Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::Pointer;
use openmodelica_util::UnorderedMap;

pub mod Annotations {
    use super::*;
    pub struct ANNOTATIONS {
        pub hideResult: bool,
        pub resizable: bool,
        pub optimizable: bool,
        pub optimizerExpression: Option<OptimizerExpression>,
    }

    pub type Annotations = ANNOTATIONS;
    pub fn create(comment: Arc<SCode::Comment>, attributes: Arc<Attributes::NFAttributes>) -> Arc<Annotations> {
        todo!()
    }

}

pub mod BackendInfo {
    use super::*;
    pub struct BACKEND_INFO {
        pub varKind: Arc<VariableKind::VariableKind>,
        pub attributes: Arc<VariableAttributes::VariableAttributes>,
        pub annotations: Arc<Annotations::Annotations>,
        pub var_pre: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        pub var_seed: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        pub var_pder: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        pub var_start: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        pub parent: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
    }

    pub type BackendInfo = BACKEND_INFO;
    pub fn toString(backendInfo: Arc<BackendInfo>) -> String {
        todo!()
    }

    pub fn map(binfo: Arc<BackendInfo>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<BackendInfo> {
        todo!()
    }

    pub fn getVarKind(binfo: Arc<BackendInfo>) -> Arc<VariableKind::VariableKind> {
        todo!()
    }

    pub fn setVarKind(binfo: Arc<BackendInfo>, varKind: Arc<VariableKind::VariableKind>) -> Arc<BackendInfo> {
        todo!()
    }

    pub fn setStateSelect(info: Arc<BackendInfo>, stateSelect_val: StateSelect, overwrite: bool) -> Arc<BackendInfo> {
        todo!()
    }

    pub fn setParent(binfo: Arc<BackendInfo>, parent: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<BackendInfo> {
        todo!()
    }

    pub type setPartner = fn(Arc<BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Arc<BackendInfo>;

    pub fn setVarPre() -> () {
        todo!()
    }

    pub fn setVarSeed() -> () {
        todo!()
    }

    pub fn setVarPDer() -> () {
        todo!()
    }

    pub fn setVarStart() -> () {
        todo!()
    }

    pub fn setAttributes(binfo: Arc<BackendInfo>, attributes: Arc<VariableAttributes::VariableAttributes>, annotations: Arc<Annotations::Annotations>) -> Arc<BackendInfo> {
        todo!()
    }

    pub fn setHideResult(binfo: Arc<BackendInfo>, hideResult: bool) -> Arc<BackendInfo> {
        todo!()
    }

    pub fn scalarize(binfo: Arc<BackendInfo>, length: i32) -> metamodelica::List<Arc<BackendInfo>> {
        todo!()
    }

}

pub struct DISTRIBUTION {
    pub name: Arc<Expression::NFExpression>,
    pub params: Arc<Expression::NFExpression>,
    pub paramNames: Arc<Expression::NFExpression>,
}

pub type Distribution = DISTRIBUTION;

pub enum OptimizerExpression {
    MAYER,
    LAGRANGE,
    PATH_CONSTRAINT,
    INITIAL_CONSTRAINT,
    FINAL_CONSTRAINT,
    INITIAL_TIME,
    FINAL_TIME,
}

pub enum StateSelect {
    NEVER,
    AVOID,
    DEFAULT,
    PREFER,
    ALWAYS,
}

pub enum TearingSelect {
    NEVER,
    AVOID,
    DEFAULT,
    PREFER,
    ALWAYS,
}

pub enum Uncertainty {
    GIVEN,
    SOUGHT,
    REFINE,
    PROPAGATE,
}

pub mod VariableAttributes {
    use super::*;
    pub enum VariableAttributes {
        VAR_ATTR_REAL {
            quantity: Option<Arc<Expression::NFExpression>>,
            unit: Option<Arc<Expression::NFExpression>>,
            displayUnit: Option<Arc<Expression::NFExpression>>,
            min: Option<Arc<Expression::NFExpression>>,
            max: Option<Arc<Expression::NFExpression>>,
            start: Option<Arc<Expression::NFExpression>>,
            fixed: Option<Arc<Expression::NFExpression>>,
            nominal: Option<Arc<Expression::NFExpression>>,
            stateSelect: Option<StateSelect>,
            tearingSelect: Option<TearingSelect>,
            uncertainty: Option<Uncertainty>,
            distribution: Option<Arc<Distribution>>,
            binding: Option<Arc<Expression::NFExpression>>,
            isProtected: Option<bool>,
            finalPrefix: Option<bool>,
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_INT {
            quantity: Option<Arc<Expression::NFExpression>>,
            min: Option<Arc<Expression::NFExpression>>,
            max: Option<Arc<Expression::NFExpression>>,
            start: Option<Arc<Expression::NFExpression>>,
            fixed: Option<Arc<Expression::NFExpression>>,
            uncertainty: Option<Uncertainty>,
            distribution: Option<Arc<Distribution>>,
            binding: Option<Arc<Expression::NFExpression>>,
            isProtected: Option<bool>,
            finalPrefix: Option<bool>,
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_BOOL {
            quantity: Option<Arc<Expression::NFExpression>>,
            start: Option<Arc<Expression::NFExpression>>,
            fixed: Option<Arc<Expression::NFExpression>>,
            binding: Option<Arc<Expression::NFExpression>>,
            isProtected: Option<bool>,
            finalPrefix: Option<bool>,
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_CLOCK {
            isProtected: Option<bool>,
            finalPrefix: Option<bool>,
        },
        VAR_ATTR_STRING {
            quantity: Option<Arc<Expression::NFExpression>>,
            start: Option<Arc<Expression::NFExpression>>,
            fixed: Option<Arc<Expression::NFExpression>>,
            binding: Option<Arc<Expression::NFExpression>>,
            isProtected: Option<bool>,
            finalPrefix: Option<bool>,
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_ENUMERATION {
            quantity: Option<Arc<Expression::NFExpression>>,
            min: Option<Arc<Expression::NFExpression>>,
            max: Option<Arc<Expression::NFExpression>>,
            start: Option<Arc<Expression::NFExpression>>,
            fixed: Option<Arc<Expression::NFExpression>>,
            binding: Option<Arc<Expression::NFExpression>>,
            isProtected: Option<bool>,
            finalPrefix: Option<bool>,
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_RECORD {
            indexMap: UnorderedMap::UnorderedMap<i32, String>,
            childrenAttr: Vec<Arc<VariableAttributes>>,
        },
    }
    pub use VariableAttributes::*;
    pub enum VarType {
        ENUMERATION,
        CLOCK,
        STRING,
    }

    pub fn toString(attr: Arc<VariableAttributes>) -> String {
        todo!()
    }

    pub fn recordString(attr_tpl: (i32, String), childrenAttr: Vec<Arc<VariableAttributes>>) -> String {
        todo!()
    }

    pub fn create(attrs: metamodelica::List<(Arc<NFBinding::NFBinding>, String)>, ty: Arc<Type::NFType>, compAttrs: Arc<Attributes::NFAttributes>, children: metamodelica::List<Arc<Variable::NFVariable>>, comment: Arc<SCode::Comment>) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn map(attributes: Arc<VariableAttributes>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn setFixed(attributes: Arc<VariableAttributes>, ty: Arc<Type::NFType>, b: bool, overwrite: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn isFixed(attributes: Arc<VariableAttributes>) -> bool {
        todo!()
    }

    pub fn setStartAttribute(attributes: Arc<VariableAttributes>, start: Arc<Expression::NFExpression>, overwrite: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn getStartAttribute(attributes: Arc<VariableAttributes>) -> Option<Arc<Expression::NFExpression>> {
        todo!()
    }

    pub fn setMin(attributes: Arc<VariableAttributes>, min_val: Option<Arc<Expression::NFExpression>>, overwrite: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn setMax(attributes: Arc<VariableAttributes>, max_val: Option<Arc<Expression::NFExpression>>, overwrite: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn setStateSelect(attributes: Arc<VariableAttributes>, stateSelect_val: StateSelect, overwrite: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn getStateSelect(attributes: Arc<VariableAttributes>) -> StateSelect {
        todo!()
    }

    pub fn setTearingSelect(attributes: Arc<VariableAttributes>, tearingSelect_val: TearingSelect, overwrite: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    pub fn getTearingSelect(attributes: Arc<VariableAttributes>) -> TearingSelect {
        todo!()
    }

    pub fn getNominal(attr: Arc<VariableAttributes>) -> Option<Arc<Expression::NFExpression>> {
        todo!()
    }

    pub fn scalarizeReal(quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, unit_iter: Arc<ExpressionIterator::NFExpressionIterator>, displayUnit_iter: Arc<ExpressionIterator::NFExpressionIterator>, min_iter: Arc<ExpressionIterator::NFExpressionIterator>, max_iter: Arc<ExpressionIterator::NFExpressionIterator>, start_iter: Arc<ExpressionIterator::NFExpressionIterator>, fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, nominal_iter: Arc<ExpressionIterator::NFExpressionIterator>, stateSelect: Option<StateSelect>, tearingSelect: Option<TearingSelect>, uncertainty: Option<Uncertainty>, distribution: Option<Arc<Distribution>>, binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, isProtected: Option<bool>, finalPrefix: Option<bool>, startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, length: i32) -> metamodelica::List<Arc<VariableAttributes>> {
        todo!()
    }

    pub fn scalarizeInt(quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, min_iter: Arc<ExpressionIterator::NFExpressionIterator>, max_iter: Arc<ExpressionIterator::NFExpressionIterator>, start_iter: Arc<ExpressionIterator::NFExpressionIterator>, fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, uncertainty: Option<Uncertainty>, distribution: Option<Arc<Distribution>>, binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, isProtected: Option<bool>, finalPrefix: Option<bool>, startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, length: i32) -> metamodelica::List<Arc<VariableAttributes>> {
        todo!()
    }

    pub fn scalarizeBool(quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, start_iter: Arc<ExpressionIterator::NFExpressionIterator>, fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, isProtected: Option<bool>, finalPrefix: Option<bool>, startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, length: i32) -> metamodelica::List<Arc<VariableAttributes>> {
        todo!()
    }

    pub fn scalarizeClock(isProtected: Option<bool>, finalPrefix: Option<bool>, length: i32) -> metamodelica::List<Arc<VariableAttributes>> {
        todo!()
    }

    pub fn scalarizeString(quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, start_iter: Arc<ExpressionIterator::NFExpressionIterator>, fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, isProtected: Option<bool>, finalPrefix: Option<bool>, startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, length: i32) -> metamodelica::List<Arc<VariableAttributes>> {
        todo!()
    }

    pub fn scalarizeEnumeration(quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, min_iter: Arc<ExpressionIterator::NFExpressionIterator>, max_iter: Arc<ExpressionIterator::NFExpressionIterator>, start_iter: Arc<ExpressionIterator::NFExpressionIterator>, fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, isProtected: Option<bool>, finalPrefix: Option<bool>, startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, length: i32) -> metamodelica::List<Arc<VariableAttributes>> {
        todo!()
    }

    pub fn scalarize(attributes: Arc<VariableAttributes>, length: i32) -> metamodelica::List<Arc<VariableAttributes>> {
        todo!()
    }

    pub fn elemType(attr: Arc<VariableAttributes>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn attributesToString(tpl_list: metamodelica::List<(Option<Arc<Expression::NFExpression>>, String)>, stateSelect: Option<StateSelect>, tearingSelect: Option<TearingSelect>) -> String {
        todo!()
    }

    pub fn attributeToString(tpl: (Option<Arc<Expression::NFExpression>>, String), buffer: metamodelica::List<String>) -> metamodelica::List<String> {
        todo!()
    }

    pub fn stateSelectString(stateSelect: StateSelect) -> String {
        todo!()
    }

    pub fn tearingSelectString(tearingSelect: TearingSelect) -> String {
        todo!()
    }

    pub fn stateSelectStringBuffer(optStateSelect: Option<StateSelect>, buffer: metamodelica::List<String>) -> metamodelica::List<String> {
        todo!()
    }

    pub fn tearingSelectStringBuffer(optTearingSelect: Option<TearingSelect>, buffer: metamodelica::List<String>) -> metamodelica::List<String> {
        todo!()
    }

    fn createReal(attrs: metamodelica::List<(Arc<NFBinding::NFBinding>, String)>, isFinal: bool, comment: Arc<SCode::Comment>) -> Arc<VariableAttributes> {
        todo!()
    }

    fn createInt(attrs: metamodelica::List<(Arc<NFBinding::NFBinding>, String)>, isFinal: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    fn createBool(attrs: metamodelica::List<(Arc<NFBinding::NFBinding>, String)>, isFinal: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    fn createString(attrs: metamodelica::List<(Arc<NFBinding::NFBinding>, String)>, isFinal: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    fn createEnum(attrs: metamodelica::List<(Arc<NFBinding::NFBinding>, String)>, isFinal: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    fn createClock(isFinal: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    fn createRecord(attrs: metamodelica::List<(Arc<NFBinding::NFBinding>, String)>, indexMap: UnorderedMap::UnorderedMap<i32, String>, children: metamodelica::List<Arc<Variable::NFVariable>>, isFinal: bool) -> Arc<VariableAttributes> {
        todo!()
    }

    fn createAttribute(binding: Arc<NFBinding::NFBinding>) -> Option<Arc<Expression::NFExpression>> {
        todo!()
    }

    fn createStateSelect(binding: Arc<NFBinding::NFBinding>) -> Option<StateSelect> {
        todo!()
    }

    fn getStateSelectName(exp: Arc<Expression::NFExpression>) -> String {
        todo!()
    }

    fn lookupStateSelectMember(name: String) -> StateSelect {
        todo!()
    }

    fn createTearingSelect(cmt: Arc<SCode::Comment>) -> Option<TearingSelect> {
        todo!()
    }

    fn getTearingSelectName(exp: Arc<Absyn::Exp>, info: SourceInfo) -> String {
        todo!()
    }

    fn lookupTearingSelectMember(name: String) -> Option<TearingSelect> {
        todo!()
    }

}

pub mod VariableKind {
    use super::*;
    pub enum VariableKind {
        TIME,
        ALGEBRAIC,
        STATE {
            index: i32,
            derivative: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
            natural: bool,
        },
        STATE_DER {
            state: Pointer::Pointer<Arc<Variable::NFVariable>>,
            alias: Option<Pointer::Pointer<Arc<Expression::NFExpression>>>,
        },
        DUMMY_DER {
            dummy_state: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        DUMMY_STATE {
            dummy_der: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        DISCRETE,
        DISCRETE_STATE,
        PREVIOUS,
        CLOCK,
        CLOCKED,
        PARAMETER {
            resize_value: Option<i32>,
        },
        CONSTANT,
        ITERATOR,
        RECORD {
            children: metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>,
            min_var: Variability,
            max_var: Variability,
        },
        START {
            original: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        EXTOBJ {
            fullClassName: Arc<Absyn::Path>,
        },
        JAC_VAR,
        JAC_TMP_VAR,
        SEED_VAR,
        OPT_CONSTR,
        OPT_FCONSTR,
        OPT_INPUT_WITH_DER,
        OPT_INPUT_DER,
        OPT_TGRID,
        OPT_LOOP_INPUT {
            replaceCref: Arc<ComponentRef::NFComponentRef>,
        },
        ALG_STATE,
        ALG_STATE_OLD,
        RESIDUAL_VAR,
        DAE_AUX_VAR,
        LOOP_ITERATION,
        LOOP_SOLVED,
        FRONTEND_DUMMY,
    }
    pub use VariableKind::*;
    pub fn toString(varKind: Arc<VariableKind>) -> String {
        todo!()
    }

    pub fn isTimeDependent(varKind: Arc<VariableKind>) -> bool {
        todo!()
    }

    pub fn fromType(ty: Arc<Type::NFType>, makeParam: bool) -> Arc<VariableKind> {
        todo!()
    }

}

