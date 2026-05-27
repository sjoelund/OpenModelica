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

use crate::AbsynUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };

pub fn stripSubmod(mut r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = metamodelica::nil());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

pub fn filterSubMods(mut r#mod: Arc<SCode::Mod>, mut filter: Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>) -> Arc<SCode::Mod> {
    pub type FilterFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>;

    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            if !(filter(m.clone()).unwrap()) { continue; }
            let __x = m.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { binding: None, subModLst: Deref @ metamodelica::List::Nil, .. } => Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD),
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

pub fn filterGivenSubModNames(mut submod: Arc<SCode::SubMod>, mut namesToKeep: Arc<metamodelica::List<ArcStr>>) -> bool {
    let mut keep: bool = false;
    keep = listMember((submod.ident.clone()).clone(), namesToKeep.clone());
    keep
}

pub fn removeGivenSubModNames(mut submod: Arc<SCode::SubMod>, mut namesToRemove: Arc<metamodelica::List<ArcStr>>) -> bool {
    let mut keep: bool = false;
    keep = !(listMember((submod.ident.clone()).clone(), namesToRemove.clone()));
    keep
}

pub fn getElementNamed(mut inIdent: ArcStr, mut inClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &((inIdent.clone(), inClass.clone())) {
        (id, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }) => {
            let mut elt: Arc<SCode::Element>;
            elt = getElementNamedFromElts((id.clone()).clone(), elts.clone())?;
            elt.clone()
        },
        (id, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. }) => {
            let mut elt: Arc<SCode::Element>;
            elt = getElementNamedFromElts((id.clone()).clone(), elts.clone())?;
            elt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getElementNamedFromElts(mut inIdent: ArcStr, mut inElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = (inIdent.clone(), inElementLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { name: id1, .. }, tail: _ }) => {
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(comp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: id1, .. }, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    let false = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { name: id1, .. }, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    let false = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { .. }, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { name: id1, .. }, tail: _ }) => {
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(cdef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: _, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElement)
}

