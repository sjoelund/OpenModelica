// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall::Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections::Sections;
use crate::NFStatement::Statement;
use crate::NFType as Type;
use crate::SCode::Element;
use crate::SCode;
use crate::SCodeUtil;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::IOStream;
use openmodelica_util::System;

pub enum NFClass {
    NOT_INSTANTIATED,
    PARTIAL_CLASS {
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
    },
    PARTIAL_BUILTIN {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    EXPANDED_CLASS {
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    EXPANDED_DERIVED {
        baseClass: Arc<InstNode::InstNode>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        dims: Vec<Arc<Dimension::NFDimension>>,
        prefixes: Arc<Prefixes::Prefixes>,
        attributes: Arc<Attributes::NFAttributes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    INSTANCED_CLASS {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        sections: Arc<NFSections::NFSections>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    INSTANCED_BUILTIN {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    TYPED_DERIVED {
        ty: Arc<Type::NFType>,
        baseClass: Arc<InstNode::InstNode>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    DAE_TYPE {
        ty: Arc<DAE::Type>,
    },
}
pub use NFClass::*;
pub fn fromSCode(elements: metamodelica::List<Arc<Element>>, isClassExtends: bool, scope: Arc<InstNode::InstNode>, prefixes: Arc<Prefixes::Prefixes>) -> Arc<Class> {
    todo!()
}

pub fn initImports(cls: Arc<Class>, parent: Arc<InstNode::InstNode>) -> Arc<Class> {
    todo!()
}

pub fn fromEnumeration(literals: metamodelica::List<Arc<SCode::Enum>>, enumType: Arc<Type::NFType>, prefixes: Arc<Prefixes::Prefixes>, enumClass: Arc<InstNode::InstNode>) -> Arc<Class> {
    todo!()
}

pub fn makeRecordConstructor(fields: metamodelica::List<Arc<InstNode::InstNode>>, out: Arc<InstNode::InstNode>) -> Arc<Class> {
    todo!()
}

pub fn initExpandedClass(cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn getSections(cls: Arc<Class>) -> Arc<NFSections::NFSections> {
    todo!()
}

pub fn setSections(sections: Arc<NFSections::NFSections>, cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn lookupElement(name: String, cls: Arc<Class>) -> (Arc<InstNode::InstNode>, bool) {
    todo!()
}

pub fn tryLookupElement(name: String, cls: Arc<Class>) -> (Option<Arc<InstNode::InstNode>>, bool) {
    todo!()
}

pub fn lookupComponentIndex(name: String, cls: Arc<Class>) -> i32 {
    todo!()
}

pub fn nthComponent(index: i32, cls: Arc<Class>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn getComponents(cls: Arc<Class>) -> Vec<Arc<InstNode::InstNode>> {
    todo!()
}

pub fn lookupAttributeBinding(name: String, cls: Arc<Class>) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn lookupAttributeValue(name: String, cls: Arc<Class>) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn isOnlyBuiltin(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isBuiltin(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn classTree(cls: Arc<Class>) -> Arc<ClassTree::ClassTree> {
    todo!()
}

pub fn setClassTree(tree: Arc<ClassTree::ClassTree>, cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn classTreeApply(cls: Arc<Class>, func: fn(Arc<ClassTree::ClassTree>) -> Arc<ClassTree::ClassTree>) -> Arc<Class> {
    todo!()
}

pub fn getModifier(cls: Arc<Class>) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn getCCModifier(cls: Arc<Class>) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn setModifier(modifier: Arc<Modifier::Modifier>, cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn mergeModifier(modifier: Arc<Modifier::Modifier>, cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn isIdentical(cls1: Arc<Class>, cls2: Arc<Class>) -> bool {
    todo!()
}

pub fn hasDimensions(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn getDimensions(cls: Arc<Class>) -> metamodelica::List<Arc<Dimension::NFDimension>> {
    todo!()
}

pub fn dimensionCount(cls: Arc<Class>) -> i32 {
    todo!()
}

pub fn getAttributes(cls: Arc<Class>) -> Arc<Attributes::NFAttributes> {
    todo!()
}

pub fn getTypeAttributes(cls: Arc<Class>) -> metamodelica::List<Arc<Modifier::Modifier>> {
    todo!()
}

pub fn getType(cls: Arc<Class>, clsNode: Arc<InstNode::InstNode>) -> Arc<Type::NFType> {
    todo!()
}

pub fn setType(ty: Arc<Type::NFType>, cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn restriction(cls: Arc<Class>) -> Arc<Restriction::NFRestriction> {
    todo!()
}

pub fn setRestriction(res: Arc<Restriction::NFRestriction>, cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn isConnectorClass(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isNonexpandableConnectorClass(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isExpandableConnectorClass(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isExternalObject(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isFunction(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isEnumeration(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isExternalFunction(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isOverdetermined(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn getPrefixes(cls: Arc<Class>) -> Arc<Prefixes::Prefixes> {
    todo!()
}

pub fn setPrefixes(prefs: Arc<Prefixes::Prefixes>, cls: Arc<Class>) -> Arc<Class> {
    todo!()
}

pub fn isEncapsulated(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn isPartial(cls: Arc<Class>) -> bool {
    todo!()
}

pub fn lastBaseClass(node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn getDerivedComments(cls: Arc<Class>, cmts: metamodelica::List<Arc<SCode::Comment>>) -> metamodelica::List<Arc<SCode::Comment>> {
    todo!()
}

pub fn constrainingClassPath(clsNode: Arc<InstNode::InstNode>) -> Arc<Absyn::Path> {
    todo!()
}

pub fn hasOperator(name: String, cls: Arc<Class>) -> bool {
    todo!()
}

pub fn makeRecordExp(clsNode: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, typed: bool) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn toFlatStream(cls: Arc<Class>, clsNode: Arc<InstNode::InstNode>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatString(cls: Arc<Class>, clsNode: Arc<InstNode::InstNode>, format: BaseModelica::OutputFormat, indent: String) -> String {
    todo!()
}


