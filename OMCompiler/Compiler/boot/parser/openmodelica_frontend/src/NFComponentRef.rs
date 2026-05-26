// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::BaseModelica;
use crate::DAE;
use crate::NFBackendExtension::Annotations;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use metamodelica::Dangerous::*;
use openmodelica_util::JSON;
use openmodelica_util::List;

pub enum NFComponentRef {
    CREF {
        node: Arc<InstNode::InstNode>,
        subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>,
        ty: Arc<Type::NFType>,
        origin: Origin,
        restCref: Arc<ComponentRef>,
    },
    EMPTY,
    WILD,
}
pub use NFComponentRef::*;
pub enum Origin {
    CREF,
    SCOPE,
    ITERATOR,
}

pub fn fromNode(node: Arc<InstNode::InstNode>, ty: Arc<Type::NFType>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>, origin: Origin) -> Arc<ComponentRef> {
    todo!()
}

pub fn prefixCref(node: Arc<InstNode::InstNode>, ty: Arc<Type::NFType>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>, restCref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn prefixScope(node: Arc<InstNode::InstNode>, ty: Arc<Type::NFType>, subs: metamodelica::List<Arc<Subscript::NFSubscript>>, restCref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn fromAbsyn(node: Arc<InstNode::InstNode>, subs: metamodelica::List<Arc<Absyn::Subscript>>, restCref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn fromAbsynCref(acref: Arc<Absyn::ComponentRef>, restCref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn fromBuiltin(node: Arc<InstNode::InstNode>, ty: Arc<Type::NFType>) -> Arc<ComponentRef> {
    todo!()
}

pub fn makeIterator(node: Arc<InstNode::InstNode>, ty: Arc<Type::NFType>) -> Arc<ComponentRef> {
    todo!()
}

pub fn isWild(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isEmpty(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isSimple(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isQualified(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isTopLevel(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isCref(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isIterator(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isInput(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isOutput(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isNameNode(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isEqualRecordChild(child: Arc<ComponentRef>, recd: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isRecordChild(child: Arc<ComponentRef>, recd: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn node(cref: Arc<ComponentRef>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn nodes(cref: Arc<ComponentRef>, accum: metamodelica::List<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<InstNode::InstNode>> {
    todo!()
}

pub fn nodesIncludingSplitSubs(cref: Arc<ComponentRef>, accum: metamodelica::List<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<InstNode::InstNode>> {
    todo!()
}

pub fn containsNode(cref: Arc<ComponentRef>, node: Arc<InstNode::InstNode>) -> bool {
    todo!()
}

pub fn nodeType(cref: Arc<ComponentRef>) -> Arc<Type::NFType> {
    todo!()
}

pub fn setNodeType(ty: Arc<Type::NFType>, cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn updateNodeType(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn scalarType(cref: Arc<ComponentRef>) -> Arc<Type::NFType> {
    todo!()
}

pub fn applyToType(cref: Arc<ComponentRef>, func: fn(Arc<Type::NFType>) -> Arc<Type::NFType>) -> Arc<ComponentRef> {
    todo!()
}

pub fn firstName(cref: Arc<ComponentRef>, baseModelica: bool) -> String {
    todo!()
}

pub fn first(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn rest(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn last(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn firstNonScope(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn append(cref: Arc<ComponentRef>, restCref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn appendScope(scope: Arc<InstNode::InstNode>, cref: Arc<ComponentRef>, includeRoot: bool) -> Arc<ComponentRef> {
    todo!()
}

pub fn prepend(restCref: Arc<ComponentRef>, cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn getComponentType(cref: Arc<ComponentRef>) -> Arc<Type::NFType> {
    todo!()
}

pub fn getSubscriptedType(cref: Arc<ComponentRef>, includeScope: bool) -> Arc<Type::NFType> {
    todo!()
}

pub fn getSubscriptedType2(restCref: Arc<ComponentRef>, accumTy: Arc<Type::NFType>, includeScope: bool) -> Arc<Type::NFType> {
    todo!()
}

pub fn lookupVarAttr(cref: Arc<ComponentRef>, attr_name: String) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn nodeVariability(cref: Arc<ComponentRef>) -> Variability {
    todo!()
}

pub fn isResizable(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn subscriptsVariability(cref: Arc<ComponentRef>, var: Variability) -> Variability {
    todo!()
}

pub fn variability(cref: Arc<ComponentRef>) -> Variability {
    todo!()
}

pub fn purity(cref: Arc<ComponentRef>) -> Purity {
    todo!()
}

pub fn visibility(cref: Arc<ComponentRef>) -> Visibility {
    todo!()
}

pub fn rename(name: String, cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn addSubscript(subscript: Arc<Subscript::NFSubscript>, cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn mergeSubscripts(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, cref: Arc<ComponentRef>, applyToScope: bool, backend: bool, reverse: bool) -> Arc<ComponentRef> {
    todo!()
}

pub fn mergeSubscripts2(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, cref: Arc<ComponentRef>, applyToScope: bool, backend: bool, reverse: bool) -> (metamodelica::List<Arc<Subscript::NFSubscript>>, Arc<ComponentRef>) {
    todo!()
}

pub fn mergeSubscriptsMapped(cref: Arc<ComponentRef>, dims_map: UnorderedMap::UnorderedMap<metamodelica::List<Arc<ComponentRef>>, metamodelica::List<Arc<Dimension::NFDimension>>>, iter_map: UnorderedMap::UnorderedMap<Arc<Subscript::NFSubscript>, Arc<ComponentRef>>) -> Arc<ComponentRef> {
    todo!()
}

pub fn hasSubscripts(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn hasNonModelSubscripts(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn hasSplitSubscripts(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn expandSplitSubscripts(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn getSubscripts(cref: Arc<ComponentRef>) -> metamodelica::List<Arc<Subscript::NFSubscript>> {
    todo!()
}

pub fn setSubscripts(subscripts: metamodelica::List<Arc<Subscript::NFSubscript>>, cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn setSubscriptsList(subscripts: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>, cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn copySubscripts(origin: Arc<ComponentRef>, target: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn subscriptsAllWithWhole(cref: Arc<ComponentRef>, accumSubs: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    todo!()
}

pub fn subscriptsAllWithWholeFlat(cref: Arc<ComponentRef>) -> metamodelica::List<Arc<Subscript::NFSubscript>> {
    todo!()
}

pub fn subscriptsAll(cref: Arc<ComponentRef>) -> metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    todo!()
}

pub fn subscriptsAllReverse(cref: Arc<ComponentRef>, accumSubs: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    todo!()
}

pub fn subscriptsAllFlat(cref: Arc<ComponentRef>) -> metamodelica::List<Arc<Subscript::NFSubscript>> {
    todo!()
}

pub fn subscriptsExceptModel(cref: Arc<ComponentRef>, accumSubs: metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    todo!()
}

pub fn subscriptsN(cref: Arc<ComponentRef>, n: i32) -> metamodelica::List<metamodelica::List<Arc<Subscript::NFSubscript>>> {
    todo!()
}

pub fn transferSubscripts(srcCref: Arc<ComponentRef>, dstCref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn applySubscripts(cref: Arc<ComponentRef>, func: fn(Arc<Subscript::NFSubscript>) -> (), applyToScope: bool) -> () {
    todo!()
}

pub fn foldSubscripts<ArgT>(cref: Arc<ComponentRef>, func: fn(Arc<Subscript::NFSubscript>, ArgT) -> ArgT, arg: ArgT, applyToScope: bool) -> ArgT {
    todo!()
}

pub fn mapSubscripts(cref: Arc<ComponentRef>, func: fn(Arc<Subscript::NFSubscript>) -> Arc<Subscript::NFSubscript>, applyToScope: bool) -> Arc<ComponentRef> {
    todo!()
}

pub fn fillSubscripts(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn replaceWholeSubscripts(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn combineSubscripts(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn compare(cref1: Arc<ComponentRef>, cref2: Arc<ComponentRef>) -> i32 {
    todo!()
}

pub fn isEqual(cref1: Arc<ComponentRef>, cref2: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isEqualStrip(cref1: Arc<ComponentRef>, cref2: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isLess(cref1: Arc<ComponentRef>, cref2: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isGreater(cref1: Arc<ComponentRef>, cref2: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isPrefix(cref1: Arc<ComponentRef>, cref2: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn toAbsyn(cref: Arc<ComponentRef>) -> Arc<Absyn::ComponentRef> {
    todo!()
}

pub fn toAbsyn_impl(cref: Arc<ComponentRef>, accumCref: Arc<Absyn::ComponentRef>) -> Arc<Absyn::ComponentRef> {
    todo!()
}

pub fn toDAE(cref: Arc<ComponentRef>) -> Arc<DAE::ComponentRef> {
    todo!()
}

pub fn toDAE_impl(cref: Arc<ComponentRef>, accumCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    todo!()
}

pub fn toString(cref: Arc<ComponentRef>) -> String {
    todo!()
}

pub fn toString_impl(cref: Arc<ComponentRef>, strl: metamodelica::List<String>) -> metamodelica::List<String> {
    todo!()
}

pub fn toFlatString(cref: Arc<ComponentRef>, format: BaseModelica::OutputFormat) -> String {
    todo!()
}

pub fn listToString(crs: metamodelica::List<Arc<ComponentRef>>) -> String {
    todo!()
}

pub fn toJSON(cref: Arc<ComponentRef>) -> Arc<JSON::JSON> {
    todo!()
}

pub fn toJSON_impl(cref: Arc<ComponentRef>, accum: metamodelica::List<Arc<JSON::JSON>>) -> metamodelica::List<Arc<JSON::JSON>> {
    todo!()
}

pub fn toJSON_context(node: Arc<InstNode::InstNode>, accum: metamodelica::List<Arc<JSON::JSON>>) -> metamodelica::List<Arc<JSON::JSON>> {
    todo!()
}

pub fn hash(cref: Arc<ComponentRef>) -> i32 {
    todo!()
}

pub fn hashStrip(cref: Arc<ComponentRef>) -> i32 {
    todo!()
}

pub fn hashContinue(cref: Arc<ComponentRef>, strip: bool, hash: i32) -> i32 {
    todo!()
}

pub fn toPath(cref: Arc<ComponentRef>) -> Arc<Absyn::Path> {
    todo!()
}

pub fn toPath_impl(cref: Arc<ComponentRef>, accumPath: Arc<Absyn::Path>) -> Arc<Absyn::Path> {
    todo!()
}

pub fn fromNodeList(nodes: metamodelica::List<Arc<InstNode::InstNode>>) -> Arc<ComponentRef> {
    todo!()
}

pub fn scalarize(cref: Arc<ComponentRef>, resize: bool) -> metamodelica::List<Arc<ComponentRef>> {
    todo!()
}

pub fn scalarizeAll(cref: Arc<ComponentRef>, resize: bool) -> metamodelica::List<Arc<ComponentRef>> {
    todo!()
}

pub fn scalarizeAll_Nesting(nested_crefs: metamodelica::List<metamodelica::List<Arc<ComponentRef>>>, cref: Arc<ComponentRef>, crefs: metamodelica::List<Arc<ComponentRef>>) -> metamodelica::List<Arc<ComponentRef>> {
    todo!()
}

pub fn scalarizeSlice(cref: Arc<ComponentRef>, slice: metamodelica::List<i32>, resize: bool) -> metamodelica::List<Arc<ComponentRef>> {
    todo!()
}

pub fn isPackageConstant(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isPackageConstant2(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn stripSubscripts(cref: Arc<ComponentRef>) -> (Arc<ComponentRef>, metamodelica::List<Arc<Subscript::NFSubscript>>) {
    todo!()
}

pub fn stripSubscriptsAll(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn stripSubscriptsExceptModel(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn stripIteratorSubscripts(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn simplifySubscripts(cref: Arc<ComponentRef>, trim: bool) -> Arc<ComponentRef> {
    todo!()
}

pub fn evaluateSubscripts(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn isDeleted(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isFromCref(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn toListReverse(cref: Arc<ComponentRef>, includeScope: bool, accum: metamodelica::List<Arc<ComponentRef>>) -> metamodelica::List<Arc<ComponentRef>> {
    todo!()
}

pub fn depth(cref: Arc<ComponentRef>) -> i32 {
    todo!()
}

pub fn size(cref: Arc<ComponentRef>, withComplex: bool, resize: bool) -> i32 {
    todo!()
}

pub fn sizes(cref: Arc<ComponentRef>, withComplex: bool, resize: bool, s_lst: metamodelica::List<i32>) -> metamodelica::List<i32> {
    todo!()
}

pub fn sizes_local(cref: Arc<ComponentRef>, withComplex: bool, resize: bool) -> metamodelica::List<i32> {
    todo!()
}

pub fn sizes_local_exp(cref: Arc<ComponentRef>, withComplex: bool) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn sizeKnown(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn subscriptsToInteger(cref: Arc<ComponentRef>) -> metamodelica::List<i32> {
    todo!()
}

pub fn subscriptsToExpression(cref: Arc<ComponentRef>, addScalar: bool) -> metamodelica::List<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn isEmptyArray(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isComplexArray(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isComplexArray2(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn containsExp(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn containsExpShallow(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>) -> bool) -> bool {
    todo!()
}

pub fn applyExp(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn applyExpShallow(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>) -> ()) -> () {
    todo!()
}

pub fn mapExp(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<ComponentRef> {
    todo!()
}

pub fn mapExpShallow(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<ComponentRef> {
    todo!()
}

pub fn foldExp<ArgT>(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn mapFoldExp<ArgT>(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (Arc<ComponentRef>, ArgT) {
    todo!()
}

pub fn mapFoldExpShallow<ArgT>(cref: Arc<ComponentRef>, func: fn(Arc<Expression::NFExpression>, ArgT) -> (Arc<Expression::NFExpression>, ArgT), arg: ArgT) -> (Arc<ComponentRef>, ArgT) {
    todo!()
}

pub fn isTime(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isSubstitute(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn isDiscrete(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn removeOuterCrefPrefix(cref: Arc<ComponentRef>) -> Arc<ComponentRef> {
    todo!()
}

pub fn mapTypes(cref: Arc<ComponentRef>, func: fn(Arc<Type::NFType>) -> Arc<Type::NFType>) -> Arc<ComponentRef> {
    todo!()
}

pub fn mapNodes(cref: Arc<ComponentRef>, func: fn(Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode>) -> Arc<ComponentRef> {
    todo!()
}

pub fn getArrayCrefOpt(scal: Arc<ComponentRef>) -> Option<Arc<ComponentRef>> {
    todo!()
}

pub fn isSliced(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn hasImplicitTrailingIndex(cref: Arc<ComponentRef>) -> bool {
    todo!()
}

pub fn iterate(cref: Arc<ComponentRef>) -> (Arc<ComponentRef>, metamodelica::List<(Arc<Expression::NFExpression>, Arc<InstNode::InstNode>)>) {
    todo!()
}

pub fn getRecordChildren(cref: Arc<ComponentRef>) -> metamodelica::List<Arc<ComponentRef>> {
    todo!()
}


