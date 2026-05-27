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
use crate::NFCallAttributes;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::Mutable;

pub struct NFExpandExp;
pub fn expand(mut exp: Arc<Expression::NFExpression>, mut backend: bool, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut expanded: bool = false;
    (exp, expanded) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            (exp.clone(), true)
        },
        Deref @ Expression::REAL { .. } => {
            (exp.clone(), true)
        },
        Deref @ Expression::STRING { .. } => {
            (exp.clone(), true)
        },
        Deref @ Expression::BOOLEAN { .. } => {
            (exp.clone(), true)
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            (exp.clone(), true)
        },
        Deref @ Expression::CREF { ty: Deref @ Type::ARRAY { .. }, .. } => {
            expandCref(exp.clone(), backend.clone(), resize.clone())?
        },
        Deref @ Expression::ARRAY { .. } if (Type::isVector(var_field!((*exp).ty, Expression::NFExpression::ARRAY).clone())) => {
            (exp.clone(), true)
        },
        Deref @ Expression::ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            (arr, expanded) = expandArray(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone())?;
            assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = arr.clone());
            (exp.clone(), expanded.clone())
        },
        Deref @ Expression::TYPENAME { .. } => {
            (expandTypename(var_field!((*exp).ty, Expression::NFExpression::TYPENAME).clone())?, true)
        },
        Deref @ Expression::RANGE { .. } => {
            expandRange(exp.clone())?
        },
        Deref @ Expression::CALL { .. } => {
            expandCall(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), exp.clone(), resize.clone())?
        },
        Deref @ Expression::SIZE { .. } => {
            expandSize(exp.clone())
        },
        Deref @ Expression::BINARY { .. } => {
            expandBinary(exp.clone(), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), resize.clone())?
        },
        Deref @ Expression::MULTARY { .. } => {
            expand(SimplifyExp::splitMultary(exp.clone())?, resize.clone(), false)?
        },
        Deref @ Expression::UNARY { .. } => {
            expandUnary(exp.clone())?
        },
        Deref @ Expression::LBINARY { .. } => {
            expandLogicalBinary(exp.clone())?
        },
        Deref @ Expression::LUNARY { .. } => {
            expandLogicalUnary(exp.clone())?
        },
        Deref @ Expression::RELATION { .. } => {
            (exp.clone(), true)
        },
        Deref @ Expression::CAST { .. } => {
            expandCast(exp.clone())?
        },
        Deref @ Expression::FILENAME { .. } => {
            (exp.clone(), true)
        },
        _ => {
            expandGeneric(exp.clone(), resize.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, expanded))
}

pub fn expandArray(mut arr: metamodelica::Array<Arc<Expression::NFExpression>>) -> Result<(metamodelica::Array<Arc<Expression::NFExpression>>, bool)> {
    let mut outArray: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut expanded: bool = true;
    let mut res: bool = false;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outArray = metamodelica::arrayFromVec(arr.clone().borrow().clone());
    let __range0 = 1..=(outArray.clone().borrow().len() as i32);
    for mut i in __range0 {
        (e, res) = expand(outArray.clone().borrow()[(i.clone()-1) as usize].clone(), false, false)?;
        if !(res.clone()) {
            expanded = false;
            return Ok((outArray, expanded));
        }
        {let _arr = outArray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = e.clone(); _arr};
    }
    Ok((outArray, expanded))
}

pub fn expandList(mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut abortOnFailure: bool) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, bool)> {
    let mut outExpl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut expanded: bool = true;
    let mut res: bool = false;
    for mut exp in &*expl.clone() {
        let mut exp = exp.clone();
        (exp, res) = expand(exp.clone(), false, false)?;
        expanded = res.clone() && expanded.clone();
        if !(res.clone()) && abortOnFailure.clone() {
            outExpl = expl.clone();
            return Ok((outExpl, expanded));
        }
        outExpl = cons(exp.clone(), outExpl.clone());
    }
    outExpl = outExpl.clone().reverse();
    Ok((outExpl, expanded))
}

