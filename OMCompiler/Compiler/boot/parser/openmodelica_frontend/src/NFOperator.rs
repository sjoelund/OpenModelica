// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::DAE;
use crate::NFOperator as Operator;
use crate::NFType as Type;
use openmodelica_util::JSON;
use openmodelica_util::Util;

pub struct OPERATOR {
    pub ty: Arc<Type::NFType>,
    pub op: Op,
}

pub type NFOperator = OPERATOR;
pub enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    ADD_EW,
    SUB_EW,
    MUL_EW,
    DIV_EW,
    POW_EW,
    ADD_SCALAR_ARRAY,
    ADD_ARRAY_SCALAR,
    SUB_SCALAR_ARRAY,
    SUB_ARRAY_SCALAR,
    MUL_SCALAR_ARRAY,
    MUL_ARRAY_SCALAR,
    MUL_VECTOR_MATRIX,
    MUL_MATRIX_VECTOR,
    SCALAR_PRODUCT,
    MATRIX_PRODUCT,
    DIV_SCALAR_ARRAY,
    DIV_ARRAY_SCALAR,
    POW_SCALAR_ARRAY,
    POW_ARRAY_SCALAR,
    POW_MATRIX,
    UMINUS,
    AND,
    OR,
    NOT,
    LESS,
    LESSEQ,
    GREATER,
    GREATEREQ,
    EQUAL,
    NEQUAL,
    USERDEFINED,
}

pub fn compare(op1: Arc<Operator>, op2: Arc<Operator>) -> i32 {
    todo!()
}

pub fn invert(operator: Arc<Operator>) -> Arc<Operator> {
    todo!()
}

pub enum TypeRestriction {
    SCALAR,
    VECTOR,
    MATRIX,
    ARRAY,
    OTHER,
}

pub fn typeRestriction(ty: Arc<Type::NFType>) -> TypeRestriction {
    todo!()
}

pub fn repairMultary(operator: Arc<Operator>, types: metamodelica::List<Arc<Type::NFType>>) -> Arc<Operator> {
    todo!()
}

pub fn repairBinary(operator: Arc<Operator>, ty1: Arc<Type::NFType>, ty2: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn isLogical(operator: Arc<Operator>) -> bool {
    todo!()
}

pub fn isRelational(operator: Arc<Operator>) -> bool {
    todo!()
}

pub fn isScalarProduct(operator: Arc<Operator>) -> bool {
    todo!()
}

pub fn fromAbsyn(inOperator: Absyn::Operator) -> Arc<Operator> {
    todo!()
}

pub fn toAbsyn(op: Arc<Operator>) -> Absyn::Operator {
    todo!()
}

pub fn toDAE(op: Arc<Operator>) -> (DAE::Operator, bool, bool) {
    todo!()
}

pub fn typeOf(op: Arc<Operator>) -> Arc<Type::NFType> {
    todo!()
}

pub fn setType(ty: Arc<Type::NFType>, op: Arc<Operator>) -> Arc<Operator> {
    todo!()
}

pub fn scalarize(op: Arc<Operator>) -> Arc<Operator> {
    todo!()
}

pub fn unlift(op: Arc<Operator>) -> Arc<Operator> {
    todo!()
}

pub fn symbol(op: Arc<Operator>, spacing: String) -> String {
    todo!()
}

pub fn toJSON(operator: Arc<Operator>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn priority(op: Arc<Operator>, lhs: bool) -> i32 {
    todo!()
}

pub fn isAssociative(op: Arc<Operator>) -> bool {
    todo!()
}

pub fn isNonAssociative(op: Arc<Operator>) -> bool {
    todo!()
}

pub fn makeAdd(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeSub(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeMul(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeScalarProduct(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeDiv(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makePow(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeAddEW(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeSubEW(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeMulEW(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeDivEW(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeUMinus(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeAnd(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeOr(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeNot(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeLess(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeLessEq(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeGreater(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeGreaterEq(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeEqual(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeNotEqual(ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn makeScalarArray(ty: Arc<Type::NFType>, op: Op) -> Arc<Operator> {
    todo!()
}

pub fn makeArrayScalar(ty: Arc<Type::NFType>, op: Op) -> Arc<Operator> {
    todo!()
}

pub fn makeEW(op: Arc<Operator>) -> Arc<Operator> {
    todo!()
}

pub fn stripEW(op: Arc<Operator>) -> Arc<Operator> {
    todo!()
}

pub fn isElementWise(op: Arc<Operator>) -> bool {
    todo!()
}

pub enum MathClassification {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    POWER,
    LOGICAL,
    RELATION,
}

pub enum SizeClassification {
    SCALAR,
    ELEMENT_WISE,
    ARRAY_SCALAR,
    SCALAR_ARRAY,
    MATRIX,
    VECTOR_MATRIX,
    MATRIX_VECTOR,
    LOGICAL,
    RELATION,
}

pub type Classification = (SizeClassification, MathClassification);

pub fn mathSymbol(mcl: MathClassification) -> String {
    todo!()
}

pub fn classificationString(cla: (SizeClassification, MathClassification)) -> String {
    todo!()
}

pub fn mathClassificationString(mcl: MathClassification) -> String {
    todo!()
}

pub fn sizeClassificationString(scl: SizeClassification) -> String {
    todo!()
}

pub fn classify(op: Arc<Operator>) -> (SizeClassification, MathClassification) {
    todo!()
}

pub fn classifyAddition(op: Arc<Operator>) -> SizeClassification {
    todo!()
}

pub fn fromClassification(cl: (SizeClassification, MathClassification), ty: Arc<Type::NFType>) -> Arc<Operator> {
    todo!()
}

pub fn getMathClassification(op: Arc<Operator>) -> MathClassification {
    todo!()
}

pub fn getSizeClassification(op: Arc<Operator>) -> SizeClassification {
    todo!()
}

pub fn combineSizeClassification(scl1: SizeClassification, scl2: SizeClassification) -> SizeClassification {
    todo!()
}

pub fn isDashClassification(mcl: MathClassification) -> bool {
    todo!()
}

pub fn isCommutative(operator: Arc<Operator>) -> bool {
    todo!()
}

pub fn isSoftCommutative(operator: Arc<Operator>) -> bool {
    todo!()
}

pub fn repetition(operator: Arc<Operator>) -> (bool, bool) {
    todo!()
}

pub fn reduction(operator: Arc<Operator>) -> bool {
    todo!()
}

pub fn isCombineable(op1: Arc<Operator>, op2: Arc<Operator>) -> bool {
    todo!()
}

pub fn isCombineableMath(mcl1: MathClassification, mcl2: MathClassification) -> bool {
    todo!()
}

pub fn isCombineableSize(scl1: SizeClassification, scl2: SizeClassification) -> bool {
    todo!()
}

pub fn toDebugString(op: Arc<Operator>) -> String {
    todo!()
}

pub fn opToString(op: Op) -> String {
    todo!()
}


