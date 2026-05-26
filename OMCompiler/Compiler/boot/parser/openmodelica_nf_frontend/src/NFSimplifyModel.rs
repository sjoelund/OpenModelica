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
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_frontend_types::DAE;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn simplify(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = simplifyVariable(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.equations = simplifyEquations(flatModel.equations.clone())?,
        flatModel.initialEquations = simplifyEquations(flatModel.initialEquations.clone())?,
        flatModel.algorithms = simplifyAlgorithms(flatModel.algorithms.clone())?,
        flatModel.initialAlgorithms = simplifyAlgorithms(flatModel.initialAlgorithms.clone())?
    );
    execStat((literal!("NFSimplifyModel.simplify")).clone())?;
    Ok(flatModel)
}

pub fn simplifyVariable(mut var: Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    assign_field!(
        var.binding = simplifyBinding(var.binding.clone())?,
        var.typeAttributes = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
            let __x = simplifyTypeAttribute(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        var.children = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (var.children.clone()).into_iter().cloned() {
            let __x = simplifyVariable(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
    );
    Ok(var)
}

pub fn simplifyBinding(mut binding: Arc<Binding::NFBinding>) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut sexp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Binding::isBound(binding.clone()) {
        exp = Binding::getTypedExp(binding.clone())?;
        sexp = SimplifyExp::simplify(exp.clone(), false)?;
        sexp = removeEmptyFunctionArguments(sexp.clone(), false)?;
        if !(referenceEq(&exp.clone(),&sexp.clone())) {
            binding = Binding::setTypedExp(sexp.clone(), binding.clone())?;
        }
    }
    Ok(binding)
}

pub fn simplifyTypeAttribute(mut attribute: (ArcStr, Arc<Binding::NFBinding>)) -> Result<(ArcStr, Arc<Binding::NFBinding>)> {
    let mut attribute: (ArcStr, Arc<Binding::NFBinding>) = attribute;
    let mut name: ArcStr = arcstr::literal!("");
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut sbinding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    (name, binding) = attribute.clone();
    sbinding = simplifyBinding(binding.clone())?;
    if !(referenceEq(&binding.clone(),&sbinding.clone())) {
        attribute = (name.clone(), sbinding.clone());
    }
    Ok(attribute)
}

pub fn simplifyDimension(mut dim: Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> {
    let mut outDim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    outDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e = SimplifyExp::simplify(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone(), false)?;
            if (referenceEq(&e.clone(),&var_field!((*dim).exp, Dimension::NFDimension::EXP).clone())) {dim.clone()} else {Dimension::fromExp(e.clone(), var_field!((*dim).var, Dimension::NFDimension::EXP).clone())?}
        },
        _ => {
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

pub fn simplifyEquations(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut outEql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    for mut eq in &*eql.clone() {
        let mut eq = eq.clone();
        outEql = simplifyEquation(eq.clone(), outEql.clone())?;
    }
    outEql = outEql.clone().reverse();
    Ok(outEql)
}

pub fn simplifyEquation(mut eq: Arc<Equation::NFEquation>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    equations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            simplifyEqualityEquation(eq.clone(), equations.clone())?
        },
        Deref @ Equation::FOR { range: Some(_), .. } => {
            let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
            body = simplifyEquations(var_field!((*eq).body, Equation::NFEquation::FOR).clone())?;
            if !(Equation::containsExpList(body.clone(), Arc::new({ let __pe_b1 = var_field!((*eq).iterator, Equation::NFEquation::FOR).clone(); move |__pe_a0| Expression::containsIterator(__pe_a0, __pe_b1.clone()) }))?) {
                equations = List::append_reverse(body.clone(), equations.clone());
            } else {
                assign_variant_field!(eq => Equation::NFEquation::FOR;
                    range = Util::applyOption(var_field!((*eq).range, Equation::NFEquation::FOR).clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone()) })),
                    body = body.clone()
                );
                equations = cons(eq.clone(), equations.clone());
            }
            equations.clone()
        },
        Deref @ Equation::IF { .. } => {
            simplifyIfEqBranches(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), var_field!((*eq).scope, Equation::NFEquation::IF).clone(), var_field!((*eq).source, Equation::NFEquation::IF).clone(), equations.clone())?
        },
        Deref @ Equation::WHEN { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, Equation::NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            assign_variant_field!(b => Equation::Branch::Branch::BRANCH;
                condition = SimplifyExp::simplify(var_field!((*b).condition, Equation::Branch::Branch::BRANCH).clone(), false)?,
                body = simplifyEquations(var_field!((*b).body, Equation::Branch::Branch::BRANCH).clone())?
            );
            b.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            cons(eq.clone(), equations.clone())
        },
        Deref @ Equation::ASSERT { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::ASSERT; condition = SimplifyExp::simplify(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), false)?);
            if (Expression::isTrue(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone())) {equations.clone()} else {cons(eq.clone(), equations.clone())}
        },
        Deref @ Equation::REINIT { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::REINIT; reinitExp = SimplifyExp::simplify(var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), false)?);
            cons(eq.clone(), equations.clone())
        },
        Deref @ Equation::NORETCALL { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e = SimplifyExp::simplify(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), false)?;
            if Expression::isCall(e.clone()) {
                assign_variant_field!(eq => Equation::NFEquation::NORETCALL; exp = removeEmptyFunctionArguments(e.clone(), false)?);
                equations = cons(eq.clone(), equations.clone());
            }
            equations.clone()
        },
        _ => {
            cons(eq.clone(), equations.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub fn simplifyEqualityEquation(mut eq: Arc<Equation::NFEquation>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = equations;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut src: Arc<DAE::ElementSource>;
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scalarize_mode: Equation::ScalarizeMode = Equation::ScalarizeMode::DONT_SCALARIZE;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { scalarizeMode: __pa0, source: __pa1, scope: __pa2, ty: __pa3, rhs: __pa4, lhs: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    scalarize_mode = __pa0.clone();
    src = __pa1.clone();
    scope = __pa2.clone();
    ty = __pa3.clone();
    rhs = __pa4.clone();
    lhs = __pa5.clone();
    ty = Type::mapDims(ty.clone(), Arc::new(simplifyDimension));
    if Type::isEmptyArray(ty.clone()) {
        return Ok(equations);
    }
    lhs = SimplifyExp::simplify(lhs.clone(), false)?;
    lhs = removeEmptyTupleElements(lhs.clone());
    rhs = SimplifyExp::simplify(rhs.clone(), false)?;
    rhs = removeEmptyFunctionArguments(rhs.clone(), false)?;
    equations = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ Expression::TUPLE { .. }, Deref @ Expression::TUPLE { .. }) => simplifyTupleElement(var_field!((*lhs).elements, Expression::NFExpression::TUPLE).clone(), var_field!((*rhs).elements, Expression::NFExpression::TUPLE).clone(), ty.clone(), src.clone(), Arc::new({ let __pe_b4 = scope.clone(); let __pe_b5 = scalarize_mode.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| Ok(Equation::makeEquality(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone(), __pe_b5.clone())) }), equations.clone())?,
        _ => cons(Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs.clone(), rhs: rhs.clone(), ty: ty.clone(), scope: scope.clone(), source: src.clone(), scalarizeMode: scalarize_mode.clone() }), equations.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub fn simplifyAlgorithms(mut algs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>) -> Result<Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>> {
    let mut outAlgs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    for mut alg in &*algs.clone() {
        let mut alg = alg.clone();
        alg = simplifyAlgorithm(alg.clone())?;
        if !(alg.statements.clone().is_empty()) {
            outAlgs = cons(alg.clone(), outAlgs.clone());
        }
    }
    outAlgs = outAlgs.clone().reverse();
    Ok(outAlgs)
}

pub fn simplifyAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = simplifyStatements(alg.statements.clone())?);
    Ok(alg)
}

