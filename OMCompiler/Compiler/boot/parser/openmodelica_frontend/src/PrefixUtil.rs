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

use crate::ComponentReference;
use crate::Expression;
use crate::ExpressionDump;
use crate::FCore;
use crate::FGraph;
use crate::InnerOuter;
use crate::Lookup;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

//import Util;
pub fn printComponentPrefixStr(mut pre: Arc<DAE::ComponentPrefix>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(pre.clone()) {
        Deref @ DAE::ComponentPrefix::NOCOMPPRE => {
            literal!("<Prefix.NOCOMPPRE()>")
        },
        Deref @ DAE::ComponentPrefix::PRE { subscripts: Deref @ metamodelica::List::Nil, next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, .. } => {
            var_field!((*pre).prefix, DAE::ComponentPrefix::PRE).clone()
        },
        Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*pre).prefix, DAE::ComponentPrefix::PRE).clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionDump::printSubscriptLstStr(var_field!((*pre).subscripts, DAE::ComponentPrefix::PRE).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ComponentPrefix::PRE { subscripts: Deref @ metamodelica::List::Nil, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*printComponentPrefixStr(var_field!((*pre).next, DAE::ComponentPrefix::PRE).clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*pre).prefix, DAE::ComponentPrefix::PRE).clone()); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ComponentPrefix::PRE { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*printComponentPrefixStr(var_field!((*pre).next, DAE::ComponentPrefix::PRE).clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*var_field!((*pre).prefix, DAE::ComponentPrefix::PRE).clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionDump::printSubscriptLstStr(var_field!((*pre).subscripts, DAE::ComponentPrefix::PRE).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printPrefixStr(mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inPrefix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::NOPRE => {
                    Ok(literal!("<Prefix.NOPRE()>"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ } => {
                    Ok(literal!("<Prefix.PREFIX(DAE.NOCOMPPRE())>"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: Deref @ metamodelica::List::Nil, next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, ci_state: _, info: _ }, classPre: _ } => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: ss, next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, ci_state: _, info: _ }, classPre: _ } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (stringAppend((r#str.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(ss.clone(), (std::sync::Arc::new(ExpressionDump::subscriptString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone())).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: Deref @ metamodelica::List::Nil, next: rest, ci_state: _, info: _ }, classPre: cp } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut rest_1: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    rest_1 = (printPrefixStr(DAE::Prefix::PREFIX { compPre: rest.clone(), classPre: cp.clone() })?).clone();
                    s = (stringAppend((rest_1.clone()).clone(), (literal!(".")).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (r#str.clone()).clone())).clone();
                    Ok(s_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: ss, next: rest, ci_state: _, info: _ }, classPre: cp } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut rest_1: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    rest_1 = (printPrefixStr(DAE::Prefix::PREFIX { compPre: rest.clone(), classPre: cp.clone() })?).clone();
                    s = (stringAppend((rest_1.clone()).clone(), (literal!(".")).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (r#str.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(ss.clone(), (std::sync::Arc::new(ExpressionDump::subscriptString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn printPrefixStr2(mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inPrefix.clone()) {
        DAE::Prefix::NOPRE => {
            literal!("")
        },
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ } => {
            literal!("")
        },
        p => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*printPrefixStr(p.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn printPrefixStr3(mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inPrefix.clone()) {
        DAE::Prefix::NOPRE => {
            literal!("<NO COMPONENT>")
        },
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ } => {
            literal!("<NO COMPONENT>")
        },
        p => {
            printPrefixStr(p.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn printPrefixStrIgnoreNoPre(mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inPrefix.clone()) {
        DAE::Prefix::NOPRE => {
            literal!("")
        },
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ } => {
            literal!("")
        },
        p => {
            printPrefixStr(p.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn printPrefix(mut p: DAE::Prefix) -> Result<()> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (printPrefixStr(p.clone())?).clone();
    Print::printBuf((s.clone()).clone())?;
    Ok(())
}

pub fn prefixAdd(mut inIdent: ArcStr, mut inType: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inIntegerLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inPrefix: DAE::Prefix, mut vt: SCode::Variability, mut ci_state: ClassInf::State, mut inInfo: SourceInfo) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
    outPrefix = (::match_deref::match_deref! { match &((inIdent.clone(), inType.clone(), inIntegerLst.clone(), inPrefix.clone(), vt.clone(), ci_state.clone())) {
        (i, _, s, DAE::Prefix::PREFIX { compPre: p, classPre: _ }, _, _) => {
            DAE::Prefix::PREFIX { compPre: Arc::new(DAE::ComponentPrefix::PRE { prefix: (i.clone()).clone(), dimensions: inType.clone(), subscripts: s.clone(), next: p.clone(), ci_state: ci_state.clone(), info: inInfo.clone() }), classPre: DAE::ClassPrefix { variability: vt.clone() } }
        },
        (i, _, s, DAE::Prefix::NOPRE, _, _) => {
            DAE::Prefix::PREFIX { compPre: Arc::new(DAE::ComponentPrefix::PRE { prefix: (i.clone()).clone(), dimensions: inType.clone(), subscripts: s.clone(), next: Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE), ci_state: ci_state.clone(), info: inInfo.clone() }), classPre: DAE::ClassPrefix { variability: vt.clone() } }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefix)
}

pub fn prefixFirst(mut inPrefix: DAE::Prefix) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
    outPrefix = (::match_deref::match_deref! { match &(inPrefix.clone()) {
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { info, ci_state, subscripts: b, dimensions: pdims, prefix: a, .. }, classPre: cp } => {
            DAE::Prefix::PREFIX { compPre: Arc::new(DAE::ComponentPrefix::PRE { prefix: (a.clone()).clone(), dimensions: pdims.clone(), subscripts: b.clone(), next: Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE), ci_state: ci_state.clone(), info: info.clone() }), classPre: cp.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefix)
}

pub fn prefixFirstCref(mut inPrefix: DAE::Prefix) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut name: ArcStr = arcstr::literal!("");
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let DAE::PREFIX { compPre: __t2, .. } = (inPrefix.clone()) else { bail!("pattern mismatch") };
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(__t2.clone()) {
        Deref @ DAE::ComponentPrefix::PRE { subscripts: __pa0, prefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    subs = __pa0.clone();
    name = __pa1.clone();
    outCref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: subs.clone() });
    Ok(outCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn prefixLast(mut inPrefix: DAE::Prefix) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
    outPrefix = 'mc: {
        let __mc_input = inPrefix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                res @ DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, .. }, classPre: _ } => {
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: p, .. }, classPre: cp } => {
                    let mut res: DAE::Prefix = DAE::Prefix::NOPRE;
                    res = prefixLast(DAE::Prefix::PREFIX { compPre: p.clone(), classPre: cp.clone() })?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPrefix)
}

pub fn prefixStripLast(mut inPrefix: DAE::Prefix) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix = DAE::Prefix::NOPRE;
    outPrefix = (match inPrefix.clone() {
        DAE::Prefix::NOPRE => {
            openmodelica_frontend_types::DAE::Prefix::NOPRE
        },
        DAE::Prefix::PREFIX { compPre: mut compPre, classPre: mut cp } => {
            let mut compPre = compPre.clone();
            compPre = compPreStripLast(compPre.clone())?;
            DAE::Prefix::PREFIX { compPre: compPre.clone(), classPre: cp.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outPrefix)
}

fn compPreStripLast(mut inCompPrefix: Arc<DAE::ComponentPrefix>) -> Result<Arc<DAE::ComponentPrefix>> {
    let mut outCompPrefix: Arc<DAE::ComponentPrefix> = Arc::new(DAE::ComponentPrefix::NOCOMPPRE);
    outCompPrefix = (::match_deref::match_deref! { match &(inCompPrefix.clone()) {
        Deref @ DAE::ComponentPrefix::NOCOMPPRE => {
            Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE)
        },
        Deref @ DAE::ComponentPrefix::PRE { next, .. } => {
            next.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCompPrefix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn prefixPath(mut inPath: Arc<Absyn::Path>, mut inPrefix: DAE::Prefix) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath.clone(), inPrefix.clone())) {
        (p, DAE::Prefix::NOPRE) => {
            p.clone()
        },
        (p, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, prefix: s, .. }, classPre: _ }) => {
            let mut p_1: Arc<Absyn::Path>;
            p_1 = Arc::new(Absyn::Path::QUALIFIED { name: (s.clone()).clone(), path: p.clone() });
            p_1.clone()
        },
        (p, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: ss, prefix: s, .. }, classPre: cp }) => {
            let mut p_1: Arc<Absyn::Path>;
            p_1 = prefixPath(Arc::new(Absyn::Path::QUALIFIED { name: (s.clone()).clone(), path: p.clone() }), DAE::Prefix::PREFIX { compPre: ss.clone(), classPre: cp.clone() })?;
            p_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn prefixToPath(mut inPrefix: DAE::Prefix) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (match inPrefix.clone() {
        DAE::Prefix::PREFIX { compPre: ref ss, classPre: _ } => {
            componentPrefixToPath(ss.clone())?
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outPath)
}

pub fn identAndPrefixToPath(mut ident: ArcStr, mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (AbsynUtil::pathString(prefixPath(Arc::new(Absyn::Path::IDENT { name: (ident.clone()).clone() }), inPrefix.clone())?, (literal!(".")).clone(), true, false)?).clone();
    Ok(r#str)
}

pub fn componentPrefixToPath(mut pre: Arc<DAE::ComponentPrefix>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = (::match_deref::match_deref! { match &(pre.clone()) {
        Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, prefix: s, .. } => {
            Arc::new(Absyn::Path::IDENT { name: (s.clone()).clone() })
        },
        Deref @ DAE::ComponentPrefix::PRE { next: ss, prefix: s, .. } => {
            Arc::new(Absyn::Path::QUALIFIED { name: (s.clone()).clone(), path: componentPrefixToPath(ss.clone())? })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(path)
}

pub fn prefixCref(mut cache: FCore::Cache, mut env: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut pre: DAE::Prefix, mut cref: Arc<DAE::ComponentRef>) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut cref_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outCache, cref_1) = prefixToCref2(cache.clone(), env.clone(), inIH.clone(), pre.clone(), Some(cref.clone()))?;
    Ok((outCache, cref_1))
}

pub fn prefixCrefNoContext(mut inPre: DAE::Prefix, mut inCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (_, outCref) = prefixToCref2(FCore::noCache(), FGraph::empty(), InnerOuter::emptyInstHierarchy().clone(), inPre.clone(), Some(inCref.clone()))?;
    Ok(outCref)
}

pub fn prefixToCref(mut pre: DAE::Prefix) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (_, cref_1) = prefixToCref2(FCore::noCache(), FGraph::empty(), InnerOuter::emptyInstHierarchy().clone(), pre.clone(), None)?;
    Ok(cref_1)
}

fn prefixToCref2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstanceHierarchy, mut inPrefix: DAE::Prefix, mut inExpComponentRefOption: Option<Arc<DAE::ComponentRef>>) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outCache, outComponentRef) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inExpComponentRefOption.clone())) {
        (_, _, _, DAE::Prefix::NOPRE, None) => {
            bail!("fail")
        },
        (_, _, _, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ }, None) => {
            bail!("fail")
        },
        (cache, _, _, DAE::Prefix::NOPRE, Some(cref)) => {
            (cache.clone(), cref.clone())
        },
        (cache, _, _, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ }, Some(cref)) => {
            (cache.clone(), cref.clone())
        },
        (cache, env, _, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { ci_state, next: xs, subscripts: s, dimensions: ds, prefix: i, .. }, classPre: cp }, None) => {
            let mut cref_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ident_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            ident_ty = Expression::liftArrayLeftList(Arc::new(DAE::Type::T_COMPLEX { complexClassType: ci_state.clone(), varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), ds.clone());
            cref_ = ComponentReferenceBasics::makeCrefIdent((i.clone()).clone(), ident_ty.clone(), s.clone());
            (cache, cref_1) = prefixToCref2(cache.clone(), env.clone(), inIH.clone(), DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_.clone()))?;
            (cache.clone(), cref_1.clone())
        },
        (cache, env, _, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { ci_state, next: xs, subscripts: s, dimensions: ds, prefix: i, .. }, classPre: cp }, Some(cref)) => {
            let mut cref_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cref_2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ident_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut cache = (*cache).clone();
            let mut cref = (*cref).clone();
            (cache, cref) = prefixSubscriptsInCref(cache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), cref.clone())?;
            ident_ty = Expression::liftArrayLeftList(Arc::new(DAE::Type::T_COMPLEX { complexClassType: ci_state.clone(), varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), ds.clone());
            cref_2 = ComponentReferenceBasics::makeCrefQual((i.clone()).clone(), ident_ty.clone(), s.clone(), cref.clone());
            (cache, cref_1) = prefixToCref2(cache.clone(), env.clone(), inIH.clone(), DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_2.clone()))?;
            (cache.clone(), cref_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outComponentRef))
}

pub fn prefixToCrefOpt(mut pre: DAE::Prefix) -> Result<Option<Arc<DAE::ComponentRef>>> {
    let mut cref_1: Option<Arc<DAE::ComponentRef>> = None;
    cref_1 = prefixToCrefOpt2(pre.clone(), None)?;
    Ok(cref_1)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn prefixToCrefOpt2(mut inPrefix: DAE::Prefix, mut inExpComponentRefOption: Option<Arc<DAE::ComponentRef>>) -> Result<Option<Arc<DAE::ComponentRef>>> {
    let mut outComponentRefOpt: Option<Arc<DAE::ComponentRef>> = None;
    outComponentRefOpt = (::match_deref::match_deref! { match &((inPrefix.clone(), inExpComponentRefOption.clone())) {
        (DAE::Prefix::NOPRE, None) => {
            None
        },
        (DAE::Prefix::NOPRE, Some(cref)) => {
            Some(cref.clone())
        },
        (DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ }, Some(cref)) => {
            Some(cref.clone())
        },
        (DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: xs, subscripts: s, prefix: i, .. }, classPre: cp }, None) => {
            let mut cref_1: Option<Arc<DAE::ComponentRef>> = None;
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref_ = ComponentReferenceBasics::makeCrefIdent((i.clone()).clone(), Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), s.clone());
            cref_1 = prefixToCrefOpt2(DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_.clone()))?;
            cref_1.clone()
        },
        (DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: xs, subscripts: s, prefix: i, .. }, classPre: cp }, Some(cref)) => {
            let mut cref_1: Option<Arc<DAE::ComponentRef>> = None;
            let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref_ = ComponentReferenceBasics::makeCrefQual((i.clone()).clone(), Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), s.clone(), cref.clone());
            cref_1 = prefixToCrefOpt2(DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_.clone()))?;
            cref_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRefOpt)
}

pub fn makeCrefFromPrefixNoFail(mut pre: DAE::Prefix) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cref = 'mc: {
        let __mc_input = pre.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::NOPRE => {
                    let mut c: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    c = ComponentReferenceBasics::makeCrefIdent((literal!("")).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    Ok(c.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE, classPre: _ } => {
                    let mut c: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    c = ComponentReferenceBasics::makeCrefIdent((literal!("")).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    Ok(c.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut c: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    c = prefixToCref(pre.clone())?;
                    Ok(c.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cref)
}

fn prefixSubscriptsInCref(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstanceHierarchy, mut pre: DAE::Prefix, mut inCr: Arc<DAE::ComponentRef>) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outCache, outCr) = prefixSubscriptsInCrefWork(inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), inCr.clone(), metamodelica::nil())?;
    Ok((outCache, outCr))
}

fn prefixSubscriptsInCrefWork(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstanceHierarchy, mut pre: DAE::Prefix, mut inCr: Arc<DAE::ComponentRef>, mut acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outCache, outCr) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), inCr.clone(), acc.clone())) {
        (cache, env, _, _, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: tp, subscriptLst: subs }, _) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cache = (*cache).clone();
            let mut subs = (*subs).clone();
            (cache, subs) = prefixSubscripts(cache.clone(), env.clone(), inIH.clone(), pre.clone(), subs.clone())?;
            cr = ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), tp.clone(), subs.clone());
            (cache.clone(), ComponentReference::implode_reverse(cons(cr.clone(), acc.clone()))?)
        },
        (cache, env, _, _, Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: tp, subscriptLst: subs, componentRef: cr }, _) => {
            let mut crid: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cache = (*cache).clone();
            let mut subs = (*subs).clone();
            let mut cr = (*cr).clone();
            (cache, subs) = prefixSubscripts(cache.clone(), env.clone(), inIH.clone(), pre.clone(), subs.clone())?;
            crid = ComponentReferenceBasics::makeCrefIdent((id.clone()).clone(), tp.clone(), subs.clone());
            (cache, cr) = prefixSubscriptsInCrefWork(cache.clone(), env.clone(), inIH.clone(), pre.clone(), cr.clone(), cons(crid.clone(), acc.clone()))?;
            (cache.clone(), cr.clone())
        },
        (cache, _, _, _, Deref @ DAE::ComponentRef::WILD, _) => {
            (cache.clone(), Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outCr))
}

