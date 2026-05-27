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
use crate::FGraph;
use crate::FResolve;
use openmodelica_ast::Absyn;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// public imports
pub type Name = ArcStr;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = FCore::Node;

pub type Data = FCore::Data;

pub type Kind = FCore::Kind;

pub type Ref = metamodelica::Array<FCore::Node>;

pub type Refs = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Children = Arc<FCore::RefTree::Tree>;

pub type Parents = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type Scope = Arc<metamodelica::List<metamodelica::Array<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Msg = Option<SourceInfo>;

pub fn path(mut inGraph: Graph, mut inPath: Arc<Absyn::Path>) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (::match_deref::match_deref! { match &((inGraph.clone(), inPath.clone())) {
        (g, _) => {
            let mut r: Ref;
            let mut t: Ref;
            t = FGraph::top(g.clone())?;
            r = t.clone();
            (g.clone(), r.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outGraph, outRef))
}

pub fn all(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph.clone() {
        mut g => {
            let mut lst: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            lst = metamodelica::nil();
            System::startTimer();
            g = FResolve::ext(FGraph::top(g.clone())?, g.clone())?;
            System::stopTimer();
            lst = List::consr(lst.clone(), System::getTimerIntervalTime());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Extends:        ")); __mm_s.push_str(&*realString(listHead(lst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::startTimer();
            g = FResolve::derived(FGraph::top(g.clone())?, g.clone())?;
            System::stopTimer();
            lst = List::consr(lst.clone(), System::getTimerIntervalTime());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Derived:        ")); __mm_s.push_str(&*realString(listHead(lst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::startTimer();
            g = FResolve::cc(FGraph::top(g.clone())?, g.clone())?;
            System::stopTimer();
            lst = List::consr(lst.clone(), System::getTimerIntervalTime());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ConstrainedBy:  ")); __mm_s.push_str(&*realString(listHead(lst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::startTimer();
            g = FResolve::clsext(FGraph::top(g.clone())?, g.clone())?;
            System::stopTimer();
            lst = List::consr(lst.clone(), System::getTimerIntervalTime());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ClassExtends:   ")); __mm_s.push_str(&*realString(listHead(lst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::startTimer();
            g = FResolve::ty(FGraph::top(g.clone())?, g.clone())?;
            System::stopTimer();
            lst = List::consr(lst.clone(), System::getTimerIntervalTime());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentTypes: ")); __mm_s.push_str(&*realString(listHead(lst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::startTimer();
            g = FResolve::cr(FGraph::top(g.clone())?, g.clone())?;
            System::stopTimer();
            lst = List::consr(lst.clone(), System::getTimerIntervalTime());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Comp Refs:      ")); __mm_s.push_str(&*realString(listHead(lst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            System::startTimer();
            g = FResolve::r#mod(FGraph::top(g.clone())?, g.clone())?;
            System::stopTimer();
            lst = List::consr(lst.clone(), System::getTimerIntervalTime());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Modifiers:      ")); __mm_s.push_str(&*realString(listHead(lst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FExpand.all:    ")); __mm_s.push_str(&*realString(List::fold(lst.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            g.clone()
        },
    });
    Ok(outGraph)
}

