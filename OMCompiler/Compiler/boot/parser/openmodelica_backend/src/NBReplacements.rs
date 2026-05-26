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

use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBInline as Inline;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstContext as InstContext;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTyping as Typing;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBReplacements.mo
/// package:     NBReplacements
/// description:
///  Replacements consists of a mapping between variables and expressions, the first binary tree of this type.
///  To eliminate a variable from an equation system a replacement rule varname->expression is added to this
///  datatype.
///  To be able to update these replacement rules incrementally a backward lookup mechanism is also required.
///  For instance, having a rule a->b and adding a rule b->c requires to find the first rule a->b and update it to
///  a->c. This is what the second binary tree is used for.
pub struct NBReplacements;
pub fn single(mut exp: Arc<Expression::NFExpression>, mut old: Arc<Expression::NFExpression>, mut new: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn traverse(mut exp: Arc<Expression::NFExpression>, mut old: Arc<Expression::NFExpression>, mut new: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        exp = if (Expression::isEqual(exp.clone(), old.clone())?) {new.clone()} else {exp.clone()};
        Ok(exp)
    }

    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = old.clone(); let __pe_b2 = new.clone(); move |__pe_a0| traverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    Ok(exp)
}

pub fn simple(mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<()> {
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        addSimple(comp.clone(), replacements.clone())?;
    }
    Ok(())
}

