// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
use crate::ElementSource;
use crate::InstUtil;
use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFFunctionDerivative as FunctionDerivative;
use crate::NFFunctionInverse as FunctionInverse;
use crate::NFInst as Inst;
use crate::NFInst::InstSettings;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFLookupState::LookupState;
use crate::NFModifier::Modifier;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use crate::NFTyping::ClassScope;
use crate::SCode;
use crate::SCodeUtil;
use metamodelica::Dangerous::listReverseInPlace;
use openmodelica_util::Array;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Graph;
use openmodelica_util::IOStream;
use openmodelica_util::List;
use openmodelica_util::Pointer;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub mod Function {
    use super::*;
    pub struct FUNCTION {
        pub path: Arc<Absyn::Path>,
        pub node: Arc<InstNode::InstNode>,
        pub inputs: metamodelica::List<Arc<InstNode::InstNode>>,
        pub outputs: metamodelica::List<Arc<InstNode::InstNode>>,
        pub locals: metamodelica::List<Arc<InstNode::InstNode>>,
        pub interfaceDiffInfo: Option<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>,
        pub slots: metamodelica::List<Arc<Slot::Slot>>,
        pub returnType: Arc<Type::NFType>,
        pub attributes: DAE::FunctionAttributes,
        pub derivatives: metamodelica::List<Arc<FunctionDerivative::NFFunctionDerivative>>,
        pub derivedInputs: metamodelica::List<i32>,
        pub inverses: Vec<Arc<FunctionInverse::NFFunctionInverse>>,
        pub status: Pointer::Pointer<FunctionStatus>,
        pub callCounter: Pointer::Pointer<i32>,
    }

    pub type Function = FUNCTION;
    pub fn new(path: Arc<Absyn::Path>, node: Arc<InstNode::InstNode>, comments: metamodelica::List<Arc<SCode::Comment>>) -> Arc<Function> {
        todo!()
    }

    pub fn lookupFunctionSimple(functionName: String, scope: Arc<InstNode::InstNode>, context: i32) -> Arc<ComponentRef::NFComponentRef> {
        todo!()
    }

    pub fn lookupFunction(functionName: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<ComponentRef::NFComponentRef> {
        todo!()
    }

    pub fn instFunction(functionName: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, bool) {
        todo!()
    }

    pub fn instFunctionRef(fn_ref: Arc<ComponentRef::NFComponentRef>, context: i32, info: SourceInfo) -> (Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, bool) {
        todo!()
    }

    pub fn instFunctionNode(node: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<InstNode::InstNode> {
        todo!()
    }

    pub fn instFunction2(fnPath: Arc<Absyn::Path>, fnNode: Arc<InstNode::InstNode>, context: i32, info: SourceInfo, parent: Arc<InstNode::InstNode>) -> (Arc<InstNode::InstNode>, bool) {
        todo!()
    }

    pub fn instFunction3(fnNode: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<InstNode::InstNode>, metamodelica::List<Arc<SCode::Comment>>) {
        todo!()
    }

    pub fn makeEnumConversionOp(enumNode: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    pub fn getCachedFuncs(inNode: Arc<InstNode::InstNode>) -> metamodelica::List<Arc<Function>> {
        todo!()
    }

    pub fn mapCachedFuncs(inNode: Arc<InstNode::InstNode>, mapFn: fn(Arc<Function>) -> Arc<Function>) -> () {
        todo!()
    }

    pub fn isEvaluated(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn markEvaluated(r#fn: Arc<Function>) -> () {
        todo!()
    }

    pub fn isSimplified(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn markSimplified(r#fn: Arc<Function>) -> () {
        todo!()
    }

    pub fn isCollected(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn collect(r#fn: Arc<Function>) -> () {
        todo!()
    }

    pub fn name(r#fn: Arc<Function>) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn setName(name: Arc<Absyn::Path>, r#fn: Arc<Function>) -> Arc<Function> {
        todo!()
    }

    pub fn nameConsiderBuiltin(r#fn: Arc<Function>) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn nameEqual(fn1: Arc<Function>, fn2: Arc<Function>) -> bool {
        todo!()
    }

    pub fn nameHash(r#fn: Arc<Function>) -> i32 {
        todo!()
    }

    pub fn signatureString(r#fn: Arc<Function>, printTypes: bool) -> String {
        todo!()
    }

    pub fn candidateFuncListString(fns: metamodelica::List<Arc<Function>>) -> String {
        todo!()
    }

    pub fn callString(r#fn: Arc<Function>, posArgs: metamodelica::List<Arc<Expression::NFExpression>>, namedArgs: metamodelica::List<(Arc<Expression::NFExpression>, String)>) -> String {
        todo!()
    }

    pub fn typeString(r#fn: Arc<Function>) -> String {
        todo!()
    }

    pub fn toStream(r#fn: Arc<Function>, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
        todo!()
    }

    pub fn toFlatStream(r#fn: Arc<Function>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream, overrideName: String) -> IOStream::IOStream {
        todo!()
    }

    pub fn toFlatString(r#fn: Arc<Function>, format: BaseModelica::OutputFormat, indent: String) -> String {
        todo!()
    }

    pub fn instance(r#fn: Arc<Function>) -> Arc<InstNode::InstNode> {
        todo!()
    }

    pub fn returnType(r#fn: Arc<Function>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn setReturnType(ty: Arc<Type::NFType>, r#fn: Arc<Function>) -> Arc<Function> {
        todo!()
    }

    pub fn getSlots(r#fn: Arc<Function>) -> metamodelica::List<Arc<Slot::Slot>> {
        todo!()
    }

    pub fn fillArgs(posArgs: metamodelica::List<Arc<TypedArg>>, namedArgs: metamodelica::List<Arc<TypedArg>>, r#fn: Arc<Function>, context: i32, info: SourceInfo) -> (metamodelica::List<Arc<TypedArg>>, bool) {
        todo!()
    }

    pub fn fillNamedArg(arg: Arc<TypedArg>, slots: Vec<Arc<Slot::Slot>>, r#fn: Arc<Function>, info: SourceInfo) -> (Vec<Arc<Slot::Slot>>, bool) {
        todo!()
    }

    pub fn collectArgs(slots: Vec<Arc<Slot::Slot>>, context: i32, info: SourceInfo) -> (metamodelica::List<Arc<TypedArg>>, bool) {
        todo!()
    }

    pub fn fillDefaultSlot(slot: Arc<Slot::Slot>, slots: Vec<Arc<Slot::Slot>>, context: i32, info: SourceInfo) -> Arc<TypedArg> {
        todo!()
    }

    pub fn fillDefaultSlot2(slot: Arc<Slot::Slot>, slots: Vec<Arc<Slot::Slot>>, context: i32, info: SourceInfo) -> Arc<TypedArg> {
        todo!()
    }

    pub fn evaluateSlotExp(exp: Arc<Expression::NFExpression>, slots: Vec<Arc<Slot::Slot>>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
        todo!()
    }

    pub fn evaluateSlotExp_traverser(exp: Arc<Expression::NFExpression>, slots: Vec<Arc<Slot::Slot>>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
        todo!()
    }

    pub fn evaluateSlotCref(crefExp: Arc<Expression::NFExpression>, slots: Vec<Arc<Slot::Slot>>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
        todo!()
    }

    pub fn applyCrefSubs(cref: Arc<ComponentRef::NFComponentRef>, exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
        todo!()
    }

    pub fn lookupSlotInArray(node: Arc<InstNode::InstNode>, slots: Vec<Arc<Slot::Slot>>) -> Option<Arc<Slot::Slot>> {
        todo!()
    }

    pub fn matchArgs(func: Arc<Function>, args: metamodelica::List<Arc<TypedArg>>, info: SourceInfo, vectorize: bool) -> (metamodelica::List<Arc<TypedArg>>, Arc<FunctionMatchKind::FunctionMatchKind>) {
        todo!()
    }

    pub fn matchArgVectorized(argExp: Arc<Expression::NFExpression>, argTy: Arc<Type::NFType>, inputTy: Arc<Type::NFType>, vectArg: Arc<Expression::NFExpression>, vectDims: metamodelica::List<Arc<Dimension::NFDimension>>, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Arc<Expression::NFExpression>, metamodelica::List<Arc<Dimension::NFDimension>>, MatchKind) {
        todo!()
    }

    pub fn fillUnknownVectorizedDims(dims: metamodelica::List<Arc<Dimension::NFDimension>>, argExp: Arc<Expression::NFExpression>) -> metamodelica::List<Arc<Dimension::NFDimension>> {
        todo!()
    }

    pub fn matchFunction(func: Arc<Function>, args: metamodelica::List<Arc<TypedArg>>, named_args: metamodelica::List<Arc<TypedArg>>, context: i32, info: SourceInfo, vectorize: bool) -> (metamodelica::List<Arc<TypedArg>>, Arc<FunctionMatchKind::FunctionMatchKind>) {
        todo!()
    }

    pub fn matchFunctions(funcs: metamodelica::List<Arc<Function>>, args: metamodelica::List<Arc<TypedArg>>, named_args: metamodelica::List<Arc<TypedArg>>, context: i32, info: SourceInfo, vectorize: bool) -> metamodelica::List<Arc<MatchedFunction::MatchedFunction>> {
        todo!()
    }

    pub fn matchFunctionsSilent(funcs: metamodelica::List<Arc<Function>>, args: metamodelica::List<Arc<TypedArg>>, named_args: metamodelica::List<Arc<TypedArg>>, context: i32, info: SourceInfo, vectorize: bool) -> metamodelica::List<Arc<MatchedFunction::MatchedFunction>> {
        todo!()
    }

    pub fn isTyped(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn typeRefCache(functionRef: Arc<ComponentRef::NFComponentRef>, context: i32) -> metamodelica::List<Arc<Function>> {
        todo!()
    }

    pub fn typeNodeCache(functionNode: Arc<InstNode::InstNode>, context: i32) -> metamodelica::List<Arc<Function>> {
        todo!()
    }

    pub fn getRefCache(fnRef: Arc<ComponentRef::NFComponentRef>) -> metamodelica::List<Arc<Function>> {
        todo!()
    }

    pub fn typeFunction(r#fn: Arc<Function>, context: i32) -> Arc<Function> {
        todo!()
    }

    pub fn typeFunctionSignature(r#fn: Arc<Function>, context: i32) -> Arc<Function> {
        todo!()
    }

    pub fn typeFunctionBody(r#fn: Arc<Function>, context: i32) -> Arc<Function> {
        todo!()
    }

    pub fn checkPureCall(exp: Arc<Expression::NFExpression>, r#fn: Arc<Function>, pure: bool) -> bool {
        todo!()
    }

    pub fn boxFunctionParameter(component: Arc<InstNode::InstNode>) -> () {
        todo!()
    }

    pub fn typePartialApplication(exp: Arc<Expression::NFExpression>, context: i32, info: SourceInfo) -> (Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity) {
        todo!()
    }

    pub fn makePartialApplicationFromSlots(slotsArray: Vec<Arc<Slot::Slot>>, r#fn: Arc<Function>, fnRef: Arc<ComponentRef::NFComponentRef>, info: SourceInfo) -> Arc<Expression::NFExpression> {
        todo!()
    }

    pub fn isBuiltin(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn isBuiltinAttr(attrs: DAE::FunctionAttributes) -> bool {
        todo!()
    }

    pub fn isSpecialBuiltin(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn isSubscriptableBuiltin(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn isImpure(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn isFunctionPointer(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn setFunctionPointer(isPointer: bool, r#fn: Arc<Function>) -> Arc<Function> {
        todo!()
    }

    pub fn isExternal(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn isExternalObjectConstructorOrDestructor(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn isPartialDerivative(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn getDerivedInputNames(r#fn: Arc<Function>) -> metamodelica::List<String> {
        todo!()
    }

    pub fn getDerivedFunctionName(r#fn: Arc<Function>) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn inlineBuiltin(r#fn: Arc<Function>) -> DAE::InlineType {
        todo!()
    }

    pub fn isDefaultRecordConstructor(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn isNonDefaultRecordConstructor(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn toDAE(r#fn: Arc<Function>, def: DAE::FunctionDefinition) -> DAE::Function {
        todo!()
    }

    pub fn makeDAEType(r#fn: Arc<Function>, boxTypes: bool) -> Arc<DAE::Type> {
        todo!()
    }

    pub fn getBody(r#fn: Arc<Function>) -> metamodelica::List<Arc<Statement::NFStatement>> {
        todo!()
    }

    pub fn hasUnboxArgs(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn hasUnboxArgsAnnotation(cmt: Arc<SCode::Comment>) -> bool {
        todo!()
    }

    pub fn hasOptionalArgument(component: Arc<SCode::Element>) -> bool {
        todo!()
    }

    pub fn mapExp(r#fn: Arc<Function>, mapFn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>, mapFnFields: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>, mapParameters: bool, mapBody: bool) -> Arc<Function> {
        todo!()
    }

    pub fn mapExpParameter(node: Arc<InstNode::InstNode>, mapFn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>, mapFnFields: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> () {
        todo!()
    }

    pub fn mapBody(r#fn: Arc<Function>, mapFn: fn(Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm>) -> Arc<Function> {
        todo!()
    }

    pub fn foldExp<ArgT>(r#fn: Arc<Function>, foldFn: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT, mapParameters: bool, mapBody: bool) -> ArgT {
        todo!()
    }

    pub fn foldExpParameter<ArgT>(node: Arc<InstNode::InstNode>, foldFn: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
        todo!()
    }

    pub fn isPartial(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn getLocalArguments(r#fn: Arc<Function>) -> metamodelica::List<Arc<Expression::NFExpression>> {
        todo!()
    }

    fn collectParams(node: Arc<InstNode::InstNode>, inputs: metamodelica::List<Arc<InstNode::InstNode>>, outputs: metamodelica::List<Arc<InstNode::InstNode>>, locals: metamodelica::List<Arc<InstNode::InstNode>>) -> (metamodelica::List<Arc<InstNode::InstNode>>, metamodelica::List<Arc<InstNode::InstNode>>, metamodelica::List<Arc<InstNode::InstNode>>) {
        todo!()
    }

    fn paramDirection(component: Arc<InstNode::InstNode>, checkVisibility: bool) -> Direction {
        todo!()
    }

    fn makeSlots(inputs: metamodelica::List<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<Slot::Slot>> {
        todo!()
    }

    fn makeSlot(component: Arc<InstNode::InstNode>, index: i32) -> Arc<Slot::Slot> {
        todo!()
    }

    fn hasOMPure(cmt: Arc<SCode::Comment>) -> bool {
        todo!()
    }

    fn getBuiltinPtr(cmt: Arc<SCode::Comment>) -> DAE::FunctionBuiltin {
        todo!()
    }

    fn mergeFunctionAnnotations(comments: metamodelica::List<Arc<SCode::Comment>>) -> Arc<SCode::Comment> {
        todo!()
    }

    fn makeAttributes(node: Arc<InstNode::InstNode>, inputs: metamodelica::List<Arc<InstNode::InstNode>>, outputs: metamodelica::List<Arc<InstNode::InstNode>>, comments: metamodelica::List<Arc<SCode::Comment>>) -> DAE::FunctionAttributes {
        todo!()
    }

    fn checkParamTypes(r#fn: Arc<Function>) -> () {
        todo!()
    }

    fn checkParamTypes2(params: metamodelica::List<Arc<InstNode::InstNode>>) -> () {
        todo!()
    }

    fn isValidParamType(ty: Arc<Type::NFType>) -> bool {
        todo!()
    }

    fn isValidParamState(cls: Arc<InstNode::InstNode>) -> bool {
        todo!()
    }

    fn checkPartialDerivativeTypes(r#fn: Arc<Function>) -> () {
        todo!()
    }

    pub fn makeReturnType(r#fn: Arc<Function>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn getBody2(node: Arc<InstNode::InstNode>) -> metamodelica::List<Arc<Statement::NFStatement>> {
        todo!()
    }

    pub fn hasSingleOrEmptyBody(r#fn: Arc<Function>) -> bool {
        todo!()
    }

    pub fn analyseUnusedParameters(r#fn: Arc<Function>) -> metamodelica::List<i32> {
        todo!()
    }

    pub fn analyseUnusedParametersExp(exp: Arc<Expression::NFExpression>, params: metamodelica::List<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn analyseUnusedParametersExp2(exp: Arc<Expression::NFExpression>, params: metamodelica::List<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn sortLocals(locals: metamodelica::List<Arc<InstNode::InstNode>>, info: SourceInfo) -> metamodelica::List<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getLocalDependencies(node: Arc<InstNode::InstNode>, locals: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>) -> metamodelica::List<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getLocalDependencies2(node: Arc<InstNode::InstNode>, locals: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>, dependencies: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>) -> UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getLocalDependenciesExp(exp: Arc<Expression::NFExpression>, locals: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>, deps: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>) -> UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getLocalDependenciesExp2(exp: Arc<Expression::NFExpression>, locals: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>, deps: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>) -> UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getLocalDependenciesDim(dim: Arc<Dimension::NFDimension>, locals: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>, deps: UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>) -> UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>> {
        todo!()
    }

    pub fn getDerivative(original: Arc<Function>, interface_map: UnorderedMap::UnorderedMap<bool, String>) -> Option<Arc<Function>> {
        todo!()
    }

    pub fn checkUseBeforeAssign(r#fn: Arc<Function>) -> () {
        todo!()
    }

    pub fn addUnassignedComponents(unassigned: Vector::Vector<Arc<InstNode::InstNode>>, variables: metamodelica::List<Arc<InstNode::InstNode>>) -> () {
        todo!()
    }

    pub fn checkUseBeforeAssign2(unassigned: Vector::Vector<Arc<InstNode::InstNode>>, statements: metamodelica::List<Arc<Statement::NFStatement>>) -> () {
        todo!()
    }

    pub fn markAssignedOutput(unassigned: Vector::Vector<Arc<InstNode::InstNode>>, assignedExp: Arc<Expression::NFExpression>) -> () {
        todo!()
    }

    pub fn checkUseBeforeAssignIf(unassigned: Vector::Vector<Arc<InstNode::InstNode>>, branches: metamodelica::List<(metamodelica::List<Arc<Statement::NFStatement>>, Arc<Expression::NFExpression>)>, info: SourceInfo) -> () {
        todo!()
    }

    pub fn checkUseBeforeAssignExp(unassigned: Vector::Vector<Arc<InstNode::InstNode>>, exp: Arc<Expression::NFExpression>, info: SourceInfo) -> () {
        todo!()
    }

    pub fn checkUseBeforeAssignExp_traverse(unassigned: Vector::Vector<Arc<InstNode::InstNode>>, exp: Arc<Expression::NFExpression>, info: SourceInfo) -> () {
        todo!()
    }

    pub fn instPartialDerivedVars(classDef: Arc<SCode::ClassDef>, inputs: metamodelica::List<Arc<InstNode::InstNode>>, r#fn: Arc<Function>, context: i32, info: SourceInfo) -> metamodelica::List<i32> {
        todo!()
    }

}

pub mod FunctionMatchKind {
    use super::*;
    pub enum FunctionMatchKind {
        EXACT,
        CAST,
        GENERIC,
        VECTORIZED {
            vectDims: metamodelica::List<Arc<Dimension::NFDimension>>,
            vectorizedArgs: metamodelica::List<i32>,
            baseMatch: Arc<FunctionMatchKind>,
        },
        NOT_COMPATIBLE,
    }
    pub use FunctionMatchKind::*;
    pub fn isValid(mk: Arc<FunctionMatchKind>) -> bool {
        todo!()
    }

    pub fn isExact(mk: Arc<FunctionMatchKind>) -> bool {
        todo!()
    }

    pub fn isVectorized(mk: Arc<FunctionMatchKind>) -> bool {
        todo!()
    }

    pub fn isExactVectorized(mk: Arc<FunctionMatchKind>) -> bool {
        todo!()
    }

}

pub enum FunctionStatus {
    BUILTIN,
    INITIAL,
    EVALUATED,
    SIMPLIFIED,
    COLLECTED,
}

pub mod MatchedFunction {
    use super::*;
    pub struct MATCHED_FUNC {
        pub func: Arc<Function::Function>,
        pub args: metamodelica::List<Arc<TypedArg>>,
        pub mk: Arc<FunctionMatchKind::FunctionMatchKind>,
    }

    pub type MatchedFunction = MATCHED_FUNC;
    pub fn getExactMatches(matchedFunctions: metamodelica::List<Arc<MatchedFunction>>) -> metamodelica::List<Arc<MatchedFunction>> {
        todo!()
    }

    pub fn getExactVectorizedMatches(matchedFunctions: metamodelica::List<Arc<MatchedFunction>>) -> metamodelica::List<Arc<MatchedFunction>> {
        todo!()
    }

    pub fn isVectorized(mf: Arc<MatchedFunction>) -> bool {
        todo!()
    }

}

pub type NamedArg = (Arc<Expression::NFExpression>, String);

pub mod Slot {
    use super::*;
    pub struct SLOT {
        pub node: Arc<InstNode::InstNode>,
        pub ty: SlotType,
        pub default: Option<Arc<Expression::NFExpression>>,
        pub arg: Option<Arc<TypedArg>>,
        pub index: i32,
        pub evalStatus: SlotEvalStatus,
    }

    pub type Slot = SLOT;
    pub fn positional(slot: Arc<Slot>) -> bool {
        todo!()
    }

    pub fn named(slot: Arc<Slot>) -> bool {
        todo!()
    }

    pub fn name(slot: Arc<Slot>) -> String {
        todo!()
    }

    pub fn hasNode(node: Arc<InstNode::InstNode>, slot: Arc<Slot>) -> bool {
        todo!()
    }

}

pub enum SlotEvalStatus {
    NOT_EVALUATED,
    EVALUATING,
    EVALUATED,
}

pub enum SlotType {
    POSITIONAL,
    NAMED,
    GENERIC,
}

pub struct TYPED_ARG {
    pub name: Option<String>,
    pub value: Arc<Expression::NFExpression>,
    pub ty: Arc<Type::NFType>,
    pub var: Variability,
    pub purity: Purity,
}

pub type TypedArg = TYPED_ARG;