fn prefixSubscripts(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstanceHierarchy, mut pre: DAE::Prefix, mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Subscript>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    (outCache, outSubs) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), inSubs.clone())) {
        (cache, _, _, _, Deref @ metamodelica::List::Nil) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, _, _, Deref @ metamodelica::List::Cons { head: sub, tail: subs }) => {
            let mut cache = (*cache).clone();
            let mut sub = (*sub).clone();
            let mut subs = (*subs).clone();
            (cache, sub) = prefixSubscript(cache.clone(), env.clone(), inIH.clone(), pre.clone(), sub.clone())?;
            (cache, subs) = prefixSubscripts(cache.clone(), env.clone(), inIH.clone(), pre.clone(), subs.clone())?;
            (cache.clone(), cons(sub.clone(), subs.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outSubs))
}

fn prefixSubscript(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstanceHierarchy, mut pre: DAE::Prefix, mut sub: Arc<DAE::Subscript>) -> Result<(FCore::Cache, Arc<DAE::Subscript>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSub: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    (outCache, outSub) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), pre.clone(), sub.clone())) {
        (cache, _, _, _, Deref @ DAE::Subscript::WHOLEDIM) => {
            (cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM))
        },
        (cache, env, _, _, Deref @ DAE::Subscript::SLICE { exp }) => {
            let mut cache = (*cache).clone();
            let mut exp = (*exp).clone();
            (cache, exp) = prefixExpWork(cache.clone(), env.clone(), inIH.clone(), exp.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Subscript::SLICE { exp: exp.clone() }))
        },
        (cache, env, _, _, Deref @ DAE::Subscript::WHOLE_NONEXP { exp }) => {
            let mut cache = (*cache).clone();
            let mut exp = (*exp).clone();
            (cache, exp) = prefixExpWork(cache.clone(), env.clone(), inIH.clone(), exp.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: exp.clone() }))
        },
        (cache, env, _, _, Deref @ DAE::Subscript::INDEX { exp }) => {
            let mut cache = (*cache).clone();
            let mut exp = (*exp).clone();
            (cache, exp) = prefixExpWork(cache.clone(), env.clone(), inIH.clone(), exp.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Subscript::INDEX { exp: exp.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outSub))
}