pub fn simplifyStatements(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut outStmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    for mut s in &*stmts.clone() {
        let mut s = s.clone();
        outStmts = simplifyStatement(s.clone(), outStmts.clone())?;
    }
    outStmts = outStmts.clone().reverse();
    Ok(outStmts)
}

pub fn simplifyStatement(mut stmt: Arc<Statement::NFStatement>, mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = statements;
    statements = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            simplifyAssignment(stmt.clone(), statements.clone())?
        },
        Deref @ Statement::FOR { body: Deref @ metamodelica::List::Nil, .. } => {
            statements.clone()
        },
        Deref @ Statement::FOR { range: Some(e), .. } => {
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            dim = Type::nthDimension(Expression::typeOf(e.clone()), 1)?;
            if !(Dimension::isZero(dim.clone())?) {
                assign_variant_field!(stmt => Statement::NFStatement::FOR;
                    range = Some(SimplifyExp::simplify(e.clone(), false)?),
                    body = simplifyStatements(var_field!((*stmt).body, Statement::NFStatement::FOR).clone())?
                );
                statements = cons(stmt.clone(), statements.clone());
            }
            statements.clone()
        },
        Deref @ Statement::IF { .. } => {
            simplifyIfStmtBranches(var_field!((*stmt).branches, Statement::NFStatement::IF).clone(), var_field!((*stmt).source, Statement::NFStatement::IF).clone(), Arc::new(fnptr!(Statement::makeIf, Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>, Arc<DAE::ElementSource>)), Arc::new(simplifyStatements), statements.clone())?
        },
        Deref @ Statement::WHEN { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = simplifyWhenBranches(var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone())?);
            if (var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone().is_empty()) {statements.clone()} else {cons(stmt.clone(), statements.clone())}
        },
        Deref @ Statement::ASSERT { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::ASSERT;
                condition = SimplifyExp::simplify(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), false)?,
                message = SimplifyExp::simplify(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), false)?,
                level = SimplifyExp::simplify(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), false)?
            );
            cons(stmt.clone(), statements.clone())
        },
        Deref @ Statement::TERMINATE { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::TERMINATE; message = SimplifyExp::simplify(var_field!((*stmt).message, Statement::NFStatement::TERMINATE).clone(), false)?);
            cons(stmt.clone(), statements.clone())
        },
        Deref @ Statement::WHILE { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHILE;
                condition = SimplifyExp::simplify(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), false)?,
                body = simplifyStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone())?
            );
            cons(stmt.clone(), statements.clone())
        },
        Deref @ Statement::NORETCALL { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e = SimplifyExp::simplify(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), false)?;
            if Expression::isCall(e.clone()) {
                assign_variant_field!(stmt => Statement::NFStatement::NORETCALL; exp = removeEmptyFunctionArguments(e.clone(), false)?);
                statements = cons(stmt.clone(), statements.clone());
            }
            statements.clone()
        },
        _ => {
            cons(stmt.clone(), statements.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statements)
}