pub fn expandCref(mut crefExp: Arc<Expression::NFExpression>, mut backend: bool, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    (arrayExp, expanded) = (::match_deref::match_deref! { match &(crefExp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            if Type::hasZeroDimension(var_field!((*crefExp).ty, Expression::NFExpression::CREF).clone()) {
                arrayExp = Expression::makeEmptyArray(var_field!((*crefExp).ty, Expression::NFExpression::CREF).clone());
                expanded = true;
            } else if Type::hasKnownSize(var_field!((*crefExp).ty, Expression::NFExpression::CREF).clone()) {
                subs = expandCref2(var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone(), backend.clone(), resize.clone(), metamodelica::nil())?;
                arrayExp = expandCref3(subs.clone(), var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone(), Type::arrayElementType(var_field!((*crefExp).ty, Expression::NFExpression::CREF).clone()), metamodelica::nil())?;
                expanded = true;
            } else {
                arrayExp = crefExp.clone();
                expanded = false;
            }
            (arrayExp.clone(), expanded.clone())
        },
        _ => (crefExp.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((arrayExp, expanded))
}

pub fn expandCref2(mut cref: Arc<ComponentRef::NFComponentRef>, mut backend: bool, mut resize: bool, mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>> {
    use crate::NFComponentRef::Origin;
    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = subs;
    let mut cr_subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    subs = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } if (backend.clone() || var_field!((*cref).origin, ComponentRef::NFComponentRef::CREF).clone() == Origin::CREF.clone()) => {
            dims = Type::arrayDims(var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone());
            cr_subs = Subscript::expandList(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), dims.clone(), resize.clone())?;
            if (cr_subs.clone().is_empty() && !(dims.clone().is_empty())) {metamodelica::nil()} else {expandCref2(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), backend.clone(), resize.clone(), cons(cr_subs.clone(), subs.clone()))?}
        },
        _ => subs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subs)
}

