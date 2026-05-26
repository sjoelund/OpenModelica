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

use crate::BaseModelica;
use crate::NFAlgorithm as Algorithm;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFStatement as Statement;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub enum NFSections {
    SECTIONS {
        equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
        initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
        algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>,
        initialAlgorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>,
    },
    EXTERNAL {
        name: ArcStr,
        args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
        outputRef: Arc<ComponentRef::NFComponentRef>,
        language: ArcStr,
        ann: Option<Arc<SCode::Annotation>>,
        explicit: bool,
        info: SourceInfo,
    },
    EMPTY,
}
impl Default for NFSections {
    fn default() -> Self { Self::EMPTY }
}
pub use self::NFSections::{SECTIONS,EXTERNAL,EMPTY};
pub fn new(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut initialAlgorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>) -> Arc<NFSections> {
    let mut sections: Arc<NFSections> = Arc::new(NFSections::EMPTY);
    if equations.clone().is_empty() && initialEquations.clone().is_empty() && algorithms.clone().is_empty() && initialAlgorithms.clone().is_empty() {
        sections = Arc::new(crate::NFSections::EMPTY);
    } else {
        sections = Arc::new(NFSections::SECTIONS { equations: equations.clone(), initialEquations: initialEquations.clone(), algorithms: algorithms.clone(), initialAlgorithms: initialAlgorithms.clone() });
    }
    sections
}

pub fn equations(mut sections: Arc<NFSections>) -> Arc<metamodelica::List<Arc<Equation::NFEquation>>> {
    let mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    equations = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => var_field!((*sections).equations, NFSections::SECTIONS).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equations
}

pub fn prepend(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut initialAlgorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut sections: Arc<NFSections>) -> Arc<NFSections> {
    let mut sections: Arc<NFSections> = sections;
    sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => Arc::new(NFSections::SECTIONS { equations: listAppend(equations.clone(), var_field!((*sections).equations, NFSections::SECTIONS).clone()), initialEquations: listAppend(initialEquations.clone(), var_field!((*sections).initialEquations, NFSections::SECTIONS).clone()), algorithms: listAppend(algorithms.clone(), var_field!((*sections).algorithms, NFSections::SECTIONS).clone()), initialAlgorithms: listAppend(initialAlgorithms.clone(), var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone()) }),
        _ => Arc::new(NFSections::SECTIONS { equations: equations.clone(), initialEquations: initialEquations.clone(), algorithms: algorithms.clone(), initialAlgorithms: initialAlgorithms.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sections
}

pub fn prependEquation(mut eq: Arc<Equation::NFEquation>, mut sections: Arc<NFSections>, mut isInitial: bool) -> Result<Arc<NFSections>> {
    let mut sections: Arc<NFSections> = sections;
    sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            if isInitial.clone() {
                assign_variant_field!(sections => NFSections::SECTIONS; initialEquations = cons(eq.clone(), var_field!((*sections).initialEquations, NFSections::SECTIONS).clone()));
            } else {
                assign_variant_field!(sections => NFSections::SECTIONS; equations = cons(eq.clone(), var_field!((*sections).equations, NFSections::SECTIONS).clone()));
            }
            sections.clone()
        },
        Deref @ EMPTY { .. } => if (isInitial.clone()) {Arc::new(NFSections::SECTIONS { equations: metamodelica::nil(), initialEquations: list![eq.clone()], algorithms: metamodelica::nil(), initialAlgorithms: metamodelica::nil() })} else {Arc::new(NFSections::SECTIONS { equations: list![eq.clone()], initialEquations: metamodelica::nil(), algorithms: metamodelica::nil(), initialAlgorithms: metamodelica::nil() })},
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSections.prependEquation")); __mm_s.push_str(&*literal!(" got invalid Sections to prepend equation to")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sections)
}

pub fn prependAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut sections: Arc<NFSections>, mut isInitial: bool) -> Result<Arc<NFSections>> {
    let mut sections: Arc<NFSections> = sections;
    sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            if isInitial.clone() {
                assign_variant_field!(sections => NFSections::SECTIONS; initialAlgorithms = cons(alg.clone(), var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone()));
            } else {
                assign_variant_field!(sections => NFSections::SECTIONS; algorithms = cons(alg.clone(), var_field!((*sections).algorithms, NFSections::SECTIONS).clone()));
            }
            sections.clone()
        },
        Deref @ EMPTY { .. } => if (isInitial.clone()) {Arc::new(NFSections::SECTIONS { equations: metamodelica::nil(), initialEquations: metamodelica::nil(), algorithms: metamodelica::nil(), initialAlgorithms: list![alg.clone()] })} else {Arc::new(NFSections::SECTIONS { equations: metamodelica::nil(), initialEquations: metamodelica::nil(), algorithms: list![alg.clone()], initialAlgorithms: metamodelica::nil() })},
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSections.prependAlgorithm")); __mm_s.push_str(&*literal!(" got invalid Sections to prepend algorithm to")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sections)
}