pub fn prefixCrefInnerOuter(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inCref: Arc<DAE::ComponentRef>, mut inPrefix: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outCache, outCref) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inCref.clone(), inPrefix.clone())) {
        (cache, _, ih, cref, pre) => {
            let mut newCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            newCref = InnerOuter::prefixOuterCrefWithTheInnerPrefix(ih.clone(), cref.clone(), pre.clone())?;
            (cache.clone(), newCref.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outCref))
}

pub fn prefixExp(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut exp: Arc<DAE::Exp>, mut pre: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::Exp>)> {
    let mut cache: FCore::Cache = cache;
    let mut exp: Arc<DAE::Exp> = exp;
    if let Ok((__pa0, __pa1)) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), exp.clone(), pre.clone()) {
        cache = __pa0.clone();
        exp = __pa1.clone();
    } else {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PrefixUtil.prefixExp")); __mm_s.push_str(&*literal!(" failed on exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*makePrefixString(pre.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok((cache, exp))
}

fn prefixExpWork(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inExp: Arc<DAE::Exp>, mut pre: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::Exp>)> {
    let mut cache: FCore::Cache = cache;
    let mut outExp: Arc<DAE::Exp>;
    (cache, outExp) = (::match_deref::match_deref! { match &((inExp.clone(), pre.clone())) {
        (e, DAE::Prefix::NOPRE) if (!(System::getHasInnerOuterDefinitions())) => {
            (cache.clone(), e.clone())
        },
        (e @ Deref @ DAE::Exp::ICONST { .. }, _) => {
            (cache.clone(), e.clone())
        },
        (e @ Deref @ DAE::Exp::RCONST { .. }, _) => {
            (cache.clone(), e.clone())
        },
        (e @ Deref @ DAE::Exp::SCONST { .. }, _) => {
            (cache.clone(), e.clone())
        },
        (e @ Deref @ DAE::Exp::BCONST { .. }, _) => {
            (cache.clone(), e.clone())
        },
        (e @ Deref @ DAE::Exp::ENUM_LITERAL { .. }, _) => {
            (cache.clone(), e.clone())
        },
        (Deref @ DAE::Exp::CREF { ty: t, componentRef: cr }, _) => {
            let mut crefExp: Arc<DAE::Exp>;
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut t = (*t).clone();
            if System::getHasInnerOuterDefinitions() && !(ih.clone().is_empty()) {
                if '__try0: {
                    cr_1 = unwrap_break_err!(InnerOuter::prefixOuterCrefWithTheInnerPrefix(ih.clone(), cr.clone(), pre.clone()), '__try0);
                    (cache, t) = unwrap_break_err!(prefixExpressionsInType(cache.clone(), env.clone(), ih.clone(), pre.clone(), t.clone()), '__try0);
                    outExp = unwrap_break_err!(Expression::makeCrefExp(cr_1.clone(), t.clone()), '__try0);
                    return Ok((cache, outExp));
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
            }
            if openmodelica_frontend_types::DAE::Prefix::NOPRE == pre.clone() {
                crefExp = inExp.clone();
            } else {
                (cache, crefExp) = prefixExpCref(cache.clone(), env.clone(), ih.clone(), inExp.clone(), pre.clone())?;
            }
            (cache.clone(), crefExp.clone())
        },
        (Deref @ DAE::Exp::CLKCONST { clk }, _) => {
            let mut clk = (*clk).clone();
            (cache, clk) = prefixClockKind(cache.clone(), env.clone(), ih.clone(), clk.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::CLKCONST { clk: clk.clone() }))
        },
        (Deref @ DAE::Exp::ASUB { sub: subs, exp: e1 }, _) => {
            let mut e2: Arc<DAE::Exp>;
            let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut e1 = (*e1).clone();
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (cache, es_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), expl.clone(), pre.clone())?;
            (cache, e1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            e2 = Expression::makeASUB(e1.clone(), es_1.clone())?;
            (cache.clone(), e2.clone())
        },
        (Deref @ DAE::Exp::TSUB { exp: e1, ix: index_, ty: t }, _) => {
            let mut e2: Arc<DAE::Exp>;
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            e2 = Arc::new(DAE::Exp::TSUB { exp: e1.clone(), ix: index_.clone(), ty: t.clone() });
            (cache.clone(), e2.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp2: e2, operator: o, exp1: e1 }, _) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache, e2_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e2.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: o.clone(), exp2: e2_1.clone() }))
        },
        (Deref @ DAE::Exp::UNARY { exp: e1, operator: o }, _) => {
            let mut e1_1: Arc<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::UNARY { operator: o.clone(), exp: e1_1.clone() }))
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: o, exp1: e1 }, _) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache, e2_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e2.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: o.clone(), exp2: e2_1.clone() }))
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, operator: o }, _) => {
            let mut e1_1: Arc<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::LUNARY { operator: o.clone(), exp: e1_1.clone() }))
        },
        (Deref @ DAE::Exp::RELATION { optionExpisASUB: isExpisASUB, index: index_, exp2: e2, operator: o, exp1: e1 }, _) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache, e2_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e2.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: o.clone(), exp2: e2_1.clone(), index: index_.clone(), optionExpisASUB: isExpisASUB.clone() }))
        },
        (Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, _) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e3_1: Arc<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache, e2_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e2.clone(), pre.clone())?;
            (cache, e3_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e3.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() }))
        },
        (Deref @ DAE::Exp::SIZE { sz: Some(dim), exp: cref }, _) => {
            let mut cref_1: Arc<DAE::Exp>;
            let mut dim_1: Arc<DAE::Exp>;
            (cache, cref_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), cref.clone(), pre.clone())?;
            (cache, dim_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), dim.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::SIZE { exp: cref_1.clone(), sz: Some(dim_1.clone()) }))
        },
        (Deref @ DAE::Exp::SIZE { sz: None, exp: cref }, _) => {
            let mut cref_1: Arc<DAE::Exp>;
            (cache, cref_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), cref.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::SIZE { exp: cref_1.clone(), sz: None }))
        },
        (Deref @ DAE::Exp::CALL { path: f, expLst: es, attr }, _) => {
            let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (cache, es_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), es.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::CALL { path: f.clone(), expLst: es_1.clone(), attr: attr.clone() }))
        },
        (e @ Deref @ DAE::Exp::PARTEVALFUNCTION { .. }, _) => {
            let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut e = (*e).clone();
            (cache, es_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), var_field!((*e).expList, DAE::Exp::PARTEVALFUNCTION).clone(), pre.clone())?;
            assign_variant_field!(e => DAE::Exp::PARTEVALFUNCTION; expList = es_1.clone());
            (cache.clone(), e.clone())
        },
        (Deref @ DAE::Exp::RECORD { path: f, exps: es, comp: fieldNames, ty: t }, _) => {
            (cache, _) = prefixExpList(cache.clone(), env.clone(), ih.clone(), es.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::RECORD { path: f.clone(), exps: es.clone(), comp: fieldNames.clone(), ty: t.clone() }))
        },
        (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, _) => {
            (cache.clone(), inExp.clone())
        },
        (Deref @ DAE::Exp::ARRAY { array: es, scalar: sc, ty: t }, _) => {
            let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (cache, es_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), es.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: sc.clone(), array: es_1.clone() }))
        },
        (Deref @ DAE::Exp::TUPLE { PR: es }, _) => {
            let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (cache, es_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), es.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::TUPLE { PR: es_1.clone() }))
        },
        (Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Nil, .. }, _) => {
            (cache.clone(), inExp.clone())
        },
        (Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: x, tail: xs }, integer: a, ty: t }, _) => {
            let mut x_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut xs_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut t = (*t).clone();
            (cache, x_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), x.clone(), pre.clone())?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(prefixExpWork(cache.clone(), env.clone(), ih.clone(), Arc::new(DAE::Exp::MATRIX { ty: t.clone(), integer: a.clone(), matrix: xs.clone() }), pre.clone())?) {
                (__pa0, Deref @ DAE::Exp::MATRIX { ty: __pa1, integer: _, matrix: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            t = __pa1.clone();
            xs_1 = __pa2.clone();
            (cache.clone(), Arc::new(DAE::Exp::MATRIX { ty: t.clone(), integer: a.clone(), matrix: cons(x_1.clone(), xs_1.clone()) }))
        },
        (Deref @ DAE::Exp::RANGE { stop, step: None, start, ty: t }, _) => {
            let mut start_1: Arc<DAE::Exp>;
            let mut stop_1: Arc<DAE::Exp>;
            (cache, start_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), start.clone(), pre.clone())?;
            (cache, stop_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), stop.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::RANGE { ty: t.clone(), start: start_1.clone(), step: None, stop: stop_1.clone() }))
        },
        (Deref @ DAE::Exp::RANGE { stop, step: Some(step), start, ty: t }, _) => {
            let mut start_1: Arc<DAE::Exp>;
            let mut stop_1: Arc<DAE::Exp>;
            let mut step_1: Arc<DAE::Exp>;
            (cache, start_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), start.clone(), pre.clone())?;
            (cache, step_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), step.clone(), pre.clone())?;
            (cache, stop_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), stop.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::RANGE { ty: t.clone(), start: start_1.clone(), step: Some(step_1.clone()), stop: stop_1.clone() }))
        },
        (Deref @ DAE::Exp::CAST { exp: e, ty: tp }, _) => {
            let mut e_1: Arc<DAE::Exp>;
            (cache, e_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: e_1.clone() }))
        },
        (Deref @ DAE::Exp::REDUCTION { iterators: riters, expr: exp, reductionInfo }, _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut riters = (*riters).clone();
            (cache, exp_1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), exp.clone(), pre.clone())?;
            (cache, riters) = prefixIterators(cache.clone(), env.clone(), ih.clone(), riters.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: exp_1.clone(), iterators: riters.clone() }))
        },
        (Deref @ DAE::Exp::LIST { valList: es }, _) => {
            let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (cache, es_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), es.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::LIST { valList: es_1.clone() }))
        },
        (Deref @ DAE::Exp::CONS { car: e1, cdr: e2 }, _) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (cache, e1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache, e2) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e2.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::CONS { car: e1.clone(), cdr: e2.clone() }))
        },
        (Deref @ DAE::Exp::META_TUPLE { listExp: es }, _) => {
            let mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (cache, es_1) = prefixExpList(cache.clone(), env.clone(), ih.clone(), es.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::META_TUPLE { listExp: es_1.clone() }))
        },
        (Deref @ DAE::Exp::META_OPTION { exp: Some(e1) }, _) => {
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::META_OPTION { exp: Some(e1.clone()) }))
        },
        (Deref @ DAE::Exp::META_OPTION { exp: None }, _) => {
            (cache.clone(), Arc::new(DAE::Exp::META_OPTION { exp: None }))
        },
        (Deref @ DAE::Exp::METARECORDCALL { .. }, _) => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            (cache, expl) = prefixExpList(cache.clone(), env.clone(), ih.clone(), var_field!((*inExp).args, DAE::Exp::METARECORDCALL).clone(), pre.clone())?;
            (cache.clone(), Arc::new(DAE::Exp::METARECORDCALL { path: var_field!((*inExp).path, DAE::Exp::METARECORDCALL).clone(), args: expl.clone(), fieldNames: var_field!((*inExp).fieldNames, DAE::Exp::METARECORDCALL).clone(), index: var_field!((*inExp).index, DAE::Exp::METARECORDCALL).clone(), typeVars: var_field!((*inExp).typeVars, DAE::Exp::METARECORDCALL).clone() }))
        },
        (e @ Deref @ DAE::Exp::UNBOX { exp: e1, .. }, _) => {
            let mut e = (*e).clone();
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            assign_variant_field!(e => DAE::Exp::UNBOX; exp = e1.clone());
            (cache.clone(), e.clone())
        },
        (e @ Deref @ DAE::Exp::BOX { exp: e1 }, _) => {
            let mut e = (*e).clone();
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e1.clone(), pre.clone())?;
            assign_variant_field!(e => DAE::Exp::BOX; exp = e1.clone());
            (cache.clone(), e.clone())
        },
        (e, DAE::Prefix::NOPRE) => {
            (cache.clone(), e.clone())
        },
        (e @ Deref @ DAE::Exp::EMPTY { .. }, _) => {
            (cache.clone(), e.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PrefixUtil.prefixExpWork")); __mm_s.push_str(&*literal!(" failed on exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*makePrefixString(pre.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, outExp))
}

