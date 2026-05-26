// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn::Path;
use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableKind;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClockKind as ClockKind;
use crate::NFComplexType as ComplexType;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction as Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFVariable as Variable;
use crate::Values;
use crate::ValuesUtil;
use metamodelica::Dangerous::*;
use openmodelica_util::Array;
use openmodelica_util::Flags;
use openmodelica_util::JSON;
use openmodelica_util::List;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub enum NFExpression {
    INTEGER {
        value: i32,
    },
    REAL {
        value: f64,
    },
    STRING {
        value: String,
    },
    BOOLEAN {
        value: bool,
    },
    ENUM_LITERAL {
        ty: Arc<Type::NFType>,
        name: String,
        index: i32,
    },
    CLKCONST {
        clk: Arc<ClockKind::NFClockKind>,
    },
    CREF {
        ty: Arc<Type::NFType>,
        cref: Arc<ComponentRef::NFComponentRef>,
    },
    TYPENAME {
        ty: Arc<Type::NFType>,
    },
    ARRAY {
        ty: Arc<Type::NFType>,
        elements: Vec<Arc<Expression>>,
        literal: bool,
    },
    MATRIX {
        elements: metamodelica::List<metamodelica::List<Arc<Expression>>>,
    },
    RANGE {
        ty: Arc<Type::NFType>,
        start: Arc<Expression>,
        step: Option<Arc<Expression>>,
        stop: Arc<Expression>,
    },
    TUPLE {
        ty: Arc<Type::NFType>,
        elements: metamodelica::List<Arc<Expression>>,
    },
    RECORD {
        path: Arc<Path>,
        ty: Arc<Type::NFType>,
        elements: metamodelica::List<Arc<Expression>>,
    },
    CALL {
        call: Arc<Call::NFCall>,
    },
    SIZE {
        exp: Arc<Expression>,
        dimIndex: Option<Arc<Expression>>,
    },
    END,
    BINARY {
        exp1: Arc<Expression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<Expression>,
    },
    UNARY {
        operator: Arc<Operator::NFOperator>,
        exp: Arc<Expression>,
    },
    LBINARY {
        exp1: Arc<Expression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<Expression>,
    },
    LUNARY {
        operator: Arc<Operator::NFOperator>,
        exp: Arc<Expression>,
    },
    RELATION {
        exp1: Arc<Expression>,
        operator: Arc<Operator::NFOperator>,
        exp2: Arc<Expression>,
        index: i32,
    },
    MULTARY {
        arguments: metamodelica::List<Arc<Expression>>,
        inv_arguments: metamodelica::List<Arc<Expression>>,
        operator: Arc<Operator::NFOperator>,
    },
    IF {
        ty: Arc<Type::NFType>,
        condition: Arc<Expression>,
        trueBranch: Arc<Expression>,
        falseBranch: Arc<Expression>,
    },
    CAST {
        ty: Arc<Type::NFType>,
        exp: Arc<Expression>,
    },
    BOX {
        exp: Arc<Expression>,
    },
    UNBOX {
        exp: Arc<Expression>,
        ty: Arc<Type::NFType>,
    },
    SUBSCRIPTED_EXP {
        exp: Arc<Expression>,
        subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>,
        ty: Arc<Type::NFType>,
        split: bool,
    },
    TUPLE_ELEMENT {
        tupleExp: Arc<Expression>,
        index: i32,
        ty: Arc<Type::NFType>,
    },
    RECORD_ELEMENT {
        recordExp: Arc<Expression>,
        index: i32,
        fieldName: String,
        ty: Arc<Type::NFType>,
    },
    MUTABLE {
        exp: Mutable::Mutable<Arc<Expression>>,
    },
    EMPTY {
        ty: Arc<Type::NFType>,
    },
    PARTIAL_FUNCTION_APPLICATION {
        r#fn: Arc<ComponentRef::NFComponentRef>,
        args: metamodelica::List<Arc<Expression>>,
        argNames: metamodelica::List<String>,
        ty: Arc<Type::NFType>,
    },
    FILENAME {
        filename: String,
    },
    SHARED_LITERAL {
        index: i32,
        exp: Arc<Expression>,
    },
    INSTANCE_NAME {
        scope: Arc<InstNode::InstNode>,
    },
}
pub use NFExpression::*;
pub fn isArray(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isEmptyArray(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isVector(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isCref(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isFunctionInputCref(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isWildCref(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isCall(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isImpureCall(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isExternalCall(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isCallNamed(exp: Arc<Expression>, name: String) -> bool {
    todo!()
}

pub fn isConnectionCall(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isTrue(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isAllTrue(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isFalse(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isTrivialCref(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn hash(exp: Arc<Expression>) -> i32 {
    todo!()
}

pub fn hashContinue(exp: Arc<Expression>, hash: i32) -> i32 {
    todo!()
}

pub fn isEqual(exp1: Arc<Expression>, exp2: Arc<Expression>) -> bool {
    todo!()
}

pub fn compare(exp1: Arc<Expression>, exp2: Arc<Expression>) -> i32 {
    todo!()
}

pub fn compareOpt(expl1: Option<Arc<Expression>>, expl2: Option<Arc<Expression>>) -> i32 {
    todo!()
}

pub fn compareList(expl1: metamodelica::List<Arc<Expression>>, expl2: metamodelica::List<Arc<Expression>>) -> i32 {
    todo!()
}

pub fn typeOf(exp: Arc<Expression>) -> Arc<Type::NFType> {
    todo!()
}

pub fn sizeOf(exp: Arc<Expression>) -> i32 {
    todo!()
}

pub fn sizeZero(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn setType(ty: Arc<Type::NFType>, exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn applyToType(exp: Arc<Expression>, func: fn(Arc<Type::NFType>) -> Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn typeCastOpt(exp: Option<Arc<Expression>>, ty: Arc<Type::NFType>) -> Option<Arc<Expression>> {
    todo!()
}

pub fn typeCast(exp: Arc<Expression>, ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn typeCastGeneric(exp: Arc<Expression>, ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn realValue(exp: Arc<Expression>) -> f64 {
    todo!()
}

pub fn makeReal(value: f64) -> Arc<Expression> {
    todo!()
}

pub fn integerValue(exp: Arc<Expression>) -> i32 {
    todo!()
}

pub fn integerValueOrDefault(exp: Arc<Expression>, value: i32) -> i32 {
    todo!()
}

pub fn makeInteger(value: i32) -> Arc<Expression> {
    todo!()
}

pub fn stringValue(exp: Arc<Expression>) -> String {
    todo!()
}

pub fn booleanValue(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn makeArray(ty: Arc<Type::NFType>, expl: Vec<Arc<Expression>>, literal: bool) -> Arc<Expression> {
    todo!()
}

pub fn makeArrayCheckLiteral(ty: Arc<Type::NFType>, expl: Vec<Arc<Expression>>) -> Arc<Expression> {
    todo!()
}

pub fn makeEmptyArray(ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeIntegerArray(values: metamodelica::List<i32>) -> Arc<Expression> {
    todo!()
}

pub fn makeRealArray(values: metamodelica::List<f64>) -> Arc<Expression> {
    todo!()
}

pub fn makeRealMatrix(values: metamodelica::List<metamodelica::List<f64>>) -> Arc<Expression> {
    todo!()
}

pub fn makeExpArray(elements: Vec<Arc<Expression>>, elementType: Arc<Type::NFType>, isLiteral: bool) -> Arc<Expression> {
    todo!()
}

pub fn makeRecord(recordName: Arc<Path>, recordType: Arc<Type::NFType>, fields: metamodelica::List<Arc<Expression>>) -> Arc<Expression> {
    todo!()
}

pub fn makeRange(start: Arc<Expression>, step: Option<Arc<Expression>>, stop: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn makeIntegerRange(start: i32, step: i32, stop: i32) -> Arc<Expression> {
    todo!()
}

pub fn getIntegerRange(range: Arc<Expression>, resize: bool) -> (i32, i32, i32) {
    todo!()
}

pub fn getInteger(exp: Arc<Expression>, resize: bool) -> i32 {
    todo!()
}

pub fn makeTuple(expl: metamodelica::List<Arc<Expression>>) -> Arc<Expression> {
    todo!()
}

pub fn rangeSize(range: Arc<Expression>, resize: bool) -> i32 {
    todo!()
}

pub fn rangeSizeExp(range: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn applySubscripts(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, exp: Arc<Expression>, applyToScope: bool) -> Arc<Expression> {
    todo!()
}

pub fn applySubscript(subscript: Arc<Subscript::NFSubscript>, exp: Arc<Expression>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, applyToScope: bool) -> Arc<Expression> {
    todo!()
}

pub fn applySubscriptCref(subscript: Arc<Subscript::NFSubscript>, cref: Arc<ComponentRef::NFComponentRef>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, applyToScope: bool) -> Arc<Expression> {
    todo!()
}

pub fn applySubscriptTypename(subscript: Arc<Subscript::NFSubscript>, ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn applyIndexSubscriptTypename(ty: Arc<Type::NFType>, index: Arc<Subscript::NFSubscript>) -> Arc<Expression> {
    todo!()
}

pub fn applySubscriptArray(subscript: Arc<Subscript::NFSubscript>, exp: Arc<Expression>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, applyToScope: bool) -> Arc<Expression> {
    todo!()
}

pub fn typeSubscriptedArray(elements: Vec<Arc<Expression>>, subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, ty: Arc<Type::NFType>, literal: bool) -> (Arc<Type::NFType>, bool) {
    todo!()
}

pub fn applyIndexSubscriptArray(exp: Arc<Expression>, index: Arc<Subscript::NFSubscript>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Expression> {
    todo!()
}

pub fn applyIndexExpArray(exp: Arc<Expression>, index: Arc<Expression>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Expression> {
    todo!()
}

pub fn applySubscriptRange(subscript: Arc<Subscript::NFSubscript>, exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn applyIndexSubscriptRange(rangeExp: Arc<Expression>, index: Arc<Subscript::NFSubscript>) -> Arc<Expression> {
    todo!()
}

pub fn applyIndexSubscriptRange2(startExp: Arc<Expression>, stepExp: Option<Arc<Expression>>, stopExp: Arc<Expression>, index: i32) -> Arc<Expression> {
    todo!()
}

pub fn applySubscriptCall(subscript: Arc<Subscript::NFSubscript>, exp: Arc<Expression>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, applyToScope: bool) -> Arc<Expression> {
    todo!()
}

pub fn applySubscriptArrayConstructor(subscript: Arc<Subscript::NFSubscript>, call: Arc<Call::NFCall>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>) -> Arc<Expression> {
    todo!()
}

pub fn applyIndexSubscriptArrayConstructor(call: Arc<Call::NFCall>, index: Arc<Subscript::NFSubscript>) -> Arc<Expression> {
    todo!()
}

pub fn applySubscriptIf(subscript: Arc<Subscript::NFSubscript>, exp: Arc<Expression>, restSubscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, applyToScope: bool) -> Arc<Expression> {
    todo!()
}

pub fn makeSubscriptedExp(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, exp: Arc<Expression>, backend: bool) -> Arc<Expression> {
    todo!()
}

pub fn replaceIterator(exp: Arc<Expression>, iterator: Arc<InstNode::InstNode>, iteratorValue: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn replaceIterator2(exp: Arc<Expression>, iterator: Arc<InstNode::InstNode>, iteratorValue: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn containsIterator(exp: Arc<Expression>, iterator: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

pub fn arrayFromList(inExps: metamodelica::List<Arc<Expression>>, elemTy: Arc<Type::NFType>, inDims: metamodelica::List<Arc<Dimension::NFDimension>>) -> Arc<Expression> {
    todo!()
}

pub fn arrayFromList_impl(inExps: metamodelica::List<Arc<Expression>>, elemTy: Arc<Type::NFType>, inDims: metamodelica::List<Arc<Dimension::NFDimension>>) -> Arc<Expression> {
    todo!()
}

pub fn makeEnumLiteral(enumType: Arc<Type::NFType>, index: i32) -> Arc<Expression> {
    todo!()
}

pub fn makeEnumLiterals(enumType: Arc<Type::NFType>) -> metamodelica::List<Arc<Expression>> {
    todo!()
}

pub fn isIntegerValue(exp: Arc<Expression>, value: i32) -> bool {
    todo!()
}

pub fn toInteger(exp: Arc<Expression>) -> i32 {
    todo!()
}

pub fn toStringTyped(exp: Arc<Expression>) -> String {
    todo!()
}

pub fn toString(exp: Arc<Expression>) -> String {
    todo!()
}

pub fn toFlatString(exp: Arc<Expression>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn operandString(operand: Arc<Expression>, operator: Arc<Expression>, lhs: bool) -> String {
    todo!()
}

pub fn operandFlatString(operand: Arc<Expression>, operator: Arc<Expression>, lhs: bool, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn multaryString(arguments: metamodelica::List<Arc<Expression>>, exp: Arc<Expression>, operator: Arc<Operator::NFOperator>, parenthesize: bool) -> String {
    todo!()
}

pub fn multaryFlatString(arguments: metamodelica::List<Arc<Expression>>, exp: Arc<Expression>, operator: Arc<Operator::NFOperator>, format: BaseModelica::OutputFormat, parenthesize: bool) -> String {
    todo!()
}

pub fn priority(exp: Arc<Expression>, lhs: bool) -> i32 {
    todo!()
}

pub fn isAssociativeExp(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isNonAssociativeExp(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn getName(exp: Arc<Expression>) -> String {
    todo!()
}

pub fn enumLiteralPath(exp: Arc<Expression>) -> Arc<Path> {
    todo!()
}

pub fn getNominal(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn computeNominal(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn toAbsyn(exp: Arc<Expression>) -> Arc<Absyn::Exp> {
    todo!()
}

pub fn toDAE(exp: Arc<Expression>, allowEmpty: bool) -> Arc<DAE::Exp> {
    todo!()
}

pub fn toDAERecord(ty: Arc<Type::NFType>, path: Arc<Path>, args: metamodelica::List<Arc<Expression>>) -> Arc<DAE::Exp> {
    todo!()
}

pub fn toDAEValue(exp: Arc<Expression>) -> Arc<Values::Value> {
    todo!()
}

pub fn toDAEValueRecord(ty: Arc<Type::NFType>, path: Arc<Path>, args: metamodelica::List<Arc<Expression>>) -> Arc<Values::Value> {
    todo!()
}

pub fn dimensionCount(exp: Arc<Expression>) -> i32 {
    todo!()
}

pub fn dimensions(exp: Arc<Expression>) -> metamodelica::List<Arc<Dimension::NFDimension>> {
    todo!()
}

pub fn map(exp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn fakeMap(exp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn mapOpt(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Option<Arc<Expression>> {
    todo!()
}

pub fn mapReverse(exp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn mapShallow(exp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn mapShallowOpt(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Option<Arc<Expression>> {
    todo!()
}

pub fn mapArrayElements(exp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn foldArray<ArgT>(expl: Vec<Arc<Expression>>, func: fn(Arc<Expression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn foldList<ArgT>(expl: metamodelica::List<Arc<Expression>>, func: fn(Arc<Expression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn foldOpt<ArgT>(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn fold<ArgT>(exp: Arc<Expression>, func: fn(Arc<Expression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn applyArray(expl: Vec<Arc<Expression>>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn applyList(expl: metamodelica::List<Arc<Expression>>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn applyOpt(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn apply(exp: Arc<Expression>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn applyArrayShallow(expl: Vec<Arc<Expression>>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn applyListShallow(expl: metamodelica::List<Arc<Expression>>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn applyShallow(exp: Arc<Expression>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn applyShallowOpt(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>) -> ()) -> () {
    todo!()
}

pub fn mapFold<ArgT>(exp: Arc<Expression>, func: fn(Arc<Expression>, ArgT) -> (Arc<Expression>, ArgT), arg: ArgT) -> (Arc<Expression>, ArgT) {
    todo!()
}

pub fn mapFoldOpt<ArgT>(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>, ArgT) -> (Arc<Expression>, ArgT), arg: ArgT) -> (Option<Arc<Expression>>, ArgT) {
    todo!()
}

pub fn mapFoldShallow<ArgT>(exp: Arc<Expression>, func: fn(Arc<Expression>, ArgT) -> (Arc<Expression>, ArgT), arg: ArgT) -> (Arc<Expression>, ArgT) {
    todo!()
}

pub fn mapFoldOptShallow<ArgT>(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>, ArgT) -> (Arc<Expression>, ArgT), arg: ArgT) -> (Option<Arc<Expression>>, ArgT) {
    todo!()
}

pub fn containsOpt(exp: Option<Arc<Expression>>, func: fn(Arc<Expression>) -> bool) -> bool {
    todo!()
}

pub fn contains(exp: Arc<Expression>, func: fn(Arc<Expression>) -> bool) -> bool {
    todo!()
}

pub fn arrayContains(expl: Vec<Arc<Expression>>, func: fn(Arc<Expression>) -> bool) -> bool {
    todo!()
}

pub fn listContains(expl: metamodelica::List<Arc<Expression>>, func: fn(Arc<Expression>) -> bool) -> bool {
    todo!()
}

pub fn containsShallow(exp: Arc<Expression>, func: fn(Arc<Expression>) -> bool) -> bool {
    todo!()
}

pub fn arrayFirstScalar(arrayExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn arrayAllEqual(arrayExp: Arc<Expression>) -> bool {
    todo!()
}

pub fn arrayAllEqual2(arrayExp: Arc<Expression>, element: Arc<Expression>) -> bool {
    todo!()
}

pub fn fromCref(cref: Arc<ComponentRef::NFComponentRef>, includeScope: bool) -> Arc<Expression> {
    todo!()
}

pub fn toCref(exp: Arc<Expression>) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn extractCrefs(exp: Arc<Expression>) -> UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

pub fn extractCref(exp: Arc<Expression>, crefs: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

pub fn isResizableCref(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isIterator(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn containsAnyIterator(exp: Arc<Expression>, context: i32) -> bool {
    todo!()
}

pub fn isTime(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isSubstitute(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isZero(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isNonZero(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isOne(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isMinusOne(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isNaN(nan: Arc<Expression>) -> bool {
    todo!()
}

pub fn isPositive(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isNegative(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isNonPositive(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isNonNegative(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isGreaterOrEqual(lhs: Arc<Expression>, rhs: Arc<Expression>) -> bool {
    todo!()
}

pub fn hasArrayType(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isScalar(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isScalarLiteral(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isLiteral(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isLiteralXML(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isLiteralReplace(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isKnownSizeFill(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isInteger(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isReal(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isConstNumber(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isBoolean(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isRecord(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isRecordOrRecordArray(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn fillType(ty: Arc<Type::NFType>, fillExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn fillArgs(fillExp: Arc<Expression>, dims: metamodelica::List<Arc<Expression>>) -> Arc<Expression> {
    todo!()
}

pub fn fillArray(n: i32, fillExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn fillArray_impl(n: i32, fillExp: Arc<Expression>, ty: Arc<Type::NFType>, isLiteral: bool) -> (Arc<Expression>, Arc<Type::NFType>) {
    todo!()
}

pub fn liftArray(dim: Arc<Dimension::NFDimension>, exp: Arc<Expression>) -> (Arc<Expression>, Arc<Type::NFType>) {
    todo!()
}

pub fn liftArrayList(dims: metamodelica::List<Arc<Dimension::NFDimension>>, exp: Arc<Expression>) -> (Arc<Expression>, Arc<Type::NFType>) {
    todo!()
}

pub fn makeZero(ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeOperatorRecordZero(recordNode: Arc<InstNode::InstNode>) -> Arc<Expression> {
    todo!()
}

pub fn makeOne(ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeMinusOne(ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeNaN(ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeMaxValue(ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeMinValue(ty: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeDefaultValue(ty: Arc<Type::NFType>, min: Option<Arc<Expression>>, max: Option<Arc<Expression>>) -> Arc<Expression> {
    todo!()
}

pub fn r#box(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn unbox(boxedExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn isNegated(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn negate(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn logicNegate(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn revertRange(range: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn sliceRange(range: Arc<Expression>, slice: (i32, i32, i32)) -> Arc<Expression> {
    todo!()
}

pub fn arrayElements(array: Arc<Expression>) -> Vec<Arc<Expression>> {
    todo!()
}

pub fn arrayElementList(array: Arc<Expression>) -> metamodelica::List<Arc<Expression>> {
    todo!()
}

pub fn arrayScalarElements(exp: Arc<Expression>) -> metamodelica::List<Arc<Expression>> {
    todo!()
}

pub fn arrayScalarElements_impl(exp: Arc<Expression>, elements: metamodelica::List<Arc<Expression>>) -> metamodelica::List<Arc<Expression>> {
    todo!()
}

pub fn arrayScalarElement(arrayExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn hasArrayCall(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn hasArrayCall2(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn transposeArray(arrayExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn makeIdentityMatrix(n: i32, elementType: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn makeTriuMask(n: i32, elTy: Arc<Type::NFType>) -> Arc<Expression> {
    todo!()
}

pub fn promote(e: Arc<Expression>, ty: Arc<Type::NFType>, n: i32) -> (Arc<Expression>, Arc<Type::NFType>) {
    todo!()
}

pub fn promote2(exp: Arc<Expression>, isArray: bool, dims: i32, types: metamodelica::List<Arc<Type::NFType>>) -> Arc<Expression> {
    todo!()
}

pub fn variability(exp: Arc<Expression>) -> Variability {
    todo!()
}

pub fn variabilityArray(expl: Vec<Arc<Expression>>, var: Variability) -> Variability {
    todo!()
}

pub fn variabilityList(expl: metamodelica::List<Arc<Expression>>, var: Variability) -> Variability {
    todo!()
}

pub fn purity(exp: Arc<Expression>) -> Purity {
    todo!()
}

pub fn purityArray(expl: Vec<Arc<Expression>>, pur: Purity) -> Purity {
    todo!()
}

pub fn purityList(expl: metamodelica::List<Arc<Expression>>, pur: Purity) -> Purity {
    todo!()
}

pub fn makeMutable(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn makeImmutable(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn isMutable(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn updateMutable(mutableExp: Arc<Expression>, value: Arc<Expression>) -> () {
    todo!()
}

pub fn applyMutable(mutableExp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> () {
    todo!()
}

pub fn isEmpty(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isEnd(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn enumIndexExp(enumExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn toScalar(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn tupleElement(exp: Arc<Expression>, ty: Arc<Type::NFType>, index: i32) -> Arc<Expression> {
    todo!()
}

pub fn recordElement(elementName: String, recordExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn nthRecordElement(index: i32, recordExp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn getRecordElements(exp: Arc<Expression>) -> metamodelica::List<Arc<Expression>> {
    todo!()
}

pub fn retype(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn nthEnumLiteral(ty: Arc<Type::NFType>, n: i32) -> Arc<Expression> {
    todo!()
}

pub fn createIterationRanges(exp: Arc<Expression>, iterators: metamodelica::List<(Arc<Expression>, Arc<InstNode::InstNode>)>) -> (Arc<Expression>, metamodelica::List<Arc<Expression>>, metamodelica::List<Mutable::Mutable<Arc<Expression>>>) {
    todo!()
}

pub fn foldReduction(exp: Arc<Expression>, iterators: metamodelica::List<(Arc<Expression>, Arc<InstNode::InstNode>)>, foldExp: Arc<Expression>, mapFn: fn(Arc<Expression>) -> Arc<Expression>, foldFn: fn(Arc<Expression>, Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn foldReduction2(exp: Arc<Expression>, ranges: metamodelica::List<Arc<Expression>>, iterators: metamodelica::List<Mutable::Mutable<Arc<Expression>>>, foldExp: Arc<Expression>, mapFn: fn(Arc<Expression>) -> Arc<Expression>, foldFn: fn(Arc<Expression>, Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn isPure(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn containsCref(exp: Arc<Expression>, cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

pub fn isCrefEqual(exp: Arc<Expression>, b: bool, cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    todo!()
}

pub fn containsCrefSet(exp: Arc<Expression>, set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

pub fn isCrefEqualSet(exp: Arc<Expression>, b: bool, set: UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>) -> bool {
    todo!()
}

pub fn filterSplitIndices(exp: Arc<Expression>, node: Arc<InstNode::InstNode>) -> Arc<Expression> {
    todo!()
}

pub fn filterSplitIndices2(sub: Arc<Subscript::NFSubscript>, node: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

pub fn expandSplitIndices(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn expandNonListedSplitIndices(exp: Arc<Expression>, indicesToKeep: metamodelica::List<Arc<InstNode::InstNode>>) -> Arc<Expression> {
    todo!()
}

pub fn isSplitSubscriptedExp(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn mapSplitExpressions(exp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn replaceSplitSubscripts(exp: Arc<Expression>, subRepls: Option<UnorderedMap::UnorderedMap<Arc<Expression>, Arc<Subscript::NFSubscript>>>) -> (Arc<Expression>, Option<UnorderedMap::UnorderedMap<Arc<Expression>, Arc<Subscript::NFSubscript>>>) {
    todo!()
}

pub fn replaceSplitSubscripts2(subscript: Arc<Subscript::NFSubscript>, subRepls: Option<UnorderedMap::UnorderedMap<Arc<Expression>, Arc<Subscript::NFSubscript>>>) -> (Arc<Subscript::NFSubscript>, Option<UnorderedMap::UnorderedMap<Arc<Expression>, Arc<Subscript::NFSubscript>>>) {
    todo!()
}

pub fn mapSplitExpressions2(exp: Arc<Expression>, dimSizes: metamodelica::List<Arc<Expression>>, subExps: metamodelica::List<Arc<Expression>>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn mapSplitExpressions3(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn mapCrefScalars(crefExp: Arc<Expression>, mapFn: fn(Arc<ComponentRef::NFComponentRef>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn mapCrefScalars2(exp: Arc<Expression>, mapFn: fn(Arc<ComponentRef::NFComponentRef>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn isFunctionPointer(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isClockOrSampleFunction(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isConnector(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn isComponentExpression(exp: Arc<Expression>) -> bool {
    todo!()
}

pub fn clone(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn toJSON(exp: Arc<Expression>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn tupleElements(exp: Arc<Expression>) -> metamodelica::List<Arc<Expression>> {
    todo!()
}

pub fn wrapCall(exp: Arc<Expression>, fun: fn(Arc<Call::NFCall>) -> Arc<Call::NFCall>) -> Arc<Expression> {
    todo!()
}

pub fn repairOperator(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn makeUnary(op: Arc<Operator::NFOperator>, exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn replaceLiteral(exp: Arc<Expression>, map: UnorderedMap::UnorderedMap<i32, Arc<Expression>>, idx_ptr: Pointer::Pointer<i32>) -> Arc<Expression> {
    todo!()
}

pub fn replaceLiteralArrayElements(exp: Arc<Expression>, map: UnorderedMap::UnorderedMap<i32, Arc<Expression>>, idx_ptr: Pointer::Pointer<i32>) -> Arc<Expression> {
    todo!()
}

pub fn replaceCrefWithBinding(cref: Arc<ComponentRef::NFComponentRef>, exp: Arc<Expression>, func: fn(Arc<Expression>) -> Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn replaceResizableParameterWithOriginal(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn replaceResizableParameter(exp: Arc<Expression>) -> Arc<Expression> {
    todo!()
}

pub fn mulResultType(tl: Arc<Type::NFType>, tr: Arc<Type::NFType>) -> Arc<Type::NFType> {
    todo!()
}

pub fn mmul(lhs: Arc<Expression>, rhs: Arc<Expression>, baseOp: Arc<Operator::NFOperator>) -> Arc<Expression> {
    todo!()
}

pub fn productOfListExceptSelf(arguments: metamodelica::List<Arc<Expression>>, mulOp: Arc<Operator::NFOperator>) -> metamodelica::List<Arc<Expression>> {
    todo!()
}


