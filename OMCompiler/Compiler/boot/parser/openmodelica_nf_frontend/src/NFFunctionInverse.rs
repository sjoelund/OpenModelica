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

use crate::NFCall as Call;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct NFFunctionInverse {
    pub inputParam: Arc<ComponentRef::NFComponentRef>,
    pub inverseCall: Arc<Expression::NFExpression>,
    pub info: SourceInfo,
}

pub type FUNCTION_INV = NFFunctionInverse;

pub fn instInverses(mut fnNode: Arc<InstNode::InstNode>, mut r#fn: Arc<Function::Function>) -> Result<metamodelica::Array<Arc<NFFunctionInverse>>> {
    let mut inverses: metamodelica::Array<Arc<NFFunctionInverse>>;
    let mut inv_mods: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
    let mut invs: Arc<metamodelica::List<Arc<NFFunctionInverse>>> = metamodelica::nil();
    inv_mods = getInverseAnnotations(InstNode::definition(fnNode.clone())?)?;
    if !(inv_mods.clone().is_empty()) && !((r#fn.outputs.clone().len() as i32) == 1) {
        Error::addSourceMessage(Error::FUNCTION_INVALID_OUTPUTS_FOR_INVERSE.clone(), list![(AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone()], SCodeUtil::getModifierInfo(listHead(inv_mods.clone())?))?;
        bail!("fail");
    }
    for mut m in &*inv_mods.clone() {
        let mut m = m.clone();
        invs = instInverseMod(m.clone(), fnNode.clone(), r#fn.clone(), invs.clone())?;
    }
    inverses = metamodelica::arrayFromVec(invs.clone().into_iter().cloned().collect());
    Ok(inverses)
}

pub fn typeInverse(mut fnInv: Arc<NFFunctionInverse>) -> Result<Arc<NFFunctionInverse>> {
    let mut fnInv: Arc<NFFunctionInverse> = fnInv;
    assign_field!(
        fnInv.inputParam = Typing::typeCref(fnInv.inputParam.clone(), NFInstContext::RELAXED.clone(), fnInv.info.clone())?.0,
        fnInv.inverseCall = Typing::typeExp(fnInv.inverseCall.clone(), NFInstContext::RELAXED.clone(), fnInv.info.clone(), false)?.0
    );
    Ok(fnInv)
}

pub fn toDAE(mut fnInv: Arc<NFFunctionInverse>) -> Result<DAE::FunctionDefinition> {
    let mut invDef: DAE::FunctionDefinition;
    invDef = DAE::FunctionDefinition::FUNCTION_INVERSE { inputParam: ComponentRef::toDAE(fnInv.inputParam.clone())?, inverseCall: Expression::toDAE(fnInv.inverseCall.clone(), false)? };
    Ok(invDef)
}

pub fn toSubMod(mut fnInv: Arc<NFFunctionInverse>) -> Result<Arc<SCode::SubMod>> {
    let mut subMod: Arc<SCode::SubMod>;
    let mut inv_mod: Arc<SCode::SubMod>;
    let mut call_exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    call_exp = Expression::toAbsyn(fnInv.inverseCall.clone())?;
    inv_mod = Arc::new(SCode::SubMod { ident: (ComponentRef::firstName(fnInv.inputParam.clone(), false)?).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(call_exp.clone()), comment: None, info: fnInv.info.clone() }) });
    subMod = Arc::new(SCode::SubMod { ident: (literal!("inverse")).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![inv_mod.clone()], binding: None, comment: None, info: fnInv.info.clone() }) });
    Ok(subMod)
}

pub fn getFunction(mut fnInv: Arc<NFFunctionInverse>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function>;
    let mut call: Arc<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &(fnInv.inverseCall.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    call = __pa0.clone();
    r#fn = Call::typedFunction(call.clone())?;
    Ok(r#fn)
}

fn getInverseAnnotations(mut definition: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<SCode::Mod>>>> {
    let mut invMods: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
    invMods = (::match_deref::match_deref! { match &(definition.clone()) {
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            SCodeUtil::lookupAnnotations(ann.clone(), (literal!("inverse")).clone())?
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(invMods)
}

fn instInverseMod(mut r#mod: Arc<SCode::Mod>, mut fnNode: Arc<InstNode::InstNode>, mut r#fn: Arc<Function::Function>, mut fnInvs: Arc<metamodelica::List<Arc<NFFunctionInverse>>>) -> Result<Arc<metamodelica::List<Arc<NFFunctionInverse>>>> {
    let mut fnInvs: Arc<metamodelica::List<Arc<NFFunctionInverse>>> = fnInvs;
    fnInvs = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            for mut s in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut s = s.clone();
                fnInvs = instInverseSubMod(s.clone(), fnNode.clone(), r#fn.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone(), fnInvs.clone())?;
            }
            fnInvs.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunctionInverse.instInverseMod")); __mm_s.push_str(&*literal!(" got invalid modifier")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(fnInvs)
}

fn instInverseSubMod(mut submod: Arc<SCode::SubMod>, mut fnNode: Arc<InstNode::InstNode>, mut r#fn: Arc<Function::Function>, mut info: SourceInfo, mut fnInvs: Arc<metamodelica::List<Arc<NFFunctionInverse>>>) -> Result<Arc<metamodelica::List<Arc<NFFunctionInverse>>>> {
    let mut fnInvs: Arc<metamodelica::List<Arc<NFFunctionInverse>>> = fnInvs;
    let mut name: ArcStr = arcstr::literal!("");
    let mut aparam: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut param: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut call_aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut call_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    fnInvs = (::match_deref::match_deref! { match &(submod.clone()) {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(call_aexp @ Deref @ Absyn::Exp::CALL { .. }), subModLst: Deref @ metamodelica::List::Nil, .. }, ident: name } => {
            aparam = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() });
            match '__try0: {
                (param, _, _) = unwrap_break_err!(Lookup::lookupLocalCref(aparam.clone(), fnNode.clone(), NFInstContext::RELAXED.clone(), info.clone()), '__try0);
                let true = (InstNode::isInput(ComponentRef::node(param.clone())?)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<_, anyhow::Error>((param.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    param = __try0_o0;
                }
                Err(_) => {
                    Error::addSourceMessage(Error::INVALID_FUNCTION_ANNOTATION_INPUT.clone(), list![(name.clone()).clone(), (AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
                    bail!("fail");
                }
            }
            call_exp = Inst::instExp(call_aexp.clone(), fnNode.clone(), NFInstContext::RELAXED.clone(), info.clone())?;
            cons(Arc::new(NFFunctionInverse { inputParam: param.clone(), inverseCall: call_exp.clone(), info: info.clone() }), fnInvs.clone())
        },
        Deref @ SCode::SubMod { .. } => {
            Error::addStrictMessage(Error::INVALID_FUNCTION_ANNOTATION_ATTR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*submod.ident.clone()); __mm_s.push_str(&*SCodeDump::printModStr(submod.r#mod.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone(), (literal!("inverse")).clone()], info.clone())?;
            fnInvs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(fnInvs)
}


