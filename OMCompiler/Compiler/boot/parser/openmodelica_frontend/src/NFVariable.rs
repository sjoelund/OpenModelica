// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::BaseModelica;
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableAttributes;
use crate::NFBackendExtension::VariableKind;
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::AccessLevel;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use crate::SCode;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;

pub struct VARIABLE {
    pub name: Arc<ComponentRef::NFComponentRef>,
    pub ty: Arc<Type::NFType>,
    pub binding: Arc<Binding::NFBinding>,
    pub visibility: Visibility,
    pub attributes: Arc<Attributes::NFAttributes>,
    pub typeAttributes: metamodelica::List<(Arc<Binding::NFBinding>, String)>,
    pub children: metamodelica::List<Arc<Variable>>,
    pub comment: Arc<SCode::Comment>,
    pub info: SourceInfo,
    pub backendinfo: Arc<BackendInfo::BackendInfo>,
}

pub type NFVariable = VARIABLE;
pub fn fromCref(cref: Arc<ComponentRef::NFComponentRef>) -> Arc<Variable> {
    todo!()
}

pub fn name(var: Arc<Variable>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn size(var: Arc<Variable>, resize: bool) -> i32 {
    todo!()
}

pub fn hash(var: Arc<Variable>) -> i32 {
    todo!()
}

pub fn equalName(var1: Arc<Variable>, var2: Arc<Variable>) -> bool {
    todo!()
}

pub fn expand(var: Arc<Variable>, backend: bool) -> metamodelica::List<Arc<Variable>> {
    todo!()
}

pub fn expandChildren(var: Arc<Variable>, arrayDims: metamodelica::List<Arc<Dimension::NFDimension>>, addDimensions: bool) -> metamodelica::List<Arc<Variable>> {
    todo!()
}

pub fn typeOf(var: Arc<Variable>) -> Arc<Type::NFType> {
    todo!()
}

pub fn attributes(variable: Arc<Variable>) -> Arc<Attributes::NFAttributes> {
    todo!()
}

pub fn variability(variable: Arc<Variable>) -> Variability {
    todo!()
}

pub fn setVariability(variable: Arc<Variable>, variability: Variability) -> Arc<Variable> {
    todo!()
}

pub fn visibility(variable: Arc<Variable>) -> Visibility {
    todo!()
}

pub fn isComplex(var: Arc<Variable>) -> bool {
    todo!()
}

pub fn isComplexArray(var: Arc<Variable>) -> bool {
    todo!()
}

pub fn isStructural(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isEmptyArray(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isDeleted(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isPresent(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isPotential(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isFlow(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isStream(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isInput(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isTopLevelInput(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isPublic(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isProtected(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isEncrypted(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn isAccessible(variable: Arc<Variable>) -> bool {
    todo!()
}

pub fn lookupTypeAttribute(name: String, var: Arc<Variable>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn applyToType(var: Arc<Variable>, func: fn(Arc<Type::NFType>) -> Arc<Type::NFType>) -> Arc<Variable> {
    todo!()
}

pub fn propagateAnnotation(name: String, overwrite: bool, evaluate: bool, var: Arc<Variable>) -> Arc<Variable> {
    todo!()
}

pub fn removeNonTopLevelDirection(var: Arc<Variable>) -> Arc<Variable> {
    todo!()
}

pub type ApplyFn = fn(Arc<Expression::NFExpression>) -> ();

pub fn applyExp(var: Arc<Variable>, r#fn: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpShallow(var: Arc<Variable>, r#fn: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub type MapFn = fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>;

pub fn mapExp(var: Arc<Variable>, r#fn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Variable> {
    todo!()
}

pub fn mapExpShallow(var: Arc<Variable>, r#fn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Variable> {
    todo!()
}

pub fn toString(var: Arc<Variable>, indent: String, printBindingType: bool) -> String {
    todo!()
}

pub fn toStream(var: Arc<Variable>, indent: String, printBindingType: bool, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStream(var: Arc<Variable>, format: BaseModelica::OutputFormat, indent: String, printBindingType: bool, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStreamBinding(binding: Arc<Binding::NFBinding>, printBindingType: bool, format: BaseModelica::OutputFormat, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStreamModifier(children: metamodelica::List<Arc<Variable>>, overwrittenBinding: bool, printBindingType: bool, format: BaseModelica::OutputFormat, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn moveBinding(var: Arc<Variable>, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> (Arc<Variable>, metamodelica::List<Arc<Equation::NFEquation>>) {
    todo!()
}

pub fn getVariableAttributes(var: Arc<Variable>) -> Arc<VariableAttributes::VariableAttributes> {
    todo!()
}

pub fn getNominal(var: Arc<Variable>) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}


