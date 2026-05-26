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

use openmodelica_frontend_types::Values;
use openmodelica_util::OMSimulatorExt;

pub fn ceval(mut inFunctionName: ArcStr, mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((inFunctionName.clone(), inVals.clone())) {
        (Deref @ "loadOMSimulator", Deref @ metamodelica::List::Nil) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::loadOMSimulator();
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "unloadOMSimulator", Deref @ metamodelica::List::Nil) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::unloadOMSimulator();
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addBus", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addBus((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: crefA }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: crefB }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addConnection((crefA.clone()).clone(), (crefB.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addConnector", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: causality, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: type_, .. }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addConnector((cref.clone()).clone(), causality.clone() - 1, type_.clone() - 1);
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addConnectorToBus", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addConnectorToBus((busCref.clone()).clone(), (connectorCref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addConnectorToTLMBus", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: stype_ }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addConnectorToTLMBus((busCref.clone()).clone(), (connectorCref.clone()).clone(), (stype_.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addDynamicValueIndicator", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s_lower }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s_upper }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: stepSize }, tail: Deref @ metamodelica::List::Nil } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addDynamicValueIndicator((signal.clone()).clone(), (s_lower.clone()).clone(), (s_upper.clone()).clone(), stepSize.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addEventIndicator", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addEventIndicator((signal.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addExternalModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: path }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: startscript }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addExternalModel((cref.clone()).clone(), (path.clone()).clone(), (startscript.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addSignalsToResults", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: regex }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addSignalsToResults((cref.clone()).clone(), (regex.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addStaticValueIndicator", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: lower }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: upper }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: stepSize }, tail: Deref @ metamodelica::List::Nil } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addStaticValueIndicator((signal.clone()).clone(), lower.clone(), upper.clone(), stepSize.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addSubModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: fmuPath }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addSubModel((cref.clone()).clone(), (fmuPath.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addSystem", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: type_, .. }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addSystem((cref.clone()).clone(), type_.clone() - 1);
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addTimeIndicator", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addTimeIndicator((signal.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addTLMBus", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: domain, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: dimensions }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: interpolation, .. }, tail: Deref @ metamodelica::List::Nil } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addTLMBus((cref.clone()).clone(), domain.clone() - 1, dimensions.clone(), interpolation.clone() - 1);
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_addTLMConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: crefA }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: crefB }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: delay }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: alpha }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: linearimpedance }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: angularimpedance }, tail: Deref @ metamodelica::List::Nil } } } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_addTLMConnection((crefA.clone()).clone(), (crefB.clone()).clone(), delay.clone(), alpha.clone(), linearimpedance.clone(), angularimpedance.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_compareSimulationResults", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameA }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameB }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: var }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: relTol }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: absTol }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_compareSimulationResults((filenameA.clone()).clone(), (filenameB.clone()).clone(), (var.clone()).clone(), relTol.clone(), absTol.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_copySystem", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: source }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: target }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_copySystem((source.clone()).clone(), (target.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_delete", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_delete((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_deleteConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: crefA }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: crefB }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_deleteConnection((crefA.clone()).clone(), (crefB.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_deleteConnectorFromBus", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_deleteConnectorFromBus((busCref.clone()).clone(), (connectorCref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_deleteConnectorFromTLMBus", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_deleteConnectorFromTLMBus((busCref.clone()).clone(), (connectorCref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_export", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_export((cref.clone()).clone(), (filename.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_exportDependencyGraphs", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: initialization }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: event }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: simulation }, tail: Deref @ metamodelica::List::Nil } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_exportDependencyGraphs((cref.clone()).clone(), (initialization.clone()).clone(), (event.clone()).clone(), (simulation.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_exportSnapshot", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut contents: ArcStr = arcstr::literal!("");
            let mut status: i32 = 0;
            (contents, status) = OMSimulatorExt::oms_exportSnapshot((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (contents.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_extractFMIKind", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            let mut kind: i32 = 0;
            (kind, status) = OMSimulatorExt::oms_extractFMIKind((filename.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::INTEGER { integer: kind.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getBoolean", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            let mut b: bool = false;
            (b, status) = OMSimulatorExt::oms_getBoolean((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: b.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getFixedStepSize", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut rvalue: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut status: i32 = 0;
            (rvalue, status) = OMSimulatorExt::oms_getFixedStepSize((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: rvalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getInteger", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            let mut ivalue: i32 = 0;
            (ivalue, status) = OMSimulatorExt::oms_getInteger((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::INTEGER { integer: ivalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getModelState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            let mut ivalue: i32 = 0;
            (ivalue, status) = OMSimulatorExt::oms_getModelState((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::INTEGER { integer: ivalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getReal", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut rvalue: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut status: i32 = 0;
            (rvalue, status) = OMSimulatorExt::oms_getReal((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: rvalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getSolver", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            let mut ivalue: i32 = 0;
            (ivalue, status) = OMSimulatorExt::oms_getSolver((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::INTEGER { integer: ivalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getStartTime", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut rvalue: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut status: i32 = 0;
            (rvalue, status) = OMSimulatorExt::oms_getStartTime((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: rvalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getStopTime", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut rvalue: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut status: i32 = 0;
            (rvalue, status) = OMSimulatorExt::oms_getStopTime((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: rvalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getSubModelPath", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut path: ArcStr = arcstr::literal!("");
            let mut status: i32 = 0;
            (path, status) = OMSimulatorExt::oms_getSubModelPath((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (path.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getSystemType", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            let mut ivalue: i32 = 0;
            (ivalue, status) = OMSimulatorExt::oms_getSystemType((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::INTEGER { integer: ivalue.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getTolerance", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut absoluteTolerance: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut relativeTolerance: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut status: i32 = 0;
            (absoluteTolerance, relativeTolerance, status) = OMSimulatorExt::oms_getTolerance((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: absoluteTolerance.clone() }), Arc::new(Values::Value::REAL { real: relativeTolerance.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_getVariableStepSize", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut initialStepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut minimumStepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut maximumStepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut status: i32 = 0;
            (initialStepSize, minimumStepSize, maximumStepSize, status) = OMSimulatorExt::oms_getVariableStepSize((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: initialStepSize.clone() }), Arc::new(Values::Value::REAL { real: minimumStepSize.clone() }), Arc::new(Values::Value::REAL { real: maximumStepSize.clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_faultInjection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: faultType, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: faultValue }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_faultInjection((signal.clone()).clone(), faultType.clone() - 1, faultValue.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_importFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut cref: ArcStr = arcstr::literal!("");
            let mut status: i32 = 0;
            (cref, status) = OMSimulatorExt::oms_importFile((filename.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_importSnapshot", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: snapshot }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_importSnapshot((cref.clone()).clone(), (snapshot.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_initialize", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_initialize((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_instantiate", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_instantiate((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_list", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut contents: ArcStr = arcstr::literal!("");
            let mut status: i32 = 0;
            (contents, status) = OMSimulatorExt::oms_list((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (contents.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_listUnconnectedConnectors", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut contents: ArcStr = arcstr::literal!("");
            let mut status: i32 = 0;
            (contents, status) = OMSimulatorExt::oms_listUnconnectedConnectors((cref.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (contents.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_loadSnapshot", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: snapshot }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut newCref: ArcStr = arcstr::literal!("");
            let mut status: i32 = 0;
            (newCref, status) = OMSimulatorExt::oms_loadSnapshot((cref.clone()).clone(), (snapshot.clone()).clone());
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (newCref.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: status.clone() })] })
        },
        (Deref @ "oms_newModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_newModel((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_removeSignalsFromResults", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: regex }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_removeSignalsFromResults((cref.clone()).clone(), (regex.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_rename", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: newCref }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_rename((cref.clone()).clone(), (newCref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_reset", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_reset((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_RunFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_RunFile((filename.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setBoolean", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setBoolean((cref.clone()).clone(), b.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setCommandLineOption", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setCommandLineOption((cmd.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setFixedStepSize", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: stepSize }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setFixedStepSize((cref.clone()).clone(), stepSize.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setInteger", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: ivalue }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setInteger((cref.clone()).clone(), ivalue.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setLogFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setLogFile((filename.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setLoggingInterval", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: loggingInterval }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setLoggingInterval((cref.clone()).clone(), loggingInterval.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setLoggingLevel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: logLevel }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setLoggingLevel(logLevel.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setReal", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rvalue }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setReal((cref.clone()).clone(), rvalue.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setRealInputDerivative", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rvalue }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setRealInputDerivative((cref.clone()).clone(), rvalue.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setResultFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: bufferSize }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setResultFile((cref.clone()).clone(), (filename.clone()).clone(), bufferSize.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setSignalFilter", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: regex }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setSignalFilter((cref.clone()).clone(), (regex.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setSolver", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: solver, .. }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setSolver((cref.clone()).clone(), solver.clone() - 1);
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setStartTime", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: startTime }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setStartTime((cref.clone()).clone(), startTime.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setStopTime", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: stopTime }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setStopTime((cref.clone()).clone(), stopTime.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setTempDirectory", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: newTempDir }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setTempDirectory((newTempDir.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setTLMPositionAndOrientation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A11 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A12 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A13 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A21 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A22 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A23 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A31 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A32 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: A33 }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setTLMPositionAndOrientation((cref.clone()).clone(), x1.clone(), x2.clone(), x3.clone(), A11.clone(), A12.clone(), A13.clone(), A21.clone(), A22.clone(), A23.clone(), A31.clone(), A32.clone(), A33.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setTLMSocketData", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: address }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: managerPort }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: monitorPort }, tail: Deref @ metamodelica::List::Nil } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setTLMSocketData((cref.clone()).clone(), (address.clone()).clone(), managerPort.clone(), monitorPort.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setTolerance", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: absoluteTolerance }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: relativeTolerance }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setTolerance((cref.clone()).clone(), absoluteTolerance.clone(), relativeTolerance.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setVariableStepSize", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: initialStepSize }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: minimumStepSize }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: maximumStepSize }, tail: Deref @ metamodelica::List::Nil } } } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setVariableStepSize((cref.clone()).clone(), initialStepSize.clone(), minimumStepSize.clone(), maximumStepSize.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_setWorkingDirectory", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: newWorkingDir }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_setWorkingDirectory((newWorkingDir.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_simulate", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_simulate((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_stepUntil", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: stopTime }, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_stepUntil((cref.clone()).clone(), stopTime.clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_terminate", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut status: i32 = 0;
            status = OMSimulatorExt::oms_terminate((cref.clone()).clone());
            Arc::new(Values::Value::INTEGER { integer: status.clone() })
        },
        (Deref @ "oms_getVersion", Deref @ metamodelica::List::Nil) => {
            let mut version: ArcStr = arcstr::literal!("");
            version = (OMSimulatorExt::oms_getVersion()).clone();
            Arc::new(Values::Value::STRING { string: (version.clone()).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

