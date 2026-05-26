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

use crate::StringUtil;
use crate::System;
use crate::Util;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Version {
    /// Semantic version number MAJOR.MINOR.PATCH, see https://semver.org/.
    SEMVER {
        major: i32,
        minor: i32,
        patch: i32,
        prerelease: Arc<metamodelica::List<ArcStr>>,
        meta: Arc<metamodelica::List<ArcStr>>,
    },
    /// Non-semantic version number
    NONSEMVER {
        version: ArcStr,
    },
}
impl Default for Version {
    fn default() -> Self {
        Self::NONSEMVER {
            version: Default::default(),
        }
    }
}
pub use self::Version::{SEMVER,NONSEMVER};

pub fn parse(mut s: ArcStr, mut nonsemverAsZeroZeroZero: bool) -> Result<Version> {
    let mut v: Version;
    let mut n: i32 = 0;
    let mut major: ArcStr = arcstr::literal!("");
    let mut minor: ArcStr = arcstr::literal!("");
    let mut patch: ArcStr = arcstr::literal!("");
    let mut nextString: ArcStr = arcstr::literal!("");
    let mut versions: ArcStr = arcstr::literal!("");
    let mut prereleaseLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut metaLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut matches: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut split: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut versionsLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let semverRegex: ArcStr = literal!("^([0-9][0-9]*\\.?[0-9]*\\.?[0-9]*)([+-][0-9A-Za-z.-]*)?$");
    (n, matches) = System::regex((s.clone()).clone(), (semverRegex.clone()).clone(), 5, true, false);
    if n.clone() < 2 {
        if ((s.clone()).clone().len() as i32) == 0 {
            v = Version::NONSEMVER { version: (literal!("")).clone() };
            return Ok(v);
        }
        if nonsemverAsZeroZeroZero.clone() {
            (prereleaseLst, metaLst) = splitPrereleaseAndMeta((s.clone()).clone())?;
            v = Version::SEMVER { major: 0, minor: 0, patch: 0, prerelease: prereleaseLst.clone(), meta: metaLst.clone() };
        } else {
            v = Version::NONSEMVER { version: (s.clone()).clone() };
        }
        return Ok(v);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matches.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    versions = __pa0.clone();
    split = __pa1.clone();
    versionsLst = Util::stringSplitAtChar((versions.clone()).clone(), (literal!(".")).clone())?;
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(versionsLst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    major = __pa3.clone();
    versionsLst = __pa4.clone();
    if !(versionsLst.clone().is_empty()) {
        let (__pa5, __pa6) = ::match_deref::match_deref! { match &(versionsLst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        minor = __pa5.clone();
        versionsLst = __pa6.clone();
    } else {
        minor = (literal!("0")).clone();
    }
    if !(versionsLst.clone().is_empty()) {
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(versionsLst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa7, tail: __pa8 } => (__pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        patch = __pa7.clone();
        versionsLst = __pa8.clone();
    } else {
        patch = (literal!("0")).clone();
    }
    (prereleaseLst, metaLst) = splitPrereleaseAndMeta((if (split.clone().is_empty()) {literal!("")} else {(split.clone()).get(1)?}).clone())?;
    v = Version::SEMVER { major: stringInt((major.clone()).clone())?, minor: stringInt((minor.clone()).clone())?, patch: stringInt((patch.clone()).clone())?, prerelease: prereleaseLst.clone(), meta: metaLst.clone() };
    Ok(v)
}

pub fn compare(mut v1: Version, mut v2: Version, mut comparePrerelease: bool, mut compareBuildInformation: bool) -> Result<i32> {
    let mut c: i32 = 0;
    c = (match (v1.clone(), v2.clone()) {
        (Version::NONSEMVER { .. }, Version::NONSEMVER { .. }) => stringCompare((var_field!(v1.version, Version::NONSEMVER).clone()).clone(), (var_field!(v2.version, Version::NONSEMVER).clone()).clone()),
        (Version::NONSEMVER { .. }, _) => -1,
        (_, Version::NONSEMVER { .. }) => 1,
        (Version::SEMVER { .. }, Version::SEMVER { .. }) => {
            if var_field!(v1.major, Version::SEMVER).clone() == 0 && var_field!(v1.minor, Version::SEMVER).clone() == 0 && var_field!(v1.patch, Version::SEMVER).clone() == 0 || var_field!(v2.major, Version::SEMVER).clone() == 0 && var_field!(v2.minor, Version::SEMVER).clone() == 0 && var_field!(v2.patch, Version::SEMVER).clone() == 0 {
                c = 0;
            } else {
                c = Util::intCompare(var_field!(v1.major, Version::SEMVER).clone(), var_field!(v2.major, Version::SEMVER).clone());
                if c.clone() != 0 {
                    return Ok(c);
                }
                c = Util::intCompare(var_field!(v1.minor, Version::SEMVER).clone(), var_field!(v2.minor, Version::SEMVER).clone());
                if c.clone() != 0 {
                    return Ok(c);
                }
                c = Util::intCompare(var_field!(v1.patch, Version::SEMVER).clone(), var_field!(v2.patch, Version::SEMVER).clone());
                if c.clone() != 0 {
                    return Ok(c);
                }
            }
            if comparePrerelease.clone() {
                c = compareIdentifierList(var_field!(v1.prerelease, Version::SEMVER).clone(), var_field!(v2.prerelease, Version::SEMVER).clone())?;
            }
            if c.clone() == 0 && compareBuildInformation.clone() {
                c = compareIdentifierList(var_field!(v1.meta, Version::SEMVER).clone(), var_field!(v2.meta, Version::SEMVER).clone())?;
            }
            c.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(c)
}

pub fn toString(mut v: Version) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((match v.clone() {
        Version::SEMVER { .. } => {
            out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", var_field!(v.major, Version::SEMVER).clone()))); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", var_field!(v.minor, Version::SEMVER).clone()))); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", var_field!(v.patch, Version::SEMVER).clone()))); ArcStr::from(__mm_s) }).clone();
            if !(var_field!(v.prerelease, Version::SEMVER).clone().is_empty()) {
                out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*out.clone()); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*stringDelimitList(var_field!(v.prerelease, Version::SEMVER).clone(), (literal!(".")).clone())); ArcStr::from(__mm_s) }).clone();
            }
            if !(var_field!(v.meta, Version::SEMVER).clone().is_empty()) {
                out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*out.clone()); __mm_s.push_str(&*literal!("+")); __mm_s.push_str(&*stringDelimitList(var_field!(v.meta, Version::SEMVER).clone(), (literal!(".")).clone())); ArcStr::from(__mm_s) }).clone();
            }
            out.clone()
        },
        Version::NONSEMVER { .. } => var_field!(v.version, Version::NONSEMVER).clone(),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(out)
}

