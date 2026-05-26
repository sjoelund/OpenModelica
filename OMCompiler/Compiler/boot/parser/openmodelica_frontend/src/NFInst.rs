// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn::Path;
use crate::Absyn;
use crate::AbsynUtil;
use crate::DAE;
use crate::ElementSource;
use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFCall as Call;
use crate::NFCheckModel as CheckModel;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectBreakTree as ConnectBreakTree;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFConvertDAE as ConvertDAE;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEvalConstants as EvalConstants;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFInstNode::NodeTree;
use crate::NFInstUtil as InstUtil;
use crate::NFLookup as Lookup;
use crate::NFModifier::Modifier;
use crate::NFModifier::ModifierScope;
use crate::NFOperator as Operator;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPackage as Package;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFScalarize as Scalarize;
use crate::NFSections::Sections;
use crate::NFSimplifyModel as SimplifyModel;
use crate::NFStateMachineFlatten as StateMachineFlatten;
use crate::NFStatement as Statement;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFUnitCheck as UnitCheck;
use crate::NFVariable as Variable;
use crate::NFVerifyModel as VerifyModel;
use crate::SCode;
use crate::SCodeDump;
use crate::SCodeUtil;
use metamodelica::Dangerous::listReverseInPlace;
use metamodelica::Dangerous;
use openmodelica_util::Array;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::FlagsUtil;
use openmodelica_util::List;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;

pub enum ExtendsVisibility {
    PUBLIC,
    DERIVED_PROTECTED,
    PROTECTED,
}

pub mod InstSettings {
    use super::*;
    pub struct SETTINGS {
        pub mergeExtendsSections: bool,
        pub resizableArrays: bool,
    }

    pub type InstSettings = SETTINGS;
    pub fn create() -> Arc<InstSettings> {
        todo!()
    }

}