pub fn addSimple(mut comp: Arc<StrongComponent::NBStrongComponent>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            let mut varName: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut solvedEq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut status: Solve::Status = Solve::Status::UNPROCESSED;
            let mut replace_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            varName = BVariable::getVarName(var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone());
            (solvedEq, status, _) = Solve::solveBody(Pointer::access(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), varName.clone(), UnorderedMap::new(AbsynUtil::pathHash, fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>), 1))?;
            if status.clone() == Solve::Status::EXPLICIT.clone() {
                let __pa0 = ::match_deref::match_deref! { match &(Equation::getRHS(solvedEq.clone())?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                replace_exp = __pa0.clone();
                replace_exp = Expression::map(replace_exp.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
                replace_exp = SimplifyExp::simplifyDump(replace_exp.clone(), true, (literal!("NBReplacements.addSimple")).clone(), (literal!("")).clone())?;
                addInputArgTpl((varName.clone(), replace_exp.clone()), replacements.clone(), true)?;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBReplacements.addSimple")); __mm_s.push_str(&*literal!(" failed because strong component cannot be solved explicitly: ")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            ()
        },
        Deref @ StrongComponent::SLICED_COMPONENT { .. } => {
            let mut varName: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut solvedEq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut status: Solve::Status = Solve::Status::UNPROCESSED;
            let mut replace_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            varName = BVariable::getVarName(Slice::getT(var_field!((*comp).var, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone()));
            (solvedEq, status, _) = Solve::solveBody(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone())), varName.clone(), UnorderedMap::new(AbsynUtil::pathHash, fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>), 1))?;
            if status.clone() == Solve::Status::EXPLICIT.clone() {
                let __pa0 = ::match_deref::match_deref! { match &(Equation::getRHS(solvedEq.clone())?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                replace_exp = __pa0.clone();
                replace_exp = Expression::map(replace_exp.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
                replace_exp = SimplifyExp::simplifyDump(replace_exp.clone(), true, (literal!("NBReplacements.addSimple")).clone(), (literal!("")).clone())?;
                addInputArgTpl((varName.clone(), replace_exp.clone()), replacements.clone(), true)?;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBReplacements.addSimple")); __mm_s.push_str(&*literal!(" failed because strong component cannot be solved explicitly: ")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBReplacements.addSimple")); __mm_s.push_str(&*literal!(" failed because strong component is not simple: ")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn applySimple(mut eqData: Arc<EqData::EqData>, mut varData: Arc<VarData::VarData>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<(Arc<EqData::EqData>, Arc<VarData::VarData>)> {
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut entries: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut aliasCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut replacement: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var: Arc<Variable::NFVariable>;
    if UnorderedMap::isEmpty(replacements.clone()) {
        return Ok((eqData, varData));
    }
    eqData = EqData::mapExp(eqData.clone(), { let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) })?;
    varData = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ BVariable::VarData::VAR_DATA_SIM { .. } => {
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                variables = BVariable::VariablePointers::map(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?,
                aliasVars = BVariable::VariablePointers::map(var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM).clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?
            );
            varData.clone()
        },
        Deref @ BVariable::VarData::VAR_DATA_JAC { .. } => {
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_JAC; variables = BVariable::VariablePointers::map(var_field!((*varData).variables, VarData::VarData::VAR_DATA_JAC).clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?);
            varData.clone()
        },
        Deref @ BVariable::VarData::VAR_DATA_HES { .. } => {
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_HES; variables = BVariable::VariablePointers::map(var_field!((*varData).variables, VarData::VarData::VAR_DATA_HES).clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?);
            varData.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    entries = UnorderedMap::toList(replacements.clone());
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        (aliasCref, replacement) = entry.clone();
        var_ptr = BVariable::getVarPointer(aliasCref.clone(), metamodelica::sourceInfo!())?;
        var = Pointer::access(var_ptr.clone());
        assign_field!(var.binding = Binding::update(var.binding.clone(), replacement.clone())?);
        Pointer::update(var_ptr.clone(), var.clone());
    }
    Ok((eqData, varData))
}

pub fn applySimpleExp(mut exp: Arc<Expression::NFExpression>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut stripped: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
            if UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), replacements.clone()) {
                res = UnorderedMap::getOrFail(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), replacements.clone());
            } else {
                stripped = ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
                if UnorderedMap::contains(stripped.clone(), replacements.clone()) {
                    subs = ComponentRef::subscriptsAllWithWholeFlat(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
                    res = UnorderedMap::getOrFail(stripped.clone(), replacements.clone());
                    res = Expression::applySubscripts(subs.clone(), res.clone(), true)?;
                } else {
                    res = exp.clone();
                }
            }
            res.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn applySimpleVar(mut var: Arc<Variable::NFVariable>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { binding: binding @ Deref @ Binding::TYPED_BINDING { .. }, .. } => {
            let mut binding = (*binding).clone();
            assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = Expression::map(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }))?);
            assign_field!(var.binding = binding.clone());
            var.clone()
        },
        _ => {
            var.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn replaceVarPtr(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = var_ptr;
    let mut cref: Option<Arc<ComponentRef::NFComponentRef>> = None;
    cref = UnorderedMap::get(BVariable::getVarName(var_ptr.clone()), replacements.clone());
    if Util::isSome(cref.clone()) {
        var_ptr = BVariable::getVarPointer(Util::getOption(cref.clone())?, metamodelica::sourceInfo!())?;
    }
    Ok(var_ptr)
}

pub fn simpleToString(mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    let mut entries: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut constStr: ArcStr = literal!("");
    let mut aliasStr: ArcStr = literal!("");
    let mut nonTrivialStr: ArcStr = literal!("");
    let mut key: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    entries = UnorderedMap::toList(replacements.clone());
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        (key, value) = entry.clone();
        if Expression::isConstNumber(value.clone()) {
            constStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*constStr.clone()); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*ComponentRef::toString(key.clone())?); __mm_s.push_str(&*literal!("\t ==> \t")); __mm_s.push_str(&*Expression::toString(value.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        } else if !(Expression::isTrivialCref(value.clone())) {
            nonTrivialStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*nonTrivialStr.clone()); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*ComponentRef::toString(key.clone())?); __mm_s.push_str(&*literal!("\t ==> \t")); __mm_s.push_str(&*Expression::toString(value.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            aliasStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*aliasStr.clone()); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*ComponentRef::toString(key.clone())?); __mm_s.push_str(&*literal!("\t ==> \t")); __mm_s.push_str(&*Expression::toString(value.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("[dumprepl] Constant Replacements:")).clone())); __mm_s.push_str(&*constStr.clone()); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("[dumprepl] Trivial Alias Replacements:")).clone())); __mm_s.push_str(&*aliasStr.clone()); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4((literal!("[dumprepl] Nontrivial Alias Replacements:")).clone())); __mm_s.push_str(&*nonTrivialStr.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn replaceFunctions(mut eqData: Arc<EqData::EqData>, mut variables: Arc<VariablePointers::VariablePointers>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Function::Function>>>) -> Result<Arc<EqData::EqData>> {
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut prev_replacements: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<Expression::NFExpression>>> = UnorderedMap::new(fnptr!(Expression::hash, Arc<Expression::NFExpression>), Expression::isEqual, 1);
    if UnorderedMap::isEmpty(replacements.clone()) {
        return Ok(eqData);
    }
    eqData = EqData::mapExp(eqData.clone(), { let __pe_b1 = replacements.clone(); let __pe_b2 = prev_replacements.clone(); let __pe_b3 = variables.clone(); move |__pe_a0| applyFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) })?;
    Ok(eqData)
}