pub fn append(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut initialAlgorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut sections: Arc<NFSections>) -> Arc<NFSections> {
    let mut sections: Arc<NFSections> = sections;
    sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => Arc::new(NFSections::SECTIONS { equations: listAppend(var_field!((*sections).equations, NFSections::SECTIONS).clone(), equations.clone()), initialEquations: listAppend(var_field!((*sections).initialEquations, NFSections::SECTIONS).clone(), initialEquations.clone()), algorithms: listAppend(var_field!((*sections).algorithms, NFSections::SECTIONS).clone(), algorithms.clone()), initialAlgorithms: listAppend(var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone(), initialAlgorithms.clone()) }),
        _ => Arc::new(NFSections::SECTIONS { equations: equations.clone(), initialEquations: initialEquations.clone(), algorithms: algorithms.clone(), initialAlgorithms: initialAlgorithms.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sections
}

pub fn join(mut sections1: Arc<NFSections>, mut sections2: Arc<NFSections>) -> Result<Arc<NFSections>> {
    let mut sections: Arc<NFSections> = Arc::new(NFSections::EMPTY);
    sections = (::match_deref::match_deref! { match &((sections1.clone(), sections2.clone())) {
        (Deref @ EMPTY { .. }, _) => sections2.clone(),
        (_, Deref @ EMPTY { .. }) => sections1.clone(),
        (Deref @ SECTIONS { .. }, Deref @ SECTIONS { .. }) => Arc::new(NFSections::SECTIONS { equations: listAppend(var_field!((*sections1).equations, NFSections::SECTIONS).clone(), var_field!((*sections2).equations, NFSections::SECTIONS).clone()), initialEquations: listAppend(var_field!((*sections1).initialEquations, NFSections::SECTIONS).clone(), var_field!((*sections2).initialEquations, NFSections::SECTIONS).clone()), algorithms: listAppend(var_field!((*sections1).algorithms, NFSections::SECTIONS).clone(), var_field!((*sections2).algorithms, NFSections::SECTIONS).clone()), initialAlgorithms: listAppend(var_field!((*sections1).initialAlgorithms, NFSections::SECTIONS).clone(), var_field!((*sections2).initialAlgorithms, NFSections::SECTIONS).clone()) }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(sections)
}

pub fn map(mut sections: Arc<NFSections>, mut eqFn: Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>, mut algFn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>, mut ieqFn: Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>, mut ialgFn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>) -> Arc<NFSections> {
    pub type EquationFn = fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>>;

    pub type AlgorithmFn = fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>>;

    pub fn eqId(mut eq: Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation> {
        let mut eq: Arc<Equation::NFEquation> = eq;
        eq
    }

    pub fn algId(mut alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
        let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
        alg
    }

    let mut sections: Arc<NFSections> = sections;
    let mut eq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ieq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut alg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut ialg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            eq = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*sections).equations, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = eqFn(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ieq = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*sections).initialEquations, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = ieqFn(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            alg = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (var_field!((*sections).algorithms, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = algFn(a.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ialg = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = ialgFn(a.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            sections = Arc::new(NFSections::SECTIONS { equations: eq.clone(), initialEquations: ieq.clone(), algorithms: alg.clone(), initialAlgorithms: ialg.clone() });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sections
}

pub fn map1<ArgT: Clone + 'static>(mut sections: Arc<NFSections>, mut arg: ArgT, mut eqFn: Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, ArgT) -> Result<Arc<Equation::NFEquation>> + 'static>, mut algFn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, ArgT) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>, mut ieqFn: Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, ArgT) -> Result<Arc<Equation::NFEquation>> + 'static>, mut ialgFn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, ArgT) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>) -> Arc<NFSections> {
    pub type EquationFn<ArgT: Clone> = fn(Arc<Equation::NFEquation>, ArgT) -> Result<Arc<Equation::NFEquation>>;

    pub type AlgorithmFn<ArgT: Clone> = fn(Arc<Algorithm::NFAlgorithm>, ArgT) -> Result<Arc<Algorithm::NFAlgorithm>>;

    let mut sections: Arc<NFSections> = sections;
    let mut eq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ieq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut alg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut ialg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            eq = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*sections).equations, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = eqFn(e.clone(), arg.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ieq = {
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*sections).initialEquations, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = ieqFn(e.clone(), arg.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            alg = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (var_field!((*sections).algorithms, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = algFn(a.clone(), arg.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ialg = {
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone()).into_iter().cloned() {
            let __x = ialgFn(a.clone(), arg.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            sections = Arc::new(NFSections::SECTIONS { equations: eq.clone(), initialEquations: ieq.clone(), algorithms: alg.clone(), initialAlgorithms: ialg.clone() });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sections
}

pub fn mapExp(mut sections: Arc<NFSections>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFSections> {
    pub type MapFn = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut sections: Arc<NFSections> = sections;
    let mut eq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ieq: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut alg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    let mut ialg: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
    sections = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            eq = Equation::mapExpList(var_field!((*sections).equations, NFSections::SECTIONS).clone(), mapFn.clone());
            ieq = Equation::mapExpList(var_field!((*sections).initialEquations, NFSections::SECTIONS).clone(), mapFn.clone());
            alg = Algorithm::mapExpList(var_field!((*sections).algorithms, NFSections::SECTIONS).clone(), mapFn.clone());
            ialg = Algorithm::mapExpList(var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone(), mapFn.clone());
            Arc::new(NFSections::SECTIONS { equations: eq.clone(), initialEquations: ieq.clone(), algorithms: alg.clone(), initialAlgorithms: ialg.clone() })
        },
        Deref @ EXTERNAL { .. } => {
            assign_variant_field!(sections => NFSections::EXTERNAL; args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*sections).args, NFSections::EXTERNAL).clone()).into_iter().cloned() {
            let __x = mapFn(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            sections.clone()
        },
        _ => sections.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sections
}

pub fn foldExp<ArgT: Clone + 'static>(mut sections: Arc<NFSections>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFn<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    arg = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            arg = Equation::foldExpList(var_field!((*sections).equations, NFSections::SECTIONS).clone(), foldFn.clone(), arg.clone())?;
            arg = Equation::foldExpList(var_field!((*sections).initialEquations, NFSections::SECTIONS).clone(), foldFn.clone(), arg.clone())?;
            arg = Algorithm::foldExpList(var_field!((*sections).algorithms, NFSections::SECTIONS).clone(), foldFn.clone(), arg.clone())?;
            arg = Algorithm::foldExpList(var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone(), foldFn.clone(), arg.clone())?;
            arg.clone()
        },
        Deref @ EXTERNAL { .. } => List::fold(var_field!((*sections).args, NFSections::EXTERNAL).clone(), foldFn.clone(), arg.clone()),
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub fn apply(mut sections: Arc<NFSections>, mut eqFn: Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<()> + 'static>, mut algFn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<()> + 'static>, mut ieqFn: Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<()> + 'static>, mut ialgFn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<()> + 'static>) -> () {
    pub type EquationFn = fn(Arc<Equation::NFEquation>) -> Result<()>;

    pub type AlgorithmFn = fn(Arc<Algorithm::NFAlgorithm>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            for mut eq in &*var_field!((*sections).equations, NFSections::SECTIONS).clone() {
                let mut eq = eq.clone();
                eqFn(eq.clone()).unwrap();
            }
            for mut ieq in &*var_field!((*sections).initialEquations, NFSections::SECTIONS).clone() {
                let mut ieq = ieq.clone();
                ieqFn(ieq.clone()).unwrap();
            }
            for mut alg in &*var_field!((*sections).algorithms, NFSections::SECTIONS).clone() {
                let mut alg = alg.clone();
                algFn(alg.clone()).unwrap();
            }
            for mut ialg in &*var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone() {
                let mut ialg = ialg.clone();
                ialgFn(ialg.clone()).unwrap();
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn isEmpty(mut sections: Arc<NFSections>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn toStream(mut sections: Arc<NFSections>, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            for mut alg in &*var_field!((*sections).algorithms, NFSections::SECTIONS).clone() {
                let mut alg = alg.clone();
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = IOStream::append(s.clone(), (literal!("algorithm\n")).clone())?;
                s = Statement::toStreamList(alg.statements.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
            }
            ()
        },
        Deref @ EXTERNAL { .. } => {
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("external \"")).clone())?;
            s = IOStream::append(s.clone(), (var_field!((*sections).language, NFSections::EXTERNAL).clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("\"")).clone())?;
            if var_field!((*sections).explicit, NFSections::EXTERNAL).clone() {
                if !(ComponentRef::isEmpty(var_field!((*sections).outputRef, NFSections::EXTERNAL).clone())) {
                    s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
                    s = IOStream::append(s.clone(), (ComponentRef::toString(var_field!((*sections).outputRef, NFSections::EXTERNAL).clone())?).clone())?;
                    s = IOStream::append(s.clone(), (literal!(" =")).clone())?;
                }
                s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
                s = IOStream::append(s.clone(), (var_field!((*sections).name, NFSections::EXTERNAL).clone()).clone())?;
                s = IOStream::append(s.clone(), (literal!("(")).clone())?;
                s = IOStream::append(s.clone(), stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*sections).args, NFSections::EXTERNAL).clone()).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone()))?;
                s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            }
            if isSome(var_field!((*sections).ann, NFSections::EXTERNAL).clone()) {
                s = IOStream::append(s.clone(), (DAEDumpTypes::dumpCompAnnotationStr(Some(Arc::new(SCode::Comment { annotation_: var_field!((*sections).ann, NFSections::EXTERNAL).clone(), comment: None })))?).clone())?;
            }
            s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn toFlatStream(mut sections: Arc<NFSections>, mut scopeName: Arc<Absyn::Path>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut ann: Arc<SCode::Annotation>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut modLib: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut modInc: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut modLibDir: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut modIncDir: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let () = (::match_deref::match_deref! { match &(sections.clone()) {
        Deref @ SECTIONS { .. } => {
            for mut alg in &*var_field!((*sections).algorithms, NFSections::SECTIONS).clone() {
                let mut alg = alg.clone();
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = IOStream::append(s.clone(), (literal!("algorithm\n")).clone())?;
                s = Statement::toFlatStreamList(alg.statements.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s.clone())?;
            }
            ()
        },
        Deref @ EXTERNAL { .. } => {
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("external \"")).clone())?;
            s = IOStream::append(s.clone(), (var_field!((*sections).language, NFSections::EXTERNAL).clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("\"")).clone())?;
            if var_field!((*sections).explicit, NFSections::EXTERNAL).clone() {
                if !(ComponentRef::isEmpty(var_field!((*sections).outputRef, NFSections::EXTERNAL).clone())) {
                    s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
                    s = IOStream::append(s.clone(), (ComponentRef::toFlatString(var_field!((*sections).outputRef, NFSections::EXTERNAL).clone(), format.clone())?).clone())?;
                    s = IOStream::append(s.clone(), (literal!(" =")).clone())?;
                }
                s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
                s = IOStream::append(s.clone(), (var_field!((*sections).name, NFSections::EXTERNAL).clone()).clone())?;
                s = IOStream::append(s.clone(), (literal!("(")).clone())?;
                s = IOStream::append(s.clone(), stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (var_field!((*sections).args, NFSections::EXTERNAL).clone()).into_iter().cloned() {
            let __x = Expression::toFlatString(e.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone()))?;
                s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            }
            if isSome(var_field!((*sections).ann, NFSections::EXTERNAL).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*sections).ann, NFSections::EXTERNAL).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                ann = __pa0.clone();
                r#mod = ann.modification.clone();
                modLib = SCodeUtil::filterSubMods(r#mod.clone(), Arc::new({ let __pe_b1 = list![(literal!("Library")).clone()]; move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(__pe_a0, __pe_b1.clone())) }));
                modInc = SCodeUtil::filterSubMods(r#mod.clone(), Arc::new({ let __pe_b1 = list![(literal!("Include")).clone()]; move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(__pe_a0, __pe_b1.clone())) }));
                if SCodeUtil::isEmptyMod(modLib.clone()) {
                    modLibDir = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD);
                } else {
                    modLibDir = SCodeUtil::filterSubMods(r#mod.clone(), Arc::new({ let __pe_b1 = list![(literal!("LibraryDirectory")).clone()]; move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(__pe_a0, __pe_b1.clone())) }));
                    if SCodeUtil::isEmptyMod(modLibDir.clone()) {
                        modLibDir = Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(SCode::SubMod { ident: (literal!("LibraryDirectory")).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(scopeName.clone())?); __mm_s.push_str(&*literal!("/Resources/Library")); ArcStr::from(__mm_s) }).clone() })), comment: None, info: Error::dummyInfo.clone() }) })], binding: None, comment: None, info: Error::dummyInfo.clone() });
                    }
                }
                if SCodeUtil::isEmptyMod(modInc.clone()) {
                    modIncDir = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD);
                } else {
                    modIncDir = SCodeUtil::filterSubMods(r#mod.clone(), Arc::new({ let __pe_b1 = list![(literal!("IncludeDirectory")).clone()]; move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(__pe_a0, __pe_b1.clone())) }));
                    if SCodeUtil::isEmptyMod(modLibDir.clone()) {
                        modLibDir = Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![Arc::new(SCode::SubMod { ident: (literal!("IncludeDirectory")).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(scopeName.clone())?); __mm_s.push_str(&*literal!("/Resources/Include")); ArcStr::from(__mm_s) }).clone() })), comment: None, info: Error::dummyInfo.clone() }) })], binding: None, comment: None, info: Error::dummyInfo.clone() });
                    }
                }
                assign_field!(ann.modification = SCodeUtil::mergeSCodeMods(SCodeUtil::mergeSCodeMods(modLib.clone(), modLibDir.clone())?, SCodeUtil::mergeSCodeMods(modInc.clone(), modIncDir.clone())?)?);
                s = IOStream::append(s.clone(), (SCodeDump::printAnnotationStr(Arc::new(SCode::Comment { annotation_: Some(ann.clone()), comment: None }), SCodeDump::defaultOptions.clone())?).clone())?;
            }
            s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}


