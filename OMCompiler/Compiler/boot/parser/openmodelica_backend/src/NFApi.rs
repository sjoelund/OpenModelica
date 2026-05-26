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

use crate::CevalScriptBackend;
use crate::InteractiveUtil;
use crate::SimCode;
use crate::SimCodeMain;
use crate::SymbolTable;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend::Parser;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFCeval as Ceval;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFClassTree::ClassTree;
use openmodelica_nf_frontend::NFComponent as Component;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConnectBreakTree;
use openmodelica_nf_frontend::NFConnection as Connection;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFEquation as Equation;
use openmodelica_nf_frontend::NFEvalConstants as EvalConstants;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFlatten as Flatten;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFImport as Import;
use openmodelica_nf_frontend::NFInst as Inst;
use openmodelica_nf_frontend::NFInst::InstSettings;
use openmodelica_nf_frontend::NFInstContext as InstContext;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFInstNode::InstNodeType;
use openmodelica_nf_frontend::NFInstUtil as InstUtil;
use openmodelica_nf_frontend::NFLookup as Lookup;
use openmodelica_nf_frontend::NFModifier::Modifier;
use openmodelica_nf_frontend::NFModifier::ModifierScope;
use openmodelica_nf_frontend::NFPackage as Package;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFPrefixes::Purity;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFRestriction as Restriction;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFSections as Sections;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSimplifyModel as SimplifyModel;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTyping as Typing;
use openmodelica_nf_frontend::NFUnitCheck as UnitCheck;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_nf_frontend::NFVerifyModel as VerifyModel;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::JSON;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub const ANNOTATION_CONTEXT: i32 = intBitOr(InstContext::RELAXED, InstContext::ANNOTATION);

pub const INST_API_ANNOTATION_CONTEXT: i32 = intBitOr(ANNOTATION_CONTEXT, InstContext::INSTANCE_API);

pub const FAST_CONTEXT: i32 = intBitOr(InstContext::RELAXED, InstContext::FAST_LOOKUP);

pub fn evaluateAnnotation(mut absynProgram: Absyn::Program, mut classPath: Arc<Path>, mut inAnnotation: Arc<Absyn::Annotation>) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    let mut b: bool = false;
    let mut s: bool = false;
    b = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    s = FlagsUtil::set(Flags::NF_SCALARIZE.clone(), true)?;
    match '__try0: {
        outString = unwrap_break_err!(evaluateAnnotation_dispatch(absynProgram.clone(), classPath.clone(), inAnnotation.clone(), false), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::SCODE_INST.clone(), b.clone()), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s.clone()), '__try0);
        Ok::<_, anyhow::Error>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            FlagsUtil::set(Flags::SCODE_INST.clone(), b.clone())?;
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s.clone())?;
            bail!("fail");
        }
    }
    Ok(outString)
}

pub fn evaluateAnnotations(mut absynProgram: Absyn::Program, mut classPath: Arc<Path>, mut inElements: Arc<metamodelica::List<Arc<Absyn::Element>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut b: bool = false;
    let mut s: bool = false;
    b = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    s = FlagsUtil::set(Flags::NF_SCALARIZE.clone(), true)?;
    match '__try0: {
        outStringLst = unwrap_break_err!(evaluateAnnotations_dispatch(absynProgram.clone(), classPath.clone(), inElements.clone()), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::SCODE_INST.clone(), b.clone()), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s.clone()), '__try0);
        Ok::<_, anyhow::Error>((outStringLst.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outStringLst = __try0_o0;
        }
        Err(_) => {
            FlagsUtil::set(Flags::SCODE_INST.clone(), b.clone())?;
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s.clone())?;
            bail!("fail");
        }
    }
    Ok(outStringLst)
}

pub fn mkFullyQual(mut absynProgram: Absyn::Program, mut classPath: Arc<Path>, mut pathToQualify: Arc<Path>, mut failOnError: bool) -> Result<Arc<Path>> {
    let mut qualPath: Arc<Path> = pathToQualify.clone();
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut expanded_cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut id1: ArcStr = arcstr::literal!("");
    let mut id2: ArcStr = arcstr::literal!("");
    let mut b: bool = false;
    let mut s: bool = false;
    let mut context: i32 = 0;
    let _ = (::match_deref::match_deref! { match &((classPath.clone(), pathToQualify.clone())) {
        (Deref @ Absyn::Path::QUALIFIED { name: id1, path: _ }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: _ }) if (id1.clone() == id2.clone()) => {
            return Ok(qualPath);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    s = FlagsUtil::set(Flags::NF_SCALARIZE.clone(), true)?;
    match '__try0: {
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::setCheckpoint((literal!("NFApi.mkFullyQual")).clone());
        }
        (program, name, expanded_cls) = unwrap_break_err!(frontEndLookup(absynProgram.clone(), classPath.clone()), '__try0);
        context = InstContext::set(InstContext::RELAXED.clone(), InstContext::FAST_LOOKUP.clone());
        if InstNode::isDerivedClass(expanded_cls.clone()) {
            cls = unwrap_break_err!(Lookup::lookupClassName(pathToQualify.clone(), InstNode::classParent(expanded_cls.clone())?, context.clone(), Absyn::dummyInfo.clone(), false), '__try0);
        } else {
            cls = unwrap_break_err!(Lookup::lookupClassName(pathToQualify.clone(), expanded_cls.clone(), context.clone(), Absyn::dummyInfo.clone(), false), '__try0);
        }
        qualPath = InstNode::fullPath(cls.clone(), false);
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::rollBack((literal!("NFApi.mkFullyQual")).clone());
        }
        unwrap_break_err!(FlagsUtil::set(Flags::SCODE_INST.clone(), b.clone()), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s.clone()), '__try0);
        Ok::<_, anyhow::Error>((qualPath.clone(),))
    } {
        Ok((__try0_o0,)) => {
            qualPath = __try0_o0;
        }
        Err(_) => {
            if !(Flags::isSet(Flags::NF_API_NOISE.clone())?) {
                ErrorExt::rollBack((literal!("NFApi.mkFullyQual")).clone());
            }
            FlagsUtil::set(Flags::SCODE_INST.clone(), b.clone())?;
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s.clone())?;
            if failOnError.clone() {
                bail!("fail");
            } else {
                qualPath = pathToQualify.clone();
            }
        }
    }
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFApi.mkFullyQual(")); __mm_s.push_str(&*AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*AbsynUtil::pathString(pathToQualify.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(") -> ")); __mm_s.push_str(&*AbsynUtil::pathString(qualPath.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(qualPath)
}

fn frontEndFront(mut absynProgram: Absyn::Program, mut classPath: Arc<Path>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, ArcStr, Arc<InstNode::InstNode>)> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut inst_cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cache: Arc<metamodelica::List<((Absyn::Program, Arc<Path>), (Arc<metamodelica::List<Arc<SCode::Element>>>, ArcStr, Arc<InstNode::InstNode>))>> = metamodelica::nil();
    cache = openmodelica_frontend::Globals::instNFInstCacheIndex.with(|__root| __root.borrow().clone());
    if !(cache.clone().is_empty()) {
        for mut i in &*cache.clone() {
            let mut i = i.clone();
            if referenceEq(&absynProgram.clone(),&Util::tuple21(Util::tuple21(i.clone()))) {
                if AbsynUtil::pathEqual(classPath.clone(), Util::tuple22(Util::tuple21(i.clone()))) {
                    (program, name, inst_cls) = Util::tuple22(i.clone());
                    return Ok((program, name, inst_cls));
                }
                cache = metamodelica::nil();
                openmodelica_frontend::Globals::instNFInstCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
                break;
            } else {
                if AbsynUtil::pathEqual(classPath.clone(), Util::tuple22(Util::tuple21(i.clone()))) {
                    cache = metamodelica::nil();
                    openmodelica_frontend::Globals::instNFInstCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
                    break;
                }
            }
        }
    }
    (program, name, inst_cls) = frontEndFront_dispatch(absynProgram.clone(), classPath.clone())?;
    if (cache.clone().len() as i32) > 100 {
        cache = List::firstN(cache.clone(), 10)?;
    }
    cache = cons(((absynProgram.clone(), classPath.clone()), (program.clone(), name.clone(), inst_cls.clone())), cache.clone());
    openmodelica_frontend::Globals::instNFInstCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
    Ok((program, name, inst_cls))
}

fn mkTop(mut absynProgram: Absyn::Program, mut name: ArcStr) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<InstNode::InstNode>)> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scode_builtin: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut graphicProgramSCode: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut placementProgram: Absyn::Program;
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cache: Arc<metamodelica::List<(Absyn::Program, (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<InstNode::InstNode>))>> = metamodelica::nil();
    let mut update: bool = true;
    cache = openmodelica_frontend::Globals::instNFNodeCacheIndex.with(|__root| __root.borrow().clone());
    if !(cache.clone().is_empty()) {
        if referenceEq(&absynProgram.clone(),&Util::tuple21(listHead(cache.clone())?)) {
            (program, top) = Util::tuple22(listHead(cache.clone())?);
            InstNode::clearGeneratedInners(top.clone())?;
            update = false;
        } else {
            update = true;
            cache = metamodelica::nil();
            openmodelica_frontend::Globals::instNFNodeCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
        }
    }
    if update.clone() {
        (_, scode_builtin) = FBuiltin::getInitialFunctions()?;
        program = AbsynToSCode::translateAbsyn2SCode(absynProgram.clone())?;
        program = listAppend(scode_builtin.clone(), program.clone());
        placementProgram = InteractiveUtil::modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?;
        graphicProgramSCode = AbsynToSCode::translateAbsyn2SCode(placementProgram.clone())?;
        Inst::resetGlobalFlags()?;
        top = Inst::makeTopNode(program.clone(), graphicProgramSCode.clone())?;
        if Flags::isSet(Flags::EXEC_STAT.clone())? {
            execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFApi.mkTop(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
        }
        cache = list![(absynProgram.clone(), (program.clone(), top.clone()))];
        openmodelica_frontend::Globals::instNFNodeCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
    }
    Ok((program, top))
}

fn frontEndFront_dispatch(mut absynProgram: Absyn::Program, mut classPath: Arc<Path>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, ArcStr, Arc<InstNode::InstNode>)> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut inst_cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    name = (AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false)?).clone();
    (program, top) = mkTop(absynProgram.clone(), (name.clone()).clone())?;
    cls = Lookup::lookupClassName(classPath.clone(), top.clone(), InstContext::RELAXED.clone(), Absyn::dummyInfo.clone(), false)?;
    cls = InstNode::makeRootClass(cls.clone(), Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), None);
    inst_cls = Inst::instantiate(cls.clone(), Arc::new(openmodelica_nf_frontend::NFModifier::Modifier::NOMOD), Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), InstContext::RELAXED.clone(), false)?;
    Inst::insertGeneratedInners(inst_cls.clone(), top.clone(), InstContext::RELAXED.clone())?;
    Inst::instExpressions(inst_cls.clone(), inst_cls.clone(), Arc::new(openmodelica_nf_frontend::NFSections::EMPTY), NFConnectBreakTree::new(), InstContext::RELAXED.clone(), Inst::DEFAULT_SETTINGS.clone())?;
    Inst::updateImplicitVariability(inst_cls.clone(), Flags::isSet(Flags::EVAL_PARAM.clone())?, InstContext::RELAXED.clone())?;
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFApi.frontEndFront_dispatch(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    }
    Inst::clearCaches()?;
    Ok((program, name, inst_cls))
}

fn frontEndLookup(mut absynProgram: Absyn::Program, mut classPath: Arc<Path>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, ArcStr, Arc<InstNode::InstNode>)> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut expanded_cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cache: Arc<metamodelica::List<((Absyn::Program, Arc<Path>), (Arc<metamodelica::List<Arc<SCode::Element>>>, ArcStr, Arc<InstNode::InstNode>))>> = metamodelica::nil();
    cache = openmodelica_frontend::Globals::instNFLookupCacheIndex.with(|__root| __root.borrow().clone());
    if !(cache.clone().is_empty()) {
        for mut i in &*cache.clone() {
            let mut i = i.clone();
            if referenceEq(&absynProgram.clone(),&Util::tuple21(Util::tuple21(i.clone()))) {
                if AbsynUtil::pathEqual(classPath.clone(), Util::tuple22(Util::tuple21(i.clone()))) {
                    (program, name, expanded_cls) = Util::tuple22(i.clone());
                    return Ok((program, name, expanded_cls));
                }
                cache = metamodelica::nil();
                openmodelica_frontend::Globals::instNFLookupCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
                break;
            } else {
                if AbsynUtil::pathEqual(classPath.clone(), Util::tuple22(Util::tuple21(i.clone()))) {
                    cache = metamodelica::nil();
                    openmodelica_frontend::Globals::instNFLookupCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
                    break;
                }
            }
        }
    }
    (program, name, expanded_cls) = frontEndLookup_dispatch(absynProgram.clone(), classPath.clone())?;
    if (cache.clone().len() as i32) > 100 {
        cache = List::firstN(cache.clone(), 10)?;
    }
    cache = cons(((absynProgram.clone(), classPath.clone()), (program.clone(), name.clone(), expanded_cls.clone())), cache.clone());
    openmodelica_frontend::Globals::instNFLookupCacheIndex.with(|__root| *__root.borrow_mut() = cache.clone());
    Ok((program, name, expanded_cls))
}