pub fn isElementExtends(mut ele: Arc<SCode::Element>) -> bool {
    let mut isExtend: bool = false;
    isExtend = (::match_deref::match_deref! { match &(ele.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtend
}

pub fn isElementExtendsOrClassExtends(mut ele: Arc<SCode::Element>) -> bool {
    let mut isExtend: bool = false;
    isExtend = (::match_deref::match_deref! { match &(ele.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtend
}

pub fn isNotElementClassExtends(mut ele: Arc<SCode::Element>) -> bool {
    let mut isExtend: bool = false;
    isExtend = (::match_deref::match_deref! { match &(ele.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtend
}

pub fn isParameterOrConst(mut inVariability: SCode::Variability) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVariability.clone() {
        SCode::Variability::PARAM => true,
        SCode::Variability::CONST => true,
        _ => false,
    });
    outBoolean
}

pub fn isConstant(mut inVariability: SCode::Variability) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inVariability.clone() {
        SCode::Variability::CONST => true,
        _ => false,
    });
    outBoolean
}

pub fn countParts(mut inClass: Arc<SCode::Element>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
                    let mut res: i32 = 0;
                    res = (elts.clone().len() as i32);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => {
                    let mut res: i32 = 0;
                    res = (elts.clone().len() as i32);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

pub fn componentNames(mut inClass: Arc<SCode::Element>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            res = componentNamesFromElts(elts.clone());
            res.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            res = componentNamesFromElts(elts.clone());
            res.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStringLst
}

pub fn componentNamesFromElts(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outComponentNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outComponentNames = List::filterMap(inElements.clone(), (std::sync::Arc::new(componentName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
    outComponentNames
}

pub fn componentName(mut inComponent: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inComponent.clone()) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    Ok(outName)
}

pub fn elementInfo(mut e: Arc<SCode::Element>) -> SourceInfo {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { info: i, .. } => {
            i.clone()
        },
        Deref @ SCode::Element::CLASS { info: i, .. } => {
            i.clone()
        },
        Deref @ SCode::Element::EXTENDS { info: i, .. } => {
            i.clone()
        },
        Deref @ SCode::Element::IMPORT { info: i, .. } => {
            i.clone()
        },
        _ => {
            Absyn::dummyInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    info
}

pub fn setElementName(mut e: Arc<SCode::Element>, mut name: ArcStr) -> Arc<SCode::Element> {
    let mut e: Arc<SCode::Element> = e;
    let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(e => SCode::Element::CLASS; name = name.clone());
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(e => SCode::Element::COMPONENT; name = name.clone());
            ()
        },
        Deref @ SCode::Element::DEFINEUNIT { .. } => {
            assign_variant_field!(e => SCode::Element::DEFINEUNIT; name = name.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    e
}

pub fn elementName(mut e: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { name: s, .. } => s.clone(),
        Deref @ SCode::Element::CLASS { name: s, .. } => s.clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(s)
}

pub fn elementNameInfo(mut element: Arc<SCode::Element>) -> Result<(ArcStr, SourceInfo)> {
    let mut name: ArcStr = arcstr::literal!("");
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (name, info) = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { info, name, .. } => (name.clone(), info.clone()),
        Deref @ SCode::Element::CLASS { info, name, .. } => (name.clone(), info.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok((name, info))
}

pub fn elementNames(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    names = List::fold(elts.clone(), (std::sync::Arc::new(fnptr!(elementNamesWork, Arc<SCode::Element>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), metamodelica::nil());
    names
}

fn elementNamesWork(mut e: Arc<SCode::Element>, mut acc: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut out: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((e.clone(), acc.clone())) {
        (Deref @ SCode::Element::COMPONENT { name: s, .. }, _) => {
            cons((s.clone()).clone(), acc.clone())
        },
        (Deref @ SCode::Element::CLASS { name: s, .. }, _) => {
            cons((s.clone()).clone(), acc.clone())
        },
        _ => {
            acc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn renameElement(mut element: Arc<SCode::Element>, mut name: ArcStr) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; name = name.clone());
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; name = name.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn elementNameEqual(mut inElement1: Arc<SCode::Element>, mut inElement2: Arc<SCode::Element>) -> bool {
    let mut outEqual: bool = false;
    outEqual = (::match_deref::match_deref! { match &((inElement1.clone(), inElement2.clone())) {
        (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => var_field!((*inElement1).name, SCode::Element::CLASS).clone() == var_field!((*inElement2).name, SCode::Element::CLASS).clone(),
        (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => var_field!((*inElement1).name, SCode::Element::COMPONENT).clone() == var_field!((*inElement2).name, SCode::Element::COMPONENT).clone(),
        (Deref @ SCode::Element::DEFINEUNIT { .. }, Deref @ SCode::Element::DEFINEUNIT { .. }) => var_field!((*inElement1).name, SCode::Element::DEFINEUNIT).clone() == var_field!((*inElement2).name, SCode::Element::DEFINEUNIT).clone(),
        (Deref @ SCode::Element::EXTENDS { .. }, Deref @ SCode::Element::EXTENDS { .. }) => AbsynUtil::pathEqual(var_field!((*inElement1).baseClassPath, SCode::Element::EXTENDS).clone(), var_field!((*inElement2).baseClassPath, SCode::Element::EXTENDS).clone()),
        (Deref @ SCode::Element::IMPORT { .. }, Deref @ SCode::Element::IMPORT { .. }) => AbsynUtil::importEqual(var_field!((*inElement1).imp, SCode::Element::IMPORT).clone(), var_field!((*inElement2).imp, SCode::Element::IMPORT).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEqual
}

pub fn enumName(mut e: Arc<SCode::Enum>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Enum { literal: s, .. } => s.clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(s)
}

pub fn isRecord(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isTypeVar(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isPolymorphicTypeVar(mut cls: Arc<SCode::Element>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. }, .. }, restriction: SCode::Restriction::R_TYPE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isOperatorRecord(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: true }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isFunction(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isUniontype(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_UNIONTYPE { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isFunctionRestriction(mut inRestriction: SCode::Restriction) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inRestriction.clone() {
        SCode::Restriction::R_FUNCTION { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isFunctionOrExtFunctionRestriction(mut r: SCode::Restriction) -> bool {
    let mut res: bool = false;
    res = (match r.clone() {
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { .. } } => true,
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } } => true,
        _ => false,
    });
    res
}

pub fn isOperator(mut el: Arc<SCode::Element>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_OPERATOR, .. } => true,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isEnumeration(mut el: Arc<SCode::Element>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn className(mut inClass: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    Ok(outName)
}

pub fn classSetPartial(mut cls: Arc<SCode::Element>, mut inPartial: SCode::Partial) -> Result<Arc<SCode::Element>> {
    let mut cls: Arc<SCode::Element> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(cls => SCode::Element::CLASS; partialPrefix = inPartial.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn elementEqual(mut element1: Arc<SCode::Element>, mut element2: Arc<SCode::Element>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (element1.clone(), element2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => {
                    Ok(stringEq((var_field!((*element1).name, SCode::Element::CLASS).clone()).clone(), (var_field!((*element2).name, SCode::Element::CLASS).clone()).clone()) && prefixesEqual(var_field!((*element1).prefixes, SCode::Element::CLASS).clone(), var_field!((*element2).prefixes, SCode::Element::CLASS).clone())? && var_field!((*element1).encapsulatedPrefix, SCode::Element::CLASS).clone() == var_field!((*element2).encapsulatedPrefix, SCode::Element::CLASS).clone() && var_field!((*element1).partialPrefix, SCode::Element::CLASS).clone() == var_field!((*element2).partialPrefix, SCode::Element::CLASS).clone() && restrictionEqual(var_field!((*element1).restriction, SCode::Element::CLASS).clone(), var_field!((*element2).restriction, SCode::Element::CLASS).clone()) && classDefEqual(var_field!((*element1).classDef, SCode::Element::CLASS).clone(), var_field!((*element2).classDef, SCode::Element::CLASS).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
                    Ok(stringEq((var_field!((*element1).name, SCode::Element::COMPONENT).clone()).clone(), (var_field!((*element2).name, SCode::Element::COMPONENT).clone()).clone()) && prefixesEqual(var_field!((*element1).prefixes, SCode::Element::COMPONENT).clone(), var_field!((*element2).prefixes, SCode::Element::COMPONENT).clone())? && attributesEqual(var_field!((*element1).attributes, SCode::Element::COMPONENT).clone(), var_field!((*element2).attributes, SCode::Element::COMPONENT).clone())? && modEqual(var_field!((*element1).modifications, SCode::Element::COMPONENT).clone(), var_field!((*element2).modifications, SCode::Element::COMPONENT).clone())? && AbsynUtil::typeSpecEqual(var_field!((*element1).typeSpec, SCode::Element::COMPONENT).clone(), var_field!((*element2).typeSpec, SCode::Element::COMPONENT).clone()) && var_field!((*element1).condition, SCode::Element::COMPONENT).clone() == var_field!((*element2).condition, SCode::Element::COMPONENT).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::EXTENDS { .. }, Deref @ SCode::Element::EXTENDS { .. }) => {
                    Ok(AbsynUtil::pathEqual(var_field!((*element1).baseClassPath, SCode::Element::EXTENDS).clone(), var_field!((*element2).baseClassPath, SCode::Element::EXTENDS).clone()) && modEqual(var_field!((*element1).modifications, SCode::Element::EXTENDS).clone(), var_field!((*element2).modifications, SCode::Element::EXTENDS).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::IMPORT { .. }, Deref @ SCode::Element::IMPORT { .. }) => {
                    Ok(AbsynUtil::importEqual(var_field!((*element1).imp, SCode::Element::IMPORT).clone(), var_field!((*element2).imp, SCode::Element::IMPORT).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::DEFINEUNIT { .. }, Deref @ SCode::Element::DEFINEUNIT { .. }) => {
                    Ok(stringEq((var_field!((*element1).name, SCode::Element::DEFINEUNIT).clone()).clone(), (var_field!((*element2).name, SCode::Element::DEFINEUNIT).clone()).clone()) && var_field!((*element1).exp, SCode::Element::DEFINEUNIT).clone() == var_field!((*element2).exp, SCode::Element::DEFINEUNIT).clone() && var_field!((*element1).weight, SCode::Element::DEFINEUNIT).clone() == var_field!((*element2).weight, SCode::Element::DEFINEUNIT).clone())
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
    Ok(equal)
}

// stefan
pub fn annotationEqual(mut annotation1: Arc<SCode::Annotation>, mut annotation2: Arc<SCode::Annotation>) -> bool {
    let mut equal: bool = modEqual(annotation1.modification.clone(), annotation2.modification.clone()).unwrap();
    equal
}

pub fn restrictionEqual(mut restr1: SCode::Restriction, mut restr2: SCode::Restriction) -> bool {
    let mut equal: bool = false;
    equal = (match (restr1.clone(), restr2.clone()) {
        (SCode::Restriction::R_CLASS, SCode::Restriction::R_CLASS) => {
            true
        },
        (SCode::Restriction::R_OPTIMIZATION, SCode::Restriction::R_OPTIMIZATION) => {
            true
        },
        (SCode::Restriction::R_MODEL, SCode::Restriction::R_MODEL) => {
            true
        },
        (SCode::Restriction::R_RECORD { isOperator: true }, SCode::Restriction::R_RECORD { isOperator: true }) => {
            true
        },
        (SCode::Restriction::R_RECORD { isOperator: false }, SCode::Restriction::R_RECORD { isOperator: false }) => {
            true
        },
        (SCode::Restriction::R_BLOCK, SCode::Restriction::R_BLOCK) => {
            true
        },
        (SCode::Restriction::R_CONNECTOR { isExpandable: true }, SCode::Restriction::R_CONNECTOR { isExpandable: true }) => {
            true
        },
        (SCode::Restriction::R_CONNECTOR { isExpandable: false }, SCode::Restriction::R_CONNECTOR { isExpandable: false }) => {
            true
        },
        (SCode::Restriction::R_OPERATOR, SCode::Restriction::R_OPERATOR) => {
            true
        },
        (SCode::Restriction::R_TYPE, SCode::Restriction::R_TYPE) => {
            true
        },
        (SCode::Restriction::R_PACKAGE, SCode::Restriction::R_PACKAGE) => {
            true
        },
        (SCode::Restriction::R_FUNCTION { functionRestriction: mut funcRest1 }, SCode::Restriction::R_FUNCTION { functionRestriction: mut funcRest2 }) => {
            funcRestrictionEqual(funcRest1.clone(), funcRest2.clone())
        },
        (SCode::Restriction::R_ENUMERATION, SCode::Restriction::R_ENUMERATION) => {
            true
        },
        (SCode::Restriction::R_PREDEFINED_INTEGER, SCode::Restriction::R_PREDEFINED_INTEGER) => {
            true
        },
        (SCode::Restriction::R_PREDEFINED_REAL, SCode::Restriction::R_PREDEFINED_REAL) => {
            true
        },
        (SCode::Restriction::R_PREDEFINED_STRING, SCode::Restriction::R_PREDEFINED_STRING) => {
            true
        },
        (SCode::Restriction::R_PREDEFINED_BOOLEAN, SCode::Restriction::R_PREDEFINED_BOOLEAN) => {
            true
        },
        (SCode::Restriction::R_PREDEFINED_CLOCK, SCode::Restriction::R_PREDEFINED_CLOCK) => {
            true
        },
        (SCode::Restriction::R_PREDEFINED_ENUMERATION, SCode::Restriction::R_PREDEFINED_ENUMERATION) => {
            true
        },
        (SCode::Restriction::R_UNIONTYPE { .. }, SCode::Restriction::R_UNIONTYPE { .. }) => {
            {
        let mut __acc: Option<bool> = None;
        for (t1, t2) in (&(var_field!(restr1.typeVars, SCode::Restriction::R_UNIONTYPE).clone())).into_iter().zip((&(var_field!(restr2.typeVars, SCode::Restriction::R_UNIONTYPE).clone())).into_iter()) {
            let __x = t1.clone() == t2.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction")).unwrap()
    }
        },
        _ => {
            false
        },
    });
    equal
}

pub fn funcRestrictionEqual(mut funcRestr1: SCode::FunctionRestriction, mut funcRestr2: SCode::FunctionRestriction) -> bool {
    let mut equal: bool = false;
    equal = (match (funcRestr1.clone(), funcRestr2.clone()) {
        (SCode::FunctionRestriction::FR_NORMAL_FUNCTION { .. }, SCode::FunctionRestriction::FR_NORMAL_FUNCTION { .. }) => AbsynUtil::purityEqual(var_field!(funcRestr1.purity, SCode::FunctionRestriction::FR_NORMAL_FUNCTION).clone(), var_field!(funcRestr2.purity, SCode::FunctionRestriction::FR_NORMAL_FUNCTION).clone(), false),
        (SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. }, SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. }) => AbsynUtil::purityEqual(var_field!(funcRestr1.purity, SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION).clone(), var_field!(funcRestr2.purity, SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION).clone(), false),
        (SCode::FunctionRestriction::FR_OPERATOR_FUNCTION, SCode::FunctionRestriction::FR_OPERATOR_FUNCTION) => true,
        (SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR, SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR) => true,
        (SCode::FunctionRestriction::FR_PARALLEL_FUNCTION, SCode::FunctionRestriction::FR_PARALLEL_FUNCTION) => true,
        (SCode::FunctionRestriction::FR_KERNEL_FUNCTION, SCode::FunctionRestriction::FR_KERNEL_FUNCTION) => true,
        _ => false,
    });
    equal
}

pub fn enumEqual(mut e1: Arc<SCode::Enum>, mut e2: Arc<SCode::Enum>) -> bool {
    let mut isEqual: bool = e1.literal.clone() == e2.literal.clone();
    isEqual
}

fn classDefEqual(mut cdef1: Arc<SCode::ClassDef>, mut cdef2: Arc<SCode::ClassDef>) -> Result<bool> {
    let mut equal: bool = false;
    equal = (::match_deref::match_deref! { match &((cdef1.clone(), cdef2.clone())) {
        (Deref @ SCode::ClassDef::PARTS { .. }, Deref @ SCode::ClassDef::PARTS { .. }) => List::isEqualOnTrue(var_field!((*cdef1).elementLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).elementLst, SCode::ClassDef::PARTS).clone(), (std::sync::Arc::new(elementEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<SCode::Element>) -> Result<bool> + 'static>)) && List::isEqualOnTrue(var_field!((*cdef1).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).normalEquationLst, SCode::ClassDef::PARTS).clone(), (std::sync::Arc::new(equationEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<SCode::Equation>) -> Result<bool> + 'static>)) && List::isEqualOnTrue(var_field!((*cdef1).initialEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).initialEquationLst, SCode::ClassDef::PARTS).clone(), (std::sync::Arc::new(equationEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<SCode::Equation>) -> Result<bool> + 'static>)) && List::isEqualOnTrue(var_field!((*cdef1).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), (std::sync::Arc::new(fnptr!(algorithmEqual, Arc<SCode::AlgorithmSection>, Arc<SCode::AlgorithmSection>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, Arc<SCode::AlgorithmSection>) -> Result<bool> + 'static>)) && List::isEqualOnTrue(var_field!((*cdef1).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), (std::sync::Arc::new(fnptr!(algorithmEqual, Arc<SCode::AlgorithmSection>, Arc<SCode::AlgorithmSection>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::AlgorithmSection>, Arc<SCode::AlgorithmSection>) -> Result<bool> + 'static>)),
        (Deref @ SCode::ClassDef::DERIVED { .. }, Deref @ SCode::ClassDef::DERIVED { .. }) => AbsynUtil::typeSpecEqual(var_field!((*cdef1).typeSpec, SCode::ClassDef::DERIVED).clone(), var_field!((*cdef2).typeSpec, SCode::ClassDef::DERIVED).clone()) && modEqual(var_field!((*cdef1).modifications, SCode::ClassDef::DERIVED).clone(), var_field!((*cdef2).modifications, SCode::ClassDef::DERIVED).clone())? && attributesEqual(var_field!((*cdef1).attributes, SCode::ClassDef::DERIVED).clone(), var_field!((*cdef2).attributes, SCode::ClassDef::DERIVED).clone())?,
        (Deref @ SCode::ClassDef::ENUMERATION { .. }, Deref @ SCode::ClassDef::ENUMERATION { .. }) => List::isEqualOnTrue(var_field!((*cdef1).enumLst, SCode::ClassDef::ENUMERATION).clone(), var_field!((*cdef2).enumLst, SCode::ClassDef::ENUMERATION).clone(), (std::sync::Arc::new(fnptr!(enumEqual, Arc<SCode::Enum>, Arc<SCode::Enum>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Enum>, Arc<SCode::Enum>) -> Result<bool> + 'static>)),
        (Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }) => modEqual(var_field!((*cdef1).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), var_field!((*cdef2).modifications, SCode::ClassDef::CLASS_EXTENDS).clone())? && classDefEqual(var_field!((*cdef1).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), var_field!((*cdef2).composition, SCode::ClassDef::CLASS_EXTENDS).clone())?,
        (Deref @ SCode::ClassDef::PDER { .. }, Deref @ SCode::ClassDef::PDER { .. }) => List::isEqualOnTrue(var_field!((*cdef1).derivedVariables, SCode::ClassDef::PDER).clone(), var_field!((*cdef2).derivedVariables, SCode::ClassDef::PDER).clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

fn arraydimOptEqual(mut adopt1: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut adopt2: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> bool {
    let mut equal: bool = false;
    equal = (::match_deref::match_deref! { match &((adopt1.clone(), adopt2.clone())) {
        (None, None) => {
            true
        },
        (Some(lst1), Some(lst2)) => {
            List::isEqualOnTrue(lst1.clone(), lst2.clone(), (std::sync::Arc::new(subscriptEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, Arc<Absyn::Subscript>) -> Result<bool> + 'static>))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

fn subscriptEqual(mut sub1: Arc<Absyn::Subscript>, mut sub2: Arc<Absyn::Subscript>) -> Result<bool> {
    let mut equal: bool = false;
    equal = (::match_deref::match_deref! { match &((sub1.clone(), sub2.clone())) {
        (Deref @ Absyn::Subscript::NOSUB, Deref @ Absyn::Subscript::NOSUB) => {
            true
        },
        (Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 }, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e2 }) => {
            AbsynUtil::expEqual(e1.clone(), e2.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(equal)
}

fn algorithmEqual(mut alg1: Arc<SCode::AlgorithmSection>, mut alg2: Arc<SCode::AlgorithmSection>) -> bool {
    let mut equal: bool = false;
    equal = List::isEqualOnTrue(alg1.statements.clone(), alg2.statements.clone(), (std::sync::Arc::new(statementEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, Arc<SCode::Statement>) -> Result<bool> + 'static>));
    equal
}

fn statementEqual(mut ai1: Arc<SCode::Statement>, mut ai2: Arc<SCode::Statement>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (ai1.clone(), ai2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Statement::ALG_ASSIGN { value: e1, assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr1 }, .. }, Deref @ SCode::Statement::ALG_ASSIGN { value: e2, assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr2 }, .. }) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut equal: bool = equal.clone();
                    b1 = AbsynUtil::crefEqual(cr1.clone(), cr2.clone());
                    b2 = AbsynUtil::expEqual(e1.clone(), e2.clone())?;
                    equal = boolAnd(b1.clone(), b2.clone());
                    Ok(equal.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Statement::ALG_ASSIGN { value: e12, assignComponent: e11 @ Deref @ Absyn::Exp::TUPLE { expressions: _ }, .. }, Deref @ SCode::Statement::ALG_ASSIGN { value: e22, assignComponent: e21 @ Deref @ Absyn::Exp::TUPLE { expressions: _ }, .. }) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut equal: bool = equal.clone();
                    b1 = AbsynUtil::expEqual(e11.clone(), e21.clone())?;
                    b2 = AbsynUtil::expEqual(e12.clone(), e22.clone())?;
                    equal = boolAnd(b1.clone(), b2.clone());
                    Ok(equal.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a1, a2) => {
                    let mut alg1: Arc<Absyn::Algorithm> = Arc::new(Absyn::Algorithm::ALG_BREAK);
                    let mut alg2: Arc<Absyn::Algorithm> = Arc::new(Absyn::Algorithm::ALG_BREAK);
                    let __pa0 = ::match_deref::match_deref! { match &(statementToAlgorithmItem(a1.clone())?) {
                        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    alg1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(statementToAlgorithmItem(a2.clone())?) {
                        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa1, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    alg2 = __pa1.clone();
                    Ok(alg1.clone() == alg2.clone())
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
    Ok(equal)
}

fn equationEqual(mut eq1: Arc<SCode::Equation>, mut eq2: Arc<SCode::Equation>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (eq1.clone(), eq2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_IF { elseBranch: fb1, thenBranch: tb1, condition: ifcond1, .. }, Deref @ SCode::Equation::EQ_IF { elseBranch: fb2, thenBranch: tb2, condition: ifcond2, .. }) => {
                    let true = (equationEqual2(tb1.clone(), tb2.clone())?) else { bail!("pattern mismatch") };
                    let true = (List::isEqualOnTrue(fb1.clone(), fb2.clone(), (std::sync::Arc::new(equationEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<SCode::Equation>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    let true = (List::isEqualOnTrue(ifcond1.clone(), ifcond2.clone(), (std::sync::Arc::new(AbsynUtil::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<Absyn::Exp>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_EQUALS { expRight: e12, expLeft: e11, .. }, Deref @ SCode::Equation::EQ_EQUALS { expRight: e22, expLeft: e21, .. }) => {
                    let true = (AbsynUtil::expEqual(e11.clone(), e21.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e12.clone(), e22.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_PDE { domain: cr1, expRight: e12, expLeft: e11, .. }, Deref @ SCode::Equation::EQ_PDE { domain: cr2, expRight: e22, expLeft: e21, .. }) => {
                    let true = (AbsynUtil::expEqual(e11.clone(), e21.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e12.clone(), e22.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::crefEqual(cr1.clone(), cr2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_CONNECT { crefRight: cr12, crefLeft: cr11, .. }, Deref @ SCode::Equation::EQ_CONNECT { crefRight: cr22, crefLeft: cr21, .. }) => {
                    let true = (AbsynUtil::crefEqual(cr11.clone(), cr21.clone())) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::crefEqual(cr12.clone(), cr22.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_FOR { eEquationLst: eql1, range: Some(exp1), index: id1, .. }, Deref @ SCode::Equation::EQ_FOR { eEquationLst: eql2, range: Some(exp2), index: id2, .. }) => {
                    let true = (List::isEqualOnTrue(eql1.clone(), eql2.clone(), (std::sync::Arc::new(equationEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<SCode::Equation>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(exp1.clone(), exp2.clone())?) else { bail!("pattern mismatch") };
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_FOR { eEquationLst: eql1, range: None, index: id1, .. }, Deref @ SCode::Equation::EQ_FOR { eEquationLst: eql2, range: None, index: id2, .. }) => {
                    let true = (List::isEqualOnTrue(eql1.clone(), eql2.clone(), (std::sync::Arc::new(equationEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<SCode::Equation>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_WHEN { eEquationLst: elst1, condition: cond1, .. }, Deref @ SCode::Equation::EQ_WHEN { eEquationLst: elst2, condition: cond2, .. }) => {
                    let true = (List::isEqualOnTrue(elst1.clone(), elst2.clone(), (std::sync::Arc::new(equationEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<SCode::Equation>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(cond1.clone(), cond2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_ASSERT { message: m1, condition: c1, .. }, Deref @ SCode::Equation::EQ_ASSERT { message: m2, condition: c2, .. }) => {
                    let true = (AbsynUtil::expEqual(c1.clone(), c2.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(m1.clone(), m2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_REINIT { .. }, Deref @ SCode::Equation::EQ_REINIT { .. }) => {
                    let true = (AbsynUtil::expEqual(var_field!((*eq1).cref, SCode::Equation::EQ_REINIT).clone(), var_field!((*eq2).cref, SCode::Equation::EQ_REINIT).clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(var_field!((*eq1).expReinit, SCode::Equation::EQ_REINIT).clone(), var_field!((*eq2).expReinit, SCode::Equation::EQ_REINIT).clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_NORETCALL { exp: e1, .. }, Deref @ SCode::Equation::EQ_NORETCALL { exp: e2, .. }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
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
    Ok(equal)
}

fn equationEqual2(mut inTb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>, mut inTb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>) -> Result<bool> {
    let mut bOut: bool = false;
    bOut = 'mc: {
        let __mc_input = (inTb1.clone(), inTb2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: tb_1, tail: tb1 }, Deref @ metamodelica::List::Cons { head: tb_2, tail: tb2 }) => {
                    let true = (List::isEqualOnTrue(tb_1.clone(), tb_2.clone(), (std::sync::Arc::new(equationEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<SCode::Equation>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    let true = (equationEqual2(tb1.clone(), tb2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(bOut)
}

pub fn modEqual(mut mod1: Arc<SCode::Mod>, mut mod2: Arc<SCode::Mod>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (mod1.clone(), mod2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submodlst1, binding: Some(e1), comment: _, .. }, Deref @ SCode::Mod::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submodlst2, binding: Some(e2), comment: _, .. }) => {
                    let true = (f1.clone() == f2.clone()) else { bail!("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(submodlst1.clone(), submodlst2.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submodlst1, binding: None, comment: _, .. }, Deref @ SCode::Mod::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submodlst2, binding: None, comment: _, .. }) => {
                    let true = (f1.clone() == f2.clone()) else { bail!("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(submodlst1.clone(), submodlst2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::NOMOD, Deref @ SCode::Mod::NOMOD) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::REDECL { finalPrefix: f1, eachPrefix: each1, element: elt1 }, Deref @ SCode::Mod::REDECL { finalPrefix: f2, eachPrefix: each2, element: elt2 }) => {
                    let true = (f1.clone() == f2.clone()) else { bail!("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (elementEqual(elt1.clone(), elt2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::BREAK_COMPONENT { .. }, Deref @ SCode::Mod::BREAK_COMPONENT { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::BREAK_CONNECT { .. }, Deref @ SCode::Mod::BREAK_CONNECT { .. }) => {
                    Ok(AbsynUtil::crefEqual(var_field!((*mod1).lhs, SCode::Mod::BREAK_CONNECT).clone(), var_field!((*mod2).lhs, SCode::Mod::BREAK_CONNECT).clone()) && AbsynUtil::crefEqual(var_field!((*mod1).rhs, SCode::Mod::BREAK_CONNECT).clone(), var_field!((*mod2).lhs, SCode::Mod::BREAK_CONNECT).clone()))
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
    Ok(equal)
}

fn subModsEqual(mut inSubModLst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inSubModLst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (inSubModLst1.clone(), inSubModLst2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: id1, r#mod: mod1 }, tail: subModLst1 }, Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: id2, r#mod: mod2 }, tail: subModLst2 }) => {
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (modEqual(mod1.clone(), mod2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(subModLst1.clone(), subModLst2.clone())?) else { bail!("pattern mismatch") };
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
    Ok(equal)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subscriptsEqual(mut inSs1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inSs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (inSs1.clone(), inSs2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB, tail: ss1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB, tail: ss2 }) => {
                    Ok(subscriptsEqual(ss1.clone(), ss2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 }, tail: ss1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e2 }, tail: ss2 }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subscriptsEqual(ss1.clone(), ss2.clone())?) else { bail!("pattern mismatch") };
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
    Ok(equal)
}

pub fn attributesEqual(mut attr1: SCode::Attributes, mut attr2: SCode::Attributes) -> Result<bool> {
    let mut equal: bool = false;
    equal = arrayDimEqual(attr1.arrayDims.clone(), attr2.arrayDims.clone())? && attr1.connectorType.clone() == attr2.connectorType.clone() && parallelismEqual(attr1.parallelism.clone(), attr2.parallelism.clone()) && variabilityEqual(attr1.variability.clone(), attr2.variability.clone()) && AbsynUtil::directionEqual(attr1.direction.clone(), attr2.direction.clone()) && AbsynUtil::isFieldEqual(attr1.isField.clone(), attr2.isField.clone());
    Ok(equal)
}

pub fn parallelismEqual(mut prl1: SCode::Parallelism, mut prl2: SCode::Parallelism) -> bool {
    let mut equal: bool = false;
    equal = (match (prl1.clone(), prl2.clone()) {
        (SCode::Parallelism::PARGLOBAL, SCode::Parallelism::PARGLOBAL) => true,
        (SCode::Parallelism::PARLOCAL, SCode::Parallelism::PARLOCAL) => true,
        (SCode::Parallelism::NON_PARALLEL, SCode::Parallelism::NON_PARALLEL) => true,
        _ => false,
    });
    equal
}

pub fn variabilityEqual(mut var1: SCode::Variability, mut var2: SCode::Variability) -> bool {
    let mut equal: bool = false;
    equal = (match (var1.clone(), var2.clone()) {
        (SCode::Variability::VAR, SCode::Variability::VAR) => true,
        (SCode::Variability::DISCRETE, SCode::Variability::DISCRETE) => true,
        (SCode::Variability::PARAM, SCode::Variability::PARAM) => true,
        (SCode::Variability::CONST, SCode::Variability::CONST) => true,
        _ => false,
    });
    equal
}

fn arrayDimEqual(mut iad1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut iad2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (iad1.clone(), iad2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB, tail: ad1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB, tail: ad2 }) => {
                    let true = (arrayDimEqual(ad1.clone(), ad2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 }, tail: ad1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e2 }, tail: ad2 }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    let true = (arrayDimEqual(ad1.clone(), ad2.clone())?) else { bail!("pattern mismatch") };
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
    Ok(equal)
}

pub fn setClassRestriction(mut r: SCode::Restriction, mut cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(cl => SCode::Element::CLASS; restriction = r.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn setClassName(mut name: ArcStr, mut cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            if name.clone() != var_field!((*cl).name, SCode::Element::CLASS).clone() {
                assign_variant_field!(cl => SCode::Element::CLASS; name = name.clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn makeClassPartial(mut inClass: Arc<SCode::Element>) -> Arc<SCode::Element> {
    let mut outClass: Arc<SCode::Element> = inClass.clone();
    outClass = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::NOT_PARTIAL, .. } => {
            assign_variant_field!(outClass => SCode::Element::CLASS; partialPrefix = openmodelica_frontend_types::SCode::Partial::PARTIAL);
            outClass.clone()
        },
        _ => outClass.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outClass
}

pub fn setClassPartialPrefix(mut partialPrefix: SCode::Partial, mut cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            if !(partialPrefix.clone() == var_field!((*cl).partialPrefix, SCode::Element::CLASS).clone()) {
                assign_variant_field!(cl => SCode::Element::CLASS; partialPrefix = partialPrefix.clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn findIteratorIndexedCrefsInEquations(mut inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut inIterator: ArcStr, mut inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    outCrefs = List::fold1(inEqs.clone(), (std::sync::Arc::new(findIteratorIndexedCrefsInEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArcStr, Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>> + 'static>), (inIterator.clone()).clone(), inCrefs.clone());
    outCrefs
}

pub fn findIteratorIndexedCrefsInEquation(mut inEq: Arc<SCode::Equation>, mut inIterator: ArcStr, mut inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    outCrefs = foldEquationsExps(inEq.clone(), Arc::new({ let __pe_b1 = (inIterator.clone()).clone(); move |__pe_a0, __pe_a2| AbsynUtil::findIteratorIndexedCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }), inCrefs.clone())?;
    Ok(outCrefs)
}

pub fn findIteratorIndexedCrefsInStatements(mut inStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut inIterator: ArcStr, mut inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    outCrefs = List::fold1(inStatements.clone(), (std::sync::Arc::new(findIteratorIndexedCrefsInStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArcStr, Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>> + 'static>), (inIterator.clone()).clone(), inCrefs.clone());
    outCrefs
}

pub fn findIteratorIndexedCrefsInStatement(mut inStatement: Arc<SCode::Statement>, mut inIterator: ArcStr, mut inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = metamodelica::nil();
    outCrefs = foldStatementsExps(inStatement.clone(), Arc::new({ let __pe_b1 = (inIterator.clone()).clone(); move |__pe_a0, __pe_a2| AbsynUtil::findIteratorIndexedCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }), inCrefs.clone())?;
    Ok(outCrefs)
}

fn filterComponents(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<ArcStr>>) {
    let mut outComponents: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outComponentNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outComponents, outComponentNames) = List::map_2(inElements.clone(), (std::sync::Arc::new(filterComponents2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<(Arc<SCode::Element>, ArcStr)> + 'static>));
    (outComponents, outComponentNames)
}

fn filterComponents2(mut inElement: Arc<SCode::Element>) -> Result<(Arc<SCode::Element>, ArcStr)> {
    let mut outComponent: Arc<SCode::Element>;
    let mut outName: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    outComponent = inElement.clone();
    Ok((outComponent, outName))
}

pub fn getClassComponents(mut cl: Arc<SCode::Element>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut compNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (compElts, compNames) = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            let mut comps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (comps, names) = filterComponents(elts.clone());
            (comps.clone(), names.clone())
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut comps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (comps, names) = filterComponents(elts.clone());
            (comps.clone(), names.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((compElts, compNames))
}

pub fn getClassElements(mut cl: Arc<SCode::Element>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    elts = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => elts.clone(),
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => elts.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elts
}

pub fn makeEnumType(mut inEnum: Arc<SCode::Enum>, mut inInfo: SourceInfo) -> Result<Arc<SCode::Element>> {
    let mut outEnumType: Arc<SCode::Element>;
    let mut literal: ArcStr = arcstr::literal!("");
    let mut comment: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inEnum.clone()) {
        Deref @ SCode::Enum { comment: __pa0, literal: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comment = __pa0.clone();
    literal = __pa1.clone();
    checkValidEnumLiteral((literal.clone()).clone(), inInfo.clone())?;
    outEnumType = Arc::new(SCode::Element::COMPONENT { name: (literal.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultConstAttr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: comment.clone(), condition: None, info: inInfo.clone() });
    Ok(outEnumType)
}

pub fn variabilityOr(mut inConst1: SCode::Variability, mut inConst2: SCode::Variability) -> SCode::Variability {
    let mut outConst: SCode::Variability = SCode::Variability::CONST;
    outConst = (match (inConst1.clone(), inConst2.clone()) {
        (SCode::Variability::CONST, _) => openmodelica_frontend_types::SCode::Variability::CONST,
        (_, SCode::Variability::CONST) => openmodelica_frontend_types::SCode::Variability::CONST,
        (SCode::Variability::PARAM, _) => openmodelica_frontend_types::SCode::Variability::PARAM,
        (_, SCode::Variability::PARAM) => openmodelica_frontend_types::SCode::Variability::PARAM,
        (SCode::Variability::DISCRETE, _) => openmodelica_frontend_types::SCode::Variability::DISCRETE,
        (_, SCode::Variability::DISCRETE) => openmodelica_frontend_types::SCode::Variability::DISCRETE,
        _ => openmodelica_frontend_types::SCode::Variability::VAR,
    });
    outConst
}

pub fn statementToAlgorithmItem(mut stmt: Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> {
    let mut algi: Arc<Absyn::AlgorithmItem>;
    algi = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { assignComponent, value, comment: _, info } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: assignComponent.clone(), value: value.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_IF { boolExpr, trueBranch, elseIfBranch: branches, elseBranch, comment: _, info } => {
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>> = metamodelica::nil();
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>> = metamodelica::nil();
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
            algs1 = List::map(trueBranch.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            conditions = List::map(branches.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
            stmtsList = List::map(branches.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
            algsLst = List::mapList(stmtsList.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            abranches = List::zip(conditions.clone(), algsLst.clone());
            algs2 = List::map(elseBranch.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_IF { ifExp: boolExpr.clone(), trueBranch: algs1.clone(), elseIfAlgorithmBranch: abranches.clone(), elseBranch: algs2.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_FOR { index: iterator, range, forBody: body, comment: _, info } => {
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            algs1 = List::map(body.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_FOR { iterators: list![Arc::new(Absyn::ForIterator { name: (iterator.clone()).clone(), guardExp: None, range: range.clone() })], forBody: algs1.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_PARFOR { index: iterator, range, parforBody: body, comment: _, info } => {
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            algs1 = List::map(body.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_PARFOR { iterators: list![Arc::new(Absyn::ForIterator { name: (iterator.clone()).clone(), guardExp: None, range: range.clone() })], parforBody: algs1.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_WHILE { boolExpr, whileBody: body, comment: _, info } => {
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            algs1 = List::map(body.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_WHILE { boolExpr: boolExpr.clone(), whileBody: algs1.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_WHEN_A { branches, comment: _, info } => {
            let mut boolExpr: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>> = metamodelica::nil();
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>> = metamodelica::nil();
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::map(branches.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            boolExpr = __pa0.clone();
            conditions = __pa1.clone();
            stmtsList = List::map(branches.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::mapList(stmtsList.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>))) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            algs1 = __pa2.clone();
            algsLst = __pa3.clone();
            abranches = List::zip(conditions.clone(), algsLst.clone());
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: boolExpr.clone(), whenBody: algs1.clone(), elseWhenAlgorithmBranch: abranches.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("assert")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![var_field!((*stmt).condition, SCode::Statement::ALG_ASSERT).clone(), var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone(), var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone()], argNames: metamodelica::nil() }) }), comment: None, info: var_field!((*stmt).info, SCode::Statement::ALG_ASSERT).clone() })
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("terminate")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![var_field!((*stmt).message, SCode::Statement::ALG_TERMINATE).clone()], argNames: metamodelica::nil() }) }), comment: None, info: var_field!((*stmt).info, SCode::Statement::ALG_TERMINATE).clone() })
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("reinit")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![var_field!((*stmt).cref, SCode::Statement::ALG_REINIT).clone(), var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone()], argNames: metamodelica::nil() }) }), comment: None, info: var_field!((*stmt).info, SCode::Statement::ALG_REINIT).clone() })
        },
        Deref @ SCode::Statement::ALG_NORETCALL { exp: Deref @ Absyn::Exp::CALL { functionArgs, function_: functionCall, .. }, comment: _, info } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: functionCall.clone(), functionArgs: functionArgs.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_RETURN { comment: _, info } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_RETURN), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_BREAK { comment: _, info } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_BREAK), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_CONTINUE { comment: _, info } => {
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_CONTINUE), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_FAILURE { stmts: body, comment: _, info } => {
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            algs1 = List::map(body.clone(), (std::sync::Arc::new(statementToAlgorithmItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_FAILURE { equ: algs1.clone() }), comment: None, info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(algi)
}

pub fn emptyModOrEquality(mut r#mod: Arc<SCode::Mod>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => true,
        Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isComponentWithDirection(mut elt: Arc<SCode::Element>, mut dir1: Absyn::Direction) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((elt.clone(), dir1.clone())) {
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: dir2, .. }, .. }, _) => {
            AbsynUtil::directionEqual(dir1.clone(), dir2.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isComponent(mut elt: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isNotComponent(mut elt: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isClassOrComponent(mut inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsClassOrComponent: bool = false;
    outIsClassOrComponent = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { .. } => true,
        Deref @ SCode::Element::COMPONENT { .. } => true,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIsClassOrComponent)
}

pub fn isClass(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outIsClass: bool = false;
    outIsClass = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

pub fn isImport(mut element: Arc<SCode::Element>) -> bool {
    let mut isImport: bool = false;
    isImport = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isImport
}

pub fn foldEquations<ArgT: Clone + 'static>(mut inEquation: Arc<SCode::Equation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<ArgT> + 'static>, mut inArg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT;
    outArg = inFunc(inEquation.clone(), inArg.clone())?;
    outArg = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            outArg = List::foldList(var_field!((*inEquation).thenBranch, SCode::Equation::EQ_IF).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _) -> Result<_> + 'static> = inFunc.clone(); move |__pe_a0, __pe_a2| foldEquations(__pe_a0, __pe_b1.clone(), __pe_a2) }), outArg.clone());
            List::fold1(var_field!((*inEquation).elseBranch, SCode::Equation::EQ_IF).clone(), (std::sync::Arc::new(foldEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone(), (std::sync::Arc::new(foldEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
            outArg = List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_WHEN).clone(), (std::sync::Arc::new(foldEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            for mut branch in &*var_field!((*inEquation).elseBranches, SCode::Equation::EQ_WHEN).clone() {
                let mut branch = branch.clone();
                (_, eql) = branch.clone();
                outArg = List::fold1(eql.clone(), (std::sync::Arc::new(foldEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArg)
}

pub fn foldEquationsExps<ArgT: Clone + 'static>(mut inEquation: Arc<SCode::Equation>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>, mut inArg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT = inArg.clone();
    outArg = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            outArg = List::fold(var_field!((*inEquation).condition, SCode::Equation::EQ_IF).clone(), inFunc.clone(), outArg.clone());
            outArg = List::foldList(var_field!((*inEquation).thenBranch, SCode::Equation::EQ_IF).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, _) -> Result<_> + 'static> = inFunc.clone(); move |__pe_a0, __pe_a2| foldEquationsExps(__pe_a0, __pe_b1.clone(), __pe_a2) }), outArg.clone());
            List::fold1(var_field!((*inEquation).elseBranch, SCode::Equation::EQ_IF).clone(), (std::sync::Arc::new(foldEquationsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Equation::EQ_EQUALS { .. } => {
            outArg = inFunc(var_field!((*inEquation).expLeft, SCode::Equation::EQ_EQUALS).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).expRight, SCode::Equation::EQ_EQUALS).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            outArg = inFunc(var_field!((*inEquation).expLeft, SCode::Equation::EQ_PDE).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).expRight, SCode::Equation::EQ_PDE).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            outArg = inFunc(Arc::new(Absyn::Exp::CREF { componentRef: var_field!((*inEquation).crefLeft, SCode::Equation::EQ_CONNECT).clone() }), outArg.clone())?;
            outArg = inFunc(Arc::new(Absyn::Exp::CREF { componentRef: var_field!((*inEquation).crefRight, SCode::Equation::EQ_CONNECT).clone() }), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            if isSome(var_field!((*inEquation).range, SCode::Equation::EQ_FOR).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*inEquation).range, SCode::Equation::EQ_FOR).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
            }
            List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone(), (std::sync::Arc::new(foldEquationsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
            outArg = List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_WHEN).clone(), (std::sync::Arc::new(foldEquationsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            for mut branch in &*var_field!((*inEquation).elseBranches, SCode::Equation::EQ_WHEN).clone() {
                let mut branch = branch.clone();
                (exp, eql) = branch.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
                outArg = List::fold1(eql.clone(), (std::sync::Arc::new(foldEquationsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        Deref @ SCode::Equation::EQ_ASSERT { .. } => {
            outArg = inFunc(var_field!((*inEquation).condition, SCode::Equation::EQ_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).message, SCode::Equation::EQ_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).level, SCode::Equation::EQ_ASSERT).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => {
            inFunc(var_field!((*inEquation).message, SCode::Equation::EQ_TERMINATE).clone(), outArg.clone())?
        },
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            outArg = inFunc(var_field!((*inEquation).cref, SCode::Equation::EQ_REINIT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).expReinit, SCode::Equation::EQ_REINIT).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => {
            inFunc(var_field!((*inEquation).exp, SCode::Equation::EQ_NORETCALL).clone(), outArg.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArg)
}

pub fn foldStatementsExps<ArgT: Clone + 'static>(mut inStatement: Arc<SCode::Statement>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>, mut inArg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT = inArg.clone();
    outArg = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => {
            outArg = inFunc(var_field!((*inStatement).assignComponent, SCode::Statement::ALG_ASSIGN).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inStatement).value, SCode::Statement::ALG_ASSIGN).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::Statement::ALG_IF { .. } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
            outArg = inFunc(var_field!((*inStatement).boolExpr, SCode::Statement::ALG_IF).clone(), outArg.clone())?;
            outArg = List::fold1(var_field!((*inStatement).trueBranch, SCode::Statement::ALG_IF).clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            for mut branch in &*var_field!((*inStatement).elseIfBranch, SCode::Statement::ALG_IF).clone() {
                let mut branch = branch.clone();
                (exp, stmts) = branch.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
                outArg = List::fold1(stmts.clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            if isSome(var_field!((*inStatement).range, SCode::Statement::ALG_FOR).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*inStatement).range, SCode::Statement::ALG_FOR).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
            }
            List::fold1(var_field!((*inStatement).forBody, SCode::Statement::ALG_FOR).clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            if isSome(var_field!((*inStatement).range, SCode::Statement::ALG_PARFOR).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*inStatement).range, SCode::Statement::ALG_PARFOR).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
            }
            List::fold1(var_field!((*inStatement).parforBody, SCode::Statement::ALG_PARFOR).clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            outArg = inFunc(var_field!((*inStatement).boolExpr, SCode::Statement::ALG_WHILE).clone(), outArg.clone())?;
            List::fold1(var_field!((*inStatement).whileBody, SCode::Statement::ALG_WHILE).clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
            for mut branch in &*var_field!((*inStatement).branches, SCode::Statement::ALG_WHEN_A).clone() {
                let mut branch = branch.clone();
                (exp, stmts) = branch.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
                outArg = List::fold1(stmts.clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            outArg = inFunc(var_field!((*inStatement).condition, SCode::Statement::ALG_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inStatement).message, SCode::Statement::ALG_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inStatement).level, SCode::Statement::ALG_ASSERT).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            inFunc(var_field!((*inStatement).message, SCode::Statement::ALG_TERMINATE).clone(), outArg.clone())?
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            outArg = inFunc(var_field!((*inStatement).cref, SCode::Statement::ALG_REINIT).clone(), outArg.clone())?;
            inFunc(var_field!((*inStatement).newValue, SCode::Statement::ALG_REINIT).clone(), outArg.clone())?
        },
        Deref @ SCode::Statement::ALG_NORETCALL { .. } => {
            inFunc(var_field!((*inStatement).exp, SCode::Statement::ALG_NORETCALL).clone(), outArg.clone())?
        },
        Deref @ SCode::Statement::ALG_FAILURE { .. } => {
            List::fold1(var_field!((*inStatement).stmts, SCode::Statement::ALG_FAILURE).clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Statement::ALG_TRY { .. } => {
            outArg = List::fold1(var_field!((*inStatement).body, SCode::Statement::ALG_TRY).clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone());
            List::fold1(var_field!((*inStatement).elseBody, SCode::Statement::ALG_TRY).clone(), (std::sync::Arc::new(foldStatementsExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _, _) -> Result<_> + 'static>), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::Statement::ALG_RETURN { .. } => {
            outArg.clone()
        },
        Deref @ SCode::Statement::ALG_BREAK { .. } => {
            outArg.clone()
        },
        Deref @ SCode::Statement::ALG_CONTINUE { .. } => {
            outArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArg)
}

pub fn mapFoldEquationsList<ArgT: Clone + 'static>(mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<metamodelica::List<Arc<SCode::Equation>>>, ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>;

    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = eql;
    let mut arg: ArgT = arg;
    (eql, arg) = List::mapFold(eql.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _) -> Result<_> + 'static> = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldEquations(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
    (eql, arg)
}

pub fn mapFoldEquations<ArgT: Clone + 'static>(mut eq: Arc<SCode::Equation>, mut traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<SCode::Equation>, ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>;

    let mut eq: Arc<SCode::Equation> = eq;
    let mut arg: ArgT = arg;
    (eq, arg) = traverser(eq.clone(), arg.clone()).unwrap();
    (eq, arg) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { condition: expl1, thenBranch: then_branch, elseBranch: else_branch, comment, info } => {
            let mut then_branch = (*then_branch).clone();
            let mut else_branch = (*else_branch).clone();
            (then_branch, arg) = List::mapFold(then_branch.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _) -> Result<_> + 'static> = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldEquationsList(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (else_branch, arg) = mapFoldEquationsList(else_branch.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Equation::EQ_IF { condition: expl1.clone(), thenBranch: then_branch.clone(), elseBranch: else_branch.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
            (eql, arg) = mapFoldEquationsList(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone(), traverser.clone(), arg.clone());
            assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = eql.clone());
            (eq.clone(), arg.clone())
        },
        Deref @ SCode::Equation::EQ_WHEN { condition: e1, eEquationLst: eql, elseBranches: else_when, comment, info } => {
            let mut eql = (*eql).clone();
            let mut else_when = (*else_when).clone();
            (eql, arg) = mapFoldEquationsList(eql.clone(), traverser.clone(), arg.clone());
            (else_when, arg) = List::mapFold(else_when.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _) -> Result<_> + 'static> = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldElseWhenEquations(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (Arc::new(SCode::Equation::EQ_WHEN { condition: e1.clone(), eEquationLst: eql.clone(), elseBranches: else_when.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => {
            (eq.clone(), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (eq, arg)
}

fn mapFoldElseWhenEquations<ArgT: Clone + 'static>(mut elseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), mut traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>, mut arg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>;

    let mut elseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>) = elseWhen;
    let mut arg: ArgT = arg;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    (exp, eql) = elseWhen.clone();
    (eql, arg) = mapFoldEquationsList(eql.clone(), traverser.clone(), arg.clone());
    elseWhen = (exp.clone(), eql.clone());
    (elseWhen, arg)
}

pub fn mapFoldEquationListExps<ArgT: Clone + 'static>(mut inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> (Arc<metamodelica::List<Arc<SCode::Equation>>>, ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outEquations: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut outArg: ArgT;
    (outEquations, outArg) = List::map1Fold(inEquations.clone(), (std::sync::Arc::new(mapFoldEquationExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, _, _) -> Result<_> + 'static>), traverser.clone(), inArg.clone());
    (outEquations, outArg)
}

pub fn mapFoldEquationExps<ArgT: Clone + 'static>(mut eq: Arc<SCode::Equation>, mut traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut eq: Arc<SCode::Equation> = eq;
    let mut arg: ArgT = arg;
    (eq, arg) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { condition: expl1, thenBranch: then_branch, elseBranch: else_branch, comment, info } => {
            let mut expl1 = (*expl1).clone();
            (expl1, arg) = AbsynUtil::traverseExpList(expl1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Equation::EQ_IF { condition: expl1.clone(), thenBranch: then_branch.clone(), elseBranch: else_branch.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_EQUALS { expLeft: e1, expRight: e2, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_EQUALS { expLeft: e1.clone(), expRight: e2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: e1, expRight: e2, domain, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_PDE { expLeft: e1.clone(), expRight: e2.clone(), domain: domain.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_CONNECT { crefLeft: cr1, crefRight: cr2, comment, info } => {
            let mut cr1 = (*cr1).clone();
            let mut cr2 = (*cr2).clone();
            (cr1, arg) = mapFoldComponentRefExps(cr1.clone(), traverser.clone(), arg.clone())?;
            (cr2, arg) = mapFoldComponentRefExps(cr2.clone(), traverser.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_CONNECT { crefLeft: cr1.clone(), crefRight: cr2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_FOR { index, range: Some(e1), eEquationLst: eql, comment, info } => {
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_FOR { index: (index.clone()).clone(), range: Some(e1.clone()), eEquationLst: eql.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_WHEN { condition: e1, eEquationLst: eql, elseBranches: else_when, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut else_when = (*else_when).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (else_when, arg) = List::map1Fold(else_when.clone(), std::sync::Arc::new(fnptr!(mapFoldElseWhenExps, (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), _, _)), traverser.clone(), arg.clone());
            (Arc::new(SCode::Equation::EQ_WHEN { condition: e1.clone(), eEquationLst: eql.clone(), elseBranches: else_when.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_ASSERT { condition: e1, message: e2, level: e3, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut e3 = (*e3).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (e3, arg) = traverser(e3.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_TERMINATE { message: e1, comment, info } => {
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_TERMINATE { message: e1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: e1, expReinit: e2, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_REINIT { cref: e1.clone(), expReinit: e2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Equation::EQ_NORETCALL { exp: e1, comment, info } => {
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_NORETCALL { exp: e1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => {
            (eq.clone(), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, arg))
}

fn mapFoldComponentRefExps<ArgT: Clone + 'static>(mut inCref: Arc<Absyn::ComponentRef>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<Absyn::ComponentRef>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outCref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut outArg: ArgT;
    (outCref, outArg) = (::match_deref::match_deref! { match &((inCref.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr }, _, _) => {
            let mut arg: ArgT;
            let mut cr = (*cr).clone();
            (cr, arg) = mapFoldComponentRefExps(cr.clone(), inFunc.clone(), inArg.clone())?;
            (AbsynUtil::crefMakeFullyQualified(cr.clone()), arg.clone())
        },
        (Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cr, subscripts: subs, name }, _, _) => {
            let mut arg: ArgT;
            let mut cr = (*cr).clone();
            let mut subs = (*subs).clone();
            (cr, arg) = mapFoldComponentRefExps(cr.clone(), inFunc.clone(), inArg.clone())?;
            (subs, arg) = List::map1Fold(subs.clone(), (std::sync::Arc::new(mapFoldSubscriptExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, _, _) -> Result<_> + 'static>), inFunc.clone(), arg.clone());
            (Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (name.clone()).clone(), subscripts: subs.clone(), componentRef: cr.clone() }), arg.clone())
        },
        (Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: subs, name }, _, _) => {
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            (subs, arg) = List::map1Fold(subs.clone(), (std::sync::Arc::new(mapFoldSubscriptExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Subscript>, _, _) -> Result<_> + 'static>), inFunc.clone(), inArg.clone());
            (Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subs.clone() }), arg.clone())
        },
        (Deref @ Absyn::ComponentRef::WILD, _, _) => {
            (inCref.clone(), inArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCref, outArg))
}

fn mapFoldSubscriptExps<ArgT: Clone + 'static>(mut inSubscript: Arc<Absyn::Subscript>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<Absyn::Subscript>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outSubscript: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    let mut outArg: ArgT;
    (outSubscript, outArg) = (::match_deref::match_deref! { match &((inSubscript.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ Absyn::Subscript::SUBSCRIPT { subscript: sub_exp }, traverser, arg) => {
            let mut sub_exp = (*sub_exp).clone();
            let mut arg = (*arg).clone();
            (sub_exp, arg) = traverser(sub_exp.clone(), arg.clone())?;
            (Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: sub_exp.clone() }), arg.clone())
        },
        (Deref @ Absyn::Subscript::NOSUB, _, _) => {
            (inSubscript.clone(), inArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSubscript, outArg))
}

fn mapFoldElseWhenExps<ArgT: Clone + 'static>(mut inElseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), mut traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outElseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>);
    let mut outArg: ArgT;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    (exp, eql) = inElseWhen.clone();
    (exp, outArg) = traverser(exp.clone(), inArg.clone()).unwrap();
    outElseWhen = (exp.clone(), eql.clone());
    (outElseWhen, outArg)
}

fn mapFoldForIteratorExps<ArgT: Clone + 'static>(mut inIterator: Arc<Absyn::ForIterator>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<Absyn::ForIterator>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outIterator: Arc<Absyn::ForIterator>;
    let mut outArg: ArgT;
    (outIterator, outArg) = (::match_deref::match_deref! { match &((inIterator.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: None }, _, arg) => {
            (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: None, range: None }), arg.clone())
        },
        (Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(range) }, traverser, arg) => {
            let mut range = (*range).clone();
            let mut arg = (*arg).clone();
            (range, arg) = traverser(range.clone(), arg.clone())?;
            (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: None, range: Some(range.clone()) }), arg.clone())
        },
        (Deref @ Absyn::ForIterator { name: ident, guardExp: Some(guardExp), range: Some(range) }, traverser, arg) => {
            let mut guardExp = (*guardExp).clone();
            let mut range = (*range).clone();
            let mut arg = (*arg).clone();
            (guardExp, arg) = traverser(guardExp.clone(), arg.clone())?;
            (range, arg) = traverser(range.clone(), arg.clone())?;
            (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: Some(guardExp.clone()), range: Some(range.clone()) }), arg.clone())
        },
        (Deref @ Absyn::ForIterator { name: ident, guardExp: Some(guardExp), range: None }, traverser, arg) => {
            let mut guardExp = (*guardExp).clone();
            let mut arg = (*arg).clone();
            (guardExp, arg) = traverser(guardExp.clone(), arg.clone())?;
            (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: Some(guardExp.clone()), range: None }), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outIterator, outArg))
}

pub fn mapFoldStatementsList<ArgT: Clone + 'static>(mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<metamodelica::List<Arc<SCode::Statement>>>, ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>;

    let mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>> = statements;
    let mut arg: ArgT = arg;
    (statements, arg) = List::mapFold(statements.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _) -> Result<_> + 'static> = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldStatements(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
    (statements, arg)
}

pub fn mapFoldStatements<ArgT: Clone + 'static>(mut stmt: Arc<SCode::Statement>, mut traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<SCode::Statement>, ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>;

    let mut stmt: Arc<SCode::Statement> = stmt;
    let mut arg: ArgT = arg;
    (stmt, arg) = traverser(stmt.clone(), arg.clone()).unwrap();
    (stmt, arg) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_IF { boolExpr: e, trueBranch: stmts1, elseIfBranch: branches, elseBranch: stmts2, comment, info } => {
            let mut stmts1 = (*stmts1).clone();
            let mut branches = (*branches).clone();
            let mut stmts2 = (*stmts2).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (branches, arg) = List::mapFold(branches.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _) -> Result<_> + 'static> = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldBranchStatements(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (stmts2, arg) = mapFoldStatementsList(stmts2.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_IF { boolExpr: e.clone(), trueBranch: stmts1.clone(), elseIfBranch: branches.clone(), elseBranch: stmts2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Statement::ALG_FOR { index: iter, range, forBody: stmts1, comment, info } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_FOR { index: (iter.clone()).clone(), range: range.clone(), forBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Statement::ALG_PARFOR { index: iter, range, parforBody: stmts1, comment, info } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_PARFOR { index: (iter.clone()).clone(), range: range.clone(), parforBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Statement::ALG_WHILE { boolExpr: e, whileBody: stmts1, comment, info } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_WHILE { boolExpr: e.clone(), whileBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Statement::ALG_WHEN_A { branches, comment, info } => {
            let mut branches = (*branches).clone();
            (branches, arg) = List::mapFold(branches.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, _) -> Result<_> + 'static> = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldBranchStatements(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (Arc::new(SCode::Statement::ALG_WHEN_A { branches: branches.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::Statement::ALG_FAILURE { stmts: stmts1, comment, info } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_FAILURE { stmts: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => {
            (stmt.clone(), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (stmt, arg)
}

fn mapFoldBranchStatements<ArgT: Clone + 'static>(mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), mut traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>, mut arg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>;

    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>) = branch;
    let mut arg: ArgT = arg;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    (exp, stmts) = branch.clone();
    (stmts, arg) = mapFoldStatementsList(stmts.clone(), traverser.clone(), arg.clone());
    branch = (exp.clone(), stmts.clone());
    (branch, arg)
}

pub fn mapFoldStatementListExps<ArgT: Clone + 'static>(mut inStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> (Arc<metamodelica::List<Arc<SCode::Statement>>>, ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outStatements: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    let mut outArg: ArgT;
    (outStatements, outArg) = List::map1Fold(inStatements.clone(), std::sync::Arc::new(fnptr!(mapFoldStatementExps, Arc<SCode::Statement>, _, _)), inFunc.clone(), inArg.clone());
    (outStatements, outArg)
}

pub fn mapFoldStatementExps<ArgT: Clone + 'static>(mut inStatement: Arc<SCode::Statement>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> (Arc<SCode::Statement>, ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outStatement: Arc<SCode::Statement>;
    let mut outArg: ArgT;
    (outStatement, outArg) = (::match_deref::match_deref! { match &((inStatement.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: e1, value: e2, comment, info }, traverser, arg) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (e2, arg) = traverser(e2.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_ASSIGN { assignComponent: e1.clone(), value: e2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_IF { boolExpr: e1, trueBranch: stmts1, elseIfBranch: branches, elseBranch: stmts2, comment, info }, traverser, arg) => {
            let mut e1 = (*e1).clone();
            let mut branches = (*branches).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (branches, arg) = List::map1Fold(branches.clone(), std::sync::Arc::new(fnptr!(mapFoldBranchExps, (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), _, _)), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_IF { boolExpr: e1.clone(), trueBranch: stmts1.clone(), elseIfBranch: branches.clone(), elseBranch: stmts2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_FOR { index: iterator, range: Some(e1), forBody: stmts1, comment, info }, traverser, arg) => {
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_FOR { index: (iterator.clone()).clone(), range: Some(e1.clone()), forBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_PARFOR { index: iterator, range: Some(e1), parforBody: stmts1, comment, info }, traverser, arg) => {
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_PARFOR { index: (iterator.clone()).clone(), range: Some(e1.clone()), parforBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_WHILE { boolExpr: e1, whileBody: stmts1, comment, info }, traverser, arg) => {
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_WHILE { boolExpr: e1.clone(), whileBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_WHEN_A { branches, comment, info }, traverser, arg) => {
            let mut branches = (*branches).clone();
            let mut arg = (*arg).clone();
            (branches, arg) = List::map1Fold(branches.clone(), std::sync::Arc::new(fnptr!(mapFoldBranchExps, (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), _, _)), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_WHEN_A { branches: branches.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_ASSERT { .. }, traverser, arg) => {
            let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e3: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(var_field!((*inStatement).condition, SCode::Statement::ALG_ASSERT).clone(), arg.clone()).unwrap();
            (e2, arg) = traverser(var_field!((*inStatement).message, SCode::Statement::ALG_ASSERT).clone(), arg.clone()).unwrap();
            (e3, arg) = traverser(var_field!((*inStatement).level, SCode::Statement::ALG_ASSERT).clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: var_field!((*inStatement).comment, SCode::Statement::ALG_ASSERT).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_ASSERT).clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_TERMINATE { .. }, traverser, arg) => {
            let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(var_field!((*inStatement).message, SCode::Statement::ALG_TERMINATE).clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_TERMINATE { message: e1.clone(), comment: var_field!((*inStatement).comment, SCode::Statement::ALG_TERMINATE).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_TERMINATE).clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_REINIT { .. }, traverser, arg) => {
            let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(var_field!((*inStatement).cref, SCode::Statement::ALG_REINIT).clone(), arg.clone()).unwrap();
            (e2, arg) = traverser(var_field!((*inStatement).newValue, SCode::Statement::ALG_REINIT).clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_REINIT { cref: e1.clone(), newValue: e2.clone(), comment: var_field!((*inStatement).comment, SCode::Statement::ALG_REINIT).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_REINIT).clone() }), arg.clone())
        },
        (Deref @ SCode::Statement::ALG_NORETCALL { exp: e1, comment, info }, traverser, arg) => {
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_NORETCALL { exp: e1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => {
            (inStatement.clone(), inArg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outStatement, outArg)
}

fn mapFoldBranchExps<ArgT: Clone + 'static>(mut inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), mut traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>;

    let mut outBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>);
    let mut outArg: ArgT;
    let mut arg: ArgT;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    (exp, stmts) = inBranch.clone();
    (exp, outArg) = traverser(exp.clone(), inArg.clone()).unwrap();
    outBranch = (exp.clone(), stmts.clone());
    (outBranch, outArg)
}

pub fn elementIsClass(mut el: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn elementIsImport(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outIsImport: bool = false;
    outIsImport = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsImport
}

pub fn elementIsPublicImport(mut el: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PUBLIC, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn elementIsProtectedImport(mut el: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PROTECTED, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn getElementClass(mut el: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element>;
    cl = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::CLASS { .. } => el.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cl)
}

pub static knownExternalCFunctions: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("sin")).clone(), (literal!("cos")).clone(), (literal!("tan")).clone(), (literal!("asin")).clone(), (literal!("acos")).clone(), (literal!("atan")).clone(), (literal!("atan2")).clone(), (literal!("sinh")).clone(), (literal!("cosh")).clone(), (literal!("tanh")).clone(), (literal!("exp")).clone(), (literal!("log")).clone(), (literal!("log10")).clone(), (literal!("sqrt")).clone()] });

pub fn isBuiltinFunction(mut cl: Arc<SCode::Element>, mut inVars: Arc<metamodelica::List<ArcStr>>, mut outVars: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &((cl.clone(), inVars.clone(), outVars.clone())) {
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), funcName: None, .. }), .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, name, .. }, _, _) => {
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), funcName: Some(name), .. }), .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, .. }, _, _) => {
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), funcName: None, .. }), .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_PARALLEL_FUNCTION }, name, .. }, _, _) => {
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), funcName: Some(name), .. }), .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_PARALLEL_FUNCTION }, .. }, _, _) => {
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { args, output_: Some(Deref @ Absyn::ComponentRef::CREF_IDENT { name: outVar2, subscripts: Deref @ metamodelica::List::Nil }), lang: Some(Deref @ "C"), funcName: Some(name), .. }), .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, .. }, _, Deref @ metamodelica::List::Cons { head: outVar1, tail: Deref @ metamodelica::List::Nil }) => {
            let mut argsStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let true = (listMember((name.clone()).clone(), knownExternalCFunctions.clone())) else { bail!("pattern mismatch") };
            let true = (outVar2.clone() == outVar1.clone()) else { bail!("pattern mismatch") };
            argsStr = List::mapMap(args.clone(), (std::sync::Arc::new(AbsynUtil::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::ComponentRef>> + 'static>), (std::sync::Arc::new(AbsynUtil::crefIdent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>));
            let true = (argsStr.clone() == inVars.clone()) else { bail!("pattern mismatch") };
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "C"), funcName: None, .. }), .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, name, .. }, _, _) => {
            let true = (listMember((name.clone()).clone(), knownExternalCFunctions.clone())) else { bail!("pattern mismatch") };
            name.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(name)
}

pub fn getEquationInfo(mut inEquation: Arc<SCode::Equation>) -> Result<SourceInfo> {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_IF).clone(),
        Deref @ SCode::Equation::EQ_EQUALS { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_EQUALS).clone(),
        Deref @ SCode::Equation::EQ_PDE { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_PDE).clone(),
        Deref @ SCode::Equation::EQ_CONNECT { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_CONNECT).clone(),
        Deref @ SCode::Equation::EQ_FOR { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_FOR).clone(),
        Deref @ SCode::Equation::EQ_WHEN { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_WHEN).clone(),
        Deref @ SCode::Equation::EQ_ASSERT { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_ASSERT).clone(),
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_TERMINATE).clone(),
        Deref @ SCode::Equation::EQ_REINIT { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_REINIT).clone(),
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_NORETCALL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(info)
}

pub fn getStatementInfo(mut inStatement: Arc<SCode::Statement>) -> Result<SourceInfo> {
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outInfo = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_ASSIGN).clone(),
        Deref @ SCode::Statement::ALG_IF { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_IF).clone(),
        Deref @ SCode::Statement::ALG_FOR { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_FOR).clone(),
        Deref @ SCode::Statement::ALG_PARFOR { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_PARFOR).clone(),
        Deref @ SCode::Statement::ALG_WHILE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_WHILE).clone(),
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_WHEN_A).clone(),
        Deref @ SCode::Statement::ALG_ASSERT { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_ASSERT).clone(),
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_TERMINATE).clone(),
        Deref @ SCode::Statement::ALG_REINIT { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_REINIT).clone(),
        Deref @ SCode::Statement::ALG_NORETCALL { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_NORETCALL).clone(),
        Deref @ SCode::Statement::ALG_RETURN { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_RETURN).clone(),
        Deref @ SCode::Statement::ALG_BREAK { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_BREAK).clone(),
        Deref @ SCode::Statement::ALG_FAILURE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_FAILURE).clone(),
        Deref @ SCode::Statement::ALG_TRY { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_TRY).clone(),
        Deref @ SCode::Statement::ALG_CONTINUE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_CONTINUE).clone(),
        _ => {
            Error::addInternalError((literal!("SCodeUtil.getStatementInfo failed")).clone(), metamodelica::sourceInfo!())?;
            Absyn::dummyInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInfo)
}

pub fn prependSubModToMod(mut subMod: Arc<SCode::SubMod>, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![subMod.clone()], binding: None, comment: None, info: Error::dummyInfo.clone() }),
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = cons(subMod.clone(), var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()));
            r#mod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(r#mod)
}

pub fn addElementToClass(mut inElement: Arc<SCode::Element>, mut inClassDef: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outClassDef: Arc<SCode::Element>;
    let mut cdef: Arc<SCode::ClassDef>;
    let __pa0 = ::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::Element::CLASS { classDef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cdef = __pa0.clone();
    cdef = addElementToCompositeClassDef(inElement.clone(), cdef.clone())?;
    outClassDef = setClassDef(cdef.clone(), inClassDef.clone())?;
    Ok(outClassDef)
}

pub fn addElementToCompositeClassDef(mut element: Arc<SCode::Element>, mut classDef: Arc<SCode::ClassDef>) -> Result<Arc<SCode::ClassDef>> {
    let mut classDef: Arc<SCode::ClassDef> = classDef;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = cons(element.clone(), var_field!((*classDef).elementLst, SCode::ClassDef::PARTS).clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(classDef)
}

pub fn visibilityBool(mut inVisibility: SCode::Visibility) -> Result<bool> {
    let mut bVisibility: bool = false;
    bVisibility = (match inVisibility.clone() {
        SCode::Visibility::PUBLIC => true,
        SCode::Visibility::PROTECTED => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bVisibility)
}

pub fn boolVisibility(mut inBoolVisibility: bool) -> SCode::Visibility {
    let mut outVisibility: SCode::Visibility = SCode::Visibility::PROTECTED;
    outVisibility = (match inBoolVisibility.clone() {
        true => openmodelica_frontend_types::SCode::Visibility::PUBLIC,
        false => openmodelica_frontend_types::SCode::Visibility::PROTECTED,
    });
    outVisibility
}

pub fn visibilityEqual(mut inVisibility1: SCode::Visibility, mut inVisibility2: SCode::Visibility) -> bool {
    let mut outEqual: bool = false;
    outEqual = (match (inVisibility1.clone(), inVisibility2.clone()) {
        (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
        (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
        _ => false,
    });
    outEqual
}

pub fn eachBool(mut inEach: SCode::Each) -> Result<bool> {
    let mut bEach: bool = false;
    bEach = (match inEach.clone() {
        SCode::Each::EACH => true,
        SCode::Each::NOT_EACH => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bEach)
}

pub fn boolEach(mut inBoolEach: bool) -> SCode::Each {
    let mut outEach: SCode::Each = SCode::Each::EACH;
    outEach = (match inBoolEach.clone() {
        true => openmodelica_frontend_types::SCode::Each::EACH,
        false => openmodelica_frontend_types::SCode::Each::NOT_EACH,
    });
    outEach
}

pub fn prefixesRedeclare(mut inPrefixes: Arc<SCode::Prefixes>) -> Result<SCode::Redeclare> {
    let mut outRedeclare: SCode::Redeclare = SCode::Redeclare::NOT_REDECLARE;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::Prefixes { redeclarePrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRedeclare = __pa0.clone();
    Ok(outRedeclare)
}

pub fn prefixesSetRedeclare(mut prefixes: Arc<SCode::Prefixes>, mut inRedeclare: SCode::Redeclare) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.redeclarePrefix = inRedeclare.clone());
    prefixes
}

pub fn prefixesSetReplaceable(mut prefixes: Arc<SCode::Prefixes>, mut inReplaceable: Arc<SCode::Replaceable>) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.replaceablePrefix = inReplaceable.clone());
    prefixes
}

pub fn redeclareBool(mut inRedeclare: SCode::Redeclare) -> Result<bool> {
    let mut bRedeclare: bool = false;
    bRedeclare = (match inRedeclare.clone() {
        SCode::Redeclare::REDECLARE => true,
        SCode::Redeclare::NOT_REDECLARE => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bRedeclare)
}

pub fn boolRedeclare(mut inBoolRedeclare: bool) -> SCode::Redeclare {
    let mut outRedeclare: SCode::Redeclare = SCode::Redeclare::NOT_REDECLARE;
    outRedeclare = (match inBoolRedeclare.clone() {
        true => openmodelica_frontend_types::SCode::Redeclare::REDECLARE,
        false => openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE,
    });
    outRedeclare
}

pub fn replaceableBool(mut inReplaceable: Arc<SCode::Replaceable>) -> Result<bool> {
    let mut bReplaceable: bool = false;
    bReplaceable = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { .. } => true,
        Deref @ SCode::Replaceable::NOT_REPLACEABLE => false,
        _ => bail!("match: no arm matched"),
    } });
    Ok(bReplaceable)
}

pub fn replaceableOptConstraint(mut inReplaceable: Arc<SCode::Replaceable>) -> Result<Option<Arc<SCode::ConstrainClass>>> {
    let mut outOptConstrainClass: Option<Arc<SCode::ConstrainClass>> = None;
    outOptConstrainClass = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc } => {
            cc.clone()
        },
        Deref @ SCode::Replaceable::NOT_REPLACEABLE => {
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outOptConstrainClass)
}

pub fn boolReplaceable(mut inBoolReplaceable: bool, mut inOptConstrainClass: Option<Arc<SCode::ConstrainClass>>) -> Result<Arc<SCode::Replaceable>> {
    let mut outReplaceable: Arc<SCode::Replaceable> = Arc::new(SCode::Replaceable::NOT_REPLACEABLE);
    outReplaceable = (::match_deref::match_deref! { match &((inBoolReplaceable.clone(), inOptConstrainClass.clone())) {
        (true, _) => Arc::new(SCode::Replaceable::REPLACEABLE { cc: inOptConstrainClass.clone() }),
        (false, Some(_)) => {
            println!("{}", (literal!("Ignoring constraint class because replaceable prefix is not present!\n")).clone());
            Arc::new(openmodelica_frontend_types::SCode::Replaceable::NOT_REPLACEABLE)
        },
        (false, _) => Arc::new(openmodelica_frontend_types::SCode::Replaceable::NOT_REPLACEABLE),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outReplaceable)
}

pub fn encapsulatedBool(mut inEncapsulated: SCode::Encapsulated) -> Result<bool> {
    let mut bEncapsulated: bool = false;
    bEncapsulated = (match inEncapsulated.clone() {
        SCode::Encapsulated::ENCAPSULATED => true,
        SCode::Encapsulated::NOT_ENCAPSULATED => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bEncapsulated)
}

pub fn boolEncapsulated(mut inBoolEncapsulated: bool) -> SCode::Encapsulated {
    let mut outEncapsulated: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    outEncapsulated = (match inBoolEncapsulated.clone() {
        true => openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED,
        false => openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
    });
    outEncapsulated
}

pub fn partialBool(mut inPartial: SCode::Partial) -> Result<bool> {
    let mut bPartial: bool = false;
    bPartial = (match inPartial.clone() {
        SCode::Partial::PARTIAL => true,
        SCode::Partial::NOT_PARTIAL => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bPartial)
}

pub fn boolPartial(mut inBoolPartial: bool) -> SCode::Partial {
    let mut outPartial: SCode::Partial = SCode::Partial::NOT_PARTIAL;
    outPartial = (match inBoolPartial.clone() {
        true => openmodelica_frontend_types::SCode::Partial::PARTIAL,
        false => openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
    });
    outPartial
}

pub fn prefixesFinal(mut inPrefixes: Arc<SCode::Prefixes>) -> Result<SCode::Final> {
    let mut outFinal: SCode::Final = SCode::Final::FINAL;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::Prefixes { finalPrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outFinal = __pa0.clone();
    Ok(outFinal)
}

pub fn finalBool(mut inFinal: SCode::Final) -> Result<bool> {
    let mut bFinal: bool = false;
    bFinal = (match inFinal.clone() {
        SCode::Final::FINAL => true,
        SCode::Final::NOT_FINAL => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bFinal)
}

pub fn finalEqual(mut inFinal1: SCode::Final, mut inFinal2: SCode::Final) -> bool {
    let mut bFinal: bool = false;
    bFinal = (match (inFinal1.clone(), inFinal2.clone()) {
        (SCode::Final::FINAL, SCode::Final::FINAL) => true,
        (SCode::Final::NOT_FINAL, SCode::Final::NOT_FINAL) => true,
        _ => false,
    });
    bFinal
}

pub fn boolFinal(mut inBoolFinal: bool) -> SCode::Final {
    let mut outFinal: SCode::Final = SCode::Final::FINAL;
    outFinal = if (inBoolFinal.clone()) {openmodelica_frontend_types::SCode::Final::FINAL} else {openmodelica_frontend_types::SCode::Final::NOT_FINAL};
    outFinal
}

pub fn connectorTypeEqual(mut inConnectorType1: SCode::ConnectorType, mut inConnectorType2: SCode::ConnectorType) -> Result<bool> {
    let mut outEqual: bool = false;
    outEqual = (match (inConnectorType1.clone(), inConnectorType2.clone()) {
        (SCode::ConnectorType::POTENTIAL, SCode::ConnectorType::POTENTIAL) => true,
        (SCode::ConnectorType::FLOW, SCode::ConnectorType::FLOW) => true,
        (SCode::ConnectorType::STREAM, SCode::ConnectorType::STREAM) => true,
        _ => bail!("match: no arm matched"),
    });
    Ok(outEqual)
}

pub fn potentialBool(mut inConnectorType: SCode::ConnectorType) -> bool {
    let mut outPotential: bool = false;
    outPotential = (match inConnectorType.clone() {
        SCode::ConnectorType::POTENTIAL => true,
        _ => false,
    });
    outPotential
}

pub fn flowBool(mut inConnectorType: SCode::ConnectorType) -> bool {
    let mut outFlow: bool = false;
    outFlow = (match inConnectorType.clone() {
        SCode::ConnectorType::FLOW => true,
        _ => false,
    });
    outFlow
}

pub fn boolFlow(mut inBoolFlow: bool) -> SCode::ConnectorType {
    let mut outFlow: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    outFlow = (match inBoolFlow.clone() {
        true => openmodelica_frontend_types::SCode::ConnectorType::FLOW,
        _ => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
    });
    outFlow
}

pub fn streamBool(mut inStream: SCode::ConnectorType) -> bool {
    let mut bStream: bool = false;
    bStream = (match inStream.clone() {
        SCode::ConnectorType::STREAM => true,
        _ => false,
    });
    bStream
}

pub fn boolStream(mut inBoolStream: bool) -> SCode::ConnectorType {
    let mut outStream: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    outStream = (match inBoolStream.clone() {
        true => openmodelica_frontend_types::SCode::ConnectorType::STREAM,
        _ => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
    });
    outStream
}

pub fn mergeAttributesFromClass(mut inAttributes: SCode::Attributes, mut inClass: Arc<SCode::Element>) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    outAttributes = (::match_deref::match_deref! { match &((inAttributes.clone(), inClass.clone())) {
        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { attributes: cls_attr, .. }, .. }) => {
            let mut attr: SCode::Attributes;
            let Some(__pa0) = (mergeAttributes(inAttributes.clone(), Some(cls_attr.clone()))?) else { bail!("pattern mismatch") };
            attr = __pa0.clone();
            attr.clone()
        },
        _ => {
            inAttributes.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAttributes)
}

pub fn mergeAttributes(mut ele: SCode::Attributes, mut oEle: Option<SCode::Attributes>) -> Result<Option<SCode::Attributes>> {
    let mut outoEle: Option<SCode::Attributes> = None;
    outoEle = (match (ele.clone(), oEle.clone()) {
        (_, None) => {
            Some(ele.clone())
        },
        (SCode::Attributes { arrayDims: ref ad1, connectorType: mut ct1, parallelism: mut p1, variability: mut v1, direction: mut d1, isField: mut isf1 }, Some(SCode::Attributes { arrayDims: _, connectorType: mut ct2, parallelism: mut p2, variability: mut v2, direction: mut d2, isField: mut isf2 })) => {
            let mut p: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
            let mut v: SCode::Variability = SCode::Variability::CONST;
            let mut d: Absyn::Direction = Absyn::Direction::BIDIR;
            let mut isf: Absyn::IsField = Absyn::IsField::FIELD;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            let mut ct: SCode::ConnectorType = SCode::ConnectorType::FLOW;
            ct = propagateConnectorType(ct1.clone(), ct2.clone());
            p = propagateParallelism(p1.clone(), p2.clone());
            v = propagateVariability(v1.clone(), v2.clone());
            d = propagateDirection(d1.clone(), d2.clone());
            isf = propagateIsField(isf1.clone(), isf2.clone());
            ad = ad1.clone();
            Some(SCode::Attributes { arrayDims: ad.clone(), connectorType: ct.clone(), parallelism: p.clone(), variability: v.clone(), direction: d.clone(), isField: isf.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outoEle)
}

pub fn prefixesVisibility(mut inPrefixes: Arc<SCode::Prefixes>) -> Result<SCode::Visibility> {
    let mut outVisibility: SCode::Visibility = SCode::Visibility::PROTECTED;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::Prefixes { visibility: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outVisibility = __pa0.clone();
    Ok(outVisibility)
}

pub fn prefixesSetVisibility(mut prefixes: Arc<SCode::Prefixes>, mut inVisibility: SCode::Visibility) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.visibility = inVisibility.clone());
    prefixes
}

pub fn eachEqual(mut each1: SCode::Each, mut each2: SCode::Each) -> bool {
    let mut equal: bool = false;
    equal = (match (each1.clone(), each2.clone()) {
        (SCode::Each::NOT_EACH, SCode::Each::NOT_EACH) => true,
        (SCode::Each::EACH, SCode::Each::EACH) => true,
        _ => false,
    });
    equal
}

pub fn replaceableEqual(mut r1: Arc<SCode::Replaceable>, mut r2: Arc<SCode::Replaceable>) -> Result<bool> {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (r1.clone(), r2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Replaceable::NOT_REPLACEABLE, Deref @ SCode::Replaceable::NOT_REPLACEABLE) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: m1, constrainingClass: p1, .. }) }, Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: m2, constrainingClass: p2, .. }) }) => {
                    let true = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
                    let true = (modEqual(m1.clone(), m2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Replaceable::REPLACEABLE { cc: None }, Deref @ SCode::Replaceable::REPLACEABLE { cc: None }) => {
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
    Ok(equal)
}

pub fn prefixesEqual(mut prefixes1: Arc<SCode::Prefixes>, mut prefixes2: Arc<SCode::Prefixes>) -> Result<bool> {
    let mut equal: bool = false;
    equal = prefixes1.visibility.clone() == prefixes2.visibility.clone() && prefixes1.redeclarePrefix.clone() == prefixes2.redeclarePrefix.clone() && prefixes1.finalPrefix.clone() == prefixes2.finalPrefix.clone() && AbsynUtil::innerOuterEqual(prefixes1.innerOuter.clone(), prefixes2.innerOuter.clone()) && replaceableEqual(prefixes1.replaceablePrefix.clone(), prefixes2.replaceablePrefix.clone())?;
    Ok(equal)
}

pub fn prefixesReplaceable(mut prefixes: Arc<SCode::Prefixes>) -> Result<Arc<SCode::Replaceable>> {
    let mut repl: Arc<SCode::Replaceable> = Arc::new(SCode::Replaceable::NOT_REPLACEABLE);
    let __pa0 = ::match_deref::match_deref! { match &(prefixes.clone()) {
        Deref @ SCode::Prefixes { replaceablePrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    repl = __pa0.clone();
    Ok(repl)
}

pub fn elementPrefixes(mut inElement: Arc<SCode::Element>) -> Result<Arc<SCode::Prefixes>> {
    let mut outPrefixes: Arc<SCode::Prefixes>;
    outPrefixes = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { .. } => var_field!((*inElement).prefixes, SCode::Element::CLASS).clone(),
        Deref @ SCode::Element::COMPONENT { .. } => var_field!((*inElement).prefixes, SCode::Element::COMPONENT).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefixes)
}

pub fn setElementPrefixes(mut prefixes: Arc<SCode::Prefixes>, mut element: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; prefixes = prefixes.clone());
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; prefixes = prefixes.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn isElementReplaceable(mut inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut isReplaceable: bool = false;
    let mut pf: Arc<SCode::Prefixes>;
    pf = elementPrefixes(inElement.clone())?;
    isReplaceable = replaceableBool(prefixesReplaceable(pf.clone())?)?;
    Ok(isReplaceable)
}

pub fn isElementRedeclare(mut inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut isRedeclare: bool = false;
    let mut pf: Arc<SCode::Prefixes>;
    pf = elementPrefixes(inElement.clone())?;
    isRedeclare = redeclareBool(prefixesRedeclare(pf.clone())?)?;
    Ok(isRedeclare)
}

pub fn prefixesInnerOuter(mut inPrefixes: Arc<SCode::Prefixes>) -> Result<Absyn::InnerOuter> {
    let mut outInnerOuter: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::Prefixes { innerOuter: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outInnerOuter = __pa0.clone();
    Ok(outInnerOuter)
}

pub fn prefixesSetInnerOuter(mut prefixes: Arc<SCode::Prefixes>, mut innerOuter: Absyn::InnerOuter) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.innerOuter = innerOuter.clone());
    prefixes
}

pub fn removeAttributeDimensions(mut attributes: SCode::Attributes) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.arrayDims = metamodelica::nil();
    attributes
}

pub fn setAttributesDirection(mut attributes: SCode::Attributes, mut direction: Absyn::Direction) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.direction = direction.clone();
    attributes
}

pub fn attrVariability(mut attr: SCode::Attributes) -> Result<SCode::Variability> {
    let mut var: SCode::Variability = SCode::Variability::CONST;
    var = (match attr.clone() {
        SCode::Attributes { variability: mut v, .. } => {
            v.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(var)
}

pub fn setAttributesVariability(mut attributes: SCode::Attributes, mut variability: SCode::Variability) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.variability = variability.clone();
    attributes
}

pub fn isDerivedClassDef(mut inClassDef: Arc<SCode::ClassDef>) -> bool {
    let mut isDerived: bool = false;
    isDerived = (::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::ClassDef::DERIVED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDerived
}

pub fn isConnector(mut inRestriction: SCode::Restriction) -> bool {
    let mut isConnector: bool = false;
    isConnector = (match inRestriction.clone() {
        SCode::Restriction::R_CONNECTOR { .. } => true,
        _ => false,
    });
    isConnector
}

pub fn removeBuiltinsFromTopScope(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outProgram = List::filterOnTrue(inProgram.clone(), (std::sync::Arc::new(fnptr!(isNotBuiltinClass, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>));
    outProgram
}

fn isNotBuiltinClass(mut inClass: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), .. }), .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn getElementAnnotation(mut element: Arc<SCode::Element>, mut name: ArcStr) -> Option<Arc<SCode::Annotation>> {
    let mut outAnnotation: Option<Arc<SCode::Annotation>> = None;
    outAnnotation = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => var_field!((*element).ann, SCode::Element::EXTENDS).clone(),
        Deref @ SCode::Element::CLASS { .. } => var_field!((*element).cmt, SCode::Element::CLASS).annotation_.clone(),
        Deref @ SCode::Element::COMPONENT { .. } => var_field!((*element).comment, SCode::Element::COMPONENT).annotation_.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAnnotation
}

pub fn lookupAnnotation(mut ann: Arc<SCode::Annotation>, mut name: ArcStr) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut id: ArcStr = arcstr::literal!("");
    r#mod = (::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: submods, .. } } => {
            for mut sm in &*submods.clone() {
                let mut sm = sm.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(sm.clone()) {
                    Deref @ SCode::SubMod { ident: __pa0, r#mod: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                id = __pa0.clone();
                r#mod = __pa1.clone();
                if id.clone() == name.clone() {
                    return Ok(r#mod);
                }
            }
            Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)
        },
        _ => Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn lookupAnnotationBinding(mut ann: Arc<SCode::Annotation>, mut name: ArcStr) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    binding = getModifierBinding(lookupAnnotation(ann.clone(), (name.clone()).clone())?);
    Ok(binding)
}

pub fn lookupBooleanAnnotation(mut ann: Arc<SCode::Annotation>, mut name: ArcStr) -> Result<Option<bool>> {
    let mut value: Option<bool> = None;
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    let mut bval: bool = false;
    binding = lookupAnnotationBinding(ann.clone(), (name.clone()).clone())?;
    value = (::match_deref::match_deref! { match &(binding.clone()) {
        Some(Deref @ Absyn::Exp::BOOL { value: bval }) => Some(bval.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub fn lookupBooleanAnnotationMod(mut r#mod: Arc<SCode::Mod>) -> Option<bool> {
    let mut value: Option<bool> = None;
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    let mut bval: bool = false;
    binding = getModifierBinding(r#mod.clone());
    value = (::match_deref::match_deref! { match &(binding.clone()) {
        Some(Deref @ Absyn::Exp::BOOL { value: bval }) => Some(bval.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

pub fn lookupAnnotations(mut ann: Arc<SCode::Annotation>, mut name: ArcStr) -> Result<Arc<metamodelica::List<Arc<SCode::Mod>>>> {
    let mut mods: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut id: ArcStr = arcstr::literal!("");
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    mods = (::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: submods, .. } } => {
            for mut sm in &*submods.clone() {
                let mut sm = sm.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(sm.clone()) {
                    Deref @ SCode::SubMod { ident: __pa0, r#mod: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                id = __pa0.clone();
                r#mod = __pa1.clone();
                if id.clone() == name.clone() {
                    mods = cons(r#mod.clone(), mods.clone());
                }
            }
            mods.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(mods)
}

pub fn lookupElementAnnotation(mut element: Arc<SCode::Element>, mut name: ArcStr) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut ann: Option<Arc<SCode::Annotation>> = None;
    ann = getElementAnnotation(element.clone(), (name.clone()).clone());
    r#mod = if (isSome(ann.clone())) {lookupAnnotation(Util::getOption(ann.clone())?, (name.clone()).clone())?} else {Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)};
    Ok(r#mod)
}

pub fn lookupElementAnnotationBinding(mut element: Arc<SCode::Element>, mut name: ArcStr) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    binding = getModifierBinding(lookupElementAnnotation(element.clone(), (name.clone()).clone())?);
    Ok(binding)
}

pub fn hasBooleanNamedAnnotationInClass(mut inClass: Arc<SCode::Element>, mut namedAnnotation: ArcStr) -> Result<bool> {
    let mut hasAnn: bool = false;
    hasAnn = (::match_deref::match_deref! { match &((inClass.clone(), namedAnnotation.clone())) {
        (Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. }, _) => {
            hasBooleanNamedAnnotation(ann.clone(), (namedAnnotation.clone()).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasAnn)
}

pub fn hasBooleanNamedAnnotationInComponent(mut inComponent: Arc<SCode::Element>, mut namedAnnotation: ArcStr) -> Result<bool> {
    let mut hasAnn: bool = false;
    hasAnn = (::match_deref::match_deref! { match &((inComponent.clone(), namedAnnotation.clone())) {
        (Deref @ SCode::Element::COMPONENT { comment: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. }, _) => {
            hasBooleanNamedAnnotation(ann.clone(), (namedAnnotation.clone()).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasAnn)
}

pub fn commentAnnotation(mut cmt: Arc<SCode::Comment>) -> Option<Arc<SCode::Annotation>> {
    let mut ann: Option<Arc<SCode::Annotation>> = cmt.annotation_.clone();
    ann
}

pub fn optCommentAnnotation(mut cmt: Option<Arc<SCode::Comment>>) -> Option<Arc<SCode::Annotation>> {
    let mut ann: Option<Arc<SCode::Annotation>> = None;
    ann = (::match_deref::match_deref! { match &(cmt.clone()) {
        Some(Deref @ SCode::Comment { annotation_: ann, .. }) => ann.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ann
}

pub fn optCommentHasBooleanNamedAnnotation(mut comm: Option<Arc<SCode::Comment>>, mut annotationName: ArcStr) -> Result<bool> {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &((comm.clone(), annotationName.clone())) {
        (Some(Deref @ SCode::Comment { annotation_: Some(ann), .. }), _) => {
            hasBooleanNamedAnnotation(ann.clone(), (annotationName.clone()).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

pub fn commentHasBooleanNamedAnnotation(mut comm: Arc<SCode::Comment>, mut annotationName: ArcStr) -> Result<bool> {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &((comm.clone(), annotationName.clone())) {
        (Deref @ SCode::Comment { annotation_: Some(ann), .. }, _) => {
            hasBooleanNamedAnnotation(ann.clone(), (annotationName.clone()).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

pub fn hasBooleanNamedAnnotation(mut inAnnotation: Arc<SCode::Annotation>, mut inName: ArcStr) -> Result<bool> {
    let mut outHasEntry: bool = false;
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    binding = lookupAnnotationBinding(inAnnotation.clone(), (inName.clone()).clone())?;
    outHasEntry = (::match_deref::match_deref! { match &(binding.clone()) {
        Some(Deref @ Absyn::Exp::BOOL { value: true }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outHasEntry)
}

pub fn optCommentHasBooleanNamedAnnotationFalse(mut comm: Option<Arc<SCode::Comment>>, mut annotationName: ArcStr) -> Result<bool> {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &((comm.clone(), annotationName.clone())) {
        (Some(Deref @ SCode::Comment { annotation_: Some(ann), .. }), _) => {
            hasBooleanNamedAnnotationFalse(ann.clone(), (annotationName.clone()).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

pub fn hasBooleanNamedAnnotationFalse(mut inAnnotation: Arc<SCode::Annotation>, mut inName: ArcStr) -> Result<bool> {
    let mut outHasEntry: bool = false;
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    binding = lookupAnnotationBinding(inAnnotation.clone(), (inName.clone()).clone())?;
    outHasEntry = (::match_deref::match_deref! { match &(binding.clone()) {
        Some(Deref @ Absyn::Exp::BOOL { value: false }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outHasEntry)
}

pub fn getEvaluateAnnotation(mut cmt: Arc<SCode::Comment>) -> Result<Option<bool>> {
    let mut value: Option<bool> = None;
    let mut ann: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    let mut binding: Option<Arc<Absyn::Exp>> = None;
    value = (::match_deref::match_deref! { match &(cmt.clone()) {
        Deref @ SCode::Comment { annotation_: Some(ann), .. } => lookupBooleanAnnotation(ann.clone(), (literal!("Evaluate")).clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub fn appendAnnotationToCommentOption(mut inAnnotation: Arc<SCode::Annotation>, mut inComment: Option<Arc<SCode::Comment>>, mut check_replace: bool) -> Result<Option<Arc<SCode::Comment>>> {
    let mut outComment: Option<Arc<SCode::Comment>> = None;
    outComment = (::match_deref::match_deref! { match &(inComment.clone()) {
        Some(comment) => {
            Some(appendAnnotationToComment(inAnnotation.clone(), comment.clone(), check_replace.clone())?)
        },
        _ => {
            Some(Arc::new(SCode::Comment { annotation_: Some(inAnnotation.clone()), comment: None }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

pub fn appendAnnotationToComment(mut inAnnotation: Arc<SCode::Annotation>, mut inComment: Arc<SCode::Comment>, mut check_replace: bool) -> Result<Arc<SCode::Comment>> {
    fn isNotElem(mut r#mod: Arc<SCode::SubMod>, mut mods: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> bool {
        let mut b: bool = true;
        for mut m in &*mods.clone() {
            let mut m = m.clone();
            if r#mod.ident.clone() == m.ident.clone() {
                b = false;
                return b;
            }
        }
        b
    }

    let mut outComment: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    outComment = (::match_deref::match_deref! { match &((inAnnotation.clone(), inComment.clone())) {
        (_, Deref @ SCode::Comment { annotation_: None, comment: cmt }) => {
            Arc::new(SCode::Comment { annotation_: Some(inAnnotation.clone()), comment: cmt.clone() })
        },
        (Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: mods1, .. } }, Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: r#mod @ Deref @ SCode::Mod::MOD { .. } }), comment: cmt }) => {
            let mut r#mod = (*r#mod).clone();
            if !(check_replace.clone()) {
                assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = listAppend(mods1.clone(), var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()));
            } else {
                assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = listAppend(mods1.clone(), List::filterOnTrue(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), Arc::new({ let __pe_b1 = mods1.clone(); move |__pe_a0| Ok(isNotElem(__pe_a0, __pe_b1.clone())) }))));
            }
            Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: r#mod.clone() })), comment: cmt.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComment)
}

pub fn getModifierInfo(mut inMod: Arc<SCode::Mod>) -> SourceInfo {
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outInfo = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { info, .. } => {
            info.clone()
        },
        Deref @ SCode::Mod::REDECL { element: el, .. } => {
            elementInfo(el.clone())
        },
        Deref @ SCode::Mod::BREAK_COMPONENT { .. } => {
            var_field!((*inMod).info, SCode::Mod::BREAK_COMPONENT).clone()
        },
        Deref @ SCode::Mod::BREAK_CONNECT { .. } => {
            var_field!((*inMod).info, SCode::Mod::BREAK_CONNECT).clone()
        },
        _ => {
            Absyn::dummyInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInfo
}

pub fn getModifierBinding(mut inMod: Arc<SCode::Mod>) -> Option<Arc<Absyn::Exp>> {
    let mut outBinding: Option<Arc<Absyn::Exp>> = None;
    outBinding = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => var_field!((*inMod).binding, SCode::Mod::MOD).clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBinding
}

pub fn setModifierBinding(mut binding: Option<Arc<Absyn::Exp>>, mut r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; binding = binding.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

pub fn getComponentCondition(mut element: Arc<SCode::Element>) -> Option<Arc<Absyn::Exp>> {
    let mut condition: Option<Arc<Absyn::Exp>> = None;
    condition = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => var_field!((*element).condition, SCode::Element::COMPONENT).clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    condition
}

pub fn removeComponentCondition(mut element: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; condition = None);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn isInnerComponent(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outIsInner: bool = false;
    outIsInner = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } => {
            AbsynUtil::isInner(io.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsInner
}

pub fn makeElementProtected(mut element: Arc<SCode::Element>) -> Arc<SCode::Element> {
    let mut element: Arc<SCode::Element> = element;
    let mut prefixes: Arc<SCode::Prefixes>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { prefixes: prefixes @ Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC, .. }, .. } => {
            let mut prefixes = (*prefixes).clone();
            assign_field!(prefixes.visibility = openmodelica_frontend_types::SCode::Visibility::PROTECTED);
            assign_variant_field!(element => SCode::Element::COMPONENT; prefixes = prefixes.clone());
            ()
        },
        Deref @ SCode::Element::EXTENDS { visibility: SCode::Visibility::PUBLIC, .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; visibility = openmodelica_frontend_types::SCode::Visibility::PROTECTED);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    element
}

pub fn isElementPublic(mut inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsPublic: bool = false;
    outIsPublic = visibilityBool(elementVisibility(inElement.clone())?)?;
    Ok(outIsPublic)
}

pub fn isElementProtected(mut inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsProtected: bool = false;
    outIsProtected = !(visibilityBool(elementVisibility(inElement.clone())?)?);
    Ok(outIsProtected)
}

pub fn isElementEncapsulated(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outIsEncapsulated: bool = false;
    outIsEncapsulated = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEncapsulated
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getElementsFromElement(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inElement: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outProgram = (::match_deref::match_deref! { match &((inProgram.clone(), inElement.clone())) {
        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, .. }) => {
            els.clone()
        },
        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, .. }, .. }) => {
            els.clone()
        },
        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: p, .. }, .. }, .. }) => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut e: Arc<SCode::Element>;
            e = getElementWithPath(inProgram.clone(), p.clone())?;
            els = getElementsFromElement(inProgram.clone(), e.clone())?;
            els.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outProgram)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getElementWithId(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inId: ArcStr) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &((inProgram.clone(), inId.clone())) {
        (Deref @ metamodelica::List::Cons { head: e @ Deref @ SCode::Element::CLASS { name: n, .. }, tail: _ }, i) if (stringEq((n.clone()).clone(), (i.clone()).clone())) => {
            e.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e @ Deref @ SCode::Element::COMPONENT { name: n, .. }, tail: _ }, i) if (stringEq((n.clone()).clone(), (i.clone()).clone())) => {
            e.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e @ Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: _ }, i) if (stringEq((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone(), (i.clone()).clone())) => {
            e.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, i) => {
            getElementWithId(rest.clone(), (i.clone()).clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getElementWithPath(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &((inProgram.clone(), inPath.clone())) {
        (_, Deref @ Absyn::Path::FULLYQUALIFIED { path: p }) => {
            getElementWithPath(inProgram.clone(), p.clone())?
        },
        (_, Deref @ Absyn::Path::IDENT { name: i }) => {
            let mut e: Arc<SCode::Element>;
            e = getElementWithId(inProgram.clone(), (i.clone()).clone())?;
            e.clone()
        },
        (_, Deref @ Absyn::Path::QUALIFIED { name: i, path: p }) => {
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut e: Arc<SCode::Element>;
            e = getElementWithId(inProgram.clone(), (i.clone()).clone())?;
            sp = getElementsFromElement(inProgram.clone(), e.clone())?;
            e = getElementWithPath(sp.clone(), p.clone())?;
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

pub fn getElementName(mut e: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { name: s, .. } => {
            s.clone()
        },
        Deref @ SCode::Element::CLASS { name: s, .. } => {
            s.clone()
        },
        Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. } => {
            AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(s)
}

pub fn getElementTypePath(mut element: Arc<SCode::Element>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => AbsynUtil::typeSpecPath(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone())?,
        Deref @ SCode::Element::EXTENDS { .. } => var_field!((*element).baseClassPath, SCode::Element::EXTENDS).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(path)
}

pub fn setBaseClassPath(mut element: Arc<SCode::Element>, mut inBcPath: Arc<Absyn::Path>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; baseClassPath = inBcPath.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn getBaseClassPath(mut inE: Arc<SCode::Element>) -> Result<Arc<Absyn::Path>> {
    let mut outBcPath: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outBcPath = __pa0.clone();
    Ok(outBcPath)
}

pub fn setComponentTypeSpec(mut element: Arc<SCode::Element>, mut typeSpec: Arc<Absyn::TypeSpec>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; typeSpec = typeSpec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn getComponentTypeSpec(mut inE: Arc<SCode::Element>) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::Element::COMPONENT { typeSpec: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outTypeSpec = __pa0.clone();
    Ok(outTypeSpec)
}

pub fn setComponentMod(mut element: Arc<SCode::Element>, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = r#mod.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn getComponentMod(mut inE: Arc<SCode::Element>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::Element::COMPONENT { modifications: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outMod = __pa0.clone();
    Ok(outMod)
}

pub fn isDerivedClass(mut inClass: Arc<SCode::Element>) -> bool {
    let mut isDerived: bool = false;
    isDerived = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDerived
}

pub fn isClassExtends(mut cls: Arc<SCode::Element>) -> bool {
    let mut isCE: bool = false;
    isCE = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCE
}

pub fn getDerivedTypeSpec(mut inE: Arc<SCode::Element>) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outTypeSpec = __pa0.clone();
    Ok(outTypeSpec)
}

pub fn getDerivedMod(mut inE: Arc<SCode::Element>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outMod = __pa0.clone();
    Ok(outMod)
}

pub fn setClassPrefixes(mut prefixes: Arc<SCode::Prefixes>, mut cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(cl => SCode::Element::CLASS; prefixes = prefixes.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn getClassDef(mut inClass: Arc<SCode::Element>) -> Result<Arc<SCode::ClassDef>> {
    let mut outCdef: Arc<SCode::ClassDef>;
    outCdef = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { classDef: outCdef, .. } => outCdef.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCdef)
}

pub fn setClassDef(mut classDef: Arc<SCode::ClassDef>, mut cls: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cls: Arc<SCode::Element> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(cls => SCode::Element::CLASS; classDef = classDef.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn getClassBody(mut inClass: Arc<SCode::Element>) -> Result<Arc<SCode::ClassDef>> {
    let mut outCdef: Arc<SCode::ClassDef>;
    outCdef = getClassDef(inClass.clone())?;
    outCdef = (::match_deref::match_deref! { match &(outCdef.clone()) {
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => var_field!((*outCdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(),
        _ => outCdef.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCdef)
}

pub fn equationsContainReinit(mut inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> bool {
    let mut hasReinit: bool = false;
    hasReinit = (::match_deref::match_deref! { match &(inEqs.clone()) {
        _ => {
            let mut b: bool = false;
            b = List::applyAndFold(inEqs.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(equationContainReinit, Arc<SCode::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<bool> + 'static>), false);
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

pub fn equationContainReinit(mut inEq: Arc<SCode::Equation>) -> bool {
    let mut hasReinit: bool = false;
    hasReinit = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            true
        },
        Deref @ SCode::Equation::EQ_WHEN { elseBranches: tpl_el, eEquationLst: eqs, .. } => {
            let mut b: bool = false;
            let mut eqs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
            b = equationsContainReinit(eqs.clone());
            eqs_lst = List::map(tpl_el.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
            b = List::applyAndFold(eqs_lst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(equationsContainReinit, Arc<metamodelica::List<Arc<SCode::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<bool> + 'static>), b.clone());
            b.clone()
        },
        Deref @ SCode::Equation::EQ_IF { elseBranch: eqs, thenBranch: eqs_lst, .. } => {
            let mut b: bool = false;
            b = equationsContainReinit(eqs.clone());
            b = List::applyAndFold(eqs_lst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(equationsContainReinit, Arc<metamodelica::List<Arc<SCode::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<bool> + 'static>), b.clone());
            b.clone()
        },
        Deref @ SCode::Equation::EQ_FOR { eEquationLst: eqs, .. } => {
            let mut b: bool = false;
            b = equationsContainReinit(eqs.clone());
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

pub fn algorithmsContainReinit(mut inAlgs: Arc<metamodelica::List<Arc<SCode::Statement>>>) -> bool {
    let mut hasReinit: bool = false;
    hasReinit = (::match_deref::match_deref! { match &(inAlgs.clone()) {
        _ => {
            let mut b: bool = false;
            b = List::applyAndFold(inAlgs.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(algorithmContainReinit, Arc<SCode::Statement>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<bool> + 'static>), false);
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

pub fn algorithmContainReinit(mut inAlg: Arc<SCode::Statement>) -> bool {
    let mut hasReinit: bool = false;
    hasReinit = (::match_deref::match_deref! { match &(inAlg.clone()) {
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            true
        },
        Deref @ SCode::Statement::ALG_WHEN_A { branches: tpl_alg, .. } => {
            let mut b: bool = false;
            let mut algs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>> = metamodelica::nil();
            algs_lst = List::map(tpl_alg.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
            b = List::applyAndFold(algs_lst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(algorithmsContainReinit, Arc<metamodelica::List<Arc<SCode::Statement>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<SCode::Statement>>>) -> Result<bool> + 'static>), false);
            b.clone()
        },
        Deref @ SCode::Statement::ALG_IF { elseBranch: algs2, elseIfBranch: tpl_alg, trueBranch: algs1, .. } => {
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut b3: bool = false;
            let mut algs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>> = metamodelica::nil();
            b1 = algorithmsContainReinit(algs1.clone());
            algs_lst = List::map(tpl_alg.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)));
            b2 = List::applyAndFold(algs_lst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(algorithmsContainReinit, Arc<metamodelica::List<Arc<SCode::Statement>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<SCode::Statement>>>) -> Result<bool> + 'static>), b1.clone());
            b3 = algorithmsContainReinit(algs2.clone());
            b = boolOr(b1.clone(), boolOr(b2.clone(), b3.clone()));
            b.clone()
        },
        Deref @ SCode::Statement::ALG_FOR { forBody: algs, .. } => {
            let mut b: bool = false;
            b = algorithmsContainReinit(algs.clone());
            b.clone()
        },
        Deref @ SCode::Statement::ALG_WHILE { whileBody: algs, .. } => {
            let mut b: bool = false;
            b = algorithmsContainReinit(algs.clone());
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

pub fn getClassPartialPrefix(mut inElement: Arc<SCode::Element>) -> Result<SCode::Partial> {
    let mut outPartial: SCode::Partial = SCode::Partial::NOT_PARTIAL;
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { partialPrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outPartial = __pa0.clone();
    Ok(outPartial)
}

pub fn getClassRestriction(mut inElement: Arc<SCode::Element>) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { restriction: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRestriction = __pa0.clone();
    Ok(outRestriction)
}

pub fn isRedeclareSubMod(mut inSubMod: Arc<SCode::SubMod>) -> bool {
    let mut outIsRedeclare: bool = false;
    outIsRedeclare = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::REDECL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsRedeclare
}

pub fn isBreakSubMod(mut subMod: Arc<SCode::SubMod>) -> bool {
    let mut isBreak: bool = false;
    isBreak = (::match_deref::match_deref! { match &(subMod.r#mod.clone()) {
        Deref @ SCode::Mod::BREAK_COMPONENT { .. } => true,
        Deref @ SCode::Mod::BREAK_CONNECT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub fn isBreakComponentSubMod(mut subMod: Arc<SCode::SubMod>) -> bool {
    let mut isBreak: bool = false;
    isBreak = (::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::BREAK_COMPONENT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub fn isBreakConnectSubMod(mut subMod: Arc<SCode::SubMod>) -> bool {
    let mut isBreak: bool = false;
    isBreak = (::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::BREAK_CONNECT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub fn componentMod(mut inElement: Arc<SCode::Element>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { modifications: r#mod, .. } => {
            r#mod.clone()
        },
        _ => {
            Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn elementMod(mut inElement: Arc<SCode::Element>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { modifications: r#mod, .. } => {
            r#mod.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, .. }, .. } => {
            r#mod.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: r#mod, .. }, .. } => {
            r#mod.clone()
        },
        Deref @ SCode::Element::EXTENDS { modifications: r#mod, .. } => {
            r#mod.clone()
        },
        _ => {
            Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn setElementMod(mut element: Arc<SCode::Element>, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = r#mod.clone());
            ()
        },
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; classDef = setClassDefMod(var_field!((*element).classDef, SCode::Element::CLASS).clone(), r#mod.clone()));
            ()
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = r#mod.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

fn setClassDefMod(mut classDef: Arc<SCode::ClassDef>, mut inMod: Arc<SCode::Mod>) -> Arc<SCode::ClassDef> {
    let mut classDef: Arc<SCode::ClassDef> = classDef;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::ClassDef::DERIVED { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::DERIVED; modifications = inMod.clone());
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; modifications = inMod.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    classDef
}

pub fn isBuiltinElement(mut inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsBuiltin: bool = false;
    outIsBuiltin = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), .. }), .. }, .. } => {
            true
        },
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            hasBooleanNamedAnnotation(ann.clone(), (literal!("__OpenModelica_builtin")).clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsBuiltin)
}

pub fn isExternalFunctionRestriction(mut inRestr: SCode::FunctionRestriction) -> bool {
    let mut isExternal: bool = false;
    isExternal = (match inRestr.clone() {
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } => true,
        _ => false,
    });
    isExternal
}

pub fn isImpureFunctionRestriction(mut inRestr: SCode::FunctionRestriction) -> bool {
    let mut isExternal: bool = false;
    isExternal = (match inRestr.clone() {
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => true,
        SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => true,
        _ => false,
    });
    isExternal
}

pub fn isRestrictionImpure(mut inRestr: SCode::Restriction, mut hasZeroOutputPreMSL3_2: bool) -> bool {
    let mut isImpure: bool = false;
    isImpure = (match inRestr.clone() {
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } } => true,
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } } => true,
        SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::NO_PURITY { .. } } } => !(hasZeroOutputPreMSL3_2.clone()),
        _ => false,
    });
    isImpure
}

pub fn getFunctionRestrictionPurity(mut restr: SCode::FunctionRestriction) -> Absyn::FunctionPurity {
    let mut purity: Absyn::FunctionPurity = Absyn::FunctionPurity::IMPURE;
    purity = (match restr.clone() {
        SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: mut purity } => purity.clone(),
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: mut purity } => purity.clone(),
        _ => openmodelica_ast::Absyn::FunctionPurity::NO_PURITY,
    });
    purity
}

pub fn elementInnerOuter(mut element: Arc<SCode::Element>) -> Result<Absyn::InnerOuter> {
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    io = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => prefixesInnerOuter(var_field!((*element).prefixes, SCode::Element::CLASS).clone())?,
        Deref @ SCode::Element::COMPONENT { .. } => prefixesInnerOuter(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone())?,
        _ => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(io)
}

pub fn elementVisibility(mut element: Arc<SCode::Element>) -> Result<SCode::Visibility> {
    let mut visibility: SCode::Visibility = SCode::Visibility::PROTECTED;
    visibility = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => var_field!((*element).visibility, SCode::Element::IMPORT).clone(),
        Deref @ SCode::Element::EXTENDS { .. } => var_field!((*element).visibility, SCode::Element::EXTENDS).clone(),
        Deref @ SCode::Element::CLASS { .. } => prefixesVisibility(var_field!((*element).prefixes, SCode::Element::CLASS).clone())?,
        Deref @ SCode::Element::COMPONENT { .. } => prefixesVisibility(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone())?,
        Deref @ SCode::Element::DEFINEUNIT { .. } => var_field!((*element).visibility, SCode::Element::DEFINEUNIT).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(visibility)
}

pub fn isClassNamed(mut inName: ArcStr, mut inClass: Arc<SCode::Element>) -> bool {
    let mut outIsNamed: bool = false;
    outIsNamed = (::match_deref::match_deref! { match &((inName.clone(), inClass.clone())) {
        (_, Deref @ SCode::Element::CLASS { name, .. }) => {
            stringEq((inName.clone()).clone(), (name.clone()).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

pub fn isElementNamed(mut name: ArcStr, mut element: Arc<SCode::Element>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => var_field!((*element).name, SCode::Element::CLASS).clone() == name.clone(),
        Deref @ SCode::Element::COMPONENT { .. } => var_field!((*element).name, SCode::Element::COMPONENT).clone() == name.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn getElementComment(mut inElement: Arc<SCode::Element>) -> Option<Arc<SCode::Comment>> {
    let mut outComment: Option<Arc<SCode::Comment>> = None;
    outComment = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { comment: cmt, .. } => {
            Some(cmt.clone())
        },
        Deref @ SCode::Element::CLASS { cmt, .. } => {
            Some(cmt.clone())
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            Some(Arc::new(SCode::Comment { annotation_: var_field!((*inElement).ann, SCode::Element::EXTENDS).clone(), comment: None }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComment
}

pub fn stripAnnotationFromComment(mut inComment: Option<Arc<SCode::Comment>>) -> Option<Arc<SCode::Comment>> {
    let mut outComment: Option<Arc<SCode::Comment>> = None;
    outComment = (::match_deref::match_deref! { match &(inComment.clone()) {
        Some(Deref @ SCode::Comment { annotation_: _, comment: r#str }) => {
            Some(Arc::new(SCode::Comment { annotation_: None, comment: r#str.clone() }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComment
}

pub fn isOverloadedFunction(mut inElement: Arc<SCode::Element>) -> bool {
    let mut isOverloaded: bool = false;
    isOverloaded = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::OVERLOAD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOverloaded
}

pub fn mergeWithOriginal(mut newClass: Arc<SCode::Element>, mut oldClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut newClass: Arc<SCode::Element> = newClass;
    let () = 'mc: {
        let __mc_input = (newClass.clone(), oldClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (isFunction(newClass.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: cd1, prefixes: prefixes1, .. }, Deref @ SCode::Element::CLASS { classDef: cd2, prefixes: prefixes2, .. }) => {
                    let mut mCCNew: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut mCCOld: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut newClass: Arc<SCode::Element> = newClass.clone();
                    mCCNew = getConstrainedByModifiers(prefixes1.clone());
                    mCCOld = getConstrainedByModifiers(prefixes2.clone());
                    assign_variant_field!(newClass => SCode::Element::CLASS;
                        classDef = mergeClassDef(cd1.clone(), cd2.clone(), mCCNew.clone(), mCCOld.clone())?,
                        prefixes = propagatePrefixes(prefixes1.clone(), prefixes2.clone())?
                    );
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(newClass)
}

pub fn getConstrainedByModifiers(mut inPrefixes: Arc<SCode::Prefixes>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: m, .. }) }, .. } => {
            m.clone()
        },
        _ => {
            Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn mergeClassDef(mut inNew: Arc<SCode::ClassDef>, mut inOld: Arc<SCode::ClassDef>, mut inCCModNew: Arc<SCode::Mod>, mut inCCModOld: Arc<SCode::Mod>) -> Result<Arc<SCode::ClassDef>> {
    let mut outNew: Arc<SCode::ClassDef>;
    outNew = (::match_deref::match_deref! { match &((inNew.clone(), inOld.clone(), inCCModNew.clone(), inCCModOld.clone())) {
        (Deref @ SCode::ClassDef::DERIVED { typeSpec: ts1, modifications: m1, attributes: a1 }, Deref @ SCode::ClassDef::DERIVED { typeSpec: _, modifications: m2, attributes: a2 }, _, _) => {
            let mut n: Arc<SCode::ClassDef>;
            let mut m1 = (*m1).clone();
            let mut m2 = (*m2).clone();
            let mut a2 = (*a2).clone();
            m2 = mergeModifiers(m2.clone(), inCCModOld.clone())?;
            m1 = mergeModifiers(m1.clone(), inCCModNew.clone())?;
            m2 = mergeModifiers(m1.clone(), m2.clone())?;
            a2 = propagateAttributes(a2.clone(), a1.clone(), false)?;
            n = Arc::new(SCode::ClassDef::DERIVED { typeSpec: ts1.clone(), modifications: m2.clone(), attributes: a2.clone() });
            n.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNew)
}

pub fn mergeModifiers(mut inNewMod: Arc<SCode::Mod>, mut inOldMod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inNewMod.clone(), inOldMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::NOMOD) => {
                    Ok(inNewMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::NOMOD, _) => {
                    Ok(inOldMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::REDECL { .. }, _) => {
                    Ok(inNewMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::MOD { finalPrefix: f1, eachPrefix: e1, subModLst: sl1, binding: b1, comment: cmt, info: i1 }, Deref @ SCode::Mod::MOD { finalPrefix: f2, eachPrefix: e2, subModLst: sl2, binding: b2, comment: _, .. }) => {
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    let mut b: Option<Arc<Absyn::Exp>> = None;
                    let mut m: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    b = if (isSome(b1.clone())) {b1.clone()} else {b2.clone()};
                    sl = mergeSubMods(sl1.clone(), sl2.clone())?;
                    if referenceEq(&b.clone(),&b1.clone()) && referenceEq(&sl.clone(),&sl1.clone()) {
                        m = inNewMod.clone();
                    } else if referenceEq(&b.clone(),&b2.clone()) && referenceEq(&sl.clone(),&sl2.clone()) && f1.clone() == f2.clone() && e1.clone() == e2.clone() {
                        m = inOldMod.clone();
                    } else {
                        m = Arc::new(SCode::Mod::MOD { finalPrefix: f1.clone(), eachPrefix: e1.clone(), subModLst: sl.clone(), binding: b.clone(), comment: cmt.clone(), info: i1.clone() });
                    }
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNewMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn mergeSubMods(mut inNew: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inOld: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    outSubs = 'mc: {
        let __mc_input = (inNew.clone(), inOld.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inOld.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: s, tail: rest }, _) => {
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    let mut old: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    old = removeSub(s.clone(), inOld.clone())?;
                    sl = mergeSubMods(rest.clone(), old.clone())?;
                    Ok(cons(s.clone(), sl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNew.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSubs)
}

fn removeSub(mut inSub: Arc<SCode::SubMod>, mut inOld: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    outSubs = 'mc: {
        let __mc_input = (inSub.clone(), inOld.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(inOld.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::SubMod { ident: id1, .. }, Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: id2, .. }, tail: rest }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(rest.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: s, tail: rest }) => {
                    let mut rest = (*rest).clone();
                    rest = removeSub(inSub.clone(), rest.clone())?;
                    Ok(cons(s.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSubs)
}

pub fn mergeComponentModifiers(mut newComp: Arc<SCode::Element>, mut oldComp: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut newComp: Arc<SCode::Element> = newComp;
    let () = (::match_deref::match_deref! { match &((newComp.clone(), oldComp.clone())) {
        (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            assign_variant_field!(newComp => SCode::Element::COMPONENT; modifications = mergeModifiers(var_field!((*newComp).modifications, SCode::Element::COMPONENT).clone(), var_field!((*oldComp).modifications, SCode::Element::COMPONENT).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newComp)
}

pub fn propagateAttributes(mut inOriginalAttributes: SCode::Attributes, mut inNewAttributes: SCode::Attributes, mut inNewTypeIsArray: bool) -> Result<SCode::Attributes> {
    let mut outNewAttributes: SCode::Attributes;
    let mut dims1: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut dims2: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut ct1: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    let mut ct2: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    let mut prl1: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    let mut prl2: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    let mut var1: SCode::Variability = SCode::Variability::CONST;
    let mut var2: SCode::Variability = SCode::Variability::CONST;
    let mut dir1: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut dir2: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut if1: Absyn::IsField = Absyn::IsField::FIELD;
    let mut if2: Absyn::IsField = Absyn::IsField::FIELD;
    let SCode::ATTR { arrayDims: __pa0, connectorType: __pa1, parallelism: __pa2, variability: __pa3, direction: __pa4, isField: __pa5 } = (inOriginalAttributes.clone()) else { bail!("pattern mismatch") };
    dims1 = __pa0.clone();
    ct1 = __pa1.clone();
    prl1 = __pa2.clone();
    var1 = __pa3.clone();
    dir1 = __pa4.clone();
    if1 = __pa5.clone();
    let SCode::ATTR { arrayDims: __pa6, connectorType: __pa7, parallelism: __pa8, variability: __pa9, direction: __pa10, isField: __pa11 } = (inNewAttributes.clone()) else { bail!("pattern mismatch") };
    dims2 = __pa6.clone();
    ct2 = __pa7.clone();
    prl2 = __pa8.clone();
    var2 = __pa9.clone();
    dir2 = __pa10.clone();
    if2 = __pa11.clone();
    if !(inNewTypeIsArray.clone()) {
        dims2 = propagateArrayDimensions(dims1.clone(), dims2.clone());
    }
    ct2 = propagateConnectorType(ct1.clone(), ct2.clone());
    prl2 = propagateParallelism(prl1.clone(), prl2.clone());
    var2 = propagateVariability(var1.clone(), var2.clone());
    dir2 = propagateDirection(dir1.clone(), dir2.clone());
    if2 = propagateIsField(if1.clone(), if2.clone());
    outNewAttributes = SCode::Attributes { arrayDims: dims2.clone(), connectorType: ct2.clone(), parallelism: prl2.clone(), variability: var2.clone(), direction: dir2.clone(), isField: if2.clone() };
    Ok(outNewAttributes)
}

pub fn propagateArrayDimensions(mut inOriginalDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inNewDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outNewDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    outNewDims = (::match_deref::match_deref! { match &((inOriginalDims.clone(), inNewDims.clone())) {
        (_, Deref @ metamodelica::List::Nil) => inOriginalDims.clone(),
        _ => inNewDims.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNewDims
}

pub fn propagateConnectorType(mut inOriginalConnectorType: SCode::ConnectorType, mut inNewConnectorType: SCode::ConnectorType) -> SCode::ConnectorType {
    let mut outNewConnectorType: SCode::ConnectorType = SCode::ConnectorType::FLOW;
    outNewConnectorType = (match inNewConnectorType.clone() {
        SCode::ConnectorType::POTENTIAL => inOriginalConnectorType.clone(),
        _ => inNewConnectorType.clone(),
    });
    outNewConnectorType
}

pub fn propagateParallelism(mut inOriginalParallelism: SCode::Parallelism, mut inNewParallelism: SCode::Parallelism) -> SCode::Parallelism {
    let mut outNewParallelism: SCode::Parallelism = SCode::Parallelism::NON_PARALLEL;
    outNewParallelism = (match inNewParallelism.clone() {
        SCode::Parallelism::NON_PARALLEL => inOriginalParallelism.clone(),
        _ => inNewParallelism.clone(),
    });
    outNewParallelism
}

pub fn propagateVariability(mut inOriginalVariability: SCode::Variability, mut inNewVariability: SCode::Variability) -> SCode::Variability {
    let mut outNewVariability: SCode::Variability = SCode::Variability::CONST;
    outNewVariability = (match inNewVariability.clone() {
        SCode::Variability::VAR => inOriginalVariability.clone(),
        _ => inNewVariability.clone(),
    });
    outNewVariability
}

pub fn propagateDirection(mut inOriginalDirection: Absyn::Direction, mut inNewDirection: Absyn::Direction) -> Absyn::Direction {
    let mut outNewDirection: Absyn::Direction = Absyn::Direction::BIDIR;
    outNewDirection = (match inNewDirection.clone() {
        Absyn::Direction::BIDIR => inOriginalDirection.clone(),
        _ => inNewDirection.clone(),
    });
    outNewDirection
}

pub fn propagateIsField(mut inOriginalIsField: Absyn::IsField, mut inNewIsField: Absyn::IsField) -> Absyn::IsField {
    let mut outNewIsField: Absyn::IsField = Absyn::IsField::FIELD;
    outNewIsField = (match inNewIsField.clone() {
        Absyn::IsField::NONFIELD => inOriginalIsField.clone(),
        _ => inNewIsField.clone(),
    });
    outNewIsField
}

pub fn propagateAttributesVar(mut originalVar: Arc<SCode::Element>, mut newVar: Arc<SCode::Element>, mut isNewTypeArray: bool) -> Result<Arc<SCode::Element>> {
    let mut newVar: Arc<SCode::Element> = newVar;
    let () = (::match_deref::match_deref! { match &((originalVar.clone(), newVar.clone())) {
        (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            assign_variant_field!(newVar => SCode::Element::COMPONENT;
                prefixes = propagatePrefixes(var_field!((*originalVar).prefixes, SCode::Element::COMPONENT).clone(), var_field!((*newVar).prefixes, SCode::Element::COMPONENT).clone())?,
                attributes = propagateAttributes(var_field!((*originalVar).attributes, SCode::Element::COMPONENT).clone(), var_field!((*newVar).attributes, SCode::Element::COMPONENT).clone(), isNewTypeArray.clone())?
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newVar)
}

pub fn propagateAttributesClass(mut originalClass: Arc<SCode::Element>, mut newClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut newClass: Arc<SCode::Element> = newClass;
    let () = (::match_deref::match_deref! { match &((originalClass.clone(), newClass.clone())) {
        (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => {
            assign_variant_field!(newClass => SCode::Element::CLASS; prefixes = propagatePrefixes(var_field!((*originalClass).prefixes, SCode::Element::CLASS).clone(), var_field!((*newClass).prefixes, SCode::Element::CLASS).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newClass)
}

pub fn propagatePrefixes(mut originalPrefixes: Arc<SCode::Prefixes>, mut newPrefixes: Arc<SCode::Prefixes>) -> Result<Arc<SCode::Prefixes>> {
    let mut newPrefixes: Arc<SCode::Prefixes> = newPrefixes;
    let () = (::match_deref::match_deref! { match &((originalPrefixes.clone(), newPrefixes.clone())) {
        (Deref @ SCode::Prefixes { .. }, Deref @ SCode::Prefixes { .. }) => {
            assign_field!(newPrefixes.innerOuter = propagatePrefixInnerOuter(originalPrefixes.innerOuter.clone(), newPrefixes.innerOuter.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newPrefixes)
}

pub fn propagatePrefixInnerOuter(mut inOriginalIO: Absyn::InnerOuter, mut inIO: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut outIO: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    outIO = (match inIO.clone() {
        Absyn::InnerOuter::NOT_INNER_OUTER => inOriginalIO.clone(),
        _ => inIO.clone(),
    });
    outIO
}

pub fn isPackage(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isPartial(mut inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::PARTIAL, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isValidPackageElement(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outIsValid: bool = false;
    outIsValid = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::CONST, .. }, .. } => true,
        Deref @ SCode::Element::COMPONENT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsValid
}

pub fn classIsExternalObject(mut cl: Arc<SCode::Element>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, .. } => {
            isExternalObject(els.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isExternalObject(mut els: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool = false;
    res = if ((els.clone().len() as i32) == 3) {hasExtendsOfExternalObject(els.clone()) && hasExternalObjectDestructor(els.clone()) && hasExternalObjectConstructor(els.clone())} else {false};
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasExtendsOfExternalObject(mut inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: path, .. }, tail: _ } if (AbsynUtil::pathEqual(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("ExternalObject")).clone() }))) => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: els } => {
            hasExtendsOfExternalObject(els.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasExternalObjectDestructor(mut inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { name: Deref @ "destructor", .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: els } => {
            hasExternalObjectDestructor(els.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasExternalObjectConstructor(mut inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { name: Deref @ "constructor", .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: els } => {
            hasExternalObjectConstructor(els.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn getExternalObjectDestructor(mut inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element>;
    cl = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: cl @ Deref @ SCode::Element::CLASS { name: Deref @ "destructor", .. }, tail: _ } => {
            cl.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: els } => {
            getExternalObjectDestructor(els.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn getExternalObjectConstructor(mut inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element>;
    cl = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: cl @ Deref @ SCode::Element::CLASS { name: Deref @ "constructor", .. }, tail: _ } => {
            cl.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: els } => {
            getExternalObjectConstructor(els.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn isInstantiableClassRestriction(mut inRestriction: SCode::Restriction) -> bool {
    let mut outIsInstantiable: bool = false;
    outIsInstantiable = (match inRestriction.clone() {
        SCode::Restriction::R_CLASS => true,
        SCode::Restriction::R_MODEL => true,
        SCode::Restriction::R_RECORD { .. } => true,
        SCode::Restriction::R_BLOCK => true,
        SCode::Restriction::R_CONNECTOR { .. } => true,
        SCode::Restriction::R_TYPE => true,
        SCode::Restriction::R_ENUMERATION => true,
        _ => false,
    });
    outIsInstantiable
}

pub fn isInitial(mut inInitial: SCode::Initial) -> bool {
    let mut isIn: bool = false;
    isIn = (match inInitial.clone() {
        SCode::Initial::INITIAL => true,
        _ => false,
    });
    isIn
}

pub fn checkSameRestriction(mut inResNew: SCode::Restriction, mut inResOrig: SCode::Restriction, mut inInfoNew: SourceInfo, mut inInfoOrig: SourceInfo) -> (SCode::Restriction, SourceInfo) {
    let mut outRes: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut outInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (outRes, outInfo) = (match (inResNew.clone(), inResOrig.clone(), inInfoNew.clone(), inInfoOrig.clone()) {
        (_, _, _, _) => (inResNew.clone(), inInfoNew.clone()),
    });
    (outRes, outInfo)
}

pub fn setComponentName(mut element: Arc<SCode::Element>, mut name: ArcStr) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; name = name.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn isArrayComponent(mut inElement: Arc<SCode::Element>) -> bool {
    let mut outIsArray: bool = false;
    outIsArray = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsArray
}

pub fn isEmptyMod(mut r#mod: Arc<SCode::Mod>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn getConstrainingMod(mut element: Arc<SCode::Element>) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    r#mod = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: r#mod, .. }) }, .. }, .. } => r#mod.clone(),
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, .. }, .. } => r#mod.clone(),
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: r#mod, .. }) }, .. }, .. } => r#mod.clone(),
        Deref @ SCode::Element::COMPONENT { modifications: r#mod, .. } => r#mod.clone(),
        _ => Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isEmptyClassDef(mut cdef: Arc<SCode::ClassDef>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && isNone(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone()),
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => isEmptyClassDef(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone()),
        Deref @ SCode::ClassDef::ENUMERATION { .. } => var_field!((*cdef).enumLst, SCode::ClassDef::ENUMERATION).clone().is_empty(),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn stripCommentsFromProgram(mut program: Arc<metamodelica::List<Arc<SCode::Element>>>, mut stripAnnotations: bool, mut stripComments: bool) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = program;
    program = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (program.clone()).into_iter().cloned() {
            let __x = stripCommentsFromElement(e.clone(), stripAnnotations.clone(), stripComments.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(program)
}

pub fn stripCommentsFromElement(mut element: Arc<SCode::Element>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => {
            if stripAnn.clone() {
                assign_variant_field!(element => SCode::Element::EXTENDS; ann = None);
            }
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = stripCommentsFromMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), stripAnn.clone(), stripCmt.clone())?);
            ()
        },
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS;
                classDef = stripCommentsFromClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), stripAnn.clone(), stripCmt.clone())?,
                cmt = stripCommentsFromComment(var_field!((*element).cmt, SCode::Element::CLASS).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT;
                modifications = stripCommentsFromMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), stripAnn.clone(), stripCmt.clone())?,
                comment = stripCommentsFromComment(var_field!((*element).comment, SCode::Element::COMPONENT).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub fn stripCommentsFromMod(mut r#mod: Arc<SCode::Mod>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            let __x = stripCommentsFromSubMod(m.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::Mod::REDECL { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::REDECL; element = stripCommentsFromElement(var_field!((*r#mod).element, SCode::Mod::REDECL).clone(), stripAnn.clone(), stripCmt.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn stripCommentsFromSubMod(mut submod: Arc<SCode::SubMod>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Arc<SCode::SubMod>> {
    let mut submod: Arc<SCode::SubMod> = submod;
    assign_field!(submod.r#mod = stripCommentsFromMod(submod.r#mod.clone(), stripAnn.clone(), stripCmt.clone())?);
    Ok(submod)
}

pub fn stripCommentsFromClassDef(mut cdef: Arc<SCode::ClassDef>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Arc<SCode::ClassDef>> {
    let mut cdef: Arc<SCode::ClassDef> = cdef;
    cdef = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
            let mut ieql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
            let mut alg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
            let mut ialg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
            let mut ext: Option<Arc<SCode::ExternalDecl>> = None;
            el = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromElement(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            eql = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut eq in (var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(eq.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ieql = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut ieq in (var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(ieq.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            alg = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromAlgorithm(a.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ialg = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for mut ia in (var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromAlgorithm(ia.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ext = stripCommentsFromExternalDecl(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone(), stripAnn.clone(), stripCmt.clone())?;
            Arc::new(SCode::ClassDef::PARTS { elementLst: el.clone(), normalEquationLst: eql.clone(), initialEquationLst: ieql.clone(), normalAlgorithmLst: alg.clone(), initialAlgorithmLst: ialg.clone(), constraintLst: var_field!((*cdef).constraintLst, SCode::ClassDef::PARTS).clone(), clsattrs: var_field!((*cdef).clsattrs, SCode::ClassDef::PARTS).clone(), externalDecl: ext.clone() })
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::CLASS_EXTENDS;
                modifications = stripCommentsFromMod(var_field!((*cdef).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), stripAnn.clone(), stripCmt.clone())?,
                composition = stripCommentsFromClassDef(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), stripAnn.clone(), stripCmt.clone())?
            );
            cdef.clone()
        },
        Deref @ SCode::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::DERIVED; modifications = stripCommentsFromMod(var_field!((*cdef).modifications, SCode::ClassDef::DERIVED).clone(), stripAnn.clone(), stripCmt.clone())?);
            cdef.clone()
        },
        Deref @ SCode::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::ENUMERATION; enumLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Enum>>> = metamodelica::nil();
        for mut e in (var_field!((*cdef).enumLst, SCode::ClassDef::ENUMERATION).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEnum(e.clone(), stripAnn.clone(), stripCmt.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            cdef.clone()
        },
        _ => {
            cdef.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cdef)
}

pub fn stripCommentsFromEnum(mut r#enum: Arc<SCode::Enum>, mut stripAnn: bool, mut stripCmt: bool) -> Arc<SCode::Enum> {
    let mut r#enum: Arc<SCode::Enum> = r#enum;
    assign_field!(r#enum.comment = stripCommentsFromComment(r#enum.comment.clone(), stripAnn.clone(), stripCmt.clone()));
    r#enum
}

pub fn stripCommentsFromComment(mut cmt: Arc<SCode::Comment>, mut stripAnn: bool, mut stripCmt: bool) -> Arc<SCode::Comment> {
    let mut cmt: Arc<SCode::Comment> = cmt;
    if stripAnn.clone() {
        assign_field!(cmt.annotation_ = None);
    }
    if stripCmt.clone() {
        assign_field!(cmt.comment = None);
    }
    cmt
}

pub fn stripCommentsFromExternalDecl(mut extDecl: Option<Arc<SCode::ExternalDecl>>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Option<Arc<SCode::ExternalDecl>>> {
    let mut extDecl: Option<Arc<SCode::ExternalDecl>> = extDecl;
    let mut ext_decl: Arc<SCode::ExternalDecl>;
    if isSome(extDecl.clone()) && stripAnn.clone() {
        let __pa0 = ::match_deref::match_deref! { match &(extDecl.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ext_decl = __pa0.clone();
        assign_field!(ext_decl.annotation_ = None);
        extDecl = Some(ext_decl.clone());
    }
    Ok(extDecl)
}

pub fn stripCommentsFromEquation(mut eq: Arc<SCode::Equation>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Arc<SCode::Equation>> {
    let mut eq: Arc<SCode::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                thenBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
        for mut branch in (var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (branch.clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_IF).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_EQUALS { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_EQUALS).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_PDE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_CONNECT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR;
                eEquationLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_FOR).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                eEquationLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).eEquationLst, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = stripCommentsFromWhenEqBranch(b.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_WHEN).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_ASSERT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_ASSERT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_TERMINATE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_REINIT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_NORETCALL).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(eq)
}

pub fn stripCommentsFromWhenEqBranch(mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), mut stripAnn: bool, mut stripCmt: bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)> {
    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>) = branch;
    let mut cond: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    (cond, body) = branch.clone();
    body = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (body.clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    branch = (cond.clone(), body.clone());
    Ok(branch)
}

pub fn stripCommentsFromAlgorithm(mut alg: Arc<SCode::AlgorithmSection>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Arc<SCode::AlgorithmSection>> {
    let mut alg: Arc<SCode::AlgorithmSection> = alg;
    assign_field!(alg.statements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (alg.statements.clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(alg)
}

pub fn stripCommentsFromStatement(mut stmt: Arc<SCode::Statement>, mut stripAnn: bool, mut stripCmt: bool) -> Result<Arc<SCode::Statement>> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSIGN).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_IF { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                trueBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).trueBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatementBranch(b.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_IF).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR;
                forBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).forBody, SCode::Statement::ALG_FOR).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_FOR).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR;
                parforBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).parforBody, SCode::Statement::ALG_PARFOR).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_PARFOR).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE;
                whileBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).whileBody, SCode::Statement::ALG_WHILE).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHILE).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A;
                branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatementBranch(b.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHEN_A).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSERT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_TERMINATE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_REINIT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_NORETCALL { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_NORETCALL).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_RETURN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_RETURN; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_RETURN).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_BREAK { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_BREAK; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_BREAK).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_FAILURE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FAILURE; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_FAILURE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_TRY { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TRY;
                body = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).body, SCode::Statement::ALG_TRY).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (var_field!((*stmt).elseBody, SCode::Statement::ALG_TRY).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_TRY).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_CONTINUE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_CONTINUE; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_CONTINUE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(stmt)
}

pub fn stripCommentsFromStatementBranch(mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), mut stripAnn: bool, mut stripCmt: bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)> {
    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>) = branch;
    let mut cond: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
    (cond, body) = branch.clone();
    body = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (body.clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    branch = (cond.clone(), body.clone());
    Ok(branch)
}

pub fn checkValidEnumLiteral(mut inLiteral: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    if listMember((inLiteral.clone()).clone(), list![(literal!("quantity")).clone(), (literal!("min")).clone(), (literal!("max")).clone(), (literal!("start")).clone(), (literal!("fixed")).clone()]) {
        Error::addSourceMessage(Error::INVALID_ENUM_LITERAL.clone(), list![(inLiteral.clone()).clone()], inInfo.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn isRedeclareElement(mut element: Arc<SCode::Element>) -> bool {
    let mut isElement: bool = false;
    isElement = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE, .. }, .. } => true,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isElement
}

pub fn mergeSCodeOptAnn(mut inModOuter: Option<Arc<SCode::Annotation>>, mut inModInner: Option<Arc<SCode::Annotation>>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut outMod: Option<Arc<SCode::Annotation>> = None;
    outMod = (::match_deref::match_deref! { match &((inModOuter.clone(), inModInner.clone())) {
        (None, _) => {
            inModInner.clone()
        },
        (_, None) => {
            inModOuter.clone()
        },
        (Some(Deref @ SCode::Annotation { modification: mod1 }), Some(Deref @ SCode::Annotation { modification: mod2 })) => {
            let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            r#mod = mergeSCodeMods(mod1.clone(), mod2.clone())?;
            Some(Arc::new(SCode::Annotation { modification: r#mod.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

pub fn mergeSCodeMods(mut inModOuter: Arc<SCode::Mod>, mut inModInner: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &((inModOuter.clone(), inModInner.clone())) {
        (Deref @ SCode::Mod::NOMOD, _) => {
            inModInner.clone()
        },
        (_, Deref @ SCode::Mod::NOMOD) => {
            inModOuter.clone()
        },
        (Deref @ SCode::Mod::MOD { .. }, Deref @ SCode::Mod::MOD { .. }) => {
            let mut subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            let mut binding: Option<Arc<Absyn::Exp>> = None;
            subMods = listAppend(var_field!((*inModOuter).subModLst, SCode::Mod::MOD).clone(), var_field!((*inModInner).subModLst, SCode::Mod::MOD).clone());
            binding = if (isSome(var_field!((*inModOuter).binding, SCode::Mod::MOD).clone())) {var_field!((*inModOuter).binding, SCode::Mod::MOD).clone()} else {var_field!((*inModInner).binding, SCode::Mod::MOD).clone()};
            Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*inModOuter).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*inModOuter).eachPrefix, SCode::Mod::MOD).clone(), subModLst: subMods.clone(), binding: binding.clone(), comment: var_field!((*inModOuter).comment, SCode::Mod::MOD).clone(), info: var_field!((*inModOuter).info, SCode::Mod::MOD).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn hasNamedExternalCall(mut name: ArcStr, mut def: Arc<SCode::ClassDef>) -> bool {
    let mut hasCall: bool = false;
    hasCall = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: Some(fn_name), .. }), .. } => {
            fn_name.clone() == name.clone()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            hasNamedExternalCall((name.clone()).clone(), var_field!((*def).composition, SCode::ClassDef::CLASS_EXTENDS).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasCall
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn classDefHasSections(mut cdef: Arc<SCode::ClassDef>, mut checkExternal: bool) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => !(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && if (checkExternal.clone()) {isNone(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone())} else {true}),
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => classDefHasSections(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), checkExternal.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn mapElements(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>;

    let mut elements: Arc<metamodelica::List<Arc<SCode::Element>>> = elements;
    elements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (elements.clone()).into_iter().cloned() {
            let __x = mapElement(e.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    elements
}

pub fn mapElement(mut element: Arc<SCode::Element>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>) -> Arc<SCode::Element> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>;

    let mut element: Arc<SCode::Element> = element;
    let mut def: Arc<SCode::ClassDef>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            def = mapElementsClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), func.clone());
            if !(referenceEq(&def.clone(),&var_field!((*element).classDef, SCode::Element::CLASS).clone())) {
                assign_variant_field!(element => SCode::Element::CLASS; classDef = def.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    element = func(element.clone()).unwrap();
    element
}

pub fn mapElementsClassDef(mut classDef: Arc<SCode::ClassDef>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>) -> Arc<SCode::ClassDef> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>;

    let mut classDef: Arc<SCode::ClassDef> = classDef;
    let mut def: Arc<SCode::ClassDef>;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (var_field!((*classDef).elementLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = mapElement(e.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            def = mapElementsClassDef(var_field!((*classDef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), func.clone());
            if !(referenceEq(&def.clone(),&var_field!((*classDef).composition, SCode::ClassDef::CLASS_EXTENDS).clone())) {
                assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; composition = def.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    classDef
}

pub fn mapEquationsList(mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>> + 'static>) -> Arc<metamodelica::List<Arc<SCode::Equation>>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>> + 'static>;

    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = eql;
    eql = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (eql.clone()).into_iter().cloned() {
            let __x = mapEquations(e.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    eql
}

pub fn mapEquations(mut eq: Arc<SCode::Equation>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>> + 'static>) -> Arc<SCode::Equation> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>> + 'static>;

    let mut eq: Arc<SCode::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                thenBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = mapEquationsList(b.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = mapEquationsList(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = mapEquationsList(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                eEquationLst = mapEquationsList(var_field!((*eq).eEquationLst, SCode::Equation::EQ_WHEN).clone(), func.clone()),
                elseBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), mapEquationsList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eq = func(eq.clone()).unwrap();
    eq
}

pub fn mapEquationExps(mut eq: Arc<SCode::Equation>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<Arc<SCode::Equation>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>;

    let mut eq: Arc<SCode::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF; condition = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).condition, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::Equation::EQ_EQUALS { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS;
                expLeft = func(var_field!((*eq).expLeft, SCode::Equation::EQ_EQUALS).clone())?,
                expRight = func(var_field!((*eq).expRight, SCode::Equation::EQ_EQUALS).clone())?
            );
            ()
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE;
                expLeft = func(var_field!((*eq).expLeft, SCode::Equation::EQ_PDE).clone())?,
                expRight = func(var_field!((*eq).expRight, SCode::Equation::EQ_PDE).clone())?,
                domain = AbsynUtil::mapCrefExps(var_field!((*eq).domain, SCode::Equation::EQ_PDE).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = AbsynUtil::mapCrefExps(var_field!((*eq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), func.clone()),
                crefRight = AbsynUtil::mapCrefExps(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            if isSome(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone()) {
                assign_variant_field!(eq => SCode::Equation::EQ_FOR; range = Some(func(Util::getOption(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone())?)?));
            }
            ()
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                condition = func(var_field!((*eq).condition, SCode::Equation::EQ_WHEN).clone())?,
                elseBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = Util::applyTuple21(b.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ SCode::Equation::EQ_ASSERT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT;
                condition = func(var_field!((*eq).condition, SCode::Equation::EQ_ASSERT).clone())?,
                message = func(var_field!((*eq).message, SCode::Equation::EQ_ASSERT).clone())?,
                level = func(var_field!((*eq).level, SCode::Equation::EQ_ASSERT).clone())?
            );
            ()
        },
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE; message = func(var_field!((*eq).message, SCode::Equation::EQ_TERMINATE).clone())?);
            ()
        },
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT;
                cref = func(var_field!((*eq).cref, SCode::Equation::EQ_REINIT).clone())?,
                expReinit = func(var_field!((*eq).expReinit, SCode::Equation::EQ_REINIT).clone())?
            );
            ()
        },
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL; exp = func(var_field!((*eq).exp, SCode::Equation::EQ_NORETCALL).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(eq)
}

pub fn mapAlgorithmStatements(mut alg: Arc<SCode::AlgorithmSection>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>) -> Arc<SCode::AlgorithmSection> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>;

    let mut alg: Arc<SCode::AlgorithmSection> = alg;
    assign_field!(alg.statements = mapStatementsList(alg.statements.clone(), func.clone()));
    alg
}

pub fn mapStatementsList(mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>) -> Arc<metamodelica::List<Arc<SCode::Statement>>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>;

    let mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>> = statements;
    statements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (statements.clone()).into_iter().cloned() {
            let __x = mapStatements(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    statements
}

pub fn mapStatements(mut stmt: Arc<SCode::Statement>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>) -> Arc<SCode::Statement> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>;

    let mut stmt: Arc<SCode::Statement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_IF { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                trueBranch = mapStatementsList(var_field!((*stmt).trueBranch, SCode::Statement::ALG_IF).clone(), func.clone()),
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), mapStatementsList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = mapStatementsList(var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR; forBody = mapStatementsList(var_field!((*stmt).forBody, SCode::Statement::ALG_FOR).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR; parforBody = mapStatementsList(var_field!((*stmt).parforBody, SCode::Statement::ALG_PARFOR).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE; whileBody = mapStatementsList(var_field!((*stmt).whileBody, SCode::Statement::ALG_WHILE).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), mapStatementsList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::Statement::ALG_FAILURE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FAILURE; stmts = mapStatementsList(var_field!((*stmt).stmts, SCode::Statement::ALG_FAILURE).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_TRY { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TRY;
                body = mapStatementsList(var_field!((*stmt).body, SCode::Statement::ALG_TRY).clone(), func.clone()),
                elseBody = mapStatementsList(var_field!((*stmt).body, SCode::Statement::ALG_TRY).clone(), func.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stmt = func(stmt.clone()).unwrap();
    stmt
}

pub fn mapStatementExps(mut stmt: Arc<SCode::Statement>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<Arc<SCode::Statement>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>;

    let mut stmt: Arc<SCode::Statement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN;
                assignComponent = func(var_field!((*stmt).assignComponent, SCode::Statement::ALG_ASSIGN).clone())?,
                value = func(var_field!((*stmt).value, SCode::Statement::ALG_ASSIGN).clone())?
            );
            ()
        },
        Deref @ SCode::Statement::ALG_IF { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                boolExpr = func(var_field!((*stmt).boolExpr, SCode::Statement::ALG_IF).clone())?,
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            if isSome(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone()) {
                assign_variant_field!(stmt => SCode::Statement::ALG_FOR; range = Some(func(Util::getOption(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone())?)?));
            }
            ()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            if isSome(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone()) {
                assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR; range = Some(func(Util::getOption(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone())?)?));
            }
            ()
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE; boolExpr = func(var_field!((*stmt).boolExpr, SCode::Statement::ALG_WHILE).clone())?);
            ()
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT;
                condition = func(var_field!((*stmt).condition, SCode::Statement::ALG_ASSERT).clone())?,
                message = func(var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone())?,
                level = func(var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone())?
            );
            ()
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE; message = func(var_field!((*stmt).message, SCode::Statement::ALG_TERMINATE).clone())?);
            ()
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT;
                cref = func(var_field!((*stmt).cref, SCode::Statement::ALG_REINIT).clone())?,
                newValue = func(var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone())?
            );
            ()
        },
        Deref @ SCode::Statement::ALG_NORETCALL { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL; exp = func(var_field!((*stmt).exp, SCode::Statement::ALG_NORETCALL).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub fn lookupModInMod(mut name: ArcStr, mut r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut m = m.clone();
                if m.ident.clone() == name.clone() {
                    outMod = m.r#mod.clone();
                    return outMod;
                }
            }
            Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)
        },
        _ => Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn isNonEmptyAlgorithm(mut alg: Arc<SCode::AlgorithmSection>) -> bool {
    let mut res: bool = !(alg.statements.clone().is_empty());
    res
}

pub fn onlyLiteralsInMod(mut r#mod: Arc<SCode::Mod>) -> Result<bool> {
    let mut onlyLiterals: bool = false;
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    onlyLiterals = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                onlyLiterals = AbsynUtil::onlyLiteralsInExp(Util::getOption(var_field!((*r#mod).binding, SCode::Mod::MOD).clone())?)?;
            } else {
                onlyLiterals = true;
            }
            if onlyLiterals.clone() {
                for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                    let mut m = m.clone();
                    onlyLiterals = onlyLiteralsInMod(m.r#mod.clone())?;
                    if !(onlyLiterals.clone()) {
                        break;
                    }
                }
            }
            onlyLiterals.clone()
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(onlyLiterals)
}

pub fn transformPathedElementInProgram(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>, mut program: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, bool)> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>;

    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = program;
    let mut success: bool = false;
    (program, success) = List::findMap(program.clone(), Arc::new({ let __pe_b0 = path.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static> = func.clone(); move |__pe_a2| transformPathedElementInElement(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?;
    Ok((program, success))
}

pub fn transformPathedElementInElement(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>, mut element: Arc<SCode::Element>) -> Result<(Arc<SCode::Element>, bool)> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>;

    let mut element: Arc<SCode::Element> = element;
    let mut success: bool = false;
    let mut cdef: Arc<SCode::ClassDef>;
    success = isElementNamed((AbsynUtil::pathFirstIdent(path.clone())?).clone(), element.clone());
    if success.clone() {
        if AbsynUtil::pathIsIdent(path.clone()) {
            element = func(element.clone())?;
        } else if isClass(element.clone()) {
            (cdef, success) = transformPathedElementInClassDef(AbsynUtil::pathRest(path.clone())?, func.clone(), getClassDef(element.clone())?)?;
            if success.clone() {
                element = setClassDef(cdef.clone(), element.clone())?;
            }
        }
    }
    Ok((element, success))
}

pub fn transformPathedElementInClassDef(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>, mut cls: Arc<SCode::ClassDef>) -> Result<(Arc<SCode::ClassDef>, bool)> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>;

    let mut cls: Arc<SCode::ClassDef> = cls;
    let mut success: bool = false;
    let mut elems: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut cdef: Arc<SCode::ClassDef>;
    success = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            (elems, success) = transformPathedElementInProgram(path.clone(), func.clone(), var_field!((*cls).elementLst, SCode::ClassDef::PARTS).clone())?;
            if success.clone() {
                assign_variant_field!(cls => SCode::ClassDef::PARTS; elementLst = elems.clone());
            }
            success.clone()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            (cdef, success) = transformPathedElementInClassDef(path.clone(), func.clone(), var_field!((*cls).composition, SCode::ClassDef::CLASS_EXTENDS).clone())?;
            if success.clone() {
                assign_variant_field!(cls => SCode::ClassDef::CLASS_EXTENDS; composition = cdef.clone());
            }
            success.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cls, success))
}

pub fn makeMod(mut isFinal: bool, mut isEach: bool, mut subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut binding: Option<Arc<Absyn::Exp>>, mut comment: Option<ArcStr>, mut info: SourceInfo) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    r#mod = Arc::new(SCode::Mod::MOD { finalPrefix: if (isFinal.clone()) {openmodelica_frontend_types::SCode::Final::FINAL} else {openmodelica_frontend_types::SCode::Final::NOT_FINAL}, eachPrefix: if (isEach.clone()) {openmodelica_frontend_types::SCode::Each::EACH} else {openmodelica_frontend_types::SCode::Each::NOT_EACH}, subModLst: subMods.clone(), binding: binding.clone(), comment: comment.clone(), info: info.clone() });
    r#mod
}

pub fn makeSingleAnnotation(mut name: ArcStr, mut value: Arc<Absyn::Exp>) -> Arc<SCode::Annotation> {
    let mut ann: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    ann = Arc::new(SCode::Annotation { modification: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(value.clone()), comment: None, info: Absyn::dummyInfo.clone() }) })], binding: None, comment: None, info: Absyn::dummyInfo.clone() }) });
    ann
}

pub fn setAnnotationInComment(mut name: ArcStr, mut value: Arc<Absyn::Exp>, mut cmt: Arc<SCode::Comment>, mut replace: bool) -> Result<Arc<SCode::Comment>> {
    let mut cmt: Arc<SCode::Comment> = cmt;
    let mut ann: Arc<SCode::Annotation> = Arc::new(<SCode::Annotation as ::std::default::Default>::default());
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    if isNone(cmt.annotation_.clone()) {
        assign_field!(cmt.annotation_ = Some(makeSingleAnnotation((name.clone()).clone(), value.clone())));
        return Ok(cmt);
    } else {
        assign_field!(cmt.annotation_ = Some(setAnnotationValue((name.clone()).clone(), value.clone(), Util::getOption(cmt.annotation_.clone())?, replace.clone())?));
    }
    Ok(cmt)
}

pub fn setAnnotationValue(mut name: ArcStr, mut value: Arc<Absyn::Exp>, mut ann: Arc<SCode::Annotation>, mut replace: bool) -> Result<Arc<SCode::Annotation>> {
    fn replace_mod(mut name: ArcStr, mut value: Arc<Absyn::Exp>, mut replace: bool, mut r#mod: Arc<SCode::SubMod>) -> (Arc<SCode::SubMod>, bool) {
        let mut r#mod: Arc<SCode::SubMod> = r#mod;
        let mut found: bool = false;
        found = r#mod.ident.clone() == name.clone();
        if found.clone() && replace.clone() {
            assign_field!(r#mod.r#mod = setModifierBinding(Some(value.clone()), r#mod.r#mod.clone()));
        }
        (r#mod, found)
    }

    let mut ann: Arc<SCode::Annotation> = ann;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut found: bool = false;
    let () = (::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ SCode::Annotation { modification: r#mod @ Deref @ SCode::Mod::MOD { .. } } => {
            let mut r#mod = (*r#mod).clone();
            (submods, found) = List::findMap(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); let __pe_b1 = value.clone(); let __pe_b2 = replace.clone(); move |__pe_a3| Ok(replace_mod(__pe_b0.clone(), __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }))?;
            if !(found.clone()) {
                submods = cons(Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: makeMod(false, false, metamodelica::nil(), Some(value.clone()), None, Absyn::dummyInfo.clone()) }), submods.clone());
            }
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = submods.clone());
            assign_field!(ann.modification = r#mod.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ann)
}

