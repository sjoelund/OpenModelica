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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prefix {
    EMPTY_PREFIX {
        /// The path of the class the prefix originates from.
        classPath: Option<Arc<Absyn::Path>>,
    },
    PREFIX {
        name: ArcStr,
        dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>,
        restPrefix: Arc<Prefix>,
    },
}
pub use self::Prefix::{EMPTY_PREFIX,PREFIX};

pub static emptyPrefix: std::sync::LazyLock<Arc<Prefix>> = std::sync::LazyLock::new(|| { Arc::new(Prefix::EMPTY_PREFIX { classPath: None }) });

pub static functionPrefix: std::sync::LazyLock<Arc<Prefix>> = std::sync::LazyLock::new(|| { Arc::new(Prefix::EMPTY_PREFIX { classPath: None }) });

pub fn makePrefix(mut inName: ArcStr, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::PREFIX { name: (inName.clone()).clone(), dims: inDims.clone(), restPrefix: emptyPrefix.clone() });
    outPrefix
}

pub fn makeEmptyPrefix(mut inClassPath: Arc<Absyn::Path>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::EMPTY_PREFIX { classPath: Some(inClassPath.clone()) });
    outPrefix
}

pub fn add(mut inName: ArcStr, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::PREFIX { name: (inName.clone()).clone(), dims: inDimensions.clone(), restPrefix: inPrefix.clone() });
    outPrefix
}

pub fn addPath(mut inPath: Arc<Absyn::Path>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromPath2(inPath.clone(), inPrefix.clone())?;
    Ok(outPrefix)
}

