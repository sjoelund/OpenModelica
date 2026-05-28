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
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn checkModel(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<(i32, i32)> {
    let mut variables: i32 = 0;
    let mut equations: i32 = 0;
    for mut v in &*flatModel.variables.clone() {
        let mut v = v.clone();
        (variables, equations) = countVariableSize(v.clone(), variables.clone(), equations.clone())?;
    }
    equations = equations.clone() + Equation::sizeOfList(flatModel.equations.clone())?;
    for mut a in &*flatModel.algorithms.clone() {
        let mut a = a.clone();
        equations = equations.clone() + countAlgorithmSize(a.clone())?;
    }
    Ok((variables, equations))
}

pub fn countVariableSize(mut var: Arc<Variable::NFVariable>, mut variables: i32, mut equations: i32) -> Result<(i32, i32)> {
    let mut variables: i32 = variables;
    let mut equations: i32 = equations;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    let mut var_size: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { attributes: __pa0, binding: __pa1, ty: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    attr = __pa0.clone();
    binding = __pa1.clone();
    ty = __pa2.clone();
    if attr.variability.clone() < Variability::DISCRETE.clone() {
        return Ok((variables.clone(), equations.clone()));
    }
    if Type::isExternalObject(ty.clone()) {
        return Ok((variables.clone(), equations.clone()));
    }
    var_size = Type::sizeOf(ty.clone(), false)?;
    variables = variables.clone() + var_size.clone();
    if Variable::isTopLevelInput(var.clone()) {
        equations = equations.clone() + var_size.clone();
    } else {
        equations = equations.clone() + Type::sizeOf(Binding::getType(binding.clone())?, false)?;
    }
    Ok((variables, equations))
}

pub fn countAlgorithmSize(mut alg: Arc<Algorithm::NFAlgorithm>) -> Result<i32> {
    let mut equations: i32 = 0;
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    crefs = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    crefs = List::fold(alg.statements.clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs.clone());
    equations = equations.clone() + UnorderedSet::size(crefs.clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Algorithm size: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", UnorderedSet::size(crefs.clone())))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    for mut cr in &*UnorderedSet::toList(crefs.clone()) {
        let mut cr = cr.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(equations)
}

fn statementOutputs(mut stmt: Arc<Statement::NFStatement>, mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = crefs;
    crefs = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => Expression::fold(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), (std::sync::Arc::new(statementOutputCrefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs.clone())?,
        Deref @ Statement::FOR { .. } => List::fold(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs.clone()),
        Deref @ Statement::IF { .. } => {
            for mut b in &*var_field!((*stmt).branches, Statement::NFStatement::IF).clone() {
                let mut b = b.clone();
                crefs = List::fold(Util::tuple22(b.clone()), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs.clone());
            }
            crefs.clone()
        },
        Deref @ Statement::WHEN { .. } => {
            for mut b in &*var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone() {
                let mut b = b.clone();
                crefs = List::fold(Util::tuple22(b.clone()), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs.clone());
            }
            crefs.clone()
        },
        Deref @ Statement::WHILE { .. } => List::fold(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone(), (std::sync::Arc::new(statementOutputs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs.clone()),
        _ => crefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

fn statementOutputCrefFinder(mut exp: Arc<Expression::NFExpression>, mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = crefs;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    crefs = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            (cref, _) = ComponentRef::stripSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
            Expression::fold((ExpandExp::expand(Expression::fromCref(cref.clone(), false)?, false, false)?).0, (std::sync::Arc::new(statementOutputCrefFinder2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs.clone())?
        },
        _ => crefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

fn statementOutputCrefFinder2(mut exp: Arc<Expression::NFExpression>, mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = crefs;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()) && !(ComponentRef::isIterator(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))) => {
            UnorderedSet::add(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), crefs.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