pub fn Inst_makeTopNode(program: metamodelica::List<Arc<SCode::Element>>, annotationProgram: metamodelica::List<Arc<SCode::Element>>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn addIteratorToScope(name: String, scope: Arc<InstNode::InstNode>, info: SourceInfo, iter_type: Arc<Type::NFType>) -> (Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn applyExtendsVisibility(node: Arc<InstNode::InstNode>, visibility: ExtendsVisibility) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn applyModifier(modifier: Arc<Modifier::Modifier>, cls: Arc<ClassTree::ClassTree>, parent: Arc<InstNode::InstNode>, context: i32) -> Arc<ClassTree::ClassTree> {
    todo!()
}

pub fn checkAssignmentRestriction(lhs: Arc<Expression::NFExpression>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkBindingRestriction(restriction: Arc<Restriction::NFRestriction>, binding: Arc<Binding::NFBinding>, component: Arc<InstNode::InstNode>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkBuiltinTypeExtends(builtinExtends: Arc<InstNode::InstNode>, tree: Arc<ClassTree::ClassTree>, node: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn checkElementNotReplaceable(node: Arc<InstNode::InstNode>) -> () {
    todo!()
}

pub fn checkExtendsLoop(node: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, path: Arc<Path>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkExternalDeclLanguage(language: String, info: SourceInfo) -> () {
    todo!()
}

pub fn checkInstanceRestriction(node: Arc<InstNode::InstNode>, path: Arc<Path>, context: i32) -> () {
    todo!()
}

pub fn checkIteratorShadowing(name: String, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkOuterComponentMod(node: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn checkPartialClass(node: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn checkPartialComponent(compNode: Arc<InstNode::InstNode>, compAttr: Arc<Attributes::NFAttributes>, clsNode: Arc<InstNode::InstNode>, isPartial: bool, res: Arc<Restriction::NFRestriction>, context: i32, info: SourceInfo) -> () {
    todo!()
}

pub fn checkRecursiveDefinition(componentType: Arc<InstNode::InstNode>, component: Arc<InstNode::InstNode>, limitReached: bool) -> () {
    todo!()
}

pub fn checkReplaceableBaseClass(baseClasses: metamodelica::List<Arc<InstNode::InstNode>>, basePath: Arc<Path>, info: SourceInfo) -> () {
    todo!()
}

pub fn checkTopLevelOuter(name: String, outerNode: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn checkUnsubscriptableCref(cref: Arc<ComponentRef::NFComponentRef>, info: SourceInfo) -> () {
    todo!()
}

pub fn clearCaches() -> () {
    todo!()
}

pub fn expand(node: Arc<InstNode::InstNode>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn expandClass(node: Arc<InstNode::InstNode>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn expandClass2(node: Arc<InstNode::InstNode>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn expandClassDerived(element: Arc<SCode::Element>, definition: Arc<SCode::ClassDef>, node: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn expandClassDerivedComplex(element: Arc<SCode::Element>, definition: Arc<SCode::ClassDef>, node: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn expandClassParts(def: Arc<SCode::Element>, node: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> (Arc<InstNode::InstNode>, Option<UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>>) {
    todo!()
}

pub fn expandExtends(ext: Arc<InstNode::InstNode>, builtinExt: Arc<InstNode::InstNode>, context: i32, nameMap: UnorderedMap::UnorderedMap<Arc<Absyn::ComponentRef>, String>) -> (Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn expandExternalObject(clsTree: Arc<ClassTree::ClassTree>, node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn filterInstanceAPIEquations(eql: metamodelica::List<Arc<SCode::Equation>>) -> metamodelica::List<Arc<SCode::Equation>> {
    todo!()
}

pub fn getConstrainingMod(element: Arc<SCode::Element>, parent: Arc<InstNode::InstNode>, outerMod: Arc<Modifier::Modifier>) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn insertGeneratedInners(node: Arc<InstNode::InstNode>, topScope: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn instAlgorithmSection(algorithmSection: Arc<SCode::AlgorithmSection>, scope: Arc<InstNode::InstNode>, context: i32) -> Arc<Algorithm::NFAlgorithm> {
    todo!()
}

pub fn instAlgorithmSections(algorithmSections: metamodelica::List<Arc<SCode::AlgorithmSection>>, scope: Arc<InstNode::InstNode>, context: i32) -> metamodelica::List<Arc<Algorithm::NFAlgorithm>> {
    todo!()
}

pub fn instBinding(binding: Arc<Binding::NFBinding>, context: i32) -> Arc<Binding::NFBinding> {
    todo!()
}

pub fn instBuiltinAttribute(attribute: Arc<Modifier::Modifier>, node: Arc<InstNode::InstNode>, context: i32) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn instClass(node: Arc<InstNode::InstNode>, modifier: Arc<Modifier::Modifier>, attributes: Arc<Attributes::NFAttributes>, useBinding: bool, instLevel: i32, parent: Arc<InstNode::InstNode>, context: i32) -> (Arc<InstNode::InstNode>, Arc<Attributes::NFAttributes>) {
    todo!()
}

pub fn instClassDef(cls: Arc<Class::NFClass>, outerMod: Arc<Modifier::Modifier>, attributes: Arc<Attributes::NFAttributes>, useBinding: bool, node: Arc<InstNode::InstNode>, parent: Arc<InstNode::InstNode>, instLevel: i32, context: i32) -> (Arc<Attributes::NFAttributes>, Arc<InstNode::InstNode>) {
    todo!()
}

pub fn instClassForConnection(classPath: Arc<Path>, program: metamodelica::List<Arc<SCode::Element>>, annotationProgram: metamodelica::List<Arc<SCode::Element>>) -> metamodelica::List<metamodelica::List<String>> {
    todo!()
}

pub fn instClassInProgram(classPath: Arc<Path>, program: metamodelica::List<Arc<SCode::Element>>, annotationProgram: metamodelica::List<Arc<SCode::Element>>, relaxedFrontend: bool, dumpFlat: bool) -> (Arc<FlatModel::NFFlatModel>, Arc<BaseAvlTree::Tree>, String) {
    todo!()
}

pub fn instClassPrefixes(cls: Arc<SCode::Element>) -> Arc<NFClass::Prefixes::Prefixes> {
    todo!()
}

pub fn instComplexType(ty: Arc<Type::NFType>, context: i32) -> () {
    todo!()
}

pub fn instComponent(node: Arc<InstNode::InstNode>, attributes: Arc<Attributes::NFAttributes>, innerMod: Arc<Modifier::Modifier>, useBinding: bool, instLevel: i32, context: i32, originalAttr: Option<Arc<Attributes::NFAttributes>>, propagatedSubs: metamodelica::List<Arc<Subscript::NFSubscript>>) -> () {
    todo!()
}

pub fn instComponentDef(component: Arc<SCode::Element>, outerMod: Arc<Modifier::Modifier>, innerMod: Arc<Modifier::Modifier>, attributes: Arc<Attributes::NFAttributes>, useBinding: bool, node: Arc<InstNode::InstNode>, parent: Arc<InstNode::InstNode>, instLevel: i32, originalAttr: Option<Arc<Attributes::NFAttributes>>, propagatedSubs: metamodelica::List<Arc<Subscript::NFSubscript>>, context: i32) -> () {
    todo!()
}

pub fn instComponentExpressions(component: Arc<InstNode::InstNode>, context: i32, settings: Arc<InstSettings::InstSettings>) -> () {
    todo!()
}

pub fn instConnectorCref(absynCref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn instConstrainingMod(element: Arc<SCode::Element>, parent: Arc<InstNode::InstNode>) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn instCref(absynCref: Arc<Absyn::ComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn instCrefComponent(cref: Arc<ComponentRef::NFComponentRef>, node: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn instCrefFunction(cref: Arc<ComponentRef::NFComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn instCrefSubscripts(cref: Arc<ComponentRef::NFComponentRef>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<ComponentRef::NFComponentRef> {
    todo!()
}

pub fn instCrefTypename(cref: Arc<ComponentRef::NFComponentRef>, node: Arc<InstNode::InstNode>, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn instDimension(dimension: Arc<Dimension::NFDimension>, context: i32, settings: Arc<InstSettings::InstSettings>, info: SourceInfo) -> Arc<Dimension::NFDimension> {
    todo!()
}

pub fn instElementModifier(element: Arc<SCode::Element>, component: Arc<InstNode::InstNode>, parent: Arc<InstNode::InstNode>) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn instEquation(scodeEq: Arc<SCode::Equation>, scope: Arc<InstNode::InstNode>, connectBreaks: Arc<NFConnectBreakTree::Tree>, context: i32, equations: metamodelica::List<Arc<Equation::NFEquation>>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn instEquations(scodeEql: metamodelica::List<Arc<SCode::Equation>>, scope: Arc<InstNode::InstNode>, connectBreaks: Arc<NFConnectBreakTree::Tree>, context: i32) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn instExp(absynExp: Arc<Absyn::Exp>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn instExpOpt(absynExp: Option<Arc<Absyn::Exp>>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Option<Arc<Expression::NFExpression>> {
    todo!()
}

pub fn instExpressions(node: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, sections: Arc<NFSections::NFSections>, connectBreaks: Arc<NFConnectBreakTree::Tree>, context: i32, settings: Arc<InstSettings::InstSettings>) -> Arc<NFSections::NFSections> {
    todo!()
}

pub fn instExtends(node: Arc<InstNode::InstNode>, attributes: Arc<Attributes::NFAttributes>, useBinding: bool, instLevel: i32, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn instExternalDecl(extDecl: Arc<SCode::ExternalDecl>, scope: Arc<InstNode::InstNode>, context: i32) -> Arc<NFSections::NFSections> {
    todo!()
}

pub fn instExternalObjectStructors(ty: Arc<Type::NFType>, parent: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn instPackage(node: Arc<InstNode::InstNode>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn instPartEvalFunction(func: Arc<Absyn::ComponentRef>, funcArgs: Arc<Absyn::FunctionArgs>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn instRecordConstructor(node: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

pub fn instResizable(exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    todo!()
}

pub fn instSections(node: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, connectBreaks: Arc<NFConnectBreakTree::Tree>, context: i32, sections: Arc<NFSections::NFSections>) -> Arc<NFSections::NFSections> {
    todo!()
}

pub fn instSections2(parts: Arc<SCode::ClassDef>, scope: Arc<InstNode::InstNode>, connectBreaks: Arc<NFConnectBreakTree::Tree>, context: i32, sections: Arc<NFSections::NFSections>) -> Arc<NFSections::NFSections> {
    todo!()
}

pub fn instStatement(scodeStmt: Arc<SCode::Statement>, scope: Arc<InstNode::InstNode>, context: i32) -> Arc<Statement::NFStatement> {
    todo!()
}

pub fn instStatements(scodeStmtl: metamodelica::List<Arc<SCode::Statement>>, scope: Arc<InstNode::InstNode>, context: i32) -> metamodelica::List<Arc<Statement::NFStatement>> {
    todo!()
}

pub fn instSubscript(subscript: Arc<Subscript::NFSubscript>, scope: Arc<InstNode::InstNode>, context: i32, info: SourceInfo) -> Arc<Subscript::NFSubscript> {
    todo!()
}

pub fn instTypeSpec(typeSpec: Arc<Absyn::TypeSpec>, modifier: Arc<Modifier::Modifier>, attributes: Arc<Attributes::NFAttributes>, useBinding: bool, scope: Arc<InstNode::InstNode>, parent: Arc<InstNode::InstNode>, info: SourceInfo, instLevel: i32, context: i32) -> (Arc<InstNode::InstNode>, Arc<Attributes::NFAttributes>) {
    todo!()
}

pub fn instantiate(node: Arc<InstNode::InstNode>, r#mod: Arc<Modifier::Modifier>, parent: Arc<InstNode::InstNode>, context: i32, instPartial: bool) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn instantiateRootClass(clsNode: Arc<InstNode::InstNode>, context: i32, r#mod: Arc<Modifier::Modifier>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn instantiateRootFunction(funcNode: Arc<InstNode::InstNode>, context: i32) -> (Arc<FlatModel::NFFlatModel>, Arc<BaseAvlTree::Tree>, String) {
    todo!()
}

pub fn lookupRootClass(path: Arc<Path>, topScope: Arc<InstNode::InstNode>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn makeComplexType(restriction: Arc<Restriction::NFRestriction>, node: Arc<InstNode::InstNode>, cls: Arc<Class::NFClass>) -> Arc<Type::NFType> {
    todo!()
}

pub fn makeEnumerationType(literals: metamodelica::List<Arc<SCode::Enum>>, scope: Arc<InstNode::InstNode>) -> Arc<Type::NFType> {
    todo!()
}

pub fn makeExternalObjectType(tree: Arc<ClassTree::ClassTree>, node: Arc<InstNode::InstNode>) -> Arc<ComplexType::NFComplexType> {
    todo!()
}

pub fn makeRecordComplexType(node: Arc<InstNode::InstNode>, cls: Arc<Class::NFClass>) -> Arc<ComplexType::NFComplexType> {
    todo!()
}

pub fn makeSource(comment: Arc<SCode::Comment>, info: SourceInfo) -> Arc<DAE::ElementSource> {
    todo!()
}

pub fn makeTopNode(topClasses: metamodelica::List<Arc<SCode::Element>>, annotationClasses: metamodelica::List<Arc<SCode::Element>>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn markBuiltinTypeNodes(node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn markBuiltinTypeNodesByAnnotation(node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn markImplicitWhenExp(exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

pub fn markImplicitWhenExp_traverser(exp: Arc<Expression::NFExpression>) -> () {
    todo!()
}

pub fn modifyExtends(extendsNode: Arc<InstNode::InstNode>, scope: Arc<InstNode::InstNode>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn partialInstClass(node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn partialInstClass2(definition: Arc<SCode::Element>, scope: Arc<InstNode::InstNode>) -> Arc<Class::NFClass> {
    todo!()
}

pub fn propagateRedeclaredMod(r#mod: Arc<Modifier::Modifier>, component: Arc<InstNode::InstNode>) -> Arc<Modifier::Modifier> {
    todo!()
}

pub fn redeclareClass(redeclareNode: Arc<InstNode::InstNode>, originalNode: Arc<InstNode::InstNode>, outerMod: Arc<Modifier::Modifier>, constrainingMod: Arc<Modifier::Modifier>, context: i32) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn redeclareClassElement(redeclareCls: Mutable::Mutable<Arc<InstNode::InstNode>>, replaceableCls: Mutable::Mutable<Arc<InstNode::InstNode>>, context: i32) -> Mutable::Mutable<Arc<InstNode::InstNode>> {
    todo!()
}

pub fn redeclareClasses(tree: Arc<ClassTree::ClassTree>, parent: Arc<InstNode::InstNode>, context: i32) -> Arc<ClassTree::ClassTree> {
    todo!()
}

pub fn redeclareComponent(redeclareNode: Arc<InstNode::InstNode>, originalNode: Arc<InstNode::InstNode>, outerMod: Arc<Modifier::Modifier>, constrainingMod: Arc<Modifier::Modifier>, propagatedSubs: metamodelica::List<Arc<Subscript::NFSubscript>>, outerAttr: Arc<Attributes::NFAttributes>, redeclaredNode: Arc<InstNode::InstNode>, instLevel: i32, context: i32) -> () {
    todo!()
}

pub fn redeclareComponentElement(redeclareComp: Mutable::Mutable<Arc<InstNode::InstNode>>, replaceableComp: Mutable::Mutable<Arc<InstNode::InstNode>>, instLevel: i32, context: i32) -> Mutable::Mutable<Arc<InstNode::InstNode>> {
    todo!()
}

pub fn redeclareElements(chain: metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>, instLevel: i32, context: i32) -> () {
    todo!()
}

pub fn redeclareEnum(redeclareClass: Arc<Class::NFClass>, originalClass: Arc<Class::NFClass>, prefixes: Arc<NFClass::Prefixes::Prefixes>, outerMod: Arc<Modifier::Modifier>, redeclareNode: Arc<InstNode::InstNode>, originalNode: Arc<InstNode::InstNode>, context: i32) -> Arc<Class::NFClass> {
    todo!()
}

pub fn resetGlobalFlags() -> () {
    todo!()
}

pub fn updateComponentType(component: Arc<InstNode::InstNode>, cls: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    todo!()
}

pub fn updateImplicitVariability(node: Arc<InstNode::InstNode>, parentEval: bool, context: i32) -> () {
    todo!()
}

pub fn updateImplicitVariabilityAlg(alg: Arc<Algorithm::NFAlgorithm>) -> () {
    todo!()
}

pub fn updateImplicitVariabilityComp(component: Arc<InstNode::InstNode>, parentEval: bool, context: i32) -> () {
    todo!()
}

pub fn updateImplicitVariabilityEq(eq: Arc<Equation::NFEquation>, inWhen: bool) -> () {
    todo!()
}

pub fn updateImplicitVariabilityEql(eql: metamodelica::List<Arc<Equation::NFEquation>>, inWhen: bool) -> () {
    todo!()
}

pub fn updateImplicitVariabilityStmt(stmt: Arc<Statement::NFStatement>, inWhen: bool) -> () {
    todo!()
}

pub fn updateImplicitVariabilityStmts(stmtl: metamodelica::List<Arc<Statement::NFStatement>>, inWhen: bool) -> () {
    todo!()
}

pub fn updateParameterBinding(node: Arc<InstNode::InstNode>, context: i32) -> () {
    todo!()
}

