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
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionDump;
use crate::FHashTableCrToUnit as HashTableCrToUnit;
use crate::FHashTableStringToUnit as HashTableStringToUnit;
use crate::FHashTableUnitToString as HashTableUnitToString;
use crate::FUnit as Unit;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Functionargs {
    pub name: ArcStr,
    pub invars: Arc<metamodelica::List<ArcStr>>,
    pub outvars: Arc<metamodelica::List<ArcStr>>,
    pub inunits: Arc<metamodelica::List<ArcStr>>,
    pub outunits: Arc<metamodelica::List<ArcStr>>,
}

pub type FUNCTIONUNITS = Functionargs;


fn parseFunctionList(mut infunction: DAE::Function) -> Result<Arc<metamodelica::List<Functionargs>>> {
    let mut outTpl: Arc<metamodelica::List<Functionargs>> = metamodelica::nil();
    let mut inelt: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut outelt: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut inunits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outunits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut inargs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outargs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut unitString: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    s = (getFunctionName(infunction.clone())).clone();
    inelt = DAEUtil::getFunctionInputVars(infunction.clone())?;
    outelt = DAEUtil::getFunctionOutputVars(infunction.clone())?;
    inunits = List::filterMap(inelt.clone(), Arc::new(fnptr!(getUnits, Arc<DAE::Element>)));
    outunits = List::filterMap(outelt.clone(), Arc::new(fnptr!(getUnits, Arc<DAE::Element>)));
    inargs = List::filterMap(inelt.clone(), Arc::new(getVars));
    outargs = List::filterMap(outelt.clone(), Arc::new(getVars));
    outTpl = list![Functionargs { name: (s.clone()).clone(), invars: inargs.clone(), outvars: outargs.clone(), inunits: inunits.clone(), outunits: outunits.clone() }];
    Ok(outTpl)
}

pub fn getFunctionName(mut inFunction: DAE::Function) -> ArcStr {
    let mut outString: ArcStr = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(DAEUtil::functionName(inFunction.clone()).unwrap()), (literal!(".")).clone(), true, false).unwrap();
    outString
}

pub fn getVars(mut inElement: Arc<DAE::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cr, .. } => {
            ComponentReference::crefStr(cr.clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn getUnits(mut inElement: Arc<DAE::Element>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(Deref @ DAE::Exp::SCONST { string: unitString }), .. }), ty: Deref @ DAE::Type::T_REAL { .. }, .. } if (unitString.clone() != literal!("")) => {
            unitString.clone()
        },
        _ => {
            literal!("NONE")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn updateDAElist(mut indaelist: DAE::DAElist, mut indaevarlist: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<DAE::DAElist> {
    let mut outdaelist: DAE::DAElist;
    outdaelist = (::match_deref::match_deref! { match &((indaelist.clone(), indaevarlist.clone())) {
        (DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { comment, source: eltsrc, ident, .. }, tail: Deref @ metamodelica::List::Nil } }, varlist2) => {
            let mut outdae: DAE::DAElist;
            outdae = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMP { ident: (ident.clone()).clone(), dAElist: varlist2.clone(), source: eltsrc.clone(), comment: comment.clone() })] };
            outdae.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outdaelist)
}

fn returnVar(mut inVar: Arc<DAE::Element>, mut inHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<Arc<DAE::Element>> {
    let mut outVar: Arc<DAE::Element>;
    outVar = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(_), .. }), .. } => {
            inVar.clone()
        },
        Deref @ DAE::Element::VAR { variableAttributesOption: attr, componentRef: cr, .. } => {
            let mut var: Arc<DAE::Element>;
            let mut ut: Unit::Unit;
            let mut s: ArcStr = arcstr::literal!("");
            let mut attr = (*attr).clone();
            if BaseHashTable::hasKey(cr.clone(), inHtCr2U.clone()) {
                ut = BaseHashTable::get(cr.clone(), inHtCr2U.clone())?;
                if Unit::isUnit(ut.clone()) {
                    s = (Unit::unitString(ut.clone(), inHtU2S.clone())?).clone();
                    attr = DAEUtil::setUnitAttr(attr.clone(), Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() }))?;
                    assign_variant_field!(inVar => DAE::Element::VAR; variableAttributesOption = attr.clone());
                    var = inVar.clone();
                } else {
                    var = inVar.clone();
                }
            } else {
                var = inVar.clone();
            }
            var.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVar)
}

fn notification(mut inHtCr2U1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut inHtCr2U2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut lt1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Unit::Unit)>> = metamodelica::nil();
    lt1 = BaseHashTable::hashTableList(inHtCr2U1.clone());
    r#str = (notification2(lt1.clone(), inHtCr2U2.clone(), inHtU2S.clone())?).clone();
    if Flags::isSet(Flags::DUMP_UNIT.clone())? && r#str.clone() != literal!("") {
        Error::addCompilerNotification((r#str.clone()).clone())?;
    }
    Ok(())
}

