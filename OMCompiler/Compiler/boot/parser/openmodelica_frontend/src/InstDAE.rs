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

use crate::ComponentReference;
use crate::DAEDump;
use crate::DAEUtil;
use crate::FCore;
use crate::FGraph;
use crate::InnerOuter;
use crate::InstBinding;
use crate::InstUtil;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

pub fn daeDeclare(mut inCache: FCore::Cache, mut inParentEnv: FCore::Graph, mut inClassEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut inState: ClassInf::State, mut inType: Arc<DAE::Type>, mut inAttributes: SCode::Attributes, mut visibility: SCode::Visibility, mut inBinding: Option<Arc<DAE::Exp>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inStartValue: Option<Arc<DAE::Exp>>, mut inVarAttr: Option<Arc<DAE::VariableAttributes>>, mut inComment: Option<Arc<SCode::Comment>>, mut io: Absyn::InnerOuter, mut finalPrefix: SCode::Final, mut source: Arc<DAE::ElementSource>, mut declareComplexVars: bool) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = (inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), inComponentRef.clone(), inState.clone(), inType.clone(), inAttributes.clone(), visibility.clone(), inBinding.clone(), inInstDims.clone(), inStartValue.clone(), inVarAttr.clone(), inComment.clone(), io.clone(), finalPrefix.clone(), source.clone(), declareComplexVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, vn, ci_state, ty, SCode::Attributes { direction: dir, variability: var, parallelism: prl, connectorType: ct, .. }, vis, e, inst_dims, start, dae_var_attr, comment, _, _, _, _) => {
                    let mut ct1: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
                    let mut dae: DAE::DAElist;
                    let mut daeParallelism: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
                    let mut info: SourceInfo;
                    let mut vk: DAE::VarKind = DAE::VarKind::CONST;
                    let mut vd: DAE::VarDirection = DAE::VarDirection::BIDIR;
                    let mut vv: DAE::VarVisibility = DAE::VarVisibility::PROTECTED;
                    let mut dae_var_attr = (*dae_var_attr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(source.clone()) {
                        Deref @ DAE::ElementSource { info: __pa0, partOfLst: _, instance: _, connectEquationOptLst: _, typeLst: _, operations: _, comment: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa0.clone();
                    ct1 = DAEUtil::toConnectorType(ct.clone(), ci_state.clone());
                    daeParallelism = DAEUtil::toDaeParallelism(vn.clone(), prl.clone(), ci_state.clone(), info.clone())?;
                    vk = InstUtil::makeDaeVariability(var.clone())?;
                    vd = InstUtil::makeDaeDirection(dir.clone())?;
                    vv = InstUtil::makeDaeProt(vis.clone())?;
                    dae_var_attr = DAEUtil::setFinalAttr(dae_var_attr.clone(), SCodeUtil::finalBool(finalPrefix.clone())?)?;
                    dae = daeDeclare2(vn.clone(), ty.clone(), ct1.clone(), vk.clone(), vd.clone(), daeParallelism.clone(), vv.clone(), e.clone(), inst_dims.clone(), start.clone(), dae_var_attr.clone(), comment.clone(), io.clone(), source.clone(), declareComplexVars.clone())?;
                    showDAE(inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), inState.clone(), dae.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Inst.daeDeclare failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDae)
}

fn showDAE(mut inCache: FCore::Cache, mut inParentEnv: FCore::Graph, mut inClassEnv: FCore::Graph, mut inState: ClassInf::State, mut inDAE: DAE::DAElist) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inCache.clone(), inParentEnv.clone(), inClassEnv.clone(), inState.clone(), inDAE.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (Flags::isSet(Flags::SHOW_DAE_GENERATION.clone())?) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut sstr: ArcStr = arcstr::literal!("");
            let mut comp: Arc<DAE::Element>;
            let mut dae: DAE::DAElist;
            let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            els = DAEUtil::daeElements(inDAE.clone())?;
            sstr = (ClassInfUtil::printStateStr(inState.clone())).clone();
            sstr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*sstr.clone()); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone();
            comp = Arc::new(DAE::Element::COMP { ident: (sstr.clone()).clone(), dAElist: els.clone(), source: DAE::emptyElementSource.clone(), comment: None });
            dae = DAE::DAElist { elementLst: list![comp.clone()] };
            r#str = (if (System::getPartialInstantiation()) {literal!(" partial")} else {literal!(" full")}).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE: parent: ")); __mm_s.push_str(&*FGraph::getGraphNameStr(inParentEnv.clone())?); __mm_s.push_str(&*literal!(" class: ")); __mm_s.push_str(&*FGraph::getGraphNameStr(inClassEnv.clone())?); __mm_s.push_str(&*literal!(" state: ")); __mm_s.push_str(&*sstr.clone()); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*DAEDump::dumpStr(dae.clone(), AvlTreePathFunction::Tree::EMPTY()?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (if (System::getPartialInstantiation()) {literal!(" partial")} else {literal!(" full")}).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE: ")); __mm_s.push_str(&*ClassInfUtil::printStateStr(inState.clone())); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" - could not print\n")); ArcStr::from(__mm_s) }).clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn daeDeclare2(mut inComponentRef: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inConnectorType: Arc<DAE::ConnectorType>, mut inVarKind: DAE::VarKind, mut inVarDirection: DAE::VarDirection, mut inParallelism: DAE::VarParallelism, mut protection: DAE::VarVisibility, mut inExpExpOption: Option<Arc<DAE::Exp>>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inStartValue: Option<Arc<DAE::Exp>>, mut inAttr: Option<Arc<DAE::VariableAttributes>>, mut inComment: Option<Arc<SCode::Comment>>, mut io: Absyn::InnerOuter, mut source: Arc<DAE::ElementSource>, mut declareComplexVars: bool) -> Result<DAE::DAElist> {
    let mut outDAe: DAE::DAElist;
    outDAe = 'mc: {
        let __mc_input = (inComponentRef.clone(), inType.clone(), inConnectorType.clone(), inVarKind.clone(), inVarDirection.clone(), inParallelism.clone(), protection.clone(), inExpExpOption.clone(), inInstDims.clone(), inStartValue.clone(), inAttr.clone(), inComment.clone(), io.clone(), source.clone(), declareComplexVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_INTEGER { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: DAE::T_INTEGER_DEFAULT.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_REAL { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: DAE::T_REAL_DEFAULT.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_BOOL { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: DAE::T_BOOL_DEFAULT.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_CLOCK { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: DAE::T_CLOCK_DEFAULT.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_STRING { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: DAE::T_STRING_DEFAULT.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(DAE::emptyDae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_CODE { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: inType.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, ty @ Deref @ DAE::Type::T_ENUMERATION { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: ty.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: ty.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. }, ct, kind, dir, daePrl, prot, e, inst_dims, start, dae_var_attr, comment, _, _, _) => {
                    let mut dae: DAE::DAElist;
                    let mut dae_var_attr = (*dae_var_attr).clone();
                    (_, dae_var_attr) = InstBinding::instDaeVariableAttributes(FCore::emptyCache(), FGraph::empty(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), tp.clone(), metamodelica::nil())?;
                    dae = daeDeclare2(vn.clone(), tp.clone(), ct.clone(), kind.clone(), dir.clone(), daePrl.clone(), prot.clone(), e.clone(), inst_dims.clone(), start.clone(), dae_var_attr.clone(), comment.clone(), io.clone(), source.clone(), declareComplexVars.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { .. }, tail: Deref @ metamodelica::List::Nil } }, ct, kind, dir, daePrl, prot, e, inst_dims, start, dae_var_attr, comment, _, _, _) => {
                    let mut dae: DAE::DAElist;
                    dae = daeDeclare2(vn.clone(), tp.clone(), ct.clone(), kind.clone(), dir.clone(), daePrl.clone(), prot.clone(), e.clone(), inst_dims.clone(), start.clone(), dae_var_attr.clone(), comment.clone(), io.clone(), source.clone(), declareComplexVars.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_ARRAY { ty: tp, .. }, ct, kind, dir, daePrl, prot, e, inst_dims, start, dae_var_attr, comment, _, _, _) => {
                    let mut dae: DAE::DAElist;
                    let false = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    dae = daeDeclare2(vn.clone(), tp.clone(), ct.clone(), kind.clone(), dir.clone(), daePrl.clone(), prot.clone(), e.clone(), inst_dims.clone(), start.clone(), dae_var_attr.clone(), comment.clone(), io.clone(), source.clone(), declareComplexVars.clone())?;
                    Ok(dae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: Deref @ metamodelica::List::Nil }, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo;
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    s = (ComponentReferenceBasics::printComponentRefStr(vn.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Error::addSourceMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(s.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, true) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: ty.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, tty @ Deref @ DAE::Type::T_FUNCTION { .. }, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut path: Arc<Absyn::Path>;
                    let mut tty = (*tty).clone();
                    finst_dims = List::flatten(inst_dims.clone());
                    path = ComponentReference::crefToPath(vn.clone())?;
                    let __owned_variant_path_0 = path.clone();
                    if let DAE::Type::T_FUNCTION { path, .. } = &mut tty {
                        *path = __owned_variant_path_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than DAE::Type::T_FUNCTION"); }
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: tty.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vn, ty, ct, kind, dir, daePrl, prot, e, inst_dims, _, dae_var_attr, comment, _, _, _) => {
                    let mut finst_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    let true = (Types::isBoxedType(ty.clone())) else { bail!("pattern mismatch") };
                    finst_dims = List::flatten(inst_dims.clone());
                    Ok(DAE::DAElist { elementLst: list![Arc::new(DAE::Element::VAR { componentRef: vn.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: daePrl.clone(), protection: prot.clone(), ty: ty.clone(), binding: e.clone(), dims: finst_dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: false })] })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(DAE::emptyDae.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDAe)
}