pub fn isPrerelease(mut v: Version) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(v.clone()) {
        Version::SEMVER { prerelease: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hasMetaInformation(mut v: Version) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(v.clone()) {
        Version::SEMVER { meta: Deref @ metamodelica::List::Nil, .. } => false,
        Version::NONSEMVER { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isSemVer(mut v: Version) -> bool {
    let mut b: bool = false;
    b = (match v.clone() {
        Version::SEMVER { .. } => true,
        _ => false,
    });
    b
}

fn splitPrereleaseAndMeta(mut s: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut prereleaseLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut metaLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut meta: ArcStr = arcstr::literal!("");
    let mut prerelease: ArcStr = arcstr::literal!("");
    let mut split: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    prereleaseLst = metamodelica::nil();
    metaLst = metamodelica::nil();
    if stringEmpty((s.clone()).clone()) {
        return Ok((prereleaseLst, metaLst));
    }
    if stringGetStringChar((s.clone()).clone(), 1)? == literal!("+") {
        metaLst = if (((s.clone()).clone().len() as i32) > 1) {Util::stringSplitAtChar((StringUtil::rest((s.clone()).clone())).clone(), (literal!(".")).clone())?} else {metamodelica::nil()};
        return Ok((prereleaseLst, metaLst));
    }
    split = Util::stringSplitAtChar((s.clone()).clone(), (literal!("+")).clone())?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(split.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    prerelease = __pa0.clone();
    split = __pa1.clone();
    meta = (if (split.clone().is_empty()) {literal!("")} else {(split.clone()).get(1)?}).clone();
    if stringGetStringChar((prerelease.clone()).clone(), 1)? == literal!("-") {
        prerelease = (StringUtil::rest((prerelease.clone()).clone())).clone();
    }
    prereleaseLst = if (((prerelease.clone()).clone().len() as i32) > 0) {Util::stringSplitAtChar((prerelease.clone()).clone(), (literal!(".")).clone())?} else {metamodelica::nil()};
    metaLst = if (((meta.clone()).clone().len() as i32) > 0) {Util::stringSplitAtChar((meta.clone()).clone(), (literal!(".")).clone())?} else {metamodelica::nil()};
    Ok((prereleaseLst, metaLst))
}

fn compareIdentifierList(mut w1: Arc<metamodelica::List<ArcStr>>, mut w2: Arc<metamodelica::List<ArcStr>>) -> Result<i32> {
    let mut c: i32 = 0;
    let mut l1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut l2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    l1 = w1.clone();
    l2 = w2.clone();
    if l1.clone().is_empty() && !(l2.clone().is_empty()) {
        c = 1;
    }
    if l2.clone().is_empty() && !(l1.clone().is_empty()) {
        c = -1;
    }
    while !(l1.clone().is_empty() && l2.clone().is_empty()) {
        (c, l1, l2) = (::match_deref::match_deref! { match &((l1.clone(), l2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (-1, l1.clone(), l2.clone()),
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Nil) => (1, l1.clone(), l2.clone()),
        (Deref @ metamodelica::List::Cons { head: s1, tail: l1 }, Deref @ metamodelica::List::Cons { head: s2, tail: l2 }) => (compareIdentifier((s1.clone()).clone(), (s2.clone()).clone())?, l1.clone(), l2.clone()),
        _ => bail!("match: no arm matched"),
    } });
        if c.clone() != 0 {
            return Ok(c);
        }
    }
    c = 0;
    Ok(c)
}

fn compareIdentifier(mut s1: ArcStr, mut s2: ArcStr) -> Result<i32> {
    let mut c: i32 = 0;
    if Util::isIntegerString((s1.clone()).clone()) {
        c = if (Util::isIntegerString((s2.clone()).clone())) {Util::intCompare(stringInt((s1.clone()).clone())?, stringInt((s2.clone()).clone())?)} else {-1};
        return Ok(c);
    }
    if Util::isIntegerString((s2.clone()).clone()) {
        c = 1;
    }
    c = stringCompare((s1.clone()).clone(), (s2.clone()).clone());
    Ok(c)
}