fn notification2(mut inLt1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Unit::Unit)>>, mut inHtCr2U2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    let mut cr1: Arc<DAE::ComponentRef> = DAE::emptyCref.clone();
    let mut factor1: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;
    let mut i6: i32 = 0;
    let mut i7: i32 = 0;
    outS = stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t1 in (inLt1.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(t1.clone()) {
        (cr1, Unit::Unit::MASTER { .. }) => {
            let mut b: bool = false;
            b = false;
            if '__try0: {
                let Unit::UNIT { factor: __pa1, mol: __pa2, cd: __pa3, m: __pa4, s: __pa5, A: __pa6, K: __pa7, g: __pa8 } = (unwrap_break_err!(BaseHashTable::get(cr1.clone(), inHtCr2U2.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                factor1 = __pa1.clone();
                i1 = __pa2.clone();
                i2 = __pa3.clone();
                i3 = __pa4.clone();
                i4 = __pa5.clone();
                i5 = __pa6.clone();
                i6 = __pa7.clone();
                i7 = __pa8.clone();
                b = true;
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentReference::crefStr(cr1.clone())?); __mm_s.push_str(&*literal!("\" has the Unit \"")); __mm_s.push_str(&*Unit::unitString(Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }, inHtU2S.clone())?); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outS)
}

pub fn algo(mut invarlist: Arc<metamodelica::List<Arc<DAE::Element>>>, mut ineqList: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inargs: Arc<metamodelica::List<Functionargs>>, mut inHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut inHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) {
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)));
    let mut HtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr));
    let mut HtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr));
    let mut HtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr));
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    (HtCr2U, b1, HtS2U, HtU2S) = List::fold(invarlist.clone(), Arc::new(foldBindingExp), (inHtCr2U.clone(), true, inHtS2U.clone(), inHtU2S.clone()));
    (HtCr2U, b2, HtS2U, HtU2S) = List::fold1(ineqList.clone(), Arc::new(foldEquation), inargs.clone(), (HtCr2U.clone(), true, HtS2U.clone(), HtU2S.clone()));
    b3 = BaseHashTable::hasKey(Unit::UPDATECREF.clone(), HtCr2U.clone());
    outTpl = (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone());
    outTpl
}

fn foldBindingExp(mut inVar: Arc<DAE::Element>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), bool, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), bool, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))> {
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), bool, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)));
    outTpl = (::match_deref::match_deref! { match &((inVar.clone(), inTpl.clone())) {
        (Deref @ DAE::Element::VAR { source, binding: Some(exp), ty: Deref @ DAE::Type::T_REAL { .. }, componentRef: cref, .. }, (HtCr2U, b, HtS2U, HtU2S)) => {
            let mut crefExp: Arc<DAE::Exp>;
            let mut eq: Arc<DAE::Element>;
            let mut HtCr2U = (*HtCr2U).clone();
            let mut b = (*b).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            crefExp = Expression::crefExp(cref.clone())?;
            eq = Arc::new(DAE::Element::EQUATION { exp: crefExp.clone(), scalar: exp.clone(), source: source.clone() });
            (HtCr2U, b, HtS2U, HtU2S) = foldEquation(eq.clone(), metamodelica::nil(), (HtCr2U.clone(), b.clone(), HtS2U.clone(), HtU2S.clone()))?;
            (HtCr2U.clone(), b.clone(), HtS2U.clone(), HtU2S.clone())
        },
        (Deref @ DAE::Element::VAR { binding: Some(_), ty: Deref @ DAE::Type::T_REAL { .. }, .. }, (HtCr2U, _, HtS2U, HtU2S)) => {
            (HtCr2U.clone(), false, HtS2U.clone(), HtU2S.clone())
        },
        _ => {
            inTpl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn foldEquation(mut inEq: Arc<DAE::Element>, mut inargs: Arc<metamodelica::List<Functionargs>>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), bool, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), bool, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))> {
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), bool, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)));
    let mut HtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr));
    let mut HtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr));
    let mut HtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr));
    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
    let mut b: bool = false;
    (HtCr2U, b, HtS2U, HtU2S) = inTpl.clone();
    (HtCr2U, HtS2U, HtU2S, expListList) = foldEquation2(inEq.clone(), HtCr2U.clone(), HtS2U.clone(), HtU2S.clone(), inargs.clone())?;
    List::map2_0(expListList.clone(), Arc::new(Errorfunction), inEq.clone(), HtU2S.clone());
    outTpl = (HtCr2U.clone(), b.clone(), HtS2U.clone(), HtU2S.clone());
    Ok(outTpl)
}

