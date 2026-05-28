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
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::Branch;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPackage as Package;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvalSettings {
    pub scalarize: bool,
}

impl Default for EvalSettings {
    fn default() -> Self {
        Self {
            scalarize: Default::default(),
        }
    }
}

pub type SETTINGS = EvalSettings;


pub fn evaluate(mut flatModel: Arc<FlatModel::NFFlatModel>, mut context: i32) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut settings: EvalSettings = <EvalSettings as ::std::default::Default>::default();
    settings = EvalSettings { scalarize: Flags::isSet(Flags::NF_SCALARIZE.clone())? };
    assign_field!(
        flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = evaluateVariable(v.clone(), context.clone(), settings.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        flatModel.equations = evaluateEquations(flatModel.equations.clone()),
        flatModel.initialEquations = evaluateEquations(flatModel.initialEquations.clone()),
        flatModel.algorithms = evaluateAlgorithms(flatModel.algorithms.clone()),
        flatModel.initialAlgorithms = evaluateAlgorithms(flatModel.initialAlgorithms.clone())
    );
    execStat((literal!("NFEvalConstants.evaluate")).clone())?;
    Ok(flatModel)
}

pub fn evaluateVariable(mut var: Arc<Variable::NFVariable>, mut context: i32, mut settings: EvalSettings) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut structural: bool = false;
    let mut variability: Variability = Variability::CONSTANT;
    variability = Variable::variability(var.clone());
    structural = variability.clone() <= Variability::STRUCTURAL_PARAMETER.clone() && !(Type::isExternalObject(var.ty.clone()));
    binding = evaluateBinding(var.binding.clone(), var.name.clone(), structural.clone(), variability.clone(), context.clone())?;
    if !(referenceEq(&binding.clone(),&var.binding.clone())) {
        assign_field!(var.binding = binding.clone());
    }
    assign_field!(
        var.typeAttributes = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
            let __x = evaluateTypeAttribute(a.clone(), var.name.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        var.children = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (var.children.clone()).into_iter().cloned() {
            let __x = evaluateVariable(v.clone(), context.clone(), settings.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
    );
    Ok(var)
}

pub fn evaluateBinding(mut binding: Arc<Binding::NFBinding>, mut prefix: Arc<ComponentRef::NFComponentRef>, mut structural: bool, mut variability: Variability, mut context: i32) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eexp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    if Binding::isBound(binding.clone()) {
        exp = Binding::getTypedExp(binding.clone())?;
        if structural.clone() {
            info = Binding::getInfo(binding.clone());
            eexp = evaluateExp(exp.clone(), info.clone())?;
            eexp = SimplifyExp::simplify(eexp.clone(), false)?;
            if !(Expression::isLiteral(eexp.clone()) || Expression::isKnownSizeFill(eexp.clone())?) {
                if variability.clone() > Variability::CONSTANT.clone() || InstContext::inRelaxed(context.clone()) {
                    eexp = Ceval::tryEvalExp(eexp.clone(), Ceval::noTarget().clone());
                } else {
                    eexp = Ceval::evalExp(eexp.clone(), Ceval::EvalTarget::new(info.clone(), context.clone(), None))?;
                }
            }
            eexp = Flatten::flattenExp(eexp.clone(), Arc::new(Flatten::Prefix::Prefix::PREFIX { root: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), prefix: prefix.clone() }), info.clone())?;
        } else {
            eexp = evaluateExp(exp.clone(), Binding::getInfo(binding.clone()))?;
        }
        if !(referenceEq(&exp.clone(),&eexp.clone())) {
            binding = Binding::setTypedExp(eexp.clone(), binding.clone())?;
        }
    }
    Ok(binding)
}

pub fn evaluateTypeAttribute(mut attribute: (ArcStr, Arc<Binding::NFBinding>), mut prefix: Arc<ComponentRef::NFComponentRef>, mut context: i32) -> Result<(ArcStr, Arc<Binding::NFBinding>)> {
    let mut attribute: (ArcStr, Arc<Binding::NFBinding>) = attribute;
    let mut name: ArcStr = arcstr::literal!("");
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut sbinding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut structural: bool = false;
    (name, binding) = attribute.clone();
    structural = name.clone() == literal!("fixed") || name.clone() == literal!("stateSelect");
    sbinding = evaluateBinding(binding.clone(), prefix.clone(), structural.clone(), Variability::PARAMETER.clone(), context.clone())?;
    if !(referenceEq(&binding.clone(),&sbinding.clone())) {
        attribute = (name.clone(), sbinding.clone());
    }
    Ok(attribute)
}

