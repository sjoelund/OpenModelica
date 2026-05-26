// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::BaseModelica;
use crate::DAE;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalConstants as EvalConstants;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionStatus;
use crate::NFInst as Inst;
use crate::NFInst::InstSettings;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFLookup as Lookup;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::IOStream;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;

pub mod Field {
    use super::*;
    pub enum Field {
        INPUT {
            name: String,
        },
        LOCAL {
            name: String,
        },
    }
    pub use Field::*;
    pub fn isInput(field: Arc<Field>) -> bool {
        todo!()
    }

    pub fn name(field: Arc<Field>) -> String {
        todo!()
    }

}

pub fn checkLocalFieldOrder(locals: metamodelica::List<Arc<InstNode::InstNode>>, recNode: Arc<InstNode::InstNode>, info: SourceInfo) -> () {
    todo!()
}

pub fn collectRecordField(component: Arc<InstNode::InstNode>, fields: metamodelica::List<Arc<Field::Field>>) -> metamodelica::List<Arc<Field::Field>> {
    todo!()
}

pub fn collectRecordFields(recNode: Arc<InstNode::InstNode>) -> (Vec<Arc<Field::Field>>, UnorderedMap::UnorderedMap<i32, String>) {
    todo!()
}

pub fn collectRecordParam(component: Arc<InstNode::InstNode>, inputs: metamodelica::List<Arc<InstNode::InstNode>>, locals: metamodelica::List<Arc<InstNode::InstNode>>) -> (metamodelica::List<Arc<InstNode::InstNode>>, metamodelica::List<Arc<InstNode::InstNode>>) {
    todo!()
}

pub fn collectRecordParams(recNode: Arc<InstNode::InstNode>) -> (metamodelica::List<Arc<InstNode::InstNode>>, metamodelica::List<Arc<InstNode::InstNode>>, metamodelica::List<Arc<InstNode::InstNode>>) {
    todo!()
}

pub fn fieldsToDAE(fields: metamodelica::List<Arc<Field::Field>>) -> metamodelica::List<String> {
    todo!()
}

pub fn foldInputFields<T, ArgT>(fields: metamodelica::List<Arc<Field::Field>>, args: metamodelica::List<T>, func: fn(T, ArgT) -> ArgT, foldArg: ArgT) -> ArgT {
    todo!()
}

pub fn getDeclarationNode(recordNode: Arc<InstNode::InstNode>, evaluate: bool) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn instDefaultConstructor(path: Arc<Absyn::Path>, node: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn instRecord(node: Arc<InstNode::InstNode>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn setFieldDirection(field: Arc<InstNode::InstNode>, direction: Direction) -> () {
    todo!()
}

pub fn toDeclarationStream(recordNode: Arc<InstNode::InstNode>, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatDeclarationStream(recordNode: Arc<InstNode::InstNode>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

