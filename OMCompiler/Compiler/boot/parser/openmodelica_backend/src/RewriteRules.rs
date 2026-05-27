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

use crate::GlobalScriptDump;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_frontend::Expression;
use openmodelica_frontend::Parser;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

/// rule to rewrite fromExp -> toExp,
///  there are FrontEnd and BackEnd rules
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rule {
    /// rule to rewrite fromExp -> toExp, apply to FrontEnd AST exps
    FRONTEND_RULE {
        from: Arc<Absyn::Exp>,
        to: Arc<Absyn::Exp>,
    },
    /// rule to rewrite fromExp -> toExp, apply to the BackEnd AST exps
    BACKEND_RULE {
        from: Arc<Absyn::Exp>,
        to: Arc<Absyn::Exp>,
    },
}
pub use self::Rule::{FRONTEND_RULE,BACKEND_RULE};

pub type Rules = Arc<metamodelica::List<Rule>>;

/// a bind '$1' bound to an exp
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bind {
    /// a bind '$1' bound to an exp (frontend)
    FRONTEND_BIND {
        slot: Arc<Absyn::Exp>,
        value: Arc<Absyn::Exp>,
    },
    /// a bind '$1' bound to an exp (backend)
    BACKEND_BIND {
        slot: Arc<DAE::Exp>,
        value: Arc<DAE::Exp>,
    },
}
pub use self::Bind::{FRONTEND_BIND,BACKEND_BIND};

pub type Binds = Arc<metamodelica::List<Bind>>;

