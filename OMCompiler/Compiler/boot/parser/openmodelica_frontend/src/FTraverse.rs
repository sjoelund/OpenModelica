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
use crate::FNode;
use crate::FVisit;
use openmodelica_ast::Absyn;

// public imports
// protected imports
/// An identifier is just a string
pub type Ident = ArcStr;

pub type Import = Absyn::Import;

pub type Node = FCore::Node;

pub type Ref = metamodelica::Array<FCore::Node>;

pub type Data = FCore::Data;

pub type Visited = FCore::Visited;

pub type Graph = FCore::Graph;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WalkOptions {
    /// breadth first search
    BFS,
    /// depth first search
    DFS,
}
pub use self::WalkOptions::{BFS,DFS};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisitOptions {
    /// mark node as visited and report an error if already visited
    VISIT,
    /// do not mark as visited
    NO_VISIT,
}
pub use self::VisitOptions::{VISIT,NO_VISIT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Options {
    NO_OPTIONS,
    OPTIONS {
        ws: WalkOptions,
        vs: VisitOptions,
    },
}
pub use self::Options::{NO_OPTIONS,OPTIONS};

pub fn walk<Extra: Clone + 'static>(mut inGraph: Graph, mut inWalker: Arc<dyn ::std::ops::Fn((FCore::Graph, metamodelica::Array<FCore::Node>, Extra)) -> Result<(FCore::Graph, metamodelica::Array<FCore::Node>, Extra)> + 'static>, mut inExtra: Extra, mut inOptions: Options) -> (Graph, Extra) {
    pub type Walker<Extra: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn((FCore::Graph, metamodelica::Array<FCore::Node>, Extra)) -> Result<(FCore::Graph, metamodelica::Array<FCore::Node>, Extra)> + 'static>;

    let mut outGraph: Graph;
    let mut outExtra: Extra;
    (outGraph, outExtra) = (match inOptions.clone() {
        _ => (inGraph.clone(), inExtra.clone()),
    });
    (outGraph, outExtra)
}