fn foldEquation2(mut eq: Arc<DAE::Element>, mut htCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut htS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), mut htU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)), mut args: Arc<metamodelica::List<Functionargs>>) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)), Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>>)> {
    let mut htCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)) = htCr2U;
    let mut htS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)) = htS2U;
    let mut htU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)) = htU2S;
    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
    inconsistentUnits = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ DAE::Element::DEFINE { .. } => {
            let mut temp: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::Exp>;
            lhs = Arc::new(DAE::Exp::CREF { componentRef: var_field!((*eq).componentRef, DAE::Element::DEFINE).clone(), ty: DAE::T_REAL_DEFAULT.clone() });
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).exp, DAE::Element::DEFINE).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: lhs.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::INITIALDEFINE { .. } => {
            let mut temp: Arc<DAE::Exp>;
            let mut lhs: Arc<DAE::Exp>;
            lhs = Arc::new(DAE::Exp::CREF { componentRef: var_field!((*eq).componentRef, DAE::Element::INITIALDEFINE).clone(), ty: DAE::T_REAL_DEFAULT.clone() });
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).exp, DAE::Element::INITIALDEFINE).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: lhs.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path }, .. }, exp: Deref @ DAE::Exp::TUPLE { PR: expl }, .. } => {
            let mut expList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
            let mut expList3: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
            let mut s1: ArcStr = arcstr::literal!("");
            let mut outvars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut outunitlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            s1 = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            s1 = (System::trim((s1.clone()).clone(), (literal!(".")).clone())).clone();
            (_, outvars, _, outunitlist) = getNamedUnitlist((s1.clone()).clone(), args.clone());
            (htCr2U, htS2U, htU2S, expList2) = foldCallArg1(expl.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, outunitlist.clone(), outvars.clone(), (s1.clone()).clone())?;
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(var_field!((*eq).scalar, DAE::Element::EQUATION).clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            expList3 = __pa3.clone();
            List::append_reverse(expList2.clone(), expList3.clone())
        },
        Deref @ DAE::Element::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path }, .. }, exp: lhs, .. } => {
            let mut temp: Arc<DAE::Exp>;
            let mut expList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
            let mut expList3: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut ut1: Unit::Unit;
            let mut ut2: Unit::Unit;
            let mut s1: ArcStr = arcstr::literal!("");
            let mut formalargs: ArcStr = arcstr::literal!("");
            let mut formalvar: ArcStr = arcstr::literal!("");
            let mut outvars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut outunitlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            s1 = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            s1 = (System::trim((s1.clone()).clone(), (literal!(".")).clone())).clone();
            (_, outvars, _, outunitlist) = getNamedUnitlist((s1.clone()).clone(), args.clone());
            let (__pa0, (__pa1, __pa2, __pa3), _) = insertUnitInEquation(lhs.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            ut1 = __pa0.clone();
            htCr2U = __pa1.clone();
            htS2U = __pa2.clone();
            htU2S = __pa3.clone();
            formalargs = (listHead(outunitlist.clone())?).clone();
            formalvar = (listHead(outvars.clone())?).clone();
            ut2 = if (formalargs.clone() == literal!("NONE")) {Unit::Unit::MASTER { varList: metamodelica::nil() }} else {Unit::parseUnitString((formalargs.clone()).clone(), Unit::getKnownUnits()?)?};
            (b, _, _) = UnitTypesEqual(ut1.clone(), ut2.clone(), htCr2U.clone())?;
            if b.clone() {
                expList2 = metamodelica::nil();
            } else {
                temp = makenewcref(lhs.clone(), (formalvar.clone()).clone(), (s1.clone()).clone())?;
                expList2 = cons(list![(lhs.clone(), ut1.clone()), (temp.clone(), ut2.clone())], metamodelica::nil());
            }
            let (_, (__pa4, __pa5, __pa6), __pa7) = insertUnitInEquation(var_field!((*eq).scalar, DAE::Element::EQUATION).clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa4.clone();
            htS2U = __pa5.clone();
            htU2S = __pa6.clone();
            expList3 = __pa7.clone();
            List::append_reverse(expList2.clone(), expList3.clone())
        },
        Deref @ DAE::Element::EQUATION { .. } => {
            let mut temp: Arc<DAE::Exp>;
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).scalar, DAE::Element::EQUATION).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: var_field!((*eq).exp, DAE::Element::EQUATION).clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::EQUEQUATION { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIALEQUATION { .. } => {
            let mut temp: Arc<DAE::Exp>;
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).exp2, DAE::Element::INITIALEQUATION).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: var_field!((*eq).exp1, DAE::Element::INITIALEQUATION).clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => {
            let mut temp: Arc<DAE::Exp>;
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).array, DAE::Element::ARRAY_EQUATION).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: var_field!((*eq).exp, DAE::Element::ARRAY_EQUATION).clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
            let mut temp: Arc<DAE::Exp>;
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).array, DAE::Element::INITIAL_ARRAY_EQUATION).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: var_field!((*eq).exp, DAE::Element::INITIAL_ARRAY_EQUATION).clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { .. } => {
            let mut temp: Arc<DAE::Exp>;
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).rhs, DAE::Element::COMPLEX_EQUATION).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: var_field!((*eq).lhs, DAE::Element::COMPLEX_EQUATION).clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
            let mut temp: Arc<DAE::Exp>;
            temp = Arc::new(DAE::Exp::BINARY { exp1: var_field!((*eq).rhs, DAE::Element::INITIAL_COMPLEX_EQUATION).clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: var_field!((*eq).lhs, DAE::Element::INITIAL_COMPLEX_EQUATION).clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(temp.clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::WHEN_EQUATION { .. } => {
            for mut e in &*var_field!((*eq).equations, DAE::Element::WHEN_EQUATION).clone() {
                let mut e = e.clone();
                (htCr2U, htS2U, htU2S, inconsistentUnits) = foldEquation2(e.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), args.clone())?;
            }
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::IF_EQUATION { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(var_field!((*eq).exp, DAE::Element::NORETCALL).clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(var_field!((*eq).exp, DAE::Element::INITIAL_NORETCALL).clone(), (htCr2U.clone(), htS2U.clone(), htU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, args.clone())?;
            htCr2U = __pa0.clone();
            htS2U = __pa1.clone();
            htU2S = __pa2.clone();
            inconsistentUnits = __pa3.clone();
            inconsistentUnits.clone()
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::REINIT { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            metamodelica::nil()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FUnitCheck.foldEquation2")); __mm_s.push_str(&*literal!(" failed on: ")); __mm_s.push_str(&*DAEDump::dumpEquationStr(eq.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((htCr2U, htS2U, htU2S, inconsistentUnits))
}

fn makenewcref(mut inexp: Arc<DAE::Exp>, mut instring: ArcStr, mut instring1: ArcStr) -> Result<Arc<DAE::Exp>> {
    let mut outexp: Arc<DAE::Exp>;
    outexp = (::match_deref::match_deref! { match &((inexp.clone(), instring.clone(), instring1.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, .. }, s1, s2) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut name = (*name).clone();
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("()")); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone();
            cr = ComponentReference::makeUntypedCrefIdent((name.clone()).clone());
            assign_variant_field!(inexp => DAE::Exp::CREF; componentRef = cr.clone());
            outexp = inexp.clone();
            outexp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outexp)
}

fn insertUnitInEquation(mut inEq: Arc<DAE::Exp>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))), mut inUt: Unit::Unit, mut inargs: Arc<metamodelica::List<Functionargs>>) -> Result<(Unit::Unit, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))), Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>>)> {
    let mut outUt: Unit::Unit;
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)));
    let mut outexpList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
    (outUt, outTpl, outexpList) = 'mc: {
        let __mc_input = (inEq.clone(), inTpl.clone(), inUt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    ut = __pa10.clone();
                    HtCr2U = __pa11.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut2 = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut2.clone(), inargs.clone())?;
                    ut = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    ut = __pa10.clone();
                    HtCr2U = __pa11.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    expListList = cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut2 = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut2.clone(), inargs.clone())?;
                    ut = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    expListList = cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    ut = __pa10.clone();
                    HtCr2U = __pa11.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut2 = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut2.clone(), inargs.clone())?;
                    ut = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    ut = __pa10.clone();
                    HtCr2U = __pa11.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    expListList = cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut2 = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut2.clone(), inargs.clone())?;
                    ut = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    expListList = cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    ut = Unit::unitMul(ut.clone(), ut2.clone())?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::UNIT { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa4.clone();
                    HtS2U = __pa5.clone();
                    HtU2S = __pa6.clone();
                    expListList2 = __pa7.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lcr = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    ut = Unit::unitDiv(inUt.clone(), ut2.clone())?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtCr2U, _, _, _) = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), HtCr2U.clone());
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::UNIT { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa4.clone();
                    HtS2U = __pa5.clone();
                    HtU2S = __pa6.clone();
                    expListList2 = __pa7.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut2 = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa5 }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lcr = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    ut = Unit::unitDiv(inUt.clone(), ut2.clone())?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtCr2U, _, _, _) = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), HtCr2U.clone());
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa4.clone();
                    HtS2U = __pa5.clone();
                    HtU2S = __pa6.clone();
                    expListList2 = __pa7.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    ut = Unit::unitDiv(ut.clone(), ut2.clone())?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::UNIT { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa4.clone();
                    HtS2U = __pa5.clone();
                    HtU2S = __pa6.clone();
                    expListList2 = __pa7.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lcr = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    ut = Unit::unitMul(inUt.clone(), ut2.clone())?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtCr2U, _, _, _) = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), HtCr2U.clone());
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::UNIT { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa4.clone();
                    HtS2U = __pa5.clone();
                    HtU2S = __pa6.clone();
                    expListList2 = __pa7.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut2 = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa5 }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lcr = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    ut = Unit::unitDiv(ut2.clone(), inUt.clone())?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtCr2U, _, _, _) = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), HtCr2U.clone());
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa4.clone();
                    HtS2U = __pa5.clone();
                    HtU2S = __pa6.clone();
                    expListList2 = __pa7.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::RCONST { real: r } }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut i: i32 = 0;
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    i = ((r.clone()).0 as i32);
                    let true = (realEq(r.clone(), intReal(i.clone()))) else { bail!("pattern mismatch") };
                    ut = Unit::unitPow(ut.clone(), i.clone())?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::RCONST { real: r } }, (HtCr2U, HtS2U, HtU2S), ut @ Unit::Unit::UNIT { .. }) => {
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut i3: i32 = 0;
                    let mut i4: i32 = 0;
                    let mut i5: i32 = 0;
                    let mut i6: i32 = 0;
                    let mut i7: i32 = 0;
                    let mut lcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut factor1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lcr = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let Unit::UNIT { factor: __pa5, mol: __pa6, cd: __pa7, m: __pa8, s: __pa9, A: __pa10, K: __pa11, g: __pa12 } = (Unit::unitRoot(ut.clone(), r.clone())?) else { bail!("pattern mismatch") };
                    factor1 = __pa5.clone();
                    i1 = __pa6.clone();
                    i2 = __pa7.clone();
                    i3 = __pa8.clone();
                    i4 = __pa9.clone();
                    i5 = __pa10.clone();
                    i6 = __pa11.clone();
                    i7 = __pa12.clone();
                    (HtCr2U, _, _, _) = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }, HtCr2U.clone());
                    s1 = (Unit::unitString(Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }, HtU2S.clone())?).clone();
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }), HtU2S.clone())?;
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::RCONST { real: _ } }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?;
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lcr = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    ut = Unit::unitMul(inUt.clone(), Unit::Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 })?;
                    (HtCr2U, _, _, _) = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), HtCr2U.clone());
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    ut = Unit::unitDiv(ut.clone(), Unit::Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 })?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut i3: i32 = 0;
                    let mut i4: i32 = 0;
                    let mut i5: i32 = 0;
                    let mut i6: i32 = 0;
                    let mut i7: i32 = 0;
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut factor1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let Unit::UNIT { factor: __pa5, mol: __pa6, cd: __pa7, m: __pa8, s: __pa9, A: __pa10, K: __pa11, g: __pa12 } = (Unit::unitRoot(ut.clone(), metamodelica::OrderedFloat(2.0_f64))?) else { bail!("pattern mismatch") };
                    factor1 = __pa5.clone();
                    i1 = __pa6.clone();
                    i2 = __pa7.clone();
                    i3 = __pa8.clone();
                    i4 = __pa9.clone();
                    i5 = __pa10.clone();
                    i6 = __pa11.clone();
                    i7 = __pa12.clone();
                    s1 = (Unit::unitString(Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }, HtU2S.clone())?).clone();
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }), HtU2S.clone())?;
                    Ok((Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lcr = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    ut = Unit::unitPow(inUt.clone(), 2)?;
                    s1 = (Unit::unitString(ut.clone(), HtU2S.clone())?).clone();
                    (HtCr2U, _, _, _) = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), HtCr2U.clone());
                    (HtS2U, _, _, _) = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((s1.clone(), ut.clone()), HtU2S.clone())?;
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs.clone())?;
                    HtCr2U = __pa0.clone();
                    HtS2U = __pa1.clone();
                    HtU2S = __pa2.clone();
                    expListList = __pa3.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: exp2, expElse: exp3 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList3: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList2 = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp3.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList3 = __pa9.clone();
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    ut = __pa10.clone();
                    HtCr2U = __pa11.clone();
                    expListList = List::append_reverse(expListList2.clone(), expListList3.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: exp2, expElse: exp3 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList3: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp2.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList2 = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp3.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), ut.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList3 = __pa9.clone();
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    expListList = List::append_reverse(expListList2.clone(), expListList3.clone());
                    expListList = cons(list![(exp2.clone(), ut.clone()), (exp3.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    ut = __pa10.clone();
                    HtCr2U = __pa11.clone();
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp2, exp1, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut expListList2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut2 = __pa5.clone();
                    HtCr2U = __pa6.clone();
                    HtS2U = __pa7.clone();
                    HtU2S = __pa8.clone();
                    expListList2 = __pa9.clone();
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())?) else { bail!("pattern mismatch") };
                    expListList = List::append_reverse(expListList.clone(), expListList2.clone());
                    expListList = cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: ExpList, path: Deref @ Absyn::Path::IDENT { .. }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    (HtCr2U, HtS2U, HtU2S, expListList) = foldCallArg(ExpList.clone(), HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())?;
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: ExpList, path: Deref @ Absyn::Path::FULLYQUALIFIED { path }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut invars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut inunitlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    s1 = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    s1 = (System::trim((s1.clone()).clone(), (literal!(".")).clone())).clone();
                    (invars, _, inunitlist, _) = getNamedUnitlist((s1.clone()).clone(), inargs.clone());
                    (HtCr2U, HtS2U, HtU2S, expListList) = foldCallArg1(ExpList.clone(), HtCr2U.clone(), HtS2U.clone(), HtU2S.clone(), inUt.clone(), inunitlist.clone(), invars.clone(), (s1.clone()).clone())?;
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: exp1 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp1.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), inUt.clone(), inargs.clone())?;
                    ut = __pa0.clone();
                    HtCr2U = __pa1.clone();
                    HtS2U = __pa2.clone();
                    HtU2S = __pa3.clone();
                    expListList = __pa4.clone();
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut ut: Unit::Unit;
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let true = (ComponentReferenceBasics::crefEqual(cr.clone(), DAE::crefTime.clone())?) else { bail!("pattern mismatch") };
                    ut = Unit::Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 };
                    (HtS2U, _, _, _) = addUnit2HtS2U((literal!("time"), ut.clone()), HtS2U.clone())?;
                    (HtU2S, _, _, _) = addUnit2HtU2S((literal!("time"), ut.clone()), HtU2S.clone())?;
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_REAL { .. }, componentRef: cr }, (HtCr2U, _, _), _) => {
                    let mut ut: Unit::Unit;
                    ut = BaseHashTable::get(cr.clone(), HtCr2U.clone())?;
                    Ok((ut.clone(), inTpl.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, inTpl.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outUt, outTpl, outexpList))
}

