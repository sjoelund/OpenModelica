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

use crate::ExpressionBasics;
use crate::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub fn crefDims(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    outDimensionLst = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: idType, .. } => {
            TypesDump::getDimensions(idType.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: idType, componentRef: cr, .. } => {
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            dims = TypesDump::getDimensions(idType.clone());
            res = crefDims(cr.clone())?;
            res = listAppend(dims.clone(), res.clone());
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDimensionLst)
}

pub fn crefSubs(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    outSubscriptLst = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. } => {
            subs.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: subs, componentRef: cr, .. } => {
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            res = crefSubs(cr.clone())?;
            res = listAppend(subs.clone(), res.clone());
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscriptLst)
}

/* **************************************************/
/* Compare  */
/* **************************************************/
pub fn crefLastIdentEqual(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut equal: bool = false;
    let mut id1: ArcStr = arcstr::literal!("");
    let mut id2: ArcStr = arcstr::literal!("");
    id1 = (crefLastIdent(cr1.clone())?).clone();
    id2 = (crefLastIdent(cr2.clone())?).clone();
    equal = stringEq((id1.clone()).clone(), (id2.clone()).clone());
    Ok(equal)
}

pub fn crefFirstCrefEqual(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut equal: bool = false;
    let mut pcr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut pcr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    pcr1 = crefFirstCref(cr1.clone())?;
    pcr2 = crefFirstCref(cr2.clone())?;
    equal = crefEqual(pcr1.clone(), pcr2.clone())?;
    Ok(equal)
}

pub fn crefFirstCrefLastCrefEqual(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut equal: bool = false;
    let mut pcr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut pcr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    pcr1 = crefFirstCref(cr1.clone())?;
    pcr2 = crefLastCref(cr2.clone())?;
    equal = crefEqual(pcr1.clone(), pcr2.clone())?;
    Ok(equal)
}