pub fn expandCref3(mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefType: Arc<Type::NFType>, mut accum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    arrayExp = (::match_deref::match_deref! { match &(subs.clone()) {
        Deref @ metamodelica::List::Nil => Arc::new(Expression::NFExpression::CREF { ty: crefType.clone(), cref: ComponentRef::setSubscriptsList(accum.clone(), cref.clone())? }),
        _ => expandCref4(listHead(subs.clone())?, metamodelica::nil(), accum.clone(), listRest(subs.clone())?, cref.clone(), crefType.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arrayExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn expandCref4(mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut comb: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut accum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>, mut restSubs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>, mut cref: Arc<ComponentRef::NFComponentRef>, mut crefType: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut slice: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut i: i32 = 0;
    arrayExp = (::match_deref::match_deref! { match &(subs.clone()) {
        Deref @ metamodelica::List::Nil => expandCref3(restSubs.clone(), cref.clone(), crefType.clone(), cons(comb.clone().reverse(), accum.clone()))?,
        Deref @ metamodelica::List::Cons { head: Deref @ Subscript::EXPANDED_SLICE { indices: slice }, tail: rest } => {
            expl = metamodelica::arrayCreate((slice.clone().len() as i32), Arc::new(Expression::NFExpression::INTEGER { value: 0 }));
            i = 1;
            for mut idx in &*slice.clone() {
                let mut idx = idx.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(expl.clone(), i.clone(), expandCref4(rest.clone(), cons(idx.clone(), comb.clone()), accum.clone(), restSubs.clone(), cref.clone(), crefType.clone())?) };
                i = i.clone() + 1;
            }
            arr_ty = Type::liftArrayLeft(Expression::typeOf(expl.clone().borrow()[(1-1) as usize].clone()), Dimension::fromExpArray(expl.clone()));
            Expression::makeArray(arr_ty.clone(), expl.clone(), false)
        },
        _ => expandCref4(listRest(subs.clone())?, cons(listHead(subs.clone())?, comb.clone()), accum.clone(), restSubs.clone(), cref.clone(), crefType.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arrayExp)
}

pub fn expandTypename(mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { elementType: Deref @ Type::BOOLEAN, .. } => {
            Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![Arc::new(Expression::NFExpression::BOOLEAN { value: false }), Arc::new(Expression::NFExpression::BOOLEAN { value: true })].into_iter().cloned().collect()), true)
        },
        Deref @ Type::ARRAY { elementType: Deref @ Type::ENUMERATION { .. }, .. } => {
            let mut lits: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            lits = Expression::makeEnumLiterals(var_field!((*ty).elementType, Type::NFType::ARRAY).clone())?;
            Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(lits.clone().into_iter().cloned().collect()), true)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpandExp.expandTypename")); __mm_s.push_str(&*literal!(" got invalid typename")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn expandRange(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut range_iter: Arc<RangeIterator::NFRangeIterator>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    expanded = Expression::isLiteral(exp.clone());
    if expanded.clone() {
        outExp = Ceval::evalExp(exp.clone(), Ceval::noTarget().clone())?;
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn expandCall(mut call: Arc<Call::NFCall>, mut exp: Arc<Expression::NFExpression>, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    (outExp, expanded) = 'mc: {
        let __mc_input = call.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Call::TYPED_CALL { .. } => {
                    if !((Function::isBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone()) && !(Function::isImpure(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone())))) { bail!("guard") }
                    Ok(expandBuiltinCall(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone(), var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone(), call.clone(), resize.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
                    Ok(expandArrayConstructor(var_field!((*call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*call).ty, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(expandGeneric(exp.clone(), resize.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, expanded))
}

pub fn expandBuiltinCall(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut fn_path: Arc<Absyn::Path> = Function::nameConsiderBuiltin(r#fn.clone())?;
    (outExp, expanded) = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(fn_path.clone())?) {
        Deref @ "cat" => expandBuiltinCat(args.clone(), call.clone(), resize.clone())?,
        Deref @ "der" => expandBuiltinGeneric(call.clone())?,
        Deref @ "diagonal" => expandBuiltinDiagonal(listHead(args.clone())?)?,
        Deref @ "fill" => expandBuiltinFill(args.clone())?,
        Deref @ "pre" => expandBuiltinGeneric(call.clone())?,
        Deref @ "previous" => expandBuiltinGeneric(call.clone())?,
        Deref @ "promote" => expandBuiltinPromote(args.clone())?,
        Deref @ "transpose" => expandBuiltinTranspose(listHead(args.clone())?)?,
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, expanded))
}

pub fn expandBuiltinCat(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    (expl, expanded) = expandList(listRest(args.clone())?, true)?;
    if expanded.clone() {
        exp = Ceval::evalBuiltinCat(listHead(args.clone())?, expl.clone(), Ceval::noTarget().clone())?;
    } else {
        (exp, _) = expandGeneric(Arc::new(Expression::NFExpression::CALL { call: call.clone() }), resize.clone())?;
    }
    Ok((exp, expanded))
}

pub fn expandBuiltinPromote(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut n: i32 = 0;
    let mut eexp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nexp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eexp = __pa0.clone();
    nexp = __pa1.clone();
    let __pa3 = ::match_deref::match_deref! { match &(nexp.clone()) {
        Deref @ Expression::INTEGER { value: __pa3 } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa3.clone();
    (eexp, expanded) = expand(eexp.clone(), false, false)?;
    (exp, _) = Expression::promote(eexp.clone(), Expression::typeOf(eexp.clone()), n.clone())?;
    Ok((exp, expanded))
}

pub fn expandBuiltinDiagonal(mut arg: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    (outExp, expanded) = expand(arg.clone(), false, false)?;
    if expanded.clone() {
        outExp = Ceval::evalBuiltinDiagonal(outExp.clone())?;
    }
    Ok((outExp, expanded))
}

pub fn expandBuiltinFill(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = true;
    outExp = Expression::fillArgs(listHead(args.clone())?, listRest(args.clone())?)?;
    Ok((outExp, expanded))
}

pub fn expandBuiltinTranspose(mut arg: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    (outExp, expanded) = expand(arg.clone(), false, false)?;
    if expanded.clone() {
        outExp = Expression::transposeArray(outExp.clone())?;
    }
    Ok((outExp, expanded))
}

pub fn expandBuiltinGeneric(mut call: Arc<Call::NFCall>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = true;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut attr: Arc<NFCallAttributes::NFCallAttributes>;
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { r#fn: __pa0, ty: __pa1, var: __pa2, purity: __pa3, arguments: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil }, attributes: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa0.clone();
    ty = __pa1.clone();
    var = __pa2.clone();
    pur = __pa3.clone();
    arg = __pa4.clone();
    attr = __pa5.clone();
    ty = Type::arrayElementType(ty.clone());
    let __pa7 = ::match_deref::match_deref! { match &(expand(arg.clone(), false, false)?) {
        (__pa7, true) => __pa7.clone(),
        _ => bail!("pattern mismatch"),
    } };
    arg = __pa7.clone();
    outExp = expandBuiltinGeneric2(arg.clone(), r#fn.clone(), ty.clone(), var.clone(), pur.clone(), attr.clone());
    Ok((outExp, expanded))
}

pub fn expandBuiltinGeneric2(mut exp: Arc<Expression::NFExpression>, mut r#fn: Arc<Function::Function>, mut ty: Arc<Type::NFType>, mut var: Variability, mut pur: Purity, mut attr: Arc<NFCallAttributes::NFCallAttributes>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { literal: true, .. } => {
            exp.clone()
        },
        Deref @ Expression::ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            arr = Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = r#fn.clone(); let __pe_b2 = ty.clone(); let __pe_b3 = var.clone(); let __pe_b4 = pur.clone(); let __pe_b5 = attr.clone(); move |__pe_a0| Ok(expandBuiltinGeneric2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone())) }));
            Expression::makeArray(Type::setArrayElementType(var_field!((*exp).ty, Expression::NFExpression::ARRAY).clone(), ty.clone()), arr.clone(), false)
        },
        _ => {
            Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_CALL { r#fn: r#fn.clone(), ty: ty.clone(), var: var.clone(), purity: pur.clone(), arguments: list![exp.clone()], attributes: attr.clone() }) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn expandArrayConstructor(mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = true;
    let mut e: Arc<Expression::NFExpression> = exp.clone();
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut iter: Mutable::Mutable<Arc<Expression::NFExpression>>;
    let mut iters: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>> = metamodelica::nil();
    for mut i in &*iterators.clone() {
        let mut i = i.clone();
        (node, range) = i.clone();
        iter = Mutable::create(Arc::new(Expression::NFExpression::EMPTY { ty: InstNode::getType(node.clone())? }));
        e = Expression::replaceIterator(e.clone(), node.clone(), Arc::new(Expression::NFExpression::MUTABLE { exp: iter.clone() }))?;
        iters = cons(iter.clone(), iters.clone());
        let __pa0 = ::match_deref::match_deref! { match &(expand(range.clone(), false, false)?) {
            (__pa0, true) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa0.clone();
        ranges = cons(range.clone(), ranges.clone());
    }
    result = expandArrayConstructor2(e.clone(), ty.clone(), ranges.clone(), iters.clone())?;
    Ok((result, expanded))
}

pub fn expandArrayConstructor2(mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut iterators: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ranges_rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut iter: Mutable::Mutable<Arc<Expression::NFExpression>>;
    let mut iters_rest: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>> = metamodelica::nil();
    let mut range_iter: Arc<ExpressionIterator::NFExpressionIterator> = Arc::new(ExpressionIterator::NONE_ITERATOR);
    let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut el_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if ranges.clone().is_empty() {
        (result, _) = expand(SimplifyExp::simplify(exp.clone(), false)?, false, false)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ranges.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa0.clone();
        ranges_rest = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(iterators.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        iter = __pa2.clone();
        iters_rest = __pa3.clone();
        range_iter = ExpressionIterator::fromExp(range.clone(), false, false)?;
        el_ty = Type::unliftArray(ty.clone())?;
        while ExpressionIterator::hasNext(range_iter.clone())? {
            (range_iter, value) = ExpressionIterator::next(range_iter.clone())?;
            Mutable::update(iter.clone(), value.clone());
            expl = cons(expandArrayConstructor2(exp.clone(), el_ty.clone(), ranges_rest.clone(), iters_rest.clone())?, expl.clone());
        }
        result = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(expl.clone().reverse().into_iter().cloned().collect()), false);
    }
    Ok(result)
}

pub fn expandSize(mut exp: Arc<Expression::NFExpression>) -> (Arc<Expression::NFExpression>, bool) {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = true;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SIZE { dimIndex: None, exp: e } => {
            let mut dims: i32 = 0;
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            ty = Expression::typeOf(e.clone());
            dims = Type::dimensionCount(ty.clone());
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (1..=dims.clone()).into_iter() {
            let __x = Arc::new(Expression::NFExpression::SIZE { exp: e.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() })) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![Dimension::fromInteger(dims.clone(), Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec(expl.clone().into_iter().cloned().collect()), false)
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, expanded)
}

pub fn expandBinary(mut exp: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    use crate::NFOperator::Op;
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    (outExp, expanded) = (match op.op.clone() {
        Operator::Op::ADD_SCALAR_ARRAY => expandBinaryScalarArray(exp.clone(), Op::ADD.clone())?,
        Operator::Op::ADD_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::ADD.clone())?,
        Operator::Op::SUB_SCALAR_ARRAY { .. } => expandBinaryScalarArray(exp.clone(), Op::SUB.clone())?,
        Operator::Op::SUB_ARRAY_SCALAR => expandBinaryArrayScalar(exp.clone(), Op::SUB.clone())?,
        Operator::Op::MUL_SCALAR_ARRAY => expandBinaryScalarArray(exp.clone(), Op::MUL.clone())?,
        Operator::Op::MUL_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::MUL.clone())?,
        Operator::Op::MUL_VECTOR_MATRIX => expandBinaryVectorMatrix(exp.clone())?,
        Operator::Op::MUL_MATRIX_VECTOR => expandBinaryMatrixVector(exp.clone())?,
        Operator::Op::SCALAR_PRODUCT => expandBinaryDotProduct(exp.clone())?,
        Operator::Op::MATRIX_PRODUCT => expandBinaryMatrixProduct(exp.clone())?,
        Operator::Op::DIV_SCALAR_ARRAY { .. } => expandBinaryScalarArray(exp.clone(), Op::DIV.clone())?,
        Operator::Op::DIV_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::DIV.clone())?,
        Operator::Op::POW_SCALAR_ARRAY { .. } => expandBinaryScalarArray(exp.clone(), Op::POW.clone())?,
        Operator::Op::POW_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::POW.clone())?,
        Operator::Op::POW_MATRIX => expandBinaryPowMatrix(exp.clone(), resize.clone())?,
        _ => expandBinaryElementWise(exp.clone())?,
    });
    if !(expanded.clone()) {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn expandBinaryElementWise(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    op = __pa1.clone();
    exp1 = __pa2.clone();
    if Type::isArray(Operator::typeOf(op.clone())) {
        (exp1, expanded) = expand(exp1.clone(), false, false)?;
        if expanded.clone() {
            (exp2, expanded) = expand(exp2.clone(), false, false)?;
        }
        if expanded.clone() {
            outExp = expandBinaryElementWise2(exp1.clone(), Operator::stripEW(op.clone()), exp2.clone(), (std::sync::Arc::new(SimplifyExp::simplifyBinaryOp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Operator::NFOperator>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        } else {
            outExp = exp.clone();
        }
    } else {
        outExp = exp.clone();
        expanded = true;
    }
    Ok((outExp, expanded))
}

pub fn expandBinaryElementWise2(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Operator::NFOperator>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> {
    pub type MakeFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Operator::NFOperator>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl1: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut expl2: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut eop: Arc<Operator::NFOperator>;
    expl1 = Expression::arrayElements(exp1.clone())?;
    expl2 = Expression::arrayElements(exp2.clone())?;
    ty = Operator::typeOf(op.clone());
    eop = Operator::setType(Type::unliftArray(ty.clone())?, op.clone());
    if Type::dimensionCount(ty.clone()) > 1 {
        expl = Array::threadMap(expl1.clone(), expl2.clone(), Arc::new({ let __pe_b1 = eop.clone(); let __pe_b3: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Operator::NFOperator>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0, __pe_a2| expandBinaryElementWise2(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_b3.clone()) }))?;
    } else {
        expl = Array::threadMap(expl1.clone(), expl2.clone(), Arc::new({ let __pe_b1 = eop.clone(); move |__pe_a0, __pe_a2| func(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
    }
    exp = Expression::makeArray(ty.clone(), expl.clone(), false);
    Ok(exp)
}

pub fn expandBinaryScalarArray(mut exp: Arc<Expression::NFExpression>, mut scalarOp: Operator::Op) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    op = __pa1.clone();
    exp1 = __pa2.clone();
    (exp2, expanded) = expand(exp2.clone(), false, false)?;
    if expanded.clone() {
        op = Arc::new(Operator::NFOperator { ty: Type::arrayElementType(Operator::typeOf(op.clone())), op: scalarOp.clone() });
        outExp = Expression::mapArrayElements(exp2.clone(), Arc::new({ let __pe_b0 = exp1.clone(); let __pe_b1 = op.clone(); move |__pe_a2| SimplifyExp::simplifyBinaryOp(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }));
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn makeScalarArrayBinary_traverser(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(exp2.clone()) {
        Deref @ Expression::ARRAY { .. } => exp2.clone(),
        _ => SimplifyExp::simplifyBinaryOp(exp1.clone(), op.clone(), exp2.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn expandBinaryArrayScalar(mut exp: Arc<Expression::NFExpression>, mut scalarOp: Operator::Op) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    op = __pa1.clone();
    exp1 = __pa2.clone();
    (exp1, expanded) = expand(exp1.clone(), false, false)?;
    if expanded.clone() {
        op = Arc::new(Operator::NFOperator { ty: Type::arrayElementType(Operator::typeOf(op.clone())), op: scalarOp.clone() });
        outExp = Expression::mapArrayElements(exp1.clone(), Arc::new({ let __pe_b1 = op.clone(); let __pe_b2 = exp2.clone(); move |__pe_a0| SimplifyExp::simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn expandBinaryVectorMatrix(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut m: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, exp1: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    exp1 = __pa1.clone();
    (exp2, expanded) = expand(exp2.clone(), false, false)?;
    if expanded.clone() {
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Expression::transposeArray(exp2.clone())?) {
            Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __pa2, dimensions: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: __pa4, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa2.clone();
        m = __pa3.clone();
        arr = __pa4.clone();
        ty = Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![m.clone()] });
        if arr.clone().borrow().is_empty() {
            outExp = Expression::makeZero(ty.clone())?;
        } else {
            (exp1, expanded) = expand(exp1.clone(), false, false)?;
            if expanded.clone() {
                arr = Array::map(arr.clone(), Arc::new({ let __pe_b0 = exp1.clone(); move |__pe_a1| makeScalarProduct(__pe_b0.clone(), __pe_a1) }));
                outExp = Expression::makeArray(ty.clone(), arr.clone(), false);
            } else {
                outExp = exp.clone();
            }
        }
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn expandBinaryMatrixVector(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut n: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, exp1: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    exp1 = __pa1.clone();
    (exp1, expanded) = expand(exp1.clone(), false, false)?;
    if expanded.clone() {
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(exp1.clone()) {
            Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __pa2, dimensions: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: __pa4, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa2.clone();
        n = __pa3.clone();
        arr = __pa4.clone();
        ty = Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![n.clone()] });
        if arr.clone().borrow().is_empty() {
            outExp = Expression::makeZero(ty.clone())?;
        } else {
            (exp2, expanded) = expand(exp2.clone(), false, false)?;
            if expanded.clone() {
                arr = Array::map(arr.clone(), Arc::new({ let __pe_b1 = exp2.clone(); move |__pe_a0| makeScalarProduct(__pe_a0, __pe_b1.clone()) }));
                outExp = Expression::makeArray(ty.clone(), arr.clone(), false);
            } else {
                outExp = exp.clone();
            }
        }
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn expandBinaryDotProduct(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, exp1: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    exp1 = __pa1.clone();
    (exp1, expanded) = expand(exp1.clone(), false, false)?;
    if expanded.clone() {
        (exp2, expanded) = expand(exp2.clone(), false, false)?;
    }
    if expanded.clone() {
        outExp = makeScalarProduct(exp1.clone(), exp2.clone())?;
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn makeScalarProduct(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arr1: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr2: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mul_op: Arc<Operator::NFOperator>;
    let mut add_op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::ARRAY { ty: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    arr1 = __pa1.clone();
    let __pa2 = ::match_deref::match_deref! { match &(exp2.clone()) {
        Deref @ Expression::ARRAY { ty: _, elements: __pa2, .. } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    arr2 = __pa2.clone();
    elem_ty = Type::unliftArray(ty.clone())?;
    if arr1.clone().borrow().is_empty() {
        exp = Expression::makeZero(elem_ty.clone())?;
    } else {
        mul_op = Operator::makeMul(elem_ty.clone());
        add_op = Operator::makeAdd(elem_ty.clone());
        arr1 = Array::threadMap(arr1.clone(), arr2.clone(), Arc::new({ let __pe_b1 = mul_op.clone(); move |__pe_a0, __pe_a2| SimplifyExp::simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
        exp = Array::reduce(arr1.clone(), Arc::new({ let __pe_b1 = add_op.clone(); move |__pe_a0, __pe_a2| SimplifyExp::simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
    }
    Ok(exp)
}

pub fn expandBinaryMatrixProduct(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, exp1: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    exp1 = __pa1.clone();
    (exp1, expanded) = expand(exp1.clone(), false, false)?;
    if expanded.clone() {
        (exp2, expanded) = expand(exp2.clone(), false, false)?;
    }
    if expanded.clone() {
        outExp = makeBinaryMatrixProduct(exp1.clone(), exp2.clone())?;
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn makeBinaryMatrixProduct(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arr1: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr2: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mat_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut n: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut p: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut len: i32 = 0;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __pa0, dimensions: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    n = __pa1.clone();
    arr1 = __pa2.clone();
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(Expression::transposeArray(exp2.clone())?) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, elements: __pa5, .. } => (__pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    p = __pa4.clone();
    arr2 = __pa5.clone();
    mat_ty = Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![n.clone(), p.clone()] });
    if arr2.clone().borrow().is_empty() {
        exp = Expression::makeZero(mat_ty.clone())?;
    } else {
        row_ty = Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![p.clone()] });
        len = (arr1.clone().borrow().len() as i32);
        arr = metamodelica::arrayCreate(len.clone(), exp1.clone());
        for mut i in 1..=len.clone() {
            e = arr1.clone().borrow()[(i.clone()-1) as usize].clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), Expression::makeArray(row_ty.clone(), makeBinaryMatrixProduct2(e.clone(), arr2.clone()), false)) };
        }
        exp = Expression::makeArray(mat_ty.clone(), arr.clone(), false);
    }
    Ok(exp)
}

pub fn makeBinaryMatrixProduct2(mut row: Arc<Expression::NFExpression>, mut matrix: metamodelica::Array<Arc<Expression::NFExpression>>) -> metamodelica::Array<Arc<Expression::NFExpression>> {
    let mut outRow: metamodelica::Array<Arc<Expression::NFExpression>>;
    outRow = Array::map(matrix.clone(), Arc::new({ let __pe_b0 = row.clone(); move |__pe_a1| makeScalarProduct(__pe_b0.clone(), __pe_a1) }));
    outRow
}

pub fn expandBinaryPowMatrix(mut exp: Arc<Expression::NFExpression>, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let mut n: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    op = __pa1.clone();
    exp1 = __pa2.clone();
    (outExp, expanded) = (::match_deref::match_deref! { match &(exp2.clone()) {
        Deref @ Expression::INTEGER { value: 0 } => {
            n = Dimension::size(listHead(Type::arrayDims(Operator::typeOf(op.clone())))?, false)?;
            (Expression::makeIdentityMatrix(n.clone(), Arc::new(crate::NFType::REAL))?, true)
        },
        Deref @ Expression::INTEGER { value: n } if (n.clone() > 0) => {
            (exp1, expanded) = expand(exp1.clone(), false, false)?;
            if expanded.clone() {
                outExp = expandBinaryPowMatrix2(exp1.clone(), n.clone())?;
            }
            (outExp.clone(), expanded.clone())
        },
        _ => expandGeneric(exp.clone(), resize.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, expanded))
}

pub fn expandBinaryPowMatrix2(mut matrix: Arc<Expression::NFExpression>, mut n: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (match n.clone() {
        1 => matrix.clone(),
        2 => makeBinaryMatrixProduct(matrix.clone(), matrix.clone())?,
        _ if (intMod(n.clone(), 2) == 0) => {
            exp = expandBinaryPowMatrix2(matrix.clone(), intDiv(n.clone(), 2))?;
            makeBinaryMatrixProduct(exp.clone(), exp.clone())?
        },
        _ => {
            exp = expandBinaryPowMatrix2(matrix.clone(), n.clone() - 1)?;
            makeBinaryMatrixProduct(matrix.clone(), exp.clone())?
        },
    });
    Ok(exp)
}

pub fn expandUnary(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut operand: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let mut scalar_op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::UNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    op = __pa0.clone();
    operand = __pa1.clone();
    (operand, expanded) = expand(operand.clone(), false, false)?;
    if expanded.clone() {
        scalar_op = Operator::scalarize(op.clone());
        outExp = Expression::mapArrayElements(operand.clone(), Arc::new({ let __pe_b1 = scalar_op.clone(); move |__pe_a0| SimplifyExp::simplifyUnaryOp(__pe_a0, __pe_b1.clone()) }));
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn expandLogicalBinary(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LBINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp2 = __pa0.clone();
    op = __pa1.clone();
    exp1 = __pa2.clone();
    if Type::isArray(Operator::typeOf(op.clone())) {
        (exp1, expanded) = expand(exp1.clone(), false, false)?;
        if expanded.clone() {
            (exp2, expanded) = expand(exp2.clone(), false, false)?;
        }
        if expanded.clone() {
            outExp = expandBinaryElementWise2(exp1.clone(), op.clone(), exp2.clone(), (std::sync::Arc::new(makeLBinaryOp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Operator::NFOperator>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        } else {
            outExp = exp.clone();
        }
    } else {
        outExp = exp.clone();
        expanded = true;
    }
    Ok((outExp, expanded))
}

pub fn makeLBinaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Expression::isScalarLiteral(exp1.clone()) && Expression::isScalarLiteral(exp2.clone()) {
        exp = Ceval::evalLogicBinaryOp(exp1.clone(), op.clone(), exp2.clone(), Ceval::noTarget().clone())?;
    } else {
        exp = Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
    }
    Ok(exp)
}

pub fn expandLogicalUnary(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut operand: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let mut scalar_op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LUNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    op = __pa0.clone();
    operand = __pa1.clone();
    (operand, expanded) = expand(operand.clone(), false, false)?;
    if expanded.clone() {
        scalar_op = Operator::scalarize(op.clone());
        outExp = Expression::mapArrayElements(operand.clone(), Arc::new({ let __pe_b1 = scalar_op.clone(); move |__pe_a0| Ok(makeLogicalUnaryOp(__pe_a0, __pe_b1.clone())) }));
    } else {
        outExp = exp.clone();
    }
    Ok((outExp, expanded))
}

pub fn makeLogicalUnaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::LUNARY { operator: op.clone(), exp: exp1.clone() });
    exp
}

pub fn expandCast(mut castExp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(castExp.clone()) {
        Deref @ Expression::CAST { ty: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    exp = __pa1.clone();
    (outExp, expanded) = expand(exp.clone(), false, false)?;
    if expanded.clone() && !(referenceEq(&exp.clone(),&outExp.clone())) {
        outExp = Expression::typeCast(outExp.clone(), ty.clone())?;
    } else {
        outExp = castExp.clone();
    }
    Ok((outExp, expanded))
}

pub fn expandGeneric(mut exp: Arc<Expression::NFExpression>, mut resize: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    ty = Expression::typeOf(exp.clone());
    if Type::isArray(ty.clone()) {
        expanded = Type::hasKnownSize(ty.clone());
        if expanded.clone() {
            dims = Type::arrayDims(ty.clone());
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut e in (RangeIterator::toList(RangeIterator::fromDim(d.clone(), resize.clone())?)).into_iter().cloned() {
            let __x = Arc::new(Subscript::NFSubscript::INDEX { index: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            outExp = expandGeneric2(subs.clone(), exp.clone(), ty.clone(), metamodelica::nil())?;
        } else {
            outExp = exp.clone();
        }
    } else {
        outExp = exp.clone();
        expanded = true;
    }
    Ok((outExp, expanded))
}

pub fn expandGeneric2(mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>, mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut accum: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut sub: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut rest_subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    let mut i: i32 = 0;
    outExp = (::match_deref::match_deref! { match &(subs.clone()) {
        Deref @ metamodelica::List::Cons { head: sub, tail: rest_subs } => {
            t = Type::unliftArray(ty.clone())?;
            expl = metamodelica::arrayCreate((sub.clone().len() as i32), exp.clone());
            i = 1;
            for mut s in &*sub.clone() {
                let mut s = s.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(expl.clone(), i.clone(), expandGeneric2(rest_subs.clone(), exp.clone(), t.clone(), cons(s.clone(), accum.clone()))?) };
                i = i.clone() + 1;
            }
            Expression::makeArray(ty.clone(), expl.clone(), false)
        },
        Deref @ metamodelica::List::Nil => {
            outExp = exp.clone();
            for mut s in &*accum.clone().reverse() {
                let mut s = s.clone();
                outExp = Expression::applySubscript(s.clone(), outExp.clone(), metamodelica::nil(), false)?;
            }
            outExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn expandCallArgs(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut call: Arc<Call::NFCall>;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = (expand(arg.clone(), false, false)?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}