pub fn simplifyWhenBranches(mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>) -> Result<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>> {
    let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = branches;
    branches = (::match_deref::match_deref! { match &(branches.clone()) {
        Deref @ metamodelica::List::Cons { head: (condition, body), tail: tail } => {
            let mut condition = (*condition).clone();
            let mut body = (*body).clone();
            condition = SimplifyExp::simplify(condition.clone(), false)?;
            body = simplifyStatements(body.clone())?;
            if (Expression::isBoolean(condition.clone())) {simplifyWhenBranches(tail.clone())?} else {cons((condition.clone(), body.clone()), simplifyWhenBranches(tail.clone())?)}
        },
        _ => {
            branches.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(branches)
}

pub fn simplifyAssignment(mut stmt: Arc<Statement::NFStatement>, mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = statements;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs_rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut src: Arc<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { source: __pa0, ty: __pa1, rhs: __pa2, lhs: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    src = __pa0.clone();
    ty = __pa1.clone();
    rhs = __pa2.clone();
    lhs = __pa3.clone();
    ty = Type::mapDims(ty.clone(), Arc::new(simplifyDimension));
    if Type::isEmptyArray(ty.clone()) {
        return Ok(statements);
    }
    lhs = SimplifyExp::simplify(lhs.clone(), false)?;
    lhs = removeEmptyTupleElements(lhs.clone());
    rhs = SimplifyExp::simplify(rhs.clone(), false)?;
    rhs = removeEmptyFunctionArguments(rhs.clone(), false)?;
    statements = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ Expression::TUPLE { .. }, Deref @ Expression::TUPLE { .. }) => simplifyTupleElement(var_field!((*lhs).elements, Expression::NFExpression::TUPLE).clone(), var_field!((*rhs).elements, Expression::NFExpression::TUPLE).clone(), ty.clone(), src.clone(), Arc::new(fnptr!(Statement::makeAssignment, Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, Arc<DAE::ElementSource>)), statements.clone())?,
        _ => cons(Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: lhs.clone(), rhs: rhs.clone(), ty: ty.clone(), source: src.clone() }), statements.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statements)
}

pub fn simplifyTupleElement<ElementT: Clone + 'static>(mut lhsTuple: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut rhsTuple: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut ty: Arc<Type::NFType>, mut src: Arc<DAE::ElementSource>, mut makeFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, Arc<DAE::ElementSource>) -> Result<ElementT> + 'static>, mut statements: Arc<metamodelica::List<ElementT>>) -> Result<Arc<metamodelica::List<ElementT>>> {
    pub type MakeElement<ElementT: Clone> = fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>, Arc<Type::NFType>, Arc<DAE::ElementSource>) -> Result<ElementT>;

    let mut statements: Arc<metamodelica::List<ElementT>> = statements;
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rest_rhs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = rhsTuple.clone();
    let mut ety: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut rest_ty: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::TUPLE { types: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    rest_ty = __pa0.clone();
    for mut lhs in &*lhsTuple.clone() {
        let mut lhs = lhs.clone();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(rest_rhs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        rhs = __pa1.clone();
        rest_rhs = __pa2.clone();
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rest_ty.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ety = __pa3.clone();
        rest_ty = __pa4.clone();
        if !(Expression::isWildCref(lhs.clone())) {
            statements = cons(makeFn(lhs.clone(), rhs.clone(), ety.clone(), src.clone())?, statements.clone());
        }
    }
    Ok(statements)
}

pub fn removeEmptyTupleElements(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::TUPLE { ty: Deref @ Type::TUPLE { types: tyl, .. }, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::TUPLE; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for (e, t) in (&(var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone())).into_iter().zip((&(tyl.clone())).into_iter()) {
            let __x = if (Type::isEmptyArray(t.clone())) {Arc::new(Expression::NFExpression::CREF { ty: t.clone(), cref: Arc::new(crate::NFComponentRef::WILD) })} else {e.clone()};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn removeEmptyFunctionArguments(mut exp: Arc<Expression::NFExpression>, mut isArg: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut is_arg: bool = false;
    if isArg.clone() {
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (Type::isEmptyArray(var_field!((*exp).ty, Expression::NFExpression::CREF).clone())) => {
            outExp = Expression::fillType(var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }))?;
            return Ok(outExp);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    is_arg = isArg.clone() || Expression::isCall(exp.clone());
    outExp = Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = is_arg.clone(); move |__pe_a0| removeEmptyFunctionArguments(__pe_a0, __pe_b1.clone()) }))?;
    Ok(outExp)
}

pub fn simplifyIfEqBranches(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut scope: Arc<InstNode::InstNode>, mut src: Arc<DAE::ElementSource>, mut elements: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut elements: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = elements;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut var: Variability = Variability::CONSTANT;
    let mut accum: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>> = metamodelica::nil();
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        accum = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { condition: cond, conditionVar: var, body } => {
            let mut cond = (*cond).clone();
            cond = SimplifyExp::simplify(cond.clone(), false)?;
            if Expression::isTrue(cond.clone()) {
                if accum.clone().is_empty() {
                    for mut eq in &*body.clone() {
                        let mut eq = eq.clone();
                        elements = simplifyEquation(eq.clone(), elements.clone())?;
                    }
                    return Ok(elements);
                } else {
                    accum = cons(Equation::makeBranch(cond.clone(), simplifyEquations(body.clone())?, Variability::CONTINUOUS.clone()), accum.clone());
                    accum = List::trim(accum.clone(), Arc::new(Equation::Branch::isEmpty))?;
                    elements = cons(Equation::makeIf(accum.clone().reverse(), scope.clone(), src.clone()), elements.clone());
                    return Ok(elements);
                }
            } else if !(Expression::isFalse(cond.clone())) {
                accum = cons(Equation::makeBranch(cond.clone(), simplifyEquations(body.clone())?, Variability::CONTINUOUS.clone()), accum.clone());
            }
            accum.clone()
        },
        Deref @ Equation::Branch::INVALID_BRANCH { branch: Deref @ Equation::Branch::BRANCH { conditionVar: var, condition: cond, .. }, .. } => {
            let mut cond = (*cond).clone();
            if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                cond = Ceval::evalExp(cond.clone(), Ceval::noTarget().clone())?;
            }
            if !(Expression::isFalse(cond.clone())) {
                Equation::Branch::triggerErrors(branch.clone())?;
            }
            accum.clone()
        },
        _ => cons(branch.clone(), accum.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    accum = List::trim(accum.clone(), Arc::new(Equation::Branch::isEmpty))?;
    if !(accum.clone().is_empty()) {
        elements = cons(Equation::makeIf(accum.clone().reverse(), scope.clone(), src.clone()), elements.clone());
    }
    Ok(elements)
}

pub fn simplifyIfStmtBranches<ElemT: Clone + 'static>(mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<ElemT>>)>>, mut src: Arc<DAE::ElementSource>, mut makeFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<ElemT>>)>>, Arc<DAE::ElementSource>) -> Result<ElemT> + 'static>, mut simplifyFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ElemT>>) -> Result<Arc<metamodelica::List<ElemT>>> + 'static>, mut elements: Arc<metamodelica::List<ElemT>>) -> Result<Arc<metamodelica::List<ElemT>>> {
    pub type MakeFunc<ElemT: Clone> = fn(Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<ElemT>>)>>, Arc<DAE::ElementSource>) -> Result<ElemT>;

    pub type SimplifyFunc<ElemT: Clone> = fn(Arc<metamodelica::List<ElemT>>) -> Result<Arc<metamodelica::List<ElemT>>>;

    let mut elements: Arc<metamodelica::List<ElemT>> = elements;
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<ElemT>> = metamodelica::nil();
    let mut accum: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<ElemT>>)>> = metamodelica::nil();
    for mut branch in &*branches.clone() {
        let mut branch = branch.clone();
        (cond, body) = branch.clone();
        cond = SimplifyExp::simplify(cond.clone(), false)?;
        if Expression::isTrue(cond.clone()) {
            if accum.clone().is_empty() {
                elements = listAppend(simplifyFunc(body.clone())?.reverse(), elements.clone());
                return Ok(elements);
            } else {
                accum = cons((cond.clone(), simplifyFunc(body.clone())?), accum.clone());
                break;
            }
        } else if !(Expression::isFalse(cond.clone())) {
            accum = cons((cond.clone(), simplifyFunc(body.clone())?), accum.clone());
        }
    }
    if !(accum.clone().is_empty()) {
        elements = cons(makeFunc(accum.clone().reverse(), src.clone())?, elements.clone());
    }
    Ok(elements)
}