fn frontEndLookup_dispatch(mut absynProgram: Absyn::Program, mut classPath: Arc<Path>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, ArcStr, Arc<InstNode::InstNode>)> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut expanded_cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scode_builtin: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut graphicProgramSCode: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut placementProgram: Absyn::Program;
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cache: Arc<metamodelica::List<(Absyn::Program, (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<InstNode::InstNode>))>> = metamodelica::nil();
    let mut update: bool = true;
    name = (AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false)?).clone();
    (program, top) = mkTop(absynProgram.clone(), (name.clone()).clone())?;
    if AbsynUtil::pathEqual(classPath.clone(), Arc::new(Path::IDENT { name: (literal!("AllLoadedClasses")).clone() })) {
        expanded_cls = top.clone();
    } else {
        cls = Inst::lookupRootClass(classPath.clone(), top.clone(), FAST_CONTEXT.clone())?;
        expanded_cls = Inst::expand(cls.clone(), FAST_CONTEXT.clone())?;
    }
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFApi.frontEndLookup_dispatch(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    }
    Inst::clearCaches()?;
    Ok((program, name, expanded_cls))
}

pub fn getInheritedClasses(mut classPath: Arc<Path>, mut program: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Path>>>> {
    let mut extendsPaths: Arc<metamodelica::List<Arc<Path>>> = metamodelica::nil();
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut start_idx: i32 = 0;
    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
        extendsPaths = metamodelica::nil();
        return Ok(extendsPaths);
    }
    (_, _, cls_node) = frontEndLookup(program.clone(), classPath.clone())?;
    if !(InstNode::isClass(cls_node.clone())) {
        extendsPaths = metamodelica::nil();
        return Ok(extendsPaths);
    }
    cls = InstNode::getClass(cls_node.clone())?;
    extendsPaths = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_DERIVED { .. } => list![InstNode::fullPath(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), true)],
        _ => {
            exts = ClassTree::getExtends(Class::classTree(cls.clone())?);
            start_idx = if (SCodeUtil::isClassExtends(InstNode::definition(cls_node.clone())?)) {2} else {1};
            {
        let mut __acc: Arc<metamodelica::List<Arc<Path>>> = metamodelica::nil();
        for mut i in (start_idx.clone()..=(exts.clone().borrow().len() as i32)).into_iter() {
            let __x = InstNode::fullPath(exts.borrow()[(i.clone()-1) as usize].clone(), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(extendsPaths)
}

pub fn getNthInheritedClass(mut classPath: Arc<Path>, mut index: i32, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
        result = ValuesMake::makeBoolean(false);
        return Ok(result);
    }
    (_, _, cls_node) = frontEndLookup(program.clone(), classPath.clone())?;
    if !(InstNode::isClass(cls_node.clone())) {
        result = ValuesMake::makeBoolean(false);
        return Ok(result);
    }
    cls = InstNode::getClass(cls_node.clone())?;
    exts = 'mc: {
        let __mc_input = cls.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Class::EXPANDED_DERIVED { .. } => {
                    Ok(metamodelica::arrayFromVec(list![var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone()].into_iter().cloned().collect()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ClassTree::getExtends(Class::classTree(cls.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if index.clone() < 1 || index.clone() > (exts.clone().borrow().len() as i32) {
        result = ValuesMake::makeBoolean(false);
        return Ok(result);
    }
    result = ValuesMake::makeCodeTypeName(InstNode::fullPath(exts.borrow()[(index.clone()-1) as usize].clone(), true));
    Ok(result)
}

#[derive(Clone, Debug, PartialEq)]
pub enum InstanceTree {
    COMPONENT {
        node: Arc<InstNode::InstNode>,
        binding: Option<Arc<Binding::NFBinding>>,
        cls: Arc<InstanceTree>,
    },
    CLASS {
        node: Arc<InstNode::InstNode>,
        elements: Arc<metamodelica::List<Arc<InstanceTree>>>,
        isExtends: bool,
    },
    BUILTIN_BASE_CLASS {
        name: ArcStr,
    },
    EMPTY,
}
impl Default for InstanceTree {
    fn default() -> Self { Self::EMPTY }
}
pub use self::InstanceTree::{COMPONENT,CLASS,BUILTIN_BASE_CLASS,EMPTY};

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<InstanceTree>
// Expr: Constructor { name: 'NFApi.InstanceTree.BUILTIN_BASE_CLASS', args: [Lit(Str('enumeration'))], named_args: [], ty: RustStruct('NFApi.InstanceTree.BUILTIN_BASE_CLASS'), field_names: ['name'] }
pub fn ENUM_BASE() -> Arc<InstanceTree> { todo!("non-Sync, non-const-emittable constant ENUM_BASE — extend codegen") }

pub fn getModelInstance(mut classPath: Arc<Path>, mut contextPath: Arc<Path>, mut modifier: ArcStr, mut prettyPrint: bool) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut context: i32 = 0;
    let mut inst_tree: Arc<InstanceTree> = Arc::new(InstanceTree::EMPTY);
    let mut inst_settings: Arc<InstSettings::InstSettings>;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    match '__try0: {
        context = InstContext::set(InstContext::RELAXED.clone(), InstContext::CLASS.clone());
        context = InstContext::set(context.clone(), InstContext::INSTANCE_API.clone());
        inst_settings = Arc::new(InstSettings::InstSettings { resizableArrays: false, mergeExtendsSections: false });
        (_, top) = unwrap_break_err!(mkTop(SymbolTable::getAbsyn(), (AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false)?).clone()), '__try0);
        r#mod = parseModifier((modifier.clone()).clone(), top.clone());
        cls_node = unwrap_break_err!(Inst::lookupRootClass(classPath.clone(), top.clone(), context.clone()), '__try0);
        if SCodeUtil::isFunction(InstNode::definition(cls_node.clone())?) {
            context = InstContext::unset(context.clone(), InstContext::CLASS.clone());
            context = InstContext::set(context.clone(), InstContext::FUNCTION.clone());
        }
        if unwrap_break_err!(AbsynUtil::pathFirstIdent(contextPath.clone()), '__try0) != literal!("__NoContext") {
            cls_node = InstNode::setNodeType(Arc::new(InstNodeType::ROOT_CLASS { parent: Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), context: Some(contextPath.clone()) }), cls_node.clone());
        }
        cls_node = unwrap_break_err!(Inst::instantiateRootClass(cls_node.clone(), context.clone(), r#mod.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("Inst.instantiateRootClass")).clone()), '__try0);
        inst_tree = unwrap_break_err!(buildInstanceTree(cls_node.clone(), false), '__try0);
        unwrap_break_err!(execStat((literal!("NFApi.buildInstanceTree")).clone()), '__try0);
        unwrap_break_err!(Inst::instExpressions(cls_node.clone(), cls_node.clone(), Arc::new(openmodelica_nf_frontend::NFSections::EMPTY), NFConnectBreakTree::new(), context.clone(), inst_settings.clone()), '__try0);
        unwrap_break_err!(Inst::updateImplicitVariability(cls_node.clone(), Flags::isSet(Flags::EVAL_PARAM.clone())?, context.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("Inst.instExpressions")).clone()), '__try0);
        unwrap_break_err!(Typing::typeClassType(cls_node.clone(), Binding::EMPTY_BINDING().clone(), context.clone(), cls_node.clone()), '__try0);
        unwrap_break_err!(Typing::typeComponents(cls_node.clone(), context.clone(), false), '__try0);
        unwrap_break_err!(execStat((literal!("Typing.typeComponents")).clone()), '__try0);
        unwrap_break_err!(Typing::typeBindings(cls_node.clone(), context.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("Typing.typeBinding")).clone()), '__try0);
        json = unwrap_break_err!(dumpJSONInstanceTree(inst_tree.clone(), cls_node.clone(), true, false, false), '__try0);
        unwrap_break_err!(execStat((literal!("NFApi.dumpJSONInstanceTree")).clone()), '__try0);
        res = Arc::new(Values::Value::STRING { string: (unwrap_break_err!(JSON::toString(json.clone(), prettyPrint.clone()), '__try0)).clone() });
        unwrap_break_err!(execStat((literal!("JSON.toString")).clone()), '__try0);
        unwrap_break_err!(Inst::clearCaches(), '__try0);
        Ok::<_, anyhow::Error>((cls_node.clone(), context.clone(), inst_settings.clone(), inst_tree.clone(), json.clone(), r#mod.clone(), res.clone(), top.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7)) => {
            cls_node = __try0_o0;
            context = __try0_o1;
            inst_settings = __try0_o2;
            inst_tree = __try0_o3;
            json = __try0_o4;
            r#mod = __try0_o5;
            res = __try0_o6;
            top = __try0_o7;
        }
        Err(_) => {
            Inst::clearCaches()?;
            bail!("fail");
        }
    }
    Ok(res)
}

pub fn getModelInstanceAnnotation(mut classPath: Arc<Path>, mut filter: Arc<metamodelica::List<ArcStr>>, mut prettyPrint: bool) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut context: i32 = 0;
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    match '__try0: {
        context = InstContext::set(InstContext::RELAXED.clone(), InstContext::CLASS.clone());
        context = InstContext::set(context.clone(), InstContext::INSTANCE_API.clone());
        (_, top) = unwrap_break_err!(mkTop(SymbolTable::getAbsyn(), (AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false)?).clone()), '__try0);
        cls_node = unwrap_break_err!(Inst::lookupRootClass(classPath.clone(), top.clone(), context.clone()), '__try0);
        cls_node = InstNode::resolveInner(cls_node.clone());
        json = unwrap_break_err!(dumpJSONInstanceAnnotation(cls_node.clone(), filter.clone()), '__try0);
        res = Arc::new(Values::Value::STRING { string: (unwrap_break_err!(JSON::toString(json.clone(), prettyPrint.clone()), '__try0)).clone() });
        unwrap_break_err!(Inst::clearCaches(), '__try0);
        Ok::<_, anyhow::Error>((cls_node.clone(), context.clone(), json.clone(), res.clone(), top.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            cls_node = __try0_o0;
            context = __try0_o1;
            json = __try0_o2;
            res = __try0_o3;
            top = __try0_o4;
        }
        Err(_) => {
            Inst::clearCaches()?;
            bail!("fail");
        }
    }
    Ok(res)
}

pub fn parseModifier(mut modifierValue: ArcStr, mut scope: Arc<InstNode::InstNode>) -> Arc<Modifier::Modifier> {
    let mut outMod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    let mut amod: Arc<Absyn::Modification>;
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Parser::stringMod(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("dummy")); __mm_s.push_str(&*modifierValue.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("<internal>")).clone()), '__try0)) {
            Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__pa1), .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        amod = __pa1.clone();
        smod = unwrap_break_err!(AbsynToSCode::translateMod(Some(amod.clone()), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false), '__try0);
        outMod = unwrap_break_err!(Modifier::create(smod.clone(), (literal!("")).clone(), Arc::new(ModifierScope::ModifierScope::COMPONENT { name: (literal!("")).clone() }), scope.clone()), '__try0);
        Ok::<_, anyhow::Error>((outMod.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outMod = __try0_o0;
        }
        Err(_) => {
            outMod = Arc::new(openmodelica_nf_frontend::NFModifier::Modifier::NOMOD);
        }
    }
    outMod
}

pub fn buildInstanceTree(mut node: Arc<InstNode::InstNode>, mut isDerived: bool) -> Result<Arc<InstanceTree>> {
    let mut tree: Arc<InstanceTree> = Arc::new(InstanceTree::EMPTY);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut elems: Arc<metamodelica::List<Arc<InstanceTree>>> = metamodelica::nil();
    cls_node = InstNode::resolveInner(node.clone());
    cls = InstNode::getClass(cls_node.clone())?;
    if !(isDerived.clone()) && Class::isOnlyBuiltin(cls.clone()) && !(Class::isEnumeration(cls.clone())?) {
        tree = Arc::new(crate::NFApi::InstanceTree::EMPTY);
        return Ok(tree);
    }
    cls_tree = Class::classTree(cls.clone())?;
    tree = (::match_deref::match_deref! { match &((cls.clone(), cls_tree.clone())) {
        (Deref @ Class::EXPANDED_DERIVED { .. }, _) => {
            elems = list![buildInstanceTree(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), true)?];
            Arc::new(InstanceTree::CLASS { node: node.clone(), elements: elems.clone(), isExtends: isDerived.clone() })
        },
        (_, Deref @ ClassTree::INSTANTIATED_TREE { .. }) => {
            elems = buildInstanceTreeElements(InstNode::definition(cls_node.clone())?, cls_tree.clone())?;
            if InstNode::isRootClass(node.clone()) {
                elems = buildInstanceTreeGeneratedInners(cls_tree.clone(), elems.clone())?;
            }
            Arc::new(InstanceTree::CLASS { node: node.clone(), elements: elems.clone(), isExtends: isDerived.clone() })
        },
        (_, Deref @ ClassTree::FLAT_TREE { .. }) => Arc::new(InstanceTree::CLASS { node: node.clone(), elements: if (InstNode::isEnumerationType(cls_node.clone())) {list![ENUM_BASE().clone()]} else {metamodelica::nil()}, isExtends: isDerived.clone() }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFApi.buildInstanceTree")); __mm_s.push_str(&*literal!(" got unknown class tree")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

pub fn buildInstanceTreeElements(mut classDefinition: Arc<SCode::Element>, mut classTree: Arc<ClassTree::ClassTree>) -> Result<Arc<metamodelica::List<Arc<InstanceTree>>>> {
    let mut elements: Arc<metamodelica::List<Arc<InstanceTree>>> = metamodelica::nil();
    let mut scode_elems: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut clss: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
    let mut comps: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
    let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut cls_index: i32 = 1;
    let mut comp_index: i32 = 1;
    let mut ext_index: i32 = 1;
    let mut tree: Arc<InstanceTree> = Arc::new(InstanceTree::EMPTY);
    let mut local_comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut node_ptr: Mutable::Mutable<Arc<InstNode::InstNode>>;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(classTree.clone()) {
        Deref @ ClassTree::INSTANTIATED_TREE { localComponents: __pa0, exts: __pa1, components: __pa2, classes: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    local_comps = __pa0.clone();
    exts = __pa1.clone();
    comps = __pa2.clone();
    clss = __pa3.clone();
    scode_elems = SCodeUtil::getClassElements(classDefinition.clone());
    if !(local_comps.clone().is_empty()) {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(local_comps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        comp_index = __pa4.clone();
        local_comps = __pa5.clone();
    }
    for mut e in &*scode_elems.clone() {
        let mut e = e.clone();
        elements = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => {
            tree = buildInstanceTree(exts.borrow()[(ext_index.clone()-1) as usize].clone(), true)?;
            ext_index = ext_index.clone() + 1;
            cons(tree.clone(), elements.clone())
        },
        Deref @ SCode::Element::CLASS { .. } if (SCodeUtil::isElementReplaceable(e.clone())?) => {
            while InstNode::name(Mutable::access(clss.borrow()[(cls_index.clone()-1) as usize].clone()))? != var_field!((*e).name, SCode::Element::CLASS).clone() {
                cls_index = cls_index.clone() + 1;
            }
            tree = Arc::new(InstanceTree::CLASS { node: Mutable::access(clss.borrow()[(cls_index.clone()-1) as usize].clone()), elements: metamodelica::nil(), isExtends: false });
            cls_index = cls_index.clone() + 1;
            cons(tree.clone(), elements.clone())
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            while true {
                node = Mutable::access(comps.borrow()[(comp_index.clone()-1) as usize].clone());
                if InstNode::name(node.clone())? == var_field!((*e).name, SCode::Element::COMPONENT).clone() && !(InstNode::isGeneratedInner(node.clone())) {
                    break;
                }
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(local_comps.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                comp_index = __pa0.clone();
                local_comps = __pa1.clone();
            }
            tree = buildInstanceTreeComponent(node.clone())?;
            elements = cons(tree.clone(), elements.clone());
            if !(local_comps.clone().is_empty()) {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(local_comps.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                comp_index = __pa2.clone();
                local_comps = __pa3.clone();
            }
            elements.clone()
        },
        _ => elements.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    elements = elements.clone().reverse();
    Ok(elements)
}

pub fn buildInstanceTreeGeneratedInners(mut classTree: Arc<ClassTree::ClassTree>, mut elements: Arc<metamodelica::List<Arc<InstanceTree>>>) -> Result<Arc<metamodelica::List<Arc<InstanceTree>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<InstanceTree>>> = metamodelica::nil();
    let mut comps: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
    let mut comp: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut elems: Arc<metamodelica::List<Arc<InstanceTree>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(classTree.clone()) {
        Deref @ ClassTree::INSTANTIATED_TREE { components: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    let __range1 = (1..=(comps.clone().borrow().len() as i32)).rev();
    for mut i in __range1 {
        if InstNode::isGeneratedInner(Mutable::access(comps.borrow()[(i.clone()-1) as usize].clone())) {
            elems = cons(buildInstanceTreeComponent(Mutable::access(comps.borrow()[(i.clone()-1) as usize].clone()))?, elems.clone());
        } else {
            break;
        }
    }
    if elems.clone().is_empty() {
        outElements = elements.clone();
    } else {
        outElements = listAppend(elements.clone(), elems.clone());
    }
    Ok(outElements)
}

pub fn buildInstanceTreeComponent(mut node: Arc<InstNode::InstNode>) -> Result<Arc<InstanceTree>> {
    let mut tree: Arc<InstanceTree> = Arc::new(InstanceTree::EMPTY);
    let mut inner_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<InstanceTree> = Arc::new(InstanceTree::EMPTY);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut opt_binding: Option<Arc<Binding::NFBinding>> = None;
    inner_node = InstNode::resolveOuter(node.clone());
    cls_node = InstNode::classScope(inner_node.clone());
    if InstNode::isEmpty(cls_node.clone()) {
        cls = Arc::new(crate::NFApi::InstanceTree::EMPTY);
    } else {
        cls = buildInstanceTree(cls_node.clone(), false)?;
    }
    if InstNode::isComponent(inner_node.clone()) {
        binding = Component::getBinding(InstNode::component(inner_node.clone())?);
        opt_binding = if (Binding::isBound(binding.clone())) {Some(binding.clone())} else {None};
    } else {
        opt_binding = None;
    }
    tree = Arc::new(InstanceTree::COMPONENT { node: node.clone(), binding: opt_binding.clone(), cls: cls.clone() });
    Ok(tree)
}

pub fn dumpJSONInstanceTree(mut tree: Arc<InstanceTree>, mut scope: Arc<InstNode::InstNode>, mut root: bool, mut isDeleted: bool, mut isExtends: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut elems: Arc<metamodelica::List<Arc<InstanceTree>>> = metamodelica::nil();
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut cmt: Option<Arc<SCode::Comment>> = None;
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut def: Arc<SCode::Element>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ InstanceTree::CLASS { elements: __pa0, node: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    elems = __pa0.clone();
    node = __pa1.clone();
    node = InstNode::resolveOuter(node.clone());
    def = InstNode::definition(node.clone())?;
    cmt = SCodeUtil::getElementComment(def.clone());
    json = JSON::addPair((literal!("name")).clone(), dumpJSONNodePath(node.clone(), !(isExtends.clone())), json.clone())?;
    json = JSON::addPairNotNull((literal!("dims")).clone(), dumpJSONClassDims(node.clone(), def.clone())?, json.clone())?;
    json = JSON::addPair((literal!("restriction")).clone(), JSON::makeString((SCodeDump::restrictionStringPP(SCodeUtil::getClassRestriction(def.clone())?)?).clone()), json.clone())?;
    json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONClassPrefixes(def.clone(), InstNode::parent(node.clone()))?, json.clone())?;
    json = dumpJSONCommentOpt(cmt.clone(), scope.clone(), json.clone(), true, true, false)?;
    json = JSON::addPairNotNull((literal!("elements")).clone(), dumpJSONElements(elems.clone(), node.clone(), isDeleted.clone())?, json.clone())?;
    if !(isDeleted.clone()) {
        json = dumpJSONImports(node.clone(), json.clone())?;
        sections = Class::getSections(InstNode::getClass(node.clone())?)?;
        json = dumpJSONEquations(sections.clone(), node.clone(), json.clone())?;
    }
    json = JSON::addPair((literal!("source")).clone(), dumpJSONSourceInfo(InstNode::info(node.clone())?, true)?, json.clone())?;
    Ok(json)
}

pub fn dumpJSONInstanceAnnotation(mut node: Arc<InstNode::InstNode>, mut filter: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut cmt: Option<Arc<SCode::Comment>> = None;
    let mut ann: Arc<SCode::Annotation>;
    let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut scope: Arc<InstNode::InstNode> = node.clone();
    let mut context: i32 = 0;
    let mut annotation_is_literal: bool = true;
    let mut def: Arc<SCode::Element>;
    Inst::expand(node.clone(), InstContext::RELAXED.clone())?;
    def = InstNode::definition(node.clone())?;
    json = JSON::addPair((literal!("name")).clone(), dumpJSONNodePath(node.clone(), false), json.clone())?;
    json = JSON::addPair((literal!("restriction")).clone(), JSON::makeString((Restriction::toString(InstNode::restriction(node.clone()))).clone()), json.clone())?;
    json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONClassPrefixes(def.clone(), InstNode::parent(node.clone()))?, json.clone())?;
    exts = ClassTree::getExtends(Class::classTree(InstNode::getClass(node.clone())?)?);
    if !(exts.clone().borrow().is_empty()) {
        j = JSON::emptyArray(0);
        let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut ext in __range0 {
            j = JSON::addElement(dumpJSONInstanceAnnotationExtends(ext.clone(), filter.clone())?, j.clone())?;
        }
        json = JSON::addPair((literal!("elements")).clone(), j.clone(), json.clone())?;
    }
    cmt = SCodeUtil::getElementComment(InstNode::definition(node.clone())?);
    cmt = (::match_deref::match_deref! { match &(cmt.clone()) {
        Some(Deref @ SCode::Comment { annotation_: Some(ann @ Deref @ SCode::Annotation { .. }), .. }) => {
            let mut ann = (*ann).clone();
            if !(filter.clone().is_empty()) {
                assign_field!(ann.modification = SCodeUtil::filterSubMods(ann.modification.clone(), Arc::new({ let __pe_b1 = filter.clone(); move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(__pe_a0, __pe_b1.clone())) })));
            }
            annotation_is_literal = SCodeUtil::onlyLiteralsInMod(ann.modification.clone())?;
            if (SCodeUtil::isEmptyMod(ann.modification.clone())) {None} else {Some(Arc::new(SCode::Comment { annotation_: Some(ann.clone()), comment: None }))}
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(annotation_is_literal.clone()) {
        ErrorExt::setCheckpoint((literal!("NFApi.dumpJSONInstanceAnnotation")).clone());
        if '__try1: {
            context = InstContext::set(InstContext::CLASS.clone(), InstContext::RELAXED.clone());
            scope = InstNode::makeRootClass(scope.clone(), Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), None);
            scope = unwrap_break_err!(Inst::instantiate(scope.clone(), Arc::new(openmodelica_nf_frontend::NFModifier::Modifier::NOMOD), Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), context.clone(), true), '__try1);
            unwrap_break_err!(Inst::insertGeneratedInners(scope.clone(), InstNode::topScope(scope.clone()), context.clone()), '__try1);
            unwrap_break_err!(Inst::instExpressions(scope.clone(), scope.clone(), Arc::new(openmodelica_nf_frontend::NFSections::EMPTY), NFConnectBreakTree::new(), context.clone(), Inst::DEFAULT_SETTINGS.clone()), '__try1);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
        ErrorExt::rollBack((literal!("NFApi.dumpJSONInstanceAnnotation")).clone());
    }
    json = dumpJSONCommentOpt(cmt.clone(), scope.clone(), json.clone(), true, true, true)?;
    Ok(json)
}

pub fn dumpJSONInstanceAnnotationExtends(mut ext: Arc<InstNode::InstNode>, mut filter: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("extends")).clone() }), json.clone())?;
    json = JSON::addPair((literal!("baseClass")).clone(), dumpJSONInstanceAnnotation(ext.clone(), filter.clone())?, json.clone())?;
    Ok(json)
}

pub fn dumpJSONNodePath(mut node: Arc<InstNode::InstNode>, mut ignoreBaseClass: bool) -> Arc<JSON::JSON> {
    let mut json: Arc<JSON::JSON> = dumpJSONPath(InstNode::enclosingScopePath(node.clone(), false, ignoreBaseClass.clone()).unwrap());
    json
}

pub fn dumpJSONNodeEnclosingPath(mut node: Arc<InstNode::InstNode>) -> Arc<JSON::JSON> {
    let mut json: Arc<JSON::JSON> = dumpJSONPath(InstNode::enclosingScopePath(node.clone(), true, false).unwrap());
    json
}

pub fn dumpJSONPath(mut path: Arc<Path>) -> Arc<JSON::JSON> {
    let mut json: Arc<JSON::JSON> = JSON::makeString((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false).unwrap()).clone());
    json
}

pub fn dumpJSONElements(mut elements: Arc<metamodelica::List<Arc<InstanceTree>>>, mut scope: Arc<InstNode::InstNode>, mut isDeleted: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    if isDeleted.clone() {
        for mut e in &*elements.clone() {
            let mut e = e.clone();
            j = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ InstanceTree::CLASS { isExtends: true, .. } => dumpJSONExtends(e.clone(), isDeleted.clone())?,
        _ => JSON::makeNull(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            json = JSON::addElementNotNull(j.clone(), json.clone())?;
        }
    } else {
        for mut e in &*elements.clone() {
            let mut e = e.clone();
            j = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ InstanceTree::CLASS { isExtends: true, .. } => dumpJSONExtends(e.clone(), isDeleted.clone())?,
        Deref @ InstanceTree::CLASS { .. } => dumpJSONReplaceableClass(var_field!((*e).node, InstanceTree::CLASS).clone(), scope.clone())?,
        Deref @ InstanceTree::COMPONENT { .. } => dumpJSONComponent(var_field!((*e).node, InstanceTree::COMPONENT).clone(), var_field!((*e).binding, InstanceTree::COMPONENT).clone(), var_field!((*e).cls, InstanceTree::COMPONENT).clone())?,
        Deref @ InstanceTree::BUILTIN_BASE_CLASS { .. } => dumpJSONBuiltinBaseClass((var_field!((*e).name, InstanceTree::BUILTIN_BASE_CLASS).clone()).clone())?,
        _ => JSON::makeNull(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            json = JSON::addElementNotNull(j.clone(), json.clone())?;
        }
    }
    Ok(json)
}

pub fn dumpJSONExtends(mut ext: Arc<InstanceTree>, mut isDeleted: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_def: Arc<SCode::Element>;
    let mut ext_def: Arc<SCode::Element>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let __pa0 = ::match_deref::match_deref! { match &(ext.clone()) {
        Deref @ InstanceTree::CLASS { node: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    node = __pa0.clone();
    cls_def = InstNode::definition(node.clone())?;
    let __pa1 = ::match_deref::match_deref! { match &(InstNode::extendsDefinition(node.clone())?) {
        Some(__pa1) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ext_def = __pa1.clone();
    json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("extends")).clone() }), json.clone())?;
    json = dumpJSONSCodeMod(getExtendsModifier(ext_def.clone(), node.clone())?, node.clone(), json.clone())?;
    json = dumpJSONCommentOpt(SCodeUtil::getElementComment(ext_def.clone()), node.clone(), json.clone(), true, true, false)?;
    cls = InstNode::getClass(node.clone())?;
    if Class::isOnlyBuiltin(cls.clone()) && !(Class::isEnumeration(cls.clone())?) {
        json = JSON::addPair((literal!("baseClass")).clone(), JSON::makeString((InstNode::name(node.clone())?).clone()), json.clone())?;
    } else {
        json = JSON::addPair((literal!("baseClass")).clone(), dumpJSONInstanceTree(ext.clone(), node.clone(), false, isDeleted.clone(), true)?, json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONBuiltinBaseClass(mut name: ArcStr) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("extends")).clone() }), json.clone())?;
    json = JSON::addPair((literal!("baseClass")).clone(), JSON::makeString((name.clone()).clone()), json.clone())?;
    Ok(json)
}

pub fn getExtendsModifier(mut definition: Arc<SCode::Element>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    r#mod = (::match_deref::match_deref! { match &(definition.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => var_field!((*definition).modifications, SCode::Element::EXTENDS).clone(),
        Deref @ SCode::Element::CLASS { .. } => SCodeUtil::elementMod(InstNode::definition(InstNode::getDerivedNode(node.clone(), false))?),
        _ => Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn dumpJSONReplaceableClass(mut cls: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut elem: Arc<SCode::Element>;
    let mut cdef: Arc<SCode::ClassDef>;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut derivedNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut path: Arc<Path>;
    let mut odims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> = None;
    let mut cmt: Arc<SCode::Comment>;
    node = InstNode::getRedeclaredNode(cls.clone());
    elem = InstNode::definition(node.clone())?;
    json = dumpJSONSCodeClass(elem.clone(), scope.clone(), node.clone(), true, json.clone())?;
    json = JSON::addPair((literal!("source")).clone(), dumpJSONSourceInfo(InstNode::info(node.clone())?, true)?, json.clone())?;
    Ok(json)
}

pub fn dumpJSONComponent(mut component: Arc<InstNode::InstNode>, mut originalBinding: Option<Arc<Binding::NFBinding>>, mut cls: Arc<InstanceTree>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut elem: Arc<SCode::Element>;
    let mut is_constant: bool = false;
    let mut cmt: Arc<SCode::Comment>;
    let mut ann: Arc<SCode::Annotation>;
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut path: Arc<Path>;
    node = InstNode::resolveOuter(component.clone());
    comp = InstNode::component(node.clone())?;
    elem = InstNode::definition(node.clone())?;
    scope = InstNode::parent(node.clone());
    json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("component")).clone() }), json.clone())?;
    json = JSON::addPair((literal!("name")).clone(), JSON::makeString((InstNode::name(node.clone())?).clone()), json.clone())?;
    let () = (::match_deref::match_deref! { match &((comp.clone(), elem.clone())) {
        (Deref @ Component::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) if (Component::isDeleted(comp.clone())?) => {
            json = JSON::addPair((literal!("type")).clone(), dumpJSONComponentType(cls.clone(), node.clone(), var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone(), true)?, json.clone())?;
            json = dumpJSONSCodeMod(var_field!((*elem).modifications, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone())?;
            json = JSON::addPair((literal!("condition")).clone(), JSON::makeBoolean(false), json.clone())?;
            json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT).clone(), var_field!((*elem).prefixes, SCode::Element::COMPONENT).clone(), scope.clone())?, json.clone())?;
            json = dumpJSONComment(var_field!((*elem).comment, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone(), true, true, false)?;
            ()
        },
        (Deref @ Component::INVALID_COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            json = JSON::addPair((literal!("type")).clone(), dumpJSONComponentType(cls.clone(), node.clone(), Component::getType(comp.clone())?, false)?, json.clone())?;
            json = dumpJSONSCodeMod(var_field!((*elem).modifications, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone())?;
            json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT).clone(), var_field!((*elem).prefixes, SCode::Element::COMPONENT).clone(), scope.clone())?, json.clone())?;
            json = dumpJSONComment(var_field!((*elem).comment, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone(), true, true, false)?;
            json = JSON::addPair((literal!("$error")).clone(), JSON::makeString((var_field!((*comp).errors, Component::NFComponent::INVALID_COMPONENT).clone()).clone()), json.clone())?;
            ()
        },
        (Deref @ Component::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            json = JSON::addPair((literal!("type")).clone(), dumpJSONComponentType(cls.clone(), node.clone(), var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone(), false)?, json.clone())?;
            if Type::isArray(var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone()) {
                json = JSON::addPair((literal!("dims")).clone(), dumpJSONDims(var_field!((*elem).attributes, SCode::Element::COMPONENT).arrayDims.clone(), Type::arrayDims(var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone()))?, json.clone())?;
            }
            json = dumpJSONSCodeMod(var_field!((*elem).modifications, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone())?;
            is_constant = var_field!((*comp).attributes, Component::NFComponent::COMPONENT).variability.clone() <= Variability::PARAMETER.clone() && Binding::purity(var_field!((*comp).binding, Component::NFComponent::COMPONENT).clone()) == Purity::PURE.clone();
            if Binding::isExplicitlyBound(var_field!((*comp).binding, Component::NFComponent::COMPONENT).clone()) {
                json = JSON::addPair((literal!("value")).clone(), dumpJSONBinding(var_field!((*comp).binding, Component::NFComponent::COMPONENT).clone(), originalBinding.clone(), is_constant.clone())?, json.clone())?;
            }
            if Binding::isBound(var_field!((*comp).condition, Component::NFComponent::COMPONENT).clone()) {
                json = JSON::addPair((literal!("condition")).clone(), dumpJSONBinding(var_field!((*comp).condition, Component::NFComponent::COMPONENT).clone(), None, true)?, json.clone())?;
            }
            json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT).clone(), var_field!((*elem).prefixes, SCode::Element::COMPONENT).clone(), scope.clone())?, json.clone())?;
            json = dumpJSONComment(var_field!((*comp).comment, Component::NFComponent::COMPONENT).clone(), scope.clone(), json.clone(), true, true, false)?;
            if InstNode::isGeneratedInner(node.clone()) {
                json = JSON::addPair((literal!("generated")).clone(), JSON::makeBoolean(true), json.clone())?;
            }
            ()
        },
        (Deref @ Component::COMPONENT_DEF { .. }, Deref @ SCode::Element::COMPONENT { .. }) if (AbsynUtil::isOnlyOuter(var_field!((*elem).prefixes, SCode::Element::COMPONENT).innerOuter.clone())) => {
            path = AbsynUtil::typeSpecPath(var_field!((*elem).typeSpec, SCode::Element::COMPONENT).clone())?;
            match '__try0: {
                (ty_node, _) = unwrap_break_err!(Lookup::lookupName(path.clone(), scope.clone(), InstContext::set(InstContext::RELAXED.clone(), InstContext::FAST_LOOKUP.clone()), false), '__try0);
                json = unwrap_break_err!(JSON::addPair((literal!("type")).clone(), dumpJSONSCodeClass(InstNode::definition(ty_node.clone())?, ty_node.clone(), InstNode::resolveInner(component.clone()), false, JSON::makeNull())?, json.clone()), '__try0);
                Ok::<_, anyhow::Error>((json.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    json = __try0_o0;
                }
                Err(_) => {
                    json = JSON::addPair((literal!("type")).clone(), dumpJSONPath(path.clone()), json.clone())?;
                }
            }
            json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT).clone(), var_field!((*elem).prefixes, SCode::Element::COMPONENT).clone(), scope.clone())?, json.clone())?;
            json = dumpJSONComment(var_field!((*elem).comment, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone(), true, true, false)?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFApi.dumpJSONComponent")); __mm_s.push_str(&*literal!(" got unknown component ")); __mm_s.push_str(&*InstNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONComponentType(mut cls: Arc<InstanceTree>, mut node: Arc<InstNode::InstNode>, mut ty: Arc<Type::NFType>, mut isDeleted: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = (::match_deref::match_deref! { match &((cls.clone(), Type::arrayElementType(ty.clone()))) {
        (_, Deref @ Type::ENUMERATION { .. }) => dumpJSONEnumType(cls.clone(), node.clone())?,
        (_, Deref @ Type::UNKNOWN) => dumpJSONSCodeElementType(InstNode::definition(node.clone())?)?,
        (Deref @ InstanceTree::CLASS { .. }, _) => dumpJSONInstanceTree(cls.clone(), node.clone(), true, isDeleted.clone(), false)?,
        _ => dumpJSONTypeName(ty.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONSCodeElementType(mut elem: Arc<SCode::Element>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let () = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            json = JSON::addPair((literal!("name")).clone(), dumpJSONPath(AbsynUtil::typeSpecPath(var_field!((*elem).typeSpec, SCode::Element::COMPONENT).clone())?), json.clone())?;
            json = JSON::addPair((literal!("missing")).clone(), JSON::makeBoolean(true), json.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONEnumType(mut tree: Arc<InstanceTree>, mut enumNode: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut node: Arc<InstNode::InstNode> = InstNode::resolveInner(InstNode::classScope(enumNode.clone()));
    let mut def: Arc<SCode::Element>;
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut json_elems: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut json_ext: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut elems: Arc<metamodelica::List<Arc<InstanceTree>>> = metamodelica::nil();
    def = InstNode::definition(node.clone())?;
    json = JSON::makeNull();
    json = JSON::addPair((literal!("name")).clone(), dumpJSONNodePath(node.clone(), false), json.clone())?;
    json = JSON::addPairNotNull((literal!("dims")).clone(), dumpJSONClassDims(node.clone(), def.clone())?, json.clone())?;
    json = JSON::addPair((literal!("restriction")).clone(), JSON::makeString((SCodeDump::restrictionStringPP(SCodeUtil::getClassRestriction(def.clone())?)?).clone()), json.clone())?;
    json = dumpJSONCommentOpt(SCodeUtil::getElementComment(def.clone()), node.clone(), json.clone(), true, true, false)?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ InstanceTree::CLASS { elements: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    elems = __pa0.clone();
    json_elems = dumpJSONElements(elems.clone(), node.clone(), false)?;
    comps = ClassTree::getComponents(Class::classTree(InstNode::getClass(node.clone())?)?)?;
    json_elems = dumpJSONEnumTypeLiterals(comps.clone(), InstNode::parent(node.clone()), json_elems.clone())?;
    json = JSON::addPair((literal!("elements")).clone(), json_elems.clone(), json.clone())?;
    json = JSON::addPair((literal!("source")).clone(), dumpJSONSourceInfo(InstNode::info(node.clone())?, true)?, json.clone())?;
    Ok(json)
}

pub fn dumpJSONEnumTypeLiterals(mut literals: metamodelica::Array<Arc<InstNode::InstNode>>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let __range0 = 6..=(literals.clone().borrow().len() as i32);
    for mut i in __range0 {
        json = JSON::addElement(dumpJSONEnumTypeLiteral(literals.borrow()[(i.clone()-1) as usize].clone(), scope.clone())?, json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONEnumTypeLiteral(mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("component")).clone() }), json.clone())?;
    json = JSON::addPair((literal!("name")).clone(), JSON::makeString((InstNode::name(node.clone())?).clone()), json.clone())?;
    json = dumpJSONComment(Component::comment(InstNode::component(node.clone())?)?, scope.clone(), json.clone(), true, true, false)?;
    Ok(json)
}

pub fn dumpJSONTypeName(mut ty: Arc<Type::NFType>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = JSON::makeString((Type::toString(Type::arrayElementType(ty.clone()))?).clone());
    Ok(json)
}

pub fn dumpJSONBinding(mut binding: Arc<Binding::NFBinding>, mut originalBinding: Option<Arc<Binding::NFBinding>>, mut evaluate: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut bind: Arc<Binding::NFBinding> = binding.clone();
    let mut context: i32 = 0;
    if isSome(originalBinding.clone()) && Binding::isEvaluated(binding.clone()) {
        if '__try0: {
            context = InstContext::set(InstContext::RELAXED.clone(), InstContext::INSTANCE_API.clone());
            bind = unwrap_break_err!(Inst::instBinding(Util::getOption(originalBinding.clone())?, context.clone()), '__try0);
            bind = unwrap_break_err!(Typing::typeBinding(bind.clone(), context.clone()), '__try0);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    exp = Binding::getExp(bind.clone())?;
    exp = Expression::map(exp.clone(), Arc::new(Expression::expandSplitIndices))?;
    json = JSON::addPair((literal!("binding")).clone(), Expression::toJSON(exp.clone())?, json.clone())?;
    if evaluate.clone() && !(Expression::isLiteral(exp.clone())) {
        ErrorExt::setCheckpoint((literal!("NFApi.dumpJSONBinding")).clone());
        if '__try1: {
            exp = unwrap_break_err!(Ceval::evalExp(exp.clone(), Ceval::EvalTarget::new(Absyn::dummyInfo.clone(), InstContext::INSTANCE_API.clone(), None)), '__try1);
            exp = unwrap_break_err!(Expression::map(exp.clone(), Arc::new(Expression::expandSplitIndices)), '__try1);
            json = unwrap_break_err!(JSON::addPair((literal!("value")).clone(), Expression::toJSON(exp.clone())?, json.clone()), '__try1);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
        ErrorExt::rollBack((literal!("NFApi.dumpJSONBinding")).clone());
    }
    Ok(json)
}

pub fn dumpJSONClassDims(mut node: Arc<InstNode::InstNode>, mut element: Arc<SCode::Element>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut absyn_dims: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    ty = InstNode::getType(node.clone())?;
    if Type::isArray(ty.clone()) {
        absyn_dims = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(absyn_dims), .. }, .. }, .. } => absyn_dims.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        json = dumpJSONDims(absyn_dims.clone(), Type::arrayDims(ty.clone()))?;
    } else {
        json = JSON::makeNull();
    }
    Ok(json)
}

pub fn dumpJSONDims(mut absynDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut typedDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut ty_json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut absyn_json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = JSON::addPairNotNull((literal!("absyn")).clone(), dumpJSONAbsynDims(absynDims.clone())?, json.clone())?;
    ty_json = JSON::makeNull();
    for mut d in &*typedDims.clone() {
        let mut d = d.clone();
        ty_json = JSON::addElement(JSON::makeString((Dimension::toString(d.clone())?).clone()), ty_json.clone())?;
    }
    json = JSON::addPairNotNull((literal!("typed")).clone(), ty_json.clone(), json.clone())?;
    Ok(json)
}

pub fn dumpJSONAbsynDims(mut dims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    for mut d in &*dims.clone() {
        let mut d = d.clone();
        json = JSON::addElement(JSON::makeString((Dump::printSubscriptStr(d.clone())?).clone()), json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONAttributes(mut attrs: SCode::Attributes, mut prefs: Arc<SCode::Prefixes>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut s: ArcStr = arcstr::literal!("");
    json = dumpJSONSCodePrefixes(prefs.clone(), scope.clone())?;
    s = (SCodeDump::connectorTypeStr(attrs.connectorType.clone())?).clone();
    if !(stringEmpty((s.clone()).clone())) {
        json = JSON::addPair((literal!("connector")).clone(), JSON::makeString((s.clone()).clone()), json.clone())?;
    }
    s = (SCodeDump::unparseVariability(attrs.variability.clone())?).clone();
    if !(stringEmpty((s.clone()).clone())) {
        json = JSON::addPair((literal!("variability")).clone(), JSON::makeString((s.clone()).clone()), json.clone())?;
    }
    if AbsynUtil::isInput(attrs.direction.clone()) {
        json = JSON::addPair((literal!("direction")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("input")).clone() }), json.clone())?;
    } else if AbsynUtil::isOutput(attrs.direction.clone()) {
        json = JSON::addPair((literal!("direction")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("output")).clone() }), json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONSCodePrefixes(mut prefixes: Arc<SCode::Prefixes>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    if !(SCodeUtil::visibilityBool(prefixes.visibility.clone())?) {
        json = JSON::addPair((literal!("public")).clone(), JSON::makeBoolean(false), json.clone())?;
    }
    if SCodeUtil::finalBool(prefixes.finalPrefix.clone())? {
        json = JSON::addPair((literal!("final")).clone(), JSON::makeBoolean(true), json.clone())?;
    }
    if AbsynUtil::isInner(prefixes.innerOuter.clone()) {
        json = JSON::addPair((literal!("inner")).clone(), JSON::makeBoolean(true), json.clone())?;
    }
    if AbsynUtil::isOuter(prefixes.innerOuter.clone()) {
        json = JSON::addPair((literal!("outer")).clone(), JSON::makeBoolean(true), json.clone())?;
    }
    json = JSON::addPairNotNull((literal!("replaceable")).clone(), dumpJSONReplaceable(prefixes.replaceablePrefix.clone(), scope.clone())?, json.clone())?;
    if SCodeUtil::redeclareBool(prefixes.redeclarePrefix.clone())? {
        json = JSON::addPair((literal!("redeclare")).clone(), JSON::makeBoolean(true), json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONClassPrefixes(mut element: Arc<SCode::Element>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut prefs: Arc<SCode::Prefixes>;
    let mut cdef: Arc<SCode::ClassDef>;
    json = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { prefixes: prefs, classDef: cdef, .. } => {
            json = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::DERIVED { .. } => dumpJSONAttributes(var_field!((**cdef).attributes, SCode::ClassDef::DERIVED).clone(), var_field!((*element).prefixes, SCode::Element::CLASS).clone(), scope.clone())?,
        _ => dumpJSONSCodePrefixes(var_field!((*element).prefixes, SCode::Element::CLASS).clone(), scope.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if SCodeUtil::partialBool(var_field!((*element).partialPrefix, SCode::Element::CLASS).clone())? {
                json = JSON::addPair((literal!("partial")).clone(), JSON::makeBoolean(true), json.clone())?;
            }
            if SCodeUtil::encapsulatedBool(var_field!((*element).encapsulatedPrefix, SCode::Element::CLASS).clone())? {
                json = JSON::addPair((literal!("encapsulated")).clone(), JSON::makeBoolean(true), json.clone())?;
            }
            json.clone()
        },
        _ => JSON::makeNull(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONReplaceable(mut repl: Arc<SCode::Replaceable>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut cc: Arc<SCode::ConstrainClass>;
    json = (::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) } => {
            json = JSON::makeNull();
            json = JSON::addPair((literal!("constrainedby")).clone(), dumpJSONPath(cc.constrainingClass.clone()), json.clone())?;
            json = dumpJSONSCodeMod(cc.modifier.clone(), scope.clone(), json.clone())?;
            json = dumpJSONCommentOpt(Some(cc.comment.clone()), scope.clone(), json.clone(), true, true, false)?;
            json.clone()
        },
        Deref @ SCode::Replaceable::REPLACEABLE { .. } => JSON::makeBoolean(true),
        _ => JSON::makeNull(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONCommentOpt(mut cmtOpt: Option<Arc<SCode::Comment>>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>, mut dumpComment: bool, mut dumpAnnotation: bool, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    if isSome(cmtOpt.clone()) {
        json = dumpJSONComment(Util::getOption(cmtOpt.clone())?, scope.clone(), json.clone(), dumpComment.clone(), dumpAnnotation.clone(), failOnError.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONComment(mut cmt: Arc<SCode::Comment>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>, mut dumpComment: bool, mut dumpAnnotation: bool, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    if isSome(cmt.comment.clone()) && dumpComment.clone() {
        json = JSON::addPair((literal!("comment")).clone(), JSON::makeString((Util::getOption(cmt.comment.clone())?).clone()), json.clone())?;
    }
    if dumpAnnotation.clone() {
        json = dumpJSONAnnotationOpt(cmt.annotation_.clone(), scope.clone(), metamodelica::nil(), failOnError.clone(), json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONCommentAnnotation(mut cmtOpt: Option<Arc<SCode::Comment>>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>, mut filter: Arc<metamodelica::List<ArcStr>>, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut cmt: Arc<SCode::Comment>;
    if isSome(cmtOpt.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(cmtOpt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cmt = __pa0.clone();
        json = dumpJSONAnnotationOpt(cmt.annotation_.clone(), scope.clone(), filter.clone(), failOnError.clone(), json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONAnnotationOpt(mut annOpt: Option<Arc<SCode::Annotation>>, mut scope: Arc<InstNode::InstNode>, mut filter: Arc<metamodelica::List<ArcStr>>, mut failOnError: bool, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut ann: Arc<SCode::Annotation>;
    if isSome(annOpt.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(annOpt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ann = __pa0.clone();
        json = JSON::addPair((literal!("annotation")).clone(), dumpJSONAnnotationMod(ann.modification.clone(), scope.clone(), filter.clone(), failOnError.clone())?, json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONAnnotationMod(mut r#mod: Arc<SCode::Mod>, mut scope: Arc<InstNode::InstNode>, mut filter: Arc<metamodelica::List<ArcStr>>, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => dumpJSONAnnotationSubMods(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), scope.clone(), filter.clone(), failOnError.clone())?,
        _ => JSON::makeNull(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONAnnotationSubMods(mut subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut scope: Arc<InstNode::InstNode>, mut filter: Arc<metamodelica::List<ArcStr>>, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    for mut m in &*subMods.clone() {
        let mut m = m.clone();
        if filter.clone().is_empty() || List::contains(filter.clone(), (m.ident.clone()).clone(), Arc::new(stringEq)) {
            json = dumpJSONAnnotationSubMod(m.clone(), scope.clone(), failOnError.clone(), json.clone())?;
        }
    }
    Ok(json)
}

pub fn dumpJSONAnnotationSubMod(mut subMod: Arc<SCode::SubMod>, mut scope: Arc<InstNode::InstNode>, mut failOnError: bool, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut name: ArcStr = arcstr::literal!("");
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut absyn_binding: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut binding_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ SCode::SubMod { r#mod: __pa0, ident: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r#mod = __pa0.clone();
    name = __pa1.clone();
    let () = (::match_deref::match_deref! { match &((name.clone(), r#mod.clone())) {
        (Deref @ "choices", Deref @ SCode::Mod::MOD { .. }) => {
            j = dumpJSONChoicesAnnotation(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), scope.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone(), failOnError.clone())?;
            json = JSON::addPairNotNull((name.clone()).clone(), j.clone(), json.clone())?;
            ()
        },
        (_, Deref @ SCode::Mod::MOD { binding: Some(absyn_binding), .. }) => {
            j = dumpJSONAnnotationExp(absyn_binding.clone(), scope.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone(), failOnError.clone())?;
            json = JSON::addPair((name.clone()).clone(), j.clone(), json.clone())?;
            ()
        },
        (_, Deref @ SCode::Mod::MOD { .. }) => {
            json = JSON::addPair((name.clone()).clone(), dumpJSONAnnotationSubMods(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), scope.clone(), metamodelica::nil(), failOnError.clone())?, json.clone())?;
            ()
        },
        (_, Deref @ SCode::Mod::NOMOD { .. }) => {
            json = JSON::addPair((name.clone()).clone(), JSON::emptyListObject(), json.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONAnnotationExp(mut absynExp: Arc<Absyn::Exp>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = (::match_deref::match_deref! { match &(absynExp.clone()) {
        Deref @ Absyn::Exp::INTEGER { .. } => JSON::makeInteger(var_field!((*absynExp).value, Absyn::Exp::INTEGER).clone()),
        Deref @ Absyn::Exp::REAL { .. } => JSON::makeNumber(stringReal((var_field!((*absynExp).value, Absyn::Exp::REAL).clone()).clone())?),
        Deref @ Absyn::Exp::STRING { .. } => JSON::makeString((var_field!((*absynExp).value, Absyn::Exp::STRING).clone()).clone()),
        Deref @ Absyn::Exp::BOOL { .. } => JSON::makeBoolean(var_field!((*absynExp).value, Absyn::Exp::BOOL).clone()),
        Deref @ Absyn::Exp::ARRAY { .. } if (!(AbsynUtil::isLiteralExp(absynExp.clone()))) => {
            json = JSON::emptyArray((var_field!((*absynExp).arrayExp, Absyn::Exp::ARRAY).clone().len() as i32));
            for mut e in &*var_field!((*absynExp).arrayExp, Absyn::Exp::ARRAY).clone() {
                let mut e = e.clone();
                j = dumpJSONAnnotationExp(e.clone(), scope.clone(), info.clone(), failOnError.clone())?;
                json = JSON::addElement(j.clone(), json.clone())?;
            }
            json.clone()
        },
        _ => dumpJSONAnnotationExp2(absynExp.clone(), scope.clone(), info.clone(), failOnError.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONAnnotationExp2(mut absynExp: Arc<Absyn::Exp>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    ErrorExt::setCheckpoint((literal!("NFApi.dumpJSONAnnotationExp2")).clone());
    match '__try0: {
        exp = unwrap_break_err!(Inst::instExp(absynExp.clone(), scope.clone(), INST_API_ANNOTATION_CONTEXT.clone(), info.clone()), '__try0);
        (exp, _, _, _) = unwrap_break_err!(Typing::typeExp(exp.clone(), INST_API_ANNOTATION_CONTEXT.clone(), info.clone(), false), '__try0);
        exp = unwrap_break_err!(SimplifyExp::simplify(exp.clone(), false), '__try0);
        json = unwrap_break_err!(Expression::toJSON(exp.clone()), '__try0);
        Ok::<_, anyhow::Error>((json.clone(),))
    } {
        Ok((__try0_o0,)) => {
            json = __try0_o0;
        }
        Err(_) => {
            if failOnError.clone() {
                bail!("fail");
            }
            json = JSON::makeNull();
            json = JSON::addPair((literal!("$error")).clone(), JSON::makeString((ErrorExt::printCheckpointMessagesStr(false)).clone()), json.clone())?;
            json = JSON::addPair((literal!("value")).clone(), dumpJSONAbsynExpression(absynExp.clone())?, json.clone())?;
        }
    }
    ErrorExt::delCheckpoint((literal!("NFApi.dumpJSONAnnotationExp2")).clone());
    Ok(json)
}

pub fn dumpJSONSourceInfo(mut info: SourceInfo, mut dumpFilename: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    if dumpFilename.clone() {
        json = JSON::addPair((literal!("filename")).clone(), JSON::makeString((Testsuite::friendly(info.fileName.clone())?).clone()), json.clone())?;
    }
    json = JSON::addPair((literal!("lineStart")).clone(), JSON::makeInteger(info.lineNumberStart.clone()), json.clone())?;
    json = JSON::addPair((literal!("columnStart")).clone(), JSON::makeInteger(info.columnNumberStart.clone()), json.clone())?;
    json = JSON::addPair((literal!("lineEnd")).clone(), JSON::makeInteger(info.lineNumberEnd.clone()), json.clone())?;
    json = JSON::addPair((literal!("columnEnd")).clone(), JSON::makeInteger(info.columnNumberEnd.clone()), json.clone())?;
    if info.isReadOnly.clone() {
        json = JSON::addPair((literal!("readonly")).clone(), JSON::makeBoolean(true), json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONAbsynExpression(mut exp: Arc<Absyn::Exp>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut i: i32 = 0;
    let mut r: ArcStr = arcstr::literal!("");
    json = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::INTEGER { .. } => JSON::makeInteger(var_field!((*exp).value, Absyn::Exp::INTEGER).clone()),
        Deref @ Absyn::Exp::REAL { .. } => JSON::makeNumber(stringReal((var_field!((*exp).value, Absyn::Exp::REAL).clone()).clone())?),
        Deref @ Absyn::Exp::CREF { .. } => dumpJSONAbsynCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone())?,
        Deref @ Absyn::Exp::STRING { .. } => JSON::makeString((var_field!((*exp).value, Absyn::Exp::STRING).clone()).clone()),
        Deref @ Absyn::Exp::BOOL { .. } => JSON::makeBoolean(var_field!((*exp).value, Absyn::Exp::BOOL).clone()),
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::INTEGER { value: i }, op: Absyn::Operator::UMINUS { .. } } => JSON::makeInteger(-(i.clone())),
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::REAL { value: r }, op: Absyn::Operator::UMINUS { .. } } => JSON::makeNumber(-(stringReal((r.clone()).clone())?)),
        Deref @ Absyn::Exp::CALL { .. } => {
            json = JSON::makeNull();
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("call")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), dumpJSONAbsynCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone())?, json.clone())?;
            json = dumpJSONAbsynFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::CALL).clone(), json.clone())?;
            json.clone()
        },
        Deref @ Absyn::Exp::ARRAY { .. } => {
            json = JSON::emptyArray((var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone().len() as i32));
            for mut e in &*var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone() {
                let mut e = e.clone();
                json = JSON::addElement(dumpJSONAbsynExpression(e.clone())?, json.clone())?;
            }
            json.clone()
        },
        _ => JSON::makeString((Dump::printExpStr(AbsynUtil::stripCommentExpressions(exp.clone(), true)?)?).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONAbsynCref(mut cref: Arc<Absyn::ComponentRef>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json = JSON::makeString((Dump::printComponentRefStr(cref.clone())?).clone());
    Ok(json)
}

pub fn dumpJSONAbsynFunctionArgs(mut args: Arc<Absyn::FunctionArgs>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut json_args: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } => {
            if !(var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone().is_empty()) {
                json_args = JSON::makeNull();
                for mut arg in &*var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone() {
                    let mut arg = arg.clone();
                    json_args = JSON::addElement(dumpJSONAbsynExpression(arg.clone())?, json_args.clone())?;
                }
                json = JSON::addPair((literal!("args")).clone(), json_args.clone(), json.clone())?;
            }
            if !(var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone().is_empty()) {
                json_args = JSON::makeNull();
                for mut arg in &*var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone() {
                    let mut arg = arg.clone();
                    json_args = JSON::addPair((arg.argName.clone()).clone(), dumpJSONAbsynExpression(arg.argValue.clone())?, json_args.clone())?;
                }
                json = JSON::addPair((literal!("namedArgs")).clone(), json_args.clone(), json.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONImports(mut node: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut n: Arc<InstNode::InstNode> = node.clone();
    let mut imps: metamodelica::Array<Arc<Import::NFImport>>;
    let mut resolved_imps: Arc<metamodelica::List<Arc<Import::NFImport>>> = metamodelica::nil();
    let mut json_imp: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut json_imp_array: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    json_imp_array = JSON::makeNull();
    while !(InstNode::isEmpty(n.clone())) {
        imps = ClassTree::getImports(Class::classTree(InstNode::getClass(n.clone())?)?)?;
        if !(imps.clone().borrow().is_empty()) {
            resolved_imps = Import::resolveList(imps.clone());
            resolved_imps = resolved_imps.clone().reverse();
            for mut imp in &*resolved_imps.clone() {
                let mut imp = imp.clone();
                let () = (::match_deref::match_deref! { match &(imp.clone()) {
        Deref @ Import::RESOLVED_IMPORT { .. } => {
            json_imp = JSON::makeNull();
            json_imp = JSON::addPair((literal!("path")).clone(), dumpJSONPath(InstNode::fullPath(var_field!((*imp).node, Import::NFImport::RESOLVED_IMPORT).clone(), false)), json_imp.clone())?;
            if !(stringEmpty((var_field!((*imp).shortName, Import::NFImport::RESOLVED_IMPORT).clone()).clone())) {
                json_imp = JSON::addPair((literal!("shortName")).clone(), JSON::makeString((var_field!((*imp).shortName, Import::NFImport::RESOLVED_IMPORT).clone()).clone()), json_imp.clone())?;
            }
            json_imp_array = JSON::addElement(json_imp.clone(), json_imp_array.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
        }
        n = InstNode::parent(n.clone());
    }
    json = JSON::addPairNotNull((literal!("imports")).clone(), json_imp_array.clone(), json.clone())?;
    Ok(json)
}

pub fn dumpJSONEquations(mut sections: Arc<Sections::NFSections>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut connections: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut transitions: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut initial_states: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut context: i32 = 0;
    (connections, transitions, initial_states) = sortEquations(Sections::equations(sections.clone()), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
    context = InstContext::set(InstContext::CLASS.clone(), InstContext::RELAXED.clone());
    transitions = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (transitions.clone()).into_iter().cloned() {
            let __x = Typing::typeEquation(e.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    initial_states = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (initial_states.clone()).into_iter().cloned() {
            let __x = Typing::typeEquation(e.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    j = dumpJSONConnections(connections.clone(), scope.clone())?;
    json = JSON::addPairNotNull((literal!("connections")).clone(), j.clone(), json.clone())?;
    j = dumpJSONStateCalls(initial_states.clone(), scope.clone())?;
    json = JSON::addPairNotNull((literal!("initialStates")).clone(), j.clone(), json.clone())?;
    j = dumpJSONStateCalls(transitions.clone(), scope.clone())?;
    json = JSON::addPairNotNull((literal!("transitions")).clone(), j.clone(), json.clone())?;
    Ok(json)
}

pub fn sortEquations(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut connections: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut transitions: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut initialStates: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>)> {
    let mut connections: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = connections;
    let mut transitions: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = transitions;
    let mut initialStates: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = initialStates;
    for mut eq in &*equations.clone().reverse() {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::CONNECT { .. } => {
            connections = cons(eq.clone(), connections.clone());
            ()
        },
        Deref @ Equation::FOR { .. } => {
            (connections, transitions, initialStates) = sortEquations(var_field!((*eq).body, Equation::NFEquation::FOR).clone(), connections.clone(), transitions.clone(), initialStates.clone())?;
            ()
        },
        Deref @ Equation::IF { .. } => {
            for mut b in &*var_field!((*eq).branches, Equation::NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            (connections, transitions, initialStates) = sortEquations(var_field!((*b).body, Equation::Branch::Branch::BRANCH).clone(), connections.clone(), transitions.clone(), initialStates.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ Equation::NORETCALL { .. } => {
            if Expression::isCallNamed(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), (literal!("transition")).clone())? {
                transitions = cons(eq.clone(), transitions.clone());
            } else if Expression::isCallNamed(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), (literal!("initialState")).clone())? {
                initialStates = cons(eq.clone(), initialStates.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((connections, transitions, initialStates))
}

pub fn dumpJSONConnections(mut connections: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    for mut conn in &*connections.clone() {
        let mut conn = conn.clone();
        json = JSON::addElement(dumpJSONConnection(conn.clone(), scope.clone())?, json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONConnection(mut connEq: Arc<Equation::NFEquation>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut src: Arc<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(connEq.clone()) {
        Deref @ Equation::CONNECT { source: __pa0, rhs: __pa1, lhs: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    src = __pa0.clone();
    rhs = __pa1.clone();
    lhs = __pa2.clone();
    json = JSON::addPair((literal!("lhs")).clone(), Expression::toJSON(lhs.clone())?, json.clone())?;
    json = JSON::addPair((literal!("rhs")).clone(), Expression::toJSON(rhs.clone())?, json.clone())?;
    json = dumpJSONCommentAnnotation(ElementSource::getOptComment(src.clone())?, scope.clone(), json.clone(), metamodelica::nil(), false)?;
    Ok(json)
}

pub fn dumpJSONStateCalls(mut callEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    for mut eq in &*callEqs.clone() {
        let mut eq = eq.clone();
        json = JSON::addElement(dumpJSONStateCall(eq.clone(), scope.clone())?, json.clone())?;
    }
    Ok(json)
}

pub fn dumpJSONStateCall(mut callEq: Arc<Equation::NFEquation>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut call: Arc<Call::NFCall>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut src: Arc<DAE::ElementSource>;
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let () = (::match_deref::match_deref! { match &(callEq.clone()) {
        Deref @ Equation::NORETCALL { source: src, exp: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: args, .. } }, .. } => {
            j = JSON::emptyArray((args.clone().len() as i32));
            for mut arg in &*args.clone() {
                let mut arg = arg.clone();
                j = JSON::addElement(Expression::toJSON(arg.clone())?, j.clone())?;
            }
            json = JSON::addPair((literal!("arguments")).clone(), j.clone(), json.clone())?;
            json = dumpJSONCommentAnnotation(ElementSource::getOptComment(src.clone())?, scope.clone(), json.clone(), metamodelica::nil(), false)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONReplaceableElements(mut clsNode: Arc<InstNode::InstNode>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    cls_tree = Class::classTree(InstNode::getClass(clsNode.clone())?)?;
    let __range0 = ClassTree::getComponents(cls_tree.clone())?.borrow().iter().cloned().collect::<Vec<_>>();
    for mut c in __range0 {
        if InstNode::isReplaceable(c.clone())? {
            j = JSON::makeNull();
            j = JSON::addPair((literal!("name")).clone(), JSON::makeString((InstNode::name(c.clone())?).clone()), j.clone())?;
            j = JSON::addPair((literal!("type")).clone(), dumpJSONTypeName(InstNode::getType(c.clone())?)?, j.clone())?;
            json = JSON::addElement(j.clone(), json.clone())?;
        }
    }
    let __range1 = ClassTree::getClasses(cls_tree.clone())?.borrow().iter().cloned().collect::<Vec<_>>();
    for mut c in __range1 {
        if InstNode::isReplaceable(c.clone())? {
            json = JSON::addElement(JSON::makeString((InstNode::name(c.clone())?).clone()), json.clone())?;
        }
    }
    Ok(json)
}

pub fn dumpJSONSCodeMod(mut r#mod: Arc<SCode::Mod>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    j = dumpJSONSCodeMod_impl(r#mod.clone(), scope.clone(), false)?;
    json = JSON::addPairNotNull((literal!("modifiers")).clone(), j.clone(), json.clone())?;
    Ok(json)
}

pub fn dumpJSONSCodeMod_impl(mut r#mod: Arc<SCode::Mod>, mut scope: Arc<InstNode::InstNode>, mut isChoices: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut binding_json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut m = m.clone();
                json = JSON::addPair((m.ident.clone()).clone(), dumpJSONSCodeMod_impl(m.r#mod.clone(), scope.clone(), false)?, json.clone())?;
            }
            if SCodeUtil::finalBool(var_field!((*r#mod).finalPrefix, SCode::Mod::MOD).clone())? {
                json = JSON::addPair((literal!("final")).clone(), JSON::makeBoolean(true), json.clone())?;
            }
            if SCodeUtil::eachBool(var_field!((*r#mod).eachPrefix, SCode::Mod::MOD).clone())? {
                json = JSON::addPair((literal!("each")).clone(), JSON::makeBoolean(true), json.clone())?;
            }
            if isChoices.clone() && isSome(var_field!((*r#mod).comment, SCode::Mod::MOD).clone()) {
                json = JSON::addPair((literal!("comment")).clone(), JSON::makeString((Util::getOption(var_field!((*r#mod).comment, SCode::Mod::MOD).clone())?).clone()), json.clone())?;
            }
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                binding_json = JSON::makeString((Dump::printExpStr(AbsynUtil::stripCommentExpressions(Util::getOption(var_field!((*r#mod).binding, SCode::Mod::MOD).clone())?, true)?)?).clone());
                if JSON::isNull(json.clone()) {
                    json = binding_json.clone();
                } else {
                    json = JSON::addPair((literal!("$value")).clone(), binding_json.clone(), json.clone())?;
                }
            }
            ()
        },
        Deref @ SCode::Mod::REDECL { .. } => {
            if SCodeUtil::finalBool(var_field!((*r#mod).finalPrefix, SCode::Mod::REDECL).clone())? {
                json = JSON::addPair((literal!("final")).clone(), JSON::makeBoolean(true), json.clone())?;
            }
            if SCodeUtil::eachBool(var_field!((*r#mod).eachPrefix, SCode::Mod::REDECL).clone())? {
                json = JSON::addPair((literal!("each")).clone(), JSON::makeBoolean(true), json.clone())?;
            }
            json = JSON::addPair((literal!("$value")).clone(), dumpJSONSCodeElement(var_field!((*r#mod).element, SCode::Mod::REDECL).clone(), scope.clone(), JSON::makeNull())?, json.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONRedeclareType(mut element: Arc<SCode::Element>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut path: Arc<Path>;
    let mut context: i32 = 0;
    let mut cls: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let () = 'mc: {
        let __mc_input = element.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { .. } => {
                    let mut context: i32 = context.clone();
                    let mut cls: Arc<InstNode::InstNode> = cls.clone();
                    let mut path: Arc<Path>;
                    let mut json: Arc<JSON::JSON> = json.clone();
                    path = AbsynUtil::typeSpecPath(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone())?;
                    context = InstContext::set(InstContext::RELAXED.clone(), InstContext::FAST_LOOKUP.clone());
                    (cls, _) = Lookup::lookupName(path.clone(), scope.clone(), context.clone(), false)?;
                    json = JSON::addPair((literal!("$type")).clone(), dumpJSONNodePath(cls.clone(), false), json.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(json)
}

pub fn dumpJSONSCodeElement(mut element: Arc<SCode::Element>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    json = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("component")).clone() }), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone()), json.clone())?;
            json = JSON::addPair((literal!("type")).clone(), dumpJSONPath(AbsynUtil::typeSpecPath(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone())?), json.clone())?;
            json = JSON::addPairNotNull((literal!("dims")).clone(), dumpJSONDims(var_field!((*element).attributes, SCode::Element::COMPONENT).arrayDims.clone(), metamodelica::nil())?, json.clone())?;
            json = dumpJSONSCodeMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone())?;
            json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONAttributes(var_field!((*element).attributes, SCode::Element::COMPONENT).clone(), var_field!((*element).prefixes, SCode::Element::COMPONENT).clone(), scope.clone())?, json.clone())?;
            if isSome(var_field!((*element).condition, SCode::Element::COMPONENT).clone()) {
                json = JSON::addPair((literal!("condition")).clone(), dumpJSONAbsynExpression(Util::getOption(var_field!((*element).condition, SCode::Element::COMPONENT).clone())?)?, json.clone())?;
            }
            json = dumpJSONComment(var_field!((*element).comment, SCode::Element::COMPONENT).clone(), scope.clone(), json.clone(), true, true, false)?;
            json.clone()
        },
        Deref @ SCode::Element::CLASS { .. } => dumpJSONSCodeClass(element.clone(), Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), scope.clone(), false, json.clone())?,
        _ => json.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONSCodeType(mut path: Arc<Path>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut ty_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    match '__try0: {
        (ty_node, _) = unwrap_break_err!(Lookup::lookupName(path.clone(), scope.clone(), InstContext::set(InstContext::RELAXED.clone(), InstContext::FAST_LOOKUP.clone()), false), '__try0);
        json = unwrap_break_err!(JSON::addPair((literal!("type")).clone(), dumpJSONSCodeClass(InstNode::definition(ty_node.clone())?, ty_node.clone(), scope.clone(), false, JSON::makeNull())?, json.clone()), '__try0);
        Ok::<_, anyhow::Error>((json.clone(),))
    } {
        Ok((__try0_o0,)) => {
            json = __try0_o0;
        }
        Err(_) => {
            json = JSON::addPair((literal!("type")).clone(), dumpJSONPath(path.clone()), json.clone())?;
        }
    }
    Ok(json)
}

pub fn dumpJSONSCodeClass(mut element: Arc<SCode::Element>, mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut isRedeclare: bool, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut odims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> = None;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            json = JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("class")).clone() }), json.clone())?;
            if InstNode::isEmpty(node.clone()) || isRedeclare.clone() {
                json = JSON::addPair((literal!("name")).clone(), JSON::makeString((var_field!((*element).name, SCode::Element::CLASS).clone()).clone()), json.clone())?;
            } else {
                json = JSON::addPair((literal!("name")).clone(), dumpJSONNodeEnclosingPath(node.clone()), json.clone())?;
            }
            json = JSON::addPair((literal!("restriction")).clone(), JSON::makeString((SCodeDump::restrictionStringPP(var_field!((*element).restriction, SCode::Element::CLASS).clone())?).clone()), json.clone())?;
            json = JSON::addPairNotNull((literal!("prefixes")).clone(), dumpJSONClassPrefixes(element.clone(), scope.clone())?, json.clone())?;
            json = dumpJSONSCodeClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), scope.clone(), isRedeclare.clone(), json.clone())?;
            json = dumpJSONComment(var_field!((*element).cmt, SCode::Element::CLASS).clone(), scope.clone(), json.clone(), true, !(isRedeclare.clone()), false)?;
            if isRedeclare.clone() {
                json = dumpJSONCommentAnnotation(Some(var_field!((*element).cmt, SCode::Element::CLASS).clone()), scope.clone(), json.clone(), list![(literal!("Dialog")).clone(), (literal!("choices")).clone(), (literal!("choicesAllMatching")).clone()], false)?;
            }
            if !(isRedeclare.clone()) {
                json = dumpJSONSCodeTypeExtends(node.clone(), scope.clone(), json.clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(json)
}

pub fn dumpJSONSCodeTypeExtends(mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut json: Arc<JSON::JSON>) -> Arc<JSON::JSON> {
    let mut json: Arc<JSON::JSON> = json;
    let mut expanded_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut json_elements: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut json_ext: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    if InstNode::isEmpty(node.clone()) {
        return json;
    }
    if '__try0: {
        expanded_node = unwrap_break_err!(Inst::expand(node.clone(), InstContext::RELAXED.clone()), '__try0);
        exts = ClassTree::getExtends(Class::classTree(InstNode::getClass(expanded_node.clone()).unwrap()).unwrap());
        if !(exts.clone().borrow().is_empty()) {
            json_elements = JSON::makeNull();
            let __range1 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range1 {
                json_ext = JSON::makeNull();
                json_ext = unwrap_break_err!(JSON::addPair((literal!("$kind")).clone(), Arc::new(JSON::JSON::STRING { r#str: (literal!("extends")).clone() }), json_ext.clone()), '__try0);
                json_ext = unwrap_break_err!(JSON::addPair((literal!("baseClass")).clone(), dumpJSONSCodeClass(InstNode::definition(ext.clone()).unwrap(), ext.clone(), scope.clone(), false, JSON::makeNull()).unwrap(), json_ext.clone()), '__try0);
                json_elements = unwrap_break_err!(JSON::addElement(json_ext.clone(), json_elements.clone()), '__try0);
            }
            json = unwrap_break_err!(JSON::addPair((literal!("elements")).clone(), json_elements.clone(), json.clone()), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    json
}

pub fn dumpJSONSCodeClassDef(mut classDef: Arc<SCode::ClassDef>, mut scope: Arc<InstNode::InstNode>, mut qualifyPath: bool, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = json;
    let mut path: Arc<Path>;
    let mut odims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> = None;
    let mut derivedNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: odims, path }, .. } => {
            if qualifyPath.clone() {
                match '__try0: {
                    (derivedNode, _) = unwrap_break_err!(Lookup::lookupName(path.clone(), scope.clone(), InstContext::RELAXED.clone(), false), '__try0);
                    json = unwrap_break_err!(JSON::addPair((literal!("baseClass")).clone(), dumpJSONNodeEnclosingPath(derivedNode.clone()), json.clone()), '__try0);
                    Ok::<_, anyhow::Error>((derivedNode.clone(), json.clone()))
                } {
                    Ok((__try0_o0, __try0_o1)) => {
                        derivedNode = __try0_o0;
                        json = __try0_o1;
                    }
                    Err(_) => {
                        bail!("try/else: outputs not set in else branch");
                    }
                }
            } else {
                json = JSON::addPair((literal!("baseClass")).clone(), dumpJSONPath(path.clone()), json.clone())?;
            }
            if isSome(odims.clone()) {
                json = JSON::addPairNotNull((literal!("dims")).clone(), dumpJSONDims(Util::getOption(odims.clone())?, metamodelica::nil())?, json.clone())?;
            }
            json = dumpJSONSCodeMod(var_field!((*classDef).modifications, SCode::ClassDef::DERIVED).clone(), scope.clone(), json.clone())?;
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            json = dumpJSONSCodeMod(var_field!((*classDef).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), scope.clone(), json.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn dumpJSONChoicesAnnotation(mut mods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo, mut failOnError: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    let mut smod: Arc<SCode::SubMod>;
    let mut choices: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut others: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut choices_mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    choices = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut m in (mods.clone()).into_iter().cloned() {
            if !(m.ident.clone() == literal!("choice")) { continue; }
            let __x = m.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    others = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut m in (mods.clone()).into_iter().cloned() {
            if !(m.ident.clone() != literal!("choice")) { continue; }
            let __x = m.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if !(choices.clone().is_empty()) {
        j = JSON::emptyArray((choices.clone().len() as i32));
        for mut m in &*choices.clone() {
            let mut m = m.clone();
            m = (::match_deref::match_deref! { match &(m.r#mod.clone()) {
        Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::List::Cons { head: smod, tail: Deref @ metamodelica::List::Nil }, binding: None, .. } => smod.clone(),
        _ => m.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            j = JSON::addElement(dumpJSONSCodeMod_impl(m.r#mod.clone(), scope.clone(), true)?, j.clone())?;
        }
        json = JSON::addPair((literal!("choice")).clone(), j.clone(), json.clone())?;
    }
    for mut m in &*others.clone() {
        let mut m = m.clone();
        json = dumpJSONAnnotationSubMod(m.clone(), scope.clone(), failOnError.clone(), json.clone())?;
    }
    Ok(json)
}

pub fn modifierToJSON(mut modifier: ArcStr, mut prettyPrint: bool) -> Result<Arc<Values::Value>> {
    let mut jsonString: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut amod: Arc<Absyn::Modification>;
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut json: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let __pa0 = ::match_deref::match_deref! { match &(Parser::stringMod(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("dummy")); __mm_s.push_str(&*modifier.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("<internal>")).clone())?) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__pa0), .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    amod = __pa0.clone();
    smod = AbsynToSCode::translateMod(Some(amod.clone()), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
    json = dumpJSONSCodeMod_impl(smod.clone(), Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), false)?;
    jsonString = Arc::new(Values::Value::STRING { string: (JSON::toString(json.clone(), prettyPrint.clone())?).clone() });
    Ok(jsonString)
}

#[derive(Clone, Debug, PartialEq)]
pub struct MoveEnv {
    pub scope: Arc<InstNode::InstNode>,
    pub destinationPath: Arc<Path>,
}

pub type MOVE_ENV = MoveEnv;


pub fn updateMovedClassPaths(mut cls: Arc<Absyn::Class>, mut clsPath: Arc<Path>, mut destination: Absyn::Within) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut context: i32 = 0;
    let mut top: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut env: MoveEnv;
    let mut dest_path: Arc<Path>;
    (_, top) = mkTop(SymbolTable::getAbsyn(), (AbsynUtil::pathString(clsPath.clone(), (literal!(".")).clone(), true, false)?).clone())?;
    cls_node = Inst::lookupRootClass(clsPath.clone(), top.clone(), FAST_CONTEXT.clone())?;
    Inst::expand(cls_node.clone(), FAST_CONTEXT.clone())?;
    dest_path = (match destination.clone() {
        Absyn::Within::WITHIN { .. } => AbsynUtil::suffixPath(var_field!(destination.path, Absyn::Within::WITHIN).clone(), (InstNode::name(cls_node.clone())?).clone())?,
        _ => Arc::new(Path::IDENT { name: (InstNode::name(cls_node.clone())?).clone() }),
    });
    env = MoveEnv { scope: cls_node.clone(), destinationPath: dest_path.clone() };
    assign_field!(cls.body = updateMovedClassDef(cls.body.clone(), env.clone())?);
    Ok(cls)
}

pub fn updateMovedClass(mut cls: Arc<Absyn::Class>, mut env: MoveEnv) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls_env: MoveEnv;
    if classHasScope(cls.clone()) {
        (cls_node, _) = Lookup::lookupLocalSimpleName((cls.name.clone()).clone(), env.scope.clone())?;
        Inst::expand(cls_node.clone(), FAST_CONTEXT.clone())?;
        cls_env = MoveEnv { scope: cls_node.clone(), destinationPath: AbsynUtil::suffixPath(env.destinationPath.clone(), (cls.name.clone()).clone())? };
    } else {
        cls_env = env.clone();
    }
    assign_field!(cls.body = updateMovedClassDef(cls.body.clone(), cls_env.clone())?);
    Ok(cls)
}

pub fn classHasScope(mut cls: Arc<Absyn::Class>) -> bool {
    let mut hasScope: bool = false;
    hasScope = (::match_deref::match_deref! { match &(cls.body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => true,
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasScope
}

pub fn updateMovedClassDef(mut cdef: Arc<Absyn::ClassDef>, mut env: MoveEnv) -> Result<Arc<Absyn::ClassDef>> {
    let mut cdef: Arc<Absyn::ClassDef> = cdef;
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS;
                classParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = updateMovedClassPart(p.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = updateMovedAnnotation(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED;
                typeSpec = updateMovedTypeSpec(var_field!((*cdef).typeSpec, Absyn::ClassDef::DERIVED).clone(), env.clone())?,
                attributes = updateMovedElementAttributes(var_field!((*cdef).attributes, Absyn::ClassDef::DERIVED).clone(), env.clone())?,
                arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).arguments, Absyn::ClassDef::DERIVED).clone()).into_iter().cloned() {
            let __x = updateMovedElementArg(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = updateMovedCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::DERIVED).clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS;
                modifications = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).modifications, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = updateMovedElementArg(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                parts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = updateMovedClassPart(p.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = updateMovedAnnotation(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::ClassDef::PDER { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PDER;
                functionName = updateMovedPath(var_field!((*cdef).functionName, Absyn::ClassDef::PDER).clone(), env.clone())?,
                comment = updateMovedCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::PDER).clone(), env.clone())?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cdef)
}

pub fn updateMovedClassPart(mut part: Arc<Absyn::ClassPart>, mut env: MoveEnv) -> Result<Arc<Absyn::ClassPart>> {
    let mut part: Arc<Absyn::ClassPart> = part;
    let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut i in (var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone()).into_iter().cloned() {
            let __x = updateMovedElementItem(i.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut i in (var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone()).into_iter().cloned() {
            let __x = updateMovedElementItem(i.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = updateMovedEquationItems(var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone(), env.clone()));
            ()
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = updateMovedEquationItems(var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone(), env.clone()));
            ()
        },
        Deref @ Absyn::ClassPart::ALGORITHMS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::ALGORITHMS; contents = updateMovedAlgorithmItems(var_field!((*part).contents, Absyn::ClassPart::ALGORITHMS).clone(), env.clone()));
            ()
        },
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALALGORITHMS; contents = updateMovedAlgorithmItems(var_field!((*part).contents, Absyn::ClassPart::INITIALALGORITHMS).clone(), env.clone()));
            ()
        },
        Deref @ Absyn::ClassPart::EXTERNAL { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EXTERNAL; annotation_ = updateMovedAnnotationOpt(var_field!((*part).annotation_, Absyn::ClassPart::EXTERNAL).clone(), env.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(part)
}

pub fn updateMovedElementItem(mut item: Arc<Absyn::ElementItem>, mut env: MoveEnv) -> Result<Arc<Absyn::ElementItem>> {
    let mut item: Arc<Absyn::ElementItem> = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = updateMovedElement(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone(), env.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

pub fn updateMovedElement(mut element: Arc<Absyn::Element>, mut env: MoveEnv) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = updateMovedElementSpec(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), env.clone())?);
            if isSome(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone()) {
                assign_variant_field!(element => Absyn::Element::ELEMENT; constrainClass = Some(updateMovedConstrainClass(Util::getOption(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone())?, env.clone())?));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub fn updateMovedConstrainClass(mut cc: Arc<Absyn::ConstrainClass>, mut env: MoveEnv) -> Result<Arc<Absyn::ConstrainClass>> {
    let mut cc: Arc<Absyn::ConstrainClass> = cc;
    assign_field!(
        cc.elementSpec = updateMovedElementSpec(cc.elementSpec.clone(), env.clone())?,
        cc.comment = updateMovedCommentOpt(cc.comment.clone(), env.clone())?
    );
    Ok(cc)
}

pub fn updateMovedElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut env: MoveEnv) -> Result<Arc<Absyn::ElementSpec>> {
    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = updateMovedClass(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), env.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS;
                path = updateMovedPath(var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone(), env.clone())?,
                elementArg = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone()).into_iter().cloned() {
            let __x = updateMovedElementArg(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                annotationOpt = updateMovedAnnotationOpt(var_field!((*spec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone(), env.clone())?
            );
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS;
                attributes = updateMovedElementAttributes(var_field!((*spec).attributes, Absyn::ElementSpec::COMPONENTS).clone(), env.clone())?,
                typeSpec = updateMovedTypeSpec(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS).clone(), env.clone())?,
                components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
            let __x = updateMovedComponentItem(c.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(spec)
}

pub fn updateMovedElementAttributes(mut attr: Absyn::ElementAttributes, mut env: MoveEnv) -> Result<Absyn::ElementAttributes> {
    let mut attr: Absyn::ElementAttributes = attr;
    if !(attr.arrayDim.clone().is_empty()) {
        attr.arrayDim = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (attr.arrayDim.clone()).into_iter().cloned() {
            let __x = updateMovedSubscript(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    Ok(attr)
}

pub fn updateMovedElementArg(mut arg: Arc<Absyn::ElementArg>, mut env: MoveEnv) -> Result<Arc<Absyn::ElementArg>> {
    let mut arg: Arc<Absyn::ElementArg> = arg;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            if isSome(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(updateMovedModification(Util::getOption(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone())?, env.clone())?));
            }
            ()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION; elementSpec = updateMovedElementSpec(var_field!((*arg).elementSpec, Absyn::ElementArg::REDECLARATION).clone(), env.clone())?);
            if isSome(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone()) {
                assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION; constrainClass = Some(updateMovedConstrainClass(Util::getOption(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone())?, env.clone())?));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub fn updateMovedModification(mut r#mod: Arc<Absyn::Modification>, mut env: MoveEnv) -> Result<Arc<Absyn::Modification>> {
    let mut r#mod: Arc<Absyn::Modification> = r#mod;
    let mut eq_mod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    assign_field!(r#mod.elementArgLst = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (r#mod.elementArgLst.clone()).into_iter().cloned() {
            let __x = updateMovedElementArg(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    eq_mod = r#mod.eqMod.clone();
    let () = (::match_deref::match_deref! { match &(eq_mod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => {
            assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = updateMovedExp(var_field!((*eq_mod).exp, Absyn::EqMod::EQMOD).clone(), env.clone())?);
            assign_field!(r#mod.eqMod = eq_mod.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn updateMovedComponentItem(mut item: Arc<Absyn::ComponentItem>, mut env: MoveEnv) -> Result<Arc<Absyn::ComponentItem>> {
    let mut item: Arc<Absyn::ComponentItem> = item;
    assign_field!(item.component = updateMovedComponent(item.component.clone(), env.clone())?);
    if isSome(item.condition.clone()) {
        assign_field!(item.condition = Some(updateMovedExp(Util::getOption(item.condition.clone())?, env.clone())?));
    }
    Ok(item)
}

pub fn updateMovedComponent(mut component: Absyn::Component, mut env: MoveEnv) -> Result<Absyn::Component> {
    let mut component: Absyn::Component = component;
    if !(component.arrayDim.clone().is_empty()) {
        component.arrayDim = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut d in (component.arrayDim.clone()).into_iter().cloned() {
            let __x = updateMovedSubscript(d.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    if isSome(component.modification.clone()) {
        component.modification = Some(updateMovedModification(Util::getOption(component.modification.clone())?, env.clone())?);
    }
    Ok(component)
}

pub fn updateMovedEquationItems(mut items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut env: MoveEnv) -> Arc<metamodelica::List<Arc<Absyn::EquationItem>>> {
    let mut items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = items;
    (items, _) = AbsynUtil::traverseEquationItemListBidir(items.clone(), Arc::new(updateMovedExp_traverser), Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, Arc<Absyn::Exp>, _)), env.clone());
    items
}

pub fn updateMovedAlgorithmItems(mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut env: MoveEnv) -> Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> {
    let mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = items;
    (items, _) = AbsynUtil::traverseAlgorithmItemListBidir(items.clone(), Arc::new(updateMovedExp_traverser), Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, Arc<Absyn::Exp>, _)), env.clone());
    items
}

pub fn updateMovedTypeSpec(mut ty: Arc<Absyn::TypeSpec>, mut env: MoveEnv) -> Result<Arc<Absyn::TypeSpec>> {
    let mut ty: Arc<Absyn::TypeSpec> = ty;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = updateMovedPath(var_field!((*ty).path, Absyn::TypeSpec::TPATH).clone(), env.clone())?);
            if isSome(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone()) {
                assign_variant_field!(ty => Absyn::TypeSpec::TPATH; arrayDim = Some({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (Util::getOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone())?).into_iter().cloned() {
            let __x = updateMovedSubscript(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            }
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX; path = updateMovedPath(var_field!((*ty).path, Absyn::TypeSpec::TCOMPLEX).clone(), env.clone())?);
            if isSome(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone()) {
                assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX; arrayDim = Some({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (Util::getOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone())?).into_iter().cloned() {
            let __x = updateMovedSubscript(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ty)
}

pub fn updateMovedPath(mut path: Arc<Path>, mut env: MoveEnv) -> Result<Arc<Path>> {
    let mut path: Arc<Path> = path;
    let mut qualified_path: Arc<Path>;
    let mut opt_path: Option<Arc<Path>> = None;
    if let Ok(__iflet0) = Lookup::lookupSimpleNameRootPath((AbsynUtil::pathFirstIdent(path.clone())?).clone(), env.scope.clone(), FAST_CONTEXT.clone()) {
        qualified_path = __iflet0;
    } else {
        return Ok(path);
    }
    if AbsynUtil::pathIsFullyQualified(qualified_path.clone()) {
        qualified_path = AbsynUtil::makeNotFullyQualified(qualified_path.clone());
        if AbsynUtil::pathIsIdent(qualified_path.clone()) && AbsynUtil::pathFirstIdent(qualified_path.clone())? == AbsynUtil::pathFirstIdent(env.destinationPath.clone())? {
            path = AbsynUtil::pathRest(path.clone())?;
        } else {
            opt_path = AbsynUtil::pathStripSamePrefix(qualified_path.clone(), env.destinationPath.clone())?;
            if isSome(opt_path.clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(opt_path.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                qualified_path = __pa1.clone();
                if AbsynUtil::pathIsQual(path.clone()) {
                    path = AbsynUtil::joinPaths(qualified_path.clone(), AbsynUtil::pathRest(path.clone())?)?;
                } else {
                    path = qualified_path.clone();
                }
            }
        }
    }
    Ok(path)
}

pub fn updateMovedCommentOpt(mut cmt: Option<Arc<Absyn::Comment>>, mut env: MoveEnv) -> Result<Option<Arc<Absyn::Comment>>> {
    let mut cmt: Option<Arc<Absyn::Comment>> = cmt;
    if isSome(cmt.clone()) {
        cmt = Some(updateMovedComment(Util::getOption(cmt.clone())?, env.clone())?);
    }
    Ok(cmt)
}

pub fn updateMovedComment(mut cmt: Arc<Absyn::Comment>, mut env: MoveEnv) -> Result<Arc<Absyn::Comment>> {
    let mut cmt: Arc<Absyn::Comment> = cmt;
    assign_field!(cmt.annotation_ = updateMovedAnnotationOpt(cmt.annotation_.clone(), env.clone())?);
    Ok(cmt)
}

pub fn updateMovedAnnotationOpt(mut ann: Option<Arc<Absyn::Annotation>>, mut env: MoveEnv) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut ann: Option<Arc<Absyn::Annotation>> = ann;
    if isSome(ann.clone()) {
        ann = Some(updateMovedAnnotation(Util::getOption(ann.clone())?, env.clone())?);
    }
    Ok(ann)
}

pub fn updateMovedAnnotation(mut ann: Arc<Absyn::Annotation>, mut env: MoveEnv) -> Result<Arc<Absyn::Annotation>> {
    let mut ann: Arc<Absyn::Annotation> = ann;
    assign_field!(ann.elementArgs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (ann.elementArgs.clone()).into_iter().cloned() {
            let __x = updateMovedElementArg(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(ann)
}

pub fn updateMovedSubscript(mut sub: Arc<Absyn::Subscript>, mut env: MoveEnv) -> Result<Arc<Absyn::Subscript>> {
    let mut sub: Arc<Absyn::Subscript> = sub;
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = updateMovedExp(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), env.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sub)
}

pub fn updateMovedExp(mut exp: Arc<Absyn::Exp>, mut env: MoveEnv) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(exp.clone(), Arc::new(updateMovedExp_traverser), env.clone())?;
    Ok(exp)
}

pub fn updateMovedExp_traverser(mut exp: Arc<Absyn::Exp>, mut env: MoveEnv) -> Result<(Arc<Absyn::Exp>, MoveEnv)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut env: MoveEnv = env;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = updateMovedCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), env.clone())?);
            ()
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CALL; function_ = updateMovedCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone(), env.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, env))
}

pub fn updateMovedCref(mut cref: Arc<Absyn::ComponentRef>, mut env: MoveEnv) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut qualified_path: Arc<Path>;
    let mut qualified_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut opt_path: Option<Arc<Path>> = None;
    if AbsynUtil::crefIsFullyQualified(cref.clone()) || AbsynUtil::crefIsWild(cref.clone()) {
        return Ok(cref);
    }
    if let Ok(__iflet0) = Lookup::lookupSimpleNameRootPath((AbsynUtil::crefFirstIdent(cref.clone())?).clone(), env.scope.clone(), FAST_CONTEXT.clone()) {
        qualified_path = __iflet0;
    } else {
        return Ok(cref);
    }
    if AbsynUtil::pathIsFullyQualified(qualified_path.clone()) {
        qualified_path = AbsynUtil::makeNotFullyQualified(qualified_path.clone());
        if AbsynUtil::pathIsIdent(qualified_path.clone()) && AbsynUtil::pathFirstIdent(qualified_path.clone())? == AbsynUtil::pathFirstIdent(env.destinationPath.clone())? {
            cref = AbsynUtil::crefStripFirst(cref.clone())?;
        } else {
            opt_path = AbsynUtil::pathStripSamePrefix(qualified_path.clone(), env.destinationPath.clone())?;
            if isSome(opt_path.clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(opt_path.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                qualified_path = __pa1.clone();
                qualified_cref = AbsynUtil::pathToCref(qualified_path.clone())?;
                if AbsynUtil::crefIsQual(cref.clone()) {
                    cref = AbsynUtil::joinCrefs(qualified_cref.clone(), AbsynUtil::crefStripFirst(cref.clone())?)?;
                } else {
                    cref = qualified_cref.clone();
                }
            }
        }
    }
    Ok(cref)
}

pub fn translateResidualsDAE(mut path: Arc<Path>, mut fileNamePrefix: ArcStr) -> Result<bool> {
    let mut success: bool = true;
    let mut disable_single_flow_eq: bool = false;
    let mut non_std_flags: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut flat_model: Arc<FlatModel::NFFlatModel>;
    let mut funcs: Arc<Flatten::FunctionTreeImpl::Tree> = Arc::new(Flatten::FunctionTreeImpl::Tree::EMPTY);
    let mut simSettings: Option<SimCode::SimulationSettings> = None;
    disable_single_flow_eq = FlagsUtil::set(Flags::DISABLE_SINGLE_FLOW_EQ.clone(), true)?;
    non_std_flags = FlagsUtil::appendConfigStringList(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("implicitParameterStartAttribute")).clone())?;
    if '__try0: {
        (flat_model, funcs, _) = unwrap_break_err!(CevalScriptBackend::runFrontEndNF(path.clone(), false, false), '__try0);
        (flat_model, funcs) = unwrap_break_err!(InstUtil::createExtractorModel(flat_model.clone(), funcs.clone()), '__try0);
        unwrap_break_err!(InstUtil::dumpFlatModelDebug((literal!("translateResidualsDAE")).clone(), flat_model.clone(), funcs.clone()), '__try0);
        simSettings = Some(unwrap_break_err!(CevalScriptBackend::convertSimulationOptionsToSimCode(CevalScriptBackend::buildSimulationOptionsFromModelExperimentAnnotation(path.clone(), (fileNamePrefix.clone()).clone(), None)?), '__try0));
        unwrap_break_err!(SimCodeMain::translateModelCallBackend(flat_model.clone(), funcs.clone(), path.clone(), (fileNamePrefix.clone()).clone(), true, simSettings.clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    FlagsUtil::setConfigStringList(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), non_std_flags.clone())?;
    FlagsUtil::set(Flags::DISABLE_SINGLE_FLOW_EQ.clone(), disable_single_flow_eq.clone())?;
    Ok(success)
}