fn prefixExpCref(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstanceHierarchy, mut inCref: Arc<DAE::Exp>, mut inPrefix: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::Exp>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outCref: Arc<DAE::Exp>;
    let mut is_iter: Option<bool> = None;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let __pa0 = ::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    (is_iter, cache) = Lookup::isIterator(inCache.clone(), inEnv.clone(), cr.clone())?;
    (outCache, outCref) = prefixExpCref2(cache.clone(), inEnv.clone(), inIH.clone(), is_iter.clone(), inCref.clone(), inPrefix.clone())?;
    Ok((outCache, outCref))
}

fn prefixExpCref2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: InstanceHierarchy, mut inIsIter: Option<bool>, mut inCref: Arc<DAE::Exp>, mut inPrefix: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::Exp>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outCref: Arc<DAE::Exp>;
    (outCache, outCref) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inIsIter.clone(), inCref.clone(), inPrefix.clone())) {
        (cache, _, _, Some(false), Deref @ DAE::Exp::CREF { ty, componentRef: cr }, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cache = (*cache).clone();
            let mut ty = (*ty).clone();
            let mut cr = (*cr).clone();
            (cache, cr) = prefixCref(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), cr.clone())?;
            (cache, ty) = prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), ty.clone())?;
            exp = Expression::makeCrefExp(cr.clone(), ty.clone())?;
            (cache.clone(), exp.clone())
        },
        (_, _, _, Some(true), _, _) => {
            (inCache.clone(), inCref.clone())
        },
        (cache, _, _, None, Deref @ DAE::Exp::CREF { ty, componentRef: cr }, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cache = (*cache).clone();
            let mut ty = (*ty).clone();
            let mut cr = (*cr).clone();
            (cache, cr) = prefixSubscriptsInCref(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), cr.clone())?;
            (cache, ty) = prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), ty.clone())?;
            exp = Expression::makeCrefExp(cr.clone(), ty.clone())?;
            (cache.clone(), exp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outCref))
}

