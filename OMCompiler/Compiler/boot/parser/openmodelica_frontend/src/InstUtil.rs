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

use crate::AvlSetCR;
use crate::Ceval;
use crate::ComponentReference;
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::HashSet;
use crate::HashTable;
use crate::InnerOuter;
use crate::Inst;
use crate::InstExtends;
use crate::InstFunction;
use crate::Lookup;
use crate::Mod;
use crate::NFSCodeFlatten;
use crate::Patternm;
use crate::PrefixUtil;
use crate::Static;
use crate::Types;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use crate::UnitChecker;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTable5;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::InstBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = Arc<metamodelica::List<InnerOuter::TopInstance>>;

pub type InstDims = Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>;

pub fn newIdent() -> Arc<DAE::ComponentRef> {
    let mut outComponentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut i: i32 = 0;
    let mut is: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    i = tick();
    is = (intString(i.clone())).clone();
    s = (stringAppend((literal!("__TMP__")).clone(), (is.clone()).clone())).clone();
    outComponentRef = ComponentReferenceBasics::makeCrefIdent((s.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    outComponentRef
}

fn isNotFunction(mut cls: Arc<SCode::Element>) -> bool {
    let mut res: bool = false;
    res = SCodeUtil::isFunction(cls.clone());
    res = boolNot(res.clone());
    res
}

pub fn scodeFlatten(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = (inProgram.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let false = (Flags::isSet(Flags::DO_SCODE_DEP.clone())?) else { bail!("pattern mismatch") };
                    Ok(inProgram.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Path::IDENT { name: Deref @ "" }) => {
                    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = outProgram.clone();
                    outProgram = scodeFlattenProgram(inProgram.clone())?;
                    Ok(outProgram.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = outProgram.clone();
                    (outProgram, _) = NFSCodeFlatten::flattenClassInProgram(inPath.clone(), inProgram.clone())?;
                    Ok(outProgram.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProgram)
}

fn scodeFlattenProgram(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>> = outProgram.clone();
                    ErrorExt::setCheckpoint((literal!("scodeFlattenProgram")).clone());
                    outProgram = NFSCodeFlatten::flattenCompleteProgram(inProgram.clone())?;
                    ErrorExt::delCheckpoint((literal!("scodeFlattenProgram")).clone());
                    Ok(outProgram.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack((literal!("scodeFlattenProgram")).clone());
                    Ok(inProgram.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProgram)
}

pub fn reEvaluateInitialIfEqns(mut cache: FCore::Cache, mut env: FCore::Graph, mut dae: DAE::DAElist, mut isTopCall: bool) -> Result<DAE::DAElist> {
    let mut odae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    odae = (match (cache.clone(), env.clone(), dae.clone(), isTopCall.clone()) {
        (_, _, DAE::DAElist { elementLst: ref elems }, true) => {
            let mut elems = elems.clone();
            elems = List::fold2r(elems.clone(), (std::sync::Arc::new(reEvaluateInitialIfEqns2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<DAE::Element>, FCore::Cache, FCore::Graph) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> + 'static>), cache.clone(), env.clone(), metamodelica::nil()).reverse();
            DAE::DAElist { elementLst: elems.clone() }
        },
        (_, _, _, false) => {
            dae.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(odae)
}

fn reEvaluateInitialIfEqns2(mut acc: Arc<metamodelica::List<Arc<DAE::Element>>>, mut elem: Arc<DAE::Element>, mut inCache: FCore::Cache, mut env: FCore::Graph) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut oelems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    oelems = 'mc: {
        let __mc_input = (acc.clone(), elem.clone(), inCache.clone(), env.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Element::INITIAL_IF_EQUATION { equations3: fb, equations2: tbs, condition1: conds, .. }, cache, _) => {
                    let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut selectedBranch: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut blist: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    (_, valList) = Ceval::cevalList(cache.clone(), env.clone(), conds.clone(), true, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    blist = List::map(valList.clone(), (std::sync::Arc::new(ValuesUtil::valueBool) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<bool> + 'static>));
                    selectedBranch = List::findBoolList(blist.clone(), tbs.clone(), fb.clone())?;
                    selectedBranch = makeDAEElementInitial(selectedBranch.clone())?;
                    Ok(listAppend(selectedBranch.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cons(elem.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oelems)
}

fn makeDAEElementInitial(mut inElems: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElems: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outElems = (::match_deref::match_deref! { match &(inElems.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e1, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            cons(Arc::new(DAE::Element::INITIALDEFINE { componentRef: cr.clone(), exp: e1.clone(), source: s.clone() }), outElems.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            cons(Arc::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: dims.clone(), exp: e1.clone(), array: e2.clone(), source: s.clone() }), outElems.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            cons(Arc::new(DAE::Element::INITIALEQUATION { exp1: e1.clone(), exp2: e2.clone(), source: s.clone() }), outElems.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: tbs, equations3: fb, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            cons(Arc::new(DAE::Element::INITIAL_IF_EQUATION { condition1: expl.clone(), equations2: tbs.clone(), equations3: fb.clone(), source: s.clone() }), outElems.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: al, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            cons(Arc::new(DAE::Element::INITIALALGORITHM { algorithm_: al.clone(), source: s.clone() }), outElems.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            cons(Arc::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1.clone(), rhs: e2.clone(), source: s.clone() }), outElems.clone())
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: elems } => {
            outElems = makeDAEElementInitial(elems.clone())?;
            cons(elem.clone(), outElems.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElems)
}

pub fn lookupTopLevelClass(mut inName: ArcStr, mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPrintError: bool) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    outClass = 'mc: {
        let __mc_input = (inName.clone(), inProgram.clone(), inPrintError.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut cls: Arc<SCode::Element>;
                    cls = List::getMemberOnTrue((inName.clone()).clone(), inProgram.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isClassNamed, ArcStr, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SCode::Element>) -> Result<bool> + 'static>))?;
                    Ok(cls.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, true) => {
                    Error::addMessage(Error::LOAD_MODEL_ERROR.clone(), list![(inName.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

pub fn fixInstClassType(mut ty: Arc<DAE::Type>, mut isPartialFn: bool) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (ty.clone(), isPartialFn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::TYPE { path: path1 }, .. }, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut path2: Arc<Absyn::Path>;
                    name = (AbsynUtil::pathLastIdent(path1.clone())?).clone();
                    path2 = AbsynUtil::stripLast(path1.clone())?;
                    ::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(path2.clone())?) {
                        Deref @ "$Code" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    path2 = AbsynUtil::stripLast(path2.clone())?;
                    ::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(path2.clone())?) {
                        Deref @ "OpenModelica" => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(Util::assoc((name.clone()).clone(), list![(literal!("Expression"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION })), (literal!("ExpressionOrModification"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION_OR_MODIFICATION })), (literal!("TypeName"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_TYPENAME })), (literal!("VariableName"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME })), (literal!("VariableNames"), Arc::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAMES }))])?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false) => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true) => {
                    Ok(Types::makeFunctionPolymorphicReference(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn updateEnumerationEnvironment(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inType: Arc<DAE::Type>, mut inClass: Arc<SCode::Element>, mut inCi_State: ClassInf::State) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inType.clone(), inClass.clone(), inCi_State.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Type::T_ENUMERATION { path: p, literalVarLst: vars, names, .. }, _, ClassInf::State::ENUMERATION { path: pname }) => {
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, env_1) = updateEnumerationEnvironment1(cache.clone(), env.clone(), (AbsynUtil::pathString(pname.clone(), (literal!(".")).clone(), true, false)?).clone(), names.clone(), vars.clone(), p.clone())?;
                    Ok((cache.clone(), env_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, _) => {
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv))
}

fn updateEnumerationEnvironment1(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inName: ArcStr, mut inNames: Arc<metamodelica::List<ArcStr>>, mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inName.clone(), inNames.clone(), inVars.clone(), inPath.clone())) {
        (cache, env, _, Deref @ metamodelica::List::Cons { head: nn, tail: names }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, .. }, tail: vars }, p) => {
            let mut env_1: FCore::Graph;
            let mut env_2: FCore::Graph;
            let mut compenv: FCore::Graph;
            let mut var: Arc<DAE::Var> = Arc::new(<DAE::Var as ::std::default::Default>::default());
            let mut cache = (*cache).clone();
            (cache, var, _, _, _, compenv) = Lookup::lookupIdentLocal(cache.clone(), env.clone(), (nn.clone()).clone())?;
            assign_field!(var.ty = ty.clone());
            env_1 = FGraph::updateComp(env.clone(), var.clone(), crate::FCore::Status::VAR_DAE, compenv.clone())?;
            (cache, env_2) = updateEnumerationEnvironment1(cache.clone(), env_1.clone(), (var.name.clone()).clone(), names.clone(), vars.clone(), p.clone())?;
            (cache.clone(), env_2.clone())
        },
        (cache, env, _, Deref @ metamodelica::List::Nil, _, _) => {
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

pub fn updateDeducedUnits(mut callScope: bool, mut store: UnitAbsyn::InstStore, mut dae: DAE::DAElist) -> DAE::DAElist {
    let mut outDae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    outDae = (match (callScope.clone(), store.clone(), dae.clone()) {
        (true, UnitAbsyn::InstStore::INSTSTORE { store: UnitAbsyn::Store { storeVector: mut vec, numElts: _ }, ht: mut ht, checkResult: _ }, DAE::DAElist { elementLst: ref elts }) => {
            let mut elts = elts.clone();
            elts = List::map2(elts.clone(), (std::sync::Arc::new(updateDeducedUnits2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, metamodelica::Array<Option<UnitAbsyn::Unit>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<Arc<DAE::Element>> + 'static>), vec.clone(), ht.clone());
            DAE::DAElist { elementLst: elts.clone() }
        },
        _ => {
            dae.clone()
        },
    });
    outDae
}

fn updateDeducedUnits2(mut elt: Arc<DAE::Element>, mut vec: metamodelica::Array<Option<UnitAbsyn::Unit>>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr))) -> Result<Arc<DAE::Element>> {
    let mut oelt: Arc<DAE::Element>;
    oelt = 'mc: {
        let __mc_input = (elt.clone(), vec.clone(), ht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { variableAttributesOption: varOpt @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: None, .. }), componentRef: cr, .. }, _, _) => {
                    let mut indx: i32 = 0;
                    let mut unitStr: ArcStr = arcstr::literal!("");
                    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut varOpt = (*varOpt).clone();
                    indx = BaseHashTable::get(cr.clone(), ht.clone())?;
                    let Some(__pa0) = (vec.borrow()[(indx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    unit = __pa0.clone();
                    unitStr = (UnitAbsynBuilder::unit2str(unit.clone())?).clone();
                    varOpt = DAEUtil::setUnitAttr(varOpt.clone(), Arc::new(DAE::Exp::SCONST { string: (unitStr.clone()).clone() }))?;
                    Ok(DAEUtil::setVariableAttributes(elt.clone(), varOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oelt)
}

pub fn reportUnitConsistency(mut topScope: bool, mut store: UnitAbsyn::InstStore) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (topScope.clone(), store.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (true, UnitAbsyn::InstStore::INSTSTORE { store: mut st, ht: _, checkResult: Some(UnitAbsyn::UnitCheckResult::CONSISTENT) }) = __mc_input.clone() else { bail!("nomatch") };
            let mut complete: bool = false;
            (complete, _) = UnitChecker::isComplete(st.clone())?;
            Error::addMessage(if (complete.clone()) {Error::CONSISTENT_UNITS.clone()} else {Error::INCOMPLETE_UNITS.clone()}, metamodelica::nil())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn extractConnectorPrefix(mut connectorRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut prefixCon: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    prefixCon = 'mc: {
        let __mc_input = connectorRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, subscriptLst: subs, componentRef: _ } => {
                    Ok(ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), ty.clone(), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: child } => {
                    let mut child = (*child).clone();
                    child = extractConnectorPrefix(child.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), ty.clone(), subs.clone(), child.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(prefixCon)
}

fn updateCrefTypesWithConnectorPrefix(mut cr1: Arc<DAE::ComponentRef>, mut cr2: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = 'mc: {
        let __mc_input = (cr1.clone(), cr2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, subscriptLst: subs }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: name2, identType: _, subscriptLst: _, componentRef: child2 }) => {
                    let true = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), ty.clone(), subs.clone(), child2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: child }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: name2, identType: _, subscriptLst: _, componentRef: child2 }) => {
                    let mut outCref: Arc<DAE::ComponentRef> = outCref.clone();
                    let true = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    outCref = updateCrefTypesWithConnectorPrefix(child.clone(), child2.clone())?;
                    Ok(ComponentReferenceBasics::makeCrefQual((name.clone()).clone(), ty.clone(), subs.clone(), outCref.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ***** FAILURE with ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1.clone())?); __mm_s.push_str(&*literal!(" _and_ ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

fn checkClassEqual(mut c1: Arc<SCode::Element>, mut c2: Arc<SCode::Element>) -> Result<bool> {
    let mut areEqual: bool = false;
    areEqual = 'mc: {
        let __mc_input = (c1.clone(), c2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((Config::acceptMetaModelicaGrammar()? && !(c1.clone() == c2.clone()))) { bail!("guard") }
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE, .. }, _) => {
                    if !((!(c1.clone() == c2.clone()))) { bail!("guard") }
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: r, .. }, _) => {
                    let false = (SCodeUtil::isFunctionRestriction(r.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { initialAlgorithmLst: initialAlgorithmLst1, normalAlgorithmLst: normalAlgorithmLst1, .. }, .. }, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { initialAlgorithmLst: initialAlgorithmLst2, normalAlgorithmLst: normalAlgorithmLst2, .. }, .. }) => {
                    let true = (intEq((normalAlgorithmLst1.clone().len() as i32), (normalAlgorithmLst2.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let true = (intEq((initialAlgorithmLst1.clone().len() as i32), (initialAlgorithmLst2.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: cd1 @ Deref @ SCode::ClassDef::DERIVED { .. }, .. }, Deref @ SCode::Element::CLASS { classDef: cd2 @ Deref @ SCode::ClassDef::DERIVED { .. }, .. }) => {
                    Ok(cd1.clone() == cd2.clone())
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
    Ok(areEqual)
}

pub fn prefixEqualUnlessBasicType(mut pre1: DAE::Prefix, mut pre2: DAE::Prefix, mut cls: Arc<SCode::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (pre1.clone(), pre2.clone(), cls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_ENUMERATION, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_INTEGER, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_REAL, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_STRING, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_BOOLEAN, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_CLOCK, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { name: idn, .. }) => {
                    if !((idn.clone() == literal!("Real") || idn.clone() == literal!("Integer") || idn.clone() == literal!("String") || idn.clone() == literal!("Boolean"))) { bail!("guard") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. }) => {
                    let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (pre1.clone() == pre2.clone()) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn isBuiltInClass(mut className: ArcStr) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(className.clone()) {
        Deref @ "Real" => true,
        Deref @ "Integer" => true,
        Deref @ "String" => true,
        Deref @ "Boolean" => true,
        Deref @ "Clock" => Config::synchronousFeaturesAllowed(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn equalityConstraintOutputDimension(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> i32 {
    let mut outDimension: i32 = 0;
    outDimension = (::match_deref::match_deref! { match &(inElements.clone()) {
        Deref @ metamodelica::List::Nil => {
            0
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::INTEGER { value: dim } }, tail: Deref @ metamodelica::List::Nil }, direction: Absyn::Direction::OUTPUT, .. }, .. }, tail: _ } => {
            dim.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: tail } => {
            let mut dim: i32 = 0;
            dim = equalityConstraintOutputDimension(tail.clone());
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDimension
}

pub fn equalityConstraint(mut inEnv: FCore::Graph, mut inCdefelts: Arc<metamodelica::List<Arc<SCode::Element>>>, mut info: SourceInfo) -> Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> {
    let mut outResult: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut els: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut path: Arc<Absyn::Path>;
    let mut dimension: i32 = 0;
    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(FGraph::getScopePath(inEnv.clone()), '__try0)) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        path = __pa1.clone();
        path = unwrap_break_err!(AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("equalityConstraint")).clone() })), '__try0);
        path = AbsynUtil::makeFullyQualified(path.clone());
        Ok::<_, anyhow::Error>((path.clone(),))
    } {
        Ok((__try0_o0,)) => {
            path = __try0_o0;
        }
        Err(_) => {
            return outResult;
        }
    }
    for mut el in &*inCdefelts.clone() {
        let mut el = el.clone();
        if '__try2: {
            let __pa3 = ::match_deref::match_deref! { match &(el.clone()) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa3, .. }, restriction: SCode::Restriction::R_FUNCTION { .. }, name: Deref @ "equalityConstraint", .. } => __pa3.clone(),
                _ => break '__try2 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            els = __pa3.clone();
            dimension = equalityConstraintOutputDimension(els.clone());
            inlineType = unwrap_break_err!(classIsInlineFunc(el.clone()), '__try2);
            outResult = Some((path.clone(), dimension.clone(), inlineType.clone()));
            return outResult;
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    outResult
}

pub fn handleUnitChecking(mut cache: FCore::Cache, mut env: FCore::Graph, mut inStore: UnitAbsyn::InstStore, mut pre: DAE::Prefix, mut compDAE: DAE::DAElist, mut daes: Arc<metamodelica::List<DAE::DAElist>>, mut className: ArcStr) -> Result<(FCore::Cache, FCore::Graph, UnitAbsyn::InstStore)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    (outCache, outEnv, outStore) = (::match_deref::match_deref! { match &((cache.clone(), env.clone(), inStore.clone(), pre.clone(), compDAE.clone(), daes.clone(), className.clone())) {
        (_, _, store, _, _, _, _) => {
            (cache.clone(), env.clone(), store.clone())
        },
        (_, _, store, _, _, _, _) => {
            let mut daetemp: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
            let mut ut: Arc<metamodelica::List<Arc<UnitAbsyn::UnitTerm>>> = metamodelica::nil();
            let mut store = (*store).clone();
            daetemp = DAEUtil::joinDaeLst(daes.clone())?;
            (store, ut) = UnitAbsynBuilder::instBuildUnitTerms(env.clone(), daetemp.clone(), compDAE.clone(), store.clone())?;
            UnitAbsynBuilder::registerUnitWeights(cache.clone(), env.clone(), compDAE.clone())?;
            store = UnitChecker::check(ut.clone(), store.clone())?;
            (cache.clone(), env.clone(), store.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outStore))
}

fn checkExtendsRestrictionMatch(mut r1: SCode::Restriction, mut r2: SCode::Restriction) -> Result<()> {
    let _ = (match (r1.clone(), r2.clone()) {
        (SCode::Restriction::R_PACKAGE, SCode::Restriction::R_PACKAGE) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: _ } }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }) => (),
        (SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }, SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }) => (),
        (SCode::Restriction::R_TYPE, SCode::Restriction::R_TYPE) => (),
        (SCode::Restriction::R_RECORD { isOperator: _ }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_TYPE) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (SCode::Restriction::R_BLOCK, SCode::Restriction::R_RECORD { isOperator: false }) => (),
        (SCode::Restriction::R_BLOCK, SCode::Restriction::R_BLOCK) => (),
        (SCode::Restriction::R_MODEL, SCode::Restriction::R_RECORD { isOperator: false }) => (),
        (SCode::Restriction::R_MODEL, SCode::Restriction::R_BLOCK) => (),
        (SCode::Restriction::R_MODEL, SCode::Restriction::R_MODEL) => (),
        (SCode::Restriction::R_MODEL, SCode::Restriction::R_CLASS) => (),
        (SCode::Restriction::R_CLASS, SCode::Restriction::R_MODEL) => (),
        (SCode::Restriction::R_CLASS, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CLASS, SCode::Restriction::R_BLOCK) => (),
        (SCode::Restriction::R_CLASS, SCode::Restriction::R_CLASS) => (),
        (SCode::Restriction::R_OPERATOR, SCode::Restriction::R_OPERATOR) => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn checkExtendsForTypeRestiction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inRestriction: SCode::Restriction, mut inSCodeElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inRestriction.clone(), inSCodeElementLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, r, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: id }, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let true = (listMember(r.clone(), list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, SCode::Restriction::R_CONNECTOR { isExpandable: false }, SCode::Restriction::R_CONNECTOR { isExpandable: true }])) else { bail!("pattern mismatch") };
                    let true = (listMember((id.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone()])) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, r, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: id }, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
                    let true = (listMember(r.clone(), list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, SCode::Restriction::R_CONNECTOR { isExpandable: false }, SCode::Restriction::R_CONNECTOR { isExpandable: true }])) else { bail!("pattern mismatch") };
                    let true = (listMember((id.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone(), (literal!("Clock")).clone()])) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    if '__try0: {
                        (_, _, _) = unwrap_break_err!(Lookup::lookupClass(inCache.clone(), inEnv.clone(), p.clone(), None), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, r1, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r2: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(inCache.clone(), inEnv.clone(), p.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { restriction: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    checkExtendsRestrictionMatch(r1.clone(), r2.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, r1, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r2: SCode::Restriction = SCode::Restriction::R_BLOCK;
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(inCache.clone(), inEnv.clone(), p.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { restriction: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error!: ")); __mm_s.push_str(&*SCodeDump::restrString(r1.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv.clone())?); __mm_s.push_str(&*literal!(" cannot be extended by ")); __mm_s.push_str(&*SCodeDump::restrString(r2.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" due to derived/base class restrictions.\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn checkDerivedRestriction(mut parentRestriction: SCode::Restriction, mut childRestriction: SCode::Restriction, mut childName: ArcStr) -> bool {
    let mut b: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    let mut b4: bool = false;
    let mut strLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut rstLst: Arc<metamodelica::List<SCode::Restriction>> = metamodelica::nil();
    strLst = if (Config::synchronousFeaturesAllowed()) {list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("String")).clone(), (literal!("Boolean")).clone(), (literal!("Clock")).clone()]} else {list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("String")).clone(), (literal!("Boolean")).clone()]};
    b1 = listMember((childName.clone()).clone(), strLst.clone());
    rstLst = if (Config::synchronousFeaturesAllowed()) {list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_CLOCK]} else {list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING, openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN]};
    b2 = listMember(childRestriction.clone(), rstLst.clone());
    b3 = parentRestriction.clone() == openmodelica_frontend_types::SCode::Restriction::R_TYPE;
    b4 = parentRestriction.clone() == SCode::Restriction::R_CONNECTOR { isExpandable: false } || parentRestriction.clone() == SCode::Restriction::R_CONNECTOR { isExpandable: true };
    b = boolOr(b1.clone(), boolOr(b2.clone(), boolOr(b3.clone(), boolAnd(boolOr(b1.clone(), b2.clone()), b4.clone()))));
    b
}

pub fn matchModificationToComponents(mut inElems: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inmod: Arc<DAE::Mod>, mut callingScope: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inElems.clone(), inmod.clone(), callingScope.clone())) {
        (_, Deref @ DAE::Mod::NOMOD, _) => {
            ()
        },
        (_, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Nil, .. }, _) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, _, _) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (Mod::prettyPrintMod(inmod.clone(), 0)?).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" not found in <")); __mm_s.push_str(&*callingScope.clone()); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::UNUSED_MODIFIER.clone(), list![(s2.clone()).clone()])?;
            bail!("fail")
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: cn, .. }, tail: elems }, r#mod, _) => {
            let mut r#mod = (*r#mod).clone();
            r#mod = Mod::removeMod(r#mod.clone(), (cn.clone()).clone())?;
            matchModificationToComponents(elems.clone(), r#mod.clone(), (callingScope.clone()).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { .. }, tail: elems }, _, _) => {
            matchModificationToComponents(elems.clone(), inmod.clone(), (callingScope.clone()).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { .. }, name: cn, .. }, tail: elems }, r#mod, _) => {
            let mut r#mod = (*r#mod).clone();
            r#mod = Mod::removeMod(r#mod.clone(), (cn.clone()).clone())?;
            matchModificationToComponents(elems.clone(), r#mod.clone(), (callingScope.clone()).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::IMPORT { .. }, tail: elems }, _, _) => {
            matchModificationToComponents(elems.clone(), inmod.clone(), (callingScope.clone()).clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE, .. }, .. }, tail: elems }, _, _) => {
            matchModificationToComponents(elems.clone(), inmod.clone(), (callingScope.clone()).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn elementNameMember(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut els: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut isNamed: bool = false;
    isNamed = listMember(Util::tuple21(inElement.clone()), els.clone());
    isNamed
}

pub fn extractConstantPlusDepsTpl(mut inComps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut ocr: Option<Arc<DAE::ComponentRef>>, mut allComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr, mut ieql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut iieql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut ialgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut iialgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>) -> Result<(Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::Equation>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>)> {
    let mut oel: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut oeql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut oieql: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut oalgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    let mut oialgs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
    (oel, oeql, oieql, oalgs, oialgs) = 'mc: {
        let __mc_input = (inComps.clone(), ocr.clone(), allComps.clone(), className.clone(), ieql.clone(), iieql.clone(), ialgs.clone(), iialgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok((metamodelica::nil(), ieql.clone(), iieql.clone(), ialgs.clone(), iialgs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _, _, _, _, _) => {
                    Ok((inComps.clone(), ieql.clone(), iieql.clone(), ialgs.clone(), iialgs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(_), _, _, _, _, _, _) => {
                    let mut lst: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut oel: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = oel.clone();
                    lst = List::map(inComps.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    lst = extractConstantPlusDeps2(lst.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), metamodelica::nil())?;
                    let false = (lst.clone().is_empty()) else { bail!("pattern mismatch") };
                    lst = lst.clone().reverse();
                    oel = List::filter1OnTrue(inComps.clone(), (std::sync::Arc::new(fnptr!(elementNameMember, (Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<Arc<SCode::Element>>>)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<bool> + 'static>), lst.clone());
                    Ok((oel.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(cr), _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps failure to find ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(", returning \n")); ArcStr::from(__mm_s) }).clone())?;
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps elements to instantiate:")); __mm_s.push_str(&*intString((inComps.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oel, oeql, oieql, oalgs, oialgs))
}

pub fn extractConstantPlusDeps(mut inComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut ocr: Option<Arc<DAE::ComponentRef>>, mut allComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = (inComps.clone(), ocr.clone(), allComps.clone(), className.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    Ok(inComps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(_), _, _) => {
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    outComps = extractConstantPlusDeps2(inComps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), metamodelica::nil())?;
                    let false = (outComps.clone().is_empty()) else { bail!("pattern mismatch") };
                    outComps = outComps.clone().reverse();
                    Ok(outComps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(cr), _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps failure to find ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(", returning \n")); ArcStr::from(__mm_s) }).clone())?;
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps elements to instantiate:")); __mm_s.push_str(&*intString((inComps.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComps)
}

fn extractConstantPlusDeps2(mut inComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut ocr: Option<Arc<DAE::ComponentRef>>, mut inAllComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr, mut inExisting: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = (inComps.clone(), ocr.clone(), inAllComps.clone(), className.clone(), inExisting.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Some(_), _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _, _) => {
                    Ok(inComps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: selem @ Deref @ SCode::Element::CLASS { name: name2, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, _, existing) => {
                    let mut allComps = (*allComps).clone();
                    let mut existing = (*existing).clone();
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    allComps = cons(selem.clone(), allComps.clone());
                    existing = cons((name2.clone()).clone(), existing.clone());
                    outComps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(cons(selem.clone(), outComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: selem @ Deref @ SCode::Element::COMPONENT { modifications: scmod, name: name2, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }), allComps, _, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut allComps = (*allComps).clone();
                    let mut existing = (*existing).clone();
                    let true = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    crefs = getCrefFromMod(scmod.clone())?;
                    allComps = listAppend(comps.clone(), allComps.clone());
                    existing = cons((name2.clone()).clone(), existing.clone());
                    recDeps = extractConstantPlusDeps3(crefs.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(cons(selem.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: selem @ Deref @ SCode::Element::COMPONENT { name: name2, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }), allComps, _, existing) => {
                    let mut allComps = (*allComps).clone();
                    let false = (stringEq((name.clone()).clone(), (name2.clone()).clone())) else { bail!("pattern mismatch") };
                    allComps = cons(selem.clone(), allComps.clone());
                    Ok(extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compMod @ Deref @ SCode::Element::EXTENDS { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, _, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut allComps = (*allComps).clone();
                    allComps = cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(cons(compMod.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compMod @ Deref @ SCode::Element::IMPORT { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, _, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut allComps = (*allComps).clone();
                    allComps = cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(cons(compMod.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: compMod @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, _, existing) => {
                    let mut recDeps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut allComps = (*allComps).clone();
                    allComps = cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(comps.clone(), ocr.clone(), allComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(cons(compMod.clone(), recDeps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!(" failure in get_Constant_PlusDeps \n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComps)
}

fn extractConstantPlusDeps3(mut inAcrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut remainingComps: Arc<metamodelica::List<Arc<SCode::Element>>>, mut className: ArcStr, mut inExisting: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = (inAcrefs.clone(), remainingComps.clone(), className.clone(), inExisting.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: acr }, tail: acrefs }, _, _, existing) => {
                    Ok(extractConstantPlusDeps3(cons(acr.clone(), acrefs.clone()), remainingComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: s1, subscripts: _, componentRef: acr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: _, subscripts: _ } }, tail: acrefs }, _, _, existing) => {
                    if !((stringEq((className.clone()).clone(), (s1.clone()).clone()))) { bail!("guard") }
                    Ok(extractConstantPlusDeps3(cons(acr.clone(), acrefs.clone()), remainingComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: _, subscripts: _, componentRef: _ }, tail: acrefs }, _, _, existing) => {
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    outComps = extractConstantPlusDeps3(acrefs.clone(), remainingComps.clone(), (className.clone()).clone(), existing.clone())?;
                    Ok(outComps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: s1, subscripts: _ }, tail: acrefs }, _, _, existing) => {
                    if !((List::isMemberOnTrue((s1.clone()).clone(), existing.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)))) { bail!("guard") }
                    Ok(extractConstantPlusDeps3(acrefs.clone(), remainingComps.clone(), (className.clone()).clone(), existing.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: s1, subscripts: _ }, tail: acrefs }, _, _, existing) => {
                    let mut localComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cref_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut existing = (*existing).clone();
                    let mut outComps: Arc<metamodelica::List<Arc<SCode::Element>>> = outComps.clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent((s1.clone()).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    localComps = extractConstantPlusDeps2(remainingComps.clone(), Some(cref_.clone()), metamodelica::nil(), (className.clone()).clone(), existing.clone())?;
                    names = SCodeUtil::componentNamesFromElts(localComps.clone());
                    existing = listAppend(names.clone(), existing.clone());
                    outComps = extractConstantPlusDeps3(acrefs.clone(), remainingComps.clone(), (className.clone()).clone(), existing.clone())?;
                    outComps = listAppend(localComps.clone(), outComps.clone());
                    Ok(outComps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComps)
}

pub fn removeSelfReference(mut className: ArcStr, mut path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = if (stringEq((className.clone()).clone(), (AbsynUtil::pathFirstIdent(path.clone())?).clone())) {AbsynUtil::removePrefix(Arc::new(Absyn::Path::IDENT { name: (className.clone()).clone() }), path.clone())?} else {path.clone()};
    Ok(outPath)
}

pub fn printExtcomps(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inElements.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (el, r#mod), tail: els } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?).clone();
            println!("{}", (s.clone()).clone());
            println!("{}", (literal!(", ")).clone());
            println!("{}", (Mod::printModStr(r#mod.clone())?).clone());
            println!("{}", (literal!("\n")).clone());
            printExtcomps(els.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn constantEls(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    outElements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut el in (elements.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: attr, .. } => SCodeUtil::isConstant(SCodeUtil::attrVariability(attr.clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = el.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outElements)
}

pub fn constantAndParameterEls(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    outElements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut el in (elements.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: attr, .. } => SCodeUtil::isParameterOrConst(SCodeUtil::attrVariability(attr.clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = el.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outElements)
}

fn removeBindings(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outElements = (::match_deref::match_deref! { match &(elements.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name, prefixes, attributes, typeSpec, modifications: _, comment, condition, info }, tail: els } => {
            let mut els1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            els1 = removeBindings(els.clone());
            cons(Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attributes.clone(), typeSpec: typeSpec.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), els1.clone())
        },
        Deref @ metamodelica::List::Cons { head: el, tail: els } => {
            let mut els1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            els1 = removeBindings(els.clone());
            cons(el.clone(), els1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

fn removeExtBindings(mut elements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> {
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    outElements = (::match_deref::match_deref! { match &(elements.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name, prefixes, attributes, typeSpec, modifications: _, comment, condition, info }, _), tail: els } => {
            let mut els1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            els1 = removeExtBindings(els.clone());
            cons((Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attributes.clone(), typeSpec: typeSpec.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)), els1.clone())
        },
        Deref @ metamodelica::List::Cons { head: el, tail: els } => {
            let mut els1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            els1 = removeExtBindings(els.clone());
            cons(el.clone(), els1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getModsForDep(mut inDepCref: Arc<Absyn::ComponentRef>, mut inElems: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<DAE::Mod>> {
    let mut omods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    omods = 'mc: {
        let __mc_input = (inDepCref.clone(), inElems.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { .. }, Deref @ DAE::Mod::NOMOD), tail: elems }) => {
                    Ok(getModsForDep(dep.clone(), elems.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { name: name1, .. }, cmod), tail: _ }) => {
                    let mut name2: ArcStr = arcstr::literal!("");
                    let mut cmod = (*cmod).clone();
                    name2 = (Dump::printComponentRefStr(dep.clone())?).clone();
                    let true = (stringEq((name2.clone()).clone(), (name1.clone()).clone())) else { bail!("pattern mismatch") };
                    cmod = Arc::new(DAE::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(DAE::SubMod { ident: (name2.clone()).clone(), r#mod: cmod.clone() })], binding: None, info: Absyn::dummyInfo.clone() });
                    Ok(cmod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::List::Cons { head: _, tail: elems }) => {
                    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    cmod = getModsForDep(dep.clone(), elems.clone())?;
                    Ok(cmod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omods)
}

fn getOptionArraydim(mut inAbsynArrayDimOption: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    outArrayDim = (::match_deref::match_deref! { match &(inAbsynArrayDimOption.clone()) {
        Some(dim) => {
            dim.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArrayDim
}

pub fn addNomod(mut inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> {
    let mut outElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    outElements = {
        let mut __acc: Arc<metamodelica::List<(Arc<SCode::Element>, ())>> = metamodelica::nil();
        for mut x in (inElements.clone()).into_iter().cloned() {
            let __x = (x.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outElements
}

pub fn sortElementList(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inEnv: FCore::Graph, mut isFunctionScope: bool) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = inElements;
    let mut outE: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut cycleElts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut cycles: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>> = metamodelica::nil();
    let mut g: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>> = metamodelica::nil();
    g = Graph::buildGraph(inElements.clone(), (std::sync::Arc::new(getElementDependencies) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> + 'static>), (inElements.clone(), isFunctionScope.clone()));
    (outE, cycles) = Graph::topologicalSort(g.clone(), (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>))?;
    if !(Config::acceptMetaModelicaGrammar()?) {
        inElements = listAppend(outE.clone(), List::map(cycles.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _))));
    }
    checkCyclicalComponents(cycles.clone(), inEnv.clone())?;
    Ok(inElements)
}

fn printGraph(mut env: FCore::Graph, mut g: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>, mut order: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut cycles: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (env.clone(), g.clone(), order.clone(), cycles.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Graph for env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Graph::printGraph(g.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Element order:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(order.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>)), (literal!("\n\t")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cycles:\n")); __mm_s.push_str(&*Graph::printGraph(cycles.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDepsFromExps(mut inExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inAllElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut isFunction: bool) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    outDependencies = (::match_deref::match_deref! { match &((inExps.clone(), inDependencies.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            inDependencies.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: rest }, deps) => {
            let mut deps = (*deps).clone();
            let (_, (_, _, __pa0, _)) = AbsynUtil::traverseExpBidir(e.clone(), (std::sync::Arc::new(getElementDependenciesTraverserEnter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (std::sync::Arc::new(getElementDependenciesTraverserExit) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (inAllElements.clone(), metamodelica::nil(), deps.clone(), isFunction.clone()))?;
            deps = __pa0.clone();
            deps = getDepsFromExps(rest.clone(), inAllElements.clone(), deps.clone(), isFunction.clone())?;
            deps.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDependencies)
}

fn removeCurrentElementFromArrayDimDeps(mut name: ArcStr, mut inDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    outDependencies = {
        let mut __acc: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
        for mut dep in (inDependencies.clone()).into_iter().cloned() {
            if !(!(stringEq((name.clone()).clone(), (SCodeUtil::elementName(Util::tuple21(dep.clone()))?).clone()))) { continue; }
            let __x = dep.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outDependencies)
}

pub fn getExpsFromConstrainClass(mut inRP: Arc<SCode::Replaceable>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut outBindingExp: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut outSubsExps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    (outBindingExp, outSubsExps) = (::match_deref::match_deref! { match &(inRP.clone()) {
        Deref @ SCode::Replaceable::NOT_REPLACEABLE => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: None } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: m, .. }) } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            (l1, l2) = getExpsFromMod(m.clone())?;
            (l1.clone(), l2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBindingExp, outSubsExps))
}

fn getExpsFromSubMods(mut inSubMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outSubsExps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outSubsExps = (::match_deref::match_deref! { match &(inSubMods.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { r#mod, .. }, tail: rest } => {
            let mut e: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut sm: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            (e, sm) = getExpsFromMod(r#mod.clone())?;
            exps = getExpsFromSubMods(rest.clone())?;
            exps = listAppend(e.clone(), listAppend(sm.clone(), exps.clone()));
            exps.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubsExps)
}

pub fn getCrefFromMod(mut inMod: Arc<SCode::Mod>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outCrefs = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = outCrefs.clone();
                    (l1, l2) = getExpsFromMod(inMod.clone())?;
                    outCrefs = List::flatten(List::map2(listAppend(l1.clone(), l2.clone()), (std::sync::Arc::new(AbsynUtil::getCrefFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool, bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> + 'static>), true, true));
                    Ok(outCrefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.getCrefFromMod")); __mm_s.push_str(&*literal!(": could not retrieve crefs from SCode.Mod: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefs)
}

pub fn getExpsFromMod(mut inMod: Arc<SCode::Mod>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut outBindingExp: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut outSubsExps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    (outBindingExp, outSubsExps) = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::NOMOD => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Mod::MOD { binding: None, subModLst: Deref @ metamodelica::List::Nil, .. } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Mod::MOD { binding: Some(e), subModLst: subs, .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            se = getExpsFromSubMods(subs.clone())?;
            (list![e.clone()], se.clone())
        },
        Deref @ SCode::Mod::MOD { binding: None, subModLst: subs, .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            se = getExpsFromSubMods(subs.clone())?;
            (metamodelica::nil(), se.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: ado }, modifications: m, attributes: _ }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, .. }, .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l3: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l4: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (_, se) = AbsynUtil::getExpsFromArrayDimOpt(ado.clone())?;
            (l3, l4) = getExpsFromMod(m.clone())?;
            l1 = listAppend(se.clone(), listAppend(l1.clone(), l3.clone()));
            l4 = listAppend(l2.clone(), l4.clone());
            (l1.clone(), l4.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: m, .. }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, .. }, .. } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l3: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l4: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (l3, l4) = getExpsFromMod(m.clone())?;
            l3 = listAppend(l1.clone(), l3.clone());
            l4 = listAppend(l2.clone(), l4.clone());
            (l3.clone(), l4.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, .. }, .. } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (l1.clone(), l2.clone())
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: ad, .. }, modifications: m, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, .. }, .. } => {
            let mut se: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l2: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l3: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut l4: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            (l1, l2) = getExpsFromConstrainClass(rp.clone())?;
            (_, se) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
            (l3, l4) = getExpsFromMod(m.clone())?;
            l1 = listAppend(se.clone(), listAppend(l1.clone(), l3.clone()));
            l4 = listAppend(l2.clone(), l4.clone());
            (l1.clone(), l4.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outBindingExp, outSubsExps))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getCrefFromDim(mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outAbsynComponentRefLst = 'mc: {
        let __mc_input = inArrayDim.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: exp }, tail: rest } => {
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    l1 = getCrefFromDim(rest.clone())?;
                    l2 = AbsynUtil::getCrefFromExp(exp.clone(), true, true)?;
                    res = List::union(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB, tail: rest } => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    res = getCrefFromDim(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.getCrefFromDim failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynComponentRefLst)
}

pub fn getElementDependencies(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inAllElementsAndIsFunctionScope: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outDependencies: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    outDependencies = 'mc: {
        let __mc_input = (inElement.clone(), inAllElementsAndIsFunctionScope.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { modifications: r#mod, attributes: SCode::Attributes { variability: var, arrayDims: ad, .. }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, condition: cExpOpt, name, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let true = (SCodeUtil::isParameterOrConst(var.clone())) else { bail!("pattern mismatch") };
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (bexps, sexps) = getExpsFromMod(r#mod.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(Mod::unelabMod(daeMod.clone())?)?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = getDepsFromExps(List::fromOption(cExpOpt.clone()), inAllElements.clone(), deps.clone(), isFunction.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction, .. }, .. }, _), (_, true)) => {
                    let true = (AbsynUtil::isInputOrOutput(direction.clone())?) else { bail!("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { modifications: r#mod, attributes: SCode::Attributes { arrayDims: ad, .. }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, condition: cExpOpt, name, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (bexps, sexps) = getExpsFromMod(r#mod.clone())?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    exps = listAppend(bexps.clone(), exps.clone());
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(Mod::unelabMod(daeMod.clone())?)?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    exps = listAppend(bexps.clone(), exps.clone());
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = getDepsFromExps(List::fromOption(cExpOpt.clone()), inAllElements.clone(), deps.clone(), isFunction.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { attributes: SCode::Attributes { arrayDims: ad, .. }, modifications: r#mod, .. }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, name, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (_, sexps) = getExpsFromMod(r#mod.clone())?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (_, sexps) = getExpsFromMod(Mod::unelabMod(daeMod.clone())?)?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl, .. }, prefixes: Deref @ SCode::Prefixes { .. }, name, .. }, _), (inAllElements, isFunction)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    exps = getExpsFromExternalDecl(externalDecl.clone())?;
                    deps = getDepsFromExps(exps.clone(), inAllElements.clone(), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps((name.clone()).clone(), deps.clone())?;
                    deps = List::union(deps.clone(), metamodelica::nil());
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDependencies)
}

fn getExpsFromExternalDecl(mut inExternalDecl: Option<Arc<SCode::ExternalDecl>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outExps = (::match_deref::match_deref! { match &(inExternalDecl.clone()) {
        None => {
            metamodelica::nil()
        },
        Some(Deref @ SCode::ExternalDecl { args: exps, .. }) => {
            exps.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExps)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getExpsFromDefaults(mut inEls: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inAcc: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outExps = 'mc: {
        let __mc_input = (inEls.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { modifications: m, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, .. }, tail: rest }, _) => {
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut sexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut bexps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    exps = inAcc.clone();
                    (bexps, sexps) = getExpsFromConstrainClass(rp.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(m.clone())?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    exps = getExpsFromDefaults(rest.clone(), exps.clone())?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    exps = getExpsFromDefaults(rest.clone(), inAcc.clone())?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExps)
}

fn getElementDependenciesTraverserEnter(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::CREF { componentRef: cref }, (all_el, stack, accum_el, b)) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut e: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    e = List::find1(all_el.clone(), (std::sync::Arc::new(fnptr!(isElementNamed, (Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr) -> Result<bool> + 'static>), (id.clone()).clone())?;
                    Ok((exp.clone(), (all_el.clone(), stack.clone(), cons(e.clone(), accum_el.clone()), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::CALL { function_: cref, .. }, (all_el, stack, accum_el, b)) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut e: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    id = (AbsynUtil::crefFirstIdent(cref.clone())?).clone();
                    e = List::find1(all_el.clone(), (std::sync::Arc::new(fnptr!(isElementNamed, (Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), ArcStr) -> Result<bool> + 'static>), (id.clone()).clone())?;
                    Ok((exp.clone(), (all_el.clone(), stack.clone(), cons(e.clone(), accum_el.clone()), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::IFEXP { .. }, (all_el, stack, accum_el, false)) => {
                    Ok((exp.clone(), (all_el.clone(), cons(accum_el.clone(), stack.clone()), metamodelica::nil(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

fn getElementDependenciesTraverserExit(mut inExp: Arc<Absyn::Exp>, mut inTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTuple: (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::IFEXP { ifExp, .. }, (all_el, Deref @ metamodelica::List::Cons { head: stack_el, tail: rest_stack }, _, false)) => {
                    let mut deps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let (_, (_, _, __pa0, _)) = AbsynUtil::traverseExpBidir(ifExp.clone(), (std::sync::Arc::new(getElementDependenciesTraverserEnter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (std::sync::Arc::new(getElementDependenciesTraverserExit) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool)) -> Result<(Arc<Absyn::Exp>, (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, bool))> + 'static>), (all_el.clone(), metamodelica::nil(), metamodelica::nil(), false))?;
                    deps = __pa0.clone();
                    Ok((exp.clone(), (all_el.clone(), rest_stack.clone(), listAppend(deps.clone(), stack_el.clone()), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

fn isElementNamed(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inName: ArcStr) -> bool {
    let mut isNamed: bool = false;
    isNamed = (::match_deref::match_deref! { match &(inElement.clone()) {
        (Deref @ SCode::Element::COMPONENT { name, .. }, _) => {
            name.clone() == inName.clone()
        },
        (Deref @ SCode::Element::CLASS { name, .. }, _) => {
            name.clone() == inName.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNamed
}

fn isElementEqual(mut inElement1: (Arc<SCode::Element>, Arc<DAE::Mod>), mut inElement2: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = (::match_deref::match_deref! { match &((inElement1.clone(), inElement2.clone())) {
        ((Deref @ SCode::Element::COMPONENT { name: id1, .. }, _), (Deref @ SCode::Element::COMPONENT { name: id2, .. }, _)) => {
            stringEqual((id1.clone()).clone(), (id2.clone()).clone())
        },
        ((Deref @ SCode::Element::CLASS { name: id1, .. }, _), (Deref @ SCode::Element::CLASS { name: id2, .. }, _)) => {
            stringEqual((id1.clone()).clone(), (id2.clone()).clone())
        },
        _ => {
            stringEq((elementName(inElement1.clone())?).clone(), (elementName(inElement2.clone())?).clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

fn checkCyclicalComponents(mut inCycles: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>>, mut inEnv: FCore::Graph) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inCycles.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut graph: Arc<metamodelica::List<((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)>> = metamodelica::nil();
                    graph = Graph::filterGraph(inCycles.clone(), (std::sync::Arc::new(fnptr!(isElementParamOrConst, (Arc<SCode::Element>, Arc<DAE::Mod>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>));
                    ::match_deref::match_deref! { match &(Graph::findCycles(graph.clone(), (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>))?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cycles: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut cycles_strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cycles_str: ArcStr = arcstr::literal!("");
                    let mut scope_str: ArcStr = arcstr::literal!("");
                    cycles = Graph::findCycles(inCycles.clone(), (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<bool> + 'static>))?;
                    names = List::mapList(cycles.clone(), (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> + 'static>));
                    cycles_strs = List::map1(names.clone(), (std::sync::Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ArcStr>>, ArcStr) -> Result<ArcStr> + 'static>), (literal!(",")).clone());
                    cycles_str = stringDelimitList(cycles_strs.clone(), (literal!("}, {")).clone());
                    cycles_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*cycles_str.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
                    scope_str = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
                    Error::addMessage(Error::CIRCULAR_COMPONENTS.clone(), list![(scope_str.clone()).clone(), (cycles_str.clone()).clone()])?;
                    if !(Flags::isSet(Flags::IGNORE_CYCLES.clone())?) {
                        bail!("fail");
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn isElementParamOrConst(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> bool {
    let mut outIsParamOrConst: bool = false;
    outIsParamOrConst = (::match_deref::match_deref! { match &(inElement.clone()) {
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: var, .. }, .. }, _) => {
            SCodeUtil::isParameterOrConst(var.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsParamOrConst
}

fn elementName(mut inElement: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    let mut elem: Arc<SCode::Element>;
    outName = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outName: ArcStr = outName.clone();
                    let mut elem: Arc<SCode::Element>;
                    (elem, _) = inElement.clone();
                    outName = (SCodeUtil::elementName(elem.clone())?).clone();
                    Ok(outName.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (SCodeDump::shortElementStr(Util::tuple21(inElement.clone()))?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outName)
}

pub fn classdefElts2(mut inElements: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut partialPrefix: SCode::Partial) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)> {
    let mut outClassDefs: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outConstEls: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    (outClassDefs, outConstEls) = 'mc: {
        let __mc_input = (inElements.clone(), partialPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE, .. }, _), tail: xs }, SCode::Partial::PARTIAL) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((cons(cdef.clone(), cdefs.clone()), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cdef @ Deref @ SCode::Element::CLASS { .. }, _), tail: xs }, SCode::Partial::NOT_PARTIAL) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((cons(cdef.clone(), cdefs.clone()), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: el @ (Deref @ SCode::Element::COMPONENT { attributes: attr, .. }, _), tail: xs }, SCode::Partial::NOT_PARTIAL) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let SCode::CONST { .. } = (SCodeUtil::attrVariability(attr.clone())?) else { bail!("pattern mismatch") };
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((cdefs.clone(), cons(el.clone(), els.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, _) => {
                    let mut cdefs: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut els: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    (cdefs, els) = classdefElts2(xs.clone(), partialPrefix.clone())?;
                    Ok((cdefs.clone(), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outClassDefs, outConstEls))
}

pub fn classdefAndImpElts(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>) {
    let mut cdefElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut restElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (cdefElts, restElts) = (::match_deref::match_deref! { match &(elts.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            (_, restElts) = classdefAndImpElts(xs.clone());
            (cons(cdef.clone(), restElts.clone()), restElts.clone())
        },
        Deref @ metamodelica::List::Cons { head: imp @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            (cdefElts, restElts) = classdefAndImpElts(xs.clone());
            (cons(imp.clone(), cdefElts.clone()), restElts.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: xs } => {
            (cdefElts, restElts) = classdefAndImpElts(xs.clone());
            (cdefElts.clone(), cons(e.clone(), restElts.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (cdefElts, restElts)
}

/*
protected function extendsElts
"author: PA
  This function filters out the extends Element in an Element list"
  input list<SCode.Element> inSCodeElementLst;
  output list<SCode.Element> outSCodeElementLst;
algorithm
  outSCodeElementLst := match (inSCodeElementLst)
    local
      list<SCode.Element> res,xs;
      SCode.Element cdef;
    case ({}) then {};
    case (((cdef as SCode.EXTENDS(baseClassPath = _)) :: xs))
      algorithm
        res = extendsElts(xs);
      then
        (cdef :: res);
    case ((_ :: xs))
      algorithm
        res = extendsElts(xs);
      then
        res;
  end match;
end extendsElts;
*/
// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn componentElts(mut inSCodeElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outSCodeElementLst: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outSCodeElementLst = (::match_deref::match_deref! { match &(inSCodeElementLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::COMPONENT { .. }, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            res = componentElts(xs.clone());
            cons(cdef.clone(), res.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            res = componentElts(xs.clone());
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSCodeElementLst
}

pub fn addClassdefsToEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inClasses: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inImpl: bool, mut inRedeclareMod: Option<Arc<DAE::Mod>>, mut checkDuplicates: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = inIH.clone();
    for mut c in &*inClasses.clone() {
        let mut c = c.clone();
        (outCache, outEnv, outIH) = addClassdefToEnv(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), c.clone(), inImpl.clone(), inRedeclareMod.clone(), checkDuplicates.clone())?;
    }
    Ok((outCache, outEnv, outIH))
}

fn addClassdefToEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inSCodeElement: Arc<SCode::Element>, mut inBoolean: bool, mut redeclareMod: Option<Arc<DAE::Mod>>, mut checkDuplicates: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSCodeElement.clone(), redeclareMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, sel1 @ Deref @ SCode::Element::CLASS { .. }, Some(m)) => {
                    let mut env_1: FCore::Graph;
                    let mut cl2: Arc<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut m = (*m).clone();
                    m = Mod::lookupCompModification(m.clone(), (var_field!((**sel1).name, SCode::Element::CLASS).clone()).clone())?;
                    let false = (m.clone() == Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)) else { bail!("pattern mismatch") };
                    env_1 = FGraph::mkClassNode(env.clone(), sel1.clone(), pre.clone(), m.clone(), false)?;
                    (cache, env_1, ih, cl2) = addClassdefsToEnv3(cache.clone(), env_1.clone(), ih.clone(), pre.clone(), redeclareMod.clone(), sel1.clone())?;
                    ih = InnerOuter::addClassIfInner(cl2.clone(), pre.clone(), env_1.clone(), ih.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, sel1 @ Deref @ SCode::Element::CLASS { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    let mut ih = (*ih).clone();
                    env_1 = FGraph::mkClassNode(env.clone(), sel1.clone(), pre.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), checkDuplicates.clone())?;
                    ih = InnerOuter::addClassIfInner(sel1.clone(), pre.clone(), env_1.clone(), ih.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, imp @ Deref @ SCode::Element::IMPORT { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    env_1 = FGraph::mkImportNode(env.clone(), imp.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, elt @ Deref @ SCode::Element::DEFINEUNIT { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    env_1 = FGraph::mkDefunitNode(env.clone(), elt.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.addClassdefToEnv2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH))
}

fn checkCompEnvPathVsCompTypePath(mut inCompEnvPath: Option<Arc<Absyn::Path>>, mut inCompTypePath: Arc<Absyn::Path>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inCompEnvPath.clone(), inCompTypePath.clone())) {
        (_, Deref @ Absyn::Path::IDENT { name: _ }) => {
            ()
        },
        (Some(ep), tp) => {
            let mut tp = (*tp).clone();
            tp = AbsynUtil::stripLast(tp.clone())?;
            let true = (AbsynUtil::pathPrefixOf(tp.clone(), ep.clone())) else { bail!("pattern mismatch") };
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn addComponentsToEnv(mut cache: FCore::Cache, mut env: FCore::Graph, mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut r#mod: Arc<DAE::Mod>, mut prefix: DAE::Prefix, mut state: ClassInf::State, mut components: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut r#impl: bool) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>> = ih;
    let mut comp: Arc<SCode::Element>;
    let mut comp2: Arc<SCode::Element>;
    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut local_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut comp_mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut mod2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut ty_path: Arc<Absyn::Path>;
    let mut prefs: Arc<SCode::Prefixes> = Arc::new(<SCode::Prefixes as ::std::default::Default>::default());
    let mut attr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    let mut dattr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
    let mut error: bool = false;
    let mut err_msg: ArcStr = arcstr::literal!("");
    for mut compmod in &*components.clone() {
        let mut compmod = compmod.clone();
        (comp, cmod) = compmod.clone();
        error = 'mc: {
        let __mc_input = comp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: ty_path, .. }, .. } => {
                    if !((var_field!((*comp).name, SCode::Element::COMPONENT).clone() == AbsynUtil::pathLastIdent(ty_path.clone())?)) { bail!("guard") }
                    let mut err_msg: ArcStr = err_msg.clone();
                    checkCompEnvPathVsCompTypePath(FGraph::getScopePath(env.clone())?, ty_path.clone())?;
                    err_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*comp).name, SCode::Element::COMPONENT).clone()); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::COMPONENT_NAME_SAME_AS_TYPE_NAME.clone(), list![(err_msg.clone()).clone(), (AbsynUtil::pathString(ty_path.clone(), (literal!(".")).clone(), true, false)?).clone()], var_field!((*comp).info, SCode::Element::COMPONENT).clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { attributes: attr @ SCode::Attributes { .. }, prefixes: prefs @ Deref @ SCode::Prefixes { .. }, .. } => {
                    let mut env: FCore::Graph = env.clone();
                    let mut cmod: Arc<DAE::Mod> = cmod.clone();
                    let mut dattr: Arc<DAE::Attributes> = dattr.clone();
                    let mut ty_path: Arc<Absyn::Path>;
                    let mut comp2: Arc<SCode::Element>;
                    let mut mod2: Arc<DAE::Mod> = mod2.clone();
                    let mut ih: Arc<metamodelica::List<InnerOuter::TopInstance>> = ih.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut comp_mod: Arc<DAE::Mod> = comp_mod.clone();
                    let mut local_mod: Arc<DAE::Mod> = local_mod.clone();
                    let mut comp: Arc<SCode::Element> = comp.clone();
                    ty_path = AbsynUtil::typeSpecPath(var_field!((*comp).typeSpec, SCode::Element::COMPONENT).clone())?;
                    local_mod = Mod::lookupModificationP(r#mod.clone(), ty_path.clone())?;
                    if SCodeUtil::finalBool(SCodeUtil::prefixesFinal(prefs.clone())?)? {
                        assign_variant_field!(comp => SCode::Element::COMPONENT; modifications = traverseModAddFinal(var_field!((*comp).modifications, SCode::Element::COMPONENT).clone())?);
                    }
                    (cache, env, ih, comp2, mod2) = Inst::redeclareType(cache.clone(), env.clone(), ih.clone(), local_mod.clone(), comp.clone(), prefix.clone(), state.clone(), r#impl.clone(), cmod.clone())?;
                    comp_mod = Mod::lookupCompModification(r#mod.clone(), (var_field!((*comp).name, SCode::Element::COMPONENT).clone()).clone())?;
                    cmod = Mod::merge(comp_mod.clone(), cmod.clone(), (literal!("")).clone(), true)?;
                    dattr = DAEUtil::translateSCodeAttrToDAEAttr(attr.clone(), prefs.clone())?;
                    env = FGraph::mkComponentNode(env.clone(), Arc::new(DAE::Var { name: (var_field!((*comp).name, SCode::Element::COMPONENT).clone()).clone(), attributes: dattr.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), comp.clone(), cmod.clone(), crate::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { .. } => {
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
        if error.clone() {
            bail!("fail");
        }
    }
    Ok((cache, env, ih))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCrefsFromCompdims(mut inComponents: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outCrefs = 'mc: {
        let __mc_input = inComponents.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: arraydim, .. }, .. }, _), tail: xs } => {
                    let mut crefs1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    crefs1 = getCrefFromDim(arraydim.clone())?;
                    crefs2 = getCrefsFromCompdims(xs.clone())?;
                    crefs = listAppend(crefs1.clone(), crefs2.clone());
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
                    crefs = getCrefsFromCompdims(xs.clone())?;
                    Ok(crefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCrefs)
}

fn memberCrefs(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inComponentRefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>) -> bool {
    let mut outIsMember: bool = false;
    outIsMember = List::isMemberOnTrue(inComponentRef.clone(), inComponentRefs.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqualNoSubs, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>));
    outIsMember
}

pub fn chainRedeclares(mut inModOuter: Arc<DAE::Mod>, mut inModInner: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut b: bool = false;
    outMod = (::match_deref::match_deref! { match &((inModOuter.clone(), inModInner.clone())) {
        (_, _) => {
            (outMod, b) = chainRedeclare_dispatch(inModOuter.clone(), inModInner.clone())?;
            if (b.clone()) {outMod.clone()} else {inModInner.clone()}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

pub fn chainRedeclare_dispatch(mut inModOuter: Arc<DAE::Mod>, mut inModInner: Arc<SCode::Mod>) -> Result<(Arc<SCode::Mod>, bool)> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut change: bool = false;
    (outMod, change) = 'mc: {
        let __mc_input = (inModOuter.clone(), inModInner.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::REDECL { finalPrefix: f, eachPrefix: e, element: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: nDerivedInner }, .. }, .. }, name: nInner, .. } }) => {
                    let mut cls: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::lookupCompModification(inModOuter.clone(), (nDerivedInner.clone()).clone())?) {
                        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls = __pa0.clone();
                    cls = SCodeUtil::setClassName((nInner.clone()).clone(), cls.clone())?;
                    Ok((Arc::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: cls.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::REDECL { finalPrefix: f, eachPrefix: e, element: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: _ }, .. }, .. }, name: nInner, .. } }) => {
                    let mut cls: Arc<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::lookupCompModification(inModOuter.clone(), (nInner.clone()).clone())?) {
                        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cls = __pa0.clone();
                    Ok((Arc::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: cls.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: name, r#mod: m @ Deref @ SCode::Mod::REDECL { .. } }, tail: rest }, binding: b, comment: cmt, info }) => {
                    let mut subs: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    let mut m2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    (m2, _) = chainRedeclare_dispatch(inModOuter.clone(), m.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(chainRedeclare_dispatch(inModOuter.clone(), Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: rest.clone(), binding: b.clone(), comment: cmt.clone(), info: info.clone() }))?) {
                        (Deref @ SCode::Mod::MOD { subModLst: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    Ok((Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: cons(Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: m2.clone() }), subs.clone()), binding: b.clone(), comment: cmt.clone(), info: info.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: Deref @ metamodelica::List::Cons { head: sm, tail: rest }, binding: b, comment: cmt, info }) => {
                    let mut subs: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    let mut change: bool = change.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(chainRedeclare_dispatch(inModOuter.clone(), Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: rest.clone(), binding: b.clone(), comment: cmt.clone(), info: info.clone() }))?) {
                        (Deref @ SCode::Mod::MOD { subModLst: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    subs = __pa0.clone();
                    change = __pa1.clone();
                    Ok((Arc::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: cons(sm.clone(), subs.clone()), binding: b.clone(), comment: cmt.clone(), info: info.clone() }), change.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inModInner.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMod, change))
}

fn addRecordConstructorsToTheCache(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inMod: Arc<DAE::Mod>, mut inPrefix: DAE::Prefix, mut inState: ClassInf::State, mut inDirection: Absyn::Direction, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inState.clone(), inDirection.clone(), inClass.clone(), inInstDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, ClassInf::State::FUNCTION { path, .. }, _, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, name, .. }, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut ih: InstanceHierarchy = metamodelica::nil();
                    println!("{}", (literal!("Depreciated record constructor used: Inst.addRecordConstructorsToTheCache")).clone());
                    let true = (AbsynUtil::isInputOrOutput(inDirection.clone())?) else { bail!("pattern mismatch") };
                    let false = (stringEq((AbsynUtil::pathLastIdent(path.clone())?).clone(), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    (cache, env, ih) = InstFunction::implicitFunctionInstantiation(inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outIH))
}

pub fn checkMultiplyDeclared(mut cache: FCore::Cache, mut env: FCore::Graph, mut r#mod: Arc<DAE::Mod>, mut prefix: DAE::Prefix, mut ciState: ClassInf::State, mut compTuple: (Arc<SCode::Element>, Arc<DAE::Mod>), mut instDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut r#impl: bool) -> Result<bool> {
    let mut alreadyDeclared: bool = false;
    alreadyDeclared = 'mc: {
        let __mc_input = (cache.clone(), env.clone(), r#mod.clone(), prefix.clone(), ciState.clone(), compTuple.clone(), instDims.clone(), r#impl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    ErrorExt::setCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }, _), _, _) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (Deref @ SCode::Element::COMPONENT { .. }, Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }), _, _) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, newComp @ (Deref @ SCode::Element::COMPONENT { name: n, .. }, _), _, _) => {
                    let mut oldElt: Arc<SCode::Element>;
                    let mut oldMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut instStatus: FCore::Status = FCore::Status::CLS_FULL;
                    let mut alreadyDeclared: bool = alreadyDeclared.clone();
                    (_, _, oldElt, oldMod, instStatus, _) = Lookup::lookupIdentLocal(cache.clone(), env.clone(), (n.clone()).clone())?;
                    checkMultipleElementsIdentical(cache.clone(), env.clone(), (oldElt.clone(), oldMod.clone()), newComp.clone())?;
                    alreadyDeclared = instStatusToBool(instStatus.clone())?;
                    ErrorExt::delCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    Ok(alreadyDeclared.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (Deref @ SCode::Element::COMPONENT { name: n, .. }, _), _, _) => {
                    if '__try0: {
                        (_, _, _, _, _, _) = unwrap_break_err!(Lookup::lookupIdentLocal(cache.clone(), env.clone(), (n.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }, _), _, _) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (Deref @ SCode::Element::CLASS { .. }, Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }), _, _) => {
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: n2 }, .. }, tail: _ }, .. }, name: n, .. }, _), _, _) => {
                    let mut n = (*n).clone();
                    n = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$parent")); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
                    let 0 = (System::stringFind((n.clone()).clone(), (n2.clone()).clone())?) else { bail!("pattern mismatch") };
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (newClass @ Deref @ SCode::Element::CLASS { name: n, .. }, _), _, _) => {
                    let mut oldClass: Arc<SCode::Element>;
                    (oldClass, _) = Lookup::lookupClassLocal(env.clone(), (n.clone()).clone())?;
                    checkMultipleClassesEquivalent(oldClass.clone(), newClass.clone())?;
                    ErrorExt::delCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (Deref @ SCode::Element::CLASS { name: n, .. }, _), _, _) => {
                    if '__try0: {
                        (_, _) = unwrap_break_err!(Lookup::lookupClassLocal(env.clone(), (n.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    ErrorExt::rollBack((literal!("checkMultiplyDeclared")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::delCheckpoint((literal!("checkMultiplyDeclared")).clone());
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("-Inst.checkMultiplyDeclared failed\n")).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(alreadyDeclared)
}

fn instStatusToBool(mut instStatus: FCore::Status) -> Result<bool> {
    let mut alreadyDeclared: bool = false;
    alreadyDeclared = (match instStatus.clone() {
        FCore::Status::VAR_DAE => true,
        FCore::Status::VAR_UNTYPED => false,
        FCore::Status::VAR_TYPED => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(alreadyDeclared)
}

fn checkMultipleElementsIdentical(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut oldComponent: (Arc<SCode::Element>, Arc<DAE::Mod>), mut newComponent: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), oldComponent.clone(), newComponent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (oldElt, _), (newElt, _)) => {
                    let true = (SCodeUtil::elementEqual(oldElt.clone(), newElt.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, (Deref @ SCode::Element::COMPONENT { name: n1, prefixes: prefixes1, attributes: attr1, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath1, arrayDim: ad1 }, modifications: smod1, comment: _, condition: cond1, info: _ }, _), (Deref @ SCode::Element::COMPONENT { name: n2, prefixes: prefixes2, attributes: attr2, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath2, arrayDim: ad2 }, modifications: smod2, comment: _, condition: cond2, info: _ }, _)) => {
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut c1: Arc<SCode::Element>;
                    let mut c2: Arc<SCode::Element>;
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::prefixesEqual(prefixes1.clone(), prefixes2.clone())?) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::attributesEqual(attr1.clone(), attr2.clone())?) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::modEqual(smod1.clone(), smod2.clone())?) else { bail!("pattern mismatch") };
                    let true = (ad1.clone() == ad2.clone()) else { bail!("pattern mismatch") };
                    let true = (cond1.clone() == cond2.clone()) else { bail!("pattern mismatch") };
                    (_, c1, env1) = Lookup::lookupClass(cache.clone(), env.clone(), tpath1.clone(), None)?;
                    (_, c2, env2) = Lookup::lookupClass(cache.clone(), env.clone(), tpath2.clone(), None)?;
                    let true = (stringEq((FGraph::printGraphPathStr(env1.clone())?).clone(), (FGraph::printGraphPathStr(env2.clone())?).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::elementEqual(c1.clone(), c2.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, (oldElt @ Deref @ SCode::Element::COMPONENT { name: n1, prefixes: prefixes1, attributes: attr1, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath1, arrayDim: ad1 }, modifications: smod1, comment: _, condition: cond1, info: old_info }, _), (newElt @ Deref @ SCode::Element::COMPONENT { name: n2, prefixes: prefixes2, attributes: attr2, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath2, arrayDim: ad2 }, modifications: smod2, comment: _, condition: cond2, info: new_info }, _)) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut c1: Arc<SCode::Element>;
                    let mut c2: Arc<SCode::Element>;
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (stringEq((n1.clone()).clone(), (literal!("m_flow")).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::prefixesEqual(prefixes1.clone(), prefixes2.clone())?) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::attributesEqual(attr1.clone(), attr2.clone())?) else { bail!("pattern mismatch") };
                    let false = (SCodeUtil::modEqual(smod1.clone(), smod2.clone())?) else { bail!("pattern mismatch") };
                    let true = (ad1.clone() == ad2.clone()) else { bail!("pattern mismatch") };
                    let true = (cond1.clone() == cond2.clone()) else { bail!("pattern mismatch") };
                    (_, c1, env1) = Lookup::lookupClass(cache.clone(), env.clone(), tpath1.clone(), None)?;
                    (_, c2, env2) = Lookup::lookupClass(cache.clone(), env.clone(), tpath2.clone(), None)?;
                    let true = (stringEq((FGraph::printGraphPathStr(env1.clone())?).clone(), (FGraph::printGraphPathStr(env2.clone())?).clone())) else { bail!("pattern mismatch") };
                    let true = (SCodeUtil::elementEqual(c1.clone(), c2.clone())?) else { bail!("pattern mismatch") };
                    s1 = (SCodeDump::unparseElementStr(oldElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (SCodeDump::unparseElementStr(newElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inherited elements are not identical: bug: https://trac.modelica.org/Modelica/ticket/627\n\tfirst:  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\n\tsecond: ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\nContinue ....")); ArcStr::from(__mm_s) }).clone();
                    Error::addMultiSourceMessage(Error::COMPILER_WARNING.clone(), list![(s.clone()).clone()], list![old_info.clone(), new_info.clone()])?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (oldElt @ Deref @ SCode::Element::COMPONENT { info: old_info, .. }, _), (newElt @ Deref @ SCode::Element::COMPONENT { info: new_info, .. }, _)) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (SCodeDump::unparseElementStr(oldElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (SCodeDump::unparseElementStr(newElt.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Error::addMultiSourceMessage(Error::DUPLICATE_ELEMENTS_NOT_IDENTICAL.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], list![old_info.clone(), new_info.clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn checkMultipleClassesEquivalent(mut oldClass: Arc<SCode::Element>, mut newClass: Arc<SCode::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (oldClass.clone(), newClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, .. }, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst, .. }, restriction: SCode::Restriction::R_ENUMERATION, .. }) => {
                    let mut sl1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut sl2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    sl1 = List::map(enumLst.clone(), (std::sync::Arc::new(SCodeUtil::enumName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Enum>) -> Result<ArcStr> + 'static>));
                    sl2 = List::map(elementLst.clone(), (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
                    let true = (List::isEqualOnTrue(sl1.clone(), sl2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst, .. }, restriction: SCode::Restriction::R_ENUMERATION, .. }, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, .. }) => {
                    let mut sl1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut sl2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    sl1 = List::map(enumLst.clone(), (std::sync::Arc::new(SCodeUtil::enumName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Enum>) -> Result<ArcStr> + 'static>));
                    sl2 = List::map(elementLst.clone(), (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
                    let true = (List::isEqualOnTrue(sl1.clone(), sl2.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oldCl, newCl) => {
                    let true = (SCodeUtil::elementEqual(oldCl.clone(), newCl.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oldCl, newCl) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut info1: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut info2: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    s1 = (SCodeDump::unparseElementStr(oldCl.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    s2 = (SCodeDump::unparseElementStr(newCl.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    info1 = SCodeUtil::elementInfo(oldCl.clone());
                    info2 = SCodeUtil::elementInfo(newCl.clone());
                    Error::addMultiSourceMessage(Error::DUPLICATE_CLASSES_NOT_EQUIVALENT.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], list![info1.clone(), info2.clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn removeOptCrefFromCrefs(mut inCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut inCref: Option<Arc<Absyn::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outCrefs = (::match_deref::match_deref! { match &((inCrefs.clone(), inCref.clone())) {
        (_, Some(cref)) => {
            removeCrefFromCrefs(inCrefs.clone(), cref.clone())?
        },
        _ => {
            inCrefs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn removeCrefFromCrefs(mut inAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut inComponentRef: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    outAbsynComponentRefLst = (::match_deref::match_deref! { match &((inAbsynComponentRefLst.clone(), inComponentRef.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: n1 }, tail: rest }, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: n2 }) if (stringEq((n1.clone()).clone(), (n2.clone()).clone())) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            rest_1 = removeCrefFromCrefs(rest.clone(), cr2.clone())?;
            rest_1.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: n1, .. }, tail: rest }, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n2, .. }) if (stringEq((n1.clone()).clone(), (n2.clone()).clone())) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            rest_1 = removeCrefFromCrefs(rest.clone(), cr2.clone())?;
            rest_1.clone()
        },
        (Deref @ metamodelica::List::Cons { head: cr1, tail: rest }, cr2) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            rest_1 = removeCrefFromCrefs(rest.clone(), cr2.clone())?;
            cons(cr1.clone(), rest_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAbsynComponentRefLst)
}

pub fn keepConstrainingTypeModifersOnly(mut inMod: Arc<DAE::Mod>, mut elems: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<DAE::Mod>> {
    let mut filteredMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    filteredMod = (::match_deref::match_deref! { match &((inMod.clone(), elems.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            inMod.clone()
        },
        (Deref @ DAE::Mod::NOMOD, _) => {
            Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)
        },
        (Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }, _) => {
            inMod.clone()
        },
        (Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: subs, binding: oe, info }, _) => {
            let mut compNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut subs = (*subs).clone();
            compNames = List::map(elems.clone(), (std::sync::Arc::new(SCodeUtil::elementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
            subs = keepConstrainingTypeModifersOnly2(subs.clone(), compNames.clone())?;
            Arc::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: subs.clone(), binding: oe.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(filteredMod)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn keepConstrainingTypeModifersOnly2(mut isubs: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut elems: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::SubMod>>>> {
    let mut osubs: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    osubs = (::match_deref::match_deref! { match &((isubs.clone(), elems.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (subs, Deref @ metamodelica::List::Nil) => {
            subs.clone()
        },
        (Deref @ metamodelica::List::Cons { head: sub @ Deref @ DAE::SubMod { ident: n, .. }, tail: subs }, _) if (List::isMemberOnTrue((n.clone()).clone(), elems.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))) => {
            cons(sub.clone(), keepConstrainingTypeModifersOnly2(subs.clone(), elems.clone())?)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: subs }, _) => {
            keepConstrainingTypeModifersOnly2(subs.clone(), elems.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osubs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn extractConstrainingComps(mut cc: Option<Arc<SCode::ConstrainClass>>, mut env: FCore::Graph, mut pre: DAE::Prefix) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut elems: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    elems = 'mc: {
        let __mc_input = (cc.clone(), env.clone(), pre.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ SCode::ConstrainClass { constrainingClass: path, .. }), _, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut selems: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extendselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut compelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extcompelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut classextendselts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut classes: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut extcomps: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClass(FCore::emptyCache(), env.clone(), path.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa0, .. }, name: __pa1, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    selems = __pa0.clone();
                    name = __pa1.clone();
                    (classes, classextendselts, extendselts, compelts) = splitElts(selems.clone())?;
                    (_, _, _, _, extcomps, _, _, _, _, _) = InstExtends::instExtendsAndClassExtendsList(FCore::emptyCache(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), pre.clone(), extendselts.clone(), classextendselts.clone(), selems.clone(), ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, (name.clone()).clone(), true, false)?;
                    extcompelts = List::map(extcomps.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    compelts = listAppend(classes.clone(), listAppend(compelts.clone(), extcompelts.clone()));
                    Ok(compelts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ SCode::ConstrainClass { constrainingClass: path, modifier: r#mod, comment: cmt }), _, _) => {
                    let mut compelts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut path = (*path).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(FCore::emptyCache(), env.clone(), path.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa0, .. }, .. }, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    compelts = extractConstrainingComps(Some(Arc::new(SCode::ConstrainClass { constrainingClass: path.clone(), modifier: r#mod.clone(), comment: cmt.clone() })), env.clone(), pre.clone())?;
                    Ok(compelts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(elems)
}

pub fn moveBindings(mut inEquations: DAE::DAElist, mut inVariables: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outVariables: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    if Config::getGraphicsExpMode()? {
        outVariables = inVariables.clone();
        return Ok(outVariables);
    }
    let DAE::DAE { elementLst: __pa0 } = (inEquations.clone()) else { bail!("pattern mismatch") };
    eqs = __pa0.clone();
    let DAE::DAE { elementLst: __pa1 } = (inVariables.clone()) else { bail!("pattern mismatch") };
    vars = __pa1.clone();
    Error::assertion(intEq((eqs.clone().len() as i32), (vars.clone().len() as i32)), (literal!("- InstUtil.moveBindings: Mismatched number of equations and variables.")).clone(), Absyn::dummyInfo.clone())?;
    vars = List::threadMap(eqs.clone(), vars.clone(), (std::sync::Arc::new(moveBindings2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<Arc<DAE::Element>> + 'static>));
    outVariables = DAE::DAElist { elementLst: vars.clone() };
    Ok(outVariables)
}

fn moveBindings2(mut inEquation: Arc<DAE::Element>, mut inVariable: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outVariable: Arc<DAE::Element>;
    outVariable = 'mc: {
        let __mc_input = (inEquation.clone(), inVariable.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Element::VAR { componentRef: cref, kind, direction: dir, parallelism: prl, protection: vis, ty, binding: _, dims, connectorType: ct, source: src, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: e }) => {
                    let mut bind_exp: Arc<DAE::Exp>;
                    bind_exp = moveBindings3(inEquation.clone())?;
                    Ok(Arc::new(DAE::Element::VAR { componentRef: cref.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: vis.clone(), ty: ty.clone(), binding: Some(bind_exp.clone()), dims: dims.clone(), connectorType: ct.clone(), source: src.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Element::VAR { componentRef: cref, .. }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.moveBindings failed on ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVariable)
}

fn moveBindings3(mut inEquation: Arc<DAE::Element>) -> Result<Arc<DAE::Exp>> {
    let mut outBinding: Arc<DAE::Exp>;
    outBinding = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ DAE::Element::EQUATION { scalar: bind_exp, .. } => {
            bind_exp.clone()
        },
        Deref @ DAE::Element::DEFINE { exp: bind_exp, .. } => {
            bind_exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinding)
}

pub fn checkModificationOnOuter(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inName: ArcStr, mut inCref: Arc<DAE::ComponentRef>, mut inMod: Arc<DAE::Mod>, mut inVariability: SCode::Variability, mut inInnerOuter: Absyn::InnerOuter, mut inImpl: bool, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inName.clone(), inCref.clone(), inMod.clone(), inVariability.clone(), inInnerOuter.clone(), inImpl.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, SCode::Variability::CONST, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, SCode::Variability::PARAM, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: InnerOuter::TopInstance { sm, .. }, tail: _ }, _, _, _, Deref @ DAE::Mod::MOD { .. }, _, Absyn::InnerOuter::OUTER, _, _) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), sm.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (InnerOuter::modificationOnOuter(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), (inName.clone()).clone(), inCref.clone(), inMod.clone(), inInnerOuter.clone(), inImpl.clone(), inInfo.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn checkFunctionVar(mut inName: ArcStr, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inInfo: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inName.clone(), inAttributes.clone(), inPrefixes.clone(), inInfo.clone())) {
        (_, SCode::Attributes { direction: Absyn::Direction::BIDIR, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC, .. }, _) => {
            Error::addSourceMessage(Error::NON_FORMAL_PUBLIC_FUNCTION_VAR.clone(), list![(inName.clone()).clone()], inInfo.clone())?;
            ()
        },
        (_, SCode::Attributes { direction: Absyn::Direction::BIDIR, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED, .. }, _) => (),
        (_, SCode::Attributes { .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED, .. }, _) => {
            Error::addSourceMessage(Error::PROTECTED_FORMAL_FUNCTION_VAR.clone(), list![(inName.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkFunctionVarType(mut inType: Arc<DAE::Type>, mut inState: ClassInf::State, mut inVarName: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inType.clone(), inState.clone(), inVarName.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let true = (Types::isValidFunctionVarType(inType.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty_str: ArcStr = arcstr::literal!("");
                    ty_str = (TypesDump::getTypeName(inType.clone())?).clone();
                    Error::addSourceMessage(Error::INVALID_FUNCTION_VAR_TYPE.clone(), list![(ty_str.clone()).clone(), (inVarName.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn liftNonBasicTypes(mut tp: Arc<DAE::Type>, mut dimt: Arc<DAE::Dimension>) -> Result<Arc<DAE::Type>> {
    let mut outTp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outTp = 'mc: {
        let __mc_input = (tp.clone(), dimt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. }, _) => {
                    let false = (TypesDump::getDimensions(ty.clone()).is_empty()) else { bail!("pattern mismatch") };
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Types::liftArray(tp.clone(), dimt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTp)
}

pub fn checkHigherVariability(mut compConst: DAE::Const, mut bindConst: DAE::Const, mut pre: DAE::Prefix, mut name: ArcStr, mut binding: Arc<DAE::Exp>, mut info: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (compConst.clone(), bindConst.clone(), pre.clone(), name.clone(), binding.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c, c1, _, _, _, _) => {
                    if !((c.clone() == c1.clone())) { bail!("guard") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Const::C_PARAM, DAE::Const::C_UNKNOWN, _, _, _, _) => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c, c1, _, n, e, _) => {
                    let mut sc: ArcStr = arcstr::literal!("");
                    let mut sc1: ArcStr = arcstr::literal!("");
                    let mut se: ArcStr = arcstr::literal!("");
                    let mut sn: ArcStr = arcstr::literal!("");
                    sn = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStr2(pre.clone())?); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
                    sc = (DAEUtil::constStr(c.clone())?).clone();
                    sc1 = (DAEUtil::constStr(c1.clone())?).clone();
                    se = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    Error::addSourceMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(sn.clone()).clone(), (sc.clone()).clone(), (se.clone()).clone(), (sc1.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn makeArrayType(mut inDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inDimensionLst.clone(), inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, ty) => {
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: dim, tail: xs }, tty) => {
                    let mut ty_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty_1 = makeArrayType(xs.clone(), tty.clone())?;
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: ty_1.clone(), dims: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.makeArrayType failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn getUsertypeDimensions(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inClass: Arc<SCode::Element>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, mut inBoolean: bool) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>, Arc<SCode::Element>, Arc<DAE::Mod>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut classToInstantiate: Arc<SCode::Element>;
    let mut outMods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    (outCache, outDimensionLst, classToInstantiate, outMods) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone(), inBoolean.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { name: id, .. }, _, _) => {
                    if !((id.clone() == literal!("Real") || id.clone() == literal!("Integer") || id.clone() == literal!("String") || id.clone() == literal!("Boolean"))) { bail!("guard") }
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. }, _, _) => {
                    let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { .. }, restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, pre, cl @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: ad, path: Deref @ Absyn::Path::IDENT { .. }, .. }, .. }, info, name: id, .. }, dims, r#impl) => {
                    let mut owncref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut ad_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    owncref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() });
                    ad_1 = getOptionArraydim(ad.clone());
                    (cache, dim1) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Integer")).clone() }), ad_1.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), dims.clone())?;
                    Ok((cache.clone(), dim1.clone(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::PARTIAL, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::NOT_PARTIAL, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, info, name: id, .. }, _, _) => {
                    Error::addSourceMessage(Error::META_FUNCTION_TYPE_NO_PARTIAL_PREFIX.clone(), list![(id.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_UNIONTYPE { .. }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: ad, path: cn }, .. }, info, restriction: SCode::Restriction::R_TYPE, name: id, .. }, dims, r#impl) => {
                    let mut cl: Arc<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut owncref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut ad_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut type_mods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut eq: Option<DAE::EqMod> = None;
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    (cache, cl, cenv) = Lookup::lookupClass(cache.clone(), env.clone(), cn.clone(), Some(info.clone()))?;
                    owncref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() });
                    ad_1 = getOptionArraydim(ad.clone());
                    env = addEnumerationLiteralsToEnv(env.clone(), cl.clone())?;
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    eq = Mod::modEquation(mod_1.clone())?;
                    (cache, dim1, cl, type_mods) = getUsertypeDimensions(cache.clone(), cenv.clone(), ih.clone(), pre.clone(), cl.clone(), dims.clone(), r#impl.clone())?;
                    (cache, dim2) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), cn.clone(), ad_1.clone(), eq.clone(), r#impl.clone(), true, false, pre.clone(), info.clone(), dims.clone())?;
                    type_mods = Mod::addEachIfNeeded(type_mods.clone(), dim2.clone())?;
                    type_mods = Mod::merge(mod_1.clone(), type_mods.clone(), (literal!("")).clone(), true)?;
                    res = listAppend(dim2.clone(), dim1.clone());
                    Ok((cache.clone(), res.clone(), cl.clone(), type_mods.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { initialAlgorithmLst: Deref @ metamodelica::List::Nil, normalAlgorithmLst: Deref @ metamodelica::List::Nil, initialEquationLst: Deref @ metamodelica::List::Nil, normalEquationLst: Deref @ metamodelica::List::Nil, elementLst: els, .. }, .. }, _, r#impl) => {
                    let mut cl: Arc<SCode::Element>;
                    let mut mod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut type_mods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut res: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut path: Arc<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(splitElts(els.clone())?) {
                        (_, _, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, visibility: _, modifications: __pa1, ann: _, info: __pa2 }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    r#mod = __pa1.clone();
                    info = __pa2.clone();
                    (cache, mod_1) = Mod::elabModForBasicType(cache.clone(), env.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::EXTENDS { path: path.clone() }, info.clone())?;
                    (cache, cl, _) = Lookup::lookupClass(cache.clone(), env.clone(), path.clone(), None)?;
                    (cache, res, cl, type_mods) = getUsertypeDimensions(cache.clone(), env.clone(), ih.clone(), pre.clone(), cl.clone(), metamodelica::nil(), r#impl.clone())?;
                    type_mods = Mod::merge(mod_1.clone(), type_mods.clone(), (literal!("")).clone(), true)?;
                    Ok((cache.clone(), res.clone(), cl.clone(), type_mods.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    let mut id: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    id = (SCodeDump::unparseElementStr(inClass.clone(), SCodeDump::defaultOptions.clone())?).clone();
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.getUsertypeDimensions failed: ")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimensionLst, classToInstantiate, outMods))
}

fn addEnumerationLiteralsToEnv(mut inEnv: FCore::Graph, mut inClass: Arc<SCode::Element>) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = (inEnv.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: enums, .. }, restriction: SCode::Restriction::R_ENUMERATION, .. }) => {
                    let mut env: FCore::Graph;
                    env = List::fold(enums.clone(), (std::sync::Arc::new(addEnumerationLiteralToEnv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, FCore::Graph) -> Result<FCore::Graph> + 'static>), inEnv.clone());
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inEnv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

fn addEnumerationLiteralToEnv(mut inEnum: Arc<SCode::Element>, mut inEnv: FCore::Graph) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = (inEnum.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { name: lit, .. }, _) => {
                    let mut env: FCore::Graph;
                    env = FGraph::mkComponentNode(inEnv.clone(), Arc::new(DAE::Var { name: (lit.clone()).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), inEnum.clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
                    Ok(env.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("InstUtil.addEnumerationLiteralToEnv: Unknown enumeration type!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEnv)
}

pub fn updateClassInfState(mut inCache: FCore::Cache, mut inNewEnv: FCore::Graph, mut inOldEnv: FCore::Graph, mut inCIState: ClassInf::State) -> Result<ClassInf::State> {
    let mut outCIState: ClassInf::State;
    outCIState = 'mc: {
        let __mc_input = (inCache.clone(), inNewEnv.clone(), inOldEnv.clone(), inCIState.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, mut ci_state) = __mc_input.clone() else { bail!("nomatch") };
            let true = (FGraph::isTopScope(inNewEnv.clone())?) else { bail!("pattern mismatch") };
            Ok(ci_state.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, mut ci_state) = __mc_input.clone() else { bail!("nomatch") };
            let true = (stringEq((FGraph::getGraphNameStr(inNewEnv.clone())?).clone(), (FGraph::getGraphNameStr(inOldEnv.clone())?).clone())) else { bail!("pattern mismatch") };
            Ok(ci_state.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut ci_state: ClassInf::State;
            let mut rest: FCore::Graph;
            let mut id: ArcStr = arcstr::literal!("");
            let mut cls: Arc<SCode::Element>;
            let false = (FGraph::isTopScope(inNewEnv.clone())?) else { bail!("pattern mismatch") };
            id = (FNode::refName(FGraph::lastScopeRef(inNewEnv.clone())?)?).clone();
            (rest, _) = FGraph::stripLastScopeRef(inNewEnv.clone())?;
            (_, cls, _) = Lookup::lookupClassIdent(inCache.clone(), rest.clone(), (id.clone()).clone(), None)?;
            ci_state = ClassInfUtil::start(SCodeUtil::getClassRestriction(cls.clone())?, FGraph::getGraphName(inNewEnv.clone())?)?;
            Ok(ci_state.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inCIState.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCIState)
}

pub fn evalEnumAndBoolDim(mut inDimension: Arc<DAE::Dimension>) -> Arc<DAE::Dimension> {
    let mut outDimension: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    outDimension = (::match_deref::match_deref! { match &(inDimension.clone()) {
        Deref @ DAE::Dimension::DIM_BOOLEAN => {
            Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 })
        },
        _ => {
            inDimension.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDimension
}

/*TODO: mahge: Remove me*/
pub fn instDimExpNonSplit(mut inDimension: Arc<DAE::Dimension>, mut inBoolean: bool) -> Result<Arc<DAE::Subscript>> {
    let mut outSubscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    outSubscript = (::match_deref::match_deref! { match &((inDimension.clone(), inBoolean.clone())) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN, _) => {
            Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM)
        },
        (Deref @ DAE::Dimension::DIM_INTEGER { integer: i }, _) => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })
        },
        (Deref @ DAE::Dimension::DIM_ENUM { size: i, .. }, _) => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })
        },
        (Deref @ DAE::Dimension::DIM_BOOLEAN, _) => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: Arc::new(DAE::Exp::ICONST { integer: 2 }) })
        },
        (Deref @ DAE::Dimension::DIM_EXP { exp: e }, _) => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: e.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscript)
}

pub fn instWholeDimFromMod(mut dimensionExp: Arc<DAE::Dimension>, mut modifier: Arc<DAE::Mod>, mut inVarName: ArcStr, mut inInfo: SourceInfo) -> Result<Arc<DAE::Dimension>> {
    let mut outDimension: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    outDimension = 'mc: {
        let __mc_input = (dimensionExp.clone(), modifier.clone(), inVarName.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, .. }), .. }, _, _) => {
                    let mut d: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::expDimensions(exp.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    d = __pa0.clone();
                    Ok(d.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, .. }), .. }, _, _) => {
                    let mut exp_str: ArcStr = arcstr::literal!("");
                    exp_str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    Error::addSourceMessage(Error::FAILURE_TO_DEDUCE_DIMS_FROM_MOD.clone(), list![(inVarName.clone()).clone(), (exp_str.clone()).clone()], inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.instWholeDimFromMod failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDimension)
}

pub fn propagateAttributes(mut inDae: DAE::DAElist, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inInfo: SourceInfo) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let DAE::DAE { elementLst: __pa0 } = (inDae.clone()) else { bail!("pattern mismatch") };
    elts = __pa0.clone();
    elts = List::map3(elts.clone(), (std::sync::Arc::new(propagateAllAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, SCode::Attributes, Arc<SCode::Prefixes>, SourceInfo) -> Result<Arc<DAE::Element>> + 'static>), inAttributes.clone(), inPrefixes.clone(), inInfo.clone());
    outDae = DAE::DAElist { elementLst: elts.clone() };
    Ok(outDae)
}

fn propagateAllAttributes(mut inElement: Arc<DAE::Element>, mut inAttributes: SCode::Attributes, mut inPrefixes: Arc<SCode::Prefixes>, mut inInfo: SourceInfo) -> Result<Arc<DAE::Element>> {
    let mut outElement: Arc<DAE::Element>;
    outElement = (::match_deref::match_deref! { match &((inElement.clone(), inAttributes.clone(), inPrefixes.clone(), inInfo.clone())) {
        (_, SCode::Attributes { direction: Absyn::Direction::BIDIR, variability: SCode::Variability::VAR, parallelism: SCode::Parallelism::NON_PARALLEL, connectorType: SCode::ConnectorType::POTENTIAL, .. }, Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER, finalPrefix: SCode::Final::NOT_FINAL, visibility: SCode::Visibility::PUBLIC, .. }, _) => {
            inElement.clone()
        },
        (Deref @ DAE::Element::VAR { encrypted: e, innerOuter: io2, comment: cmt, variableAttributesOption: var_attrs, source, connectorType: ct2, dims, binding, ty, protection: vvis, parallelism: vprl, direction: vdir, kind: vk, componentRef: cr }, SCode::Attributes { direction: dir, variability: var, parallelism: sprl, connectorType: ct1, .. }, Deref @ SCode::Prefixes { innerOuter: io1, finalPrefix: fp, visibility: vis, .. }, _) => {
            let mut io2 = (*io2).clone();
            let mut var_attrs = (*var_attrs).clone();
            let mut ct2 = (*ct2).clone();
            let mut vvis = (*vvis).clone();
            let mut vprl = (*vprl).clone();
            let mut vdir = (*vdir).clone();
            let mut vk = (*vk).clone();
            vdir = propagateDirection(vdir.clone(), dir.clone(), cr.clone(), inInfo.clone())?;
            vk = propagateVariability(vk.clone(), var.clone());
            vprl = propagateParallelism(vprl.clone(), sprl.clone(), cr.clone(), inInfo.clone())?;
            vvis = propagateVisibility(vvis.clone(), vis.clone());
            var_attrs = propagateFinal(var_attrs.clone(), fp.clone())?;
            io2 = propagateInnerOuter(io2.clone(), io1.clone());
            ct2 = propagateConnectorType(ct2.clone(), ct1.clone(), cr.clone(), inInfo.clone())?;
            Arc::new(DAE::Element::VAR { componentRef: cr.clone(), kind: vk.clone(), direction: vdir.clone(), parallelism: vprl.clone(), protection: vvis.clone(), ty: ty.clone(), binding: binding.clone(), dims: dims.clone(), connectorType: ct2.clone(), source: source.clone(), variableAttributesOption: var_attrs.clone(), comment: cmt.clone(), innerOuter: io2.clone(), encrypted: e.clone() })
        },
        (Deref @ DAE::Element::COMP { comment: cmt, source, dAElist: el, ident }, _, _, _) => {
            let mut el = (*el).clone();
            el = List::map3(el.clone(), (std::sync::Arc::new(propagateAllAttributes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, SCode::Attributes, Arc<SCode::Prefixes>, SourceInfo) -> Result<Arc<DAE::Element>> + 'static>), inAttributes.clone(), inPrefixes.clone(), inInfo.clone());
            Arc::new(DAE::Element::COMP { ident: (ident.clone()).clone(), dAElist: el.clone(), source: source.clone(), comment: cmt.clone() })
        },
        _ => {
            inElement.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn propagateDirection(mut inVarDirection: DAE::VarDirection, mut inDirection: Absyn::Direction, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<DAE::VarDirection> {
    let mut outVarDirection: DAE::VarDirection = DAE::VarDirection::BIDIR;
    outVarDirection = (::match_deref::match_deref! { match &((inVarDirection.clone(), inDirection.clone(), inCref.clone(), inInfo.clone())) {
        (_, Absyn::Direction::BIDIR, _, _) => {
            inVarDirection.clone()
        },
        (DAE::VarDirection::BIDIR, _, _, _) => {
            absynDirToDaeDir(inDirection.clone())?
        },
        _ => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            s1 = (Dump::directionSymbol(inDirection.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
            s3 = (DAEDump::dumpDirectionStr(inVarDirection.clone())?).clone();
            Error::addSourceMessage(Error::COMPONENT_INPUT_OUTPUT_MISMATCH.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarDirection)
}

fn propagateParallelism(mut inVarParallelism: DAE::VarParallelism, mut inParallelism: SCode::Parallelism, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<DAE::VarParallelism> {
    let mut outVarParallelism: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
    outVarParallelism = 'mc: {
        let __mc_input = (inVarParallelism.clone(), inParallelism.clone(), inCref.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Parallelism::NON_PARALLEL, _, _) => {
                    Ok(inVarParallelism.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::VarParallelism::NON_PARALLEL, _, _, _) => {
                    Ok(DAEUtil::scodePrlToDaePrl(inParallelism.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (daeprl1, _, _, _) => {
                    let mut daeprl2: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
                    daeprl2 = DAEUtil::scodePrlToDaePrl(inParallelism.clone())?;
                    let true = (DAEUtil::daeParallelismEqual(daeprl1.clone(), daeprl2.clone())) else { bail!("pattern mismatch") };
                    Ok(daeprl1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut daeprl2: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
                    daeprl2 = DAEUtil::scodePrlToDaePrl(inParallelism.clone())?;
                    s1 = (DAEDump::dumpVarParallelismStr(daeprl2.clone())?).clone();
                    s2 = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
                    s3 = (DAEDump::dumpVarParallelismStr(inVarParallelism.clone())?).clone();
                    s4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Component declared as '")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("' when having the variable '")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("' declared as '")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("' : Subcomponent parallelism modified to.")); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::PARMODELICA_WARNING.clone(), list![(s4.clone()).clone()], inInfo.clone())?;
                    Ok(daeprl2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarParallelism)
}

fn propagateVisibility(mut inVarVisibility: DAE::VarVisibility, mut inVisibility: SCode::Visibility) -> DAE::VarVisibility {
    let mut outVarVisibility: DAE::VarVisibility = DAE::VarVisibility::PROTECTED;
    outVarVisibility = (match (inVarVisibility.clone(), inVisibility.clone()) {
        (_, SCode::Visibility::PROTECTED) => openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
        _ => inVarVisibility.clone(),
    });
    outVarVisibility
}

fn propagateVariability(mut inVarKind: DAE::VarKind, mut inVariability: SCode::Variability) -> DAE::VarKind {
    let mut outVarKind: DAE::VarKind = DAE::VarKind::CONST;
    outVarKind = (match (inVarKind.clone(), inVariability.clone()) {
        (_, SCode::Variability::VAR) => inVarKind.clone(),
        (DAE::VarKind::DISCRETE, _) => inVarKind.clone(),
        (_, SCode::Variability::DISCRETE) => openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        (DAE::VarKind::CONST, _) => inVarKind.clone(),
        (_, SCode::Variability::CONST) => openmodelica_frontend_types::DAE::VarKind::CONST,
        (DAE::VarKind::PARAM, _) => inVarKind.clone(),
        (_, SCode::Variability::PARAM) => openmodelica_frontend_types::DAE::VarKind::PARAM,
        _ => inVarKind.clone(),
    });
    outVarKind
}

fn propagateFinal(mut inVarAttributes: Option<Arc<DAE::VariableAttributes>>, mut inFinal: SCode::Final) -> Result<Option<Arc<DAE::VariableAttributes>>> {
    let mut outVarAttributes: Option<Arc<DAE::VariableAttributes>> = None;
    outVarAttributes = (::match_deref::match_deref! { match &((inVarAttributes.clone(), inFinal.clone())) {
        (_, SCode::Final::FINAL) => DAEUtil::setFinalAttr(inVarAttributes.clone(), SCodeUtil::finalBool(inFinal.clone())?)?,
        _ => inVarAttributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarAttributes)
}

fn propagateInnerOuter(mut inVarInnerOuter: Absyn::InnerOuter, mut inInnerOuter: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut outVarInnerOuter: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    outVarInnerOuter = (match (inVarInnerOuter.clone(), inInnerOuter.clone()) {
        (_, Absyn::InnerOuter::NOT_INNER_OUTER) => inVarInnerOuter.clone(),
        (Absyn::InnerOuter::NOT_INNER_OUTER, _) => inInnerOuter.clone(),
        _ => inVarInnerOuter.clone(),
    });
    outVarInnerOuter
}

fn propagateConnectorType(mut inVarConnectorType: Arc<DAE::ConnectorType>, mut inConnectorType: SCode::ConnectorType, mut inCref: Arc<DAE::ComponentRef>, mut inInfo: SourceInfo) -> Result<Arc<DAE::ConnectorType>> {
    let mut outVarConnectorType: Arc<DAE::ConnectorType> = Arc::new(DAE::ConnectorType::FLOW);
    outVarConnectorType = (::match_deref::match_deref! { match &((inVarConnectorType.clone(), inConnectorType.clone(), inCref.clone(), inInfo.clone())) {
        (_, SCode::ConnectorType::POTENTIAL, _, _) => {
            inVarConnectorType.clone()
        },
        (Deref @ DAE::ConnectorType::POTENTIAL, SCode::ConnectorType::FLOW, _, _) => {
            Arc::new(openmodelica_frontend_types::DAE::ConnectorType::FLOW)
        },
        (Deref @ DAE::ConnectorType::NON_CONNECTOR, SCode::ConnectorType::FLOW, _, _) => {
            Arc::new(openmodelica_frontend_types::DAE::ConnectorType::FLOW)
        },
        (Deref @ DAE::ConnectorType::POTENTIAL, SCode::ConnectorType::STREAM, _, _) => {
            Arc::new(DAE::ConnectorType::STREAM { associatedFlow: None })
        },
        (Deref @ DAE::ConnectorType::NON_CONNECTOR, SCode::ConnectorType::STREAM, _, _) => {
            Arc::new(DAE::ConnectorType::STREAM { associatedFlow: None })
        },
        _ => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            s1 = (SCodeDump::connectorTypeStr(inConnectorType.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
            s3 = (DAEDump::dumpConnectorType(inVarConnectorType.clone())).clone();
            Error::addSourceMessage(Error::INVALID_TYPE_PREFIX.clone(), list![(s1.clone()).clone(), (literal!("variable")).clone(), (s2.clone()).clone(), (s3.clone()).clone()], inInfo.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarConnectorType)
}

fn absynDirToDaeDir(mut inDirection: Absyn::Direction) -> Result<DAE::VarDirection> {
    let mut outVarDirection: DAE::VarDirection = DAE::VarDirection::BIDIR;
    outVarDirection = (match inDirection.clone() {
        Absyn::Direction::INPUT => openmodelica_frontend_types::DAE::VarDirection::INPUT,
        Absyn::Direction::OUTPUT => openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        Absyn::Direction::BIDIR => openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        _ => bail!("match: no arm matched"),
    });
    Ok(outVarDirection)
}

fn attrIsParam(mut inAttributes: SCode::Attributes) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inAttributes.clone() {
        SCode::Attributes { variability: SCode::Variability::PARAM, .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn elabComponentArraydimFromEnv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    (outCache, outDimensionLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }, _) => {
                    let mut m: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut m_1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut cmod_1: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut m_2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut mod_2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
                    let mut eq: DAE::EqMod;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { modifications: __pa1, .. }, __pa2, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    m = __pa1.clone();
                    cmod = __pa2.clone();
                    cmod_1 = Mod::stripSubmod(cmod.clone());
                    m_1 = SCodeUtil::stripSubmod(m.clone());
                    (cache, m_2) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, m_1.clone(), false, Mod::ModScope::COMPONENT { name: (id.clone()).clone() }, info.clone())?;
                    mod_2 = Mod::merge(cmod_1.clone(), m_2.clone(), (literal!("")).clone(), true)?;
                    let Some(__pa3) = (Mod::modEquation(mod_2.clone())?) else { bail!("pattern mismatch") };
                    eq = __pa3.clone();
                    (cache, dims) = elabComponentArraydimFromEnv2(cache.clone(), eq.clone(), env.clone())?;
                    Ok((cache.clone(), dims.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }, _) => {
                    let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), env.clone(), (id.clone()).clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: __pa1, .. }, .. }, _, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ad = __pa1.clone();
                    (cache, subs, _) = Static::elabSubscripts(cache.clone(), env.clone(), ad.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    dims = Expression::subscriptDimensions(subs.clone());
                    Ok((cache.clone(), dims.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cref, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.elabComponentArraydimFromEnv failed: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimensionLst))
}

fn elabComponentArraydimFromEnv2(mut inCache: FCore::Cache, mut inEqMod: DAE::EqMod, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    (outCache, outDimensionLst) = (match (inCache.clone(), inEqMod.clone(), inEnv.clone()) {
        (mut cache, DAE::EqMod::TYPED { properties: DAE::Properties::PROP { type_: ref t, .. }, .. }, _) => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut lst_1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            lst = Types::getDimensionSizes(t.clone())?;
            lst_1 = List::map(lst.clone(), (std::sync::Arc::new(fnptr!(Expression::intDimension, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Dimension>> + 'static>));
            (cache.clone(), lst_1.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outCache, outDimensionLst))
}

pub fn elabArraydimOpt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut path: Arc<Absyn::Path>, mut inAbsynArrayDimOption: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut inTypesEqModOption: Option<DAE::EqMod>, mut inBoolean: bool, mut performVectorization: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    (outCache, outDimensionLst) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inComponentRef.clone(), path.clone(), inAbsynArrayDimOption.clone(), inTypesEqModOption.clone(), inBoolean.clone(), performVectorization.clone(), inPrefix.clone(), info.clone(), inInstDims.clone())) {
        (cache, env, owncref, _, Some(ad), eq, r#impl, doVect, pre, _, inst_dims) => {
            let mut res: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, res) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), path.clone(), ad.clone(), eq.clone(), r#impl.clone(), doVect.clone(), false, pre.clone(), info.clone(), inst_dims.clone())?;
            (cache.clone(), res.clone())
        },
        (cache, _, _, _, None, _, _, _, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outDimensionLst))
}

pub fn elabArraydim(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<Absyn::ComponentRef>, mut path: Arc<Absyn::Path>, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inTypesEqModOption: Option<DAE::EqMod>, mut inBoolean: bool, mut performVectorization: bool, mut isFunctionInput: bool, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Dimension>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    (outCache, outDimensionLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), path.clone(), inArrayDim.clone(), inTypesEqModOption.clone(), inBoolean.clone(), performVectorization.clone(), isFunctionInput.clone(), inPrefix.clone(), inInfo.clone(), inInstDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, _, ad, _, _, doVect, true, pre, info, _) => {
                    let mut dim: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, dim) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), true, doVect.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, _, ad, None, r#impl, doVect, _, pre, info, _) => {
                    let mut dim: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, dim) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, _, ad, Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: prop, modifierAsAbsynExp: _, .. }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dim3: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    t = Types::getPropType(prop.clone())?;
                    (cache, dim1) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    dim2 = elabArraydimType(t.clone(), ad.clone(), e.clone(), path.clone(), pre.clone(), cref.clone(), info.clone(), inst_dims.clone())?;
                    dim3 = List::threadMap(dim1.clone(), dim2.clone(), (std::sync::Arc::new(compatibleArraydim) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> + 'static>));
                    Ok((cache.clone(), dim3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, _, ad, Some(DAE::EqMod::UNTYPED { exp: aexp }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dim3: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut e_1: Arc<DAE::Exp>;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), aexp.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    t = Types::getPropType(prop.clone())?;
                    (cache, dim1) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    dim2 = elabArraydimType(t.clone(), ad.clone(), e_1.clone(), path.clone(), pre.clone(), cref.clone(), info.clone(), inst_dims.clone())?;
                    dim3 = List::threadMap(dim1.clone(), dim2.clone(), (std::sync::Arc::new(compatibleArraydim) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> + 'static>));
                    Ok((cache.clone(), dim3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, _, ad, Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: DAE::Properties::PROP { type_: t, constFlag: _ }, modifierAsAbsynExp: _, info: info2 }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dim2: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut e_str: ArcStr = arcstr::literal!("");
                    let mut t_str: ArcStr = arcstr::literal!("");
                    let mut dim_str: ArcStr = arcstr::literal!("");
                    let false = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    (_, dim1) = Static::elabArrayDims(cache.clone(), env.clone(), cref.clone(), ad.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    dim2 = elabArraydimType(t.clone(), ad.clone(), e.clone(), path.clone(), pre.clone(), cref.clone(), info.clone(), inst_dims.clone())?;
                    if '__try0: {
                        let _ = List::threadMap(dim1.clone(), dim2.clone(), (std::sync::Arc::new(compatibleArraydim) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> + 'static>));
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    e_str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    t_str = (TypesDump::unparseType(t.clone())?).clone();
                    dim_str = (ExpressionBasics::dimensionsString(dim1.clone())).clone();
                    Error::addMultiSourceMessage(Error::ARRAY_DIMENSION_MISMATCH.clone(), list![(e_str.clone()).clone(), (t_str.clone()).clone(), (dim_str.clone()).clone()], cons(info2.clone(), cons(info.clone(), metamodelica::nil())))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cref, _, ad, eq, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- InstUtil.elabArraydim failed on: \n\tcref:")).clone())?;
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Dump::printComponentRefStr(cref.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::printArraydimStr(ad.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*TypesDump::unparseOptionEqMod(eq.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDimensionLst))
}

fn compatibleArraydim(mut inDimension1: Arc<DAE::Dimension>, mut inDimension2: Arc<DAE::Dimension>) -> Result<Arc<DAE::Dimension>> {
    let mut outDimension: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    outDimension = (::match_deref::match_deref! { match &((inDimension1.clone(), inDimension2.clone())) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN, Deref @ DAE::Dimension::DIM_UNKNOWN) => {
            Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN)
        },
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN) => {
            inDimension1.clone()
        },
        (Deref @ DAE::Dimension::DIM_UNKNOWN, _) => {
            inDimension2.clone()
        },
        (_, Deref @ DAE::Dimension::DIM_EXP { .. }) => {
            inDimension1.clone()
        },
        (Deref @ DAE::Dimension::DIM_EXP { .. }, _) => {
            inDimension2.clone()
        },
        (_, _) => {
            let true = (intEq(Expression::dimensionSize(inDimension1.clone())?, Expression::dimensionSize(inDimension2.clone())?)) else { bail!("pattern mismatch") };
            inDimension1.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- InstUtil.compatibleArraydim failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDimension)
}

fn elabArraydimType(mut inType: Arc<DAE::Type>, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inExp: Arc<DAE::Exp>, mut inPath: Arc<Absyn::Path>, mut inPrefix: DAE::Prefix, mut inCref: Arc<Absyn::ComponentRef>, mut inInfo: SourceInfo, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut flat_id: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut ad_str: ArcStr = arcstr::literal!("");
    let mut ty_str: ArcStr = arcstr::literal!("");
    let mut exp_str: ArcStr = arcstr::literal!("");
    let mut name_str: ArcStr = arcstr::literal!("");
    flat_id = if (Config::splitArrays()?) {metamodelica::nil()} else {List::flatten(inInstDims.clone())};
    match '__try0: {
        let true = (unwrap_break_err!(Types::numberOfDimensions(inType.clone()), '__try0) >= (inArrayDim.clone().len() as i32) + (flat_id.clone().len() as i32)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        outDimensions = unwrap_break_err!(elabArraydimType2(inType.clone(), inArrayDim.clone(), flat_id.clone()), '__try0);
        Ok::<_, anyhow::Error>((outDimensions.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDimensions = __try0_o0;
        }
        Err(_) => {
            ad_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*Dump::printArraydimStr(inArrayDim.clone())?); ArcStr::from(__mm_s) }).clone();
            ty_str = (TypesDump::unparseTypeNoAttr(inType.clone())?).clone();
            exp_str = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            name_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(inPrefix.clone())?); __mm_s.push_str(&*Dump::printComponentRefStr(inCref.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessageAndFail(Error::MODIFIER_DECLARATION_TYPE_MISMATCH_ERROR.clone(), list![(name_str.clone()).clone(), (ad_str.clone()).clone(), (exp_str.clone()).clone(), (ty_str.clone()).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok(outDimensions)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn elabArraydimType2(mut inType: Arc<DAE::Type>, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut tyDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut tydim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    outDimensions = 'mc: {
        let __mc_input = (inType.clone(), inArrayDim.clone(), inDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil } }, _, Deref @ metamodelica::List::Cons { head: dim, tail: rest_dims }) => {
                    compatibleArraydim(d.clone(), dim.clone())?;
                    Ok(elabArraydimType2(t.clone(), inArrayDim.clone(), rest_dims.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil } }, _, Deref @ metamodelica::List::Nil) => {
                    Ok(cons(d.clone(), elabArraydimType2(t.clone(), listRest(inArrayDim.clone())?, metamodelica::nil())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Undefined! The type detected: ")).clone())?;
                    Debug::traceln((TypesDump::printTypeStr(inType.clone())?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDimensions)
}

pub fn addFunctionsToDAE(mut inCache: FCore::Cache, mut funcs: Arc<metamodelica::List<DAE::Function>>, mut inPartialPrefix: SCode::Partial) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    outCache = (::match_deref::match_deref! { match &((inCache.clone(), funcs.clone(), inPartialPrefix.clone())) {
        (cache, _, _) => {
            let mut cache = (*cache).clone();
            cache = FCore::addDaeFunction(cache.clone(), funcs.clone())?;
            cache.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCache)
}

pub fn addNameToDerivativeMapping(mut inElts: Arc<metamodelica::List<DAE::Function>>, mut path: Arc<Absyn::Path>) -> Arc<metamodelica::List<DAE::Function>> {
    let mut outElts: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    outElts = {
        let mut __acc: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
        for mut r#fn in (inElts.clone()).into_iter().cloned() {
            let __x = (match r#fn.clone() {
        DAE::Function::FUNCTION { .. } => {
            let __owned_variant_functions_0 = addNameToDerivativeMappingFunctionDefs(var_field!(r#fn.functions, DAE::Function::FUNCTION).clone(), path.clone());
            if let DAE::Function::FUNCTION { functions, .. } = &mut r#fn {
                *functions = __owned_variant_functions_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            r#fn.clone()
        },
        _ => r#fn.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outElts
}

fn addNameToDerivativeMappingFunctionDefs(mut inFuncs: Arc<metamodelica::List<DAE::FunctionDefinition>>, mut path: Arc<Absyn::Path>) -> Arc<metamodelica::List<DAE::FunctionDefinition>> {
    let mut outFuncs: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
    outFuncs = {
        let mut __acc: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
        for mut r#fn in (inFuncs.clone()).into_iter().cloned() {
            let __x = (match r#fn.clone() {
        DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. } => {
            let __owned_variant_lowerOrderDerivatives_0 = cons(path.clone(), var_field!(r#fn.lowerOrderDerivatives, DAE::FunctionDefinition::FUNCTION_DER_MAPPER).clone());
            if let DAE::FunctionDefinition::FUNCTION_DER_MAPPER { lowerOrderDerivatives, .. } = &mut r#fn {
                *lowerOrderDerivatives = __owned_variant_lowerOrderDerivatives_0;
            } else { panic!("owned-variant field-assign: value held a different variant than DAE::FunctionDefinition::FUNCTION_DER_MAPPER"); }
            r#fn.clone()
        },
        _ => r#fn.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outFuncs
}

pub fn getDeriveAnnotation(mut cd: Arc<SCode::ClassDef>, mut cmt: Arc<SCode::Comment>, mut baseFunc: Arc<Absyn::Path>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<Arc<metamodelica::List<DAE::FunctionDefinition>>> {
    let mut element: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
    element = 'mc: {
        let __mc_input = (cd.clone(), cmt.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { annotation_: Some(ann), .. }), elementLst: elemDecl, .. }, _, _, _, _, _, _, _) => {
                    Ok(getDeriveAnnotation2(ann.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: elemDecl, .. }, Deref @ SCode::Comment { annotation_: Some(ann), .. }, _, _, _, _, _, _) => {
                    Ok(getDeriveAnnotation2(ann.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(element)
}

fn getDeriveAnnotation2(mut ann: Arc<SCode::Annotation>, mut elemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut baseFunc: Arc<Absyn::Path>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<Arc<metamodelica::List<DAE::FunctionDefinition>>> {
    let mut element: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
    element = (::match_deref::match_deref! { match &((ann.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())) {
        (Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: smlst, .. } }, _, _, _, _, _, _, _) => {
            getDeriveAnnotation3(smlst.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDeriveAnnotation3(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut elemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut baseFunc: Arc<Absyn::Path>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<Arc<metamodelica::List<DAE::FunctionDefinition>>> {
    let mut element: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
    element = 'mc: {
        let __mc_input = inSubs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "derivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), subModLst: subs2, .. } }, tail: subs } => {
                    let mut deriveFunc: Arc<Absyn::Path>;
                    let mut defaultDerivative: Option<Arc<Absyn::Path>> = None;
                    let mut order: i32 = 0;
                    let mut conditionRefs: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = metamodelica::nil();
                    let mut mapper: DAE::FunctionDefinition;
                    deriveFunc = AbsynUtil::crefToPath(acr.clone())?;
                    (_, deriveFunc) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), deriveFunc.clone())?;
                    order = getDerivativeOrder(subs2.clone());
                    ErrorExt::setCheckpoint((literal!("getDeriveAnnotation3")).clone());
                    conditionRefs = getDeriveCondition(subs2.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?;
                    ErrorExt::rollBack((literal!("getDeriveAnnotation3")).clone());
                    conditionRefs = List::sort(conditionRefs.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::derivativeOrder, (i32, DAE::derivativeCond), (i32, DAE::derivativeCond))) as std::sync::Arc<dyn ::std::ops::Fn((i32, DAE::derivativeCond), (i32, DAE::derivativeCond)) -> Result<bool> + 'static>))?;
                    defaultDerivative = getDerivativeSubModsOptDefault(subs.clone(), inCache.clone(), inEnv.clone(), inPrefix.clone())?;
                    mapper = DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: baseFunc.clone(), derivativeFunction: deriveFunc.clone(), derivativeOrder: order.clone(), conditionRefs: conditionRefs.clone(), defaultDerivative: defaultDerivative.clone(), lowerOrderDerivatives: metamodelica::nil() };
                    Ok(list![mapper.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: subs } => {
                    Ok(getDeriveAnnotation3(subs.clone(), elemDecl.clone(), baseFunc.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(element)
}

fn getDeriveCondition(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut elemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<Arc<metamodelica::List<(i32, DAE::derivativeCond)>>> {
    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = metamodelica::nil();
    outconds = 'mc: {
        let __mc_input = (inSubs.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "noDerivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs }, _, _, _, _, _, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut varPos: i32 = 0;
                    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = outconds.clone();
                    name = (Dump::printComponentRefStr(acr.clone())?).clone();
                    outconds = getDeriveCondition(subs.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?;
                    varPos = setFunctionInputIndex(elemDecl.clone(), (name.clone()).clone(), 1)?;
                    Ok(cons((varPos.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: Arc::new(DAE::Exp::ICONST { integer: 99 }) }), outconds.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "zeroDerivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs }, _, _, _, _, _, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut varPos: i32 = 0;
                    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = outconds.clone();
                    name = (Dump::printComponentRefStr(acr.clone())?).clone();
                    outconds = getDeriveCondition(subs.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?;
                    varPos = setFunctionInputIndex(elemDecl.clone(), (name.clone()).clone(), 1)?;
                    Ok(cons((varPos.clone(), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE), outconds.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "noDerivative", r#mod: m @ Deref @ SCode::Mod::MOD { .. } }, tail: subs }, _, _, _, _, _, _) => {
                    let mut sub: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut cond: DAE::derivativeCond = DAE::derivativeCond::ZERO_DERIVATIVE;
                    let mut varPos: i32 = 0;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut outconds: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = outconds.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mod::elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), m.clone(), false, Mod::ModScope::COMPONENT { name: (literal!("noDerivative")).clone() }, info.clone())?) {
                        (__pa0, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    sub = __pa1.clone();
                    (name, cond) = extractNameAndExp(sub.clone());
                    outconds = getDeriveCondition(subs.clone(), elemDecl.clone(), cache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?;
                    varPos = setFunctionInputIndex(elemDecl.clone(), (name.clone()).clone(), 1)?;
                    Ok(cons((varPos.clone(), cond.clone()), outconds.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: subs }, _, _, _, _, _, _) => {
                    Ok(getDeriveCondition(subs.clone(), elemDecl.clone(), inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), info.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outconds)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn setFunctionInputIndex(mut inElemDecl: Arc<metamodelica::List<Arc<SCode::Element>>>, mut r#str: ArcStr, mut currPos: i32) -> Result<i32> {
    let mut index: i32 = 0;
    index = 'mc: {
        let __mc_input = (inElemDecl.clone(), r#str.clone(), currPos.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" failure in setFunctionInputIndex, didn't find any index for: ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT, .. }, name: str2, .. }, tail: _ }, _, _) => {
                    let true = (stringEq((str2.clone()).clone(), (r#str.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(currPos.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT, .. }, .. }, tail: elemDecl }, _, _) => {
                    Ok(setFunctionInputIndex(elemDecl.clone(), (r#str.clone()).clone(), currPos.clone() + 1)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: elemDecl }, _, _) => {
                    Ok(setFunctionInputIndex(elemDecl.clone(), (r#str.clone()).clone(), currPos.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(index)
}

fn extractNameAndExp(mut m: Arc<DAE::SubMod>) -> (ArcStr, DAE::derivativeCond) {
    let mut inputVar: ArcStr = arcstr::literal!("");
    let mut cond: DAE::derivativeCond = DAE::derivativeCond::ZERO_DERIVATIVE;
    (inputVar, cond) = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, .. }), .. }, ident: inputVar } => {
            (inputVar.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: e.clone() })
        },
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: None, .. }, ident: inputVar } => {
            (inputVar.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: Arc::new(DAE::Exp::ICONST { integer: 1 }) })
        },
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: None, .. }, ident: inputVar } => {
            (inputVar.clone(), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE)
        },
        _ => {
            (literal!(""), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (inputVar, cond)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDerivativeSubModsOptDefault(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix) -> Result<Option<Arc<Absyn::Path>>> {
    let mut defaultDerivative: Option<Arc<Absyn::Path>> = None;
    defaultDerivative = 'mc: {
        let __mc_input = (inSubs.clone(), inCache.clone(), inEnv.clone(), inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "derivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: _ }, _, _, _) => {
                    let mut p: Arc<Absyn::Path>;
                    p = AbsynUtil::crefToPath(acr.clone())?;
                    (_, p) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), p.clone())?;
                    Ok(Some(p.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: subs }, _, _, _) => {
                    Ok(getDerivativeSubModsOptDefault(subs.clone(), inCache.clone(), inEnv.clone(), inPrefix.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(defaultDerivative)
}

fn getDerivativeOrder(mut inSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> i32 {
    let mut order: i32 = 0;
    order = (::match_deref::match_deref! { match &(inSubs.clone()) {
        Deref @ metamodelica::List::Nil => {
            1
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "order", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::INTEGER { value: order }), .. } }, tail: _ } => {
            order.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: subs } => {
            getDerivativeOrder(subs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    order
}

pub fn setFullyQualifiedTypename(mut inType: Arc<DAE::Type>, mut path: Arc<Absyn::Path>) -> Arc<DAE::Type> {
    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    resType = (::match_deref::match_deref! { match &(inType.clone()) {
        resType @ Deref @ DAE::Type::T_FUNCTION { .. } => {
            let mut resType = (*resType).clone();
            assign_variant_field!(resType => DAE::Type::T_FUNCTION; path = path.clone());
            resType.clone()
        },
        _ => {
            inType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    resType
}

pub fn classIsInlineFunc(mut elt: Arc<SCode::Element>) -> Result<DAE::InlineType> {
    let mut outInlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    outInlineType = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { .. } => InstBasics::commentIsInlineFunc(var_field!((*elt).cmt, SCode::Element::CLASS).clone())?,
        _ => openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInlineType)
}

pub fn stripFuncOutputsMod(mut elem: Arc<SCode::Element>) -> Arc<SCode::Element> {
    let mut stripped_elem: Arc<SCode::Element>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    stripped_elem = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::COMPONENT { modifications: r#mod @ Deref @ SCode::Mod::MOD { binding: Some(_), .. }, attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT, .. }, .. } => {
            let mut r#mod = (*r#mod).clone();
            assign_variant_field!(r#mod => SCode::Mod::MOD; binding = None);
            assign_variant_field!(elem => SCode::Element::COMPONENT; modifications = r#mod.clone());
            elem.clone()
        },
        _ => elem.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stripped_elem
}

pub fn checkExternalFunction(mut els: Arc<metamodelica::List<Arc<DAE::Element>>>, mut decl: DAE::ExternalDecl, mut name: ArcStr) -> Result<()> {
    let mut i: i32 = 0;
    if decl.language.clone() == literal!("builtin") {
        return Ok(());
    }
    List::map2_0(els.clone(), (std::sync::Arc::new(checkExternalFunctionOutputAssigned) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::ExternalDecl, ArcStr) -> Result<()> + 'static>), decl.clone(), (name.clone()).clone());
    checkFunctionInputUsed(els.clone(), Some(decl.clone()), (name.clone()).clone())?;
    Ok(())
}

pub fn checkFunctionInputUsed(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut decl: Option<DAE::ExternalDecl>, mut name: ArcStr) -> Result<()> {
    let mut invars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut algs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (vars, _, _, _, algs, _, _, _, _, _) = DAEUtil::splitElements(elts.clone())?;
    invars = List::filterOnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isInputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
    invars = List::select(invars.clone(), (std::sync::Arc::new(checkInputUsedAnnotation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>));
    invars = checkExternalDeclInputUsed(invars.clone(), decl.clone())?;
    invars = List::select1(invars.clone(), (std::sync::Arc::new(fnptr!(checkVarBindingsInputUsed, Arc<DAE::Element>, Arc<metamodelica::List<Arc<DAE::Element>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<bool> + 'static>), vars.clone());
    let (_, (_, __pa0)) = DAEUtil::traverseDAEElementList(algs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(checkExpInputUsed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)> + 'static>), invars.clone()));
    invars = __pa0.clone();
    List::map1_0(invars.clone(), (std::sync::Arc::new(warnUnusedFunctionVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, ArcStr) -> Result<()> + 'static>), (name.clone()).clone());
    Ok(())
}

pub fn checkInputUsedAnnotation(mut inElement: Arc<DAE::Element>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { comment: cmt, .. } => {
            result = SCodeUtil::optCommentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__OpenModelica_UnusedVariable")).clone())?;
            !(result.clone())
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn warnUnusedFunctionVar(mut v: Arc<DAE::Element>, mut name: ArcStr) -> Result<()> {
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut r#str: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Element::VAR { source: __pa0, componentRef: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    source = __pa0.clone();
    cr = __pa1.clone();
    r#str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
    Error::addSourceMessage(Error::FUNCTION_UNUSED_INPUT.clone(), list![(r#str.clone()).clone(), (name.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
    Ok(())
}

fn checkExternalDeclInputUsed(mut inames: Arc<metamodelica::List<Arc<DAE::Element>>>, mut decl: Option<DAE::ExternalDecl>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut onames: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    onames = (::match_deref::match_deref! { match &((inames.clone(), decl.clone())) {
        (names, None) => {
            names.clone()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (names, Some(DAE::ExternalDecl { args, returnArg: arg, .. })) => {
            let mut names = (*names).clone();
            names = List::select1(names.clone(), (std::sync::Arc::new(fnptr!(checkExternalDeclArgs, Arc<DAE::Element>, Arc<metamodelica::List<DAE::ExtArg>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<metamodelica::List<DAE::ExtArg>>) -> Result<bool> + 'static>), cons(arg.clone(), args.clone()));
            names.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(onames)
}

fn checkExpInputUsed(mut inExp: Arc<DAE::Exp>, mut inEls: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut exp: Arc<DAE::Exp>;
    let mut els: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    (exp, els) = 'mc: {
        let __mc_input = (inExp.clone(), inEls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, els) => {
                    let mut els = (*els).clone();
                    els = List::select1(els.clone(), (std::sync::Arc::new(checkExpInputUsed3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone());
                    Ok((exp.clone(), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CALL { path, .. }, els) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut els = (*els).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    cr = ComponentReference::pathToCref(path.clone())?;
                    els = List::select1(els.clone(), (std::sync::Arc::new(checkExpInputUsed3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone());
                    Ok((exp.clone(), els.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inEls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((exp, els))
}

fn checkExpInputUsed3(mut el: Arc<DAE::Element>, mut cr2: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut noteq: bool = false;
    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let __pa0 = ::match_deref::match_deref! { match &(el.clone()) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr1 = __pa0.clone();
    noteq = !(ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?);
    Ok(noteq)
}

fn checkVarBindingsInputUsed(mut v: Arc<DAE::Element>, mut els: Arc<metamodelica::List<Arc<DAE::Element>>>) -> bool {
    let mut notfound: bool = false;
    notfound = !(List::isMemberOnTrue(v.clone(), els.clone(), (std::sync::Arc::new(checkVarBindingInputUsed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<bool> + 'static>)));
    notfound
}

fn checkVarBindingInputUsed(mut v: Arc<DAE::Element>, mut el: Arc<DAE::Element>) -> Result<bool> {
    let mut found: bool = false;
    found = (::match_deref::match_deref! { match &((v.clone(), el.clone())) {
        (Deref @ DAE::Element::VAR { .. }, Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, .. }) => {
            false
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, .. }, Deref @ DAE::Element::VAR { binding: Some(exp), .. }) => {
            Expression::expHasCref(exp.clone(), cr.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(found)
}

fn checkExternalDeclArgs(mut v: Arc<DAE::Element>, mut args: Arc<metamodelica::List<DAE::ExtArg>>) -> bool {
    let mut notfound: bool = false;
    notfound = !(List::isMemberOnTrue(v.clone(), args.clone(), (std::sync::Arc::new(extArgCrefEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::ExtArg) -> Result<bool> + 'static>)));
    notfound
}

fn checkExternalFunctionOutputAssigned(mut v: Arc<DAE::Element>, mut decl: DAE::ExternalDecl, mut name: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((v.clone(), decl.clone(), name.clone())) {
        (Deref @ DAE::Element::VAR { source, binding, componentRef: cr, direction: DAE::VarDirection::OUTPUT, .. }, DAE::ExternalDecl { args, returnArg: arg, .. }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            if !(List::isMemberOnTrue(v.clone(), cons(arg.clone(), args.clone()), (std::sync::Arc::new(extArgCrefEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::ExtArg) -> Result<bool> + 'static>)) || isSome(binding.clone())) {
                r#str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                Error::addSourceMessage(Error::EXTERNAL_NOT_SINGLE_RESULT.clone(), list![(r#str.clone()).clone(), (name.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                bail!("fail");
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn extArgCrefEq(mut v: Arc<DAE::Element>, mut arg: DAE::ExtArg) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((v.clone(), arg.clone())) {
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARG { componentRef: cr2, .. }) => {
            let mut cr2 = (*cr2).clone();
            cr2 = ComponentReferenceBasics::crefFirstCref(cr2.clone())?;
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        (Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT, .. }, _) => {
            false
        },
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARGSIZE { componentRef: cr2, .. }) => {
            let mut cr2 = (*cr2).clone();
            cr2 = ComponentReferenceBasics::crefFirstCref(cr2.clone())?;
            ComponentReferenceBasics::crefEqualNoStringCompare(cr1.clone(), cr2.clone())?
        },
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARGEXP { exp, .. }) => {
            Expression::expHasCref(exp.clone(), cr1.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isExtExplicitCall(mut inExternalDecl: Arc<SCode::ExternalDecl>) -> bool {
    let mut isExplicit: bool = false;
    isExplicit = (::match_deref::match_deref! { match &(inExternalDecl.clone()) {
        Deref @ SCode::ExternalDecl { funcName: Some(_), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExplicit
}

fn isInoutVar(mut inElement: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = isOutputVar(inElement.clone()) || isInputVar(inElement.clone());
    b
}

fn isOutputVar(mut inElement: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isInputVar(mut inElement: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn instExtGetFname(mut inExternalDecl: Arc<SCode::ExternalDecl>, mut inIdent: ArcStr) -> Result<ArcStr> {
    let mut outIdent: ArcStr = arcstr::literal!("");
    outIdent = ((::match_deref::match_deref! { match &((inExternalDecl.clone(), inIdent.clone())) {
        (Deref @ SCode::ExternalDecl { funcName: Some(id), .. }, _) => {
            id.clone()
        },
        (Deref @ SCode::ExternalDecl { funcName: None, .. }, fid) => {
            fid.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

pub fn instExtGetAnnotation(mut inExternalDecl: Arc<SCode::ExternalDecl>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut outAnnotation: Option<Arc<SCode::Annotation>> = None;
    outAnnotation = (::match_deref::match_deref! { match &(inExternalDecl.clone()) {
        Deref @ SCode::ExternalDecl { annotation_: ann, .. } => {
            ann.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAnnotation)
}

pub fn instExtGetLang(mut inExternalDecl: Arc<SCode::ExternalDecl>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inExternalDecl.clone()) {
        Deref @ SCode::ExternalDecl { lang: Some(lang), .. } => {
            lang.clone()
        },
        Deref @ SCode::ExternalDecl { lang: None, .. } => {
            literal!("C")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn elabExpListExt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<DAE::Properties>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outTypesPropertiesLst: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    (outCache, outExpExpLst, outTypesPropertiesLst) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inPrefix.clone(), info.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), metamodelica::nil(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: e, tail: rest }, r#impl, pre, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut p: DAE::Properties;
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, exp, p) = elabExpExt(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            (cache, exps, props) = elabExpListExt(cache.clone(), env.clone(), rest.clone(), r#impl.clone(), pre.clone(), info.clone())?;
            (cache.clone(), cons(exp.clone(), exps.clone()), cons(p.clone(), props.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExpExpLst, outTypesPropertiesLst))
}

fn elabExpExt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: arraycr, tail: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "size", .. }, .. }, r#impl, pre, _) => {
                    let mut dimp: Arc<DAE::Exp>;
                    let mut arraycrefe: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut arraycrprop: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), env.clone(), dim.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, __pa2 @ DAE::Properties::PROP { type_: _, constFlag: _ }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dimp = __pa1.clone();
                    prop = __pa2.clone();
                    (cache, dimp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), dimp.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (cache, arraycrefe, arraycrprop) = Static::elabExp(cache.clone(), env.clone(), arraycr.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, arraycrefe, arraycrprop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), arraycrefe.clone(), arraycrprop.clone(), r#impl.clone(), info.clone())?;
                    exp = Arc::new(DAE::Exp::SIZE { exp: arraycrefe.clone(), sz: Some(dimp.clone()) });
                    Ok((cache.clone(), exp.clone(), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, absynExp, r#impl, pre, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e, prop) = Static::elabExp(cache.clone(), env.clone(), absynExp.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, e, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), e.clone(), prop.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("-Inst.elabExpExt failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp, outProperties))
}

pub fn instExtGetFargs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExternalDecl: Arc<SCode::ExternalDecl>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<DAE::ExtArg>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDAEExtArgLst: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
    (outCache, outDAEExtArgLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExternalDecl.clone(), inBoolean.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::ExternalDecl { args: absexps, lang, .. }, r#impl, pre, _) => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut props: Arc<metamodelica::List<DAE::Properties>> = metamodelica::nil();
                    let mut extargs: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, exps, props) = elabExpListExt(cache.clone(), env.clone(), absexps.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, extargs) = instExtGetFargs2(cache.clone(), env.clone(), absexps.clone(), exps.clone(), props.clone(), lang.clone(), info.clone())?;
                    Ok((cache.clone(), extargs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- InstUtil.instExtGetFargs failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outDAEExtArgLst))
}

fn instExtGetFargs2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut absynExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTypesPropertiesLst: Arc<metamodelica::List<DAE::Properties>>, mut lang: Option<ArcStr>, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<DAE::ExtArg>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outDAEExtArgLst: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
    (outCache, outDAEExtArgLst) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), absynExps.clone(), inExpExpLst.clone(), inTypesPropertiesLst.clone(), lang.clone(), info.clone())) {
        (cache, _, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: ae, tail: aes }, Deref @ metamodelica::List::Cons { head: e, tail: exps }, Deref @ metamodelica::List::Cons { head: p, tail: props }, _, _) => {
            let mut extargs: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut extarg: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
            let mut cache = (*cache).clone();
            let (__pa0, Some(__pa1)) = (instExtGetFargsSingle(cache.clone(), env.clone(), ae.clone(), e.clone(), p.clone(), lang.clone(), info.clone())?) else { bail!("pattern mismatch") };
            cache = __pa0.clone();
            extarg = __pa1.clone();
            (cache, extargs) = instExtGetFargs2(cache.clone(), env.clone(), aes.clone(), exps.clone(), props.clone(), lang.clone(), info.clone())?;
            (cache.clone(), cons(extarg.clone(), extargs.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outDAEExtArgLst))
}

fn instExtGetFargsSingle(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut absynExp: Arc<Absyn::Exp>, mut inExp: Arc<DAE::Exp>, mut inProperties: DAE::Properties, mut lang: Option<ArcStr>, mut info: SourceInfo) -> Result<(FCore::Cache, Option<DAE::ExtArg>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExtArg: Option<DAE::ExtArg> = None;
    (outCache, outExtArg) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), absynExp.clone(), inExp.clone(), inProperties.clone(), lang.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Exp::CREF { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, .. }, DAE::Properties::PROP { constFlag: DAE::Const::C_VAR, .. }, _, _) => {
                    let mut fattr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut fcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cache, _, ty, _, _, _, _, _, _) = Lookup::lookupVarLocal(inCache.clone(), inEnv.clone(), cref.clone())?;
                    fcr = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
                    (cache, fattr, _, _, _, _, _, _, _) = Lookup::lookupVarLocal(cache.clone(), inEnv.clone(), fcr.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARG { componentRef: cref.clone(), direction: DAEUtil::getAttrDirection(fattr.clone()), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Exp::CREF { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, .. }, DAE::Properties::PROP { constFlag: DAE::Const::C_VAR, .. }, _, _) => {
                    let mut attr: Arc<DAE::Attributes> = Arc::new(<DAE::Attributes as ::std::default::Default>::default());
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cache, attr, ty, _, _, _, _, _, _) = Lookup::lookupVarLocal(inCache.clone(), inEnv.clone(), cref.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARG { componentRef: cref.clone(), direction: DAEUtil::getAttrDirection(attr.clone()), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ DAE::Exp::CREF { componentRef: cref, .. }, DAE::Properties::PROP { .. }, _, _) => {
                    let mut crefstr: ArcStr = arcstr::literal!("");
                    let mut scope: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        (_, _, _, _, _, _, _, _, _) = unwrap_break_err!(Lookup::lookupVarLocal(cache.clone(), env.clone(), cref.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    crefstr = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
                    scope = (FGraph::printGraphPathStr(env.clone())?).clone();
                    Error::addMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(crefstr.clone()).clone(), (scope.clone()).clone()])?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ DAE::Exp::SIZE { sz: Some(dim), exp: Deref @ DAE::Exp::CREF { componentRef: cref, .. } }, DAE::Properties::PROP { .. }, _, _) => {
                    let mut varty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    (cache, _, varty, _, _, _, _, _, _) = Lookup::lookupVarLocal(cache.clone(), env.clone(), cref.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGSIZE { componentRef: cref.clone(), type_: varty.clone(), exp: dim.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, DAE::Properties::PROP { constFlag: DAE::Const::C_CONST, type_: ty }, _, _) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, exp, _) = Ceval::cevalIfConstant(cache.clone(), env.clone(), inExp.clone(), inProperties.clone(), false, info.clone())?;
                    let true = (Expression::isScalarConst(exp.clone())) else { bail!("pattern mismatch") };
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: exp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, DAE::Properties::PROP { type_: ty, .. }, Some(Deref @ "builtin"), _) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, DAE::Properties::PROP { type_: ty, .. }, _, _) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::CREF { componentRef: _ }, _, DAE::Properties::PROP { type_: ty, .. }, _, _) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, exp, DAE::Properties::PROP { .. }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    Error::addSourceMessage(Error::EXTERNAL_ARG_WRONG_EXP.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok((cache.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExtArg))
}

pub fn instExtGetRettype(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExternalDecl: Arc<SCode::ExternalDecl>, mut inBoolean: bool, mut inPrefix: DAE::Prefix, mut info: SourceInfo) -> Result<(FCore::Cache, DAE::ExtArg)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExtArg: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
    (outCache, outExtArg) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExternalDecl.clone(), inBoolean.clone(), inPrefix.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ SCode::ExternalDecl { output_: None, .. }, _, _, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::ExtArg::NOEXTARG))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::ExternalDecl { output_: Some(cref), lang, .. }, r#impl, pre, _) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut extarg: DAE::ExtArg = DAE::ExtArg::NOEXTARG;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cref.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2, _))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    exp = __pa1.clone();
                    prop = __pa2.clone();
                    let (__pa3, Some(__pa4)) = (instExtGetFargsSingle(cache.clone(), env.clone(), Arc::new(Absyn::Exp::CREF { componentRef: cref.clone() }), exp.clone(), prop.clone(), lang.clone(), info.clone())?) else { bail!("pattern mismatch") };
                    cache = __pa3.clone();
                    extarg = __pa4.clone();
                    assertExtArgOutputIsCrefVariable(lang.clone(), extarg.clone(), Types::getPropType(prop.clone())?, Types::propAllConst(prop.clone())?, info.clone())?;
                    Ok((cache.clone(), extarg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- InstUtil.instExtRettype failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExtArg))
}

fn assertExtArgOutputIsCrefVariable(mut lang: Option<ArcStr>, mut arg: DAE::ExtArg, mut ty: Arc<DAE::Type>, mut c: DAE::Const, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((lang.clone(), arg.clone(), ty.clone(), c.clone(), info.clone())) {
        (Some(Deref @ "builtin"), _, _, _, _) => {
            ()
        },
        (_, _, Deref @ DAE::Type::T_ARRAY { .. }, _, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (TypesDump::unparseType(ty.clone())?).clone();
            Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_ARRAY_TYPE.clone(), list![(r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (_, DAE::ExtArg::EXTARG { .. }, _, DAE::Const::C_VAR, _) => {
            ()
        },
        (_, _, _, DAE::Const::C_VAR, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (DAEDump::dumpExtArgStr(arg.clone())?).clone();
            Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_NOT_CREF.clone(), list![(r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => {
            Error::addSourceMessage(Error::EXTERNAL_FUNCTION_RESULT_NOT_VAR.clone(), metamodelica::nil(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn makeDaeProt(mut visibility: SCode::Visibility) -> Result<DAE::VarVisibility> {
    let mut res: DAE::VarVisibility = DAE::VarVisibility::PROTECTED;
    res = (match visibility.clone() {
        SCode::Visibility::PROTECTED => openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
        SCode::Visibility::PUBLIC => openmodelica_frontend_types::DAE::VarVisibility::PUBLIC,
        _ => bail!("match: no arm matched"),
    });
    Ok(res)
}

pub fn makeDaeVariability(mut inVariability: SCode::Variability) -> Result<DAE::VarKind> {
    let mut outVariability: DAE::VarKind = DAE::VarKind::CONST;
    outVariability = (match inVariability.clone() {
        SCode::Variability::VAR => openmodelica_frontend_types::DAE::VarKind::VARIABLE,
        SCode::Variability::PARAM => openmodelica_frontend_types::DAE::VarKind::PARAM,
        SCode::Variability::CONST => openmodelica_frontend_types::DAE::VarKind::CONST,
        SCode::Variability::DISCRETE => openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        _ => bail!("match: no arm matched"),
    });
    Ok(outVariability)
}

pub fn makeDaeDirection(mut inDirection: Absyn::Direction) -> Result<DAE::VarDirection> {
    let mut outDirection: DAE::VarDirection = DAE::VarDirection::BIDIR;
    outDirection = (match inDirection.clone() {
        Absyn::Direction::INPUT => openmodelica_frontend_types::DAE::VarDirection::INPUT,
        Absyn::Direction::OUTPUT => openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        Absyn::Direction::BIDIR => openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        _ => bail!("match: no arm matched"),
    });
    Ok(outDirection)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn mktype(mut inPath: Arc<Absyn::Path>, mut inState: ClassInf::State, mut inTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inTypesTypeOption: Option<Arc<DAE::Type>>, mut inEqualityConstraint: Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, mut inClass: Arc<SCode::Element>, mut inheritedComment: Arc<SCode::Comment>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inPath.clone(), inState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_INTEGER { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_INTEGER { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_REAL { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_REAL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_STRING { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_STRING { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_BOOL { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_BOOL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_CLOCK { .. }, v, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_CLOCK { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_ENUM { .. }, _, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::FUNCTION { .. }, vl, _, _, cl) => {
                    let mut functype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut funcattr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
                    funcattr = getFunctionAttributes(cl.clone(), vl.clone(), inheritedComment.clone())?;
                    functype = Types::makeFunctionType(p.clone(), vl.clone(), funcattr.clone())?;
                    Ok(functype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::ENUMERATION { path: p }, _, Some(enumtype), _, _) => {
                    let mut enumtype = (*enumtype).clone();
                    enumtype = Types::makeEnumerationType(p.clone(), enumtype.clone())?;
                    Ok(enumtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE { .. }, _, Some(Deref @ DAE::Type::T_ARRAY { ty: arrayType, .. }), None, _) => {
                    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut classState: ClassInf::State;
                    classState = arrayTTypeToClassInfState(arrayType.clone())?;
                    resType = mktype(inPath.clone(), classState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone(), inheritedComment.clone())?;
                    Ok(resType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE { .. }, _, Some(Deref @ DAE::Type::T_ARRAY { ty: arrayType, .. }), Some(_), _) => {
                    let mut resType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut classState: ClassInf::State;
                    classState = arrayTTypeToClassInfState(arrayType.clone())?;
                    resType = mktype(inPath.clone(), classState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone(), inheritedComment.clone())?;
                    resType = Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: inState.clone(), varLst: metamodelica::nil(), complexType: resType.clone(), equalityConstraint: inEqualityConstraint.clone() });
                    Ok(resType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_TUPLE { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_OPTION { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_LIST { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_POLYMORPHIC { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_ARRAY { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_UNIONTYPE { path: _, .. }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::META_UNIONTYPE { path: _, .. }, _, _, _, _) => {
                    let mut pstr: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    pstr = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    info = SCodeUtil::elementInfo(inClass.clone());
                    Error::addSourceMessage(Error::META_UNIONTYPE_ALIAS_MODS.clone(), list![(pstr.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, None, equalityConstraint, _) => {
                    if '__try0: {
                        let ClassInf::META_UNIONTYPE { path: _, .. } = (st.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: st.clone(), varLst: l.clone(), equalityConstraint: equalityConstraint.clone(), usedExternally: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, Some(bc), equalityConstraint, _) => {
                    if '__try0: {
                        let ClassInf::META_UNIONTYPE { path: _, .. } = (st.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: st.clone(), varLst: l.clone(), complexType: bc.clone(), equalityConstraint: equalityConstraint.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn arrayTTypeToClassInfState(mut arrayType: Arc<DAE::Type>) -> Result<ClassInf::State> {
    let mut classInfState: ClassInf::State;
    classInfState = (::match_deref::match_deref! { match &(arrayType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            ClassInf::State::TYPE_INTEGER { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            ClassInf::State::TYPE_REAL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            ClassInf::State::TYPE_STRING { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            ClassInf::State::TYPE_BOOL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            ClassInf::State::TYPE_CLOCK { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }
        },
        Deref @ DAE::Type::T_ARRAY { ty: t, .. } => {
            let mut cs: ClassInf::State;
            cs = arrayTTypeToClassInfState(t.clone())?;
            cs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(classInfState)
}

pub fn mktypeWithArrays(mut inPath: Arc<Absyn::Path>, mut inState: ClassInf::State, mut inTypesVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inTypesTypeOption: Option<Arc<DAE::Type>>, mut inClass: Arc<SCode::Element>, mut inheritedComment: Arc<SCode::Comment>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inPath.clone(), inState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ci, _, Some(tp), _) => {
                    let true = (Types::isArray(tp.clone())) else { bail!("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(ClassInfUtil::isConnector(ci.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_INTEGER { .. }, v, _, _) => {
                    let _ = getOptPath(p.clone());
                    Ok(Arc::new(DAE::Type::T_INTEGER { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_REAL { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_REAL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_STRING { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_STRING { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_BOOL { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_BOOL { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_CLOCK { .. }, v, _, _) => {
                    Ok(Arc::new(DAE::Type::T_CLOCK { varLst: v.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_ENUM { .. }, _, _, _) => {
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::FUNCTION { .. }, vl, _, cl) => {
                    let mut functype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut funcattr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
                    funcattr = getFunctionAttributes(cl.clone(), vl.clone(), inheritedComment.clone())?;
                    functype = Types::makeFunctionType(p.clone(), vl.clone(), funcattr.clone())?;
                    Ok(functype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::ENUMERATION { .. }, _, Some(enumtype), _) => {
                    let mut enumtype = (*enumtype).clone();
                    enumtype = Types::makeEnumerationType(p.clone(), enumtype.clone())?;
                    Ok(enumtype.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, None, _) => {
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: st.clone(), varLst: l.clone(), equalityConstraint: None, usedExternally: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, Some(bc), _) => {
                    Ok(Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: st.clone(), varLst: l.clone(), complexType: bc.clone(), equalityConstraint: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("InstUtil.mktypeWithArrays failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn getOptPath(mut inPath: Arc<Absyn::Path>) -> Option<Arc<Absyn::Path>> {
    let mut outAbsynPathOption: Option<Arc<Absyn::Path>> = None;
    outAbsynPathOption = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "" } => {
            None
        },
        p => {
            Some(p.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynPathOption
}

fn checkProt(mut inVisibility: SCode::Visibility, mut inMod: Arc<DAE::Mod>, mut inComponentRef: Arc<DAE::ComponentRef>, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inVisibility.clone(), inMod.clone(), inComponentRef.clone(), info.clone())) {
        (SCode::Visibility::PUBLIC, _, _, _) => {
            ()
        },
        (_, Deref @ DAE::Mod::NOMOD, _, _) => {
            ()
        },
        (_, Deref @ DAE::Mod::MOD { finalPrefix: _, eachPrefix: _, subModLst: Deref @ metamodelica::List::Nil, binding: None, .. }, _, _) => {
            ()
        },
        (SCode::Visibility::PROTECTED, _, cref, _) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (ComponentReferenceBasics::printComponentRefStr(cref.clone())?).clone();
            str2 = (Mod::prettyPrintMod(inMod.clone(), 0)?).clone();
            Error::addSourceMessage(Error::MODIFY_PROTECTED.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()], info.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn getStateSelectFromExpOption(mut inExpExpOption: Option<Arc<DAE::Exp>>) -> Option<DAE::StateSelect> {
    let mut outDAEStateSelectOption: Option<DAE::StateSelect> = None;
    outDAEStateSelectOption = (::match_deref::match_deref! { match &(inExpExpOption.clone()) {
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "never" }, name: Deref @ "StateSelect" }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::NEVER),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "avoid" }, name: Deref @ "StateSelect" }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::AVOID),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "default" }, name: Deref @ "StateSelect" }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::DEFAULT),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "prefer" }, name: Deref @ "StateSelect" }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::PREFER),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "always" }, name: Deref @ "StateSelect" }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::ALWAYS),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDAEStateSelectOption
}

pub fn isSubModNamed(mut inName: ArcStr, mut inSubMod: Arc<DAE::SubMod>) -> bool {
    let mut isNamed: bool = false;
    isNamed = (::match_deref::match_deref! { match &((inName.clone(), inSubMod.clone())) {
        (_, Deref @ DAE::SubMod { ident: submod_name, .. }) => {
            stringEqual((inName.clone()).clone(), (submod_name.clone()).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNamed
}

pub fn liftRecordBinding(mut inType: Arc<DAE::Type>, mut inExp: Arc<DAE::Exp>, mut inValue: Arc<Values::Value>) -> Result<(Arc<DAE::Exp>, Arc<Values::Value>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outExp, outValue) = 'mc: {
        let __mc_input = (inType.clone(), inExp.clone(), inValue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, _, _) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut int_dim: i32 = 0;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    int_dim = Expression::dimensionSize(dim.clone())?;
                    (exp, val) = liftRecordBinding(ty.clone(), inExp.clone(), inValue.clone())?;
                    ety = Types::simplifyType(inType.clone())?;
                    expl = List::fill(exp.clone(), int_dim.clone());
                    vals = List::fill(val.clone(), int_dim.clone());
                    exp = Arc::new(DAE::Exp::ARRAY { ty: ety.clone(), scalar: true, array: expl.clone() });
                    val = Arc::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: list![int_dim.clone()] });
                    Ok((exp.clone(), val.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Types::isArray(inType.clone())) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), inValue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outValue))
}

pub fn isTopCall(mut inCallingScope: InstTypes::CallingScope) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inCallingScope.clone() {
        InstTypes::CallingScope::TOP_CALL => true,
        _ => false,
    });
    outBoolean
}

pub fn extractCurrentName(mut sele: Arc<SCode::Element>) -> Result<(ArcStr, SourceInfo)> {
    let mut ostring: ArcStr = arcstr::literal!("");
    let mut oinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    (ostring, oinfo) = (::match_deref::match_deref! { match &(sele.clone()) {
        Deref @ SCode::Element::CLASS { info, name, .. } => {
            (name.clone(), info.clone())
        },
        Deref @ SCode::Element::COMPONENT { info, name, .. } => {
            (name.clone(), info.clone())
        },
        Deref @ SCode::Element::EXTENDS { info, baseClassPath: path, .. } => {
            let mut ret: ArcStr = arcstr::literal!("");
            ret = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            (ret.clone(), info.clone())
        },
        Deref @ SCode::Element::IMPORT { info, imp, .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (AbsynUtil::printImportString(imp.clone())?).clone();
            (name.clone(), info.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((ostring, oinfo))
}

pub fn reorderConnectEquationsExpandable(mut cache: FCore::Cache, mut env: FCore::Graph, mut inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<SCode::Equation>>>)> {
    let mut cache: FCore::Cache = cache;
    let mut outEquations: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut delst: DoubleEnded::MutableList<Arc<SCode::Equation>>;
    let mut expandableEqs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut crefLeft: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut crefRight: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if if (inEquations.clone().is_empty()) {true} else {!(System::getHasExpandableConnectors())} {
        outEquations = inEquations.clone();
        return Ok((cache, outEquations));
    }
    ErrorExt::setCheckpoint((literal!("expandableConnectorsOrder")).clone());
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    expandableEqs = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut eq in (inEquations.clone()).into_iter().cloned() {
            if !('mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_CONNECT { crefRight, crefLeft, .. } => {
                    let mut ty2: Arc<DAE::Type> = ty2.clone();
                    let mut ty1: Arc<DAE::Type> = ty1.clone();
                    (_, ty1, _, _) = Lookup::lookupConnectorVar(env.clone(), ComponentReference::toExpCref(crefLeft.clone())?, true)?;
                    let true = (Types::isExpandableConnector(ty1.clone())) else { bail!("pattern mismatch") };
                    (_, ty2, _, _) = Lookup::lookupConnectorVar(env.clone(), ComponentReference::toExpCref(crefRight.clone())?, true)?;
                    let true = (Types::isExpandableConnector(ty2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    DoubleEnded::push_back(delst.clone(), eq.clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }) { continue; }
            let __x = eq.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if expandableEqs.clone().is_empty() {
        ErrorExt::delCheckpoint((literal!("expandableConnectorsOrder")).clone());
        outEquations = inEquations.clone();
        return Ok((cache, outEquations));
    }
    ErrorExt::rollBack((literal!("expandableConnectorsOrder")).clone());
    DoubleEnded::push_list_front(delst.clone(), expandableEqs.clone())?;
    DoubleEnded::push_list_back(delst.clone(), expandableEqs.clone());
    let _ = (::match_deref::match_deref! { match &(expandableEqs.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
            DoubleEnded::push_list_back(delst.clone(), expandableEqs.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEquations = DoubleEnded::toListAndClear(delst.clone(), metamodelica::nil());
    Ok((cache, outEquations))
}

pub fn sortInnerFirstTplLstElementMod(mut inTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    outTplLstElementMod = 'mc: {
        let __mc_input = inTplLstElementMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (System::getHasInnerOuterDefinitions()) else { bail!("pattern mismatch") };
                    Ok(inTplLstElementMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut innerElts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut innerouterElts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut otherElts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut sorted: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut innerModelicaServices: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut innerModelica: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut innerOthers: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    (innerElts, innerouterElts, otherElts) = splitInnerAndOtherTplLstElementMod(inTplLstElementMod.clone());
                    (innerModelicaServices, innerModelica, innerOthers) = splitInners(innerElts.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                    sorted = listAppend(innerModelicaServices.clone(), listAppend(innerModelica.clone(), listAppend(innerOthers.clone(), listAppend(innerouterElts.clone(), otherElts.clone()))));
                    Ok(sorted.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTplLstElementMod)
}

fn splitInners(mut inTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inAcc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inAcc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inAcc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<(Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)> {
    let mut outModelicaServices: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outModelica: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outOthers: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    (outModelicaServices, outModelica, outOthers) = 'mc: {
        let __mc_input = (inTplLstElementMod.clone(), inAcc1.clone(), inAcc2.clone(), inAcc3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((inAcc1.clone().reverse(), inAcc2.clone().reverse(), inAcc3.clone().reverse()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: em, tail: rest }, _, _, _) => {
                    let mut acc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut acc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut acc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut e: Arc<SCode::Element>;
                    let mut p: Arc<Absyn::Path>;
                    e = Util::tuple21(em.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getComponentTypeSpec(e.clone())?) {
                        Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    let true = (stringEq((literal!("ModelicaServices")).clone(), (AbsynUtil::pathFirstIdent(p.clone())?).clone())) else { bail!("pattern mismatch") };
                    (acc1, acc2, acc3) = splitInners(rest.clone(), cons(em.clone(), inAcc1.clone()), inAcc2.clone(), inAcc3.clone())?;
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: em, tail: rest }, _, _, _) => {
                    let mut acc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut acc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut acc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut e: Arc<SCode::Element>;
                    let mut p: Arc<Absyn::Path>;
                    e = Util::tuple21(em.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getComponentTypeSpec(e.clone())?) {
                        Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p = __pa0.clone();
                    let true = (stringEq((literal!("Modelica")).clone(), (AbsynUtil::pathFirstIdent(p.clone())?).clone())) else { bail!("pattern mismatch") };
                    (acc1, acc2, acc3) = splitInners(rest.clone(), inAcc1.clone(), cons(em.clone(), inAcc2.clone()), inAcc3.clone())?;
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: em @ (_, _), tail: rest }, _, _, _) => {
                    let mut acc1: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut acc2: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    let mut acc3: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
                    (acc1, acc2, acc3) = splitInners(rest.clone(), inAcc1.clone(), inAcc2.clone(), cons(em.clone(), inAcc3.clone()))?;
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outModelicaServices, outModelica, outOthers))
}

pub fn splitInnerAndOtherTplLstElementMod(mut inTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> (Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) {
    let mut outInnerTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outInnerOuterTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outOtherTplLstElementMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut comp: Arc<SCode::Element>;
    let mut io: Absyn::InnerOuter = Absyn::InnerOuter::INNER;
    for mut e in &*inTplLstElementMod.clone().reverse() {
        let mut e = e.clone();
        (comp, _) = e.clone();
        let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } if (AbsynUtil::isInner(io.clone())) => {
            if AbsynUtil::isOuter(io.clone()) {
                outInnerOuterTplLstElementMod = cons(e.clone(), outInnerOuterTplLstElementMod.clone());
            } else {
                outInnerTplLstElementMod = cons(e.clone(), outInnerTplLstElementMod.clone());
            }
            ()
        },
        _ => {
            outOtherTplLstElementMod = cons(e.clone(), outOtherTplLstElementMod.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    (outInnerTplLstElementMod, outInnerOuterTplLstElementMod, outOtherTplLstElementMod)
}

pub fn splitEltsOrderInnerOuter(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut cdefImpElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut extElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (cdefImpElts, classextendsElts, extElts, compElts) = (::match_deref::match_deref! { match &(elts.clone()) {
        _ => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut comps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(elts.clone())?;
            comps = listAppend(innerComps.clone(), otherComps.clone());
            (cdefImpElts.clone(), classextendsElts.clone(), extElts.clone(), comps.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cdefImpElts, classextendsElts, extElts, compElts))
}

pub fn splitElts(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut cdefImpElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut extElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    for mut elt in &*elts.clone() {
        let mut elt = elt.clone();
        let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            if (::match_deref::match_deref! { match &(var_field!((*elt).classDef, SCode::Element::CLASS).clone()) {
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
                classextendsElts = cons(elt.clone(), classextendsElts.clone());
            } else {
                cdefImpElts = cons(elt.clone(), cdefImpElts.clone());
            }
            ()
        },
        Deref @ SCode::Element::IMPORT { .. } => {
            cdefImpElts = cons(elt.clone(), cdefImpElts.clone());
            ()
        },
        Deref @ SCode::Element::DEFINEUNIT { .. } => {
            cdefImpElts = cons(elt.clone(), cdefImpElts.clone());
            ()
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            extElts = cons(elt.clone(), extElts.clone());
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            compElts = cons(elt.clone(), compElts.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    cdefImpElts = cdefImpElts.clone().reverse();
    classextendsElts = classextendsElts.clone().reverse();
    extElts = extElts.clone().reverse();
    compElts = compElts.clone().reverse();
    Ok((cdefImpElts, classextendsElts, extElts, compElts))
}

pub fn splitEltsNoComponents(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut impElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut defElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut filtered: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (impElts, defElts, classextendsElts, filtered) = (::match_deref::match_deref! { match &(elts.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts.clone(), defElts.clone(), cons(elt.clone(), classextendsElts.clone()), filtered.clone())
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts.clone(), cons(elt.clone(), defElts.clone()), classextendsElts.clone(), cons(elt.clone(), filtered.clone()))
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (cons(elt.clone(), impElts.clone()), defElts.clone(), classextendsElts.clone(), filtered.clone())
        },
        Deref @ metamodelica::List::Cons { head: elt @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts.clone(), cons(elt.clone(), defElts.clone()), classextendsElts.clone(), cons(elt.clone(), filtered.clone()))
        },
        Deref @ metamodelica::List::Cons { head: elt, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs.clone())?;
            (impElts.clone(), defElts.clone(), classextendsElts.clone(), cons(elt.clone(), filtered.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((impElts, defElts, classextendsElts, filtered))
}

pub fn splitEltsInnerAndOther(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    let mut cdefImpElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut extElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut innerCompElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut otherCompElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (cdefImpElts, classextendsElts, extElts, innerCompElts, otherCompElts) = (::match_deref::match_deref! { match &(elts.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts.clone(), cons(cdef.clone(), classextendsElts.clone()), extElts.clone(), innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cons(cdef.clone(), cdefImpElts.clone()), classextendsElts.clone(), extElts.clone(), innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: imp @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cons(imp.clone(), cdefImpElts.clone()), classextendsElts.clone(), extElts.clone(), innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: imp @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cons(imp.clone(), cdefImpElts.clone()), classextendsElts.clone(), extElts.clone(), innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: ext @ Deref @ SCode::Element::EXTENDS { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts.clone(), classextendsElts.clone(), cons(ext.clone(), extElts.clone()), innerComps.clone(), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let true = (AbsynUtil::isInner(io.clone())) else { bail!("pattern mismatch") };
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts.clone(), classextendsElts.clone(), extElts.clone(), cons(comp.clone(), innerComps.clone()), otherComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { .. }, tail: xs } => {
            let mut innerComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut otherComps: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs.clone())?;
            (cdefImpElts.clone(), classextendsElts.clone(), extElts.clone(), innerComps.clone(), cons(comp.clone(), otherComps.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cdefImpElts, classextendsElts, extElts, innerCompElts, otherCompElts))
}

fn orderComponents(mut inComp: Arc<SCode::Element>, mut inCompElts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outCompElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outCompElts = (::match_deref::match_deref! { match &((inComp.clone(), inCompElts.clone())) {
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT, .. }, .. }, _) => {
            cons(inComp.clone(), inCompElts.clone())
        },
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT, .. }, .. }, _) => {
            cons(inComp.clone(), inCompElts.clone())
        },
        (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER, .. }, .. }, _) => {
            cons(inComp.clone(), inCompElts.clone())
        },
        (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER_OUTER, .. }, .. }, _) => {
            cons(inComp.clone(), inCompElts.clone())
        },
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::CONST, .. }, .. }, _) => {
            cons(inComp.clone(), inCompElts.clone())
        },
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::PARAM, .. }, .. }, _) => {
            cons(inComp.clone(), inCompElts.clone())
        },
        (Deref @ SCode::Element::COMPONENT { .. }, _) => {
            let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            compElts = listAppend(inCompElts.clone(), list![inComp.clone()]);
            compElts.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCompElts)
}

fn splitClassExtendsElts(mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<SCode::Element>>>) {
    let mut classextendsElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut outElts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    (classextendsElts, outElts) = (::match_deref::match_deref! { match &(elts.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (classextendsElts, res) = splitClassExtendsElts(xs.clone());
            (cons(cdef.clone(), classextendsElts.clone()), res.clone())
        },
        Deref @ metamodelica::List::Cons { head: cdef, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            (classextendsElts, res) = splitClassExtendsElts(xs.clone());
            (classextendsElts.clone(), cons(cdef.clone(), res.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (classextendsElts, outElts)
}

fn addClassdefsToEnv3(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inIH: Arc<metamodelica::List<InnerOuter::TopInstance>>, mut inPrefix: DAE::Prefix, mut inMod: Option<Arc<DAE::Mod>>, mut sele: Arc<SCode::Element>) -> Result<(FCore::Cache, FCore::Graph, Arc<metamodelica::List<InnerOuter::TopInstance>>, Arc<SCode::Element>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut oenv: FCore::Graph;
    let mut outIH: Arc<metamodelica::List<InnerOuter::TopInstance>> = metamodelica::nil();
    let mut osele: Arc<SCode::Element>;
    (outCache, oenv, outIH, osele) = (::match_deref::match_deref! { match &((inCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), inMod.clone(), sele.clone())) {
        (_, _, _, _, None, _) => {
            bail!("fail")
        },
        (cache, _, ih, pre, Some(Deref @ DAE::Mod::MOD { subModLst: lsm, .. }), Deref @ SCode::Element::CLASS { name: r#str, .. }) => {
            let mut mo2: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut sele2: Arc<SCode::Element>;
            let mut env2: FCore::Graph;
            let mut cache = (*cache).clone();
            let mut ih = (*ih).clone();
            (mo2, _) = extractCorrectClassMod2(lsm.clone(), (r#str.clone()).clone(), metamodelica::nil())?;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Inst::redeclareType(cache.clone(), env.clone(), ih.clone(), mo2.clone(), sele.clone(), pre.clone(), ClassInf::State::MODEL { path: Arc::new(Absyn::Path::IDENT { name: (r#str.clone()).clone() }) }, true, Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD))?) {
                (__pa0, __pa1, __pa2, __pa3 @ Deref @ SCode::Element::CLASS { .. }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            env2 = __pa1.clone();
            ih = __pa2.clone();
            sele2 = __pa3.clone();
            (cache.clone(), env2.clone(), ih.clone(), sele2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, oenv, outIH, osele))
}

fn extractCorrectClassMod2(mut smod: Arc<metamodelica::List<Arc<DAE::SubMod>>>, mut name: ArcStr, mut premod: Arc<metamodelica::List<Arc<DAE::SubMod>>>) -> Result<(Arc<DAE::Mod>, Arc<metamodelica::List<Arc<DAE::SubMod>>>)> {
    let mut omod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut restmods: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    (omod, restmods) = (::match_deref::match_deref! { match &((smod.clone(), name.clone(), premod.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            (Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), premod.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod }, tail: rest }, _, _) if (stringEq((id.clone()).clone(), (name.clone()).clone())) => {
            let mut rest2: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            rest2 = listAppend(premod.clone(), rest.clone());
            (r#mod.clone(), rest2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: sub, tail: rest }, _, _) => {
            let mut r#mod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
            let mut rest2: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
            (r#mod, rest2) = extractCorrectClassMod2(rest.clone(), (name.clone()).clone(), premod.clone())?;
            (r#mod.clone(), cons(sub.clone(), rest2.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((omod, restmods))
}

pub fn traverseModAddFinal(mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = 'mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::NOMOD => {
                    Ok(r#mod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { element: element1, eachPrefix: each_, .. } => {
                    let mut element2: Arc<SCode::Element>;
                    element2 = traverseModAddFinal3(element1.clone())?;
                    Ok(if (referenceEq(&element1.clone(),&element2.clone())) {r#mod.clone()} else {Arc::new(SCode::Mod::REDECL { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: each_.clone(), element: element2.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: subs1, binding: eq, comment: cmt, info } => {
                    let mut subs2: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    subs2 = List::mapCheckReferenceEq(subs1.clone(), (std::sync::Arc::new(traverseModAddFinal4) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<Arc<SCode::SubMod>> + 'static>));
                    Ok(if (openmodelica_frontend_types::SCode::Final::FINAL == f.clone() && referenceEq(&subs1.clone(),&subs2.clone())) {r#mod.clone()} else {Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: each_.clone(), subModLst: subs2.clone(), binding: eq.clone(), comment: cmt.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("InstUtil.traverseModAddFinal")).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(r#mod)
}

fn traverseModAddFinal3(mut inElement: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { name, prefixes, attributes: attr, typeSpec: tySpec, modifications: oldmod, comment: cmt, condition: cond, info } => {
                    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    r#mod = traverseModAddFinal(oldmod.clone())?;
                    Ok(if (referenceEq(&oldmod.clone(),&r#mod.clone())) {inElement.clone()} else {Arc::new(SCode::Element::COMPONENT { name: (name.clone()).clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: tySpec.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::IMPORT { .. } => {
                    Ok(inElement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    Ok(inElement.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { baseClassPath: p, visibility: vis, modifications: oldmod, ann, info } => {
                    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    r#mod = traverseModAddFinal(oldmod.clone())?;
                    Ok(if (referenceEq(&oldmod.clone(),&r#mod.clone())) {inElement.clone()} else {Arc::new(SCode::Element::EXTENDS { baseClassPath: p.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() })})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!(" we failed with traverseModAddFinal3\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElement)
}

fn traverseModAddFinal4(mut sub: Arc<SCode::SubMod>) -> Result<Arc<SCode::SubMod>> {
    let mut sub: Arc<SCode::SubMod> = sub;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    r#mod = traverseModAddFinal(sub.r#mod.clone())?;
    if !(referenceEq(&sub.r#mod.clone(),&r#mod.clone())) {
        assign_field!(sub.r#mod = r#mod.clone());
    }
    Ok(sub)
}

pub fn traverseModAddDims(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inMod: Arc<SCode::Mod>, mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPrefix.clone(), inMod.clone(), inInstDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, r#mod, _) => {
                    let true = (Config::splitArrays()?) else { bail!("pattern mismatch") };
                    Ok(r#mod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok(inMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, pre, r#mod, inst_dims) => {
                    let mut mod2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut exps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut aexps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
                    exps = List::map(inst_dims.clone(), (std::sync::Arc::new(fnptr!(Expression::dimensionsToExps, Arc<metamodelica::List<Arc<DAE::Dimension>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>));
                    aexps = List::mapList(exps.clone(), (std::sync::Arc::new(Expression::unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>));
                    mod2 = traverseModAddDims4(cache.clone(), env.clone(), pre.clone(), r#mod.clone(), aexps.clone(), true)?;
                    Ok(mod2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

fn traverseModAddDims4(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inMod: Arc<SCode::Mod>, mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inIsTop: bool) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inPrefix.clone(), inMod.clone(), inExps.clone())) {
        (_, _, _, Deref @ SCode::Mod::NOMOD, _) => {
            inMod.clone()
        },
        (_, _, _, Deref @ SCode::Mod::REDECL { .. }, _) => {
            inMod.clone()
        },
        (cache, env, pre, Deref @ SCode::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH, .. }, exps) => {
            let mut submods2: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            let mut binding: Option<Arc<Absyn::Exp>> = None;
            submods2 = traverseModAddDims5(cache.clone(), env.clone(), pre.clone(), var_field!((*inMod).subModLst, SCode::Mod::MOD).clone(), exps.clone())?;
            binding = insertSubsInBinding(var_field!((*inMod).binding, SCode::Mod::MOD).clone(), exps.clone())?;
            Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*inMod).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: submods2.clone(), binding: binding.clone(), comment: var_field!((*inMod).comment, SCode::Mod::MOD).clone(), info: var_field!((*inMod).info, SCode::Mod::MOD).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

fn traverseModAddDims5(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inPrefix: DAE::Prefix, mut inMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    outMods = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inPrefix.clone(), inMods.clone(), inExps.clone())) {
        (_, _, _, Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (cache, env, pre, Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod }, tail: smods }, _) => {
            let mut mod2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut smods2: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
            mod2 = traverseModAddDims4(cache.clone(), env.clone(), pre.clone(), r#mod.clone(), inExps.clone(), false)?;
            smods2 = traverseModAddDims5(cache.clone(), env.clone(), pre.clone(), smods.clone(), inExps.clone())?;
            cons(Arc::new(SCode::SubMod { ident: (n.clone()).clone(), r#mod: mod2.clone() }), smods2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMods)
}

fn insertSubsInBinding(mut inOpt: Option<Arc<Absyn::Exp>>, mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outOpt: Option<Arc<Absyn::Exp>> = None;
    outOpt = (::match_deref::match_deref! { match &((inOpt.clone(), inExps.clone())) {
        (None, _) => {
            None
        },
        (Some(e), exps) => {
            let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            vars = generateUnusedNamesLstCall(e.clone(), exps.clone())?;
            subs = List::mapList(vars.clone(), (std::sync::Arc::new(fnptr!(stringSub, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Subscript>> + 'static>));
            (e2, _) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(AbsynUtil::crefInsertSubscriptLstLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>)> + 'static>), subs.clone())?;
            e2 = wrapIntoForLst(e2.clone(), vars.clone(), exps.clone())?;
            Some(e2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outOpt)
}

fn generateUnusedNames(mut inExp: Arc<Absyn::Exp>, mut inList: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outNames, _) = generateUnusedNames2(inList.clone(), 1)?;
    Ok(outNames)
}

fn generateUnusedNames2(mut inList: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inInt: i32) -> Result<(Arc<metamodelica::List<ArcStr>>, i32)> {
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outInt: i32 = 0;
    (outNames, outInt) = (::match_deref::match_deref! { match &((inList.clone(), inInt.clone())) {
        (Deref @ metamodelica::List::Nil, i) => {
            (metamodelica::nil(), i.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: exps }, i) => {
            let mut i1: i32 = 0;
            let mut i2: i32 = 0;
            let mut s: ArcStr = arcstr::literal!("");
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            s = (intString(i.clone())).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("i")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            i1 = i.clone() + 1;
            (names, i2) = generateUnusedNames2(exps.clone(), i1.clone())?;
            (cons((s.clone()).clone(), names.clone()), i2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNames, outInt))
}

fn generateUnusedNamesLst(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inInt: i32) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, i32)> {
    let mut outNames: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut outInt: i32 = 0;
    (outNames, outInt) = (::match_deref::match_deref! { match &((inList.clone(), inInt.clone())) {
        (Deref @ metamodelica::List::Nil, i) => {
            (metamodelica::nil(), i.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e0, tail: exps }, i) => {
            let mut i1: i32 = 0;
            let mut i2: i32 = 0;
            let mut names: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            let mut ns: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (ns, i1) = generateUnusedNames2(e0.clone(), i.clone())?;
            (names, i2) = generateUnusedNamesLst(exps.clone(), i1.clone())?;
            (cons(ns.clone(), names.clone()), i2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNames, outInt))
}

fn generateUnusedNamesLstCall(mut inExp: Arc<Absyn::Exp>, mut inList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outNames: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    (outNames, _) = generateUnusedNamesLst(inList.clone(), 1)?;
    Ok(outNames)
}

fn stringsSubs(mut inNames: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    outSubs = (::match_deref::match_deref! { match &(inNames.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: n, tail: names } => {
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            subs = stringsSubs(names.clone());
            cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (n.clone()).clone(), subscripts: metamodelica::nil() }) }) }), subs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSubs
}

fn stringSub(mut inName: ArcStr) -> Arc<Absyn::Subscript> {
    let mut outSub: Arc<Absyn::Subscript> = Arc::new(Absyn::Subscript::NOSUB);
    outSub = (match inName.clone() {
        mut n => {
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (n.clone()).clone(), subscripts: metamodelica::nil() }) }) })
        },
    });
    outSub
}

fn wrapIntoFor(mut inExp: Arc<Absyn::Exp>, mut inNames: Arc<metamodelica::List<ArcStr>>, mut inRanges: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), inNames.clone(), inRanges.clone())) {
        (e, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            e.clone()
        },
        (e, Deref @ metamodelica::List::Cons { head: n, tail: names }, Deref @ metamodelica::List::Cons { head: r, tail: ranges }) => {
            let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            e2 = wrapIntoFor(e.clone(), names.clone(), ranges.clone())?;
            Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("array")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: e2.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, iterators: list![Arc::new(Absyn::ForIterator { name: (n.clone()).clone(), guardExp: None, range: Some(Arc::new(Absyn::Exp::RANGE { start: Arc::new(Absyn::Exp::INTEGER { value: 1 }), step: None, stop: r.clone() })) })] }), typeVars: metamodelica::nil() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn wrapIntoForLst(mut inExp: Arc<Absyn::Exp>, mut inNames: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut inRanges: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), inNames.clone(), inRanges.clone())) {
        (e, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            e.clone()
        },
        (e, Deref @ metamodelica::List::Cons { head: n, tail: names }, Deref @ metamodelica::List::Cons { head: r, tail: ranges }) => {
            let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e3: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            e2 = wrapIntoForLst(e.clone(), names.clone(), ranges.clone())?;
            e3 = wrapIntoFor(e2.clone(), n.clone(), r.clone())?;
            e3.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn componentHasCondition(mut component: (Arc<SCode::Element>, Arc<DAE::Mod>)) -> bool {
    let mut hasCondition: bool = false;
    hasCondition = (::match_deref::match_deref! { match &(component.clone()) {
        (Deref @ SCode::Element::COMPONENT { condition: Some(_), .. }, _) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasCondition
}

pub fn instElementCondExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut component: Arc<SCode::Element>, mut prefix: DAE::Prefix, mut info: SourceInfo) -> Result<(Option<bool>, FCore::Cache)> {
    let mut outCondValue: Option<bool> = None;
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outCondValue, outCache) = 'mc: {
        let __mc_input = component.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { condition: Some(cond_exp), .. } => {
                    let mut cond_val: bool = false;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    (cond_val, cache) = instConditionalDeclaration(inCache.clone(), inEnv.clone(), cond_exp.clone(), prefix.clone(), info.clone())?;
                    Ok((Some(cond_val.clone()), cache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { condition: Some(_), .. } => {
                    Ok((None, inCache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Some(true), inCache.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCondValue, outCache))
}

fn instConditionalDeclaration(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCondition: Arc<Absyn::Exp>, mut inPrefix: DAE::Prefix, mut inInfo: SourceInfo) -> Result<(bool, FCore::Cache)> {
    let mut outIsConditional: bool = false;
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut e: Arc<DAE::Exp>;
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut is_bool: bool = false;
    let mut is_param: bool = false;
    let mut b: bool = false;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabExp(inCache.clone(), inEnv.clone(), inCondition.clone(), false, false, inPrefix.clone(), inInfo.clone())?) {
        (__pa0, __pa1, DAE::Properties::PROP { constFlag: __pa2, type_: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outCache = __pa0.clone();
    e = __pa1.clone();
    c = __pa2.clone();
    t = __pa3.clone();
    if !(Types::isBoolean(t.clone())) {
        Error::addSourceMessageAndFail(Error::IF_CONDITION_TYPE_ERROR.clone(), list![(Dump::printExpStr(inCondition.clone())?).clone(), (TypesDump::unparseTypeNoAttr(t.clone())?).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if !(Types::isParameterOrConstant(c.clone())) {
        Error::addSourceMessageAndFail(Error::COMPONENT_CONDITION_VARIABILITY.clone(), list![(Dump::printExpStr(inCondition.clone())?).clone()], inInfo.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, val) = Ceval::ceval(outCache.clone(), inEnv.clone(), e.clone(), false, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
    outIsConditional = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::BOOL { boolean: b } => b.clone(),
        Deref @ Values::Value::EMPTY { .. } => {
            if !(Config::getGraphicsExpMode()?) {
                Error::addSourceMessage(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone(), list![(Dump::printExpStr(inCondition.clone())?).clone()], inInfo.clone())?;
                bail!("fail");
            }
            true
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.instConditionalDeclaration got unexpected value ")); __mm_s.push_str(&*ValuesDump::valString(val.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outIsConditional, outCache))
}

pub fn propagateClassPrefix(mut attr: SCode::Attributes, mut pre: DAE::Prefix) -> SCode::Attributes {
    let mut outAttr: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    outAttr = (match (attr.clone(), pre.clone()) {
        (_, DAE::Prefix::PREFIX { compPre: _, classPre: DAE::ClassPrefix { variability: SCode::Variability::VAR } }) => {
            attr.clone()
        },
        (SCode::Attributes { variability: SCode::Variability::CONST, .. }, _) => {
            attr.clone()
        },
        (SCode::Attributes { arrayDims: ref ad, connectorType: mut ct, parallelism: mut prl, variability: _, direction: mut dir, isField: mut isf }, DAE::Prefix::PREFIX { compPre: _, classPre: DAE::ClassPrefix { variability: mut vt } }) => {
            SCode::Attributes { arrayDims: ad.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: vt.clone(), direction: dir.clone(), isField: isf.clone() }
        },
        _ => {
            attr.clone()
        },
    });
    outAttr
}

pub fn checkUseConstValue(mut useConstValue: bool, mut ie: Arc<DAE::Exp>, mut v: Option<Arc<Values::Value>>) -> Result<Arc<DAE::Exp>> {
    let mut outE: Arc<DAE::Exp>;
    outE = 'mc: {
        let __mc_input = (useConstValue.clone(), ie.clone(), v.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, e, _) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, Some(val)) => {
                    let mut e: Arc<DAE::Exp>;
                    e = ValuesUtil::valueExp(val.clone(), None)?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, _) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outE)
}

pub fn propagateAbSCDirection(mut inVariability: SCode::Variability, mut inAttributes: SCode::Attributes, mut inClassAttributes: Option<SCode::Attributes>, mut inInfo: SourceInfo) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes = <SCode::Attributes as ::std::default::Default>::default();
    outAttributes = (match (inVariability.clone(), inAttributes.clone(), inClassAttributes.clone(), inInfo.clone()) {
        (SCode::Variability::CONST, _, _, _) => {
            inAttributes.clone()
        },
        (SCode::Variability::PARAM, _, _, _) => {
            inAttributes.clone()
        },
        _ => {
            let mut dir: Absyn::Direction = Absyn::Direction::BIDIR;
            let SCode::ATTR { direction: __pa0, .. } = (inAttributes.clone()) else { bail!("pattern mismatch") };
            dir = __pa0.clone();
            dir = propagateAbSCDirection2(dir.clone(), inClassAttributes.clone(), inInfo.clone())?;
            SCodeUtil::setAttributesDirection(inAttributes.clone(), dir.clone())
        },
    });
    Ok(outAttributes)
}

pub fn propagateAbSCDirection2(mut v1: Absyn::Direction, mut optDerAttr: Option<SCode::Attributes>, mut inInfo: SourceInfo) -> Result<Absyn::Direction> {
    let mut v3: Absyn::Direction = Absyn::Direction::BIDIR;
    v3 = (match (v1.clone(), optDerAttr.clone(), inInfo.clone()) {
        (_, None, _) => {
            v1.clone()
        },
        (Absyn::Direction::BIDIR, Some(SCode::Attributes { direction: mut v2, .. }), _) => {
            v2.clone()
        },
        (_, Some(SCode::Attributes { direction: Absyn::Direction::BIDIR, .. }), _) => {
            v1.clone()
        },
        (_, Some(SCode::Attributes { direction: mut v2, .. }), _) if (v1.clone() == v2.clone()) => {
            v1.clone()
        },
        _ => {
            println!("{}", (literal!(" failure in propagateAbSCDirection2, Absyn.DIRECTION mismatch")).clone());
            Error::addSourceMessage(Error::COMPONENT_INPUT_OUTPUT_MISMATCH.clone(), list![(literal!("")).clone(), (literal!("")).clone()], inInfo.clone())?;
            bail!("fail")
        },
    });
    Ok(v3)
}

pub fn makeCrefBaseType(mut inBaseType: Arc<DAE::Type>, mut inDimensions: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = Types::simplifyType(makeCrefBaseType2(inBaseType.clone(), inDimensions.clone())?)?;
    Ok(outType)
}

fn makeCrefBaseType2(mut inBaseType: Arc<DAE::Type>, mut inDimensions: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    outType = 'mc: {
        let __mc_input = (inBaseType.clone(), inDimensions.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. }, _) => {
                    let false = (TypesDump::getDimensions(ty.clone()).is_empty()) else { bail!("pattern mismatch") };
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(inBaseType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    dims = List::last(inDimensions.clone())?;
                    ty = Expression::liftArrayLeftList(inBaseType.clone(), dims.clone());
                    Ok(ty.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn getCrefFromCompDim(mut inEle: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut cref: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    cref = 'mc: {
        let __mc_input = inEle.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: ads, .. }, .. } => {
                    Ok(AbsynUtil::getCrefsFromSubs(ads.clone(), true, true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cref)
}

pub fn getCrefFromCond(mut cond: Option<Arc<Absyn::Exp>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    crefs = (::match_deref::match_deref! { match &(cond.clone()) {
        None => {
            metamodelica::nil()
        },
        Some(e) => {
            AbsynUtil::getCrefFromExp(e.clone(), true, true)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

fn checkVariabilityOfUpdatedComponent(mut variability: SCode::Variability, mut cref: Arc<Absyn::ComponentRef>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((variability.clone(), cref.clone())) {
        (SCode::Variability::VAR, _) => (),
        (SCode::Variability::DISCRETE, _) => (),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn propagateBinding(mut inVarsDae: DAE::DAElist, mut inEquationsDae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outVarsDae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut vars1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut equations1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut v1: Arc<DAE::Element>;
    let mut i: i32 = 0;
    let mut is: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut path: Arc<Absyn::Path>;
    let DAE::DAE { elementLst: __pa0 } = (inVarsDae.clone()) else { bail!("pattern mismatch") };
    vars = __pa0.clone();
    let DAE::DAE { elementLst: __pa1 } = (inEquationsDae.clone()) else { bail!("pattern mismatch") };
    equations = __pa1.clone();
    if vars.clone().is_empty() || equations.clone().is_empty() {
        outVarsDae = inVarsDae.clone();
        return Ok(outVarsDae);
    }
    vars1 = metamodelica::nil();
    is = metamodelica::nil();
    for mut v in &*vars.clone() {
        let mut v = v.clone();
        v1 = 'mc: {
        let __mc_input = v.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v1 @ Deref @ DAE::Element::VAR { .. } => {
                    let mut v1 = (*v1).clone();
                    let mut e: Arc<DAE::Exp>;
                    let mut i: i32 = i.clone();
                    let mut is: Arc<metamodelica::List<i32>> = is.clone();
                    (e, i) = findCorrespondingBinding(var_field!((*v1).componentRef, DAE::Element::VAR).clone(), equations.clone(), 1)?;
                    assign_variant_field!(v1 => DAE::Element::VAR; binding = Some(e.clone()));
                    is = cons(i.clone(), is.clone());
                    Ok(v1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        vars1 = cons(v1.clone(), vars1.clone());
    }
    vars1 = vars1.clone().reverse();
    equations1 = List::deletePositions(equations.clone(), is.clone(), false)?;
    equations1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut eq in (equations.clone()).into_iter().cloned() {
            let __x = DAEUtil::moveElementToInitialSection(eq.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    i = 1;
    for mut eq in &*equations1.clone() {
        let mut eq = eq.clone();
        let _ = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ DAE::Element::INITIALEQUATION { exp2: Deref @ DAE::Exp::CALL { path, .. }, exp1: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { .. }, .. }, .. } if (AbsynUtil::pathLastIdent(path.clone())? == literal!("constructor")) => {
            is = cons(i.clone(), is.clone());
            ()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            vars1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
        for mut v in (vars1.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Element::VAR { binding: Some(_), .. } if (ComponentReferenceBasics::crefPrefixOf(cr.clone(), var_field!((*v).componentRef, DAE::Element::VAR).clone())?) => {
            is = cons(i.clone(), is.clone());
            v.clone()
        },
        Deref @ DAE::Element::VAR { binding: None, .. } if (ComponentReferenceBasics::crefPrefixOf(cr.clone(), var_field!((*v).componentRef, DAE::Element::VAR).clone())?) => {
            assign_variant_field!(v => DAE::Element::VAR; variableAttributesOption = DAEUtil::setFixedAttr(var_field!((*v).variableAttributesOption, DAE::Element::VAR).clone(), Some(Arc::new(DAE::Exp::BCONST { bool: false })))?);
            Error::addSourceMessage(Error::MOVING_PARAMETER_BINDING_TO_INITIAL_EQ_SECTION.clone(), list![(ComponentReferenceBasics::printComponentRefStr(var_field!((*v).componentRef, DAE::Element::VAR).clone())?).clone()], var_field!((*eq).source, DAE::Element::INITIAL_COMPLEX_EQUATION).info.clone())?;
            v.clone()
        },
        _ => v.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        i = i.clone() + 1;
    }
    equations1 = List::deletePositions(equations1.clone(), is.clone(), false)?;
    outVarsDae = DAE::DAElist { elementLst: listAppend(equations1.clone(), vars1.clone()) };
    Ok(outVarsDae)
}

fn findCorrespondingBinding(mut inCref: Arc<DAE::ComponentRef>, mut inEquations: Arc<metamodelica::List<Arc<DAE::Element>>>, mut i: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut i: i32 = i;
    outExp = (::match_deref::match_deref! { match &((inCref.clone(), inEquations.clone())) {
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { exp: e, componentRef: cref2, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { scalar: e, exp: Deref @ DAE::Exp::CREF { componentRef: cref2, ty: _ }, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr2: cref3, cr1: cref2, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            Expression::crefExp(cref3.clone())?
        },
        (cref, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { rhs: e, lhs: Deref @ DAE::Exp::CREF { componentRef: cref2, ty: _ }, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(cref.clone(), cref2.clone())?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::List::Cons { head: _, tail: equations }) => {
            (outExp, i) = findCorrespondingBinding(cref.clone(), equations.clone(), i.clone() + 1)?;
            outExp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, i))
}

pub fn isPartial(mut partialPrefix: SCode::Partial, mut mods: Arc<DAE::Mod>) -> SCode::Partial {
    let mut outPartial: SCode::Partial = SCode::Partial::NOT_PARTIAL;
    outPartial = (::match_deref::match_deref! { match &((partialPrefix.clone(), mods.clone())) {
        (SCode::Partial::PARTIAL, Deref @ DAE::Mod::NOMOD) => openmodelica_frontend_types::SCode::Partial::PARTIAL,
        _ => openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPartial
}

pub fn isFunctionInput(mut classState: ClassInf::State, mut direction: Absyn::Direction) -> bool {
    let mut functionInput: bool = false;
    functionInput = (match (classState.clone(), direction.clone()) {
        (ClassInf::State::FUNCTION { .. }, Absyn::Direction::INPUT) => true,
        _ => false,
    });
    functionInput
}

pub fn extractComment(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<SCode::Comment>> {
    let mut cmt: Arc<SCode::Comment> = Arc::new(SCode::Comment { annotation_: None, comment: None });
    for mut elt in &*elts.clone() {
        let mut elt = elt.clone();
        let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ DAE::Element::COMMENT { cmt } => {
            return Ok(cmt);
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(cmt)
}

fn mergeClassComments(mut comment1: Arc<SCode::Comment>, mut comment2: Arc<SCode::Comment>) -> Result<Arc<SCode::Comment>> {
    let mut outComment: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    outComment = 'mc: {
        let __mc_input = (comment1.clone(), comment2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { info, subModLst: mods1, .. } }), comment: str1 }, Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: mods2, .. } }), comment: str2 }) => {
                    let mut r#str: Option<ArcStr> = None;
                    let mut mods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
                    r#str = if (isSome(str1.clone())) {str1.clone()} else {str2.clone()};
                    mods = listAppend(mods1.clone(), mods2.clone());
                    Ok(Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: mods.clone(), binding: None, comment: None, info: info.clone() }) })), comment: r#str.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Comment { annotation_: ann1, comment: str1 }, Deref @ SCode::Comment { annotation_: ann2, comment: str2 }) => {
                    let mut ann: Option<Arc<SCode::Annotation>> = None;
                    let mut r#str: Option<ArcStr> = None;
                    r#str = if (isSome(str1.clone())) {str1.clone()} else {str2.clone()};
                    ann = if (isSome(ann1.clone())) {ann1.clone()} else {ann2.clone()};
                    Ok(Arc::new(SCode::Comment { annotation_: ann.clone(), comment: r#str.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outComment)
}

pub fn makeNonExpSubscript(mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<DAE::Subscript>> {
    let mut outSubscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    outSubscript = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: e } => {
            Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: e.clone() })
        },
        subscript @ Deref @ DAE::Subscript::WHOLE_NONEXP { exp: _ } => {
            subscript.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscript)
}

fn getFunctionAttributes(mut cl: Arc<SCode::Element>, mut vl: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inheritedComment: Arc<SCode::Comment>) -> Result<DAE::FunctionAttributes> {
    let mut attr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
    let mut restriction: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut fres: SCode::FunctionRestriction = SCode::FunctionRestriction::FR_KERNEL_FUNCTION;
    let mut isOpenModelicaPure: bool = false;
    let mut isImpure: bool = false;
    let mut hasOutVars: bool = false;
    let mut unboxArgs: bool = false;
    let mut isBuiltin: DAE::FunctionBuiltin = DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR;
    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    let mut name: ArcStr = arcstr::literal!("");
    let mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut purity: Absyn::FunctionPurity = Absyn::FunctionPurity::IMPURE;
    let mut daePurity: DAE::Purity = DAE::Purity::PURE;
    restriction = SCodeUtil::getClassRestriction(cl.clone())?;
    let SCode::Restriction::R_FUNCTION { functionRestriction: __pa0 } = (restriction.clone()) else { bail!("pattern mismatch") };
    fres = __pa0.clone();
    daePurity = InstBasics::getFunctionRestrictionPurity(SCodeUtil::getFunctionRestrictionPurity(fres.clone()), inheritedComment.clone(), false)?;
    attr = 'mc: {
        let __mc_input = fres.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: mut purity } = __mc_input.clone() else { bail!("nomatch") };
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut unboxArgs: bool = unboxArgs.clone();
            let mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>> = inVars.clone();
            let mut name: ArcStr = name.clone();
            let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = outVars.clone();
            let mut isImpure: bool = isImpure.clone();
            isImpure = AbsynUtil::isImpure(purity.clone(), false);
            inVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isInputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
            outVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isOutputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
            name = (SCodeUtil::isBuiltinFunction(cl.clone(), List::map(inVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>)), List::map(outVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>)))?).clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone())?;
            unboxArgs = SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_UnboxArguments")).clone())?;
            Ok(DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity.clone(), isFunctionPointer: false, isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some((name.clone()).clone()), unboxArgs: unboxArgs.clone() }, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION = __mc_input.clone() else { bail!("nomatch") };
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isOpenModelicaPure: bool = isOpenModelicaPure.clone();
            let mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>> = inVars.clone();
            let mut unboxArgs: bool = unboxArgs.clone();
            let mut name: ArcStr = name.clone();
            let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>> = outVars.clone();
            inVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isInputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
            outVars = List::select(vl.clone(), (std::sync::Arc::new(Types::isOutputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
            name = (SCodeUtil::isBuiltinFunction(cl.clone(), List::map(inVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>)), List::map(outVars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>)))?).clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone())?;
            isOpenModelicaPure = !(SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_Impure")).clone())?);
            unboxArgs = SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_UnboxArguments")).clone())?;
            Ok(DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity.clone(), isFunctionPointer: false, isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some((name.clone()).clone()), unboxArgs: unboxArgs.clone() }, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION = __mc_input.clone() else { bail!("nomatch") };
            let mut isOpenModelicaPure: bool = isOpenModelicaPure.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone())?;
            isBuiltin = if (SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_BuiltinPtr")).clone())?) {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR} else {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN};
            isOpenModelicaPure = !(SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_Impure")).clone())?);
            Ok(DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity.clone(), isFunctionPointer: false, isBuiltin: isBuiltin.clone(), functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_KERNEL_FUNCTION = __mc_input.clone() else { bail!("nomatch") };
            Ok(DAE::FunctionAttributes { inline: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, generateEvents: false, purity: daePurity.clone(), isFunctionPointer: false, isBuiltin: openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_KERNEL_FUNCTION })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut daePurity: DAE::Purity = daePurity.clone();
            let mut hasOutVars: bool = hasOutVars.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment.clone())?;
            hasOutVars = List::any(vl.clone(), (std::sync::Arc::new(Types::isOutputVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
            isBuiltin = if (SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment.clone(), (literal!("__OpenModelica_BuiltinPtr")).clone())?) {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR} else {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN};
            if daePurity.clone() == DAE::Purity::UNDEFINED.clone() && SCodeUtil::isExternalFunctionRestriction(fres.clone()) && !(hasOutVars.clone() || Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())?) {
                daePurity = DAE::Purity::IMPURE.clone();
            }
            Ok(DAE::FunctionAttributes { inline: inlineType.clone(), generateEvents: false, purity: daePurity.clone(), isFunctionPointer: false, isBuiltin: isBuiltin.clone(), functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL })
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(attr)
}

pub fn checkFunctionElement(mut elt: Arc<DAE::Element>, mut isExternal: bool, mut info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((elt.clone(), isExternal.clone(), info.clone())) {
        (Deref @ DAE::Element::VAR { .. }, _, _) => {
            ()
        },
        (Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: Deref @ DAE::Exp::METARECORDCALL { .. }, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, _, _) => {
            ()
        },
        (Deref @ DAE::Element::ALGORITHM { .. }, false, _) => {
            ()
        },
        (Deref @ DAE::Element::COMMENT { .. }, _, _) => {
            ()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (DAEDump::dumpElementsStr(list![elt.clone()])?).clone();
            Error::addSourceMessage(Error::FUNCTION_ELEMENT_WRONG_KIND.clone(), list![(r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn printElementAndModList(mut inLstElAndMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ((::match_deref::match_deref! { match &(inLstElAndMod.clone()) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: (e, m), tail: rest } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            s1 = (SCodeDump::unparseElementStr(e.clone(), SCodeDump::defaultOptions.clone())?).clone();
            s2 = (Mod::printModStr(m.clone())?).clone();
            s3 = (printElementAndModList(rest.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Element:\n")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\nModifier: ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*s3.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStr)
}

fn splitClassDefsAndComponents(mut inLstElAndMod: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<(Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>)> {
    let mut outClassDefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut outComponentDefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    (outClassDefs, outComponentDefs) = (::match_deref::match_deref! { match &(inLstElAndMod.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: (e @ Deref @ SCode::Element::COMPONENT { .. }, m), tail: rest } => {
            let mut clsdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            let mut compdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            (clsdefs, compdefs) = splitClassDefsAndComponents(rest.clone())?;
            (clsdefs.clone(), cons((e.clone(), m.clone()), compdefs.clone()))
        },
        Deref @ metamodelica::List::Cons { head: (e, m), tail: rest } => {
            let mut clsdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            let mut compdefs: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
            (clsdefs, compdefs) = splitClassDefsAndComponents(rest.clone())?;
            (cons((e.clone(), m.clone()), clsdefs.clone()), compdefs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDefs, outComponentDefs))
}

pub fn selectModifiers(mut fromMerging: Arc<DAE::Mod>, mut fromRedeclareType: Arc<DAE::Mod>, mut typePath: Arc<Absyn::Path>) -> Result<(Arc<DAE::Mod>, Arc<DAE::Mod>)> {
    let mut bindingMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut classMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    (bindingMod, classMod) = 'mc: {
        let __mc_input = (fromMerging.clone(), fromRedeclareType.clone(), typePath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (redeclareBasicType(fromMerging.clone())?) else { bail!("pattern mismatch") };
                    Ok((fromRedeclareType.clone(), fromRedeclareType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((fromMerging.clone(), fromRedeclareType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((bindingMod, classMod))
}

pub fn redeclareBasicType(mut r#mod: Arc<DAE::Mod>) -> Result<bool> {
    let mut isRedeclareOfBasicType: bool = false;
    isRedeclareOfBasicType = 'mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
                    name = (AbsynUtil::pathFirstIdent(path.clone())?).clone();
                    let true = (listMember((name.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone(), (literal!("Clock")).clone()])) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let false = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
                    name = (AbsynUtil::pathFirstIdent(path.clone())?).clone();
                    let true = (listMember((name.clone()).clone(), list![(literal!("Real")).clone(), (literal!("Integer")).clone(), (literal!("Boolean")).clone(), (literal!("String")).clone()])) else { bail!("pattern mismatch") };
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
    Ok(isRedeclareOfBasicType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn optimizeFunctionCheckForLocals(mut path: Arc<Absyn::Path>, mut inElts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut oalg: Option<Arc<DAE::Element>>, mut acc: Arc<metamodelica::List<Arc<DAE::Element>>>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElts: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    outElts = (::match_deref::match_deref! { match &((path.clone(), inElts.clone(), oalg.clone(), acc.clone(), invars.clone(), outvars.clone())) {
        (_, Deref @ metamodelica::List::Nil, None, _, _, _) => {
            acc.clone().reverse()
        },
        (_, Deref @ metamodelica::List::Nil, Some(Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, source }), _, _, _) => {
            let mut stmts = (*stmts).clone();
            stmts = optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone().reverse(), outvars.clone().reverse(), metamodelica::nil())?;
            cons(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone() }), source: source.clone() }), acc.clone()).reverse()
        },
        (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Nil }, .. }, tail: elts }, _, _, _, _) => {
            optimizeFunctionCheckForLocals(path.clone(), elts.clone(), oalg.clone(), acc.clone(), invars.clone(), outvars.clone())?
        },
        (_, Deref @ metamodelica::List::Cons { head: elt1 @ Deref @ DAE::Element::ALGORITHM { source, .. }, tail: elts }, Some(elt2), _, _, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            if !(Config::acceptMetaModelicaGrammar()?) {
                Error::addSourceMessage(Error::FUNCTION_MULTIPLE_ALGORITHM.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            }
            optimizeFunctionCheckForLocals(path.clone(), elts.clone(), Some(elt1.clone()), cons(elt2.clone(), acc.clone()), invars.clone(), outvars.clone())?
        },
        (_, Deref @ metamodelica::List::Cons { head: elt @ Deref @ DAE::Element::ALGORITHM { .. }, tail: elts }, None, _, _, _) => {
            optimizeFunctionCheckForLocals(path.clone(), elts.clone(), Some(elt.clone()), acc.clone(), invars.clone(), outvars.clone())?
        },
        (_, Deref @ metamodelica::List::Cons { head: elt @ Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, .. }, tail: elts }, _, _, _, _) => {
            optimizeFunctionCheckForLocals(path.clone(), elts.clone(), oalg.clone(), cons(elt.clone(), acc.clone()), invars.clone(), cons((name.clone()).clone(), outvars.clone()))?
        },
        (_, Deref @ metamodelica::List::Cons { head: elt @ Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, .. }, tail: elts }, _, _, _, _) => {
            optimizeFunctionCheckForLocals(path.clone(), elts.clone(), oalg.clone(), cons(elt.clone(), acc.clone()), cons((name.clone()).clone(), invars.clone()), outvars.clone())?
        },
        (_, Deref @ metamodelica::List::Cons { head: elt, tail: elts }, _, _, _, _) => {
            optimizeFunctionCheckForLocals(path.clone(), elts.clone(), oalg.clone(), cons(elt.clone(), acc.clone()), invars.clone(), outvars.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElts)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn optimizeLastStatementTail(mut path: Arc<Absyn::Path>, mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>, mut acc: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut ostmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    ostmts = (::match_deref::match_deref! { match &((path.clone(), inStmts.clone(), invars.clone(), outvars.clone(), acc.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: stmt, tail: Deref @ metamodelica::List::Nil }, _, _, _) => {
            let mut stmt = (*stmt).clone();
            stmt = optimizeStatementTail(path.clone(), stmt.clone(), invars.clone(), outvars.clone())?;
            cons(stmt.clone(), acc.clone()).reverse()
        },
        (_, Deref @ metamodelica::List::Cons { head: stmt, tail: stmts }, _, _, _) => {
            optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone(), outvars.clone(), cons(stmt.clone(), acc.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ostmts)
}

fn optimizeStatementTail(mut path: Arc<Absyn::Path>, mut inStmt: Arc<DAE::Statement>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<DAE::Statement>> {
    let mut ostmt: Arc<DAE::Statement>;
    ostmt = 'mc: {
        let __mc_input = (path.clone(), inStmt.clone(), invars.clone(), outvars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, exp1: lhs, exp: rhs, source }, _, _) => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut stmt: Arc<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    name = (Expression::simpleCrefName(lhs.clone())?).clone();
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), list![(name.clone()).clone()], invars.clone(), outvars.clone(), source.clone())?;
                    stmt = if (Expression::isTailCall(rhs.clone())) {Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() })} else {Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() })};
                    Ok(stmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, expExpLst: lhsLst, exp: rhs, source }, _, _) => {
                    let mut lhsNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut stmt: Arc<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    lhsNames = List::map(lhsLst.clone(), (std::sync::Arc::new(Expression::simpleCrefName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>));
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), lhsNames.clone(), invars.clone(), outvars.clone(), source.clone())?;
                    stmt = if (Expression::isTailCall(rhs.clone())) {Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() })} else {Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: lhsLst.clone(), exp: rhs.clone(), source: source.clone() })};
                    Ok(stmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Statement::STMT_IF { exp: cond, statementLst: stmts, else_, source }, _, _) => {
                    let mut stmts = (*stmts).clone();
                    let mut else_ = (*else_).clone();
                    stmts = optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone(), outvars.clone(), metamodelica::nil())?;
                    else_ = optimizeElseTail(path.clone(), else_.clone(), invars.clone(), outvars.clone())?;
                    Ok(Arc::new(DAE::Statement::STMT_IF { exp: cond.clone(), statementLst: stmts.clone(), else_: else_.clone(), source: source.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Statement::STMT_NORETCALL { exp: rhs, source }, _, Deref @ metamodelica::List::Nil) => {
                    let mut stmt: Arc<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), metamodelica::nil(), invars.clone(), metamodelica::nil(), source.clone())?;
                    stmt = Arc::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() });
                    Ok(stmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inStmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ostmt)
}

fn optimizeElseTail(mut path: Arc<Absyn::Path>, mut inElse: Arc<DAE::Else>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<DAE::Else>> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    outElse = 'mc: {
        let __mc_input = (path.clone(), inElse.clone(), invars.clone(), outvars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Else::ELSEIF { exp: cond, statementLst: stmts, else_ }, _, _) => {
                    let mut stmts = (*stmts).clone();
                    let mut else_ = (*else_).clone();
                    stmts = optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone(), outvars.clone(), metamodelica::nil())?;
                    else_ = optimizeElseTail(path.clone(), else_.clone(), invars.clone(), outvars.clone())?;
                    Ok(Arc::new(DAE::Else::ELSEIF { exp: cond.clone(), statementLst: stmts.clone(), else_: else_.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Else::ELSE { statementLst: stmts }, _, _) => {
                    let mut stmts = (*stmts).clone();
                    stmts = optimizeLastStatementTail(path.clone(), stmts.clone(), invars.clone(), outvars.clone(), metamodelica::nil())?;
                    Ok(Arc::new(DAE::Else::ELSE { statementLst: stmts.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inElse.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElse)
}

fn optimizeStatementTail2(mut path: Arc<Absyn::Path>, mut rhs: Arc<DAE::Exp>, mut lhsVars: Arc<metamodelica::List<ArcStr>>, mut invars: Arc<metamodelica::List<ArcStr>>, mut outvars: Arc<metamodelica::List<ArcStr>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<DAE::Exp>> {
    let mut orhs: Arc<DAE::Exp>;
    let true = (lhsVars.clone() == outvars.clone()) else { bail!("pattern mismatch") };
    let __pa0 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path.clone(), rhs.clone(), invars.clone(), lhsVars.clone(), source.clone())?) {
        (__pa0, true) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orhs = __pa0.clone();
    Ok(orhs)
}

fn optimizeStatementTail3(mut path: Arc<Absyn::Path>, mut rhs: Arc<DAE::Exp>, mut vars: Arc<metamodelica::List<ArcStr>>, mut lhsVars: Arc<metamodelica::List<ArcStr>>, mut source: Arc<DAE::ElementSource>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut orhs: Arc<DAE::Exp>;
    let mut isTailRecursive: bool = false;
    (orhs, isTailRecursive) = 'mc: {
        let __mc_input = (path.clone(), rhs.clone(), vars.clone(), source.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path1, call @ Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::NO_TAIL, .. }, path: path2, .. }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut call = (*call).clone();
                    let mut attr = (*attr).clone();
                    let true = (AbsynUtil::pathEqual(path1.clone(), path2.clone())) else { bail!("pattern mismatch") };
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tail recursion of: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" with input vars: ")); __mm_s.push_str(&*stringDelimitList(vars.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
                    if Flags::isSet(Flags::TAIL.clone())? {
                        Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    }
                    assign_field!(attr.tailCall = DAE::TailCall::TAIL { vars: vars.clone(), outVars: lhsVars.clone() });
                    assign_variant_field!(call => DAE::Exp::CALL; attr = attr.clone());
                    Ok((call.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, _, _) => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut e2 = (*e2).clone();
                    let mut e3 = (*e3).clone();
                    (e2, b1) = optimizeStatementTail3(path.clone(), e2.clone(), vars.clone(), lhsVars.clone(), source.clone())?;
                    (e3, b2) = optimizeStatementTail3(path.clone(), e3.clone(), vars.clone(), lhsVars.clone(), source.clone())?;
                    let true = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok((Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e2.clone(), expElse: e3.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchType @ DAE::MatchType::MATCH { switch: _ }, inputs, aliases, localDecls, cases, et }, _, _) => {
                    let mut cases = (*cases).clone();
                    cases = optimizeStatementTailMatchCases(path.clone(), cases.clone(), false, metamodelica::nil(), vars.clone(), lhsVars.clone(), source.clone())?;
                    Ok((Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: matchType.clone(), inputs: inputs.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases.clone(), et: et.clone() }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((rhs.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((orhs, isTailRecursive))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn optimizeStatementTailMatchCases(mut path: Arc<Absyn::Path>, mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut changed: bool, mut inAcc: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut vars: Arc<metamodelica::List<ArcStr>>, mut lhsVars: Arc<metamodelica::List<ArcStr>>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<DAE::MatchCase>>>> {
    let mut ocases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    ocases = 'mc: {
        let __mc_input = (path.clone(), inCases.clone(), changed.clone(), inAcc.clone(), vars.clone(), source.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, true, acc, _, _) => {
                    Ok(acc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls, body, result: Some(exp), resultInfo, jump, info }, tail: cases }, _, acc, _, _) => {
                    let mut case_: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
                    let mut exp = (*exp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path.clone(), exp.clone(), vars.clone(), lhsVars.clone(), source.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    case_ = Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: localDecls.clone(), body: body.clone(), result: Some(exp.clone()), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() });
                    Ok(optimizeStatementTailMatchCases(path.clone(), cases.clone(), true, cons(case_.clone(), acc.clone()), vars.clone(), lhsVars.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls, body, result: Some(Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Nil }), resultInfo, jump, info }, tail: cases }, _, acc, _, _) => {
                    let mut case_: Arc<DAE::MatchCase> = Arc::new(<DAE::MatchCase as ::std::default::Default>::default());
                    let mut exp: Arc<DAE::Exp>;
                    let mut sourceStmt: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut body = (*body).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::last(body.clone())?) {
                        Deref @ DAE::Statement::STMT_NORETCALL { exp: __pa0, source: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    sourceStmt = __pa1.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path.clone(), exp.clone(), vars.clone(), lhsVars.clone(), source.clone())?) {
                        (__pa2, true) => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa2.clone();
                    body = List::set(body.clone(), (body.clone().len() as i32), Arc::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: sourceStmt.clone() }))?;
                    case_ = Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: localDecls.clone(), body: body.clone(), result: Some(Arc::new(DAE::Exp::TUPLE { PR: metamodelica::nil() })), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() });
                    Ok(optimizeStatementTailMatchCases(path.clone(), cases.clone(), true, cons(case_.clone(), acc.clone()), vars.clone(), lhsVars.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: case_, tail: cases }, _, acc, _, _) => {
                    Ok(optimizeStatementTailMatchCases(path.clone(), cases.clone(), changed.clone(), cons(case_.clone(), acc.clone()), vars.clone(), lhsVars.clone(), source.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ocases)
}

pub fn pushStructuralParameters(mut cache: FCore::Cache) -> FCore::Cache {
    let mut ocache: FCore::Cache = FCore::Cache::NO_CACHE;
    ocache = (::match_deref::match_deref! { match &(cache.clone()) {
        FCore::Cache::CACHE { initialGraph: ie, functions: f, evaluatedParams: (ht, crs), modelName: p } => {
            FCore::Cache::CACHE { initialGraph: ie.clone(), functions: f.clone(), evaluatedParams: (ht.clone(), cons(metamodelica::nil(), crs.clone())), modelName: p.clone() }
        },
        _ => {
            cache.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ocache
}

pub fn popStructuralParameters(mut cache: FCore::Cache, mut pre: DAE::Prefix) -> Result<FCore::Cache> {
    let mut ocache: FCore::Cache = FCore::Cache::NO_CACHE;
    ocache = (::match_deref::match_deref! { match &((cache.clone(), pre.clone())) {
        (FCore::Cache::CACHE { initialGraph: ie, functions: f, evaluatedParams: (ht, Deref @ metamodelica::List::Cons { head: crs, tail: crss }), modelName: p }, _) => {
            let mut ht = (*ht).clone();
            ht = prefixAndAddCrefsToHt(cache.clone(), ht.clone(), pre.clone(), crs.clone())?;
            FCore::Cache::CACHE { initialGraph: ie.clone(), functions: f.clone(), evaluatedParams: (ht.clone(), crss.clone()), modelName: p.clone() }
        },
        (FCore::Cache::CACHE { initialGraph: _, functions: _, evaluatedParams: (_, Deref @ metamodelica::List::Nil), modelName: _ }, _) => {
            cache.clone()
        },
        (FCore::Cache::NO_CACHE, _) => {
            cache.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ocache)
}

fn prefixAndAddCrefsToHt(mut cache: FCore::Cache, mut set: Arc<AvlSetCR::Tree>, mut pre: DAE::Prefix, mut icrs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<AvlSetCR::Tree>> {
    let mut set: Arc<AvlSetCR::Tree> = set;
    for mut cr in &*icrs.clone() {
        let mut cr = cr.clone();
        (_, cr) = PrefixUtil::prefixCref(cache.clone(), FGraph::empty(), InnerOuter::emptyInstHierarchy().clone(), pre.clone(), cr.clone())?;
        set = AvlSetCR::add(set.clone(), cr.clone())?;
    }
    Ok(set)
}

fn numStructuralParameterScopes(mut cache: FCore::Cache) -> Result<i32> {
    let mut i: i32 = 0;
    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let FCore::CACHE { evaluatedParams: (_, __pa0), .. } = (cache.clone()) else { bail!("pattern mismatch") };
    lst = __pa0.clone();
    i = (lst.clone().len() as i32);
    Ok(i)
}

pub fn checkFunctionDefUse(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut info: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (elts.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let _ = checkFunctionDefUse2(elts.clone(), None, metamodelica::nil(), metamodelica::nil(), info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("InstUtil.checkFunctionDefUse failed")).clone()], info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn checkFunctionDefUse2(mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>, mut alg: Option<Arc<metamodelica::List<Arc<DAE::Statement>>>>, mut inUnbound: Arc<metamodelica::List<ArcStr>>, mut inOutputs: Arc<metamodelica::List<ArcStr>>, mut inInfo: SourceInfo) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outUnbound: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outUnbound = (::match_deref::match_deref! { match &((elts.clone(), alg.clone(), inUnbound.clone(), inOutputs.clone(), inInfo.clone())) {
        (Deref @ metamodelica::List::Nil, None, _, _, _) => {
            inUnbound.clone()
        },
        (Deref @ metamodelica::List::Nil, Some(stmts), unbound, outputs, _) => {
            let mut unbound = (*unbound).clone();
            (_, _, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), false, (false, false, unbound.clone()));
            unbound = List::fold1(outputs.clone(), (std::sync::Arc::new(checkOutputDefUse) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, SourceInfo, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), inInfo.clone(), unbound.clone());
            unbound.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, .. }, tail: rest }, _, unbound, _, _) => {
            let mut unbound = (*unbound).clone();
            unbound = checkFunctionDefUse2(rest.clone(), alg.clone(), unbound.clone(), inOutputs.clone(), inInfo.clone())?;
            unbound.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: None, dims, ty: Deref @ DAE::Type::T_COMPLEX { varLst: vars, complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, direction: dir, .. }, tail: rest }, _, unbound, _, _) => {
            let mut outputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut vars = (*vars).clone();
            let mut unbound = (*unbound).clone();
            vars = List::filterOnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(Types::varIsVariable, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
            names = List::map1r(List::map(vars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone());
            outNames = if (DAEUtil::varDirectionEqual(dir.clone(), openmodelica_frontend_types::DAE::VarDirection::OUTPUT)) {names.clone()} else {metamodelica::nil()};
            names = if (Expression::dimensionsKnownAndNonZero(dims.clone())) {names.clone()} else {metamodelica::nil()};
            unbound = listAppend(names.clone(), unbound.clone());
            outputs = listAppend(outNames.clone(), inOutputs.clone());
            unbound = checkFunctionDefUse2(rest.clone(), alg.clone(), unbound.clone(), outputs.clone(), inInfo.clone())?;
            unbound.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { binding: None, dims, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, direction: dir, .. }, tail: rest }, _, unbound, _, _) => {
            let mut outputs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut unbound = (*unbound).clone();
            unbound = List::consOnTrue(Expression::dimensionsKnownAndNonZero(dims.clone()), (name.clone()).clone(), unbound.clone());
            outputs = List::consOnTrue(DAEUtil::varDirectionEqual(dir.clone(), openmodelica_frontend_types::DAE::VarDirection::OUTPUT), (name.clone()).clone(), inOutputs.clone());
            unbound = checkFunctionDefUse2(rest.clone(), alg.clone(), unbound.clone(), outputs.clone(), inInfo.clone())?;
            unbound.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: rest }, None, unbound, _, _) => {
            let mut unbound = (*unbound).clone();
            unbound = checkFunctionDefUse2(rest.clone(), Some(stmts.clone()), unbound.clone(), inOutputs.clone(), inInfo.clone())?;
            unbound.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, unbound, _, _) => {
            let mut unbound = (*unbound).clone();
            unbound = checkFunctionDefUse2(rest.clone(), alg.clone(), unbound.clone(), inOutputs.clone(), inInfo.clone())?;
            unbound.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outUnbound)
}

fn checkOutputDefUse(mut name: ArcStr, mut info: SourceInfo, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outUnbound: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut b: bool = false;
    b = listMember((name.clone()).clone(), inUnbound.clone());
    Error::assertionOrAddSourceMessage(!(b.clone()), Error::WARNING_DEF_USE.clone(), list![(name.clone()).clone()], info.clone())?;
    outUnbound = List::filter1OnTrue(inUnbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone());
    Ok(outUnbound)
}

fn checkFunctionDefUseStmt(mut inStmt: Arc<DAE::Statement>, mut inLoop: bool, mut inUnbound: (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> {
    let mut outUnbound: (bool, bool, Arc<metamodelica::List<ArcStr>>);
    outUnbound = (::match_deref::match_deref! { match &((inStmt.clone(), inLoop.clone(), inUnbound.clone())) {
        (_, _, (true, _, _)) => {
            inUnbound.clone()
        },
        (_, _, (false, true, _)) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            info = ElementSource::getElementSourceFileInfo(ElementSource::getStatementSource(inStmt.clone())?);
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("InstUtil.checkFunctionDefUseStmt failed")).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ DAE::Statement::STMT_ASSIGN { source, exp: rhs, exp1: lhs, .. }, _, (_, _, unbound)) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unbound = traverseCrefSubs(lhs.clone(), info.clone(), unbound.clone())?;
            unbound = crefFiltering(lhs.clone(), unbound.clone())?;
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, exp: rhs, expExpLst: lhss, .. }, _, (_, _, unbound)) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unbound = List::fold1(lhss.clone(), (std::sync::Arc::new(traverseCrefSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, SourceInfo, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), info.clone(), unbound.clone());
            unbound = List::fold(lhss.clone(), (std::sync::Arc::new(crefFiltering) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), unbound.clone());
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: rhs, lhs, .. }, _, (_, _, unbound)) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(rhs.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unbound = traverseCrefSubs(lhs.clone(), info.clone(), unbound.clone())?;
            unbound = crefFiltering(lhs.clone(), unbound.clone())?;
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_IF { exp, statementLst: stmts, else_, source }, _, (_, _, unbound)) => {
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            (b1, b2, unbound) = checkFunctionDefUseElse(Arc::new(DAE::Else::ELSEIF { exp: exp.clone(), statementLst: stmts.clone(), else_: else_.clone() }), unbound.clone(), inLoop.clone(), info.clone())?;
            (b1.clone(), b2.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_FOR { source, statementLst: stmts, range: exp, iter, .. }, _, (_, _, unbound)) => {
            let mut b: bool = false;
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (iter.clone()).clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()));
            (b.clone(), b.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_PARFOR { source, statementLst: stmts, range: exp, iter, .. }, _, (_, _, unbound)) => {
            let mut b: bool = false;
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (iter.clone()).clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()));
            (b.clone(), b.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_WHILE { source, statementLst: stmts, exp }, _, (_, _, unbound)) => {
            let mut b: bool = false;
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()));
            (b.clone(), b.clone(), unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: Deref @ DAE::Exp::BCONST { bool: false }, .. }, _, (_, _, _)) => {
            (true, true, metamodelica::nil())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { source, msg: exp2, cond: exp1, .. }, _, (_, _, unbound)) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp1.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            let (_, (__pa1, _)) = Expression::traverseExpTopDown(exp2.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa1.clone();
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_TERMINATE { source, msg: exp }, _, (_, _, unbound)) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (true, true, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Nil, path: Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }, .. }, .. }, _, (_, _, _)) => {
            (true, true, metamodelica::nil())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { source, exp }, _, (_, _, unbound)) => {
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut unbound = (*unbound).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_BREAK { .. }, _, (_, _, unbound)) => {
            (true, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_RETURN { .. }, _, (_, _, unbound)) => {
            (true, true, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_CONTINUE { .. }, _, (_, _, unbound)) => {
            (false, false, unbound.clone())
        },
        (Deref @ DAE::Statement::STMT_ARRAY_INIT { .. }, _, _) => {
            inUnbound.clone()
        },
        (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, _, (_, _, unbound)) => {
            let mut b: bool = false;
            let mut unbound = (*unbound).clone();
            (_, b, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), inLoop.clone(), (false, false, unbound.clone()));
            (b.clone(), b.clone(), unbound.clone())
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            r#str = (DAEDump::ppStatementStr(inStmt.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.checkFunctionDefUseStmt failed: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
            info = ElementSource::getElementSourceFileInfo(ElementSource::getStatementSource(inStmt.clone())?);
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUnbound)
}

fn checkFunctionDefUseElse(mut inElse: Arc<DAE::Else>, mut inUnbound: Arc<metamodelica::List<ArcStr>>, mut inLoop: bool, mut info: SourceInfo) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> {
    let mut outUnbound: (bool, bool, Arc<metamodelica::List<ArcStr>>);
    outUnbound = (::match_deref::match_deref! { match &((inElse.clone(), inUnbound.clone(), inLoop.clone(), info.clone())) {
        (Deref @ DAE::Else::NOELSE, _, _, _) => {
            (false, false, inUnbound.clone())
        },
        (Deref @ DAE::Else::ELSEIF { exp, statementLst: stmts, else_ }, unbound, iloop, _) => {
            let mut unboundBranch: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut b3: bool = false;
            let mut b4: bool = false;
            let mut unbound = (*unbound).clone();
            let mut iloop = (*iloop).clone();
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(exp.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (b1, b2, unboundBranch) = checkFunctionDefUseElse(else_.clone(), unbound.clone(), inLoop.clone(), info.clone())?;
            (b3, b4, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), inLoop.clone(), (false, false, unbound.clone()));
            iloop = true;
            unbound = if (iloop.clone()) {List::intersectionOnTrue(unboundBranch.clone(), unbound.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))} else {unbound.clone()};
            unbound = if (!(iloop.clone() || b1.clone())) {List::union(unboundBranch.clone(), unbound.clone())} else {unbound.clone()};
            b1 = b1.clone() && b3.clone();
            b2 = b2.clone() && b4.clone();
            (b1.clone(), b2.clone(), unbound.clone())
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, unbound, _, _) => {
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut unbound = (*unbound).clone();
            (b1, b2, unbound) = List::fold1(stmts.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), inLoop.clone(), (false, false, unbound.clone()));
            (b1.clone(), b2.clone(), unbound.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outUnbound)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn crefFiltering(mut inExp: Arc<DAE::Exp>, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outUnbound: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outUnbound = (::match_deref::match_deref! { match &((inExp.clone(), inUnbound.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _) => {
            inUnbound.clone()
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id2, .. }, ident: id1, .. }, .. }, unbound) => {
            let mut unbound = (*unbound).clone();
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id1.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*id2.clone()); ArcStr::from(__mm_s) }).clone());
            unbound.clone()
        },
        (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id1, .. } }, unbound) => {
            let mut id1 = (*id1).clone();
            let mut unbound = (*unbound).clone();
            id1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id1.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone();
            unbound = List::filter2OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::notStrncmp, ArcStr, ArcStr, i32)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, i32) -> Result<bool> + 'static>), (id1.clone()).clone(), ((id1.clone()).clone().len() as i32));
            unbound.clone()
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, unbound) => {
            let mut unbound = (*unbound).clone();
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone());
            unbound.clone()
        },
        (Deref @ DAE::Exp::ASUB { exp, .. }, unbound) => {
            crefFiltering(exp.clone(), unbound.clone())?
        },
        (Deref @ DAE::Exp::PATTERN { pattern }, unbound) => {
            let mut unbound = (*unbound).clone();
            (_, unbound) = Patternm::traversePattern(pattern.clone(), (std::sync::Arc::new(fnptr!(patternFiltering, Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)> + 'static>), unbound.clone())?;
            unbound.clone()
        },
        _ => {
            inUnbound.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUnbound)
}

fn patternFiltering(mut inPat: Arc<DAE::Pattern>, mut inLst: Arc<metamodelica::List<ArcStr>>) -> (Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>) {
    let mut outPat: Arc<DAE::Pattern> = inPat.clone();
    let mut unbound: Arc<metamodelica::List<ArcStr>> = inLst.clone();
    unbound = (::match_deref::match_deref! { match &(inPat.clone()) {
        Deref @ DAE::Pattern::PAT_AS { .. } => List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*inPat).id, DAE::Pattern::PAT_AS).clone()).clone()),
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. } => List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*inPat).id, DAE::Pattern::PAT_AS_FUNC_PTR).clone()).clone()),
        _ => unbound.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outPat, unbound)
}

fn traverseCrefSubs(mut exp: Arc<DAE::Exp>, mut info: SourceInfo, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outUnbound: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outUnbound = (::match_deref::match_deref! { match &((exp.clone(), info.clone(), inUnbound.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, unbound) => {
            let mut unbound = (*unbound).clone();
            let (_, (__pa0, _)) = Expression::traverseExpTopDownCrefHelper(cr.clone(), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unbound.clone()
        },
        _ => {
            inUnbound.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUnbound)
}

fn findUnboundVariableUse(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (Arc<metamodelica::List<ArcStr>>, SourceInfo);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (exp @ Deref @ DAE::Exp::SIZE { .. }, arg) => {
            (exp.clone(), false, arg.clone())
        },
        (exp @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "isPresent" }, .. }, arg) => {
            (exp.clone(), false, arg.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, (_, info)) => {
            Error::addSourceMessage(Error::WARNING_DEF_USE.clone(), list![(literal!("_")).clone()], info.clone())?;
            (inExp.clone(), true, inTpl.clone())
        },
        (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (unbound, info)) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            let mut unbound = (*unbound).clone();
            b = listMember((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), unbound.clone());
            r#str = (ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone();
            Error::assertionOrAddSourceMessage(!(b.clone()), Error::WARNING_DEF_USE.clone(), list![(r#str.clone()).clone()], info.clone())?;
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (r#str.clone()).clone());
            (exp.clone(), true, (unbound.clone(), info.clone()))
        },
        (exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, (unbound, info)) => {
            let mut b: bool = false;
            let mut unbound = (*unbound).clone();
            b = listMember((name.clone()).clone(), unbound.clone());
            Error::assertionOrAddSourceMessage(!(b.clone()), Error::WARNING_DEF_USE.clone(), list![(name.clone()).clone()], info.clone())?;
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::stringNotEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (name.clone()).clone());
            (exp.clone(), true, (unbound.clone(), info.clone()))
        },
        (exp @ Deref @ DAE::Exp::MATCHEXPRESSION { cases, localDecls, inputs, .. }, (unbound, info)) => {
            let mut unboundLocal: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut unbounds: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            let mut unbound = (*unbound).clone();
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(Arc::new(DAE::Exp::LIST { valList: inputs.clone() }), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            unboundLocal = checkFunctionDefUse2(localDecls.clone(), None, unbound.clone(), metamodelica::nil(), info.clone())?;
            unbounds = List::map1(cases.clone(), (std::sync::Arc::new(findUnboundVariableUseInCase) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::MatchCase>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), unboundLocal.clone());
            unbound = List::fold1r(unbounds.clone(), std::sync::Arc::new(fnptr!(List::intersectionOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), unbound.clone());
            (exp.clone(), false, (unbound.clone(), info.clone()))
        },
        (exp, arg) => {
            (exp.clone(), true, arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn findUnboundVariableUseInCase(mut case_: Arc<DAE::MatchCase>, mut inUnbound: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut unbound: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    unbound = (::match_deref::match_deref! { match &((case_.clone(), inUnbound.clone())) {
        (Deref @ DAE::MatchCase { resultInfo, info, result, body, patternGuard, patterns, .. }, unbound) => {
            let mut unbound = (*unbound).clone();
            (_, unbound) = Patternm::traversePatternList(patterns.clone(), (std::sync::Arc::new(fnptr!(patternFiltering, Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<DAE::Pattern>, Arc<metamodelica::List<ArcStr>>)> + 'static>), unbound.clone())?;
            let (_, (__pa0, _)) = Expression::traverseExpTopDown(Arc::new(DAE::Exp::META_OPTION { exp: patternGuard.clone() }), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), info.clone()))?;
            unbound = __pa0.clone();
            (_, _, unbound) = List::fold1(body.clone(), (std::sync::Arc::new(checkFunctionDefUseStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>, bool, (bool, bool, Arc<metamodelica::List<ArcStr>>)) -> Result<(bool, bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), true, (false, false, unbound.clone()));
            let (_, (__pa1, _)) = Expression::traverseExpTopDown(Arc::new(DAE::Exp::META_OPTION { exp: result.clone() }), (std::sync::Arc::new(findUnboundVariableUse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, SourceInfo))> + 'static>), (unbound.clone(), resultInfo.clone()))?;
            unbound = __pa1.clone();
            unbound.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(unbound)
}

pub fn checkParallelismWRTEnv(mut inEnv: FCore::Graph, mut inName: ArcStr, mut inAttr: SCode::Attributes, mut inInfo: SourceInfo) -> Result<bool> {
    let mut isValid: bool = false;
    isValid = 'mc: {
        let __mc_input = (inEnv.clone(), inName.clone(), inAttr.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, SCode::Attributes { direction: mut dir, parallelism: mut prl, .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut errorString: ArcStr = arcstr::literal!("");
            let mut scopeName: ArcStr = arcstr::literal!("");
            let mut isparglobal: bool = false;
            let mut hasnodir: bool = false;
            let mut r: metamodelica::Array<FCore::Node>;
            r = FGraph::lastScopeRef(inEnv.clone())?;
            let false = (FNode::isRefTop(r.clone())?) else { bail!("pattern mismatch") };
            scopeName = (FNode::refName(r.clone())?).clone();
            let true = (FGraph::checkScopeType(list![r.clone()], Some(crate::FCore::ScopeType::PARALLEL_SCOPE))?) else { bail!("pattern mismatch") };
            isparglobal = SCodeUtil::parallelismEqual(prl.clone(), openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL);
            hasnodir = !(AbsynUtil::isInputOrOutput(dir.clone())?);
            let true = (isparglobal.clone() && hasnodir.clone()) else { bail!("pattern mismatch") };
            errorString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- local parglobal component '")); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*literal!("' is declared in parallel/parkernel function '")); __mm_s.push_str(&*scopeName.clone()); __mm_s.push_str(&*literal!("'. \n")); __mm_s.push_str(&*literal!("- parglobal variables can be declared only in normal functions. \n")); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::PARMODELICA_ERROR.clone(), list![(errorString.clone()).clone()], inInfo.clone())?;
            Ok(false)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isValid)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn instDimsHasZeroDims(mut inInstDims: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>) -> Result<bool> {
    let mut outHasZeroDims: bool = false;
    outHasZeroDims = 'mc: {
        let __mc_input = inInstDims.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: dims, tail: _ } => {
                    let true = (List::any(dims.clone(), (std::sync::Arc::new(Expression::dimensionIsZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest_dims } => {
                    Ok(instDimsHasZeroDims(rest_dims.clone())?)
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
    Ok(outHasZeroDims)
}

pub fn noModForUpdatedComponents(mut variability: SCode::Variability, mut updatedComps: (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::ComponentRef>, i32)>>), i32, (HashTable5::FuncHashCref, HashTable5::FuncCrefEqual, HashTable5::FuncCrefStr, HashTable5::FuncExpStr)), mut cref: Arc<Absyn::ComponentRef>, mut mods: Arc<DAE::Mod>, mut cmod: Arc<DAE::Mod>, mut m: Arc<SCode::Mod>) -> Result<(Arc<DAE::Mod>, Arc<DAE::Mod>, Arc<SCode::Mod>)> {
    let mut outMods: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outCmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outM: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    (outMods, outCmod, outM) = 'mc: {
        let __mc_input = (variability.clone(), updatedComps.clone(), cref.clone(), mods.clone(), cmod.clone(), m.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    if !((BaseHashTable::hasKey(cref.clone(), updatedComps.clone()))) { bail!("guard") }
                    checkVariabilityOfUpdatedComponent(variability.clone(), cref.clone())?;
                    Ok((Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((mods.clone(), cmod.clone(), m.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMods, outCmod, outM))
}

pub fn propagateModFinal(mut inMod: Arc<DAE::Mod>, mut inFinal: SCode::Final) -> SCode::Final {
    let mut outFinal: SCode::Final = SCode::Final::FINAL;
    outFinal = (::match_deref::match_deref! { match &((inMod.clone(), inFinal.clone())) {
        (_, SCode::Final::FINAL) => {
            inFinal.clone()
        },
        (Deref @ DAE::Mod::MOD { finalPrefix: fp, .. }, _) => {
            fp.clone()
        },
        _ => {
            inFinal.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outFinal
}

//------------------------------
//------  PDE extension:  ------
//------------------------------
pub type DomainFieldOpt = Option<(Arc<Absyn::ComponentRef>, Arc<DAE::ComponentRef>)>;

pub type DomainFieldsLst = Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>>;

pub fn addGhostCells(mut inCompelts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>, mut inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> {
    let mut outCompelts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    let mut fieldNamesP: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ghostCompelts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    fieldNamesP = List::fold(inEqs.clone(), (std::sync::Arc::new(fieldsInPderEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), metamodelica::nil());
    ghostCompelts = List::fold1(inCompelts.clone(), (std::sync::Arc::new(addGhostCells2) as std::sync::Arc<dyn ::std::ops::Fn((Arc<SCode::Element>, Arc<DAE::Mod>), Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> + 'static>), fieldNamesP.clone(), metamodelica::nil());
    outCompelts = listAppend(inCompelts.clone(), ghostCompelts.clone());
    outCompelts
}

pub fn fieldsInPderEq(mut eq: Arc<SCode::Equation>, mut inFieldNames: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outFieldNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outFieldNames = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_PDE { expRight: rhs_exp, expLeft: lhs_exp, .. } => {
            let mut fieldNames1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (_, fieldNames1) = AbsynUtil::traverseExpTopDown(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(fieldInPderExp, Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)> + 'static>), inFieldNames.clone())?;
            (_, fieldNames1) = AbsynUtil::traverseExpTopDown(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(fieldInPderExp, Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>)> + 'static>), fieldNames1.clone())?;
            listAppend(inFieldNames.clone(), fieldNames1.clone())
        },
        _ => {
            inFieldNames.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFieldNames)
}

pub fn fieldInPderExp(mut inExp: Arc<Absyn::Exp>, mut inFieldNames: Arc<metamodelica::List<ArcStr>>) -> (Arc<Absyn::Exp>, Arc<metamodelica::List<ArcStr>>) {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outFieldNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outFieldNames = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: newFieldName, .. } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, .. } => {
            List::unionElt((newFieldName.clone()).clone(), inFieldNames.clone())
        },
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: newFieldName, .. } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, .. } => {
            List::unionElt((newFieldName.clone()).clone(), inFieldNames.clone())
        },
        _ => {
            inFieldNames.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp = inExp.clone();
    (outExp, outFieldNames)
}

pub fn addGhostCells2(mut inCompelt: (Arc<SCode::Element>, Arc<DAE::Mod>), mut fieldNamesP: Arc<metamodelica::List<ArcStr>>, mut inGhosts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>>> {
    let mut outGhosts: Arc<metamodelica::List<(Arc<SCode::Element>, Arc<DAE::Mod>)>> = metamodelica::nil();
    outGhosts = 'mc: {
        let __mc_input = inCompelt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { name, prefixes, attributes: SCode::Attributes { arrayDims, connectorType, parallelism, variability, direction, isField: Absyn::IsField::FIELD }, typeSpec, modifications: r#mod @ Deref @ SCode::Mod::MOD { .. }, comment, condition, info }, daeMod) => {
                    if !((listMember((name.clone()).clone(), fieldNamesP.clone()))) { bail!("guard") }
                    let mut ghostL: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    let mut ghostR: (Arc<SCode::Element>, Arc<DAE::Mod>);
                    let mut r#mod = (*r#mod).clone();
                    todo!("unhandled field-assign shape: r#mod.subModLst");
                    ghostL = (Arc::new(SCode::Element::COMPONENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: arrayDims.clone(), connectorType: connectorType.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: typeSpec.clone(), modifications: r#mod.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), daeMod.clone());
                    ghostR = (Arc::new(SCode::Element::COMPONENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: arrayDims.clone(), connectorType: connectorType.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: typeSpec.clone(), modifications: r#mod.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), daeMod.clone());
                    Ok(cons(ghostL.clone(), cons(ghostR.clone(), inGhosts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGhosts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGhosts)
}

pub fn isSubModDomainOrStart(mut subMod: Arc<SCode::SubMod>) -> bool {
    let mut isNotDomain: bool = false;
    isNotDomain = (::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ SCode::SubMod { ident: idn, .. } if (idn.clone() == literal!("domain") || idn.clone() == literal!("start")) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNotDomain
}

pub fn elabField(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut name: ArcStr, mut attr: SCode::Attributes, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inMod: Arc<DAE::Mod>, mut inInfo: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<DAE::Dimension>>>, Arc<DAE::Mod>, DomainFieldOpt)> {
    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut outMod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut outFieldDomOpt: DomainFieldOpt = None;
    (outDims, outMod, outFieldDomOpt) = ({
        let mut N: i32 = -1;
        (::match_deref::match_deref! { match &((attr.clone(), inMod.clone())) {
        (SCode::Attributes { isField: Absyn::IsField::NONFIELD, .. }, _) => {
            (inDims.clone(), inMod.clone(), None)
        },
        (SCode::Attributes { isField: Absyn::IsField::FIELD, .. }, Deref @ DAE::Mod::MOD { info, binding, subModLst, eachPrefix, finalPrefix }) => {
            let mut dim_f: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
            let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut domainSubMod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
            let mut subModLst = (*subModLst).clone();
            (domainSubMod, subModLst) = List::findAndRemove(subModLst.clone(), (std::sync::Arc::new(fnptr!(findDomainSubMod, Arc<DAE::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>) -> Result<bool> + 'static>))?;
            dcr = getQualDcr(domainSubMod.clone(), inInfo.clone())?;
            (N, dcr) = getNDcr(dcr.clone())?;
            if N.clone() == -1 {
                Error::addSourceMessageAndFail(Error::PDEModelica_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Domain of the field variable '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' not found.")); ArcStr::from(__mm_s) }).clone()], inInfo.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            subModLst = subModLst.clone().reverse();
            subModLst = List::map(subModLst.clone(), (std::sync::Arc::new(addEach) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::SubMod>) -> Result<Arc<DAE::SubMod>> + 'static>));
            outMod = Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: eachPrefix.clone(), subModLst: subModLst.clone(), binding: binding.clone(), info: info.clone() });
            dim_f = Arc::new(DAE::Dimension::DIM_INTEGER { integer: N.clone() });
            (cons(dim_f.clone(), inDims.clone()), outMod.clone(), Some((Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: metamodelica::nil() }), dcr.clone())))
        },
        (_, Deref @ DAE::Mod::NOMOD) => {
            Error::addSourceMessageAndFail(Error::PDEModelica_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' has no domain modifier.")); ArcStr::from(__mm_s) }).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            (inDims.clone(), inMod.clone(), None)
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok((outDims, outMod, outFieldDomOpt))
}

fn findDomainSubMod(mut subMod: Arc<DAE::SubMod>) -> bool {
    let mut isDomain: bool = false;
    isDomain = (::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ DAE::SubMod { ident: Deref @ "domain", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDomain
}

fn getQualDcr(mut domainSubMod: Arc<DAE::SubMod>, mut inInfo: SourceInfo) -> Result<Arc<DAE::ComponentRef>> {
    let mut dcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    dcr = (::match_deref::match_deref! { match &(domainSubMod.clone()) {
        Deref @ DAE::SubMod { r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DomainLineSegment1D" } } }, .. }, componentRef: cr }, .. }), .. }, ident: Deref @ "domain" } => {
            cr.clone()
        },
        _ => {
            Error::addSourceMessageAndFail(Error::PDEModelica_ERROR.clone(), list![(literal!("The domain type is wrong.\n")).clone()], inInfo.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dcr)
}

fn getNDcr(mut dcr: Arc<DAE::ComponentRef>) -> Result<(i32, Arc<DAE::ComponentRef>)> {
    let mut outN: i32 = 0;
    let mut outCrOpt: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outN, outCrOpt) = (::match_deref::match_deref! { match &(dcr.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr1, .. } => {
            getNDcr(cr1.clone())?
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, .. } => {
            let mut N: i32 = 0;
            let Some(__pa0) = (List::findSome(varLst.clone(), (std::sync::Arc::new(fnptr!(findN, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Option<i32>> + 'static>))) else { bail!("pattern mismatch") };
            N = __pa0.clone();
            (N.clone(), dcr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outN, outCrOpt))
}

fn findN(mut inVar: Arc<DAE::Var>) -> Option<i32> {
    let mut optN: Option<i32> = None;
    optN = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::INTEGER { integer: N }), .. }, name: Deref @ "N", .. } => {
            Some(N.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    optN
}

fn addEach(mut inSubMod: Arc<DAE::SubMod>) -> Result<Arc<DAE::SubMod>> {
    let mut outSubMod: Arc<DAE::SubMod> = Arc::new(<DAE::SubMod as ::std::default::Default>::default());
    let mut ident: ArcStr = arcstr::literal!("");
    let mut finalPrefix: SCode::Final = SCode::Final::FINAL;
    let mut eachPrefix: SCode::Each = SCode::Each::EACH;
    let mut subModLst: Arc<metamodelica::List<Arc<DAE::SubMod>>> = metamodelica::nil();
    let mut binding: Option<DAE::EqMod> = None;
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outSubMod = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ DAE::SubMod { ident, r#mod: Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix: _, subModLst, binding, info } } => Arc::new(DAE::SubMod { ident: (ident.clone()).clone(), r#mod: Arc::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, subModLst: subModLst.clone(), binding: binding.clone(), info: info.clone() }) }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMod)
}

//----end elabField and sub funs
pub fn optAppendField(mut inDomFieldsLst: DomainFieldsLst, mut fieldDomOpt: DomainFieldOpt) -> DomainFieldsLst {
    let mut outDomFieldsLst: DomainFieldsLst = metamodelica::nil();
    outDomFieldsLst = (::match_deref::match_deref! { match &(fieldDomOpt.clone()) {
        None => {
            inDomFieldsLst.clone()
        },
        Some((fieldCr, domainCr)) => {
            let mut found: bool = false;
            (outDomFieldsLst, found) = List::map2Fold(inDomFieldsLst.clone(), (std::sync::Arc::new(optAppendFieldMapFun) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), Arc<DAE::ComponentRef>, Arc<Absyn::ComponentRef>, bool) -> Result<((Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), bool)> + 'static>), domainCr.clone(), fieldCr.clone(), false, metamodelica::nil());
            if !(found.clone()) {
                outDomFieldsLst = cons((domainCr.clone(), list![fieldCr.clone()]), inDomFieldsLst.clone());
            }
            outDomFieldsLst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDomFieldsLst
}

fn optAppendFieldMapFun(mut inDomainFields: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), mut domainCrToAdd: Arc<DAE::ComponentRef>, mut fieldCrToAdd: Arc<Absyn::ComponentRef>, mut inFound: bool) -> Result<((Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), bool)> {
    let mut outDomainFields: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>);
    let mut outFound: bool = false;
    (outDomainFields, outFound) = 'mc: {
        let __mc_input = (inDomainFields.clone(), inFound.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((domainCr, fieldCrLst), false) => {
                    let true = (ComponentReferenceBasics::crefEqual(domainCr.clone(), domainCrToAdd.clone())?) else { bail!("pattern mismatch") };
                    Ok(((domainCr.clone(), cons(fieldCrToAdd.clone(), fieldCrLst.clone())), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDomainFields.clone(), inFound.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDomainFields, outFound))
}

//----end optAppendField and sub funs
pub fn discretizePDE(mut inEQ: Arc<SCode::Equation>, mut inDomFieldLst: DomainFieldsLst, mut inDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut newDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    newDiscretizedEQs = (::match_deref::match_deref! { match &(inEQ.clone()) {
        Deref @ SCode::Equation::EQ_PDE { info, comment, domain: domainCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, expRight: rhs_exp, expLeft: lhs_exp } => {
            let mut N: i32 = 0;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            (N, fieldLst) = getDomNFields(inDomFieldLst.clone(), domainCr.clone(), info.clone())?;
            creatFieldEqs(lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N.clone(), fieldLst.clone(), comment.clone(), info.clone())?
        },
        Deref @ SCode::Equation::EQ_PDE { info, comment, domain: domainCr @ Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "interior", .. } }, expRight: rhs_exp, expLeft: lhs_exp } => {
            let mut domainCr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut N: i32 = 0;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            domainCr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst.clone(), domainCr1.clone(), info.clone())?;
            creatFieldEqs(lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N.clone(), fieldLst.clone(), comment.clone(), info.clone())?
        },
        Deref @ SCode::Equation::EQ_PDE { info, comment, domain: Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "left", .. } }, expRight: rhs_exp, expLeft: lhs_exp } => {
            let mut domainCr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut N: i32 = 0;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            let mut rhs_exp = (*rhs_exp).clone();
            let mut lhs_exp = (*lhs_exp).clone();
            domainCr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst.clone(), domainCr1.clone(), info.clone())?;
            (lhs_exp, _) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 1)?;
            (rhs_exp, _) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), 1)?;
            list![newEQFun(1, lhs_exp.clone(), rhs_exp.clone(), domainCr1.clone(), N.clone(), true, fieldLst.clone(), comment.clone(), info.clone())?]
        },
        Deref @ SCode::Equation::EQ_PDE { info, comment, domain: Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "right", .. } }, expRight: rhs_exp, expLeft: lhs_exp } => {
            let mut domainCr1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut N: i32 = 0;
            let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
            let mut rhs_exp = (*rhs_exp).clone();
            let mut lhs_exp = (*lhs_exp).clone();
            domainCr1 = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst.clone(), domainCr1.clone(), info.clone())?;
            (lhs_exp, _) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), N.clone())?;
            (rhs_exp, _) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, Arc<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, i32) -> Result<(Arc<Absyn::Exp>, i32)> + 'static>), N.clone())?;
            list![newEQFun(N.clone(), lhs_exp.clone(), rhs_exp.clone(), domainCr1.clone(), N.clone(), true, fieldLst.clone(), comment.clone(), info.clone())?]
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            println!("{}", (literal!("Unhandled type of EQ_PDE in discretizePDE\n")).clone());
            bail!("fail");
            metamodelica::nil()
        },
        _ => {
            list![inEQ.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDiscretizedEQs = listAppend(inDiscretizedEQs.clone(), newDiscretizedEQs.clone());
    Ok(outDiscretizedEQs)
}

fn extrapFieldTraverseFun(mut inExp: Arc<Absyn::Exp>, mut inN: i32) -> (Arc<Absyn::Exp>, i32) {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outN: i32 = inN.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::List::Nil }, .. }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: Deref @ "extrapolateField" }, .. } => {
            let mut i: i32 = 0;
            if inN.clone() == 1 {
                i = 1;
            } else {
                i = -1;
            }
            Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: inN.clone() }) }), subscripts.clone()) }) }) }), op: openmodelica_ast::Absyn::Operator::SUB, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: inN.clone() + i.clone() }) }), subscripts.clone()) }) }) })
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outN)
}

fn getDomNFields(mut inDomFieldLst: DomainFieldsLst, mut inDomainCr: Arc<Absyn::ComponentRef>, mut info: SourceInfo) -> Result<(i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)> {
    let mut outN: i32 = 0;
    let mut outFieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(List::findSome(inDomFieldLst.clone(), Arc::new({ let __pe_b1 = inDomainCr.clone(); move |__pe_a0| domNFieldsFindFun(__pe_a0, __pe_b1.clone()) }))) {
            Some((__pa1, __pa2)) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        outN = __pa1.clone();
        outFieldLst = __pa2.clone();
        Ok::<_, anyhow::Error>((outFieldLst.clone(), outN.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outFieldLst = __try0_o0;
            outN = __try0_o1;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![(literal!("There are no fields defined within the domain of this equation.")).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok((outN, outFieldLst))
}

fn domNFieldsFindFun(mut inDomFields: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>), mut inDomainCr: Arc<Absyn::ComponentRef>) -> Result<Option<(i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)>> {
    let mut outOptNFields: Option<(i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>)> = None;
    outOptNFields = 'mc: {
        let __mc_input = inDomFields.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (domainCr, fieldCrLst) => {
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut N: i32 = 0;
                    let true = (absynDAECrefEqualName(inDomainCr.clone(), domainCr.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(domainCr.clone()) {
                        Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varLst = __pa0.clone();
                    let Some(__pa2) = (List::findSome(varLst.clone(), (std::sync::Arc::new(fnptr!(findN, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<Option<i32>> + 'static>))) else { bail!("pattern mismatch") };
                    N = __pa2.clone();
                    Ok(Some((N.clone(), fieldCrLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outOptNFields)
}

fn absynDAECrefEqualName(mut domainCr1: Arc<Absyn::ComponentRef>, mut domainCr2: Arc<DAE::ComponentRef>) -> bool {
    let mut equal: bool = false;
    let mut name1: ArcStr = arcstr::literal!("");
    let mut name2: ArcStr = arcstr::literal!("");
    equal = (::match_deref::match_deref! { match &((domainCr1.clone(), domainCr2.clone())) {
        (Deref @ Absyn::ComponentRef::CREF_IDENT { name: name1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: name2, .. }) if (stringEqual((name1.clone()).clone(), (name2.clone()).clone())) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

fn extrapolateFieldEq(mut isRight: bool, mut fieldCr: Arc<Absyn::ComponentRef>, mut domainCr: Arc<Absyn::ComponentRef>, mut N: i32, mut comment: Arc<SCode::Comment>, mut info: SourceInfo, mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>) -> Result<Arc<SCode::Equation>> {
    let mut outEQ: Arc<SCode::Equation>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    let mut i1: i32 = 1;
    let mut i2: i32 = 2;
    let mut i3: i32 = 3;
    if List::isMemberOnTrue(fieldCr.clone(), fieldLst.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqual, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>)) {
        (name, subscripts) = (::match_deref::match_deref! { match &(fieldCr.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts, name } => (name.clone(), subscripts.clone()),
        _ => bail!("match: no arm matched"),
    } });
        if isRight.clone() {
            i1 = N.clone();
            i2 = N.clone() - 1;
            i3 = N.clone() - 2;
        }
        outEQ = Arc::new(SCode::Equation::EQ_EQUALS { expLeft: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i1.clone() }) }), subscripts.clone()) }) }), expRight: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i2.clone() }) }), subscripts.clone()) }) }) }), op: openmodelica_ast::Absyn::Operator::SUB, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i3.clone() }) }), subscripts.clone()) }) }) }), comment: comment.clone(), info: info.clone() });
    } else {
        bail!("fail");
    }
    Ok(outEQ)
}

fn creatFieldEqs(mut lhs_exp: Arc<Absyn::Exp>, mut rhs_exp: Arc<Absyn::Exp>, mut domainCr: Arc<Absyn::ComponentRef>, mut N: i32, mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outDiscretizedEQs: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
    let mut bl: bool = false;
    let mut br: bool = false;
    (_, bl) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(hasPderTraverseFun, Arc<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>), false)?;
    (_, br) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(hasPderTraverseFun, Arc<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, bool) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>), false)?;
    outDiscretizedEQs = (match (bl.clone(), br.clone()) {
        (false, false) => {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut i in (1..=N.clone()).into_iter() {
            let __x = newEQFun(i.clone(), lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N.clone(), false, fieldLst.clone(), comment.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        _ => {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut i in (1..=N.clone()).into_iter() {
            let __x = newEQFun(i.clone(), lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N.clone(), false, fieldLst.clone(), comment.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
    });
    Ok(outDiscretizedEQs)
}

fn hasPderTraverseFun(mut inExp: Arc<Absyn::Exp>, mut inHasPder: bool) -> (Arc<Absyn::Exp>, bool) {
    let mut outExp: Arc<Absyn::Exp> = inExp.clone();
    let mut outHasPder: bool = false;
    outHasPder = (::match_deref::match_deref! { match &((inExp.clone(), inHasPder.clone())) {
        (_, true) => true,
        (Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, .. }, _) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHasPder)
}

fn newEQFun(mut i: i32, mut inLhs_exp: Arc<Absyn::Exp>, mut inRhs_exp: Arc<Absyn::Exp>, mut domainCr: Arc<Absyn::ComponentRef>, mut N: i32, mut isBC: bool, mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, mut comment: Arc<SCode::Comment>, mut info: SourceInfo) -> Result<Arc<SCode::Equation>> {
    let mut outEQ: Arc<SCode::Equation>;
    let mut outLhs_exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outRhs_exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (outLhs_exp, _) = AbsynUtil::traverseExpTopDown(inLhs_exp.clone(), (std::sync::Arc::new(discretizeTraverseFun) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool)) -> Result<(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool))> + 'static>), (i.clone(), fieldLst.clone(), domainCr.clone(), info.clone(), false, N.clone(), isBC.clone()))?;
    (outRhs_exp, _) = AbsynUtil::traverseExpTopDown(inRhs_exp.clone(), (std::sync::Arc::new(discretizeTraverseFun) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool)) -> Result<(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool))> + 'static>), (i.clone(), fieldLst.clone(), domainCr.clone(), info.clone(), false, N.clone(), isBC.clone()))?;
    outEQ = Arc::new(SCode::Equation::EQ_EQUALS { expLeft: outLhs_exp.clone(), expRight: outRhs_exp.clone(), comment: comment.clone(), info: info.clone() });
    Ok(outEQ)
}

fn discretizeTraverseFun(mut inExp: Arc<Absyn::Exp>, mut inTup: (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool)) -> Result<(Arc<Absyn::Exp>, (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool))> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut outTup: (i32, Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, Arc<Absyn::ComponentRef>, SourceInfo, bool, i32, bool);
    let mut i: i32 = 0;
    let mut N: i32 = 0;
    let mut fieldLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> = metamodelica::nil();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut skip: bool = false;
    let mut failVar: bool = false;
    let mut isBC: bool = false;
    let mut domainCr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut domName: ArcStr = arcstr::literal!("");
    failVar = false;
    (i, fieldLst, domainCr, info, skip, N, isBC) = inTup.clone();
    let __pa0 = ::match_deref::match_deref! { match &(domainCr.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    domName = __pa0.clone();
    if skip.clone() {
        outExp = inExp.clone();
        outTup = inTup.clone();
        return Ok((outExp, outTup));
    }
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: Deref @ "x" }, subscripts: Deref @ metamodelica::List::Nil, name: domName } } => {
                    Ok(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { subscripts: list![Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() }) })], name: (literal!("x")).clone() }), subscripts: metamodelica::nil(), name: (domName.clone()).clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } } => {
                    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let true = (List::isMemberOnTrue(fieldCr.clone(), fieldLst.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqual, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    exp = if (isBC.clone() && i.clone() == 1) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), subscripts: subscripts.clone() }) })} else if (isBC.clone() && i.clone() == N.clone()) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() }) }), subscripts.clone()) }) })};
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::List::Nil } }, argNames: _ }, typeVars: Deref @ metamodelica::List::Nil } => {
                    let mut leftVar: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut rightVar: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut failVar: bool = failVar.clone();
                    if !(List::isMemberOnTrue(fieldCr.clone(), fieldLst.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqual, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))) {
                        failVar = true;
                        Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' has different domain than the equation or is not a field.")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                    leftVar = if (i.clone() == 1) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() - 1 }) }), subscripts.clone()) }) })};
                    rightVar = if (i.clone() == N.clone()) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() + 1 }) }), subscripts.clone()) }) })};
                    Ok(Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: rightVar.clone(), op: openmodelica_ast::Absyn::Operator::SUB, exp2: leftVar.clone() }), op: openmodelica_ast::Absyn::Operator::DIV, exp2: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (domName.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("dx")).clone(), subscripts: metamodelica::nil() }) }) }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::List::Nil } } }, argNames: _ }, typeVars: Deref @ metamodelica::List::Nil } => {
                    let mut leftVar: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut actualVar: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut rightVar: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut failVar: bool = failVar.clone();
                    if !(List::isMemberOnTrue(fieldCr.clone(), fieldLst.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefEqual, Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>, Arc<Absyn::ComponentRef>) -> Result<bool> + 'static>))) {
                        failVar = true;
                        Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("' has different domain than the equation or is not a field.")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                    leftVar = if (i.clone() == 1) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostL")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() - 1 }) }), subscripts.clone()) }) })};
                    actualVar = Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() }) }), subscripts.clone()) }) });
                    rightVar = if (i.clone() == N.clone()) {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (stringAppend((name.clone()).clone(), (literal!("$ghostR")).clone())).clone(), subscripts: subscripts.clone() }) })} else {Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() + 1 }) }), subscripts.clone()) }) })};
                    Ok(Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::BINARY { exp1: leftVar.clone(), op: openmodelica_ast::Absyn::Operator::SUB, exp2: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: actualVar.clone() }) }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: rightVar.clone() }), op: openmodelica_ast::Absyn::Operator::DIV, exp2: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (domName.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("dx")).clone(), subscripts: metamodelica::nil() }) }) }), op: openmodelica_ast::Absyn::Operator::POW, exp2: Arc::new(Absyn::Exp::INTEGER { value: 2 }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: _ }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, argNames: _ }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, .. } => {
                    Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![(literal!("You are differentiating with respect to variable that is not a coordinate.")).clone()], info.clone())?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, argNames: _ }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::List::Nil }, .. } => {
                    Error::addSourceMessageAndFail(Error::COMPILER_ERROR.clone(), list![(literal!("Unsupported partial derivative.")).clone()], info.clone())?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if failVar.clone() {
        bail!("fail");
    }
    outTup = (i.clone(), fieldLst.clone(), domainCr.clone(), info.clone(), skip.clone(), N.clone(), isBC.clone());
    Ok((outExp, outTup))
}

fn findDomF<T: Clone + 'static>(mut inTup: (ArcStr, T), mut name: ArcStr) -> bool {
    let mut found: bool = false;
    found = (match inTup.clone() {
        (mut nameLoc, _) if (stringEqual((nameLoc.clone()).clone(), (name.clone()).clone())) => {
            true
        },
        _ => {
            false
        },
    });
    found
}

/*
public function findDomains
  input SCode.Element el;
  input list<tuple<String,Integer>> domainLstIn;
  output list<tuple<String,Integer>> domainLstOut;
algorithm
//TODO: rewrite to use instantiated domain elements
  domainLstOut := match el
    local
      String name;
      list<SCode.SubMod> subModLst;
      Integer N;
    case SCode.COMPONENT(typeSpec=Absyn.TPATH(path=Absyn.IDENT(name="DomainLineSegment1D")), name = name,
      modifications = SCode.MOD(subModLst = subModLst))
      then
        (name,findDomains1(subModLst))::domainLstIn;
      else
      domainLstIn;
  end match;
end findDomains;


protected function findDomains1
  input list<SCode.SubMod> subModLst;
  output Integer N;
algorithm
  try
    N := match List.find(subModLst,findDomains2)
    local
      Integer n;
    case SCode.NAMEMOD("N",SCode.MOD(binding = SOME(Absyn.INTEGER(n))))
    then
      n;
    end match;
  else
    print("\nError: Variable N not found in the domain.\n");
    fail();
  end try;
end findDomains1;

protected function findDomains2
  input SCode.SubMod subMod;
  output Boolean found;
algorithm
  found := match subMod
    case SCode.NAMEMOD(ident = "N")
      then
        true;
      else
        false;
  end match;
end findDomains2;
*/
