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

use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Variability;
use crate::NFStructural as Structural;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::DisjointSets;
use openmodelica_util::Error;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub mod FlowAlias {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FlowAlias {
        pub name: Arc<ComponentRef::NFComponentRef>,
        pub negative: bool,
        pub variable: Option<Arc<Variable::NFVariable>>,
    }

    impl Default for FlowAlias {
        fn default() -> Self {
            Self {
                name: Default::default(),
                negative: Default::default(),
                variable: Default::default(),
            }
        }
    }

    pub type FLOW_ALIAS = FlowAlias;

    pub fn isFlow(mut alias: Arc<FlowAlias>) -> bool {
        let mut isFlow: bool = Util::applyOptionOrDefault(alias.variable.clone(), (std::sync::Arc::new(fnptr!(Variable::isFlow, Arc<Variable::NFVariable>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<bool> + 'static>), false);
        isFlow
    }

}

pub fn EntryHash(mut entry: Entry) -> i32 {
    let mut hash: i32 = 0;
    hash = ComponentRef::hash(entry.name.clone());
    hash
}

pub fn EntryEqual(mut entry1: Entry, mut entry2: Entry) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = ComponentRef::isEqual(entry1.name.clone(), entry2.name.clone())?;
    Ok(isEqual)
}

pub fn EntryString(mut entry: Entry) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (ComponentRef::toString(entry.name.clone())?).clone();
    if entry.negative.clone() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn eliminateAliases(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut sets: Sets;
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut aliases: Arc<metamodelica::List<(Arc<FlowAlias::FlowAlias>, Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>)>> = metamodelica::nil();
    (flatModel, sets) = fromModel(flatModel.clone())?;
    (flatModel, aliases) = createAliases(sets.clone(), flatModel.clone())?;
    replacements = buildReplacements(aliases.clone())?;
    flatModel = applyReplacements(replacements.clone(), flatModel.clone());
    Ok(flatModel)
}

pub fn fromModel(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<(Arc<FlatModel::NFFlatModel>, Sets)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut sets: Sets;
    let mut alias_eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut other_eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut flow_aliases: Arc<metamodelica::List<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut opt_alias: Option<Arc<FlowAlias::FlowAlias>> = None;
    let mut alias: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    sets = emptySets(0);
    (alias_eqs, flow_aliases, other_eqs) = sortEquations(flatModel.equations.clone())?;
    assign_field!(flatModel.equations = other_eqs.clone());
    sets = List::threadFold(flow_aliases.clone(), alias_eqs.clone(), (std::sync::Arc::new(addAliasEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>, Arc<Equation::NFEquation>, Sets) -> Result<Sets> + 'static>), sets.clone())?;
    sets = List::fold(flatModel.variables.clone(), (std::sync::Arc::new(addAliasBinding) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>, Sets) -> Result<Sets> + 'static>), sets.clone());
    for mut v in &*flatModel.variables.clone() {
        let mut v = v.clone();
        alias = Arc::new(FlowAlias::FlowAlias { name: v.name.clone(), negative: false, variable: None });
        opt_alias = getEntry(alias.clone(), sets.clone());
        if isSome(opt_alias.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(opt_alias.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            alias = __pa0.clone();
            assign_field!(alias.variable = Some(v.clone()));
            UnorderedMap::updateKey(alias.clone(), sets.elements.clone())?;
        } else {
            vars = cons(v.clone(), vars.clone());
        }
    }
    for mut alias in &*UnorderedMap::keyList(sets.elements.clone()) {
        let mut alias = alias.clone();
        if isNone(alias.variable.clone()) {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFStreamFlowAlias.fromModel")); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentRef::toString(alias.name.clone())?); __mm_s.push_str(&*literal!(" has no associated variable")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
    }
    assign_field!(flatModel.variables = metamodelica::Dangerous::listReverseInPlace(vars.clone()));
    Ok((flatModel, sets))
}

pub fn sortEquations(mut eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>>>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>)> {
    let mut aliasEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut flowAliases: Arc<metamodelica::List<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>>> = metamodelica::nil();
    let mut otherEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut src: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    for mut eq in &*eqs.clone() {
        let mut eq = eq.clone();
        aliases = getAliasVarsFromEq(eq.clone())?;
        if aliases.clone().is_empty() {
            otherEqs = cons(eq.clone(), otherEqs.clone());
        } else {
            src = Equation::source(eq.clone())?;
            src = ElementSource::addAdditionalComment(src.clone(), (literal!("alias equation")).clone())?;
            eq = Equation::setSource(src.clone(), eq.clone())?;
            aliasEqs = cons(eq.clone(), aliasEqs.clone());
            flowAliases = cons(aliases.clone(), flowAliases.clone());
        }
    }
    aliasEqs = metamodelica::Dangerous::listReverseInPlace(aliasEqs.clone());
    flowAliases = metamodelica::Dangerous::listReverseInPlace(flowAliases.clone());
    otherEqs = metamodelica::Dangerous::listReverseInPlace(otherEqs.clone());
    Ok((aliasEqs, flowAliases, otherEqs))
}

pub fn addAliasEquation(mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>, mut eq: Arc<Equation::NFEquation>, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut scalar_aliases1: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut scalar_aliases2: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut alias1: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let mut alias2: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(aliases.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    alias1 = __pa0.clone();
    alias2 = __pa1.clone();
    if Equation::isArrayEquality(eq.clone()) {
        scalar_aliases1 = scalarizeAlias(alias1.clone())?;
        scalar_aliases2 = scalarizeAlias(alias2.clone())?;
        sets = List::threadFold(scalar_aliases1.clone(), scalar_aliases2.clone(), (std::sync::Arc::new(addAliasPair) as std::sync::Arc<dyn ::std::ops::Fn(Arc<FlowAlias::FlowAlias>, Arc<FlowAlias::FlowAlias>, Sets) -> Result<Sets> + 'static>), sets.clone())?;
    } else {
        sets = addAliasPair(alias1.clone(), alias2.clone(), sets.clone())?;
    }
    Ok(sets)
}

pub fn addAliasBinding(mut var: Arc<Variable::NFVariable>, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut bind_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut alias1: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let mut alias2: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    if Binding::hasExp(var.binding.clone()) {
        bind_exp = Binding::getExp(var.binding.clone())?;
        aliases = getAliasVarsFromExpPair(Expression::fromTypedCref(var.name.clone(), var.ty.clone()), bind_exp.clone())?;
        if !(aliases.clone().is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(aliases.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            alias1 = __pa0.clone();
            alias2 = __pa1.clone();
            sets = addAliasPair(alias1.clone(), alias2.clone(), sets.clone())?;
        }
    }
    Ok(sets)
}

pub fn addAliasPair(mut alias1: Arc<FlowAlias::FlowAlias>, mut alias2: Arc<FlowAlias::FlowAlias>, mut sets: Sets) -> Result<Sets> {
    fn find_set(mut alias: Arc<FlowAlias::FlowAlias>, mut sets: Sets) -> Result<(i32, Sets, bool)> {
        let mut set: i32 = 0;
        let mut sets: Sets = sets;
        let mut flippedSign: bool = false;
        let mut negative_alias: Arc<FlowAlias::FlowAlias> = alias.clone();
        let mut entry: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
        (set, sets) = findSet(alias.clone(), sets.clone())?;
        let __pa0 = ::match_deref::match_deref! { match &(getEntry(alias.clone(), sets.clone())) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        entry = __pa0.clone();
        flippedSign = entry.negative.clone() != alias.negative.clone();
        Ok((set, sets, flippedSign))
    }

    let mut sets: Sets = sets;
    let mut set1: i32 = 0;
    let mut set2: i32 = 0;
    let mut root1: i32 = 0;
    let mut root2: i32 = 0;
    let mut flipped_sign1: bool = false;
    let mut flipped_sign2: bool = false;
    (set1, sets, flipped_sign1) = find_set(alias1.clone(), sets.clone())?;
    (set2, sets, flipped_sign2) = find_set(alias2.clone(), sets.clone())?;
    if flipped_sign1.clone() != flipped_sign2.clone() {
        root1 = findRoot(set1.clone(), sets.nodes.clone())?;
        root2 = findRoot(set2.clone(), sets.nodes.clone())?;
        if root1.clone() == root2.clone() {
            return Ok(sets);
        }
        sets = negateSet(if (alias1.negative.clone()) {root1.clone()} else {root2.clone()}, sets.clone())?;
    }
    sets = union(set1.clone(), set2.clone(), sets.clone())?;
    Ok(sets)
}

pub fn getAliasVarsFromEq(mut eq: Arc<Equation::NFEquation>) -> Result<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>> {
    let mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    aliases = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => getAliasVarsFromExpPair(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone())?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(aliases)
}

pub fn getAliasVarsFromExpPair(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>> {
    let mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    aliases = getAliasVarsFromExp(exp1.clone(), exp2.clone(), metamodelica::nil())?;
    aliases = getAliasVarsFromExp(exp2.clone(), exp1.clone(), aliases.clone())?;
    if (aliases.clone().len() as i32) != 2 || List::none(aliases.clone(), (std::sync::Arc::new(isStreamConnectorFlow) as std::sync::Arc<dyn ::std::ops::Fn(Arc<FlowAlias::FlowAlias>) -> Result<bool> + 'static>)) {
        aliases = metamodelica::nil();
    }
    Ok(aliases)
}

pub fn getAliasVarsFromExp(mut exp: Arc<Expression::NFExpression>, mut otherExp: Arc<Expression::NFExpression>, mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>) -> Result<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>> {
    let mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = aliases;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut aliases1: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut aliases2: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut alias1: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let mut alias2: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    aliases = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::nodeVariability(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())? > Variability::DISCRETE.clone()) => cons(Arc::new(FlowAlias::FlowAlias { name: var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), negative: false, variable: None }), aliases.clone()),
        Deref @ Expression::UNARY { exp: e @ Deref @ Expression::CREF { .. }, .. } if (ComponentRef::nodeVariability(var_field!((**e).cref, Expression::NFExpression::CREF).clone())? > Variability::DISCRETE.clone()) => cons(Arc::new(FlowAlias::FlowAlias { name: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), negative: true, variable: None }), aliases.clone()),
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, .. } if (Expression::isZero(otherExp.clone())) => {
            aliases1 = getAliasVarsFromExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), metamodelica::nil())?;
            aliases2 = getAliasVarsFromExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), metamodelica::nil())?;
            if (aliases1.clone().len() as i32) == 1 && (aliases2.clone().len() as i32) == 1 {
                let __pa0 = ::match_deref::match_deref! { match &(aliases1.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                alias1 = __pa0.clone();
                let __pa2 = ::match_deref::match_deref! { match &(aliases2.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                alias2 = __pa2.clone();
                assign_field!(alias2.negative = !(alias2.negative.clone()));
                aliases = cons(alias1.clone(), cons(alias2.clone(), aliases.clone()));
            }
            aliases.clone()
        },
        _ => aliases.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(aliases)
}

pub fn isStreamConnectorFlow(mut alias: Arc<FlowAlias::FlowAlias>) -> Result<bool> {
    let mut isStreamFlow: bool = false;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if !(ComponentRef::isQualified(alias.name.clone())) {
        isStreamFlow = false;
        return Ok(isStreamFlow);
    }
    node = ComponentRef::node(alias.name.clone())?;
    if !(InstNode::isComponent(node.clone())) || !(Component::isFlow(InstNode::component(node.clone())?)) {
        isStreamFlow = false;
        return Ok(isStreamFlow);
    }
    isStreamFlow = Type::isStreamConnector(ComponentRef::nodeType(ComponentRef::rest(alias.name.clone())?)?);
    Ok(isStreamFlow)
}

pub fn scalarizeAlias(mut alias: Arc<FlowAlias::FlowAlias>) -> Result<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>> {
    let mut scalarAliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    crefs = ComponentRef::scalarize(alias.name.clone(), false)?;
    crefs = metamodelica::Dangerous::listReverseInPlace(crefs.clone());
    for mut cr in &*crefs.clone() {
        let mut cr = cr.clone();
        scalarAliases = cons(Arc::new(FlowAlias::FlowAlias { name: cr.clone(), negative: alias.negative.clone(), variable: None }), scalarAliases.clone());
    }
    Ok(scalarAliases)
}

pub fn negateSet(mut set: i32, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut indices: metamodelica::Array<i32>;
    let mut elements: Arc<UnorderedMap::UnorderedMap<Arc<FlowAlias::FlowAlias>, i32>>;
    let mut root: i32 = 0;
    let mut alias: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    nodes = sets.nodes.clone();
    elements = sets.elements.clone();
    root = findRoot(set.clone(), nodes.clone())?;
    indices = UnorderedMap::valueArray(elements.clone());
    let __range0 = 1..=(indices.clone().borrow().len() as i32);
    for mut i in __range0 {
        if findRoot(i.clone(), nodes.clone())? == root.clone() {
            alias = UnorderedMap::keyAt(elements.clone(), i.clone());
            assign_field!(alias.negative = !(alias.negative.clone()));
            Vector::updateNoBounds(elements.keys.clone(), i.clone(), alias.clone());
        }
    }
    Ok(sets)
}

pub fn createAliases(mut sets: Sets, mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<metamodelica::List<(Arc<FlowAlias::FlowAlias>, Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>)>>)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut aliases: Arc<metamodelica::List<(Arc<FlowAlias::FlowAlias>, Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>)>> = metamodelica::nil();
    let mut extracted_sets: metamodelica::Array<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>>;
    let mut representative: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let mut repr_var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut rest_aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut accum_aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut repr_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut alias_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut alias_eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    (extracted_sets, _) = extractSets(sets.clone());
    let __range0 = extracted_sets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut set in __range0 {
        (representative, rest_aliases) = defineRepresentative(set.clone())?;
        let __pa1 = ::match_deref::match_deref! { match &(representative.variable.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        repr_var = __pa1.clone();
        alias_vars = cons(repr_var.clone(), alias_vars.clone());
        repr_binding = Variable::asBinding(repr_var.clone(), Binding::Source::GENERATED.clone());
        for mut alias in &*rest_aliases.clone() {
            let mut alias = alias.clone();
            (alias, alias_eqs) = defineAlias(alias.clone(), repr_binding.clone(), alias_eqs.clone())?;
            alias_vars = cons(Util::getOption(alias.variable.clone())?, alias_vars.clone());
        }
        aliases = cons((representative.clone(), rest_aliases.clone()), aliases.clone());
    }
    aliases = aliases.clone().reverse();
    assign_field!(
        flatModel.variables = listAppend(flatModel.variables.clone(), alias_vars.clone().reverse()),
        flatModel.equations = listAppend(flatModel.equations.clone(), alias_eqs.clone().reverse())
    );
    Ok((flatModel, aliases))
}

pub fn buildReplacements(mut aliases: Arc<metamodelica::List<(Arc<FlowAlias::FlowAlias>, Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>)>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>> {
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut representative: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut negative_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rest_aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    replacements = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    for mut set in &*aliases.clone() {
        let mut set = set.clone();
        (representative, rest_aliases) = set.clone();
        exp = Expression::fromCref(representative.name.clone(), false)?;
        exp = if (representative.negative.clone()) {Expression::negate(exp.clone())} else {exp.clone()};
        negative_exp = Expression::negate(exp.clone());
        for mut alias in &*rest_aliases.clone() {
            let mut alias = alias.clone();
            UnorderedMap::addUnique(alias.name.clone(), if (alias.negative.clone()) {negative_exp.clone()} else {exp.clone()}, replacements.clone())?;
        }
    }
    Ok(replacements)
}

pub fn applyReplacements(mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    flatModel = FlatModel::mapExp(flatModel.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b0 = replacements.clone(); move |__pe_a1| applyReplacementsInExp(__pe_b0.clone(), __pe_a1) }); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }));
    flatModel
}

pub fn applyReplacementsInExp(mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut opt_val: Option<Arc<Expression::NFExpression>> = None;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            opt_val = UnorderedMap::get(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), replacements.clone());
            if (isSome(opt_val.clone())) {Util::getOption(opt_val.clone())?} else {exp.clone()}
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn defineRepresentative(mut aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>) -> Result<(Arc<FlowAlias::FlowAlias>, Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>)> {
    let mut representative: Arc<FlowAlias::FlowAlias> = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let mut restAliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut start_values: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = metamodelica::nil();
    let mut nominal_values: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = metamodelica::nil();
    let mut min_values: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut max_values: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut accum_aliases: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>> = metamodelica::nil();
    let mut min_value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut max_value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut nominal_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut min_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut max_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    for mut alias in &*aliases.clone() {
        let mut alias = alias.clone();
        (alias, start_values, nominal_values, min_values, max_values) = evalAliasAttributes(alias.clone(), start_values.clone(), nominal_values.clone(), min_values.clone(), max_values.clone())?;
        accum_aliases = cons(alias.clone(), accum_aliases.clone());
    }
    (representative, restAliases) = List::findAndRemove(aliases.clone(), (std::sync::Arc::new(fnptr!(FlowAlias::isFlow, Arc<FlowAlias::FlowAlias>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<FlowAlias::FlowAlias>) -> Result<bool> + 'static>))?;
    start_binding = if (start_values.clone().is_empty()) {Binding::EMPTY_BINDING().clone()} else {listHead(start_values.clone())?};
    nominal_binding = if (nominal_values.clone().is_empty()) {Binding::EMPTY_BINDING().clone()} else {listHead(nominal_values.clone())?};
    min_binding = computeLimit(min_values.clone(), (std::sync::Arc::new(Ceval::evalBuiltinMax2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    max_binding = computeLimit(max_values.clone(), (std::sync::Arc::new(Ceval::evalBuiltinMin2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    representative = setRepresentativeAttributes(representative.clone(), start_binding.clone(), nominal_binding.clone(), min_binding.clone(), max_binding.clone())?;
    Ok((representative, restAliases))
}

pub fn computeLimit(mut values: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut reduceFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Binding::NFBinding>> {
    type ReduceFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut limit: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if values.clone().is_empty() {
        limit = Binding::EMPTY_BINDING().clone();
    } else {
        res = List::reduce(values.clone(), reduceFn.clone())?;
        limit = Binding::makeFlat(res.clone(), Variability::CONSTANT.clone(), Binding::Source::GENERATED.clone());
    }
    Ok(limit)
}

pub fn defineAlias(mut alias: Arc<FlowAlias::FlowAlias>, mut binding: Arc<Binding::NFBinding>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<FlowAlias::FlowAlias>, Arc<metamodelica::List<Arc<Equation::NFEquation>>>)> {
    let mut alias: Arc<FlowAlias::FlowAlias> = alias;
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut var_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut bind_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut bind_eq: Arc<Equation::NFEquation>;
    let __pa0 = ::match_deref::match_deref! { match &(alias.variable.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    if Binding::isBound(var.binding.clone()) {
        var_exp = Expression::fromTypedCref(var.name.clone(), var.ty.clone());
        bind_exp = Binding::getExp(var.binding.clone())?;
        bind_eq = Equation::makeEquality(var_exp.clone(), bind_exp.clone(), var.ty.clone(), DAE::emptyElementSource().clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone());
        equations = cons(bind_eq.clone(), equations.clone());
    }
    assign_field!(
        var.binding = binding.clone(),
        var.comment = Arc::new(SCode::Comment { annotation_: var.comment.annotation_.clone(), comment: Some((literal!("Alias variable")).clone()) })
    );
    assign_field!(alias.variable = Some(var.clone()));
    Ok((alias, equations))
}

pub fn evalAliasAttributes(mut alias: Arc<FlowAlias::FlowAlias>, mut startValues: Arc<metamodelica::List<Arc<Binding::NFBinding>>>, mut nominalValues: Arc<metamodelica::List<Arc<Binding::NFBinding>>>, mut minValues: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut maxValues: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(Arc<FlowAlias::FlowAlias>, Arc<metamodelica::List<Arc<Binding::NFBinding>>>, Arc<metamodelica::List<Arc<Binding::NFBinding>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>)> {
    let mut alias: Arc<FlowAlias::FlowAlias> = alias;
    let mut startValues: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = startValues;
    let mut nominalValues: Arc<metamodelica::List<Arc<Binding::NFBinding>>> = nominalValues;
    let mut minValues: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = minValues;
    let mut maxValues: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = maxValues;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    let mut accum_attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    let mut attr_name: ArcStr = arcstr::literal!("");
    let mut attr_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut attr_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let __pa0 = ::match_deref::match_deref! { match &(alias.variable.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    attrs = var.typeAttributes.clone();
    for mut attr in &*attrs.clone() {
        let mut attr = attr.clone();
        (attr_name, attr_binding) = attr.clone();
        if Binding::hasExp(attr_binding.clone()) {
            attr = (::match_deref::match_deref! { match &(attr_name.clone()) {
        Deref @ "start" => {
            (attr_binding, _) = evalAliasAttribute(attr_binding.clone())?;
            startValues = cons(attr_binding.clone(), startValues.clone());
            (attr_name.clone(), attr_binding.clone())
        },
        Deref @ "nominal" => {
            (attr_binding, _) = evalAliasAttribute(attr_binding.clone())?;
            nominalValues = cons(attr_binding.clone(), nominalValues.clone());
            (attr_name.clone(), attr_binding.clone())
        },
        Deref @ "min" => {
            (attr_binding, attr_exp) = evalAliasAttribute(attr_binding.clone())?;
            minValues = cons(attr_exp.clone(), minValues.clone());
            (attr_name.clone(), attr_binding.clone())
        },
        Deref @ "max" => {
            (attr_binding, attr_exp) = evalAliasAttribute(attr_binding.clone())?;
            maxValues = cons(attr_exp.clone(), maxValues.clone());
            (attr_name.clone(), attr_binding.clone())
        },
        _ => attr.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        accum_attrs = cons(attr.clone(), accum_attrs.clone());
    }
    attrs = attrs.clone().reverse();
    assign_field!(var.typeAttributes = attrs.clone());
    assign_field!(alias.variable = Some(var.clone()));
    Ok((alias, startValues, nominalValues, minValues, maxValues))
}

pub fn evalAliasAttribute(mut binding: Arc<Binding::NFBinding>) -> Result<(Arc<Binding::NFBinding>, Arc<Expression::NFExpression>)> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut bindingExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    bindingExp = Binding::getExp(binding.clone())?;
    Structural::markExp(bindingExp.clone())?;
    bindingExp = Ceval::evalExp(bindingExp.clone(), Ceval::noTarget().clone())?;
    binding = Binding::setExp(bindingExp.clone(), binding.clone())?;
    Ok((binding, bindingExp))
}

pub fn setRepresentativeAttributes(mut alias: Arc<FlowAlias::FlowAlias>, mut startValue: Arc<Binding::NFBinding>, mut nominalValue: Arc<Binding::NFBinding>, mut minValue: Arc<Binding::NFBinding>, mut maxValue: Arc<Binding::NFBinding>) -> Result<Arc<FlowAlias::FlowAlias>> {
    fn add_attribute(mut name: ArcStr, mut binding: Arc<Binding::NFBinding>, mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>) -> Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> {
        let mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = attrs;
        if Binding::isBound(binding.clone()) {
            attrs = cons((name.clone(), binding.clone()), attrs.clone());
        }
        attrs
    }

    let mut alias: Arc<FlowAlias::FlowAlias> = alias;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
    let mut attr_name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(alias.variable.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    attrs = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut attr in (var.typeAttributes.clone()).into_iter().cloned() {
            if !(!(listMember((Util::tuple21(attr.clone())).clone(), list![(literal!("start")).clone(), (literal!("nominal")).clone(), (literal!("min")).clone(), (literal!("max")).clone()]))) { continue; }
            let __x = attr.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    attrs = add_attribute((literal!("max")).clone(), maxValue.clone(), attrs.clone());
    attrs = add_attribute((literal!("min")).clone(), minValue.clone(), attrs.clone());
    attrs = add_attribute((literal!("nominal")).clone(), nominalValue.clone(), attrs.clone());
    attrs = add_attribute((literal!("start")).clone(), startValue.clone(), attrs.clone());
    assign_field!(var.typeAttributes = attrs.clone());
    assign_field!(alias.variable = Some(var.clone()));
    Ok(alias)
}

pub type Entry = Arc<FlowAlias::FlowAlias>;

pub type IndexTable = Arc<UnorderedMap::UnorderedMap<Arc<FlowAlias::FlowAlias>, i32>>;

/// This is a disjoint sets data structure. The nodes are stored in an array of
///   Integers. The root elements of a set is given a negative value that
///   corresponds to its rank, while other elements are given positive values that
///   corresponds to the index of their parent in the array. The hashtable is used
///   to look up the array index of a entry, and is also used to store the entries.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sets {
    /// An array of nodes
    pub nodes: metamodelica::Array<i32>,
    /// An Entry->Integer table.
    pub elements: IndexTable,
    /// The number of nodes stored in the sets.
    pub nodeCount: i32,
}

pub type DISJOINT_SETS = Sets;


pub fn add(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
    let mut sets: Sets = sets;
    let mut index: i32 = 0;
    let mut nodes: metamodelica::Array<i32>;
    let mut elements: IndexTable;
    let mut node_count: i32 = 0;
    let Sets { nodes: __pa0, elements: __pa1, nodeCount: __pa2 } = (sets.clone()) else { bail!("pattern mismatch") };
    nodes = __pa0.clone();
    elements = __pa1.clone();
    node_count = __pa2.clone();
    index = node_count.clone() + 1;
    if index.clone() > (nodes.clone().borrow().len() as i32) {
        nodes = Array::expand(((intReal(index.clone()) * metamodelica::OrderedFloat(1.4_f64)).0 as i32), nodes.clone(), -1)?;
    }
    UnorderedMap::addNew(entry.clone(), index.clone(), elements.clone())?;
    sets = Sets { nodes: nodes.clone(), elements: elements.clone(), nodeCount: index.clone() };
    Ok((sets, index))
}

pub fn addList(mut entries: Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut elements: IndexTable;
    let mut node_count: i32 = 0;
    let mut sz: i32 = 0;
    let mut index: i32 = 0;
    let Sets { nodes: __pa0, elements: __pa1, nodeCount: __pa2 } = (sets.clone()) else { bail!("pattern mismatch") };
    nodes = __pa0.clone();
    elements = __pa1.clone();
    node_count = __pa2.clone();
    sz = (entries.clone().len() as i32);
    index = node_count.clone() + 1;
    node_count = node_count.clone() + sz.clone();
    if node_count.clone() > (nodes.clone().borrow().len() as i32) {
        nodes = Array::expand(((intReal(node_count.clone()) * metamodelica::OrderedFloat(1.4_f64)).0 as i32), nodes.clone(), -1)?;
    }
    for mut e in &*entries.clone() {
        let mut e = e.clone();
        UnorderedMap::addNew(e.clone(), index.clone(), elements.clone())?;
        index = index.clone() + 1;
    }
    sets = Sets { nodes: nodes.clone(), elements: elements.clone(), nodeCount: node_count.clone() };
    Ok(sets)
}

pub fn contains(mut entry: Entry, mut sets: Sets) -> bool {
    let mut found: bool = false;
    found = isSome(UnorderedMap::get(entry.clone(), sets.elements.clone()));
    found
}

pub fn emptySets(mut setCount: i32) -> Sets {
    let mut sets: Sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut elements: IndexTable;
    let mut sz: i32 = 0;
    sz = std::cmp::max(setCount.clone(), 3);
    nodes = arrayCreate(sz.clone(), -1);
    elements = UnorderedMap::new((std::sync::Arc::new(fnptr!(EntryHash, Arc<FlowAlias::FlowAlias>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<FlowAlias::FlowAlias>) -> Result<i32> + 'static>), (std::sync::Arc::new(EntryEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<FlowAlias::FlowAlias>, Arc<FlowAlias::FlowAlias>) -> Result<bool> + 'static>), 1);
    sets = Sets { nodes: nodes.clone(), elements: elements.clone(), nodeCount: 0 };
    sets
}

pub fn extractSets(mut sets: Sets) -> (metamodelica::Array<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>>, Sets) {
    let mut setsArray: metamodelica::Array<Arc<metamodelica::List<Arc<FlowAlias::FlowAlias>>>>;
    let mut assignedSets: Sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut set_idx: i32 = 0;
    let mut idx: i32 = 0;
    let mut entries: metamodelica::Array<(Arc<FlowAlias::FlowAlias>, i32)>;
    let mut e: Entry = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    nodes = sets.nodes.clone();
    for mut i in 1..=sets.nodeCount.clone() {
        if nodes.borrow()[(i.clone()-1) as usize].clone() < 0 {
            set_idx = set_idx.clone() + 1;
            {
                let __cell0 = -(set_idx.clone());
                nodes.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
            }
        }
    }
    setsArray = arrayCreate(set_idx.clone(), metamodelica::nil());
    entries = UnorderedMap::toArray(sets.elements.clone());
    let __range1 = (1..=(entries.clone().borrow().len() as i32)).rev();
    for mut i in __range1 {
        (e, idx) = entries.clone().borrow()[(i.clone()-1) as usize].clone();
        set_idx = nodes.borrow()[(idx.clone()-1) as usize].clone();
        while set_idx.clone() > 0 {
            set_idx = nodes.borrow()[(set_idx.clone()-1) as usize].clone();
        }
        set_idx = -(set_idx.clone());
        {
            let __cell2 = cons(e.clone(), setsArray.borrow()[(set_idx.clone()-1) as usize].clone());
            setsArray.clone().borrow_mut()[(set_idx.clone()-1) as usize] = __cell2;
        }
    }
    assignedSets = Sets { nodes: nodes.clone(), elements: sets.elements.clone(), nodeCount: sets.nodeCount.clone() };
    (setsArray, assignedSets)
}

pub fn find(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
    let mut sets: Sets = sets;
    let mut index: i32 = 0;
    let mut oindex: Option<i32> = None;
    oindex = UnorderedMap::get(entry.clone(), sets.elements.clone());
    if isSome(oindex.clone()) {
        let Some(__pa0) = (oindex.clone()) else { bail!("pattern mismatch") };
        index = __pa0.clone();
    } else {
        (sets, index) = add(entry.clone(), sets.clone())?;
    }
    Ok((sets, index))
}

pub fn findRoot(mut nodeIndex: i32, mut nodes: metamodelica::Array<i32>) -> Result<i32> {
    let mut rootIndex: i32 = nodeIndex.clone();
    let mut parent: i32 = nodes.borrow()[(nodeIndex.clone()-1) as usize].clone();
    let mut idx: i32 = nodeIndex.clone();
    while parent.clone() > 0 {
        rootIndex = parent.clone();
        parent = nodes.borrow()[(parent.clone()-1) as usize].clone();
    }
    parent = nodes.borrow()[(nodeIndex.clone()-1) as usize].clone();
    while parent.clone() > 0 {
        {let _arr = nodes.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = rootIndex.clone(); _arr};
        idx = parent.clone();
        parent = nodes.borrow()[(parent.clone()-1) as usize].clone();
    }
    Ok(rootIndex)
}

pub fn findSet(mut entry: Entry, mut sets: Sets) -> Result<(i32, Sets)> {
    let mut set: i32 = 0;
    let mut updatedSets: Sets;
    let mut index: i32 = 0;
    (updatedSets, index) = find(entry.clone(), sets.clone())?;
    set = findRoot(index.clone(), updatedSets.nodes.clone())?;
    Ok((set, updatedSets))
}

pub fn findSetArrayIndex(mut entry: Entry, mut sets: Sets) -> i32 {
    let mut set: i32 = 0;
    set = UnorderedMap::getOrFail(entry.clone(), sets.elements.clone());
    while set.clone() > 0 {
        set = sets.nodes.borrow()[(set.clone()-1) as usize].clone();
    }
    set = -(set.clone());
    set
}

pub fn getEntry(mut entry: Entry, mut sets: Sets) -> Option<Arc<FlowAlias::FlowAlias>> {
    let mut outEntry: Option<Arc<FlowAlias::FlowAlias>> = None;
    outEntry = UnorderedMap::getKey(entry.clone(), sets.elements.clone());
    outEntry
}

pub fn getNodeCount(mut sets: Sets) -> i32 {
    let mut nodeCount: i32 = sets.nodeCount.clone();
    nodeCount
}

pub fn merge(mut entry1: Entry, mut entry2: Entry, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut set1: i32 = 0;
    let mut set2: i32 = 0;
    (set1, sets) = findSet(entry1.clone(), sets.clone())?;
    (set2, sets) = findSet(entry2.clone(), sets.clone())?;
    sets = union(set1.clone(), set2.clone(), sets.clone())?;
    Ok(sets)
}

pub fn printSets(mut sets: Sets) -> Result<()> {
    let mut nodes: metamodelica::Array<i32>;
    let mut entries: Arc<metamodelica::List<(Arc<FlowAlias::FlowAlias>, i32)>> = metamodelica::nil();
    let mut e: Entry = Arc::new(<FlowAlias::FlowAlias as ::std::default::Default>::default());
    let mut i: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(sets.nodeCount.clone())); __mm_s.push_str(&*literal!(" sets:\n")); ArcStr::from(__mm_s) }).clone());
    nodes = sets.nodes.clone();
    entries = UnorderedMap::toList(sets.elements.clone());
    for mut p in &*entries.clone() {
        let mut p = p.clone();
        (e, i) = p.clone();
        println!("{}", (literal!("[")).clone());
        println!("{}", ArcStr::from(::std::format!("{}", i.clone())));
        println!("{}", (literal!("]")).clone());
        println!("{}", (EntryString(e.clone())?).clone());
        println!("{}", (literal!(" -> ")).clone());
        println!("{}", ArcStr::from(::std::format!("{}", nodes.borrow()[(i.clone()-1) as usize].clone())));
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn union(mut set1: i32, mut set2: i32, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut rank1: i32 = 0;
    let mut rank2: i32 = 0;
    if set1.clone() != set2.clone() {
        rank1 = sets.nodes.borrow()[(set1.clone()-1) as usize].clone();
        rank2 = sets.nodes.borrow()[(set2.clone()-1) as usize].clone();
        if rank1.clone() > rank2.clone() {
            {let _arr = sets.nodes.clone(); _arr.borrow_mut()[(set2.clone()-1) as usize] = set1.clone(); _arr};
        } else if rank1.clone() < rank2.clone() {
            {let _arr = sets.nodes.clone(); _arr.borrow_mut()[(set1.clone()-1) as usize] = set2.clone(); _arr};
        } else {
            {let _arr = sets.nodes.clone(); let _val = sets.nodes.borrow()[(set1.clone()-1) as usize].clone() - 1; _arr.borrow_mut()[(set1.clone()-1) as usize] = _val; _arr};
            {let _arr = sets.nodes.clone(); _arr.borrow_mut()[(set2.clone()-1) as usize] = set1.clone(); _arr};
        }
    }
    Ok(sets)
}

