// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::BaseModelica;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRestriction as Restriction;
use crate::NFType as Type;
use crate::SCode::Element;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::IOStream;

pub enum NFComponent {
    COMPONENT_DEF {
        definition: Arc<Element>,
        modifier: Arc<Modifier::Modifier>,
    },
    COMPONENT {
        classInst: Arc<InstNode::InstNode>,
        ty: Arc<Type::NFType>,
        binding: Arc<Binding::NFBinding>,
        condition: Arc<Binding::NFBinding>,
        attributes: Arc<Attributes::NFAttributes>,
        comment: Arc<SCode::Comment>,
        state: ComponentState,
        info: SourceInfo,
    },
    ITERATOR {
        ty: Arc<Type::NFType>,
        variability: Variability,
        info: SourceInfo,
    },
    ENUM_LITERAL {
        literal: Arc<Expression::NFExpression>,
        comment: Arc<SCode::Comment>,
    },
    TYPE_ATTRIBUTE {
        ty: Arc<Type::NFType>,
        modifier: Arc<Modifier::Modifier>,
    },
    INVALID_COMPONENT {
        component: Arc<Component>,
        errors: String,
    },
    WILD,
}
pub use NFComponent::*;
pub enum ComponentState {
    PartiallyInstantiated,
    FullyInstantiated,
    Typed,
    TypeChecked,
}

pub fn new(definition: Arc<Element>) -> Arc<Component> {
    todo!()
}

pub fn newEnum(enumType: Arc<Type::NFType>, literalName: String, comment: Arc<SCode::Comment>, literalIndex: i32) -> Arc<Component> {
    todo!()
}

pub fn newIterator(iterType: Arc<Type::NFType>, info: SourceInfo) -> Arc<Component> {
    todo!()
}

pub fn definition(component: Arc<Component>) -> Arc<Element> {
    todo!()
}

pub fn isDefinition(component: Arc<Component>) -> bool {
    todo!()
}

pub fn info(component: Arc<Component>) -> SourceInfo {
    todo!()
}

pub fn classInstance(component: Arc<Component>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn setClassInstance(classInst: Arc<InstNode::InstNode>, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn getModifier(component: Arc<Component>) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn setModifier(modifier: Arc<Modifier::Modifier>, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn mergeModifier(modifier: Arc<Modifier::Modifier>, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn getType(component: Arc<Component>) -> Arc<Type::NFType> {
    todo!()
}

pub fn setType(ty: Arc<Type::NFType>, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn isTyped(component: Arc<Component>) -> bool {
    todo!()
}

pub fn unliftType(component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn getAttributes(component: Arc<Component>) -> Arc<Attributes::NFAttributes> {
    todo!()
}

pub fn setAttributes(attr: Arc<Attributes::NFAttributes>, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn setComment(comment: Arc<SCode::Comment>, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn getBinding(component: Arc<Component>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn getImplicitBinding(component: Arc<Component>, scope: Arc<InstNode::InstNode>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn getTypeAttributeBinding(component: Arc<Component>, attrName: String) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn setBinding(binding: Arc<Binding::NFBinding>, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn hasBinding(component: Arc<Component>, parent: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

pub fn getCondition(component: Arc<Component>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn hasCondition(component: Arc<Component>) -> bool {
    todo!()
}

pub fn direction(component: Arc<Component>) -> Direction {
    todo!()
}

pub fn isInput(component: Arc<Component>) -> bool {
    todo!()
}

pub fn setDirection(direction: Direction, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn isOutput(component: Arc<Component>) -> bool {
    todo!()
}

pub fn parallelism(component: Arc<Component>) -> Parallelism {
    todo!()
}

pub fn variability(component: Arc<Component>) -> Variability {
    todo!()
}

pub fn setVariability(variability: Variability, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn isConst(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isParameter(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isStructuralParameter(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isVar(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isRedeclare(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isFinal(component: Arc<Component>) -> bool {
    todo!()
}

pub fn setFinal(component: Arc<Component>, isFinal: bool) -> Arc<Component> {
    todo!()
}

pub fn isResizable(component: Arc<Component>) -> bool {
    todo!()
}

pub fn innerOuter(component: Arc<Component>) -> InnerOuter {
    todo!()
}

pub fn isInnerOuter(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isInner(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isOuter(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isOnlyOuter(component: Arc<Component>) -> bool {
    todo!()
}

pub fn connectorType(component: Arc<Component>) -> i32 {
    todo!()
}

pub fn setConnectorType(cty: i32, component: Arc<Component>) -> Arc<Component> {
    todo!()
}

pub fn isFlow(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isConnector(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isExpandableConnector(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isExternalObject(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isIdentical(comp1: Arc<Component>, comp2: Arc<Component>) -> bool {
    todo!()
}

pub fn toString(name: String, component: Arc<Component>) -> String {
    todo!()
}

pub fn toFlatStream(name: String, component: Arc<Component>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn typeAttrsToFlatStream(typeAttrs: metamodelica::List<(Arc<Binding::NFBinding>, String)>, componentType: Arc<Type::NFType>, format: BaseModelica::OutputFormat, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatString(name: String, component: Arc<Component>, format: BaseModelica::OutputFormat, indent: String) -> String {
    todo!()
}

pub fn dimensionCount(component: Arc<Component>) -> i32 {
    todo!()
}

pub fn comment(component: Arc<Component>) -> Arc<SCode::Comment> {
    todo!()
}

pub fn getEvaluateAnnotation(component: Arc<Component>) -> Option<bool> {
    todo!()
}

pub fn isFixed(component: Arc<Component>) -> bool {
    todo!()
}

pub fn getUnitAttribute(component: Arc<Component>, defaultUnit: String) -> String {
    todo!()
}

pub fn isDeleted(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isInvalid(component: Arc<Component>) -> bool {
    todo!()
}

pub fn isTypeAttribute(component: Arc<Component>) -> bool {
    todo!()
}

pub fn countConnectorVars(component: Arc<Component>, isRoot: bool) -> (i32, i32, i32, bool) {
    todo!()
}