pub fn evaluateExp(mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (outExp, _) = evaluateExpTraverser(exp.clone(), info.clone(), false)?;
    Ok(outExp)
}

pub fn evaluateExpTraverser(mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo, mut changed: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outChanged: bool = false;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    (outExp, outChanged) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            let (__pa2, __pa0, __pa1, __pa3) = ::match_deref::match_deref! { match &(Expression::mapFoldShallow(exp.clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }), false)?) {
                (__pa2 @ Deref @ Expression::CREF { ty: __pa0, cref: __pa1 }, __pa3) => (__pa2.clone(), __pa0.clone(), __pa1.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            cref = __pa1.clone();
            outExp = __pa2.clone();
            outChanged = __pa3.clone();
            var = ComponentRef::nodeVariability(cref.clone())?;
            if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() && !(Type::isExternalObject(ty.clone())) {
                if var.clone() > Variability::CONSTANT.clone() {
                    ErrorExt::setCheckpoint((literal!("NFEvalConstants.evaluateExpTraverser")).clone());
                    match '__try4: {
                        e = unwrap_break_err!(Ceval::evalCref(cref.clone(), outExp.clone(), Ceval::noTarget().clone(), false, true), '__try4);
                        e = unwrap_break_err!(Flatten::flattenExp(e.clone(), Arc::new(Flatten::Prefix::Prefix::PREFIX { root: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), prefix: cref.clone() }), info.clone()), '__try4);
                        outExp = e.clone();
                        outChanged = true;
                        Ok::<_, anyhow::Error>((e.clone(), outChanged.clone(), outExp.clone()))
                    } {
                        Ok((__try4_o0, __try4_o1, __try4_o2)) => {
                            e = __try4_o0;
                            outChanged = __try4_o1;
                            outExp = __try4_o2;
                        }
                        Err(_) => {
                            bail!("try/else: outputs not set in else branch");
                        }
                    }
                    ErrorExt::rollBack((literal!("NFEvalConstants.evaluateExpTraverser")).clone());
                } else {
                    outExp = Ceval::evalCref(cref.clone(), outExp.clone(), Ceval::EvalTarget::new(info.clone(), InstContext::NO_CONTEXT.clone(), None), false, true)?;
                    outExp = Flatten::flattenExp(outExp.clone(), Arc::new(Flatten::Prefix::Prefix::PREFIX { root: Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), prefix: cref.clone() }), info.clone())?;
                    outChanged = true;
                }
            } else if outChanged.clone() {
                ty = ComponentRef::getSubscriptedType(cref.clone(), false)?;
            }
            ty2 = evaluateType(ty.clone(), info.clone())?;
            if !(referenceEq(&ty.clone(),&ty2.clone())) {
                outExp = Expression::setType(ty2.clone(), outExp.clone())?;
            }
            (outExp.clone(), outChanged.clone())
        },
        Deref @ Expression::ARRAY { literal: true, .. } => (exp.clone(), false),
        Deref @ Expression::IF { .. } => evaluateIfExp(exp.clone(), info.clone())?,
        Deref @ Expression::SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                (e, outChanged) = Expression::mapFoldShallow(e.clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }), false)?;
                if outChanged.clone() {
                    assign_variant_field!(exp => Expression::NFExpression::SIZE; dimIndex = Some(e.clone()));
                }
            }
            (exp.clone(), outChanged.clone())
        },
        Deref @ Expression::RANGE { .. } => {
            (outExp, outChanged) = Expression::mapFoldShallow(exp.clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }), false)?;
            if outChanged.clone() {
                outExp = Expression::retype(outExp.clone())?;
            }
            (outExp.clone(), outChanged.clone())
        },
        _ => {
            (outExp, outChanged) = Expression::mapFoldShallow(exp.clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }), false)?;
            ty = Expression::typeOf(outExp.clone());
            ty2 = evaluateType(ty.clone(), info.clone())?;
            (if (referenceEq(&ty.clone(),&ty2.clone())) {outExp.clone()} else {Expression::setType(ty2.clone(), outExp.clone())?}, outChanged.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outChanged = changed.clone() || outChanged.clone();
    Ok((outExp, outChanged))
}

