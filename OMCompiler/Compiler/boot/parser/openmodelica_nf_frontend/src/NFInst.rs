// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCheckModel as CheckModel;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClassTree;
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
use crate::NFSections as Sections;
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
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

pub mod InstSettings {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InstSettings {
        /// Merge sections from extends clauses if true
        pub mergeExtendsSections: bool,
        /// Consider all arrays resizable if true
        pub resizableArrays: bool,
    }

    pub type SETTINGS = InstSettings;

    pub fn create() -> Arc<InstSettings> {
        let mut settings: Arc<InstSettings> = Arc::new(InstSettings { resizableArrays: Flags::getConfigBool(Flags::RESIZABLE_ARRAYS.clone()).unwrap(), mergeExtendsSections: true });
        settings
    }

}

pub static DEFAULT_SETTINGS: std::sync::LazyLock<Arc<InstSettings::InstSettings>> = std::sync::LazyLock::new(|| { Arc::new(InstSettings::InstSettings { resizableArrays: false, mergeExtendsSections: true }) });

pub fn Inst_makeTopNode(mut program: Arc<metamodelica::List<Arc<SCode::Element>>>, mut annotationProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<InstNode::InstNode> {
    let mut topNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Inst_makeTopNode"), lang: Some("C"), output_: Some(CREF_IDENT { name: "topNode", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "program", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "annotationProgram", subscripts: Nil } }, tail: Nil } }, annotation_: None }, annotation: None }
    topNode
}

pub fn instClassInProgram(mut classPath: Arc<Path>, mut program: Arc<metamodelica::List<Arc<SCode::Element>>>, mut annotationProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut relaxedFrontend: bool, mut dumpFlat: bool) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<Flatten::FunctionTreeImpl::Tree>, ArcStr)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel>;
    let mut functions: Arc<Flatten::FunctionTreeImpl::Tree> = Arc::new(Flatten::FunctionTreeImpl::Tree::EMPTY);
    let mut flatString: ArcStr = arcstr::literal!("");
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut inst_cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut context: i32 = 0;
    let mut var_count: i32 = 0;
    let mut eq_count: i32 = 0;
    let mut expose_local_ios: i32 = 0;
    let mut prog: Arc<metamodelica::List<Arc<SCode::Element>>> = program.clone();
    let mut settings: Arc<InstSettings::InstSettings>;
    resetGlobalFlags()?;
    context = if (relaxedFrontend.clone() || Flags::getConfigBool(Flags::CHECK_MODEL.clone())? || Flags::isSet(Flags::NF_API.clone())?) {InstContext::RELAXED.clone()} else {InstContext::NO_CONTEXT.clone()};
    top = makeTopNode(prog.clone(), annotationProgram.clone())?;
    cls = lookupRootClass(classPath.clone(), top.clone(), context.clone())?;
    if SCodeUtil::isFunction(InstNode::definition(cls.clone())?) {
        (flatModel, functions, flatString) = instantiateRootFunction(cls.clone(), context.clone())?;
        return Ok((flatModel, functions, flatString));
    }
    inst_cls = instantiateRootClass(cls.clone(), context.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD))?;
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instantiate(")); __mm_s.push_str(&*AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    settings = InstSettings::create();
    instExpressions(inst_cls.clone(), inst_cls.clone(), Arc::new(crate::NFSections::EMPTY), ConnectBreakTree::new(), context.clone(), settings.clone())?;
    execStat((literal!("NFInst.instExpressions")).clone())?;
    updateImplicitVariability(inst_cls.clone(), Flags::isSet(Flags::EVAL_PARAM.clone())?, context.clone())?;
    execStat((literal!("NFInst.updateImplicitVariability")).clone())?;
    Typing::typeClass(inst_cls.clone(), context.clone())?;
    flatModel = Flatten::flatten(inst_cls.clone(), classPath.clone(), true)?;
    flatModel = EvalConstants::evaluate(flatModel.clone(), context.clone())?;
    InstUtil::dumpFlatModelDebug((literal!("eval")).clone(), flatModel.clone(), Flatten::FunctionTreeImpl::new())?;
    flatModel = UnitCheck::checkUnits(flatModel.clone())?;
    if !(Flags::getConfigBool(Flags::NO_SIMPLIFY.clone())?) {
        flatModel = SimplifyModel::simplify(flatModel.clone())?;
        InstUtil::dumpFlatModelDebug((literal!("simplify")).clone(), flatModel.clone(), Flatten::FunctionTreeImpl::new())?;
    }
    flatModel = StateMachineFlatten::flatten(flatModel.clone())?;
    InstUtil::dumpFlatModelDebug((literal!("stateMachineFlatten")).clone(), flatModel.clone(), Flatten::FunctionTreeImpl::new())?;
    flatModel = Package::collectConstants(flatModel.clone())?;
    functions = Flatten::collectFunctions(flatModel.clone())?;
    if !(Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), (literal!("scalarize")).clone())?) {
        flatString = (if (dumpFlat.clone()) {InstUtil::dumpFlatModel(flatModel.clone(), functions.clone())?} else {literal!("")}).clone();
    }
    InstUtil::printStructuralParameters(flatModel.clone())?;
    if Flags::isSet(Flags::NF_SCALARIZE.clone())? {
        flatModel = Scalarize::scalarize(flatModel.clone())?;
    } else {
        assign_field!(
            flatModel.variables = List::filterOnFalse(flatModel.variables.clone(), (std::sync::Arc::new(fnptr!(Variable::isEmptyArray, Arc<Variable::NFVariable>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<bool> + 'static>)),
            flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Flatten::fillVectorizedVariableBinding(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        );
    }
    flatModel = InstUtil::replaceEmptyArrays(flatModel.clone());
    InstUtil::dumpFlatModelDebug((literal!("scalarize")).clone(), flatModel.clone(), functions.clone())?;
    if Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), (literal!("scalarize")).clone())? {
        flatString = (if (dumpFlat.clone()) {InstUtil::dumpFlatModel(flatModel.clone(), functions.clone())?} else {literal!("")}).clone();
    }
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        flatModel = SimplifyModel::combineBinaries(flatModel.clone())?;
        execStat((literal!("combineBinaries")).clone())?;
        assign_field!(
            flatModel.equations = Equation::mapExpList(flatModel.equations.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Call::NFCall>) -> Result<Arc<Call::NFCall>> + 'static> = Arc::new({ let __pe_b1 = Pointer::create(1); move |__pe_a0| Call::toArrayConstructor(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Ok(Expression::wrapCall(__pe_a0, __pe_b1.clone())) })),
            flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut var in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Variable::mapExp(var.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Call::NFCall>) -> Result<Arc<Call::NFCall>> + 'static> = Arc::new({ let __pe_b1 = Pointer::create(1); move |__pe_a0| Call::toArrayConstructor(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Ok(Expression::wrapCall(__pe_a0, __pe_b1.clone())) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        );
        execStat((literal!("replaceArrayConstructors")).clone())?;
    }
    VerifyModel::verify(flatModel.clone(), InstNode::isPartial(inst_cls.clone()))?;
    (flatModel, functions) = InstUtil::expandSlicedCrefs(flatModel.clone(), functions.clone())?;
    flatModel = InstUtil::combineSubscripts(flatModel.clone())?;
    assign_field!(flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut var in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Variable::propagateAnnotation((literal!("HideResult")).clone(), false, true, var.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    flatModel = FlatModel::removeNonTopLevelDirections(flatModel.clone())?;
    if Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("protected") || Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("encrypted") {
        flatModel = FlatModel::obfuscate(flatModel.clone())?;
    }
    clearCaches()?;
    Ok((flatModel, functions, flatString))
}

pub fn instClassForConnection(mut classPath: Arc<Path>, mut program: Arc<metamodelica::List<Arc<SCode::Element>>>, mut annotationProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut connList: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut conns: Arc<Connections::NFConnections>;
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut inst_cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut context: i32 = 0;
    resetGlobalFlags()?;
    context = if (Flags::getConfigBool(Flags::CHECK_MODEL.clone())? || Flags::isSet(Flags::NF_API.clone())?) {InstContext::RELAXED.clone()} else {InstContext::NO_CONTEXT.clone()};
    top = makeTopNode(program.clone(), annotationProgram.clone())?;
    cls = lookupRootClass(classPath.clone(), top.clone(), context.clone())?;
    inst_cls = instantiateRootClass(cls.clone(), context.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD))?;
    instExpressions(inst_cls.clone(), inst_cls.clone(), Arc::new(crate::NFSections::EMPTY), ConnectBreakTree::new(), context.clone(), DEFAULT_SETTINGS.clone())?;
    Typing::typeClass(inst_cls.clone(), context.clone())?;
    conns = Flatten::flattenConnection(inst_cls.clone(), classPath.clone())?;
    connList = Connections::toStringList(conns.clone());
    clearCaches()?;
    Ok(connList)
}

pub fn resetGlobalFlags() -> Result<()> {
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        if !(Flags::isSet(Flags::FORCE_SCALARIZE.clone())?) {
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), false)?;
        }
        FlagsUtil::set(Flags::VECTORIZE_BINDINGS.clone(), true)?;
    }
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        FlagsUtil::set(Flags::NF_EXPAND_OPERATIONS.clone(), false)?;
        FlagsUtil::set(Flags::NF_EXPAND_FUNC_ARGS.clone(), false)?;
    }
    System::setUsesCardinality(false);
    System::setHasOverconstrainedConnectors(false);
    System::setHasStreamConnectors(false);
    Ok(())
}

pub fn clearCaches() -> Result<()> {
    EvalFunction::clearLibraryCache()?;
    Ok(())
}