fn getNamedUnitlist(mut instring: ArcStr, mut inargs: Arc<metamodelica::List<Functionargs>>) -> (Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>) {
    let mut outargs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outargs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outargs3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outargs4: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outargs, outargs2, outargs3, outargs4) = (::match_deref::match_deref! { match &((instring.clone(), inargs.clone())) {
        (fnname, Deref @ metamodelica::List::Cons { head: Functionargs { name: fnname1, invars, outvars, inunits: inunitlist, outunits: outunitlist }, tail: _ }) if (stringEq((fnname.clone()).clone(), (fnname1.clone()).clone())) => {
            let mut inunitlist = (*inunitlist).clone();
            let mut outunitlist = (*outunitlist).clone();
            inunitlist = inunitlist.clone();
            outunitlist = outunitlist.clone();
            (invars.clone(), outvars.clone(), inunitlist.clone(), outunitlist.clone())
        },
        (fnname, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut invars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut inunitlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut outunitlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut outvars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (invars, outvars, inunitlist, outunitlist) = getNamedUnitlist((fnname.clone()).clone(), rest.clone());
            (invars.clone(), outvars.clone(), inunitlist.clone(), outunitlist.clone())
        },
        (_, _) => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outargs, outargs2, outargs3, outargs4)
}

fn UnitTypesEqual(mut inut: Unit::Unit, mut inut2: Unit::Unit, mut inHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr))) -> Result<(bool, Unit::Unit, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)))> {
    let mut b: bool = false;
    let mut outUt: Unit::Unit;
    let mut outHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr));
    (b, outUt, outHtCr2U) = 'mc: {
        let __mc_input = (inut.clone(), inut2.clone(), inHtCr2U.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::UNIT { factor: mut factor1, mol: mut i1, cd: mut i2, m: mut i3, s: mut i4, A: mut i5, K: mut i6, g: mut i7 }, Unit::Unit::UNIT { factor: mut factor2, mol: mut j1, cd: mut j2, m: mut j3, s: mut j4, A: mut j5, K: mut j6, g: mut j7 }, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (realEq(factor1.clone(), factor2.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i1.clone(), j1.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i2.clone(), j2.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i3.clone(), j3.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i4.clone(), j4.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i5.clone(), j5.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i6.clone(), j6.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i7.clone(), j7.clone())) else { bail!("pattern mismatch") };
            Ok((true, Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }, inHtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::UNIT { factor: mut factor1, mol: mut i1, cd: mut i2, m: mut i3, s: mut i4, A: mut i5, K: mut i6, g: mut i7 }, Unit::Unit::UNIT { factor: mut factor2, mol: mut j1, cd: mut j2, m: mut j3, s: mut j4, A: mut j5, K: mut j6, g: mut j7 }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r = realMax(realAbs(factor1.clone()), realAbs(factor2.clone()));
            let true = (realLe(realDiv(realAbs((factor1.clone()) - (factor2.clone())), r.clone()), metamodelica::OrderedFloat(1e-3_f64))) else { bail!("pattern mismatch") };
            let true = (intEq(i1.clone(), j1.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i2.clone(), j2.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i3.clone(), j3.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i4.clone(), j4.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i5.clone(), j5.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i6.clone(), j6.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(i7.clone(), j7.clone())) else { bail!("pattern mismatch") };
            Ok((true, Unit::Unit::UNIT { factor: factor1.clone(), mol: i1.clone(), cd: i2.clone(), m: i3.clone(), s: i4.clone(), A: i5.clone(), K: i6.clone(), g: i7.clone() }, inHtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut ut @ Unit::Unit::UNIT { .. }, Unit::Unit::MASTER { varList: ref lcr }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut HtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr));
            HtCr2U = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), inHtCr2U.clone());
            Ok((true, ut.clone(), HtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::MASTER { varList: ref lcr }, mut ut @ Unit::Unit::UNIT { .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut HtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr));
            HtCr2U = List::fold1(lcr.clone(), Arc::new(updateHtCr2U), ut.clone(), inHtCr2U.clone());
            Ok((true, ut.clone(), HtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::MASTER { varList: ref lcr }, Unit::Unit::MASTER { varList: ref lcr2 }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut lcr2 = lcr2.clone();
            lcr2 = List::append_reverse(lcr.clone(), lcr2.clone());
            Ok((true, Unit::Unit::MASTER { varList: lcr2.clone() }, inHtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::UNKNOWN { unit: mut s }, Unit::Unit::UNKNOWN { unit: mut s2 }, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (stringEqual((s.clone()).clone(), (s2.clone()).clone())) else { bail!("pattern mismatch") };
            Ok((true, Unit::Unit::UNKNOWN { unit: (s.clone()).clone() }, inHtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::UNKNOWN { unit: mut s }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok((true, Unit::Unit::UNKNOWN { unit: (s.clone()).clone() }, inHtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, Unit::Unit::UNKNOWN { unit: mut s }, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok((true, Unit::Unit::UNKNOWN { unit: (s.clone()).clone() }, inHtCr2U.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((false, inut.clone(), inHtCr2U.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((b, outUt, outHtCr2U))
}

fn updateHtCr2U(mut inCr: Arc<DAE::ComponentRef>, mut inUt: Unit::Unit, mut inHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr))> {
    let mut outHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr));
    outHtCr2U = 'mc: {
        let __mc_input = (inCr.clone(), inUt.clone(), inHtCr2U.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (BaseHashTable::hasKey(Unit::UPDATECREF.clone(), inHtCr2U.clone())) else { bail!("pattern mismatch") };
                    BaseHashTable::update((inCr.clone(), inUt.clone()), inHtCr2U.clone())?;
                    Ok(inHtCr2U.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut HtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr));
                    HtCr2U = BaseHashTable::add((Unit::UPDATECREF.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }), inHtCr2U.clone())?;
                    BaseHashTable::update((inCr.clone(), inUt.clone()), HtCr2U.clone())?;
                    Ok(HtCr2U.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHtCr2U)
}

fn Errorfunction(mut inexpList: Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>, mut inEq: Arc<DAE::Element>, mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inexpList.clone(), inEq.clone(), inHtU2S.clone())) {
        (expList, _, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut info: SourceInfo;
            info = getSourceInfo(inEq.clone())?;
            s = (DAEDump::dumpEquationStr(inEq.clone())?).clone();
            s1 = (Errorfunction2(expList.clone(), inHtU2S.clone())?).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following equation is INCONSISTENT due to specified unit information:")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![(s2.clone()).clone()], info.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The units of following sub-expressions need to be equal:\n")); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getSourceInfo(mut inequation: Arc<DAE::Element>) -> Result<SourceInfo> {
    let mut outinfo: SourceInfo;
    outinfo = (::match_deref::match_deref! { match &(inequation.clone()) {
        Deref @ DAE::Element::EQUATION { source: Deref @ DAE::ElementSource { info, .. }, .. } => {
            info.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outinfo)
}

fn Errorfunction2(mut inexpList: Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>, mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    outS = ((::match_deref::match_deref! { match &((inexpList.clone(), inHtU2S.clone())) {
        (Deref @ metamodelica::List::Cons { head: (exp, ut), tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            s = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            s1 = (Unit::unitString(ut.clone(), inHtU2S.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (exp, ut), tail: expList }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            s1 = (Unit::unitString(ut.clone(), inHtU2S.clone())?).clone();
            s2 = (Errorfunction2(expList.clone(), inHtU2S.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\"\n")); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outS)
}

pub fn GetVarList(mut indaelist: DAE::DAElist) -> Arc<metamodelica::List<Arc<DAE::Element>>> {
    let mut outstring: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut varlist: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outstring = (::match_deref::match_deref! { match &(indaelist.clone()) {
        DAE::DAElist { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMP { dAElist: varlist, .. }, tail: Deref @ metamodelica::List::Nil } } => varlist.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outstring
}

pub fn GetElementList(mut eqlist: DAE::DAElist) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outstring: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut eq1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut eq2: Arc<DAE::Element>;
    outstring = (match eqlist.clone() {
        DAE::DAElist { elementLst: ref eq1 } => eq1.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outstring)
}

fn foldCallArg(mut inExpList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut inHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)), Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>>)> {
    let mut outHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)) = inHtCr2U.clone();
    let mut outHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)) = inHtS2U.clone();
    let mut outHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)) = inHtU2S.clone();
    let mut outExpListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
    for mut exp in &*inExpList.clone() {
        let mut exp = exp.clone();
        let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(exp.clone(), (outHtCr2U.clone(), outHtS2U.clone(), outHtU2S.clone()), Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::nil())?;
        outHtCr2U = __pa0.clone();
        outHtS2U = __pa1.clone();
        outHtU2S = __pa2.clone();
        expListList = __pa3.clone();
        outExpListList = List::append_reverse(expListList.clone(), outExpListList.clone());
    }
    outExpListList = outExpListList.clone().reverse();
    Ok((outHtCr2U, outHtS2U, outHtU2S, outExpListList))
}

fn foldCallArg1(mut inExpList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), mut inHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)), mut inunit: Unit::Unit, mut unitlist: Arc<metamodelica::List<ArcStr>>, mut invars: Arc<metamodelica::List<ArcStr>>, mut fname: ArcStr) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)), Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>>)> {
    let mut outHtCr2U: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)) = inHtCr2U.clone();
    let mut outHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)) = inHtS2U.clone();
    let mut outHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)) = inHtU2S.clone();
    let mut outExpListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
    let mut expListList: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Exp>, Unit::Unit)>>>> = metamodelica::nil();
    let mut ut: Unit::Unit;
    let mut ut1: Unit::Unit;
    let mut ut2: Unit::Unit;
    let mut s: ArcStr = arcstr::literal!("");
    let mut formalarg: ArcStr = arcstr::literal!("");
    let mut formalvar: ArcStr = arcstr::literal!("");
    let mut args: Arc<metamodelica::List<Functionargs>> = metamodelica::nil();
    let mut exp: Arc<DAE::Exp>;
    let mut temp: Arc<DAE::Exp>;
    let mut count: i32 = 0;
    let mut b: bool = false;
    for mut i in 1..=(inExpList.clone().len() as i32) {
        exp = (inExpList.clone()).get(i.clone())?;
        formalarg = ((unitlist.clone()).get(i.clone())?).clone();
        formalvar = ((invars.clone()).get(i.clone())?).clone();
        let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(exp.clone(), (outHtCr2U.clone(), outHtS2U.clone(), outHtU2S.clone()), inunit.clone(), metamodelica::nil())?;
        ut = __pa0.clone();
        outHtCr2U = __pa1.clone();
        outHtS2U = __pa2.clone();
        outHtU2S = __pa3.clone();
        expListList = __pa4.clone();
        if formalarg.clone() == literal!("NONE") {
            ut1 = Unit::Unit::MASTER { varList: metamodelica::nil() };
        } else {
            ut1 = Unit::parseUnitString((formalarg.clone()).clone(), Unit::getKnownUnits()?)?;
        }
        s = (Unit::unitString(ut.clone(), outHtU2S.clone())?).clone();
        (b, ut, _) = UnitTypesEqual(ut.clone(), ut1.clone(), outHtCr2U.clone())?;
        if b.clone() == true {
            expListList = metamodelica::nil();
        } else {
            temp = makenewcref(exp.clone(), (formalvar.clone()).clone(), (fname.clone()).clone())?;
            expListList = cons(list![(exp.clone(), ut.clone()), (temp.clone(), ut1.clone())], metamodelica::nil());
        }
        outExpListList = List::append_reverse(expListList.clone(), outExpListList.clone());
    }
    Ok((outHtCr2U, outHtS2U, outHtU2S, outExpListList))
}