pub fn evaluateType(mut ty: Arc<Type::NFType>, mut info: SourceInfo) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = ty;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { .. } => {
            assign_variant_field!(ty => Type::NFType::ARRAY; dimensions = {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (var_field!((*ty).dimensions, Type::NFType::ARRAY).clone()).into_iter().cloned() {
            let __x = evaluateDimension(d.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ty.clone()
        },
        Deref @ Type::CONDITIONAL_ARRAY { .. } => Type::simplifyConditionalArray(ty.clone()),
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn evaluateDimension(mut dim: Arc<Dimension::NFDimension>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut outDim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    outDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e = evaluateExp(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone(), info.clone())?;
            if (referenceEq(&e.clone(),&var_field!((*dim).exp, Dimension::NFDimension::EXP).clone())) {dim.clone()} else {Dimension::fromExp(e.clone(), var_field!((*dim).var, Dimension::NFDimension::EXP).clone())?}
        },
        _ => {
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

pub fn evaluateIfExp(mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outChanged: bool = false;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c1: bool = false;
    let mut c2: bool = false;
    let mut matched_branch: Type::Branch = Type::Branch::NONE;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    tb = __pa2.clone();
    fb = __pa3.clone();
    (cond, outChanged) = evaluateExpTraverser(cond.clone(), info.clone(), false)?;
    cond = SimplifyExp::simplify(cond.clone(), false)?;
    (outExp, outChanged) = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::BOOLEAN { .. } => {
            (outExp, _) = evaluateExpTraverser(if (var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone()) {tb.clone()} else {fb.clone()}, info.clone(), false)?;
            (outExp.clone(), true)
        },
        _ => {
            (tb, c1) = evaluateExpTraverser(tb.clone(), info.clone(), false)?;
            (fb, c2) = evaluateExpTraverser(fb.clone(), info.clone(), false)?;
            (Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: tb.clone(), falseBranch: fb.clone() }), outChanged.clone() || c1.clone() || c2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outChanged))
}

pub fn evaluateEquations(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Arc<metamodelica::List<Arc<Equation::NFEquation>>> {
    let mut outEql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (eql.clone()).into_iter().cloned() {
            let __x = evaluateEquation(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outEql
}

pub fn evaluateEquation(mut eq: Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    let mut info: SourceInfo = Equation::info(eq.clone());
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ty = Type::mapDims(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0| evaluateDimension(__pe_a0, __pe_b1.clone()) }));
            e1 = evaluateExp(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), info.clone())?;
            Arc::new(Equation::NFEquation::EQUALITY { lhs: e1.clone(), rhs: e2.clone(), ty: ty.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone(), scalarizeMode: var_field!((*eq).scalarizeMode, Equation::NFEquation::EQUALITY).clone() })
        },
        Deref @ Equation::FOR { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::FOR;
                range = Util::applyOption(var_field!((*eq).range, Equation::NFEquation::FOR).clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0| evaluateExp(__pe_a0, __pe_b1.clone()) })),
                body = evaluateEquations(var_field!((*eq).body, Equation::NFEquation::FOR).clone())
            );
            eq.clone()
        },
        Deref @ Equation::IF { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::IF; branches = {
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, Equation::NFEquation::IF).clone()).into_iter().cloned() {
            let __x = evaluateEqBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eq.clone()
        },
        Deref @ Equation::WHEN { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, Equation::NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = evaluateEqBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eq.clone()
        },
        Deref @ Equation::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = evaluateExp(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), info.clone())?;
            e3 = evaluateExp(var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), info.clone())?;
            Arc::new(Equation::NFEquation::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::ASSERT).clone(), source: var_field!((*eq).source, Equation::NFEquation::ASSERT).clone() })
        },
        Deref @ Equation::TERMINATE { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::TERMINATE; message = evaluateExp(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), info.clone())?);
            eq.clone()
        },
        Deref @ Equation::REINIT { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::REINIT; reinitExp = evaluateExp(var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), info.clone())?);
            eq.clone()
        },
        Deref @ Equation::NORETCALL { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::NORETCALL; exp = evaluateExp(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), info.clone())?);
            eq.clone()
        },
        _ => {
            eq.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub fn evaluateEqBranch(mut branch: Arc<Branch::Branch>, mut info: SourceInfo) -> Result<Arc<Branch::Branch>> {
    let mut outBranch: Arc<Branch::Branch>;
    outBranch = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { body, condition, .. } => {
            let mut body = (*body).clone();
            let mut condition = (*condition).clone();
            condition = evaluateExp(condition.clone(), info.clone())?;
            body = evaluateEquations(body.clone());
            Arc::new(Branch::Branch::BRANCH { condition: condition.clone(), conditionVar: var_field!((*branch).conditionVar, Branch::Branch::BRANCH).clone(), body: body.clone() })
        },
        _ => {
            branch.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBranch)
}

pub fn evaluateAlgorithms(mut algs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>) -> Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> {
    let mut outAlgs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (algs.clone()).into_iter().cloned() {
            let __x = evaluateAlgorithm(a.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outAlgs
}

pub fn evaluateAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = evaluateStatements(alg.statements.clone()));
    alg
}

pub fn evaluateStatements(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Arc<metamodelica::List<Arc<Statement::NFStatement>>> {
    let mut outStmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (stmts.clone()).into_iter().cloned() {
            let __x = evaluateStatement(s.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outStmts
}

pub fn evaluateStatement(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    let mut info: SourceInfo = Statement::info(stmt.clone());
    stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            ty = Type::mapDims(var_field!((*stmt).ty, Statement::NFStatement::ASSIGNMENT).clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0| evaluateDimension(__pe_a0, __pe_b1.clone()) }));
            e1 = evaluateExp(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), info.clone())?;
            Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: e1.clone(), rhs: e2.clone(), ty: ty.clone(), source: var_field!((*stmt).source, Statement::NFStatement::ASSIGNMENT).clone() })
        },
        Deref @ Statement::FOR { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::FOR;
                range = Util::applyOption(var_field!((*stmt).range, Statement::NFStatement::FOR).clone(), Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0| evaluateExp(__pe_a0, __pe_b1.clone()) })),
                body = evaluateStatements(var_field!((*stmt).body, Statement::NFStatement::FOR).clone())
            );
            stmt.clone()
        },
        Deref @ Statement::IF { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::IF; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, Statement::NFStatement::IF).clone()).into_iter().cloned() {
            let __x = evaluateStmtBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ Statement::WHEN { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = evaluateStmtBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            stmt.clone()
        },
        Deref @ Statement::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            e1 = evaluateExp(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            e3 = evaluateExp(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            Arc::new(Statement::NFStatement::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*stmt).source, Statement::NFStatement::ASSERT).clone() })
        },
        Deref @ Statement::TERMINATE { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::TERMINATE; message = evaluateExp(var_field!((*stmt).message, Statement::NFStatement::TERMINATE).clone(), info.clone())?);
            stmt.clone()
        },
        Deref @ Statement::REINIT { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::REINIT; reinitExp = evaluateExp(var_field!((*stmt).reinitExp, Statement::NFStatement::REINIT).clone(), info.clone())?);
            stmt.clone()
        },
        Deref @ Statement::NORETCALL { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::NORETCALL; exp = evaluateExp(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), info.clone())?);
            stmt.clone()
        },
        Deref @ Statement::WHILE { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHILE;
                condition = evaluateExp(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), info.clone())?,
                body = evaluateStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone())
            );
            stmt.clone()
        },
        _ => {
            stmt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub fn evaluateStmtBranch(mut branch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)> {
    let mut outBranch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
    (cond, body) = branch.clone();
    cond = evaluateExp(cond.clone(), info.clone())?;
    body = evaluateStatements(body.clone());
    outBranch = (cond.clone(), body.clone());
    Ok(outBranch)
}

pub fn evaluateFunction(mut func: Arc<Function::Function>) -> Result<Arc<Function::Function>> {
    let mut func: Arc<Function::Function> = func;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut fn_body: Arc<Algorithm::NFAlgorithm> = Arc::new(<Algorithm::NFAlgorithm as ::std::default::Default>::default());
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    let mut is_con: bool = false;
    if !(Function::isEvaluated(func.clone())) {
        Function::markEvaluated(func.clone());
        is_con = Function::isDefaultRecordConstructor(func.clone());
        func = Function::mapExp(func.clone(), Arc::new({ let __pe_b1 = func.node.clone(); let __pe_b2 = is_con.clone(); move |__pe_a0| evaluateFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), Arc::new({ let __pe_b1 = func.node.clone(); let __pe_b2 = true; move |__pe_a0| evaluateFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), true, true)?;
        if is_con.clone() {
            Record::checkLocalFieldOrder(func.locals.clone(), func.node.clone(), InstNode::info(func.node.clone())?)?;
        }
        for mut fn_der in &*func.derivatives.clone() {
            let mut fn_der = fn_der.clone();
            for mut der_fn in &*Function::getCachedFuncs(fn_der.derivativeFn.clone())? {
                let mut der_fn = der_fn.clone();
                evaluateFunction(der_fn.clone())?;
            }
        }
    }
    Ok(func)
}

pub fn evaluateFuncExp(mut exp: Arc<Expression::NFExpression>, mut fnNode: Arc<InstNode::InstNode>, mut evaluateAll: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (outExp, _) = evaluateFuncExpTraverser(exp.clone(), fnNode.clone(), evaluateAll.clone(), false)?;
    Ok(outExp)
}

pub fn evaluateFuncExpTraverser(mut exp: Arc<Expression::NFExpression>, mut fnNode: Arc<InstNode::InstNode>, mut evaluateAll: bool, mut changed: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outChanged: bool = false;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (e, outChanged) = Expression::mapFoldShallow(exp.clone(), Arc::new({ let __pe_b1 = fnNode.clone(); let __pe_b2 = evaluateAll.clone(); move |__pe_a0, __pe_a3| evaluateFuncExpTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), false)?;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::CREF { .. } => {
            if evaluateAll.clone() || !(isLocalFunctionVariable(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), fnNode.clone())?) {
                ErrorExt::setCheckpoint((literal!("NFEvalConstants.evaluateFuncExpTraverser")).clone());
                match '__try0: {
                    outExp = unwrap_break_err!(Ceval::evalCref(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), e.clone(), Ceval::noTarget().clone(), false, true), '__try0);
                    Ok::<_, anyhow::Error>((outExp.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        outExp = __try0_o0;
                    }
                    Err(_) => {
                        outExp = e.clone();
                    }
                }
                ErrorExt::rollBack((literal!("NFEvalConstants.evaluateFuncExpTraverser")).clone());
                outChanged = true;
            } else if outChanged.clone() {
                outExp = Arc::new(Expression::NFExpression::CREF { ty: ComponentRef::getSubscriptedType(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), false)?, cref: var_field!((*e).cref, Expression::NFExpression::CREF).clone() });
            } else {
                outExp = e.clone();
            }
            outExp.clone()
        },
        _ => if (outChanged.clone()) {Expression::retype(e.clone())?} else {e.clone()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outChanged = changed.clone() || outChanged.clone();
    Ok((outExp, outChanged))
}

pub fn isLocalFunctionVariable(mut cref: Arc<ComponentRef::NFComponentRef>, mut fnNode: Arc<InstNode::InstNode>) -> Result<bool> {
    let mut res: bool = false;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut fnl: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    if ComponentRef::isPackageConstant(cref.clone())? {
        res = false;
    } else if ComponentRef::nodeVariability(cref.clone())? <= Variability::PARAMETER.clone() && ComponentRef::isCref(cref.clone()) {
        node = InstNode::instanceParent(ComponentRef::node(ComponentRef::last(cref.clone()))?);
        if InstNode::isClass(node.clone()) {
            fnl = Function::getCachedFuncs(node.clone())?;
            if fnl.clone().is_empty() {
                res = false;
            } else {
                r#fn = listHead(fnl.clone())?;
                res = InstNode::refEqual(fnNode.clone(), r#fn.node.clone());
            }
        } else {
            res = false;
        }
    } else {
        res = true;
    }
    Ok(res)
}

pub fn evaluateRecordDeclaration(mut recordNode: Arc<InstNode::InstNode>) -> Result<()> {
    ClassTree::applyComponents(Class::classTree(InstNode::getClass(recordNode.clone())?)?, Arc::new({ let __pe_b1 = recordNode.clone(); move |__pe_a0| evaluateRecordDeclarationField(__pe_a0, __pe_b1.clone()) }));
    Ok(())
}

pub fn evaluateRecordDeclarationField(mut fieldNode: Arc<InstNode::InstNode>, mut recordNode: Arc<InstNode::InstNode>) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut cls_inst: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    comp = InstNode::component(fieldNode.clone())?;
    binding = Component::getBinding(comp.clone());
    if Binding::isBound(binding.clone()) {
        binding = Binding::mapExp(binding.clone(), Arc::new({ let __pe_b1 = fieldNode.clone(); let __pe_b2 = false; move |__pe_a0| evaluateFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
        comp = Component::setBinding(binding.clone(), comp.clone())?;
    }
    cls_inst = Component::classInstance(comp.clone());
    if !(InstNode::isEmpty(cls_inst.clone())) {
        ClassTree::applyComponents(Class::classTree(InstNode::getClass(cls_inst.clone())?)?, Arc::new({ let __pe_b1 = recordNode.clone(); move |__pe_a0| evaluateRecordDeclarationField(__pe_a0, __pe_b1.clone()) }));
    }
    InstNode::updateComponent(comp.clone(), fieldNode.clone())?;
    Ok(())
}