pub fn addOptPath(mut inOptPath: Option<Arc<Absyn::Path>>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = (::match_deref::match_deref! { match &((inOptPath.clone(), inPrefix.clone())) {
        (None, _) => {
            inPrefix.clone()
        },
        (Some(p), _) => {
            addPath(p.clone(), inPrefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefix)
}

pub fn addString(mut inName: ArcStr, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = Arc::new(Prefix::PREFIX { name: (inName.clone()).clone(), dims: metamodelica::nil(), restPrefix: inPrefix.clone() });
    outPrefix
}

pub fn addStringList(mut inStrings: Arc<metamodelica::List<ArcStr>>, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromStringList2(inStrings.clone(), inPrefix.clone());
    outPrefix
}

pub fn restPrefix(mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outRestPrefix: Arc<Prefix>;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::PREFIX { restPrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRestPrefix = __pa0.clone();
    Ok(outRestPrefix)
}

pub fn firstName(mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            literal!("")
        },
        Deref @ Prefix::PREFIX { name, .. } => {
            name.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outStr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn prefixCref(mut inCref: Arc<DAE::ComponentRef>, mut inPrefix: Arc<Prefix>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &((inCref.clone(), inPrefix.clone())) {
        (_, Deref @ Prefix::EMPTY_PREFIX { .. }) => {
            inCref.clone()
        },
        (_, Deref @ Prefix::PREFIX { restPrefix: rest_prefix, name, .. }) => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT.clone(), subscriptLst: metamodelica::nil(), componentRef: inCref.clone() });
            prefixCref(cref.clone(), rest_prefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn prefixPath(mut inPath: Arc<Absyn::Path>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath.clone(), inPrefix.clone())) {
        (_, Deref @ Prefix::EMPTY_PREFIX { .. }) => {
            inPath.clone()
        },
        (_, Deref @ Prefix::PREFIX { restPrefix: rest_prefix, name, .. }) => {
            let mut path: Arc<Absyn::Path>;
            path = Arc::new(Absyn::Path::QUALIFIED { name: (name.clone()).clone(), path: inPath.clone() });
            prefixPath(path.clone(), rest_prefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn prefixStr(mut inString: ArcStr, mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inString.clone(), inPrefix.clone())) {
        (_, Deref @ Prefix::EMPTY_PREFIX { .. }) => {
            inString.clone()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (toStr(inPrefix.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*inString.clone()); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn toCref(mut inPrefix: Arc<Prefix>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::PREFIX { restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, name, .. } => {
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT.clone(), subscriptLst: metamodelica::nil() })
        },
        Deref @ Prefix::PREFIX { restPrefix: rest_prefix, name, .. } => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cref = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: DAE::T_UNKNOWN_DEFAULT.clone(), subscriptLst: metamodelica::nil() });
            prefixCref(cref.clone(), rest_prefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn toPath(mut inPrefix: Arc<Prefix>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::PREFIX { restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, name, .. } => {
            Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() })
        },
        Deref @ Prefix::PREFIX { restPrefix: rest_prefix, name, .. } => {
            let mut path: Arc<Absyn::Path>;
            path = Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() });
            prefixPath(path.clone(), rest_prefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn fromPath(mut inPath: Arc<Absyn::Path>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromPath2(inPath.clone(), emptyPrefix.clone())?;
    Ok(outPrefix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fromPath2(mut inPath: Arc<Absyn::Path>, mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = (::match_deref::match_deref! { match &((inPath.clone(), inPrefix.clone())) {
        (Deref @ Absyn::Path::QUALIFIED { name, path }, _) => {
            fromPath2(path.clone(), Arc::new(Prefix::PREFIX { name: (name.clone()).clone(), dims: metamodelica::nil(), restPrefix: inPrefix.clone() }))?
        },
        (Deref @ Absyn::Path::IDENT { name }, _) => {
            Arc::new(Prefix::PREFIX { name: (name.clone()).clone(), dims: metamodelica::nil(), restPrefix: inPrefix.clone() })
        },
        (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _) => {
            fromPath2(path.clone(), inPrefix.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefix)
}

pub fn fromStringList(mut inStrings: Arc<metamodelica::List<ArcStr>>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = fromStringList2(inStrings.clone(), emptyPrefix.clone());
    outPrefix
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fromStringList2(mut inStrings: Arc<metamodelica::List<ArcStr>>, mut inPrefix: Arc<Prefix>) -> Arc<Prefix> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = (::match_deref::match_deref! { match &((inStrings.clone(), inPrefix.clone())) {
        (Deref @ metamodelica::List::Cons { head: r#str, tail: strl }, _) => {
            fromStringList2(strl.clone(), Arc::new(Prefix::PREFIX { name: (r#str.clone()).clone(), dims: metamodelica::nil(), restPrefix: inPrefix.clone() }))
        },
        _ => {
            inPrefix.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPrefix
}

pub fn toStr(mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            literal!("")
        },
        Deref @ Prefix::PREFIX { restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, name, .. } => {
            name.clone()
        },
        Deref @ Prefix::PREFIX { restPrefix: rest_prefix, name, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toStr(rest_prefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outStr)
}

pub fn toStrWithEmpty(mut inPrefix: Arc<Prefix>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::EMPTY_PREFIX { classPath: None } => {
            literal!("E()")
        },
        Deref @ Prefix::EMPTY_PREFIX { classPath: Some(path) } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E(")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(path.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ Prefix::PREFIX { restPrefix: rest_prefix, name, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toStrWithEmpty(rest_prefix.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outStr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isPackagePrefix(mut inPrefix: Arc<Prefix>) -> bool {
    let mut outIsPackagePrefix: bool = false;
    outIsPackagePrefix = (::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::PREFIX { restPrefix: prefix, .. } => {
            isPackagePrefix(prefix.clone())
        },
        Deref @ Prefix::EMPTY_PREFIX { classPath: None } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsPackagePrefix
}

pub fn toPackagePrefix(mut inPrefix: Arc<Prefix>) -> Result<Arc<Prefix>> {
    let mut outPrefix: Arc<Prefix>;
    outPrefix = (::match_deref::match_deref! { match &(inPrefix.clone()) {
        Deref @ Prefix::PREFIX { name, dims, restPrefix: rest_prefix } => {
            let mut rest_prefix = (*rest_prefix).clone();
            rest_prefix = toPackagePrefix(rest_prefix.clone())?;
            Arc::new(Prefix::PREFIX { name: (name.clone()).clone(), dims: dims.clone(), restPrefix: rest_prefix.clone() })
        },
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            Arc::new(Prefix::EMPTY_PREFIX { classPath: None })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefix)
}