fn addUnit2HtS2U(mut inTpl: (ArcStr, Unit::Unit), mut inHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr))> {
    let mut outHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr));
    outHtS2U = BaseHashTable::add(inTpl.clone(), inHtS2U.clone())?;
    Ok(outHtS2U)
}

fn addUnit2HtU2S(mut inTpl: (ArcStr, Unit::Unit), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))> {
    let mut outHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr));
    outHtU2S = 'mc: {
        let __mc_input = (inTpl.clone(), inHtU2S.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let ((mut s, mut ut), _) = __mc_input.clone() else { bail!("nomatch") };
            let mut HtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr));
            let false = (BaseHashTable::hasKey(ut.clone(), inHtU2S.clone())) else { bail!("pattern mismatch") };
            HtU2S = BaseHashTable::add((ut.clone(), s.clone()), inHtU2S.clone())?;
            Ok(HtU2S.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inHtU2S.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outHtU2S)
}

// get unit information based on old instantiation
fn convertUnitString2unit_old(mut var: Arc<DAE::Element>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))> {
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)));
    outTpl = (::match_deref::match_deref! { match &((var.clone(), inTpl.clone())) {
        (Deref @ DAE::Element::VAR { variableAttributesOption: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(Deref @ DAE::Exp::SCONST { string: unitString }), .. }), ty: Deref @ DAE::Type::T_REAL { .. }, componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S)) if (unitString.clone() != literal!("")) => {
            let mut ut: Unit::Unit;
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            (ut, HtS2U, HtU2S) = parse((unitString.clone()).clone(), cr.clone(), HtS2U.clone(), HtU2S.clone())?;
            HtCr2U = BaseHashTable::add((cr.clone(), ut.clone()), HtCr2U.clone())?;
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S)) => {
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            HtCr2U = BaseHashTable::add((cr.clone(), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtCr2U.clone())?;
            HtS2U = addUnit2HtS2U((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtS2U.clone())?;
            HtU2S = addUnit2HtU2S((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtU2S.clone())?;
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        _ => {
            inTpl.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

//based on new Instantiation currently not fully operational
fn convertUnitString2unit(mut var: Arc<DAE::Element>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))> {
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Unit::Unit)>>), i32, (HashTableCrToUnit::FuncHashKey, HashTableCrToUnit::FuncKeyEqual, HashTableCrToUnit::FuncKeyStr, HashTableCrToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)));
    outTpl = (::match_deref::match_deref! { match &((var.clone(), inTpl.clone())) {
        (Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_REAL { varLst: varlst }, componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S)) if (false == varlst.clone().is_empty()) => {
            let mut unitString: ArcStr = arcstr::literal!("");
            let mut ut: Unit::Unit;
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            unitString = (parseVarList(varlst.clone())).clone();
            (ut, HtS2U, HtU2S) = parse((unitString.clone()).clone(), cr.clone(), HtS2U.clone(), HtU2S.clone())?;
            HtCr2U = BaseHashTable::add((cr.clone(), ut.clone()), HtCr2U.clone())?;
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S)) => {
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            HtCr2U = BaseHashTable::add((cr.clone(), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtCr2U.clone())?;
            HtS2U = addUnit2HtS2U((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtS2U.clone())?;
            HtU2S = addUnit2HtU2S((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtU2S.clone())?;
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn parseVarList(mut invarlist: Arc<metamodelica::List<Arc<DAE::Var>>>) -> ArcStr {
    let mut outstring: ArcStr = arcstr::literal!("");
    outstring = ((::match_deref::match_deref! { match &(invarlist.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { binding: eqbind, name, .. }, tail: _ } if (stringEq((name.clone()).clone(), (literal!("unit")).clone())) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (getStringFromExp(eqbind.clone())).clone();
            s.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: varlist } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (parseVarList(varlist.clone())).clone();
            s.clone()
        },
        Deref @ metamodelica::List::Nil => {
            literal!("None")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outstring
}

pub fn getStringFromExp(mut binding: Arc<DAE::Binding>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ DAE::Binding::UNBOUND => {
            literal!("")
        },
        Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::SCONST { string: str1 }, .. } => {
            str1.clone()
        },
        _ => {
            literal!("None")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    r#str
}

fn parse(mut inUnitString: ArcStr, mut inCref: Arc<DAE::ComponentRef>, mut inHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), mut inHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr))) -> Result<(Unit::Unit, (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)), (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)))> {
    let mut outUnit: Unit::Unit;
    let mut outHtS2U: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (HashTableStringToUnit::FuncHashKey, HashTableStringToUnit::FuncKeyEqual, HashTableStringToUnit::FuncKeyStr, HashTableStringToUnit::FuncValueStr)) = inHtS2U.clone();
    let mut outHtU2S: (metamodelica::Array<Arc<metamodelica::List<(Unit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (HashTableUnitToString::FuncHashKey, HashTableUnitToString::FuncKeyEqual, HashTableUnitToString::FuncKeyStr, HashTableUnitToString::FuncValueStr)) = inHtU2S.clone();
    if inUnitString.clone() == literal!("") {
        outUnit = Unit::Unit::MASTER { varList: list![inCref.clone()] };
        return Ok((outUnit, outHtS2U, outHtU2S));
    }
    match '__try0: {
        outUnit = unwrap_break_err!(BaseHashTable::get((inUnitString.clone()).clone(), inHtS2U.clone()), '__try0);
        Ok::<_, anyhow::Error>((outUnit.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outUnit = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                outUnit = unwrap_break_err!(Unit::parseUnitString((inUnitString.clone()).clone(), inHtS2U.clone()), '__try1);
                Ok::<_, anyhow::Error>((outUnit.clone(),))
            } {
                Ok((__try1_o0,)) => {
                    outUnit = __try1_o0;
                }
                Err(_) => {
                    outUnit = Unit::Unit::UNKNOWN { unit: (inUnitString.clone()).clone() };
                }
            }
            outHtS2U = addUnit2HtS2U((inUnitString.clone(), outUnit.clone()), outHtS2U.clone())?;
            outHtU2S = addUnit2HtU2S((inUnitString.clone(), outUnit.clone()), outHtU2S.clone())?;
        }
    }
    Ok((outUnit, outHtS2U, outHtU2S))
}