fn prefixIterators(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut ih: InstanceHierarchy, mut inIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut pre: DAE::Prefix) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
    (outCache, outIters) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), ih.clone(), inIters.clone(), pre.clone())) {
        (cache, _, _, Deref @ metamodelica::List::Nil, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, _, Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { id, exp, guardExp: Some(gexp), ty }, tail: iters }, _) => {
            let mut iter: Arc<DAE::ReductionIterator> = Arc::new(<DAE::ReductionIterator as ::std::default::Default>::default());
            let mut cache = (*cache).clone();
            let mut exp = (*exp).clone();
            let mut gexp = (*gexp).clone();
            let mut iters = (*iters).clone();
            (cache, exp) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), exp.clone(), pre.clone())?;
            (cache, gexp) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), gexp.clone(), pre.clone())?;
            iter = Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp.clone(), guardExp: Some(gexp.clone()), ty: ty.clone() });
            (cache, iters) = prefixIterators(cache.clone(), env.clone(), ih.clone(), iters.clone(), pre.clone())?;
            (cache.clone(), cons(iter.clone(), iters.clone()))
        },
        (cache, env, _, Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { id, exp, guardExp: None, ty }, tail: iters }, _) => {
            let mut iter: Arc<DAE::ReductionIterator> = Arc::new(<DAE::ReductionIterator as ::std::default::Default>::default());
            let mut cache = (*cache).clone();
            let mut exp = (*exp).clone();
            let mut iters = (*iters).clone();
            (cache, exp) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), exp.clone(), pre.clone())?;
            iter = Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp.clone(), guardExp: None, ty: ty.clone() });
            (cache, iters) = prefixIterators(cache.clone(), env.clone(), ih.clone(), iters.clone(), pre.clone())?;
            (cache.clone(), cons(iter.clone(), iters.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outIters))
}