pub fn applyFuncExp(mut exp: Arc<Expression::NFExpression>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Function::Function>>>, mut prev_replacements: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, Arc<Expression::NFExpression>>>, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { r#fn, .. } } if (UnorderedMap::contains(r#fn.path.clone(), replacements.clone())) => {
            let mut local_replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
            let mut input_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut local_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut binding_exp_opt: Option<Arc<Expression::NFExpression>> = None;
            let mut binding_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut body_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut res_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            res_exp = (::match_deref::match_deref! { match &(UnorderedMap::get(exp.clone(), prev_replacements.clone())) {
        Some(res_exp) => res_exp.clone(),
        _ => {
            r#fn = UnorderedMap::getOrFail(r#fn.path.clone(), replacements.clone());
            local_replacements = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
            input_crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut node in (r#fn.inputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::fromNode(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            for mut tpl in &*List::zip(input_crefs.clone(), var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
                let mut tpl = tpl.clone();
                addInputArgTpl(tpl.clone(), local_replacements.clone(), false)?;
            }
            for mut local_node in &*r#fn.locals.clone() {
                let mut local_node = local_node.clone();
                local_cref = ComponentRef::fromNode(local_node.clone(), InstNode::getType(local_node.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone());
                binding_exp_opt = InstNode::getBindingExpOpt(local_node.clone());
                if Util::isSome(binding_exp_opt.clone()) {
                    binding_exp = Expression::map(Util::getOption(binding_exp_opt.clone())?, Arc::new({ let __pe_b1 = local_replacements.clone(); move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
                } else {
                    binding_exp = Arc::new(Expression::NFExpression::CREF { ty: Arc::new(openmodelica_nf_frontend::NFType::UNKNOWN), cref: Arc::new(openmodelica_nf_frontend::NFComponentRef::WILD) });
                }
                addInputArgTpl((local_cref.clone(), binding_exp.clone()), local_replacements.clone(), false)?;
            }
            body_exp = Function::getSingleBodyExp(r#fn.clone())?;
            body_exp = Expression::map(body_exp.clone(), Arc::new({ let __pe_b1 = local_replacements.clone(); move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
            if !(List::all(input_crefs.clone(), Arc::new(fnptr!(ComponentRef::sizeKnown, Arc<ComponentRef::NFComponentRef>)))) {
                (body_exp, _, _, _) = Typing::typeExp(body_exp.clone(), InstContext::RHS.clone(), metamodelica::sourceInfo!(), true)?;
            }
            body_exp = SimplifyExp::combineBinaries(body_exp.clone())?;
            body_exp = SimplifyExp::simplifyDump(body_exp.clone(), true, (literal!("NBReplacements.applyFuncExp")).clone(), (literal!("")).clone())?;
            res_exp = Expression::map(body_exp.clone(), Arc::new({ let __pe_b1 = replacements.clone(); let __pe_b2 = prev_replacements.clone(); let __pe_b3 = variables.clone(); move |__pe_a0| applyFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            if !(r#fn.attributes.generateEvents.clone()) {
                res_exp = Expression::fakeMap(res_exp.clone(), Arc::new(wrapEvents));
            }
            UnorderedMap::add(exp.clone(), res_exp.clone(), prev_replacements.clone())?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*literal!("NBReplacements.applyFuncExp")); __mm_s.push_str(&*literal!("] Inlining: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*Expression::toString(body_exp.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            res_exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            res_exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn addInputArgTpl(mut tpl: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>), mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut lowered_lhs: bool) -> Result<()> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut children_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tmp: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut call: Arc<Call::NFCall>;
    let mut r#fn: Arc<Function::Function>;
    (cref, arg) = tpl.clone();
    UnorderedMap::add(cref.clone(), arg.clone(), replacements.clone())?;
    children = if (lowered_lhs.clone()) {BVariable::getRecordChildrenCref(cref.clone())?} else {ComponentRef::getRecordChildren(cref.clone())?};
    if !(children.clone().is_empty()) {
        children_args = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { .. } => {
            tmp = BVariable::getRecordChildrenCref(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?;
            {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut child in (tmp.clone()).into_iter().cloned() {
            let __x = Expression::fromCref(child.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        Deref @ Expression::RECORD { .. } => var_field!((*arg).elements, Expression::NFExpression::RECORD).clone(),
        Deref @ Expression::TUPLE { .. } => var_field!((*arg).elements, Expression::NFExpression::TUPLE).clone(),
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { r#fn, .. } } => {
            if Function::isDefaultRecordConstructor(r#fn.clone()) {
                children_args = var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone();
            } else if Function::isNonDefaultRecordConstructor(r#fn.clone()) {
                children_args = var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone();
            } else {
                children_args = Expression::getRecordElements(arg.clone())?;
            }
            children_args.clone()
        },
        _ => Expression::getRecordElements(arg.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if List::compareLength(children.clone(), children_args.clone())? == 0 {
            for mut child_tpl in &*List::zip(children.clone(), children_args.clone()) {
                let mut child_tpl = child_tpl.clone();
                addInputArgTpl(child_tpl.clone(), replacements.clone(), lowered_lhs.clone())?;
            }
        }
    }
    Ok(())
}

pub fn wrapEvents(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::IF;
                condition = (::match_deref::match_deref! { match &(var_field!((*exp).condition, Expression::NFExpression::IF).clone()) {
        Deref @ Expression::CALL { .. } if (Expression::isCallNamed(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), (literal!("noEvent")).clone())?) => var_field!((*exp).condition, Expression::NFExpression::IF).clone(),
        _ => Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::NO_EVENT().clone(), list![var_field!((*exp).condition, Expression::NFExpression::IF).clone()], Expression::variability(var_field!((*exp).condition, Expression::NFExpression::IF).clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::NO_EVENT().returnType.clone()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
                trueBranch = Expression::mapShallow(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), Arc::new(wrapEvents))?,
                falseBranch = Expression::mapShallow(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), Arc::new(wrapEvents))?
            );
            exp.clone()
        },
        Deref @ Expression::RELATION { .. } => Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::NO_EVENT().clone(), list![exp.clone()], Expression::variability(exp.clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::NO_EVENT().returnType.clone()) }),
        Deref @ Expression::LBINARY { .. } => Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::NO_EVENT().clone(), list![exp.clone()], Expression::variability(exp.clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::NO_EVENT().returnType.clone()) }),
        Deref @ Expression::LUNARY { .. } => Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::NO_EVENT().clone(), list![exp.clone()], Expression::variability(exp.clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::NO_EVENT().returnType.clone()) }),
        _ => Expression::mapShallow(exp.clone(), Arc::new(wrapEvents))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}


