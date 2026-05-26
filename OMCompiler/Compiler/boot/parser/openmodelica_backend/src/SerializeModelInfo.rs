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

use crate::BackendDAE;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference::writeCref;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::Expression;
use openmodelica_frontend::PrefixUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr as expStr;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Autoconf;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File::Escape::JSON;
use openmodelica_util::File;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn serialize(mut code: SimCode::SimCode, mut withOperations: bool) -> Result<ArcStr> {
    let mut fileName: ArcStr = arcstr::literal!("");
    let (true, __pa0) = (serializeWork(code.clone(), withOperations.clone())?) else { bail!("pattern mismatch") };
    fileName = __pa0.clone();
    Ok(fileName)
}

fn serializeWork(mut code: SimCode::SimCode, mut withOperations: bool) -> Result<(bool, ArcStr)> {
    let mut success: bool = false;
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut file: File::File = File::File(File::noReference())?;
    (success, fileName) = 'mc: {
        let __mc_input = code.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let SimCode::SimCode { modelInfo: mut mi @ SimCode::ModelInfo { .. }, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut eqsName: ArcStr = arcstr::literal!("");
            let mut eqsLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut fileName: ArcStr = fileName.clone();
            if Config::simCodeTarget()? == literal!("omsic") {
                fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*code.fullPathPrefix.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); __mm_s.push_str(&*code.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) }).clone();
            } else {
                fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*code.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) }).clone();
            }
            File::open(file.clone(), (fileName.clone()).clone(), File::Mode::Write.clone());
            File::write(file.clone(), (literal!("{\"format\":\"Transformational debugger info\",\"version\":1,\n\"info\":{\"name\":")).clone());
            serializePath(file.clone(), mi.name.clone())?;
            File::write(file.clone(), (literal!(",\"description\":\"")).clone());
            File::writeEscape(file.clone(), (mi.description.clone()).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"},\n\"variables\":{\n")).clone());
            serializeVars(file.clone(), mi.vars.clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("\n},\n\"equations\":[")).clone());
            File::write(file.clone(), (literal!("{\"eqIndex\":0,\"tag\":\"dummy\"}")).clone());
            for mut tpl in &*list![(literal!("initial"), code.initialEquations.clone()), (literal!("initial-lambda0"), code.initialEquations_lambda0.clone()), (literal!("removed-initial"), code.removedInitialEquations.clone()), (literal!("regular"), code.allEquations.clone()), (literal!("synchronous"), SimCodeUtil::getClockedEquations(SimCodeUtil::getSubPartitions(code.clockedPartitions.clone()))), (literal!("start"), code.startValueEquations.clone()), (literal!("nominal"), code.nominalValueEquations.clone()), (literal!("min"), code.minValueEquations.clone()), (literal!("max"), code.maxValueEquations.clone()), (literal!("parameter"), code.parameterEquations.clone()), (literal!("assertions"), code.algorithmAndEquationAsserts.clone()), (literal!("inline"), code.inlineEquations.clone()), (literal!("residuals"), List::flatten(SimCodeUtil::getSimCodeDAEModeDataEqns(code.daeModeData.clone())?)), (literal!("jacobian"), code.jacobianEquations.clone())] {
                let mut tpl = tpl.clone();
                (eqsName, eqsLst) = tpl.clone();
                for mut eq in &*SimCodeUtil::sortEqSystems(eqsLst.clone())? {
                    let mut eq = eq.clone();
                    serializeEquation(file.clone(), eq.clone(), (eqsName.clone()).clone(), withOperations.clone(), 0, false, AssignType::NORMAL.clone())?;
                }
            }
            File::write(file.clone(), (literal!("\n],\n\"functions\":[")).clone());
            serializeList(file.clone(), mi.functions.clone(), Arc::new(serializeFunction), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("\n]\n}")).clone());
            Ok((true, fileName.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("SerializeModelInfo.serialize failed")).clone(), metamodelica::sourceInfo!())?;
            Ok((false, literal!("")))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((success, fileName))
}

fn serializeVars(mut file: File::File, mut vars: SimCodeVar::SimVars, mut withOperations: bool) -> Result<()> {
    let mut b: bool = false;
    b = serializeVarsHelp(file.clone(), vars.stateVars.clone(), withOperations.clone(), true)?;
    b = serializeVarsHelp(file.clone(), vars.derivativeVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.algVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.intAlgVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.boolAlgVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.inputVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.intAliasVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.boolAliasVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.paramVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.intParamVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.boolParamVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.stringAlgVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.stringAliasVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.extObjVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.constVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.intConstVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.boolConstVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.stringConstVars.clone(), withOperations.clone(), b.clone())?;
    b = serializeVarsHelp(file.clone(), vars.jacobianVars.clone(), withOperations.clone(), b.clone())?;
    let _ = serializeVarsHelp(file.clone(), vars.sensitivityVars.clone(), withOperations.clone(), b.clone())?;
    Ok(())
}

