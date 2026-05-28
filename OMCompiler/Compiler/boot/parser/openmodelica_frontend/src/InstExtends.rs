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

use crate::FCore;
use crate::FGraph;
use crate::InnerOuter;
use crate::Inst;
use crate::InstUtil;
use crate::Lookup;
use crate::Mod;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::SCodeInstUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::AvlSetString;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

fn instExtendsList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inLocalElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inElementsFromExtendsScope: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inState: ClassInf::State, mut inClassName: ArcStr, mut inImpl: bool, mut inPartialInst: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<DAE::Mod>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::Comment>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    let mut outMod: Arc<DAE::Mod> = inMod.clone();
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = metamodelica::nil();
    let mut outNormalEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outInitialEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outNormalAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outInitialAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
    let mut duplicates: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut duplicateUnparseStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    duplicates = List::sortedDuplicates(List::sort(inElementsFromExtendsScope.clone(), (std::sync::Arc::new(SCodeUtil::elementEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<SCode::Element>) -> Result<bool> + 'static>))?, (std::sync::Arc::new(SCodeUtil::elementEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<SCode::Element>) -> Result<bool> + 'static>))?;
    if Config::acceptMetaModelicaGrammar()? {
        duplicates = List::filterOnFalse(duplicates.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isTypeVar, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>));
    }
    if !(duplicates.clone().is_empty()) {
        duplicateUnparseStrings = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (duplicates.clone()).into_iter().cloned() {
            let __x = SCodeDump::unparseElementStr(i.clone(), SCodeDump::defaultOptions.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if (duplicates.clone().len() as i32) > 1 {
            Error::addMultiSourceMessage(Error::DUPLICATE_VARIABLE_ERROR.clone(), duplicateUnparseStrings.clone(), List::map(duplicates.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::elementInfo, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<SourceInfo> + 'static>)))?;
        } else {
            Error::addSourceMessage(Error::DUPLICATE_VARIABLE_ERROR.clone(), duplicateUnparseStrings.clone(), SCodeUtil::elementInfo(listHead(duplicates.clone())?))?;
        }
        bail!("fail");
    }
    for mut el in &*inLocalElements.clone().reverse() {
        let mut el = el.clone();
        let () = 'mc: {
        let __mc_input = el.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { .. } => {
                    let mut cn: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::makeNotFullyQualified(var_field!((*el).baseClassPath, SCode::Element::EXTENDS).clone())) {
                        Deref @ Absyn::Path::IDENT { name: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cn = __pa0.clone();
                    let true = (InstUtil::isBuiltInClass((cn.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { .. } => {
                    let mut cn: ArcStr = arcstr::literal!("");
                    let mut bc_str: ArcStr = arcstr::literal!("");
                    let mut scope_str: ArcStr = arcstr::literal!("");
                    let mut base_first_id: ArcStr = arcstr::literal!("");
                    let mut emod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut eq_name: bool = false;
                    let mut ocls: Option<Arc<SCode::Element>> = None;
                    let mut cls: Arc<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut encf: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut els1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut rest_els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut import_els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut cdef_els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut clsext_els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut els2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = metamodelica::nil();
                    let mut eq1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut ieq1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut eq2: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut ieq2: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut alg1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut ialg1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut alg2: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut ialg2: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut comments1: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    let mut comments2: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
                    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut tree: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
                    let mut cacheArr: metamodelica::Array<FCore::Cache>;
                    let mut htHasEntries: bool = false;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outNormalEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = outNormalEqs.clone();
                    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = outIH.clone();
                    let mut outInitialEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = outInitialEqs.clone();
                    let mut outMod: Arc<DAE::Mod> = outMod.clone();
                    let mut outInitialAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = outInitialAlgs.clone();
                    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = outComments.clone();
                    let mut outNormalAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = outNormalAlgs.clone();
                    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = outElements.clone();
                    emod = InstUtil::chainRedeclares(outMod.clone(), var_field!((*el).modifications, SCode::Element::EXTENDS).clone())?;
                    base_first_id = (AbsynUtil::pathFirstIdent(var_field!((*el).baseClassPath, SCode::Element::EXTENDS).clone())?).clone();
                    eq_name = stringEq((inClassName.clone()).clone(), (base_first_id.clone()).clone()) && AbsynUtil::pathEqual(ClassInfUtil::getStateName(inState.clone()), AbsynUtil::joinPaths(FGraph::getGraphName(outEnv.clone())?, AbsynUtil::makeIdentPathFromString((base_first_id.clone()).clone()))?);
                    (outCache, ocls, cenv) = lookupBaseClass(var_field!((*el).baseClassPath, SCode::Element::EXTENDS).clone(), eq_name.clone(), (inClassName.clone()).clone(), outEnv.clone(), outCache.clone())?;
                    if isSome(ocls.clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(ocls.clone()) {
                            Some(__pa0) => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        cls = __pa0.clone();
                        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(cls.clone()) {
                            Deref @ SCode::Element::CLASS { cmt: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        cmt = __pa1.clone();
                        encf = __pa2.clone();
                        cn = __pa3.clone();
                    } else {
                        if Flags::getConfigBool(Flags::PERMISSIVE.clone())? {
                            bc_str = (AbsynUtil::pathString(var_field!((*el).baseClassPath, SCode::Element::EXTENDS).clone(), (literal!(".")).clone(), true, false)?).clone();
                            scope_str = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                            Error::addSourceMessage(Error::LOOKUP_BASECLASS_ERROR.clone(), list![(bc_str.clone()).clone(), (scope_str.clone()).clone()], var_field!((*el).info, SCode::Element::EXTENDS).clone())?;
                        }
                        bail!("fail");
                    }
                    (outCache, cenv, outIH, els1, eq1, ieq1, alg1, ialg1, r#mod, comments1) = instDerivedClasses(outCache.clone(), cenv.clone(), outIH.clone(), outMod.clone(), inPrefix.clone(), cls.clone(), inImpl.clone(), var_field!((*el).info, SCode::Element::EXTENDS).clone())?;
                    els1 = updateElementListVisibility(els1.clone(), var_field!((*el).visibility, SCode::Element::EXTENDS).clone());
                    tree = AvlSetString::new();
                    tree = getLocalIdentList(InstUtil::constantAndParameterEls(inElementsFromExtendsScope.clone())?, tree.clone(), (std::sync::Arc::new(getLocalIdentElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>));
                    tree = getLocalIdentList(InstUtil::constantAndParameterEls(els1.clone())?, tree.clone(), (std::sync::Arc::new(getLocalIdentElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>));
                    cacheArr = arrayCreate(1, outCache.clone());
                    emod = fixModifications(cacheArr.clone(), inEnv.clone(), emod.clone(), tree.clone())?;
                    cenv = FGraph::openScope(cenv.clone(), encf.clone(), (cn.clone()).clone(), FGraph::classInfToScopeType(inState.clone()))?;
                    (import_els, cdef_els, clsext_els, rest_els) = InstUtil::splitEltsNoComponents(els1.clone())?;
                    (outCache, cenv, outIH) = InstUtil::addClassdefsToEnv(outCache.clone(), cenv.clone(), outIH.clone(), inPrefix.clone(), import_els.clone(), inImpl.clone(), None, false)?;
                    (outCache, cenv, outIH) = InstUtil::addClassdefsToEnv(outCache.clone(), cenv.clone(), outIH.clone(), inPrefix.clone(), cdef_els.clone(), inImpl.clone(), Some(r#mod.clone()), false)?;
                    rest_els = SCodeInstUtil::addRedeclareAsElementsToExtends(rest_els.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (rest_els.clone()).into_iter().cloned() {
                    if !(SCodeUtil::isRedeclareElement(e.clone())) { continue; }
                    let __x = e.clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                    outMod = Mod::elabUntypedMod(emod.clone(), Mod::ModScope::EXTENDS { path: var_field!((*el).baseClassPath, SCode::Element::EXTENDS).clone() })?;
                    outMod = Mod::merge(r#mod.clone(), outMod.clone(), (literal!("")).clone(), false)?;
                    (outCache, _, outIH, _, els2, eq2, ieq2, alg2, ialg2, comments2) = instExtendsAndClassExtendsList2(outCache.clone(), cenv.clone(), outIH.clone(), outMod.clone(), inPrefix.clone(), rest_els.clone(), clsext_els.clone(), els1.clone(), inState.clone(), (inClassName.clone()).clone(), inImpl.clone(), inPartialInst.clone())?;
                    tree = AvlSetString::new();
                    tree = getLocalIdentList(els2.clone(), tree.clone(), (std::sync::Arc::new(getLocalIdentElementTpl) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>, bool), Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>));
                    tree = getLocalIdentList(cdef_els.clone(), tree.clone(), (std::sync::Arc::new(getLocalIdentElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>));
                    tree = getLocalIdentList(import_els.clone(), tree.clone(), (std::sync::Arc::new(getLocalIdentElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>));
                    htHasEntries = !(AvlSetString::isEmpty(tree.clone()));
                    {let _arr = cacheArr.clone(); _arr.borrow_mut()[(1-1) as usize] = outCache.clone(); _arr};
                    if htHasEntries.clone() {
                        els2 = fixList(cacheArr.clone(), cenv.clone(), els2.clone(), tree.clone(), (std::sync::Arc::new(fixLocalIdent) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, (Arc<SCode::Element>, Arc<DAE::Mod>, bool), Arc<AvlSetString::Tree>) -> Result<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)> + 'static>));
                    }
                    outElements = listAppend(els2.clone(), outElements.clone());
                    outNormalEqs = List::unionAppendListOnTrue(eq2.clone().reverse(), outNormalEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                    outInitialEqs = List::unionAppendListOnTrue(ieq2.clone().reverse(), outInitialEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                    outNormalAlgs = List::unionAppendListOnTrue(alg2.clone().reverse(), outNormalAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                    outInitialAlgs = List::unionAppendListOnTrue(ialg2.clone().reverse(), outInitialAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                    outComments = listAppend(comments1.clone(), listAppend(comments2.clone(), cons(cmt.clone(), outComments.clone())));
                    if !(inPartialInst.clone()) {
                        if htHasEntries.clone() {
                            eq1 = fixList(cacheArr.clone(), cenv.clone(), eq1.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
                            ieq1 = fixList(cacheArr.clone(), cenv.clone(), ieq1.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
                            alg1 = fixList(cacheArr.clone(), cenv.clone(), alg1.clone(), tree.clone(), (std::sync::Arc::new(fixAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::AlgorithmSection>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>));
                            ialg1 = fixList(cacheArr.clone(), cenv.clone(), ialg1.clone(), tree.clone(), (std::sync::Arc::new(fixAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::AlgorithmSection>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>));
                        }
                        outNormalEqs = List::unionAppendListOnTrue(eq1.clone().reverse(), outNormalEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                        outInitialEqs = List::unionAppendListOnTrue(ieq1.clone().reverse(), outInitialEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                        outNormalAlgs = List::unionAppendListOnTrue(alg1.clone().reverse(), outNormalAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                        outInitialAlgs = List::unionAppendListOnTrue(ialg1.clone().reverse(), outInitialAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)));
                    }
                    outCache = cacheArr.clone().borrow()[(1-1) as usize].clone();
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { .. } => {
                    if !((Flags::getConfigBool(Flags::PERMISSIVE.clone())?)) { bail!("guard") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { .. } => {
                    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = outElements.clone();
                    if SCodeUtil::isConstant(SCodeUtil::attrVariability(var_field!((*el).attributes, SCode::Element::COMPONENT).clone())?) || !(inPartialInst.clone()) {
                        outElements = cons((el.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), false), outElements.clone());
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = outComments.clone();
                    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = outElements.clone();
                    outElements = cons((el.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), false), outElements.clone());
                    outComments = list![var_field!((*el).cmt, SCode::Element::CLASS).clone()];
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::IMPORT { .. } => {
                    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = outElements.clone();
                    outElements = cons((el.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), false), outElements.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instExtendsList failed on:\n\t")); __mm_s.push_str(&*literal!("className: ")); __mm_s.push_str(&*inClassName.clone()); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("env:       ")); __mm_s.push_str(&*FGraph::printGraphPathStr(outEnv.clone())?); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("mods:      ")); __mm_s.push_str(&*Mod::printModStr(outMod.clone())?); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("elem:      ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    }
    (outElements, outMod) = updateComponentsAndClassdefs(outElements.clone(), outMod.clone(), inEnv.clone());
    Ok((outCache, outEnv, outIH, outMod, outElements, outNormalEqs, outInitialEqs, outNormalAlgs, outInitialAlgs, outComments))
}

fn lookupBaseClass(mut inPath: Arc<Absyn::Path>, mut inSelfReference: bool, mut inClassName: ArcStr, mut inEnv: FCore::Graph, mut inCache: FCore::Cache) -> Result<(FCore::Cache, Option<Arc<SCode::Element>>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outElement: Option<Arc<SCode::Element>> = None;
    let mut outEnv: FCore::Graph;
    (outCache, outElement, outEnv) = (::match_deref::match_deref! { match &((inPath.clone(), inSelfReference.clone())) {
        (Deref @ Absyn::Path::IDENT { name }, true) => {
            let mut elem: Arc<SCode::Element>;
            let mut env: FCore::Graph;
            (elem, env) = Lookup::lookupClassLocal(inEnv.clone(), (name.clone()).clone())?;
            (inCache.clone(), Some(elem.clone()), env.clone())
        },
        (_, _) => {
            let mut elem: Arc<SCode::Element>;
            let mut env: FCore::Graph;
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut path: Arc<Absyn::Path>;
            path = AbsynUtil::removePartialPrefix(Arc::new(Absyn::Path::IDENT { name: (inClassName.clone()).clone() }), inPath.clone())?;
            (cache, elem, env) = Lookup::lookupClass(inCache.clone(), inEnv.clone(), path.clone(), None)?;
            (cache.clone(), Some(elem.clone()), env.clone())
        },
        _ => {
            (inCache.clone(), None, inEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outElement, outEnv))
}

fn updateElementListVisibility(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inVisibility: SCode::Visibility) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outElements = (match inVisibility.clone() {
        SCode::Visibility::PUBLIC => inElements.clone(),
        _ => ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (inElements.clone()).into_iter().cloned() {
            let __x = SCodeUtil::makeElementProtected(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
    });
    outElements
}

pub fn instExtendsAndClassExtendsList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inExtendsElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inClassExtendsElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inElementsFromExtendsScope: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inState: ClassInf::State, mut inClassName: ArcStr, mut inImpl: bool, mut isPartialInst: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<DAE::Mod>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::Comment>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outNormalEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outInitialEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outNormalAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outInitialAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
    let mut elts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = metamodelica::nil();
    let mut cdefelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut tmpelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut extendselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    extendselts = List::map(inExtendsElementLst.clone(), (std::sync::Arc::new(SCodeInstUtil::expandEnumerationClass) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>));
    (outCache, outEnv, outIH, outMod, elts, outNormalEqs, outInitialEqs, outNormalAlgs, outInitialAlgs, outComments) = instExtendsAndClassExtendsList2(inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), extendselts.clone(), inClassExtendsElementLst.clone(), inElementsFromExtendsScope.clone(), inState.clone(), (inClassName.clone()).clone(), inImpl.clone(), isPartialInst.clone())?;
    outElements = List::map(elts.clone(), std::sync::Arc::new(fnptr!(Util::tuple312, _)));
    tmpelts = List::map(outElements.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
    (_, cdefelts, _, _) = InstUtil::splitEltsNoComponents(tmpelts.clone())?;
    (outCache, outEnv, outIH) = InstUtil::addClassdefsToEnv(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), cdefelts.clone(), inImpl.clone(), Some(outMod.clone()), false)?;
    Ok((outCache, outEnv, outIH, outMod, outElements, outNormalEqs, outInitialEqs, outNormalAlgs, outInitialAlgs, outComments))
}

fn instExtendsAndClassExtendsList2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inExtendsElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inClassExtendsElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inElementsFromExtendsScope: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inState: ClassInf::State, mut inClassName: ArcStr, mut inImpl: bool, mut isPartialInst: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<DAE::Mod>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::Comment>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = metamodelica::nil();
    let mut outNormalEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outInitialEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outNormalAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outInitialAlgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
    (outCache, outEnv, outIH, outMod, outElements, outNormalEqs, outInitialEqs, outNormalAlgs, outInitialAlgs, comments) = instExtendsList(inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inExtendsElementLst.clone(), inElementsFromExtendsScope.clone(), inState.clone(), (inClassName.clone()).clone(), inImpl.clone(), isPartialInst.clone())?;
    (outMod, outElements) = instClassExtendsList(inEnv.clone(), outMod.clone(), inClassExtendsElementLst.clone(), outElements.clone())?;
    Ok((outCache, outEnv, outIH, outMod, outElements, outNormalEqs, outInitialEqs, outNormalAlgs, outInitialAlgs, comments))
}

fn instClassExtendsList(mut inEnv: FCore::Graph, mut inMod: Arc<DAE::Mod>, mut inClassExtendsList: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>) -> Result<(Arc<DAE::Mod>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>)> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = metamodelica::nil();
    (outMod, outElements) = 'mc: {
        let __mc_input = (inMod.clone(), inClassExtendsList.clone(), inElements.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, Deref @ metamodelica::List::Nil, compelts) => {
                    Ok((emod.clone(), compelts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, Deref @ metamodelica::List::Cons { head: first @ Deref @ SCode::Element::CLASS { name, .. }, tail: rest }, compelts) => {
                    let mut emod = (*emod).clone();
                    let mut compelts = (*compelts).clone();
                    (emod, compelts) = instClassExtendsList2(inEnv.clone(), emod.clone(), (name.clone()).clone(), first.clone(), compelts.clone())?;
                    (emod, compelts) = instClassExtendsList(inEnv.clone(), emod.clone(), rest.clone(), compelts.clone())?;
                    Ok((emod.clone(), compelts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { name, .. }, tail: _ }, compelts) => {
                    let mut els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instClassExtendsList failed ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln((literal!("  Candidate classes: ")).clone())?;
                    els = List::map(compelts.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
                    names = List::map(els.clone(), (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
                    Debug::traceln(stringDelimitList(names.clone(), (literal!(",")).clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMod, outElements))
}

fn buildClassExtendsName(mut inEnvPath: ArcStr, mut inClassName: ArcStr) -> ArcStr {
    let mut outClassName: ArcStr = arcstr::literal!("");
    outClassName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$parent.")); __mm_s.push_str(&*inClassName.clone()); __mm_s.push_str(&*literal!(".$env.")); __mm_s.push_str(&*inEnvPath.clone()); ArcStr::from(__mm_s) }).clone();
    outClassName
}

fn instClassExtendsList2(mut inEnv: FCore::Graph, mut inMod: Arc<DAE::Mod>, mut inName: ArcStr, mut inClassExtendsElt: Arc<SCode::Element>, mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>) -> Result<(Arc<DAE::Mod>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>)> {
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = metamodelica::nil();
    (outMod, outElements) = 'mc: {
        let __mc_input = (inMod.clone(), inName.clone(), inClassExtendsElt.clone(), inElements.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, name1, classExtendsElt, Deref @ metamodelica::List::Cons { head: (cl @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { .. }, name: name2, .. }, mod1, b), tail: rest }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut compelt: Arc<SCode::Element>;
                    let mut classDef: Arc<SCode::ClassDef>;
                    let mut classExtendsCdef: Arc<SCode::ClassDef>;
                    let mut partialPrefix1: SCode::Partial = SCode::Partial::NOT_PARTIAL;
                    let mut partialPrefix2: SCode::Partial = SCode::Partial::NOT_PARTIAL;
                    let mut encapsulatedPrefix1: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut encapsulatedPrefix2: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut restriction1: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut restriction2: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut prefixes1: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
                    let mut prefixes2: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
                    let mut vis2: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut env_path: ArcStr = arcstr::literal!("");
                    let mut externalDecl1: Option<Arc<SCode::ExternalDecl>> = None;
                    let mut externalDecl2: Option<Arc<SCode::ExternalDecl>> = None;
                    let mut comment1: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
                    let mut comment2: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
                    let mut els1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut els2: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut nEqn1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut nEqn2: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut inEqn1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut inEqn2: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut nAlg1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut nAlg2: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut inAlg1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut inAlg2: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut inCons1: Arc<metamodelica::List<SCode::ConstraintSection>> = metamodelica::nil();
                    let mut inCons2: Arc<metamodelica::List<SCode::ConstraintSection>> = metamodelica::nil();
                    let mut clats: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut mods: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut info1: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut info2: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut emod = (*emod).clone();
                    let mut name2 = (*name2).clone();
                    let true = (name1.clone() == name2.clone()) else { bail!("pattern mismatch") };
                    env_path = (AbsynUtil::pathString(FGraph::getGraphName(inEnv.clone())?, (literal!(".")).clone(), true, false)?).clone();
                    name2 = (buildClassExtendsName((env_path.clone()).clone(), (name2.clone()).clone())).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(cl.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa0, encapsulatedPrefix: __pa1, partialPrefix: __pa2, restriction: __pa3, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa4, normalEquationLst: __pa5, initialEquationLst: __pa6, normalAlgorithmLst: __pa7, initialAlgorithmLst: __pa8, constraintLst: __pa9, clsattrs: __pa10, externalDecl: __pa11 }, cmt: __pa12, info: __pa13 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    prefixes2 = __pa0.clone();
                    encapsulatedPrefix2 = __pa1.clone();
                    partialPrefix2 = __pa2.clone();
                    restriction2 = __pa3.clone();
                    els2 = __pa4.clone();
                    nEqn2 = __pa5.clone();
                    inEqn2 = __pa6.clone();
                    nAlg2 = __pa7.clone();
                    inAlg2 = __pa8.clone();
                    inCons2 = __pa9.clone();
                    clats = __pa10.clone();
                    externalDecl2 = __pa11.clone();
                    comment2 = __pa12.clone();
                    info2 = __pa13.clone();
                    let (__pa15, __pa16, __pa17, __pa18, __pa19, __pa20, __pa21) = ::match_deref::match_deref! { match &(classExtendsElt.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa15, encapsulatedPrefix: __pa16, partialPrefix: __pa17, restriction: __pa18, classDef: __pa19, cmt: __pa20, info: __pa21 } => (__pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    prefixes1 = __pa15.clone();
                    encapsulatedPrefix1 = __pa16.clone();
                    partialPrefix1 = __pa17.clone();
                    restriction1 = __pa18.clone();
                    classExtendsCdef = __pa19.clone();
                    comment1 = __pa20.clone();
                    info1 = __pa21.clone();
                    let (__pa22, __pa23, __pa24, __pa25, __pa26, __pa27, __pa28, __pa29) = ::match_deref::match_deref! { match &(classExtendsCdef.clone()) {
                        Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: __pa22, composition: Deref @ SCode::ClassDef::PARTS { elementLst: __pa23, normalEquationLst: __pa24, initialEquationLst: __pa25, normalAlgorithmLst: __pa26, initialAlgorithmLst: __pa27, constraintLst: __pa28, clsattrs: _, externalDecl: __pa29 } } => (__pa22.clone(), __pa23.clone(), __pa24.clone(), __pa25.clone(), __pa26.clone(), __pa27.clone(), __pa28.clone(), __pa29.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    mods = __pa22.clone();
                    els1 = __pa23.clone();
                    nEqn1 = __pa24.clone();
                    inEqn1 = __pa25.clone();
                    nAlg1 = __pa26.clone();
                    inAlg1 = __pa27.clone();
                    inCons1 = __pa28.clone();
                    externalDecl1 = __pa29.clone();
                    classDef = Arc::new(SCode::ClassDef::PARTS { elementLst: els2.clone(), normalEquationLst: nEqn2.clone(), initialEquationLst: inEqn2.clone(), normalAlgorithmLst: nAlg2.clone(), initialAlgorithmLst: inAlg2.clone(), constraintLst: inCons2.clone(), clsattrs: clats.clone(), externalDecl: externalDecl2.clone() });
                    compelt = Arc::new(SCode::Element::CLASS { name: (name2.clone()).clone(), prefixes: prefixes2.clone(), encapsulatedPrefix: encapsulatedPrefix2.clone(), partialPrefix: partialPrefix2.clone(), restriction: restriction2.clone(), classDef: classDef.clone(), cmt: comment2.clone(), info: info2.clone() });
                    vis2 = SCodeUtil::prefixesVisibility(prefixes2.clone())?;
                    elt = Arc::new(SCode::Element::EXTENDS { baseClassPath: Arc::new(Absyn::Path::IDENT { name: (name2.clone()).clone() }), visibility: vis2.clone(), modifications: mods.clone(), ann: None, info: info1.clone() });
                    classDef = Arc::new(SCode::ClassDef::PARTS { elementLst: cons(elt.clone(), els1.clone()), normalEquationLst: nEqn1.clone(), initialEquationLst: inEqn1.clone(), normalAlgorithmLst: nAlg1.clone(), initialAlgorithmLst: inAlg1.clone(), constraintLst: inCons1.clone(), clsattrs: clats.clone(), externalDecl: externalDecl1.clone() });
                    elt = Arc::new(SCode::Element::CLASS { name: (name1.clone()).clone(), prefixes: prefixes1.clone(), encapsulatedPrefix: encapsulatedPrefix1.clone(), partialPrefix: partialPrefix1.clone(), restriction: restriction1.clone(), classDef: classDef.clone(), cmt: comment1.clone(), info: info1.clone() });
                    emod = Mod::renameTopLevelNamedSubMod(emod.clone(), (name1.clone()).clone(), (name2.clone()).clone());
                    Ok((emod.clone(), cons((compelt.clone(), mod1.clone(), b.clone()), cons((elt.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), true), rest.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, name1, classExtendsElt, Deref @ metamodelica::List::Cons { head: (cl @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, name: name2, .. }, mod1, b), tail: rest }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut compelt: Arc<SCode::Element>;
                    let mut classDef: Arc<SCode::ClassDef>;
                    let mut classExtendsCdef: Arc<SCode::ClassDef>;
                    let mut partialPrefix1: SCode::Partial = SCode::Partial::NOT_PARTIAL;
                    let mut partialPrefix2: SCode::Partial = SCode::Partial::NOT_PARTIAL;
                    let mut encapsulatedPrefix1: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut encapsulatedPrefix2: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
                    let mut restriction1: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut restriction2: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let mut prefixes1: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
                    let mut prefixes2: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
                    let mut vis2: SCode::Visibility = SCode::Visibility::PROTECTED;
                    let mut env_path: ArcStr = arcstr::literal!("");
                    let mut externalDecl1: Option<Arc<SCode::ExternalDecl>> = None;
                    let mut comment1: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
                    let mut comment2: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
                    let mut els1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut nEqn1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut inEqn1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut nAlg1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut inAlg1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut inCons1: Arc<metamodelica::List<SCode::ConstraintSection>> = metamodelica::nil();
                    let mut mods: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut derivedMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut info1: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut info2: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut attrs: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
                    let mut derivedTySpec: Arc<Absyn::TypeSpec>;
                    let mut emod = (*emod).clone();
                    let mut name2 = (*name2).clone();
                    let true = (name1.clone() == name2.clone()) else { bail!("pattern mismatch") };
                    env_path = (AbsynUtil::pathString(FGraph::getGraphName(inEnv.clone())?, (literal!(".")).clone(), true, false)?).clone();
                    name2 = (buildClassExtendsName((env_path.clone()).clone(), (name2.clone()).clone())).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(cl.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa0, encapsulatedPrefix: __pa1, partialPrefix: __pa2, restriction: __pa3, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa4, modifications: __pa5, attributes: __pa6 }, cmt: __pa7, info: __pa8 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    prefixes2 = __pa0.clone();
                    encapsulatedPrefix2 = __pa1.clone();
                    partialPrefix2 = __pa2.clone();
                    restriction2 = __pa3.clone();
                    derivedTySpec = __pa4.clone();
                    derivedMod = __pa5.clone();
                    attrs = __pa6.clone();
                    comment2 = __pa7.clone();
                    info2 = __pa8.clone();
                    let (__pa10, __pa11, __pa12, __pa13, __pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(classExtendsElt.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa10, encapsulatedPrefix: __pa11, partialPrefix: __pa12, restriction: __pa13, classDef: __pa14, cmt: __pa15, info: __pa16 } => (__pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    prefixes1 = __pa10.clone();
                    encapsulatedPrefix1 = __pa11.clone();
                    partialPrefix1 = __pa12.clone();
                    restriction1 = __pa13.clone();
                    classExtendsCdef = __pa14.clone();
                    comment1 = __pa15.clone();
                    info1 = __pa16.clone();
                    let (__pa17, __pa18, __pa19, __pa20, __pa21, __pa22, __pa23, __pa24) = ::match_deref::match_deref! { match &(classExtendsCdef.clone()) {
                        Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: __pa17, composition: Deref @ SCode::ClassDef::PARTS { elementLst: __pa18, normalEquationLst: __pa19, initialEquationLst: __pa20, normalAlgorithmLst: __pa21, initialAlgorithmLst: __pa22, constraintLst: __pa23, clsattrs: _, externalDecl: __pa24 } } => (__pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone(), __pa22.clone(), __pa23.clone(), __pa24.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    mods = __pa17.clone();
                    els1 = __pa18.clone();
                    nEqn1 = __pa19.clone();
                    inEqn1 = __pa20.clone();
                    nAlg1 = __pa21.clone();
                    inAlg1 = __pa22.clone();
                    inCons1 = __pa23.clone();
                    externalDecl1 = __pa24.clone();
                    classDef = Arc::new(SCode::ClassDef::DERIVED { typeSpec: derivedTySpec.clone(), modifications: derivedMod.clone(), attributes: attrs.clone() });
                    compelt = Arc::new(SCode::Element::CLASS { name: (name2.clone()).clone(), prefixes: prefixes2.clone(), encapsulatedPrefix: encapsulatedPrefix2.clone(), partialPrefix: partialPrefix2.clone(), restriction: restriction2.clone(), classDef: classDef.clone(), cmt: comment2.clone(), info: info2.clone() });
                    vis2 = SCodeUtil::prefixesVisibility(prefixes2.clone())?;
                    elt = Arc::new(SCode::Element::EXTENDS { baseClassPath: Arc::new(Absyn::Path::IDENT { name: (name2.clone()).clone() }), visibility: vis2.clone(), modifications: mods.clone(), ann: None, info: info1.clone() });
                    classDef = Arc::new(SCode::ClassDef::PARTS { elementLst: cons(elt.clone(), els1.clone()), normalEquationLst: nEqn1.clone(), initialEquationLst: inEqn1.clone(), normalAlgorithmLst: nAlg1.clone(), initialAlgorithmLst: inAlg1.clone(), constraintLst: inCons1.clone(), clsattrs: metamodelica::nil(), externalDecl: externalDecl1.clone() });
                    elt = Arc::new(SCode::Element::CLASS { name: (name1.clone()).clone(), prefixes: prefixes1.clone(), encapsulatedPrefix: encapsulatedPrefix1.clone(), partialPrefix: partialPrefix1.clone(), restriction: restriction1.clone(), classDef: classDef.clone(), cmt: comment1.clone(), info: info1.clone() });
                    emod = Mod::renameTopLevelNamedSubMod(emod.clone(), (name1.clone()).clone(), (name2.clone()).clone());
                    Ok((emod.clone(), cons((compelt.clone(), mod1.clone(), b.clone()), cons((elt.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), true), rest.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, name1, classExtendsElt, Deref @ metamodelica::List::Cons { head: first, tail: rest }) => {
                    let mut emod = (*emod).clone();
                    let mut rest = (*rest).clone();
                    (emod, rest) = instClassExtendsList2(inEnv.clone(), emod.clone(), (name1.clone()).clone(), classExtendsElt.clone(), rest.clone())?;
                    Ok((emod.clone(), cons(first.clone(), rest.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::List::Nil) => {
                    Debug::traceln((literal!("TODO: Make a proper Error message here - Inst.instClassExtendsList2 couldn't find the class to extend")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMod, outElements))
}

pub fn instDerivedClasses(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inBoolean: bool, mut inInfo: SourceInfo) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<DAE::Mod>, Arc<metamodelica::List<Arc<SCode::Comment>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv1: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSCodeElementLst2: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outSCodeEquationLst3: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outSCodeEquationLst4: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outSCodeAlgorithmLst5: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outSCodeAlgorithmLst6: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
    (outCache, outEnv1, outIH, outSCodeElementLst2, outSCodeEquationLst3, outSCodeEquationLst4, outSCodeAlgorithmLst5, outSCodeAlgorithmLst6, outMod, outComments) = instDerivedClassesWork(inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inBoolean.clone(), inInfo.clone(), false, 0)?;
    Ok((outCache, outEnv1, outIH, outSCodeElementLst2, outSCodeEquationLst3, outSCodeEquationLst4, outSCodeAlgorithmLst5, outSCodeAlgorithmLst6, outMod, outComments))
}

fn instDerivedClassesWork(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inBoolean: bool, mut inInfo: SourceInfo, mut overflow: bool, mut numIter: i32) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<DAE::Mod>, Arc<metamodelica::List<Arc<SCode::Comment>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv1: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut outSCodeElementLst2: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outSCodeEquationLst3: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outSCodeEquationLst4: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outSCodeAlgorithmLst5: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outSCodeAlgorithmLst6: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
    (outCache, outEnv1, outIH, outSCodeElementLst2, outSCodeEquationLst3, outSCodeEquationLst4, outSCodeAlgorithmLst5, outSCodeAlgorithmLst6, outMod, outComments) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inBoolean.clone(), inInfo.clone(), overflow.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, Deref @ SCode::Element::CLASS { name, .. }, _, _, _) => {
                    let true = (InstUtil::isBuiltInClass((name.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), env.clone(), ih.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inMod.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: extdecl, initialAlgorithmLst: ialg, normalAlgorithmLst: alg, initialEquationLst: ieq, normalEquationLst: eq, elementLst: elt, .. }, name, .. }, _, info, _) => {
                    Error::assertionOrAddSourceMessage(isNone(extdecl.clone()), Error::EXTENDS_EXTERNAL.clone(), list![(name.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), elt.clone(), eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), inMod.clone(), list![var_field!((*inClass).cmt, SCode::Element::CLASS).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, r#mod, pre, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: dmod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tp, arrayDim: _ }, .. }, info, .. }, r#impl, _, false) => {
                    let mut elt: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut cenv: FCore::Graph;
                    let mut daeDMOD: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut eq: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut ieq: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut alg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut ialg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut c: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut dmod = (*dmod).clone();
                    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = outComments.clone();
                    (cache, c, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), tp.clone(), Some(info.clone()))?;
                    dmod = InstUtil::chainRedeclares(r#mod.clone(), dmod.clone())?;
                    (cache, daeDMOD) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), dmod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: tp.clone() }, info.clone())?;
                    r#mod = Mod::merge(r#mod.clone(), daeDMOD.clone(), (literal!("")).clone(), true)?;
                    (cache, env, ih, elt, eq, ieq, alg, ialg, r#mod, outComments) = instDerivedClassesWork(cache.clone(), cenv.clone(), ih.clone(), r#mod.clone(), pre.clone(), c.clone(), r#impl.clone(), info.clone(), numIter.clone() >= Global::recursionDepthLimit.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), elt.clone(), eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), r#mod.clone(), cons(var_field!((*inClass).cmt, SCode::Element::CLASS).clone(), outComments.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, r#mod, pre, Deref @ SCode::Element::CLASS { info, cmt, classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, prefixes, name: n, .. }, r#impl, _, false) => {
                    let mut elt: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut eq: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut ieq: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut alg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut ialg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut c: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = outComments.clone();
                    c = SCodeInstUtil::expandEnumeration((n.clone()).clone(), enumLst.clone(), prefixes.clone(), cmt.clone(), info.clone())?;
                    (cache, env, ih, elt, eq, ieq, alg, ialg, r#mod, outComments) = instDerivedClassesWork(cache.clone(), env.clone(), ih.clone(), r#mod.clone(), pre.clone(), c.clone(), r#impl.clone(), info.clone(), numIter.clone() >= Global::recursionDepthLimit.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), elt.clone(), eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), r#mod.clone(), outComments.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, true) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    str1 = (SCodeDump::unparseElementStr(inClass.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    str2 = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                    Error::addSourceMessage(Error::RECURSION_DEPTH_DERIVED.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Inst.instDerivedClasses failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv1, outIH, outSCodeElementLst2, outSCodeEquationLst3, outSCodeEquationLst4, outSCodeAlgorithmLst5, outSCodeAlgorithmLst6, outMod, outComments))
}

fn noImportElements(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outElements = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (inElements.clone()).into_iter().cloned() {
            if !(!(SCodeUtil::elementIsImport(e.clone()))) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElements
}

fn updateComponentsAndClassdefs(mut inComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>, mut inMod: Arc<DAE::Mod>, mut inEnv: FCore::Graph) -> (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>>, Arc<DAE::Mod>) {
    let mut outComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)>> = metamodelica::nil();
    let mut outRestMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    (outComponents, outRestMod) = List::map1Fold(inComponents.clone(), (std::sync::Arc::new(updateComponentsAndClassdefs2) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>, bool), FCore::Graph, Arc<DAE::Mod>) -> Result<((Arc<SCode::Element>, Arc<DAE::Mod>, bool), Arc<DAE::Mod>)> + 'static>), inEnv.clone(), inMod.clone());
    (outComponents, outRestMod)
}

fn updateComponentsAndClassdefs2(mut inComponent: (Arc<SCode::Element>, Arc<DAE::Mod>, bool), mut inEnv: FCore::Graph, mut inMod: Arc<DAE::Mod>) -> Result<((Arc<SCode::Element>, Arc<DAE::Mod>, bool), Arc<DAE::Mod>)> {
    let mut outComponent: (Arc<SCode::Element>, Arc<DAE::Mod>, bool);
    let mut outRestMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut el: Arc<SCode::Element>;
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut b: bool = false;
    (el, r#mod, b) = inComponent.clone();
    (outComponent, outRestMod) = 'mc: {
        let __mc_input = el.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { .. } => {
                    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mod_rest: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    cmod = Mod::lookupCompModificationFromEqu(inMod.clone(), (var_field!((*el).name, SCode::Element::COMPONENT).clone()).clone())?;
                    cmod = Mod::merge(cmod.clone(), r#mod.clone(), (var_field!((*el).name, SCode::Element::COMPONENT).clone()).clone(), false)?;
                    mod_rest = inMod.clone();
                    Ok(((el.clone(), cmod.clone(), b.clone()), mod_rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { .. } => {
                    Ok((inComponent.clone(), inMod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::IMPORT { .. } => {
                    Ok(((el.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), b.clone()), inMod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. } => {
                    let mut comp: Arc<SCode::Element>;
                    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mod_rest: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mod::lookupCompModification(inMod.clone(), (var_field!((*el).name, SCode::Element::CLASS).clone()).clone())?) {
                        Deref @ DAE::Mod::REDECL { r#mod: __pa0, element: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cmod = __pa0.clone();
                    comp = __pa1.clone();
                    mod_rest = inMod.clone();
                    cmod = Mod::merge(cmod.clone(), r#mod.clone(), (var_field!((*el).name, SCode::Element::CLASS).clone()).clone(), false)?;
                    comp = SCodeUtil::mergeWithOriginal(comp.clone(), el.clone())?;
                    Ok(((comp.clone(), cmod.clone(), b.clone()), mod_rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut outComponent: (Arc<SCode::Element>, Arc<DAE::Mod>, bool);
                    cmod = Mod::lookupCompModification(inMod.clone(), (var_field!((*el).name, SCode::Element::CLASS).clone()).clone())?;
                    outComponent = if (cmod.clone() == Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)) {inComponent.clone()} else {(el.clone(), cmod.clone(), b.clone())};
                    Ok((outComponent.clone(), inMod.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstExtends.updateComponentsAndClassdefs2 failed on:\n")); __mm_s.push_str(&*literal!("env = ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())?); __mm_s.push_str(&*literal!("\nmod = ")); __mm_s.push_str(&*Mod::printModStr(inMod.clone())?); __mm_s.push_str(&*literal!("\ncmod = ")); __mm_s.push_str(&*Mod::printModStr(r#mod.clone())?); __mm_s.push_str(&*literal!("\nbool = ")); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outComponent, outRestMod))
}

fn getLocalIdentList<Type_A: Clone + 'static>(mut ielts: Arc<metamodelica::List<Type_A>>, mut tree: Arc<AvlSetString::Tree>, mut getIdent: Arc<dyn ::std::ops::Fn(Type_A, Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>) -> Arc<AvlSetString::Tree> {
    pub type getIdentFn<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_A, Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>;

    let mut tree: Arc<AvlSetString::Tree> = tree;
    for mut elt in &*ielts.clone() {
        let mut elt = elt.clone();
        tree = getIdent(elt.clone(), tree.clone()).unwrap();
    }
    tree
}

fn getLocalIdentElementTpl(mut eltTpl: (Arc<SCode::Element>, Arc<DAE::Mod>, bool), mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> {
    let mut tree: Arc<AvlSetString::Tree> = tree;
    let mut elt: Arc<SCode::Element>;
    (elt, _, _) = eltTpl.clone();
    tree = getLocalIdentElement(elt.clone(), tree.clone())?;
    Ok(tree)
}

fn getLocalIdentElement(mut elt: Arc<SCode::Element>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> {
    let mut tree: Arc<AvlSetString::Tree> = tree;
    tree = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::COMPONENT { name: id, .. } => {
            AvlSetString::add(tree.clone(), (id.clone()).clone())?
        },
        Deref @ SCode::Element::CLASS { name: id, .. } => {
            AvlSetString::add(tree.clone(), (id.clone()).clone())?
        },
        _ => {
            tree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

fn fixLocalIdent(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut elt: (Arc<SCode::Element>, Arc<DAE::Mod>, bool), mut tree: Arc<AvlSetString::Tree>) -> Result<(Arc<SCode::Element>, Arc<DAE::Mod>, bool)> {
    let mut elt: (Arc<SCode::Element>, Arc<DAE::Mod>, bool) = elt;
    let mut elt1: Arc<SCode::Element>;
    let mut elt2: Arc<SCode::Element>;
    let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut b: bool = false;
    (elt1, r#mod, b) = elt.clone();
    elt2 = fixElement(inCache.clone(), inEnv.clone(), elt1.clone(), tree.clone())?;
    if !(referenceEq(&elt1.clone(),&elt2.clone())) || !(b.clone()) {
        elt = (elt2.clone(), r#mod.clone(), true);
    }
    Ok(elt)
}

fn fixElement(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inElt: Arc<SCode::Element>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Element>> {
    let mut outElts: Arc<SCode::Element>;
    outElts = 'mc: {
        let __mc_input = (inEnv.clone(), inElt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, elt @ Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut prefixes: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
                    let mut typeSpec1: Arc<Absyn::TypeSpec>;
                    let mut typeSpec2: Arc<Absyn::TypeSpec>;
                    let mut modifications1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut modifications2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut comment: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
                    let mut condition: Option<Arc<Absyn::Exp>> = None;
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut elt2: Arc<SCode::Element>;
                    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
                    let mut env = (*env).clone();
                    let (__pa8, __pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa9) = ::match_deref::match_deref! { match &(Lookup::lookupIdentLocal(inCache.clone().borrow()[(1-1) as usize].clone(), env.clone(), (var_field!((**elt).name, SCode::Element::COMPONENT).clone()).clone())?) {
                        (_, _, __pa8 @ Deref @ SCode::Element::COMPONENT { name: __pa0, prefixes: __pa1, attributes: __pa2 @ SCode::Attributes { .. }, typeSpec: __pa3, modifications: __pa4, comment: __pa5, condition: __pa6, info: __pa7 }, _, _, __pa9) => (__pa8.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    prefixes = __pa1.clone();
                    attr = __pa2.clone();
                    typeSpec1 = __pa3.clone();
                    modifications1 = __pa4.clone();
                    comment = __pa5.clone();
                    condition = __pa6.clone();
                    info = __pa7.clone();
                    elt2 = __pa8.clone();
                    env = __pa9.clone();
                    modifications2 = fixModifications(inCache.clone(), env.clone(), modifications1.clone(), tree.clone())?;
                    typeSpec2 = fixTypeSpec(inCache.clone(), env.clone(), typeSpec1.clone(), tree.clone())?;
                    ad = fixArrayDim(inCache.clone(), env.clone(), attr.arrayDims.clone(), tree.clone());
                    if !(referenceEq(&ad.clone(),&attr.arrayDims.clone())) {
                        attr.arrayDims = ad.clone();
                    }
                    if !(referenceEq(&ad.clone(),&attr.arrayDims.clone()) && referenceEq(&typeSpec1.clone(),&typeSpec2.clone()) && referenceEq(&modifications1.clone(),&modifications2.clone())) {
                        elt2 = Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: typeSpec2.clone(), modifications: modifications2.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() });
                    }
                    Ok(elt2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, elt @ Deref @ SCode::Element::COMPONENT { attributes: attr, .. }) => {
                    let mut typeSpec2: Arc<Absyn::TypeSpec>;
                    let mut modifications2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut elt = (*elt).clone();
                    let mut attr = (*attr).clone();
                    modifications2 = fixModifications(inCache.clone(), env.clone(), var_field!((*elt).modifications, SCode::Element::COMPONENT).clone(), tree.clone())?;
                    typeSpec2 = fixTypeSpec(inCache.clone(), env.clone(), var_field!((*elt).typeSpec, SCode::Element::COMPONENT).clone(), tree.clone())?;
                    ad = fixArrayDim(inCache.clone(), env.clone(), attr.arrayDims.clone(), tree.clone());
                    if !(referenceEq(&ad.clone(),&attr.arrayDims.clone())) {
                        attr.arrayDims = ad.clone();
                    }
                    if !(referenceEq(&ad.clone(),&attr.arrayDims.clone()) && referenceEq(&var_field!((*elt).typeSpec, SCode::Element::COMPONENT).clone(),&typeSpec2.clone()) && referenceEq(&var_field!((*elt).modifications, SCode::Element::COMPONENT).clone(),&modifications2.clone())) {
                        elt = Arc::new(SCode::Element::COMPONENT { name: (var_field!((*elt).name, SCode::Element::COMPONENT).clone()).clone(), prefixes: var_field!((*elt).prefixes, SCode::Element::COMPONENT).clone(), attributes: attr.clone(), typeSpec: typeSpec2.clone(), modifications: modifications2.clone(), comment: var_field!((*elt).comment, SCode::Element::COMPONENT).clone(), condition: var_field!((*elt).condition, SCode::Element::COMPONENT).clone(), info: var_field!((*elt).info, SCode::Element::COMPONENT).clone() });
                    }
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes: prefixes @ Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED, partialPrefix, restriction, classDef: _, cmt: comment, info }) => {
                    let mut classDef1: Arc<SCode::ClassDef>;
                    let mut classDef2: Arc<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    let mut prefixes = (*prefixes).clone();
                    let mut partialPrefix = (*partialPrefix).clone();
                    let mut restriction = (*restriction).clone();
                    let mut comment = (*comment).clone();
                    let mut info = (*info).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Lookup::lookupClassLocal(env.clone(), (name.clone()).clone())?) {
                        (Deref @ SCode::Element::CLASS { classDef: __pa0, info: __pa1, cmt: __pa2, restriction: __pa3, partialPrefix: __pa4, prefixes: __pa5, .. }, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    classDef1 = __pa0.clone();
                    info = __pa1.clone();
                    comment = __pa2.clone();
                    restriction = __pa3.clone();
                    partialPrefix = __pa4.clone();
                    prefixes = __pa5.clone();
                    env = __pa6.clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, (name.clone()).clone(), FGraph::restrictionToScopeType(restriction.clone()))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&classDef1.clone(),&classDef2.clone())) {inElt.clone()} else {Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED, partialPrefix, restriction, classDef: classDef1, cmt: comment, info }) => {
                    let mut classDef2: Arc<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, (name.clone()).clone(), FGraph::restrictionToScopeType(restriction.clone()))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&classDef1.clone(),&classDef2.clone())) {inElt.clone()} else {Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes: prefixes @ Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix, restriction, classDef: _, cmt: comment, info }) => {
                    let mut classDef1: Arc<SCode::ClassDef>;
                    let mut classDef2: Arc<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    let mut prefixes = (*prefixes).clone();
                    let mut partialPrefix = (*partialPrefix).clone();
                    let mut restriction = (*restriction).clone();
                    let mut comment = (*comment).clone();
                    let mut info = (*info).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Lookup::lookupClassLocal(env.clone(), (name.clone()).clone())?) {
                        (Deref @ SCode::Element::CLASS { classDef: __pa0, info: __pa1, cmt: __pa2, restriction: __pa3, partialPrefix: __pa4, prefixes: __pa5, .. }, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    classDef1 = __pa0.clone();
                    info = __pa1.clone();
                    comment = __pa2.clone();
                    restriction = __pa3.clone();
                    partialPrefix = __pa4.clone();
                    prefixes = __pa5.clone();
                    env = __pa6.clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (name.clone()).clone(), FGraph::restrictionToScopeType(restriction.clone()))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&classDef1.clone(),&classDef2.clone())) {inElt.clone()} else {Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix, restriction, classDef: classDef1, cmt: comment, info }) => {
                    let mut classDef2: Arc<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (name.clone()).clone(), FGraph::restrictionToScopeType(restriction.clone()))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&classDef1.clone(),&classDef2.clone())) {inElt.clone()} else {Arc::new(SCode::Element::CLASS { name: (name.clone()).clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::EXTENDS { baseClassPath: extendsPath1, visibility: vis, modifications: modifications1, ann: optAnnotation, info }) => {
                    let mut modifications2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut extendsPath2: Arc<Absyn::Path>;
                    extendsPath2 = fixPath(inCache.clone(), env.clone(), extendsPath1.clone(), tree.clone())?;
                    modifications2 = fixModifications(inCache.clone(), env.clone(), modifications1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&extendsPath1.clone(),&extendsPath2.clone()) && referenceEq(&modifications1.clone(),&modifications2.clone())) {inElt.clone()} else {Arc::new(SCode::Element::EXTENDS { baseClassPath: extendsPath2.clone(), visibility: vis.clone(), modifications: modifications2.clone(), ann: optAnnotation.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Element::IMPORT { .. }) => {
                    Ok(inElt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, elt) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixElement failed: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(elt.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElts)
}

fn fixClassdef(mut cache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inCd: Arc<SCode::ClassDef>, mut inTree: Arc<AvlSetString::Tree>) -> Result<Arc<SCode::ClassDef>> {
    let mut outCd: Arc<SCode::ClassDef>;
    let mut tree: Arc<AvlSetString::Tree> = inTree.clone();
    outCd = 'mc: {
        let __mc_input = (inEnv.clone(), inCd.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::ClassDef::PARTS { elementLst: elts, normalEquationLst: ne, initialEquationLst: ie, normalAlgorithmLst: na, initialAlgorithmLst: ia, constraintLst: nc, clsattrs: clats, externalDecl: ed }) => {
                    let mut elts_1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut ne_1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut ie_1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut na_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut ia_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut nc_1: Arc<metamodelica::List<SCode::ConstraintSection>> = metamodelica::nil();
                    let mut tree: Arc<AvlSetString::Tree> = tree.clone();
                    tree = getLocalIdentList(elts.clone(), tree.clone(), (std::sync::Arc::new(getLocalIdentElement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> + 'static>));
                    elts_1 = fixList(cache.clone(), env.clone(), elts.clone(), tree.clone(), (std::sync::Arc::new(fixElement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Element>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Element>> + 'static>));
                    ne_1 = fixList(cache.clone(), env.clone(), ne.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
                    ie_1 = fixList(cache.clone(), env.clone(), ie.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
                    na_1 = fixList(cache.clone(), env.clone(), na.clone(), tree.clone(), (std::sync::Arc::new(fixAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::AlgorithmSection>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>));
                    ia_1 = fixList(cache.clone(), env.clone(), ia.clone(), tree.clone(), (std::sync::Arc::new(fixAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::AlgorithmSection>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>));
                    nc_1 = fixList(cache.clone(), env.clone(), nc.clone(), tree.clone(), (std::sync::Arc::new(fixConstraint) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, SCode::ConstraintSection, Arc<AvlSetString::Tree>) -> Result<SCode::ConstraintSection> + 'static>));
                    Ok(if (referenceEq(&elts.clone(),&elts_1.clone()) && referenceEq(&ne.clone(),&ne_1.clone()) && referenceEq(&ie.clone(),&ie_1.clone()) && referenceEq(&na.clone(),&na_1.clone()) && referenceEq(&ia.clone(),&ia_1.clone()) && referenceEq(&nc.clone(),&nc_1.clone())) {inCd.clone()} else {Arc::new(SCode::ClassDef::PARTS { elementLst: elts_1.clone(), normalEquationLst: ne_1.clone(), initialEquationLst: ie_1.clone(), normalAlgorithmLst: na_1.clone(), initialAlgorithmLst: ia_1.clone(), constraintLst: nc_1.clone(), clsattrs: clats.clone(), externalDecl: ed.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: r#mod, composition: cd @ Deref @ SCode::ClassDef::PARTS { elementLst: elts, normalEquationLst: ne, initialEquationLst: ie, normalAlgorithmLst: na, initialAlgorithmLst: ia, constraintLst: nc, clsattrs: clats, externalDecl: ed } }) => {
                    let mut elts_1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut ne_1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut ie_1: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
                    let mut na_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut ia_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
                    let mut nc_1: Arc<metamodelica::List<SCode::ConstraintSection>> = metamodelica::nil();
                    let mut mod_1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cd_1: Arc<SCode::ClassDef>;
                    mod_1 = fixModifications(cache.clone(), env.clone(), r#mod.clone(), inTree.clone())?;
                    elts_1 = fixList(cache.clone(), env.clone(), elts.clone(), tree.clone(), (std::sync::Arc::new(fixElement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Element>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Element>> + 'static>));
                    ne_1 = fixList(cache.clone(), env.clone(), ne.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
                    ie_1 = fixList(cache.clone(), env.clone(), ie.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
                    na_1 = fixList(cache.clone(), env.clone(), na.clone(), tree.clone(), (std::sync::Arc::new(fixAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::AlgorithmSection>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>));
                    ia_1 = fixList(cache.clone(), env.clone(), ia.clone(), tree.clone(), (std::sync::Arc::new(fixAlgorithm) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::AlgorithmSection>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::AlgorithmSection>> + 'static>));
                    nc_1 = fixList(cache.clone(), env.clone(), nc.clone(), tree.clone(), (std::sync::Arc::new(fixConstraint) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, SCode::ConstraintSection, Arc<AvlSetString::Tree>) -> Result<SCode::ConstraintSection> + 'static>));
                    cd_1 = if (referenceEq(&elts.clone(),&elts_1.clone()) && referenceEq(&ne.clone(),&ne_1.clone()) && referenceEq(&ie.clone(),&ie_1.clone()) && referenceEq(&na.clone(),&na_1.clone()) && referenceEq(&ia.clone(),&ia_1.clone()) && referenceEq(&nc.clone(),&nc_1.clone())) {cd.clone()} else {Arc::new(SCode::ClassDef::PARTS { elementLst: elts_1.clone(), normalEquationLst: ne_1.clone(), initialEquationLst: ie_1.clone(), normalAlgorithmLst: na_1.clone(), initialAlgorithmLst: ia_1.clone(), constraintLst: nc_1.clone(), clsattrs: clats.clone(), externalDecl: ed.clone() })};
                    Ok(if (referenceEq(&cd.clone(),&cd_1.clone()) && referenceEq(&r#mod.clone(),&mod_1.clone())) {inCd.clone()} else {Arc::new(SCode::ClassDef::CLASS_EXTENDS { modifications: mod_1.clone(), composition: cd_1.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::ClassDef::DERIVED { typeSpec: ts, modifications: r#mod, attributes: attr }) => {
                    let mut ts_1: Arc<Absyn::TypeSpec>;
                    let mut mod_1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    ts_1 = fixTypeSpec(cache.clone(), env.clone(), ts.clone(), tree.clone())?;
                    mod_1 = fixModifications(cache.clone(), env.clone(), r#mod.clone(), tree.clone())?;
                    Ok(if (referenceEq(&ts.clone(),&ts_1.clone()) && referenceEq(&r#mod.clone(),&mod_1.clone())) {inCd.clone()} else {Arc::new(SCode::ClassDef::DERIVED { typeSpec: ts_1.clone(), modifications: mod_1.clone(), attributes: attr.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd @ Deref @ SCode::ClassDef::ENUMERATION { .. }) => {
                    Ok(cd.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd @ Deref @ SCode::ClassDef::OVERLOAD { .. }) => {
                    Ok(cd.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd @ Deref @ SCode::ClassDef::PDER { .. }) => {
                    Ok(cd.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixClassDef failed: ")); __mm_s.push_str(&*SCodeDump::classDefStr(cd.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCd)
}

fn fixEquation(mut cache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inEeq: Arc<SCode::Equation>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> {
    let mut outEeq: Arc<SCode::Equation>;
    outEeq = (::match_deref::match_deref! { match &(inEeq.clone()) {
        Deref @ SCode::Equation::EQ_IF { condition: expl, thenBranch: eqll, elseBranch: eql, comment, info } => {
            let mut expl = (*expl).clone();
            let mut eqll = (*eqll).clone();
            let mut eql = (*eql).clone();
            expl = fixList(cache.clone(), inEnv.clone(), expl.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>));
            eqll = fixListList(cache.clone(), inEnv.clone(), eqll.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
            eql = fixList(cache.clone(), inEnv.clone(), eql.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
            Arc::new(SCode::Equation::EQ_IF { condition: expl.clone(), thenBranch: eqll.clone(), elseBranch: eql.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_EQUALS { expLeft: exp1, expRight: exp2, comment, info } => {
            let mut exp1 = (*exp1).clone();
            let mut exp2 = (*exp2).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp2 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
            Arc::new(SCode::Equation::EQ_EQUALS { expLeft: exp1.clone(), expRight: exp2.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: exp1, expRight: exp2, domain: cref, comment, info } => {
            let mut exp1 = (*exp1).clone();
            let mut exp2 = (*exp2).clone();
            let mut cref = (*cref).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp2 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
            cref = fixCref(cache.clone(), inEnv.clone(), cref.clone(), tree.clone())?;
            Arc::new(SCode::Equation::EQ_PDE { expLeft: exp1.clone(), expRight: exp2.clone(), domain: cref.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_CONNECT { crefLeft: cref1, crefRight: cref2, comment, info } => {
            let mut cref1 = (*cref1).clone();
            let mut cref2 = (*cref2).clone();
            cref1 = fixCref(cache.clone(), inEnv.clone(), cref1.clone(), tree.clone())?;
            cref2 = fixCref(cache.clone(), inEnv.clone(), cref2.clone(), tree.clone())?;
            Arc::new(SCode::Equation::EQ_CONNECT { crefLeft: cref1.clone(), crefRight: cref2.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_FOR { index: id, range: optExp, eEquationLst: eql, comment, info } => {
            let mut optExp = (*optExp).clone();
            let mut eql = (*eql).clone();
            optExp = fixOption(cache.clone(), inEnv.clone(), optExp.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>));
            eql = fixList(cache.clone(), inEnv.clone(), eql.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
            Arc::new(SCode::Equation::EQ_FOR { index: (id.clone()).clone(), range: optExp.clone(), eEquationLst: eql.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_WHEN { condition: exp, eEquationLst: eql, elseBranches: whenlst, comment, info } => {
            let mut exp = (*exp).clone();
            let mut eql = (*eql).clone();
            let mut whenlst = (*whenlst).clone();
            exp = fixExp(cache.clone(), inEnv.clone(), exp.clone(), tree.clone())?;
            eql = fixList(cache.clone(), inEnv.clone(), eql.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
            whenlst = fixListTuple2(cache.clone(), inEnv.clone(), whenlst.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>), (std::sync::Arc::new(fnptr!(fixListEquation, metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<AvlSetString::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<AvlSetString::Tree>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> + 'static>));
            Arc::new(SCode::Equation::EQ_WHEN { condition: exp.clone(), eEquationLst: eql.clone(), elseBranches: whenlst.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_ASSERT { condition: exp1, message: exp2, level: exp3, comment, info } => {
            let mut exp1 = (*exp1).clone();
            let mut exp2 = (*exp2).clone();
            let mut exp3 = (*exp3).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp2 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
            exp3 = fixExp(cache.clone(), inEnv.clone(), exp3.clone(), tree.clone())?;
            Arc::new(SCode::Equation::EQ_ASSERT { condition: exp1.clone(), message: exp2.clone(), level: exp3.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_TERMINATE { message: exp, comment, info } => {
            let mut exp = (*exp).clone();
            exp = fixExp(cache.clone(), inEnv.clone(), exp.clone(), tree.clone())?;
            Arc::new(SCode::Equation::EQ_TERMINATE { message: exp.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: exp1, expReinit: exp, comment, info } => {
            let mut exp1 = (*exp1).clone();
            let mut exp = (*exp).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp = fixExp(cache.clone(), inEnv.clone(), exp.clone(), tree.clone())?;
            Arc::new(SCode::Equation::EQ_REINIT { cref: exp1.clone(), expReinit: exp.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ SCode::Equation::EQ_NORETCALL { exp, comment, info } => {
            let mut exp = (*exp).clone();
            exp = fixExp(cache.clone(), inEnv.clone(), exp.clone(), tree.clone())?;
            Arc::new(SCode::Equation::EQ_NORETCALL { exp: exp.clone(), comment: comment.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEeq)
}

fn fixListEquation(mut cache: metamodelica::Array<FCore::Cache>, mut env: FCore::Graph, mut eeq: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut tree: Arc<AvlSetString::Tree>) -> Arc<metamodelica::List<Arc<SCode::Equation>>> {
    let mut outEeq: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    outEeq = fixList(cache.clone(), env.clone(), eeq.clone(), tree.clone(), (std::sync::Arc::new(fixEquation) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Equation>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Equation>> + 'static>));
    outEeq
}

fn fixAlgorithm(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inAlg: Arc<SCode::AlgorithmSection>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<SCode::AlgorithmSection>> {
    let mut outAlg: Arc<SCode::AlgorithmSection> = Arc::new(<SCode::AlgorithmSection as ::std::default::Default>::default());
    let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inAlg.clone()) {
        Deref @ SCode::AlgorithmSection { statements: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    stmts1 = __pa0.clone();
    stmts2 = fixList(inCache.clone(), inEnv.clone(), stmts1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
    outAlg = if (referenceEq(&stmts1.clone(),&stmts2.clone())) {inAlg.clone()} else {Arc::new(SCode::AlgorithmSection { statements: stmts2.clone() })};
    Ok(outAlg)
}

fn fixConstraint(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inConstrs: SCode::ConstraintSection, mut tree: Arc<AvlSetString::Tree>) -> Result<SCode::ConstraintSection> {
    let mut outConstrs: SCode::ConstraintSection = <SCode::ConstraintSection as ::std::default::Default>::default();
    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let SCode::CONSTRAINTS { constraints: __pa0 } = (inConstrs.clone()) else { bail!("pattern mismatch") };
    exps = __pa0.clone();
    exps = fixList(inCache.clone(), inEnv.clone(), exps.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>));
    outConstrs = SCode::ConstraintSection { constraints: exps.clone() };
    Ok(outConstrs)
}

fn fixListAlgorithmItem(mut cache: metamodelica::Array<FCore::Cache>, mut env: FCore::Graph, mut alg: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut tree: Arc<AvlSetString::Tree>) -> Arc<metamodelica::List<Arc<SCode::Statement>>> {
    let mut outAlg: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    outAlg = fixList(cache.clone(), env.clone(), alg.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
    outAlg
}

fn fixStatement(mut cache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inStmt: Arc<SCode::Statement>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> {
    let mut outStmt: Arc<SCode::Statement>;
    outStmt = 'mc: {
        let __mc_input = inStmt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: exp1, value: exp2, comment, info } => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    exp2_1 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
                    Ok(if (referenceEq(&exp1.clone(),&exp1_1.clone()) && referenceEq(&exp2.clone(),&exp2_1.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_ASSIGN { assignComponent: exp1_1.clone(), value: exp2_1.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_IF { boolExpr: exp1, trueBranch: truebranch1, elseIfBranch: elseifbranch1, elseBranch: elsebranch1, comment, info } => {
                    let mut exp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut elseifbranch2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
                    let mut truebranch2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    let mut elsebranch2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    truebranch2 = fixList(cache.clone(), inEnv.clone(), truebranch1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    elseifbranch2 = fixListTuple2(cache.clone(), inEnv.clone(), elseifbranch1.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>), (std::sync::Arc::new(fnptr!(fixListAlgorithmItem, metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Statement>>>, Arc<AvlSetString::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Statement>>>, Arc<AvlSetString::Tree>) -> Result<Arc<metamodelica::List<Arc<SCode::Statement>>>> + 'static>));
                    elsebranch2 = fixList(cache.clone(), inEnv.clone(), elsebranch1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    Ok(if (referenceEq(&exp1.clone(),&exp2.clone()) && referenceEq(&truebranch1.clone(),&truebranch2.clone()) && referenceEq(&elseifbranch1.clone(),&elseifbranch2.clone()) && referenceEq(&elsebranch1.clone(),&elsebranch2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_IF { boolExpr: exp2.clone(), trueBranch: truebranch2.clone(), elseIfBranch: elseifbranch2.clone(), elseBranch: elsebranch2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_FOR { index: iter, range: optExp1, forBody: body1, comment, info } => {
                    let mut optExp2: Option<Arc<Absyn::Exp>> = None;
                    let mut body2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    optExp2 = fixOption(cache.clone(), inEnv.clone(), optExp1.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>));
                    body2 = fixList(cache.clone(), inEnv.clone(), body1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    Ok(if (referenceEq(&optExp1.clone(),&optExp2.clone()) && referenceEq(&body1.clone(),&body2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_FOR { index: (iter.clone()).clone(), range: optExp2.clone(), forBody: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_PARFOR { index: iter, range: optExp1, parforBody: body1, comment, info } => {
                    let mut optExp2: Option<Arc<Absyn::Exp>> = None;
                    let mut body2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    optExp2 = fixOption(cache.clone(), inEnv.clone(), optExp1.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>));
                    body2 = fixList(cache.clone(), inEnv.clone(), body1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    Ok(if (referenceEq(&optExp1.clone(),&optExp2.clone()) && referenceEq(&body1.clone(),&body2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_PARFOR { index: (iter.clone()).clone(), range: optExp2.clone(), parforBody: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_WHILE { boolExpr: exp1, whileBody: body1, comment, info } => {
                    let mut exp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut body2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    body2 = fixList(cache.clone(), inEnv.clone(), body1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    Ok(if (referenceEq(&exp1.clone(),&exp2.clone()) && referenceEq(&body1.clone(),&body2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_WHILE { boolExpr: exp2.clone(), whileBody: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_WHEN_A { branches: whenlst, comment, info } => {
                    let mut whenlst = (*whenlst).clone();
                    whenlst = fixListTuple2(cache.clone(), inEnv.clone(), whenlst.clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>), (std::sync::Arc::new(fnptr!(fixListAlgorithmItem, metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Statement>>>, Arc<AvlSetString::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<SCode::Statement>>>, Arc<AvlSetString::Tree>) -> Result<Arc<metamodelica::List<Arc<SCode::Statement>>>> + 'static>));
                    Ok(Arc::new(SCode::Statement::ALG_WHEN_A { branches: whenlst.clone(), comment: comment.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_ASSERT { condition: exp, message: exp1, level: exp2, comment, info } => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp_1 = fixExp(cache.clone(), inEnv.clone(), exp.clone(), tree.clone())?;
                    exp1_1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    exp2_1 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
                    Ok(if (referenceEq(&exp.clone(),&exp_1.clone()) && referenceEq(&exp1.clone(),&exp1_1.clone()) && referenceEq(&exp2.clone(),&exp2_1.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_ASSERT { condition: exp_1.clone(), message: exp1_1.clone(), level: exp2_1.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_TERMINATE { message: exp1, comment, info } => {
                    let mut exp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&exp1.clone(),&exp2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_TERMINATE { message: exp2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_REINIT { cref: exp1, newValue: exp2, comment, info } => {
                    let mut exp1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut exp2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp1_1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    exp2_1 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
                    Ok(if (referenceEq(&exp1.clone(),&exp1_1.clone()) && referenceEq(&exp2.clone(),&exp2_1.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_REINIT { cref: exp1_1.clone(), newValue: exp2_1.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_NORETCALL { exp: exp1, comment, info } => {
                    let mut exp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&exp1.clone(),&exp2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_NORETCALL { exp: exp2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_RETURN { .. } => {
                    Ok(inStmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_BREAK { .. } => {
                    Ok(inStmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_FAILURE { stmts: body1, comment, info } => {
                    let mut body2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    body2 = fixList(cache.clone(), inEnv.clone(), body1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    Ok(if (referenceEq(&body1.clone(),&body2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_FAILURE { stmts: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_TRY { body: truebranch1, elseBody: elsebranch1, comment, info } => {
                    let mut truebranch2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    let mut elsebranch2: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
                    truebranch2 = fixList(cache.clone(), inEnv.clone(), truebranch1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    elsebranch2 = fixList(cache.clone(), inEnv.clone(), elsebranch1.clone(), tree.clone(), (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::Statement>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Statement>> + 'static>));
                    Ok(if (referenceEq(&truebranch1.clone(),&truebranch2.clone()) && referenceEq(&elsebranch1.clone(),&elsebranch2.clone())) {inStmt.clone()} else {Arc::new(SCode::Statement::ALG_TRY { body: truebranch2.clone(), elseBody: elsebranch2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_CONTINUE { .. } => {
                    Ok(inStmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixStatement")); __mm_s.push_str(&*literal!(" failed: ")); __mm_s.push_str(&*Dump::unparseAlgorithmStr(SCodeUtil::statementToAlgorithmItem(inStmt.clone())?)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStmt)
}

fn fixArrayDim(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut ads: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut tree: Arc<AvlSetString::Tree>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut ads: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = ads;
    ads = fixList(inCache.clone(), inEnv.clone(), ads.clone(), tree.clone(), (std::sync::Arc::new(fixSubscript) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Subscript>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Subscript>> + 'static>));
    ads
}

fn fixSubscript(mut cache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inSub: Arc<Absyn::Subscript>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Subscript>> {
    let mut outSub: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    outSub = (::match_deref::match_deref! { match &(inSub.clone()) {
        Deref @ Absyn::Subscript::NOSUB => {
            inSub.clone()
        },
        Deref @ Absyn::Subscript::SUBSCRIPT { subscript: exp1 } => {
            let mut exp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            if (referenceEq(&exp1.clone(),&exp2.clone())) {inSub.clone()} else {Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: exp2.clone() })}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSub)
}

fn fixTypeSpec(mut cache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inTs: Arc<Absyn::TypeSpec>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTs: Arc<Absyn::TypeSpec>;
    outTs = (::match_deref::match_deref! { match &(inTs.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { path: path1, arrayDim: arrayDim1 } => {
            let mut path2: Arc<Absyn::Path>;
            let mut arrayDim2: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> = None;
            arrayDim2 = fixOption(cache.clone(), inEnv.clone(), arrayDim1.clone(), tree.clone(), (std::sync::Arc::new(fnptr!(fixArrayDim, metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Subscript>>>, Arc<AvlSetString::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Subscript>>>, Arc<AvlSetString::Tree>) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> + 'static>));
            path2 = fixPath(cache.clone(), inEnv.clone(), path1.clone(), tree.clone())?;
            if (referenceEq(&arrayDim2.clone(),&arrayDim1.clone()) && referenceEq(&path1.clone(),&path2.clone())) {inTs.clone()} else {Arc::new(Absyn::TypeSpec::TPATH { path: path2.clone(), arrayDim: arrayDim2.clone() })}
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: path1, typeSpecs: typeSpecs1, arrayDim: arrayDim1 } => {
            let mut path2: Arc<Absyn::Path>;
            let mut arrayDim2: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> = None;
            let mut typeSpecs2: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>> = metamodelica::nil();
            arrayDim2 = fixOption(cache.clone(), inEnv.clone(), arrayDim1.clone(), tree.clone(), (std::sync::Arc::new(fnptr!(fixArrayDim, metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Subscript>>>, Arc<AvlSetString::Tree>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<metamodelica::List<Arc<Absyn::Subscript>>>, Arc<AvlSetString::Tree>) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> + 'static>));
            path2 = fixPath(cache.clone(), inEnv.clone(), path1.clone(), tree.clone())?;
            typeSpecs2 = fixList(cache.clone(), inEnv.clone(), typeSpecs1.clone(), tree.clone(), (std::sync::Arc::new(fixTypeSpec) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::TypeSpec>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::TypeSpec>> + 'static>));
            if (referenceEq(&arrayDim2.clone(),&arrayDim1.clone()) && referenceEq(&path1.clone(),&path2.clone()) && referenceEq(&typeSpecs1.clone(),&typeSpecs2.clone())) {inTs.clone()} else {Arc::new(Absyn::TypeSpec::TCOMPLEX { path: path2.clone(), typeSpecs: typeSpecs2.clone(), arrayDim: arrayDim2.clone() })}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTs)
}

fn fixPath(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inPath: Arc<Absyn::Path>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = inPath.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
                    Ok(inPath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut path2: Arc<Absyn::Path>;
                    id = (AbsynUtil::pathFirstIdent(inPath.clone())?).clone();
                    let true = (AvlSetString::hasKey(tree.clone(), (id.clone()).clone())?) else { bail!("pattern mismatch") };
                    path2 = FGraph::pathStripGraphScopePrefix(inPath.clone(), inEnv.clone(), false)?;
                    Ok(path2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: Arc<Absyn::Path>;
                    Lookup::lookupClassLocal(inEnv.clone(), (AbsynUtil::pathFirstIdent(inPath.clone())?).clone())?;
                    path = FGraph::pathStripGraphScopePrefix(inPath.clone(), inEnv.clone(), false)?;
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: Arc<Absyn::Path>;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cache, path) = Inst::makeFullyQualified(inCache.clone().borrow()[(1-1) as usize].clone(), inEnv.clone(), inPath.clone())?;
                    path = FGraph::pathStripGraphScopePrefix(path.clone(), inEnv.clone(), false)?;
                    {let _arr = inCache.clone(); _arr.borrow_mut()[(1-1) as usize] = cache.clone(); _arr};
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: Arc<Absyn::Path>;
                    path = FGraph::pathStripGraphScopePrefix(inPath.clone(), inEnv.clone(), false)?;
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

fn lookupVarNoErrorMessage(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ident: ArcStr) -> Result<(FCore::Graph, ArcStr)> {
    let mut outEnv: FCore::Graph;
    let mut id: ArcStr = arcstr::literal!("");
    match '__try0: {
        ErrorExt::setCheckpoint((literal!("InstExtends.lookupVarNoErrorMessage")).clone());
        (_, _, _, _, _, _, outEnv, _, id) = unwrap_break_err!(Lookup::lookupVarIdent(inCache.clone(), inEnv.clone(), (ident.clone()).clone(), metamodelica::nil()), '__try0);
        ErrorExt::rollBack((literal!("InstExtends.lookupVarNoErrorMessage")).clone());
        Ok::<_, anyhow::Error>((id.clone(), outEnv.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            id = __try0_o0;
            outEnv = __try0_o1;
        }
        Err(_) => {
            ErrorExt::rollBack((literal!("InstExtends.lookupVarNoErrorMessage")).clone());
            bail!("fail");
        }
    }
    Ok((outEnv, id))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fixCref(mut cache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inCref: Arc<Absyn::ComponentRef>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    outCref = 'mc: {
        let __mc_input = (inEnv.clone(), inCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }) => {
                    let mut env = (*env).clone();
                    env = FGraph::topScope(inEnv.clone())?;
                    Ok(fixCref(cache.clone(), env.clone(), var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), tree.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, cref) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut cref = (*cref).clone();
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    let true = (AvlSetString::hasKey(tree.clone(), (id.clone()).clone())?) else { bail!("pattern mismatch") };
                    cref = FGraph::crefStripGraphScopePrefix(cref.clone(), env.clone(), false)?;
                    cref = if (AbsynUtil::crefEqual(cref.clone(), inCref.clone())) {inCref.clone()} else {cref.clone()};
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, cref) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut denv: FCore::Graph;
                    let mut cref = (*cref).clone();
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    (denv, id) = lookupVarNoErrorMessage(cache.clone().borrow()[(1-1) as usize].clone(), env.clone(), (id.clone()).clone())?;
                    denv = FGraph::openScope(denv.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, (id.clone()).clone(), None)?;
                    cref = AbsynUtil::crefReplaceFirstIdent(cref.clone(), FGraph::getGraphName(denv.clone())?)?;
                    cref = FGraph::crefStripGraphScopePrefix(cref.clone(), env.clone(), false)?;
                    cref = if (AbsynUtil::crefEqual(cref.clone(), inCref.clone())) {inCref.clone()} else {cref.clone()};
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, cref) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut denv: FCore::Graph;
                    let mut c: Arc<SCode::Element>;
                    let mut cref = (*cref).clone();
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    (_, c, denv) = Lookup::lookupClassIdent(cache.clone().borrow()[(1-1) as usize].clone(), env.clone(), (id.clone()).clone(), None)?;
                    id = (SCodeUtil::getElementName(c.clone())?).clone();
                    denv = FGraph::openScope(denv.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, (id.clone()).clone(), None)?;
                    cref = AbsynUtil::crefReplaceFirstIdent(cref.clone(), FGraph::getGraphName(denv.clone())?)?;
                    cref = FGraph::crefStripGraphScopePrefix(cref.clone(), env.clone(), false)?;
                    cref = if (AbsynUtil::crefEqual(cref.clone(), inCref.clone())) {inCref.clone()} else {cref.clone()};
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

fn fixModifications(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inMod: Arc<SCode::Mod>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = inMod.clone();
    outMod = 'mc: {
        let __mc_input = outMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::NOMOD => {
                    Ok(inMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { .. } => {
                    let mut subModLst: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    let mut exp: Option<Arc<Absyn::Exp>> = None;
                    let mut outMod: Arc<SCode::Mod> = outMod.clone();
                    subModLst = fixList(inCache.clone(), inEnv.clone(), var_field!((*outMod).subModLst, SCode::Mod::MOD).clone(), tree.clone(), (std::sync::Arc::new(fixSubMod) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<SCode::SubMod>, Arc<AvlSetString::Tree>) -> Result<Arc<SCode::SubMod>> + 'static>));
                    if !(referenceEq(&var_field!((*outMod).subModLst, SCode::Mod::MOD).clone(),&subModLst.clone())) {
                        assign_variant_field!(outMod => SCode::Mod::MOD; subModLst = subModLst.clone());
                    }
                    exp = fixOption(inCache.clone(), inEnv.clone(), var_field!((*outMod).binding, SCode::Mod::MOD).clone(), tree.clone(), (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<Absyn::Exp>, Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> + 'static>));
                    if !(referenceEq(&exp.clone(),&var_field!((*outMod).binding, SCode::Mod::MOD).clone())) {
                        assign_variant_field!(outMod => SCode::Mod::MOD; binding = exp.clone());
                    }
                    Ok(outMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { .. }, .. } => {
                    let mut e: Arc<SCode::Element>;
                    let mut outMod: Arc<SCode::Mod> = outMod.clone();
                    e = fixElement(inCache.clone(), inEnv.clone(), var_field!((*outMod).element, SCode::Mod::REDECL).clone(), tree.clone())?;
                    if !(referenceEq(&e.clone(),&var_field!((*outMod).element, SCode::Mod::REDECL).clone())) {
                        assign_variant_field!(outMod => SCode::Mod::REDECL; element = e.clone());
                    }
                    Ok(outMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { element: e @ Deref @ SCode::Element::CLASS { classDef: cdef, .. }, .. } => {
                    let mut e = (*e).clone();
                    let mut cdef = (*cdef).clone();
                    let mut outMod: Arc<SCode::Mod> = outMod.clone();
                    cdef = fixClassdef(inCache.clone(), inEnv.clone(), cdef.clone(), tree.clone())?;
                    if !(referenceEq(&cdef.clone(),&var_field!((*e).classDef, SCode::Element::CLASS).clone())) {
                        assign_variant_field!(e => SCode::Element::CLASS; classDef = cdef.clone());
                        assign_variant_field!(outMod => SCode::Mod::REDECL; element = e.clone());
                    }
                    Ok(outMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixModifications failed: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn fixSubMod(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut subMod: Arc<SCode::SubMod>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<SCode::SubMod>> {
    let mut subMod: Arc<SCode::SubMod> = subMod;
    let mut ident: ArcStr = arcstr::literal!("");
    let mut mod1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut mod2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ SCode::SubMod { ident: __pa0, r#mod: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ident = __pa0.clone();
    mod1 = __pa1.clone();
    mod2 = fixModifications(inCache.clone(), inEnv.clone(), mod1.clone(), tree.clone())?;
    if !(referenceEq(&mod1.clone(),&mod2.clone())) {
        subMod = Arc::new(SCode::SubMod { ident: (ident.clone()).clone(), r#mod: mod2.clone() });
    }
    Ok(subMod)
}

fn fixExp(mut cache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut tree: Arc<AvlSetString::Tree>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (outExp, _) = AbsynUtil::traverseExp(inExp.clone(), (std::sync::Arc::new(fixExpTraverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<AvlSetString::Tree>)) -> Result<(Arc<Absyn::Exp>, (metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<AvlSetString::Tree>))> + 'static>), (cache.clone(), inEnv.clone(), tree.clone()))?;
    Ok(outExp)
}

fn fixExpTraverse(mut exp: Arc<Absyn::Exp>, mut tpl: (metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<AvlSetString::Tree>)) -> Result<(Arc<Absyn::Exp>, (metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<AvlSetString::Tree>))> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut tpl: (metamodelica::Array<FCore::Cache>, FCore::Graph, Arc<AvlSetString::Tree>) = tpl;
    exp = (::match_deref::match_deref! { match &((exp.clone(), tpl.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: cref }, (cache, env, tree)) => {
            let mut cref1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            cref1 = fixCref(cache.clone(), env.clone(), cref.clone(), tree.clone())?;
            if (referenceEq(&cref.clone(),&cref1.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::CREF { componentRef: cref1.clone() })}
        },
        (Deref @ Absyn::Exp::CALL { function_: cref, .. }, (cache, env, tree)) => {
            let mut cref1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            cref1 = fixCref(cache.clone(), env.clone(), cref.clone(), tree.clone())?;
            if (referenceEq(&cref.clone(),&cref1.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::CALL { function_: cref1.clone(), functionArgs: var_field!((*exp).functionArgs, Absyn::Exp::CALL).clone(), typeVars: var_field!((*exp).typeVars, Absyn::Exp::CALL).clone() })}
        },
        (Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, functionArgs: fargs }, (cache, env, tree)) => {
            let mut cref1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            cref1 = fixCref(cache.clone(), env.clone(), cref.clone(), tree.clone())?;
            if (referenceEq(&cref.clone(),&cref1.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::PARTEVALFUNCTION { function_: cref1.clone(), functionArgs: fargs.clone() })}
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, tpl))
}

fn fixOption<Type_A: Clone + 'static + PartialEq>(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inA: Option<Type_A>, mut tree: Arc<AvlSetString::Tree>, mut fixA: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>) -> Option<Type_A> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>;

    let mut outA: Option<Type_A> = None;
    outA = (match inA.clone() {
        None => {
            inA.clone()
        },
        Some(mut A1) => {
            let mut A2: Type_A;
            A2 = fixA(inCache.clone(), inEnv.clone(), A1.clone(), tree.clone()).unwrap();
            if (referenceEq(&A1.clone(),&A2.clone())) {inA.clone()} else {Some(A2.clone())}
        },
    });
    outA
}

fn fixList<Type_A: Clone + 'static + PartialEq>(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inA: Arc<metamodelica::List<Type_A>>, mut tree: Arc<AvlSetString::Tree>, mut fixA: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>) -> Arc<metamodelica::List<Type_A>> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>;

    let mut outA: Arc<metamodelica::List<Type_A>> = metamodelica::nil();
    if inA.clone().is_empty() {
        outA = inA.clone();
        return outA.clone();
    }
    outA = List::mapCheckReferenceEq(inA.clone(), Arc::new({ let __pe_b0 = inCache.clone(); let __pe_b1 = inEnv.clone(); let __pe_b3 = tree.clone(); move |__pe_a2| fixA(__pe_b0.clone(), __pe_b1.clone(), __pe_a2, __pe_b3.clone()) }));
    outA
}

fn fixListList<Type_A: Clone + 'static + PartialEq>(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inA: Arc<metamodelica::List<Arc<metamodelica::List<Type_A>>>>, mut tree: Arc<AvlSetString::Tree>, mut fixA: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>) -> Arc<metamodelica::List<Arc<metamodelica::List<Type_A>>>> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>;

    let mut outA: Arc<metamodelica::List<Arc<metamodelica::List<Type_A>>>> = metamodelica::nil();
    if inA.clone().is_empty() {
        outA = metamodelica::nil();
        return outA.clone();
    }
    outA = List::mapCheckReferenceEq(inA.clone(), Arc::new({ let __pe_b0 = inCache.clone(); let __pe_b1 = inEnv.clone(); let __pe_b3 = tree.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, _, Arc<AvlSetString::Tree>) -> Result<_> + 'static> = fixA.clone(); move |__pe_a2| Ok(fixList(__pe_b0.clone(), __pe_b1.clone(), __pe_a2, __pe_b3.clone(), __pe_b4.clone())) }));
    outA
}

fn fixListTuple2<Type_A: Clone + 'static + PartialEq, Type_B: Clone + 'static + PartialEq>(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut inRest: Arc<metamodelica::List<(Type_A, Type_B)>>, mut tree: Arc<AvlSetString::Tree>, mut fixA: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>, mut fixB: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_B, Arc<AvlSetString::Tree>) -> Result<Type_B> + 'static>) -> Arc<metamodelica::List<(Type_A, Type_B)>> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>;

    pub type FixBFn<Type_B: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_B, Arc<AvlSetString::Tree>) -> Result<Type_B> + 'static>;

    let mut outA: Arc<metamodelica::List<(Type_A, Type_B)>> = metamodelica::nil();
    outA = fixList(inCache.clone(), inEnv.clone(), inRest.clone(), tree.clone(), Arc::new({ let __pe_b4: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, _, Arc<AvlSetString::Tree>) -> Result<_> + 'static> = fixA.clone(); let __pe_b5: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, _, Arc<AvlSetString::Tree>) -> Result<_> + 'static> = fixB.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| Ok(fixTuple2(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone(), __pe_b5.clone())) }));
    outA
}

fn fixTuple2<Type_A: Clone + 'static + PartialEq, Type_B: Clone + 'static + PartialEq>(mut inCache: metamodelica::Array<FCore::Cache>, mut inEnv: FCore::Graph, mut tpl: (Type_A, Type_B), mut tree: Arc<AvlSetString::Tree>, mut fixA: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>, mut fixB: Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_B, Arc<AvlSetString::Tree>) -> Result<Type_B> + 'static>) -> (Type_A, Type_B) {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_A, Arc<AvlSetString::Tree>) -> Result<Type_A> + 'static>;

    pub type FixBFn<Type_B: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, Type_B, Arc<AvlSetString::Tree>) -> Result<Type_B> + 'static>;

    let mut tpl: (Type_A, Type_B) = tpl;
    let mut a1: Type_A;
    let mut a2: Type_A;
    let mut b1: Type_B;
    let mut b2: Type_B;
    (a1, b1) = tpl.clone();
    a2 = fixA(inCache.clone(), inEnv.clone(), a1.clone(), tree.clone()).unwrap();
    b2 = fixB(inCache.clone(), inEnv.clone(), b1.clone(), tree.clone()).unwrap();
    if !(referenceEq(&a1.clone(),&a2.clone()) && referenceEq(&b1.clone(),&b2.clone())) {
        tpl = (a2.clone(), b2.clone());
    }
    tpl
}