pub fn simplifyFunction(mut func: Arc<Function::Function>) -> Result<()> {
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut fn_body: Arc<Algorithm::NFAlgorithm>;
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    if !(Function::isSimplified(func.clone())) {
        Function::markSimplified(func.clone());
        Function::mapExp(func.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone()) }), Arc::new({ let __pe_b1 = false; move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone()) }), true, false)?;
        cls = InstNode::getClass(func.node.clone())?;
        let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { sections, .. } => {
            let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ Sections::SECTIONS { algorithms: Deref @ metamodelica::List::Cons { head: fn_body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut fn_body = (*fn_body).clone();
            assign_field!(fn_body.statements = simplifyStatements(fn_body.statements.clone())?);
            assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = list![fn_body.clone()]);
            assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; sections = sections.clone());
            InstNode::updateClass(cls.clone(), func.node.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        for mut fn_der in &*func.derivatives.clone() {
            let mut fn_der = fn_der.clone();
            for mut der_fn in &*Function::getCachedFuncs(fn_der.derivativeFn.clone())? {
                let mut der_fn = der_fn.clone();
                simplifyFunction(der_fn.clone())?;
            }
        }
    }
    Ok(())
}

pub fn combineBinaries(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut var in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Variable::mapExp(var.clone(), SimplifyExp::combineBinaries)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.equations = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eqn in (flatModel.equations.clone()).into_iter().cloned() {
            let __x = Equation::mapExp(eqn.clone(), SimplifyExp::combineBinaries);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.initialEquations = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eqn in (flatModel.initialEquations.clone()).into_iter().cloned() {
            let __x = Equation::mapExp(eqn.clone(), SimplifyExp::combineBinaries);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.algorithms = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut alg in (flatModel.algorithms.clone()).into_iter().cloned() {
            let __x = Algorithm::mapExp(alg.clone(), Arc::new(SimplifyExp::combineBinaries));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.initialAlgorithms = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut alg in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
            let __x = Algorithm::mapExp(alg.clone(), Arc::new(SimplifyExp::combineBinaries));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
    );
    Ok(flatModel)
}