fn serializeVarsHelp(mut file: File::File, mut vars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut withOperations: bool, mut inFirst: bool) -> Result<bool> {
    let mut outFirst: bool = inFirst.clone() && vars.clone().is_empty();
    serializeList(file.clone(), vars.clone(), Arc::new({ let __pe_b2 = withOperations.clone(); move |__pe_a0, __pe_a1| serializeVar(__pe_a0, __pe_a1, __pe_b2.clone()) }), !(inFirst.clone()), (literal!(",\n")).clone())?;
    Ok(outFirst)
}

fn serializeVar(mut file: File::File, mut var: SimCodeVar::SimVar, mut withOperations: bool) -> Result<()> {
    File::write(file.clone(), (literal!("\"")).clone());
    writeCref(file.clone(), var.name.clone(), escape=JSON.clone())?;
    File::write(file.clone(), (literal!("\":{\"comment\":\"")).clone());
    File::writeEscape(file.clone(), (var.comment.clone()).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\",\"kind\":\"")).clone());
    File::write(file.clone(), (varKindString(var.varKind.clone(), var.clone())?).clone());
    File::write(file.clone(), (literal!("\"")).clone());
    serializeTypeName(file.clone(), var.type_.clone());
    File::write(file.clone(), (literal!(",\"unit\":\"")).clone());
    File::writeEscape(file.clone(), (var.unit.clone()).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\",\"displayUnit\":\"")).clone());
    File::writeEscape(file.clone(), (var.displayUnit.clone()).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\",\"source\":")).clone());
    serializeSource(file.clone(), var.source.clone(), withOperations.clone())?;
    File::write(file.clone(), (literal!(",\"index\":")).clone());
    File::writeInt(file.clone(), var.index.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("}")).clone());
    Ok(())
}

