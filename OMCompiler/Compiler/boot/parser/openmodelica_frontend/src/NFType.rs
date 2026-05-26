// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::BaseModelica;
use crate::DAE;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFDimension as Dimension;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFRecord as Record;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_util::Array;
use openmodelica_util::IOStream;
use openmodelica_util::List;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub enum NFType {
    INTEGER,
    REAL,
    STRING,
    BOOLEAN,
    CLOCK,
    ENUMERATION {
        typePath: Arc<Absyn::Path>,
        literals: metamodelica::List<String>,
    },
    __ENUMERATION_ANY_NOT_USED__,
    ARRAY {
        elementType: Arc<Type>,
        dimensions: metamodelica::List<Arc<Dimension::NFDimension>>,
    },
    TUPLE {
        types: metamodelica::List<Arc<Type>>,
        names: Option<metamodelica::List<String>>,
    },
    NORETCALL,
    UNKNOWN,
    COMPLEX {
        cls: Arc<InstNode::InstNode>,
        complexTy: Arc<ComplexType::NFComplexType>,
    },
    FUNCTION {
        r#fn: Arc<Function::Function>,
        fnType: FunctionType,
    },
    METABOXED {
        ty: Arc<Type>,
    },
    POLYMORPHIC {
        name: String,
    },
    ANY,
    CONDITIONAL_ARRAY {
        trueType: Arc<Type>,
        falseType: Arc<Type>,
        matchedBranch: Branch,
    },
    UNTYPED {
        typeNode: Arc<InstNode::InstNode>,
        dimensions: Vec<Arc<Dimension::NFDimension>>,
    },
}
pub use NFType::*;
pub enum FunctionType {
    FUNCTIONAL_PARAMETER,
    FUNCTION_REFERENCE,
    FUNCTIONAL_VARIABLE,
}

pub enum Branch {
    NONE,
    TRUE,
    FALSE,
}

pub fn liftArrayLeft(ty: Arc<Type>, dim: Arc<Dimension::NFDimension>) -> Arc<Type> {
    todo!()
}

pub fn liftArrayLeftList(ty: Arc<Type>, dims: metamodelica::List<Arc<Dimension::NFDimension>>) -> Arc<Type> {
    todo!()
}

pub fn liftArrayRightList(ty: Arc<Type>, dims: metamodelica::List<Arc<Dimension::NFDimension>>) -> Arc<Type> {
    todo!()
}

pub fn unliftArray(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn unliftArrayN(N: i32, ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn isInteger(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isReal(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isBoolean(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isString(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isClock(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isContinuous(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isScalar(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isArray(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isConditionalArray(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isResizable(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn sizeKnown(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isAny(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn setConditionalArrayTypes(condType: Arc<Type>, trueType: Arc<Type>, falseType: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn removeSizeOneArraysAndRecords(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn isMatchedBranch(condition: bool, condType: Arc<Type>) -> bool {
    todo!()
}

pub fn simplifyConditionalArray(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn isVector(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isMatrix(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isSquareMatrix(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isEmptyArray(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isSingleElementArray(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isEnumeration(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isBuiltinEnumeration(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isUnspecifiedEnumeration(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isComplex(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isComplexArray(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn complexNode(ty: Arc<Type>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn complexComponents(ty: Arc<Type>) -> Vec<Arc<InstNode::InstNode>> {
    todo!()
}

pub fn isConnector(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isStreamConnector(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isExpandableConnector(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isExternalObject(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isRecord(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isBasic(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isBasicNumeric(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isNumeric(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isScalarBuiltin(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isTuple(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isUnknown(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isKnown(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isPolymorphic(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn isPolymorphicNamed(ty: Arc<Type>, name: String) -> bool {
    todo!()
}

pub fn firstTupleType(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn nthTupleType(ty: Arc<Type>, n: i32) -> Arc<Type> {
    todo!()
}

pub fn arrayElementType(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn setArrayElementType(arrayTy: Arc<Type>, elementTy: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn elementType(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn copyElementType(dstType: Arc<Type>, srcType: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn arrayDims(ty: Arc<Type>) -> metamodelica::List<Arc<Dimension::NFDimension>> {
    todo!()
}

pub fn copyDims(srcType: Arc<Type>, dstType: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn applyToDims(ty: Arc<Type>, func: fn(Arc<Dimension::NFDimension>) -> Arc<Dimension::NFDimension>) -> Arc<Type> {
    todo!()
}

pub fn nthDimension(ty: Arc<Type>, index: i32) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn dimensionCount(ty: Arc<Type>) -> i32 {
    todo!()
}

pub fn dimensionDiff(ty1: Arc<Type>, ty2: Arc<Type>) -> i32 {
    todo!()
}

pub fn hasKnownSize(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn hasZeroDimension(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn mapDims(ty: Arc<Type>, func: fn(Arc<Dimension::NFDimension>) -> Arc<Dimension::NFDimension>) -> Arc<Type> {
    todo!()
}

pub fn foldDims<ArgT>(ty: Arc<Type>, func: fn(Arc<Dimension::NFDimension>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn nthEnumLiteral(ty: Arc<Type>, index: i32) -> String {
    todo!()
}

pub fn toString(ty: Arc<Type>) -> String {
    todo!()
}

pub fn toFlatString(ty: Arc<Type>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn dimensionsToFlatString(ty: Arc<Type>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn toFlatDeclarationStream(ty: Arc<Type>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn typenameString(ty: Arc<Type>) -> String {
    todo!()
}

pub fn toDAE(ty: Arc<Type>, makeTypeVars: bool) -> Arc<DAE::Type> {
    todo!()
}

pub fn subscript(ty: Arc<Type>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>, failOnError: bool) -> Arc<Type> {
    todo!()
}

pub fn isEqual(ty1: Arc<Type>, ty2: Arc<Type>) -> bool {
    todo!()
}

pub fn hashContinue(ty: Arc<Type>, hash: i32) -> i32 {
    todo!()
}

pub fn isDiscrete(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn lookupRecordFieldType(name: String, recordType: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn recordFieldCount(recordType: Arc<Type>) -> i32 {
    todo!()
}

pub fn recordFields(recordType: Arc<Type>) -> metamodelica::List<Arc<NFRecord::Field::Field>> {
    todo!()
}

pub fn setRecordFields(field_lst: metamodelica::List<Arc<NFRecord::Field::Field>>, recordType: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn updateRecordFieldsIndexMap(fields: Vec<Arc<NFRecord::Field::Field>>, indexMap: UnorderedMap::UnorderedMap<i32, String>) -> () {
    todo!()
}

pub fn tupleFieldCount(tupleType: Arc<Type>) -> i32 {
    todo!()
}

pub fn enumName(ty: Arc<Type>) -> Arc<Absyn::Path> {
    todo!()
}

pub fn enumSize(ty: Arc<Type>) -> i32 {
    todo!()
}

pub fn r#box(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn unbox(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn isBoxed(ty: Arc<Type>) -> bool {
    todo!()
}

pub fn sizeType(arrayTy: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn simplify(ty: Arc<Type>) -> Arc<Type> {
    todo!()
}

pub fn sizeOf(ty: Arc<Type>, resize: bool) -> i32 {
    todo!()
}

pub fn complexSize(ty: Arc<Type>, resize: bool) -> Option<i32> {
    todo!()
}


