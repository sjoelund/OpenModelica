// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::BaseModelica;
use crate::DAE;
use crate::Dump;
use crate::NFBinding as Binding;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::Mutable;

pub enum NFBinding {
    UNBOUND,
    RAW_BINDING {
        bindingExp: Arc<Absyn::Exp>,
        scope: Arc<InstNode::InstNode>,
        subs: metamodelica::List<Arc<Subscript::NFSubscript>>,
        eachType: EachType,
        source: Source,
        info: SourceInfo,
    },
    UNTYPED_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
        isProcessing: bool,
        scope: Arc<InstNode::InstNode>,
        eachType: EachType,
        source: Source,
        info: SourceInfo,
    },
    TYPED_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
        bindingType: Arc<Type::NFType>,
        variability: Variability,
        purity: Purity,
        eachType: EachType,
        evalState: Mutable::Mutable<EvalState>,
        isFlattened: bool,
        source: Source,
        info: SourceInfo,
    },
    FLAT_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
        variability: Variability,
        source: Source,
    },
    CEVAL_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
    },
    INVALID_BINDING {
        binding: Arc<Binding>,
        errors: metamodelica::List<ErrorTypes::TotalMessage>,
    },
    WILD,
}
pub use NFBinding::*;
pub enum EachType {
    NOT_EACH,
    EACH,
}

pub enum EvalState {
    NOT_EVALUATED,
    EVALUATING,
    EVALUATED,
}

pub enum Source {
    BINDING,
    TYPE,
    MODIFIER,
    GENERATED,
}

pub fn fromAbsyn(bindingExp: Option<Arc<Absyn::Exp>>, eachPrefix: bool, fromType: bool, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Binding> {
    todo!()
}

pub fn isBound(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn isExplicitlyBound(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn isUnbound(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn isInvalid(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn typedExp(binding: Arc<Binding>) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn getUntypedExp(binding: Arc<Binding>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn getTypedExp(binding: Arc<Binding>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn setTypedExp(exp: Arc<Expression::NFExpression>, binding: Arc<Binding>) -> Arc<Binding> {
    todo!()
}

pub fn hasExp(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn getExp(binding: Arc<Binding>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn getExpOpt(binding: Arc<Binding>) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn setExp(exp: Arc<Expression::NFExpression>, binding: Arc<Binding>) -> Arc<Binding> {
    todo!()
}

pub fn isRecordExp(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn isCrefExp(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn recordFieldBinding(fieldNode: Arc<InstNode::InstNode>, recordBinding: Arc<Binding>) -> Arc<Binding> {
    todo!()
}

pub fn variability(binding: Arc<Binding>) -> Variability {
    todo!()
}

pub fn purity(binding: Arc<Binding>) -> Purity {
    todo!()
}

pub fn getInfo(binding: Arc<Binding>) -> SourceInfo {
    todo!()
}

pub fn getType(binding: Arc<Binding>) -> Arc<Type::NFType> {
    todo!()
}

pub fn isEach(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn isTyped(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn toString(binding: Arc<Binding>, prefix: String) -> String {
    todo!()
}

pub fn toFlatString(binding: Arc<Binding>, format: BaseModelica::OutputFormat, prefix: String) -> String {
    todo!()
}

pub fn toDebugString(binding: Arc<Binding>) -> String {
    todo!()
}

pub fn isEqual(binding1: Arc<Binding>, binding2: Arc<Binding>) -> bool {
    todo!()
}

pub fn toDAE(binding: Arc<Binding>) -> Arc<DAE::Binding> {
    todo!()
}

pub fn makeDAEBinding(exp: Arc<Expression::NFExpression>, var: Variability) -> Arc<DAE::Binding> {
    todo!()
}

pub fn toDAEExp(binding: Arc<Binding>) -> Option<Arc<DAE::Exp>> {
    todo!()
}

pub fn applyExp(binding: Arc<Binding>, r#fn: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpShallow(binding: Arc<Binding>, r#fn: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn mapExp(binding: Arc<Binding>, mapFn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Binding> {
    todo!()
}

pub fn mapExpShallow(binding: Arc<Binding>, mapFn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Binding> {
    todo!()
}

pub fn foldExp<ArgT>(binding: Arc<Binding>, foldFn: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn containsExp(binding: Arc<Binding>, predFn: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn update(binding: Arc<Binding>, exp: Arc<Expression::NFExpression>) -> Arc<Binding> {
    todo!()
}

pub fn setAttr(ty_attr: metamodelica::List<(Arc<Binding>, String)>, attr_name: String, attr_value: Arc<Binding>) -> metamodelica::List<(Arc<Binding>, String)> {
    todo!()
}

pub fn propagate(binding: Arc<Binding>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Binding> {
    todo!()
}

pub fn unpropagate(binding: Arc<Binding>, node: Arc<InstNode::InstNode>) -> Arc<Binding> {
    todo!()
}

pub fn source(binding: Arc<Binding>) -> Source {
    todo!()
}

pub fn makeUntyped(exp: Arc<Expression::NFExpression>, scope: Arc<InstNode::InstNode>, eachType: EachType, source: Source, info: SourceInfo) -> Arc<Binding> {
    todo!()
}

pub fn makeTyped(exp: Arc<Expression::NFExpression>, eachType: EachType, source: Source, info: SourceInfo, state: EvalState) -> Arc<Binding> {
    todo!()
}

pub fn makeFlat(exp: Arc<Expression::NFExpression>, var: Variability, source: Source) -> Arc<Binding> {
    todo!()
}

pub fn isEvaluated(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn hasTypeOrigin(binding: Arc<Binding>) -> bool {
    todo!()
}

pub fn expandEach(binding: Arc<Binding>, node: Arc<InstNode::InstNode>) -> Arc<Binding> {
    todo!()
}

pub fn isClockOrSampleFunction(binding: Arc<Binding>) -> bool {
    todo!()
}


