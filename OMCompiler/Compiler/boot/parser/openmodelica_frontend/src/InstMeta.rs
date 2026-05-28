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

use crate::FCore;
use crate::Lookup;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Flags;

pub fn fixUniontype(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inState: ClassInf::State, mut inClassDef: Arc<SCode::ClassDef>) -> Result<(FCore::Cache, Option<Arc<DAE::Type>>)> {
    let mut cache: FCore::Cache = inCache.clone();
    let mut outType: Option<Arc<DAE::Type>> = None;
    outType = (::match_deref::match_deref! { match &((inState.clone(), inClassDef.clone())) {
        (ClassInf::State::META_UNIONTYPE { typeVars, .. }, Deref @ SCode::ClassDef::PARTS { .. }) => {
            let mut p: Arc<Absyn::Path>;
            let mut p2: Arc<Absyn::Path>;
            let mut utPathOfRestriction: Arc<Absyn::Path>;
            let mut utPath: Arc<Absyn::Path>;
            let mut isSingleton: bool = false;
            let mut singletonType: Arc<DAE::EvaluateSingletonType> = Arc::new(DAE::EvaluateSingletonType::NOT_SINGLETON);
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut typeVarsTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            utPath = var_field!(inState.path, ClassInf::State::META_UNIONTYPE).clone();
            p = AbsynUtil::makeFullyQualified(var_field!(inState.path, ClassInf::State::META_UNIONTYPE).clone());
            names = SCodeUtil::elementNames(({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (var_field!((*inClassDef).elementLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { name: utPathOfRestriction, .. }, .. } => AbsynUtil::pathSuffixOf(utPathOfRestriction.clone(), utPath.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            paths = ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut n in (names.clone()).into_iter().cloned() {
            let __x = AbsynUtil::suffixPath(p.clone(), (n.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            isSingleton = (paths.clone().len() as i32) == 1;
            if isSingleton.clone() {
                p2 = (paths.clone()).get(1)?;
                singletonType = Arc::new(DAE::EvaluateSingletonType::EVAL_SINGLETON_TYPE_FUNCTION { fun: Arc::new({ let __pe_b0 = arrayCreate(1, (cache.clone(), inEnv.clone(), p2.clone(), None)); move || fixUniontype2(__pe_b0.clone()) }) });
            } else {
                singletonType = Arc::new(openmodelica_frontend_types::DAE::EvaluateSingletonType::NOT_SINGLETON);
            }
            typeVarsTypes = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut tv in (typeVars.clone()).into_iter().cloned() {
            let __x = Arc::new(DAE::Type::T_METAPOLYMORPHIC { name: (tv.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Some(Arc::new(DAE::Type::T_METAUNIONTYPE { paths: paths.clone(), typeVars: typeVarsTypes.clone(), knownSingleton: isSingleton.clone(), singletonType: singletonType.clone(), path: p.clone() }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, outType))
}

fn fixUniontype2(mut arr: metamodelica::Array<(FCore::Cache, FCore::Graph, Arc<Absyn::Path>, Option<Arc<DAE::Type>>)>) -> Result<Arc<DAE::Type>> {
    let mut singletonType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env: FCore::Graph;
    let mut p: Arc<Absyn::Path>;
    let mut ot: Option<Arc<DAE::Type>> = None;
    (cache, env, p, ot) = arr.clone().borrow()[(1-1) as usize].clone();
    if isNone(ot.clone()) {
        (_, singletonType, _) = Lookup::lookupType(cache.clone(), env.clone(), p.clone(), Some(metamodelica::sourceInfo!()))?;
        {let _arr = arr.clone(); _arr.borrow_mut()[(1-1) as usize] = (cache.clone(), env.clone(), p.clone(), Some(singletonType.clone())); _arr};
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(ot.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        singletonType = __pa0.clone();
    }
    Ok(singletonType)
}

pub fn checkArrayType(mut inType: Arc<DAE::Type>) -> Result<()> {
    let mut el_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    el_ty = Types::arrayElementType(inType.clone());
    let false = (!(Types::isString(el_ty.clone())) && Types::isBoxedType(el_ty.clone()) || Flags::isSet(Flags::RML.clone())?) else { bail!("pattern mismatch") };
    Ok(())
}