pub fn lookupRootClass(mut path: Arc<Path>, mut topScope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut clsNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut next_context: i32 = 0;
    let mut last: ArcStr = arcstr::literal!("");
    let mut cty: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    next_context = InstContext::set(context.clone(), InstContext::RELAXED.clone());
    ErrorExt::setCheckpoint((literal!("NFInst.lookupRootClass")).clone());
    match '__try0: {
        clsNode = unwrap_break_err!(Lookup::lookupClassName(path.clone(), topScope.clone(), next_context.clone(), Absyn::dummyInfo.clone(), false), '__try0);
        ErrorExt::delCheckpoint((literal!("NFInst.lookupRootClass")).clone());
        Ok::<_, anyhow::Error>((clsNode.clone(),))
    } {
        Ok((__try0_o0,)) => {
            clsNode = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                last = (unwrap_break_err!(AbsynUtil::pathLastIdent(path.clone()), '__try1)).clone();
                let true = (last.clone() == literal!("constructor") || last.clone() == literal!("destructor")) else { break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                (clsNode, _) = unwrap_break_err!(Lookup::lookupName(AbsynUtil::stripLast(path.clone())?, topScope.clone(), next_context.clone(), false), '__try1);
                let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(InstNode::getType(clsNode.clone()), '__try1)) {
                    Deref @ Type::COMPLEX { complexTy: __pa2, .. } => __pa2.clone(),
                    _ => break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                cty = __pa2.clone();
                if last.clone() == literal!("constructor") {
                    let __pa3 = ::match_deref::match_deref! { match &(cty.clone()) {
                        Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __pa3, .. } => __pa3.clone(),
                        _ => break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    clsNode = __pa3.clone();
                } else {
                    let __pa4 = ::match_deref::match_deref! { match &(cty.clone()) {
                        Deref @ ComplexType::EXTERNAL_OBJECT { destructor: __pa4, .. } => __pa4.clone(),
                        _ => break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    clsNode = __pa4.clone();
                }
                ErrorExt::rollBack((literal!("NFInst.lookupRootClass")).clone());
                Ok::<_, anyhow::Error>((clsNode.clone(), cty.clone(), last.clone()))
            } {
                Ok((__try1_o0, __try1_o1, __try1_o2)) => {
                    clsNode = __try1_o0;
                    cty = __try1_o1;
                    last = __try1_o2;
                }
                Err(_) => {
                    ErrorExt::delCheckpoint((literal!("NFInst.lookupRootClass")).clone());
                    bail!("fail");
                }
            }
        }
    }
    clsNode = InstUtil::mergeScalars(clsNode.clone(), path.clone(), true, InstUtil::makeMergeNameMap())?;
    checkInstanceRestriction(clsNode.clone(), path.clone(), context.clone())?;
    clsNode = InstNode::makeRootClass(clsNode.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), None);
    Ok(clsNode)
}

pub fn instantiateRootClass(mut clsNode: Arc<InstNode::InstNode>, mut context: i32, mut r#mod: Arc<Modifier::Modifier>) -> Result<Arc<InstNode::InstNode>> {
    let mut clsNode: Arc<InstNode::InstNode> = clsNode;
    clsNode = instantiate(clsNode.clone(), r#mod.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), context.clone(), false)?;
    checkPartialClass(clsNode.clone(), context.clone())?;
    insertGeneratedInners(clsNode.clone(), InstNode::topScope(clsNode.clone()), context.clone())?;
    Ok(clsNode)
}

pub fn instantiateRootFunction(mut funcNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<Flatten::FunctionTreeImpl::Tree>, ArcStr)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel>;
    let mut functions: Arc<Flatten::FunctionTreeImpl::Tree> = Arc::new(Flatten::FunctionTreeImpl::Tree::EMPTY);
    let mut flatString: ArcStr = literal!("");
    Function::instFunctionNode(funcNode.clone(), context.clone(), InstNode::info(funcNode.clone())?)?;
    functions = Flatten::FunctionTreeImpl::new();
    for mut r#fn in &*Function::typeNodeCache(funcNode.clone(), context.clone())? {
        let mut r#fn = r#fn.clone();
        functions = Flatten::flattenFunction(r#fn.clone(), functions.clone())?;
    }
    flatModel = Arc::new(FlatModel::NFFlatModel { name: Arc::new(Path::IDENT { name: (InstNode::name(funcNode.clone())?).clone() }), variables: metamodelica::nil(), equations: metamodelica::nil(), initialEquations: metamodelica::nil(), algorithms: metamodelica::nil(), initialAlgorithms: metamodelica::nil(), source: ElementSource::createElementSource(InstNode::info(funcNode.clone())?, None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))? });
    Ok((flatModel, functions, flatString))
}

pub fn instantiate(mut node: Arc<InstNode::InstNode>, mut r#mod: Arc<Modifier::Modifier>, mut parent: Arc<InstNode::InstNode>, mut context: i32, mut instPartial: bool) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    node = expand(node.clone(), context.clone())?;
    if instPartial.clone() || !(InstNode::isPartial(node.clone())) || InstContext::inRelaxed(context.clone()) || InstContext::inRedeclared(context.clone()) {
        (node, _) = instClass(node.clone(), r#mod.clone(), Attributes::DEFAULT_ATTR().clone(), true, 0, parent.clone(), context.clone())?;
    }
    Ok(node)
}

pub fn expand(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    node = partialInstClass(node.clone())?;
    node = expandClass(node.clone(), context.clone())?;
    Ok(node)
}

pub fn makeTopNode(mut topClasses: Arc<metamodelica::List<Arc<SCode::Element>>>, mut annotationClasses: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<InstNode::InstNode>> {
    let mut topNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut top_classes: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cls_elem: Arc<SCode::Element>;
    let mut ann_package: Arc<SCode::Element>;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut ann_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut elems: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut node_ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
    let mut ann_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut generated_inners: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<InstNode::InstNode>>>;
    top_classes = topClasses.clone();
    if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
        top_classes = cons(NFBuiltinFuncs::BASE_MODELICA_POSITIVE_MAX_SIMPLE.clone(), top_classes.clone());
    }
    cls_elem = Arc::new(SCode::Element::CLASS { name: (literal!("<top>")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PACKAGE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: top_classes.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: Arc::new(SCode::Comment { annotation_: None, comment: None }), info: Absyn::dummyInfo.clone() });
    generated_inners = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    node_ty = Arc::new(InstNodeType::TOP_SCOPE { annotationScope: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), generatedInners: generated_inners.clone() });
    topNode = InstNode::newClass(cls_elem.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), node_ty.clone())?;
    ann_package = Arc::new(SCode::Element::CLASS { name: (literal!("<annotations>")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: openmodelica_frontend_types::SCode::Restriction::R_PACKAGE, classDef: Arc::new(SCode::ClassDef::PARTS { elementLst: annotationClasses.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: Arc::new(SCode::Comment { annotation_: None, comment: None }), info: Absyn::dummyInfo.clone() });
    ann_node = InstNode::newClass(ann_package.clone(), topNode.clone(), Arc::new(crate::NFInstNode::InstNodeType::IMPLICIT_SCOPE))?;
    expand(ann_node.clone(), InstContext::NO_CONTEXT.clone())?;
    cls = InstNode::getClass(ann_node.clone())?;
    elems = Class::classTree(cls.clone())?;
    ClassTree::mapClasses(elems.clone(), (std::sync::Arc::new(fnptr!(markBuiltinTypeNodes, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>));
    cls = Class::setClassTree(elems.clone(), cls.clone())?;
    ann_node = InstNode::updateClass(cls.clone(), ann_node.clone())?;
    node_ty = Arc::new(InstNodeType::TOP_SCOPE { annotationScope: ann_node.clone(), generatedInners: generated_inners.clone() });
    topNode = InstNode::setNodeType(node_ty.clone(), topNode.clone());
    cls = Class::fromSCode(top_classes.clone(), false, topNode.clone(), Class::DEFAULT_PREFIXES.clone())?;
    elems = Class::classTree(cls.clone())?;
    ClassTree::mapClasses(elems.clone(), (std::sync::Arc::new(markBuiltinTypeNodesByAnnotation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>));
    ClassTree::replaceClass(Builtin::CLOCK_NODE().clone(), elems.clone())?;
    cls = Class::setClassTree(elems.clone(), cls.clone())?;
    topNode = InstNode::updateClass(cls.clone(), topNode.clone())?;
    Ok(topNode)
}

pub fn markBuiltinTypeNodes(mut node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    let mut node: Arc<InstNode::InstNode> = node;
    node = InstNode::setNodeType(Arc::new(crate::NFInstNode::InstNodeType::BUILTIN_CLASS), node.clone());
    node
}

pub fn markBuiltinTypeNodesByAnnotation(mut node: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    if SCodeUtil::hasBooleanNamedAnnotationInClass(InstNode::definition(node.clone())?, (literal!("__OpenModelica_builtin")).clone())? {
        node = InstNode::setNodeType(Arc::new(crate::NFInstNode::InstNodeType::BUILTIN_CLASS), node.clone());
    }
    Ok(node)
}

pub fn partialInstClass(mut node: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut c: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let () = (::match_deref::match_deref! { match &(InstNode::getClass(node.clone())?) {
        Deref @ Class::NOT_INSTANTIATED => {
            c = partialInstClass2(InstNode::definition(node.clone())?, node.clone())?;
            node = InstNode::updateClass(c.clone(), node.clone())?;
            c = Class::initImports(c.clone(), node.clone())?;
            node = InstNode::updateClass(c.clone(), node.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub fn partialInstClass2(mut definition: Arc<SCode::Element>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<Class::NFClass>> {
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cdef: Arc<SCode::ClassDef>;
    let mut ce_cdef: Arc<SCode::ClassDef>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut prefs: Arc<Class::Prefixes::Prefixes>;
    Error::assertion(SCodeUtil::elementIsClass(definition.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.partialInstClass2")); __mm_s.push_str(&*literal!(" got non-class element")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    let __pa0 = ::match_deref::match_deref! { match &(definition.clone()) {
        Deref @ SCode::Element::CLASS { classDef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cdef = __pa0.clone();
    prefs = instClassPrefixes(definition.clone())?;
    cls = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => Class::fromSCode(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), false, scope.clone(), prefs.clone())?,
        Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: ce_cdef @ Deref @ SCode::ClassDef::PARTS { .. }, .. } => {
            if !(SCodeUtil::isElementRedeclare(definition.clone())?) {
                Error::addSourceMessage(Error::CLASS_EXTENDS_MISSING_REDECLARE.clone(), list![(SCodeUtil::elementName(definition.clone())?).clone()], SCodeUtil::elementInfo(definition.clone()))?;
            }
            Class::fromSCode(var_field!((**ce_cdef).elementLst, SCode::ClassDef::PARTS).clone(), true, scope.clone(), prefs.clone())?
        },
        Deref @ SCode::ClassDef::ENUMERATION { .. } => {
            ty = makeEnumerationType(var_field!((*cdef).enumLst, SCode::ClassDef::ENUMERATION).clone(), scope.clone())?;
            Class::fromEnumeration(var_field!((*cdef).enumLst, SCode::ClassDef::ENUMERATION).clone(), ty.clone(), prefs.clone(), scope.clone())?
        },
        _ => Arc::new(Class::NFClass::PARTIAL_CLASS { elements: NFClassTree::EMPTY().clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD), ccMod: Arc::new(crate::NFModifier::Modifier::NOMOD), prefixes: prefs.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn instClassPrefixes(mut cls: Arc<SCode::Element>) -> Result<Arc<Class::Prefixes::Prefixes>> {
    let mut prefixes: Arc<Class::Prefixes::Prefixes>;
    let mut prefs: Arc<SCode::Prefixes>;
    prefixes = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, finalPrefix: SCode::Final::NOT_FINAL { .. }, .. }, partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED { .. }, .. } => Class::DEFAULT_PREFIXES.clone(),
        Deref @ SCode::Element::CLASS { prefixes: prefs, .. } => Arc::new(Class::Prefixes::Prefixes { encapsulatedPrefix: var_field!((*cls).encapsulatedPrefix, SCode::Element::CLASS).clone(), partialPrefix: var_field!((*cls).partialPrefix, SCode::Element::CLASS).clone(), finalPrefix: prefs.finalPrefix.clone(), innerOuter: prefs.innerOuter.clone(), replaceablePrefix: prefs.replaceablePrefix.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(prefixes)
}

pub fn makeEnumerationType(mut literals: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut lits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut path: Arc<Path>;
    path = InstNode::scopePath(scope.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?;
    lits = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (literals.clone()).into_iter().cloned() {
            let __x = e.literal.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    ty = Arc::new(Type::NFType::ENUMERATION { typePath: path.clone(), literals: lits.clone() });
    Ok(ty)
}

pub fn expandClass(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    node = (::match_deref::match_deref! { match &(InstNode::getClass(node.clone())?) {
        Deref @ Class::PARTIAL_CLASS { .. } => expandClass2(node.clone(), context.clone())?,
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub fn expandClass2(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut def: Arc<SCode::Element> = InstNode::definition(node.clone())?;
    let mut cdef: Arc<SCode::ClassDef>;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut name_map: Option<Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>> = None;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::Element::CLASS { info: __pa0, classDef: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    cdef = __pa1.clone();
    node = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            (node, name_map) = expandClassParts(def.clone(), node.clone(), context.clone(), info.clone())?;
            if isSome(name_map.clone()) {
                InstUtil::mergeScalarsComponentBindings(node.clone(), Util::getOption(name_map.clone())?)?;
            }
            node.clone()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            (node, name_map) = expandClassParts(def.clone(), node.clone(), context.clone(), info.clone())?;
            if isSome(name_map.clone()) {
                InstUtil::mergeScalarsComponentBindings(node.clone(), Util::getOption(name_map.clone())?)?;
            }
            node.clone()
        },
        Deref @ SCode::ClassDef::DERIVED { .. } => (::match_deref::match_deref! { match &(var_field!((*cdef).typeSpec, SCode::ClassDef::DERIVED).clone()) {
        Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => expandClassDerivedComplex(def.clone(), cdef.clone(), node.clone(), info.clone())?,
        _ => expandClassDerived(def.clone(), cdef.clone(), node.clone(), context.clone(), info.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ SCode::ClassDef::OVERLOAD { .. } => node.clone(),
        Deref @ SCode::ClassDef::PDER { .. } => expandClassDerived(def.clone(), Arc::new(SCode::ClassDef::DERIVED { typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: var_field!((*cdef).functionPath, SCode::ClassDef::PDER).clone(), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), attributes: SCode::defaultVarAttr.clone() }), node.clone(), context.clone(), info.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.expandClass2")); __mm_s.push_str(&*literal!(" got unknown class:\n")); __mm_s.push_str(&*SCodeDump::unparseElementStr(def.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub fn expandClassParts(mut def: Arc<SCode::Element>, mut node: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<InstNode::InstNode>, Option<Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>>)> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut nameMap: Option<Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>> = None;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut cc_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut builtin_ext: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut prefs: Arc<Class::Prefixes::Prefixes>;
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    let mut name_map: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>;
    cls = InstNode::getClass(node.clone())?;
    cls = Class::initExpandedClass(cls.clone())?;
    node = InstNode::updateClass(cls.clone(), node.clone())?;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { prefixes: __pa0, ccMod: __pa1, modifier: __pa2, elements: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    prefs = __pa0.clone();
    cc_mod = __pa1.clone();
    r#mod = __pa2.clone();
    cls_tree = __pa3.clone();
    if ClassTree::extendsCount(cls_tree.clone()) > 0 {
        name_map = InstUtil::makeMergeNameMap();
        builtin_ext = ClassTree::mapFoldExtends(cls_tree.clone(), Arc::new({ let __pe_b2 = context.clone(); let __pe_b3 = name_map.clone(); move |__pe_a0, __pe_a1| expandExtends(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
        nameMap = if (UnorderedMap::isEmpty(name_map.clone())) {None} else {Some(name_map.clone())};
    } else {
        builtin_ext = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
        nameMap = None;
    }
    if InstNode::name(builtin_ext.clone())? == literal!("ExternalObject") {
        node = expandExternalObject(cls_tree.clone(), node.clone())?;
    } else {
        if !(InstNode::isEmpty(builtin_ext.clone())) {
            checkBuiltinTypeExtends(builtin_ext.clone(), cls_tree.clone(), node.clone())?;
        }
        cls_tree = ClassTree::expand(cls_tree.clone())?;
        res = Restriction::fromSCode(SCodeUtil::getClassRestriction(def.clone())?);
        cls = Arc::new(Class::NFClass::EXPANDED_CLASS { elements: cls_tree.clone(), modifier: r#mod.clone(), ccMod: cc_mod.clone(), prefixes: prefs.clone(), restriction: res.clone() });
        node = InstNode::updateClass(cls.clone(), node.clone())?;
    }
    Ok((node, nameMap))
}

pub fn expandExtends(mut ext: Arc<InstNode::InstNode>, mut builtinExt: Arc<InstNode::InstNode>, mut context: i32, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>) -> Result<(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)> {
    let mut ext: Arc<InstNode::InstNode> = ext;
    let mut builtinExt: Arc<InstNode::InstNode> = builtinExt;
    let mut def: Arc<SCode::Element>;
    let mut base_path: Arc<Path>;
    let mut base_nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut base_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut ann: Option<Arc<SCode::Annotation>> = None;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    if InstNode::isEmpty(ext.clone()) {
        return Ok((ext, builtinExt));
    }
    def = InstNode::definition(ext.clone())?;
    let () = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: base_path, visibility: vis, modifications: smod, ann, info } => {
            scope = InstNode::parent(ext.clone());
            let (__pa1, __pa0) = ::match_deref::match_deref! { match &(Lookup::lookupBaseClassName(base_path.clone(), scope.clone(), context.clone(), info.clone())?) {
                __pa1 @ Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => (__pa1.clone(), __pa0.clone()),
                _ => bail!("pattern mismatch"),
            } };
            base_node = __pa0.clone();
            base_nodes = __pa1.clone();
            checkExtendsLoop(base_node.clone(), scope.clone(), base_path.clone(), info.clone())?;
            checkReplaceableBaseClass(base_nodes.clone(), base_path.clone(), info.clone())?;
            if InstNode::isRootClass(scope.clone()) && SCodeUtil::isEmptyMod(smod.clone()) {
                base_node = InstUtil::mergeScalars(base_node.clone(), base_path.clone(), false, nameMap.clone())?;
            }
            base_node = expand(base_node.clone(), context.clone())?;
            ext = InstNode::setNodeType(Arc::new(InstNodeType::BASE_CLASS { parent: scope.clone(), definition: def.clone(), ty: InstNode::nodeType(base_node.clone())? }), base_node.clone());
            if InstNode::isBuiltin(base_node.clone()) || Class::isBuiltin(InstNode::getClass(base_node.clone())?)? {
                builtinExt = ext.clone();
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ext, builtinExt))
}

pub fn checkExtendsLoop(mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut path: Arc<Path>, mut info: SourceInfo) -> Result<()> {
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let () = (::match_deref::match_deref! { match &(InstNode::getClass(node.clone())?) {
        Deref @ Class::EXPANDED_CLASS { elements: Deref @ ClassTree::PARTIAL_TREE { .. }, .. } => {
            Error::addSourceMessage(Error::EXTENDS_LOOP.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => {
            parent = scope.clone();
            while !(InstNode::isTopScope(parent.clone())) {
                if InstNode::refEqual(parent.clone(), node.clone()) {
                    Error::addSourceMessage(Error::EXTENDS_LOOP.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
                    bail!("fail");
                }
                parent = InstNode::parentScope(parent.clone(), false)?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkReplaceableBaseClass(mut baseClasses: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut basePath: Arc<Path>, mut info: SourceInfo) -> Result<()> {
    let mut i: i32 = 0;
    let mut pos: i32 = 0;
    let mut name: ArcStr = arcstr::literal!("");
    let mut rest: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    for mut base in &*baseClasses.clone() {
        let mut base = base.clone();
        i = i.clone() + 1;
        if SCodeUtil::isElementReplaceable(InstNode::definition(base.clone())?)? {
            if (baseClasses.clone().len() as i32) > 1 {
                rest = baseClasses.clone();
                name = (literal!("")).clone();
                for mut j in 1..=i.clone() - 1 {
                    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*InstNode::name(listHead(rest.clone())?)?); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
                    rest = listRest(rest.clone())?;
                }
                name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*InstNode::name(listHead(rest.clone())?)?); __mm_s.push_str(&*literal!(">")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
                rest = listRest(rest.clone())?;
                for mut n in &*rest.clone() {
                    let mut n = n.clone();
                    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*InstNode::name(n.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
                }
            } else {
                name = (AbsynUtil::pathString(basePath.clone(), (literal!(".")).clone(), true, false)?).clone();
            }
            Error::addMultiSourceMessage(Error::REPLACEABLE_BASE_CLASS.clone(), list![(InstNode::name(base.clone())?).clone(), (name.clone()).clone()], list![InstNode::info(base.clone())?, info.clone()])?;
            bail!("fail");
        }
    }
    Ok(())
}

pub fn expandExternalObject(mut clsTree: Arc<ClassTree::ClassTree>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut eo_ty: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut c: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    eo_ty = makeExternalObjectType(clsTree.clone(), node.clone())?;
    c = Arc::new(Class::NFClass::PARTIAL_BUILTIN { ty: Arc::new(Type::NFType::COMPLEX { cls: node.clone(), complexTy: eo_ty.clone() }), elements: NFClassTree::EMPTY_FLAT().clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD), prefixes: Class::DEFAULT_PREFIXES.clone(), restriction: Arc::new(crate::NFRestriction::EXTERNAL_OBJECT) });
    node = InstNode::updateClass(c.clone(), node.clone())?;
    Ok(node)
}

pub fn checkBuiltinTypeExtends(mut builtinExtends: Arc<InstNode::InstNode>, mut tree: Arc<ClassTree::ClassTree>, mut node: Arc<InstNode::InstNode>) -> Result<()> {
    if ClassTree::componentCount(tree.clone()) > 0 || ClassTree::extendsCount(tree.clone()) > 1 {
        Error::addSourceMessage(Error::BUILTIN_EXTENDS_INVALID_ELEMENTS.clone(), list![(InstNode::name(builtinExtends.clone())?).clone()], InstNode::info(node.clone())?)?;
        bail!("fail");
    }
    Ok(())
}

pub fn makeExternalObjectType(mut tree: Arc<ClassTree::ClassTree>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<ComplexType::NFComplexType>> {
    let mut ty: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut base_path: Arc<Path>;
    let mut constructor: Arc<InstNode::InstNode> = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
    let mut destructor: Arc<InstNode::InstNode> = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
    ty = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ClassTree::PARTIAL_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::ClassTree::PARTIAL_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                if InstNode::isComponent(comp.clone()) {
                    Error::addSourceMessage(Error::EXTERNAL_OBJECT_INVALID_ELEMENT.clone(), list![(InstNode::name(node.clone())?).clone(), (InstNode::name(comp.clone())?).clone()], InstNode::info(comp.clone())?)?;
                    bail!("fail");
                }
            }
            if (var_field!((*tree).exts, ClassTree::ClassTree::PARTIAL_TREE).clone().borrow().len() as i32) > 1 {
                let __range1 = var_field!((*tree).exts, ClassTree::ClassTree::PARTIAL_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range1 {
                    if InstNode::name(ext.clone())? != literal!("ExternalObject") && ClassTree::recursiveElementCount(Class::classTree(InstNode::getClass(ext.clone())?)?)? != 0 {
                        let __pa2 = ::match_deref::match_deref! { match &(ext.clone()) {
                            Deref @ InstNode::CLASS_NODE { nodeType: Deref @ InstNodeType::BASE_CLASS { definition: Deref @ SCode::Element::EXTENDS { baseClassPath: __pa2, .. }, .. }, .. } => __pa2.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        base_path = __pa2.clone();
                        Error::addSourceMessage(Error::EXTERNAL_OBJECT_INVALID_ELEMENT.clone(), list![(InstNode::name(node.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("extends ")); __mm_s.push_str(&*AbsynUtil::pathString(base_path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone()], InstNode::info(ext.clone())?)?;
                        bail!("fail");
                    }
                }
            }
            let __range4 = var_field!((*tree).classes, ClassTree::ClassTree::PARTIAL_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut cls in __range4 {
                let () = (::match_deref::match_deref! { match &(InstNode::name(cls.clone())?) {
        Deref @ "constructor" if (SCodeUtil::isFunction(InstNode::definition(cls.clone())?)) => {
            checkElementNotReplaceable(cls.clone())?;
            constructor = cls.clone();
            ()
        },
        Deref @ "destructor" if (SCodeUtil::isFunction(InstNode::definition(cls.clone())?)) => {
            checkElementNotReplaceable(cls.clone())?;
            destructor = cls.clone();
            ()
        },
        _ => {
            Error::addSourceMessage(Error::EXTERNAL_OBJECT_INVALID_ELEMENT.clone(), list![(InstNode::name(node.clone())?).clone(), (InstNode::name(cls.clone())?).clone()], InstNode::info(cls.clone())?)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            if InstNode::isEmpty(constructor.clone()) {
                Error::addSourceMessage(Error::EXTERNAL_OBJECT_MISSING_STRUCTOR.clone(), list![(InstNode::name(node.clone())?).clone(), (literal!("constructor")).clone()], InstNode::info(node.clone())?)?;
                bail!("fail");
            }
            if InstNode::isEmpty(destructor.clone()) {
                Error::addSourceMessage(Error::EXTERNAL_OBJECT_MISSING_STRUCTOR.clone(), list![(InstNode::name(node.clone())?).clone(), (literal!("destructor")).clone()], InstNode::info(node.clone())?)?;
                bail!("fail");
            }
            Arc::new(ComplexType::NFComplexType::EXTERNAL_OBJECT { constructor: constructor.clone(), destructor: destructor.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ty)
}

pub fn checkElementNotReplaceable(mut node: Arc<InstNode::InstNode>) -> Result<()> {
    if SCodeUtil::isElementReplaceable(InstNode::definition(node.clone())?)? {
        Error::addSourceMessage(Error::ELEMENT_REPLACEABLE_NOT_ALLOWED.clone(), list![(InstNode::name(node.clone())?).clone()], InstNode::info(node.clone())?)?;
        bail!("fail");
    }
    Ok(())
}

pub fn expandClassDerived(mut element: Arc<SCode::Element>, mut definition: Arc<SCode::ClassDef>, mut node: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut ext_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut prefs: Arc<Class::Prefixes::Prefixes>;
    let mut sattrs: SCode::Attributes;
    let mut attrs: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut cc_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(definition.clone()) {
        Deref @ SCode::ClassDef::DERIVED { attributes: __pa0, typeSpec: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    sattrs = __pa0.clone();
    ty = __pa1.clone();
    let __pa2 = ::match_deref::match_deref! { match &(Lookup::lookupBaseClassName(AbsynUtil::typeSpecPath(ty.clone())?, InstNode::parent(node.clone()), context.clone(), info.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ext_node = __pa2.clone();
    if referenceEq(&ext_node.clone(),&node.clone()) {
        Error::addSourceMessage(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone(), list![(InstNode::name(node.clone())?).clone(), (Dump::unparseTypeSpec(ty.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    ext_node = expand(ext_node.clone(), context.clone())?;
    ext_node = InstNode::clone(ext_node.clone())?;
    cls = InstNode::getClass(node.clone())?;
    prefs = Class::getPrefixes(cls.clone())?;
    if !(Class::Prefixes::isPartial(prefs.clone())) && InstNode::isPartial(ext_node.clone()) {
        assign_field!(prefs.partialPrefix = openmodelica_frontend_types::SCode::Partial::PARTIAL);
    }
    attrs = Attributes::fromDerivedSCode(sattrs.clone())?;
    dims = {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (AbsynUtil::typeSpecDimensions(ty.clone())).into_iter().cloned() {
            let __x = Arc::new(Dimension::NFDimension::RAW_DIM { dim: d.clone(), scope: InstNode::parent(node.clone()) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    r#mod = Class::getModifier(cls.clone());
    cc_mod = Class::getCCModifier(cls.clone());
    res = Restriction::fromSCode(SCodeUtil::getClassRestriction(element.clone())?);
    cls = Arc::new(Class::NFClass::EXPANDED_DERIVED { baseClass: ext_node.clone(), modifier: r#mod.clone(), ccMod: cc_mod.clone(), dims: metamodelica::arrayFromVec(dims.clone().into_iter().cloned().collect()), prefixes: prefs.clone(), attributes: attrs.clone(), restriction: res.clone() });
    node = InstNode::updateClass(cls.clone(), node.clone())?;
    Ok(node)
}

pub fn expandClassDerivedComplex(mut element: Arc<SCode::Element>, mut definition: Arc<SCode::ClassDef>, mut node: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut ty_path: Arc<Path>;
    let mut prefs: Arc<Class::Prefixes::Prefixes>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let __pa0 = ::match_deref::match_deref! { match &(definition.clone()) {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ty_path = __pa0.clone();
    ty = (::match_deref::match_deref! { match &(ty_path.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" } => Arc::new(Type::NFType::POLYMORPHIC { name: (InstNode::name(node.clone())?).clone() }),
        _ => {
            Error::addSourceMessage(Error::LOOKUP_BASECLASS_ERROR.clone(), list![(AbsynUtil::pathString(ty_path.clone(), (literal!(".")).clone(), true, false)?).clone(), (InstNode::scopeName(node.clone())).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cls = InstNode::getClass(node.clone())?;
    prefs = Class::getPrefixes(cls.clone())?;
    res = Restriction::fromSCode(SCodeUtil::getClassRestriction(element.clone())?);
    cls = Arc::new(Class::NFClass::PARTIAL_BUILTIN { ty: ty.clone(), elements: NFClassTree::EMPTY().clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD), prefixes: prefs.clone(), restriction: res.clone() });
    node = InstNode::updateClass(cls.clone(), node.clone())?;
    Ok(node)
}

pub fn instClass(mut node: Arc<InstNode::InstNode>, mut modifier: Arc<Modifier::Modifier>, mut attributes: Arc<Attributes::NFAttributes>, mut useBinding: bool, mut instLevel: i32, mut parent: Arc<InstNode::InstNode>, mut context: i32) -> Result<(Arc<InstNode::InstNode>, Arc<Attributes::NFAttributes>)> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut attributes: Arc<Attributes::NFAttributes> = attributes;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut outer_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    cls = InstNode::getClass(node.clone())?;
    outer_mod = Class::getModifier(cls.clone());
    if Modifier::hasBinding(outer_mod.clone()) {
        Error::addSourceMessage(Error::MISSING_REDECLARE_IN_CLASS_MOD.clone(), list![(InstNode::name(node.clone())?).clone()], Binding::getInfo(Modifier::binding(outer_mod.clone())))?;
        bail!("fail");
    }
    (attributes, node) = instClassDef(cls.clone(), modifier.clone(), attributes.clone(), useBinding.clone(), node.clone(), parent.clone(), instLevel.clone(), context.clone())?;
    Ok((node, attributes))
}

pub fn instClassDef(mut cls: Arc<Class::NFClass>, mut outerMod: Arc<Modifier::Modifier>, mut attributes: Arc<Attributes::NFAttributes>, mut useBinding: bool, mut node: Arc<InstNode::InstNode>, mut parent: Arc<InstNode::InstNode>, mut instLevel: i32, mut context: i32) -> Result<(Arc<Attributes::NFAttributes>, Arc<InstNode::InstNode>)> {
    let mut attributes: Arc<Attributes::NFAttributes> = attributes;
    let mut node: Arc<InstNode::InstNode> = node;
    let mut par: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut base_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut inst_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut outer_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut attrs: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { restriction: res, .. } => {
            if InstNode::isBaseClass(node.clone()) {
                par = parent.clone();
            } else {
                (node, par, _, _) = ClassTree::instantiate(node.clone(), parent.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?;
            }
            updateComponentType(parent.clone(), node.clone())?;
            attributes = Attributes::updateClassConnectorType(res.clone(), attributes.clone());
            let (__pa1, __pa0) = ::match_deref::match_deref! { match &(InstNode::getClass(node.clone())?) {
                __pa1 @ Deref @ Class::EXPANDED_CLASS { elements: __pa0, .. } => (__pa1.clone(), __pa0.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cls_tree = __pa0.clone();
            inst_cls = __pa1.clone();
            r#mod = instElementModifier(InstNode::definition(node.clone())?, node.clone(), par.clone())?;
            r#mod = Modifier::propagate(r#mod.clone(), node.clone(), par.clone());
            r#mod = Modifier::merge(r#mod.clone(), var_field!((*cls).ccMod, Class::NFClass::EXPANDED_CLASS).clone(), (literal!("")).clone())?;
            outer_mod = Modifier::propagate(var_field!((*cls).modifier, Class::NFClass::EXPANDED_CLASS).clone(), node.clone(), par.clone());
            outer_mod = Modifier::merge(outerMod.clone(), outer_mod.clone(), (literal!("")).clone())?;
            r#mod = Modifier::merge(outer_mod.clone(), r#mod.clone(), (literal!("")).clone())?;
            ClassTree::mapExtends(cls_tree.clone(), Arc::new({ let __pe_b1 = par.clone(); let __pe_b2 = context.clone(); move |__pe_a0| modifyExtends(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            ClassTree::mapExtends(cls_tree.clone(), Arc::new({ let __pe_b1 = ExtendsVisibility::PUBLIC.clone(); move |__pe_a0| applyExtendsVisibility(__pe_a0, __pe_b1.clone()) }));
            applyModifier(r#mod.clone(), cls_tree.clone(), node.clone(), context.clone())?;
            ClassTree::mapRedeclareChains(cls_tree.clone(), Arc::new({ let __pe_b1 = instLevel.clone(); let __pe_b2 = context.clone(); move |__pe_a0| redeclareElements(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            redeclareClasses(cls_tree.clone(), par.clone(), context.clone())?;
            ClassTree::mapExtends(cls_tree.clone(), Arc::new({ let __pe_b1 = attributes.clone(); let __pe_b2 = useBinding.clone(); let __pe_b3 = instLevel.clone() + 1; let __pe_b4 = context.clone(); move |__pe_a0| instExtends(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }));
            ClassTree::applyLocalComponents(cls_tree.clone(), Arc::new({ let __pe_b1 = attributes.clone(); let __pe_b2 = Arc::new(crate::NFModifier::Modifier::NOMOD); let __pe_b3 = useBinding.clone(); let __pe_b4 = instLevel.clone() + 1; let __pe_b5 = context.clone(); let __pe_b6 = None; let __pe_b7 = metamodelica::nil(); move |__pe_a0| instComponent(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone()) }))?;
            cls_tree = ClassTree::replaceDuplicates(cls_tree.clone());
            ClassTree::checkDuplicates(cls_tree.clone());
            InstNode::updateClass(Class::setClassTree(cls_tree.clone(), inst_cls.clone())?, node.clone())?;
            Restriction::checkClass(node.clone(), res.clone(), context.clone())?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { .. } => {
            (node, par, _, _) = ClassTree::instantiate(node.clone(), parent.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?;
            node = InstNode::setNodeType(Arc::new(InstNodeType::DERIVED_CLASS { ty: InstNode::nodeType(node.clone())? }), node.clone());
            let __pa0 = ::match_deref::match_deref! { match &(InstNode::getClass(node.clone())?) {
                Deref @ Class::EXPANDED_DERIVED { baseClass: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            base_node = __pa0.clone();
            r#mod = instElementModifier(InstNode::definition(node.clone())?, node.clone(), InstNode::rootParent(node.clone()))?;
            r#mod = Modifier::propagate(r#mod.clone(), node.clone(), par.clone());
            r#mod = Modifier::merge(r#mod.clone(), var_field!((*cls).ccMod, Class::NFClass::EXPANDED_DERIVED).clone(), (literal!("")).clone())?;
            outer_mod = Modifier::propagate(var_field!((*cls).modifier, Class::NFClass::EXPANDED_DERIVED).clone(), node.clone(), par.clone());
            outer_mod = Modifier::merge(outerMod.clone(), outer_mod.clone(), (literal!("")).clone())?;
            r#mod = Modifier::merge(outer_mod.clone(), r#mod.clone(), (literal!("")).clone())?;
            attrs = Attributes::updateClassConnectorType(var_field!((*cls).restriction, Class::NFClass::EXPANDED_DERIVED).clone(), var_field!((*cls).attributes, Class::NFClass::EXPANDED_DERIVED).clone());
            attributes = Attributes::mergeDerivedAttributes(attrs.clone(), attributes.clone(), parent.clone())?;
            (base_node, attributes) = instClass(base_node.clone(), r#mod.clone(), attributes.clone(), useBinding.clone(), instLevel.clone(), par.clone(), context.clone())?;
            assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED;
                baseClass = base_node.clone(),
                attributes = attributes.clone(),
                dims = metamodelica::arrayFromVec(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone().borrow().clone())
            );
            node = InstNode::updateClass(cls.clone(), node.clone())?;
            updateComponentType(parent.clone(), node.clone())?;
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { restriction: Deref @ Restriction::EXTERNAL_OBJECT, .. } => {
            inst_cls = Arc::new(Class::NFClass::INSTANCED_BUILTIN { ty: var_field!((*cls).ty, Class::NFClass::PARTIAL_BUILTIN).clone(), elements: var_field!((*cls).elements, Class::NFClass::PARTIAL_BUILTIN).clone(), restriction: var_field!((*cls).restriction, Class::NFClass::PARTIAL_BUILTIN).clone() });
            applyModifier(outerMod.clone(), var_field!((*cls).elements, Class::NFClass::PARTIAL_BUILTIN).clone(), node.clone(), context.clone())?;
            node = InstNode::replaceClass(inst_cls.clone(), node.clone())?;
            updateComponentType(parent.clone(), node.clone())?;
            instExternalObjectStructors(var_field!((*cls).ty, Class::NFClass::PARTIAL_BUILTIN).clone(), parent.clone(), context.clone())?;
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { restriction: res, ty, .. } => {
            (node, par, _, _) = ClassTree::instantiate(node.clone(), parent.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?;
            updateComponentType(parent.clone(), node.clone())?;
            cls_tree = Class::classTree(InstNode::getClass(node.clone())?)?;
            r#mod = instElementModifier(InstNode::definition(node.clone())?, node.clone(), InstNode::parent(node.clone()))?;
            outer_mod = Modifier::merge(outerMod.clone(), var_field!((*cls).modifier, Class::NFClass::PARTIAL_BUILTIN).clone(), (literal!("")).clone())?;
            r#mod = Modifier::merge(outer_mod.clone(), r#mod.clone(), (literal!("")).clone())?;
            applyModifier(r#mod.clone(), cls_tree.clone(), node.clone(), context.clone())?;
            inst_cls = Arc::new(Class::NFClass::INSTANCED_BUILTIN { ty: ty.clone(), elements: cls_tree.clone(), restriction: res.clone() });
            node = InstNode::updateClass(inst_cls.clone(), node.clone())?;
            ()
        },
        Deref @ Class::INSTANCED_CLASS { .. } => {
            node = InstNode::replaceClass(Arc::new(crate::NFClass::NOT_INSTANTIATED), node.clone())?;
            node = InstNode::setNodeType(Arc::new(crate::NFInstNode::InstNodeType::NORMAL_CLASS), node.clone());
            node = expand(node.clone(), context.clone())?;
            (node, _) = instClass(node.clone(), outerMod.clone(), attributes.clone(), useBinding.clone(), instLevel.clone(), parent.clone(), context.clone())?;
            updateComponentType(parent.clone(), node.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instClassDef")); __mm_s.push_str(&*literal!(" got unknown class.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((attributes, node))
}

pub fn updateComponentType(mut component: Arc<InstNode::InstNode>, mut cls: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
    let mut component: Arc<InstNode::InstNode> = component;
    if InstNode::isComponent(component.clone()) {
        component = InstNode::componentApply(component.clone(), (std::sync::Arc::new(Component::setClassInstance) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), cls.clone())?;
    }
    Ok(component)
}

pub fn instExternalObjectStructors(mut ty: Arc<Type::NFType>, mut parent: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut constructor: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut destructor: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut par: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    par = InstNode::parent(InstNode::parent(parent.clone()));
    if !(InstNode::isClass(par.clone()) && Class::isExternalObject(InstNode::getClass(par.clone())?)) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ty.clone()) {
            Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __pa0, destructor: __pa1 }, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        constructor = __pa0.clone();
        destructor = __pa1.clone();
        info = InstNode::info(parent.clone())?;
        Function::instFunctionNode(constructor.clone(), context.clone(), info.clone())?;
        Function::instFunctionNode(destructor.clone(), context.clone(), info.clone())?;
    }
    Ok(())
}

pub fn instPackage(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    use crate::NFInstNode::PackageCacheState;
    let mut node: Arc<InstNode::InstNode> = node;
    let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
    let mut inst: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut state: PackageCacheState = PackageCacheState::NOT_INITIALIZED;
    cache = InstNode::getPackageCache(node.clone())?;
    (inst, state) = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::PACKAGE { .. } => (var_field!((*cache).instance, CachedData::CachedData::PACKAGE).clone(), var_field!((*cache).state, CachedData::CachedData::PACKAGE).clone()),
        _ => (node.clone(), PackageCacheState::NOT_INITIALIZED.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if state.clone() == PackageCacheState::INSTANTIATED.clone() {
        node = inst.clone();
        return Ok(node);
    }
    if state.clone() == PackageCacheState::PROCESSING.clone() {
        node = inst.clone();
        return Ok(node);
    }
    if state.clone() < PackageCacheState::PARTIALLY_INSTANTIATED.clone() {
        InstNode::setPackageCache(node.clone(), node.clone(), PackageCacheState::PROCESSING.clone())?;
        inst = instantiate(node.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), context.clone(), false)?;
        InstNode::setPackageCache(node.clone(), inst.clone(), PackageCacheState::PARTIALLY_INSTANTIATED.clone())?;
    }
    if state.clone() < PackageCacheState::INSTANTIATED.clone() && !(InstContext::inFastLookup(context.clone())) && (!(InstNode::isPartial(inst.clone())) || InstContext::inRelaxed(context.clone())) {
        InstNode::setPackageCache(node.clone(), inst.clone(), PackageCacheState::INSTANTIATED.clone())?;
        instExpressions(inst.clone(), inst.clone(), Arc::new(crate::NFSections::EMPTY), ConnectBreakTree::new(), context.clone(), DEFAULT_SETTINGS.clone())?;
    }
    node = inst.clone();
    Ok(node)
}

pub fn modifyExtends(mut extendsNode: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut extendsNode: Arc<InstNode::InstNode> = extendsNode;
    let mut elem: Arc<SCode::Element>;
    let mut ext_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut ext_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    cls = InstNode::getClass(extendsNode.clone())?;
    cls_tree = Class::classTree(cls.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(InstNode::nodeType(extendsNode.clone())?) {
        Deref @ InstNodeType::BASE_CLASS { definition: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    elem = __pa0.clone();
    ext_mod = Modifier::fromElement(elem.clone(), scope.clone())?;
    ext_mod = Modifier::merge(InstNode::getModifier(extendsNode.clone()), ext_mod.clone(), (literal!("")).clone())?;
    if !(Class::isBuiltin(cls.clone())?) {
        ClassTree::mapExtends(cls_tree.clone(), Arc::new({ let __pe_b1 = extendsNode.clone(); let __pe_b2 = context.clone(); move |__pe_a0| modifyExtends(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
        let () = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupBaseClassName(var_field!((*elem).baseClassPath, SCode::Element::EXTENDS).clone(), scope.clone(), context.clone(), var_field!((*elem).info, SCode::Element::EXTENDS).clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ext_node = __pa0.clone();
            if !(referenceEq(&InstNode::definition(extendsNode.clone())?,&InstNode::definition(ext_node.clone())?)) && !(Flags::isSet(Flags::MERGE_COMPONENTS.clone())?) {
                Error::addMultiSourceMessage(Error::FOUND_OTHER_BASECLASS.clone(), list![(AbsynUtil::pathString(var_field!((*elem).baseClassPath, SCode::Element::EXTENDS).clone(), (literal!(".")).clone(), true, false)?).clone()], list![InstNode::info(extendsNode.clone())?, InstNode::info(ext_node.clone())?])?;
                bail!("fail");
            }
            ()
        },
        Deref @ SCode::Element::CLASS { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    }
    applyModifier(ext_mod.clone(), cls_tree.clone(), extendsNode.clone(), context.clone())?;
    Ok(extendsNode)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ExtendsVisibility {
    PUBLIC = 1,
    DERIVED_PROTECTED = 2,
    PROTECTED = 3,
}
impl PartialOrd for ExtendsVisibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ExtendsVisibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn applyExtendsVisibility(mut node: Arc<InstNode::InstNode>, mut visibility: ExtendsVisibility) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut vis: ExtendsVisibility = visibility.clone();
    cls = InstNode::getClass(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { elements: cls_tree @ Deref @ ClassTree::INSTANTIATED_TREE { .. }, .. } => {
            if vis.clone() == ExtendsVisibility::PUBLIC.clone() && InstNode::isProtectedBaseClass(node.clone()) || vis.clone() == ExtendsVisibility::DERIVED_PROTECTED.clone() {
                vis = ExtendsVisibility::PROTECTED.clone();
            }
            if vis.clone() == ExtendsVisibility::PROTECTED.clone() && visibility.clone() != ExtendsVisibility::PROTECTED.clone() {
                let __range0 = var_field!((**cls_tree).classes, ClassTree::ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range0 {
                    Mutable::update(c.clone(), InstNode::protectClass(Mutable::access(c.clone())));
                }
                let __range1 = var_field!((**cls_tree).components, ClassTree::ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    Mutable::update(c.clone(), InstNode::protectComponent(Mutable::access(c.clone())));
                }
            }
            ClassTree::mapExtends(cls_tree.clone(), Arc::new({ let __pe_b1 = vis.clone(); move |__pe_a0| applyExtendsVisibility(__pe_a0, __pe_b1.clone()) }));
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { .. } => {
            if vis.clone() == ExtendsVisibility::PUBLIC.clone() && InstNode::isProtectedBaseClass(node.clone()) {
                vis = ExtendsVisibility::DERIVED_PROTECTED.clone();
            }
            assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED; baseClass = applyExtendsVisibility(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), vis.clone())?);
            node = InstNode::updateClass(cls.clone(), node.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub fn instExtends(mut node: Arc<InstNode::InstNode>, mut attributes: Arc<Attributes::NFAttributes>, mut useBinding: bool, mut instLevel: i32, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut inst_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    cls = InstNode::getClass(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { elements: cls_tree @ Deref @ ClassTree::INSTANTIATED_TREE { .. }, .. } => {
            ClassTree::mapExtends(cls_tree.clone(), Arc::new({ let __pe_b1 = attributes.clone(); let __pe_b2 = useBinding.clone(); let __pe_b3 = instLevel.clone(); let __pe_b4 = context.clone(); move |__pe_a0| instExtends(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }));
            ClassTree::applyLocalComponents(cls_tree.clone(), Arc::new({ let __pe_b1 = attributes.clone(); let __pe_b2 = Arc::new(crate::NFModifier::Modifier::NOMOD); let __pe_b3 = useBinding.clone(); let __pe_b4 = instLevel.clone(); let __pe_b5 = context.clone(); let __pe_b6 = None; let __pe_b7 = metamodelica::nil(); move |__pe_a0| instComponent(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone()) }))?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED; baseClass = instExtends(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), attributes.clone(), useBinding.clone(), instLevel.clone(), context.clone())?);
            node = InstNode::updateClass(cls.clone(), node.clone())?;
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { .. } => {
            inst_cls = Arc::new(Class::NFClass::INSTANCED_BUILTIN { ty: var_field!((*cls).ty, Class::NFClass::PARTIAL_BUILTIN).clone(), elements: var_field!((*cls).elements, Class::NFClass::PARTIAL_BUILTIN).clone(), restriction: var_field!((*cls).restriction, Class::NFClass::PARTIAL_BUILTIN).clone() });
            node = InstNode::updateClass(inst_cls.clone(), node.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub fn applyModifier(mut modifier: Arc<Modifier::Modifier>, mut cls: Arc<ClassTree::ClassTree>, mut parent: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<ClassTree::ClassTree>> {
    let mut cls: Arc<ClassTree::ClassTree> = cls;
    let mut mods: Arc<metamodelica::List<Arc<Modifier::Modifier>>> = metamodelica::nil();
    let mut node_ptrs: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>> = metamodelica::nil();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut found: bool = false;
    mods = Modifier::toList(modifier.clone());
    if mods.clone().is_empty() {
        return Ok(cls);
    }
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ ClassTree::FLAT_TREE { .. } => {
            for mut r#mod in &*mods.clone() {
                let mut r#mod = r#mod.clone();
                if '__try0: {
                    (node, _) = unwrap_break_err!(ClassTree::lookupElement((Modifier::name(r#mod.clone())?).clone(), cls.clone()), '__try0);
                    unwrap_break_err!(InstNode::componentApply(node.clone(), (std::sync::Arc::new(Component::mergeModifier) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Modifier::Modifier>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), r#mod.clone()), '__try0);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(Modifier::name(r#mod.clone())?).clone(), (InstNode::name(parent.clone())?).clone()], Modifier::info(r#mod.clone())?)?;
                    if !(InstContext::inInstanceAPI(context.clone())) {
                        bail!("fail");
                    }
                }
            }
            ()
        },
        _ => {
            for mut r#mod in &*mods.clone() {
                let mut r#mod = r#mod.clone();
                match '__try0: {
                    node_ptrs = unwrap_break_err!(ClassTree::lookupElementsPtr((Modifier::name(r#mod.clone())?).clone(), cls.clone()), '__try0);
                    Ok::<_, anyhow::Error>((node_ptrs.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        node_ptrs = __try0_o0;
                    }
                    Err(_) => {
                        node_ptrs = metamodelica::nil();
                    }
                }
                found = false;
                for mut node_ptr in &*node_ptrs.clone() {
                    let mut node_ptr = node_ptr.clone();
                    node = Mutable::access(node_ptr.clone());
                    if InstNode::isEmpty(node.clone()) {
                        continue;
                    }
                    found = true;
                    node = InstNode::resolveOuter(node.clone());
                    if InstNode::isProtected(node.clone()) && !(InstNode::isExtends(parent.clone()) || InstNode::isBaseClass(parent.clone())) {
                        Error::addMultiSourceMessage(Error::NF_MODIFY_PROTECTED.clone(), list![(InstNode::name(node.clone())?).clone(), (Modifier::toString(r#mod.clone(), true)?).clone()], list![Modifier::info(r#mod.clone())?, InstNode::info(node.clone())?])?;
                        if InstContext::inInstanceAPI(context.clone()) {
                            continue;
                        } else {
                            bail!("fail");
                        }
                    }
                    if InstNode::isOnlyOuter(node.clone())? {
                        Error::addSourceMessage(Error::OUTER_ELEMENT_MOD.clone(), list![(Modifier::toString(r#mod.clone(), false)?).clone(), (Modifier::name(r#mod.clone())?).clone()], Modifier::info(r#mod.clone())?)?;
                        if InstContext::inInstanceAPI(context.clone()) {
                            continue;
                        } else {
                            bail!("fail");
                        }
                    }
                    if InstNode::isComponent(node.clone()) {
                        InstNode::componentApply(node.clone(), (std::sync::Arc::new(Component::mergeModifier) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Modifier::Modifier>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), r#mod.clone())?;
                    } else {
                        partialInstClass(node.clone())?;
                        node = InstNode::replaceClass(Class::mergeModifier(r#mod.clone(), InstNode::getClass(node.clone())?)?, node.clone())?;
                        node = InstNode::clearPackageCache(node.clone())?;
                        Mutable::update(node_ptr.clone(), node.clone());
                    }
                }
                if !(found.clone()) && !(InstContext::inInstanceAPI(context.clone())) {
                    Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(Modifier::name(r#mod.clone())?).clone(), (InstNode::name(parent.clone())?).clone()], Modifier::info(r#mod.clone())?)?;
                    bail!("fail");
                }
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn redeclareClasses(mut tree: Arc<ClassTree::ClassTree>, mut parent: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<ClassTree::ClassTree>> {
    let mut tree: Arc<ClassTree::ClassTree> = tree;
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut redecl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut cc_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ClassTree::INSTANTIATED_TREE { .. } => {
            let __range0 = var_field!((*tree).classes, ClassTree::ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut cls_ptr in __range0 {
                cls_node = Mutable::access(cls_ptr.clone());
                cls = InstNode::getClass(InstNode::resolveOuter(cls_node.clone()))?;
                r#mod = Class::getModifier(cls.clone());
                if Modifier::isRedeclare(r#mod.clone()) {
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(r#mod.clone()) {
                        Deref @ Modifier::REDECLARE { constrainingMod: __pa1, outerMod: __pa2, element: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cc_mod = __pa1.clone();
                    r#mod = __pa2.clone();
                    redecl_node = __pa3.clone();
                    cc_mod = getConstrainingMod(InstNode::definition(cls_node.clone())?, parent.clone(), cc_mod.clone())?;
                    cls_node = redeclareClass(redecl_node.clone(), cls_node.clone(), r#mod.clone(), cc_mod.clone(), context.clone())?;
                    Mutable::update(cls_ptr.clone(), cls_node.clone());
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

pub fn redeclareElements(mut chain: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>, mut instLevel: i32, mut context: i32) -> Result<()> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut node_ptr: Mutable::Mutable<Arc<InstNode::InstNode>>;
    node = Mutable::access(listHead(chain.clone())?);
    node_ptr = listHead(chain.clone())?;
    if InstNode::isClass(node.clone()) {
        for mut cls_ptr in &*listRest(chain.clone())? {
            let mut cls_ptr = cls_ptr.clone();
            node_ptr = redeclareClassElement(cls_ptr.clone(), node_ptr.clone(), context.clone())?;
        }
        node = Mutable::access(node_ptr.clone());
    } else {
        for mut comp_ptr in &*listRest(chain.clone())? {
            let mut comp_ptr = comp_ptr.clone();
            node_ptr = redeclareComponentElement(comp_ptr.clone(), node_ptr.clone(), instLevel.clone(), context.clone())?;
        }
        node = Mutable::access(node_ptr.clone());
    }
    for mut cls_ptr in &*chain.clone() {
        let mut cls_ptr = cls_ptr.clone();
        Mutable::update(cls_ptr.clone(), node.clone());
    }
    Ok(())
}

pub fn redeclareClassElement(mut redeclareCls: Mutable::Mutable<Arc<InstNode::InstNode>>, mut replaceableCls: Mutable::Mutable<Arc<InstNode::InstNode>>, mut context: i32) -> Result<Mutable::Mutable<Arc<InstNode::InstNode>>> {
    let mut outCls: Mutable::Mutable<Arc<InstNode::InstNode>>;
    let mut rdcl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut repl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    rdcl_node = Mutable::access(redeclareCls.clone());
    repl_node = Mutable::access(replaceableCls.clone());
    rdcl_node = redeclareClass(rdcl_node.clone(), repl_node.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), Arc::new(crate::NFModifier::Modifier::NOMOD), context.clone())?;
    outCls = Mutable::create(rdcl_node.clone());
    Ok(outCls)
}

pub fn redeclareComponentElement(mut redeclareComp: Mutable::Mutable<Arc<InstNode::InstNode>>, mut replaceableComp: Mutable::Mutable<Arc<InstNode::InstNode>>, mut instLevel: i32, mut context: i32) -> Result<Mutable::Mutable<Arc<InstNode::InstNode>>> {
    let mut outComp: Mutable::Mutable<Arc<InstNode::InstNode>>;
    let mut rdcl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut repl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    rdcl_node = Mutable::access(redeclareComp.clone());
    repl_node = Mutable::access(replaceableComp.clone());
    instComponent(repl_node.clone(), Attributes::DEFAULT_ATTR().clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), true, instLevel.clone(), context.clone(), None, metamodelica::nil())?;
    redeclareComponent(rdcl_node.clone(), repl_node.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), Arc::new(crate::NFModifier::Modifier::NOMOD), metamodelica::nil(), Attributes::DEFAULT_ATTR().clone(), rdcl_node.clone(), instLevel.clone(), context.clone())?;
    outComp = Mutable::create(rdcl_node.clone());
    Ok(outComp)
}

pub fn redeclareClass(mut redeclareNode: Arc<InstNode::InstNode>, mut originalNode: Arc<InstNode::InstNode>, mut outerMod: Arc<Modifier::Modifier>, mut constrainingMod: Arc<Modifier::Modifier>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut redeclaredNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut orig_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut orig_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut rdcl_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut new_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut prefs: Arc<Class::Prefixes::Prefixes>;
    let mut node_ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut orig_opt: Option<Arc<InstNode::InstNode>> = None;
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    if !(InstNode::isClass(redeclareNode.clone())) {
        Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(InstNode::typeName(originalNode.clone())?).clone(), (InstNode::name(originalNode.clone())?).clone(), (InstNode::typeName(redeclareNode.clone())?).clone()], list![InstNode::info(redeclareNode.clone())?, InstNode::info(originalNode.clone())?])?;
        bail!("fail");
    }
    partialInstClass(originalNode.clone())?;
    orig_cls = InstNode::getClass(originalNode.clone())?;
    partialInstClass(redeclareNode.clone())?;
    rdcl_cls = InstNode::getClass(redeclareNode.clone())?;
    r#mod = Class::getModifier(rdcl_cls.clone());
    r#mod = Modifier::merge(outerMod.clone(), r#mod.clone(), (literal!("")).clone())?;
    prefs = Attributes::mergeRedeclaredClassPrefixes(Class::getPrefixes(orig_cls.clone())?, Class::getPrefixes(rdcl_cls.clone())?, redeclareNode.clone())?;
    if SCodeUtil::isClassExtends(InstNode::definition(redeclareNode.clone())?) {
        orig_node = expand(originalNode.clone(), context.clone())?;
        orig_cls = InstNode::getClass(orig_node.clone())?;
        new_cls = (::match_deref::match_deref! { match &((orig_cls.clone(), rdcl_cls.clone())) {
        (_, Deref @ Class::PARTIAL_CLASS { .. }) if (Class::isBuiltin(orig_cls.clone())?) => {
            if !(SCodeUtil::isEmptyClassDef(SCodeUtil::getClassDef(InstNode::definition(redeclareNode.clone())?)?)) {
                Error::addSourceMessage(Error::BUILTIN_EXTENDS_INVALID_ELEMENTS.clone(), list![(InstNode::name(redeclareNode.clone())?).clone()], InstNode::info(redeclareNode.clone())?)?;
                bail!("fail");
            }
            Class::setPrefixes(prefs.clone(), orig_cls.clone())?
        },
        (_, Deref @ Class::PARTIAL_CLASS { .. }) => {
            node_ty = Arc::new(InstNodeType::BASE_CLASS { parent: InstNode::parent(orig_node.clone()), definition: InstNode::definition(orig_node.clone())?, ty: InstNode::nodeType(orig_node.clone())? });
            orig_node = InstNode::setNodeType(node_ty.clone(), orig_node.clone());
            cls_tree = ClassTree::setClassExtends(orig_node.clone(), var_field!((*rdcl_cls).elements, Class::NFClass::PARTIAL_CLASS).clone())?;
            Arc::new(Class::NFClass::PARTIAL_CLASS { elements: cls_tree.clone(), modifier: r#mod.clone(), ccMod: constrainingMod.clone(), prefixes: prefs.clone() })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.redeclareClass")); __mm_s.push_str(&*literal!(" got unknown classes")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        new_cls = (::match_deref::match_deref! { match &((orig_cls.clone(), rdcl_cls.clone())) {
        (Deref @ Class::PARTIAL_BUILTIN { .. }, _) => redeclareEnum(rdcl_cls.clone(), orig_cls.clone(), prefs.clone(), r#mod.clone(), redeclareNode.clone(), originalNode.clone(), context.clone())?,
        (_, Deref @ Class::PARTIAL_CLASS { .. }) => Arc::new(Class::NFClass::PARTIAL_CLASS { elements: var_field!((*rdcl_cls).elements, Class::NFClass::PARTIAL_CLASS).clone(), modifier: r#mod.clone(), ccMod: constrainingMod.clone(), prefixes: prefs.clone() }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.redeclareClass")); __mm_s.push_str(&*literal!(" got unknown classes")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    orig_opt = if (InstContext::inInstanceAPI(context.clone())) {Some(originalNode.clone())} else {None};
    redeclaredNode = InstNode::replaceClass(new_cls.clone(), redeclareNode.clone())?;
    node_ty = Arc::new(InstNodeType::REDECLARED_CLASS { parent: InstNode::parent(originalNode.clone()), originalType: InstNode::nodeType(originalNode.clone())?, originalNode: orig_opt.clone() });
    redeclaredNode = InstNode::setNodeType(node_ty.clone(), redeclaredNode.clone());
    Ok(redeclaredNode)
}

pub fn redeclareEnum(mut redeclareClass: Arc<Class::NFClass>, mut originalClass: Arc<Class::NFClass>, mut prefixes: Arc<Class::Prefixes::Prefixes>, mut outerMod: Arc<Modifier::Modifier>, mut redeclareNode: Arc<InstNode::InstNode>, mut originalNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Class::NFClass>> {
    let mut redeclaredClass: Arc<Class::NFClass> = redeclareClass.clone();
    expand(redeclareNode.clone(), context.clone())?;
    redeclaredClass = InstNode::getClass(redeclareNode.clone())?;
    redeclaredClass = (::match_deref::match_deref! { match &((redeclaredClass.clone(), originalClass.clone())) {
        (_, Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { literals: Deref @ metamodelica::List::Nil, .. }, .. }) if (InstNode::isEnumerationType(redeclareNode.clone())) => {
            redeclaredClass = Class::setPrefixes(prefixes.clone(), redeclaredClass.clone())?;
            redeclaredClass = Class::mergeModifier(outerMod.clone(), redeclaredClass.clone())?;
            redeclaredClass.clone()
        },
        (Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { literals: lits1, .. }, .. }, Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { literals: lits2, .. }, .. }) => {
            if !(lits2.clone().is_empty() || List::isEqualOnTrue(lits1.clone(), lits2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) {
                Error::addMultiSourceMessage(Error::REDECLARE_ENUM_NON_SUBTYPE.clone(), list![(InstNode::name(originalNode.clone())?).clone()], list![InstNode::info(redeclareNode.clone())?, InstNode::info(originalNode.clone())?])?;
                bail!("fail");
            }
            assign_variant_field!(redeclaredClass => Class::NFClass::PARTIAL_BUILTIN;
                prefixes = prefixes.clone(),
                modifier = Modifier::merge(outerMod.clone(), var_field!((*redeclaredClass).modifier, Class::NFClass::PARTIAL_BUILTIN).clone(), (literal!("")).clone())?
            );
            redeclaredClass.clone()
        },
        _ => {
            Error::addMultiSourceMessage(Error::REDECLARE_CLASS_NON_SUBTYPE.clone(), list![(Restriction::toString(Class::restriction(originalClass.clone()))).clone(), (InstNode::name(originalNode.clone())?).clone()], list![InstNode::info(redeclareNode.clone())?, InstNode::info(originalNode.clone())?])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(redeclaredClass)
}

pub fn instComponent(mut node: Arc<InstNode::InstNode>, mut attributes: Arc<Attributes::NFAttributes>, mut innerMod: Arc<Modifier::Modifier>, mut useBinding: bool, mut instLevel: i32, mut context: i32, mut originalAttr: Option<Arc<Attributes::NFAttributes>>, mut propagatedSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut def: Arc<SCode::Element>;
    let mut comp_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rdcl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut outer_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut inner_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut cc_mod: Arc<Modifier::Modifier> = innerMod.clone();
    let mut cc_def_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut cc_smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut name: ArcStr = arcstr::literal!("");
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut next_context: i32 = 0;
    let mut propagated_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    if InstNode::isEmpty(node.clone()) {
        return Ok(());
    }
    checkOuterComponentMod(node.clone(), context.clone())?;
    comp_node = InstNode::resolveInner(node.clone());
    comp = InstNode::component(comp_node.clone())?;
    parent = InstNode::parent(comp_node.clone());
    if !(Component::isDefinition(comp.clone())) {
        checkRecursiveDefinition(Component::classInstance(comp.clone()), comp_node.clone(), false)?;
        return Ok(());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT_DEF { modifier: __pa0, definition: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outer_mod = __pa0.clone();
    def = __pa1.clone();
    if Modifier::isRedeclare(outer_mod.clone()) {
        let (__pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(outer_mod.clone()) {
            Deref @ Modifier::REDECLARE { propagatedSubs: __pa2, constrainingMod: __pa3, outerMod: __pa4, innerMod: __pa5, element: __pa6, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        propagated_subs = __pa2.clone();
        cc_mod = __pa3.clone();
        outer_mod = __pa4.clone();
        inner_mod = __pa5.clone();
        rdcl_node = __pa6.clone();
        next_context = InstContext::set(context.clone(), InstContext::REDECLARED.clone());
        instComponentDef(def.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), inner_mod.clone(), Attributes::DEFAULT_ATTR().clone(), useBinding.clone(), comp_node.clone(), parent.clone(), instLevel.clone(), originalAttr.clone(), metamodelica::nil(), next_context.clone())?;
        cc_mod = getConstrainingMod(def.clone(), parent.clone(), cc_mod.clone())?;
        cc_mod = Modifier::merge(cc_mod.clone(), innerMod.clone(), (literal!("")).clone())?;
        outer_mod = Modifier::merge(InstNode::getModifier(rdcl_node.clone()), outer_mod.clone(), (literal!("")).clone())?;
        InstNode::setModifier(outer_mod.clone(), rdcl_node.clone())?;
        redeclareComponent(rdcl_node.clone(), node.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), cc_mod.clone(), propagated_subs.clone(), attributes.clone(), node.clone(), instLevel.clone(), context.clone())?;
    } else {
        instComponentDef(def.clone(), outer_mod.clone(), innerMod.clone(), attributes.clone(), useBinding.clone(), comp_node.clone(), parent.clone(), instLevel.clone(), originalAttr.clone(), propagatedSubs.clone(), context.clone())?;
    }
    Ok(())
}

pub fn instComponentDef(mut component: Arc<SCode::Element>, mut outerMod: Arc<Modifier::Modifier>, mut innerMod: Arc<Modifier::Modifier>, mut attributes: Arc<Attributes::NFAttributes>, mut useBinding: bool, mut node: Arc<InstNode::InstNode>, mut parent: Arc<InstNode::InstNode>, mut instLevel: i32, mut originalAttr: Option<Arc<Attributes::NFAttributes>>, mut propagatedSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut context: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ SCode::Element::COMPONENT { info, .. } => {
            let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
            let mut condition: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
            let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
            let mut ty_attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
            let mut inst_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            let mut ty_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut ty: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            let mut elementDefinition: Arc<SCode::Element>;
            let mut parent_res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
            let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
            let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
            r#mod = instElementModifier(component.clone(), node.clone(), parent.clone())?;
            if !(propagatedSubs.clone().is_empty()) {
                r#mod = Modifier::propagateSubs(r#mod.clone(), propagatedSubs.clone());
            }
            r#mod = Modifier::merge(r#mod.clone(), innerMod.clone(), (literal!("")).clone())?;
            r#mod = Modifier::merge(outerMod.clone(), r#mod.clone(), (literal!("")).clone())?;
            dims = {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (var_field!((*component).attributes, SCode::Element::COMPONENT).arrayDims.clone()).into_iter().cloned() {
            let __x = Arc::new(Dimension::NFDimension::RAW_DIM { dim: d.clone(), scope: parent.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            binding = if (useBinding.clone()) {Modifier::binding(r#mod.clone())} else {Binding::EMPTY_BINDING().clone()};
            condition = Binding::fromAbsyn(var_field!((*component).condition, SCode::Element::COMPONENT).clone(), false, false, parent.clone(), info.clone());
            parent_res = Class::restriction(InstNode::getClass(parent.clone())?);
            attr = Attributes::fromSCode(var_field!((*component).attributes, SCode::Element::COMPONENT).clone(), var_field!((*component).prefixes, SCode::Element::COMPONENT).clone())?;
            attr = Attributes::checkDeclaredComponentAttributes(attr.clone(), parent_res.clone(), node.clone())?;
            attr = Attributes::mergeComponentAttributes(attributes.clone(), attr.clone(), node.clone(), parent_res.clone())?;
            if isSome(originalAttr.clone()) {
                attr = Attributes::mergeRedeclaredComponentAttributes(Util::getOption(originalAttr.clone())?, attr.clone(), node.clone())?;
            }
            if !(attr.isFinal.clone()) && Modifier::isFinal(r#mod.clone()) {
                assign_field!(attr.isFinal = true);
            }
            inst_comp = Arc::new(Component::NFComponent::COMPONENT { classInst: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), ty: Arc::new(crate::NFType::UNKNOWN), binding: binding.clone(), condition: condition.clone(), attributes: attr.clone(), comment: var_field!((*component).comment, SCode::Element::COMPONENT).clone(), state: ComponentState::PartiallyInstantiated.clone(), info: info.clone() });
            InstNode::updateComponent(inst_comp.clone(), node.clone())?;
            r#mod = Modifier::propagate(r#mod.clone(), node.clone(), node.clone());
            (ty_node, ty_attr) = instTypeSpec(var_field!((*component).typeSpec, SCode::Element::COMPONENT).clone(), r#mod.clone(), attr.clone(), useBinding.clone() && !(Binding::isBound(binding.clone())), parent.clone(), node.clone(), info.clone(), instLevel.clone(), context.clone())?;
            InstNode::componentApply(node.clone(), (std::sync::Arc::new(Component::setType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), Arc::new(Type::NFType::UNTYPED { typeNode: ty_node.clone(), dimensions: metamodelica::arrayFromVec(dims.clone().into_iter().cloned().collect()) }))?;
            if !(InstNode::isEmpty(ty_node.clone())) {
                ty = InstNode::getClass(ty_node.clone())?;
                res = Class::restriction(ty.clone());
                elementDefinition = InstNode::definition(ty_node.clone())?;
                if Restriction::isType(res.clone()) && SCodeUtil::optCommentHasBooleanNamedAnnotationFalse(SCodeUtil::getElementComment(elementDefinition.clone()), (literal!("absoluteValue")).clone())? {
                    cmt = Component::comment(InstNode::component(node.clone())?)?;
                    cmt = SCodeUtil::setAnnotationInComment((literal!("absoluteValue")).clone(), Arc::new(Absyn::Exp::BOOL { value: false }), cmt.clone(), false)?;
                    InstNode::componentApply(node.clone(), (std::sync::Arc::new(Component::setComment) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Comment>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), cmt.clone())?;
                }
                if !(InstContext::inRedeclared(context.clone())) {
                    checkPartialComponent(node.clone(), attr.clone(), ty_node.clone(), Class::isPartial(ty.clone()), res.clone(), context.clone(), info.clone())?;
                }
                checkBindingRestriction(res.clone(), binding.clone(), node.clone(), info.clone())?;
                ty_attr = Attributes::updateVariability(ty_attr.clone(), ty.clone(), ty_node.clone(), node.clone(), context.clone())?;
                ty_attr = Attributes::updateComponentConnectorType(ty_attr.clone(), res.clone(), context.clone(), node.clone())?;
                if !(referenceEq(&attr.clone(),&ty_attr.clone())) {
                    InstNode::componentApply(node.clone(), (std::sync::Arc::new(Component::setAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Attributes::NFAttributes>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), ty_attr.clone())?;
                }
                if useBinding.clone() && Binding::isUnbound(binding.clone()) && !(InstContext::inFunction(context.clone())) && ty_attr.variability.clone() <= Variability::PARAMETER.clone() && Restriction::isType(res.clone()) {
                    updateParameterBinding(node.clone(), context.clone())?;
                }
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn instElementModifier(mut element: Arc<SCode::Element>, mut component: Arc<InstNode::InstNode>, mut parent: Arc<InstNode::InstNode>) -> Result<Arc<Modifier::Modifier>> {
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut cc_mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    r#mod = Modifier::fromElement(element.clone(), parent.clone())?;
    if InstNode::isRedeclared(component.clone())? {
        r#mod = propagateRedeclaredMod(r#mod.clone(), component.clone());
    } else {
        cc_mod = instConstrainingMod(element.clone(), parent.clone())?;
        r#mod = Modifier::merge(r#mod.clone(), cc_mod.clone(), (literal!("")).clone())?;
    }
    Ok(r#mod)
}

pub fn instConstrainingMod(mut element: Arc<SCode::Element>, mut parent: Arc<InstNode::InstNode>) -> Result<Arc<Modifier::Modifier>> {
    let mut ccMod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    ccMod = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: smod, .. }) }, .. }, .. } => {
            Modifier::create(smod.clone(), (var_field!((*element).name, SCode::Element::CLASS).clone()).clone(), Arc::new(ModifierScope::ModifierScope::CLASS { name: (var_field!((*element).name, SCode::Element::CLASS).clone()).clone() }), parent.clone())?
        },
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: smod, .. }) }, .. }, .. } => {
            Modifier::create(smod.clone(), (var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone(), Arc::new(ModifierScope::ModifierScope::COMPONENT { name: (var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone() }), parent.clone())?
        },
        _ => {
            Arc::new(crate::NFModifier::Modifier::NOMOD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ccMod)
}

pub fn getConstrainingMod(mut element: Arc<SCode::Element>, mut parent: Arc<InstNode::InstNode>, mut outerMod: Arc<Modifier::Modifier>) -> Result<Arc<Modifier::Modifier>> {
    let mut ccMod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut name: ArcStr = arcstr::literal!("");
    let mut cc_smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut mod_scope: Arc<ModifierScope::ModifierScope>;
    cc_smod = SCodeUtil::getConstrainingMod(element.clone());
    if !(SCodeUtil::isEmptyMod(cc_smod.clone())) {
        name = (SCodeUtil::elementName(element.clone())?).clone();
        ccMod = Modifier::create(cc_smod.clone(), (name.clone()).clone(), ModifierScope::fromElement(element.clone())?, parent.clone())?;
        ccMod = Modifier::merge(outerMod.clone(), ccMod.clone(), (literal!("")).clone())?;
    } else {
        ccMod = outerMod.clone();
    }
    Ok(ccMod)
}

pub fn propagateRedeclaredMod(mut r#mod: Arc<Modifier::Modifier>, mut component: Arc<InstNode::InstNode>) -> Arc<Modifier::Modifier> {
    let mut outMod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    outMod = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ InstNode::COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent }, .. } => {
            let mut parent = (*parent).clone();
            parent = InstNode::getDerivedNode(parent.clone(), true);
            outMod = propagateRedeclaredMod(r#mod.clone(), parent.clone());
            Modifier::propagateBinding(outMod.clone(), parent.clone(), parent.clone())
        },
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn checkPartialComponent(mut compNode: Arc<InstNode::InstNode>, mut compAttr: Arc<Attributes::NFAttributes>, mut clsNode: Arc<InstNode::InstNode>, mut isPartial: bool, mut res: Arc<Restriction::NFRestriction>, mut context: i32, mut info: SourceInfo) -> Result<()> {
    if Restriction::isFunction(res.clone()) {
        if !(isPartial.clone()) && !(InstContext::inRelaxed(context.clone())) {
            Error::addSourceMessage(Error::META_FUNCTION_TYPE_NO_PARTIAL_PREFIX.clone(), list![(AbsynUtil::pathString(InstNode::scopePath(clsNode.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
            bail!("fail");
        }
    } else if isPartial.clone() && compAttr.innerOuter.clone() != InnerOuter::OUTER.clone() && !(InstContext::inRelaxed(context.clone())) {
        Error::addMultiSourceMessage(Error::PARTIAL_COMPONENT_TYPE.clone(), list![(AbsynUtil::pathString(InstNode::scopePath(compNode.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone(), (InstNode::name(clsNode.clone())?).clone()], list![InstNode::info(clsNode.clone())?, info.clone()])?;
        bail!("fail");
    }
    Ok(())
}

pub fn checkBindingRestriction(mut restriction: Arc<Restriction::NFRestriction>, mut binding: Arc<Binding::NFBinding>, mut component: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<()> {
    if Binding::isBound(binding.clone()) {
        let () = (::match_deref::match_deref! { match &(restriction.clone()) {
        Deref @ Restriction::CLOCK => (),
        Deref @ Restriction::CONNECTOR { .. } => (),
        Deref @ Restriction::ENUMERATION => (),
        Deref @ Restriction::EXTERNAL_OBJECT => (),
        Deref @ Restriction::RECORD { .. } => (),
        Deref @ Restriction::TYPE => (),
        _ => {
            Error::addSourceMessage(Error::INVALID_SPECIALIZATION_FOR_BINDING_EQUATION.clone(), list![(InstNode::name(component.clone())?).clone(), (Restriction::toString(restriction.clone())).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

pub fn redeclareComponent(mut redeclareNode: Arc<InstNode::InstNode>, mut originalNode: Arc<InstNode::InstNode>, mut outerMod: Arc<Modifier::Modifier>, mut constrainingMod: Arc<Modifier::Modifier>, mut propagatedSubs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut outerAttr: Arc<Attributes::NFAttributes>, mut redeclaredNode: Arc<InstNode::InstNode>, mut instLevel: i32, mut context: i32) -> Result<()> {
    let mut orig_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut rdcl_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut new_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut condition: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut orig_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut rdcl_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let mut orig_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rdcl_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rdcl_type: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
    if !(InstNode::isComponent(redeclareNode.clone())) {
        Error::addMultiSourceMessage(Error::INVALID_REDECLARE_AS.clone(), list![(InstNode::typeName(originalNode.clone())?).clone(), (InstNode::name(originalNode.clone())?).clone(), (InstNode::typeName(redeclareNode.clone())?).clone()], list![InstNode::info(redeclareNode.clone())?, InstNode::info(originalNode.clone())?])?;
        bail!("fail");
    }
    orig_node = InstNode::resolveInner(originalNode.clone());
    orig_comp = InstNode::component(orig_node.clone())?;
    rdcl_type = Arc::new(InstNodeType::REDECLARED_COMP { parent: InstNode::parent(orig_node.clone()) });
    rdcl_node = InstNode::setNodeType(rdcl_type.clone(), redeclareNode.clone());
    rdcl_node = InstNode::copyInstancePtr(orig_node.clone(), rdcl_node.clone())?;
    rdcl_node = InstNode::updateComponent(InstNode::component(redeclareNode.clone())?, rdcl_node.clone())?;
    instComponent(rdcl_node.clone(), outerAttr.clone(), constrainingMod.clone(), true, instLevel.clone(), context.clone(), Some(Component::getAttributes(orig_comp.clone())), propagatedSubs.clone())?;
    rdcl_comp = InstNode::component(rdcl_node.clone())?;
    new_comp = (::match_deref::match_deref! { match &((orig_comp.clone(), rdcl_comp.clone())) {
        (Deref @ Component::COMPONENT { ty: orig_ty @ Deref @ Type::UNTYPED { .. }, .. }, Deref @ Component::COMPONENT { ty: rdcl_ty @ Deref @ Type::UNTYPED { .. }, .. }) => {
            let mut rdcl_ty = (*rdcl_ty).clone();
            if !(InstNode::isReplaceable(orig_node.clone())?) && !(InstContext::inInstanceAPI(context.clone())) && !(Type::isEqual(Type::arrayElementType(orig_ty.clone()), Type::arrayElementType(rdcl_ty.clone()))) {
                Error::addMultiSourceMessage(Error::REDECLARE_NON_REPLACEABLE.clone(), list![(InstNode::name(orig_node.clone())?).clone()], list![InstNode::info(orig_node.clone())?, InstNode::info(rdcl_node.clone())?])?;
                bail!("fail");
            }
            binding = Modifier::binding(outerMod.clone());
            if Binding::isUnbound(binding.clone()) {
                binding = if (Binding::isBound(var_field!((*rdcl_comp).binding, Component::NFComponent::COMPONENT).clone())) {var_field!((*rdcl_comp).binding, Component::NFComponent::COMPONENT).clone()} else {var_field!((*orig_comp).binding, Component::NFComponent::COMPONENT).clone()};
            }
            if Binding::isBound(var_field!((*rdcl_comp).condition, Component::NFComponent::COMPONENT).clone()) {
                Error::addSourceMessage(Error::REDECLARE_CONDITION.clone(), list![(InstNode::name(redeclareNode.clone())?).clone()], InstNode::info(redeclareNode.clone())?)?;
                bail!("fail");
            }
            condition = var_field!((*orig_comp).condition, Component::NFComponent::COMPONENT).clone();
            attr = var_field!((*rdcl_comp).attributes, Component::NFComponent::COMPONENT).clone();
            if Type::dimensionCount(rdcl_ty.clone()) == 0 {
                rdcl_ty = Arc::new(Type::NFType::UNTYPED { typeNode: var_field!((*rdcl_ty).typeNode, Type::NFType::UNTYPED).clone(), dimensions: var_field!((**orig_ty).dimensions, Type::NFType::UNTYPED).clone() });
            }
            cmt = var_field!((*orig_comp).comment, Component::NFComponent::COMPONENT).clone();
            Arc::new(Component::NFComponent::COMPONENT { classInst: var_field!((*rdcl_comp).classInst, Component::NFComponent::COMPONENT).clone(), ty: rdcl_ty.clone(), binding: binding.clone(), condition: condition.clone(), attributes: attr.clone(), comment: cmt.clone(), state: ComponentState::PartiallyInstantiated.clone(), info: var_field!((*rdcl_comp).info, Component::NFComponent::COMPONENT).clone() })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.redeclareComponent")); __mm_s.push_str(&*literal!(" got unknown components")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    InstNode::updateComponent(new_comp.clone(), InstNode::resolveInner(redeclaredNode.clone()))?;
    Ok(())
}

pub fn checkOuterComponentMod(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut outer_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut elem: Arc<SCode::Element>;
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outer_node = InstNode::resolveOuter(node.clone());
    elem = InstNode::definition(outer_node.clone())?;
    if AbsynUtil::isOnlyOuter(SCodeUtil::prefixesInnerOuter(SCodeUtil::elementPrefixes(elem.clone())?)?) {
        smod = SCodeUtil::componentMod(elem.clone());
        if !(SCodeUtil::isEmptyMod(smod.clone())) {
            Error::addSourceMessage(Error::OUTER_ELEMENT_MOD.clone(), list![(SCodeDump::printModStr(smod.clone(), SCodeDump::defaultOptions.clone())?).clone(), (InstNode::name(outer_node.clone())?).clone()], InstNode::info(outer_node.clone())?)?;
            if !(InstContext::inInstanceAPI(context.clone())) {
                bail!("fail");
            }
        }
    }
    Ok(())
}

pub fn instTypeSpec(mut typeSpec: Arc<Absyn::TypeSpec>, mut modifier: Arc<Modifier::Modifier>, mut attributes: Arc<Attributes::NFAttributes>, mut useBinding: bool, mut scope: Arc<InstNode::InstNode>, mut parent: Arc<InstNode::InstNode>, mut info: SourceInfo, mut instLevel: i32, mut context: i32) -> Result<(Arc<InstNode::InstNode>, Arc<Attributes::NFAttributes>)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut outAttributes: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    node = 'mc: {
        let __mc_input = typeSpec.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TPATH { .. } => {
                    let mut outAttributes: Arc<Attributes::NFAttributes> = outAttributes.clone();
                    let mut node: Arc<InstNode::InstNode> = node.clone();
                    node = Lookup::lookupClassName(var_field!((*typeSpec).path, Absyn::TypeSpec::TPATH).clone(), scope.clone(), context.clone(), info.clone(), true)?;
                    if instLevel.clone() >= 100 {
                        checkRecursiveDefinition(node.clone(), parent.clone(), true)?;
                    }
                    node = expand(node.clone(), context.clone())?;
                    (node, outAttributes) = instClass(node.clone(), modifier.clone(), attributes.clone(), useBinding.clone(), instLevel.clone(), parent.clone(), context.clone())?;
                    Ok(node.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TPATH { .. } => {
                    if !((InstContext::inInstanceAPI(context.clone()))) { bail!("guard") }
                    let mut outAttributes: Arc<Attributes::NFAttributes> = outAttributes.clone();
                    outAttributes = attributes.clone();
                    Ok(Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => {
                    println!("{}", (literal!("NFInst.instTypeSpec: TCOMPLEX not implemented.\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((node, outAttributes))
}

pub fn checkRecursiveDefinition(mut componentType: Arc<InstNode::InstNode>, mut component: Arc<InstNode::InstNode>, mut limitReached: bool) -> Result<()> {
    let mut parent: Arc<InstNode::InstNode> = InstNode::parent(component.clone());
    let mut parent_type: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if !(Class::isFunction(InstNode::getClass(parent.clone())?)) {
        while !(InstNode::isEmpty(parent.clone())) {
            parent_type = InstNode::classScope(parent.clone());
            if referenceEq(&InstNode::definition(componentType.clone())?,&InstNode::definition(parent_type.clone())?) {
                Error::addSourceMessage(Error::RECURSIVE_DEFINITION.clone(), list![(InstNode::name(component.clone())?).clone(), (InstNode::name(InstNode::classScope(InstNode::parent(component.clone())))?).clone()], InstNode::info(component.clone())?)?;
                InstNode::componentApply(component.clone(), (std::sync::Arc::new(Component::setClassInstance) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?;
                bail!("fail");
            }
            parent = InstNode::parent(parent.clone());
        }
    }
    if limitReached.clone() {
        Error::addSourceMessage(Error::INST_RECURSION_LIMIT_REACHED.clone(), list![(AbsynUtil::pathString(InstNode::scopePath(component.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone()], InstNode::info(component.clone())?)?;
        InstNode::componentApply(component.clone(), (std::sync::Arc::new(Component::setClassInstance) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?;
        bail!("fail");
    }
    Ok(())
}

pub fn updateParameterBinding(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    if InstContext::inRedeclared(context.clone()) {
        return Ok(());
    }
    comp = InstNode::component(node.clone())?;
    if !(Component::isFixed(comp.clone())?) || InstNode::hasBinding(node.clone())? {
        return Ok(());
    }
    binding = Component::getTypeAttributeBinding(comp.clone(), (literal!("start")).clone());
    if Binding::isBound(binding.clone()) && !(Binding::hasTypeOrigin(binding.clone())?) {
        if !(InstContext::inRelaxed(context.clone())) {
            Error::addSourceMessage(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone(), list![(AbsynUtil::pathString(InstNode::scopePath(node.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone(), (Binding::toString(binding.clone(), (literal!("")).clone())?).clone()], InstNode::info(node.clone())?)?;
        }
        binding = Binding::unpropagate(binding.clone(), node.clone());
        if Binding::isEach(binding.clone()) {
            binding = Binding::expandEach(binding.clone(), node.clone())?;
        }
        comp = Component::setBinding(binding.clone(), comp.clone())?;
        InstNode::updateComponent(comp.clone(), node.clone())?;
    }
    Ok(())
}

pub fn instDimension(mut dimension: Arc<Dimension::NFDimension>, mut context: i32, mut settings: Arc<InstSettings::InstSettings>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut dimension: Arc<Dimension::NFDimension> = dimension;
    dimension = (::match_deref::match_deref! { match &(dimension.clone()) {
        Deref @ Dimension::RAW_DIM { dim, .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            'mc: {
        let __mc_input = dim.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Subscript::NOSUB => {
                    Ok(Arc::new(crate::NFDimension::UNKNOWN))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
                    let mut exp: Arc<Expression::NFExpression>;
                    exp = instExp(var_field!((**dim).subscript, Absyn::Subscript::SUBSCRIPT).clone(), var_field!((*dimension).scope, Dimension::NFDimension::RAW_DIM).clone(), context.clone(), info.clone())?;
                    if settings.resizableArrays.clone() {
                        exp = Expression::map(exp.clone(), (std::sync::Arc::new(fnptr!(instResizable, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                    }
                    Ok(Arc::new(Dimension::NFDimension::UNTYPED { dimension: exp.clone(), isProcessing: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((InstContext::inRelaxed(context.clone()))) { bail!("guard") }
                    Ok(Arc::new(crate::NFDimension::UNKNOWN))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
        },
        _ => {
            dimension.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dimension)
}

pub fn instResizable(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { node: node @ Deref @ InstNode::COMPONENT_NODE { .. }, .. }, .. } if (Component::variability(Pointer::access(var_field!((**node).component, InstNode::InstNode::COMPONENT_NODE).clone())) == Variability::PARAMETER.clone()) => {
            let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
            comp = Pointer::access(var_field!((**node).component, InstNode::InstNode::COMPONENT_NODE).clone());
            let _ = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT { attributes: attr, .. } => {
            let mut attr = (*attr).clone();
            assign_field!(
                attr.variability = Variability::NON_STRUCTURAL_PARAMETER.clone(),
                attr.isResizable = true
            );
            assign_variant_field!(comp => Component::NFComponent::COMPONENT; attributes = attr.clone());
            Pointer::update(var_field!((**node).component, InstNode::InstNode::COMPONENT_NODE).clone(), comp.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn instExpressions(mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut sections: Arc<Sections::NFSections>, mut connectBreaks: Arc<ConnectBreakTree::Tree>, mut context: i32, mut settings: Arc<InstSettings::InstSettings>) -> Result<Arc<Sections::NFSections>> {
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut cls: Arc<Class::NFClass> = InstNode::getClass(node.clone())?;
    let mut inst_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut local_comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut dims: metamodelica::Array<Arc<Dimension::NFDimension>>;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut next_context: i32 = 0;
    let mut connect_breaks: Arc<ConnectBreakTree::Tree> = Arc::new(ConnectBreakTree::Tree::EMPTY);
    let mut local_connect_breaks: Arc<metamodelica::List<Mutable::Mutable<ConnectBreakTree::Entry>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { restriction: Deref @ Restriction::TYPE, elements: cls_tree, .. } => {
            let mut cls_tree = (*cls_tree).clone();
            exts = ClassTree::getExtends(cls_tree.clone());
            let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range0 {
                instExpressions(ext.clone(), ext.clone(), sections.clone(), connectBreaks.clone(), context.clone(), settings.clone())?;
            }
            if (exts.clone().borrow().len() as i32) == 1 {
                ty = Arc::new(Type::NFType::COMPLEX { cls: node.clone(), complexTy: Arc::new(ComplexType::NFComplexType::EXTENDS_TYPE { baseClass: exts.borrow()[(1-1) as usize].clone() }) });
            } else if SCodeUtil::hasBooleanNamedAnnotationInClass(InstNode::definition(node.clone())?, (literal!("__OpenModelica_builtinType")).clone())? {
                ty = Arc::new(Type::NFType::COMPLEX { cls: node.clone(), complexTy: Arc::new(crate::NFComplexType::CLASS) });
            } else {
                Error::addSourceMessage(Error::MISSING_TYPE_BASETYPE.clone(), list![(InstNode::name(node.clone())?).clone()], InstNode::info(node.clone())?)?;
                bail!("fail");
            }
            cls_tree = ClassTree::flatten(cls_tree.clone())?;
            inst_cls = Arc::new(Class::NFClass::INSTANCED_CLASS { ty: ty.clone(), elements: cls_tree.clone(), sections: Arc::new(crate::NFSections::EMPTY), prefixes: var_field!((*cls).prefixes, Class::NFClass::EXPANDED_CLASS).clone(), restriction: var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone() });
            InstNode::updateClass(inst_cls.clone(), node.clone())?;
            ()
        },
        Deref @ Class::EXPANDED_CLASS { elements: cls_tree, .. } => {
            (connect_breaks, local_connect_breaks) = ConnectBreakTree::appendBreaksInNode(node.clone(), connectBreaks.clone())?;
            if settings.mergeExtendsSections.clone() {
                let __range0 = ClassTree::getExtends(cls_tree.clone()).borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range0 {
                    sections = instExpressions(ext.clone(), ext.clone(), sections.clone(), connect_breaks.clone(), context.clone(), settings.clone())?;
                }
            } else {
                let __range1 = ClassTree::getExtends(cls_tree.clone()).borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range1 {
                    let _ = instExpressions(ext.clone(), ext.clone(), sections.clone(), connect_breaks.clone(), context.clone(), settings.clone())?;
                }
            }
            ClassTree::applyLocalComponents(cls_tree.clone(), Arc::new({ let __pe_b1 = context.clone(); let __pe_b2 = settings.clone(); move |__pe_a0| instComponentExpressions(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            assign_variant_field!(cls => Class::NFClass::EXPANDED_CLASS; elements = ClassTree::flatten(cls_tree.clone())?);
            InstNode::updateClass(cls.clone(), node.clone())?;
            next_context = if (Restriction::isFunction(var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone())) {InstContext::FUNCTION.clone()} else {InstContext::CLASS.clone()};
            next_context = InstContext::set(context.clone(), next_context.clone());
            sections = instSections(node.clone(), scope.clone(), connect_breaks.clone(), next_context.clone(), sections.clone())?;
            ConnectBreakTree::checkUnmatchedBreaks(local_connect_breaks.clone())?;
            ty = makeComplexType(var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone(), node.clone(), cls.clone())?;
            inst_cls = Arc::new(Class::NFClass::INSTANCED_CLASS { ty: ty.clone(), elements: var_field!((*cls).elements, Class::NFClass::EXPANDED_CLASS).clone(), sections: sections.clone(), prefixes: var_field!((*cls).prefixes, Class::NFClass::EXPANDED_CLASS).clone(), restriction: var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone() });
            InstNode::updateClass(inst_cls.clone(), node.clone())?;
            instComplexType(ty.clone(), context.clone())?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { dims, .. } => {
            sections = instExpressions(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), scope.clone(), sections.clone(), connectBreaks.clone(), context.clone(), settings.clone())?;
            info = InstNode::info(node.clone())?;
            let __range0 = 1..=(dims.clone().borrow().len() as i32);
            for mut i in __range0 {
                {
                    let __cell1 = instDimension(dims.borrow()[(i.clone()-1) as usize].clone(), context.clone(), settings.clone(), info.clone())?;
                    dims.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
            if Restriction::isRecord(var_field!((*cls).restriction, Class::NFClass::EXPANDED_DERIVED).clone()) {
                instRecordConstructor(node.clone(), context.clone())?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { elements: Deref @ ClassTree::FLAT_TREE { components: local_comps, .. }, .. } => {
            let __range0 = local_comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                instComponentExpressions(comp.clone(), context.clone(), settings.clone())?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::INSTANCED_CLASS { .. } => (),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instExpressions")); __mm_s.push_str(&*literal!(" got invalid class")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sections)
}

pub fn makeComplexType(mut restriction: Arc<Restriction::NFRestriction>, mut node: Arc<InstNode::InstNode>, mut cls: Arc<Class::NFClass>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cty: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    cty = (::match_deref::match_deref! { match &(restriction.clone()) {
        Deref @ Restriction::RECORD { .. } => makeRecordComplexType(InstNode::classScope(InstNode::getDerivedNode(node.clone(), true)), cls.clone())?,
        _ => Arc::new(crate::NFComplexType::CLASS),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty = Arc::new(Type::NFType::COMPLEX { cls: node.clone(), complexTy: cty.clone() });
    Ok(ty)
}

pub fn makeRecordComplexType(mut node: Arc<InstNode::InstNode>, mut cls: Arc<Class::NFClass>) -> Result<Arc<ComplexType::NFComplexType>> {
    let mut ty: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    cls_node = if (SCodeUtil::isOperatorRecord(InstNode::definition(node.clone())?)) {InstNode::classScope(node.clone())} else {InstNode::classScope(InstNode::getDerivedNode(node.clone(), true))};
    ty = Arc::new(ComplexType::NFComplexType::RECORD { constructor: cls_node.clone(), fields: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), indexMap: indexMap.clone() });
    Ok(ty)
}

pub fn instComplexType(mut ty: Arc<Type::NFType>, mut context: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: node, .. }, .. } if (!(InstNode::isModel(node.clone()))) => {
            instRecordConstructor(node.clone(), context.clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn instRecordConstructor(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
    cache = InstNode::getFuncCache(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { .. } => (),
        _ => {
            InstNode::cacheInitFunc(node.clone())?;
            if SCodeUtil::isOperatorRecord(InstNode::definition(node.clone())?) {
                OperatorOverloading::instConstructor(InstNode::fullPath(node.clone(), false), node.clone(), context.clone(), InstNode::info(node.clone())?)?;
            } else {
                Record::instDefaultConstructor(InstNode::fullPath(node.clone(), false), node.clone(), context.clone(), InstNode::info(node.clone())?)?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn instBuiltinAttribute(mut attribute: Arc<Modifier::Modifier>, mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Modifier::Modifier>> {
    let mut attribute: Arc<Modifier::Modifier> = attribute;
    let () = (::match_deref::match_deref! { match &(attribute.clone()) {
        Deref @ Modifier::MODIFIER { binding, .. } => {
            assign_variant_field!(attribute => Modifier::Modifier::MODIFIER; binding = instBinding(binding.clone(), context.clone())?);
            ()
        },
        Deref @ Modifier::REDECLARE { .. } => {
            Error::addSourceMessage(Error::INVALID_REDECLARE_IN_BASIC_TYPE.clone(), list![(Modifier::name(attribute.clone())?).clone()], Modifier::info(attribute.clone())?)?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attribute)
}

pub fn instComponentExpressions(mut component: Arc<InstNode::InstNode>, mut context: i32, mut settings: Arc<InstSettings::InstSettings>) -> Result<()> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut dims: metamodelica::Array<Arc<Dimension::NFDimension>>;
    if InstNode::isEmpty(component.clone()) {
        return Ok(());
    }
    node = InstNode::resolveInner(component.clone());
    c = InstNode::component(node.clone())?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: dims, .. }, .. } if (var_field!((*c).state, Component::NFComponent::COMPONENT).clone() == ComponentState::PartiallyInstantiated.clone()) => {
            assign_variant_field!(c => Component::NFComponent::COMPONENT; state = ComponentState::FullyInstantiated.clone());
            InstNode::updateComponent(c.clone(), node.clone())?;
            assign_variant_field!(c => Component::NFComponent::COMPONENT;
                binding = instBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context.clone())?,
                condition = instBinding(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone(), context.clone())?
            );
            if !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                instExpressions(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), node.clone(), Arc::new(crate::NFSections::EMPTY), ConnectBreakTree::new(), context.clone(), settings.clone())?;
            }
            let __range0 = 1..=(dims.clone().borrow().len() as i32);
            for mut i in __range0 {
                {
                    let __cell1 = instDimension(dims.borrow()[(i.clone()-1) as usize].clone(), context.clone(), settings.clone(), var_field!((*c).info, Component::NFComponent::COMPONENT).clone())?;
                    dims.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
                }
            }
            InstNode::updateComponent(c.clone(), node.clone())?;
            ()
        },
        Deref @ Component::COMPONENT { .. } => (),
        Deref @ Component::ENUM_LITERAL { .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { modifier: Deref @ Modifier::NOMOD, .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { .. } => {
            assign_variant_field!(c => Component::NFComponent::TYPE_ATTRIBUTE; modifier = instBuiltinAttribute(var_field!((*c).modifier, Component::NFComponent::TYPE_ATTRIBUTE).clone(), component.clone(), context.clone())?);
            InstNode::updateComponent(c.clone(), node.clone())?;
            ()
        },
        _ => {
            if !(InstContext::inRelaxed(context.clone())) {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instComponentExpressions")); __mm_s.push_str(&*literal!(" got invalid component")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("fail");
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn instBinding(mut binding: Arc<Binding::NFBinding>, mut context: i32) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    if InstContext::inInstanceAPI(context.clone()) {
        ErrorExt::setCheckpoint((literal!("NFInst.instBinding")).clone());
        match '__try0: {
            binding = unwrap_break_err!(instBinding(binding.clone(), InstContext::unset(context.clone(), InstContext::INSTANCE_API.clone())), '__try0);
            Ok::<_, anyhow::Error>((binding.clone(),))
        } {
            Ok((__try0_o0,)) => {
                binding = __try0_o0;
            }
            Err(_) => {
                binding = Arc::new(Binding::NFBinding::INVALID_BINDING { binding: binding.clone(), errors: ErrorExt::getCheckpointMessages() });
            }
        }
        ErrorExt::delCheckpoint((literal!("NFInst.instBinding")).clone());
    } else {
        binding = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::RAW_BINDING { bindingExp: Deref @ Absyn::Exp::BREAK { .. }, .. } => {
            Arc::new(crate::NFBinding::UNBOUND)
        },
        Deref @ Binding::RAW_BINDING { .. } => {
            let mut bind_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            bind_exp = instExp(var_field!((*binding).bindingExp, Binding::NFBinding::RAW_BINDING).clone(), var_field!((*binding).scope, Binding::NFBinding::RAW_BINDING).clone(), context.clone(), var_field!((*binding).info, Binding::NFBinding::RAW_BINDING).clone())?;
            if !(var_field!((*binding).subs, Binding::NFBinding::RAW_BINDING).clone().is_empty()) {
                bind_exp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: bind_exp.clone(), subscripts: var_field!((*binding).subs, Binding::NFBinding::RAW_BINDING).clone(), ty: Arc::new(crate::NFType::UNKNOWN), split: true });
            }
            Arc::new(Binding::NFBinding::UNTYPED_BINDING { bindingExp: bind_exp.clone(), isProcessing: false, scope: var_field!((*binding).scope, Binding::NFBinding::RAW_BINDING).clone(), eachType: var_field!((*binding).eachType, Binding::NFBinding::RAW_BINDING).clone(), source: var_field!((*binding).source, Binding::NFBinding::RAW_BINDING).clone(), info: var_field!((*binding).info, Binding::NFBinding::RAW_BINDING).clone() })
        },
        _ => {
            binding.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(binding)
}

pub fn instExpOpt(mut absynExp: Option<Arc<Absyn::Exp>>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut exp: Option<Arc<Expression::NFExpression>> = None;
    exp = (::match_deref::match_deref! { match &(absynExp.clone()) {
        None => {
            None
        },
        Some(aexp) => {
            Some(instExp(aexp.clone(), scope.clone(), context.clone(), info.clone())?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn instExp(mut absynExp: Arc<Absyn::Exp>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(absynExp.clone()) {
        Deref @ Absyn::Exp::INTEGER { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*absynExp).value, Absyn::Exp::INTEGER).clone() })
        },
        Deref @ Absyn::Exp::REAL { .. } => {
            Arc::new(Expression::NFExpression::REAL { value: stringReal((var_field!((*absynExp).value, Absyn::Exp::REAL).clone()).clone())? })
        },
        Deref @ Absyn::Exp::STRING { .. } => {
            Arc::new(Expression::NFExpression::STRING { value: (System::unescapedString((var_field!((*absynExp).value, Absyn::Exp::STRING).clone()).clone())).clone() })
        },
        Deref @ Absyn::Exp::BOOL { .. } => {
            Arc::new(Expression::NFExpression::BOOLEAN { value: var_field!((*absynExp).value, Absyn::Exp::BOOL).clone() })
        },
        Deref @ Absyn::Exp::CREF { .. } => {
            instCref(var_field!((*absynExp).componentRef, Absyn::Exp::CREF).clone(), scope.clone(), context.clone(), info.clone())?
        },
        Deref @ Absyn::Exp::ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            arr = Array::mapList(var_field!((*absynExp).arrayExp, Absyn::Exp::ARRAY).clone(), Arc::new({ let __pe_b1 = scope.clone(); let __pe_b2 = context.clone(); let __pe_b3 = info.clone(); move |__pe_a0| instExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            Expression::makeArrayCheckLiteral(Arc::new(crate::NFType::UNKNOWN), arr.clone())
        },
        Deref @ Absyn::Exp::MATRIX { .. } => {
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>> = metamodelica::nil();
            expll = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Expression::NFExpression>>>>> = metamodelica::nil();
        for mut el in (var_field!((*absynExp).matrix, Absyn::Exp::MATRIX).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = instExp(e.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Expression::NFExpression::MATRIX { elements: expll.clone() })
        },
        Deref @ Absyn::Exp::RANGE { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut oe: Option<Arc<Expression::NFExpression>> = None;
            e1 = instExp(var_field!((*absynExp).start, Absyn::Exp::RANGE).clone(), scope.clone(), context.clone(), info.clone())?;
            oe = instExpOpt(var_field!((*absynExp).step, Absyn::Exp::RANGE).clone(), scope.clone(), context.clone(), info.clone())?;
            e3 = instExp(var_field!((*absynExp).stop, Absyn::Exp::RANGE).clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Expression::NFExpression::RANGE { ty: Arc::new(crate::NFType::UNKNOWN), start: e1.clone(), step: oe.clone(), stop: e3.clone() })
        },
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: absynExp1, tail: Deref @ metamodelica::List::Nil } } => {
            instExp(absynExp1.clone(), scope.clone(), context.clone(), info.clone())?
        },
        Deref @ Absyn::Exp::TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*absynExp).expressions, Absyn::Exp::TUPLE).clone()).into_iter().cloned() {
            let __x = instExp(e.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Expression::NFExpression::TUPLE { ty: Arc::new(crate::NFType::UNKNOWN), elements: expl.clone() })
        },
        Deref @ Absyn::Exp::BINARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut op: Arc<Operator::NFOperator>;
            e1 = instExp(var_field!((*absynExp).exp1, Absyn::Exp::BINARY).clone(), scope.clone(), context.clone(), info.clone())?;
            e2 = instExp(var_field!((*absynExp).exp2, Absyn::Exp::BINARY).clone(), scope.clone(), context.clone(), info.clone())?;
            op = Operator::fromAbsyn(var_field!((*absynExp).op, Absyn::Exp::BINARY).clone())?;
            Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        Deref @ Absyn::Exp::UNARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut op: Arc<Operator::NFOperator>;
            e1 = instExp(var_field!((*absynExp).exp, Absyn::Exp::UNARY).clone(), scope.clone(), context.clone(), info.clone())?;
            op = Operator::fromAbsyn(var_field!((*absynExp).op, Absyn::Exp::UNARY).clone())?;
            Expression::makeUnary(op.clone(), e1.clone())
        },
        Deref @ Absyn::Exp::LBINARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut op: Arc<Operator::NFOperator>;
            e1 = instExp(var_field!((*absynExp).exp1, Absyn::Exp::LBINARY).clone(), scope.clone(), context.clone(), info.clone())?;
            e2 = instExp(var_field!((*absynExp).exp2, Absyn::Exp::LBINARY).clone(), scope.clone(), context.clone(), info.clone())?;
            op = Operator::fromAbsyn(var_field!((*absynExp).op, Absyn::Exp::LBINARY).clone())?;
            Arc::new(Expression::NFExpression::LBINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        Deref @ Absyn::Exp::LUNARY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut op: Arc<Operator::NFOperator>;
            e1 = instExp(var_field!((*absynExp).exp, Absyn::Exp::LUNARY).clone(), scope.clone(), context.clone(), info.clone())?;
            op = Operator::fromAbsyn(var_field!((*absynExp).op, Absyn::Exp::LUNARY).clone())?;
            Arc::new(Expression::NFExpression::LUNARY { operator: op.clone(), exp: e1.clone() })
        },
        Deref @ Absyn::Exp::RELATION { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut op: Arc<Operator::NFOperator>;
            e1 = instExp(var_field!((*absynExp).exp1, Absyn::Exp::RELATION).clone(), scope.clone(), context.clone(), info.clone())?;
            e2 = instExp(var_field!((*absynExp).exp2, Absyn::Exp::RELATION).clone(), scope.clone(), context.clone(), info.clone())?;
            op = Operator::fromAbsyn(var_field!((*absynExp).op, Absyn::Exp::RELATION).clone())?;
            Arc::new(Expression::NFExpression::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: -1 })
        },
        Deref @ Absyn::Exp::IFEXP { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e3 = instExp(var_field!((*absynExp).elseBranch, Absyn::Exp::IFEXP).clone(), scope.clone(), context.clone(), info.clone())?;
            for mut branch in &*var_field!((*absynExp).elseIfBranch, Absyn::Exp::IFEXP).clone().reverse() {
                let mut branch = branch.clone();
                e1 = instExp(Util::tuple21(branch.clone()), scope.clone(), context.clone(), info.clone())?;
                e2 = instExp(Util::tuple22(branch.clone()), scope.clone(), context.clone(), info.clone())?;
                e3 = Arc::new(Expression::NFExpression::IF { ty: Arc::new(crate::NFType::UNKNOWN), condition: e1.clone(), trueBranch: e2.clone(), falseBranch: e3.clone() });
            }
            e1 = instExp(var_field!((*absynExp).ifExp, Absyn::Exp::IFEXP).clone(), scope.clone(), context.clone(), info.clone())?;
            e2 = instExp(var_field!((*absynExp).trueBranch, Absyn::Exp::IFEXP).clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Expression::NFExpression::IF { ty: Arc::new(crate::NFType::UNKNOWN), condition: e1.clone(), trueBranch: e2.clone(), falseBranch: e3.clone() })
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            Call::instantiate(var_field!((*absynExp).function_, Absyn::Exp::CALL).clone(), var_field!((*absynExp).functionArgs, Absyn::Exp::CALL).clone(), scope.clone(), context.clone(), info.clone())?
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { .. } => {
            instPartEvalFunction(var_field!((*absynExp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), var_field!((*absynExp).functionArgs, Absyn::Exp::PARTEVALFUNCTION).clone(), scope.clone(), context.clone(), info.clone())?
        },
        Deref @ Absyn::Exp::END { .. } => {
            Arc::new(crate::NFExpression::END)
        },
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { .. } => {
            instExp(var_field!((*absynExp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), scope.clone(), context.clone(), info.clone())?
        },
        Deref @ Absyn::Exp::SUBSCRIPTED_EXP { .. } => {
            Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: instExp(var_field!((*absynExp).exp, Absyn::Exp::SUBSCRIPTED_EXP).clone(), scope.clone(), context.clone(), info.clone())?, subscripts: {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*absynExp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = instSubscript(Arc::new(Subscript::NFSubscript::RAW_SUBSCRIPT { subscript: s.clone() }), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, ty: Arc::new(crate::NFType::UNKNOWN), split: false })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instExp")); __mm_s.push_str(&*literal!(" got unknown expression: ")); __mm_s.push_str(&*Dump::printExpStr(absynExp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn instCref(mut absynCref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut crefExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut prefixed_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut found_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    (cref, found_scope) = (::match_deref::match_deref! { match &(absynCref.clone()) {
        Deref @ Absyn::ComponentRef::WILD { .. } => (Arc::new(crate::NFComponentRef::WILD), scope.clone()),
        Deref @ Absyn::ComponentRef::ALLWILD { .. } => (Arc::new(crate::NFComponentRef::WILD), scope.clone()),
        _ => Lookup::lookupComponent(absynCref.clone(), scope.clone(), context.clone(), info.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref = instCrefSubscripts(cref.clone(), scope.clone(), context.clone(), info.clone())?;
    crefExp = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => (::match_deref::match_deref! { match &(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone()) {
        Deref @ InstNode::COMPONENT_NODE { .. } => instCrefComponent(cref.clone(), var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), found_scope.clone(), info.clone())?,
        Deref @ InstNode::CLASS_NODE { .. } => if (Class::isFunction(InstNode::getClass(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?)) {instCrefFunction(cref.clone(), found_scope.clone(), context.clone(), info.clone())?} else {instCrefTypename(cref.clone(), var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), info.clone())?},
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instCref")); __mm_s.push_str(&*literal!(" got invalid instance node")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: cref.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefExp)
}

pub fn instCrefComponent(mut cref: Arc<ComponentRef::NFComponentRef>, mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut crefExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut prefixed_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    comp = InstNode::component(node.clone())?;
    crefExp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::ITERATOR { .. } => {
            checkUnsubscriptableCref(cref.clone(), info.clone())?;
            Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: ComponentRef::makeIterator(node.clone(), var_field!((*comp).ty, Component::NFComponent::ITERATOR).clone()) })
        },
        Deref @ Component::ENUM_LITERAL { .. } => {
            checkUnsubscriptableCref(cref.clone(), info.clone())?;
            var_field!((*comp).literal, Component::NFComponent::ENUM_LITERAL).clone()
        },
        Deref @ Component::TYPE_ATTRIBUTE { .. } => {
            Error::addSourceMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(InstNode::name(node.clone())?).clone(), (InstNode::name(InstNode::parent(node.clone()))?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: ComponentRef::appendScope(scope.clone(), cref.clone(), false)? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefExp)
}

pub fn instCrefFunction(mut cref: Arc<ComponentRef::NFComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut crefExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    fn_ref = ComponentRef::appendScope(scope.clone(), cref.clone(), true)?;
    (fn_ref, _, _) = Function::instFunctionRef(fn_ref.clone(), context.clone(), info.clone())?;
    crefExp = Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: fn_ref.clone() });
    Ok(crefExp)
}

pub fn instCrefTypename(mut cref: Arc<ComponentRef::NFComponentRef>, mut node: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut crefExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    checkUnsubscriptableCref(cref.clone(), info.clone())?;
    ty = InstNode::getType(node.clone())?;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::BOOLEAN => Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![Arc::new(crate::NFDimension::BOOLEAN)] }),
        Deref @ Type::ENUMERATION { .. } => Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![Arc::new(Dimension::NFDimension::ENUM { enumType: ty.clone() })] }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instCrefTypename")); __mm_s.push_str(&*literal!(" got unknown class node ")); __mm_s.push_str(&*InstNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    crefExp = Arc::new(Expression::NFExpression::TYPENAME { ty: ty.clone() });
    Ok(crefExp)
}

pub fn checkUnsubscriptableCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<()> {
    if ComponentRef::hasSubscripts(cref.clone()) {
        Error::addSourceMessage(Error::WRONG_NUMBER_OF_SUBSCRIPTS.clone(), list![(ComponentRef::toString(cref.clone())?).clone(), ArcStr::from(::std::format!("{}", (ComponentRef::getSubscripts(cref.clone()).len() as i32))), (literal!("0")).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn instCrefSubscripts(mut cref: Arc<ComponentRef::NFComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => {
            let mut rest_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            if !(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone().is_empty()) {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = instSubscript(s.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            rest_cr = instCrefSubscripts(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), scope.clone(), context.clone(), info.clone())?;
            if !(referenceEq(&rest_cr.clone(),&var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone())) {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = rest_cr.clone());
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn instSubscript(mut subscript: Arc<Subscript::NFSubscript>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Subscript::NFSubscript>> {
    let mut outSubscript: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut absynSub: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    let __pa0 = ::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::RAW_SUBSCRIPT { subscript: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    absynSub = __pa0.clone();
    outSubscript = (::match_deref::match_deref! { match &(absynSub.clone()) {
        Deref @ Absyn::Subscript::NOSUB { .. } => Arc::new(crate::NFSubscript::WHOLE),
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            exp = instExp(var_field!((*absynSub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), scope.clone(), context.clone(), info.clone())?;
            Subscript::fromExp(exp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscript)
}

pub fn instPartEvalFunction(mut func: Arc<Absyn::ComponentRef>, mut funcArgs: Arc<Absyn::FunctionArgs>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arg_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(funcArgs.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    nargs = __pa0.clone();
    outExp = instCref(func.clone(), scope.clone(), context.clone(), info.clone())?;
    if !(nargs.clone().is_empty()) {
        fn_ref = Expression::toCref(outExp.clone())?;
        args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (nargs.clone()).into_iter().cloned() {
            let __x = instExp(arg.argValue.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        arg_names = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (nargs.clone()).into_iter().cloned() {
            let __x = arg.argName.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        outExp = Arc::new(Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION { r#fn: fn_ref.clone(), args: args.clone(), argNames: arg_names.clone(), ty: Arc::new(crate::NFType::UNKNOWN) });
    }
    Ok(outExp)
}

pub fn instSections(mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut connectBreaks: Arc<ConnectBreakTree::Tree>, mut context: i32, mut sections: Arc<Sections::NFSections>) -> Result<Arc<Sections::NFSections>> {
    let mut sections: Arc<Sections::NFSections> = sections;
    let mut el: Arc<SCode::Element> = InstNode::definition(node.clone())?;
    let mut def: Arc<SCode::ClassDef>;
    sections = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { .. }, .. } => instSections2(var_field!((*el).classDef, SCode::Element::CLASS).clone(), scope.clone(), connectBreaks.clone(), context.clone(), sections.clone())?,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: def @ Deref @ SCode::ClassDef::PARTS { .. }, .. }, .. } => instSections2(def.clone(), scope.clone(), connectBreaks.clone(), context.clone(), sections.clone())?,
        _ => sections.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sections)
}

pub fn instSections2(mut parts: Arc<SCode::ClassDef>, mut scope: Arc<InstNode::InstNode>, mut connectBreaks: Arc<ConnectBreakTree::Tree>, mut context: i32, mut sections: Arc<Sections::NFSections>) -> Result<Arc<Sections::NFSections>> {
    let mut sections: Arc<Sections::NFSections> = sections;
    sections = (::match_deref::match_deref! { match &((parts.clone(), sections.clone())) {
        (Deref @ SCode::ClassDef::PARTS { externalDecl: Some(ext_decl), .. }, Deref @ Sections::EXTERNAL { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdMultipleExternalDeclarations")).clone())?) => {
            instExternalDecl(ext_decl.clone(), scope.clone(), context.clone())?
        },
        (_, Deref @ Sections::EXTERNAL { .. }) if (SCodeUtil::classDefHasSections(parts.clone(), true)) => {
            Error::addMultiSourceMessage(Error::MULTIPLE_SECTIONS_IN_FUNCTION.clone(), list![(InstNode::name(scope.clone())?).clone()], list![var_field!((*sections).info, Sections::NFSections::EXTERNAL).clone(), InstNode::info(scope.clone())?])?;
            bail!("fail")
        },
        (Deref @ SCode::ClassDef::PARTS { externalDecl: Some(ext_decl), .. }, _) => {
            if SCodeUtil::classDefHasSections(parts.clone(), false) {
                Error::addSourceMessage(Error::MULTIPLE_SECTIONS_IN_FUNCTION.clone(), list![(InstNode::name(scope.clone())?).clone()], InstNode::info(scope.clone())?)?;
                bail!("fail");
            }
            instExternalDecl(ext_decl.clone(), scope.clone(), context.clone())?
        },
        (Deref @ SCode::ClassDef::PARTS { .. }, _) => {
            let mut eq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut ieq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut alg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
            let mut ialg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
            let mut icontext: i32 = 0;
            icontext = InstContext::set(context.clone(), InstContext::INITIAL.clone());
            eq = instEquations(var_field!((*parts).normalEquationLst, SCode::ClassDef::PARTS).clone(), scope.clone(), connectBreaks.clone(), context.clone())?;
            ieq = instEquations(var_field!((*parts).initialEquationLst, SCode::ClassDef::PARTS).clone(), scope.clone(), connectBreaks.clone(), icontext.clone())?;
            alg = instAlgorithmSections(var_field!((*parts).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), scope.clone(), context.clone())?;
            ialg = instAlgorithmSections(var_field!((*parts).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), scope.clone(), icontext.clone())?;
            Sections::join(Sections::new(eq.clone(), ieq.clone(), alg.clone(), ialg.clone()), sections.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sections)
}

pub fn instExternalDecl(mut extDecl: Arc<SCode::ExternalDecl>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Sections::NFSections>> {
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    sections = (::match_deref::match_deref! { match &(extDecl.clone()) {
        Deref @ SCode::ExternalDecl { .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut lang: ArcStr = arcstr::literal!("");
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut ret_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            info = InstNode::info(scope.clone())?;
            name = (Util::getOptionOrDefault(extDecl.funcName.clone(), (InstNode::name(scope.clone())?).clone())).clone();
            lang = (Util::getOptionOrDefault(extDecl.lang.clone(), (literal!("C")).clone())).clone();
            checkExternalDeclLanguage((lang.clone()).clone(), info.clone())?;
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (extDecl.args.clone()).into_iter().cloned() {
            let __x = instExp(arg.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if isSome(extDecl.output_.clone()) {
                (ret_cref, _) = Lookup::lookupLocalComponent(Util::getOption(extDecl.output_.clone())?, scope.clone(), context.clone(), info.clone())?;
            } else {
                ret_cref = Arc::new(crate::NFComponentRef::EMPTY);
            }
            Arc::new(Sections::NFSections::EXTERNAL { name: (name.clone()).clone(), args: args.clone(), outputRef: ret_cref.clone(), language: (lang.clone()).clone(), ann: extDecl.annotation_.clone(), explicit: isSome(extDecl.funcName.clone()), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sections)
}

pub fn checkExternalDeclLanguage(mut language: ArcStr, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(language.clone()) {
        Deref @ "C" => (),
        Deref @ "FORTRAN 77" => (),
        Deref @ "Fortran 77" => (),
        Deref @ "builtin" => (),
        _ => {
            Error::addSourceMessage(Error::INVALID_EXTERNAL_LANGUAGE.clone(), list![(language.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn instEquations(mut scodeEql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut scope: Arc<InstNode::InstNode>, mut connectBreaks: Arc<ConnectBreakTree::Tree>, mut context: i32) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut instEql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    if InstContext::inInstanceAPI(context.clone()) {
        for mut eq in &*filterInstanceAPIEquations(scodeEql.clone()) {
            let mut eq = eq.clone();
            if '__try0: {
                instEql = unwrap_break_err!(instEquation(eq.clone(), scope.clone(), connectBreaks.clone(), context.clone(), instEql.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
    } else {
        for mut eq in &*scodeEql.clone() {
            let mut eq = eq.clone();
            instEql = instEquation(eq.clone(), scope.clone(), connectBreaks.clone(), context.clone(), instEql.clone())?;
        }
    }
    instEql = instEql.clone().reverse();
    Ok(instEql)
}

pub fn filterInstanceAPIEquations(mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Arc<metamodelica::List<Arc<SCode::Equation>>> {
    let mut outEql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        outEql = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_CONNECT { .. } => cons(eq.clone(), outEql.clone()),
        Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. }, .. }, .. } if (name.clone() == literal!("transition") || name.clone() == literal!("initialState")) => cons(eq.clone(), outEql.clone()),
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = filterInstanceAPIEquations(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone()));
            if (var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone().is_empty()) {outEql.clone()} else {cons(eq.clone(), outEql.clone())}
        },
        Deref @ SCode::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                thenBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
        for mut eql in (var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = filterInstanceAPIEquations(eql.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = filterInstanceAPIEquations(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone())
            );
            if (List::all(var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone(), std::sync::Arc::new(fnptr!(listEmpty, _))) && var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone().is_empty()) {outEql.clone()} else {cons(eq.clone(), outEql.clone())}
        },
        _ => outEql.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outEql = outEql.clone().reverse();
    outEql
}

pub fn instEquation(mut scodeEq: Arc<SCode::Equation>, mut scope: Arc<InstNode::InstNode>, mut connectBreaks: Arc<ConnectBreakTree::Tree>, mut context: i32, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    equations = (::match_deref::match_deref! { match &(scodeEq.clone()) {
        Deref @ SCode::Equation::EQ_EQUALS { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeEq).expLeft, SCode::Equation::EQ_EQUALS).clone(), scope.clone(), context.clone(), info.clone())?;
            exp2 = instExp(var_field!((*scodeEq).expRight, SCode::Equation::EQ_EQUALS).clone(), scope.clone(), context.clone(), info.clone())?;
            cons(Equation::makeEquality(exp1.clone(), exp2.clone(), Arc::new(crate::NFType::UNKNOWN), makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_EQUALS).clone(), info.clone()), scope.clone(), Equation::ScalarizeMode::NO_PREFERENCE.clone()), equations.clone())
        },
        Deref @ SCode::Equation::EQ_CONNECT { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut lhs_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut rhs_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut next_context: i32 = 0;
            if InstContext::inInitial(context.clone()) {
                Error::addSourceMessage(Error::CONNECT_IN_INITIAL_EQUATION.clone(), metamodelica::nil(), info.clone())?;
                bail!("fail");
            } else if InstContext::inWhen(context.clone()) {
                Error::addSourceMessage(Error::CONNECT_IN_WHEN.clone(), list![(Dump::printComponentRefStr(var_field!((*scodeEq).crefLeft, SCode::Equation::EQ_CONNECT).clone())?).clone(), (Dump::printComponentRefStr(var_field!((*scodeEq).crefRight, SCode::Equation::EQ_CONNECT).clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            if !(ConnectBreakTree::isConnectBroken(var_field!((*scodeEq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), var_field!((*scodeEq).crefRight, SCode::Equation::EQ_CONNECT).clone(), scope.clone(), connectBreaks.clone())?) {
                next_context = InstContext::set(context.clone(), InstContext::CONNECT.clone());
                lhs_cr = instConnectorCref(var_field!((*scodeEq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), scope.clone(), next_context.clone(), info.clone())?;
                rhs_cr = instConnectorCref(var_field!((*scodeEq).crefRight, SCode::Equation::EQ_CONNECT).clone(), scope.clone(), next_context.clone(), info.clone())?;
                if !(InstNode::isEmpty(ComponentRef::node(lhs_cr.clone())?) || InstNode::isEmpty(ComponentRef::node(rhs_cr.clone())?)) {
                    exp1 = Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: lhs_cr.clone() });
                    exp2 = Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: rhs_cr.clone() });
                    equations = cons(Arc::new(Equation::NFEquation::CONNECT { lhs: exp1.clone(), rhs: exp2.clone(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_CONNECT).clone(), info.clone()) }), equations.clone());
                }
            }
            equations.clone()
        },
        Deref @ SCode::Equation::EQ_FOR { info, .. } => {
            let mut oexp: Option<Arc<Expression::NFExpression>> = None;
            let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut for_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut next_context: i32 = 0;
            oexp = instExpOpt(var_field!((*scodeEq).range, SCode::Equation::EQ_FOR).clone(), scope.clone(), context.clone(), info.clone())?;
            checkIteratorShadowing((var_field!((*scodeEq).index, SCode::Equation::EQ_FOR).clone()).clone(), scope.clone(), var_field!((*scodeEq).info, SCode::Equation::EQ_FOR).clone())?;
            (for_scope, iter) = addIteratorToScope((var_field!((*scodeEq).index, SCode::Equation::EQ_FOR).clone()).clone(), scope.clone(), var_field!((*scodeEq).info, SCode::Equation::EQ_FOR).clone(), Arc::new(crate::NFType::UNKNOWN))?;
            next_context = InstContext::set(context.clone(), InstContext::FOR.clone());
            eql = instEquations(var_field!((*scodeEq).eEquationLst, SCode::Equation::EQ_FOR).clone(), for_scope.clone(), connectBreaks.clone(), next_context.clone())?;
            cons(Arc::new(Equation::NFEquation::FOR { iterator: iter.clone(), range: oexp.clone(), body: eql.clone(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_FOR).clone(), info.clone()) }), equations.clone())
        },
        Deref @ SCode::Equation::EQ_IF { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut c in (var_field!((*scodeEq).condition, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = instExp(c.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            next_context = InstContext::set(context.clone(), InstContext::IF.clone());
            branches = metamodelica::nil();
            for mut branch in &*var_field!((*scodeEq).thenBranch, SCode::Equation::EQ_IF).clone() {
                let mut branch = branch.clone();
                eql = instEquations(branch.clone(), scope.clone(), connectBreaks.clone(), next_context.clone())?;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expl.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                exp1 = __pa0.clone();
                expl = __pa1.clone();
                branches = cons(Equation::makeBranch(exp1.clone(), eql.clone(), Prefixes::Variability::CONTINUOUS.clone()), branches.clone());
            }
            if !(var_field!((*scodeEq).elseBranch, SCode::Equation::EQ_IF).clone().is_empty()) {
                eql = instEquations(var_field!((*scodeEq).elseBranch, SCode::Equation::EQ_IF).clone(), scope.clone(), connectBreaks.clone(), next_context.clone())?;
                branches = cons(Equation::makeBranch(Arc::new(Expression::NFExpression::BOOLEAN { value: true }), eql.clone(), Prefixes::Variability::CONTINUOUS.clone()), branches.clone());
            }
            cons(Arc::new(Equation::NFEquation::IF { branches: branches.clone().reverse(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_IF).clone(), info.clone()) }), equations.clone())
        },
        Deref @ SCode::Equation::EQ_WHEN { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            if InstContext::inWhen(context.clone()) {
                Error::addSourceMessageAndFail(Error::NESTED_WHEN.clone(), metamodelica::nil(), info.clone())?;
            } else if InstContext::inInitial(context.clone()) {
                Error::addSourceMessageAndFail(Error::INITIAL_WHEN.clone(), metamodelica::nil(), info.clone())?;
            }
            next_context = InstContext::set(context.clone(), InstContext::WHEN.clone());
            exp1 = instExp(var_field!((*scodeEq).condition, SCode::Equation::EQ_WHEN).clone(), scope.clone(), context.clone(), info.clone())?;
            eql = instEquations(var_field!((*scodeEq).eEquationLst, SCode::Equation::EQ_WHEN).clone(), scope.clone(), connectBreaks.clone(), next_context.clone())?;
            branches = list![Equation::makeBranch(exp1.clone(), eql.clone(), Prefixes::Variability::CONTINUOUS.clone())];
            for mut branch in &*var_field!((*scodeEq).elseBranches, SCode::Equation::EQ_WHEN).clone() {
                let mut branch = branch.clone();
                exp1 = instExp(Util::tuple21(branch.clone()), scope.clone(), context.clone(), info.clone())?;
                eql = instEquations(Util::tuple22(branch.clone()), scope.clone(), connectBreaks.clone(), next_context.clone())?;
                branches = cons(Equation::makeBranch(exp1.clone(), eql.clone(), Prefixes::Variability::CONTINUOUS.clone()), branches.clone());
            }
            cons(Arc::new(Equation::NFEquation::WHEN { branches: branches.clone().reverse(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_WHEN).clone(), info.clone()) }), equations.clone())
        },
        Deref @ SCode::Equation::EQ_ASSERT { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeEq).condition, SCode::Equation::EQ_ASSERT).clone(), scope.clone(), context.clone(), info.clone())?;
            exp2 = instExp(var_field!((*scodeEq).message, SCode::Equation::EQ_ASSERT).clone(), scope.clone(), context.clone(), info.clone())?;
            exp3 = instExp(var_field!((*scodeEq).level, SCode::Equation::EQ_ASSERT).clone(), scope.clone(), context.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::ASSERT { condition: exp1.clone(), message: exp2.clone(), level: exp3.clone(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_ASSERT).clone(), info.clone()) }), equations.clone())
        },
        Deref @ SCode::Equation::EQ_TERMINATE { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeEq).message, SCode::Equation::EQ_TERMINATE).clone(), scope.clone(), context.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::TERMINATE { message: exp1.clone(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_TERMINATE).clone(), info.clone()) }), equations.clone())
        },
        Deref @ SCode::Equation::EQ_REINIT { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if !(InstContext::inWhen(context.clone())) {
                Error::addSourceMessage(Error::REINIT_NOT_IN_WHEN.clone(), metamodelica::nil(), info.clone())?;
                bail!("fail");
            }
            exp1 = instExp(var_field!((*scodeEq).cref, SCode::Equation::EQ_REINIT).clone(), scope.clone(), context.clone(), info.clone())?;
            exp2 = instExp(var_field!((*scodeEq).expReinit, SCode::Equation::EQ_REINIT).clone(), scope.clone(), context.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::REINIT { cref: exp1.clone(), reinitExp: exp2.clone(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_REINIT).clone(), info.clone()) }), equations.clone())
        },
        Deref @ SCode::Equation::EQ_NORETCALL { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeEq).exp, SCode::Equation::EQ_NORETCALL).clone(), scope.clone(), context.clone(), info.clone())?;
            cons(Arc::new(Equation::NFEquation::NORETCALL { exp: exp1.clone(), scope: scope.clone(), source: makeSource(var_field!((*scodeEq).comment, SCode::Equation::EQ_NORETCALL).clone(), info.clone()) }), equations.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instEquation")); __mm_s.push_str(&*literal!(" got unknown equation")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub fn instConnectorCref(mut absynCref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut prefix: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut found_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    (cref, found_scope) = Lookup::lookupConnector(absynCref.clone(), scope.clone(), context.clone(), info.clone())?;
    cref = instCrefSubscripts(cref.clone(), scope.clone(), context.clone(), info.clone())?;
    cref = ComponentRef::appendScope(found_scope.clone(), cref.clone(), false)?;
    Ok(cref)
}

pub fn makeSource(mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Arc<DAE::ElementSource> {
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    source = Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: metamodelica::nil(), instance: Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE), connectEquationOptLst: metamodelica::nil(), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: list![comment.clone()] });
    source
}

pub fn instAlgorithmSections(mut algorithmSections: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>> {
    let mut algs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    if InstContext::inInstanceAPI(context.clone()) {
        algs = metamodelica::nil();
    } else {
        algs = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut alg in (algorithmSections.clone()).into_iter().cloned() {
            let __x = instAlgorithmSection(alg.clone(), scope.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    Ok(algs)
}

pub fn instAlgorithmSection(mut algorithmSection: Arc<SCode::AlgorithmSection>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm>;
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    statements = instStatements(algorithmSection.statements.clone(), scope.clone(), context.clone())?;
    alg = Arc::new(Algorithm::NFAlgorithm { statements: statements.clone(), inputs: metamodelica::nil(), outputs: metamodelica::nil(), stmtDiffInfo: None, scope: scope.clone(), source: DAE::emptyElementSource().clone() });
    Ok(alg)
}

pub fn instStatements(mut scodeStmtl: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    statements = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut stmt in (scodeStmtl.clone()).into_iter().cloned() {
            let __x = instStatement(stmt.clone(), scope.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(statements)
}

pub fn instStatement(mut scodeStmt: Arc<SCode::Statement>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<Statement::NFStatement>> {
    let mut statement: Arc<Statement::NFStatement>;
    statement = (::match_deref::match_deref! { match &(scodeStmt.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeStmt).assignComponent, SCode::Statement::ALG_ASSIGN).clone(), scope.clone(), context.clone(), info.clone())?;
            checkAssignmentRestriction(exp1.clone(), info.clone())?;
            exp2 = instExp(var_field!((*scodeStmt).value, SCode::Statement::ALG_ASSIGN).clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: exp1.clone(), rhs: exp2.clone(), ty: Arc::new(crate::NFType::UNKNOWN), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_ASSIGN).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_FOR { info, .. } => {
            let mut oexp: Option<Arc<Expression::NFExpression>> = None;
            let mut stmtl: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut for_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut next_context: i32 = 0;
            oexp = instExpOpt(var_field!((*scodeStmt).range, SCode::Statement::ALG_FOR).clone(), scope.clone(), context.clone(), info.clone())?;
            (for_scope, iter) = addIteratorToScope((var_field!((*scodeStmt).index, SCode::Statement::ALG_FOR).clone()).clone(), scope.clone(), info.clone(), Arc::new(crate::NFType::UNKNOWN))?;
            next_context = InstContext::set(context.clone(), InstContext::FOR.clone());
            stmtl = instStatements(var_field!((*scodeStmt).forBody, SCode::Statement::ALG_FOR).clone(), for_scope.clone(), next_context.clone())?;
            Arc::new(Statement::NFStatement::FOR { iterator: iter.clone(), range: oexp.clone(), body: stmtl.clone(), forType: Arc::new(crate::NFStatement::ForType::NORMAL), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_FOR).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_PARFOR { info, .. } => {
            let mut oexp: Option<Arc<Expression::NFExpression>> = None;
            let mut stmtl: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut for_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut next_context: i32 = 0;
            oexp = instExpOpt(var_field!((*scodeStmt).range, SCode::Statement::ALG_PARFOR).clone(), scope.clone(), context.clone(), info.clone())?;
            (for_scope, iter) = addIteratorToScope((var_field!((*scodeStmt).index, SCode::Statement::ALG_PARFOR).clone()).clone(), scope.clone(), info.clone(), Arc::new(crate::NFType::UNKNOWN))?;
            next_context = InstContext::set(context.clone(), InstContext::FOR.clone());
            stmtl = instStatements(var_field!((*scodeStmt).parforBody, SCode::Statement::ALG_PARFOR).clone(), for_scope.clone(), next_context.clone())?;
            Arc::new(Statement::NFStatement::FOR { iterator: iter.clone(), range: oexp.clone(), body: stmtl.clone(), forType: Arc::new(Statement::ForType::PARALLEL { vars: metamodelica::nil() }), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_PARFOR).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_IF { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut stmtl: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            branches = metamodelica::nil();
            next_context = InstContext::set(context.clone(), InstContext::FOR.clone());
            for mut branch in &*cons((var_field!((*scodeStmt).boolExpr, SCode::Statement::ALG_IF).clone(), var_field!((*scodeStmt).trueBranch, SCode::Statement::ALG_IF).clone()), var_field!((*scodeStmt).elseIfBranch, SCode::Statement::ALG_IF).clone()) {
                let mut branch = branch.clone();
                exp1 = instExp(Util::tuple21(branch.clone()), scope.clone(), context.clone(), info.clone())?;
                stmtl = instStatements(Util::tuple22(branch.clone()), scope.clone(), next_context.clone())?;
                branches = cons((exp1.clone(), stmtl.clone()), branches.clone());
            }
            if !(var_field!((*scodeStmt).elseBranch, SCode::Statement::ALG_IF).clone().is_empty()) {
                stmtl = instStatements(var_field!((*scodeStmt).elseBranch, SCode::Statement::ALG_IF).clone(), scope.clone(), next_context.clone())?;
                branches = cons((Arc::new(Expression::NFExpression::BOOLEAN { value: true }), stmtl.clone()), branches.clone());
            }
            Arc::new(Statement::NFStatement::IF { branches: branches.clone().reverse(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_IF).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_WHEN_A { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut stmtl: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            if !(InstContext::inValidWhenScope(context.clone())) {
                if InstContext::inWhen(context.clone()) {
                    Error::addSourceMessageAndFail(Error::NESTED_WHEN.clone(), metamodelica::nil(), info.clone())?;
                } else if InstContext::inInitial(context.clone()) {
                    Error::addSourceMessageAndFail(Error::INITIAL_WHEN.clone(), metamodelica::nil(), info.clone())?;
                } else {
                    Error::addSourceMessageAndFail(Error::INVALID_WHEN_STATEMENT_CONTEXT.clone(), metamodelica::nil(), info.clone())?;
                }
            }
            branches = metamodelica::nil();
            for mut branch in &*var_field!((*scodeStmt).branches, SCode::Statement::ALG_WHEN_A).clone() {
                let mut branch = branch.clone();
                exp1 = instExp(Util::tuple21(branch.clone()), scope.clone(), context.clone(), info.clone())?;
                next_context = InstContext::set(context.clone(), InstContext::WHEN.clone());
                stmtl = instStatements(Util::tuple22(branch.clone()), scope.clone(), next_context.clone())?;
                branches = cons((exp1.clone(), stmtl.clone()), branches.clone());
            }
            Arc::new(Statement::NFStatement::WHEN { branches: branches.clone().reverse(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_WHEN_A).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_ASSERT { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeStmt).condition, SCode::Statement::ALG_ASSERT).clone(), scope.clone(), context.clone(), info.clone())?;
            exp2 = instExp(var_field!((*scodeStmt).message, SCode::Statement::ALG_ASSERT).clone(), scope.clone(), context.clone(), info.clone())?;
            exp3 = instExp(var_field!((*scodeStmt).level, SCode::Statement::ALG_ASSERT).clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::ASSERT { condition: exp1.clone(), message: exp2.clone(), level: exp3.clone(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_ASSERT).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_TERMINATE { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeStmt).message, SCode::Statement::ALG_TERMINATE).clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::TERMINATE { message: exp1.clone(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_TERMINATE).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_REINIT { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if !(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("reinitInAlgorithms")).clone())?) {
                Error::addSourceMessage(Error::REINIT_IN_ALGORITHM.clone(), metamodelica::nil(), info.clone())?;
                bail!("fail");
            }
            if !(InstContext::inWhen(context.clone())) {
                Error::addSourceMessage(Error::REINIT_NOT_IN_WHEN.clone(), metamodelica::nil(), info.clone())?;
                bail!("fail");
            }
            exp1 = instExp(var_field!((*scodeStmt).cref, SCode::Statement::ALG_REINIT).clone(), scope.clone(), context.clone(), info.clone())?;
            exp2 = instExp(var_field!((*scodeStmt).newValue, SCode::Statement::ALG_REINIT).clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::REINIT { cref: exp1.clone(), reinitExp: exp2.clone(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_REINIT).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_NORETCALL { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            exp1 = instExp(var_field!((*scodeStmt).exp, SCode::Statement::ALG_NORETCALL).clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Statement::NFStatement::NORETCALL { exp: exp1.clone(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_NORETCALL).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_WHILE { info, .. } => {
            let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut stmtl: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            let mut next_context: i32 = 0;
            exp1 = instExp(var_field!((*scodeStmt).boolExpr, SCode::Statement::ALG_WHILE).clone(), scope.clone(), context.clone(), info.clone())?;
            next_context = InstContext::set(context.clone(), InstContext::WHILE.clone());
            stmtl = instStatements(var_field!((*scodeStmt).whileBody, SCode::Statement::ALG_WHILE).clone(), scope.clone(), next_context.clone())?;
            Arc::new(Statement::NFStatement::WHILE { condition: exp1.clone(), body: stmtl.clone(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_WHILE).clone(), info.clone()) })
        },
        Deref @ SCode::Statement::ALG_RETURN { .. } => {
            if !(InstContext::inFunction(context.clone())) {
                Error::addSourceMessage(Error::RETURN_OUTSIDE_FUNCTION.clone(), metamodelica::nil(), var_field!((*scodeStmt).info, SCode::Statement::ALG_RETURN).clone())?;
                bail!("fail");
            }
            Arc::new(Statement::NFStatement::RETURN { source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_RETURN).clone(), var_field!((*scodeStmt).info, SCode::Statement::ALG_RETURN).clone()) })
        },
        Deref @ SCode::Statement::ALG_BREAK { .. } => {
            if !(InstContext::inLoop(context.clone())) {
                Error::addSourceMessage(Error::BREAK_OUTSIDE_LOOP.clone(), metamodelica::nil(), var_field!((*scodeStmt).info, SCode::Statement::ALG_BREAK).clone())?;
                bail!("fail");
            }
            Arc::new(Statement::NFStatement::BREAK { source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_BREAK).clone(), var_field!((*scodeStmt).info, SCode::Statement::ALG_BREAK).clone()) })
        },
        Deref @ SCode::Statement::ALG_FAILURE { .. } => {
            let mut stmtl: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
            stmtl = instStatements(var_field!((*scodeStmt).stmts, SCode::Statement::ALG_FAILURE).clone(), scope.clone(), context.clone())?;
            Arc::new(Statement::NFStatement::FAILURE { body: stmtl.clone(), source: makeSource(var_field!((*scodeStmt).comment, SCode::Statement::ALG_FAILURE).clone(), var_field!((*scodeStmt).info, SCode::Statement::ALG_FAILURE).clone()) })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instStatement")); __mm_s.push_str(&*literal!(" got unknown statement")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statement)
}

pub fn checkAssignmentRestriction(mut lhs: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<()> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    let () = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isIterator(var_field!((*lhs).cref, Expression::NFExpression::CREF).clone())) => {
            Error::addSourceMessage(Error::ASSIGN_ITERATOR_ERROR.clone(), list![(ComponentRef::toString(var_field!((*lhs).cref, Expression::NFExpression::CREF).clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*lhs).cref, Expression::NFExpression::CREF).clone())) => {
            node = ComponentRef::node(var_field!((*lhs).cref, Expression::NFExpression::CREF).clone())?;
            res = Class::restriction(InstNode::getClass(node.clone())?);
            let () = (::match_deref::match_deref! { match &(res.clone()) {
        Deref @ Restriction::CLOCK => (),
        Deref @ Restriction::CONNECTOR { .. } => (),
        Deref @ Restriction::ENUMERATION => (),
        Deref @ Restriction::RECORD { .. } => (),
        Deref @ Restriction::TYPE => (),
        _ => {
            Error::addSourceMessage(Error::INVALID_SPECIALIZATION_IN_ASSIGNMENT.clone(), list![(InstNode::name(node.clone())?).clone(), (Restriction::toString(res.clone())).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Expression::TUPLE { .. } => {
            for mut e in &*var_field!((*lhs).elements, Expression::NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                checkAssignmentRestriction(e.clone(), info.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn addIteratorToScope(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo, mut iter_type: Arc<Type::NFType>) -> Result<(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)> {
    let mut scope: Arc<InstNode::InstNode> = scope;
    let mut iterator: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut iter_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    scope = InstNode::openImplicitScope(scope.clone());
    iter_comp = Arc::new(Component::NFComponent::ITERATOR { ty: iter_type.clone(), variability: Variability::CONTINUOUS.clone(), info: info.clone() });
    iterator = InstNode::fromComponent((name.clone()).clone(), iter_comp.clone(), scope.clone());
    scope = InstNode::addIterator(iterator.clone(), scope.clone())?;
    Ok((scope, iterator))
}

pub fn checkIteratorShadowing(mut name: ArcStr, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(scope.clone()) {
        Deref @ InstNode::IMPLICIT_SCOPE { .. } => {
            for mut iter in &*var_field!((*scope).locals, InstNode::InstNode::IMPLICIT_SCOPE).clone() {
                let mut iter = iter.clone();
                if InstNode::name(iter.clone())? == name.clone() {
                    Error::addMultiSourceMessage(Error::SHADOWED_ITERATOR.clone(), list![(name.clone()).clone()], list![InstNode::info(iter.clone())?, info.clone()])?;
                    return Ok(());
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn insertGeneratedInners(mut node: Arc<InstNode::InstNode>, mut topScope: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut generated_inners: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<InstNode::InstNode>>>;
    let mut inner_comps: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>> = metamodelica::nil();
    let mut n: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut on: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut name: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut base_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut name_defined: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(InstNode::nodeType(topScope.clone())?) {
        Deref @ InstNodeType::TOP_SCOPE { generatedInners: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    generated_inners = __pa0.clone();
    if UnorderedMap::isEmpty(generated_inners.clone()) {
        return Ok(());
    }
    inner_comps = metamodelica::nil();
    let __range1 = UnorderedMap::valueArray(generated_inners.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut n in __range1 {
        name = (InstNode::name(n.clone())?).clone();
        checkTopLevelOuter((name.clone()).clone(), n.clone(), node.clone(), context.clone())?;
        if !(InstContext::inInstanceAPI(context.clone())) {
            Error::addSourceMessage(Error::MISSING_INNER_ADDED.clone(), list![(InstNode::typeName(n.clone())?).clone(), (name.clone()).clone()], InstNode::info(n.clone())?)?;
        }
        if InstNode::isComponent(n.clone()) {
            instComponent(n.clone(), Attributes::DEFAULT_ATTR().clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), true, 0, InstContext::CLASS.clone(), None, metamodelica::nil())?;
            if !(InstContext::inInstanceAPI(context.clone())) {
                if '__try2: {
                    let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(SCodeUtil::lookupElementAnnotationBinding(InstNode::definition(InstNode::classScope(n.clone()))?, (literal!("missingInnerMessage")).clone()), '__try2)) {
                        Some(Deref @ Absyn::Exp::STRING { value: __pa3 }) => __pa3.clone(),
                        _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    r#str = __pa3.clone();
                    unwrap_break_err!(Error::addSourceMessage(Error::MISSING_INNER_MESSAGE.clone(), list![(System::unescapedString((r#str.clone()).clone())).clone()], InstNode::info(n.clone())?), '__try2);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
            }
            inner_comps = cons(Mutable::create(n.clone()), inner_comps.clone());
        }
    }
    if !(inner_comps.clone().is_empty()) {
        base_node = Class::lastBaseClass(node.clone());
        cls = InstNode::getClass(base_node.clone())?;
        cls_tree = ClassTree::appendComponentsToInstTree(inner_comps.clone(), Class::classTree(cls.clone())?)?;
        InstNode::updateClass(Class::setClassTree(cls_tree.clone(), cls.clone())?, base_node.clone())?;
    }
    Ok(())
}

pub fn checkTopLevelOuter(mut name: ArcStr, mut outerNode: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut is_error: bool = false;
    if InstContext::inInstanceAPI(context.clone()) {
        return Ok(());
    }
    match '__try0: {
        (node, _) = unwrap_break_err!(Lookup::lookupSimpleName((name.clone()).clone(), scope.clone(), context.clone()), '__try0);
        if unwrap_break_err!(InstNode::isInner(node.clone()), '__try0) {
            is_error = !(InstContext::inRelaxed(context.clone()) || unwrap_break_err!(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("nonStdTopLevelOuter")).clone()), '__try0));
            if is_error.clone() {
                unwrap_break_err!(Error::addSourceMessageAsError(Error::TOP_LEVEL_OUTER.clone(), list![(name.clone()).clone()], InstNode::info(node.clone())?), '__try0);
            } else {
                unwrap_break_err!(Error::addSourceMessage(Error::TOP_LEVEL_OUTER.clone(), list![(name.clone()).clone()], InstNode::info(node.clone())?), '__try0);
            }
        } else {
            unwrap_break_err!(Error::addMultiSourceMessage(Error::MISSING_INNER_NAME_CONFLICT.clone(), list![(name.clone()).clone()], list![InstNode::info(node.clone())?, InstNode::info(outerNode.clone())?]), '__try0);
            is_error = true;
        }
        Ok::<_, anyhow::Error>((is_error.clone(),))
    } {
        Ok((__try0_o0,)) => {
            is_error = __try0_o0;
        }
        Err(_) => {
            is_error = false;
        }
    }
    if is_error.clone() {
        bail!("fail");
    }
    Ok(())
}

pub fn updateImplicitVariability(mut node: Arc<InstNode::InstNode>, mut parentEval: bool, mut context: i32) -> Result<()> {
    let mut cls: Arc<Class::NFClass> = InstNode::getClass(node.clone())?;
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { elements: cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            let __range0 = var_field!((**cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                updateImplicitVariabilityComp(c.clone(), parentEval.clone(), context.clone())?;
            }
            Sections::apply(var_field!((*cls).sections, Class::NFClass::INSTANCED_CLASS).clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| updateImplicitVariabilityEq(__pe_a0, __pe_b1.clone()) }), (std::sync::Arc::new(updateImplicitVariabilityAlg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<()> + 'static>), Arc::new({ let __pe_b1 = false; move |__pe_a0| updateImplicitVariabilityEq(__pe_a0, __pe_b1.clone()) }), (std::sync::Arc::new(updateImplicitVariabilityAlg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<()> + 'static>));
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { .. } => {
            let __range0 = var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut dim in __range0 {
                Structural::markDimension(dim.clone())?;
            }
            updateImplicitVariability(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), parentEval.clone(), context.clone())?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { elements: cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            let __range0 = var_field!((**cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                updateImplicitVariabilityComp(c.clone(), parentEval.clone(), context.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn updateImplicitVariabilityComp(mut component: Arc<InstNode::InstNode>, mut parentEval: bool, mut context: i32) -> Result<()> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    if InstNode::isEmpty(component.clone()) {
        return Ok(());
    }
    node = InstNode::resolveOuter(component.clone());
    c = InstNode::component(node.clone())?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { condition, binding, .. } => {
            let mut opt_eval: Option<bool> = None;
            let mut eval: bool = false;
            opt_eval = Component::getEvaluateAnnotation(c.clone())?;
            eval = Util::getOptionOrDefault(opt_eval.clone(), false);
            if isSome(opt_eval.clone()) && !(eval.clone()) && var_field!((*c).attributes, Component::NFComponent::COMPONENT).variability.clone() == Variability::PARAMETER.clone() {
                InstNode::updateComponent(Component::setVariability(Variability::NON_STRUCTURAL_PARAMETER.clone(), c.clone()), node.clone())?;
            } else {
                if Structural::isStructuralComponent(c.clone(), var_field!((*c).attributes, Component::NFComponent::COMPONENT).clone(), binding.clone(), node.clone(), eval.clone(), parentEval.clone(), context.clone())? {
                    Structural::markComponent(c.clone(), node.clone())?;
                }
            }
            for mut dim in &*Type::arrayDims(var_field!((*c).ty, Component::NFComponent::COMPONENT).clone()) {
                let mut dim = dim.clone();
                Structural::markDimension(dim.clone())?;
            }
            if Binding::isBound(binding.clone()) {
                Structural::markExpSize(Binding::getUntypedExp(binding.clone())?)?;
            }
            if Binding::isBound(condition.clone()) {
                Structural::markExp(Binding::getUntypedExp(condition.clone())?)?;
            }
            if !(InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone())) {
                updateImplicitVariability(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), eval.clone() || parentEval.clone(), context.clone())?;
            }
            ()
        },
        Deref @ Component::TYPE_ATTRIBUTE { .. } if (listMember((InstNode::name(component.clone())?).clone(), list![(literal!("fixed")).clone(), (literal!("stateSelect")).clone()])) => {
            let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
            binding = Modifier::binding(var_field!((*c).modifier, Component::NFComponent::TYPE_ATTRIBUTE).clone());
            if Binding::isBound(binding.clone()) {
                Structural::markExp(Binding::getUntypedExp(binding.clone())?)?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn updateImplicitVariabilityEql(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut inWhen: bool) -> Result<()> {
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        updateImplicitVariabilityEq(eq.clone(), inWhen.clone())?;
    }
    Ok(())
}

pub fn updateImplicitVariabilityEq(mut eq: Arc<Equation::NFEquation>, mut inWhen: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            if inWhen.clone() {
                markImplicitWhenExp(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone())?;
            }
            ()
        },
        Deref @ Equation::CONNECT { .. } => {
            Structural::markSubscriptsInExp(var_field!((*eq).lhs, Equation::NFEquation::CONNECT).clone())?;
            Structural::markSubscriptsInExp(var_field!((*eq).rhs, Equation::NFEquation::CONNECT).clone())?;
            ()
        },
        Deref @ Equation::FOR { .. } => {
            updateImplicitVariabilityEql(var_field!((*eq).body, Equation::NFEquation::FOR).clone(), inWhen.clone())?;
            ()
        },
        Deref @ Equation::IF { .. } => {
            for mut branch in &*var_field!((*eq).branches, Equation::NFEquation::IF).clone() {
                let mut branch = branch.clone();
                let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            updateImplicitVariabilityEql(var_field!((*branch).body, Equation::Branch::Branch::BRANCH).clone(), inWhen.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            }
            ()
        },
        Deref @ Equation::WHEN { .. } => {
            for mut branch in &*var_field!((*eq).branches, Equation::NFEquation::WHEN).clone() {
                let mut branch = branch.clone();
                let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            updateImplicitVariabilityEql(var_field!((*branch).body, Equation::Branch::Branch::BRANCH).clone(), true)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn updateImplicitVariabilityAlg(mut alg: Arc<Algorithm::NFAlgorithm>) -> Result<()> {
    updateImplicitVariabilityStmts(alg.statements.clone(), false)?;
    Ok(())
}

pub fn updateImplicitVariabilityStmts(mut stmtl: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut inWhen: bool) -> Result<()> {
    for mut s in &*stmtl.clone() {
        let mut s = s.clone();
        updateImplicitVariabilityStmt(s.clone(), inWhen.clone())?;
    }
    Ok(())
}

pub fn updateImplicitVariabilityStmt(mut stmt: Arc<Statement::NFStatement>, mut inWhen: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            if inWhen.clone() {
                markImplicitWhenExp(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone())?;
            }
            ()
        },
        Deref @ Statement::FOR { .. } => {
            if inWhen.clone() {
                updateImplicitVariabilityStmts(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), true)?;
            }
            ()
        },
        Deref @ Statement::IF { .. } => {
            if inWhen.clone() {
                for mut branch in &*var_field!((*stmt).branches, Statement::NFStatement::IF).clone() {
                    let mut branch = branch.clone();
                    updateImplicitVariabilityStmts(Util::tuple22(branch.clone()), true)?;
                }
            }
            ()
        },
        Deref @ Statement::WHEN { .. } => {
            for mut branch in &*var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone() {
                let mut branch = branch.clone();
                updateImplicitVariabilityStmts(Util::tuple22(branch.clone()), true)?;
            }
            ()
        },
        Deref @ Statement::WHILE { .. } => {
            if inWhen.clone() {
                updateImplicitVariabilityStmts(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone(), true)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn markImplicitWhenExp(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    Expression::apply(exp.clone(), (std::sync::Arc::new(markImplicitWhenExp_traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
    Ok(())
}

pub fn markImplicitWhenExp_traverser(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { node, .. }, .. } => {
            let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            if InstNode::isComponent(node.clone()) {
                comp = InstNode::component(node.clone())?;
                if Component::variability(comp.clone()) == Variability::CONTINUOUS.clone() {
                    comp = Component::setVariability(Variability::IMPLICITLY_DISCRETE.clone(), comp.clone());
                    InstNode::updateComponent(comp.clone(), node.clone())?;
                }
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkPartialClass(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<()> {
    if InstNode::isPartial(node.clone()) && !(InstContext::inRelaxed(context.clone())) {
        Error::addSourceMessage(Error::INST_PARTIAL_CLASS.clone(), list![(InstNode::name(node.clone())?).clone()], InstNode::info(node.clone())?)?;
        bail!("fail");
    }
    Ok(())
}

pub fn checkInstanceRestriction(mut node: Arc<InstNode::InstNode>, mut path: Arc<Path>, mut context: i32) -> Result<()> {
    let mut elem: Arc<SCode::Element>;
    if InstContext::inRelaxed(context.clone()) {
        return Ok(());
    }
    elem = InstNode::definition(node.clone())?;
    if SCodeUtil::isFunction(elem.clone()) || SCodeUtil::isPackage(elem.clone()) {
        Error::addSourceMessage(Error::INST_INVALID_RESTRICTION.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), (SCodeDump::restrString(SCodeUtil::getClassRestriction(elem.clone())?)?).clone()], InstNode::info(node.clone())?)?;
        bail!("fail");
    }
    Ok(())
}

