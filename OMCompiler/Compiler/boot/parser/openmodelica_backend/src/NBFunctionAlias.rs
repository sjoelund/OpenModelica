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

use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBInline as Inline;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBPartitioning as Partitioning;
use crate::NBPartitioning::BClock;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Util imports
pub fn getModule() -> Result<Arc<dyn ::std::ops::Fn(Arc<VarData::VarData>, Arc<EqData::EqData>, Partition::Kind) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>> {
    let mut func: Module::functionAliasInterface;
    let mut flag: ArcStr = literal!("default");
    func = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "default" => functionAliasDefault.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub mod Call_Id {
    use super::*;
    /// key for UnorderedMap.
    ///    used to uniquely identify a function call
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Call_Id {
        pub call: Arc<Expression::NFExpression>,
        pub iter: Arc<Iterator::Iterator>,
    }

    impl Default for Call_Id {
        fn default() -> Self {
            Self {
                call: Default::default(),
                iter: Default::default(),
            }
        }
    }

    pub type CALL_ID = Call_Id;

    pub fn toString(mut id: Arc<Call_Id>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (if (!(BEquation::Iterator::isEmpty(id.iter.clone()))) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" [")); __mm_s.push_str(&*BEquation::Iterator::toString(id.iter.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(id.call.clone())?); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn hash(mut id: Arc<Call_Id>) -> Result<i32> {
        let mut hash: i32 = 0;
        hash = stringHashDjb2((toString(id.clone())?).clone());
        Ok(hash)
    }

    pub fn isEqual(mut id1: Arc<Call_Id>, mut id2: Arc<Call_Id>) -> Result<bool> {
        let mut b: bool = false;
        b = Expression::isEqual(id1.call.clone(), id2.call.clone())? && BEquation::Iterator::isEqual(id1.iter.clone(), id2.iter.clone())?;
        Ok(b)
    }

}

pub mod Call_Aux {
    use super::*;
    /// value for UnorderedMap.
    ///    represents the auxilliary variable that will be created and has
    ///    the equation kind for auxilliary equation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Call_Aux {
        pub replacer: Arc<Expression::NFExpression>,
        pub kind: EquationKind,
        pub parsed: bool,
    }

    impl Default for Call_Aux {
        fn default() -> Self {
            Self {
                replacer: Default::default(),
                kind: Default::default(),
                parsed: Default::default(),
            }
        }
    }

    pub type CALL_AUX = Call_Aux;

    pub fn toString(mut aux: Arc<Call_Aux>) -> ArcStr {
        let mut r#str: ArcStr = Expression::toString(aux.replacer.clone()).unwrap();
        r#str
    }

    pub fn getVars(mut aux: Arc<Call_Aux>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
        fn getVarsExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
            let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            vars = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => metamodelica::nil(),
        Deref @ Expression::CREF { .. } => list![BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?],
        Deref @ Expression::TUPLE { .. } => List::flatten({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut elem in (var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = getVarsExp(elem.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBFunctionAlias.Call_Aux.getVars.getVarsExp")); __mm_s.push_str(&*literal!(" failed because function alias auxilliary has a return type that currently cannot be parsed: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok(vars)
        }

        let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = getVarsExp(aux.replacer.clone()).unwrap();
        vars
    }

    pub fn createName(mut ty: Arc<Type::NFType>, mut iter: Arc<Iterator::Iterator>, mut aux_index: Pointer::Pointer<i32>, mut aux_name: ArcStr, mut init: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut new_ty: Arc<Type::NFType> = ty.clone();
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            new_ty = Type::liftArrayRightList(ty.clone(), BEquation::Iterator::dimensions(iter.clone()));
            (_, name) = BVariable::makeAuxVar((aux_name.clone()).clone(), Pointer::access(aux_index.clone()), new_ty.clone(), init.clone())?;
            subs = BEquation::Iterator::normalizedSubscripts(iter.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?;
            subs = Subscript::fillWithWholeLeft(subs.clone(), Type::dimensionCount(new_ty.clone()));
            name = ComponentRef::mergeSubscripts(subs.clone(), name.clone(), true, true, false)?;
        } else {
            (_, name) = BVariable::makeAuxVar((aux_name.clone()).clone(), Pointer::access(aux_index.clone()), new_ty.clone(), init.clone())?;
        }
        Pointer::update(aux_index.clone(), Pointer::access(aux_index.clone()) + 1);
        Ok(name)
    }

}

fn functionAliasTplString(mut tpl: (ArcStr, ArcStr), mut max_length: i32) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::tuple21(tpl.clone())); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), max_length.clone() - ((Util::tuple21(tpl.clone())).clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Util::tuple22(tpl.clone())); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn aliasListToString<T1: Clone + 'static, T2: Clone + 'static>(mut aux_lst: Arc<metamodelica::List<(T1, T2)>>, mut func1: Arc<dyn ::std::ops::Fn(T1) -> Result<ArcStr> + 'static>, mut func2: Arc<dyn ::std::ops::Fn(T2) -> Result<ArcStr> + 'static>, mut name: ArcStr) -> Result<ArcStr> {
    type idToString<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<ArcStr> + 'static>;

    type auxToString<T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T2) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr = arcstr::literal!("");
    let mut str_lst: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut max_length: i32 = 0;
    r#str = (StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" Alias")); ArcStr::from(__mm_s) }).clone())).clone();
    if aux_lst.clone().is_empty() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  <no alias>\n\n")); ArcStr::from(__mm_s) }).clone();
    } else {
        str_lst = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
        for mut tpl in (aux_lst.clone()).into_iter().cloned() {
            let __x = (func2(Util::tuple22(tpl.clone()))?, func1(Util::tuple21(tpl.clone()))?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        max_length = {
        let mut __acc: Option<i32> = None;
        for mut tpl in (str_lst.clone()).into_iter().cloned() {
            let __x = ((Util::tuple21(tpl.clone())).clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 3;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(str_lst.clone(), Arc::new({ let __pe_b1 = max_length.clone(); move |__pe_a0| Ok(functionAliasTplString(__pe_a0, __pe_b1.clone())) }), (literal!("")).clone(), (literal!("  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

fn resolveAux(mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut eq_index: Pointer::Pointer<i32>, mut init: bool, mut new_vars_disc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut new_vars_cont: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut new_vars_init: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut new_vars_recd: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut new_eqns_disc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut new_eqns_cont: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut new_eqns_init: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> {
    let mut new_vars_disc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_disc;
    let mut new_vars_cont: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_cont;
    let mut new_vars_init: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_init;
    let mut new_vars_recd: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_recd;
    let mut new_eqns_disc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = new_eqns_disc;
    let mut new_eqns_cont: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = new_eqns_cont;
    let mut new_eqns_init: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = new_eqns_init;
    let mut id: Arc<Call_Id::Call_Id> = Arc::new(<Call_Id::Call_Id as ::std::default::Default>::default());
    let mut aux: Arc<Call_Aux::Call_Aux> = Arc::new(<Call_Aux::Call_Aux as ::std::default::Default>::default());
    let mut disc: bool = false;
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut new_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut tpl in &*UnorderedMap::toList(map.clone()).reverse() {
        let mut tpl = tpl.clone();
        (id, aux) = tpl.clone();
        if !(aux.parsed.clone()) {
            new_vars = Call_Aux::getVars(aux.clone());
            disc = true;
            for mut new_var in &*new_vars.clone() {
                let mut new_var = new_var.clone();
                (disc, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd) = addAuxVar(new_var.clone(), disc.clone(), new_vars_disc.clone(), new_vars_cont.clone(), new_vars_init.clone(), new_vars_recd.clone(), init.clone())?;
            }
            new_eqn = BEquation::Equation::makeAssignment(aux.replacer.clone(), id.call.clone(), eq_index.clone(), (literal!("AUX")).clone(), id.iter.clone(), BEquation::default(aux.kind.clone(), init.clone(), None, None))?;
            if init.clone() {
                new_eqns_init = cons(new_eqn.clone(), new_eqns_init.clone());
            } else if disc.clone() {
                new_eqns_disc = cons(new_eqn.clone(), new_eqns_disc.clone());
            } else {
                new_eqns_cont = cons(new_eqn.clone(), new_eqns_cont.clone());
            }
            assign_field!(aux.parsed = true);
            UnorderedMap::add(id.clone(), aux.clone(), map.clone())?;
        }
    }
    Ok((new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd, new_eqns_disc, new_eqns_cont, new_eqns_init))
}

fn introduceFunctionAliasEquation(mut eqn: Arc<Equation::Equation>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut aux_index: Pointer::Pointer<i32>, mut eqn_index: Pointer::Pointer<i32>, mut init: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut depth: introduceFunctionAliasEquation::Depth = introduceFunctionAliasEquation::Depth::FULL;
    (eqn, _) = Inline::inlineArrayConstructorSingle(eqn.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), variables.clone(), set.clone(), eqn_index.clone(), Pointer::create(metamodelica::nil()))?;
    (iter, depth) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            (var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), if (BEquation::Equation::isWhenEquation(Pointer::create(body.clone())) || BEquation::Equation::isIfEquation(Pointer::create(body.clone()))) {Depth.CONDITION.clone()} else {Depth.FULL.clone()})
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            (Arc::new(crate::NBEquation::Iterator::EMPTY), Depth.CONDITION.clone())
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } => {
            (Arc::new(crate::NBEquation::Iterator::EMPTY), Depth.CONDITION.clone())
        },
        Deref @ BEquation::Equation::ALGORITHM { .. } => {
            (Arc::new(crate::NBEquation::Iterator::EMPTY), Depth.STOP.clone())
        },
        _ => {
            (Arc::new(crate::NBEquation::Iterator::EMPTY), Depth.FULL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if depth.clone() == Depth.FULL.clone() {
        eqn = BEquation::Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = aux_index.clone(); let __pe_b3 = iter.clone(); let __pe_b4 = init.clone(); move |__pe_a0| introduceFunctionAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }), None, (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    } else if depth.clone() == Depth.CONDITION.clone() {
        eqn = BEquation::Equation::mapCondition(eqn.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = aux_index.clone(); let __pe_b3 = iter.clone(); let __pe_b4 = init.clone(); move |__pe_a0| introduceFunctionAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }), None, (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>));
    }
    Ok(eqn)
}

fn introduceFunctionAlias(mut exp: Arc<Expression::NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut aux_index: Pointer::Pointer<i32>, mut iter: Arc<Iterator::Iterator>, mut init: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut deep_iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    deep_iter = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } => BEquation::Iterator::expand(iter.clone(), var_field!((*exp).call, Expression::NFExpression::CALL).clone())?,
        _ => iter.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp = Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = aux_index.clone(); let __pe_b3 = deep_iter.clone(); let __pe_b4 = init.clone(); move |__pe_a0| introduceFunctionAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } if (checkCallReplacement(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) => {
            introduceAlias(exp.clone(), map.clone(), aux_index.clone(), (arcstr::literal!(BVariable::FUNCTION_STR)).clone(), iter.clone(), init.clone())?
        },
        new_exp @ Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut new_exp = (*new_exp).clone();
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::map(arg.clone(), Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = aux_index.clone(); let __pe_b3 = iter.clone(); let __pe_b4 = init.clone(); move |__pe_a0| introduceArrayConstructorAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(new_exp => Expression::NFExpression::CALL; call = call.clone());
            new_exp.clone()
        },
        Deref @ Expression::MULTARY { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::MULTARY;
                arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = introduceArrayConstructorAlias(arg.clone(), map.clone(), aux_index.clone(), iter.clone(), init.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                inv_arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = introduceArrayConstructorAlias(arg.clone(), map.clone(), aux_index.clone(), iter.clone(), init.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            exp.clone()
        },
        Deref @ Expression::BINARY { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::BINARY;
                exp1 = introduceArrayConstructorAlias(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), map.clone(), aux_index.clone(), iter.clone(), init.clone())?,
                exp2 = introduceArrayConstructorAlias(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), map.clone(), aux_index.clone(), iter.clone(), init.clone())?
            );
            exp.clone()
        },
        Deref @ Expression::TUPLE_ELEMENT { tupleExp: sub_exp @ Deref @ Expression::TUPLE { .. }, .. } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            if var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone() > (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE).clone().len() as i32) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBFunctionAlias.introduceFunctionAlias")); __mm_s.push_str(&*literal!(" failed to get subscripted tuple element: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            } else {
                new_exp = (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE).clone()).get(var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone())?;
            }
            new_exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn introduceArrayConstructorAlias(mut exp: Arc<Expression::NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut aux_index: Pointer::Pointer<i32>, mut iter: Arc<Iterator::Iterator>, mut init: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => introduceAlias(exp.clone(), map.clone(), aux_index.clone(), (arcstr::literal!(BVariable::FUNCTION_STR)).clone(), iter.clone(), init.clone())?,
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { .. } } => introduceAlias(exp.clone(), map.clone(), aux_index.clone(), (arcstr::literal!(BVariable::FUNCTION_STR)).clone(), iter.clone(), init.clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn introduceAliasCrefConditional(mut exp: Arc<Expression::NFExpression>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut aux_index: Pointer::Pointer<i32>, mut iter: Arc<Iterator::Iterator>, mut init: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (UnorderedSet::contains(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()), set.clone())?) => introduceAlias(exp.clone(), map.clone(), aux_index.clone(), (arcstr::literal!(BVariable::STATE_ALIAS_STR)).clone(), iter.clone(), init.clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn introduceAlias(mut exp: Arc<Expression::NFExpression>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut aux_index: Pointer::Pointer<i32>, mut aux_name: ArcStr, mut iter: Arc<Iterator::Iterator>, mut init: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
    let mut new_iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut id: Arc<Call_Id::Call_Id> = Arc::new(<Call_Id::Call_Id as ::std::default::Default>::default());
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut aux: Arc<Call_Aux::Call_Aux> = Arc::new(<Call_Aux::Call_Aux as ::std::default::Default>::default());
    let mut aux_opt: Option<Arc<Call_Aux::Call_Aux>> = None;
    let mut tpl_lst: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    if !(BEquation::Iterator::isEmpty(iter.clone())) {
        (names, ranges, maps) = BEquation::Iterator::getFrames(iter.clone())?;
        new_iter = BEquation::Iterator::fromFrames(filterFrames(exp.clone(), names.clone(), ranges.clone(), maps.clone())?);
    } else {
        new_iter = iter.clone();
    }
    id = Arc::new(Call_Id::Call_Id { call: exp.clone(), iter: new_iter.clone() });
    aux_opt = UnorderedMap::get(id.clone(), map.clone());
    if isSome(aux_opt.clone()) {
        aux = Util::getOption(aux_opt.clone())?;
        exp = aux.replacer.clone();
    } else {
        (exp, aux_opt) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut call = (*call).clone();
            let _ = (::match_deref::match_deref! { match &((Call::functionName(call.clone())?, var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone())) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, _) => {
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = cons(listHead(var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone())?, {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (listRest(var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone())?).into_iter().cloned() {
            let __x = if (Expression::isLiteral(arg.clone()) || Expression::isCref(arg.clone())) {arg.clone()} else {introduceAlias(arg.clone(), map.clone(), aux_index.clone(), (aux_name.clone()).clone(), iter.clone(), init.clone())?};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            id = Arc::new(Call_Id::Call_Id { call: exp.clone(), iter: new_iter.clone() });
            aux_opt = UnorderedMap::get(id.clone(), map.clone());
            ()
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } }) => {
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = list![if (Expression::isLiteral(arg1.clone()) || Expression::isCref(arg1.clone())) {arg1.clone()} else {introduceAlias(arg1.clone(), map.clone(), aux_index.clone(), (aux_name.clone()).clone(), iter.clone(), init.clone())?}, arg2.clone()]);
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            id = Arc::new(Call_Id::Call_Id { call: exp.clone(), iter: new_iter.clone() });
            aux_opt = UnorderedMap::get(id.clone(), map.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (exp.clone(), aux_opt.clone())
        },
        _ => {
            (exp.clone(), aux_opt.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ty = Expression::typeOf(exp.clone());
        exp = (::match_deref::match_deref! { match &((aux_opt.clone(), ty.clone())) {
        (Some(aux), _) => aux.replacer.clone(),
        (_, Deref @ Type::TUPLE { .. }) => {
            names = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut sub_ty in (var_field!((*ty).types, Type::NFType::TUPLE).clone()).into_iter().cloned() {
            let __x = Call_Aux::createName(sub_ty.clone(), new_iter.clone(), aux_index.clone(), (aux_name.clone()).clone(), init.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            tpl_lst = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut cref in (names.clone()).into_iter().cloned() {
            let __x = if (ComponentRef::size(cref.clone(), true, false) == 0) {Expression::fromCref(Arc::new(openmodelica_nf_frontend::NFComponentRef::WILD), false)?} else {Expression::fromCref(cref.clone(), false)?};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(Expression::NFExpression::TUPLE { ty: ty.clone(), elements: tpl_lst.clone() })
        },
        _ => {
            name = Call_Aux::createName(ty.clone(), new_iter.clone(), aux_index.clone(), (aux_name.clone()).clone(), init.clone())?;
            Expression::fromCref(name.clone(), false)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if Util::isNone(aux_opt.clone()) {
            aux = Arc::new(Call_Aux::Call_Aux { replacer: exp.clone(), kind: if (Type::isDiscrete(ty.clone())) {EquationKind::DISCRETE.clone()} else {EquationKind::CONTINUOUS.clone()}, parsed: false });
            UnorderedMap::add(id.clone(), aux.clone(), map.clone())?;
        }
    }
    Ok(exp)
}

fn checkCallReplacement(mut call: Arc<Call::NFCall>) -> Result<bool> {
    let mut b: bool = false;
    let mut r#fn: Arc<Function::Function> = Call::typedFunction(call.clone())?;
    b = forceReplacement(r#fn.clone())? || !(Function::isSpecialBuiltin(r#fn.clone())? || replaceException(r#fn.clone())?);
    Ok(b)
}

fn forceReplacement(mut r#fn: Arc<Function::Function>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(Function::nameConsiderBuiltin(r#fn.clone())?)?) {
        Deref @ "cat" => true,
        Deref @ "terminal" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn replaceException(mut r#fn: Arc<Function::Function>) -> Result<bool> {
    let mut b: bool = false;
    let mut path: Arc<Absyn::Path>;
    if Function::isDefaultRecordConstructor(r#fn.clone()) || Function::isNonDefaultRecordConstructor(r#fn.clone()) || Function::isImpure(r#fn.clone()) || r#fn.outputs.clone().is_empty() {
        b = true;
        return Ok(b);
    }
    if !(Function::isBuiltin(r#fn.clone())) {
        b = false;
    } else {
        path = Function::nameConsiderBuiltin(r#fn.clone())?;
        if !(AbsynUtil::pathIsIdent(path.clone())) {
            b = false;
        } else {
            b = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(path.clone())?) {
        Deref @ "integer" => true,
        Deref @ "String" => true,
        Deref @ "$OMC$PositiveMax" => true,
        Deref @ "$OMC$inStreamDiv" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
    }
    Ok(b)
}

fn filterFrames(mut exp: Arc<Expression::NFExpression>, mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>> {
    fn collectFrames(mut exp: Arc<Expression::NFExpression>, mut frame_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut new_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut range: Option<Arc<Expression::NFExpression>> = None;
            range = UnorderedMap::get(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), frame_map.clone());
            if isSome(range.clone()) {
                UnorderedMap::add(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), Util::getOption(range.clone())?, new_map.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut frame_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::fromLists(names.clone(), ranges.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    let mut new_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut names_acc: Pointer::Pointer<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> = Pointer::create(metamodelica::nil());
    let mut ranges_acc: Pointer::Pointer<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> = Pointer::create(metamodelica::nil());
    let mut n: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut r: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut m: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
    let _ = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = frame_map.clone(); let __pe_b2 = new_map.clone(); move |__pe_a0| collectFrames(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    n = UnorderedMap::keyList(new_map.clone());
    r = UnorderedMap::valueList(new_map.clone());
    m = List::fill(None, (n.clone().len() as i32));
    frames = List::zip3(n.clone(), r.clone(), m.clone());
    Ok(frames)
}

fn addAuxVar(mut new_var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut disc: bool, mut new_vars_disc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut new_vars_cont: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut new_vars_init: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut new_vars_recd: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut init: bool) -> Result<(bool, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>)> {
    let mut disc: bool = disc;
    let mut new_vars_disc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_disc;
    let mut new_vars_cont: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_cont;
    let mut new_vars_init: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_init;
    let mut new_vars_recd: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = new_vars_recd;
    let mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    if BVariable::isRecord(new_var.clone()) {
        new_vars_recd = cons(new_var.clone(), new_vars_recd.clone());
        let __pa0 = ::match_deref::match_deref! { match &(Variable::expandChildren(Pointer::access(new_var.clone()), metamodelica::nil(), false)) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        children = __pa0.clone();
        for mut child in &*children.clone() {
            let mut child = child.clone();
            (disc, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd) = addAuxVar((BVariable::makeVarPtrCyclic(child.clone(), child.name.clone())?).0, disc.clone(), new_vars_disc.clone(), new_vars_cont.clone(), new_vars_init.clone(), new_vars_recd.clone(), init.clone())?;
        }
    } else if init.clone() {
        new_vars_init = cons(BVariable::setFixed(new_var.clone(), false, false)?, new_vars_init.clone());
    } else if BVariable::isContinuous(new_var.clone(), false)? {
        disc = false;
        new_vars_cont = cons(new_var.clone(), new_vars_cont.clone());
    } else {
        new_vars_disc = cons(new_var.clone(), new_vars_disc.clone());
    }
    Ok((disc, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd))
}

fn addClockedAlias(mut equations: Arc<EquationPointers::EquationPointers>, mut eqn_idx: Pointer::Pointer<i32>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>)> {
    let mut clock_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut infer_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut clock_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut infer_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut clck_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1);
    let mut infr_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1);
    let mut new_clocks: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut new_infers: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    let mut clock: Arc<BClock::BClock>;
    let mut clock_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    BEquation::EquationPointers::map(equations.clone(), Arc::new({ let __pe_b1 = clck_coll.clone(); let __pe_b2 = infr_coll.clone(); let __pe_b3 = new_clocks.clone(); let __pe_b4 = new_infers.clone(); let __pe_b5 = idx.clone(); move |__pe_a0| Partitioning::extractClocksEqn(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?;
    clock_vars = Pointer::access(new_clocks.clone());
    for mut tpl in &*UnorderedMap::toList(clck_coll.clone()) {
        let mut tpl = tpl.clone();
        (clock, clock_name) = tpl.clone();
        clock_eqns = cons(BEquation::Equation::makeAssignment(Expression::fromCref(clock_name.clone(), false)?, Partitioning::BClock::toExp(clock.clone())?, eqn_idx.clone(), (literal!("AUX")).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(EquationKind::CLOCKED.clone(), false, None, None))?, clock_eqns.clone());
    }
    infer_vars = Pointer::access(new_infers.clone());
    for mut tpl in &*UnorderedMap::toList(infr_coll.clone()) {
        let mut tpl = tpl.clone();
        (clock, clock_name) = tpl.clone();
        infer_eqns = cons(BEquation::Equation::makeAssignment(Expression::fromCref(clock_name.clone(), false)?, Partitioning::BClock::toExp(clock.clone())?, eqn_idx.clone(), (literal!("AUX")).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(EquationKind::CLOCKED.clone(), false, None, None))?, infer_eqns.clone());
    }
    Ok((clock_eqns, infer_eqns, clock_vars, infer_vars, clck_coll, infr_coll))
}

// type for slice collection
pub type Indices = Arc<UnorderedSet::UnorderedSet<i32>>;

fn collectSlicedStatesAliasEquation(mut eqn: Arc<Equation::Equation>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut iter: Arc<Iterator::Iterator> = BEquation::Equation::getForIterator(eqn.clone());
    BEquation::Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = iter.clone(); let __pe_b2 = map.clone(); move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), None, (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eqn)
}

fn collectSlicedStatesAlias(mut exp: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil }, r#fn: Deref @ Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, .. } } => {
            let mut iter_size: i32 = 0;
            let mut cref_size: i32 = 0;
            let mut var_size: i32 = 0;
            let mut call_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            let mut stripped_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut indices: Arc<UnorderedSet::UnorderedSet<i32>>;
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
            iter_size = BEquation::Iterator::size(iter.clone(), true);
            call_crefs = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            Slice::filterExp(arg.clone(), Arc::new({ let __pe_b2 = false; move |__pe_a0, __pe_a1| Slice::getContinuous(__pe_a0, __pe_a1, __pe_b2.clone()) }), call_crefs.clone())?;
            for mut cref in &*UnorderedSet::toList(call_crefs.clone()) {
                let mut cref = cref.clone();
                cref_size = Type::sizeOf(ComponentRef::getSubscriptedType(cref.clone(), false)?, true)?;
                var_size = BVariable::size(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?, true);
                if var_size.clone() != cref_size.clone() * iter_size.clone() {
                    stripped_cref = ComponentRef::stripSubscriptsAll(cref.clone());
                    indices = UnorderedMap::getOrDefault(stripped_cref.clone(), map.clone(), UnorderedSet::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 13));
                    (names, ranges, maps) = BEquation::Iterator::getFrames(iter.clone())?;
                    for mut index in &*Slice::getCrefInFrameIndicesLocal(cref.clone(), stripped_cref.clone(), List::zip3(names.clone(), ranges.clone(), maps.clone()), 0, true)? {
                        let mut index = index.clone();
                        UnorderedSet::add(index.clone(), indices.clone())?;
                    }
                    UnorderedMap::add(stripped_cref.clone(), indices.clone(), map.clone())?;
                }
            }
            exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = BEquation::Iterator::expand(iter.clone(), var_field!((*exp).call, Expression::NFExpression::CALL).clone())?; let __pe_b2 = map.clone(); move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { .. } } => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = BEquation::Iterator::expand(iter.clone(), var_field!((*exp).call, Expression::NFExpression::CALL).clone())?; let __pe_b2 = map.clone(); move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            exp.clone()
        },
        _ => {
            Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = iter.clone(); let __pe_b2 = map.clone(); move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn getSlicedStatesSet(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut state: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut indices: Arc<UnorderedSet::UnorderedSet<i32>>;
    for mut tpl in &*UnorderedMap::toList(map.clone()) {
        let mut tpl = tpl.clone();
        (state, indices) = tpl.clone();
        if BVariable::size(BVariable::getVarPointer(state.clone(), metamodelica::sourceInfo!())?, true) != UnorderedSet::size(indices.clone()) {
            UnorderedSet::add(state.clone(), set.clone())?;
        }
    }
    Ok(set)
}

fn introduceSlicedStateAliasEquation(mut eqn: Arc<Equation::Equation>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut aux_index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut iter: Arc<Iterator::Iterator> = BEquation::Equation::getForIterator(eqn.clone());
    eqn = BEquation::Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = set.clone(); let __pe_b2 = map.clone(); let __pe_b3 = iter.clone(); let __pe_b4 = aux_index.clone(); move |__pe_a0| introduceSlicedStateAliasExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }), None, (std::sync::Arc::new(fnptr!(Expression::fakeMap, Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eqn)
}

fn introduceSlicedStateAliasExp(mut exp: Arc<Expression::NFExpression>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<Call_Id::Call_Id>, Arc<Call_Aux::Call_Aux>>>, mut iter: Arc<Iterator::Iterator>, mut aux_index: Pointer::Pointer<i32>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil }, r#fn: Deref @ Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = list![Expression::map(arg.clone(), Arc::new({ let __pe_b1 = set.clone(); let __pe_b2 = map.clone(); let __pe_b3 = aux_index.clone(); let __pe_b4 = iter.clone(); let __pe_b5 = false; move |__pe_a0| introduceAliasCrefConditional(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?]);
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            new_exp = Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = set.clone(); let __pe_b2 = map.clone(); let __pe_b3 = BEquation::Iterator::expand(iter.clone(), var_field!((*exp).call, Expression::NFExpression::CALL).clone())?; let __pe_b4 = aux_index.clone(); move |__pe_a0| introduceSlicedStateAliasExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            new_exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { .. } } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            new_exp = Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = set.clone(); let __pe_b2 = map.clone(); let __pe_b3 = BEquation::Iterator::expand(iter.clone(), var_field!((*exp).call, Expression::NFExpression::CALL).clone())?; let __pe_b4 = aux_index.clone(); move |__pe_a0| introduceSlicedStateAliasExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            new_exp.clone()
        },
        _ => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            new_exp = Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = set.clone(); let __pe_b2 = map.clone(); let __pe_b3 = iter.clone(); let __pe_b4 = aux_index.clone(); move |__pe_a0| introduceSlicedStateAliasExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            new_exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