fn serializeTypeName(mut file: File::File, mut ty: Arc<DAE::Type>) -> () {
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            File::write(file.clone(), (literal!(",\"type\":\"Real\"")).clone());
            ()
        },
        Deref @ DAE::Type::T_INTEGER { .. } => {
            File::write(file.clone(), (literal!(",\"type\":\"Integer\"")).clone());
            ()
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            File::write(file.clone(), (literal!(",\"type\":\"Boolean\"")).clone());
            ()
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            File::write(file.clone(), (literal!(",\"type\":\"String\"")).clone());
            ()
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            File::write(file.clone(), (literal!(",\"type\":\"Enumeration\"")).clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn serializeSource(mut file: File::File, mut source: Arc<DAE::ElementSource>, mut withOperations: bool) -> Result<()> {
    let mut info: SourceInfo;
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut typeLst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut partOfLst: Arc<metamodelica::List<Absyn::Within>> = metamodelica::nil();
    let mut instance: Arc<DAE::ComponentPrefix> = Arc::new(DAE::ComponentPrefix::NOCOMPPRE);
    let mut i: i32 = 0;
    let mut operations: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(source.clone()) {
        Deref @ DAE::ElementSource { operations: __pa0, partOfLst: __pa1, instance: __pa2, info: __pa3, typeLst: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    operations = __pa0.clone();
    partOfLst = __pa1.clone();
    instance = __pa2.clone();
    info = __pa3.clone();
    typeLst = __pa4.clone();
    File::write(file.clone(), (literal!("{")).clone());
    serializeInfo(file.clone(), info.clone());
    if !(partOfLst.clone().is_empty()) {
        paths = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut w in (partOfLst.clone()).into_iter().cloned() {
            if !((match w.clone() {
        Absyn::Within::TOP => false,
        _ => true,
    })) { continue; }
            let __x = (match w.clone() {
        Absyn::Within::WITHIN { .. } => var_field!(w.path, Absyn::Within::WITHIN).clone(),
        _ => bail!("match: no arm matched"),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        File::write(file.clone(), (literal!(",\"within\":[")).clone());
        serializeList(file.clone(), paths.clone(), Arc::new(serializePath), false, (literal!(",")).clone())?;
        File::write(file.clone(), (literal!("]")).clone());
    }
    let () = (::match_deref::match_deref! { match &(instance.clone()) {
        Deref @ DAE::ComponentPrefix::NOCOMPPRE => (),
        Deref @ DAE::ComponentPrefix::PRE { .. } => {
            File::write(file.clone(), (literal!(",\"instance\":\"")).clone());
            PrefixUtil::writeComponentPrefix(file.clone(), instance.clone(), JSON.clone())?;
            File::write(file.clone(), (literal!("\"")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    if !(typeLst.clone().is_empty()) {
        File::write(file.clone(), (literal!(",\"typeLst\":[")).clone());
        serializeList(file.clone(), typeLst.clone(), Arc::new(serializePath), false, (literal!(",")).clone())?;
        File::write(file.clone(), (literal!("]")).clone());
    }
    if withOperations.clone() && !(operations.clone().is_empty()) {
        File::write(file.clone(), (literal!(",\"operations\":[")).clone());
        serializeList(file.clone(), operations.clone(), Arc::new(serializeOperation), false, (literal!(",")).clone())?;
        File::write(file.clone(), (literal!("]")).clone());
    }
    File::write(file.clone(), (literal!("}")).clone());
    Ok(())
}

fn serializeInfo(mut file: File::File, mut info: SourceInfo) -> () {
    File::write(file.clone(), (literal!("\"info\":{\"file\":\"")).clone());
    File::writeEscape(file.clone(), info.fileName.clone(), JSON.clone());
    File::write(file.clone(), (literal!("\",\"lineStart\":")).clone());
    File::writeInt(file.clone(), info.lineNumberStart.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!(",\"lineEnd\":")).clone());
    File::writeInt(file.clone(), info.lineNumberEnd.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!(",\"colStart\":")).clone());
    File::writeInt(file.clone(), info.columnNumberStart.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!(",\"colEnd\":")).clone());
    File::writeInt(file.clone(), info.columnNumberEnd.clone(), (literal!("%d")).clone());
    File::write(file.clone(), (literal!("}")).clone());
    ()
}

fn serializeOperation(mut file: File::File, mut op: Arc<DAE::SymbolicOperation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(op.clone()) {
        Deref @ DAE::SymbolicOperation::FLATTEN { dae: Some(elt), .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after\",\"display\":\"flattening\",\"data\":[\"")).clone());
            File::writeEscape(file.clone(), (System::trim((SCodeDump::equationStr(var_field!((*op).scode, DAE::SymbolicOperation::FLATTEN).clone(), SCodeDump::defaultOptions.clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\",\"")).clone());
            File::writeEscape(file.clone(), (System::trim((DAEDump::dumpEquationStr(elt.clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::FLATTEN { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"info\",\"display\":\"scode\",\"data\":[\"")).clone());
            File::writeEscape(file.clone(), (System::trim((SCodeDump::equationStr(var_field!((*op).scode, DAE::SymbolicOperation::FLATTEN).clone(), SCodeDump::defaultOptions.clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::SIMPLIFY { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after\",\"display\":\"simplify\",\"data\":[\"")).clone());
            writeEqExpStr(file.clone(), var_field!((*op).before, DAE::SymbolicOperation::SIMPLIFY).clone())?;
            File::write(file.clone(), (literal!("\",\"")).clone());
            writeEqExpStr(file.clone(), var_field!((*op).after, DAE::SymbolicOperation::SIMPLIFY).clone())?;
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_INLINE { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after\",\"display\":\"inline\",\"data\":[\"")).clone());
            writeEqExpStr(file.clone(), var_field!((*op).before, DAE::SymbolicOperation::OP_INLINE).clone())?;
            File::write(file.clone(), (literal!("\",\"")).clone());
            writeEqExpStr(file.clone(), var_field!((*op).after, DAE::SymbolicOperation::OP_INLINE).clone())?;
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::SOLVE { assertConds: Deref @ metamodelica::List::Nil, .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after\",\"display\":\"solved\",\"data\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).exp1, DAE::SymbolicOperation::SOLVE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!(" = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).exp2, DAE::SymbolicOperation::SOLVE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\",\"")).clone());
            writeCref(file.clone(), var_field!((*op).cr, DAE::SymbolicOperation::SOLVE).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!(" = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).res, DAE::SymbolicOperation::SOLVE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::SOLVE { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after-assert\",\"display\":\"solved\",\"data\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).exp1, DAE::SymbolicOperation::SOLVE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!(" = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).exp2, DAE::SymbolicOperation::SOLVE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\",\"")).clone());
            writeCref(file.clone(), var_field!((*op).cr, DAE::SymbolicOperation::SOLVE).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!(" = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).res, DAE::SymbolicOperation::SOLVE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"")).clone());
            serializeList(file.clone(), var_field!((*op).assertConds, DAE::SymbolicOperation::SOLVE).clone(), Arc::new(serializeExp), true, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_RESIDUAL { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after\",\"display\":\"residual\",\"data\":[")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).e1, DAE::SymbolicOperation::OP_RESIDUAL).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!(" = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).e2, DAE::SymbolicOperation::OP_RESIDUAL).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!(",\"0 = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).e, DAE::SymbolicOperation::OP_RESIDUAL).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::SUBSTITUTION { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"chain\",\"display\":\"substitution\",\"data\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).source, DAE::SymbolicOperation::SUBSTITUTION).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"")).clone());
            serializeList(file.clone(), var_field!((*op).substitutions, DAE::SymbolicOperation::SUBSTITUTION).clone(), Arc::new(serializeExp), true, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::SOLVED { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"info\",\"display\":\"solved\",\"data\":[\"")).clone());
            writeCref(file.clone(), var_field!((*op).cr, DAE::SymbolicOperation::SOLVED).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!(" = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).exp, DAE::SymbolicOperation::SOLVED).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_DIFFERENTIATE { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after\",\"display\":\"differentiate d/d")).clone());
            writeCref(file.clone(), var_field!((*op).cr, DAE::SymbolicOperation::OP_DIFFERENTIATE).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\",\"data\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).before, DAE::SymbolicOperation::OP_DIFFERENTIATE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\",\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*op).after, DAE::SymbolicOperation::OP_DIFFERENTIATE).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_SCALARIZE { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"before-after\",\"display\":\"scalarize [")).clone());
            File::write(file.clone(), (intString(var_field!((*op).index, DAE::SymbolicOperation::OP_SCALARIZE).clone())).clone());
            File::write(file.clone(), (literal!("]\",\"data\":[\"")).clone());
            writeEqExpStr(file.clone(), var_field!((*op).before, DAE::SymbolicOperation::OP_SCALARIZE).clone())?;
            File::write(file.clone(), (literal!("\",\"")).clone());
            writeEqExpStr(file.clone(), var_field!((*op).after, DAE::SymbolicOperation::OP_SCALARIZE).clone())?;
            File::write(file.clone(), (literal!("\"]}")).clone());
            ()
        },
        Deref @ DAE::SymbolicOperation::NEW_DUMMY_DER { .. } => {
            File::write(file.clone(), (literal!("{\"op\":\"dummy-der\",\"display\":\"dummy derivative")).clone());
            File::write(file.clone(), (literal!("\",\"data\":[\"")).clone());
            writeCref(file.clone(), var_field!((*op).chosen, DAE::SymbolicOperation::NEW_DUMMY_DER).clone())?;
            File::write(file.clone(), (literal!("\"")).clone());
            serializeList(file.clone(), var_field!((*op).candidates, DAE::SymbolicOperation::NEW_DUMMY_DER).clone(), Arc::new(serializeCref), true, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]}")).clone());
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("serializeOperation failed: ")); __mm_s.push_str(&*anyString(op.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum AssignType {
    NORMAL = 1,
    TORN = 2,
    JACOBIAN = 3,
}
impl PartialOrd for AssignType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for AssignType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

fn tagFromAssignType(mut assignType: AssignType) -> Result<ArcStr> {
    let mut tag: ArcStr = arcstr::literal!("");
    tag = ((match assignType.clone() {
        AssignType::NORMAL => literal!("assign"),
        AssignType::TORN => literal!("torn"),
        AssignType::JACOBIAN => literal!("jacobian"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(tag)
}

fn serializeEquation(mut file: File::File, mut eq: Arc<SimCode::SimEqSystem>, mut section: ArcStr, mut withOperations: bool, mut parent: i32, mut first: bool, mut assign_type: AssignType) -> Result<()> {
    if !(first.clone()) {
        File::write(file.clone(), (literal!(",")).clone());
    }
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_RESIDUAL).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"residual\",\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_RESIDUAL).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_FOR_RESIDUAL).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"residual\",\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_FOR_RESIDUAL).clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_FOR_RESIDUAL).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_FOR_RESIDUAL).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_GENERIC_RESIDUAL).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"residual\",\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_GENERIC_RESIDUAL).clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_GENERIC_RESIDUAL).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_GENERIC_RESIDUAL).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type.clone())?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            writeCref(file.clone(), var_field!((*eq).cref, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type.clone())?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_GENERIC_ASSIGN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type.clone())?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_GENERIC_ASSIGN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ENTWINED_ASSIGN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type.clone())?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_ENTWINED_ASSIGN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type.clone())?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            writeCref(file.clone(), var_field!((*eq).cref, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type.clone())?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            writeCref(file.clone(), Expression::expCref(var_field!((*eq).lhs, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone())?, escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: None, lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, .. } => {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            i = (lSystem.beqs.clone().len() as i32);
            j = (lSystem.simJac.clone().len() as i32);
            eqs = SimCodeUtil::sortEqSystems(lSystem.residual.clone())?;
            if !(eqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                for mut e in &*listRest(eqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            if eqs.clone().is_empty() && jeqs.clone().is_empty() {
                File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            } else {
                File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            }
            File::writeInt(file.clone(), lSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            crefs = metamodelica::nil();
            serializeList(file.clone(), crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[{\"size\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            if i.clone() != 0 {
                File::write(file.clone(), (literal!(",\"density\":")).clone());
                File::writeReal(file.clone(), metamodelica::OrderedFloat((j.clone() / (i.clone() * i.clone())) as f64), (literal!("%.2f")).clone());
            }
            File::write(file.clone(), (literal!(",\"A\":[")).clone());
            serializeList(file.clone(), lSystem.simJac.clone(), Arc::new({ let __pe_b2 = withOperations.clone(); move |__pe_a0, __pe_a1| serializeLinearCell(__pe_a0, __pe_a1, __pe_b2.clone()) }), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"b\":[")).clone());
            serializeList(file.clone(), lSystem.beqs.clone(), Arc::new(serializeExp), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]}]}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(atL @ Deref @ SimCode::LinearSystem { .. }), lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, .. } => {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            i = (lSystem.beqs.clone().len() as i32);
            j = (lSystem.simJac.clone().len() as i32);
            eqs = SimCodeUtil::sortEqSystems(lSystem.residual.clone())?;
            if !(eqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                for mut e in &*listRest(eqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), lSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            if eqs.clone().is_empty() && jeqs.clone().is_empty() {
                File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            } else {
                File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            }
            File::writeInt(file.clone(), lSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            crefs = metamodelica::nil();
            serializeList(file.clone(), crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[{\"size\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            if i.clone() != 0 {
                File::write(file.clone(), (literal!(",\"density\":")).clone());
                File::writeReal(file.clone(), metamodelica::OrderedFloat((j.clone() / (i.clone() * i.clone())) as f64), (literal!("%.2f")).clone());
            }
            File::write(file.clone(), (literal!(",\"A\":[")).clone());
            serializeList(file.clone(), lSystem.simJac.clone(), Arc::new({ let __pe_b2 = withOperations.clone(); move |__pe_a0, __pe_a1| serializeLinearCell(__pe_a0, __pe_a1, __pe_b2.clone()) }), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"b\":[")).clone());
            serializeList(file.clone(), lSystem.beqs.clone(), Arc::new(serializeExp), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]}]},")).clone());
            i = (atL.beqs.clone().len() as i32);
            j = (atL.simJac.clone().len() as i32);
            eqs = SimCodeUtil::sortEqSystems(atL.residual.clone())?;
            if !(eqs.clone().is_empty()) {
                serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), atL.index.clone(), true, if (atL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                for mut e in &*listRest(eqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atL.index.clone(), false, if (atL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(atL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), atL.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atL.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            if eqs.clone().is_empty() && jeqs.clone().is_empty() {
                File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            } else {
                File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            }
            File::writeInt(file.clone(), atL.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if atL.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (atL.vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            crefs = metamodelica::nil();
            serializeList(file.clone(), crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[{\"size\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            if i.clone() != 0 {
                File::write(file.clone(), (literal!(",\"density\":")).clone());
                File::writeReal(file.clone(), metamodelica::OrderedFloat((j.clone() / (i.clone() * i.clone())) as f64), (literal!("%.2f")).clone());
            }
            File::write(file.clone(), (literal!(",\"A\":[")).clone());
            serializeList(file.clone(), atL.simJac.clone(), Arc::new({ let __pe_b2 = withOperations.clone(); move |__pe_a0, __pe_a1| serializeLinearCell(__pe_a0, __pe_a1, __pe_b2.clone()) }), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"b\":[")).clone());
            serializeList(file.clone(), atL.beqs.clone(), Arc::new(serializeExp), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]}]}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::List::Cons { head: stmt @ Deref @ DAE::Statement::STMT_ASSIGN { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ALGORITHM).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*section.clone()); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            writeCref(file.clone(), Expression::expCref(stmt.exp1.clone())?, escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(stmt.exp.clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeList(file.clone(), var_field!((*eq).statements, SimCode::SimEqSystem::SES_ALGORITHM).clone(), Arc::new(serializeStatement), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), Algorithm::getStatementSource(stmt.clone())?, withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::List::Cons { head: stmt, tail: _ }, .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ALGORITHM).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*section.clone()); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"equation\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), var_field!((*eq).statements, SimCode::SimEqSystem::SES_ALGORITHM).clone(), Arc::new(serializeStatement), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), Algorithm::getStatementSource(stmt.clone())?, withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { statements: Deref @ metamodelica::List::Cons { head: stmt, tail: _ }, .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*section.clone()); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"equation\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), var_field!((*eq).statements, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM).clone(), Arc::new(serializeStatement), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), Algorithm::getStatementSource(stmt.clone())?, withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: None, nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, .. } => {
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            eqs = SimCodeUtil::sortEqSystems(nlSystem.eqs.clone())?;
            serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, if (nlSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            for mut e in &*listRest(eqs.clone())? {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, if (nlSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            }
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), nlSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), nlSystem.crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            crefs = metamodelica::nil();
            serializeList(file.clone(), crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[[")).clone());
            serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],[")).clone());
            serializeList(file.clone(), jeqs.clone(), Arc::new(serializeEquationIndex), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]]}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(atNL @ Deref @ SimCode::NonlinearSystem { .. }), nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, .. } => {
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut jeqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut constantEqns: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            eqs = SimCodeUtil::sortEqSystems(nlSystem.eqs.clone())?;
            serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, if (nlSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            for mut e in &*listRest(eqs.clone())? {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, if (nlSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            }
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), nlSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), nlSystem.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), nlSystem.crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            crefs = metamodelica::nil();
            serializeList(file.clone(), crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[[")).clone());
            serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],[")).clone());
            serializeList(file.clone(), jeqs.clone(), Arc::new(serializeEquationIndex), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]]},")).clone());
            eqs = SimCodeUtil::sortEqSystems(atNL.eqs.clone())?;
            serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), true, if (atNL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            for mut e in &*listRest(eqs.clone())? {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), false, if (atNL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            }
            jeqs = (::match_deref::match_deref! { match &(atNL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { constantEqns, columnEqns: jeqs, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => SimCodeUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(jeqs.clone().is_empty()) {
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquation(file.clone(), listHead(jeqs.clone())?, (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*listRest(jeqs.clone())? {
                    let mut e = e.clone();
                    serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), atNL.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), atNL.index.clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            if atNL.tornSystem.clone() {
                File::write(file.clone(), (literal!("\",\"tag\":\"tornsystem\"")).clone());
            } else {
                File::write(file.clone(), (literal!("\",\"tag\":\"system\"")).clone());
            }
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atNL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) }).clone());
            serializeList(file.clone(), atNL.crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            crefs = metamodelica::nil();
            serializeList(file.clone(), crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[[")).clone());
            serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],[")).clone());
            serializeList(file.clone(), jeqs.clone(), Arc::new(serializeEquationIndex), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("]]}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { .. } => {
            let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            eqs = listAppend(List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).ifbranches, SimCode::SimEqSystem::SES_IFEQUATION).clone()).into_iter().cloned() {
            let __x = Util::tuple22(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), var_field!((*eq).elsebranch, SimCode::SimEqSystem::SES_IFEQUATION).clone());
            serializeEquation(file.clone(), listHead(eqs.clone())?, (section.clone()).clone(), withOperations.clone(), 0, true, AssignType::NORMAL.clone())?;
            for mut e in &*listRest(eqs.clone())? {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), 0, false, AssignType::NORMAL.clone())?;
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_IFEQUATION).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"if-equation\",\"display\":\"if-equation\",\"equation\":[")).clone());
            serializeList(file.clone(), var_field!((*eq).ifbranches, SimCode::SimEqSystem::SES_IFEQUATION).clone(), Arc::new(serializeIfBranch), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!(",")).clone());
            serializeIfBranch(file.clone(), (Arc::new(DAE::Exp::BCONST { bool: true }), var_field!((*eq).elsebranch, SimCode::SimEqSystem::SES_IFEQUATION).clone()))?;
            File::write(file.clone(), (literal!("]}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { .. } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            serializeEquation(file.clone(), var_field!((*eq).cont, SimCode::SimEqSystem::SES_MIXED).clone(), (section.clone()).clone(), withOperations.clone(), 0, true, AssignType::NORMAL.clone())?;
            for mut e in &*var_field!((*eq).discEqs, SimCode::SimEqSystem::SES_MIXED).clone() {
                let mut e = e.clone();
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), 0, false, AssignType::NORMAL.clone())?;
            }
            File::write(file.clone(), (literal!(",\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_MIXED).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\",\"tag\":\"container\",\"display\":\"mixed\",\"defines\":[")).clone());
            serializeList(file.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (var_field!((*eq).discVars, SimCode::SimEqSystem::SES_MIXED).clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            crefs = metamodelica::nil();
            serializeList(file.clone(), crefs.clone(), Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeEquationIndex(file.clone(), var_field!((*eq).cont, SimCode::SimEqSystem::SES_MIXED).clone())?;
            for mut e1 in &*var_field!((*eq).discEqs, SimCode::SimEqSystem::SES_MIXED).clone() {
                let mut e1 = e1.clone();
                File::write(file.clone(), (literal!(",")).clone());
                serializeEquationIndex(file.clone(), e1.clone())?;
            }
            File::write(file.clone(), (literal!("]}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { .. } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_WHEN).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            for mut whenOps in &*var_field!((*eq).whenStmtLst, SimCode::SimEqSystem::SES_WHEN).clone() {
                let mut whenOps = whenOps.clone();
                let () = (match whenOps.clone() {
        mut whenOp @ BackendDAE::WhenOperator::ASSIGN { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\",\"defines\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.left, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            serializeList(file.clone(), getWhenUses(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone())?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::REINIT { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\",\"defines\":[")).clone());
            serializeCref(file.clone(), var_field!(whenOp.stateVar, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), (literal!("],\"uses\":[")).clone());
            serializeList(file.clone(), getWhenUses(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone())?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::ASSERT { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\"")).clone());
            File::write(file.clone(), (literal!(",\"uses\":[")).clone());
            crefs = Expression::extractCrefsFromExpDerPreStart(var_field!(whenOp.condition, BackendDAE::WhenOperator::ASSERT).clone(), true)?;
            serializeList(file.clone(), getWhenUses(crefs.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone())?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::TERMINATE { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\"")).clone());
            File::write(file.clone(), (literal!(",\"uses\":[")).clone());
            serializeList(file.clone(), getWhenUses(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone())?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        mut whenOp @ BackendDAE::WhenOperator::NORETCALL { .. } => {
            File::write(file.clone(), (literal!("\",\"tag\":\"when\"")).clone());
            File::write(file.clone(), (literal!(",\"uses\":[")).clone());
            serializeList(file.clone(), getWhenUses(var_field!((*eq).conditions, SimCode::SimEqSystem::SES_WHEN).clone(), var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone())?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[")).clone());
            serializeExp(file.clone(), var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone())?;
            File::write(file.clone(), (literal!("],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_WHEN).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
            }
            let () = (::match_deref::match_deref! { match &(var_field!((*eq).elseWhen, SimCode::SimEqSystem::SES_WHEN).clone()) {
        Some(e) => {
            if SimCodeUtil::simEqSystemIndex(e.clone())? != 0 {
                serializeEquation(file.clone(), e.clone(), (section.clone()).clone(), withOperations.clone(), 0, false, AssignType::NORMAL.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), (literal!("%d")).clone());
            if parent.clone() != 0 {
                File::write(file.clone(), (literal!(",\"parent\":")).clone());
                File::writeInt(file.clone(), parent.clone(), (literal!("%d")).clone());
            }
            File::write(file.clone(), (literal!(",\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type.clone())?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) }).clone());
            writeCref(file.clone(), var_field!((*eq).cref, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), escape=JSON.clone())?;
            File::write(file.clone(), (literal!("\"],\"uses\":[")).clone());
            serializeList(file.clone(), Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((*eq).exp, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), true)?, Arc::new(serializeCref), false, (literal!(",")).clone())?;
            File::write(file.clone(), (literal!("],\"equation\":[\"")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eq).exp, SimCode::SimEqSystem::SES_FOR_LOOP).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\"],\"source\":")).clone());
            serializeSource(file.clone(), var_field!((*eq).source, SimCode::SimEqSystem::SES_FOR_LOOP).clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ALIAS { .. } => {
            File::write(file.clone(), (literal!("\n{\"eqIndex\":")).clone());
            File::writeInt(file.clone(), var_field!((*eq).index, SimCode::SimEqSystem::SES_ALIAS).clone(), (literal!("%d")).clone());
            File::write(file.clone(), (literal!(",\"tag\":\"alias\",\"equation\":[")).clone());
            File::writeInt(file.clone(), var_field!((*eq).aliasOf, SimCode::SimEqSystem::SES_ALIAS).clone(), (literal!("%d")).clone());
            File::write(file.clone(), (literal!("],\"section\":\"")).clone());
            File::write(file.clone(), (section.clone()).clone());
            File::write(file.clone(), (literal!("\"}")).clone());
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("serializeEquation failed: ")); __mm_s.push_str(&*anyString(eq.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeLinearCell(mut file: File::File, mut cell: (i32, i32, Arc<SimCode::SimEqSystem>), mut withOperations: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(cell.clone()) {
        (i, j, eq @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. }) => {
            File::write(file.clone(), (literal!("{\"row\":")).clone());
            File::write(file.clone(), (intString(i.clone())).clone());
            File::write(file.clone(), (literal!(",\"column\":")).clone());
            File::write(file.clone(), (intString(j.clone())).clone());
            File::write(file.clone(), (literal!(",\"exp\":\"")).clone());
            File::writeEscape(file.clone(), (expStr(eq.exp.clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!("\",\"source\":")).clone());
            serializeSource(file.clone(), eq.source.clone(), withOperations.clone())?;
            File::write(file.clone(), (literal!("}")).clone());
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("SerializeModelInfo.serializeLinearCell failed. Expected only SES_RESIDUAL as input.")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn varKindString(mut varKind: BackendDAE::VarKind, mut var: SimCodeVar::SimVar) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match varKind.clone() {
        BackendDAE::VarKind::VARIABLE => literal!("variable"),
        BackendDAE::VarKind::STATE { .. } => literal!("state"),
        BackendDAE::VarKind::STATE_DER => literal!("derivative"),
        BackendDAE::VarKind::DUMMY_DER => literal!("dummy derivative"),
        BackendDAE::VarKind::DUMMY_STATE => literal!("dummy state"),
        BackendDAE::VarKind::CLOCKED_STATE { .. } => literal!("clocked state"),
        BackendDAE::VarKind::DISCRETE => literal!("discrete"),
        BackendDAE::VarKind::PARAM => literal!("parameter"),
        BackendDAE::VarKind::CONST => literal!("constant"),
        BackendDAE::VarKind::EXTOBJ { .. } => literal!("external object"),
        BackendDAE::VarKind::JAC_VAR => literal!("jacobian variable"),
        BackendDAE::VarKind::JAC_TMP_VAR => literal!("jacobian differentiated variable"),
        BackendDAE::VarKind::OPT_CONSTR => literal!("constraint"),
        BackendDAE::VarKind::OPT_FCONSTR => literal!("final constraint"),
        BackendDAE::VarKind::OPT_INPUT_WITH_DER => literal!("use derivation of input"),
        BackendDAE::VarKind::OPT_INPUT_DER => literal!("derivation of input"),
        BackendDAE::VarKind::OPT_TGRID => literal!("time grid for optimization"),
        BackendDAE::VarKind::OPT_LOOP_INPUT { .. } => literal!("variable for transform loop in constraint"),
        BackendDAE::VarKind::ALG_STATE => literal!("helper variable transform ode for symSolver"),
        BackendDAE::VarKind::ALG_STATE_OLD => literal!("helper variable transform ode for symSolver"),
        BackendDAE::VarKind::LOOP_ITERATION => literal!("iteration variable for solving an algebraic loop"),
        BackendDAE::VarKind::DAE_RESIDUAL_VAR => literal!("residual variable for dae mode"),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SerializeModelInfo.varKindString")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*SimCodeUtil::simVarString(var.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(r#str)
}

fn getWhenUses(mut conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut value: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut uses: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    uses = listAppend(conditions.clone(), Expression::extractCrefsFromExpDerPreStart(value.clone(), true)?);
    uses = UnorderedSet::unique_list(uses.clone(), ComponentReference::hashComponentRef, ComponentReferenceBasics::crefEqual);
    Ok(uses)
}

fn serializeStatement(mut file: File::File, mut stmt: Arc<DAE::Statement>) -> Result<()> {
    File::write(file.clone(), (literal!("\"")).clone());
    File::writeEscape(file.clone(), (System::trim((DAEDump::ppStatementStr(stmt.clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeList<ArgType: Clone + 'static>(mut file: File::File, mut lst: Arc<metamodelica::List<ArgType>>, mut func: Arc<dyn ::std::ops::Fn(File::File, ArgType) -> Result<()> + 'static>, mut append: bool, mut sep: ArcStr) -> Result<()> {
    pub type FuncType<ArgType: Clone> = fn(File::File, ArgType) -> Result<()>;

    if !(lst.clone().is_empty()) {
        if append.clone() {
            File::write(file.clone(), (sep.clone()).clone());
        }
        func(file.clone(), listHead(lst.clone())?)?;
        for mut a in &*listRest(lst.clone())? {
            let mut a = a.clone();
            File::write(file.clone(), (sep.clone()).clone());
            func(file.clone(), a.clone())?;
        }
    }
    Ok(())
}

fn serializeExp(mut file: File::File, mut exp: Arc<DAE::Exp>) -> Result<()> {
    File::write(file.clone(), (literal!("\"")).clone());
    File::writeEscape(file.clone(), (expStr(exp.clone())?).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeCref(mut file: File::File, mut cr: Arc<DAE::ComponentRef>) -> Result<()> {
    File::write(file.clone(), (literal!("\"")).clone());
    writeCref(file.clone(), cr.clone(), escape=JSON.clone())?;
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeString(mut file: File::File, mut string: ArcStr) -> () {
    File::write(file.clone(), (literal!("\"")).clone());
    File::writeEscape(file.clone(), (string.clone()).clone(), JSON.clone());
    File::write(file.clone(), (literal!("\"")).clone());
    ()
}

fn serializePath(mut file: File::File, mut path: Arc<Absyn::Path>) -> Result<()> {
    let mut p: Arc<Absyn::Path> = path.clone();
    let mut b: bool = true;
    File::write(file.clone(), (literal!("\"")).clone());
    while b.clone() {
        (p, b) = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::IDENT).clone()).clone(), JSON.clone());
            (p.clone(), false)
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone(), JSON.clone());
            File::write(file.clone(), (literal!(".")).clone());
            (var_field!((*p).path, Absyn::Path::QUALIFIED).clone(), true)
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => (var_field!((*p).path, Absyn::Path::FULLYQUALIFIED).clone(), true),
        _ => bail!("match: no arm matched"),
    } });
    }
    File::write(file.clone(), (literal!("\"")).clone());
    Ok(())
}

fn serializeEquationIndex(mut file: File::File, mut eq: Arc<SimCode::SimEqSystem>) -> Result<()> {
    File::writeInt(file.clone(), SimCodeUtil::simEqSystemIndex(eq.clone())?, (literal!("%d")).clone());
    Ok(())
}

fn serializeIfBranch(mut file: File::File, mut branch: (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)) -> Result<()> {
    let mut exp: Arc<DAE::Exp>;
    let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    (exp, eqs) = branch.clone();
    File::write(file.clone(), (literal!("[")).clone());
    serializeExp(file.clone(), exp.clone())?;
    serializeList(file.clone(), eqs.clone(), Arc::new(serializeEquationIndex), true, (literal!(",")).clone())?;
    File::write(file.clone(), (literal!("]")).clone());
    Ok(())
}

fn writeEqExpStr(mut file: File::File, mut eqExp: Arc<DAE::EquationExp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(eqExp.clone()) {
        Deref @ DAE::EquationExp::PARTIAL_EQUATION { .. } => {
            File::writeEscape(file.clone(), (expStr(var_field!((*eqExp).exp, DAE::EquationExp::PARTIAL_EQUATION).clone())?).clone(), JSON.clone());
            ()
        },
        Deref @ DAE::EquationExp::RESIDUAL_EXP { .. } => {
            File::write(file.clone(), (literal!("0 = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eqExp).exp, DAE::EquationExp::RESIDUAL_EXP).clone())?).clone(), JSON.clone());
            ()
        },
        Deref @ DAE::EquationExp::EQUALITY_EXPS { .. } => {
            File::writeEscape(file.clone(), (expStr(var_field!((*eqExp).lhs, DAE::EquationExp::EQUALITY_EXPS).clone())?).clone(), JSON.clone());
            File::write(file.clone(), (literal!(" = ")).clone());
            File::writeEscape(file.clone(), (expStr(var_field!((*eqExp).rhs, DAE::EquationExp::EQUALITY_EXPS).clone())?).clone(), JSON.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn serializeFunction(mut file: File::File, mut func: Arc<SimCodeFunction::Function::Function>) -> Result<()> {
    File::write(file.clone(), (literal!("\n")).clone());
    serializePath(file.clone(), SimCodeUtil::functionPath(func.clone())?)?;
    Ok(())
}