// frontend rewrite stuff
// ----------------------
pub fn rewriteFrontEnd(mut inExp: Arc<Absyn::Exp>) -> Result<(Arc<Absyn::Exp>, bool)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut isChanged: bool = false;
    (outExp, isChanged) = (::match_deref::match_deref! { match &(inExp.clone()) {
        _ => {
            let mut rules: Rules = metamodelica::nil();
            let mut b: bool = false;
            rules = getRulesFrontEnd(getAllRules()?);
            (outExp, b) = matchAndRewriteExpFrontEnd(inExp.clone(), rules.clone())?;
            (outExp.clone(), b.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, isChanged))
}

pub fn matchAndRewriteExpFrontEnd(mut inExp: Arc<Absyn::Exp>, mut inRules: Rules) -> Result<(Arc<Absyn::Exp>, bool)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut changed: bool = false;
    (outExp, changed) = 'mc: {
        let __mc_input = (inExp.clone(), inRules.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok((inExp.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Rule::FRONTEND_RULE { from, to }, tail: _ }) => {
                    let mut binds: Binds = metamodelica::nil();
                    let mut b: bool = false;
                    let mut outExp: Arc<Absyn::Exp> = outExp.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(matchesFrontEnd(inExp.clone(), from.clone(), metamodelica::nil())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    binds = __pa0.clone();
                    outExp = rewriteExpFrontEnd(to.clone(), binds.clone())?;
                    b = boolNot(referenceEq(&inExp.clone(),&outExp.clone()));
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FrontEnd Exp:     ")); __mm_s.push_str(&*Dump::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("FrontEnd From:    ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("FrontEnd To:      ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("FrontEnd Rewrite: ")); __mm_s.push_str(&*Dump::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!("\n---------\n")); ArcStr::from(__mm_s) }).clone());
                    Ok((outExp.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut b: bool = false;
                    let mut outExp: Arc<Absyn::Exp> = outExp.clone();
                    (outExp, b) = matchAndRewriteExpFrontEnd(inExp.clone(), rest.clone())?;
                    Ok((outExp.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, changed))
}

pub fn rewriteExpFrontEnd(mut inExp: Arc<Absyn::Exp>, mut inBinds: Binds) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (outExp, _) = AbsynUtil::traverseExp(inExp.clone(), (std::sync::Arc::new(replaceBindsFrontEnd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<Bind>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Bind>>)> + 'static>), inBinds.clone())?;
    Ok(outExp)
}

pub fn replaceBindsFrontEnd(mut inExp: Arc<Absyn::Exp>, mut inBinds: Binds) -> Result<(Arc<Absyn::Exp>, Binds)> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outBinds: Binds = metamodelica::nil();
    (outExp, outBinds) = (::match_deref::match_deref! { match &((inExp.clone(), inBinds.clone())) {
        (e1 @ Deref @ Absyn::Exp::CREF { componentRef: _ }, bnds) => {
            let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            e2 = replaceBindFrontEnd(e1.clone(), bnds.clone())?;
            (e2.clone(), bnds.clone())
        },
        _ => {
            (inExp.clone(), inBinds.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outBinds))
}

pub fn replaceBindFrontEnd(mut inExp: Arc<Absyn::Exp>, mut inBinds: Binds) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut found: bool = false;
    let mut e: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut to: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    for mut bind in &*inBinds.clone() {
        let mut bind = bind.clone();
        let Bind::FRONTEND_BIND { slot: __pa0, value: __pa1 } = (bind.clone()) else { bail!("pattern mismatch") };
        e = __pa0.clone();
        outExp = __pa1.clone();
        if AbsynUtil::expEqual(inExp.clone(), e.clone())? {
            return Ok(outExp);
        }
    }
    outExp = inExp.clone();
    Ok(outExp)
}

pub fn matchesFrontEnd(mut inExp: Arc<Absyn::Exp>, mut inUnifyWith: Arc<Absyn::Exp>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = 'mc: {
        let __mc_input = (inExp.clone(), inUnifyWith.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Exp::CREF { componentRef: _ }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (isPlaceHolderFrontEnd(inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    outBinds = cons(Bind::FRONTEND_BIND { slot: inUnifyWith.clone(), value: inExp.clone() }, inAcc.clone());
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { value: _ }, _, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::REAL { value: _ }, _, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::STRING { value: _ }, _, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BOOL { value: _ }, _, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: _ }, _, _) => {
                    let true = (AbsynUtil::expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BINARY { exp1: e1a, op: op1a, exp2: e2a }, Deref @ Absyn::Exp::BINARY { exp1: e1b, op: op1b, exp2: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesFrontEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: op1a, exp: e1a }, Deref @ Absyn::Exp::UNARY { op: op1b, exp: e1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LBINARY { exp1: e1a, op: op1a, exp2: e2a }, Deref @ Absyn::Exp::LBINARY { exp1: e1b, op: op1b, exp2: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesFrontEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LUNARY { op: op1a, exp: e1a }, Deref @ Absyn::Exp::LUNARY { op: op1b, exp: e1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RELATION { exp1: e1a, op: op1a, exp2: e2a }, Deref @ Absyn::Exp::RELATION { exp1: e1b, op: op1b, exp2: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::opEqual(op1a.clone(), op1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesFrontEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::IFEXP { ifExp: cond1a, trueBranch: e1a, elseBranch: e2a, elseIfBranch: _ }, Deref @ Absyn::Exp::IFEXP { ifExp: cond1b, trueBranch: e1b, elseBranch: e2b, elseIfBranch: _ }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesFrontEnd(cond1a.clone(), cond1b.clone(), inAcc.clone())?;
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), outBinds.clone())?;
                    outBinds = matchesFrontEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CALL { function_: cr1a, functionArgs: fargs1a, .. }, Deref @ Absyn::Exp::CALL { function_: cr1b, functionArgs: fargs1b, .. }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::crefEqual(cr1a.clone(), cr1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesFargsFrontEnd(fargs1a.clone(), fargs1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cr1a, functionArgs: fargs1a }, Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cr1b, functionArgs: fargs1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::crefEqual(cr1a.clone(), cr1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesFargsFrontEnd(fargs1a.clone(), fargs1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::ARRAY { arrayExp: exps1a }, Deref @ Absyn::Exp::ARRAY { arrayExp: exps1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstFrontEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::MATRIX { matrix: expsLst1a }, Deref @ Absyn::Exp::MATRIX { matrix: expsLst1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstLstFrontEnd(expsLst1a.clone(), expsLst1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RANGE { start: e1a, step: oe1a, stop: e2a }, Deref @ Absyn::Exp::RANGE { start: e1b, step: oe1b, stop: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesExpOptFrontEnd(oe1a.clone(), oe1b.clone(), outBinds.clone())?;
                    outBinds = matchesFrontEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::TUPLE { expressions: exps1a }, Deref @ Absyn::Exp::TUPLE { expressions: exps1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstFrontEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::END, Deref @ Absyn::Exp::END, _) => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code: _ }, Deref @ Absyn::Exp::CODE { code: _ }, _) => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::AS { id: id1a, exp: e1a }, Deref @ Absyn::Exp::AS { id: id1b, exp: e1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (stringEq((id1a.clone()).clone(), (id1b.clone()).clone())?) else { bail!("pattern mismatch") };
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CONS { head: e1a, rest: e2a }, Deref @ Absyn::Exp::CONS { head: e1b, rest: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesFrontEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::MATCHEXP { .. }, Deref @ Absyn::Exp::MATCHEXP { .. }, _) => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LIST { exps: exps1a }, Deref @ Absyn::Exp::LIST { exps: exps1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstFrontEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBinds)
}

pub fn matchesExpOptFrontEnd(mut inOExp1: Option<Arc<Absyn::Exp>>, mut inOExp2: Option<Arc<Absyn::Exp>>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inOExp1.clone(), inOExp2.clone(), inAcc.clone())) {
        (None, None, _) => {
            inAcc.clone()
        },
        (Some(e1a), Some(e1b), _) => {
            outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinds)
}

pub fn matchesExpLstFrontEnd(mut inExps1: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inExps2: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inExps1.clone(), inExps2.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::List::Cons { head: e1b, tail: exps1b }, _) => {
            outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds = matchesExpLstFrontEnd(exps1a.clone(), exps1b.clone(), outBinds.clone())?;
            outBinds.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub fn matchesFargsFrontEnd(mut inFargs1: Arc<Absyn::FunctionArgs>, mut inFargs2: Arc<Absyn::FunctionArgs>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inFargs1.clone(), inFargs2.clone(), inAcc.clone())) {
        (Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: exps1a, argNames: nargs1a }, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: exps1b, argNames: nargs1b }, _) => {
            outBinds = matchesExpLstFrontEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
            let true = (intEq((nargs1a.clone().len() as i32), (nargs1b.clone().len() as i32))) else { bail!("pattern mismatch") };
            outBinds = matchesNargsFrontEnd(sortNargsFrontEnd(nargs1a.clone())?, sortNargsFrontEnd(nargs1b.clone())?, outBinds.clone())?;
            outBinds.clone()
        },
        (Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp: e1a, iterType: _, iterators: _ }, Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp: e1b, iterType: _, iterators: _ }, _) => {
            outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub fn sortNargsFrontEnd(mut inNargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::NamedArg>>>> {
    let mut outNargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    outNargs = List::sort(inNargs.clone(), (std::sync::Arc::new(inNargComp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::NamedArg>, Arc<Absyn::NamedArg>) -> Result<bool> + 'static>))?;
    Ok(outNargs)
}

pub fn inNargComp(mut inNarg1: Arc<Absyn::NamedArg>, mut inNarg2: Arc<Absyn::NamedArg>) -> Result<bool> {
    let mut isGreater: bool = false;
    let mut id1: ArcStr = arcstr::literal!("");
    let mut id2: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inNarg1.clone()) {
        Deref @ Absyn::NamedArg { argName: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    id1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(inNarg2.clone()) {
        Deref @ Absyn::NamedArg { argName: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    id2 = __pa1.clone();
    isGreater = intGt(stringCompare((id1.clone()).clone(), (id2.clone()).clone()), 0);
    Ok(isGreater)
}

pub fn matchesNargsFrontEnd(mut inNargs1: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inNargs2: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inNargs1.clone(), inNargs2.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: n1a, argValue: e1a }, tail: nargs1a }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: n1b, argValue: e1b }, tail: nargs1b }, _) => {
            let true = (stringEq((n1a.clone()).clone(), (n1b.clone()).clone())?) else { bail!("pattern mismatch") };
            outBinds = matchesFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds = matchesNargsFrontEnd(nargs1a.clone(), nargs1b.clone(), outBinds.clone())?;
            outBinds.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub fn matchesExpLstLstFrontEnd(mut inExps1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inExps2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inExps1.clone(), inExps2.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::List::Cons { head: e1b, tail: exps1b }, _) => {
            outBinds = matchesExpLstFrontEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds = matchesExpLstLstFrontEnd(exps1a.clone(), exps1b.clone(), outBinds.clone())?;
            outBinds.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub fn isPlaceHolderFrontEnd(mut inExp: Arc<Absyn::Exp>) -> Result<bool> {
    let mut isHolder: bool = false;
    isHolder = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: _ } } => {
            let mut b: bool = false;
            b = intEq(System::stringFind((name.clone()).clone(), (literal!("'$")).clone())?, 0);
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isHolder)
}

// backend rewrite stuff
// ----------------------
pub fn rewriteBackEnd(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut isChanged: bool = false;
    (outExp, isChanged) = (::match_deref::match_deref! { match &(inExp.clone()) {
        _ => {
            let mut rules: Rules = metamodelica::nil();
            let mut b: bool = false;
            rules = getRulesBackEnd(getAllRules()?);
            (outExp, b) = matchAndRewriteExpBackEnd(inExp.clone(), rules.clone())?;
            (outExp.clone(), b.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, isChanged))
}

pub fn matchAndRewriteExpBackEnd(mut inExp: Arc<DAE::Exp>, mut inRules: Rules) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut changed: bool = false;
    (outExp, changed) = 'mc: {
        let __mc_input = (inExp.clone(), inRules.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok((inExp.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Rule::BACKEND_RULE { from: afrom, to: ato }, tail: _ }) => {
                    let mut from: Arc<DAE::Exp>;
                    let mut to: Arc<DAE::Exp>;
                    let mut binds: Binds = metamodelica::nil();
                    let mut b: bool = false;
                    let mut outExp: Arc<DAE::Exp>;
                    from = Expression::fromAbsynExp(afrom.clone())?;
                    to = Expression::fromAbsynExp(ato.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(matchesBackEnd(inExp.clone(), from.clone(), metamodelica::nil())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    binds = __pa0.clone();
                    outExp = rewriteExpBackEnd(to.clone(), binds.clone())?;
                    b = boolNot(referenceEq(&inExp.clone(),&outExp.clone()));
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackEnd Exp:     ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("BackEnd From:    ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(from.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("BackEnd To:      ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("BackEnd Rewrite: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!("\n---------\n")); ArcStr::from(__mm_s) }).clone());
                    Ok((outExp.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut b: bool = false;
                    let mut outExp: Arc<DAE::Exp>;
                    (outExp, b) = matchAndRewriteExpBackEnd(inExp.clone(), rest.clone())?;
                    Ok((outExp.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, changed))
}

pub fn rewriteExpBackEnd(mut inExp: Arc<DAE::Exp>, mut inBinds: Binds) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(replaceBindsBackEnd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Bind>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Bind>>)> + 'static>), inBinds.clone())?;
    Ok(outExp)
}

pub fn replaceBindsBackEnd(mut inExp: Arc<DAE::Exp>, mut inBinds: Binds) -> Result<(Arc<DAE::Exp>, Binds)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outBinds: Binds = metamodelica::nil();
    (outExp, outBinds) = (::match_deref::match_deref! { match &((inExp.clone(), inBinds.clone())) {
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, bnds) => {
            let mut e2: Arc<DAE::Exp>;
            e2 = replaceBindBackEnd(e1.clone(), bnds.clone())?;
            (e2.clone(), bnds.clone())
        },
        _ => {
            (inExp.clone(), inBinds.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outBinds))
}

pub fn replaceBindBackEnd(mut inExp: Arc<DAE::Exp>, mut inBinds: Binds) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut found: bool = false;
    let mut e: Arc<DAE::Exp>;
    let mut to: Arc<DAE::Exp>;
    for mut bind in &*inBinds.clone() {
        let mut bind = bind.clone();
        let Bind::BACKEND_BIND { slot: __pa0, value: __pa1 } = (bind.clone()) else { bail!("pattern mismatch") };
        e = __pa0.clone();
        to = __pa1.clone();
        if expEqual(inExp.clone(), e.clone())? {
            outExp = to.clone();
            return Ok(outExp);
        }
    }
    outExp = inExp.clone();
    Ok(outExp)
}

pub fn matchesBackEnd(mut inExp: Arc<DAE::Exp>, mut inUnifyWith: Arc<DAE::Exp>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = 'mc: {
        let __mc_input = (inExp.clone(), inUnifyWith.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (isPlaceHolderBackEnd(inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    outBinds = cons(Bind::BACKEND_BIND { slot: inUnifyWith.clone(), value: inExp.clone() }, inAcc.clone());
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: _ }, _, _) => {
                    let true = (expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: _ }, _, _) => {
                    let true = (expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: _ }, _, _) => {
                    let true = (expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: _ }, _, _) => {
                    let true = (expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, _, _) => {
                    let true = (expEqual(inExp.clone(), inUnifyWith.clone())?) else { bail!("pattern mismatch") };
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1a, operator: op1a, exp2: e2a }, Deref @ DAE::Exp::BINARY { exp1: e1b, operator: op1b, exp2: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (operatorMatches(op1a.clone(), op1b.clone())?) else { bail!("pattern mismatch") };
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesBackEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: op1a, exp: e1a }, Deref @ DAE::Exp::UNARY { operator: op1b, exp: e1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (operatorMatches(op1a.clone(), op1b.clone())?) else { bail!("pattern mismatch") };
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1a, operator: op1a, exp2: e2a }, Deref @ DAE::Exp::LBINARY { exp1: e1b, operator: op1b, exp2: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (operatorMatches(op1a.clone(), op1b.clone())?) else { bail!("pattern mismatch") };
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesBackEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: op1a, exp: e1a }, Deref @ DAE::Exp::LUNARY { operator: op1b, exp: e1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (operatorMatches(op1a.clone(), op1b.clone())?) else { bail!("pattern mismatch") };
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1a, operator: op1a, exp2: e2a, index: _, optionExpisASUB: _ }, Deref @ DAE::Exp::RELATION { exp1: e1b, operator: op1b, exp2: e2b, index: _, optionExpisASUB: _ }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (operatorMatches(op1a.clone(), op1b.clone())?) else { bail!("pattern mismatch") };
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesBackEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: cond1a, expThen: e1a, expElse: e2a }, Deref @ DAE::Exp::IFEXP { expCond: cond1b, expThen: e1b, expElse: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesBackEnd(cond1a.clone(), cond1b.clone(), inAcc.clone())?;
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), outBinds.clone())?;
                    outBinds = matchesBackEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: p1a, expLst: exps1a, attr: _ }, Deref @ DAE::Exp::CALL { path: p1b, expLst: exps1b, attr: _ }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::pathEqual(p1a.clone(), p1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesExpLstBackEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { path: p1a, expList: exps1a, ty: _, origType: _ }, Deref @ DAE::Exp::PARTEVALFUNCTION { path: p1b, expList: exps1b, ty: _, origType: _ }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    let true = (AbsynUtil::pathEqual(p1a.clone(), p1b.clone())) else { bail!("pattern mismatch") };
                    outBinds = matchesExpLstBackEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: exps1a, .. }, Deref @ DAE::Exp::ARRAY { array: exps1b, .. }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstBackEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: expsLst1a, .. }, Deref @ DAE::Exp::MATRIX { matrix: expsLst1b, .. }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstLstBackEnd(expsLst1a.clone(), expsLst1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { ty: _, start: e1a, step: oe1a, stop: e2a }, Deref @ DAE::Exp::RANGE { ty: _, start: e1b, step: oe1b, stop: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesExpOptBackEnd(oe1a.clone(), oe1b.clone(), outBinds.clone())?;
                    outBinds = matchesBackEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: exps1a }, Deref @ DAE::Exp::TUPLE { PR: exps1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstBackEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CONS { car: e1a, cdr: e2a }, Deref @ DAE::Exp::CONS { car: e1b, cdr: e2b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
                    outBinds = matchesBackEnd(e2a.clone(), e2b.clone(), outBinds.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATCHEXPRESSION { .. }, Deref @ DAE::Exp::MATCHEXPRESSION { .. }, _) => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: exps1a }, Deref @ DAE::Exp::LIST { valList: exps1b }, _) => {
                    let mut outBinds: Arc<metamodelica::List<Bind>> = outBinds.clone();
                    outBinds = matchesExpLstBackEnd(exps1a.clone(), exps1b.clone(), inAcc.clone())?;
                    Ok(outBinds.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBinds)
}

pub fn matchesExpOptBackEnd(mut inOExp1: Option<Arc<DAE::Exp>>, mut inOExp2: Option<Arc<DAE::Exp>>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inOExp1.clone(), inOExp2.clone(), inAcc.clone())) {
        (None, None, _) => {
            inAcc.clone()
        },
        (Some(e1a), Some(e1b), _) => {
            outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinds)
}

pub fn matchesExpLstBackEnd(mut inExps1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExps2: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inExps1.clone(), inExps2.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::List::Cons { head: e1b, tail: exps1b }, _) => {
            outBinds = matchesBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds = matchesExpLstBackEnd(exps1a.clone(), exps1b.clone(), outBinds.clone())?;
            outBinds.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub fn matchesExpLstLstBackEnd(mut inExps1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExps2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inAcc: Binds) -> Result<Binds> {
    let mut outBinds: Binds = metamodelica::nil();
    outBinds = (::match_deref::match_deref! { match &((inExps1.clone(), inExps2.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1a, tail: exps1a }, Deref @ metamodelica::List::Cons { head: e1b, tail: exps1b }, _) => {
            outBinds = matchesExpLstBackEnd(e1a.clone(), e1b.clone(), inAcc.clone())?;
            outBinds = matchesExpLstLstBackEnd(exps1a.clone(), exps1b.clone(), outBinds.clone())?;
            outBinds.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinds)
}

pub fn isPlaceHolderBackEnd(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut isHolder: bool = false;
    isHolder = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, ty: _ } => {
            let mut b: bool = false;
            b = intEq(System::stringFind((name.clone()).clone(), (literal!("'$")).clone())?, 0);
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isHolder)
}

fn expEqual(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = 'mc: {
        let __mc_input = (e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: i }, Deref @ DAE::Exp::RCONST { real: r }) => {
                    let true = (realEq(intReal(i.clone()), r.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let true = (realEq(intReal(i.clone()), r.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ExpressionBasics::expEqual(e1.clone(), e2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isEqual)
}

fn operatorMatches(mut op1: DAE::Operator, mut op2: DAE::Operator) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = (op1.clone(), op2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::UMINUS_ARR { .. }, DAE::Operator::UMINUS { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::ADD_ARR { .. }, DAE::Operator::ADD { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::SUB_ARR { .. }, DAE::Operator::SUB { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::MUL_ARR { .. }, DAE::Operator::MUL { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::DIV_ARR { .. }, DAE::Operator::DIV { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::MUL_ARRAY_SCALAR { .. }, DAE::Operator::MUL { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::ADD_ARRAY_SCALAR { .. }, DAE::Operator::ADD { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::SUB_SCALAR_ARRAY { .. }, DAE::Operator::SUB { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, DAE::Operator::MUL { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, DAE::Operator::MUL { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::DIV_SCALAR_ARRAY { .. }, DAE::Operator::DIV { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::DIV_ARRAY_SCALAR { .. }, DAE::Operator::DIV { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::POW_SCALAR_ARRAY { .. }, DAE::Operator::POW { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::POW_ARRAY_SCALAR { .. }, DAE::Operator::POW { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::POW_ARR { .. }, DAE::Operator::POW { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::Operator::POW_ARR2 { .. }, DAE::Operator::POW { .. }) = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(Expression::operatorEqual(op1.clone(), op2.clone())?)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(b)
}

pub fn loadRules() -> Result<()> {
    let _ = (match () {
        () => {
            let mut file: ArcStr = arcstr::literal!("");
            file = (Flags::getConfigString(Flags::REWRITE_RULES_FILE.clone())?).clone();
            loadRulesFromFile((file.clone()).clone())?;
            ()
        },
    });
    Ok(())
}

pub fn noRewriteRules() -> Result<bool> {
    let mut noRules: bool = false;
    noRules = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let None = (crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(noRules)
}

pub fn noRewriteRulesFrontEnd() -> Result<bool> {
    let mut noRules: bool = false;
    noRules = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let None = (crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(getRulesFrontEnd(getAllRules()?)) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(noRules)
}

pub fn noRewriteRulesBackEnd() -> Result<bool> {
    let mut noRules: bool = false;
    noRules = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let None = (crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(getRulesBackEnd(getAllRules()?)) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(noRules)
}

pub fn loadRulesFromFile(mut inFile: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = None);
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut oR: Option<Arc<metamodelica::List<Rule>>> = None;
                    oR = crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone());
                    let true = (isSome(oR.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut stmts: Arc<metamodelica::List<GlobalScript::Statement>> = metamodelica::nil();
                    let mut rules: Rules = metamodelica::nil();
                    let None = (crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone())) else { bail!("pattern mismatch") };
                    let GlobalScript::ISTMTS { interactiveStmtLst: __pa0, semicolon: _ } = (Parser::parseexp((inFile.clone()).clone())?) else { bail!("pattern mismatch") };
                    stmts = __pa0.clone();
                    rules = stmtsToRules(stmts.clone(), metamodelica::nil())?;
                    println!("{}", (literal!("-------------\n")).clone());
                    crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = Some(rules.clone()));
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unable to parse rewrite rules file: ")); __mm_s.push_str(&*inFile.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = None);
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn clearRules() -> () {
    crate::Globals::rewriteRulesIndex.with(|__root| *__root.borrow_mut() = None);
    ()
}

pub fn getAllRules() -> Result<Rules> {
    let mut outRules: Rules = metamodelica::nil();
    let mut orules: Option<Arc<metamodelica::List<Rule>>> = None;
    orules = crate::Globals::rewriteRulesIndex.with(|__root| __root.borrow().clone());
    let __pa0 = ::match_deref::match_deref! { match &(orules.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRules = __pa0.clone();
    Ok(outRules)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getRulesFrontEnd(mut inRules: Rules) -> Rules {
    let mut outRules: Rules = metamodelica::nil();
    outRules = (::match_deref::match_deref! { match &(inRules.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: r @ Rule::FRONTEND_RULE { .. }, tail: rest } => {
            let mut lst: Rules = metamodelica::nil();
            lst = getRulesFrontEnd(rest.clone());
            cons(r.clone(), lst.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            getRulesFrontEnd(rest.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outRules
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getRulesBackEnd(mut inRules: Rules) -> Rules {
    let mut outRules: Rules = metamodelica::nil();
    outRules = (::match_deref::match_deref! { match &(inRules.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: r @ Rule::BACKEND_RULE { .. }, tail: rest } => {
            let mut lst: Rules = metamodelica::nil();
            lst = getRulesBackEnd(rest.clone());
            cons(r.clone(), lst.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            getRulesBackEnd(rest.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outRules
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn stmtsToRules(mut inStmts: Arc<metamodelica::List<GlobalScript::Statement>>, mut inAcc: Rules) -> Result<Rules> {
    let mut outRules: Rules = metamodelica::nil();
    outRules = 'mc: {
        let __mc_input = (inStmts.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inAcc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rewrite", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: from, tail: Deref @ metamodelica::List::Cons { head: to, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil }, .. }, .. }, tail: rest }, _) => {
                    let mut acc: Rules = metamodelica::nil();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FrontEnd rule: ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    acc = stmtsToRules(rest.clone(), cons(Rule::FRONTEND_RULE { from: from.clone(), to: to.clone() }, inAcc.clone()))?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rewriteFrontEnd", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: from, tail: Deref @ metamodelica::List::Cons { head: to, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil }, .. }, .. }, tail: rest }, _) => {
                    let mut acc: Rules = metamodelica::nil();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FrontEnd rule: ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    acc = stmtsToRules(rest.clone(), cons(Rule::FRONTEND_RULE { from: from.clone(), to: to.clone() }, inAcc.clone()))?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rewriteBackEnd", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: from, tail: Deref @ metamodelica::List::Cons { head: to, tail: Deref @ metamodelica::List::Nil } }, argNames: Deref @ metamodelica::List::Nil }, .. }, .. }, tail: rest }, _) => {
                    let mut acc: Rules = metamodelica::nil();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackEnd rule: ")); __mm_s.push_str(&*Dump::printExpStr(from.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*Dump::printExpStr(to.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    acc = stmtsToRules(rest.clone(), cons(Rule::BACKEND_RULE { from: from.clone(), to: to.clone() }, inAcc.clone()))?;
                    Ok(acc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: s, tail: _ }, _) => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unable to parse rewrite rule: ")); __mm_s.push_str(&*GlobalScriptDump::printIstmtStr(s.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRules)
}