pub fn prefixExpList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inPrefix: DAE::Prefix) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut e_1: Arc<DAE::Exp>;
    for mut e in &*inExpExpLst.clone() {
        let mut e = e.clone();
        (outCache, e_1) = prefixExpWork(outCache.clone(), inEnv.clone(), inIH.clone(), e.clone(), inPrefix.clone())?;
        outExpExpLst = cons(e_1.clone(), outExpExpLst.clone());
    }
    outExpExpLst = Dangerous::listReverseInPlace(outExpExpLst.clone());
    Ok((outCache, outExpExpLst))
}

//--------------------------------------------
//   PART OF THE WORKAROUND FOR VALUEBLOCKS. KS
fn prefixStatements(mut cache: FCore::Cache, mut env: FCore::Graph, mut inIH: InstanceHierarchy, mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut p: DAE::Prefix) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Statement>>>)> {
    let mut outCache: FCore::Cache = cache.clone();
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    for mut st in &*stmts.clone() {
        let mut st = st.clone();
        let _ = (::match_deref::match_deref! { match &(st.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { type_: t, exp1: e1, exp: e, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut e1 = (*e1).clone();
            let mut e = (*e).clone();
            (outCache, e1) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e1.clone(), p.clone())?;
            (outCache, e) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_ASSIGN { type_: t.clone(), exp1: e1.clone(), exp: e.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: t, expExpLst: eLst, exp: e, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut eLst = (*eLst).clone();
            let mut e = (*e).clone();
            (outCache, e) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e.clone(), p.clone())?;
            (outCache, eLst) = prefixExpList(outCache.clone(), env.clone(), inIH.clone(), eLst.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: t.clone(), expExpLst: eLst.clone(), exp: e.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: t, lhs: e1, exp: e, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut e1 = (*e1).clone();
            let mut e = (*e).clone();
            (outCache, e1) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e1.clone(), p.clone())?;
            (outCache, e) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: t.clone(), lhs: e1.clone(), exp: e.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_FOR { type_: t, iterIsArray: bool, iter: id, range: e, statementLst: sList, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut e = (*e).clone();
            let mut sList = (*sList).clone();
            (outCache, e) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e.clone(), p.clone())?;
            (outCache, sList) = prefixStatements(outCache.clone(), env.clone(), inIH.clone(), sList.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: bool.clone(), iter: (id.clone()).clone(), range: e.clone(), statementLst: sList.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_IF { exp: e1, statementLst: sList, else_: elseBranch, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut e1 = (*e1).clone();
            let mut sList = (*sList).clone();
            let mut elseBranch = (*elseBranch).clone();
            (outCache, e1) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e1.clone(), p.clone())?;
            (outCache, sList) = prefixStatements(outCache.clone(), env.clone(), inIH.clone(), sList.clone(), p.clone())?;
            (outCache, elseBranch) = prefixElse(outCache.clone(), env.clone(), inIH.clone(), elseBranch.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_IF { exp: e1.clone(), statementLst: sList.clone(), else_: elseBranch.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_WHILE { exp: e1, statementLst: sList, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut e1 = (*e1).clone();
            let mut sList = (*sList).clone();
            (outCache, e1) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e1.clone(), p.clone())?;
            (outCache, sList) = prefixStatements(outCache.clone(), env.clone(), inIH.clone(), sList.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_WHILE { exp: e1.clone(), statementLst: sList.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond: e1, msg: e2, level: e3, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut e3 = (*e3).clone();
            (outCache, e1) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e1.clone(), p.clone())?;
            (outCache, e2) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e2.clone(), p.clone())?;
            (outCache, e3) = prefixExpWork(outCache.clone(), env.clone(), inIH.clone(), e3.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_ASSERT { cond: e1.clone(), msg: e2.clone(), level: e3.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_FAILURE { body: b, source } => {
            let mut elem: Arc<DAE::Statement>;
            let mut b = (*b).clone();
            (outCache, b) = prefixStatements(outCache.clone(), env.clone(), inIH.clone(), b.clone(), p.clone())?;
            elem = Arc::new(DAE::Statement::STMT_FAILURE { body: b.clone(), source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_RETURN { source } => {
            let mut elem: Arc<DAE::Statement>;
            elem = Arc::new(DAE::Statement::STMT_RETURN { source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_BREAK { source } => {
            let mut elem: Arc<DAE::Statement>;
            elem = Arc::new(DAE::Statement::STMT_BREAK { source: source.clone() });
            outStmts = cons(elem.clone(), outStmts.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    outStmts = Dangerous::listReverseInPlace(outStmts.clone());
    Ok((outCache, outStmts))
}

fn prefixElse(mut cache: FCore::Cache, mut env: FCore::Graph, mut inIH: InstanceHierarchy, mut elseBranch: Arc<DAE::Else>, mut p: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::Else>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    (outCache, outElse) = (::match_deref::match_deref! { match &((cache.clone(), env.clone(), inIH.clone(), elseBranch.clone(), p.clone())) {
        (localCache, _, _, Deref @ DAE::Else::NOELSE, _) => {
            (localCache.clone(), Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE))
        },
        (localCache, localEnv, ih, Deref @ DAE::Else::ELSEIF { exp: e, statementLst: lStmt, else_: el }, pre) => {
            let mut stmt: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            let mut localCache = (*localCache).clone();
            let mut e = (*e).clone();
            let mut lStmt = (*lStmt).clone();
            let mut el = (*el).clone();
            (localCache, e) = prefixExpWork(localCache.clone(), localEnv.clone(), ih.clone(), e.clone(), pre.clone())?;
            (localCache, el) = prefixElse(localCache.clone(), localEnv.clone(), ih.clone(), el.clone(), pre.clone())?;
            (localCache, lStmt) = prefixStatements(localCache.clone(), localEnv.clone(), ih.clone(), lStmt.clone(), pre.clone())?;
            stmt = Arc::new(DAE::Else::ELSEIF { exp: e.clone(), statementLst: lStmt.clone(), else_: el.clone() });
            (localCache.clone(), stmt.clone())
        },
        (localCache, localEnv, ih, Deref @ DAE::Else::ELSE { statementLst: lStmt }, pre) => {
            let mut stmt: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            let mut localCache = (*localCache).clone();
            let mut lStmt = (*lStmt).clone();
            (localCache, lStmt) = prefixStatements(localCache.clone(), localEnv.clone(), ih.clone(), lStmt.clone(), pre.clone())?;
            stmt = Arc::new(DAE::Else::ELSE { statementLst: lStmt.clone() });
            (localCache.clone(), stmt.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outElse))
}

pub fn makePrefixString(mut pre: DAE::Prefix) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = pre.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Prefix::NOPRE = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!("from top scope"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = r#str.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("from calling scope: ")); __mm_s.push_str(&*printPrefixStr(pre.clone())?); ArcStr::from(__mm_s) }).clone();
            Ok(r#str.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn prefixExpressionsInType(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPre: DAE::Prefix, mut inTy: Arc<DAE::Type>) -> Result<(FCore::Cache, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTy: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outCache, outTy) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), inTy.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    Ok((inCache.clone(), inTy.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outTy: Arc<DAE::Type> = outTy.clone();
                    let (__pa0, (__pa1, _, _, _)) = Types::traverseType(inTy.clone(), (inCache.clone(), inEnv.clone(), inIH.clone(), inPre.clone()), (std::sync::Arc::new(prefixArrayDimensions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, (FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix)) -> Result<(Arc<DAE::Type>, (FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix))> + 'static>))?;
                    outTy = __pa0.clone();
                    outCache = __pa1.clone();
                    Ok((outCache.clone(), outTy.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outTy))
}

fn prefixArrayDimensions(mut ty: Arc<DAE::Type>, mut tpl: (FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix)) -> Result<(Arc<DAE::Type>, (FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix))> {
    let mut oty: Arc<DAE::Type> = ty.clone();
    let mut otpl: (FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, DAE::Prefix);
    (oty, otpl) = (::match_deref::match_deref! { match &((oty.clone(), tpl.clone())) {
        (Deref @ DAE::Type::T_ARRAY { .. }, (cache, env, ih, pre)) => {
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, dims) = prefixDimensions(cache.clone(), env.clone(), ih.clone(), pre.clone(), var_field!((*oty).dims, DAE::Type::T_ARRAY).clone())?;
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = dims.clone());
            (oty.clone(), (cache.clone(), env.clone(), ih.clone(), pre.clone()))
        },
        _ => {
            (oty.clone(), tpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oty, otpl))
}

pub fn prefixDimensions(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPre: DAE::Prefix, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    (outCache, outDims) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), inDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok((inCache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: e }, tail: rest }) => {
                    let mut new: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut e = (*e).clone();
                    (cache, e) = prefixExpWork(inCache.clone(), inEnv.clone(), inIH.clone(), e.clone(), inPre.clone())?;
                    (cache, new) = prefixDimensions(cache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), rest.clone())?;
                    Ok((cache.clone(), cons(Arc::new(DAE::Dimension::DIM_EXP { exp: e.clone() }), new.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: d, tail: rest }) => {
                    let mut new: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cache, new) = prefixDimensions(inCache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), rest.clone())?;
                    Ok((cache.clone(), cons(d.clone(), new.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDims))
}

pub fn isPrefix(mut prefix: DAE::Prefix) -> bool {
    let mut isPrefix: bool = false;
    isPrefix = (match prefix.clone() {
        DAE::Prefix::PREFIX { .. } => true,
        _ => false,
    });
    isPrefix
}

pub fn isNoPrefix(mut inPrefix: DAE::Prefix) -> bool {
    let mut outIsEmpty: bool = false;
    outIsEmpty = (match inPrefix.clone() {
        DAE::Prefix::NOPRE => true,
        _ => false,
    });
    outIsEmpty
}

pub fn prefixClockKind(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inClkKind: Arc<DAE::ClockKind>, mut inPrefix: DAE::Prefix) -> Result<(FCore::Cache, Arc<DAE::ClockKind>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClkKind: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    (outCache, outClkKind) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inClkKind.clone(), inPrefix.clone())) {
        (cache, _, _, Deref @ DAE::ClockKind::INFERRED_CLOCK, _) => {
            (cache.clone(), inClkKind.clone())
        },
        (cache, env, ih, Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e, resolution }, p) => {
            let mut clkKind: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
            let mut cache = (*cache).clone();
            let mut e = (*e).clone();
            let mut resolution = (*resolution).clone();
            (cache, e) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e.clone(), p.clone())?;
            (cache, resolution) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), resolution.clone(), p.clone())?;
            clkKind = Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e.clone(), resolution: resolution.clone() });
            (cache.clone(), clkKind.clone())
        },
        (cache, env, ih, Deref @ DAE::ClockKind::REAL_CLOCK { interval: e }, p) => {
            let mut clkKind: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
            let mut cache = (*cache).clone();
            let mut e = (*e).clone();
            (cache, e) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e.clone(), p.clone())?;
            clkKind = Arc::new(DAE::ClockKind::REAL_CLOCK { interval: e.clone() });
            (cache.clone(), clkKind.clone())
        },
        (cache, env, ih, Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e, startInterval: interval }, p) => {
            let mut clkKind: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
            let mut cache = (*cache).clone();
            let mut e = (*e).clone();
            let mut interval = (*interval).clone();
            (cache, e) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e.clone(), p.clone())?;
            (cache, interval) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), interval.clone(), p.clone())?;
            clkKind = Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: e.clone(), startInterval: interval.clone() });
            (cache.clone(), clkKind.clone())
        },
        (cache, env, ih, Deref @ DAE::ClockKind::SOLVER_CLOCK { c: e, solverMethod: method }, p) => {
            let mut clkKind: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
            let mut cache = (*cache).clone();
            let mut e = (*e).clone();
            let mut method = (*method).clone();
            (cache, e) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), e.clone(), p.clone())?;
            (cache, method) = prefixExpWork(cache.clone(), env.clone(), ih.clone(), method.clone(), p.clone())?;
            clkKind = Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: e.clone(), solverMethod: method.clone() });
            (cache.clone(), clkKind.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outClkKind))
}

pub fn getPrefixInfo(mut inPrefix: DAE::Prefix) -> SourceInfo {
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outInfo = (::match_deref::match_deref! { match &(inPrefix.clone()) {
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { info: outInfo, .. }, .. } => outInfo.clone(),
        _ => Absyn::dummyInfo.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInfo
}

pub fn prefixHashWork(mut inPrefix: Arc<DAE::ComponentPrefix>, mut hash: i32) -> i32 {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ DAE::ComponentPrefix::PRE { .. } => prefixHashWork(var_field!((*inPrefix).next, DAE::ComponentPrefix::PRE).clone(), 31 * hash.clone() + stringHashDjb2((var_field!((*inPrefix).prefix, DAE::ComponentPrefix::PRE).clone()).clone())),
        _ => hash.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hash
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn componentPrefixPathEqual(mut pre1: Arc<DAE::ComponentPrefix>, mut pre2: Arc<DAE::ComponentPrefix>) -> bool {
    let mut eq: bool = false;
    eq = (::match_deref::match_deref! { match &((pre1.clone(), pre2.clone())) {
        (Deref @ DAE::ComponentPrefix::PRE { .. }, Deref @ DAE::ComponentPrefix::PRE { .. }) => if (var_field!((*pre1).prefix, DAE::ComponentPrefix::PRE).clone() == var_field!((*pre2).prefix, DAE::ComponentPrefix::PRE).clone()) {componentPrefixPathEqual(var_field!((*pre1).next, DAE::ComponentPrefix::PRE).clone(), var_field!((*pre2).next, DAE::ComponentPrefix::PRE).clone())} else {false},
        (Deref @ DAE::ComponentPrefix::NOCOMPPRE, Deref @ DAE::ComponentPrefix::NOCOMPPRE) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eq
}

pub fn componentPrefix(mut inPrefix: DAE::Prefix) -> Arc<DAE::ComponentPrefix> {
    let mut outPrefix: Arc<DAE::ComponentPrefix> = Arc::new(DAE::ComponentPrefix::NOCOMPPRE);
    outPrefix = (match inPrefix.clone() {
        DAE::Prefix::PREFIX { .. } => var_field!(inPrefix.compPre, DAE::Prefix::PREFIX).clone(),
        _ => Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE),
    });
    outPrefix
}

pub fn writeComponentPrefix(mut file: File::File, mut pre: Arc<DAE::ComponentPrefix>, mut escape: File::Escape) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(pre.clone()) {
        Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, .. } => {
            File::writeEscape(file.clone(), (var_field!((*pre).prefix, DAE::ComponentPrefix::PRE).clone()).clone(), escape.clone());
            ComponentReference::writeSubscripts(file.clone(), var_field!((*pre).subscripts, DAE::ComponentPrefix::PRE).clone(), escape.clone())?;
            ()
        },
        Deref @ DAE::ComponentPrefix::PRE { .. } => {
            writeComponentPrefix(file.clone(), var_field!((*pre).next, DAE::ComponentPrefix::PRE).clone(), File::Escape::None.clone())?;
            File::writeEscape(file.clone(), (var_field!((*pre).prefix, DAE::ComponentPrefix::PRE).clone()).clone(), escape.clone());
            ComponentReference::writeSubscripts(file.clone(), var_field!((*pre).subscripts, DAE::ComponentPrefix::PRE).clone(), escape.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn hasSubs(mut pre: Arc<DAE::ComponentPrefix>) -> bool {
    let mut ob: bool = false;
    ob = (::match_deref::match_deref! { match &(pre.clone()) {
        Deref @ DAE::ComponentPrefix::PRE { subscripts: Deref @ metamodelica::List::Nil, .. } => hasSubs(var_field!((*pre).next, DAE::ComponentPrefix::PRE).clone()),
        Deref @ DAE::ComponentPrefix::PRE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ob
}

pub fn removeCompPrefixFromExps(mut inExp: Arc<DAE::Exp>, mut inCompPref: Arc<DAE::ComponentPrefix>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new({ let __pe_b2 = inCompPref.clone(); move |__pe_a0, __pe_a1| removeCompPrefixFromCrefExp(__pe_a0, __pe_a1, __pe_b2.clone()) }), false)?;
    Ok(outExp)
}

fn removeCompPrefixFromCrefExp(mut inExp: Arc<DAE::Exp>, mut inB: bool, mut inCompPref: Arc<DAE::ComponentPrefix>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut b: bool = false;
    (outExp, b) = (::match_deref::match_deref! { match &(inExp.clone()) {
        exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { .. }, .. } => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut exp = (*exp).clone();
            cref = removePrefixFromCref(var_field!((*exp).componentRef, DAE::Exp::CREF).clone(), inCompPref.clone())?;
            assign_variant_field!(exp => DAE::Exp::CREF; componentRef = cref.clone());
            (exp.clone(), true)
        },
        _ => {
            (inExp.clone(), inB.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, b))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removePrefixFromCref(mut inCref: Arc<DAE::ComponentRef>, mut inCompPref: Arc<DAE::ComponentPrefix>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &((inCref.clone(), inCompPref.clone())) {
        (_, Deref @ DAE::ComponentPrefix::NOCOMPPRE) => {
            inCref.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _) => {
            inCref.clone()
        },
        (cref @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }, pref @ Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE, .. }) => {
            if stringEqual((var_field!((**cref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((**pref).prefix, DAE::ComponentPrefix::PRE).clone()).clone()) {
            }
            var_field!((**cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }, pref @ Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::PRE { prefix: _, .. }, .. }) => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut pref = (*pref).clone();
            cref = removePrefixFromCref(inCref.clone(), var_field!((*pref).next, DAE::ComponentPrefix::PRE).clone())?;
            assign_variant_field!(pref => DAE::ComponentPrefix::PRE; next = Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE));
            cref = removePrefixFromCref(cref.clone(), pref.clone())?;
            cref.clone()
        },
        (_, Deref @ DAE::ComponentPrefix::PRE { .. }) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PrefixUtil.removePrefixFromCref")); __mm_s.push_str(&*literal!(" :Cref is not qualified but we have prefix to remove: ")); __mm_s.push_str(&*ComponentReference::crefStr(inCref.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PrefixUtil.removePrefixFromCref")); __mm_s.push_str(&*literal!(" :failed on cref: ")); __mm_s.push_str(&*ComponentReference::crefStr(inCref.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

