// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
use crate::Dump;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::JSON;
use openmodelica_util::List;
use openmodelica_util::Util;

pub enum NFSubscript {
    RAW_SUBSCRIPT {
        subscript: Arc<Absyn::Subscript>,
    },
    UNTYPED {
        exp: Arc<Expression::NFExpression>,
    },
    INDEX {
        index: Arc<Expression::NFExpression>,
    },
    SLICE {
        slice: Arc<Expression::NFExpression>,
    },
    EXPANDED_SLICE {
        indices: metamodelica::List<Arc<Subscript>>,
    },
    WHOLE,
    SPLIT_PROXY {
        origin: Arc<InstNode::InstNode>,
        parent: Arc<InstNode::InstNode>,
    },
    SPLIT_INDEX {
        node: Arc<InstNode::InstNode>,
        dimIndex: i32,
    },
}
pub use NFSubscript::*;
pub fn fromExp(exp: Arc<Expression::NFExpression>) -> Arc<Subscript> {
    todo!()
}

pub fn fromTypedExp(exp: Arc<Expression::NFExpression>) -> Arc<Subscript> {
    todo!()
}

pub fn toExp(subscript: Arc<Subscript>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn toInteger(subscript: Arc<Subscript>) -> i32 {
    todo!()
}

pub fn toIntegerOpt(subscript: Arc<Subscript>) -> Option<i32> {
    todo!()
}

pub fn toIndexList(subscript: Arc<Subscript>, length: i32) -> metamodelica::List<i32> {
    todo!()
}

fn isValidIndexType(ty: Arc<Type::NFType>) -> bool {
    todo!()
}

pub fn makeIndex(exp: Arc<Expression::NFExpression>) -> Arc<Subscript> {
    todo!()
}

pub fn makeSplitIndex(node: Arc<InstNode::InstNode>, dimIndex: i32) -> Arc<Subscript> {
    todo!()
}

pub fn isIndex(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isWhole(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isSimple(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isSliced(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isScalar(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isScalarLiteral(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn equalsIterator(sub: Arc<Subscript>, iterator: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

pub fn isIterator(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn toIterator(sub: Arc<Subscript>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn isBackendIterator(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isEqual(subscript1: Arc<Subscript>, subscript2: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isEqualList(subscripts1: metamodelica::List<Arc<Subscript>>, subscripts2: metamodelica::List<Arc<Subscript>>) -> bool {
    todo!()
}

pub fn compare(subscript1: Arc<Subscript>, subscript2: Arc<Subscript>) -> i32 {
    todo!()
}

pub fn compareList(subscripts1: metamodelica::List<Arc<Subscript>>, subscripts2: metamodelica::List<Arc<Subscript>>) -> i32 {
    todo!()
}

pub fn containsExp(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn listContainsExp(subscripts: metamodelica::List<Arc<Subscript>>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn containsExpShallow(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn listContainsExpShallow(subscripts: metamodelica::List<Arc<Subscript>>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn applyExp(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpShallow(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn mapExp(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Subscript> {
    todo!()
}

pub fn mapShallowExp(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Subscript> {
    todo!()
}

pub fn foldExp<ArgT>(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn mapFoldExp<ArgT>(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (Arc<Subscript>, ArgT) {
    todo!()
}

pub fn mapFoldExpShallow<ArgT>(subscript: Arc<Subscript>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (Arc<Subscript>, ArgT) {
    todo!()
}

pub fn toAbsyn(subscript: Arc<Subscript>) -> Arc<Absyn::Subscript> {
    todo!()
}

pub fn toDAE(subscript: Arc<Subscript>) -> Arc<DAE::Subscript> {
    todo!()
}

pub fn toString(subscript: Arc<Subscript>) -> String {
    todo!()
}

pub fn toStringList(subscripts: metamodelica::List<Arc<Subscript>>) -> String {
    todo!()
}

pub fn toFlatString(subscript: Arc<Subscript>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn toFlatStringList(subscripts: metamodelica::List<Arc<Subscript>>, format: BaseModelica::OutputFormat, escapeQuotes: bool) -> String {
    todo!()
}

pub fn toJSON(subscript: Arc<Subscript>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn toJSONList(subscripts: metamodelica::List<Arc<Subscript>>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn eval(subscript: Arc<Subscript>, target: Arc<EvalTarget::EvalTarget>) -> Arc<Subscript> {
    todo!()
}

pub fn simplify(subscript: Arc<Subscript>, dimension: Arc<Dimension::NFDimension>) -> Arc<Subscript> {
    todo!()
}

pub fn simplifySlice(slice: Arc<Expression::NFExpression>, dimension: Arc<Dimension::NFDimension>) -> Arc<Subscript> {
    todo!()
}

pub fn simplifyList(subscripts: metamodelica::List<Arc<Subscript>>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, trim: bool) -> metamodelica::List<Arc<Subscript>> {
    todo!()
}

pub fn toDimension(subscript: Arc<Subscript>) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn fromDimension(dimension: Arc<Dimension::NFDimension>) -> Arc<Subscript> {
    todo!()
}

pub fn scalarize(subscript: Arc<Subscript>, dimension: Arc<Dimension::NFDimension>, resize: bool) -> metamodelica::List<Arc<Subscript>> {
    todo!()
}

pub fn scalarizeList(subscripts: metamodelica::List<Arc<Subscript>>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, resize: bool) -> metamodelica::List<metamodelica::List<Arc<Subscript>>> {
    todo!()
}

pub fn expand(subscript: Arc<Subscript>, dimension: Arc<Dimension::NFDimension>, resize: bool) -> (Arc<Subscript>, bool) {
    todo!()
}

pub fn expandSlice(subscript: Arc<Subscript>, resize: bool) -> (Arc<Subscript>, bool) {
    todo!()
}

pub fn expandList(subscripts: metamodelica::List<Arc<Subscript>>, dimensions: metamodelica::List<Arc<Dimension::NFDimension>>, resize: bool) -> metamodelica::List<Arc<Subscript>> {
    todo!()
}

pub fn variability(subscript: Arc<Subscript>) -> Variability {
    todo!()
}

pub fn variabilityList(subscripts: metamodelica::List<Arc<Subscript>>) -> Variability {
    todo!()
}

pub fn purity(subscript: Arc<Subscript>) -> Purity {
    todo!()
}

pub fn purityList(subscripts: metamodelica::List<Arc<Subscript>>) -> Purity {
    todo!()
}

pub fn mergeList(newSubs: metamodelica::List<Arc<Subscript>>, oldSubs: metamodelica::List<Arc<Subscript>>, dimensions: i32, backend: bool) -> (metamodelica::List<Arc<Subscript>>, metamodelica::List<Arc<Subscript>>) {
    todo!()
}

pub fn nth(dim: Arc<Dimension::NFDimension>, i: i32) -> Arc<Subscript> {
    todo!()
}

pub fn first(dim: Arc<Dimension::NFDimension>) -> Arc<Subscript> {
    todo!()
}

pub fn isFirst(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isSplit(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isSplitIndex(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isSplitClassProxy(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn isSplitFromOrigin(sub: Arc<Subscript>, origin: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

pub fn expandSplitIndices(subs: metamodelica::List<Arc<Subscript>>, indicesToKeep: metamodelica::List<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<Subscript>> {
    todo!()
}

pub fn hash(sub: Arc<Subscript>) -> i32 {
    todo!()
}

pub fn hashContinue(sub: Arc<Subscript>, hash: i32) -> i32 {
    todo!()
}

pub fn splitIndexDimExp(sub: Arc<Subscript>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn isLiteral(sub: Arc<Subscript>) -> bool {
    todo!()
}

pub fn fillWithWholeLeft(subs: metamodelica::List<Arc<Subscript>>, targetLength: i32) -> metamodelica::List<Arc<Subscript>> {
    todo!()
}


