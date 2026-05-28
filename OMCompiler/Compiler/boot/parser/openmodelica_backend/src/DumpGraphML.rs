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

use crate::BackendDAE;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::GraphML;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// dump GraphML stuff
//
// =============================================================================
pub fn dumpSystem(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inids: Option<metamodelica::Array<i32>>, mut filename: ArcStr, mut numberMode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inSystem.clone(), inShared.clone(), inids.clone(), filename.clone(), numberMode.clone())) {
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::NO_MATCHING, .. }, _, None, _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32 = 0;
            let mut eqnsids: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut neqns: i32 = 0;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            vars = BackendVariable::daeVars(inSystem.clone());
            eqns = BackendEquation::getEqnsFromEqSystem(inSystem.clone());
            funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
            (_, m, _) = BackendDAEUtil::getAdjacencyMatrix(inSystem.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
            mapIncRowEqn = Array::createIntRange((m.clone().borrow().len() as i32));
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("G")).clone(), false, graphInfo.clone())?;
            graphInfo = __pa0.clone();
            graph = __pa1.clone();
            let (_, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(addVarGraph) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (bool, i32, (GraphML::GraphInfo, i32))) -> Result<(BackendDAE::Var, (bool, i32, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode.clone(), 1, (graphInfo.clone(), graph.clone())))?;
            graphInfo = __pa2.clone();
            graph = __pa3.clone();
            neqns = BackendEquation::getNumberOfEquations(eqns.clone());
            eqnsids = List::intRange(neqns.clone());
            (graphInfo, graph) = List::fold3(eqnsids.clone(), (std::sync::Arc::new(addEqnGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, bool, (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> + 'static>), eqns.clone(), mapIncRowEqn.clone(), numberMode.clone(), (graphInfo.clone(), graph.clone()));
            (_, _, graphInfo) = List::fold(eqnsids.clone(), (std::sync::Arc::new(fnptr!(addEdgesGraph, i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo)) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo)> + 'static>), (1, m.clone(), graphInfo.clone()));
            GraphML::dumpGraph(graphInfo.clone(), (filename.clone()).clone())?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::NO_MATCHING, mT: Some(_), m: Some(m), .. }, _, None, _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32 = 0;
            let mut eqnsids: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut neqns: i32 = 0;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            vars = BackendVariable::daeVars(inSystem.clone());
            eqns = BackendEquation::getEqnsFromEqSystem(inSystem.clone());
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("G")).clone(), false, graphInfo.clone())?;
            graphInfo = __pa0.clone();
            graph = __pa1.clone();
            let (_, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(addVarGraph) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (bool, i32, (GraphML::GraphInfo, i32))) -> Result<(BackendDAE::Var, (bool, i32, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode.clone(), 1, (graphInfo.clone(), graph.clone())))?;
            graphInfo = __pa2.clone();
            graph = __pa3.clone();
            neqns = BackendEquation::getNumberOfEquations(eqns.clone());
            eqnsids = List::intRange(neqns.clone());
            mapIncRowEqn = Array::createIntRange((m.clone().borrow().len() as i32));
            (graphInfo, graph) = List::fold3(eqnsids.clone(), (std::sync::Arc::new(addEqnGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, bool, (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> + 'static>), eqns.clone(), mapIncRowEqn.clone(), numberMode.clone(), (graphInfo.clone(), graph.clone()));
            (_, _, graphInfo) = List::fold(eqnsids.clone(), (std::sync::Arc::new(fnptr!(addEdgesGraph, i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo)) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo)> + 'static>), (1, m.clone(), graphInfo.clone()));
            GraphML::dumpGraph(graphInfo.clone(), (filename.clone()).clone())?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: Deref @ metamodelica::List::Nil, ass2: vec2, ass1: vec1 }, .. }, _, None, _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32 = 0;
            let mut eqnsids: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut neqns: i32 = 0;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut eqnsflag: metamodelica::Array<bool>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            vars = BackendVariable::daeVars(inSystem.clone());
            eqns = BackendEquation::getEqnsFromEqSystem(inSystem.clone());
            funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
            (_, m, _, _, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(inSystem.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("G")).clone(), false, graphInfo.clone())?;
            graphInfo = __pa0.clone();
            graph = __pa1.clone();
            let (_, _, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(addVarGraphMatch) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32))) -> Result<(BackendDAE::Var, (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode.clone(), 1, vec1.clone(), (graphInfo.clone(), graph.clone())))?;
            graphInfo = __pa2.clone();
            graph = __pa3.clone();
            neqns = BackendEquation::equationArraySize(eqns.clone())?;
            eqnsids = List::intRange(neqns.clone());
            eqnsflag = arrayCreate(neqns.clone(), false);
            (graphInfo, graph) = List::fold3(eqnsids.clone(), (std::sync::Arc::new(addEqnGraphMatch) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<bool>), bool, (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> + 'static>), eqns.clone(), (vec2.clone(), mapIncRowEqn.clone(), eqnsflag.clone()), numberMode.clone(), (graphInfo.clone(), graph.clone()));
            (_, _, _, _, graphInfo) = List::fold(eqnsids.clone(), (std::sync::Arc::new(fnptr!(addDirectedEdgesGraph, i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)> + 'static>), (1, m.clone(), vec2.clone(), mapIncRowEqn.clone(), graphInfo.clone()));
            GraphML::dumpGraph(graphInfo.clone(), (filename.clone()).clone())?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: Deref @ metamodelica::List::Nil, ass2: vec2, .. }, .. }, _, Some(vec3), _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32 = 0;
            let mut eqnsids: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut neqns: i32 = 0;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            vars = BackendVariable::daeVars(inSystem.clone());
            eqns = BackendEquation::getEqnsFromEqSystem(inSystem.clone());
            funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
            (_, m, _, _, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(inSystem.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("G")).clone(), false, graphInfo.clone())?;
            graphInfo = __pa0.clone();
            graph = __pa1.clone();
            let (_, _, (__pa2, __pa3)) = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(addVarGraph) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (bool, i32, (GraphML::GraphInfo, i32))) -> Result<(BackendDAE::Var, (bool, i32, (GraphML::GraphInfo, i32)))> + 'static>), (numberMode.clone(), 1, (graphInfo.clone(), graph.clone())))?;
            graphInfo = __pa2.clone();
            graph = __pa3.clone();
            neqns = BackendEquation::equationArraySize(eqns.clone())?;
            eqnsids = List::intRange(neqns.clone());
            (graphInfo, graph) = List::fold3(eqnsids.clone(), (std::sync::Arc::new(addEqnGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, bool, (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> + 'static>), eqns.clone(), mapIncRowEqn.clone(), numberMode.clone(), (graphInfo.clone(), graph.clone()));
            (_, _, _, _, graphInfo) = List::fold(eqnsids.clone(), (std::sync::Arc::new(fnptr!(addDirectedNumEdgesGraph, i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)> + 'static>), (1, m.clone(), vec2.clone(), vec3.clone(), graphInfo.clone()));
            GraphML::dumpGraph(graphInfo.clone(), (filename.clone()).clone())?;
            ()
        },
        (Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, .. }, _, None, _, _) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut graphInfo: GraphML::GraphInfo;
            let mut graph: i32 = 0;
            let mut vec3: metamodelica::Array<i32>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            vars = BackendVariable::daeVars(inSystem.clone());
            let _ = BackendEquation::getEqnsFromEqSystem(inSystem.clone());
            funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
            (_, m, mt) = BackendDAEUtil::getAdjacencyMatrix(inSystem.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("G")).clone(), false, graphInfo.clone())?;
            graphInfo = __pa0.clone();
            graph = __pa1.clone();
            vec3 = arrayCreate((mt.clone().borrow().len() as i32), -1);
            (graphInfo, graph) = addCompsGraph(comps.clone(), vars.clone(), vec3.clone(), 1, (graphInfo.clone(), graph.clone()))?;
            mapIncRowEqn = arrayCreate((mt.clone().borrow().len() as i32), -1);
            graphInfo = addCompsEdgesGraph(comps.clone(), m.clone(), vec3.clone(), 1, 1, mapIncRowEqn.clone(), 1, graphInfo.clone())?;
            GraphML::dumpGraph(graphInfo.clone(), (filename.clone()).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn addVarGraph(mut inVar: BackendDAE::Var, mut inTpl: (bool, i32, (GraphML::GraphInfo, i32))) -> Result<(BackendDAE::Var, (bool, i32, (GraphML::GraphInfo, i32)))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (bool, i32, (GraphML::GraphInfo, i32));
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (true, mut id, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut desc: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
            labelText = (intString(id.clone())).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            desc = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLUE)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((desc.clone()).clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (true, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (false, mut id, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut labelText: ArcStr = arcstr::literal!("");
            let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
            labelText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); ArcStr::from(__mm_s) }).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLUE)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (false, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (true, mut id, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut b: bool = false;
            let mut color: ArcStr = arcstr::literal!("");
            let mut desc: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            b = BackendVariable::isVarDiscrete(v.clone());
            color = (if (b.clone()) {arcstr::literal!(GraphML::COLOR_PURPLE)} else {arcstr::literal!(GraphML::COLOR_RED)}).clone();
            labelText = (intString(id.clone())).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            desc = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((desc.clone()).clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (true, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (false, mut id, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut b: bool = false;
            let mut color: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            b = BackendVariable::isVarDiscrete(v.clone());
            color = (if (b.clone()) {arcstr::literal!(GraphML::COLOR_PURPLE)} else {arcstr::literal!(GraphML::COLOR_RED)}).clone();
            labelText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); ArcStr::from(__mm_s) }).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (false, id.clone() + 1, (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inVar.clone(), inTpl.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn addVarGraphMatch(mut inVar: BackendDAE::Var, mut inTpl: (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32))) -> Result<(BackendDAE::Var, (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32)))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (bool, i32, metamodelica::Array<i32>, (GraphML::GraphInfo, i32));
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (false, mut id, mut vec1, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut color: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
            color = (if (intGt(vec1.borrow()[(id.clone()-1) as usize].clone(), 0)) {arcstr::literal!(GraphML::COLOR_BLUE)} else {arcstr::literal!(GraphML::COLOR_YELLOW)}).clone();
            labelText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); ArcStr::from(__mm_s) }).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (false, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (true, mut id, mut vec1, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut color: ArcStr = arcstr::literal!("");
            let mut desc: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
            color = (if (intGt(vec1.borrow()[(id.clone()-1) as usize].clone(), 0)) {arcstr::literal!(GraphML::COLOR_BLUE)} else {arcstr::literal!(GraphML::COLOR_YELLOW)}).clone();
            desc = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            labelText = (intString(id.clone())).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((desc.clone()).clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (true, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (false, mut id, mut vec1, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut color: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            color = (if (intGt(vec1.borrow()[(id.clone()-1) as usize].clone(), 0)) {arcstr::literal!(GraphML::COLOR_RED)} else {arcstr::literal!(GraphML::COLOR_YELLOW)}).clone();
            labelText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(id.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); ArcStr::from(__mm_s) }).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (false, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (ref v @ BackendDAE::Var { varName: ref cr, .. }, (true, mut id, mut vec1, (mut graphInfo, mut graph))) = __mc_input.clone() else { bail!("nomatch") };
            let mut label: GraphML::NodeLabel;
            let mut color: ArcStr = arcstr::literal!("");
            let mut desc: ArcStr = arcstr::literal!("");
            let mut labelText: ArcStr = arcstr::literal!("");
            color = (if (intGt(vec1.borrow()[(id.clone()-1) as usize].clone(), 0)) {arcstr::literal!(GraphML::COLOR_RED)} else {arcstr::literal!(GraphML::COLOR_YELLOW)}).clone();
            desc = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            labelText = (intString(id.clone())).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((desc.clone()).clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            Ok((v.clone(), (true, id.clone() + 1, vec1.clone(), (graphInfo.clone(), graph.clone()))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inVar.clone(), inTpl.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn addEqnGraph(mut inNode: i32, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut numberMode: bool, mut inGraph: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut outGraph: (GraphML::GraphInfo, i32);
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut graphInfo: GraphML::GraphInfo;
    let mut graph: i32 = 0;
    let mut label: GraphML::NodeLabel;
    let mut labelText: ArcStr = arcstr::literal!("");
    outGraph = (::match_deref::match_deref! { match &((inNode.clone(), eqns.clone(), mapIncRowEqn.clone(), numberMode.clone(), inGraph.clone())) {
        (_, _, _, false, (graphInfo, graph)) => {
            let mut graphInfo = (*graphInfo).clone();
            eqn = BackendEquation::get(eqns.clone(), mapIncRowEqn.borrow()[(inNode.clone()-1) as usize].clone());
            r#str = (BackendDump::equationString(eqn.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(inNode.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str = (Util::xmlEscape((r#str.clone()).clone())?).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (r#str.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(inNode.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_GREEN)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            (graphInfo.clone(), graph.clone())
        },
        (_, _, _, true, (graphInfo, graph)) => {
            let mut graphInfo = (*graphInfo).clone();
            eqn = BackendEquation::get(eqns.clone(), mapIncRowEqn.borrow()[(inNode.clone()-1) as usize].clone());
            r#str = (BackendDump::equationString(eqn.clone())?).clone();
            r#str = (Util::xmlEscape((r#str.clone()).clone())?).clone();
            labelText = (intString(inNode.clone())).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(inNode.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_GREEN)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, Some((r#str.clone()).clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            (graphInfo.clone(), graph.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outGraph)
}

fn addEdgesGraph(mut e: i32, mut inTpl: (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo)) -> (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo) {
    let mut outTpl: (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo);
    let mut id: i32 = 0;
    let mut graph: GraphML::GraphInfo;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (id, m, graph) = inTpl.clone();
    vars = List::select(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
    vars = m.borrow()[(e.clone()-1) as usize].clone();
    (id, graph) = List::fold1(vars.clone(), (std::sync::Arc::new(addEdgeGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, (i32, GraphML::GraphInfo)) -> Result<(i32, GraphML::GraphInfo)> + 'static>), e.clone(), (id.clone(), graph.clone()));
    outTpl = (id.clone(), m.clone(), graph.clone());
    outTpl
}

fn addEqnGraphMatch(mut inNode: i32, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut atpl: (metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<bool>), mut numberMode: bool, mut inGraph: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut outGraph: (GraphML::GraphInfo, i32);
    outGraph = 'mc: {
        let __mc_input = (inNode.clone(), eqns.clone(), atpl.clone(), numberMode.clone(), inGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (vec2, mapIncRowEqn, eqnsflag), false, (graphInfo, graph)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut color: ArcStr = arcstr::literal!("");
                    let mut e: i32 = 0;
                    let mut label: GraphML::NodeLabel;
                    let mut graphInfo = (*graphInfo).clone();
                    e = mapIncRowEqn.borrow()[(inNode.clone()-1) as usize].clone();
                    let false = (eqnsflag.borrow()[(e.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    eqn = BackendEquation::get(eqns.clone(), mapIncRowEqn.borrow()[(inNode.clone()-1) as usize].clone());
                    r#str = (BackendDump::equationString(eqn.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    r#str = (Util::xmlEscape((r#str.clone()).clone())?).clone();
                    color = (if (intGt(vec2.borrow()[(inNode.clone()-1) as usize].clone(), 0)) {arcstr::literal!(GraphML::COLOR_GREEN)} else {arcstr::literal!(GraphML::COLOR_PURPLE)}).clone();
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (r#str.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(e.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((graphInfo.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (vec2, mapIncRowEqn, eqnsflag), true, (graphInfo, graph)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut color: ArcStr = arcstr::literal!("");
                    let mut e: i32 = 0;
                    let mut label: GraphML::NodeLabel;
                    let mut labelText: ArcStr = arcstr::literal!("");
                    let mut graphInfo = (*graphInfo).clone();
                    e = mapIncRowEqn.borrow()[(inNode.clone()-1) as usize].clone();
                    let false = (eqnsflag.borrow()[(e.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    eqn = BackendEquation::get(eqns.clone(), mapIncRowEqn.borrow()[(inNode.clone()-1) as usize].clone());
                    r#str = (BackendDump::equationString(eqn.clone())?).clone();
                    r#str = (Util::xmlEscape((r#str.clone()).clone())?).clone();
                    color = (if (intGt(vec2.borrow()[(inNode.clone()-1) as usize].clone(), 0)) {arcstr::literal!(GraphML::COLOR_GREEN)} else {arcstr::literal!(GraphML::COLOR_PURPLE)}).clone();
                    labelText = (intString(e.clone())).clone();
                    label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (labelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
                    (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(e.clone())); ArcStr::from(__mm_s) }).clone(), (color.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, Some((r#str.clone()).clone()), metamodelica::nil(), graph.clone(), graphInfo.clone())?;
                    Ok((graphInfo.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (_, mapIncRowEqn, eqnsflag), _, _) => {
                    let mut e: i32 = 0;
                    e = mapIncRowEqn.borrow()[(inNode.clone()-1) as usize].clone();
                    let true = (eqnsflag.borrow()[(e.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    Ok(inGraph.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraph)
}

fn addEdgeGraph(mut V: i32, mut e: i32, mut inTpl: (i32, GraphML::GraphInfo)) -> Result<(i32, GraphML::GraphInfo)> {
    let mut outTpl: (i32, GraphML::GraphInfo);
    let mut id: i32 = 0;
    let mut v: i32 = 0;
    let mut graph: GraphML::GraphInfo;
    let mut ln: GraphML::LineType = GraphML::LineType::DASHED;
    (id, graph) = inTpl.clone();
    v = intAbs(V.clone());
    ln = if (intGt(V.clone(), 0)) {openmodelica_susan::GraphML::LineType::LINE} else {openmodelica_susan::GraphML::LineType::DASHED};
    (graph, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(e.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(v.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), ln.clone(), GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), graph.clone())?;
    outTpl = (id.clone() + 1, graph.clone());
    Ok(outTpl)
}

fn addDirectedEdgesGraph(mut e: i32, mut inTpl: (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)) -> (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo) {
    let mut outTpl: (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo);
    let mut id: i32 = 0;
    let mut v: i32 = 0;
    let mut n: i32 = 0;
    let mut graph: GraphML::GraphInfo;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vec2: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    (id, m, vec2, mapIncRowEqn, graph) = inTpl.clone();
    vars = m.borrow()[(e.clone()-1) as usize].clone();
    v = vec2.borrow()[(e.clone()-1) as usize].clone();
    (id, _, graph) = List::fold1(vars.clone(), (std::sync::Arc::new(addDirectedEdgeGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, (i32, i32, GraphML::GraphInfo)) -> Result<(i32, i32, GraphML::GraphInfo)> + 'static>), mapIncRowEqn.borrow()[(e.clone()-1) as usize].clone(), (id.clone(), v.clone(), graph.clone()));
    outTpl = (id.clone(), m.clone(), vec2.clone(), mapIncRowEqn.clone(), graph.clone());
    outTpl
}

fn addDirectedEdgeGraph(mut v: i32, mut e: i32, mut inTpl: (i32, i32, GraphML::GraphInfo)) -> Result<(i32, i32, GraphML::GraphInfo)> {
    let mut outTpl: (i32, i32, GraphML::GraphInfo);
    let mut id: i32 = 0;
    let mut r: i32 = 0;
    let mut absv: i32 = 0;
    let mut graph: GraphML::GraphInfo;
    let mut arrow: (GraphML::ArrowType, GraphML::ArrowType);
    let mut lt: GraphML::LineType = GraphML::LineType::DASHED;
    (id, r, graph) = inTpl.clone();
    absv = intAbs(v.clone());
    arrow = if (intEq(r.clone(), absv.clone())) {(openmodelica_susan::GraphML::ArrowType::ARROWSTANDART, openmodelica_susan::GraphML::ArrowType::ARROWNONE)} else {(openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART)};
    lt = if (intGt(v.clone(), 0)) {openmodelica_susan::GraphML::LineType::LINE} else {openmodelica_susan::GraphML::LineType::DASHED};
    (graph, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(e.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(absv.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), lt.clone(), GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), arrow.clone(), metamodelica::nil(), graph.clone())?;
    outTpl = (id.clone() + 1, r.clone(), graph.clone());
    Ok(outTpl)
}

fn addDirectedNumEdgesGraph(mut e: i32, mut inTpl: (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo)) -> (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo) {
    let mut outTpl: (i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<i32>, GraphML::GraphInfo);
    let mut id: i32 = 0;
    let mut v: i32 = 0;
    let mut graph: GraphML::GraphInfo;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vec2: metamodelica::Array<i32>;
    let mut vec3: metamodelica::Array<i32>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut text: ArcStr = arcstr::literal!("");
    (id, m, vec2, vec3, graph) = inTpl.clone();
    vars = List::select(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
    v = vec2.borrow()[(e.clone()-1) as usize].clone();
    text = (intString(vec3.borrow()[(e.clone()-1) as usize].clone())).clone();
    (id, _, _, graph) = List::fold1(vars.clone(), (std::sync::Arc::new(addDirectedNumEdgeGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, (i32, i32, ArcStr, GraphML::GraphInfo)) -> Result<(i32, i32, ArcStr, GraphML::GraphInfo)> + 'static>), e.clone(), (id.clone(), v.clone(), text.clone(), graph.clone()));
    outTpl = (id.clone(), m.clone(), vec2.clone(), vec3.clone(), graph.clone());
    outTpl
}

fn addDirectedNumEdgeGraph(mut v: i32, mut e: i32, mut inTpl: (i32, i32, ArcStr, GraphML::GraphInfo)) -> Result<(i32, i32, ArcStr, GraphML::GraphInfo)> {
    let mut outTpl: (i32, i32, ArcStr, GraphML::GraphInfo);
    let mut id: i32 = 0;
    let mut r: i32 = 0;
    let mut n: i32 = 0;
    let mut graph: GraphML::GraphInfo;
    let mut arrow: (GraphML::ArrowType, GraphML::ArrowType);
    let mut text: ArcStr = arcstr::literal!("");
    let mut labels: Arc<metamodelica::List<GraphML::EdgeLabel>> = metamodelica::nil();
    (id, r, text, graph) = inTpl.clone();
    arrow = if (intEq(r.clone(), v.clone())) {(openmodelica_susan::GraphML::ArrowType::ARROWSTANDART, openmodelica_susan::GraphML::ArrowType::ARROWNONE)} else {(openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART)};
    labels = if (intEq(r.clone(), v.clone())) {list![GraphML::EdgeLabel { text: (text.clone()).clone(), backgroundColor: Some((literal!("#0000FF")).clone()), fontSize: GraphML::FONTSIZE_STANDARD.clone() }]} else {metamodelica::nil()};
    (graph, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(e.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("v")); __mm_s.push_str(&*intString(v.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, labels.clone(), arrow.clone(), metamodelica::nil(), graph.clone())?;
    outTpl = (id.clone() + 1, r.clone(), text.clone(), graph.clone());
    Ok(outTpl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addCompsGraph(mut iComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut vars: BackendDAE::Variables, mut varcomp: metamodelica::Array<i32>, mut iN: i32, mut iGraph: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut oGraph: (GraphML::GraphInfo, i32);
    oGraph = (::match_deref::match_deref! { match &((iComps.clone(), vars.clone(), varcomp.clone(), iN.clone(), iGraph.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            iGraph.clone()
        },
        (Deref @ metamodelica::List::Cons { head: comp, tail: rest }, _, _, _, (graphInfo, graph)) => {
            let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut label: GraphML::NodeLabel;
            let mut varcomp1: metamodelica::Array<i32>;
            let mut text: ArcStr = arcstr::literal!("");
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut graphInfo = (*graphInfo).clone();
            (_, vlst) = BackendDAETransform::getEquationAndSolvedVarIndxes(comp.clone())?;
            varcomp1 = List::fold1r(vlst.clone(), Arc::new(arrayUpdate.clone()), iN.clone(), varcomp.clone());
            varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
            text = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(iN.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*stringDelimitList(List::mapMap(varlst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
            label = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (text.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(iN.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_GREEN)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![label.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, None, metamodelica::nil(), graph.clone(), graphInfo.clone())?;
            addCompsGraph(rest.clone(), vars.clone(), varcomp1.clone(), iN.clone() + 1, (graphInfo.clone(), graph.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oGraph)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addCompsEdgesGraph(mut iComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varcomp: metamodelica::Array<i32>, mut iN: i32, mut id: i32, mut markarray: metamodelica::Array<i32>, mut mark: i32, mut iGraph: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraph: GraphML::GraphInfo;
    oGraph = (::match_deref::match_deref! { match &((iComps.clone(), m.clone(), varcomp.clone(), iN.clone(), id.clone(), markarray.clone(), mark.clone(), iGraph.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
            iGraph.clone()
        },
        (Deref @ metamodelica::List::Cons { head: comp, tail: rest }, _, _, _, _, _, _, _) => {
            let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut n: i32 = 0;
            let mut graph: GraphML::GraphInfo;
            (elst, vlst) = BackendDAETransform::getEquationAndSolvedVarIndxes(comp.clone())?;
            let _ = List::fold1r(vlst.clone(), Arc::new(arrayUpdate.clone()), mark.clone(), markarray.clone());
            vlst = getUsedVarsComp(elst.clone(), m.clone(), markarray.clone(), mark.clone());
            (n, graph) = addCompEdgesGraph(vlst.clone(), varcomp.clone(), markarray.clone(), mark.clone() + 1, iN.clone(), id.clone(), iGraph.clone())?;
            addCompsEdgesGraph(rest.clone(), m.clone(), varcomp.clone(), iN.clone() + 1, n.clone(), markarray.clone(), mark.clone() + 2, graph.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oGraph)
}

fn getUsedVarsComp(mut iEqns: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut markarray: metamodelica::Array<i32>, mut mark: i32) -> Arc<metamodelica::List<i32>> {
    let mut oVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut eq in &*iEqns.clone() {
        let mut eq = eq.clone();
        vlst = List::select1(m.borrow()[(eq.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
        vlst = List::select1r(vlst.clone(), (std::sync::Arc::new(fnptr!(isUnMarked, (metamodelica::Array<i32>, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Array<i32>, i32), i32) -> Result<bool> + 'static>), (markarray.clone(), mark.clone()));
        let _ = List::fold1r(vlst.clone(), Arc::new(arrayUpdate.clone()), mark.clone(), markarray.clone());
        oVars = listAppend(vlst.clone(), oVars.clone());
    }
    oVars
}

fn addCompEdgesGraph(mut iVars: Arc<metamodelica::List<i32>>, mut varcomp: metamodelica::Array<i32>, mut markarray: metamodelica::Array<i32>, mut mark: i32, mut iN: i32, mut id: i32, mut iGraph: GraphML::GraphInfo) -> Result<(i32, GraphML::GraphInfo)> {
    let mut oN: i32 = 0;
    let mut oGraph: GraphML::GraphInfo;
    (oN, oGraph) = 'mc: {
        let __mc_input = (iVars.clone(), varcomp.clone(), markarray.clone(), mark.clone(), iN.clone(), id.clone(), iGraph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
                    Ok((id.clone(), iGraph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v, tail: rest }, _, _, _, _, _, _) => {
                    let mut n: i32 = 0;
                    let mut c: i32 = 0;
                    let mut graph: GraphML::GraphInfo;
                    c = varcomp.borrow()[(v.clone()-1) as usize].clone();
                    let false = (intEq(markarray.borrow()[(c.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = markarray.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = mark.clone(); _arr};
                    (graph, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*intString(id.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(c.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*intString(iN.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWSTANDART, openmodelica_susan::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), iGraph.clone())?;
                    (n, graph) = addCompEdgesGraph(rest.clone(), varcomp.clone(), markarray.clone(), mark.clone(), iN.clone(), id.clone() + 1, graph.clone())?;
                    Ok((n.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _) => {
                    let mut n: i32 = 0;
                    let mut graph: GraphML::GraphInfo;
                    (n, graph) = addCompEdgesGraph(rest.clone(), varcomp.clone(), markarray.clone(), mark.clone(), iN.clone(), id.clone(), iGraph.clone())?;
                    Ok((n.clone(), graph.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oN, oGraph))
}

fn isUnMarked(mut ass: (metamodelica::Array<i32>, i32), mut indx: i32) -> bool {
    let mut b: bool = false;
    let mut arr: metamodelica::Array<i32>;
    let mut mark: i32 = 0;
    (arr, mark) = ass.clone();
    b = !(intEq(arr.borrow()[(intAbs(indx.clone())-1) as usize].clone(), mark.clone()));
    b
}