pub fn crefFirstCref(mut inCr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCr = (::match_deref::match_deref! { match &(inCr.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: t2, subscriptLst: subs, componentRef: _ } => {
            makeCrefIdent((id.clone()).clone(), t2.clone(), subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } => {
            inCr.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefLastIdent(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outIdent: ArcStr = arcstr::literal!("");
    outIdent = ((::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. } => {
            id.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (crefLastIdent(cr.clone())?).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefLastCref(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            inComponentRef.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
            let mut res: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            res = crefLastCref(cr.clone())?;
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn crefFirstIdentEqual(mut inCref1: Arc<DAE::ComponentRef>, mut inCref2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outEqual: bool = false;
    let mut id1: ArcStr = arcstr::literal!("");
    let mut id2: ArcStr = arcstr::literal!("");
    id1 = (crefFirstIdent(inCref1.clone())?).clone();
    id2 = (crefFirstIdent(inCref2.clone())?).clone();
    outEqual = stringEq((id1.clone()).clone(), (id2.clone()).clone());
    Ok(outEqual)
}

pub fn crefFirstIdent(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outIdent: ArcStr = arcstr::literal!("");
    outIdent = ((::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. } => {
            id.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, .. } => {
            id.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum CompareWithSubsType {
    WithoutSubscripts = 1,
    WithGenericSubscript = 2,
    WithGenericSubscriptNotAlphabetic = 3,
    WithIntSubscript = 4,
}
impl PartialOrd for CompareWithSubsType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for CompareWithSubsType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub mod CompareWithGenericSubscript {
    use super::*;
    pub static compareSubscript: std::sync::LazyLock<CompareWithSubsType> = std::sync::LazyLock::new(|| { CompareWithSubsType::WithGenericSubscript.clone() });

    pub fn compare(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
        let mut res: i32 = 0;
        res = (::match_deref::match_deref! { match &((cr1.clone(), cr2.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res.clone() != 0 {
                return Ok(res);
            }
            compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                if res.clone() != 0 {
                    return Ok(res);
                }
            }
            compare(var_field!((*cr1).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).componentRef, DAE::ComponentRef::CREF_QUAL).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            1
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            -1
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    pub fn compareSubs(mut ss1: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut ss2: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>> = ss2.clone();
        let mut s2: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        let mut e1: Arc<DAE::Exp>;
        let mut e2: Arc<DAE::Exp>;
        for mut s1 in &*ss1.clone() {
            let mut s1 = s1.clone();
            if ss.clone().is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            s2 = __pa0.clone();
            ss = __pa1.clone();
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = stringCompare((ExpressionBasics::printSubscriptStr(s1.clone())?).clone(), (ExpressionBasics::printSubscriptStr(s2.clone())?).clone());
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(s1.clone(), s2.clone())?;
            } else {
                i1 = ExpressionBasics::subscriptInt(s1.clone());
                i2 = ExpressionBasics::subscriptInt(s2.clone());
                res = if (i1.clone() < i2.clone()) {-1} else if (i1.clone() > i2.clone()) {1} else {0};
            }
            if res.clone() != 0 {
                return Ok(res);
            }
        }
        if !(ss.clone().is_empty()) {
            res = 1;
        }
        Ok(res)
    }

}

pub mod CompareWithGenericSubscriptNotAlphabetic {
    use super::*;
    pub fn compare(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
        let mut res: i32 = 0;
        res = (::match_deref::match_deref! { match &((cr1.clone(), cr2.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res.clone() != 0 {
                return Ok(res);
            }
            compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                if res.clone() != 0 {
                    return Ok(res);
                }
            }
            compare(var_field!((*cr1).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).componentRef, DAE::ComponentRef::CREF_QUAL).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            1
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            -1
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    pub fn compareSubs(mut ss1: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut ss2: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>> = ss2.clone();
        let mut s2: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        let mut e1: Arc<DAE::Exp>;
        let mut e2: Arc<DAE::Exp>;
        for mut s1 in &*ss1.clone() {
            let mut s1 = s1.clone();
            if ss.clone().is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            s2 = __pa0.clone();
            ss = __pa1.clone();
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = stringCompare((ExpressionBasics::printSubscriptStr(s1.clone())?).clone(), (ExpressionBasics::printSubscriptStr(s2.clone())?).clone());
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(s1.clone(), s2.clone())?;
            } else {
                i1 = ExpressionBasics::subscriptInt(s1.clone());
                i2 = ExpressionBasics::subscriptInt(s2.clone());
                res = if (i1.clone() < i2.clone()) {-1} else if (i1.clone() > i2.clone()) {1} else {0};
            }
            if res.clone() != 0 {
                return Ok(res);
            }
        }
        if !(ss.clone().is_empty()) {
            res = 1;
        }
        Ok(res)
    }

    pub static compareSubscript: std::sync::LazyLock<CompareWithSubsType> = std::sync::LazyLock::new(|| { CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() });

}

pub mod CompareWithoutSubscripts {
    use super::*;
    pub fn compare(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
        let mut res: i32 = 0;
        res = (::match_deref::match_deref! { match &((cr1.clone(), cr2.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res.clone() != 0 {
                return Ok(res);
            }
            compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                if res.clone() != 0 {
                    return Ok(res);
                }
            }
            compare(var_field!((*cr1).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).componentRef, DAE::ComponentRef::CREF_QUAL).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            1
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            -1
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    pub fn compareSubs(mut ss1: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut ss2: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>> = ss2.clone();
        let mut s2: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        let mut e1: Arc<DAE::Exp>;
        let mut e2: Arc<DAE::Exp>;
        for mut s1 in &*ss1.clone() {
            let mut s1 = s1.clone();
            if ss.clone().is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            s2 = __pa0.clone();
            ss = __pa1.clone();
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = stringCompare((ExpressionBasics::printSubscriptStr(s1.clone())?).clone(), (ExpressionBasics::printSubscriptStr(s2.clone())?).clone());
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(s1.clone(), s2.clone())?;
            } else {
                i1 = ExpressionBasics::subscriptInt(s1.clone());
                i2 = ExpressionBasics::subscriptInt(s2.clone());
                res = if (i1.clone() < i2.clone()) {-1} else if (i1.clone() > i2.clone()) {1} else {0};
            }
            if res.clone() != 0 {
                return Ok(res);
            }
        }
        if !(ss.clone().is_empty()) {
            res = 1;
        }
        Ok(res)
    }

    pub static compareSubscript: std::sync::LazyLock<CompareWithSubsType> = std::sync::LazyLock::new(|| { CompareWithSubsType::WithoutSubscripts.clone() });

}

pub mod CompareWithIntSubscript {
    use super::*;
    pub fn compare(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
        let mut res: i32 = 0;
        res = (::match_deref::match_deref! { match &((cr1.clone(), cr2.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res.clone() != 0 {
                return Ok(res);
            }
            compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                if res.clone() != 0 {
                    return Ok(res);
                }
            }
            compare(var_field!((*cr1).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).componentRef, DAE::ComponentRef::CREF_QUAL).clone())?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            1
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            res = stringCompare((var_field!((*cr1).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone());
            if res.clone() != 0 {
                return Ok(res);
            }
            if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                res = compareSubs(var_field!((*cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
            }
            if res.clone() != 0 {
                return Ok(res);
            }
            -1
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    pub fn compareSubs(mut ss1: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut ss2: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: Arc<metamodelica::List<Arc<DAE::Subscript>>> = ss2.clone();
        let mut s2: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        let mut e1: Arc<DAE::Exp>;
        let mut e2: Arc<DAE::Exp>;
        for mut s1 in &*ss1.clone() {
            let mut s1 = s1.clone();
            if ss.clone().is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            s2 = __pa0.clone();
            ss = __pa1.clone();
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = stringCompare((ExpressionBasics::printSubscriptStr(s1.clone())?).clone(), (ExpressionBasics::printSubscriptStr(s2.clone())?).clone());
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(s1.clone(), s2.clone())?;
            } else {
                i1 = ExpressionBasics::subscriptInt(s1.clone());
                i2 = ExpressionBasics::subscriptInt(s2.clone());
                res = if (i1.clone() < i2.clone()) {-1} else if (i1.clone() > i2.clone()) {1} else {0};
            }
            if res.clone() != 0 {
                return Ok(res);
            }
        }
        if !(ss.clone().is_empty()) {
            res = 1;
        }
        Ok(res)
    }

    pub static compareSubscript: std::sync::LazyLock<CompareWithSubsType> = std::sync::LazyLock::new(|| { CompareWithSubsType::WithIntSubscript.clone() });

}

pub fn crefSortFunc(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut greaterThan: bool = false;
    greaterThan = CompareWithGenericSubscript::compare(cr1.clone(), cr2.clone())? > 0;
    Ok(greaterThan)
}

pub fn crefCompareGeneric(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
    let mut comp: i32 = 0;
    comp = CompareWithGenericSubscript::compare(cr1.clone(), cr2.clone())?;
    Ok(comp)
}

pub fn crefCompareIntSubscript(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
    let mut comp: i32 = 0;
    comp = CompareWithIntSubscript::compare(cr1.clone(), cr2.clone())?;
    Ok(comp)
}

pub fn crefCompareGenericNotAlphabetic(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
    let mut comp: i32 = 0;
    comp = CompareWithGenericSubscriptNotAlphabetic::compare(cr1.clone(), cr2.clone())?;
    Ok(comp)
}

pub fn crefLexicalGreaterSubsAtEnd(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut isGreater: bool = false;
    isGreater = crefLexicalCompareSubsAtEnd(cr1.clone(), cr2.clone())? > 0;
    Ok(isGreater)
}

pub fn crefLexicalCompareSubsAtEnd(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<i32> {
    let mut res: i32 = 0;
    let mut subs1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    res = CompareWithoutSubscripts::compare(cr1.clone(), cr2.clone())?;
    if res.clone() != 0 {
        return Ok(res);
    }
    subs1 = ExpressionBasics::subscriptsInt(crefSubs(cr1.clone())?);
    subs2 = ExpressionBasics::subscriptsInt(crefSubs(cr2.clone())?);
    res = crefLexicalCompareSubsAtEnd2(subs1.clone(), subs2.clone())?;
    Ok(res)
}

fn crefLexicalCompareSubsAtEnd2(mut inSubs1: Arc<metamodelica::List<i32>>, mut inSubs2: Arc<metamodelica::List<i32>>) -> Result<i32> {
    let mut res: i32 = 0;
    let mut rest: Arc<metamodelica::List<i32>> = inSubs2.clone();
    for mut i in &*inSubs1.clone() {
        let mut i = i.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        res = __pa0.clone();
        rest = __pa1.clone();
        res = if (i.clone() > res.clone()) {1} else if (i.clone() < res.clone()) {-1} else {0};
        if res.clone() != 0 {
            return Ok(res);
        }
    }
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefContainedIn(mut containerCref: Arc<DAE::ComponentRef>, mut containedCref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (containerCref.clone(), containedCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (full, partOf) => {
                    let true = (crefEqualNoStringCompare(full.clone(), partOf.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (full @ Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr2, .. }, partOf) => {
                    let mut res: bool = false;
                    let false = (crefEqualNoStringCompare(full.clone(), partOf.clone())?) else { bail!("pattern mismatch") };
                    res = crefContainedIn(cr2.clone(), partOf.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBoolean)
}

pub fn crefPrefixOf(mut prefixCref: Arc<DAE::ComponentRef>, mut fullCref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outPrefixOf: bool = false;
    outPrefixOf = (::match_deref::match_deref! { match &((prefixCref.clone(), fullCref.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_QUAL).clone() == var_field!((*fullCref).ident, DAE::ComponentRef::CREF_QUAL).clone() && ExpressionBasics::subscriptEqual(var_field!((*prefixCref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*fullCref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())? && crefPrefixOf(var_field!((*prefixCref).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*fullCref).componentRef, DAE::ComponentRef::CREF_QUAL).clone())?,
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*fullCref).ident, DAE::ComponentRef::CREF_QUAL).clone(),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*fullCref).ident, DAE::ComponentRef::CREF_QUAL).clone() && ExpressionBasics::subscriptEqual(var_field!((*prefixCref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*fullCref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?,
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => stringEq((var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*fullCref).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone()),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*fullCref).ident, DAE::ComponentRef::CREF_IDENT).clone() && ExpressionBasics::subscriptEqual(var_field!((*prefixCref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*fullCref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPrefixOf)
}

pub fn crefPrefixOfIgnoreSubscripts(mut prefixCref: Arc<DAE::ComponentRef>, mut fullCref: Arc<DAE::ComponentRef>) -> bool {
    let mut outPrefixOf: bool = false;
    outPrefixOf = (::match_deref::match_deref! { match &((prefixCref.clone(), fullCref.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_QUAL).clone() == var_field!((*fullCref).ident, DAE::ComponentRef::CREF_QUAL).clone() && crefPrefixOfIgnoreSubscripts(var_field!((*prefixCref).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*fullCref).componentRef, DAE::ComponentRef::CREF_QUAL).clone()),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*fullCref).ident, DAE::ComponentRef::CREF_QUAL).clone(),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => var_field!((*prefixCref).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*fullCref).ident, DAE::ComponentRef::CREF_IDENT).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPrefixOf
}

pub fn crefNotPrefixOf(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((cr1.clone(), cr2.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => true,
        _ => !(crefPrefixOf(cr1.clone(), cr2.clone())?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn crefEqual(mut inComponentRef1: Arc<DAE::ComponentRef>, mut inComponentRef2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = crefEqualNoStringCompare(inComponentRef1.clone(), inComponentRef2.clone())?;
    Ok(outBoolean)
}

pub fn crefInLst(mut cref: Arc<DAE::ComponentRef>, mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> bool {
    let mut b: bool = false;
    b = List::isMemberOnTrue(cref.clone(), lst.clone(), Arc::new(crefEqual));
    b
}

pub fn crefNotInLst(mut cref: Arc<DAE::ComponentRef>, mut lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> bool {
    let mut b: bool = false;
    b = !(List::isMemberOnTrue(cref.clone(), lst.clone(), Arc::new(crefEqual)));
    b
}

pub fn crefEqualVerySlowStringCompareDoNotUse(mut inComponentRef1: Arc<DAE::ComponentRef>, mut inComponentRef2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = (inComponentRef1.clone(), inComponentRef2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (referenceEq(&inComponentRef1.clone(),&inComponentRef2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: n2, .. }) => {
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: idx1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: idx2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, ident: n2, .. }) => {
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::subscriptEqual(idx1.clone(), idx2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: idx2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, ident: n2, .. }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let 0 = (System::stringFind((n1.clone()).clone(), (n2.clone()).clone())?) else { bail!("pattern mismatch") };
                    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*n2.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionBasics::printListStr(idx2.clone(), Arc::new(ExpressionBasics::printSubscriptStr), (literal!(",")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    let true = (stringEq((s1.clone()).clone(), (n1.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: idx2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: n2, .. }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let 0 = (System::stringFind((n2.clone()).clone(), (n1.clone()).clone())?) else { bail!("pattern mismatch") };
                    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*n1.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionBasics::printListStr(idx2.clone(), Arc::new(ExpressionBasics::printSubscriptStr), (literal!(",")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    let true = (stringEq((s1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr1, subscriptLst: idx1, ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr2, subscriptLst: idx2, ident: n2, .. }) => {
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (crefEqualVerySlowStringCompareDoNotUse(cr1.clone(), cr2.clone())?) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::subscriptEqual(idx1.clone(), idx2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr1 @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: n1, .. }, cr2 @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, .. }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let 0 = (System::stringFind((n2.clone()).clone(), (n1.clone()).clone())?) else { bail!("pattern mismatch") };
                    s1 = (printComponentRefStr(cr1.clone())?).clone();
                    s2 = (printComponentRefStr(cr2.clone())?).clone();
                    let true = (stringEq((s1.clone()).clone(), (s2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr1 @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, .. }, cr2 @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: n2, .. }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let 0 = (System::stringFind((n1.clone()).clone(), (n2.clone()).clone())?) else { bail!("pattern mismatch") };
                    s1 = (printComponentRefStr(cr1.clone())?).clone();
                    s2 = (printComponentRefStr(cr2.clone())?).clone();
                    let true = (stringEq((s1.clone()).clone(), (s2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBoolean)
}

pub fn crefEqualNoStringCompare(mut inCref1: Arc<DAE::ComponentRef>, mut inCref2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut outEqual: bool = false;
    if referenceEq(&inCref1.clone(),&inCref2.clone()) {
        outEqual = true;
        return Ok(outEqual);
    }
    outEqual = (::match_deref::match_deref! { match &((inCref1.clone(), inCref2.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => var_field!((*inCref1).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*inCref2).ident, DAE::ComponentRef::CREF_IDENT).clone() && ExpressionBasics::subscriptEqual(var_field!((*inCref1).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((*inCref2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?,
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => var_field!((*inCref1).ident, DAE::ComponentRef::CREF_QUAL).clone() == var_field!((*inCref2).ident, DAE::ComponentRef::CREF_QUAL).clone() && crefEqualNoStringCompare(var_field!((*inCref1).componentRef, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*inCref2).componentRef, DAE::ComponentRef::CREF_QUAL).clone())? && ExpressionBasics::subscriptEqual(var_field!((*inCref1).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((*inCref2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqual)
}

pub fn crefEqualReturn(mut cr: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut ocr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let true = (crefEqualNoStringCompare(cr.clone(), cr2.clone())?) else { bail!("pattern mismatch") };
    ocr = cr.clone();
    Ok(ocr)
}

pub fn crefEqualWithoutLastSubs(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    res = crefEqualNoStringCompare(crefStripLastSubs(cr1.clone())?, crefStripLastSubs(cr2.clone())?)?;
    Ok(res)
}

pub fn crefEqualWithoutSubs(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> bool {
    let mut res: bool = false;
    res = crefEqualWithoutSubs2(referenceEq(&cr1.clone(),&cr2.clone()), cr1.clone(), cr2.clone());
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn crefEqualWithoutSubs2(mut refEq: bool, mut icr1: Arc<DAE::ComponentRef>, mut icr2: Arc<DAE::ComponentRef>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((refEq.clone(), icr1.clone(), icr2.clone())) {
        (true, _, _) => {
            true
        },
        (_, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, .. }) => {
            stringEq((n1.clone()).clone(), (n2.clone()).clone())
        },
        (_, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr1, ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr2, ident: n2, .. }) => {
            let mut r: bool = false;
            r = stringEq((n1.clone()).clone(), (n2.clone()).clone());
            r = if (r.clone()) {crefEqualWithoutSubs2(referenceEq(&cr1.clone(),&cr2.clone()), cr1.clone(), cr2.clone())} else {false};
            r.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn crefStripLastSubs(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: t2, ident: id, .. } => {
            makeCrefIdent((id.clone()).clone(), t2.clone(), metamodelica::nil())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, subscriptLst: s, identType: t2, ident: id } => {
            let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cr_1 = crefStripLastSubs(cr.clone())?;
            makeCrefQual((id.clone()).clone(), t2.clone(), s.clone(), cr_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn makeCrefIdent(mut ident: ArcStr, mut identType: Arc<DAE::Type>, mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Arc<DAE::ComponentRef> {
    let mut outCrefIdent: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCrefIdent = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() });
    outCrefIdent
}

pub fn makeCrefQual(mut ident: ArcStr, mut identType: Arc<DAE::Type>, mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut componentRef: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut outCrefQual: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut subCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCrefQual = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() });
    outCrefQual
}

pub fn printComponentRefStr(mut inComponentRef: Arc<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: s, .. } => {
            s.clone()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, ident: s, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (printComponentRef2Str((s.clone()).clone(), subs.clone())?).clone();
            r#str.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, subscriptLst: subs, ident: s, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut strrest: ArcStr = arcstr::literal!("");
            let mut strseb: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            b = Config::modelicaOutput()?;
            r#str = (printComponentRef2Str((s.clone()).clone(), subs.clone())?).clone();
            strrest = (printComponentRefStr(cr.clone())?).clone();
            strseb = (if (b.clone()) {literal!("__")} else {literal!(".")}).clone();
            r#str = stringAppendList(list![(r#str.clone()).clone(), (strseb.clone()).clone(), (strrest.clone()).clone()]);
            r#str.clone()
        },
        Deref @ DAE::ComponentRef::WILD => {
            literal!("_")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printComponentRef2Str(mut inIdent: ArcStr, mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inIdent.clone(), inSubscriptLst.clone())) {
        (s, Deref @ metamodelica::List::Nil) => {
            s.clone()
        },
        (s, l) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut strseba: ArcStr = arcstr::literal!("");
            let mut strsebb: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            b = Config::modelicaOutput()?;
            r#str = (ExpressionBasics::printListStr(l.clone(), Arc::new(ExpressionBasics::printSubscriptStr), (literal!(",")).clone())).clone();
            (strseba, strsebb) = if (b.clone()) {(literal!("_L"), literal!("_R"))} else {(literal!("["), literal!("]"))};
            r#str = stringAppendList(list![(s.clone()).clone(), (strseba.clone()).clone(), (r#str.clone()).clone(), (strsebb.clone()).clone()]);
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

